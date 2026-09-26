//! 🔬️ Mutation-vocabulary laws — the kebab roster matches the enum, every builder round-trips through
//! its own op text and binary forms, and a collection insert lands at the canonical sorted position.

use super::*;
use crate::schema::snapshot::{Grid3dCell, Grid3dDirection, Grid3dPinnedCell, Grid3dRule, Grid3dTile};
use protocol::{OpBinary, OpText};

fn every_mutation() -> Vec<Grid3dMutation> {
    vec![
        change_seed(99),
        resize_grid(2, 2, 2),
        change_cell_sizes(crate::schema::snapshot::Grid3dAxis::X, vec![1.0, 2.0]),
        change_periodicity(true, false, true),
        create_tile(Grid3dTile { id: "zzz".into(), label: Some("Z".into()), weight: 2.0, media: Default::default() }),
        delete_tile("wall".into()),
        change_tile_weight("wall".into(), 5.0),
        change_tile_media("wall".into(), Default::default()),
        create_rule(Grid3dRule { id: "r-zzz".into(), tile_a_id: "air".into(), tile_b_id: "wall".into(), direction: Grid3dDirection::Top, allowed: true }),
        delete_rule("r-top-air-air".into()),
        pin_cell(Grid3dPinnedCell { x: 1, y: 1, z: 1, tile_id: "wall".into() }),
        unpin_cell(0, 0, 0),
        mask_cell(Grid3dCell { x: 2, y: 2, z: 2 }),
        unmask_cell(3, 2, 3),
    ]
}

#[test]
fn the_kebab_roster_matches_the_enum_in_declaration_order() {
    assert_eq!(KINDS.len(), every_mutation().len());
    for (kind, mutation) in KINDS.iter().zip(every_mutation()) {
        assert_eq!(*kind, protocol::SemanticMutation::<Grid3dSnapshot>::semantics(&mutation).kind, "roster order must match the enum");
    }
}

#[test]
fn every_mutation_round_trips_through_its_op_text_form() {
    for mutation in every_mutation() {
        let line = OpText::print_op(&mutation);
        assert!(!line.trim().is_empty());
        assert_eq!(<Grid3dMutation as OpText>::parse_op(&line).expect("op line parses"), mutation, "round trip failed for {line}");
    }
}

#[test]
fn every_mutation_round_trips_through_its_binary_form() {
    for mutation in every_mutation() {
        let bytes = OpBinary::encode_op(&mutation).expect("op encodes");
        assert_eq!(<Grid3dMutation as OpBinary>::decode_op(&bytes).expect("op decodes"), mutation);
    }
}

#[test]
fn an_ordered_index_is_the_existing_slot_or_the_sorted_insertion_point() {
    let items = ["b".to_string(), "d".to_string()];
    let key = |item: &String| item.clone();
    assert_eq!(ordered_index(&items, "a", key), 0);
    assert_eq!(ordered_index(&items, "b", key), 0);
    assert_eq!(ordered_index(&items, "c", key), 1);
    assert_eq!(ordered_index(&items, "e", key), 2);
}

#[test]
fn every_mutation_and_its_inverse_return_the_document_to_where_it_started() {
    let base = crate::examples::blocks::snapshot();
    for mutation in every_mutation() {
        let mut snapshot = base.clone();
        let inverse = inverse_grid3d_mutation(&base, &mutation);
        if apply_grid3d_mutation(&mut snapshot, &mutation).is_err() {
            continue;
        }
        for step in &inverse {
            apply_grid3d_mutation(&mut snapshot, step).expect("inverse step applies");
        }
        assert_eq!(snapshot, base, "inverse did not restore the document for {:?}", protocol::SemanticMutation::<Grid3dSnapshot>::semantics(&mutation).kind);
    }
}

//#region 🏷️Roster
/// 🏷️ The mutation VOCABULARY the enum declares, the kebab roster `KINDS` publishes, the per-kind manifests on disk,
/// the payload schemas, the oracle catalog, the TypeScript twin and the `🧩️mutate-wfc-grid3d-1` case's feature table
/// and Python oracle are one and the same set, in one order. The committed quintets themselves are replayed per kind by
/// each mutation's own fixture test and by that case.
mod roster {
    use super::KINDS;

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
        include_str!("../../🎲️change-seed/🔣️.json"),
        include_str!("../../📐️resize-grid/🔣️.json"),
        include_str!("../../📏️change-cell-sizes/🔣️.json"),
        include_str!("../../🔁️change-periodicity/🔣️.json"),
        include_str!("../../🧱️create-tile/🔣️.json"),
        include_str!("../../🕳️delete-tile/🔣️.json"),
        include_str!("../../⚖️change-tile-weight/🔣️.json"),
        include_str!("../../🖼️change-tile-media/🔣️.json"),
        include_str!("../../🚦️create-rule/🔣️.json"),
        include_str!("../../❌️delete-rule/🔣️.json"),
        include_str!("../../📌️pin-cell/🔣️.json"),
        include_str!("../../📍️unpin-cell/🔣️.json"),
        include_str!("../../🚫️mask-cell/🔣️.json"),
        include_str!("../../🔓️unmask-cell/🔣️.json"),
    ];

    const PAYLOAD_SCHEMAS: [&str; 14] = [
        include_str!("../../🎲️change-seed/🧬️schema/🔣️.json"),
        include_str!("../../📐️resize-grid/🧬️schema/🔣️.json"),
        include_str!("../../📏️change-cell-sizes/🧬️schema/🔣️.json"),
        include_str!("../../🔁️change-periodicity/🧬️schema/🔣️.json"),
        include_str!("../../🧱️create-tile/🧬️schema/🔣️.json"),
        include_str!("../../🕳️delete-tile/🧬️schema/🔣️.json"),
        include_str!("../../⚖️change-tile-weight/🧬️schema/🔣️.json"),
        include_str!("../../🖼️change-tile-media/🧬️schema/🔣️.json"),
        include_str!("../../🚦️create-rule/🧬️schema/🔣️.json"),
        include_str!("../../❌️delete-rule/🧬️schema/🔣️.json"),
        include_str!("../../📌️pin-cell/🧬️schema/🔣️.json"),
        include_str!("../../📍️unpin-cell/🧬️schema/🔣️.json"),
        include_str!("../../🚫️mask-cell/🧬️schema/🔣️.json"),
        include_str!("../../🔓️unmask-cell/🧬️schema/🔣️.json"),
    ];

    const ORACLE_CATALOG: &str = include_str!("../../../../🔮️oracles/🔣️.json");
    const TYPESCRIPT_TWIN: &str = include_str!("../../🟦️.ts");
    const FEATURE: &str = include_str!("../../../../🧪️tests/🧩️mutate-wfc-grid3d-1/🥒️.feature");
    const PYTHON_ORACLE: &str = include_str!("../../../../🧪️tests/🧩️mutate-wfc-grid3d-1/🐍️.py");

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
}
//#endregion 🏷️Roster
