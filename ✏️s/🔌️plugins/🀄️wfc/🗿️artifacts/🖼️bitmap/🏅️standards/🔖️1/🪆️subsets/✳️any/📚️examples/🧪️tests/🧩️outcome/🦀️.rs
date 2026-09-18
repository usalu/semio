//! 🎯️ Every bundled example against the properties the WHOLE roster must hold: a well-formed
//! problem, a real pattern universe, and a solve that terminates with a verdict.

use crate::schema::snapshot::BitmapSnapshot;

fn committed() -> Vec<(&'static str, BitmapSnapshot)> {
    vec![(super::rooms_16::ID, super::rooms_16::snapshot()), (super::flowers_24::ID, super::flowers_24::snapshot())]
}

#[test]
fn the_bundled_roster_is_exactly_the_committed_set() {
    let ids: Vec<String> = super::sources().iter().map(|source| source.id().to_string()).collect();
    assert_eq!(ids, vec![super::rooms_16::ID.to_string(), super::flowers_24::ID.to_string()]);
}

#[test]
fn every_example_is_a_well_formed_problem() {
    for (id, snapshot) in committed() {
        assert_eq!(snapshot.schema, crate::WFC_BITMAP_DOCUMENT_SCHEMA, "{id}");
        let indices = snapshot.input.indices().unwrap_or_else(|| panic!("{id}: the sample decodes"));
        assert_eq!(indices.len(), (snapshot.input.width as usize) * (snapshot.input.height as usize), "{id}");
        assert!(indices.iter().all(|index| usize::from(*index) < snapshot.input.palette.len()), "{id}: every pixel names a real palette entry");
        assert!(snapshot.model.pattern_size >= 2 && snapshot.model.pattern_size <= 5, "{id}");
        assert!(snapshot.model.symmetry >= 1 && snapshot.model.symmetry <= 8, "{id}");
        if let Some(ground) = snapshot.model.ground {
            assert!((ground as usize) < snapshot.input.palette.len(), "{id}: the ground colour is in the palette");
        }
        for pin in &snapshot.pinned {
            assert!(pin.x < snapshot.output.width && pin.y < snapshot.output.height, "{id}: a pin sits inside the output");
            assert!((pin.color as usize) < snapshot.input.palette.len(), "{id}: a pin names a real palette entry");
        }
    }
}

#[test]
fn every_example_has_a_real_pattern_universe() {
    for (id, snapshot) in committed() {
        assert!(crate::inferences::pattern_universe_entropy(&snapshot) > 0.0, "{id}: a sample with one pattern teaches nothing");
    }
}

#[test]
fn every_example_solve_terminates_with_a_verdict() {
    for (id, snapshot) in committed() {
        let commit = crate::inferences::solve_with_job(&snapshot).unwrap_or_else(|error| panic!("{id}: the job completes: {error}"));
        let cells = (snapshot.output.width as usize) * (snapshot.output.height as usize);
        assert_eq!(commit.entropy.len(), cells, "{id}: one entropy value per output cell");
        if commit.contradiction {
            assert!(commit.pixels.is_empty(), "{id}: a contradiction carries no pixels");
        } else {
            let pixels = crate::schema::snapshot::decode_base64(&commit.pixels).unwrap_or_else(|| panic!("{id}: the output decodes"));
            assert_eq!(pixels.len(), cells, "{id}");
            assert!(pixels.iter().all(|index| usize::from(*index) < snapshot.input.palette.len()), "{id}: every solved pixel names a real palette entry");
            for pin in &snapshot.pinned {
                assert_eq!(u32::from(pixels[(pin.y * snapshot.output.width + pin.x) as usize]), pin.color, "{id}: the solve honoured its pin");
            }
        }
    }
}
