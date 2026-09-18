#!/usr/bin/env python3
"""🐍️ Emits the wfc2d mutation triads (payload/diff/inverse/descriptor/payload-schema), one fixture
quintet + mounted test per kind, and the crate-root `#[path]` mount block for them. Idempotent: it
always overwrites the files it owns, so re-running after a spec edit is the supported workflow."""
from __future__ import annotations
import json, os, pathlib, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
ART = ROOT / "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d"
SUB = ART / "🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = SUB / "🧬️schema/🧬️mutations"
FIX = SUB / "🧫️fixtures/🧬️mutations"
OWNER_PREFIX = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

HEAD = """use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
"""


def write(path: pathlib.Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def j(value) -> str:
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


# ── 🎨️ shared tile media ────────────────────────────────────────────────────────────────────────
def color(r, g, b, a=255):
    return {"r": r, "g": g, "b": b, "a": a}


def vector(segments, fill):
    return {"Vector": {"paths": [{"segments": segments, "fill": fill, "strokeWidth": 0.0}]}}


ROOF_MEDIA = vector(
    [{"Move": {"to": [0.0, 1.0]}}, {"Line": {"to": [0.5, 0.0]}}, {"Line": {"to": [1.0, 1.0]}}, "Close"],
    color(206, 84, 62),
)
WALL_MEDIA = vector(
    [{"Move": {"to": [0.0, 0.0]}}, {"Line": {"to": [1.0, 0.0]}}, {"Line": {"to": [1.0, 1.0]}}, {"Line": {"to": [0.0, 1.0]}}, "Close"],
    color(122, 126, 134),
)
WINDOW_MEDIA = vector(
    [{"Move": {"to": [0.2, 0.2]}}, {"Line": {"to": [0.8, 0.2]}}, {"Line": {"to": [0.8, 0.8]}}, {"Line": {"to": [0.2, 0.8]}}, "Close"],
    color(96, 165, 250),
)
# 🖼️ A 4×4 palette-indexed RASTER tile: the `change-tile-media` vector deliberately carries the
# `Bitmap` branch, so the committed quintet, the Python oracle and the TypeScript twin all exercise
# the media variant the preview windows encode to a PNG data URL.
ROOF_MEDIA_REPAINTED = {
    "Bitmap": {
        "width": 4,
        "height": 4,
        "palette": [color(30, 64, 175), color(96, 165, 250)],
        "pixels": "AAAAAAAAAAABAQEBAQEBAQ==",
    }
}


def slot(ident, x, y, w=2.0, h=2.0, pinned=None):
    row = {"id": ident, "x": x, "y": y, "width": w, "height": h}
    if pinned is not None:
        row["pinnedTileId"] = pinned
    return row


def edge(ident, a, b, relation="adjacent"):
    return {"id": ident, "fromSlotId": a, "toSlotId": b, "relation": relation}


def tile(ident, weight, media):
    return {"id": ident, "weight": weight, "media": media}


def rule(ident, a, b, allowed, relation=None):
    row = {"id": ident, "tileAId": a, "tileBId": b}
    if relation is not None:
        row["relation"] = relation
    row["allowed"] = allowed
    return row


def base_scene():
    return {
        "schema": "s.wfc.wfc2d",
        "seed": 7,
        "slots": [slot("slot-a", 0.0, 0.0), slot("slot-b", 2.0, 0.0), slot("slot-c", 4.0, 0.0, pinned="tile-wall")],
        "edges": [edge("edge-ab", "slot-a", "slot-b"), edge("edge-bc", "slot-b", "slot-c")],
        "tiles": [tile("tile-roof", 1.0, ROOF_MEDIA), tile("tile-wall", 2.0, WALL_MEDIA)],
        "rules": [rule("rule-roof-wall", "tile-roof", "tile-wall", True), rule("rule-wall-wall", "tile-wall", "tile-wall", True)],
    }


EMPTY_DIFF = {
    "schema": None,
    "seed": None,
    "slotsRemoved": [],
    "slotsUpserted": [],
    "edgesRemoved": [],
    "edgesUpserted": [],
    "tilesRemoved": [],
    "tilesUpserted": [],
    "rulesRemoved": [],
    "rulesUpserted": [],
}


def diff_of(**overrides):
    row = dict(EMPTY_DIFF)
    row.update(overrides)
    return row


# ── 🧬️ the fifteen kinds ────────────────────────────────────────────────────────────────────────
KINDS: list[dict] = []


def kind(dirname, semantic, variant, emoji, display, payload_rs, schema_props, required, diff_rs, inverse_rs, doc, case, mutation_json, after, diff_json, outcome, outcome_classes=None, sem=None):
    KINDS.append(
        dict(
            dirname=dirname,
            semantic=semantic,
            variant=variant,
            emoji=emoji,
            display=display,
            payload_rs=payload_rs,
            schema_props=schema_props,
            required=required,
            diff_rs=diff_rs,
            inverse_rs=inverse_rs,
            doc=doc,
            case=case,
            mutation_json=mutation_json,
            after=after,
            diff_json=diff_json,
            outcome=outcome,
            outcome_classes=outcome_classes or ["applied"],
            sem=sem,
        )
    )


SLOT_SCHEMA = {
    "title": "Wfc2dSlot",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "x", "y", "width", "height"],
    "properties": {
        "id": {"type": "string"},
        "x": {"type": "number"},
        "y": {"type": "number"},
        "width": {"type": "number", "exclusiveMinimum": 0},
        "height": {"type": "number", "exclusiveMinimum": 0},
        "pinnedTileId": {"type": "string"},
    },
}
EDGE_SCHEMA = {
    "title": "Wfc2dSlotEdge",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "fromSlotId", "toSlotId", "relation"],
    "properties": {"id": {"type": "string"}, "fromSlotId": {"type": "string"}, "toSlotId": {"type": "string"}, "relation": {"type": "string"}},
}
COLOR_SCHEMA = {
    "title": "Wfc2dColor",
    "type": "object",
    "additionalProperties": False,
    "required": ["r", "g", "b", "a"],
    "properties": {k: {"type": "integer", "minimum": 0, "maximum": 255} for k in "rgba"},
}
MEDIA_SCHEMA = {
    "title": "Wfc2dTileMedia",
    "type": "object",
    "additionalProperties": True,
    "description": "Externally tagged: exactly one of Empty (the bare string), Bitmap, Vector or Image.",
}
TILE_SCHEMA = {
    "title": "Wfc2dTile",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "weight"],
    "properties": {"id": {"type": "string"}, "label": {"type": "string"}, "weight": {"type": "number", "minimum": 0}, "media": MEDIA_SCHEMA},
}
RULE_SCHEMA = {
    "title": "Wfc2dRule",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "tileAId", "tileBId", "allowed"],
    "properties": {"id": {"type": "string"}, "tileAId": {"type": "string"}, "tileBId": {"type": "string"}, "relation": {"type": "string"}, "allowed": {"type": "boolean"}},
}

