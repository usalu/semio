//! 🧪️ Store-fixture law — every bundled example survives the exact envelope round trip a mounted
//! artifact store performs, and a decode-in-place hands back a bounded retirement that reaches its
//! terminal state before it is dropped.

use crate::schema::snapshot::binary::{decode_into, encode};
use crate::schema::snapshot::Grid2dSnapshot;

fn examples() -> Vec<Grid2dSnapshot> {
    crate::examples::grid2d::sources().iter().map(|source| <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(source.document()).expect("example parses")).collect()
}

#[test]
fn every_example_survives_the_pack_envelope() {
    for document in examples() {
        let bytes = encode(&document);
        assert!(bytes.len() > 64);
        assert_eq!(crate::schema::snapshot::binary::decode(&bytes).expect("pack decodes"), document);
    }
}

#[test]
fn a_decode_in_place_retires_the_displaced_document_in_bounded_steps() {
    let documents = examples();
    let mut live = documents[0].clone();
    let incoming = encode(&documents[1]);
    let mut retirement = decode_into(&mut live, &incoming).expect("decode in place");
    assert_eq!(live, documents[1], "the live projection is replaced");
    let mut steps = 0;
    while !retirement.terminal_is_empty() {
        retirement.close_step(8, 1 << 16).expect("close step");
        steps += 1;
        assert!(steps < 64, "the retirement ladder must terminate");
    }
    assert!(steps >= 1, "a real displacement releases at least one collection");
}

#[test]
fn a_starved_retirement_makes_no_progress_but_never_fails() {
    let documents = examples();
    let mut live = documents[0].clone();
    let mut retirement = decode_into(&mut live, &encode(&documents[1])).expect("decode in place");
    let step = retirement.close_step(0, 0).expect("a starved step is not an error");
    assert!(matches!(step, store::SnapshotRetirementStep::Pending { released_items: 0, .. }));
    while !retirement.terminal_is_empty() {
        retirement.close_step(8, 1 << 16).expect("close step");
    }
}
