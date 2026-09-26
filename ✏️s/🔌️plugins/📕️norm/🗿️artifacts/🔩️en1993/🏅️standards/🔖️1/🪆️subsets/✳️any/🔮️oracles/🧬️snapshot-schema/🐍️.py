#!/usr/bin/env python3
import json, sys
from pathlib import Path
try:
    import jsonschema
except ImportError as err:
    print(f"jsonschema is required for snapshot validation: {err}", file=sys.stderr)
    raise SystemExit(1)
schema = json.loads(Path(sys.argv[1]).read_text())
instance = json.loads(Path(sys.argv[2]).read_text())
if isinstance(instance.get("annex"), str):
    instance["annex"] = instance["annex"].lower()
jsonschema.validate(instance=instance, schema=schema)
print("snapshot schema OK")
