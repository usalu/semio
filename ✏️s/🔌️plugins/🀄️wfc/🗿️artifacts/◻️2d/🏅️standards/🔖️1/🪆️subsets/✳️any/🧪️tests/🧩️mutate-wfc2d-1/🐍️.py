#!/usr/bin/env python3
"""🀄️ An INDEPENDENT second implementation of the `s.wfc.wfc2d` document and its fifteen typed
mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `wfc2d` document is the INPUT to a
wave-function-collapse solve, not its output: free-placed rectangular slots, an explicit adjacency
edge list with NAMED relation classes, a tile alphabet carrying its own inline 2D media, and a rule
set, all under one `seed`. A WFC library computes a collapse; none of them carries the problem
statement as a document, and none of them reads this carrier. That this algebra is adjudicable by a
Python twin was settled by `mutate-assembly-1`, this artifact's direct ancestor, over the same
carrier shape.

**What it was written from.**

* `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` — the six members of the snapshot.
* `…/🧬️schema/🧬️mutations/🔣️.json` — the fifteen externally tagged payload shapes.
* the fifteen committed `(before, mutation, diff, outcome, after)` quintets, for the verbs, their
  argument lists, and the four things only they state: that this subset tags its mutations
  EXTERNALLY (`{"CreateSlot": {…}}`, a PascalCase variant name as the single key); that every create
  inserts at the CANONICAL ASCENDING-`id` position rather than at the end, which is what makes a
  create/delete pair point-invertible; that `delete-slot` cascades into the edges naming the slot and
  `delete-tile` cascades into BOTH the rules naming the tile and the slot pins holding it, each
  saying so with one `info`-level `mutation.cascade`; and that a diff is TOTAL — every lane is an
  explicit key, an untouched one carrying `null` or `[]`, never an omitted key.

**No Rust was read to write this.** Run it directly (`python3 🐍️.py`) to replay every committed
quintet under this subset's `🧫️fixtures/🧬️mutations/` tree; it exits non-zero on the first divergence.
"""

# region 🔖️Imports
from __future__ import annotations

import copy
import json
import pathlib
import re
import sys

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "seed", "slots", "edges", "tiles", "rules")
"""🗂️ The members the snapshot declares — and the cross-language projection."""

KINDS = (
    "change-seed",
    "create-slot",
    "delete-slot",
    "move-slot",
    "resize-slot",
    "connect-slots",
    "disconnect-slots",
    "pin-slot",
    "unpin-slot",
    "create-tile",
    "delete-tile",
    "change-tile-weight",
    "change-tile-media",
    "create-rule",
    "delete-rule",
)
"""🏷️ Every kind the catalog declares, in its declared order — also the binary tag order."""

COLLECTIONS = {"slots": "slots", "edges": "edges", "tiles": "tiles", "rules": "rules"}
"""🗃️ The four ordered id-keyed collections, each with its own removed/upserted diff pair."""

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
"""🔺️ A diff is TOTAL: every lane is present, an untouched one empty."""
# endregion 🔖️Vocabulary


# region 🔖️Algebra
def ordered_index(rows, identifier):
    """🔢 The position `identifier` occupies in an ascending-`id` collection."""
    for index, row in enumerate(rows):
        if row["id"] > identifier:
            return index
    return len(rows)


def diff_of(**lanes):
    """🔺️ One total diff with the named lanes filled in."""
    row = dict(EMPTY_DIFF)
    row.update(lanes)
    return row


def refuse(level, code):
    return diff_of(), [{"level": level, "code": code}]


def noop():
    return diff_of(), [{"level": "warning", "code": "mutation.no-op"}]


def find(rows, identifier):
    for index, row in enumerate(rows):
        if row["id"] == identifier:
            return index, row
    return -1, None


def strip_pin(slot):
    row = dict(slot)
    row.pop("pinnedTileId", None)
    return row


