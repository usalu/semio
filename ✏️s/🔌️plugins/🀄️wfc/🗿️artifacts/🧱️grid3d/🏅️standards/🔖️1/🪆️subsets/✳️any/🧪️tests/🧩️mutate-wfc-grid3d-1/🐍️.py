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


def inverse(base, kind, payload):
    """↩️ The reference's OWN inverse — a list of steps of this same vocabulary, computed against `base`, never read
    from a fixture. Every collection insert lands at its canonical sorted position, so a removed row comes back
    exactly where it was; a resize also restores the per-axis cell sizes it truncated or filled."""
    if kind == "change-seed":
        return [{"ChangeSeed": {"seed": base["seed"]}}]
    if kind == "resize-grid":
        steps = [{"ResizeGrid": {"width": base["width"], "height": base["height"], "depth": base["depth"]}}]
        return steps + [{"ChangeCellSizes": {"axis": axis, "sizes": list(base["cellSizes" + axis.upper()])}} for axis in ("x", "y", "z")]
    if kind == "change-cell-sizes":
        return [{"ChangeCellSizes": {"axis": payload["axis"], "sizes": list(base["cellSizes" + payload["axis"].upper()])}}]
    if kind == "change-periodicity":
        return [{"ChangePeriodicity": {"periodicX": base["periodicX"], "periodicY": base["periodicY"], "periodicZ": base["periodicZ"]}}]
    if kind == "create-tile":
        return [{"DeleteTile": {"id": payload["tile"]["id"]}}]
    if kind == "delete-tile":
        _, tile = find(base["tiles"], lambda item: item["id"] == payload["id"])
        steps = [{"CreateTile": {"tile": tile}}]
        steps += [{"CreateRule": {"rule": rule}} for rule in base["rules"] if payload["id"] in (rule["tileAId"], rule["tileBId"])]
        return steps + [{"PinCell": {"pinned": cell}} for cell in base["pinned"] if cell["tileId"] == payload["id"]]
    if kind in ("change-tile-weight", "change-tile-media"):
        _, tile = find(base["tiles"], lambda item: item["id"] == payload["tileId"])
        return [{"ChangeTileWeight": {"tileId": tile["id"], "weight": tile["weight"]}}] if kind == "change-tile-weight" else [{"ChangeTileMedia": {"tileId": tile["id"], "media": tile["media"]}}]
    if kind == "create-rule":
        return [{"DeleteRule": {"id": payload["rule"]["id"]}}]
    if kind == "delete-rule":
        _, rule = find(base["rules"], lambda item: item["id"] == payload["id"])
        return [{"CreateRule": {"rule": rule}}]
    if kind == "pin-cell":
        key = cell_key(payload["pinned"])
        previous = [cell for cell in base["pinned"] if cell_key(cell) == key]
        return [{"PinCell": {"pinned": previous[0]}}] if previous else [{"UnpinCell": {"x": payload["pinned"]["x"], "y": payload["pinned"]["y"], "z": payload["pinned"]["z"]}}]
    if kind == "unpin-cell":
        return [{"PinCell": {"pinned": cell}} for cell in base["pinned"] if cell_key(cell) == cell_key(payload)]
    if kind == "mask-cell":
        key = cell_key(payload["cell"])
        return [] if any(cell_key(cell) == key for cell in base["masked"]) else [{"UnmaskCell": {"x": payload["cell"]["x"], "y": payload["cell"]["y"], "z": payload["cell"]["z"]}}]
    if kind == "unmask-cell":
        return [{"MaskCell": {"cell": cell}} for cell in base["masked"] if cell_key(cell) == cell_key(payload)]
    raise AssertionError("unknown kind " + kind)


def apply_mutation(base, mutation):
    """🧬️ One externally tagged mutation: its delta, applied."""
    (variant, payload), = mutation.items()
    delta = diff_for(VARIANTS[variant], payload, base)
    return apply_diff(base, delta), delta


LEAVES = ("before", "mutation", "diff", "outcome", "after")


def variant_of(kind):
    """🐫️ `change-tile-media` → `ChangeTileMedia`, the externally tagged payload's single key."""
    return "".join(word.capitalize() for word in kind.split("-"))


def committed(ctx):
    """🧫️ The committed quintet of this scenario's row, read through the plan's declared fixtures."""
    spec = ctx.doc_json()
    if spec["kind"] != ctx.row():
        raise AssertionError("scenario %s: the doc string names %r" % (ctx.scenario["id"], spec["kind"]))
    return {leaf: json.loads(ctx.fixture_bytes(spec[leaf]).decode("utf-8")) for leaf in LEAVES}


def adapter():
    """🧭️ The platform entry point. The reference answers in the ORACLE role only, by Scenario Outline base id —
    registering it as a subject too would make it its own subject and manufacture a green self-comparison. Every law
    the standalone replay checks is asserted in role, per row, before the parity phase compares the document it answers."""
    from semio_repo_test import Adapter, Outcome

    def answer(document):
        return Outcome(document, raw=json.dumps(document, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))

    def mutate_oracle(ctx):
        kind, leaves = ctx.row(), committed(ctx)
        variant = next(iter(leaves["mutation"]))
        if VARIANTS.get(variant) != kind:
            raise AssertionError("mutate-%s: the committed mutation is a %s" % (kind, variant))
        produced, delta = apply_mutation(leaves["before"], leaves["mutation"])
        if delta != leaves["diff"]:
            raise AssertionError("mutate-%s: the produced diff differs from the committed one" % kind)
        if produced != leaves["after"]:
            raise AssertionError("mutate-%s: the produced diff does not carry before to the committed after-snapshot" % kind)
        if leaves["outcome"].get("status") != "applied" or produced == leaves["before"]:
            raise AssertionError("mutate-%s: every committed vector declares the applied status and moves the document" % kind)
        return answer(produced)

    def inverse_oracle(ctx):
        kind, leaves = ctx.row(), committed(ctx)
        restored, _ = apply_mutation(leaves["before"], leaves["mutation"])
        (variant, payload), = leaves["mutation"].items()
        for step in inverse(leaves["before"], VARIANTS[variant], payload):
            restored, _ = apply_mutation(restored, step)
        if restored != leaves["before"]:
            raise AssertionError("inverse-%s: the reference's own inverse did not restore the before-snapshot" % kind)
        return answer(restored)

    return Adapter("python").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle)


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
