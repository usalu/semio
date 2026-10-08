import json
import sys
from pathlib import Path

import jsonschema
from referencing import Registry, Resource

root = Path(sys.argv[1])
schema_dir = root / "🧬️schema"
snapshot_schema = json.loads((schema_dir / "📸️snapshot" / "🔣️.json").read_text(encoding="utf-8"))
artifact_schema = json.loads((schema_dir / "🔣️.json").read_text(encoding="utf-8"))
print("artifact $id", artifact_schema.get("$id"))
registry = Registry().with_resources([
    ("https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json", Resource.from_contents(artifact_schema)),
    (snapshot_schema["$id"], Resource.from_contents(snapshot_schema)),
])
validator = jsonschema.Draft7Validator(snapshot_schema, registry=registry)
snapshot = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
errors = sorted(validator.iter_errors(snapshot), key=lambda error: list(error.path))
for error in errors[:20]:
    print("ERR", list(error.path), error.message[:200])
print("errors:", len(errors))
