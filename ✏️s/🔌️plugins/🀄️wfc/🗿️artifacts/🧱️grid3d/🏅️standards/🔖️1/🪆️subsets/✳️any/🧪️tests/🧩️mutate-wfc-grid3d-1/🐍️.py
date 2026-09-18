#!/usr/bin/env python3
"""🐍 `s.wfc.grid3d` reference implementation — a SECOND, independent implementation of the mutation
semantics, written against the normative JSON Schema rather than ported from the Rust. It replays
every committed fixture quintet beside it and is the oracle the `🥒️.feature` table names.

Stdlib only, read-only: no `jsonschema`, no `pytest`, no network. Run it from anywhere:

    python3 ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-wfc-grid3d-1/🐍️.py
"""

import copy
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SUBSET = os.path.abspath(os.path.join(HERE, "..", ".."))
FIXTURES = os.path.join(SUBSET, "🧫️fixtures", "🧬️mutations")

KINDS = [
    "change-seed",
    "resize-grid",
    "change-cell-sizes",
    "change-periodicity",
    "create-tile",
    "delete-tile",
    "change-tile-weight",
    "change-tile-media",
    "create-rule",
    "delete-rule",
    "pin-cell",
    "unpin-cell",
    "mask-cell",
    "unmask-cell",
]

DIRECTORIES = {
    "change-seed": "🎲️change-seed",
    "resize-grid": "📐️resize-grid",
    "change-cell-sizes": "📏️change-cell-sizes",
    "change-periodicity": "🔁️change-periodicity",
    "create-tile": "🧱️create-tile",
    "delete-tile": "🕳️delete-tile",
    "change-tile-weight": "⚖️change-tile-weight",
    "change-tile-media": "🖼️change-tile-media",
    "create-rule": "🚦️create-rule",
    "delete-rule": "❌️delete-rule",
    "pin-cell": "📌️pin-cell",
    "unpin-cell": "📍️unpin-cell",
    "mask-cell": "🚫️mask-cell",
    "unmask-cell": "🔓️unmask-cell",
}

VARIANTS = {
    "ChangeSeed": "change-seed",
    "ResizeGrid": "resize-grid",
    "ChangeCellSizes": "change-cell-sizes",
    "ChangePeriodicity": "change-periodicity",
    "CreateTile": "create-tile",
    "DeleteTile": "delete-tile",
    "ChangeTileWeight": "change-tile-weight",
    "ChangeTileMedia": "change-tile-media",
    "CreateRule": "create-rule",
    "DeleteRule": "delete-rule",
    "PinCell": "pin-cell",
    "UnpinCell": "unpin-cell",
    "MaskCell": "mask-cell",
    "UnmaskCell": "unmask-cell",
}


def cell_key(cell):
    return "{}:{}:{}".format(cell["x"], cell["y"], cell["z"])


def ordered_index(items, key, item_key):
    """📐 Where `key` belongs in an already-sorted collection: the EXISTING slot when the key is
    present, else the sorted insertion point. Never an append."""
    for index, item in enumerate(items):
        if item_key(item) >= key:
            return index
    return len(items)


def resized_axis(sizes, extent):
    fill = sizes[-1] if sizes else 1.0
    out = list(sizes[:extent])
    while len(out) < extent:
        out.append(fill)
    return out


def empty_diff():
    return {
        "schema": None,
        "seed": None,
        "width": None,
        "height": None,
        "depth": None,
        "cellSizesX": None,
        "cellSizesY": None,
        "cellSizesZ": None,
        "periodicX": None,
        "periodicY": None,
        "periodicZ": None,
        "tilesRemoved": [],
        "tilesUpserted": [],
        "rulesRemoved": [],
        "rulesUpserted": [],
        "pinnedRemoved": [],
        "pinnedUpserted": [],
        "maskedRemoved": [],
        "maskedUpserted": [],
    }


