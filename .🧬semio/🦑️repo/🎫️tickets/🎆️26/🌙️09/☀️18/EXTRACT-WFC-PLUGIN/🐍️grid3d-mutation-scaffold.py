#!/usr/bin/env python3
"""🧬 Emits the fourteen `s.wfc.grid3d` mutation folders (payload + diff + inverse + manifest +
per-mutation JSON Schema). Idempotent: re-running rewrites the same bytes."""

import json
import os

ROOT = "/Users/ueli/Documents/semio"
MUT = os.path.join(ROOT, "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")
OWNER = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

HEAD = """use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
"""

KINDS = []


def kind(dirname, emoji, semantic, display, variant, module, doc, fields, builder_args, builder_body,
         semantics, label, target, diff_body, inverse_body, payload_schema, outcome_classes=None):
    KINDS.append(dict(dirname=dirname, emoji=emoji, semantic=semantic, display=display, variant=variant,
                      module=module, doc=doc, fields=fields, builder_args=builder_args,
                      builder_body=builder_body, semantics=semantics, label=label, target=target,
                      diff_body=diff_body, inverse_body=inverse_body, payload_schema=payload_schema,
                      outcome_classes=outcome_classes or ["applied"]))


# ─────────────────────────────────────────────────────────────── 1. change-seed
kind(
    "🎲️change-seed", "🎲️", "change-seed", "Change Seed", "ChangeSeed", "change_seed",
    "🎲 `s.wfc.grid3d` mutation — `ChangeSeed`: sets the deterministic WFC solve seed. PERSISTED\n//! snapshot field, authored ONLY here — never ambient — so the solve inference's `DepHash` caching\n//! stays sound (WFC is seeded-random internally).",
    "    pub seed: u64,",
    "seed: u64", "ChangeSeed { seed }",
    '{ verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" }',
    'format!("Change seed to {}", self.seed)', None,
    """    if base.seed == payload.seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seed is already {}.", payload.seed));
    }
    protocol::MutationOutcome::new(Grid3dDiff { seed: Some(payload.seed), ..Default::default() })""",
    "    let _ = payload;\n    vec![crate::mutations::change_seed(base.seed)]",
    {"type": "object", "additionalProperties": False, "required": ["seed"],
     "properties": {"seed": {"type": "integer", "minimum": 0}}},
    outcome_classes=["info", "applied"],
)

# ────────────────────────────────────────────────────────────── 2. resize-grid
kind(
    "📐️resize-grid", "📐️", "resize-grid", "Resize Grid", "ResizeGrid", "resize_grid",
    "📐 `s.wfc.grid3d` mutation — `ResizeGrid`: changes the grid extent and derives each axis' size\n//! array from it (truncated, or extended with the last authored size). Refuses outright when a pin\n//! or a mask would be left outside the new extent, so the mutation stays atomic and point-invertible\n//! instead of silently cascading a cell away.",
    "    pub width: u32,\n    pub height: u32,\n    pub depth: u32,",
    "width: u32, height: u32, depth: u32", "ResizeGrid { width, height, depth }",
    '{ verb: "resize", entity: "grid", kind: "resize-grid", record: "ResizedGrid" }',
    'format!("Resize grid to {}×{}×{}", self.width, self.height, self.depth)', None,
    """    if payload.width == 0 || payload.height == 0 || payload.depth == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A grid axis cannot be empty.".to_string(), ["grid".to_string()]);
    }
    if (base.width, base.height, base.depth) == (payload.width, payload.height, payload.depth) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The grid is already {}×{}×{}.", payload.width, payload.height, payload.depth));
    }
    if let Some(cell) = base.pinned.iter().find(|cell| cell.x >= payload.width || cell.y >= payload.height || cell.z >= payload.depth) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Pinned cell {} would fall outside the resized grid.", cell_key(cell.x, cell.y, cell.z)), [cell_key(cell.x, cell.y, cell.z)]);
    }
    if let Some(cell) = base.masked.iter().find(|cell| cell.x >= payload.width || cell.y >= payload.height || cell.z >= payload.depth) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Masked cell {} would fall outside the resized grid.", cell_key(cell.x, cell.y, cell.z)), [cell_key(cell.x, cell.y, cell.z)]);
    }
    protocol::MutationOutcome::new(Grid3dDiff {
        width: Some(payload.width),
        height: Some(payload.height),
        depth: Some(payload.depth),
        cell_sizes_x: Some(resized_axis(&base.cell_sizes_x, payload.width)),
        cell_sizes_y: Some(resized_axis(&base.cell_sizes_y, payload.height)),
        cell_sizes_z: Some(resized_axis(&base.cell_sizes_z, payload.depth)),
        ..Default::default()
    })""",
    """    let _ = payload;
    vec![
        crate::mutations::resize_grid(base.width, base.height, base.depth),
        crate::mutations::change_cell_sizes(Grid3dAxis::X, base.cell_sizes_x.clone()),
        crate::mutations::change_cell_sizes(Grid3dAxis::Y, base.cell_sizes_y.clone()),
        crate::mutations::change_cell_sizes(Grid3dAxis::Z, base.cell_sizes_z.clone()),
    ]""",
    {"type": "object", "additionalProperties": False, "required": ["width", "height", "depth"],
     "properties": {"width": {"type": "integer", "minimum": 1}, "height": {"type": "integer", "minimum": 1},
                    "depth": {"type": "integer", "minimum": 1}}},
    outcome_classes=["info", "applied", "fatal"],
)

