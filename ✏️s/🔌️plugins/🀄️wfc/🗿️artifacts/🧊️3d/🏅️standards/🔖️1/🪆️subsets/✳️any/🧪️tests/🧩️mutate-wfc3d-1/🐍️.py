#!/usr/bin/env python3
"""🐍️ wfc3d 1 — the PYTHON SECOND IMPLEMENTATION of the `s.wfc.wfc3d` editing algebra.

This file imports nothing from this repository. It re-implements, from
`../../🧬️schema/📸️snapshot/🔣️.json`, `../../🧬️schema/🧬️mutations/🔣️.json` and each
`../../🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`:

  * the document shape (schema/seed/slots/edges/tiles/rules),
  * all seventeen typed mutations, each with its own guard order
    (target-missing → invariant → no-op → apply),
  * both cascades — `delete-slot` drops every edge incident to the slot, and `delete-tile` drops
    every rule naming the tile AND releases every pin on it,
  * the sparse delta shape (`<name>Removed` bare ids, `<name>Upserted` `[index, value]` pairs),
  * and the inverse of every kind, as a list of single steps of this same vocabulary.

It then replays every committed quintet and requires four things the Rust half also requires:
the produced delta IS the committed `🔺️diff`, applying it reaches the committed `➡️after`, the
raised diagnostics ARE the committed `🎯️outcome` messages, and applying this implementation's own
inverse to `after` restores `before` exactly.

Three facts only the committed vectors state, and this file takes them from there rather than
guessing: the mutation payload is EXTERNALLY tagged (`{"DeleteSlot": {...}}`, a PascalCase variant
name as the single key); a `create-*` carries the collection's CANONICAL sorted insertion index, so
a delete followed by its inverse restores the row's position; and a refusal produces an EMPTY delta
with a diagnostic rather than an error, so applying a refused mutation is a document no-op.

What it deliberately does NOT model is the SOLVE, and therefore not the rule semantics either: the
rules are an allow-list compiled onto an empty model, which is the inference's business and is pinned
by the artifact's own deterministic-seed and contradiction tests plus the committed example outcomes.
Here a rule is just another id-keyed row, and `delete-tile` cascading into it is an editing fact.

Run standalone:  python3 🐍️.py --fixtures <…/✳️any/🧫️fixtures/🧬️mutations>
"""

import argparse
import json
import math
import os
import sys

COLLECTIONS = ("slots", "edges", "tiles", "rules")

PATCH_FIELDS = {
    "slots": (("x", False), ("y", False), ("z", False), ("width", False), ("height", False), ("depth", False), ("pinnedTileId", True)),
    "edges": (("fromSlotId", False), ("toSlotId", False), ("relation", False)),
    "tiles": (("weight", False), ("media", False), ("label", True)),
    "rules": (("tileAId", False), ("tileBId", False), ("allowed", False), ("relation", True)),
}
"""🩹 Per collection, the fields a row patch may set, in wire order; `True` marks an optional field wrapped as `{"value": …}`."""


def rows(removed=None, added=None, patched=None):
    """📂 One collection's delta: removed ids, added rows (landing at their canonical position) and per-row patches."""
    return {"removed": removed or [], "added": added or [], "patched": patched or []}


EMPTY_DIFF = {"schema": None, "seed": None, "slots": rows(), "edges": rows(), "tiles": rows(), "rules": rows()}


def patch(collection, identifier, **fields):
    """🩹 One row patch: every field present, the unset ones `None`, optional fields wrapped."""
    body = {}
    for name, optional in PATCH_FIELDS[collection]:
        if name in fields:
            body[name] = {"value": fields[name]} if optional else fields[name]
        else:
            body[name] = None
    return {"id": identifier, "patch": body}


def canonical_matches(collection, payload, identifier):
    """📍 A `create-*` names the collection's canonical sorted insertion index — a diff row carries no index, the position is canonical."""
    return payload["index"] == canonical_index(collection, identifier)


def empty_diff():
    return json.loads(json.dumps(EMPTY_DIFF))


def find(collection, identifier):
    for index, member in enumerate(collection):
        if member["id"] == identifier:
            return index, member
    return None, None


def canonical_index(collection, identifier):
    """🔤️ Where an id belongs in its collection's sorted order — a count of strictly smaller ids."""
    return sum(1 for member in collection if member["id"] < identifier)


# ---------------------------------------------------------------- diff builders