# 🎲 change-seed ------------------------------------------------------------------------------
after = base_scene()
after["seed"] = 99
kind(
    "🎲️change-seed", "change-seed", "ChangeSeed", "🎲", "Change Seed",
    """pub struct ChangeSeed {
    pub seed: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_seed(seed: u64) -> Wfc2dMutation {
    Wfc2dMutation::ChangeSeed(ChangeSeed { seed })
}""",
    {"seed": {"type": "integer", "minimum": 0}}, ["seed"],
    """pub fn diff(payload: &super::ChangeSeed, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.seed == payload.seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seed is already {}.", payload.seed));
    }
    protocol::MutationOutcome::new(Wfc2dDiff { seed: Some(payload.seed), ..Default::default() })
}""",
    """pub fn inverse(_payload: &super::ChangeSeed, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![change_seed(base.seed)]
}""",
    "🎲 WFC 2D mutation — `ChangeSeed`: sets the deterministic solve seed. PERSISTED snapshot field,\n//! authored ONLY here — never ambient — so the solve inference's `DepHash` caching stays sound.",
    "🎲️reseeds-the-solve-from-7-to-99",
    {"ChangeSeed": {"seed": 99}}, after, diff_of(seed=99), {"status": "applied"},
    outcome_classes=["info", "applied"],
    sem=("change", "seed", "ChangedSeed"),
)

# 🧩 create-slot -----------------------------------------------------------------------------
after = base_scene()
after["slots"].append(slot("slot-d", 6.0, 0.0))
kind(
    "🧩️create-slot", "create-slot", "CreateSlot", "🧩", "Create Slot",
    """pub struct CreateSlot {
    pub slot: Wfc2dSlot,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_slot(slot: Wfc2dSlot) -> Wfc2dMutation {
    Wfc2dMutation::CreateSlot(CreateSlot { slot })
}""",
    {"slot": SLOT_SCHEMA}, ["slot"],
    """pub fn diff(payload: &super::CreateSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.slots.iter().any(|slot| slot.id == payload.slot.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A slot with id \\"{}\\" already exists.", payload.slot.id), [payload.slot.id.clone()]);
    }
    if payload.slot.width <= 0.0 || payload.slot.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \\"{}\\" must have a positive width and height.", payload.slot.id), [payload.slot.id.clone()]);
    }
    if let Some(pinned) = &payload.slot.pinned_tile_id {
        if !base.tiles.iter().any(|tile| &tile.id == pinned) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \\"{}\\" pins unknown tile \\"{}\\".", payload.slot.id, pinned), [pinned.clone()]);
        }
    }
    let at = crate::mutations::ordered_index(&base.slots, &payload.slot.id, |slot| slot.id.as_str());
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(at, payload.slot.clone())], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::CreateSlot, _base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![delete_slot(payload.slot.id.clone())]
}""",
    "🧩 WFC 2D mutation — `CreateSlot`: brings a new id-keyed slot into existence at its CANONICAL\n//! ascending-id position, never at the end, so `delete-slot`'s inverse restores the same index.",
    "🧩️inserts-slot-d-in-canonical-order",
    {"CreateSlot": {"slot": slot("slot-d", 6.0, 0.0)}}, after,
    diff_of(slotsUpserted=[[3, slot("slot-d", 6.0, 0.0)]]), {"status": "applied"},
    sem=("create", "slot", "CreatedSlot"),
)

