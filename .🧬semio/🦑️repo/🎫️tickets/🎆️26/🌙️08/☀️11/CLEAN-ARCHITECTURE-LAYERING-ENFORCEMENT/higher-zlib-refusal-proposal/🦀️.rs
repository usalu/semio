//! 🗜️ Bare RFC1950 APIs retain typed refusal authority under explicit controls.
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError};
use crate::schema::snapshot::native::{decompress_zlib,compress_zlib};
#[test]
fn bare_zlib_retains_owned_refusal_and_exact_empty_stream() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️refusal/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap(){
        let hex=row["inputHex"].as_str().unwrap();
        let bytes=hex.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let maximum=usize::try_from(row["maximumBytes"].as_u64().unwrap()).unwrap();
        let result:Result<Vec<u8>,ValueError>=if row["operation"].as_str().unwrap()=="decompress" {
            let mut callback=|_|row["accept"].as_bool().unwrap();
            let mut control=NativeDecodeControl::new(maximum,&mut callback);
            decompress_zlib(&bytes,usize::try_from(row["maximumOutput"].as_u64().unwrap()).unwrap(),&mut control)
        }else{
            let mut callback=|_|row["accept"].as_bool().unwrap();
            let mut control=NativeEncodeControl::new(maximum,&mut callback);
            compress_zlib(&bytes,usize::try_from(row["maximumFile"].as_u64().unwrap()).unwrap(),&mut control)
        };
        if let Some(kind)=row["expected"].get("kind") {
            assert_eq!(result.expect_err("typed fixture refusal").kind.as_str(),kind.as_str().unwrap(),"{}",row["name"]);
        }else{
            let bytes=result.unwrap();let hex=bytes.iter().map(|byte|format!("{byte:02x}")).collect::<String>();
            assert_eq!(hex,row["expected"]["hex"].as_str().unwrap(),"{}",row["name"]);
        }
    }
}
