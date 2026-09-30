#!/usr/bin/env python3
"""✅️ W2-R stdio-b verification with the third-party Python `jsonschema` (Draft 7 + `referencing`):
1. every `x-semio-ui` in the edited files validates against the manifest meta-schema `$defs/InputUi`;
2. every committed fixture payload of an in-scope leaf (fixture `🦠️mutation` files and every fixture object tagged with the
   leaf's `mutation` const) gets the same verdict from the annotated schemas as from the pre-edit backup, and stays valid;
3. every edited file kept its layout: `json.dumps(indent=2)` files still round-trip, Prettier-clean files are still clean.

    .venv/bin/python 🧪️w2-r-stdio-b-check.py <backup.json>
"""
import importlib.util
import json
import os
import subprocess
import sys

from jsonschema import Draft7Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"
spec = importlib.util.spec_from_file_location("walk", os.path.join(HERE, "🧪️w2-r-stdio-b-walk.py"))
walk = importlib.util.module_from_spec(spec)
spec.loader.exec_module(walk)


def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def annotations(node, found):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "x-semio-ui":
                found.append(value)
            annotations(value, found)
    elif isinstance(node, list):
        for value in node:
            annotations(value, found)
    return found


def tagged(node, found):
    if isinstance(node, dict):
        if isinstance(node.get("mutation"), str):
            found.append(node)
        for value in node.values():
            tagged(value, found)
    elif isinstance(node, list):
        for value in node:
            tagged(value, found)
    return found


def registry(documents, override):
    resources = []
    for identifier, (path, document) in documents.items():
        resources.append((identifier, Resource(contents=override.get(path, document), specification=DRAFT7)))
    return Registry().with_resources(resources)


def main():
    bundle = json.load(open(sys.argv[1], encoding="utf-8"))
    edited = sorted(bundle)
    manifest = read(os.path.join(REPO, MANIFEST))
    meta = Registry().with_resource(manifest["$id"], Resource(contents=manifest, specification=DRAFT7))
    input_ui = Draft7Validator({"$ref": manifest["$id"] + "#/$defs/InputUi"}, registry=meta)
    count, bad = 0, []
    for path in edited:
        for annotation in annotations(read(os.path.join(REPO, path)), []):
            count += 1
            errors = [error.message for error in input_ui.iter_errors(annotation)]
            if errors:
                bad.append((path, errors[:2]))
    print("[1] x-semio-ui annotations: %d, invalid: %d" % (count, len(bad)))
    for entry in bad[:10]:
        print("    ", entry)

    catalog = walk.load(walk.CATALOG)["scopes"]
    documents = walk.index_documents(catalog)
    before_docs = {path: json.loads(bundle[path]) for path in edited}
    after_registry = registry(documents, {})
    before_registry = registry(documents, before_docs)
    payloads, verdicts, drift, invalid, faults = 0, 0, [], [], []
    for scope in catalog.values():
        if scope.get("level") != "mutation-leaf" or not walk.in_scope(scope["path"]):
            continue
        leaf_path = scope["path"] + "/" + scope["formats"].get("🔣️jsonschema", "🔣️.json")
        after_schema = read(os.path.join(REPO, leaf_path))
        before_schema = before_docs.get(leaf_path, after_schema)
        const = ((after_schema.get("properties") or {}).get("mutation") or {}).get("const")
        subset_root = leaf_path.split("/🧬️schema/")[0]
        leaf_dir = leaf_path.split("/")[-3]
        kind = walk.re.sub(r"^[^a-z]*", "", leaf_dir)
        names = {kind} | ({const} if const is not None else set())
        pascal = "".join(word[:1].upper() + word[1:] for word in kind.split("-"))
        fixtures_root = os.path.join(REPO, subset_root, "🧫️fixtures")
        candidates = []
        for directory, _, files in os.walk(fixtures_root):
            for name in files:
                if not name.endswith(".json"):
                    continue
                full = os.path.join(directory, name)
                relative = os.path.relpath(full, fixtures_root).split(os.sep)
                try:
                    document = read(full)
                except ValueError:
                    continue
                if relative[:2] == ["🧬️mutations", leaf_dir] and "🦠️mutation" in relative:
                    candidates.append((full, document[pascal] if isinstance(document, dict) and list(document) == [pascal] and isinstance(document[pascal], dict) else document))
                else:
                    candidates.extend((full, node) for node in tagged(document, []) if node.get("mutation") == const or (node.get("mutation") in names and set(node) == {"mutation", "payload"}))
        seen = set()
        for full, payload in candidates:
            key = (full, json.dumps(payload, sort_keys=True))
            if key in seen or not isinstance(payload, dict):
                continue
            seen.add(key)
            if set(payload) == {"mutation", "payload"} and isinstance(payload["payload"], dict):
                payload = dict(payload["payload"])
            if const is not None and "mutation" not in payload:
                payload = {**payload, "mutation": const}
            if "mutation" not in (after_schema.get("properties") or {}):
                payload = {key: value for key, value in payload.items() if key != "mutation"}
            payloads += 1
            try:
                before_ok = Draft7Validator(before_schema, registry=before_registry).is_valid(payload)
                after_ok = Draft7Validator(after_schema, registry=after_registry).is_valid(payload)
            except Exception as error:
                faults.append((leaf_path.split("/")[4] + " " + kind, os.path.relpath(full, REPO).split("🧫️fixtures/")[-1][:90], repr(error)[:120]))
                continue
            verdicts += 1
            if before_ok != after_ok:
                drift.append((leaf_path, full))
            if not after_ok:
                reason = next(Draft7Validator(after_schema, registry=after_registry).iter_errors(payload)).message[:160]
                invalid.append((leaf_path.split("/")[4] + " " + kind, os.path.relpath(full, REPO).split("🧫️fixtures/")[-1][:90], reason))
    print("[2] fixture payloads: %d, verdict drift: %d, invalid after: %d, validator faults: %d" % (payloads, len(drift), len(invalid), len(faults)))
    for entry in faults:
        print("     fault", entry)
    for entry in drift + invalid:
        print("    ", entry)

    prettier = os.path.join(REPO, "node_modules/.bin/prettier")
    layout = []
    for path in edited:
        before_text = bundle[path]
        after_text = open(os.path.join(REPO, path), encoding="utf-8").read()
        dumps = lambda text: json.dumps(json.loads(text), indent=2, ensure_ascii=False) + "\n" == text
        pretty = lambda text: subprocess.run([prettier, "--stdin-filepath", path], cwd=REPO, input=text.encode("utf-8"), capture_output=True).stdout.decode("utf-8") == text
        if dumps(before_text):
            if not dumps(after_text):
                layout.append(("dumps", path))
        elif pretty(before_text):
            if not pretty(after_text):
                layout.append(("prettier", path))
    print("[3] edited files: %d, layout drift: %d" % (len(edited), len(layout)))
    for entry in layout:
        print("    ", entry)
    sys.exit(1 if bad or drift or invalid or layout else 0)


if __name__ == "__main__":
    main()
