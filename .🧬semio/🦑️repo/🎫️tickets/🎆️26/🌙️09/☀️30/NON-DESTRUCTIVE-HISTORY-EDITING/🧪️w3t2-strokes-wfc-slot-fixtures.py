"""🧫️ W3-T2-STROKES: writes the committed `drag-slots` / `set-slot-positions` quintets of `s.wfc.wfc2d` and
`s.wfc.wfc3d` from an independent Python computation of both leaves (base-order upserts, partial / no-op /
target-missing outcomes), in each subset's own canonical form — 2d pretty-printed, 3d compact — so the Rust fixture
laws and this computation must agree byte for byte.

Run from anywhere: `python3 <this file>`.
"""

import json
import os

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
WFC = os.path.join(ROOT, "✏️s", "🔌️plugins", "🀄️wfc", "🗿️artifacts")
SUBSET_2D = os.path.join(WFC, "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧫️fixtures", "🧬️mutations")
SUBSET_3D = os.path.join(WFC, "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧫️fixtures", "🧬️mutations")
BASE_2D = os.path.join(SUBSET_2D, "↔️move-slot", "↔️drags-slot-b-down", "📸️snapshot", "⬅️before", "🔣️.json")
BASE_3D = os.path.join(SUBSET_3D, "🚚️move-slot", "🚚️lifts-room-b-one-storey", "📸️snapshot", "⬅️before", "🔣️.json")

EMPTY_DIFF = ["schema", "seed", "slotsRemoved", "slotsUpserted", "edgesRemoved", "edgesUpserted", "tilesRemoved", "tilesUpserted", "rulesRemoved", "rulesUpserted"]


def total_diff(slots_upserted):
    return {key: (None if key in ("schema", "seed") else (slots_upserted if key == "slotsUpserted" else [])) for key in EMPTY_DIFF}


def drag(before, payload, axes):
    present = {slot["id"] for slot in before["slots"]}
    missing = [target for target in payload["targets"] if target not in present]
    messages = [{"level": "warning", "code": "mutation.partial"}] if missing else []
    upserted = []
    for index, slot in enumerate(before["slots"]):
        if slot["id"] in payload["targets"]:
            moved = dict(slot)
            for axis in axes:
                moved[axis] = slot[axis] + payload["d" + axis]
            upserted.append([index, moved])
    return upserted, messages


def set_positions(before, payload, axes):
    present = {slot["id"] for slot in before["slots"]}
    missing = [position["id"] for position in payload["positions"] if position["id"] not in present]
    messages = [{"level": "warning", "code": "mutation.partial"}] if missing else []
    upserted = []
    for index, slot in enumerate(before["slots"]):
        position = next((row for row in payload["positions"] if row["id"] == slot["id"]), None)
        if position is not None and any(position[axis] != slot[axis] for axis in axes):
            moved = dict(slot)
            for axis in axes:
                moved[axis] = position[axis]
            upserted.append([index, moved])
    return upserted, messages


def apply(before, upserted):
    after = json.loads(json.dumps(before))
    for index, slot in upserted:
        after["slots"][index] = slot
    return after


CASES_2D = [
    ("✋️drag-slots", "✋️drags-slots-a-and-b-together", "DragSlots", {"targets": ["slot-b", "slot-a"], "dx": 1.5, "dy": -0.5}),
    ("✋️drag-slots", "⚠️skips-a-slot-the-board-lacks", "DragSlots", {"targets": ["slot-c", "slot-z"], "dx": 0.25, "dy": 1.0}),
    ("🎯️set-slot-positions", "🎯️sets-slots-a-and-c", "SetSlotPositions", {"positions": [{"id": "slot-c", "x": 5.0, "y": 1.0}, {"id": "slot-a", "x": -1.0, "y": 2.5}]}),
]

CASES_3D = [
    ("✋️drag-slots", "✋️lifts-both-rooms-two-storeys", "DragSlots", {"targets": ["room-a", "room-b"], "dx": 0.5, "dy": 1.0, "dz": 2.0}),
    ("✋️drag-slots", "⚠️skips-a-slot-the-corridor-lacks", "DragSlots", {"targets": ["corridor", "ghost"], "dx": -1.0, "dy": 0.0, "dz": 0.5}),
    ("🎯️set-slot-positions", "🎯️sets-the-corridor-and-room-b", "SetSlotPositions", {"positions": [{"id": "room-b", "x": 4.0, "y": 0.0, "z": 1.0}, {"id": "corridor", "x": 1.0, "y": 2.0, "z": 0.0}]}),
]


def emit(root, base_path, cases, axes, dump, base_text):
    with open(base_path, encoding="utf-8") as handle:
        before = json.load(handle)
    for directory, case, variant, payload in cases:
        upserted, messages = (drag if variant == "DragSlots" else set_positions)(before, payload, axes)
        outcome = {"status": "applied", "messages": messages} if messages else {"status": "applied"}
        files = {
            ("📸️snapshot", "⬅️before"): None,
            ("📸️snapshot", "➡️after"): apply(before, upserted),
            ("🦠️mutation",): {variant: payload},
            ("🔺️diff",): total_diff(upserted),
            ("🎯️outcome",): outcome,
        }
        for parts, value in files.items():
            path = os.path.join(root, directory, case, *parts, "🔣️.json")
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(base_text if value is None else dump(value))
        print("wrote", directory, case, "upserted", [row[1]["id"] for row in upserted], "messages", messages)


def main():
    with open(BASE_2D, encoding="utf-8") as handle:
        base_2d = handle.read()
    with open(BASE_3D, encoding="utf-8") as handle:
        base_3d = handle.read()
    emit(SUBSET_2D, BASE_2D, CASES_2D, ("x", "y"), lambda value: json.dumps(value, indent=2, ensure_ascii=False) + "\n", base_2d)
    emit(SUBSET_3D, BASE_3D, CASES_3D, ("x", "y", "z"), lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False), base_3d)


if __name__ == "__main__":
    main()
