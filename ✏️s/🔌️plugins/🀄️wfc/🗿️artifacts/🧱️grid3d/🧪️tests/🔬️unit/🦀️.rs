//! 🔬️ Artifact-root laws — the identity strings agree with each other, the capability roster is
//! buildable, and the example registry is append-only.

use crate::{artifact_kind, definition, WFC_GRID3D_DIALECT, WFC_GRID3D_DOCUMENT_SCHEMA};

#[test]
fn the_dialect_and_the_document_schema_are_the_same_string() {
    assert_eq!(WFC_GRID3D_DIALECT.artifact_kind, WFC_GRID3D_DOCUMENT_SCHEMA);
    assert_eq!(WFC_GRID3D_DOCUMENT_SCHEMA, "s.wfc.grid3d");
    assert_eq!(WFC_GRID3D_DIALECT.standard.0, "1");
}

#[test]
fn the_os_kind_id_is_the_dimension_qualified_one_not_the_dialect() {
    let kind = artifact_kind();
    assert_eq!(kind.id, "3d.wfcgrid3d");
    assert_eq!(kind.dimension, "3d");
    assert_eq!(kind.schema, WFC_GRID3D_DOCUMENT_SCHEMA);
    assert_eq!(kind.component_kind, "wfcgrid3d");
}

#[test]
fn the_capability_roster_builds_and_carries_both_locales() {
    let definition = definition().expect("the capability roster is buildable");
    let text = format!("{definition:?}");
    assert!(text.contains("3D Grid"));
    assert!(text.contains("3D-Raster"));
}

#[test]
fn the_example_roster_is_append_only_with_the_tiny_case_first() {
    let sources = crate::examples::sources();
    assert_eq!(sources.len(), 2);
    assert_eq!(sources[0].id(), crate::examples::blocks::ID);
    assert_eq!(sources[1].id(), crate::examples::pipes_3d::ID);
    assert_eq!(crate::examples::example_source_slice().len(), 2);
}

#[test]
fn a_mesh_child_handle_addresses_the_stdio_mesh_subset() {
    let handle = crate::mesh_child_handle("mesh-a");
    assert_eq!(handle.child_id, "mesh-a");
    assert_eq!(handle.target.artifact_id, "mesh-a");
    assert_eq!(handle.target.dialect.artifact_kind, "s.stdio.semio");
    assert_eq!(handle.target.dialect.subset, "mesh");
}

#[test]
fn every_pilot_language_carries_the_surface_its_role_needs() {
    for language in crate::pilot_languages() {
        match language.role {
            dsl::LanguageRole::Document | dsl::LanguageRole::Ops => assert!(language.grammar.is_some() && language.protocol.is_some(), "{} needs both surfaces", language.id),
            _ => assert!(language.protocol.is_some(), "{} needs a binary protocol", language.id),
        }
    }
}

//#region 🧫️FixtureBase
/// 🧫️ The ONE base scene every committed mutation vector starts from: a 2×2×2 non-uniform grid with
/// three tiles, three rules, one pin and one mask. Small enough to read in a diff, rich enough that
/// every collection mutation lands at a MID list position rather than an append — which is what makes
/// `inverse_restores_before` a real assertion instead of a tautology.
pub fn fixture_base() -> crate::Grid3dSnapshot {
    use crate::schema::snapshot::*;
    let tile = |id: &str, weight: f64, color: [u32; 4]| Grid3dTile {
        id: id.to_string(),
        label: None,
        weight,
        media: Grid3dTileMedia::Mesh { mesh: Grid3dMesh { positions: Vec::new(), indices: Vec::new(), color: Some(Grid3dColor { r: color[0], g: color[1], b: color[2], a: color[3] }) } },
    };
    let rule = |id: &str, a: &str, b: &str, direction: Grid3dDirection| Grid3dRule { id: id.to_string(), tile_a_id: a.to_string(), tile_b_id: b.to_string(), direction, allowed: true };
    Grid3dSnapshot {
        schema: WFC_GRID3D_DOCUMENT_SCHEMA.into(),
        seed: 7,
        width: 2,
        height: 2,
        depth: 2,
        cell_sizes_x: vec![1.0, 2.0],
        cell_sizes_y: vec![1.0, 1.0],
        cell_sizes_z: vec![1.0, 1.5],
        periodic_x: false,
        periodic_y: false,
        periodic_z: false,
        tiles: vec![tile("air", 2.0, [200, 214, 232, 40]), tile("floor", 1.0, [120, 120, 126, 255]), tile("wall", 3.0, [212, 204, 188, 255])],
        rules: vec![
            rule("r-right-air-air", "air", "air", Grid3dDirection::Right),
            rule("r-right-wall-wall", "wall", "wall", Grid3dDirection::Right),
            rule("r-top-floor-wall", "floor", "wall", Grid3dDirection::Top),
        ],
        pinned: vec![Grid3dPinnedCell { x: 0, y: 0, z: 0, tile_id: "floor".into() }],
        masked: vec![Grid3dCell { x: 1, y: 1, z: 1 }],
    }
}

