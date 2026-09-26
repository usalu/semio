#!/usr/bin/env python3
"""Validate DIN 4108 snapshot JSON against snapshot JSON Schema (third-party jsonschema)."""
from __future__ import annotations

import json
import sys
from pathlib import Path

try:
    import jsonschema
except ImportError as exc:
    print(f"FAIL: jsonschema missing: {exc}", file=sys.stderr)
    raise SystemExit(1)


def main() -> int:
    if len(sys.argv) < 3:
        print("usage: validate_snapshot.py <schema.json> <snapshot.json>", file=sys.stderr)
        return 2
    schema = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    instance = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
    jsonschema.validate(instance=instance, schema=schema)
    assert "usage" in instance and "elements" in instance and "zones" in instance
    assert isinstance(instance["elements"], list) and len(instance["elements"]) >= 1
    wall = next(e for e in instance["elements"] if e.get("id") == "wall-north")
    assert "layers" in wall and isinstance(wall["layers"], list)
    print("snapshot schema OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
