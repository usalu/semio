//! 🎯️ Every bundled example against its COMMITTED expected outcome — the problem's shape, whether a
//! consistent assignment exists at all, and the assignment itself.
//!
//! The committed `assignments` map is checked TWICE: against the spec by a pure consistency checker
//! in this file (every adjacency edge carries a tile pair the rule set admits, every pinned slot
//! keeps its pin, every id is declared), and against the LIVE solve, which is deterministic for a
//! seed. The first statement is the stronger one — it does not care which of several valid
//! assignments the solver picked — and the second is what catches engine drift.

use crate::inferences::solve_assignments;
use crate::Wfc3dSnapshot;
use std::collections::BTreeMap;

const OUTCOME_SCHEMA: &str = "s.wfc.wfc3d.example-outcome/v1";

struct Committed {
    example: &'static str,
    json: &'static str,
    snapshot: fn() -> Wfc3dSnapshot,
}

fn committed() -> Vec<Committed> {
    vec![
        Committed { example: super::two_room_corridor::ID, json: include_str!("../../🚪️two-room-corridor/🧫️fixtures/🧩️example/🔣️.json"), snapshot: super::two_room_corridor::snapshot },
        Committed { example: super::wall_roof_facade_strip::ID, json: include_str!("../../🧱️wall-roof-facade-strip/🧫️fixtures/🧩️example/🔣️.json"), snapshot: super::wall_roof_facade_strip::snapshot },
        Committed { example: super::tower_stack::ID, json: include_str!("../../🗼️tower-stack/🧫️fixtures/🧩️example/🔣️.json"), snapshot: super::tower_stack::snapshot },
    ]
}

fn expected(case: &Committed) -> serde_json::Value {
    serde_json::from_str(case.json).expect("committed outcome is json")
}

fn assignments(value: &serde_json::Value) -> BTreeMap<String, String> {
    value["assignments"].as_object().expect("assignments is an object").iter().map(|(slot, tile)| (slot.clone(), tile.as_str().expect("tile id is text").to_string())).collect()
}

/// ⛓️ Does the rule set admit this tile pair across an edge of this relation? The rules ARE the
/// compatibility table — an ALLOW-LIST — so a pair no rule mentions is forbidden. A rule is stated
/// for an UNORDERED pair, `relation: None` applies to every relation, and a forbidding rule always
/// beats an admitting one.
fn pair_is_admitted(snapshot: &Wfc3dSnapshot, relation: &str, left: &str, right: &str) -> bool {
    let applies = |rule: &&crate::schema::snapshot::GraphRule| {
        let names_pair = (rule.tile_a_id == left && rule.tile_b_id == right) || (rule.tile_a_id == right && rule.tile_b_id == left);
        names_pair && rule.relation.as_deref().is_none_or(|scope| scope == relation)
    };
    snapshot.rules.iter().filter(applies).any(|rule| rule.allowed) && !snapshot.rules.iter().filter(applies).any(|rule| !rule.allowed)
}

#[test]
fn every_example_matches_its_committed_shape() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let expected = expected(&case);
        assert_eq!(expected["schema"].as_str(), Some(OUTCOME_SCHEMA), "{}", case.example);
        assert_eq!(expected["example"].as_str(), Some(case.example));
        assert_eq!(expected["seed"].as_u64(), Some(snapshot.seed), "{} seed", case.example);
        assert_eq!(expected["slots"].as_u64(), Some(snapshot.slots.len() as u64), "{} slots", case.example);
        assert_eq!(expected["edges"].as_u64(), Some(snapshot.edges.len() as u64), "{} edges", case.example);
        assert_eq!(expected["rules"].as_u64(), Some(snapshot.rules.len() as u64), "{} rules", case.example);
        let tiles: Vec<String> = snapshot.tiles.iter().map(|tile| tile.id.clone()).collect();
        let declared: Vec<String> = expected["tiles"].as_array().expect("tiles").iter().map(|entry| entry.as_str().expect("tile id").to_string()).collect();
        assert_eq!(declared, tiles, "{} tiles", case.example);
    }
}

