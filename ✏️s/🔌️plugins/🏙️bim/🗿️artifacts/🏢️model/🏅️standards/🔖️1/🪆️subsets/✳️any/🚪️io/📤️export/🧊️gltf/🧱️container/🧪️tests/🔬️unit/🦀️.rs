use super::*;

#[test]
fn a_packed_document_splits_back_into_its_json_and_its_buffer() {
    let bytes = pack_glb("{\"asset\":{\"version\":\"2.0\"}}", &[1, 2, 3, 4, 5]);
    let (json, buffer) = split_glb(&bytes).expect("the container splits");
    assert_eq!(json.trim_end(), "{\"asset\":{\"version\":\"2.0\"}}");
    assert_eq!(buffer, [1, 2, 3, 4, 5, 0, 0, 0]);
}

#[test]
fn every_chunk_and_the_total_are_four_byte_aligned_and_the_header_is_truthful() {
    let bytes = pack_glb("{}", &[9; 7]);
    assert_eq!(bytes.len() % 4, 0);
    assert_eq!(&bytes[0..4], b"glTF");
    assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()), 2);
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize, bytes.len());
    assert_eq!(u32::from_le_bytes(bytes[12..16].try_into().unwrap()) % 4, 0);
}

#[test]
fn an_empty_buffer_writes_no_binary_chunk() {
    let bytes = pack_glb("{}", &[]);
    assert_eq!(bytes.len(), 12 + 8 + 4);
    assert!(split_glb(&bytes).expect("splits").1.is_empty());
}

#[test]
fn malformed_containers_are_refused() {
    let good = pack_glb("{}", &[1, 2, 3, 4]);
    assert!(split_glb(&good[..10]).is_err());
    let mut wrong_magic = good.clone();
    wrong_magic[0] = b'X';
    assert!(split_glb(&wrong_magic).unwrap_err().contains("magic"));
    let mut wrong_version = good.clone();
    wrong_version[4] = 1;
    assert!(split_glb(&wrong_version).unwrap_err().contains("version"));
    assert!(split_glb(&good[..good.len() - 4]).unwrap_err().contains("length"));
}
