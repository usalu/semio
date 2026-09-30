#!/usr/bin/env python3
"""⚖️ W2-S repo-wide aggregate rule (design §6 parity, coordinator-approved 2026-09-30):

- internally tagged aggregate (`#[value(tag = T)]`): every leaf schema root declares `T: {"const": <wire name>}` first in
  `properties` and first in `required`; the aggregate document is the `oneOf` of its leaf `$ref`s in Rust variant order;
- externally tagged aggregate (no tag): every aggregate branch is `{type: object, additionalProperties: false, required: [W],
  properties: {W: {$ref: leaf}}}`;
- adjacently tagged aggregate (`tag = T, content = C`): every branch is `{type: object, additionalProperties: false,
  required: [T, C], properties: {T: {const: W}, C: {$ref: leaf}}}`;
- wrapped leaf (`#[mutation_leaf(payload = <Variant>)]`, the map's `wrapper`): the leaf root describes only the editable
  payload and its whole wire enum sits under `$defs/<wrapper>`, so every branch refers to `leaf#/$defs/<wrapper>` instead of
  the root (the lint's `mutationAggregateBranch` rule; B1's glTF aggregates are written in this form).

Leaf schemas are rewritten only when they are in canonical `json.dumps(indent=2, ensure_ascii=False)` form (so no peer
formatting is lost); every other file is listed for a hand edit. Idempotent. Each file is re-read right before it is written.

Usage: python3 🧪️w2-s-aggregate-rule.py <aggregate-map.json> [--apply] <path-prefix>…
"""
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"