# ──────────────────────────────────────────────────────── 3. change-cell-sizes
kind(
    "📏️change-cell-sizes", "📏️", "change-cell-sizes", "Change Cell Sizes", "ChangeCellSizes", "change_cell_sizes",
    "📏 `s.wfc.grid3d` mutation — `ChangeCellSizes`: rewrites ONE axis' per-cell size array, the only\n//! authoring channel for the grid's non-uniformity. The array length must equal that axis' extent,\n//! so a size array can never drift out of step with `width`/`height`/`depth`.",
    "    pub axis: Grid3dAxis,\n    pub sizes: Vec<f64>,",
    "axis: Grid3dAxis, sizes: Vec<f64>", "ChangeCellSizes { axis, sizes }",
    '{ verb: "change", entity: "cell-sizes", kind: "change-cell-sizes", record: "ChangedCellSizes" }',
    'format!("Change {} cell sizes", self.axis.label())', None,
    """    let extent = payload.axis.extent(base) as usize;
    if payload.sizes.len() != extent {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Axis {} needs exactly {extent} cell sizes, got {}.", payload.axis.label(), payload.sizes.len()), [payload.axis.label().to_string()]);
    }
    if payload.sizes.iter().any(|size| !size.is_finite() || *size <= 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Axis {} cell sizes must all be finite and positive.", payload.axis.label()), [payload.axis.label().to_string()]);
    }
    if payload.axis.sizes(base) == &payload.sizes {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Axis {} already carries these cell sizes.", payload.axis.label()));
    }
    let sizes = Some(payload.sizes.clone());
    protocol::MutationOutcome::new(match payload.axis {
        Grid3dAxis::X => Grid3dDiff { cell_sizes_x: sizes, ..Default::default() },
        Grid3dAxis::Y => Grid3dDiff { cell_sizes_y: sizes, ..Default::default() },
        Grid3dAxis::Z => Grid3dDiff { cell_sizes_z: sizes, ..Default::default() },
    })""",
    "    vec![crate::mutations::change_cell_sizes(payload.axis, payload.axis.sizes(base).clone())]",
    {"type": "object", "additionalProperties": False, "required": ["axis", "sizes"],
     "properties": {"axis": {"type": "string", "enum": ["x", "y", "z"]},
                    "sizes": {"type": "array", "items": {"type": "number", "exclusiveMinimum": 0}}}},
    outcome_classes=["info", "applied", "fatal"],
)

