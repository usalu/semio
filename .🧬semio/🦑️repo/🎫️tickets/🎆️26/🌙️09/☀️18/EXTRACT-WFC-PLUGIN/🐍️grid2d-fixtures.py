#!/usr/bin/env python3
"""🧫 Emits the `s.wfc.grid2d` fixture quintets, their mounted Rust tests, the subset-level
`🥒️.feature`/`🐍️.py` oracle-replay triplet and the `🔮️oracles/🔣️.json` manifest — all from ONE
hand-planned case table, so a row that gains or loses a vector cannot leave either half behind.

The domain answers come from `🐍️grid2d-oracle.py`, a second implementation that never reads the
Rust crate. Run from anywhere: `python3 🐍️grid2d-fixtures.py`."""
from __future__ import annotations

import copy
import json
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import importlib

oracle = importlib.import_module("🐍️grid2d-oracle")
Float = oracle.Float

ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSET = ROOT / "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUTATIONS = SUBSET / "🧬️schema/🧬️mutations"
FIXTURES = SUBSET / "🧫️fixtures/🧬️mutations"


#region 🧪 the toy scene
def toy() -> dict:
    line = oracle.path(
        [{"kind": "moveTo", "to": oracle.point(0.0, 0.5)}, {"kind": "lineTo", "to": oracle.point(1.0, 0.5)}],
        stroke=oracle.color(56, 189, 248),
        stroke_width=0.2,
    )
    return oracle.snapshot(
        seed=7,
        width=3,
        height=2,
        cell_width=4.0,
        cell_height=4.0,
        periodic_x=False,
        periodic_y=False,
        tiles=[
            oracle.tile("empty", 2.0, oracle.vector_media([]), "Empty"),
            oracle.tile("straight", 1.0, oracle.vector_media([line]), "Straight"),
        ],
        rules=[
            oracle.rule("rule-bottom-empty-empty", "empty", "empty", "BOTTOM", True),
            oracle.rule("rule-right-empty-empty", "empty", "empty", "RIGHT", True),
            oracle.rule("rule-right-straight-straight", "straight", "straight", "RIGHT", True),
        ],
        pins=[oracle.pinned(0, 0, "empty"), oracle.pinned(1, 0, "straight")],
        masks=[oracle.cell(2, 1)],
    )
#endregion


#region 📋 the case table
CORNER_TILE = oracle.tile("corner", 1.0, oracle.vector_media([]), "Corner")
BITMAP_MEDIA = oracle.bitmap_media(2, 2, [oracle.color(15, 23, 42), oracle.color(148, 163, 184)], "AAEBAA==")