def mutate(document, mutation):
    """🔺️ The whole dispatch: `(diff, messages)` for one externally tagged mutation."""
    [(variant, payload)] = mutation.items()
    if variant == "ChangeSeed":
        return noop() if document["seed"] == payload["seed"] else (diff_of(seed=payload["seed"]), [])
    if variant == "CreateSlot":
        slot = payload["slot"]
        if find(document["slots"], slot["id"])[0] != -1:
            return refuse("fatal", "mutation.duplicate-id")
        if slot["width"] <= 0 or slot["height"] <= 0:
            return refuse("fatal", "mutation.invariant")
        if "pinnedTileId" in slot and find(document["tiles"], slot["pinnedTileId"])[0] == -1:
            return refuse("fatal", "mutation.invariant")
        return diff_of(slotsUpserted=[[ordered_index(document["slots"], slot["id"]), slot]]), []
    if variant == "DeleteSlot":
        identifier = payload["id"]
        if find(document["slots"], identifier)[0] == -1:
            return refuse("error", "mutation.target-missing")
        incident = [edge["id"] for edge in document["edges"] if identifier in (edge["fromSlotId"], edge["toSlotId"])]
        messages = [] if not incident else [{"level": "info", "code": "mutation.cascade"}]
        return diff_of(slotsRemoved=[identifier], edgesRemoved=incident), messages
    if variant == "MoveSlot":
        index, slot = find(document["slots"], payload["id"])
        if index == -1:
            return refuse("error", "mutation.target-missing")
        if slot["x"] == payload["x"] and slot["y"] == payload["y"]:
            return noop()
        return diff_of(slotsUpserted=[[index, {**slot, "x": payload["x"], "y": payload["y"]}]]), []
    if variant == "ResizeSlot":
        index, slot = find(document["slots"], payload["id"])
        if index == -1:
            return refuse("error", "mutation.target-missing")
        if payload["width"] <= 0 or payload["height"] <= 0:
            return refuse("fatal", "mutation.invariant")
        if slot["width"] == payload["width"] and slot["height"] == payload["height"]:
            return noop()
        return diff_of(slotsUpserted=[[index, {**slot, "width": payload["width"], "height": payload["height"]}]]), []
    if variant == "ConnectSlots":
        edge = payload["edge"]
        if find(document["edges"], edge["id"])[0] != -1:
            return refuse("fatal", "mutation.duplicate-id")
        for endpoint in (edge["fromSlotId"], edge["toSlotId"]):
            if find(document["slots"], endpoint)[0] == -1:
                return refuse("fatal", "mutation.invariant")
        if not edge["relation"]:
            return refuse("fatal", "mutation.invariant")
        return diff_of(edgesUpserted=[[ordered_index(document["edges"], edge["id"]), edge]]), []
    if variant == "DisconnectSlots":
        if find(document["edges"], payload["id"])[0] == -1:
            return refuse("error", "mutation.target-missing")
        return diff_of(edgesRemoved=[payload["id"]]), []
    if variant == "PinSlot":
        index, slot = find(document["slots"], payload["id"])
        if index == -1:
            return refuse("error", "mutation.target-missing")
        if find(document["tiles"], payload["tileId"])[0] == -1:
            return refuse("fatal", "mutation.invariant")
        if slot.get("pinnedTileId") == payload["tileId"]:
            return noop()
        return diff_of(slotsUpserted=[[index, {**slot, "pinnedTileId": payload["tileId"]}]]), []
    if variant == "UnpinSlot":
        index, slot = find(document["slots"], payload["id"])
        if index == -1:
            return refuse("error", "mutation.target-missing")
        if "pinnedTileId" not in slot:
            return noop()
        return diff_of(slotsUpserted=[[index, strip_pin(slot)]]), []
    if variant == "CreateTile":
        tile = payload["tile"]
        if find(document["tiles"], tile["id"])[0] != -1:
            return refuse("fatal", "mutation.duplicate-id")
        if tile["weight"] < 0:
            return refuse("fatal", "mutation.invariant")
        return diff_of(tilesUpserted=[[ordered_index(document["tiles"], tile["id"]), tile]]), []
    if variant == "DeleteTile":
        identifier = payload["id"]
        if find(document["tiles"], identifier)[0] == -1:
            return refuse("error", "mutation.target-missing")
        orphaned = [rule["id"] for rule in document["rules"] if identifier in (rule["tileAId"], rule["tileBId"])]
        released = [[index, strip_pin(slot)] for index, slot in enumerate(document["slots"]) if slot.get("pinnedTileId") == identifier]
        messages = [] if not orphaned and not released else [{"level": "info", "code": "mutation.cascade"}]
        return diff_of(tilesRemoved=[identifier], rulesRemoved=orphaned, slotsUpserted=released), messages
    if variant == "ChangeTileWeight":
        index, tile = find(document["tiles"], payload["tileId"])
        if index == -1:
            return refuse("error", "mutation.target-missing")
        if payload["weight"] < 0:
            return refuse("fatal", "mutation.invariant")
        if tile["weight"] == payload["weight"]:
            return noop()
        return diff_of(tilesUpserted=[[index, {**tile, "weight": payload["weight"]}]]), []
    if variant == "ChangeTileMedia":
        index, tile = find(document["tiles"], payload["tileId"])
        if index == -1:
            return refuse("error", "mutation.target-missing")
        if tile.get("media") == payload["media"]:
            return noop()
        return diff_of(tilesUpserted=[[index, {**tile, "media": payload["media"]}]]), []
    if variant == "CreateRule":
        rule = payload["rule"]
        if find(document["rules"], rule["id"])[0] != -1:
            return refuse("fatal", "mutation.duplicate-id")
        for tile_id in (rule["tileAId"], rule["tileBId"]):
            if find(document["tiles"], tile_id)[0] == -1:
                return refuse("fatal", "mutation.invariant")
        return diff_of(rulesUpserted=[[ordered_index(document["rules"], rule["id"]), rule]]), []
    if variant == "DeleteRule":
        if find(document["rules"], payload["id"])[0] == -1:
            return refuse("error", "mutation.target-missing")
        return diff_of(rulesRemoved=[payload["id"]]), []
    raise AssertionError(f"unknown wfc2d mutation variant {variant!r}")


