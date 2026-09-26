#!/usr/bin/env python3
"""Validate a FULL VDI 3805 snapshot JSON against the snapshot JSON Schema (third-party jsonschema)."""

from __future__ import annotations

import json
import sys
from pathlib import Path

try:
    import jsonschema
except ImportError as exc:  # pragma: no cover
    print("FAIL: third-party jsonschema dependency is missing", file=sys.stderr)
    print(exc, file=sys.stderr)
    raise SystemExit(1)

def main() -> int:
    if len(sys.argv) < 3:
        print("usage: validate_schema.py <schema.json> <snapshot.json>", file=sys.stderr)
        return 2
    schema = json.loads(Path(sys.argv[1]).read_text())
    instance = json.loads(Path(sys.argv[2]).read_text())
    jsonschema.validate(instance=instance, schema=schema)
    print("ok")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
