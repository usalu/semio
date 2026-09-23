#!/usr/bin/env python3
"""Validate the bitmap fill tick payload shape against the normative fixture vector."""

from __future__ import annotations

import base64
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
FIXTURE = ROOT / "bitmap-fill-oracle-vector.json"


def main() -> int:
    payload = json.loads(FIXTURE.read_text())
    assert isinstance(payload, dict), "payload must be an object"
    assert set(payload.keys()) == {"pixels", "decided", "width", "height", "contradiction", "done", "trace"}
    assert isinstance(payload["pixels"], str) and payload["pixels"]
    assert isinstance(payload["decided"], str) and payload["decided"]
    assert isinstance(payload["width"], int) and payload["width"] == 2
    assert isinstance(payload["height"], int) and payload["height"] == 2
    assert payload["contradiction"] is False
    assert payload["done"] is False
    assert payload["trace"] == [
        {"index": 0, "color": 0, "discarded": False},
        {"index": 3, "color": 1, "discarded": True},
    ]
    pixels = base64.b64decode(payload["pixels"])
    decided = base64.b64decode(payload["decided"])
    cells = payload["width"] * payload["height"]
    assert len(pixels) == cells
    assert len(decided) == cells
    decided_count = sum(1 for byte in decided if byte != 0)
    assert decided_count == 2, decided_count
    assert decided[0] != 0 and pixels[0] == 0, "palette index 0 stays a real colour on a decided cell"
    assert decided[3] != 0 and pixels[3] == 1
    assert decided[1] == 0 and decided[2] == 0
    print("ok", FIXTURE.name, "decided", decided_count)
    return 0


if __name__ == "__main__":
    sys.exit(main())
