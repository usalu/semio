//! 🔢️ Executes the actual private absolute reader against the closed unsigned corpus.
use super::*;
use std::cell::Cell;

struct ObservedSource<'a> {
    bytes: &'a [u8],
    reads: Cell<usize>,
    offsets: Cell<[u64; 16]>,
    widths: Cell<[usize; 16]>,
}
impl PackSource for ObservedSource<'_> {
    async fn len(&self) -> u64 { self.bytes.len() as u64 }
    async fn read_at(&self, offset: u64, buffer: &mut [u8]) -> Result<usize, PackError> {
        let index = self.reads.get();
        assert!(index < 16, "absolute unsigned reader exceeded bounded observed reads");
        let mut offsets = self.offsets.get(); offsets[index] = offset; self.offsets.set(offsets);
        let mut widths = self.widths.get(); widths[index] = buffer.len(); self.widths.set(widths);
        self.reads.set(index + 1);
        let start = usize::try_from(offset).unwrap();
        if start >= self.bytes.len() { return Ok(0); }
        let count = buffer.len().min(self.bytes.len() - start);
        buffer[..count].copy_from_slice(&self.bytes[start..start + count]);
        Ok(count)
    }
}
fn hex(bytes: &str) -> Vec<u8> {
    bytes.as_bytes().chunks_exact(2).map(|pair| {
        let nibble = |byte: u8| match byte { b'0'..=b'9' => byte - b'0', b'a'..=b'f' => byte - b'a' + 10, _ => panic!("invalid authored hexadecimal byte") };
        (nibble(pair[0]) << 4) | nibble(pair[1])
    }).collect()
}

#[semio_framework_async_macros::async_test]
async fn pack_absolute_unsigned_varint_complete_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let offset = fixture["offset"].as_u64().unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 34);
    for row in cases {
        let id = row["id"].as_str().unwrap();
        let mut bytes = hex(fixture["prefixHex"].as_str().unwrap());
        bytes.extend(hex(row["bodyHex"].as_str().unwrap()));
        bytes.extend(hex(row["trailingHex"].as_str().unwrap()));
        let source = ObservedSource { bytes: &bytes, reads: Cell::new(0), offsets: Cell::new([0;16]), widths: Cell::new([0;16]) };
        let result = read_varint_u64_at(&source, offset).await;
        let outcome = &row["outcome"];
        if let Some(value) = outcome["valueDecimal"].as_str() {
            let expected = value.parse::<u64>().unwrap();
            let consumed = outcome["consumed"].as_u64().unwrap();
            assert_eq!(result.unwrap(), (expected, consumed), "{id}");
            assert_eq!(&bytes[(offset + consumed) as usize..], hex(row["trailingHex"].as_str().unwrap()), "{id}");
            let mut position = offset as usize;
            assert_eq!(read_varint_u64(&bytes, &mut position).unwrap(), expected, "{id}");
            assert_eq!(position, (offset + consumed) as usize, "{id}");
        } else {
            let expected_offset = outcome["offset"].as_u64().unwrap();
            match (outcome["kind"].as_str().unwrap(), result.unwrap_err()) {
                ("truncated", PackError::Truncated(actual)) => assert_eq!(actual, expected_offset, "{id}"),
                ("malformed", PackError::Malformed {what, offset: actual, ..}) => { assert_eq!(what, "varint", "{id}"); assert_eq!(actual, expected_offset, "{id}"); }
                (_, error) => panic!("{id}: unexpected actual refusal {error:?}"),
            }
            let mut position = offset as usize;
            match (outcome["kind"].as_str().unwrap(), read_varint_u64(&bytes, &mut position).unwrap_err()) {
                ("truncated", PackError::Truncated(actual)) => assert_eq!(actual, expected_offset, "{id}"),
                ("malformed", PackError::Malformed {what, offset: actual, ..}) => { assert_eq!(what, "varint", "{id}"); assert_eq!(actual, offset, "{id}"); }
                (_, error) => panic!("{id}: unexpected canonical primitive refusal {error:?}"),
            }
        }
        let reads = row["reads"].as_u64().unwrap() as usize;
        assert_eq!(source.reads.get(), reads, "{id}");
        for index in 0..reads {
            assert_eq!(source.offsets.get()[index], offset + index as u64, "{id}");
            assert_eq!(source.widths.get()[index], 1, "{id}");
        }
    }
}
