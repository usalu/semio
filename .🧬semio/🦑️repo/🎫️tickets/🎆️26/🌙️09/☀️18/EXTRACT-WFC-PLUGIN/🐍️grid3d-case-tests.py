#!/usr/bin/env python3
"""🧪 Emits one mounted fixture-replay test module per committed `s.wfc.grid3d` mutation vector, and
prints the `#[path]` mount block the crate root needs for them. Idempotent."""

import io
import os

ROOT = "/Users/ueli/Documents/semio"
A = os.path.join(ROOT, "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d")
MUT = os.path.join(A, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")

CASES = [
    ("change_seed", "🎲️change-seed", "🎲️reseeds-the-solve-from-7-to-99", "reseeds_the_solve_from_7_to_99", "`change-seed` writes ONE scalar lane; every collection lane stays empty."),
    ("resize_grid", "📐️resize-grid", "📐️grows-the-grid-to-3x2x2", "grows_the_grid_to_3x2x2", "`resize-grid` writes the extent AND every derived cell-size array in one atomic delta, and touches no collection."),
    ("change_cell_sizes", "📏️change-cell-sizes", "📏️stretches-the-x-axis-columns", "stretches_the_x_axis_columns", "`change-cell-sizes` writes exactly ONE axis lane — the other two stay null."),
    ("change_periodicity", "🔁️change-periodicity", "🔁️wraps-the-x-axis", "wraps_the_x_axis", "`change-periodicity` writes all three flags together, because they are one setting."),
    ("create_tile", "🧱️create-tile", "🧱️adds-the-roof-tile", "adds_the_roof_tile", "`create-tile` inserts at the CANONICAL SORTED position (`roof` lands between `floor` and `wall`), never at the end — which is what makes `delete-tile`'s inverse round-trip."),
    ("delete_tile", "🕳️delete-tile", "🕳️removes-the-air-tile-and-cascades", "removes_the_air_tile_and_cascades", "`delete-tile` cascades every rule that named the tile, in the same atomic delta."),
    ("change_tile_weight", "⚖️change-tile-weight", "⚖️raises-the-wall-tile-bias", "raises_the_wall_tile_bias", "`change-tile-weight` upserts the tile IN PLACE at its existing index; the id and the media are untouched."),
    ("change_tile_media", "🖼️change-tile-media", "🖼️replaces-the-wall-tile-mesh", "replaces_the_wall_tile_mesh", "`change-tile-media` upserts the tile in place; the id and the weight are untouched."),
    ("create_rule", "🚦️create-rule", "🚦️allows-air-above-air", "allows_air_above_air", "`create-rule` inserts at the canonical sorted position — `r-top-air-air` lands BEFORE `r-top-floor-wall`."),
    ("delete_rule", "❌️delete-rule", "❌️removes-the-floor-wall-rule", "removes_the_floor_wall_rule", "`delete-rule` removes one rule and nothing else; deleting an `allowed` rule NARROWS the solve."),
    ("pin_cell", "📌️pin-cell", "📌️pins-the-far-cell-to-wall", "pins_the_far_cell_to_wall", "`pin-cell` inserts at the canonical cell-key position, so `1:0:0` lands after `0:0:0`."),
    ("unpin_cell", "📍️unpin-cell", "📍️releases-the-origin-cell", "releases_the_origin_cell", "`unpin-cell` removes by cell key; the inverse restores the pin at its own index."),
    ("mask_cell", "🚫️mask-cell", "🚫️carves-out-the-far-edge-cell", "carves_out_the_far_edge_cell", "`mask-cell` inserts at the canonical cell-key position, so `0:1:1` lands BEFORE the already-masked `1:1:1`."),
    ("unmask_cell", "🔓️unmask-cell", "🔓️restores-the-masked-corner", "restores_the_masked_corner", "`unmask-cell` removes by cell key and touches nothing else."),
]

TEMPLATE = '''//! 🧪️ `{semantic}` fixture — `{case}`.
//!
//! {rationale}
//!
//! Source of truth is the committed JSON quintet beside this file; the Rust builder that produced it
//! is `🧪️tests/🔬️unit`'s own `fixture_base`/`fixture_vectors`, so a fixture can never disagree with
//! the mutation it claims to encode.

use crate::diff::Grid3dDiff;
use crate::mutations::{{apply_grid3d_mutation, inverse_grid3d_mutation, Grid3dMutation}};
use crate::schema::snapshot::Grid3dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/🎯️outcome/🔣️.json");

fn before() -> Grid3dSnapshot {{
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Grid3dSnapshot {{
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Grid3dMutation {{
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    let mut snapshot = before();
    apply_grid3d_mutation(&mut snapshot, &mutation()).expect("{semantic} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{semantic}/{case}: applied state differs from the committed after-snapshot");
}}

/// ↩️ Applying the mutation then its inverse restores `before` exactly — row POSITION included.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_grid3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_grid3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_grid3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{semantic}/{case}: the inverse did not restore the before-snapshot");
}}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Grid3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{semantic}/{case}: committed {{side}} JSON is not canonical");
    }}
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{semantic}/{case}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises.
#[test]
fn declared_outcome_holds() {{
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <Grid3dMutation as protocol::Mutation<Grid3dSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {{
            let level = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        }})
        .collect();
    assert_eq!(produced, declared, "{semantic}/{case}: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = apply_grid3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {{
        "applied" => {{
            assert!(applied, "{semantic}/{case}: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "{semantic}/{case}: declared applied but the snapshot came back unchanged");
        }}
        "rejected" => assert_eq!(snapshot, before(), "{semantic}/{case}: a rejected mutation must leave the snapshot untouched"),
        other => panic!("{semantic}/{case}: unknown outcome status {{other:?}}"),
    }}
}}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the assertion that pins
/// WHICH lanes this kind is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {{
    let raised = <Grid3dMutation as protocol::Mutation<Grid3dSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{semantic}/{case}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: Grid3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{semantic}/{case}: committed diff JSON is not canonical");
}}

/// 🩹️ Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// COMPLETE description of what this kind changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: Grid3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <Grid3dDiff as protocol::MutationDiff<Grid3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{semantic}/{case}: committed diff did not carry before to after");
}}
'''

MOUNT = '''                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{dir}/🧪️tests/{case}/🦀️.rs"]
                            mod tests_{ident};
'''

mounts = {}
for module, directory, case, ident, rationale in CASES:
    semantic = directory.split("️", 1)[-1] if "️" in directory else directory
    semantic = directory[1:] if not directory[0].isalpha() else directory
    semantic = "".join(character for character in directory if character.isalpha() or character == "-").lstrip("-")
    path = os.path.join(MUT, directory, "🧪️tests", case, "🦀️.rs")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    io.open(path, "w", encoding="utf-8").write(TEMPLATE.format(semantic=semantic, case=case, dir=directory, rationale=rationale))
    mounts[module] = MOUNT.format(dir=directory, case=case, ident=ident)

# mount every case module inside its own kind's `#[path = "."]` block in the crate root
root = os.path.join(A, "🦀️.rs")
source = io.open(root, encoding="utf-8").read()
for module, mount in mounts.items():
    anchor = f'''                        pub mod {module} {{\n'''
    start = source.index(anchor)
    end = source.index("                        }\n", start)
    block = source[start:end]
    if "mod tests_" in block:
        continue
    source = source[:end] + mount + source[end:]
io.open(root, "w", encoding="utf-8").write(source)
print(f"wrote {len(CASES)} case tests and mounted them in the crate root")
