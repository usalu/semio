#!/usr/bin/env python3
import json, sys
from pathlib import Path
try:
    import jsonschema
except ImportError as exc:
    print(f"jsonschema is required: {exc}", file=sys.stderr)
    raise SystemExit(1)
schema = json.loads(Path(sys.argv[1]).read_text())
instance = json.loads(Path(sys.argv[2]).read_text())
jsonschema.validate(instance=instance, schema=schema)
print("snapshot schema OK")
