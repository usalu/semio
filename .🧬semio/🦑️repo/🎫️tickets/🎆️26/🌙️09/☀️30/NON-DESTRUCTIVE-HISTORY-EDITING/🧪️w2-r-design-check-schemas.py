#!/usr/bin/env python3
"""🧾️ W2-R design group: third-party (Python ``jsonschema``, Draft 7) checks over every leaf payload schema of the
design rollout group — puzzle 3d/5d, shooting, cad, procedural, note, forms, raster, draw.

1. Every ``x-semio-ui`` annotation in a leaf schema, or in a record document a leaf reaches through ``$ref``, validates
   against the manifest meta-schema ``$defs/InputUi``.
2. Every committed fixture payload (``🦠️mutation/🔣️.json``) validates against its leaf payload schema; an externally
   tagged payload (``{"PascalKind": {...}}``) is unwrapped and the aggregate tag ``mutation`` is dropped unless the
   leaf declares it. Payloads that already disagree with their leaf (snake_case keys, null indices) are pre-existing
   drift: compare a before/after pair, an annotation pass must not add a failure.

Prints one line per failure and a summary; ``--json <path>`` writes the failure list so a before/after pair can be
compared. Run with the repository venv: ``.venv/bin/python <this file> [--json out.json]``.

@see ./🧪️w2-r-design-annotate-inputs.py
@see ../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json
"""
import json
import os
import sys

import jsonschema
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

REPO = "/Users/ueli/Documents/semio"
PLUGINS = "✏️s/🔌️plugins"
ROOTS = [f"{PLUGINS}/🧩️puzzle/🗿️artifacts/🧊️3d", f"{PLUGINS}/🧩️puzzle/🗿️artifacts/🖐️5d", f"{PLUGINS}/🎥️shooting", f"{PLUGINS}/📐️cad", f"{PLUGINS}/🌀️procedural", f"{PLUGINS}/🗒️note", f"{PLUGINS}/📋️forms", f"{PLUGINS}/🖨️raster", f"{PLUGINS}/🖍️draw"]
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"
MUTATIONS = "🧬️mutations"


def load(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return json.load(handle)


def kind_of(name):
    return next((name[at:] for at, character in enumerate(name) if character.isascii() and character.isalpha()), name)


def documents():
    found = {}
    for root in ROOTS:
        for directory, _, files in os.walk(os.path.join(REPO, root)):
            for name in files:
                if not name.endswith(".json"):
                    continue
                path = os.path.relpath(os.path.join(directory, name), REPO)
                try:
                    document = load(path)
                except (ValueError, OSError):
                    continue
                if isinstance(document, dict) and isinstance(document.get("$schema"), str):
                    found[path] = document
    return found


def leaves(found):
    return sorted(path for path in found if path.endswith("/🧬️schema/🔣️.json") and os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(path)))) == MUTATIONS)


def fixtures(leaf):
    leaf_dir = os.path.dirname(os.path.dirname(leaf))
    kind = kind_of(os.path.basename(leaf_dir))
    base = os.path.dirname(os.path.dirname(os.path.dirname(leaf_dir)))
    payloads = []
    for parent in (os.path.join(base, "🧫️fixtures", MUTATIONS), os.path.join(base, "🧫️fixtures")):
        absolute = os.path.join(REPO, parent)
        if not os.path.isdir(absolute):
            continue
        for entry in sorted(os.listdir(absolute)):
            if kind_of(entry) != kind or not os.path.isdir(os.path.join(absolute, entry)):
                continue
            for case in sorted(os.listdir(os.path.join(absolute, entry))):
                payload = os.path.join(parent, entry, case, "🦠️mutation", "🔣️.json")
                if os.path.isfile(os.path.join(REPO, payload)):
                    payloads.append(payload)
    return payloads


def annotations(node, pointer=""):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "x-semio-ui":
                yield pointer, value
            else:
                yield from annotations(value, f"{pointer}/{key}")
    elif isinstance(node, list):
        for index, value in enumerate(node):
            yield from annotations(value, f"{pointer}/{index}")


def main():
    found = documents()
    manifest = load(MANIFEST)
    registry = Registry().with_resources([(manifest["$id"], Resource.from_contents(manifest, default_specification=DRAFT7))] + [(document["$id"], Resource.from_contents(document, default_specification=DRAFT7)) for document in found.values() if isinstance(document.get("$id"), str)])
    meta = jsonschema.Draft7Validator({"$ref": "%s#/$defs/InputUi" % manifest["$id"]}, registry=registry)
    failures = []
    ui_count = 0
    for path, document in sorted(found.items()):
        for pointer, value in annotations(document):
            ui_count += 1
            for error in meta.iter_errors(value):
                failures.append({"check": "input-ui", "file": path, "pointer": pointer, "detail": error.message})
    leaf_paths = leaves(found)
    payload_count = 0
    for leaf in leaf_paths:
        schema = found[leaf]
        validator = jsonschema.Draft7Validator(schema, registry=registry)
        declared = schema.get("properties", {}) if isinstance(schema.get("properties"), dict) else {}
        for payload_path in fixtures(leaf):
            payload_count += 1
            payload = load(payload_path)
            if isinstance(payload, dict) and len(payload) == 1 and next(iter(payload))[:1].isupper() and next(iter(payload)) not in declared and isinstance(next(iter(payload.values())), dict):
                payload = next(iter(payload.values()))
            if isinstance(payload, dict) and "mutation" in payload and "mutation" not in declared:
                payload = {key: value for key, value in payload.items() if key != "mutation"}
            try:
                errors = [error.message for error in validator.iter_errors(payload)]
            except Exception as error:
                errors = ["unresolvable: %s" % error]
            for message in errors:
                failures.append({"check": "fixture", "file": payload_path, "pointer": "", "detail": message[:240]})
    for failure in failures:
        print("[%s] %s %s — %s" % (failure["check"], failure["file"], failure["pointer"], failure["detail"]))
    print("checked %d x-semio-ui annotation(s) in %d document(s), %d fixture payload(s) of %d leaves: %d failure(s)" % (ui_count, len(found), payload_count, len(leaf_paths), len(failures)))
    if "--json" in sys.argv:
        with open(sys.argv[sys.argv.index("--json") + 1], "w", encoding="utf-8") as handle:
            json.dump(failures, handle, indent=2, ensure_ascii=False)
    return 1 if any(failure["check"] == "input-ui" for failure in failures) else 0


if __name__ == "__main__":
    sys.exit(main())
