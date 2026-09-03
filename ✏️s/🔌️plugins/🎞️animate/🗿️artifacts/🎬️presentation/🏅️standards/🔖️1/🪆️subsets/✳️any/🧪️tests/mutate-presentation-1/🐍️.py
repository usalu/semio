"""🐍️ `s.animate.presentation`'s second, independent implementation of its own nine-kind mutation
vocabulary.

`s.animate.presentation` is a semio-NATIVE artifact — the `animate.presentation.dsl` envelope is
defined by this repository alone and no package in any ecosystem reads it. The reference is
therefore a second IMPLEMENTATION, written from this subset's own committed
`../../🧬️schema/📸️snapshot/🔣️.json` and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`resize`/`replace`/`create`/`delete`/`rename`/`reorder` verb entries and `📓️derivation-rules.md`'s
per-ordered-collection recipe. It imports nothing from the Rust it judges and transliterates none of
it.

🗂️ SHAPE. `tiles` are read from `../../../🧪️tests/mutate-presentation-1/🧫️fixtures/🔣️.json` — this
case's own committed, derived-once base (the SAME `local://🔣️.json` this feature's `Given` step
declares). `source`, by contrast, is NOT committed anywhere this file can read: it is whatever the
real `.dsl.semio` example decodes to through production's own parser, which this reference does not
reimplement.

⚠️ Honest boundary. `resize-source-frame` and `replace-source` are therefore modelled with `source`
as an OPAQUE marker rather than a real value: applying either kind changes the marker (so `differs
from base` holds, honestly, on the fact that the field was TOUCHED) and the inverse restores the
PRIOR marker — this verifies the identity-of-touch, not the real frame/source content, and that
limitation is stated here rather than concealed. The seven `tiles`-scoped kinds are verified for
real: transcribed VERBATIM from this feature's own committed `Examples` `params` column (committed,
checked-in material), applied to the real committed `tiles` base, and checked both forward (`differs
from base`) and backward (`inverse restores base exactly`).
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
BASE_URI = "local://🔣️.json"

#: 🧫️ Transcribed verbatim from this feature's `Examples` `params` column.
PARAMS = {
    "resize-source-frame": {"ResizeSourceFrame": {"newFrame": {"x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0}}},
    "replace-source": {"ReplaceSource": {"newSource": {"src": "/fixture-deck.png", "kind": "figure", "frame": {"x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0}, "sourceAspect": 1.5, "pdfPage": None}}},
    "create-tile": {"CreateTile": {"index": 1, "tile": {"id": "t-macro", "name": "Macro", "crop": {"x": 0.4, "y": 0.4, "width": 0.2, "height": 0.2}}}},
    "delete-tile": {"DeleteTile": {"id": "t-hero"}},
    "delete-tiles": {"DeleteTiles": {"ids": ["t-alpha", "t-omega"]}},
    "rename-tile": {"RenameTile": {"id": "t-hero", "newName": "Lead"}},
    "resize-tile-crop": {"ResizeTileCrop": {"id": "t-hero", "newCrop": {"x": 0.3, "y": 0.3, "width": 0.4, "height": 0.4}}},
    "reorder-tiles": {"ReorderTiles": {"id": "t-hero", "toIndex": 2}},
    "replace-tiles": {"ReplaceTiles": {"newTiles": []}},
}

SOURCE_SCOPED = {"resize-source-frame", "replace-source"}


def _read_base(ctx: Context):
    tiles = json.loads(ctx.fixture_bytes(BASE_URI))["tiles"]
    return {"source": "BASE", "tiles": tiles}
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 The EXTERNALLY-tagged form this vocabulary uses (`PresentationMutation` declares no
    `#[serde(tag)]`), with camelCase payload fields — `{"<Variant>": {...}}`."""
    if isinstance(wire, dict) and len(wire) == 1:
        tag = next(iter(wire))
        if isinstance(wire[tag], dict):
            return tag, wire[tag]
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))
# endregion 🔖️Wire


# region 🔖️Tree
def tile_index(graph, tid):
    return next((i for i, t in enumerate(graph["tiles"]) if t["id"] == tid), None)
# endregion 🔖️Tree


# region 🔖️Vocabulary
def apply_resize_source_frame(graph, payload):
    after = copy.deepcopy(graph)
    after["source"] = ("TOUCHED", json.dumps(payload["newFrame"], sort_keys=True))
    return after


def apply_replace_source(graph, payload):
    after = copy.deepcopy(graph)
    after["source"] = ("TOUCHED", json.dumps(payload["newSource"], sort_keys=True))
    return after


def apply_create_tile(graph, payload):
    after = copy.deepcopy(graph)
    after["tiles"].insert(payload["index"], copy.deepcopy(payload["tile"]))
    return after