CASES = [
    dict(kind="change-seed", dir="🎲️change-seed", module="change_seed", case="🎲️reseeds-the-solve-from-7-to-99", ident="tests_reseeds_the_solve_from_7_to_99", mutation={"ChangeSeed": {"seed": 99}},
         note="`change-seed` sets the scalar `seed` lane only — every id-keyed collection delta stays empty, so the solve inference re-runs without the spec itself moving."),
    dict(kind="resize-grid", dir="📐️resize-grid", module="resize_grid", case="📐️shrinks-the-board-and-drops-the-outside-cells", ident="tests_shrinks_the_board_and_drops_the_outside_cells",
         mutation={"ResizeGrid": {"width": 2, "height": 1}},
         note="Shrinking cascades every pinned and masked cell that falls outside the new extent — a cell state may never address a cell the grid does not have."),
    dict(kind="change-cell-size", dir="📏️change-cell-size", module="change_cell_size", case="📏️widens-every-cell", ident="tests_widens_every_cell",
         mutation={"ChangeCellSize": {"cellWidth": Float(8.0), "cellHeight": Float(6.0)}},
         note="Cell size is pure geometry: the tile catalogue, the rules and every cell state are untouched."),
    dict(kind="change-periodicity", dir="🔁️change-periodicity", module="change_periodicity", case="🔁️wraps-the-x-axis", ident="tests_wraps_the_x_axis",
         mutation={"ChangePeriodicity": {"periodicX": True, "periodicY": False}},
         note="A periodic axis becomes `Boundary::Wrap` in the solve topology; the persisted document only records the two flags."),
    dict(kind="create-tile", dir="🌱️create-tile", module="create_tile", case="🌱️inserts-the-corner-tile-in-sorted-order", ident="tests_inserts_the_corner_tile_in_sorted_order",
         mutation={"CreateTile": {"tile": copy.deepcopy(CORNER_TILE)}},
         note="`corner` sorts BEFORE `empty`, so the upsert lands at index 0 — an append would break `delete-tile`'s own inverse."),
    dict(kind="delete-tile", dir="🗑️delete-tile", module="delete_tile", case="🗑️removes-the-straight-tile-and-cascades-its-rule-and-pin", ident="tests_removes_the_straight_tile_and_cascades_its_rule_and_pin",
         mutation={"DeleteTile": {"id": "straight"}},
         note="Deleting a tile cascades to every rule naming it AND every cell pinned to it — a rule or pin may never dangle."),
    dict(kind="change-tile-weight", dir="⚖️change-tile-weight", module="change_tile_weight", case="⚖️biases-the-solve-towards-empty", ident="tests_biases_the_solve_towards_empty",
         mutation={"ChangeTileWeight": {"id": "empty", "weight": Float(5.0)}},
         note="A weight change is an in-place upsert at the tile's EXISTING index: re-biasing never moves a row."),
    dict(kind="change-tile-media", dir="🎨️change-tile-media", module="change_tile_media", case="🎨️redraws-the-empty-tile-as-a-bitmap", ident="tests_redraws_the_empty_tile_as_a_bitmap",
         mutation={"ChangeTileMedia": {"id": "empty", "media": copy.deepcopy(BITMAP_MEDIA)}},
         note="Media is what a tile LOOKS like, never what it means — the pattern universe, the rules and the pins keep addressing the same id."),
    dict(kind="create-rule", dir="🚦️create-rule", module="create_rule", case="🚦️lets-two-straights-stack-vertically", ident="tests_lets_two_straights_stack_vertically",
         mutation={"CreateRule": {"rule": oracle.rule("rule-bottom-straight-straight", "straight", "straight", "BOTTOM", True)}},
         note="The pair `(straight, straight, BOTTOM)` had no rule, so it was FORBIDDEN by default; the new row whitelists it."),
    dict(kind="delete-rule", dir="❌delete-rule", module="delete_rule", case="❌️forbids-the-straight-pair-again", ident="tests_forbids_the_straight_pair_again",
         mutation={"DeleteRule": {"id": "rule-right-straight-straight"}},
         note="Removing the row returns the pair to the FORBIDDEN default for that direction."),
    dict(kind="pin-cell", dir="📌️pin-cell", module="pin_cell", case="📌️fixes-the-right-cell-to-the-straight-tile", ident="tests_fixes_the_right_cell_to_the_straight_tile",
         mutation={"PinCell": {"x": 2, "y": 0, "tileId": "straight"}},
         note="A pin is a hard pre-assignment the solver seeds its domains with; the new row lands at its canonical row-major position."),
    dict(kind="unpin-cell", dir="📍️unpin-cell", module="unpin_cell", case="📍️releases-the-pinned-straight-cell", ident="tests_releases_the_pinned_straight_cell",
         mutation={"UnpinCell": {"x": 1, "y": 0}},
         note="Unpinning releases one cell back to the solver and touches nothing else."),
    dict(kind="mask-cell", dir="🕳️mask-cell", module="mask_cell", case="🕳️cuts-the-pinned-corner-out-of-the-problem", ident="tests_cuts_the_pinned_corner_out_of_the_problem",
         mutation={"MaskCell": {"x": 0, "y": 0}},
         note="Masking cuts a cell out of the problem entirely, so it cascades away the pin that cell held."),
    dict(kind="unmask-cell", dir="🔳️unmask-cell", module="unmask_cell", case="🔳️puts-the-hole-back-into-the-problem", ident="tests_puts_the_hole_back_into_the_problem",
         mutation={"UnmaskCell": {"x": 2, "y": 1}},
         note="Unmasking returns the cell to the problem; it comes back unpinned, because the mask cascaded its pin away."),
]
#endregion


#region 🧪 emitters
TEST_TEMPLATE = '''//! 🧪️ `{kind}` fixture — `{case}`.
//!
//! {note}
//!
//! Source of truth is the committed JSON quintet beside this file; this module only replays it.

use crate::diff::Grid2dDiff;
use crate::mutations::{{apply_grid2d_mutation, inverse_grid2d_mutation, Grid2dMutation}};
use crate::schema::snapshot::Grid2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{dir}/{case}/🎯️outcome/🔣️.json");

fn before() -> Grid2dSnapshot {{
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Grid2dSnapshot {{
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Grid2dMutation {{
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    let mut snapshot = before();
    apply_grid2d_mutation(&mut snapshot, &mutation()).expect("{kind} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{kind}/{case}: applied state differs from the committed after-snapshot");
}}

/// ↩️ Applying the mutation then its inverse restores `before` exactly — VALUE and POSITION.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_grid2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_grid2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_grid2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{kind}/{case}: inverse did not restore the before-snapshot");
}}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Grid2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{kind}/{case}: committed {{side}} JSON is not canonical");
    }}
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{kind}/{case}: committed mutation JSON is not canonical");
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
    let raised = <Grid2dMutation as protocol::Mutation<Grid2dSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {{
            let level = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        }})
        .collect();
    assert_eq!(produced, declared, "{kind}/{case}: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = apply_grid2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {{
        "applied" => {{
            assert!(applied, "{kind}/{case}: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "{kind}/{case}: declared applied but the snapshot came back unchanged");
        }}
        "rejected" => assert_eq!(snapshot, before(), "{kind}/{case}: a rejected mutation must leave the snapshot untouched"),
        other => panic!("{kind}/{case}: unknown outcome status {{other:?}}"),
    }}
}}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins WHICH lanes
/// `{kind}` is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {{
    let base = before();
    let raised = <Grid2dMutation as protocol::Mutation<Grid2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{kind}/{case}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: Grid2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{kind}/{case}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: Grid2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <Grid2dDiff as protocol::MutationDiff<Grid2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{kind}/{case}: committed diff did not carry before to after");
}}
'''


