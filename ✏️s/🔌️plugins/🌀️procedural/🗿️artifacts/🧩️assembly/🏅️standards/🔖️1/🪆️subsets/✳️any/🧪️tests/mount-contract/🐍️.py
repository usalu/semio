#!/usr/bin/env python3
"""Language-agnostic assembly mount contract — json is the third-party twin."""
import json
from pathlib import Path

EXPECTED = {
    "editorAppId": "s.assembly@1/*#editor",
    "viewerAppId": "s.assembly@1/*#viewer",
    "windowKindId": "framework.window.tree",
    "windowLabel": {"en": "Structure", "de": "Struktur"},
    "examples": [
        {"id": "two-room-corridor", "label": {"en": "Two Rooms And A Corridor", "de": "Zwei Räume und ein Korridor"}, "minSlots": 3},
        {"id": "wall-roof-facade-strip", "label": {"en": "Wall And Roof Facade Strip", "de": "Wand-Dach-Fassadenstreifen"}, "minSlots": 4},
    ],
}

def assert_mount_contract(payload: dict) -> None:
    raw = json.dumps(payload, sort_keys=True, ensure_ascii=False)
    parsed = json.loads(raw)
    assert parsed["editorAppId"] == EXPECTED["editorAppId"]
    assert parsed["viewerAppId"] == EXPECTED["viewerAppId"]
    assert parsed["windowKindId"] == EXPECTED["windowKindId"]
    assert parsed["windowLabel"] == EXPECTED["windowLabel"]
    by_id = {row["id"]: row for row in parsed["examples"]}
    for example in EXPECTED["examples"]:
        got = by_id[example["id"]]
        assert got["label"] == example["label"]
        assert got["slots"] >= example["minSlots"]

if __name__ == "__main__":
    vector = Path(__file__).with_name("expected.json")
    assert_mount_contract(json.loads(vector.read_text()))
    print("ok")
