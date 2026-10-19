#[allow(unused_imports)]
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::decode_glb;
#[allow(unused_imports)]
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::export::encode_glb;
use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot;
use semio_s_artifact_stdio_gltf_model_snapshot::standards::v2_0::subsets::any::schema::snapshot::{GltfDocument, GltfBuffer, GltfScene, GltfSourceForm};
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gltf::codec::{encode, decode};

fn snapshot(bytes: Vec<u8>) -> GltfSnapshot {
    let mut document = GltfDocument::default();
    document.buffers.push(GltfBuffer { byte_length: bytes.len(), uri: None, name: None, extensions: None, extras: None });
    document.scenes.push(GltfScene { nodes: Vec::new(), name: Some("s".into()), extensions: None, extras: None });
    document.scene = Some(0);
    GltfSnapshot { document, buffers: vec![bytes], source_form: GltfSourceForm::Glb, ..GltfSnapshot::default() }
}

#[test]
fn a_snapshot_round_trips_through_the_container_with_its_first_buffer_as_binary_chunk() {
    let original = snapshot(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let bytes = encode(&original).expect("the snapshot encodes");
    assert_eq!(&bytes[0..4], b"glTF");
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize, bytes.len());
    let again = decode(&bytes).expect("the container decodes");
    assert_eq!((again.document.scene, again.document.scenes.len(), again.buffers[0].clone()), (Some(0), 1, vec![1, 2, 3, 4, 5, 6, 7, 8]));
}

#[test]
fn a_snapshot_without_a_buffer_writes_no_binary_chunk_and_chunks_are_padded_to_four_bytes() {
    let bytes = encode(&GltfSnapshot::default()).expect("the empty snapshot encodes");
    assert_eq!(bytes.len() % 4, 0);
    assert!(decode(&bytes).expect("decodes").buffers.is_empty());
    let odd = encode(&snapshot(vec![9; 7])).expect("encodes");
    assert_eq!(odd.len() % 4, 0);
}

#[test]
fn malformed_containers_are_refused() {
    let good = encode(&snapshot(vec![1, 2, 3, 4])).expect("encodes");
    assert!(decode(&good[..10]).is_err());
    let mut wrong_magic = good.clone();
    wrong_magic[0] = b'x';
    assert!(decode(&wrong_magic).is_err());
}