def diff_change_seed(document, payload):
    if document["seed"] == payload["seed"]:
        return empty_diff(), [("warning", "mutation.no-op")]
    delta = empty_diff()
    delta["seed"] = payload["seed"]
    return delta, []


def diff_create_slot(document, payload):
    slot = payload["slot"]
    if find(document["slots"], slot["id"])[1] is not None:
        return empty_diff(), [("fatal", "mutation.duplicate-id")]
    if slot["width"] <= 0 or slot["height"] <= 0 or slot["depth"] <= 0:
        return empty_diff(), [("fatal", "mutation.invariant")]
    pinned = slot.get("pinnedTileId")
    if pinned is not None and find(document["tiles"], pinned)[1] is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if not canonical_matches(document["slots"], payload, slot["id"]):
        return empty_diff(), [("fatal", "mutation.invariant")]
    delta = empty_diff()
    delta["slots"]["added"] = [slot]
    return delta, []


def diff_delete_slot(document, payload):
    index, _ = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    incident = [edge["id"] for edge in document["edges"] if payload["id"] in (edge["fromSlotId"], edge["toSlotId"])]
    delta = empty_diff()
    delta["slots"]["removed"] = [payload["id"]]
    delta["edges"]["removed"] = incident
    return delta, ([("info", "mutation.cascade")] if incident else [])


def diff_move_slot(document, payload):
    index, slot = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if (slot["x"], slot["y"], slot["z"]) == (payload["x"], payload["y"], payload["z"]):
        return empty_diff(), [("warning", "mutation.no-op")]
    delta = empty_diff()
    delta["slots"]["patched"] = [patch("slots", slot["id"], x=payload["x"], y=payload["y"], z=payload["z"])]
    return delta, []


def diff_resize_slot(document, payload):
    index, slot = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if payload["width"] <= 0 or payload["height"] <= 0 or payload["depth"] <= 0:
        return empty_diff(), [("fatal", "mutation.invariant")]
    if (slot["width"], slot["height"], slot["depth"]) == (payload["width"], payload["height"], payload["depth"]):
        return empty_diff(), [("warning", "mutation.no-op")]
    delta = empty_diff()
    delta["slots"]["patched"] = [patch("slots", slot["id"], width=payload["width"], height=payload["height"], depth=payload["depth"])]
    return delta, []


def diff_connect_slots(document, payload):
    edge = payload["edge"]
    if find(document["edges"], edge["id"])[1] is not None:
        return empty_diff(), [("fatal", "mutation.duplicate-id")]
    if find(document["slots"], edge["fromSlotId"])[1] is None or find(document["slots"], edge["toSlotId"])[1] is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if edge["fromSlotId"] == edge["toSlotId"]:
        return empty_diff(), [("fatal", "mutation.invariant")]
    pair = {edge["fromSlotId"], edge["toSlotId"]}
    if any(existing["relation"] == edge["relation"] and {existing["fromSlotId"], existing["toSlotId"]} == pair for existing in document["edges"]):
        return empty_diff(), [("warning", "mutation.no-op")]
    if not canonical_matches(document["edges"], payload, edge["id"]):
        return empty_diff(), [("fatal", "mutation.invariant")]
    delta = empty_diff()
    delta["edges"]["added"] = [edge]
    return delta, []


