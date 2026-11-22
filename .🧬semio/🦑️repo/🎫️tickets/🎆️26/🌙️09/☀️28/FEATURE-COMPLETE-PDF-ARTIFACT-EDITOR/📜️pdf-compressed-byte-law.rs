use filters::{PdfStreamDataCursor, StreamDataGrant, StreamDataStatus};

struct MissingByte<'a> { bytes: &'a [u8], missing: usize }

impl ByteReadSource for MissingByte<'_> {
    fn byte_len(&self) -> usize { self.bytes.len() }
    fn byte_at(&self, at: usize) -> Option<u8> { if at == self.missing { None } else { self.bytes.get(at).copied() } }
}

fn stream_output(mut cursor: PdfStreamDataCursor, source: &impl ByteReadSource, expected: Result<&[u8], ValueRefusalKind>) {
    let mut held = 0;
    let grant = StreamDataGrant { work: 1, allocation_bytes: 65_536, release_bytes: 65_536 };
    assert!((0..30_000).any(|_| {
        let receipt = cursor.advance(source, grant, false);
        assert!(receipt.work <= 1 && receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
        held += receipt.allocated_bytes;
        held = held.checked_sub(receipt.released_bytes).unwrap();
        match receipt.status {
            Ok(StreamDataStatus::More) => false,
            Ok(StreamDataStatus::Complete) => {
                let output = cursor.take_output().unwrap();
                assert_eq!(output, expected.unwrap());
                held -= output.capacity();
                true
            }
            Err(error) => {
                assert_eq!(error.kind, expected.unwrap_err());
                assert!(cursor.take_output().is_none());
                true
            }
            event => panic!("unexpected segmented codec turn {event:?}"),
        }
    }));
    cursor.begin_close();
    assert!((0..30_000).any(|_| {
        let receipt = cursor.close_step(grant);
        assert!(receipt.work <= 1 && receipt.allocated_bytes == 0 && receipt.released_bytes <= grant.release_bytes);
        held = held.checked_sub(receipt.released_bytes).unwrap();
        receipt.status.unwrap() == StreamDataStatus::Closed
    }));
    assert_eq!(held, 0);
}

fn exercise_codecs() {
    let zlib = [120, 156, 99, 224, 18, 145, 99, 208, 48, 178, 1, 0, 2, 116, 0, 211];
    let plain = [0, 10, 20, 30, 0, 40, 50, 60];
    stream_output(PdfStreamDataCursor::flate(8), &NonContiguous(&zlib), Ok(&plain));
    let mut damaged = zlib;
    damaged[15] ^= 1;
    stream_output(PdfStreamDataCursor::flate(8), &NonContiguous(&damaged), Err(ValueRefusalKind::InvalidValue));
    stream_output(PdfStreamDataCursor::flate(8), &MissingByte { bytes: &zlib, missing: 15 }, Err(ValueRefusalKind::InvariantViolated));
    stream_output(PdfStreamDataCursor::flate_predicted(8, 6, 15, 3, 1, 8).unwrap(), &NonContiguous(&zlib), Ok(&[10, 20, 30, 40, 50, 60]));
    stream_output(PdfStreamDataCursor::lzw(2, true), &NonContiguous(&[128, 16, 72, 80, 16]), Ok(b"AB"));
    stream_output(PdfStreamDataCursor::ascii85(12), &NonContiguous(b"<~87cURD]j7BEbo80~>"), Ok(b"Hello world!"));
    stream_output(PdfStreamDataCursor::ascii_hex(1), &MissingByte { bytes: b"41>", missing: 1 }, Err(ValueRefusalKind::InvariantViolated));
    stream_output(PdfStreamDataCursor::ascii85(12), &MissingByte { bytes: b"<~87cURD]j7BEbo80~>", missing: 14 }, Err(ValueRefusalKind::InvariantViolated));
    stream_output(PdfStreamDataCursor::run_length(6), &NonContiguous(&[2, b'A', b'B', b'C', 254, b'X', 128]), Ok(b"ABCXXX"));
    println!("[DEBUG] Verified segmented Flate, predictor, LZW, ASCII85, RunLength and admitted-byte/Adler-32 failures under one-unit grants");
}

