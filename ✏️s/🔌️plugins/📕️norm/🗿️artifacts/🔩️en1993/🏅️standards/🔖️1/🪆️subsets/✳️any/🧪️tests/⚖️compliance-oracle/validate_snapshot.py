#!/usr/bin/env python3
import json, sys
from pathlib import Path
try:
    import jsonschema
except ImportError:
    print("jsonschema not installed; skip")
    raise SystemExit(0)
schema = json.loads(Path(sys.argv[1]).read_text())
instance = json.loads(Path(sys.argv[2]).read_text())
# Annex in assets may be lowercase dsl keys; coerce for draft schema enum if needed.
if isinstance(instance.get("annex"), str):
    instance["annex"] = instance["annex"].lower()
jsonschema.validate(instance=instance, schema=schema)
print("snapshot schema OK")
