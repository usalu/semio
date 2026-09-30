#!/usr/bin/env python3
"""🧾️ W2-R-mid: validates every committed fixture mutation payload (`🦠️mutation/🔣️.json`) of the wfc, block, fem and layout
plugins and of the os-owned leaves against its leaf payload schema twice — the pre-annotation snapshot and the annotated file —
with Python `jsonschema` (Draft 7 + `referencing`, cross-document `$ref`s resolved through every `$id` document of the repo's
framework and plugin trees). The verdicts must be identical: `x-semio-ui` is an annotation and changes no validation outcome.

Run: .venv/bin/python 🧪️w2-r-mid-validate-fixtures.py <before-snapshot-dir>"""
import json
import os
import sys

from jsonschema import Draft7Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

REPO = "/Users/ueli/Documents/semio"
ROOTS = ["✏️s/🔌️plugins/🀄️wfc", "✏️s/🔌️plugins/🧱️block", "✏️s/🔌️plugins/🏗️fem", "✏️s/🔌️plugins/📏️layout", "🧰️framework/🛍️products/💻️os", "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap"]
INDEXED = ["🧰️framework/🔨️modules", "🧰️framework/🛍️products/💻️os", *ROOTS]


def registry():
    resources = []
    for root in INDEXED:
        for directory, names, files in os.walk(os.path.join(REPO, root)):
            names[:] = [name for name in names if name not in ("node_modules", "target", "🧫️fixtures")]
            for name in files:
                if not name.endswith(".json"):
                    continue
                try:
                    document = json.load(open(os.path.join(directory, name), encoding="utf-8"))
                except (ValueError, OSError):
                    continue
                if isinstance(document, dict) and isinstance(document.get("$id"), str):
                    resources.append((document["$id"], Resource.from_contents(document, default_specification=DRAFT7)))
    return Registry().with_resources(resources)


def leaf_schemas():
    found = {}
    for root in ROOTS:
        for directory, names, files in os.walk(os.path.join(REPO, root)):
            if "🧫️fixtures" in directory:
                continue
            if directory.endswith("/🧬️schema") and "🔣️.json" in files and os.path.basename(os.path.dirname(os.path.dirname(directory))) == "🧬️mutations":
                leaf = os.path.basename(os.path.dirname(directory))
                found.setdefault(leaf, []).append(os.path.join(directory, "🔣️.json"))
    return found


def fixtures():
    for root in ROOTS:
        for directory, names, files in os.walk(os.path.join(REPO, root)):
            if directory.endswith("/🦠️mutation") and "🔣️.json" in files and "/🧫️fixtures/🧬️mutations/" in directory:
                yield os.path.join(directory, "🔣️.json")


def schema_for(fixture, leaves):
    head, tail = fixture.split("/🧫️fixtures/🧬️mutations/", 1)
    leaf = tail.split("/")[0]
    direct = os.path.join(head, "🧬️schema", "🧬️mutations", leaf, "🧬️schema", "🔣️.json")
    if os.path.exists(direct):
        return direct
    candidates = [path for path in leaves.get(leaf, []) if path.startswith(head)]
    return min(candidates, key=len) if candidates else None


def payloads(raw, schema):
    properties = schema.get("properties", {}) if isinstance(schema, dict) else {}
    yield "raw", raw
    if isinstance(raw, dict) and "mutation" in raw and "mutation" not in properties:
        yield "untagged", {key: value for key, value in raw.items() if key != "mutation"}
    if isinstance(raw, dict) and len(raw) == 1 and isinstance(next(iter(raw.values())), dict) and next(iter(raw))[:1].isupper():
        yield "unwrapped", next(iter(raw.values()))


def verdict(schema, raw, reg):
    validator = Draft7Validator(schema, registry=reg)
    results = []
    for form, payload in payloads(raw, schema):
        errors = sorted(error.message for error in validator.iter_errors(payload))
        results.append((form, errors))
        if not errors:
            return "valid(%s)" % form, results
    return "invalid", results


def main():
    before_root = sys.argv[1]
    reg = registry()
    leaves = leaf_schemas()
    tally = {"valid": 0, "invalid": 0, "noSchema": 0, "changed": 0}
    for fixture in sorted(fixtures()):
        schema_path = schema_for(fixture, leaves)
        if schema_path is None:
            tally["noSchema"] += 1
            continue
        raw = json.load(open(fixture, encoding="utf-8"))
        after = json.load(open(schema_path, encoding="utf-8"))
        snapshot = os.path.join(before_root, os.path.relpath(schema_path, REPO))
        before = json.load(open(snapshot, encoding="utf-8")) if os.path.exists(snapshot) else after
        verdict_after, detail = verdict(after, raw, reg)
        verdict_before, _ = verdict(before, raw, reg)
        if verdict_after != verdict_before:
            tally["changed"] += 1
            print("CHANGED %s: before %s, after %s" % (os.path.relpath(fixture, REPO), verdict_before, verdict_after))
        tally["valid" if verdict_after.startswith("valid") else "invalid"] += 1
        if not verdict_after.startswith("valid"):
            print("INVALID(before and after) %s: %s" % (os.path.relpath(fixture, REPO), detail[0][1][:2]))
    print(json.dumps(tally))
    return 1 if tally["changed"] else 0


if __name__ == "__main__":
    sys.exit(main())
