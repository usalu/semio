#!/usr/bin/env python3
"""🧪️ W2-R stdio-a: validates every committed mutation fixture payload of the group's artifacts against its annotated leaf
payload schema AND against the pre-annotation schema at git HEAD with Python `jsonschema` (Draft 7 + `referencing`), so the
verdicts prove the `x-semio-ui` rollout changed no validation outcome; then validates every `x-semio-ui` in the group's
schema files against the manifest meta-schema `$defs/InputUi`.

    .venv/bin/python 🧪️w2-r-stdio-a-check-fixtures.py"""
import importlib.util
import json
import os
import subprocess
import sys

from jsonschema import Draft7Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("walk", os.path.join(HERE, "🧪️w2-r-stdio-a-walk.py"))
walk = importlib.util.module_from_spec(spec)
spec.loader.exec_module(walk)
REPO = walk.REPO
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"


def load(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return json.load(handle)


CHANGED = None


def head(path):
    global CHANGED
    if CHANGED is None:
        listed = subprocess.run(["git", "-C", REPO, "-c", "core.quotePath=false", "diff", "--name-only", "HEAD", "--", walk.BASE], capture_output=True, text=True).stdout
        CHANGED = set(listed.splitlines())
    if path not in CHANGED:
        return load(path)
    result = subprocess.run(["git", "-C", REPO, "show", "HEAD:" + path], capture_output=True, text=True)
    return json.loads(result.stdout) if result.returncode == 0 else None


def registry(index, reader):
    documents = [(ident, reader(path)) for ident, path in index.items()]
    return Registry().with_resources((ident, Resource(contents=document, specification=DRAFT7)) for ident, document in documents if document is not None)


def fixture_cases():
    """🧫️ (fixture file, leaf schema path, payload) for every committed mutation payload fixture under the group."""
    for artifact in walk.ARTIFACTS:
        for root, _, files in os.walk(os.path.join(REPO, walk.BASE + artifact)):
            relative = os.path.relpath(root, REPO)
            if "/🧫️fixtures/🧬️mutations/" not in relative + "/" or "🔣️.json" not in files:
                continue
            subset, tail = relative.split("/🧫️fixtures/🧬️mutations/", 1)
            parts = tail.split("/")
            if parts[-1] == "🦠️mutation":
                leaf = "/".join(parts[:-2])
                shape = "mutation"
            elif parts[-1] == "🧬️operation":
                leaf = "/".join(parts[:-2])
                shape = "operation"
            else:
                continue
            schema = "%s/🧬️schema/🧬️mutations/%s/🧬️schema/🔣️.json" % (subset, leaf)
            yield relative + "/🔣️.json", schema, shape


def payload_for(schema, fixture, shape):
    if shape == "mutation" and isinstance(fixture.get("payload"), dict) and set(fixture) == {"mutation", "payload"}:
        return fixture["payload"]
    if shape == "operation" and "oneOf" in schema:
        return {"phase": "apply", "value": fixture}
    return fixture


def main():
    index = walk.document_index()
    after = registry(index, lambda path: load(path))
    before = registry(index, head)
    rows = []
    for fixture_path, schema_path, shape in sorted(fixture_cases()):
        if not os.path.exists(os.path.join(REPO, schema_path)):
            rows.append((fixture_path, "no-leaf-schema", None, None))
            continue
        fixture = load(fixture_path)
        verdicts = []
        for schema, reg in ((load(schema_path), after), (head(schema_path), before)):
            payload = payload_for(schema, fixture, shape)
            errors = sorted(Draft7Validator(schema, registry=reg).iter_errors(payload), key=lambda error: list(error.path))
            verdicts.append([("/".join(map(str, error.path)) + ": " + error.message)[:160] for error in errors])
        rows.append((fixture_path, "ok" if not verdicts[0] else "invalid", verdicts[0], verdicts[0] == verdicts[1]))
    valid = sum(1 for row in rows if row[1] == "ok")
    unchanged = sum(1 for row in rows if row[3])
    for fixture_path, status, errors, same in rows:
        if status != "ok":
            print(status.upper(), "same-as-HEAD" if same else "CHANGED", fixture_path.split("🗿️artifacts/")[1])
            for error in (errors or [])[:3]:
                print("    ", error)
    print("fixtures=%d valid=%d invalid=%d no-leaf-schema=%d verdict-unchanged-vs-HEAD=%d" % (len(rows), valid, sum(1 for row in rows if row[1] == "invalid"), sum(1 for row in rows if row[1] == "no-leaf-schema"), unchanged))
    manifest = load(MANIFEST)
    meta = Draft7Validator({"$ref": manifest["$id"] + "#/$defs/InputUi"}, registry=after)
    annotations = 0
    faults = 0
    files = set(walk.leaf_paths()) | {path for path in index.values() if path.startswith(walk.BASE) and any(path.startswith(walk.BASE + artifact + "/") for artifact in walk.ARTIFACTS)}

    def visit(node, where):
        nonlocal annotations, faults
        if isinstance(node, dict):
            if "x-semio-ui" in node:
                annotations += 1
                for error in meta.iter_errors(node["x-semio-ui"]):
                    faults += 1
                    print("INPUTUI", where, error.message[:160])
            for key, value in node.items():
                visit(value, where)
        elif isinstance(node, list):
            for value in node:
                visit(value, where)

    for path in sorted(files):
        visit(load(path), path.split("🗿️artifacts/")[1])
    print("x-semio-ui annotations=%d InputUi-violations=%d" % (annotations, faults))
    return 1 if faults else 0


if __name__ == "__main__":
    sys.exit(main())
