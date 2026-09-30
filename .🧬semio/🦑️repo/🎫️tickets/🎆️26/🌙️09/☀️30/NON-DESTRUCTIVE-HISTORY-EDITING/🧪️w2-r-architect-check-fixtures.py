#!/usr/bin/env python3
"""🧾️ W2-R architect: validates every committed `🦠️mutation` fixture payload under the architect and remodel plugins against
its leaf payload schema (`<leaf>/🧬️schema/🔣️.json`) with Python `jsonschema` (Draft 7 + `referencing`), and every
`x-semio-ui` annotation in those leaf schemas against the manifest `$defs/InputUi` meta-schema.

    .venv/bin/python <ticket>/🧪️w2-r-architect-check-fixtures.py [--list]
"""
import json
import os
import sys

from jsonschema import Draft7Validator
from referencing import Registry, Resource

REPO = "/Users/ueli/Documents/semio"
ROOTS = ("✏️s/🔌️plugins/🏛️architect", "✏️s/🔌️plugins/📸️remodel")
FIXTURES = "🧫️fixtures/🧬️mutations/"
LEAVES = "🧬️schema/🧬️mutations/"
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"


def documents():
    for root in ROOTS:
        for directory, _, files in os.walk(os.path.join(REPO, root)):
            for name in files:
                if name.endswith(".json"):
                    path = os.path.join(directory, name)
                    try:
                        yield path, json.load(open(path, encoding="utf-8"))
                    except (ValueError, UnicodeDecodeError):
                        continue


def annotations(node, pointer=""):
    if isinstance(node, dict):
        if "x-semio-ui" in node:
            yield pointer, node["x-semio-ui"]
        for key, value in node.items():
            yield from annotations(value, pointer + "/" + key)
    elif isinstance(node, list):
        for index, value in enumerate(node):
            yield from annotations(value, pointer + "/" + str(index))


def main():
    loaded = dict(documents())
    registry = Registry().with_resources((doc["$id"], Resource.from_contents(doc)) for doc in loaded.values() if isinstance(doc, dict) and isinstance(doc.get("$id"), str))
    manifest = json.load(open(os.path.join(REPO, MANIFEST), encoding="utf-8"))
    input_ui = Draft7Validator({**manifest["$defs"]["InputUi"], "$defs": manifest["$defs"]})
    leaves = {path for path in loaded if LEAVES in path and path.endswith("/🧬️schema/🔣️.json") and path.count("/🧬️schema/") >= 2}
    ui_errors = ui_count = 0
    for path in sorted(leaves):
        for pointer, value in annotations(loaded[path]):
            ui_count += 1
            for error in input_ui.iter_errors(value):
                ui_errors += 1
                print("UI  %s%s: %s" % (os.path.relpath(path, REPO), pointer, error.message))
    checked = failed = 0
    for path in sorted(loaded):
        if FIXTURES not in path or not path.endswith("/🦠️mutation/🔣️.json"):
            continue
        head, rest = path.split(FIXTURES, 1)
        leaf_rel = os.path.dirname(os.path.dirname(os.path.dirname(rest)))
        leaf = os.path.join(head + LEAVES + leaf_rel, "🧬️schema", "🔣️.json")
        if leaf not in loaded:
            print("MISS %s -> %s" % (os.path.relpath(path, REPO), os.path.relpath(leaf, REPO)))
            failed += 1
            continue
        schema = loaded[leaf]
        payload = dict(loaded[path])
        if "mutation" not in schema.get("properties", {}):
            payload.pop("mutation", None)
        checked += 1
        errors = sorted(Draft7Validator(schema, registry=registry).iter_errors(payload), key=lambda error: list(error.absolute_path))
        if errors:
            failed += 1
            print("FAIL %s: %s at %s" % (os.path.relpath(path, REPO), errors[0].message[:160], "/".join(map(str, errors[0].absolute_path))))
        elif "--list" in sys.argv:
            print("ok   %s" % os.path.relpath(path, REPO))
    print("leaf schemas=%d annotations=%d annotation-errors=%d fixtures=%d fixture-failures=%d" % (len(leaves), ui_count, ui_errors, checked, failed))
    return 1 if failed or ui_errors else 0


if __name__ == "__main__":
    sys.exit(main())
