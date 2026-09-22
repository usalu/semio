#!/usr/bin/env python3
"""Language-agnostic check of the grid3d fill tick payload shape."""

import json
import sys
from pathlib import Path

VECTOR = {
    "assignments": [
        {"x": 0, "y": 0, "z": 0, "tileId": "air"},
        {"x": 1, "y": 0, "z": 0},
        {"x": 0, "y": 1, "z": 0, "tileId": "wall"},
        {"x": 1, "y": 1, "z": 0},
        {"x": 0, "y": 0, "z": 1},
        {"x": 1, "y": 0, "z": 1, "tileId": "roof"},
    ],
    "contradiction": False,
    "done": False,
}


def validate(payload: dict) -> None:
    assert set(payload.keys()) == {"assignments", "contradiction", "done"}
    assert isinstance(payload["contradiction"], bool)
    assert isinstance(payload["done"], bool)
    assert isinstance(payload["assignments"], list)
    decided = 0
    for cell in payload["assignments"]:
        assert "x" in cell and "y" in cell and "z" in cell
        assert isinstance(cell["x"], int) and cell["x"] >= 0
        assert isinstance(cell["y"], int) and cell["y"] >= 0
        assert isinstance(cell["z"], int) and cell["z"] >= 0
        if "tileId" in cell:
            assert isinstance(cell["tileId"], str) and len(cell["tileId"]) >= 1
            decided += 1
    assert decided == 3
    assert payload["done"] is False
    assert payload["contradiction"] is False


def main() -> int:
    validate(VECTOR)
    fixture = Path(__file__).resolve().parents[0]
    # Prefer the artifact fixture when this script is copied next to a checkout path argument.
    if len(sys.argv) > 1:
        validate(json.loads(Path(sys.argv[1]).read_text()))
    print("grid3d-fill-oracle: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
