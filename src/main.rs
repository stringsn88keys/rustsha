use sha2::{Sha512, Digest};
use base64::{encode};


fn main() {

    let mut counter = 0;
    loop {
        let mut hasher = Sha512::new();
        hasher.update(format!("{}{:010}", "Message", counter));
        let result = hasher.finalize();
        // "z4PhNX7vuL3xVChQ1m2AB9Yg5AULVxXcg/SpIdNs6c5H0NE8XYXysP+DGNKHfuwvY7kxvUdBeoGlODJ6+SfaPg=="
        let encoded = encode(result);
        if encoded.starts_with("NICE") {
            println!("{}", encoded);
            break;
        }
        counter+=1;
    }
    println!("{}", counter);
}
