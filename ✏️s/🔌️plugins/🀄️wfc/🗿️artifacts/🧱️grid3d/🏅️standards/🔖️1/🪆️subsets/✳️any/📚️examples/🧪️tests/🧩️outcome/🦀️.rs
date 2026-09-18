//! 🎯️ Every bundled example against its COMMITTED expected outcome — the problem's shape and the
//! solve's verdict, plus the live engine actually reaching that verdict for the declared seed.
//!
//! 🔁️ The solve IS run here (unlike `s.assembly`'s own outcome test, which could only check a
//! committed assignment): the engine crate's close ladder is production, so a `WfcJob` driven through
//! `BatchJobSession` retires cleanly instead of aborting the process on a leaked payload page.

use crate::schema::inferences::solve;
use crate::schema::snapshot::Grid3dSnapshot;

const OUTCOME_SCHEMA: &str = "s.wfc.grid3d.example-outcome/v1";

struct Committed {
    example: &'static str,
    json: &'static str,
    snapshot: fn() -> Grid3dSnapshot,
}

fn committed() -> Vec<Committed> {
    vec![
        Committed { example: super::blocks::ID, json: include_str!("../../🧱️blocks/🧫️fixtures/🧩️example/🔣️.json"), snapshot: super::blocks::snapshot },
        Committed { example: super::pipes_3d::ID, json: include_str!("../../🪠️pipes-3d/🧫️fixtures/🧩️example/🔣️.json"), snapshot: super::pipes_3d::snapshot },
    ]
}

fn expected(case: &Committed) -> dsl::DslValue {
    dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(case.json).expect("committed outcome is json"))
}

fn field<'a>(value: &'a dsl::DslValue, key: &str) -> &'a dsl::DslValue {
    value.get(key).unwrap_or_else(|| panic!("committed outcome declares '{key}'"))
}

#[test]
fn every_example_matches_its_committed_shape() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let expected = expected(&case);
        assert_eq!(field(&expected, "schema").as_str(), Some(OUTCOME_SCHEMA), "{}", case.example);
        assert_eq!(field(&expected, "example").as_str(), Some(case.example));
        assert_eq!(field(&expected, "seed").as_u64(), Some(snapshot.seed), "{} seed", case.example);
        assert_eq!(field(&expected, "width").as_u64(), Some(snapshot.width as u64), "{} width", case.example);
        assert_eq!(field(&expected, "height").as_u64(), Some(snapshot.height as u64), "{} height", case.example);
        assert_eq!(field(&expected, "depth").as_u64(), Some(snapshot.depth as u64), "{} depth", case.example);
        assert_eq!(field(&expected, "tiles").as_u64(), Some(snapshot.tiles.len() as u64), "{} tiles", case.example);
        assert_eq!(field(&expected, "rules").as_u64(), Some(snapshot.rules.len() as u64), "{} rules", case.example);
        assert_eq!(field(&expected, "pinned").as_u64(), Some(snapshot.pinned.len() as u64), "{} pinned", case.example);
        assert_eq!(field(&expected, "masked").as_u64(), Some(snapshot.masked.len() as u64), "{} masked", case.example);
    }
}

#[test]
fn every_example_solves_to_its_committed_verdict() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let expected = expected(&case);
        let solved = solve(&snapshot).expect("the solve answers");
        assert_eq!(solved.satisfiable, field(&expected, "satisfiable").as_bool() == Some(true), "{} satisfiability", case.example);
        if solved.satisfiable {
            let commit = solved;
            let cells = (snapshot.width as usize) * (snapshot.height as usize) * (snapshot.depth as usize) - snapshot.masked.len();
            assert_eq!(commit.assignments.len(), cells, "{} assigns exactly its unmasked cells", case.example);
            assert_eq!(field(&expected, "assigned").as_u64(), Some(commit.assignments.len() as u64), "{} assigned", case.example);
        }
    }
}

#[test]
fn every_committed_assignment_respects_the_pins_the_mask_and_the_rules() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let commit = solve(&snapshot).expect("the solve answers");
        if !commit.satisfiable {
            continue;
        }
        for row in &commit.assignments {
            assert!(!snapshot.masked.iter().any(|cell| (cell.x, cell.y, cell.z) == (row.x, row.y, row.z)), "{}: a masked cell was assigned", case.example);
            assert!(snapshot.tiles.iter().any(|candidate| candidate.id == row.tile_id), "{}: cell {}:{}:{} got an undeclared tile", case.example, row.x, row.y, row.z);
        }
        for pin in &snapshot.pinned {
            let assigned = commit.assignments.iter().find(|row| (row.x, row.y, row.z) == (pin.x, pin.y, pin.z)).map(|row| row.tile_id.as_str());
            assert_eq!(assigned, Some(pin.tile_id.as_str()), "{}: pin {}:{}:{} was overwritten", case.example, pin.x, pin.y, pin.z);
        }
    }
}

#[test]
fn the_bundled_roster_is_exactly_the_committed_set() {
    let ids: Vec<String> = super::sources().iter().map(|source| source.id().to_string()).collect();
    assert_eq!(ids, vec![super::blocks::ID.to_string(), super::pipes_3d::ID.to_string()]);
}