def load(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return handle.read()


def canonical(value, text):
    rendered = json.dumps(value, indent=2, ensure_ascii=False)
    return rendered + ("\n" if text.endswith("\n") else "") == text, rendered + ("\n" if text.endswith("\n") else "")


def write(path, text):
    with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
        handle.write(text)


def members(text, start=None):
    """The members of the JSON object text at `start` (default: the root): key -> (key_start, value_start, value_end)."""
    decoder, found = json.JSONDecoder(), {}
    skip = lambda index: index + len(text[index:]) - len(text[index:].lstrip())
    index = skip((text.index("{") if start is None else start) + 1)
    while text[index] != "}":
        key, colon = decoder.raw_decode(text, index)
        value_start = skip(skip(colon) + 1)
        _, value_end = decoder.raw_decode(text, value_start)
        found[key] = (index, value_start, value_end)
        index = skip(value_end)
        index = skip(index + 1) if text[index] == "," else index
    return found


def insert_first(text, open_index, item):
    """Inserts `item` as the first element of the object/array whose opening bracket is at `open_index`, in its own style."""
    first = open_index + 1
    while text[first].isspace():
        first += 1
    gap = text[open_index + 1:first]
    if text[first] in "}]":
        return text[:open_index + 1] + item + text[open_index + 1:]
    indent = gap[gap.rfind("\n") + 1:] if "\n" in gap else ""
    rendered = item.replace("\n", "\n" + indent) if "\n" in gap else item.replace("\n", " ")
    return text[:first] + rendered + "," + gap + text[first:]


def tag_by_text(text, tag, wire):
    """Adds the tag const property and the required entry to a non-canonical leaf text, keeping every other byte."""
    root = members(text)
    required = root.get("required")
    if required is None or json.loads(text[required[1]:required[2]]).count(tag) == 0:
        if required is None:
            key_start = root["properties"][0]
            before = text[:key_start]
            gap = before[before.rfind(",") + 1:] if before.rfind(",") > before.rfind("{") else before[before.rfind("{") + 1:]
            text = text[:key_start] + '"required": [%s],' % json.dumps(tag) + gap + text[key_start:]
        else:
            text = insert_first(text, required[1], json.dumps(tag))
    root = members(text)
    names = root.get("propertyNames")
    if names is not None:
        enum = members(text, names[1]).get("enum")
        if enum is not None and tag not in json.loads(text[enum[1]:enum[2]]):
            text = insert_first(text, enum[1], json.dumps(tag))
            root = members(text)
    properties = root["properties"]
    if tag not in json.loads(text[properties[1]:properties[2]]):
        multiline = "\n" in text[properties[1]:properties[2]]
        member = '"%s": %s' % (tag, json.dumps({"const": wire}, indent=2 if multiline else None, ensure_ascii=False))
        text = insert_first(text, properties[1], member)
    return text


def inline_payload(leaf):
    """Folds a `$defs/Payload` (+ duplicate `$defs/Wire`) split into the root, so the root is the leaf wire form itself."""
    root_ref = leaf.get("$ref") or next((member.get("$ref") for member in leaf.get("allOf", []) if isinstance(member, dict)), None)
    if root_ref != "#/$defs/Payload" or "properties" in leaf:
        return leaf, None
    payload = leaf["$defs"]["Payload"]
    wire = leaf["$defs"].get("Wire")
    if wire is not None and any(wire["properties"].get(key) not in (value, {"$ref": "#/$defs/Payload/properties/" + key}) for key, value in payload["properties"].items()):
        return None, "$defs/Wire diverges from $defs/Payload"
    rebuilt = {key: value for key, value in leaf.items() if key not in ("$ref", "allOf", "$defs", "type", "additionalProperties", "required", "properties")}
    rebuilt["type"] = "object"
    rebuilt["additionalProperties"] = payload.get("additionalProperties", False)
    rebuilt["required"] = payload.get("required", [])
    rebuilt["properties"] = payload["properties"]
    rest = {key: value for key, value in leaf["$defs"].items() if key not in ("Payload", "Wire")}
    if rest:
        rebuilt["$defs"] = rest
    return rebuilt, None


def with_tag(leaf, tag, wire):
    root = leaf
    if "$ref" in root and "properties" not in root:
        return None, "root is a $ref (Payload/Wire split)"
    if not isinstance(root.get("properties"), dict):
        return None, "root has no properties"
    properties = root["properties"]
    declared = properties.get(tag)
    changed = not (isinstance(declared, dict) and declared.get("const") == wire)
    properties = {tag: declared if not changed else {"const": wire}, **{key: value for key, value in properties.items() if key != tag}}
    required = root.get("required", [])
    changed = changed or tag not in required
    required = [tag] + [name for name in required if name != tag]
    names = root.get("propertyNames")
    if isinstance(names, dict) and isinstance(names.get("enum"), list) and tag not in names["enum"]:
        changed = True
        root = {**root, "propertyNames": {**names, "enum": [tag] + names["enum"]}}
    rebuilt = {}
    for key, value in root.items():
        if key == "properties":
            if "required" not in root:
                rebuilt["required"] = required
            rebuilt["properties"] = properties
        elif key == "required":
            rebuilt["required"] = required
        else:
            rebuilt[key] = value
    return (rebuilt if changed else leaf), None


def aggregate_document(schema_path, leaf_ids, members):
    """A new aggregate document: `$id` beside its leaves (`<base>/mutation/<kind>/schema.json` → `<base>/mutations.json`, else
    beside the lane module), and the rule-derived `oneOf`; a file shared by several enums carries one `$defs` entry per enum."""
    bases = {leaf_id.rsplit("/mutation/", 1)[0] for leaf_id in leaf_ids if "/mutation/" in leaf_id}
    if len(bases) == 1:
        identifier = bases.pop() + "/mutations.json"
    else:
        module_path = os.path.join(os.path.dirname(os.path.dirname(schema_path)), "🔣️.json")
        if not os.path.exists(os.path.join(REPO, module_path)):
            return None
        identifier = json.loads(load(module_path))["$id"].rsplit("/", 1)[0] + "/mutations.json"
    document = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": identifier}
    if len(members) == 1:
        name, branches = next(iter(members.items()))
        document.update({"title": name, "oneOf": branches})
    else:
        document.update({"title": " | ".join(members), "oneOf": [{"$ref": "#/$defs/%s" % name} for name in members], "$defs": {name: {"title": name, "oneOf": branches} for name, branches in members.items()}})
    return document


def main():
    map_path, *rest = sys.argv[1:]
    apply = "--apply" in rest
    prefixes = [arg for arg in rest if arg != "--apply"]
    aggregates = json.load(open(map_path, encoding="utf-8"))
    manual, written, unchanged, created = [], [], 0, {}
    for aggregate in aggregates:
        if not any(aggregate["path"].startswith(prefix) for prefix in prefixes):
            continue
        tag, content = aggregate["tag"], aggregate["content"]
        branches, leaf_ids = [], []
        for variant in aggregate["variants"]:
            if len(variant["leaves"]) != 1:
                manual.append((aggregate["path"], "variant %s has %d leaves" % (variant["variant"], len(variant["leaves"]))))
                continue
            leaf_path = variant["leaves"][0]
            text = load(leaf_path)
            leaf = json.loads(text)
            leaf_id = leaf.get("$id")
            if not isinstance(leaf_id, str):
                manual.append((leaf_path, "leaf has no $id"))
                continue
            leaf_ids.append(leaf_id)
            wrapper = variant.get("wrapper")
            reference = {"$ref": leaf_id if wrapper is None else "%s#/$defs/%s" % (leaf_id, wrapper.replace("~", "~0").replace("/", "~1"))}
            if tag is not None and content is None:
                branches.append(reference)
                inlined, why = inline_payload(leaf)
                if why is not None:
                    manual.append((leaf_path, why))
                    continue
                rebuilt, why = with_tag(inlined, tag, variant["wire"])
                if inlined is not leaf and why is None:
                    written.append(leaf_path)
                    if apply:
                        if load(leaf_path) != text:
                            manual.append((leaf_path, "changed while scanning; rerun"))
                            continue
                        write(leaf_path, json.dumps(rebuilt, indent=2, ensure_ascii=False) + "\n")
                    continue
                if why is not None:
                    manual.append((leaf_path, why))
                elif rebuilt is leaf:
                    unchanged += 1
                else:
                    ok, _ = canonical(leaf, text)
                    if isinstance(leaf["properties"].get(tag), dict) and leaf["properties"][tag].get("const") != variant["wire"]:
                        manual.append((leaf_path, "declares %s as %s, not const %s" % (tag, json.dumps(leaf["properties"][tag]), variant["wire"])))
                        continue
                    updated = canonical(rebuilt, text)[1] if ok else tag_by_text(text, tag, variant["wire"])
                    if json.loads(updated) != rebuilt:
                        manual.append((leaf_path, "text insertion diverges from the rebuilt document"))
                        continue
                    written.append(leaf_path)
                    if apply:
                        if load(leaf_path) != text:
                            manual.append((leaf_path, "changed while scanning; rerun"))
                            continue
                        write(leaf_path, updated)
            else:
                wire = variant["wire"]
                if tag is None:
                    branches.append({"type": "object", "additionalProperties": False, "required": [wire], "properties": {wire: reference}})
                else:
                    branches.append({"type": "object", "additionalProperties": False, "required": [tag, content], "properties": {tag: {"const": wire}, content: reference}})
        schema_path = aggregate["schema"]
        siblings = [other for other in aggregates if other["schema"] == schema_path]
        if not os.path.exists(os.path.join(REPO, schema_path)):
            if len(branches) != len(aggregate["variants"]):
                manual.append((schema_path, "aggregate schema absent and a variant has no leaf $id"))
                continue
            created.setdefault(schema_path, {"ids": leaf_ids, "members": {}})["members"][aggregate["name"]] = branches
            if len(created[schema_path]["members"]) == len(siblings):
                document = aggregate_document(schema_path, created[schema_path]["ids"], created[schema_path]["members"])
                if document is None:
                    manual.append((schema_path, "no leaf id base and no lane module to place the aggregate $id beside"))
                    continue
                written.append(schema_path)
                if apply:
                    write(schema_path, json.dumps(document, indent=2, ensure_ascii=False) + "\n")
            continue
        text = load(schema_path)
        document = json.loads(text)
        if len(siblings) > 1:
            member = document.get("$defs", {}).get(aggregate["name"], {})
            if member.get("oneOf") == branches:
                unchanged += 1
                continue
            document.setdefault("$defs", {})[aggregate["name"]] = {"title": aggregate["name"], **{key: value for key, value in member.items() if key not in ("title", "oneOf")}, "oneOf": branches}
            written.append(schema_path)
            if apply:
                write(schema_path, json.dumps(document, indent=2, ensure_ascii=False) + "\n")
            continue
        if document.get("oneOf") == branches:
            unchanged += 1
            continue
        if "oneOf" not in document:
            rebuilt = {**{key: document[key] for key in ("$schema", "$id", "title", "description") if key in document}, "oneOf": branches}
        else:
            rebuilt = {key: (branches if key == "oneOf" else value) for key, value in document.items()}
        ok, rendered = canonical(document, text)
        if ok or "oneOf" not in document:
            updated = canonical(rebuilt, text)[1]
        else:
            _, start, end = members(text)["oneOf"]
            multiline = "\n" in text
            updated = text[:start] + json.dumps(branches, indent=2 if multiline else None, ensure_ascii=False).replace("\n", "\n  ") + text[end:]
        if json.loads(updated) != rebuilt:
            manual.append((schema_path, "aggregate rewrite diverges from the rebuilt document"))
            continue
        written.append(schema_path)
        if apply:
            if load(schema_path) != text:
                manual.append((schema_path, "changed while scanning; rerun"))
                continue
            write(schema_path, updated)
    for path, why in manual:
        print("[manual] %s — %s" % (path, why))
    for path in written:
        print("[%s] %s" % ("wrote" if apply else "would write", path))
    print("[w2-s] %s %d file(s), %d already conform, %d manual" % ("wrote" if apply else "would write", len(written), unchanged, len(manual)))


if __name__ == "__main__":
    main()