# ─────────────────────────────────────────────────────── 4. change-periodicity
kind(
    "🔁️change-periodicity", "🔁️", "change-periodicity", "Change Periodicity", "ChangePeriodicity", "change_periodicity",
    "🔁 `s.wfc.grid3d` mutation — `ChangePeriodicity`: sets all three wrap flags at once. A periodic\n//! axis becomes `Boundary::Wrap` in the solve topology; a non-periodic one stays `Boundary::Open`.",
    "    pub periodic_x: bool,\n    pub periodic_y: bool,\n    pub periodic_z: bool,",
    "periodic_x: bool, periodic_y: bool, periodic_z: bool", "ChangePeriodicity { periodic_x, periodic_y, periodic_z }",
    '{ verb: "change", entity: "periodicity", kind: "change-periodicity", record: "ChangedPeriodicity" }',
    'format!("Change periodicity to {}/{}/{}", self.periodic_x, self.periodic_y, self.periodic_z)', None,
    """    if (base.periodic_x, base.periodic_y, base.periodic_z) == (payload.periodic_x, payload.periodic_y, payload.periodic_z) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The grid already carries this periodicity.".to_string());
    }
    protocol::MutationOutcome::new(Grid3dDiff { periodic_x: Some(payload.periodic_x), periodic_y: Some(payload.periodic_y), periodic_z: Some(payload.periodic_z), ..Default::default() })""",
    "    let _ = payload;\n    vec![crate::mutations::change_periodicity(base.periodic_x, base.periodic_y, base.periodic_z)]",
    {"type": "object", "additionalProperties": False, "required": ["periodicX", "periodicY", "periodicZ"],
     "properties": {"periodicX": {"type": "boolean"}, "periodicY": {"type": "boolean"}, "periodicZ": {"type": "boolean"}}},
    outcome_classes=["info", "applied"],
)

TILE_SCHEMA = {
    "title": "Grid3dTile", "type": "object", "additionalProperties": False,
    "required": ["id", "weight", "media"],
    "properties": {
        "id": {"type": "string"},
        "label": {"type": "string"},
        "weight": {"type": "number", "exclusiveMinimum": 0},
        "media": {"$comment": "Grid3dTileMedia — tagged by `kind`.", "type": "object"},
    },
}

# ────────────────────────────────────────────────────────────── 5. create-tile
kind(
    "🧱️create-tile", "🧱️", "create-tile", "Create Tile", "CreateTile", "create_tile",
    "🧱 `s.wfc.grid3d` mutation — `CreateTile`: brings one placeable tile into the pattern universe,\n//! inserted at its CANONICAL SORTED position so `delete-tile`'s inverse restores it in place.",
    "    pub tile: Grid3dTile,",
    "tile: Grid3dTile", "CreateTile { tile }",
    '{ verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" }',
    'format!("Create tile \\"{}\\"", self.tile.id)', "vec![self.tile.id.clone()]",
    """    if payload.tile.id.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A tile id cannot be empty.".to_string(), [String::new()]);
    }
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A tile with id \\"{}\\" already exists.", payload.tile.id), [payload.tile.id.clone()]);
    }
    if !payload.tile.weight.is_finite() || payload.tile.weight <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \\"{}\\" needs a finite positive weight.", payload.tile.id), [payload.tile.id.clone()]);
    }
    let at = crate::mutations::ordered_index(&base.tiles, &payload.tile.id, |tile| tile.id.clone());
    protocol::MutationOutcome::new(Grid3dDiff { tiles_upserted: vec![(at, payload.tile.clone())], ..Default::default() })""",
    "    let _ = base;\n    vec![crate::mutations::delete_tile(payload.tile.id.clone())]",
    {"type": "object", "additionalProperties": False, "required": ["tile"], "properties": {"tile": TILE_SCHEMA}},
    outcome_classes=["applied", "fatal"],
)

