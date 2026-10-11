//! 🔬️ The binary facet round-trips, refuses a foreign envelope, and releases a displaced snapshot
//! through the bounded retirement rather than an unbounded `Drop`.


#[test]
fn encoding_then_decoding_is_a_fixed_point() {
    let snapshot = crate::examples::blocks::snapshot();
    let bytes = crate::standards::v1::subsets::any::io::binary::snapshot::encode(&snapshot);
    assert!(bytes.len() > 64);
    assert_eq!(crate::standards::v1::subsets::any::io::binary::snapshot::decode(&bytes).expect("pack decodes"), snapshot);
}

#[test]
fn a_foreign_envelope_is_refused_rather_than_misread() {
    assert!(crate::standards::v1::subsets::any::io::binary::snapshot::decode(b"not a semio pack at all").is_err());
}

/// 🎟️ The exact frame capacity one displaced snapshot's retirement is born with.
fn admission() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<crate::schema::snapshot::Grid3dSnapshot>(), maximum_depth: 1, ..Default::default() }
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

#[test]
fn a_displaced_snapshot_is_released_in_bounded_steps() {
    let mut live = crate::examples::blocks::snapshot();
    let bytes = crate::standards::v1::subsets::any::io::binary::snapshot::encode(&crate::examples::pipes_3d::snapshot());
    let (mut retirement, birth) = crate::standards::v1::subsets::any::io::binary::snapshot::decode_into(&mut live, &bytes, admission()).expect("decode in place");
    assert!(birth.fits(admission()));
    assert_eq!(live, crate::examples::pipes_3d::snapshot());
    assert!(retire_all(&mut retirement) >= 1, "the displaced collections are released by funded steps");
}
