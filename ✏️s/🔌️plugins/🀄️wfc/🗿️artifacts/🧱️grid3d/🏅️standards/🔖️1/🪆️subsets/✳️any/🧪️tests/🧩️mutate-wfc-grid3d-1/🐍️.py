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


PATCH_FIELDS = {
    "tiles": (("weight", False), ("media", False), ("label", True)),
    "rules": (("tileAId", False), ("tileBId", False), ("direction", False), ("allowed", False)),
    "pinned": (("tileId", False),),
    "masked": (),
}
"""🩹 Per collection, the fields a row patch may set, in wire order; `True` marks an optional field wrapped as `{"value": …}`."""


def rows(removed=None, added=None, patched=None):
    """📂 One collection's delta: removed keys, added rows (landing at their canonical position) and per-row patches."""
    return {"removed": removed or [], "added": added or [], "patched": patched or []}


def patch(collection, identifier, **fields):
    """🩹 One row patch: every field present, the unset ones `None`, optional fields wrapped."""
    body = {}
    for name, optional in PATCH_FIELDS[collection]:
        body[name] = ({"value": fields[name]} if optional else fields[name]) if name in fields else None
    return {"id": identifier, "patch": body}


def axis_patch(sizes, target):
    """📏 The axis patch from `sizes` to `target`: a new length when it differs, then one row per cell whose size differs or is new."""
    changed = [{"index": index, "size": size} for index, size in enumerate(target) if index >= len(sizes) or sizes[index] != size]
    patch_ = {"length": len(target) if len(sizes) != len(target) else None, "sizes": changed}
    return patch_ if patch_["length"] is not None or patch_["sizes"] else None


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
        "tiles": rows(),
        "rules": rows(),
        "pinned": rows(),
        "masked": rows(),
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
        diff["cellSizesX"] = axis_patch(base["cellSizesX"], resized_axis(base["cellSizesX"], payload["width"]))
        diff["cellSizesY"] = axis_patch(base["cellSizesY"], resized_axis(base["cellSizesY"], payload["height"]))
        diff["cellSizesZ"] = axis_patch(base["cellSizesZ"], resized_axis(base["cellSizesZ"], payload["depth"]))
    elif kind == "change-cell-sizes":
        diff["cellSizes" + payload["axis"].upper()] = axis_patch(base["cellSizes" + payload["axis"].upper()], list(payload["sizes"]))
    elif kind == "change-periodicity":
        diff["periodicX"] = payload["periodicX"]
        diff["periodicY"] = payload["periodicY"]
        diff["periodicZ"] = payload["periodicZ"]
    elif kind == "create-tile":
        diff["tiles"] = rows(added=[payload["tile"]])
    elif kind == "delete-tile":
        target = payload["id"]
        diff["tiles"] = rows(removed=[target])
        diff["rules"] = rows(removed=[rule["id"] for rule in base["rules"] if target in (rule["tileAId"], rule["tileBId"])])
        diff["pinned"] = rows(removed=[cell_key(cell) for cell in base["pinned"] if cell["tileId"] == target])
    elif kind == "change-tile-weight":
        index, tile = find(base["tiles"], lambda item: item["id"] == payload["tileId"])
        diff["tiles"] = rows(patched=[patch("tiles", tile["id"], weight=payload["weight"])])
    elif kind == "change-tile-media":
        index, tile = find(base["tiles"], lambda item: item["id"] == payload["tileId"])
        diff["tiles"] = rows(patched=[patch("tiles", tile["id"], media=payload["media"])])
    elif kind == "create-rule":
        diff["rules"] = rows(added=[payload["rule"]])
    elif kind == "delete-rule":
        diff["rules"] = rows(removed=[payload["id"]])
    elif kind == "pin-cell":
        pinned = payload["pinned"]
        key = cell_key(pinned)
        if any(cell_key(cell) == key for cell in base["pinned"]):
            diff["pinned"] = rows(patched=[patch("pinned", key, tileId=pinned["tileId"])])
        else:
            diff["pinned"] = rows(added=[pinned])
    elif kind == "unpin-cell":
        diff["pinned"] = rows(removed=[cell_key(payload)])
    elif kind == "mask-cell":
        diff["masked"] = rows(added=[payload["cell"]])
    elif kind == "unmask-cell":
        diff["masked"] = rows(removed=[cell_key(payload)])
    else:
        raise AssertionError("unknown kind " + kind)
    return diff


def find(items, predicate):
    for index, item in enumerate(items):
        if predicate(item):
            return index, item
    raise AssertionError("no member matches")


def apply_collection(base, delta, collection, item_key):
    """📂 Removals first, then canonical-position insertions, then field patches; unknown targets are refused."""
    items = list(base)
    for key in delta["removed"]:
        at = next((slot for slot, item in enumerate(items) if item_key(item) == key), None)
        if at is None:
            raise AssertionError(f"removed {key!r} does not exist")
        del items[at]
    for row in delta["added"]:
        if any(item_key(item) == item_key(row) for item in items):
            raise AssertionError(f"added {item_key(row)!r} already exists")
        items.insert(ordered_index(items, item_key(row), item_key), row)
    optional = {name for name, wrapped in PATCH_FIELDS[collection] if wrapped}
    for entry in delta["patched"]:
        at = next((slot for slot, item in enumerate(items) if item_key(item) == entry["id"]), None)
        if at is None:
            raise AssertionError(f"patched {entry['id']!r} does not exist")
        row = dict(items[at])
        for name, value in entry["patch"].items():
            if value is None:
                continue
            if name in optional:
                if value["value"] is None:
                    row.pop(name, None)
                else:
                    row[name] = value["value"]
            else:
                row[name] = value
        items[at] = row
    return items


def apply_axis(sizes, patch_):
    """📏 The axis after the patch: resized, then every row's cell set."""
    out = list(sizes)
    if patch_["length"] is not None:
        out = (out + [0.0] * patch_["length"])[: patch_["length"]]
    for row in patch_["sizes"]:
        out[row["index"]] = row["size"]
    return out


def apply_diff(base, diff):
    out = copy.deepcopy(base)
    for scalar in ("schema", "seed", "width", "height", "depth", "periodicX", "periodicY", "periodicZ"):
        if diff[scalar] is not None:
            out[scalar] = diff[scalar]
    for axis in ("cellSizesX", "cellSizesY", "cellSizesZ"):
        if diff[axis] is not None:
            out[axis] = apply_axis(out[axis], diff[axis])
    out["tiles"] = apply_collection(out["tiles"], diff["tiles"], "tiles", lambda item: item["id"])
    out["rules"] = apply_collection(out["rules"], diff["rules"], "rules", lambda item: item["id"])
    out["pinned"] = apply_collection(out["pinned"], diff["pinned"], "pinned", cell_key)
    out["masked"] = apply_collection(out["masked"], diff["masked"], "masked", cell_key)
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
    return {leaf: json.loads(ctx.input_bytes(spec[leaf]).decode("utf-8")) for leaf in LEAVES}


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