# ────────────────────────────────────────────────────────────── 6. delete-tile
kind(
    "🕳️delete-tile", "🕳️", "delete-tile", "Delete Tile", "DeleteTile", "delete_tile",
    "🕳 `s.wfc.grid3d` mutation — `DeleteTile`: removes one tile AND cascades every adjacency rule and\n//! every cell pin that named it, so the document never keeps a dangling tile reference. The inverse\n//! is base-derived and restores each cascaded row at its own canonical sorted position.",
    "    pub id: String,",
    "id: String", "DeleteTile { id }",
    '{ verb: "delete", entity: "tile", kind: "delete-tile", record: "DeletedTile" }',
    'format!("Delete tile \\"{}\\"", self.id)', "vec![self.id.clone()]",
    """    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("No tile with id \\"{}\\" exists.", payload.id), [payload.id.clone()]);
    }
    let rules_removed: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let pinned_removed: Vec<String> = base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| cell_key(cell.x, cell.y, cell.z)).collect();
    protocol::MutationOutcome::new(Grid3dDiff { tiles_removed: vec![payload.id.clone()], rules_removed, pinned_removed, ..Default::default() })""",
    """    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else { return Vec::new() };
    let mut steps = vec![crate::mutations::create_tile(tile.clone())];
    steps.extend(base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| crate::mutations::create_rule(rule.clone())));
    steps.extend(base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| crate::mutations::pin_cell(cell.clone())));
    steps""",
    {"type": "object", "additionalProperties": False, "required": ["id"], "properties": {"id": {"type": "string"}}},
    outcome_classes=["applied", "fatal"],
)

# ───────────────────────────────────────────────────── 7. change-tile-weight
kind(
    "⚖️change-tile-weight", "⚖️", "change-tile-weight", "Change Tile Weight", "ChangeTileWeight", "change_tile_weight",
    "⚖ `s.wfc.grid3d` mutation — `ChangeTileWeight`: retunes one tile's selection bias, the weight the\n//! solver's weighted-roulette sampler and the entropy inference both read.",
    "    pub tile_id: String,\n    pub weight: f64,",
    "tile_id: String, weight: f64", "ChangeTileWeight { tile_id, weight }",
    '{ verb: "change", entity: "tile-weight", kind: "change-tile-weight", record: "ChangedTileWeight" }',
    'format!("Change weight of tile \\"{}\\"", self.tile_id)', "vec![self.tile_id.clone()]",
    """    let Some(index) = tile_index(base, &payload.tile_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("No tile with id \\"{}\\" exists.", payload.tile_id), [payload.tile_id.clone()]);
    };
    if !payload.weight.is_finite() || payload.weight <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \\"{}\\" needs a finite positive weight.", payload.tile_id), [payload.tile_id.clone()]);
    }
    if base.tiles[index].weight == payload.weight {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \\"{}\\" already carries that weight.", payload.tile_id));
    }
    let mut tile = base.tiles[index].clone();
    tile.weight = payload.weight;
    protocol::MutationOutcome::new(Grid3dDiff { tiles_upserted: vec![(index, tile)], ..Default::default() })""",
    """    let Some(index) = tile_index(base, &payload.tile_id) else { return Vec::new() };
    vec![crate::mutations::change_tile_weight(payload.tile_id.clone(), base.tiles[index].weight)]""",
    {"type": "object", "additionalProperties": False, "required": ["tileId", "weight"],
     "properties": {"tileId": {"type": "string"}, "weight": {"type": "number", "exclusiveMinimum": 0}}},
    outcome_classes=["info", "applied", "fatal"],
)