# 🕳 delete-slot -----------------------------------------------------------------------------
after = base_scene()
after["slots"] = [s for s in after["slots"] if s["id"] != "slot-a"]
after["edges"] = [e for e in after["edges"] if e["id"] != "edge-ab"]
kind(
    "🕳️delete-slot", "delete-slot", "DeleteSlot", "🕳", "Delete Slot",
    """pub struct DeleteSlot {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_slot(id: String) -> Wfc2dMutation {
    Wfc2dMutation::DeleteSlot(DeleteSlot { id })
}""",
    {"id": {"type": "string"}}, ["id"],
    """pub fn diff(payload: &super::DeleteSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if !base.slots.iter().any(|slot| slot.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let incident: Vec<String> = base.edges.iter().filter(|edge| edge.from_slot_id == payload.id || edge.to_slot_id == payload.id).map(|edge| edge.id.clone()).collect();
    let outcome = protocol::MutationOutcome::new(Wfc2dDiff { slots_removed: vec![payload.id.clone()], edges_removed: incident.clone(), ..Default::default() });
    if incident.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting slot \\"{}\\" also removed {} connected edge(s): {}.", payload.id, incident.len(), incident.join(", ")))
    }
}""",
    """pub fn inverse(payload: &super::DeleteSlot, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    let mut restore = vec![create_slot(slot.clone())];
    for edge in &base.edges {
        if edge.from_slot_id == payload.id || edge.to_slot_id == payload.id {
            restore.push(connect_slots(edge.clone()));
        }
    }
    restore
}""",
    "🕳 WFC 2D mutation — `DeleteSlot`: removes a slot AND cascades every edge incident to it, so the\n//! adjacency graph never keeps a dangling endpoint.",
    "🚫️removes-slot-a-and-cascades-edge-ab",
    {"DeleteSlot": {"id": "slot-a"}}, after,
    diff_of(slotsRemoved=["slot-a"], edgesRemoved=["edge-ab"]),
    {"status": "applied", "messages": [{"level": "info", "code": "mutation.cascade"}]},
    outcome_classes=["error", "info", "applied"],
    sem=("delete", "slot", "DeletedSlot"),
)

# ↔ move-slot --------------------------------------------------------------------------------
after = base_scene()
after["slots"][1] = slot("slot-b", 2.0, 3.0)
moved = slot("slot-b", 2.0, 3.0)
kind(
    "↔️move-slot", "move-slot", "MoveSlot", "↔", "Move Slot",
    """pub struct MoveSlot {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_slot(id: String, x: f64, y: f64) -> Wfc2dMutation {
    Wfc2dMutation::MoveSlot(MoveSlot { id, x, y })
}""",
    {"id": {"type": "string"}, "x": {"type": "number"}, "y": {"type": "number"}}, ["id", "x", "y"],
    """pub fn diff(payload: &super::MoveSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let slot = &base.slots[index];
    if slot.x == payload.x && slot.y == payload.y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Slot \\"{}\\" is already at that position.", payload.id));
    }
    let moved = crate::schema::snapshot::Wfc2dSlot { x: payload.x, y: payload.y, ..slot.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(index, moved)], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::MoveSlot, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    vec![move_slot(payload.id.clone(), slot.x, slot.y)]
}""",
    "↔ WFC 2D mutation — `MoveSlot`: the settled end of a node drag in the `wfc-graph` window. One\n//! mutation per GESTURE, never per pointer tick — mid-drag frames ride the window transient.",
    "↔️drags-slot-b-down",
    {"MoveSlot": {"id": "slot-b", "x": 2.0, "y": 3.0}}, after,
    diff_of(slotsUpserted=[[1, moved]]), {"status": "applied"},
    outcome_classes=["error", "info", "applied"],
    sem=("move", "slot", "MovedSlot"),
)

# 📐 resize-slot -----------------------------------------------------------------------------
after = base_scene()
after["slots"][1] = slot("slot-b", 2.0, 0.0, 4.0, 3.0)
resized = slot("slot-b", 2.0, 0.0, 4.0, 3.0)
kind(
    "📐️resize-slot", "resize-slot", "ResizeSlot", "📐", "Resize Slot",
    """pub struct ResizeSlot {
    pub id: String,
    pub width: f64,
    pub height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_slot(id: String, width: f64, height: f64) -> Wfc2dMutation {
    Wfc2dMutation::ResizeSlot(ResizeSlot { id, width, height })
}""",
    {"id": {"type": "string"}, "width": {"type": "number", "exclusiveMinimum": 0}, "height": {"type": "number", "exclusiveMinimum": 0}}, ["id", "width", "height"],
    """pub fn diff(payload: &super::ResizeSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.width <= 0.0 || payload.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slot \\"{}\\" must keep a positive width and height.", payload.id), [payload.id.clone()]);
    }
    let slot = &base.slots[index];
    if slot.width == payload.width && slot.height == payload.height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Slot \\"{}\\" already has that size.", payload.id));
    }
    let resized = crate::schema::snapshot::Wfc2dSlot { width: payload.width, height: payload.height, ..slot.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(index, resized)], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::ResizeSlot, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    vec![resize_slot(payload.id.clone(), slot.width, slot.height)]
}""",
    "📐 WFC 2D mutation — `ResizeSlot`: changes the rectangle a slot occupies. The tile media is drawn\n//! SCALED into that rectangle, so this is also how a tile's on-canvas size is authored.",
    "📐️widens-slot-b",
    {"ResizeSlot": {"id": "slot-b", "width": 4.0, "height": 3.0}}, after,
    diff_of(slotsUpserted=[[1, resized]]), {"status": "applied"},
    outcome_classes=["error", "info", "applied"],
    sem=("resize", "slot", "ResizedSlot"),
)

