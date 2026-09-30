#!/usr/bin/env python3
"""🧾️ W2-R energy: third-party (Python `jsonschema`) checks for the energy leaf payload schemas.

1. Every committed fixture mutation payload under `✏️s/🔌️plugins/🔋️energy` meets its leaf schema (the `payload` envelope of
   config leaves unwrapped): a positive fixture validates; a negative one (outcome `mutation.invariant`) is rejected, unless
   its outcome names an `invariant` the leaf declares in `x-semio-invariant` (then it validates).
2. The config `🔁️mutations.json` corpus keeps its declared `valid` verdicts.
3. Every `x-semio-ui` annotation in scope validates against the manifest meta-schema `$defs/InputUi`.

Prints one line per failure and a summary; exits 1 when any check fails. `--baseline` only reports."""
import glob
import json
import os
import sys

from jsonschema import Draft7Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

REPO = "/Users/ueli/Documents/semio"
SCOPE = "✏️s/🔌️plugins/🔋️energy"
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"


def load(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return json.load(handle)


def leaf_index():
    leaves = {}
    for path in glob.glob(f"{SCOPE}/**/🧬️mutations/*/🧬️schema/🔣️.json", root_dir=REPO, recursive=True):
        owner, rest = path.split("/🧬️schema/🧬️mutations/", 1)
        leaves[(owner, rest.split("/", 1)[0])] = path
    return leaves


def kind_of(name):
    return name[next((at for at, character in enumerate(name) if character.isascii() and character.isalpha()), 0):]


def fixture_leaf(path, leaves):
    owner, rest = path.split("/🧫️fixtures/", 1)
    segments = [segment for segment in rest.split("/") if segment != "🧬️mutations"]
    exact = leaves.get((owner, segments[0]))
    if exact is not None:
        return exact
    truncated = [path for (root, name), path in leaves.items() if root == owner and name.startswith(segments[0])]
    return truncated[0] if len(truncated) == 1 else None


def instance(payload, schema):
    properties = schema.get("properties", {})
    if isinstance(payload, dict) and "payload" in payload and "payload" not in properties:
        return payload["payload"]
    return {key: value for key, value in payload.items() if key not in ("mutation", "kind") or key in properties}


def annotations(node, pointer=""):
    if isinstance(node, dict):
        if "x-semio-ui" in node:
            yield pointer, node["x-semio-ui"]
        for key, value in node.items():
            if key != "x-semio-ui":
                yield from annotations(value, f"{pointer}/{key}")
    elif isinstance(node, list):
        for at, value in enumerate(node):
            yield from annotations(value, f"{pointer}/{at}")


def main():
    baseline = "--baseline" in sys.argv
    leaves = leaf_index()
    failures = []
    checked = 0
    negatives = 0
    for path in sorted(glob.glob(f"{SCOPE}/**/🦠️mutation/🔣️.json", root_dir=REPO, recursive=True)):
        leaf = fixture_leaf(path, leaves)
        if leaf is None:
            failures.append(f"UNMAPPED {path}")
            continue
        schema = load(leaf)
        checked += 1
        outcome_path = path.replace("/🦠️mutation/🔣️.json", "/🎯️outcome/🔣️.json")
        outcome = load(outcome_path) if os.path.exists(os.path.join(REPO, outcome_path)) else {}
        errors = list(Draft7Validator(schema).iter_errors(instance(load(path), schema)))
        negative = outcome.get("status") == "rejected" and outcome.get("code") == "mutation.invariant"
        if negative and "invariant" in outcome:
            negatives += 1
            declared = {row["id"] for row in schema.get("x-semio-invariant", [])}
            if outcome["invariant"] not in declared:
                failures.append(f"INVARIANT {path}: {outcome['invariant']} is not declared in the leaf x-semio-invariant")
            for error in errors:
                failures.append(f"FIXTURE {path}: {error.message}")
        elif negative:
            negatives += 1
            if not errors:
                failures.append(f"NEGATIVE {path}: the leaf schema accepts a payload the domain refuses as mutation.invariant")
        else:
            for error in errors:
                failures.append(f"FIXTURE {path}: {error.message}")
    corpus_checked = 0
    for path in sorted(glob.glob(f"{SCOPE}/**/🎚️config/🧫️fixtures/🔁️mutations.json", root_dir=REPO, recursive=True)):
        owner = path.split("/🧫️fixtures/", 1)[0]
        for at, case in enumerate(load(path)):
            leaf = next((candidate for (root, name), candidate in leaves.items() if root == owner and kind_of(name) == case["kind"]), None)
            if leaf is None:
                failures.append(f"UNMAPPED {path}#{at} {case['kind']}")
                continue
            schema = load(leaf)
            corpus_checked += 1
            valid = Draft7Validator(schema).is_valid(instance(case["mutation"], schema))
            if valid != case["valid"]:
                failures.append(f"CORPUS {path}#{at} {case['kind']}: schema verdict {valid}, declared {case['valid']}")
    manifest = load(MANIFEST)
    registry = Registry().with_resource(manifest["$id"], Resource.from_contents(manifest, default_specification=DRAFT7))
    meta = Draft7Validator({"$ref": f"{manifest['$id']}#/$defs/InputUi"}, registry=registry)
    annotated = 0
    for leaf in sorted(leaves.values()):
        for pointer, annotation in annotations(load(leaf)):
            annotated += 1
            for error in meta.iter_errors(annotation):
                failures.append(f"INPUTUI {leaf}{pointer}: {error.message}")
    for failure in failures:
        print(failure)
    print(f"fixtures={checked} negatives={negatives} corpus={corpus_checked} annotations={annotated} failures={len(failures)}")
    sys.exit(0 if baseline or not failures else 1)


if __name__ == "__main__":
    main()