/// 🧫️ Every committed vector: `(kind directory, case directory, the mutation)`. The generator below
/// is the only authority the quintets are written from, so a fixture can never disagree with the
/// builder that produced it.
pub fn fixture_vectors() -> Vec<(&'static str, &'static str, crate::Grid3dMutation)> {
    use crate::mutations::*;
    use crate::schema::snapshot::*;
    vec![
        ("🎲️change-seed", "🎲️reseeds-the-solve-from-7-to-99", change_seed(99)),
        ("📐️resize-grid", "📐️grows-the-grid-to-3x2x2", resize_grid(3, 2, 2)),
        ("📏️change-cell-sizes", "📏️stretches-the-x-axis-columns", change_cell_sizes(Grid3dAxis::X, vec![2.5, 0.5])),
        ("🔁️change-periodicity", "🔁️wraps-the-x-axis", change_periodicity(true, false, false)),
        (
            "🧱️create-tile",
            "🧱️adds-the-roof-tile",
            create_tile(Grid3dTile {
                id: "roof".into(),
                label: Some("Roof".into()),
                weight: 1.5,
                media: Grid3dTileMedia::Mesh { mesh: Grid3dMesh { positions: Vec::new(), indices: Vec::new(), color: Some(Grid3dColor { r: 172, g: 84, b: 62, a: 255 }) } },
            }),
        ),
        ("🕳️delete-tile", "🕳️removes-the-air-tile-and-cascades", delete_tile("air".into())),
        ("⚖️change-tile-weight", "⚖️raises-the-wall-tile-bias", change_tile_weight("wall".into(), 5.0)),
        (
            "🖼️change-tile-media",
            "🖼️replaces-the-wall-tile-mesh",
            change_tile_media("wall".into(), Grid3dTileMedia::Mesh { mesh: Grid3dMesh { positions: Vec::new(), indices: Vec::new(), color: Some(Grid3dColor { r: 40, g: 44, b: 52, a: 255 }) } }),
        ),
        ("🚦️create-rule", "🚦️allows-air-above-air", create_rule(Grid3dRule { id: "r-top-air-air".into(), tile_a_id: "air".into(), tile_b_id: "air".into(), direction: Grid3dDirection::Top, allowed: true })),
        ("❌️delete-rule", "❌️removes-the-floor-wall-rule", delete_rule("r-top-floor-wall".into())),
        ("📌️pin-cell", "📌️pins-the-far-cell-to-wall", pin_cell(Grid3dPinnedCell { x: 1, y: 0, z: 0, tile_id: "wall".into() })),
        ("📍️unpin-cell", "📍️releases-the-origin-cell", unpin_cell(0, 0, 0)),
        ("🚫️mask-cell", "🚫️carves-out-the-far-edge-cell", mask_cell(Grid3dCell { x: 0, y: 1, z: 1 })),
        ("🔓️unmask-cell", "🔓️restores-the-masked-corner", unmask_cell(1, 1, 1)),
    ]
}

/// 🧫️ Every committed vector is applicable to the base and point-invertible against it — the law the
/// generated quintets encode, asserted here so a regenerated fixture tree can never be silently wrong.
#[test]
fn every_committed_vector_applies_to_the_base_and_inverts_back() {
    use crate::mutations::{apply_grid3d_mutation, inverse_grid3d_mutation};
    let base = fixture_base();
    for (kind, case, mutation) in fixture_vectors() {
        let mut snapshot = base.clone();
        let inverse = inverse_grid3d_mutation(&base, &mutation);
        apply_grid3d_mutation(&mut snapshot, &mutation).unwrap_or_else(|error| panic!("{kind}/{case}: the vector must apply to the base ({error})"));
        assert_ne!(snapshot, base, "{kind}/{case}: an `applied` vector must change the document");
        for step in &inverse {
            apply_grid3d_mutation(&mut snapshot, step).unwrap_or_else(|error| panic!("{kind}/{case}: the inverse must apply ({error})"));
        }
        assert_eq!(snapshot, base, "{kind}/{case}: the inverse did not restore the base");
    }
}