def apply_collection(base, removed, upserted):
    rows = [row for row in base if row["id"] not in removed]
    for index, value in upserted:
        at = next((position for position, row in enumerate(rows) if row["id"] == value["id"]), -1)
        if at == -1:
            rows.insert(index, value)
        else:
            rows[at] = value
    return rows


def apply_diff(document, diff):
    """🩹 Applies a total diff — the Rust `MutationDiff::apply` twin."""
    result = copy.deepcopy(document)
    if diff["schema"] is not None:
        result["schema"] = diff["schema"]
    if diff["seed"] is not None:
        result["seed"] = diff["seed"]
    for member in COLLECTIONS:
        lane = member[:-1] if member.endswith("s") else member
        result[member] = apply_collection(result[member], diff[f"{member}Removed"], diff[f"{member}Upserted"])
        del lane
    return result


def inverse(document, mutation):
    """↩️ The inverse steps, mirroring each `↩️inverse/🦀️.rs` leaf exactly."""
    [(variant, payload)] = mutation.items()
    if variant == "ChangeSeed":
        return [{"ChangeSeed": {"seed": document["seed"]}}]
    if variant == "CreateSlot":
        return [{"DeleteSlot": {"id": payload["slot"]["id"]}}]
    if variant == "DeleteSlot":
        index, slot = find(document["slots"], payload["id"])
        if index == -1:
            return []
        steps = [{"CreateSlot": {"slot": slot}}]
        steps += [{"ConnectSlots": {"edge": edge}} for edge in document["edges"] if payload["id"] in (edge["fromSlotId"], edge["toSlotId"])]
        return steps
    if variant == "MoveSlot":
        _, slot = find(document["slots"], payload["id"])
        return [] if slot is None else [{"MoveSlot": {"id": slot["id"], "x": slot["x"], "y": slot["y"]}}]
    if variant == "ResizeSlot":
        _, slot = find(document["slots"], payload["id"])
        return [] if slot is None else [{"ResizeSlot": {"id": slot["id"], "width": slot["width"], "height": slot["height"]}}]
    if variant == "ConnectSlots":
        return [{"DisconnectSlots": {"id": payload["edge"]["id"]}}]
    if variant == "DisconnectSlots":
        _, edge = find(document["edges"], payload["id"])
        return [] if edge is None else [{"ConnectSlots": {"edge": edge}}]
    if variant == "PinSlot":
        _, slot = find(document["slots"], payload["id"])
        if slot is None:
            return []
        previous = slot.get("pinnedTileId")
        return [{"UnpinSlot": {"id": slot["id"]}}] if previous is None else [{"PinSlot": {"id": slot["id"], "tileId": previous}}]
    if variant == "UnpinSlot":
        _, slot = find(document["slots"], payload["id"])
        if slot is None or slot.get("pinnedTileId") is None:
            return []
        return [{"PinSlot": {"id": slot["id"], "tileId": slot["pinnedTileId"]}}]
    if variant == "CreateTile":
        return [{"DeleteTile": {"id": payload["tile"]["id"]}}]
    if variant == "DeleteTile":
        identifier = payload["id"]
        _, tile = find(document["tiles"], identifier)
        if tile is None:
            return []
        steps = [{"CreateTile": {"tile": tile}}]
        steps += [{"CreateRule": {"rule": rule}} for rule in document["rules"] if identifier in (rule["tileAId"], rule["tileBId"])]
        steps += [{"PinSlot": {"id": slot["id"], "tileId": identifier}} for slot in document["slots"] if slot.get("pinnedTileId") == identifier]
        return steps
    if variant == "ChangeTileWeight":
        _, tile = find(document["tiles"], payload["tileId"])
        return [] if tile is None else [{"ChangeTileWeight": {"tileId": tile["id"], "weight": tile["weight"]}}]
    if variant == "ChangeTileMedia":
        _, tile = find(document["tiles"], payload["tileId"])
        return [] if tile is None else [{"ChangeTileMedia": {"tileId": tile["id"], "media": tile.get("media")}}]
    if variant == "CreateRule":
        return [{"DeleteRule": {"id": payload["rule"]["id"]}}]
    _, rule = find(document["rules"], payload["id"])
    return [] if rule is None else [{"CreateRule": {"rule": rule}}]


