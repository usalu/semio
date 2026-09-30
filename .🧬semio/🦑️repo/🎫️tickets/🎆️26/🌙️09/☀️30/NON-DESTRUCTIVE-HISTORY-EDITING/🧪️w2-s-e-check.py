#!/usr/bin/env python3
"""🧾️ W2-S-E third-party oracle (Python `jsonschema` Draft 7 + `referencing`) over the layout + tail scope.

1. Every leaf payload schema is a valid Draft 7 schema.
2. Every `x-semio-ui` node in those schemas validates against the manifest `$defs/InputUi` meta-schema.
3. Every committed `🦠️mutation/🔣️.json` fixture validates against its leaf payload schema, cut out of the aggregate wire the
   way `payload_value()` does (an externally tagged aggregate's one-key wrapper is removed; an internally tagged wire is
   validated whole, because every leaf declares the tag `const`; an adjacently tagged wire yields its content member), and
   the whole wire validates against the aggregate `🧬️mutations/🔣️.json` document next to the leaf directory. A negative
   witness (outcome `mutation.invariant`) must instead be rejected by its leaf schema or name a declared `invariant`.

Run: .venv/bin/python 🧪️w2-s-e-check.py
"""
import json
import os

from jsonschema import Draft7Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

REPO = "/Users/ueli/Documents/semio"
ROOTS = ["✏️s/🔌️plugins/📏️layout", "✏️s/🔌️plugins/➗️mathematical", "✏️s/🔌️plugins/📜️imperative", "✏️s/🔌️plugins/🏭️process", "✏️s/🔌️plugins/🎬️sequence", "✏️s/🔌️plugins/🌊️flow", "✏️s/🔌️plugins/🕸️dag", "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store", "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite", "🧰️framework/🛍️products/💻️os/🎚️config"]
INDEXED = ["🧰️framework/🔨️modules", *ROOTS]
SKIPPED = {"node_modules", "target", "dist", "🗑️generated"}


def walk(root):
    for directory, names, files in os.walk(os.path.join(REPO, root)):
        names[:] = [name for name in names if name not in SKIPPED]
        yield directory, files


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def registry():
    resources = []
    for root in INDEXED:
        for directory, files in walk(root):
            if "🧫️fixtures" in directory.split("/") and "/🧬️mutations/" not in directory:
                continue
            for name in files:
                if name.endswith(".json"):
                    try:
                        document = load(os.path.join(directory, name))
                    except ValueError:
                        continue
                    if isinstance(document, dict) and isinstance(document.get("$id"), str):
                        resources.append((document["$id"], Resource.from_contents(document, default_specification=DRAFT7)))
    return Registry().with_resources(resources)


def ui_nodes(node, pointer=""):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "x-semio-ui":
                yield pointer, value
            else:
                yield from ui_nodes(value, f"{pointer}/{key}")
    elif isinstance(node, list):
        for index, value in enumerate(node):
            yield from ui_nodes(value, f"{pointer}/{index}")


def main():
    store = registry()
    manifest = load(os.path.join(REPO, "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"))
    input_ui = Draft7Validator({"$ref": f"{manifest['$id']}#/$defs/InputUi"}, registry=store)
    leaves, uis, bad_ui, bad_schema, fixtures, bad_fixture, negatives = 0, 0, [], [], 0, [], 0
    for root in ROOTS:
        for directory, files in walk(root):
            if "🔣️.json" not in files or "/🧬️mutations/" not in directory or any(part.endswith("fixtures") for part in directory.split("/")):
                continue
            descriptor = load(os.path.join(directory, "🔣️.json"))
            if not isinstance(descriptor, dict) or not isinstance(descriptor.get("payloadSchema"), str):
                continue
            leaves += 1
            schema_path = os.path.join(directory, descriptor["payloadSchema"])
            schema = load(schema_path)
            try:
                Draft7Validator.check_schema(schema)
            except Exception as error:
                bad_schema.append((schema_path, str(error)[:200]))
            for pointer, node in ui_nodes(schema):
                uis += 1
                errors = [error.message for error in input_ui.iter_errors(node)]
                if errors:
                    bad_ui.append((schema_path, pointer, errors[0][:160]))
            case_root = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(directory))), "🧫️fixtures", "🧬️mutations", os.path.basename(directory))
            candidates = [case_root, os.path.join(directory, "🧫️fixtures")]
            aggregate_path = os.path.join(os.path.dirname(directory), "🔣️.json")
            aggregate = load(aggregate_path) if os.path.exists(aggregate_path) else None
            for candidate in candidates:
                for case, case_files in walk(os.path.relpath(candidate, REPO)) if os.path.isdir(candidate) else []:
                    if not case.endswith("🦠️mutation") or "🔣️.json" not in case_files:
                        continue
                    fixtures += 1
                    wire = load(os.path.join(case, "🔣️.json"))
                    outcome_path = os.path.join(os.path.dirname(case), "🎯️outcome", "🔣️.json")
                    outcome = load(outcome_path) if os.path.exists(outcome_path) else {}
                    negative = outcome.get("status") == "rejected" and outcome.get("code") == "mutation.invariant"
                    variant = descriptor.get("aggregateVariant")
                    content = next((key for branch in (aggregate or {}).get("oneOf", []) for key, node in branch.get("properties", {}).items() if isinstance(node, dict) and "$ref" in node and len(branch.get("required", [])) == 2), None)
                    payload = wire[variant] if isinstance(wire, dict) and list(wire) == [variant] else wire[content] if content is not None and isinstance(wire, dict) and content in wire else wire
                    leaf_errors = [error.message for error in Draft7Validator(schema, registry=store).iter_errors(payload)]
                    if negative:
                        negatives += 1
                        if not leaf_errors and not outcome.get("invariant"):
                            bad_fixture.append((os.path.relpath(case, REPO), "leaf (negative accepted)", "the schema accepts a payload the domain refuses as mutation.invariant"))
                        continue
                    for label, target, instance in (("leaf", schema, payload), ("aggregate", aggregate, wire)):
                        if target is None:
                            continue
                        errors = leaf_errors if label == "leaf" else [error.message for error in Draft7Validator(target, registry=store).iter_errors(instance)]
                        if errors:
                            bad_fixture.append((os.path.relpath(case, REPO), label, errors[0][:200]))
    for path, error in bad_schema:
        print(f"[w2-s-e] schema invalid {path}: {error}")
    for path, pointer, error in bad_ui:
        print(f"[w2-s-e] x-semio-ui invalid {path}#{pointer}: {error}")
    for path, label, error in bad_fixture:
        print(f"[w2-s-e] fixture rejected by {label} {path}: {error}")
    print(f"[w2-s-e] {leaves} leaves, {uis} x-semio-ui nodes ({len(bad_ui)} invalid), {len(bad_schema)} invalid schemas, {fixtures} fixture wires ({negatives} negative, {len(bad_fixture)} rejections)")
    raise SystemExit(1 if bad_ui or bad_schema or bad_fixture else 0)


if __name__ == "__main__":
    main()