# ────────────────────────────────────────────────────── 8. change-tile-media
kind(
    "🖼️change-tile-media", "🖼️", "change-tile-media", "Change Tile Media", "ChangeTileMedia", "change_tile_media",
    "🖼 `s.wfc.grid3d` mutation — `ChangeTileMedia`: swaps one tile's geometry, inline or composed-child,\n//! without disturbing its id, weight or any rule that names it.",
    "    pub tile_id: String,\n    pub media: Grid3dTileMedia,",
    "tile_id: String, media: Grid3dTileMedia", "ChangeTileMedia { tile_id, media }",
    '{ verb: "change", entity: "tile-media", kind: "change-tile-media", record: "ChangedTileMedia" }',
    'format!("Change media of tile \\"{}\\"", self.tile_id)', "vec![self.tile_id.clone()]",
    """    let Some(index) = tile_index(base, &payload.tile_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("No tile with id \\"{}\\" exists.", payload.tile_id), [payload.tile_id.clone()]);
    };
    if base.tiles[index].media == payload.media {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \\"{}\\" already carries that media.", payload.tile_id));
    }
    let mut tile = base.tiles[index].clone();
    tile.media = payload.media.clone();
    protocol::MutationOutcome::new(Grid3dDiff { tiles_upserted: vec![(index, tile)], ..Default::default() })""",
    """    let Some(index) = tile_index(base, &payload.tile_id) else { return Vec::new() };
    vec![crate::mutations::change_tile_media(payload.tile_id.clone(), base.tiles[index].media.clone())]""",
    {"type": "object", "additionalProperties": False, "required": ["tileId", "media"],
     "properties": {"tileId": {"type": "string"}, "media": {"$comment": "Grid3dTileMedia — tagged by `kind`.", "type": "object"}}},
    outcome_classes=["info", "applied", "fatal"],
)

RULE_SCHEMA = {
    "title": "Grid3dRule", "type": "object", "additionalProperties": False,
    "required": ["id", "tileAId", "tileBId", "direction", "allowed"],
    "properties": {
        "id": {"type": "string"},
        "tileAId": {"type": "string"},
        "tileBId": {"type": "string"},
        "direction": {"type": "string", "enum": ["LEFT", "RIGHT", "FRONT", "BACK", "BOTTOM", "TOP"]},
        "allowed": {"type": "boolean"},
    },
}

# ────────────────────────────────────────────────────────────── 9. create-rule
kind(
    "🚦️create-rule", "🚦️", "create-rule", "Create Rule", "CreateRule", "create_rule",
    "🚦 `s.wfc.grid3d` mutation — `CreateRule`: admits or denies one ordered tile pair across one of the\n//! six face directions. A pair no rule mentions for a direction is NOT allowed: the rule set is a\n//! closed allow-list, never a deny-list.",
    "    pub rule: Grid3dRule,",
    "rule: Grid3dRule", "CreateRule { rule }",
    '{ verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" }',
    'format!("Create rule \\"{}\\"", self.rule.id)', "vec![self.rule.id.clone()]",
    """    if payload.rule.id.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A rule id cannot be empty.".to_string(), [String::new()]);
    }
    if base.rules.iter().any(|rule| rule.id == payload.rule.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A rule with id \\"{}\\" already exists.", payload.rule.id), [payload.rule.id.clone()]);
    }
    for tile in [&payload.rule.tile_a_id, &payload.rule.tile_b_id] {
        if !base.tiles.iter().any(|candidate| &candidate.id == tile) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule \\"{}\\" names unknown tile \\"{tile}\\".", payload.rule.id), [tile.clone()]);
        }
    }
    let at = crate::mutations::ordered_index(&base.rules, &payload.rule.id, |rule| rule.id.clone());
    protocol::MutationOutcome::new(Grid3dDiff { rules_upserted: vec![(at, payload.rule.clone())], ..Default::default() })""",
    "    let _ = base;\n    vec![crate::mutations::delete_rule(payload.rule.id.clone())]",
    {"type": "object", "additionalProperties": False, "required": ["rule"], "properties": {"rule": RULE_SCHEMA}},
    outcome_classes=["applied", "fatal"],
)

