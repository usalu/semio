//! 🧱️ Retained export backing, frame addresses and bounded format primitives.
use semio_plugin::app::ArtifactOutputChunks;

pub const MAX_LAYOUT_EXPORT_OUTPUT_BYTES:usize=ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES;
pub const OUTPUT_CHUNK_BYTES:usize=ArtifactOutputChunks::CHUNK_BYTES;

#[path="../📤️output/🦀️.rs"]
pub mod output;
#[path="../🗃️plan/🦀️.rs"]
pub mod plan_storage;
#[path="../🖼️frame-planning/👁️state.rs"]
pub mod frame_planning;
#[path="../📕️pdf-png-and-package-encoding/📦️zip/🦀️.rs"]
pub mod zip;
#[path="../📕️pdf-png-and-package-encoding/🖼️raster/🦀️.rs"]
pub mod raster;
#[path="../📕️pdf-png-and-package-encoding/🔤️unicode/🦀️.rs"]
pub mod unicode;
#[path="../📕️pdf-png-and-package-encoding/📝️text/🦀️.rs"]
pub mod semantic_text;
#[path="../📕️pdf-png-and-package-encoding/🧭️projection/🦀️.rs"]
pub mod projection;

#[cfg(any(test,feature="testkit"))]
#[path="🧪️testing/🦀️.rs"]
pub mod test_oracle;
#[cfg(test)]
pub use test_oracle::{close_output_fixture,fixture_effect,LAYOUT_EXPORT_FIXTURE_TURN};
#[cfg(test)]
use semio_plugin::Fault;

pub fn crc32_update(mut crc: u32, bytes: &[u8]) -> u32 {
    crc = !crc;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb88320 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}

pub fn decode_base64(value: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::with_capacity(value.len() / 4 * 3);
    for quartet in value.as_bytes().chunks_exact(4) {
        let a = base64_value(quartet[0])?;
        let b = base64_value(quartet[1])?;
        let c = base64_value(quartet[2])?;
        let d = base64_value(quartet[3])?;
        result.push(a << 2 | b >> 4);
        if c != 64 {
            result.push(b << 4 | c >> 2);
        }
        if d != 64 {
            result.push(c << 6 | d);
        }
    }
    if value.len() % 4 != 0 {
        return Err("layout-export-invalid-base64".into());
    }
    Ok(result)
}

fn base64_value(byte: u8) -> Result<u8, String> {
    match byte {
        b'A'..=b'Z' => Ok(byte - b'A'),
        b'a'..=b'z' => Ok(byte - b'a' + 26),
        b'0'..=b'9' => Ok(byte - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        b'=' => Ok(64),
        _ => Err("layout-export-invalid-base64".into()),
    }
}
