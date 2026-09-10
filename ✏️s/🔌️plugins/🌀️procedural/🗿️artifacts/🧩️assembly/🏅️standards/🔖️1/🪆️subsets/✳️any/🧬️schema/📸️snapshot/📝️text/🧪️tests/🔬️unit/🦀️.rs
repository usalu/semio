//! 🧪️ Laws of the `.assembly` text codec: the print is parseable, the parse is total over what the
//! print emits, and the one field that goes through a twin (`AssemblyRule::params`) survives.

use super::{parse_dsl, print_dsl, COMPONENT_GRAMMAR_SEMIO};
use crate::schema::snapshot::{AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge, AssemblySnapshot, ASSEMBLY_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

fn populated() -> AssemblySnapshot {
    AssemblySnapshot {
        schema: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        seed: 99,
        slots: vec![
            AssemblySlot { id: "a".into(), x: 1.5, y: -2.0, z: 0.25, pinned_module_id: None },
            AssemblySlot { id: "b".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: Some("wall".into()) },
        ],
        edges: vec![AssemblySlotEdge { id: "ab".into(), from_slot_id: "a".into(), to_slot_id: "b".into() }],
        modules: vec![crate::module_child_handle("wall"), crate::module_child_handle("roof")],
        weights: vec![AssemblyModuleWeight { module_id: "wall".into(), weight: 2.5 }],
        rules: vec![
            AssemblyRule { id: "r0".into(), module_a_id: "wall".into(), module_b_id: "roof".into(), allowed: true, params: SemioValue::default() },
            AssemblyRule {
                id: "r1".into(),
                module_a_id: "roof".into(),
                module_b_id: "roof".into(),
                allowed: false,
                params: SemioValue::Map { entries: vec![SemioValueEntry { key: "reason".into(), value: SemioValue::Str { value: "no roof beside roof".into() } }] },
            },
        ],
    }
}

#[test]
fn the_empty_document_round_trips() {
    let empty = AssemblySnapshot::default();
    assert_eq!(parse_dsl(&print_dsl(&empty)).expect("empty document parses"), empty);
}

#[test]
fn a_populated_document_round_trips() {
    let document = populated();
    assert_eq!(parse_dsl(&print_dsl(&document)).expect("populated document parses"), document);
}

#[test]
fn printing_is_idempotent() {
    let once = print_dsl(&populated());
    let twice = print_dsl(&parse_dsl(&once).expect("first print parses"));
    assert_eq!(once, twice);
}

#[test]
fn structured_rule_params_survive_the_dsl_value_twin() {
    let parsed = parse_dsl(&print_dsl(&populated())).expect("populated document parses");
    assert_eq!(parsed.rules[1].params, populated().rules[1].params);
    assert_eq!(parsed.rules[0].params, SemioValue::default());
}

#[test]
fn the_print_carries_this_facets_own_envelope() {
    let printed = print_dsl(&AssemblySnapshot::default());
    assert!(printed.starts_with("semio procedural.assembly.dsl"), "unexpected preamble: {}", printed.lines().next().unwrap_or_default());
}

#[test]
fn the_grammar_leaf_declares_this_facet() {
    assert!(COMPONENT_GRAMMAR_SEMIO.contains("grammar assembly.snapshot"));
    assert!(COMPONENT_GRAMMAR_SEMIO.contains("extension assembly"));
}

#[test]
fn a_line_the_grammar_never_emits_is_refused() {
    assert!(parse_dsl("semio procedural.assembly.dsl v1\nnot-a-field=1\n").is_err());
}