def diff_disconnect_slots(document, payload):
    index, _ = find(document["edges"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    delta = empty_diff()
    delta["edges"]["removed"] = [payload["id"]]
    return delta, []


def diff_pin_slot(document, payload):
    index, slot = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if find(document["tiles"], payload["tileId"])[1] is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if slot.get("pinnedTileId") == payload["tileId"]:
        return empty_diff(), [("warning", "mutation.no-op")]
    delta = empty_diff()
    delta["slots"]["patched"] = [patch("slots", slot["id"], pinnedTileId=payload["tileId"])]
    return delta, []


def diff_unpin_slot(document, payload):
    index, slot = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if slot.get("pinnedTileId") is None:
        return empty_diff(), [("warning", "mutation.no-op")]
    delta = empty_diff()
    delta["slots"]["patched"] = [patch("slots", slot["id"], pinnedTileId=None)]
    return delta, []


def diff_create_tile(document, payload):
    tile = payload["tile"]
    if find(document["tiles"], tile["id"])[1] is not None:
        return empty_diff(), [("fatal", "mutation.duplicate-id")]
    if not tile["weight"] > 0:
        return empty_diff(), [("fatal", "mutation.invariant")]
    if not canonical_matches(document["tiles"], payload, tile["id"]):
        return empty_diff(), [("fatal", "mutation.invariant")]
    delta = empty_diff()
    delta["tiles"]["added"] = [tile]
    return delta, []


def diff_delete_tile(document, payload):
    index, _ = find(document["tiles"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    rules = [rule["id"] for rule in document["rules"] if payload["id"] in (rule["tileAId"], rule["tileBId"])]
    released = [patch("slots", slot["id"], pinnedTileId=None) for slot in document["slots"] if slot.get("pinnedTileId") == payload["id"]]
    delta = empty_diff()
    delta["tiles"]["removed"] = [payload["id"]]
    delta["rules"]["removed"] = rules
    delta["slots"]["patched"] = released
    return delta, ([("info", "mutation.cascade")] if rules or released else [])


def diff_change_tile_weight(document, payload):
    index, tile = find(document["tiles"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if not payload["weight"] > 0:
        return empty_diff(), [("fatal", "mutation.invariant")]
    if tile["weight"] == payload["weight"]:
        return empty_diff(), [("warning", "mutation.no-op")]
    delta = empty_diff()
    delta["tiles"]["patched"] = [patch("tiles", tile["id"], weight=payload["weight"])]
    return delta, []


def diff_change_tile_media(document, payload):
    index, tile = find(document["tiles"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if tile["media"] == payload["media"]:
        return empty_diff(), [("warning", "mutation.no-op")]
    delta = empty_diff()
    delta["tiles"]["patched"] = [patch("tiles", tile["id"], media=payload["media"])]
    return delta, []


def diff_create_rule(document, payload):
    rule = payload["rule"]
    if find(document["rules"], rule["id"])[1] is not None:
        return empty_diff(), [("fatal", "mutation.duplicate-id")]
    if find(document["tiles"], rule["tileAId"])[1] is None or find(document["tiles"], rule["tileBId"])[1] is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    if not canonical_matches(document["rules"], payload, rule["id"]):
        return empty_diff(), [("fatal", "mutation.invariant")]
    delta = empty_diff()
    delta["rules"]["added"] = [rule]
    return delta, []


def diff_delete_rule(document, payload):
    index, _ = find(document["rules"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "mutation.target-missing")]
    delta = empty_diff()
    delta["rules"]["removed"] = [payload["id"]]
    return delta, []


def distinct(identifiers):
    """🛂️ At least one id, and none twice."""
    return len(identifiers) > 0 and len(set(identifiers)) == len(identifiers)


def finite(*values):
    return all(isinstance(value, (int, float)) and math.isfinite(value) for value in values)


def partial(missing):
    """⚠️ The `mutation.partial` warning a multi-slot leaf raises for slots the document lacks."""
    return [("warning", "mutation.partial")] if missing else []


def diff_drag_slots(document, payload):
    targets = payload["targets"]
    if not distinct(targets) or not finite(payload["dx"], payload["dy"], payload["dz"]):
        return empty_diff(), [("fatal", "mutation.invariant")]
    missing = [identifier for identifier in targets if find(document["slots"], identifier)[1] is None]
    if len(missing) == len(targets):
        return empty_diff(), [("error", "mutation.target-missing")]
    if (payload["dx"], payload["dy"], payload["dz"]) == (0, 0, 0):
        return empty_diff(), partial(missing) + [("warning", "mutation.no-op")]
    delta = empty_diff()
    for slot in document["slots"]:
        if slot["id"] in targets:
            delta["slots"]["patched"].append(patch("slots", slot["id"], x=slot["x"] + payload["dx"], y=slot["y"] + payload["dy"], z=slot["z"] + payload["dz"]))
    return delta, partial(missing)


def diff_set_slot_positions(document, payload):
    positions = payload["positions"]
    identifiers = [position["id"] for position in positions]
    if not distinct(identifiers) or not all(finite(position["x"], position["y"], position["z"]) for position in positions):
        return empty_diff(), [("fatal", "mutation.invariant")]
    missing = [identifier for identifier in identifiers if find(document["slots"], identifier)[1] is None]
    if len(missing) == len(identifiers):
        return empty_diff(), [("error", "mutation.target-missing")]
    delta = empty_diff()
    for slot in document["slots"]:
        position = next((row for row in positions if row["id"] == slot["id"]), None)
        if position is not None and (position["x"], position["y"], position["z"]) != (slot["x"], slot["y"], slot["z"]):
            delta["slots"]["patched"].append(patch("slots", slot["id"], x=position["x"], y=position["y"], z=position["z"]))
    if not delta["slots"]["patched"]:
        return empty_diff(), partial(missing) + [("warning", "mutation.no-op")]
    return delta, partial(missing)


DIFF_BUILDERS = {
    "ChangeSeed": diff_change_seed,
    "CreateSlot": diff_create_slot,
    "DeleteSlot": diff_delete_slot,
    "MoveSlot": diff_move_slot,
    "ResizeSlot": diff_resize_slot,
    "ConnectSlots": diff_connect_slots,
    "DisconnectSlots": diff_disconnect_slots,
    "PinSlot": diff_pin_slot,
    "UnpinSlot": diff_unpin_slot,
    "CreateTile": diff_create_tile,
    "DeleteTile": diff_delete_tile,
    "ChangeTileWeight": diff_change_tile_weight,
    "ChangeTileMedia": diff_change_tile_media,
    "CreateRule": diff_create_rule,
    "DeleteRule": diff_delete_rule,
    "DragSlots": diff_drag_slots,
    "SetSlotPositions": diff_set_slot_positions,
}


# ---------------------------------------------------------------- apply


def positional(document, legacy):
    """📍 The keyed-rows delta as positional list deltas: removals at their base index, insertions at their canonical after index."""
    out = dict(legacy)
    for field in COLLECTIONS:
        base, keyed = document[field], legacy[field]
        removed = [{"id": identifier, "index": next(at for at, row in enumerate(base) if row["id"] == identifier)} for identifier in keyed["removed"] if any(row["id"] == identifier for row in base)]
        gone = {entry["id"] for entry in removed}
        running = [row for row in base if row["id"] not in gone]
        for row in keyed["added"]:
            running.insert(canonical_index(running, row["id"]), row)
        inserted = [{"index": next(at for at, existing in enumerate(running) if existing["id"] == row["id"]), "row": row} for row in keyed["added"]]
        out[field] = {"removed": removed, "inserted": inserted, "moved": [], "modified": keyed["patched"]}
    return out


def apply_collection(base, delta, collection):
    """🧬️ The positional list-delta apply: removed and moved ids are checked at their base index, inserted and moved rows take their after slots, survivors fill the rest in base order, then the patches write."""
    taken = set()
    for entry in delta["removed"] + [{"id": move["id"], "index": move["from"]} for move in delta["moved"]]:
        if entry["index"] >= len(base) or base[entry["index"]]["id"] != entry["id"] or entry["index"] in taken:
            raise ValueError(f"{entry['id']!r} is not at base index {entry['index']}")
        taken.add(entry["index"])
    slots = [None] * (len(base) - len(delta["removed"]) + len(delta["inserted"]))
    for entry in delta["inserted"]:
        if entry["index"] >= len(slots) or slots[entry["index"]] is not None:
            raise ValueError(f"inserted {entry['row']['id']!r} has no free after slot {entry['index']}")
        slots[entry["index"]] = entry["row"]
    for move in delta["moved"]:
        slots[move["to"]] = base[move["from"]]
    survivors = iter([row for at, row in enumerate(base) if at not in taken])
    items = [slot if slot is not None else next(survivors) for slot in slots]
    if len({row["id"] for row in items}) != len(items):
        raise ValueError("two rows of the after list carry the same id")
    optional = {name for name, wrapped in PATCH_FIELDS[collection] if wrapped}
    for entry in delta["modified"]:
        existing, current = find(items, entry["id"])
        if existing is None:
            raise ValueError(f"modified {entry['id']!r} does not exist")
        row = dict(current)
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
        items[existing] = row
    return items


def apply_diff(document, delta):
    result = json.loads(json.dumps(document))
    if delta.get("schema") is not None:
        result["schema"] = delta["schema"]
    if delta.get("seed") is not None:
        result["seed"] = delta["seed"]
    for field in COLLECTIONS:
        result[field] = apply_collection(result[field], delta[field], field)
    return result


# ---------------------------------------------------------------- inverse


def inverse(document, variant, payload):
    """↩️ Every inverse is a list of single steps of this SAME vocabulary, each carrying the member's
    own BASE position, so a removed row comes back exactly where it was."""
    if variant == "ChangeSeed":
        return [{"ChangeSeed": {"seed": document["seed"]}}]
    if variant == "CreateSlot":
        return [{"DeleteSlot": {"id": payload["slot"]["id"]}}]
    if variant == "DeleteSlot":
        index, slot = find(document["slots"], payload["id"])
        if index is None:
            return []
        steps = [{"CreateSlot": {"index": index, "slot": slot}}]
        for edge_index, edge in enumerate(document["edges"]):
            if payload["id"] in (edge["fromSlotId"], edge["toSlotId"]):
                steps.append({"ConnectSlots": {"index": edge_index, "edge": edge}})
        return steps
    if variant == "MoveSlot":
        _, slot = find(document["slots"], payload["id"])
        return [] if slot is None else [{"MoveSlot": {"id": slot["id"], "x": slot["x"], "y": slot["y"], "z": slot["z"]}}]
    if variant == "ResizeSlot":
        _, slot = find(document["slots"], payload["id"])
        return [] if slot is None else [{"ResizeSlot": {"id": slot["id"], "width": slot["width"], "height": slot["height"], "depth": slot["depth"]}}]
    if variant == "ConnectSlots":
        return [{"DisconnectSlots": {"id": payload["edge"]["id"]}}]
    if variant == "DisconnectSlots":
        index, edge = find(document["edges"], payload["id"])
        return [] if index is None else [{"ConnectSlots": {"index": index, "edge": edge}}]
    if variant in ("PinSlot", "UnpinSlot"):
        _, slot = find(document["slots"], payload["id"])
        if slot is None:
            return []
        previous = slot.get("pinnedTileId")
        if previous is not None:
            return [{"PinSlot": {"id": slot["id"], "tileId": previous}}]
        return [{"UnpinSlot": {"id": slot["id"]}}] if variant == "PinSlot" else []
    if variant == "CreateTile":
        return [{"DeleteTile": {"id": payload["tile"]["id"]}}]
    if variant == "DeleteTile":
        index, tile = find(document["tiles"], payload["id"])
        if index is None:
            return []
        steps = [{"CreateTile": {"index": index, "tile": tile}}]
        for rule_index, rule in enumerate(document["rules"]):
            if payload["id"] in (rule["tileAId"], rule["tileBId"]):
                steps.append({"CreateRule": {"index": rule_index, "rule": rule}})
        for slot in document["slots"]:
            if slot.get("pinnedTileId") == payload["id"]:
                steps.append({"PinSlot": {"id": slot["id"], "tileId": payload["id"]}})
        return steps
    if variant == "ChangeTileWeight":
        _, tile = find(document["tiles"], payload["id"])
        return [] if tile is None else [{"ChangeTileWeight": {"id": tile["id"], "weight": tile["weight"]}}]
    if variant == "ChangeTileMedia":
        _, tile = find(document["tiles"], payload["id"])
        return [] if tile is None else [{"ChangeTileMedia": {"id": tile["id"], "media": tile["media"]}}]
    if variant == "CreateRule":
        return [{"DeleteRule": {"id": payload["rule"]["id"]}}]
    if variant == "DeleteRule":
        index, rule = find(document["rules"], payload["id"])
        return [] if index is None else [{"CreateRule": {"index": index, "rule": rule}}]
    if variant == "DragSlots":
        if not distinct(payload["targets"]) or not finite(payload["dx"], payload["dy"], payload["dz"]) or (payload["dx"], payload["dy"], payload["dz"]) == (0, 0, 0):
            return []
        positions = [{"id": slot["id"], "x": slot["x"], "y": slot["y"], "z": slot["z"]} for slot in document["slots"] if slot["id"] in payload["targets"]]
        return [{"SetSlotPositions": {"positions": positions}}] if positions else []
    if variant == "SetSlotPositions":
        requested = payload["positions"]
        if not distinct([position["id"] for position in requested]) or not all(finite(position["x"], position["y"], position["z"]) for position in requested):
            return []
        moved = [slot for slot in document["slots"] if any(position["id"] == slot["id"] and (position["x"], position["y"], position["z"]) != (slot["x"], slot["y"], slot["z"]) for position in requested)]
        positions = [{"id": slot["id"], "x": slot["x"], "y": slot["y"], "z": slot["z"]} for slot in moved]
        return [{"SetSlotPositions": {"positions": positions}}] if positions else []
    raise ValueError(f"unknown mutation variant {variant}")


def apply_mutation(document, mutation):
    (variant, payload), = mutation.items()
    legacy, messages = DIFF_BUILDERS[variant](document, payload)
    delta = positional(document, legacy)
    return apply_diff(document, delta), delta, messages


# ---------------------------------------------------------------- adapter


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
        if list(leaves["mutation"]) != [variant_of(kind)]:
            raise AssertionError("mutate-%s: the committed payload declares %r" % (kind, list(leaves["mutation"])))
        produced, delta, messages = apply_mutation(leaves["before"], leaves["mutation"])
        if delta != leaves["diff"]:
            raise AssertionError("mutate-%s: the produced delta is not the committed 🔺️diff" % kind)
        if produced != leaves["after"]:
            raise AssertionError("mutate-%s: applying the mutation does not reach the committed after-snapshot" % kind)
        declared = [(row["level"], row["code"]) for row in leaves["outcome"].get("messages", [])]
        if list(messages) != declared:
            raise AssertionError("mutate-%s: raised diagnostics %r are not the committed %r" % (kind, messages, declared))
        if produced == leaves["before"] and ("warning", "mutation.no-op") not in declared:
            raise AssertionError("mutate-%s: the vector does not move the document and its outcome declares no no-op" % kind)
        return answer(produced)

    def inverse_oracle(ctx):
        kind, leaves = ctx.row(), committed(ctx)
        restored, _, _ = apply_mutation(leaves["before"], leaves["mutation"])
        (variant, payload), = leaves["mutation"].items()
        for step in inverse(leaves["before"], variant, payload):
            restored, _, _ = apply_mutation(restored, step)
        if restored != leaves["before"]:
            raise AssertionError("inverse-%s: the reference's own inverse did not restore the before-snapshot" % kind)
        return answer(restored)

    def identity_oracle(ctx):
        committed_bytes = ctx.input_bytes(ctx.step_input_uris()[0])
        document = json.loads(committed_bytes.decode("utf-8"))
        reparsed = json.loads(json.dumps(document, separators=(",", ":"), ensure_ascii=False))
        if reparsed != document:
            raise AssertionError("identity-round-trip: re-serializing and re-reading the document moved it")
        if not all(tile.get("media") for tile in reparsed["tiles"]):
            raise AssertionError("identity-round-trip: a tile lost its inline media")
        return answer(reparsed)

    return Adapter("python").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("identity-round-trip", identity_oracle)


# ---------------------------------------------------------------- vector replay


def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def replay(vector_directory):
    """▶️ One committed quintet, checked four ways."""
    before = read(os.path.join(vector_directory, "📸️snapshot", "⬅️before", "🔣️.json"))
    after = read(os.path.join(vector_directory, "📸️snapshot", "➡️after", "🔣️.json"))
    mutation = read(os.path.join(vector_directory, "🦠️mutation", "🔣️.json"))
    committed_diff = read(os.path.join(vector_directory, "🔺️diff", "🔣️.json"))
    outcome = read(os.path.join(vector_directory, "🎯️outcome", "🔣️.json"))

    problems = []
    produced, delta, messages = apply_mutation(before, mutation)
    if delta != committed_diff:
        problems.append("the produced delta is not the committed 🔺️diff")
    if produced != after:
        problems.append("applying the mutation does not reach the committed ➡️after")
    declared = [(row["level"], row["code"]) for row in outcome.get("messages", [])]
    if messages != declared:
        problems.append(f"raised diagnostics {messages} are not the committed {declared}")

    (variant, payload), = mutation.items()
    restored = produced
    for step in inverse(before, variant, payload):
        restored, _, _ = apply_mutation(restored, step)
    if restored != before:
        problems.append("the inverse does not restore ⬅️before exactly")
    return problems


def main():
    parser = argparse.ArgumentParser(description="wfc3d python second implementation")
    parser.add_argument("--fixtures", required=True, help="the 🧫️fixtures/🧬️mutations directory")
    arguments = parser.parse_args()

    failures = 0
    vectors = 0
    for mutation_directory in sorted(os.listdir(arguments.fixtures)):
        mutation_path = os.path.join(arguments.fixtures, mutation_directory)
        if not os.path.isdir(mutation_path):
            continue
        for case in sorted(os.listdir(mutation_path)):
            case_path = os.path.join(mutation_path, case)
            if not os.path.isdir(case_path):
                continue
            vectors += 1
            problems = replay(case_path)
            status = "ok" if not problems else "FAILED"
            print(f"{mutation_directory}/{case}: {status}")
            for problem in problems:
                print(f"    - {problem}")
                failures += 1
    print(f"\n{vectors} vectors, {failures} problems")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
