"""🧪️ S2-W2C third-party check: every shared shell corpus the wgpu shell reads validates against its JSON Schema
(python `jsonschema` + `referencing`, with the kernel history-patch schema registered for its `$ref`s).

Run: `cd /Users/ueli/Documents/semio && python3 <this file>`; prints one line per corpus and exits non-zero on a breach.
"""

import json
import pathlib
import sys

from jsonschema import Draft7Validator
from referencing import Registry, Resource

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
HELPERS = ENGINE / "🧱️elements/🛠️ShellHelpers"
KERNEL = ROOT / "🧰️framework/🔨️modules/🎠️kernel/🧬️schema/🔣️history-patch/🔣️.json"

CORPORA = [
    (HELPERS / "🧫️fixtures/🧫️time-travel-focus/🔣️.json", HELPERS / "🧬️schema/🔣️time-travel-focus/🔣️.json"),
    (HELPERS / "🧫️fixtures/🧫️time-travel-band/🔣️.json", HELPERS / "🧬️schema/🔣️time-travel-band/🔣️.json"),
    (ENGINE / "🧫️fixtures/📎️local-folder-bindings/🔣️.json", ENGINE / "🧬️schema/📎️local-folder-bindings/🔣️.json"),
]


def load(path):
    return json.loads(path.read_text())


kernel = load(KERNEL)
registry = Registry().with_resource(kernel["$id"], Resource.from_contents(kernel))
failed = False
for corpus_path, schema_path in CORPORA:
    if not schema_path.exists():
        print(f"SKIP {corpus_path.parent.name}: no schema at {schema_path.relative_to(ROOT)}")
        continue
    schema = load(schema_path)
    Draft7Validator.check_schema(schema)
    errors = sorted(Draft7Validator(schema, registry=registry).iter_errors(load(corpus_path)), key=lambda error: list(error.path))
    for error in errors:
        print(f"FAIL {corpus_path.parent.name}: /{'/'.join(map(str, error.path))}: {error.message}")
    failed = failed or bool(errors)
    if not errors:
        print(f"PASS {corpus_path.parent.name}")
sys.exit(1 if failed else 0)
