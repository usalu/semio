#!/usr/bin/env python3
"""🧬 Emits the 14 `s.wfc.grid2d` mutation triad leaves (payload `🦀️.rs`, manifest `🔣️.json`,
per-kind payload JSON Schema). The `🔺️diff`/`↩️inverse` bodies are hand-authored beside this
script's output and are NOT generated — only the mechanical shells are."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[7]
MUT = ROOT / "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
OWNER = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
BASE_ID = "https://json.schemas.assets.semio-tech.com/s/wfc/grid2d/1/any/mutation"

NUM = {"type": "number"}
INT = {"type": "integer", "minimum": 0}
STR = {"type": "string"}
BOOL = {"type": "boolean"}

COLOR = {
    "title": "WfcColor",
    "type": "object",
    "additionalProperties": False,
    "required": ["r", "g", "b", "a"],
    "properties": {k: {"type": "integer", "minimum": 0, "maximum": 255} for k in "rgba"},
}
POINT = {"title": "WfcPoint2", "type": "object", "additionalProperties": False, "required": ["x", "y"], "properties": {"x": NUM, "y": NUM}}
SEGMENT = {
    "title": "WfcPathSegment",
    "type": "object",
    "required": ["kind"],
    "properties": {"kind": {"enum": ["moveTo", "lineTo", "quadTo", "cubicTo", "close"]}, "to": POINT, "ctrl": POINT, "ctrl1": POINT, "ctrl2": POINT},
}
VECTOR_PATH = {
    "title": "WfcVectorPath",
    "type": "object",
    "additionalProperties": False,
    "required": ["segments", "strokeWidth"],
    "properties": {"segments": {"type": "array", "items": SEGMENT}, "fill": COLOR, "stroke": COLOR, "strokeWidth": NUM},
}
MEDIA = {
    "title": "WfcTileMedia2d",
    "type": "object",
    "required": ["kind"],
    "properties": {
        "kind": {"enum": ["bitmap", "vector", "image"]},
        "width": INT,
        "height": INT,
        "palette": {"type": "array", "items": COLOR},
        "pixels": {"type": "string", "format": "base64"},
        "paths": {"type": "array", "items": VECTOR_PATH},
        "child": {"type": "object"},
    },
}
TILE = {
    "title": "WfcTile2d",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "weight", "media"],
    "properties": {"id": STR, "label": STR, "weight": NUM, "media": MEDIA},
}
RULE = {
    "title": "WfcAdjacencyRule2d",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "tileAId", "tileBId", "direction", "allowed"],
    "properties": {"id": STR, "tileAId": STR, "tileBId": STR, "direction": {"enum": ["LEFT", "RIGHT", "TOP", "BOTTOM"]}, "allowed": BOOL},
}

# slug, dir emoji, struct, verb, entity, record, display name, fields[(rust name, rust type, json key, json schema)], label fmt, targets
KINDS = [
    dict(
        slug="change-seed", dir="🎲️change-seed", emoji="🎲️", struct="ChangeSeed", fn="change_seed", verb="change", entity="seed", record="ChangedSeed", display="Change Seed",
        doc="🎲 Sets the deterministic WFC solve seed. PERSISTED snapshot field, authored ONLY here — never ambient — so the solve inference's `DepHash` caching stays sound.",
        fields=[("seed", "u64", "seed", INT)], label='format!("Change seed to {}", self.seed)', target=None, outcomes=["info", "applied"],
    ),
    dict(
        slug="resize-grid", dir="📐️resize-grid", emoji="📐️", struct="ResizeGrid", fn="resize_grid", verb="resize", entity="grid", record="ResizedGrid", display="Resize Grid",
        doc="📐 Changes the cell extent of the grid. Cells that fall outside the new extent are cascaded away — a pinned or masked cell may never address a cell the grid does not have.",
        fields=[("width", "u32", "width", {"type": "integer", "minimum": 1}), ("height", "u32", "height", {"type": "integer", "minimum": 1})],
        label='format!("Resize grid to {}×{}", self.width, self.height)', target=None, outcomes=["info", "applied"],
    ),
    dict(
        slug="change-cell-size", dir="📏️change-cell-size", emoji="📏️", struct="ChangeCellSize", fn="change_cell_size", verb="change", entity="cell-size", record="ChangedCellSize", display="Change Cell Size",
        doc="📏 Changes the world size of one cell — the box every tile's media is scaled into, and the `grid_factor` the grid window snaps to.",
        fields=[("cell_width", "f64", "cellWidth", NUM), ("cell_height", "f64", "cellHeight", NUM)],
        label='format!("Change cell size to {} × {}", self.cell_width, self.cell_height)', target=None, outcomes=["info", "applied"],
    ),
    dict(
        slug="change-periodicity", dir="🔁️change-periodicity", emoji="🔁️", struct="ChangePeriodicity", fn="change_periodicity", verb="change", entity="periodicity", record="ChangedPeriodicity", display="Change Periodicity",
        doc="🔁 Sets whether each axis wraps. A periodic axis becomes `Boundary::Wrap` in the solve topology; a non-periodic one becomes `Boundary::Open`.",
        fields=[("periodic_x", "bool", "periodicX", BOOL), ("periodic_y", "bool", "periodicY", BOOL)],
        label='format!("Change periodicity to x={} y={}", self.periodic_x, self.periodic_y)', target=None, outcomes=["info", "applied"],
    ),
    dict(
        slug="create-tile", dir="🌱️create-tile", emoji="🌱️", struct="CreateTile", fn="create_tile", verb="create", entity="tile", record="CreatedTile", display="Create Tile",
        doc="🌱 Brings a new tile into the pattern universe, inserted at its canonical sorted position so a later `delete-tile` inverse restores it exactly where it was.",
        fields=[("tile", "WfcTile2d", "tile", TILE)], label='format!("Create tile \\"{}\\"", self.tile.id)', target="vec![self.tile.id.clone()]", outcomes=["applied"],
    ),
    dict(
        slug="delete-tile", dir="🗑️delete-tile", emoji="🗑️", struct="DeleteTile", fn="delete_tile", verb="delete", entity="tile", record="DeletedTile", display="Delete Tile",
        doc="🗑 Removes a tile and cascades to every adjacency rule naming it and every cell pinned to it — a rule or pin may never dangle.",
        fields=[("id", "String", "id", STR)], label='format!("Delete tile \\"{}\\"", self.id)', target="vec![self.id.clone()]", outcomes=["info", "applied"],
    ),
    dict(
        slug="change-tile-weight", dir="⚖️change-tile-weight", emoji="⚖️", struct="ChangeTileWeight", fn="change_tile_weight", verb="change", entity="tile-weight", record="ChangedTileWeight", display="Change Tile Weight",
        doc="⚖ Re-biases one tile's sampling weight — the `WeightTable` column the solver's entropy heuristic reads.",
        fields=[("id", "String", "id", STR), ("weight", "f64", "weight", {"type": "number", "exclusiveMinimum": 0})],
        label='format!("Change weight of tile \\"{}\\" to {}", self.id, self.weight)', target="vec![self.id.clone()]", outcomes=["info", "applied"],
    ),
    dict(
        slug="change-tile-media", dir="🎨️change-tile-media", emoji="🎨️", struct="ChangeTileMedia", fn="change_tile_media", verb="change", entity="tile-media", record="ChangedTileMedia", display="Change Tile Media",
        doc="🎨 Replaces what one tile LOOKS like — never what it means: the pattern universe, the rules and every pin keep addressing the same tile id.",
        fields=[("id", "String", "id", STR), ("media", "WfcTileMedia2d", "media", MEDIA)],
        label='format!("Change media of tile \\"{}\\"", self.id)', target="vec![self.id.clone()]", outcomes=["info", "applied"],
    ),
    dict(
        slug="create-rule", dir="🚦️create-rule", emoji="🚦️", struct="CreateRule", fn="create_rule", verb="create", entity="rule", record="CreatedRule", display="Create Rule",
        doc="🚦 Declares whether tile B may sit in one direction of tile A. Unspecified pairs default to FORBIDDEN, so the rule set is the complete whitelist.",
        fields=[("rule", "WfcAdjacencyRule2d", "rule", RULE)], label='format!("Create rule \\"{}\\"", self.rule.id)', target="vec![self.rule.id.clone()]", outcomes=["applied"],
    ),
    dict(
        slug="delete-rule", dir="❌delete-rule", emoji="❌", struct="DeleteRule", fn="delete_rule", verb="delete", entity="rule", record="DeletedRule", display="Delete Rule",
        doc="❌ Removes one adjacency rule — the pair falls back to the FORBIDDEN default for that direction.",
        fields=[("id", "String", "id", STR)], label='format!("Delete rule \\"{}\\"", self.id)', target="vec![self.id.clone()]", outcomes=["applied"],
    ),
    dict(
        slug="pin-cell", dir="📌️pin-cell", emoji="📌️", struct="PinCell", fn="pin_cell", verb="fix", entity="cell", record="FixedCell", display="Pin Cell",
        doc="📌 Pre-assigns one cell to a tile — a hard fix the solver seeds its domains with, not a hint.",
        fields=[("x", "u32", "x", INT), ("y", "u32", "y", INT), ("tile_id", "String", "tileId", STR)],
        label='format!("Pin cell ({}, {}) to \\"{}\\"", self.x, self.y, self.tile_id)', target='vec![format!("{},{}", self.x, self.y)]', outcomes=["info", "applied"],
    ),
    dict(
        slug="unpin-cell", dir="📍️unpin-cell", emoji="📍️", struct="UnpinCell", fn="unpin_cell", verb="clear", entity="cell-pin", record="ClearedCellPin", display="Unpin Cell",
        doc="📍 Releases one pre-assigned cell back to the solver.",
        fields=[("x", "u32", "x", INT), ("y", "u32", "y", INT)], label='format!("Unpin cell ({}, {})", self.x, self.y)', target='vec![format!("{},{}", self.x, self.y)]', outcomes=["applied"],
    ),
    dict(
        slug="mask-cell", dir="🕳️mask-cell", emoji="🕳️", struct="MaskCell", fn="mask_cell", verb="remove", entity="cell", record="RemovedCell", display="Mask Cell",
        doc="🕳 Cuts one cell out of the problem entirely. A masked cell carries no domain, draws no tile and cascades away any pin it held.",
        fields=[("x", "u32", "x", INT), ("y", "u32", "y", INT)], label='format!("Mask cell ({}, {})", self.x, self.y)', target='vec![format!("{},{}", self.x, self.y)]', outcomes=["info", "applied"],
    ),
    dict(
        slug="unmask-cell", dir="🔳️unmask-cell", emoji="🔳️", struct="UnmaskCell", fn="unmask_cell", verb="restore", entity="cell", record="RestoredCell", display="Unmask Cell",
        doc="🔳 Puts a masked cell back into the problem.",
        fields=[("x", "u32", "x", INT), ("y", "u32", "y", INT)], label='format!("Unmask cell ({}, {})", self.x, self.y)', target='vec![format!("{},{}", self.x, self.y)]', outcomes=["applied"],
    ),
]

IMPORTS = {
    "WfcTile2d": "WfcTile2d",
    "WfcAdjacencyRule2d": "WfcAdjacencyRule2d",
    "WfcTileMedia2d": "WfcTileMedia2d",
}


def leaf_rs(kind):
    extra = sorted({IMPORTS[ty] for _, ty, _, _ in kind["fields"] if ty in IMPORTS})
    names = ["Grid2dSnapshot"] + extra
    snapshot_import = names[0] if len(names) == 1 else "{" + ", ".join(names) + "}"
    fields = "\n".join(f"    pub {name}: {ty}," for name, ty, _, _ in kind["fields"])
    args = ", ".join(f"{name}: {ty}" for name, ty, _, _ in kind["fields"])
    ctor = ", ".join(name for name, _, _, _ in kind["fields"])
    target = f"\n    fn target(&self) -> Vec<String> {{\n        {kind['target']}\n    }}" if kind["target"] else ""
    return f"""//! {kind['doc']}

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::{snapshot_import};
use protocol::{{MutationKind, SemanticDescriptor}};
use semio_framework_value_derive::{{FromValue, ToValue}};

