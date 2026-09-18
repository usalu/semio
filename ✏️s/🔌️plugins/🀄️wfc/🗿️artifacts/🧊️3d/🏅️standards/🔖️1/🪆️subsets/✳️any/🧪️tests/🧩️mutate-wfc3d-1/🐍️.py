#!/usr/bin/env python3
"""🐍️ wfc3d 1 — the PYTHON SECOND IMPLEMENTATION of the `s.wfc.wfc3d` editing algebra.

This file imports nothing from this repository. It re-implements, from
`../../🧬️schema/📸️snapshot/🔣️.json`, `../../🧬️schema/🧬️mutations/🔣️.json` and each
`../../🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`:

  * the document shape (schema/seed/slots/edges/tiles/rules),
  * all fifteen typed mutations, each with its own guard order
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
import os
import sys

COLLECTIONS = (
    ("slots", "slotsRemoved", "slotsUpserted"),
    ("edges", "edgesRemoved", "edgesUpserted"),
    ("tiles", "tilesRemoved", "tilesUpserted"),
    ("rules", "rulesRemoved", "rulesUpserted"),
)

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
        return empty_diff(), [("warning", "wfc3d.seed.unchanged")]
    delta = empty_diff()
    delta["seed"] = payload["seed"]
    return delta, []


def diff_create_slot(document, payload):
    slot = payload["slot"]
    if find(document["slots"], slot["id"])[1] is not None:
        return empty_diff(), [("fatal", "wfc3d.slot.duplicate-id")]
    if slot["width"] <= 0 or slot["height"] <= 0 or slot["depth"] <= 0:
        return empty_diff(), [("fatal", "wfc3d.slot.degenerate-box")]
    pinned = slot.get("pinnedTileId")
    if pinned is not None and find(document["tiles"], pinned)[1] is None:
        return empty_diff(), [("fatal", "wfc3d.slot.unknown-pinned-tile")]
    delta = empty_diff()
    delta["slotsUpserted"] = [[payload["index"], slot]]
    return delta, []


def diff_delete_slot(document, payload):
    index, _ = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.slot.missing")]
    incident = [edge["id"] for edge in document["edges"] if payload["id"] in (edge["fromSlotId"], edge["toSlotId"])]
    delta = empty_diff()
    delta["slotsRemoved"] = [payload["id"]]
    delta["edgesRemoved"] = incident
    return delta, ([("info", "wfc3d.slot.edges-cascaded")] if incident else [])


def diff_move_slot(document, payload):
    index, slot = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.slot.missing")]
    if (slot["x"], slot["y"], slot["z"]) == (payload["x"], payload["y"], payload["z"]):
        return empty_diff(), [("warning", "wfc3d.slot.position-unchanged")]
    moved = dict(slot)
    moved["x"], moved["y"], moved["z"] = payload["x"], payload["y"], payload["z"]
    delta = empty_diff()
    delta["slotsUpserted"] = [[index, moved]]
    return delta, []


def diff_resize_slot(document, payload):
    index, slot = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.slot.missing")]
    if payload["width"] <= 0 or payload["height"] <= 0 or payload["depth"] <= 0:
        return empty_diff(), [("fatal", "wfc3d.slot.degenerate-box")]
    if (slot["width"], slot["height"], slot["depth"]) == (payload["width"], payload["height"], payload["depth"]):
        return empty_diff(), [("warning", "wfc3d.slot.extent-unchanged")]
    resized = dict(slot)
    resized["width"], resized["height"], resized["depth"] = payload["width"], payload["height"], payload["depth"]
    delta = empty_diff()
    delta["slotsUpserted"] = [[index, resized]]
    return delta, []


def diff_connect_slots(document, payload):
    edge = payload["edge"]
    if find(document["edges"], edge["id"])[1] is not None:
        return empty_diff(), [("fatal", "wfc3d.edge.duplicate-id")]
    if find(document["slots"], edge["fromSlotId"])[1] is None or find(document["slots"], edge["toSlotId"])[1] is None:
        return empty_diff(), [("error", "wfc3d.slot.missing")]
    if edge["fromSlotId"] == edge["toSlotId"]:
        return empty_diff(), [("fatal", "wfc3d.edge.self-loop")]
    pair = {edge["fromSlotId"], edge["toSlotId"]}
    if any(existing["relation"] == edge["relation"] and {existing["fromSlotId"], existing["toSlotId"]} == pair for existing in document["edges"]):
        return empty_diff(), [("warning", "wfc3d.edge.already-connected")]
    delta = empty_diff()
    delta["edgesUpserted"] = [[payload["index"], edge]]
    return delta, []


def diff_disconnect_slots(document, payload):
    index, _ = find(document["edges"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.edge.missing")]
    delta = empty_diff()
    delta["edgesRemoved"] = [payload["id"]]
    return delta, []


def diff_pin_slot(document, payload):
    index, slot = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.slot.missing")]
    if find(document["tiles"], payload["tile_id"])[1] is None:
        return empty_diff(), [("error", "wfc3d.tile.missing")]
    if slot.get("pinnedTileId") == payload["tile_id"]:
        return empty_diff(), [("warning", "wfc3d.slot.pin-unchanged")]
    pinned = dict(slot)
    pinned["pinnedTileId"] = payload["tile_id"]
    delta = empty_diff()
    delta["slotsUpserted"] = [[index, pinned]]
    return delta, []


def diff_unpin_slot(document, payload):
    index, slot = find(document["slots"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.slot.missing")]
    if slot.get("pinnedTileId") is None:
        return empty_diff(), [("warning", "wfc3d.slot.pin-absent")]
    released = {key: value for key, value in slot.items() if key != "pinnedTileId"}
    delta = empty_diff()
    delta["slotsUpserted"] = [[index, released]]
    return delta, []


def diff_create_tile(document, payload):
    tile = payload["tile"]
    if find(document["tiles"], tile["id"])[1] is not None:
        return empty_diff(), [("fatal", "wfc3d.tile.duplicate-id")]
    if not tile["weight"] > 0:
        return empty_diff(), [("fatal", "wfc3d.tile.non-positive-weight")]
    delta = empty_diff()
    delta["tilesUpserted"] = [[payload["index"], tile]]
    return delta, []


def diff_delete_tile(document, payload):
    index, _ = find(document["tiles"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.tile.missing")]
    rules = [rule["id"] for rule in document["rules"] if payload["id"] in (rule["tileAId"], rule["tileBId"])]
    released = []
    for slot_index, slot in enumerate(document["slots"]):
        if slot.get("pinnedTileId") == payload["id"]:
            released.append([slot_index, {key: value for key, value in slot.items() if key != "pinnedTileId"}])
    delta = empty_diff()
    delta["tilesRemoved"] = [payload["id"]]
    delta["rulesRemoved"] = rules
    delta["slotsUpserted"] = released
    return delta, ([("info", "wfc3d.tile.references-cascaded")] if rules or released else [])


def diff_change_tile_weight(document, payload):
    index, tile = find(document["tiles"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.tile.missing")]
    if not payload["weight"] > 0:
        return empty_diff(), [("fatal", "wfc3d.tile.non-positive-weight")]
    if tile["weight"] == payload["weight"]:
        return empty_diff(), [("warning", "wfc3d.tile.weight-unchanged")]
    reweighted = dict(tile)
    reweighted["weight"] = payload["weight"]
    delta = empty_diff()
    delta["tilesUpserted"] = [[index, reweighted]]
    return delta, []


def diff_change_tile_media(document, payload):
    index, tile = find(document["tiles"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.tile.missing")]
    if tile["media"] == payload["media"]:
        return empty_diff(), [("warning", "wfc3d.tile.media-unchanged")]
    redressed = dict(tile)
    redressed["media"] = payload["media"]
    delta = empty_diff()
    delta["tilesUpserted"] = [[index, redressed]]
    return delta, []


def diff_create_rule(document, payload):
    rule = payload["rule"]
    if find(document["rules"], rule["id"])[1] is not None:
        return empty_diff(), [("fatal", "wfc3d.rule.duplicate-id")]
    if find(document["tiles"], rule["tileAId"])[1] is None or find(document["tiles"], rule["tileBId"])[1] is None:
        return empty_diff(), [("fatal", "wfc3d.rule.unknown-tile")]
    delta = empty_diff()
    delta["rulesUpserted"] = [[payload["index"], rule]]
    return delta, []


def diff_delete_rule(document, payload):
    index, _ = find(document["rules"], payload["id"])
    if index is None:
        return empty_diff(), [("error", "wfc3d.rule.missing")]
    delta = empty_diff()
    delta["rulesRemoved"] = [payload["id"]]
    return delta, []


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
}


# ---------------------------------------------------------------- apply


def apply_collection(base, removed, upserted):
    """🧬️ The same validated, index-checked apply the Rust diff type performs."""
    items = [member for member in base if member["id"] not in removed]
    for index, value in upserted:
        existing, _ = find(items, value["id"])
        if existing is not None:
            items[existing] = value
        else:
            if index > len(items):
                raise ValueError(f"insertion index {index} exceeds length {len(items)}")
            items.insert(index, value)
    return items


def apply_diff(document, delta):
    result = json.loads(json.dumps(document))
    if delta.get("schema") is not None:
        result["schema"] = delta["schema"]
    if delta.get("seed") is not None:
        result["seed"] = delta["seed"]
    for field, removed_key, upserted_key in COLLECTIONS:
        result[field] = apply_collection(result[field], delta[removed_key], delta[upserted_key])
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
            return [{"PinSlot": {"id": slot["id"], "tile_id": previous}}]
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
                steps.append({"PinSlot": {"id": slot["id"], "tile_id": payload["id"]}})
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
    raise ValueError(f"unknown mutation variant {variant}")


def apply_mutation(document, mutation):
    (variant, payload), = mutation.items()
    delta, messages = DIFF_BUILDERS[variant](document, payload)
    return apply_diff(document, delta), delta, messages


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
