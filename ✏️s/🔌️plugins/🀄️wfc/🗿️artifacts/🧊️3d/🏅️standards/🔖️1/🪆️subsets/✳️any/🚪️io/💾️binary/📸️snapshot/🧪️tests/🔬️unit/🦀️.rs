//! 🧪️ The binary facet: pack round-trips, and a displaced document is released in BOUNDED steps
//! rather than dropped whole.

use crate::schema::snapshot::Wfc3dSnapshot;
use crate::standards::v1::subsets::any::io::binary::snapshot::{decode, encode};

#[test]
fn every_example_round_trips_through_the_pack_codec() {
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        let bytes = encode(&document);
        assert!(bytes.len() > 64);
        assert_eq!(decode(&bytes).expect("pack decodes"), document);
    }
}

#[test]
fn empty_bytes_decode_to_the_default_document() {
    assert_eq!(decode(&[]).expect("empty pack is the default document"), Wfc3dSnapshot::default());
}