//#region 🔖️{kind['struct']}
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct {kind['struct']} {{
{fields}
}}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn {kind['fn']}({args}) -> Grid2dMutation {{
    Grid2dMutation::{kind['struct']}({kind['struct']} {{ {ctor} }})
}}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for {kind['struct']} {{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor {{ verb: "{kind['verb']}", entity: "{kind['entity']}", kind: "{kind['slug']}", record: "{kind['record']}" }};

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {{
        super::diff::diff(self, base)
    }}
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {{
        super::inverse::inverse(self, base)
    }}
    fn label(&self) -> String {{
        {kind['label']}
    }}{target}
}}
//#endregion 🔖️{kind['struct']}
"""


def manifest(kind):
    return {
        "schemaVersion": 1,
        "owner": f"{OWNER}/{kind['dir']}",
        "semanticKind": kind["slug"],
        "displayName": kind["display"],
        "emoji": kind["emoji"],
        "aggregateVariant": kind["struct"],
        "payloadSchema": "🧬️schema/🔣️.json",
        "textOpcode": None,
        "binaryTag": None,
        "invertibility": "explicit-mutation",
        "diffParticipation": "detect",
        "outcomeClasses": kind["outcomes"],
        "composition": "atomic",
        "requiredLanguageSurfaces": ["rust", "json-schema"],
    }


def payload_schema(kind):
    return {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": f"{BASE_ID}/{kind['slug']}/schema.json",
        "title": kind["struct"],
        "type": "object",
        "additionalProperties": False,
        "required": [json_key for _, _, json_key, _ in kind["fields"]],
        "properties": {json_key: schema for _, _, json_key, schema in kind["fields"]},
    }


def main():
    for kind in KINDS:
        base = MUT / kind["dir"]
        (base / "🧬️schema").mkdir(parents=True, exist_ok=True)
        (base / "🔺️diff").mkdir(parents=True, exist_ok=True)
        (base / "↩️inverse").mkdir(parents=True, exist_ok=True)
        (base / "🦀️.rs").write_text(leaf_rs(kind), encoding="utf-8")
        (base / "🔣️.json").write_text(json.dumps(manifest(kind), ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        (base / "🧬️schema/🔣️.json").write_text(json.dumps(payload_schema(kind), ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        print("wrote", kind["slug"])


if __name__ == "__main__":
    main()