# 🔗 connect-slots ---------------------------------------------------------------------------
after = base_scene()
after["edges"].insert(1, edge("edge-ac", "slot-a", "slot-c"))
kind(
    "🔗️connect-slots", "connect-slots", "ConnectSlots", "🔗", "Connect Slots",
    """pub struct ConnectSlots {
    pub edge: Wfc2dSlotEdge,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_slots(edge: Wfc2dSlotEdge) -> Wfc2dMutation {
    Wfc2dMutation::ConnectSlots(ConnectSlots { edge })
}""",
    {"edge": EDGE_SCHEMA}, ["edge"],
    """pub fn diff(payload: &super::ConnectSlots, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.edges.iter().any(|edge| edge.id == payload.edge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \\"{}\\" already exists.", payload.edge.id), [payload.edge.id.clone()]);
    }
    for endpoint in [&payload.edge.from_slot_id, &payload.edge.to_slot_id] {
        if !base.slots.iter().any(|slot| &slot.id == endpoint) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \\"{}\\" references unknown slot \\"{}\\".", payload.edge.id, endpoint), [endpoint.clone()]);
        }
    }
    if payload.edge.relation.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \\"{}\\" must name a relation.", payload.edge.id), [payload.edge.id.clone()]);
    }
    let at = crate::mutations::ordered_index(&base.edges, &payload.edge.id, |edge| edge.id.as_str());
    protocol::MutationOutcome::new(Wfc2dDiff { edges_upserted: vec![(at, payload.edge.clone())], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::ConnectSlots, _base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![disconnect_slots(payload.edge.id.clone())]
}""",
    "🔗 WFC 2D mutation — `ConnectSlots`: the settled end of a connect gesture in the `wfc-graph`\n//! window. The edge's `relation` names the adjacency class its rules are scoped to.",
    "🔗️joins-slot-a-to-slot-c",
    {"ConnectSlots": {"edge": edge("edge-ac", "slot-a", "slot-c")}}, after,
    diff_of(edgesUpserted=[[1, edge("edge-ac", "slot-a", "slot-c")]]), {"status": "applied"},
    sem=("connect", "slots", "ConnectedSlots"),
)

# ✂ disconnect-slots -------------------------------------------------------------------------
after = base_scene()
after["edges"] = [e for e in after["edges"] if e["id"] != "edge-ab"]
kind(
    "✂️disconnect-slots", "disconnect-slots", "DisconnectSlots", "✂", "Disconnect Slots",
    """pub struct DisconnectSlots {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_slots(id: String) -> Wfc2dMutation {
    Wfc2dMutation::DisconnectSlots(DisconnectSlots { id })
}""",
    {"id": {"type": "string"}}, ["id"],
    """pub fn diff(payload: &super::DisconnectSlots, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if !base.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc2dDiff { edges_removed: vec![payload.id.clone()], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::DisconnectSlots, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(edge) = base.edges.iter().find(|edge| edge.id == payload.id) else {
        return Vec::new();
    };
    vec![connect_slots(edge.clone())]
}""",
    "✂ WFC 2D mutation — `DisconnectSlots`: severs one adjacency edge, leaving both slots in place.",
    "✂️severs-edge-ab",
    {"DisconnectSlots": {"id": "edge-ab"}}, after,
    diff_of(edgesRemoved=["edge-ab"]), {"status": "applied"},
    outcome_classes=["error", "applied"],
    sem=("disconnect", "slots", "DisconnectedSlots"),
)

# 📌 pin-slot --------------------------------------------------------------------------------
after = base_scene()
after["slots"][0] = slot("slot-a", 0.0, 0.0, pinned="tile-roof")
pinned_a = slot("slot-a", 0.0, 0.0, pinned="tile-roof")
kind(
    "📌️pin-slot", "pin-slot", "PinSlot", "📌", "Pin Slot",
    """pub struct PinSlot {
    pub id: String,
    pub tile_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_slot(id: String, tile_id: String) -> Wfc2dMutation {
    Wfc2dMutation::PinSlot(PinSlot { id, tile_id })
}""",
    {"id": {"type": "string"}, "tileId": {"type": "string"}}, ["id", "tileId"],
    """pub fn diff(payload: &super::PinSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !base.tiles.iter().any(|tile| tile.id == payload.tile_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \\"{}\\" does not exist.", payload.tile_id), [payload.tile_id.clone()]);
    }
    let slot = &base.slots[index];
    if slot.pinned_tile_id.as_deref() == Some(payload.tile_id.as_str()) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Slot \\"{}\\" is already pinned to \\"{}\\".", payload.id, payload.tile_id));
    }
    let pinned = crate::schema::snapshot::Wfc2dSlot { pinned_tile_id: Some(payload.tile_id.clone()), ..slot.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(index, pinned)], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::PinSlot, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    match &slot.pinned_tile_id {
        Some(previous) => vec![pin_slot(payload.id.clone(), previous.clone())],
        None => vec![unpin_slot(payload.id.clone())],
    }
}""",
    "📌 WFC 2D mutation — `PinSlot`: hard-assigns one tile to one slot. A pin is a DOMAIN RESTRICTION\n//! the solve must respect, never a solved assignment written back into the document.",
    "📌️pins-slot-a-to-the-roof-tile",
    {"PinSlot": {"id": "slot-a", "tileId": "tile-roof"}}, after,
    diff_of(slotsUpserted=[[0, pinned_a]]), {"status": "applied"},
    outcome_classes=["error", "info", "applied"],
    sem=("fix", "slot", "FixedSlot"),
)