# ───────────────────────────────────────────────────────────── 10. delete-rule
kind(
    "❌️delete-rule", "❌️", "delete-rule", "Delete Rule", "DeleteRule", "delete_rule",
    "❌ `s.wfc.grid3d` mutation — `DeleteRule`: drops one adjacency rule. Because the rule set is a\n//! closed allow-list, deleting an `allowed` rule NARROWS the solve rather than widening it.",
    "    pub id: String,",
    "id: String", "DeleteRule { id }",
    '{ verb: "delete", entity: "rule", kind: "delete-rule", record: "DeletedRule" }',
    'format!("Delete rule \\"{}\\"", self.id)', "vec![self.id.clone()]",
    """    if !base.rules.iter().any(|rule| rule.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("No rule with id \\"{}\\" exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Grid3dDiff { rules_removed: vec![payload.id.clone()], ..Default::default() })""",
    """    let Some(rule) = base.rules.iter().find(|rule| rule.id == payload.id) else { return Vec::new() };
    vec![crate::mutations::create_rule(rule.clone())]""",
    {"type": "object", "additionalProperties": False, "required": ["id"], "properties": {"id": {"type": "string"}}},
    outcome_classes=["applied", "fatal"],
)

PINNED_SCHEMA = {
    "title": "Grid3dPinnedCell", "type": "object", "additionalProperties": False,
    "required": ["x", "y", "z", "tileId"],
    "properties": {"x": {"type": "integer", "minimum": 0}, "y": {"type": "integer", "minimum": 0},
                   "z": {"type": "integer", "minimum": 0}, "tileId": {"type": "string"}},
}
CELL_SCHEMA = {
    "title": "Grid3dCell", "type": "object", "additionalProperties": False,
    "required": ["x", "y", "z"],
    "properties": {"x": {"type": "integer", "minimum": 0}, "y": {"type": "integer", "minimum": 0},
                   "z": {"type": "integer", "minimum": 0}},
}

# ──────────────────────────────────────────────────────────────── 11. pin-cell
kind(
    "📌️pin-cell", "📌️", "pin-cell", "Pin Cell", "PinCell", "pin_cell",
    "📌 `s.wfc.grid3d` mutation — `PinCell`: fixes one cell to one tile before the solve runs. A pin on\n//! a masked cell is refused (the cell is not in the topology at all), and a pin that replaces an\n//! existing one inverts back to the tile that was there.",
    "    pub pinned: Grid3dPinnedCell,",
    "pinned: Grid3dPinnedCell", "PinCell { pinned }",
    '{ verb: "fix", entity: "cell", kind: "pin-cell", record: "Fixed" }',
    'format!("Pin cell {} to tile \\"{}\\"", cell_key(self.pinned.x, self.pinned.y, self.pinned.z), self.pinned.tile_id)',
    "vec![cell_key(self.pinned.x, self.pinned.y, self.pinned.z)]",
    """    let key = cell_key(payload.pinned.x, payload.pinned.y, payload.pinned.z);
    if !cell_in_grid(base, payload.pinned.x, payload.pinned.y, payload.pinned.z) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} is outside the grid."), [key]);
    }
    if !base.tiles.iter().any(|tile| tile.id == payload.pinned.tile_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} pins unknown tile \\"{}\\".", payload.pinned.tile_id), [payload.pinned.tile_id.clone()]);
    }
    if masked_index(base, payload.pinned.x, payload.pinned.y, payload.pinned.z).is_some() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} is masked out and cannot be pinned."), [key]);
    }
    if base.pinned.iter().any(|cell| cell == &payload.pinned) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cell {key} already carries that pin."));
    }
    let at = crate::mutations::ordered_index(&base.pinned, &key, |cell| cell_key(cell.x, cell.y, cell.z));
    protocol::MutationOutcome::new(Grid3dDiff { pinned_upserted: vec![(at, payload.pinned.clone())], ..Default::default() })""",
    """    match pinned_index(base, payload.pinned.x, payload.pinned.y, payload.pinned.z) {
        Some(index) => vec![crate::mutations::pin_cell(base.pinned[index].clone())],
        None => vec![crate::mutations::unpin_cell(payload.pinned.x, payload.pinned.y, payload.pinned.z)],
    }""",
    {"type": "object", "additionalProperties": False, "required": ["pinned"], "properties": {"pinned": PINNED_SCHEMA}},
    outcome_classes=["info", "applied", "fatal"],
)