#[test]
fn every_committed_assignment_is_consistent_with_its_own_spec() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let expected = expected(&case);
        let assignment = assignments(&expected);
        assert_eq!(expected["satisfiable"].as_bool(), Some(!assignment.is_empty()), "{}: satisfiability must agree with the committed assignment", case.example);

        let tiles: Vec<&str> = snapshot.tiles.iter().map(|tile| tile.id.as_str()).collect();
        for slot in &snapshot.slots {
            let assigned = assignment.get(&slot.id).unwrap_or_else(|| panic!("{}: slot {} has no committed assignment", case.example, slot.id));
            assert!(tiles.contains(&assigned.as_str()), "{}: slot {} is assigned an undeclared tile", case.example, slot.id);
            if let Some(pinned) = &slot.pinned_tile_id {
                assert_eq!(assigned, pinned, "{}: slot {} ignores its own pin", case.example, slot.id);
            }
        }
        assert_eq!(assignment.len(), snapshot.slots.len(), "{} assigns exactly its own slots", case.example);

        for edge in &snapshot.edges {
            let left = &assignment[&edge.from_slot_id];
            let right = &assignment[&edge.to_slot_id];
            assert!(pair_is_admitted(&snapshot, &edge.relation, left, right), "{}: edge {} places a pair no rule admits, {left}/{right}", case.example, edge.id);
        }
    }
}

/// 🔁 The live solve is a pure function of the document, so the committed assignment IS what the
/// engine produces today. A drift here is an engine change, not a fixture problem.
#[test]
fn the_live_solve_reproduces_every_committed_assignment() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let solved = solve_assignments(&snapshot);
        assert_eq!(solved, assignments(&expected(&case)), "{}: the live solve drifted from its committed outcome", case.example);
    }
}

#[test]
fn every_example_rule_and_tile_reference_is_declared() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let tiles: Vec<&str> = snapshot.tiles.iter().map(|tile| tile.id.as_str()).collect();
        for rule in &snapshot.rules {
            assert!(tiles.contains(&rule.tile_a_id.as_str()), "{}: rule {} names an undeclared tile", case.example, rule.id);
            assert!(tiles.contains(&rule.tile_b_id.as_str()), "{}: rule {} names an undeclared tile", case.example, rule.id);
        }
        for slot in &snapshot.slots {
            if let Some(pinned) = &slot.pinned_tile_id {
                assert!(tiles.contains(&pinned.as_str()), "{}: slot {} pins an undeclared tile", case.example, slot.id);
            }
        }
    }
}

#[test]
fn every_example_edge_names_a_declared_slot() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let slots: Vec<&str> = snapshot.slots.iter().map(|slot| slot.id.as_str()).collect();
        for edge in &snapshot.edges {
            assert!(slots.contains(&edge.from_slot_id.as_str()), "{}: edge {} leaves an undeclared slot", case.example, edge.id);
            assert!(slots.contains(&edge.to_slot_id.as_str()), "{}: edge {} enters an undeclared slot", case.example, edge.id);
        }
    }
}

#[test]
fn the_bundled_roster_is_exactly_the_committed_set() {
    let ids: Vec<String> = super::sources().iter().map(|source| source.id().to_string()).collect();
    assert_eq!(ids, vec![super::two_room_corridor::ID.to_string(), super::wall_roof_facade_strip::ID.to_string(), super::tower_stack::ID.to_string()]);
}

/// 🏭️ Writes each example's printed `🗣️.dsl.semio` asset and its committed outcome from the Rust
/// builder — the builder is the authority, the asset is its print. Ignored by default: it authors
/// files, it does not assert.
#[test]
#[ignore]
fn regenerate_committed_example_assets() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples");
    let slugs = [("🚪️two-room-corridor", super::two_room_corridor::ID), ("🧱️wall-roof-facade-strip", super::wall_roof_facade_strip::ID), ("🗼️tower-stack", super::tower_stack::ID)];
    for (case, (slug, id)) in committed().into_iter().zip(slugs) {
        let snapshot = (case.snapshot)();
        std::fs::write(root.join(slug).join("🖼️assets").join(slug).join("🗣️.dsl.semio"), crate::schema::snapshot::text::print_dsl(&snapshot)).expect("write dsl asset");
        let solved = solve_assignments(&snapshot);
        let outcome = serde_json::json!({
            "schema": OUTCOME_SCHEMA,
            "example": id,
            "seed": snapshot.seed,
            "slots": snapshot.slots.len(),
            "edges": snapshot.edges.len(),
            "tiles": snapshot.tiles.iter().map(|tile| tile.id.clone()).collect::<Vec<_>>(),
            "rules": snapshot.rules.len(),
            "satisfiable": !solved.is_empty(),
            "assignments": solved,
        });
        std::fs::write(root.join(slug).join("🧫️fixtures/🧩️example/🔣️.json"), serde_json::to_string_pretty(&outcome).expect("outcome json")).expect("write outcome");
    }
}
