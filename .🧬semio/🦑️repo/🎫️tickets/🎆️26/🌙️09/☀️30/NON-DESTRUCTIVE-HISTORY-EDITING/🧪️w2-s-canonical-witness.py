#!/usr/bin/env python3
"""🧾️ W2-S: brings committed wire witnesses (`…/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json`) to the
canonical Rust number spelling: every integral JSON number the leaf schema types as `number` (a Rust `f64`/`f32`) is written
as a float (`3.0`), exactly as `pack::to_json_string` emits it; `integer` nodes keep integers. The schema walk follows local
and cross-document `$ref`s (every `$id` document under `✏️s` and `🧰️framework`), `allOf`, and the first `oneOf`/`anyOf`
branch whose type admits the value. The Rust owner test asserts the result byte-for-byte (canonical JSON equality).

Usage: python3 🧪️w2-s-canonical-witness.py <leaf-schema> <witness.json>…
       python3 🧪️w2-s-canonical-witness.py --auto <witness.json | directory>…   (leaf schema derived from each witness path)
"""
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
_DOCUMENTS = None


def documents():
    global _DOCUMENTS
    if _DOCUMENTS is None:
        _DOCUMENTS = {}
        for root in ("✏️s", "🧰️framework"):
            for directory, subdirs, names in os.walk(os.path.join(REPO, root)):
                subdirs[:] = [name for name in subdirs if name not in ("node_modules", "target", "🗑️generated", "dist")]
                if "🧬️schema" not in directory:
                    continue
                for name in names:
                    if name.endswith(".json"):
                        try:
                            document = json.load(open(os.path.join(directory, name), encoding="utf-8"))
                        except Exception:
                            continue
                        if isinstance(document, dict) and isinstance(document.get("$id"), str):
                            _DOCUMENTS[document["$id"]] = document
    return _DOCUMENTS


def resolve(document, node, depth=0):
    while isinstance(node, dict) and isinstance(node.get("$ref"), str) and depth < 32:
        reference = node["$ref"]
        base, _, fragment = reference.partition("#")
        if base:
            document = documents().get(base, document if base == document.get("$id") else None)
            if document is None:
                return None, None
        node = document
        for segment in [part for part in fragment.split("/") if part]:
            node = node.get(segment.replace("~1", "/").replace("~0", "~")) if isinstance(node, dict) else None
        depth += 1
    return document, node


def types(node):
    declared = node.get("type")
    return set(declared if isinstance(declared, list) else [declared] if isinstance(declared, str) else [])


def admits(document, node, value):
    document, node = resolve(document, node)
    if not isinstance(node, dict):
        return True
    kinds = types(node)
    if not kinds:
        return True
    actual = "null" if value is None else "boolean" if isinstance(value, bool) else "integer" if isinstance(value, int) else "number" if isinstance(value, float) else "string" if isinstance(value, str) else "array" if isinstance(value, list) else "object"
    return actual in kinds or (actual == "integer" and "number" in kinds)


def canonical(document, node, value):
    document, node = resolve(document, node)
    if not isinstance(node, dict):
        return value
    for member in node.get("allOf", []):
        value = canonical(document, member, value)
    for keyword in ("oneOf", "anyOf"):
        branches = [branch for branch in node.get(keyword, []) if admits(document, branch, value) and not (value is not None and types(resolve(document, branch)[1] or {}) == {"null"})]
        if branches:
            value = canonical(document, branches[0], value)
    kinds = types(node)
    if isinstance(value, int) and not isinstance(value, bool) and "number" in kinds and "integer" not in kinds:
        return float(value)
    if isinstance(value, dict):
        properties = node.get("properties", {}) if isinstance(node.get("properties"), dict) else {}
        extra = node.get("additionalProperties")
        return {key: canonical(document, properties[key], item) if key in properties else canonical(document, extra, item) if isinstance(extra, dict) else item for key, item in value.items()}
    if isinstance(value, list):
        items = node.get("items")
        if isinstance(items, dict):
            return [canonical(document, items, item) for item in value]
        if isinstance(items, list):
            return [canonical(document, items[index], item) if index < len(items) else item for index, item in enumerate(value)]
    return value


def leaf_schema_of(witness):
    head, _, rest = witness.partition("/🧫️fixtures/🧬️mutations/")
    leaf = rest.split("/🧾️wire-witness/")[0]
    return os.path.join(head, "🧬️schema", "🧬️mutations", leaf, "🧬️schema", "🔣️.json")


def main():
    arguments = sys.argv[1:]
    auto = "--auto" in arguments
    arguments = [argument for argument in arguments if argument != "--auto"]
    if auto:
        paths = []
        for argument in arguments:
            if os.path.isdir(os.path.join(REPO, argument)):
                for directory, _, names in os.walk(os.path.join(REPO, argument)):
                    if "🧾️wire-witness" in directory and directory.endswith("🦠️mutation") and "🔣️.json" in names:
                        paths.append(os.path.relpath(os.path.join(directory, "🔣️.json"), REPO))
            else:
                paths.append(argument)
        pairs = [(leaf_schema_of(path), path) for path in sorted(paths)]
    else:
        pairs = [(arguments[0], path) for path in arguments[1:]]
    changed = 0
    for schema_path, path in pairs:
        schema = json.load(open(os.path.join(REPO, schema_path), encoding="utf-8"))
        text = open(os.path.join(REPO, path), encoding="utf-8").read()
        value = json.loads(text)
        updated = canonical(schema, schema, value)
        rendered = json.dumps(updated, indent=2, ensure_ascii=False) + "\n"
        if rendered != text:
            changed += 1
            open(os.path.join(REPO, path), "w", encoding="utf-8").write(rendered)
    print("[w2-s] canonical witnesses: %d of %d rewritten" % (changed, len(pairs)))


if __name__ == "__main__":
    main()
