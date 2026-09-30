#!/usr/bin/env python3
"""🔎️ W2-S glTF third-party check (Python `jsonschema` Draft7 + `referencing`) of the shared snapshot/diff documents,
the phase-tagged leaf schemas and the committed fixture corpus.

Checks: every `🦠️mutation` fixture against the aggregate and its leaf (the editable content: `payload.value` of a wrapped
`#[mutation_leaf(payload = Apply)]` leaf, else `payload`), every snapshot fixture
against `snapshot.json`, every committed `🔺️diff` against `diff.json`, the mutation-carriers apply/restore wires against the
aggregate, every `x-semio-ui` against the manifest `$defs/InputUi`, and that no leaf restates the diff (no `GltfDiff` def).

Run from the repository root: `.venv/bin/python <ticket>/🧪️w2-s-gltf-check.py`.
"""
import json
import sys
from pathlib import Path

from jsonschema import Draft7Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

ROOT = Path(__file__).resolve().parents[7]
ANY = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any"
SCHEMA = ANY / "🧬️schema"
LEAVES = SCHEMA / "🧬️mutations"
FIXTURES = ANY / "🧫️fixtures/🧬️mutations"
MANIFEST = ROOT / "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"


def load(path):
    return json.loads(path.read_text())


documents = [SCHEMA / "📸️snapshot/🔣️.json", SCHEMA / "🔺️diff/🔣️.json", SCHEMA / "🔺️diff/📝️text/🔣️.json", LEAVES / "🔣️.json", MANIFEST, *sorted(LEAVES.glob("*/*/🧬️schema/🔣️.json"))]
schemas = {path: load(path) for path in documents}
registry = Registry().with_resources((schema["$id"], Resource(schema, specification=DRAFT7)) for schema in schemas.values())
validator = lambda schema: Draft7Validator(schema, registry=registry)
for path, schema in schemas.items():
    Draft7Validator.check_schema(schema)

failures = []
counts = {}


def check(label, schema, instance, where):
    errors = sorted(validator(schema).iter_errors(instance), key=lambda error: list(error.path))
    counts[label] = counts.get(label, 0) + 1
    if errors:
        failures.append(f"{label} {where}: {errors[0].json_path} {errors[0].message[:240]}")


snapshot = schemas[SCHEMA / "📸️snapshot/🔣️.json"]
diff = schemas[SCHEMA / "🔺️diff/🔣️.json"]
aggregate = schemas[LEAVES / "🔣️.json"]
leaf_by_wire = {branch["properties"]["mutation"]["const"]: branch["properties"]["payload"]["$ref"] for branch in aggregate["oneOf"]}
leaf_by_id = {schema["$id"]: schema for schema in schemas.values()}

for fixture in sorted(FIXTURES.rglob("🦠️mutation/🔣️.json")):
    wire = load(fixture)
    where = str(fixture.relative_to(FIXTURES).parent.parent)
    check("aggregate", aggregate, wire, where)
    target = leaf_by_wire[wire["mutation"]]
    content = wire["payload"]
    check("leaf", leaf_by_id[target.split("#")[0]], content["value"] if "phase" in content else content, where)
for side in sorted(FIXTURES.rglob("📸️snapshot/*/🔣️.json")):
    check("snapshot", snapshot, load(side), str(side.relative_to(FIXTURES)))
for committed in sorted(FIXTURES.rglob("🔺️diff/🔣️.json")):
    check("diff", diff, load(committed), str(committed.relative_to(FIXTURES)))
carriers = load(LEAVES / "🧫️fixtures/📨️mutation-carriers/🔣️.json")
for phase in ("apply", "restore"):
    check("carrier", aggregate, carriers[phase], phase)

input_ui = validator({"$ref": "https://json.schemas.assets.semio-tech.com/framework/manifest/schema.json#/$defs/InputUi"})


def annotations(node, path):
    if isinstance(node, dict):
        if "x-semio-ui" in node:
            counts["x-semio-ui"] = counts.get("x-semio-ui", 0) + 1
            for error in input_ui.iter_errors(node["x-semio-ui"]):
                failures.append(f"x-semio-ui {path}: {error.message[:200]}")
        for key, value in node.items():
            annotations(value, f"{path}/{key}")
    elif isinstance(node, list):
        for index, value in enumerate(node):
            annotations(value, f"{path}/{index}")


for path, schema in schemas.items():
    if path != MANIFEST:
        annotations(schema, str(path.relative_to(SCHEMA)))
    if path.parent.name == "🧬️schema" and path.parent.parent.parent.parent == LEAVES and "GltfDiff" in schema.get("$defs", {}):
        failures.append(f"leaf restates GltfDiff: {path.relative_to(LEAVES)}")

print(json.dumps(counts))
for failure in failures:
    print("✘", failure)
print(f"{len(failures)} failure(s)")
sys.exit(1 if failures else 0)
