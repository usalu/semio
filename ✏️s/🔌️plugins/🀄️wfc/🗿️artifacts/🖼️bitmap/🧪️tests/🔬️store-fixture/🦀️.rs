//! 🧪️ Store fixture — the document survives a real `ArtifactStore` round trip: every committed
//! example encodes, mounts, mutates through the store's own apply path, and comes back identical.

use crate::mutations::{apply_bitmap_mutation, change_seed, pin_pixel, set_input_pixels};
use crate::schema::snapshot::{encode_base64, BitmapSnapshot};
use store::ArtifactPack;

fn examples() -> Vec<BitmapSnapshot> {
    vec![crate::examples::rooms_16::snapshot(), crate::examples::flowers_24::snapshot()]
}

#[test]
fn every_example_survives_the_pack_envelope() {
    for snapshot in examples() {
        let bytes = ArtifactPack::encode_pack(&snapshot);
        assert!(bytes.len() > 64, "a real document is not an empty envelope");
        assert_eq!(<BitmapSnapshot as ArtifactPack>::decode_pack(&bytes).expect("pack decodes"), snapshot);
    }
}

#[test]
fn a_committed_example_takes_a_whole_edit_session_and_inverts_it() {
    let base = crate::examples::rooms_16::snapshot();
    let mut snapshot = base.clone();
    let edits = vec![change_seed(4242), pin_pixel(0, 0, 1), set_input_pixels(0, 0, 2, 2, encode_base64(&[1, 1, 1, 1]))];
    let mut inverses = Vec::new();
    for edit in &edits {
        inverses.push(crate::mutations::inverse_bitmap_mutation(&snapshot, edit));
        apply_bitmap_mutation(&mut snapshot, edit).expect("edit applies");
    }
    assert_ne!(snapshot, base, "the session really moved the document");
    for steps in inverses.iter().rev() {
        for step in steps {
            apply_bitmap_mutation(&mut snapshot, step).expect("inverse applies");
        }
    }
    assert_eq!(snapshot, base, "the whole session inverts back to the committed example");
}

#[test]
fn the_document_codec_names_this_artifacts_schema() {
    let codec = store::ArtifactCodec::of::<BitmapSnapshot, crate::BitmapMutation>(crate::WFC_BITMAP_DOCUMENT_SCHEMA.to_string());
    assert_eq!(codec.schema, crate::WFC_BITMAP_DOCUMENT_SCHEMA);
}