// [DEBUG] temporary generator — writes the committed fixture quintets this lane commits. Run with
// `cargo test -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib -- --ignored debug_emit_mutation_fixtures`.
#[test]
#[ignore]
fn debug_emit_mutation_fixtures() {
    use crate::mutations::apply_grid3d_mutation;
    use protocol::Mutation;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    let base = fixture_base();
    for (kind, case, mutation) in fixture_vectors() {
        let directory = root.join(kind).join(case);
        std::fs::create_dir_all(directory.join("📸️snapshot/⬅️before")).expect("fixture directory");
        std::fs::create_dir_all(directory.join("📸️snapshot/➡️after")).expect("fixture directory");
        std::fs::create_dir_all(directory.join("🦠️mutation")).expect("fixture directory");
        std::fs::create_dir_all(directory.join("🎯️outcome")).expect("fixture directory");
        std::fs::create_dir_all(directory.join("🔺️diff")).expect("fixture directory");
        let mut after = base.clone();
        apply_grid3d_mutation(&mut after, &mutation).expect("the vector applies");
        let outcome = <crate::Grid3dMutation as Mutation<crate::Grid3dSnapshot>>::diff(&mutation, &base);
        std::fs::write(directory.join("📸️snapshot/⬅️before/🔣️.json"), pretty(&dsl::json::to_json_string(&base))).expect("write before");
        std::fs::write(directory.join("📸️snapshot/➡️after/🔣️.json"), pretty(&dsl::json::to_json_string(&after))).expect("write after");
        std::fs::write(directory.join("🦠️mutation/🔣️.json"), pretty(&dsl::json::to_json_string(&mutation))).expect("write mutation");
        std::fs::write(directory.join("🔺️diff/🔣️.json"), pretty(&dsl::json::to_json_string(outcome.diff()))).expect("write diff");
        std::fs::write(directory.join("🎯️outcome/🔣️.json"), "{\n  \"status\": \"applied\"\n}\n").expect("write outcome");
    }
}

// [DEBUG] temporary generator — prints each bundled example into its committed `🗣️.dsl.semio` asset,
// so the asset is a PRINT of the Rust builder and never a second, drifting authority.
#[test]
#[ignore]
fn debug_emit_example_assets() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples");
    for (directory, snapshot) in [("🧱️blocks", crate::examples::blocks::snapshot as fn() -> crate::Grid3dSnapshot), ("🪠️pipes-3d", crate::examples::pipes_3d::snapshot)] {
        let asset = root.join(directory).join("🖼️assets").join(directory).join("🗣️.dsl.semio");
        std::fs::create_dir_all(asset.parent().expect("asset directory")).expect("asset directory");
        std::fs::write(asset, crate::schema::snapshot::text::print_dsl(&snapshot())).expect("write dsl");
    }
}

// [DEBUG] temporary pretty-printer for the generator above. It re-INDENTS the canonical text rather
// than re-serializing a parsed value: a `serde_json::Value` round trip would re-sort every object's
// keys alphabetically, and the committed fixtures are canonical in KEY ORDER as well as in value —
// the order the artifact's own records declare their fields in.
#[cfg(test)]
fn pretty(compact: &str) -> String {
    let characters: Vec<char> = compact.chars().collect();
    let mut out = String::with_capacity(compact.len() * 2);
    let mut depth = 0usize;
    let mut cursor = 0usize;
    while cursor < characters.len() {
        let character = characters[cursor];
        cursor += 1;
        match character {
            '"' => {
                out.push(character);
                let mut escaped = false;
                while cursor < characters.len() {
                    let inside = characters[cursor];
                    cursor += 1;
                    out.push(inside);
                    if escaped {
                        escaped = false;
                    } else if inside == '\\' {
                        escaped = true;
                    } else if inside == '"' {
                        break;
                    }
                }
            }
            '{' | '[' => {
                let close = if character == '{' { '}' } else { ']' };
                if characters.get(cursor) == Some(&close) {
                    out.push(character);
                    out.push(close);
                    cursor += 1;
                    continue;
                }
                depth += 1;
                out.push(character);
                out.push('\n');
                out.push_str(&"  ".repeat(depth));
            }
            '}' | ']' => {
                depth = depth.saturating_sub(1);
                out.push('\n');
                out.push_str(&"  ".repeat(depth));
                out.push(character);
            }
            ',' => {
                out.push(character);
                out.push('\n');
                out.push_str(&"  ".repeat(depth));
            }
            ':' => out.push_str(": "),
            other => out.push(other),
        }
    }
    out.push('\n');
    out
}
//#endregion 🧫️FixtureBase
