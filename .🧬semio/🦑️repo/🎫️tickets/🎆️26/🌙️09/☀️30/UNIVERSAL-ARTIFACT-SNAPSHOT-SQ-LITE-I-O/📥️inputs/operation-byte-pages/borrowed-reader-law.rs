#[test]
fn owned_operation_byte_pages_borrowed_reader_keeps_the_same_8194_source_without_flattening() {
    use crate::codec::{ByteReader, ByteSpan};
    use super::bytes::OperationByteOutput;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected: Vec<u8> = fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32, 8192)).collect();
    let mut owner = OwnedOperationBytes::try_new(8194, 65536).unwrap();
    let mut allow = |_| true;
    let mut control = crate::value::NativeEncodeControl::new(65536, &mut allow);
    owner.write_bytes(&expected, &mut control).unwrap();
    let ((), allocated, returned) = crate::test_allocation::observe_backing(|| {
        let span = ByteSpan::from_source(&owner);
        assert_eq!(span.len(), expected.len());
        assert!(span.iter().eq(expected.iter().copied()));
        assert!(std::ptr::eq(span.get(4096).unwrap(), owner.byte_ref(4096).unwrap()));
        let mut reader = ByteReader::from_source(&owner);
        assert_eq!(reader.read_u16_le().unwrap(), 0x3731);
        reader.read_span(4092).unwrap();
        assert_eq!(reader.read_u64_le().unwrap(), 0x2020202020202020);
        assert_eq!(reader.position(), 4102);
        let mut fork = reader.fork();
        assert_eq!(fork.read_f64_le().unwrap().to_bits(), 0x2020202020202020);
        assert_eq!(reader.position(), 4102);
        let mut refused = ByteReader::from_source(&owner);
        let error = refused.read_bytes(8194).unwrap_err();
        assert_eq!(error.kind(), crate::value::ValueRefusalKind::UnsupportedOwner);
        assert!(matches!(error, crate::PackRefusal::RetainedMalformed { offset: 0, .. }));
        assert_eq!(refused.position(), 0);
        assert!(refused.read_span(8195).is_err());
        assert_eq!(refused.position(), 0);
        let tail = span.slice(8190, 4).unwrap();
        assert!(tail.iter().eq([32; 4]));
    });
    assert_eq!((allocated, returned), (0, 0));
    for _ in 0..8322 {
        if owner.close_one(1, 4096).unwrap() == OperationByteCloseStep::Complete { break; }
    }
    assert!(owner.terminal_is_empty());
    println!("[DEBUG] Paged operation reader borrows the same full 8194 source, crosses physical pages with fixed stack words, and never allocates a flattened operation");
}
