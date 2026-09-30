#!/usr/bin/env python3
"""🔬️ W2-S-E leaf parity probe: every `#[derive(MutationLeaf)]` struct under the given roots against its leaf payload schema.

For each leaf descriptor (`aggregateVariant` + `payloadSchema`) it finds the leaf struct in the leaf's `🦀️.rs` files, derives
each field's wire name (`#[value(rename)]`, else the container `#[value(rename_all)]`, else the ident), and reports:
missing/extra schema properties, `required` drift (non-`Option` fields without `#[value(default)]` are required), `Option`
fields whose schema does not admit `null`, and scalar type drift (String/number/integer/bool/array).

Usage: python3 🧪️w2-s-e-leaf-parity.py <root>…
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
SCALARS = {"String": "string", "str": "string", "bool": "boolean", "f64": "number", "f32": "number"}
INTEGERS = {"u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize"}


def camel(ident):
    words = ident.split("_")
    return words[0] + "".join(word[:1].upper() + word[1:] for word in words[1:])


def cased(ident, rename_all):
    if rename_all == "camelCase":
        return camel(ident)
    if rename_all == "kebab-case":
        return ident.replace("_", "-")
    return ident


def value_attrs(text):
    pairs = dict(re.findall(r'(\w+)\s*=\s*"([^"]*)"', text))
    flags = set(re.findall(r'(?<![\w"=])\b(default|skip)\b(?!\s*=)', text))
    return pairs, flags


def structs(source):
    """Every `pub struct Name { … }` with its container `#[value]` attributes and fields (wire, type, attrs)."""
    found = {}
    for match in re.finditer(r'((?:#\[[^\]]*(?:\[[^\]]*\][^\]]*)*\]\s*)*)pub struct (\w+)\s*\{', source):
        attributes, name = match.group(1), match.group(2)
        start = match.end()
        depth, index = 1, start
        while depth > 0 and index < len(source):
            depth += {"{": 1, "}": -1}.get(source[index], 0)
            index += 1
        body = source[start:index - 1]
        container = " ".join(re.findall(r'#\[value\(([^\]]*)\)\]', attributes))
        rename_all = value_attrs(container)[0].get("rename_all")
        fields, pending = [], ""
        depth, token = 0, ""
        for char in body:
            if char in "<([":
                depth += 1
            elif char in ">)]":
                depth -= 1
            if char == "," and depth == 0:
                fields.append(token)
                token = ""
            else:
                token += char
        if token.strip():
            fields.append(token)
        parsed = []
        for field in fields:
            lines = [line.strip() for line in field.strip().splitlines() if line.strip() and not line.strip().startswith("//")]
            attrs = " ".join(line for line in lines if line.startswith("#["))
            decl = " ".join(line for line in lines if not line.startswith("#["))
            decl_match = re.match(r'pub\s+(\w+)\s*:\s*(.+)$', decl)
            if decl_match is None:
                continue
            ident, ty = decl_match.group(1), decl_match.group(2).strip()
            value = " ".join(re.findall(r'#\[value\(([^\]]*)\)\]', attrs))
            pairs, flags = value_attrs(value)
            parsed.append({"ident": ident, "wire": pairs.get("rename", cased(ident, rename_all)), "type": ty, "default": "default" in flags or "default" in pairs, "skip_none": pairs.get("skip_serializing_if") == "Option::is_none"})
        found[name] = {"derive": "MutationLeaf" in attributes, "renameAll": rename_all, "fields": parsed}
    return found


def schema_types(node, documents):
    if not isinstance(node, dict):
        return set()
    if "$ref" in node and node["$ref"].startswith("#/"):
        target = documents
        for segment in node["$ref"][2:].split("/"):
            target = target.get(segment, {}) if isinstance(target, dict) else {}
        return schema_types(target, documents) | {"ref"}
    if "$ref" in node:
        return {"ref"}
    types = node.get("type")
    result = set([types] if isinstance(types, str) else types or [])
    for keyword in ("oneOf", "anyOf"):
        for branch in node.get(keyword, []):
            result |= schema_types(branch, documents)
    if "const" in node and not result:
        result.add("const")
    return result


def expected(ty):
    inner = ty
    option = False
    if ty.startswith("Option<") and ty.endswith(">"):
        option, inner = True, ty[7:-1].strip()
    if inner in SCALARS:
        return option, {SCALARS[inner]}
    if inner in INTEGERS:
        return option, {"integer"}
    if inner.startswith("Vec<") or inner.startswith("["):
        return option, {"array"}
    return option, None


def main():
    roots = sys.argv[1:]
    total, drift = 0, 0
    for root in roots:
        for directory, _dirs, files in os.walk(os.path.join(REPO, root)):
            if "🔣️.json" not in files or "/🧬️mutations/" not in directory or any(part.endswith("fixtures") for part in directory.split("/")):
                continue
            try:
                descriptor = json.load(open(os.path.join(directory, "🔣️.json"), encoding="utf-8"))
            except (ValueError, OSError):
                continue
            if not isinstance(descriptor, dict) or not isinstance(descriptor.get("aggregateVariant"), str) or not isinstance(descriptor.get("payloadSchema"), str):
                continue
            leaf_structs = {}
            for sub, _d, names in os.walk(directory):
                if "🧪️tests" in sub.split("/"):
                    continue
                for name in names:
                    if name == "🦀️.rs":
                        leaf_structs.update({key: value for key, value in structs(open(os.path.join(sub, name), encoding="utf-8").read()).items() if value["derive"]})
            rel = os.path.relpath(directory, REPO)
            total += 1
            if len(leaf_structs) != 1:
                print("[?] %s — %d MutationLeaf structs (%s)" % (rel, len(leaf_structs), ", ".join(leaf_structs)))
                continue
            (name, struct), = leaf_structs.items()
            schema_path = os.path.join(directory, descriptor["payloadSchema"])
            schema = json.load(open(schema_path, encoding="utf-8"))
            root_node = schema
            while "$ref" in root_node and root_node["$ref"].startswith("#/") and "properties" not in root_node:
                target = schema
                for segment in root_node["$ref"][2:].split("/"):
                    target = target[segment]
                root_node = target
            properties = root_node.get("properties", {})
            required = set(root_node.get("required", []))
            wires = {field["wire"]: field for field in struct["fields"]}
            problems = []
            tag = [key for key, value in properties.items() if isinstance(value, dict) and "const" in value and key not in wires]
            for wire, field in wires.items():
                option, types = expected(field["type"])
                if wire not in properties:
                    problems.append("missing property %s (%s)" % (wire, field["type"]))
                    continue
                must = not option and not field["default"]
                if must and wire not in required:
                    problems.append("%s is not required" % wire)
                if not must and wire in required:
                    problems.append("%s is required although %s" % (wire, "Option" if option else "#[value(default)]"))
                found = schema_types(properties[wire], schema)
                if option and not field["skip_none"] and "null" not in found:
                    problems.append("%s (%s) does not admit null" % (wire, field["type"]))
                if types is not None and found and not (types & found) and "ref" not in found:
                    problems.append("%s typed %s, Rust %s" % (wire, sorted(found), field["type"]))
            for key in properties:
                if key not in wires and key not in tag:
                    problems.append("extra property %s" % key)
            if problems:
                drift += 1
                print("[drift] %s (%s, rename_all=%s)" % (rel, name, struct["renameAll"]))
                for problem in problems:
                    print("        %s" % problem)
    print("[w2-s-e] %d leaves, %d drifting" % (total, drift))


if __name__ == "__main__":
    main()