def diff_for(kind, payload, base):
    """🔺 The sparse delta one mutation produces — the whole semantics of this artifact, restated."""
    diff = empty_diff()
    if kind == "change-seed":
        diff["seed"] = payload["seed"]
    elif kind == "resize-grid":
        diff["width"] = payload["width"]
        diff["height"] = payload["height"]
        diff["depth"] = payload["depth"]
        diff["cellSizesX"] = resized_axis(base["cellSizesX"], payload["width"])
        diff["cellSizesY"] = resized_axis(base["cellSizesY"], payload["height"])
        diff["cellSizesZ"] = resized_axis(base["cellSizesZ"], payload["depth"])
    elif kind == "change-cell-sizes":
        diff["cellSizes" + payload["axis"].upper()] = list(payload["sizes"])
    elif kind == "change-periodicity":
        diff["periodicX"] = payload["periodicX"]
        diff["periodicY"] = payload["periodicY"]
        diff["periodicZ"] = payload["periodicZ"]
    elif kind == "create-tile":
        tile = payload["tile"]
        diff["tilesUpserted"] = [[ordered_index(base["tiles"], tile["id"], lambda item: item["id"]), tile]]
    elif kind == "delete-tile":
        target = payload["id"]
        diff["tilesRemoved"] = [target]
        diff["rulesRemoved"] = [rule["id"] for rule in base["rules"] if target in (rule["tileAId"], rule["tileBId"])]
        diff["pinnedRemoved"] = [cell_key(cell) for cell in base["pinned"] if cell["tileId"] == target]
    elif kind == "change-tile-weight":
        index, tile = find(base["tiles"], lambda item: item["id"] == payload["tileId"])
        tile = copy.deepcopy(tile)
        tile["weight"] = payload["weight"]
        diff["tilesUpserted"] = [[index, tile]]
    elif kind == "change-tile-media":
        index, tile = find(base["tiles"], lambda item: item["id"] == payload["tileId"])
        tile = copy.deepcopy(tile)
        tile["media"] = payload["media"]
        diff["tilesUpserted"] = [[index, tile]]
    elif kind == "create-rule":
        rule = payload["rule"]
        diff["rulesUpserted"] = [[ordered_index(base["rules"], rule["id"], lambda item: item["id"]), rule]]
    elif kind == "delete-rule":
        diff["rulesRemoved"] = [payload["id"]]
    elif kind == "pin-cell":
        pinned = payload["pinned"]
        diff["pinnedUpserted"] = [[ordered_index(base["pinned"], cell_key(pinned), cell_key), pinned]]
    elif kind == "unpin-cell":
        diff["pinnedRemoved"] = [cell_key(payload)]
    elif kind == "mask-cell":
        cell = payload["cell"]
        diff["maskedUpserted"] = [[ordered_index(base["masked"], cell_key(cell), cell_key), cell]]
    elif kind == "unmask-cell":
        diff["maskedRemoved"] = [cell_key(payload)]
    else:
        raise AssertionError("unknown kind " + kind)
    return diff


def find(items, predicate):
    for index, item in enumerate(items):
        if predicate(item):
            return index, item
    raise AssertionError("no member matches")


def apply_collection(base, removed, upserted, item_key):
    items = [item for item in base if item_key(item) not in removed]
    for index, value in upserted:
        at = next((slot for slot, item in enumerate(items) if item_key(item) == item_key(value)), None)
        if at is None:
            items.insert(index, value)
        else:
            items[at] = value
    return items


def apply_diff(base, diff):
    out = copy.deepcopy(base)
    for scalar in ("schema", "seed", "width", "height", "depth", "cellSizesX", "cellSizesY", "cellSizesZ", "periodicX", "periodicY", "periodicZ"):
        if diff[scalar] is not None:
            out[scalar] = diff[scalar]
    out["tiles"] = apply_collection(out["tiles"], diff["tilesRemoved"], diff["tilesUpserted"], lambda item: item["id"])
    out["rules"] = apply_collection(out["rules"], diff["rulesRemoved"], diff["rulesUpserted"], lambda item: item["id"])
    out["pinned"] = apply_collection(out["pinned"], diff["pinnedRemoved"], diff["pinnedUpserted"], cell_key)
    out["masked"] = apply_collection(out["masked"], diff["maskedRemoved"], diff["maskedUpserted"], cell_key)
    return out


def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def replay():
    problems = []
    replayed = 0
    for kind in KINDS:
        root = os.path.join(FIXTURES, DIRECTORIES[kind])
        if not os.path.isdir(root):
            problems.append("{}: no fixture directory at {}".format(kind, root))
            continue
        cases = sorted(entry for entry in os.listdir(root) if not entry.startswith("."))
        if not cases:
            problems.append("{}: no committed case".format(kind))
        for case in cases:
            case_root = os.path.join(root, case)
            before = read(os.path.join(case_root, "📸️snapshot", "⬅️before", "🔣️.json"))
            after = read(os.path.join(case_root, "📸️snapshot", "➡️after", "🔣️.json"))
            mutation = read(os.path.join(case_root, "🦠️mutation", "🔣️.json"))
            committed_diff = read(os.path.join(case_root, "🔺️diff", "🔣️.json"))
            outcome = read(os.path.join(case_root, "🎯️outcome", "🔣️.json"))
            variant = next(iter(mutation))
            if VARIANTS.get(variant) != kind:
                problems.append("{}/{}: the committed mutation is a {}".format(kind, case, variant))
                continue
            produced = diff_for(kind, mutation[variant], before)
            if produced != committed_diff:
                problems.append("{}/{}: produced diff differs from the committed one".format(kind, case))
            if apply_diff(before, committed_diff) != after:
                problems.append("{}/{}: the committed diff does not carry before to after".format(kind, case))
            if outcome.get("status") != "applied":
                problems.append("{}/{}: every committed vector declares the applied status".format(kind, case))
            replayed += 1
    return replayed, problems


def main():
    replayed, problems = replay()
    for problem in problems:
        print("✗ " + problem)
    print("replayed {} vector(s) across {} kinds".format(replayed, len(KINDS)))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