# 🔓 unpin-slot ------------------------------------------------------------------------------
after = base_scene()
after["slots"][2] = slot("slot-c", 4.0, 0.0)
unpinned_c = slot("slot-c", 4.0, 0.0)
kind(
    "🔓️unpin-slot", "unpin-slot", "UnpinSlot", "🔓", "Unpin Slot",
    """pub struct UnpinSlot {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_slot(id: String) -> Wfc2dMutation {
    Wfc2dMutation::UnpinSlot(UnpinSlot { id })
}""",
    {"id": {"type": "string"}}, ["id"],
    """pub fn diff(payload: &super::UnpinSlot, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let slot = &base.slots[index];
    if slot.pinned_tile_id.is_none() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Slot \\"{}\\" carries no pin.", payload.id));
    }
    let released = crate::schema::snapshot::Wfc2dSlot { pinned_tile_id: None, ..slot.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { slots_upserted: vec![(index, released)], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::UnpinSlot, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    match &slot.pinned_tile_id {
        Some(previous) => vec![pin_slot(payload.id.clone(), previous.clone())],
        None => Vec::new(),
    }
}""",
    "🔓 WFC 2D mutation — `UnpinSlot`: releases a slot's hard pre-assignment back to the solver.",
    "🔓️releases-the-slot-c-pin",
    {"UnpinSlot": {"id": "slot-c"}}, after,
    diff_of(slotsUpserted=[[2, unpinned_c]]), {"status": "applied"},
    outcome_classes=["error", "info", "applied"],
    sem=("clear", "slot", "ClearedSlot"),
)

# 🀄 create-tile -----------------------------------------------------------------------------
after = base_scene()
after["tiles"].append(tile("tile-window", 1.5, WINDOW_MEDIA))
kind(
    "🀄️create-tile", "create-tile", "CreateTile", "🀄", "Create Tile",
    """pub struct CreateTile {
    pub tile: Wfc2dTile,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_tile(tile: Wfc2dTile) -> Wfc2dMutation {
    Wfc2dMutation::CreateTile(CreateTile { tile })
}""",
    {"tile": TILE_SCHEMA}, ["tile"],
    """pub fn diff(payload: &super::CreateTile, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A tile with id \\"{}\\" already exists.", payload.tile.id), [payload.tile.id.clone()]);
    }
    if !payload.tile.weight.is_finite() || payload.tile.weight < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \\"{}\\" needs a finite non-negative weight.", payload.tile.id), [payload.tile.id.clone()]);
    }
    let at = crate::mutations::ordered_index(&base.tiles, &payload.tile.id, |tile| tile.id.as_str());
    protocol::MutationOutcome::new(Wfc2dDiff { tiles_upserted: vec![(at, payload.tile.clone())], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::CreateTile, _base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![delete_tile(payload.tile.id.clone())]
}""",
    "🀄 WFC 2D mutation — `CreateTile`: adds one pattern to the alphabet the solve draws from, with\n//! its own inline media and selection weight.",
    "🀄️adds-the-window-tile",
    {"CreateTile": {"tile": tile("tile-window", 1.5, WINDOW_MEDIA)}}, after,
    diff_of(tilesUpserted=[[2, tile("tile-window", 1.5, WINDOW_MEDIA)]]), {"status": "applied"},
    sem=("create", "tile", "CreatedTile"),
)

# 🗑 delete-tile -----------------------------------------------------------------------------
after = base_scene()
after["tiles"] = [t for t in after["tiles"] if t["id"] != "tile-wall"]
after["rules"] = []
after["slots"][2] = slot("slot-c", 4.0, 0.0)
kind(
    "🗑️delete-tile", "delete-tile", "DeleteTile", "🗑", "Delete Tile",
    """pub struct DeleteTile {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_tile(id: String) -> Wfc2dMutation {
    Wfc2dMutation::DeleteTile(DeleteTile { id })
}""",
    {"id": {"type": "string"}}, ["id"],
    """pub fn diff(payload: &super::DeleteTile, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let orphaned_rules: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let released: Vec<(usize, crate::schema::snapshot::Wfc2dSlot)> = base
        .slots
        .iter()
        .enumerate()
        .filter(|(_, slot)| slot.pinned_tile_id.as_deref() == Some(payload.id.as_str()))
        .map(|(index, slot)| (index, crate::schema::snapshot::Wfc2dSlot { pinned_tile_id: None, ..slot.clone() }))
        .collect();
    let cascaded = orphaned_rules.len() + released.len();
    let outcome = protocol::MutationOutcome::new(Wfc2dDiff { tiles_removed: vec![payload.id.clone()], rules_removed: orphaned_rules.clone(), slots_upserted: released.clone(), ..Default::default() });
    if cascaded == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting tile \\"{}\\" also removed {} rule(s) and released {} pin(s).", payload.id, orphaned_rules.len(), released.len()))
    }
}""",
    """pub fn inverse(payload: &super::DeleteTile, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    let mut restore = vec![create_tile(tile.clone())];
    for rule in &base.rules {
        if rule.tile_a_id == payload.id || rule.tile_b_id == payload.id {
            restore.push(create_rule(rule.clone()));
        }
    }
    for slot in &base.slots {
        if slot.pinned_tile_id.as_deref() == Some(payload.id.as_str()) {
            restore.push(pin_slot(slot.id.clone(), payload.id.clone()));
        }
    }
    restore
}""",
    "🗑 WFC 2D mutation — `DeleteTile`: removes one pattern from the alphabet AND cascades every rule\n//! that named it plus every slot pin that held it, so no reference is ever left dangling.",
    "🚫️removes-the-wall-tile-and-cascades-rules-and-pins",
    {"DeleteTile": {"id": "tile-wall"}}, after,
    diff_of(tilesRemoved=["tile-wall"], rulesRemoved=["rule-roof-wall", "rule-wall-wall"], slotsUpserted=[[2, slot("slot-c", 4.0, 0.0)]]),
    {"status": "applied", "messages": [{"level": "info", "code": "mutation.cascade"}]},
    outcome_classes=["error", "info", "applied"],
    sem=("delete", "tile", "DeletedTile"),
)

