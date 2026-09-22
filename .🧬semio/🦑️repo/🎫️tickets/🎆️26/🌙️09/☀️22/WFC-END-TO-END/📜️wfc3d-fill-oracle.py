#!/usr/bin/env python3
"""Language-agnostic oracle for the wfc3d fill tick payload vector."""

from __future__ import annotations

import json
import sys
from pathlib import Path

FIXTURE = (
    Path("✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any")
    / "✏️editor/🎭️modes/✏️edit/🛠️tools/📑️fill/🧫️fixtures/🎞️partial-tick.json"
)


def validate(payload: dict) -> None:
    assert isinstance(payload, dict)
    assert set(payload) >= {"assignments", "contradiction", "done"}
    assert isinstance(payload["assignments"], dict)
    assert isinstance(payload["contradiction"], bool)
    assert isinstance(payload["done"], bool)
    decided = sum(1 for value in payload["assignments"].values() if value is not None)
    assert decided == 2
    assert payload["assignments"]["room-a"] == "room"
    assert payload["assignments"]["corridor"] is None
    assert payload["assignments"]["room-b"] == "room"
    assert payload["contradiction"] is False
    assert payload["done"] is False


def main() -> int:
    repo = Path(__file__).resolve()
    while repo.name != "semio" and repo != repo.parent:
        repo = repo.parent
    path = repo / FIXTURE
    payload = json.loads(path.read_text())
    validate(payload)
    print("ok", path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
