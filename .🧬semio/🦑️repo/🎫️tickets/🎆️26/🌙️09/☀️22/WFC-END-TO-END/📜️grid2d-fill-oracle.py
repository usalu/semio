#!/usr/bin/env python3
"""Validate the grid2d fill tick payload shape against the normative fixture vector."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
FIXTURE = next(ROOT.glob("*partial-tick.json"))


def main() -> int:
    payload = json.loads(FIXTURE.read_text())
    assert isinstance(payload, dict), "payload must be an object"
    assert set(payload.keys()) == {"assignments", "contradiction", "done"}
    assert isinstance(payload["contradiction"], bool)
    assert isinstance(payload["done"], bool)
    assert isinstance(payload["assignments"], list)
    decided = 0
    for cell in payload["assignments"]:
        assert isinstance(cell, dict)
        assert "x" in cell and "y" in cell
        assert isinstance(cell["x"], int) and cell["x"] >= 0
        assert isinstance(cell["y"], int) and cell["y"] >= 0
        if "tileId" in cell:
            assert isinstance(cell["tileId"], str) and cell["tileId"]
            decided += 1
        else:
            assert set(cell.keys()) == {"x", "y"}
    assert decided == 2, decided
    assert payload["done"] is False
    assert payload["contradiction"] is False
    print("ok", FIXTURE.name, "decided", decided)
    return 0


if __name__ == "__main__":
    sys.exit(main())
