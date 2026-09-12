//! 🎯️ Every bundled example against its COMMITTED expected outcome — the problem's shape, whether a
//! consistent assignment exists at all, and the assignment itself.
//!
//! **The outcome is CHECKED, not re-solved, and that is deliberate.** The committed `assignments`
//! map is verified against the spec by a pure consistency checker in this file: every adjacency edge
//! must carry a module pair the rule set allows, every pinned slot must keep its pin, and every id
//! must be declared. Running the live solver here instead is currently impossible, not merely
//! slower: `AssemblySolve`/`AssemblyContradiction` go through `solve_with_job`, and
//! `AssemblyInferenceJob::step` fails the framework's retained-page handback assertion
//! (`🧰️framework/🔨️modules/🧵️job/🦀️.rs:483`, then `:663` in `Drop`) and ABORTS the process —
//! the same abort its own `maximum_admission_is_moved_without_clone_and_previews_on_fixed_cadence`
//! unit test already takes. That defect predates this lane and is recorded in
//! `📓️assembly-mount-2026-09-10.md`; a checker is the honest oracle until it is fixed, and it is a
//! STRICTER statement than "the solver agreed with itself" anyway.

use crate::schema::snapshot::AssemblySnapshot;
use std::collections::BTreeMap;

const OUTCOME_SCHEMA: &str = "s.procedural.assembly.example-outcome/v1";

struct Committed {
    example: &'static str,
    json: &'static str,
    snapshot: fn() -> AssemblySnapshot,
}

fn committed() -> Vec<Committed> {
    vec![
        Committed { example: super::two_room_corridor::ID, json: include_str!("../../🚪️two-room-corridor/🧫️fixtures/🧩️example/🔣️.json"), snapshot: super::two_room_corridor::snapshot },
        Committed { example: super::wall_roof_facade_strip::ID, json: include_str!("../../🧱️wall-roof-facade-strip/🧫️fixtures/🧩️example/🔣️.json"), snapshot: super::wall_roof_facade_strip::snapshot },
    ]
}

fn expected(case: &Committed) -> dsl::DslValue {
    dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(case.json).expect("committed outcome is json"))
}

fn field<'a>(value: &'a dsl::DslValue, key: &str) -> &'a dsl::DslValue {
    value.get(key).unwrap_or_else(|| panic!("committed outcome declares '{key}'"))
}

fn assignments(value: &dsl::DslValue) -> BTreeMap<String, String> {
    field(value, "assignments").as_object().expect("assignments is an object").iter().map(|(slot, module)| (slot.clone(), module.as_str().expect("module id is text").to_string())).collect()
}

/// ⛓️ Does the rule set admit this module pair across one edge? A rule is stated for an UNORDERED
/// module pair (the adjacency graph carries no direction the rules distinguish), and a pair no rule
/// mentions is admitted — the vocabulary is a deny-list over an otherwise free domain, which is what
/// makes `rule-roof-roof` load-bearing in `wall-roof-facade-strip`.
fn pair_is_allowed(snapshot: &AssemblySnapshot, left: &str, right: &str) -> bool {
    snapshot
        .rules
        .iter()
        .find(|rule| (rule.module_a_id == left && rule.module_b_id == right) || (rule.module_a_id == right && rule.module_b_id == left))
        .is_none_or(|rule| rule.allowed)
}

#[test]
fn every_example_matches_its_committed_outcome() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let expected = expected(&case);
        assert_eq!(field(&expected, "schema").as_str(), Some(OUTCOME_SCHEMA), "{}", case.example);
        assert_eq!(field(&expected, "example").as_str(), Some(case.example));
        assert_eq!(field(&expected, "seed").as_u64(), Some(snapshot.seed), "{} seed", case.example);
        assert_eq!(field(&expected, "slots").as_u64(), Some(snapshot.slots.len() as u64), "{} slots", case.example);
        assert_eq!(field(&expected, "edges").as_u64(), Some(snapshot.edges.len() as u64), "{} edges", case.example);
        assert_eq!(field(&expected, "rules").as_u64(), Some(snapshot.rules.len() as u64), "{} rules", case.example);
        assert_eq!(field(&expected, "weights").as_u64(), Some(snapshot.weights.len() as u64), "{} weights", case.example);

        let modules: Vec<String> = snapshot.modules.iter().map(|module| module.child_id.clone()).collect();
        let declared: Vec<String> = field(&expected, "modules").as_array().expect("modules is an array").iter().map(|entry| entry.as_str().expect("module id is text").to_string()).collect();
        assert_eq!(declared, modules, "{} modules", case.example);
    }
}

#[test]
fn every_committed_assignment_is_consistent_with_its_own_spec() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let expected = expected(&case);
        let assignment = assignments(&expected);
        assert_eq!(field(&expected, "satisfiable").as_bool(), Some(!assignment.is_empty()), "{} satisfiability must agree with the committed assignment", case.example);

        let modules: Vec<&str> = snapshot.modules.iter().map(|module| module.child_id.as_str()).collect();
        for slot in &snapshot.slots {
            let assigned = assignment.get(&slot.id).unwrap_or_else(|| panic!("{}: slot {} has no committed assignment", case.example, slot.id));
            assert!(modules.contains(&assigned.as_str()), "{}: slot {} is assigned an undeclared module", case.example, slot.id);
            if let Some(pinned) = &slot.pinned_module_id {
                assert_eq!(assigned, pinned, "{}: slot {} ignores its own pin", case.example, slot.id);
            }
        }
        assert_eq!(assignment.len(), snapshot.slots.len(), "{} assigns exactly its own slots", case.example);

        for edge in &snapshot.edges {
            let left = &assignment[&edge.from_slot_id];
            let right = &assignment[&edge.to_slot_id];
            assert!(pair_is_allowed(&snapshot, left, right), "{}: edge {} places a forbidden pair {left}/{right}", case.example, edge.id);
        }
    }
}

#[test]
fn every_example_rule_and_weight_names_a_declared_module() {
    for case in committed() {
        let snapshot = (case.snapshot)();
        let modules: Vec<&str> = snapshot.modules.iter().map(|module| module.child_id.as_str()).collect();
        for rule in &snapshot.rules {
            assert!(modules.contains(&rule.module_a_id.as_str()), "{}: rule {} names an undeclared module", case.example, rule.id);
            assert!(modules.contains(&rule.module_b_id.as_str()), "{}: rule {} names an undeclared module", case.example, rule.id);
        }
        for weight in &snapshot.weights {
            assert!(modules.contains(&weight.module_id.as_str()), "{}: weight {} names an undeclared module", case.example, weight.module_id);
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
    assert_eq!(ids, vec![super::two_room_corridor::ID.to_string(), super::wall_roof_facade_strip::ID.to_string()]);
}

// [DEBUG] temporary generator — writes each example's printed asset for this lane to commit.
#[test]
#[ignore]
fn debug_emit_example_assets() {
    let out = std::path::Path::new("/private/tmp/claude-501/-Users-ueli-Documents-semio/9f5f6952-6c25-4056-a743-773b3668812e/scratchpad/asm-gen");
    std::fs::create_dir_all(out).expect("scratch dir");
    for case in committed() {
        std::fs::write(out.join(format!("{}.dsl.semio", case.example)), crate::schema::snapshot::text::print_dsl(&(case.snapshot)())).expect("write dsl");
    }
}