# endregion 🔖️Algebra


# region 🔖️Replay
FIXTURES = pathlib.Path(__file__).resolve().parents[1].parent / "🧫️fixtures" / "🧬️mutations"


def read(path):
    return json.loads(path.read_text(encoding="utf-8"))


def replay():
    """▶️ Replays every committed quintet; returns the list of divergences it found."""
    problems = []
    seen = set()
    for kind_dir in sorted(FIXTURES.iterdir()):
        if not kind_dir.is_dir():
            continue
        seen.add(re.search(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+$", kind_dir.name).group(0))
        for case_dir in sorted(kind_dir.iterdir()):
            if not case_dir.is_dir():
                continue
            label = f"{kind_dir.name}/{case_dir.name}"
            before = read(case_dir / "📸️snapshot" / "⬅️before" / "🔣️.json")
            after = read(case_dir / "📸️snapshot" / "➡️after" / "🔣️.json")
            mutation = read(case_dir / "🦠️mutation" / "🔣️.json")
            committed_diff = read(case_dir / "🔺️diff" / "🔣️.json")
            outcome = read(case_dir / "🎯️outcome" / "🔣️.json")
            produced_diff, messages = mutate(before, mutation)
            if produced_diff != committed_diff:
                problems.append(f"{label}: produced diff differs from the committed 🔺️diff")
            declared = [(row["level"], row["code"]) for row in outcome.get("messages", [])]
            if [(row["level"], row["code"]) for row in messages] != declared:
                problems.append(f"{label}: diagnostics differ from the committed 🎯️outcome ({messages} vs {declared})")
            produced_after = apply_diff(before, committed_diff)
            if produced_after != after:
                problems.append(f"{label}: the committed diff did not carry before to after")
            restored = apply_diff(apply_diff(before, produced_diff), diff_of())
            for step in inverse(before, mutation):
                step_diff, _ = mutate(restored, step)
                restored = apply_diff(restored, step_diff)
            if restored != before:
                problems.append(f"{label}: the inverse did not restore the before-snapshot")
            for member in MEMBERS:
                if member not in before:
                    problems.append(f"{label}: the before-snapshot is missing member {member}")
    missing = [kind for kind in KINDS if kind not in seen]
    if missing:
        problems.append(f"no fixture case for: {', '.join(missing)}")
    return problems


def main():
    problems = replay()
    for problem in problems:
        print(f"✗ {problem}")
    print(f"{'✗' if problems else '✓'} wfc2d oracle: {len(KINDS)} kinds, {len(problems)} divergence(s)")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
# endregion 🔖️Replay