# ⚖ change-tile-weight -----------------------------------------------------------------------
after = base_scene()
after["tiles"][1] = tile("tile-wall", 5.0, WALL_MEDIA)
kind(
    "⚖️change-tile-weight", "change-tile-weight", "ChangeTileWeight", "⚖", "Change Tile Weight",
    """pub struct ChangeTileWeight {
    pub tile_id: String,
    pub weight: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_weight(tile_id: String, weight: f64) -> Wfc2dMutation {
    Wfc2dMutation::ChangeTileWeight(ChangeTileWeight { tile_id, weight })
}""",
    {"tileId": {"type": "string"}, "weight": {"type": "number", "minimum": 0}}, ["tileId", "weight"],
    """pub fn diff(payload: &super::ChangeTileWeight, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.tile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \\"{}\\" does not exist.", payload.tile_id), [payload.tile_id.clone()]);
    };
    if !payload.weight.is_finite() || payload.weight < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \\"{}\\" needs a finite non-negative weight.", payload.tile_id), [payload.tile_id.clone()]);
    }
    let tile = &base.tiles[index];
    if tile.weight == payload.weight {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \\"{}\\" already has that weight.", payload.tile_id));
    }
    let reweighted = crate::schema::snapshot::Wfc2dTile { weight: payload.weight, ..tile.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { tiles_upserted: vec![(index, reweighted)], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::ChangeTileWeight, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.tile_id) else {
        return Vec::new();
    };
    vec![change_tile_weight(payload.tile_id.clone(), tile.weight)]
}""",
    "⚖ WFC 2D mutation — `ChangeTileWeight`: retunes one tile's selection bias, the `WeightTable`\n//! input the engine's Shannon-entropy heuristic reads.",
    "⚖️raises-the-wall-tile-bias",
    {"ChangeTileWeight": {"tileId": "tile-wall", "weight": 5.0}}, after,
    diff_of(tilesUpserted=[[1, tile("tile-wall", 5.0, WALL_MEDIA)]]), {"status": "applied"},
    outcome_classes=["error", "info", "applied"],
    sem=("change", "tile-weight", "ChangedTileWeight"),
)

# 🎨 change-tile-media -----------------------------------------------------------------------
after = base_scene()
after["tiles"][0] = tile("tile-roof", 1.0, ROOF_MEDIA_REPAINTED)
kind(
    "🎨️change-tile-media", "change-tile-media", "ChangeTileMedia", "🎨", "Change Tile Media",
    """pub struct ChangeTileMedia {
    pub tile_id: String,
    pub media: Wfc2dTileMedia,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_media(tile_id: String, media: Wfc2dTileMedia) -> Wfc2dMutation {
    Wfc2dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media })
}""",
    {"tileId": {"type": "string"}, "media": MEDIA_SCHEMA}, ["tileId", "media"],
    """pub fn diff(payload: &super::ChangeTileMedia, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.tile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \\"{}\\" does not exist.", payload.tile_id), [payload.tile_id.clone()]);
    };
    let tile = &base.tiles[index];
    if tile.media == payload.media {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \\"{}\\" already carries that media.", payload.tile_id));
    }
    let repainted = crate::schema::snapshot::Wfc2dTile { media: payload.media.clone(), ..tile.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { tiles_upserted: vec![(index, repainted)], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::ChangeTileMedia, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.tile_id) else {
        return Vec::new();
    };
    vec![change_tile_media(payload.tile_id.clone(), tile.media.clone())]
}""",
    "🎨 WFC 2D mutation — `ChangeTileMedia`: replaces what a tile LOOKS like without touching its id,\n//! weight or any rule that names it — including swapping a vector tile for a raster one, which is\n//! what this kind's committed vector does.",
    "🎨️repaints-the-roof-tile-as-a-raster",
    {"ChangeTileMedia": {"tileId": "tile-roof", "media": ROOF_MEDIA_REPAINTED}}, after,
    diff_of(tilesUpserted=[[0, tile("tile-roof", 1.0, ROOF_MEDIA_REPAINTED)]]), {"status": "applied"},
    outcome_classes=["error", "info", "applied"],
    sem=("change", "tile-media", "ChangedTileMedia"),
)

# 🚦 create-rule -----------------------------------------------------------------------------
after = base_scene()
after["rules"].insert(0, rule("rule-roof-roof", "tile-roof", "tile-roof", False))
kind(
    "🚦️create-rule", "create-rule", "CreateRule", "🚦", "Create Rule",
    """pub struct CreateRule {
    pub rule: Wfc2dRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rule(rule: Wfc2dRule) -> Wfc2dMutation {
    Wfc2dMutation::CreateRule(CreateRule { rule })
}""",
    {"rule": RULE_SCHEMA}, ["rule"],
    """pub fn diff(payload: &super::CreateRule, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.rules.iter().any(|rule| rule.id == payload.rule.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A rule with id \\"{}\\" already exists.", payload.rule.id), [payload.rule.id.clone()]);
    }
    for tile_id in [&payload.rule.tile_a_id, &payload.rule.tile_b_id] {
        if !base.tiles.iter().any(|tile| &tile.id == tile_id) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rule \\"{}\\" references unknown tile \\"{}\\".", payload.rule.id, tile_id), [tile_id.clone()]);
        }
    }
    let at = crate::mutations::ordered_index(&base.rules, &payload.rule.id, |rule| rule.id.as_str());
    protocol::MutationOutcome::new(Wfc2dDiff { rules_upserted: vec![(at, payload.rule.clone())], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::CreateRule, _base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![delete_rule(payload.rule.id.clone())]
}""",
    "🚦 WFC 2D mutation — `CreateRule`: declares one tile pair allowed or forbidden, optionally only\n//! across one relation class. A deny always beats an allow of the same pair at compile time.",
    "⛔️forbids-roof-over-roof",
    {"CreateRule": {"rule": rule("rule-roof-roof", "tile-roof", "tile-roof", False)}}, after,
    diff_of(rulesUpserted=[[0, rule("rule-roof-roof", "tile-roof", "tile-roof", False)]]), {"status": "applied"},
    sem=("create", "rule", "CreatedRule"),
)

