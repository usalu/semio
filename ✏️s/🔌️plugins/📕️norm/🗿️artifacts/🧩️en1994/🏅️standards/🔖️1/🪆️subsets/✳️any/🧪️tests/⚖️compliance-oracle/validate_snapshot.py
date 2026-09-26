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
jsonschema.validate(instance=instance, schema=schema)
print("snapshot schema OK")