def apply_delete_tile(graph, payload):
    after = copy.deepcopy(graph)
    after["tiles"] = [t for t in after["tiles"] if t["id"] != payload["id"]]
    return after


def apply_delete_tiles(graph, payload):
    after = copy.deepcopy(graph)
    ids = set(payload["ids"])
    after["tiles"] = [t for t in after["tiles"] if t["id"] not in ids]
    return after


def apply_rename_tile(graph, payload):
    after = copy.deepcopy(graph)
    after["tiles"][tile_index(after, payload["id"])]["name"] = payload["newName"]
    return after


def apply_resize_tile_crop(graph, payload):
    after = copy.deepcopy(graph)
    after["tiles"][tile_index(after, payload["id"])]["crop"] = copy.deepcopy(payload["newCrop"])
    return after


def apply_reorder_tiles(graph, payload):
    after = copy.deepcopy(graph)
    current = tile_index(after, payload["id"])
    clamped = min(payload["toIndex"], len(after["tiles"]) - 1)
    item = after["tiles"].pop(current)
    after["tiles"].insert(clamped, item)
    return after


def apply_replace_tiles(graph, payload):
    after = copy.deepcopy(graph)
    after["tiles"] = copy.deepcopy(payload["newTiles"])
    return after


APPLIERS = {
    "resize-source-frame": apply_resize_source_frame,
    "replace-source": apply_replace_source,
    "create-tile": apply_create_tile,
    "delete-tile": apply_delete_tile,
    "delete-tiles": apply_delete_tiles,
    "rename-tile": apply_rename_tile,
    "resize-tile-crop": apply_resize_tile_crop,
    "reorder-tiles": apply_reorder_tiles,
    "replace-tiles": apply_replace_tiles,
}


def inverse_apply(kind, base_graph, payload, mutated_graph):
    """↩️ Every inverse is computed from BASE, never from the diff."""
    after = copy.deepcopy(mutated_graph)
    if kind == "resize-source-frame" or kind == "replace-source":
        after["source"] = base_graph["source"]
        return after
    if kind == "create-tile":
        after["tiles"] = [t for t in after["tiles"] if t["id"] != payload["tile"]["id"]]
        return after
    if kind == "delete-tile":
        tid = payload["id"]
        original_index = tile_index(base_graph, tid)
        after["tiles"].insert(original_index, copy.deepcopy(next(t for t in base_graph["tiles"] if t["id"] == tid)))
        return after
    if kind == "delete-tiles":
        for tid in payload["ids"]:
            original_index = tile_index(base_graph, tid)
            if original_index is not None:
                after["tiles"].insert(original_index, copy.deepcopy(next(t for t in base_graph["tiles"] if t["id"] == tid)))
        return after
    if kind == "rename-tile":
        old_name = next(t for t in base_graph["tiles"] if t["id"] == payload["id"])["name"]
        after["tiles"][tile_index(after, payload["id"])]["name"] = old_name
        return after
    if kind == "resize-tile-crop":
        old_crop = next(t for t in base_graph["tiles"] if t["id"] == payload["id"])["crop"]
        after["tiles"][tile_index(after, payload["id"])]["crop"] = copy.deepcopy(old_crop)
        return after
    if kind == "reorder-tiles":
        original_index = tile_index(base_graph, payload["id"])
        current = tile_index(after, payload["id"])
        clamped = min(original_index, len(after["tiles"]) - 1)
        item = after["tiles"].pop(current)
        after["tiles"].insert(clamped, item)
        return after
    if kind == "replace-tiles":
        after["tiles"] = copy.deepcopy(base_graph["tiles"])
        return after
    raise AssertionError(f"no inverse rule for kind {kind!r}")
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        base = _read_base(ctx)
        after = APPLIERS[kind](base, list(PARAMS[kind].values())[0])
        assert after != base, f"mutate-{kind}: the mutation must move the projection, but it produced the base graph unchanged"
        payload_bytes = json.dumps(after, sort_keys=True, separators=(",", ":"), default=str).encode("utf-8")
        return Outcome(projection=after, raw=payload_bytes)
    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        base = _read_base(ctx)
        payload = list(PARAMS[kind].values())[0]
        mutated = APPLIERS[kind](base, payload)
        restored = inverse_apply(kind, base, payload, mutated)
        assert restored == base, f"inverse-{kind}: {restored} != committed base graph {base}"
        payload_bytes = json.dumps(restored, sort_keys=True, separators=(",", ":"), default=str).encode("utf-8")
        return Outcome(projection=restored, raw=payload_bytes)
    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only."""
    built = Adapter("python")
    for kind in PARAMS:
        built = built.oracle(f"mutate-{kind}", _mutate_for(kind)).oracle(f"inverse-{kind}", _inverse_for(kind))
    return built
# endregion 🔖️Registration
