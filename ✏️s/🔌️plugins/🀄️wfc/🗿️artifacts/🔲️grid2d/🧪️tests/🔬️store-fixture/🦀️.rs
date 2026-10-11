//! 🧪️ Store-fixture law — every bundled example survives the exact envelope round trip a mounted
//! artifact store performs, and a decode-in-place hands back a bounded retirement that reaches its
//! terminal state before it is dropped.

use crate::io::binary::snapshot::{decode_into, encode};
use crate::schema::snapshot::Grid2dSnapshot;

/// 🎟️ The exact frame capacity one displaced snapshot's retirement is born with.
fn admission() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<Grid2dSnapshot>(), maximum_depth: 1, ..Default::default() }
}

/// ♻️ Drains a retirement with grants quoted from its own next demand and returns the step count.
fn retire_all(retirement: &mut Box<dyn store::ErasedSnapshotRetirement>) -> usize {
    let mut steps = 0;
    while !retirement.terminal_is_empty() {
        let demand = retirement.next_demand(1 << 16).expect("a retained snapshot quotes its next demand");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = retirement.close_step(grant).expect("a funded step makes progress");
        assert!(step.progress().fits(grant));
        steps += 1;
        assert!(steps < 100_000, "the retirement ladder must terminate");
    }
    steps
}

fn examples() -> Vec<Grid2dSnapshot> {
    crate::examples::grid2d::sources().iter().map(|source| <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(&source.document()).expect("example parses")).collect()
}

#[test]
fn every_example_survives_the_pack_envelope() {
    for document in examples() {
        let bytes = encode(&document);
        assert!(bytes.len() > 64);
        assert_eq!(crate::io::binary::snapshot::decode(&bytes).expect("pack decodes"), document);
    }
}

#[test]
fn a_decode_in_place_retires_the_displaced_document_in_bounded_steps() {
    let documents = examples();
    let mut live = documents[0].clone();
    let incoming = encode(&documents[1]);
    let (mut retirement, birth) = decode_into(&mut live, &incoming, admission()).expect("decode in place");
    assert!(birth.fits(admission()));
    assert_eq!(live, documents[1], "the live projection is replaced");
    assert!(retire_all(&mut retirement) >= 1, "a real displacement releases at least one collection");
}

#[test]
fn a_starved_retirement_makes_no_progress_but_never_fails() {
    let documents = examples();
    let mut live = documents[0].clone();
    let (mut retirement, _) = decode_into(&mut live, &encode(&documents[1]), admission()).expect("decode in place");
    let starved = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, ..admission() };
    assert_eq!(retirement.close_step(starved).expect("a starved step is not an error"), semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
    retire_all(&mut retirement);
}