# ────────────────────────────────────────────────────────────── 12. unpin-cell
kind(
    "📍️unpin-cell", "📍️", "unpin-cell", "Unpin Cell", "UnpinCell", "unpin_cell",
    "📍 `s.wfc.grid3d` mutation — `UnpinCell`: releases one cell back to the full tile domain.",
    "    pub x: u32,\n    pub y: u32,\n    pub z: u32,",
    "x: u32, y: u32, z: u32", "UnpinCell { x, y, z }",
    '{ verb: "clear", entity: "cell", kind: "unpin-cell", record: "Cleared" }',
    'format!("Unpin cell {}", cell_key(self.x, self.y, self.z))',
    "vec![cell_key(self.x, self.y, self.z)]",
    """    let key = cell_key(payload.x, payload.y, payload.z);
    if pinned_index(base, payload.x, payload.y, payload.z).is_none() {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("Cell {key} carries no pin."), [key]);
    }
    protocol::MutationOutcome::new(Grid3dDiff { pinned_removed: vec![key], ..Default::default() })""",
    """    match pinned_index(base, payload.x, payload.y, payload.z) {
        Some(index) => vec![crate::mutations::pin_cell(base.pinned[index].clone())],
        None => Vec::new(),
    }""",
    {"type": "object", "additionalProperties": False, "required": ["x", "y", "z"],
     "properties": {"x": {"type": "integer", "minimum": 0}, "y": {"type": "integer", "minimum": 0},
                    "z": {"type": "integer", "minimum": 0}}},
    outcome_classes=["applied", "fatal"],
)

# ─────────────────────────────────────────────────────────────── 13. mask-cell
kind(
    "🚫️mask-cell", "🚫️", "mask-cell", "Mask Cell", "MaskCell", "mask_cell",
    "🚫 `s.wfc.grid3d` mutation — `MaskCell`: carves one cell out of the topology entirely, which is how\n//! a non-box shape is cut from the regular grid. A pinned cell is refused rather than silently\n//! unpinned, so the mutation stays atomic and point-invertible.",
    "    pub cell: Grid3dCell,",
    "cell: Grid3dCell", "MaskCell { cell }",
    '{ verb: "remove", entity: "cell", kind: "mask-cell", record: "Removed" }',
    'format!("Mask cell {}", cell_key(self.cell.x, self.cell.y, self.cell.z))',
    "vec![cell_key(self.cell.x, self.cell.y, self.cell.z)]",
    """    let key = cell_key(payload.cell.x, payload.cell.y, payload.cell.z);
    if !cell_in_grid(base, payload.cell.x, payload.cell.y, payload.cell.z) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} is outside the grid."), [key]);
    }
    if pinned_index(base, payload.cell.x, payload.cell.y, payload.cell.z).is_some() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} is pinned; unpin it before masking."), [key]);
    }
    if masked_index(base, payload.cell.x, payload.cell.y, payload.cell.z).is_some() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cell {key} is already masked."));
    }
    let at = crate::mutations::ordered_index(&base.masked, &key, |cell| cell_key(cell.x, cell.y, cell.z));
    protocol::MutationOutcome::new(Grid3dDiff { masked_upserted: vec![(at, payload.cell)], ..Default::default() })""",
    "    let _ = base;\n    vec![crate::mutations::unmask_cell(payload.cell.x, payload.cell.y, payload.cell.z)]",
    {"type": "object", "additionalProperties": False, "required": ["cell"], "properties": {"cell": CELL_SCHEMA}},
    outcome_classes=["info", "applied", "fatal"],
)