# ❌ delete-rule -----------------------------------------------------------------------------
after = base_scene()
after["rules"] = [r for r in after["rules"] if r["id"] != "rule-wall-wall"]
kind(
    "❌delete-rule", "delete-rule", "DeleteRule", "❌", "Delete Rule",
    """pub struct DeleteRule {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_rule(id: String) -> Wfc2dMutation {
    Wfc2dMutation::DeleteRule(DeleteRule { id })
}""",
    {"id": {"type": "string"}}, ["id"],
    """pub fn diff(payload: &super::DeleteRule, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if !base.rules.iter().any(|rule| rule.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rule \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc2dDiff { rules_removed: vec![payload.id.clone()], ..Default::default() })
}""",
    """pub fn inverse(payload: &super::DeleteRule, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    let Some(rule) = base.rules.iter().find(|rule| rule.id == payload.id) else {
        return Vec::new();
    };
    vec![create_rule(rule.clone())]
}""",
    "❌ WFC 2D mutation — `DeleteRule`: withdraws one adjacency permission.",
    "🚫️removes-the-wall-wall-rule",
    {"DeleteRule": {"id": "rule-wall-wall"}}, after,
    diff_of(rulesRemoved=["rule-wall-wall"]), {"status": "applied"},
    outcome_classes=["error", "applied"],
    sem=("delete", "rule", "DeletedRule"),
)

# ── 🖨️ emit ────────────────────────────────────────────────────────────────────────────────────
SNAKE = {
    "change-seed": "change_seed",
    "create-slot": "create_slot",
    "delete-slot": "delete_slot",
    "move-slot": "move_slot",
    "resize-slot": "resize_slot",
    "connect-slots": "connect_slots",
    "disconnect-slots": "disconnect_slots",
    "pin-slot": "pin_slot",
    "unpin-slot": "unpin_slot",
    "create-tile": "create_tile",
    "delete-tile": "delete_tile",
    "change-tile-weight": "change_tile_weight",
    "change-tile-media": "change_tile_media",
    "create-rule": "create_rule",
    "delete-rule": "delete_rule",
}

PAYLOAD_TYPE_IMPORTS = {
    "create-slot": "use crate::schema::snapshot::Wfc2dSlot;\n",
    "connect-slots": "use crate::schema::snapshot::Wfc2dSlotEdge;\n",
    "create-tile": "use crate::schema::snapshot::Wfc2dTile;\n",
    "change-tile-media": "use crate::schema::snapshot::Wfc2dTileMedia;\n",
    "create-rule": "use crate::schema::snapshot::Wfc2dRule;\n",
}

INVERSE_IMPORTS = {
    "change-seed": "use crate::mutations::{change_seed, Wfc2dMutation};",
    "create-slot": "use crate::mutations::{delete_slot, Wfc2dMutation};",
    "delete-slot": "use crate::mutations::{connect_slots, create_slot, Wfc2dMutation};",
    "move-slot": "use crate::mutations::{move_slot, Wfc2dMutation};",
    "resize-slot": "use crate::mutations::{resize_slot, Wfc2dMutation};",
    "connect-slots": "use crate::mutations::{disconnect_slots, Wfc2dMutation};",
    "disconnect-slots": "use crate::mutations::{connect_slots, Wfc2dMutation};",
    "pin-slot": "use crate::mutations::{pin_slot, unpin_slot, Wfc2dMutation};",
    "unpin-slot": "use crate::mutations::{pin_slot, Wfc2dMutation};",
    "create-tile": "use crate::mutations::{delete_tile, Wfc2dMutation};",
    "delete-tile": "use crate::mutations::{create_rule, create_tile, pin_slot, Wfc2dMutation};",
    "change-tile-weight": "use crate::mutations::{change_tile_weight, Wfc2dMutation};",
    "change-tile-media": "use crate::mutations::{change_tile_media, Wfc2dMutation};",
    "create-rule": "use crate::mutations::{delete_rule, Wfc2dMutation};",
    "delete-rule": "use crate::mutations::{create_rule, Wfc2dMutation};",
}


