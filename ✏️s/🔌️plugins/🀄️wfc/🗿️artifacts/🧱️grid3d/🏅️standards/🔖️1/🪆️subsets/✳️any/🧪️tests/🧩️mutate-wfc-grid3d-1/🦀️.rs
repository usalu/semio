//! 🥒️ `mutate-wfc-grid3d-1` — the subset-level oracle-replay case. It is the exhaustive statement
//! that the mutation VOCABULARY the Rust enum declares, the kebab roster `KINDS` publishes, the
//! per-kind manifests on disk carry and the cross-language twins mirror are one and the same set.
//!
//! 🔣️ The committed fixture quintets themselves are replayed per kind by each mutation's own
//! `🧪️tests/<case>` module; this file is what keeps a kind from being added in one place and
//! forgotten in the other four.

use crate::schema::mutations::KINDS;

/// 🏷️ Every kind's directory name under `🧬️schema/🧬️mutations/`, in `KINDS` order.
const DIRECTORIES: [&str; 14] = [
    "🎲️change-seed",
    "📐️resize-grid",
    "📏️change-cell-sizes",
    "🔁️change-periodicity",
    "🧱️create-tile",
    "🕳️delete-tile",
    "⚖️change-tile-weight",
    "🖼️change-tile-media",
    "🚦️create-rule",
    "❌️delete-rule",
    "📌️pin-cell",
    "📍️unpin-cell",
    "🚫️mask-cell",
    "🔓️unmask-cell",
];

/// 🔣️ Each kind's committed manifest, mounted by `include_str!` so a renamed directory breaks the
/// build instead of silently dropping a row from the roster.
const MANIFESTS: [&str; 14] = [
    include_str!("../../🧬️schema/🧬️mutations/🎲️change-seed/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/📐️resize-grid/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/📏️change-cell-sizes/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🔁️change-periodicity/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🧱️create-tile/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🕳️delete-tile/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/⚖️change-tile-weight/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🖼️change-tile-media/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🚦️create-rule/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/❌️delete-rule/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/📌️pin-cell/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/📍️unpin-cell/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🚫️mask-cell/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🔓️unmask-cell/🔣️.json"),
];

const PAYLOAD_SCHEMAS: [&str; 14] = [
    include_str!("../../🧬️schema/🧬️mutations/🎲️change-seed/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/📐️resize-grid/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/📏️change-cell-sizes/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🔁️change-periodicity/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🧱️create-tile/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🕳️delete-tile/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/⚖️change-tile-weight/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🖼️change-tile-media/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🚦️create-rule/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/❌️delete-rule/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/📌️pin-cell/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/📍️unpin-cell/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🚫️mask-cell/🧬️schema/🔣️.json"),
    include_str!("../../🧬️schema/🧬️mutations/🔓️unmask-cell/🧬️schema/🔣️.json"),
];

const ORACLE_CATALOG: &str = include_str!("../../🔮️oracles/🔣️.json");
const TYPESCRIPT_TWIN: &str = include_str!("../../🧬️schema/🧬️mutations/🟦️.ts");
const FEATURE: &str = include_str!("🥒️.feature");
const PYTHON_ORACLE: &str = include_str!("🐍️.py");

fn value(text: &str) -> dsl::DslValue {
    dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(text).expect("committed json"))
}

fn field(value: &dsl::DslValue, key: &str) -> String {
    value.get(key).and_then(dsl::DslValue::as_str).unwrap_or_default().to_string()
}

#[test]
fn every_manifest_declares_the_kind_its_own_slot_in_the_roster_names() {
    assert_eq!(KINDS.len(), MANIFESTS.len());
    for (index, text) in MANIFESTS.iter().enumerate() {
        let manifest = value(text);
        assert_eq!(field(&manifest, "semanticKind"), KINDS[index], "manifest {} names the wrong kind", DIRECTORIES[index]);
        assert!(field(&manifest, "owner").ends_with(DIRECTORIES[index]), "manifest {} names the wrong owner directory", DIRECTORIES[index]);
        assert_eq!(field(&manifest, "payloadSchema"), "🧬️schema/🔣️.json");
        assert_eq!(field(&manifest, "invertibility"), "explicit-mutation");
        assert_eq!(field(&manifest, "composition"), "atomic");
    }
}

#[test]
fn every_payload_schema_is_a_self_contained_draft_07_document() {
    for (index, text) in PAYLOAD_SCHEMAS.iter().enumerate() {
        let schema = value(text);
        assert_eq!(field(&schema, "$schema"), "http://json-schema.org/draft-07/schema#", "{}", DIRECTORIES[index]);
        assert!(!text.contains("$defs"), "{} must stay self-contained", DIRECTORIES[index]);
        assert!(field(&schema, "$id").contains(KINDS[index]), "{} names the wrong id", DIRECTORIES[index]);
    }
}

#[test]
fn the_oracle_catalog_carries_a_vector_for_every_kind() {
    let catalog = value(ORACLE_CATALOG);
    let catalogs = catalog.get("mutationCatalogs").and_then(dsl::DslValue::as_array).expect("the manifest declares mutationCatalogs");
    let vectors = catalogs[0].get("vectors").and_then(dsl::DslValue::as_array).expect("the catalog declares vectors");
    assert_eq!(vectors.len(), KINDS.len());
    for (index, vector) in vectors.iter().enumerate() {
        assert_eq!(field(vector, "mutationId"), KINDS[index]);
        assert_eq!(field(vector, "mutationDirectoryName"), DIRECTORIES[index]);
    }
}

#[test]
fn the_typescript_twin_publishes_the_same_roster_in_the_same_order() {
    let start = TYPESCRIPT_TWIN.find("GRID3D_MUTATION_KINDS = [").expect("the twin publishes its roster");
    let end = TYPESCRIPT_TWIN[start..].find(']').expect("the roster is a literal array") + start;
    let declared: Vec<&str> = TYPESCRIPT_TWIN[start..end].lines().filter_map(|line| line.trim().strip_prefix('"')).filter_map(|line| line.split('"').next()).collect();
    assert_eq!(declared, KINDS.to_vec());
}

#[test]
fn the_feature_table_and_the_python_oracle_cover_every_kind() {
    for kind in KINDS {
        assert!(FEATURE.contains(kind), "the feature table is missing {kind}");
        assert!(PYTHON_ORACLE.contains(kind), "the python oracle is missing {kind}");
    }
}