def write(path: pathlib.Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def emit_fixtures() -> None:
    for case in CASES:
        base = toy()
        outcome = oracle.build(base, case["mutation"])
        after = oracle.apply(base, outcome.delta)
        root = FIXTURES / case["dir"] / case["case"]
        write(root / "📸️snapshot/⬅️before/🔣️.json", oracle.dumps(base))
        write(root / "📸️snapshot/➡️after/🔣️.json", oracle.dumps(after))
        write(root / "🦠️mutation/🔣️.json", oracle.dumps(case["mutation"]))
        write(root / "🎯️outcome/🔣️.json", oracle.dumps(outcome.as_json()))
        write(root / "🔺️diff/🔣️.json", oracle.dumps(outcome.delta))
        write(MUTATIONS / case["dir"] / "🧪️tests" / case["case"] / "🦀️.rs", TEST_TEMPLATE.format(kind=case["kind"], case=case["case"], dir=case["dir"], note=case["note"]))
        print("fixture", case["kind"])


def emit_feature() -> None:
    rows = "\n".join(
        f"      | {case['kind']} | {case['case']} | asset://wfc/grid2d/1/any/mutations/{case['kind']}/{case['case']} | applied |" for case in CASES
    )
    text = f"""# 🥒️ `s.wfc.grid2d@1/*` mutation replay — one scenario per mutation kind, every vector declared
# as an `asset://` fixture URI so a plan can pin it by digest instead of inlining a payload.
Feature: Mutating a 2D WFC grid
  The persisted document is the PROBLEM only. Every mutation below carries its committed
  before-snapshot to its committed after-snapshot, and its inverse carries it back — value AND
  position, because every collection insert lands at its canonical sorted index.

  Scenario Outline: <kind> replays its committed vector
    Given the committed before-snapshot of <vector>
    When the <kind> mutation of <vector> is applied
    Then the document equals the committed after-snapshot
    And the produced diff equals the committed diff
    And the declared outcome is <code>
    And applying the inverse restores the before-snapshot

    Examples:
      | kind | vector | asset | code |
{rows}
"""
    write(SUBSET / "🧪️tests/🔲️mutate-grid2d-1/🥒️.feature", text)
    print("feature")


def emit_python_replay() -> None:
    text = '''#!/usr/bin/env python3
"""🐍 `s.wfc.grid2d` oracle replay — the second implementation the `🥒️.feature` scenarios are
measured against. It reads ONLY the committed fixture quintets and this subset's own reference
semantics (`🐍️grid2d-oracle.py` in the ticket folder, vendored here as `reference`), never the Rust
crate: two implementations that agree are evidence, one implementation echoing itself is not."""
from __future__ import annotations

import json
import pathlib
import sys

FIXTURES = pathlib.Path(__file__).resolve().parents[2] / "🧫️fixtures/🧬️mutations"

VECTORS = [
'''
    for case in CASES:
        text += f'    ("{case["kind"]}", "{case["dir"]}", "{case["case"]}"),\n'
    text += '''
]


def load(root: pathlib.Path, *parts: str) -> object:
    return json.loads((root.joinpath(*parts)).read_text(encoding="utf-8"))


def replay() -> int:
    failures = 0
    for kind, directory, case in VECTORS:
        root = FIXTURES / directory / case
        before = load(root, "📸️snapshot/⬅️before/🔣️.json")
        after = load(root, "📸️snapshot/➡️after/🔣️.json")
        delta = load(root, "🔺️diff/🔣️.json")
        outcome = load(root, "🎯️outcome/🔣️.json")
        mutation = load(root, "🦠️mutation/🔣️.json")
        produced = reference.apply(before, delta)
        if produced != after:
            print(f"FAIL {kind}/{case}: committed diff does not carry before to after")
            failures += 1
        rebuilt = reference.build(before, mutation)
        if json.loads(reference.dumps(rebuilt.delta)) != delta:
            print(f"FAIL {kind}/{case}: reference diff differs from the committed diff")
            failures += 1
        if json.loads(reference.dumps(rebuilt.as_json())) != outcome:
            print(f"FAIL {kind}/{case}: reference outcome differs from the committed outcome")
            failures += 1
        restored = produced
        for step in reference.inverse(before, mutation):
            restored = reference.apply(restored, reference.build(restored, step).delta)
        if restored != before:
            print(f"FAIL {kind}/{case}: inverse did not restore the before-snapshot")
            failures += 1
    print(f"{len(VECTORS)} vectors replayed, {failures} failure(s)")
    return failures


if __name__ == "__main__":
    ticket = pathlib.Path(__file__).resolve().parents[11] / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/EXTRACT-WFC-PLUGIN"
    sys.path.insert(0, str(ticket))
    import importlib

    reference = importlib.import_module("🐍️grid2d-oracle")
    raise SystemExit(1 if replay() else 0)
'''
    write(SUBSET / "🧪️tests/🔲️mutate-grid2d-1/🐍️.py", text)
    print("python replay")


def emit_oracle_manifest() -> None:
    manifest = {
        "$schema": "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json",
        "schemaVersion": 2,
        "oracles": [
            {
                "id": "wfc-grid2d-reference",
                "ecosystem": "python",
                "package": "",
                "capabilities": ["mutation-apply", "mutation-inverse", "mutation-diff"],
                "comparisonProfiles": ["exact-json"],
                "license": "in-repo",
                "testOnly": True,
                "kind": "verified-native-second-implementation",
                "nativeSecondImplementation": {
                    "noThirdPartySurvey": {
                        "ecosystemsSearched": ["pypi", "crates.io", "npm"],
                        "candidatesConsidered": [
                            "No third-party package implements THIS document's mutation algebra: the published WaveFunctionCollapse libraries (mxgmn/WaveFunctionCollapse ports, `wfc`, `fast-wfc`) solve a tiled model, they do not model an editable problem document with an invertible, canonically-positioned mutation log.",
                            "The SOLVER half is separately cross-checked inside `semio-s-plugin-wfc-engine` by its own brute-force `oracle` module, so this manifest's scope is the DOCUMENT algebra only."
                        ],
                    },
                    "fixtureCoverage": {"vectors": len(CASES)},
                },
                "rationale": "The fixture quintets are computed by `🐍️grid2d-oracle.py`, an independent Python transcription of the normative JSON Schema leaves plus the fourteen mutations' diff/apply/inverse semantics. It never imports the Rust crate, so a shared bug cannot hide in both halves.",
            }
        ],
        "noOracleDecisions": [
            {
                "id": "wfc-grid2d-no-third-party-document-oracle",
                "capability": "mutation-algebra",
                "blockers": ["No published library models an invertible WFC PROBLEM document; every candidate models the solve only."],
                "substitutes": ["specification-vectors", "metamorphic-laws"],
                "notes": "The metamorphic law every vector asserts is `apply ∘ inverse = identity` over value AND position — the exact failure mode an append-shaped insert produces.",
            }
        ],
        "mutationCatalogs": [
            {
                "id": "wfc-grid2d-1-any",
                "capability": "mutation-algebra",
                "standardDirectoryName": "🔖️1",
                "subsetDirectoryName": "✳️any",
                "vectors": [
                    {"mutationId": case["kind"], "sourceMutationDirectoryName": case["dir"], "mutationDirectoryName": case["dir"], "scenarios": [{"id": case["case"], "directoryName": case["case"]}]}
                    for case in CASES
                ],
            }
        ],
    }
    write(SUBSET / "🔮️oracles/🔣️.json", json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
    print("oracle manifest")


def emit_mount_block() -> None:
    lines = []
    for case in CASES:
        lines.append(f'                        #[path = "."]')
        lines.append(f'                        pub mod {case["module"]} {{')
        lines.append(f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{case["dir"]}/🦀️.rs"]')
        lines.append("                            mod component;")
        lines.append(f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{case["dir"]}/🔺️diff/🦀️.rs"]')
        lines.append("                            pub mod diff;")
        lines.append(f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{case["dir"]}/↩️inverse/🦀️.rs"]')
        lines.append("                            pub mod inverse;")
        lines.append("                            pub use component::*;")
        lines.append("                            #[cfg(test)]")
        lines.append(f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{case["dir"]}/🧪️tests/{case["case"]}/🦀️.rs"]')
        lines.append(f'                            mod {case["ident"]};')
        lines.append("                        }")
    write(pathlib.Path(__file__).resolve().parent / "🗑️generated/grid2d/mount-block.rs.txt", "\n".join(lines) + "\n")
    print("mount block")


if __name__ == "__main__":
    emit_fixtures()
    emit_feature()
    emit_python_replay()
    emit_oracle_manifest()
    emit_mount_block()
#endregion