def emit():
    mounts: list[str] = []
    for spec in KINDS:
        d = MUT / spec["dirname"]
        semantic = spec["semantic"]
        variant = spec["variant"]
        verb, entity, record = spec["sem"]
        extra = PAYLOAD_TYPE_IMPORTS.get(semantic, "")
        write(
            d / "🦀️.rs",
            f"""//! {spec['doc']}

{HEAD}{extra}
//#region 🔖️{variant}
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
{spec['payload_rs']}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for {variant} {{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor {{ verb: "{verb}", entity: "{entity}", kind: "{semantic}", record: "{record}" }};

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {{
        super::diff::diff(self, base)
    }}
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {{
        super::inverse::inverse(self, base)
    }}
    fn label(&self) -> String {{
        "{spec['display']}".into()
    }}
}}
//#endregion 🔖️{variant}
""",
        )
        write(
            d / "🔺️diff/🦀️.rs",
            f"""//! 🔺️ Sparse diff builder for `{variant}` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

{spec['diff_rs']}
""",
        )
        write(
            d / "↩️inverse/🦀️.rs",
            f"""//! ↩️ Inverse for `{variant}` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

{INVERSE_IMPORTS[semantic]}
use crate::schema::snapshot::Wfc2dSnapshot;

{spec['inverse_rs']}
""",
        )
        write(
            d / "🔣️.json",
            j(
                {
                    "schemaVersion": 1,
                    "owner": f"{OWNER_PREFIX}/{spec['dirname']}",
                    "semanticKind": semantic,
                    "displayName": spec["display"],
                    "emoji": spec["emoji"],
                    "aggregateVariant": variant,
                    "payloadSchema": "🧬️schema/🔣️.json",
                    "textOpcode": semantic,
                    "binaryTag": None,
                    "invertibility": "explicit-mutation",
                    "diffParticipation": "detect",
                    "outcomeClasses": spec["outcome_classes"],
                    "composition": "atomic",
                    "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
                }
            ),
        )
        write(
            d / "🧬️schema/🔣️.json",
            j(
                {
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "$id": f"https://json.schemas.assets.semio-tech.com/s/wfc/wfc2d/1/any/mutation/{semantic}/schema.json",
                    "title": variant,
                    "type": "object",
                    "additionalProperties": False,
                    "required": spec["required"],
                    "properties": spec["schema_props"],
                }
            ),
        )

        case = spec["case"]
        fixture = FIX / spec["dirname"] / case
        write(fixture / "📸️snapshot/⬅️before/🔣️.json", j(base_scene()))
        write(fixture / "📸️snapshot/➡️after/🔣️.json", j(spec["after"]))
        write(fixture / "🦠️mutation/🔣️.json", j(spec["mutation_json"]))
        write(fixture / "🎯️outcome/🔣️.json", j(spec["outcome"]))
        write(fixture / "🔺️diff/🔣️.json", j(spec["diff_json"]))

        rel = "../../../../../🧫️fixtures/🧬️mutations/" + spec["dirname"] + "/" + case
        write(
            d / "🧪️tests" / case / "🦀️.rs",
            f"""//! 🧪️ `{semantic}` fixture — `{case}`.
//!
//! Source of truth is the committed JSON quintet beside this file: the mutation is decoded from it,
//! applied, inverted, and its produced diff compared field for field against the committed delta.

use crate::diff::Wfc2dDiff;
use crate::mutations::{{apply_wfc2d_mutation, inverse_wfc2d_mutation, Wfc2dMutation}};
use crate::schema::snapshot::Wfc2dSnapshot;

const BEFORE: &str = include_str!("{rel}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("{rel}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("{rel}/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("{rel}/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("{rel}/🎯️outcome/🔣️.json");

fn before() -> Wfc2dSnapshot {{
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Wfc2dSnapshot {{
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Wfc2dMutation {{
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    let mut snapshot = before();
    apply_wfc2d_mutation(&mut snapshot, &mutation()).expect("mutation applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{semantic}/{case}: applied state differs from the committed after-snapshot");
}}

/// ↩️ Applying the mutation then its inverse restores `before` exactly — VALUE and POSITION.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_wfc2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_wfc2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_wfc2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{semantic}/{case}: inverse did not restore the before-snapshot");
}}

/// 🔣️ Both committed snapshots and the mutation are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Wfc2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{semantic}/{case}: committed {{side}} JSON is not canonical");
    }}
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{semantic}/{case}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome — status AND every diagnostic the diff builder raises — matches reality.
#[test]
fn declared_outcome_holds() {{
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <Wfc2dMutation as protocol::Mutation<Wfc2dSnapshot>>::diff(&mutation(), &before());
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
    let applied = apply_wfc2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {{
        "applied" => {{
            assert!(applied, "{semantic}/{case}: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "{semantic}/{case}: declared applied but the snapshot came back unchanged");
        }}
        "rejected" => {{
            assert_eq!(snapshot, before(), "{semantic}/{case}: a rejected mutation must leave the snapshot untouched");
        }}
        other => panic!("{semantic}/{case}: unknown outcome status {{other:?}}"),
    }}
}}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins WHICH
/// collections and fields the kind is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {{
    let base = before();
    let raised = <Wfc2dMutation as protocol::Mutation<Wfc2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{semantic}/{case}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: Wfc2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{semantic}/{case}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: Wfc2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <Wfc2dDiff as protocol::MutationDiff<Wfc2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{semantic}/{case}: committed diff did not carry before to after");
}}
""",
        )

        snake = SNAKE[semantic]
        base_path = f"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{spec['dirname']}"
        test_mod = "tests_" + case.split("️", 1)[-1].replace("-", "_")
        mounts.append(
            f"""                        #[path = "."]
                        pub mod {snake} {{
                            #[path = "{base_path}/🦀️.rs"]
                            mod component;
                            #[path = "{base_path}/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "{base_path}/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "{base_path}/🧪️tests/{case}/🦀️.rs"]
                            mod {test_mod};
                        }}"""
        )
    (ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/EXTRACT-WFC-PLUGIN/🗑️generated/wfc2d/mount-block.txt").write_text("\n".join(mounts) + "\n", encoding="utf-8")
    print(f"emitted {len(KINDS)} mutation kinds + fixtures; mount block written")


if __name__ == "__main__":
    emit()