# ───────────────────────────────────────────────────────────── 14. unmask-cell
kind(
    "🔓️unmask-cell", "🔓️", "unmask-cell", "Unmask Cell", "UnmaskCell", "unmask_cell",
    "🔓 `s.wfc.grid3d` mutation — `UnmaskCell`: puts one carved-out cell back into the topology.",
    "    pub x: u32,\n    pub y: u32,\n    pub z: u32,",
    "x: u32, y: u32, z: u32", "UnmaskCell { x, y, z }",
    '{ verb: "restore", entity: "cell", kind: "unmask-cell", record: "Restored" }',
    'format!("Unmask cell {}", cell_key(self.x, self.y, self.z))',
    "vec![cell_key(self.x, self.y, self.z)]",
    """    let key = cell_key(payload.x, payload.y, payload.z);
    if masked_index(base, payload.x, payload.y, payload.z).is_none() {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("Cell {key} is not masked."), [key]);
    }
    protocol::MutationOutcome::new(Grid3dDiff { masked_removed: vec![key], ..Default::default() })""",
    """    match masked_index(base, payload.x, payload.y, payload.z) {
        Some(index) => vec![crate::mutations::mask_cell(base.masked[index])],
        None => Vec::new(),
    }""",
    {"type": "object", "additionalProperties": False, "required": ["x", "y", "z"],
     "properties": {"x": {"type": "integer", "minimum": 0}, "y": {"type": "integer", "minimum": 0},
                    "z": {"type": "integer", "minimum": 0}}},
    outcome_classes=["applied", "fatal"],
)


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


for entry in KINDS:
    base = os.path.join(MUT, entry["dirname"])
    target_fn = f"""
    fn target(&self) -> Vec<String> {{
        {entry["target"]}
    }}""" if entry["target"] else ""
    write(os.path.join(base, "🦀️.rs"), f"""//! {entry["doc"]}

{HEAD}
//#region 🔖️{entry["variant"]}
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct {entry["variant"]} {{
{entry["fields"]}
}}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn {entry["module"]}({entry["builder_args"]}) -> Grid3dMutation {{
    Grid3dMutation::{entry["variant"]}({entry["builder_body"]})
}}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for {entry["variant"]} {{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor {entry["semantics"]};

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {{
        super::diff::diff(self, base)
    }}
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {{
        super::inverse::inverse(self, base)
    }}
    fn label(&self) -> String {{
        {entry["label"]}
    }}{target_fn}
}}
//#endregion 🔖️{entry["variant"]}
""")

    write(os.path.join(base, "🔺️diff", "🦀️.rs"), f"""//! 🔺️ Sparse diff builder for `{entry["variant"]}` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::{entry["variant"]}, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {{
{entry["diff_body"]}
}}
""")

    write(os.path.join(base, "↩️inverse", "🦀️.rs"), f"""//! ↩️ Inverse for `{entry["variant"]}` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::{entry["variant"]}, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {{
{entry["inverse_body"]}
}}
""")

    manifest = {
        "schemaVersion": 1,
        "owner": f"{OWNER}/{entry['dirname']}",
        "semanticKind": entry["semantic"],
        "displayName": entry["display"],
        "emoji": entry["emoji"],
        "aggregateVariant": entry["variant"],
        "payloadSchema": "🧬️schema/🔣️.json",
        "textOpcode": None,
        "binaryTag": None,
        "invertibility": "explicit-mutation",
        "diffParticipation": "detect",
        "outcomeClasses": entry["outcome_classes"],
        "composition": "atomic",
        "requiredLanguageSurfaces": ["rust", "json-schema"],
    }
    write(os.path.join(base, "🔣️.json"), json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")

    payload_schema = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": f"https://json.schemas.assets.semio-tech.com/s/wfc/grid3d/1/any/mutation/{entry['semantic']}/schema.json",
        "title": entry["variant"],
    }
    payload_schema.update(entry["payload_schema"])
    write(os.path.join(base, "🧬️schema", "🔣️.json"), json.dumps(payload_schema, ensure_ascii=False, indent=2) + "\n")

print(f"wrote {len(KINDS)} mutation folders under {MUT}")
for entry in KINDS:
    print(f"  {entry['dirname']} -> {entry['module']}")
