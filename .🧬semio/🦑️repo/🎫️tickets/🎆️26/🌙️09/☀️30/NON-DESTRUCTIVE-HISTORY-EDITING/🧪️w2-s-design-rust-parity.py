#!/usr/bin/env python3
"""🦀️ W2-S-C Rust ↔ leaf-schema parity scan for the design group.

For every mutation leaf (descriptor `🔣️.json` with `aggregateVariant` + `payloadSchema`) under the given roots, reads the
`#[derive(dsl::MutationLeaf)]` payload struct of the leaf's `🦀️.rs` and compares its value_derive wire form with the leaf
payload schema, recursing into every named `ToValue` struct and unit enum of the owning plugin that a field reaches
(directly, through `Option`, `Vec` or `Box`) where the schema holds an inline object/array/enum node:

- `missing`   a wire field the schema does not declare;
- `extra`     a schema property no Rust field emits (the aggregate tag excluded);
- `nullable`  an `Option` field value_derive emits as `null` whose schema does not admit `null`;
- `required`  a field that is always emitted (non-Option, non-skipped) but not `required`, or a skippable one that is;
- `enum`      a unit-enum field whose schema `enum` differs from the variant wire names.

Field wire names follow value_derive (`rename`, container `rename_all`); `Option<T>` without `skip_serializing_if` is
emitted as `null`; a field is omitted only under `skip_serializing_if`/`skip`.

Usage: python3 🧪️w2-s-design-rust-parity.py <root>… [--json]
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
SKIPPED = {"node_modules", "target", "🗑️generated", "dist"}


def words(ident):
    return [part.lower() for part in ident.split("_") if part]


def pascal_words(ident):
    return [part.lower() for part in re.findall(r"[A-Z]+[a-z0-9]*|[a-z0-9]+", ident)]


def cased(parts, rename_all, fallback):
    if rename_all == "camelCase":
        return parts[0] + "".join(part[:1].upper() + part[1:] for part in parts[1:])
    if rename_all == "kebab-case":
        return "-".join(parts)
    if rename_all == "lowercase":
        return "".join(parts)
    if rename_all == "snake_case":
        return "_".join(parts)
    return fallback


def value_pairs(attribute):
    match = re.match(r"\s*value\s*\((.*)\)\s*$", attribute, re.S)
    if match is None:
        return {}
    return {item.group(1): item.group(2) for item in re.finditer(r'(\w+)(?:\s*=\s*"([^"]*)")?', match.group(1))}


def split_top(text):
    depth, start, items = 0, 0, []
    for index, char in enumerate(text):
        if char in "<([{":
            depth += 1
        elif char in ">)]}":
            depth -= 1
        elif char == "," and depth == 0:
            items.append(text[start:index])
            start = index + 1
    items.append(text[start:])
    return [item for item in items if item.strip()]


ITEM = re.compile(r"^[ \t]*pub (struct|enum) (\w+)(?:<[^>{]*>)?\s*\{", re.M)


def leading_attributes(source, start):
    lines, cursor = [], start
    while cursor > 0:
        previous = source.rfind("\n", 0, cursor - 1)
        line = source[previous + 1:cursor].strip()
        if line.startswith("#[") or line.startswith("///") or line.startswith("//") or (lines and lines[-1].startswith("#[") and not line.startswith("pub") and line and not line.endswith(";") and not line.endswith("}")):
            lines.append(line)
            cursor = previous + 1
            if previous < 0:
                break
        else:
            break
    return "\n".join(reversed([line for line in lines if not line.startswith("//")]))


class Match:
    def __init__(self, match, attributes):
        self.match, self.attributes = match, attributes

    def group(self, index):
        return (self.attributes, self.match.group(1), self.match.group(2))[index - 1]

    def end(self):
        return self.match.end()


def item_matches(source):
    for match in ITEM.finditer(source):
        yield Match(match, leading_attributes(source, match.start()))


def items(source):
    for match in item_matches(source):
        attributes = match.group(1)
        container = {}
        for attribute in re.findall(r"#\[(value\s*\(.*?\))\]", attributes, re.S):
            container.update(value_pairs(attribute))
        depth, index = 1, match.end()
        while depth > 0 and index < len(source):
            depth += {"{": 1, "}": -1}.get(source[index], 0)
            index += 1
        body = re.sub(r"//[^\n]*", "", source[match.end():index - 1])
        members = []
        for chunk in split_top(body):
            attrs = re.findall(r"#\[(.*?)\]\s*(?=#|pub|\w)", chunk, re.S)
            declaration = re.sub(r"#\[.*?\]\s*(?=#|pub|\w)", "", chunk, flags=re.S).strip()
            pairs = {}
            for attribute in attrs:
                pairs.update(value_pairs(attribute))
            if match.group(2) == "struct":
                found = re.match(r"(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:\s*(.+)$", declaration, re.S)
                if found is not None:
                    members.append({"ident": found.group(1), "type": " ".join(found.group(2).split()), "attrs": pairs})
            else:
                found = re.match(r"(\w+)\s*(.*)$", declaration, re.S)
                if found is not None:
                    members.append({"ident": found.group(1), "unit": found.group(2).strip() == "", "attrs": pairs})
        yield match.group(2), match.group(3), attributes, container, members


def registry(root):
    types = {}
    for directory, subdirectories, files in os.walk(os.path.join(REPO, root)):
        subdirectories[:] = [name for name in subdirectories if name not in SKIPPED]
        if "🦀️.rs" in files:
            for kind, name, attributes, container, members in items(open(os.path.join(directory, "🦀️.rs"), encoding="utf-8").read()):
                if "ToValue" in attributes and name not in types:
                    types[name] = {"kind": kind, "container": container, "members": members, "attributes": attributes}
    return types


def admits_null(node):
    kinds = node.get("type")
    if kinds == "null" or (isinstance(kinds, list) and "null" in kinds):
        return True
    if "type" not in node and "$ref" not in node and not any(key in node for key in ("oneOf", "anyOf", "enum", "const", "properties", "items")):
        return True
    return any(admits_null(branch) for key in ("oneOf", "anyOf") for branch in node.get(key, []) if isinstance(branch, dict))


def concrete(node):
    for key in ("oneOf", "anyOf"):
        branches = [branch for branch in node.get(key, []) if isinstance(branch, dict) and branch != {"type": "null"}]
        if len(branches) == 1 and "properties" not in node:
            return branches[0]
    return node


def inner_type(text):
    text = text.strip()
    for wrapper in ("Option<", "Vec<", "Box<"):
        if text.startswith(wrapper) and text.endswith(">"):
            return wrapper[:-1], text[len(wrapper):-1]
    return None, text


def compare(types, rust_type, node, path, rows, leaf, seen):
    wrapper, inner = inner_type(rust_type)
    node = concrete(node)
    if wrapper == "Option" or wrapper == "Box":
        return compare(types, inner, node, path, rows, leaf, seen)
    if wrapper == "Vec":
        if isinstance(node.get("items"), dict):
            compare(types, inner, node["items"], path + "/-", rows, leaf, seen)
        return
    name = inner.split("::")[-1].strip()
    declared = types.get(name)
    if declared is None or "$ref" in node or (name, path) in seen:
        return
    seen.add((name, path))
    container = declared["container"]
    if declared["kind"] == "enum":
        if all(member["unit"] for member in declared["members"]) and isinstance(node.get("enum"), list):
            expected = [member["attrs"].get("rename") or cased(pascal_words(member["ident"]), container.get("rename_all"), member["ident"]) for member in declared["members"]]
            actual = [value for value in node["enum"] if value is not None]
            if sorted(expected) != sorted(actual):
                rows.append({"leaf": leaf, "class": "enum", "field": path, "detail": f"{name} wires {expected}, the schema enumerates {actual}"})
        return
    if not isinstance(node.get("properties"), dict):
        return
    fields(types, name, container, declared["members"], node, path, rows, leaf, seen, None)


def fields(types, name, container, members, node, path, rows, leaf, seen, tag):
    properties = node.get("properties", {})
    required = set(node.get("required", []))
    emitted = set()
    for field in members:
        attrs = field["attrs"]
        if "flatten" in attrs:
            continue
        key = attrs.get("rename") or cased(words(field["ident"]), container.get("rename_all"), field["ident"])
        emitted.add(key)
        option = field["type"].startswith("Option<")
        skipped = "skip_serializing_if" in attrs or "skip" in attrs or "skip_serializing" in attrs
        child = properties.get(key)
        where = f"{path}/{key}"
        if child is None:
            rows.append({"leaf": leaf, "class": "missing", "field": where, "detail": f"{name}.{field['ident']}: {field['type']}"})
            continue
        if option and not skipped and not admits_null(child):
            rows.append({"leaf": leaf, "class": "nullable", "field": where, "detail": f"{name}.{field['ident']}: {field['type']} is emitted as null"})
        if not skipped and not option and key not in required:
            rows.append({"leaf": leaf, "class": "required", "field": where, "detail": f"{name}.{field['ident']} is always emitted but optional in the schema"})
        if skipped and key in required:
            rows.append({"leaf": leaf, "class": "required", "field": where, "detail": f"{name}.{field['ident']} may be omitted but is required in the schema"})
        compare(types, field["type"], child, where, rows, leaf, seen)
    for key in properties:
        if key not in emitted and key != tag:
            rows.append({"leaf": leaf, "class": "extra", "field": f"{path}/{key}", "detail": f"{name} emits no {key!r}"})


def plugin_root(relative):
    segments = relative.split("/")
    return "/".join(segments[: segments.index("🔌️plugins") + 2])


def scan(root, cache):
    rows = []
    for directory, subdirectories, files in os.walk(os.path.join(REPO, root)):
        subdirectories[:] = [name for name in subdirectories if name not in SKIPPED and not name.endswith("fixtures")]
        sources = [os.path.join(directory, name) for name in ("🦀️.rs", "🦠️mutation/🦀️.rs") if os.path.isfile(os.path.join(directory, name))]
        if "🔣️.json" not in files or not sources or "/🧬️mutations/" not in directory:
            continue
        try:
            descriptor = json.load(open(os.path.join(directory, "🔣️.json"), encoding="utf-8"))
        except ValueError:
            continue
        if not isinstance(descriptor, dict) or not isinstance(descriptor.get("aggregateVariant"), str):
            continue
        relative = os.path.relpath(directory, REPO)
        plugin = plugin_root(relative)
        types = cache.setdefault(plugin, registry(plugin))
        leaf_items = [item for source in sources for item in items(open(source, encoding="utf-8").read()) if "MutationLeaf" in item[2]]
        schema = json.load(open(os.path.join(directory, descriptor["payloadSchema"]), encoding="utf-8"))
        if not leaf_items or leaf_items[0][0] != "struct":
            rows.append({"leaf": relative, "class": "unparsed", "field": None, "detail": "no MutationLeaf struct with named fields"})
            continue
        _, name, _, container, members = leaf_items[0]
        wire_keys = {field["attrs"].get("rename") or cased(words(field["ident"]), container.get("rename_all"), field["ident"]) for field in members}
        tag = next((key for key, value in schema.get("properties", {}).items() if isinstance(value, dict) and "const" in value and key not in wire_keys), None)
        fields(types, name, container, members, schema, "", rows, relative, set(), tag)
    return rows


def main():
    roots = [arg for arg in sys.argv[1:] if arg != "--json"]
    cache = {}
    rows = [row for root in roots for row in scan(root, cache)]
    if "--json" in sys.argv:
        print(json.dumps(rows, indent=1, ensure_ascii=False))
    else:
        for row in rows:
            print(f"[{row['class']}] {row['leaf'].split('/🧬️mutations/')[-1]} :: {row['field']} — {row['detail']}")
        print(f"[w2-s-c] {len(rows)} parity finding(s)")


if __name__ == "__main__":
    main()
