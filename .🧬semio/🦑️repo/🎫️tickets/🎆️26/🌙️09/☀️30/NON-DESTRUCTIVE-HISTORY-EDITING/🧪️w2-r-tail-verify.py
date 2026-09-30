#!/usr/bin/env python3
"""✅️ W2-R tail verification with third-party Python `jsonschema` (Draft 7 + `referencing`):
1. every `x-semio-ui` in the tail group's leaf payload schemas validates against the manifest `$defs/InputUi` meta-schema;
2. every leaf payload schema of the group is a valid Draft-7 schema;
3. every committed fixture payload (`🧫️fixtures/🧬️mutations/<leaf>/<case>/🦠️mutation/🔣️.json`) gets the same verdict from the
   updated leaf schema as from the pre-annotation schema (`originals.json`), and the count of accepted payloads is reported.
Usage: .venv/bin/python 🧪️w2-r-tail-verify.py <originals.json>"""
import json
import os
import sys

import jsonschema
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

REPO = "/Users/ueli/Documents/semio"
CATALOG = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"
ROOTS = ["✒️writer", "➗️mathematical", "🌊️flow", "🌍️gis", "🌿️vcs", "🎞️animate", "🎪️demonstrator", "🎬️sequence", "🏭️process", "💠️lowpoly", "💡️reasoning", "📖️playbook", "📜️imperative", "🔱️trinity", "🕸️dag", "🪐️space", "🪵️sourcing"]


def load(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return json.load(handle)


def registry():
    resources = []
    seen = set()
    for scope in load(CATALOG)["scopes"].values():
        base = os.path.join(REPO, scope["path"])
        for directory, _, names in os.walk(base) if os.path.isdir(base) else []:
            for name in names:
                path = os.path.join(directory, name)
                if not name.endswith(".json") or path in seen:
                    continue
                seen.add(path)
                try:
                    document = json.load(open(path, encoding="utf-8"))
                except Exception:
                    continue
                if isinstance(document, dict) and isinstance(document.get("$id"), str):
                    resources.append((document["$id"], Resource(contents=document, specification=DRAFT7)))
    manifest = load(MANIFEST)
    resources.append((manifest["$id"], Resource(contents=manifest, specification=DRAFT7)))
    return Registry().with_resources(resources), manifest["$id"]


def annotations(node, path=""):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "x-semio-ui":
                yield path, value
            else:
                yield from annotations(value, path + "/" + key)
    elif isinstance(node, list):
        for index, value in enumerate(node):
            yield from annotations(value, "%s/%d" % (path, index))


def leaves():
    for root in ROOTS:
        for directory, subdirs, names in os.walk(os.path.join(REPO, "✏️s/🔌️plugins", root)):
            subdirs[:] = [d for d in subdirs if d not in ("node_modules", "target", "🗑️generated")]
            parts = directory.split("/")
            if parts[-1] == "🧬️schema" and len(parts) > 2 and parts[-3] == "🧬️mutations" and "🔣️.json" in names:
                yield os.path.join(directory, "🔣️.json")


def fixtures(schema_path):
    leaf_dir = os.path.dirname(os.path.dirname(schema_path))
    marker = "/🧬️schema/🧬️mutations/"
    head, _, leaf = leaf_dir.rpartition(marker)
    base = os.path.join(head, "🧫️fixtures", "🧬️mutations", leaf)
    for directory, _, names in os.walk(base) if os.path.isdir(base) else []:
        if os.path.basename(directory) == "🦠️mutation" and "🔣️.json" in names:
            yield os.path.join(directory, "🔣️.json")


def verdict(schema, payload, registry_value):
    properties = schema.get("properties") if isinstance(schema.get("properties"), dict) else None
    if isinstance(payload, dict) and len(payload) == 1 and next(iter(payload))[:1].isupper() and isinstance(next(iter(payload.values())), dict) and (properties is None or next(iter(payload)) not in properties):
        payload = next(iter(payload.values()))
    if isinstance(payload, dict) and "mutation" in payload and (properties is None or "mutation" not in properties):
        payload = {key: value for key, value in payload.items() if key != "mutation"}
    try:
        errors = sorted(jsonschema.Draft7Validator(schema, registry=registry_value).iter_errors(payload), key=lambda error: list(error.path))
        return "ok" if not errors else "invalid: %s" % errors[0].message[:120]
    except Exception as error:
        return "unresolvable: %s" % type(error).__name__


def main():
    originals = json.load(open(sys.argv[1], encoding="utf-8")) if len(sys.argv) > 1 else {}
    registry_value, manifest_id = registry()
    input_ui = jsonschema.Draft7Validator({"$ref": manifest_id + "#/$defs/InputUi"}, registry=registry_value)
    ui_count = ui_errors = schema_errors = payloads = accepted = changed = 0
    verdicts = {}
    for path in sorted(leaves()):
        schema = json.load(open(path, encoding="utf-8"))
        try:
            jsonschema.Draft7Validator.check_schema(schema)
        except jsonschema.SchemaError as error:
            schema_errors += 1
            print("SCHEMA", path, error.message[:160])
        for pointer, value in annotations(schema):
            ui_count += 1
            for error in input_ui.iter_errors(value):
                ui_errors += 1
                print("UI", path, pointer, error.message[:160])
        original = json.loads(originals[path]) if path in originals else schema
        for fixture in sorted(fixtures(path)):
            payload = json.load(open(fixture, encoding="utf-8"))
            after = verdict(schema, payload, registry_value)
            before = verdict(original, payload, registry_value)
            payloads += 1
            accepted += after == "ok"
            verdicts[after.split(":")[0]] = verdicts.get(after.split(":")[0], 0) + 1
            if after != before:
                changed += 1
                print("VERDICT CHANGED", fixture, before, "->", after)
            elif after != "ok":
                print("[note] %s %s" % (os.path.relpath(fixture, REPO).split("/🧫️fixtures/")[-1], after))
    print("x-semio-ui annotations: %d, InputUi errors: %d; leaf schemas Draft-7 errors: %d" % (ui_count, ui_errors, schema_errors))
    print("fixture payloads: %d, verdicts %s, changed by annotation: %d" % (payloads, verdicts, changed))
    return 1 if ui_errors or schema_errors or changed else 0


if __name__ == "__main__":
    sys.exit(main())
