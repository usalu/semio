//! 🧪️ Store fixture — the document really mounts: an envelope over a real example, edited through
//! the mutation machinery and closed the way a host closes it.

use crate::mutations::{apply_wfc2d_mutation, change_seed, pin_slot, Wfc2dMutation};
use crate::Wfc2dSnapshot;

/// 🏪️ A snapshot survives the native pack envelope AND the DSL envelope, in both directions.
#[test]
fn a_mounted_document_round_trips_through_both_envelopes() {
    let document = crate::examples::wall_roof_facade_strip::document();
    let text = store::ArtifactDsl::print_dsl(&document);
    let parsed = <Wfc2dSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("the printed document parses");
    assert_eq!(parsed, document);
    let bytes = <Wfc2dSnapshot as store::ArtifactPack>::encode_pack(&document);
    let decoded = <Wfc2dSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("the packed document decodes");
    assert_eq!(decoded, document);
}

/// ♻️ A decode-in-place hands back a BOUNDED retirement, and the retirement reaches terminal-empty
/// before it is dropped — dropping one early is a hard assertion, not a leak.
#[test]
fn decode_in_place_retires_the_displaced_document() {
    let mut live = crate::examples::two_room_corridor::document();
    let bytes = <Wfc2dSnapshot as store::ArtifactPack>::encode_pack(&crate::examples::hex_ring::document());
    let mut retirement = crate::standards::v1::subsets::any::io::snapshot::binary::decode_into(&mut live, &bytes).expect("decode in place");
    assert_eq!(live, crate::examples::hex_ring::document());
    while !retirement.terminal_is_empty() {
        retirement.close_step(8, 1 << 16).expect("retirement steps");
    }
}

/// 🧬️ A short edit ladder applies and inverts against a mounted projection.
#[test]
fn an_edit_ladder_applies_and_inverts() {
    let base = crate::examples::two_room_corridor::document();
    let ladder: Vec<Wfc2dMutation> = vec![change_seed(4242), pin_slot("room-a".into(), "room".into())];
    let mut projection = base.clone();
    let mut inverses: Vec<Vec<Wfc2dMutation>> = Vec::new();
    for mutation in &ladder {
        inverses.push(crate::mutations::inverse_wfc2d_mutation(&projection, mutation));
        apply_wfc2d_mutation(&mut projection, mutation).expect("ladder step applies");
    }
    assert_ne!(projection, base);
    for steps in inverses.iter().rev() {
        for step in steps {
            apply_wfc2d_mutation(&mut projection, step).expect("inverse step applies");
        }
    }
    assert_eq!(projection, base, "the edit ladder did not invert back to the mounted document");
}
