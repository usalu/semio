#!/usr/bin/env python3
"""🗃️ W2-S norm inventory: every norm mutation aggregate and leaf with its Rust wire shape and payload schema.

Usage: python3 🧪️w2-s-norm-inventory.py [--json <out>] [--table]
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
NORM = "✏️s/🔌️plugins/📕️norm"


def strip_comments(source):
    source = re.sub(r"/\*.*?\*/", "", source, flags=re.S)
    return re.sub(r"//[^\n]*", "", source)


def split_top(text, sep=","):
    parts, depth, current = [], 0, ""
    for char in text:
        if char in "<([{":
            depth += 1
        elif char in ">)]}":
            depth -= 1
        if char == sep and depth == 0:
            parts.append(current)
            current = ""
        else:
            current += char
    if current.strip():
        parts.append(current)
    return parts


def value_attrs(attrs):
    pairs = {}
    for attr in attrs:
        match = re.match(r"\s*value\s*\((.*)\)\s*$", attr, flags=re.S)
        if not match:
            continue
        for item in split_top(match.group(1)):
            kv = re.match(r'\s*(\w+)\s*=\s*"([^"]*)"\s*$', item)
            if kv:
                pairs[kv.group(1)] = kv.group(2)
            elif item.strip():
                pairs[item.strip()] = True
    return pairs


def parse_attrs_before(source, index):
    attrs = []
    cursor = index
    while True:
        chunk = source[:cursor].rstrip()
        if not chunk.endswith("]"):
            break
        depth, pos = 0, len(chunk) - 1
        while pos >= 0:
            if chunk[pos] == "]":
                depth += 1
            elif chunk[pos] == "[":
                depth -= 1
                if depth == 0:
                    break
            pos -= 1
        if pos < 1 or chunk[pos - 1] != "#":
            break
        attrs.insert(0, chunk[pos + 1 : -1])
        cursor = pos - 1
    return attrs


def camel(words):
    return words[0] + "".join(word[:1].upper() + word[1:] for word in words[1:])


def wire_field(ident, rename, rename_all):
    if rename:
        return rename
    if rename_all == "camelCase":
        return camel(ident.split("_"))
    if rename_all == "kebab-case":
        return ident.replace("_", "-")
    return ident


def parse_struct(source, name):
    source = strip_comments(source)
    match = re.search(r"pub\s+struct\s+" + re.escape(name) + r"\b\s*(\{|\(|;)", source)
    if not match:
        return None
    attrs = parse_attrs_before(source, match.start())
    container = value_attrs(attrs)
    fields = []
    if match.group(1) == "{":
        depth, pos = 1, match.end()
        while depth and pos < len(source):
            depth += {"{": 1, "}": -1}.get(source[pos], 0)
            pos += 1
        body = source[match.end() : pos - 1]
        for part in split_top(body):
            part = part.strip()
            if not part:
                continue
            field_attrs = re.findall(r"#\[(.*?)\]\s*", part, flags=re.S)
            decl = re.sub(r"#\[.*?\]\s*", "", part, flags=re.S).strip()
            fm = re.match(r"(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:\s*(.+)$", decl, flags=re.S)
            if not fm:
                continue
            fattrs = value_attrs(field_attrs)
            fields.append({"ident": fm.group(1), "type": " ".join(fm.group(2).split()), "rename": fattrs.get("rename"), "skip": bool(fattrs.get("skip")), "default": "default" in fattrs, "wire": wire_field(fm.group(1), fattrs.get("rename"), container.get("rename_all"))})
    return {"attrs": container, "fields": fields, "tuple": match.group(1) == "("}


def parse_aggregate(path):
    source = strip_comments(open(path, encoding="utf-8").read())
    out = []
    for match in re.finditer(r"pub\s+enum\s+(\w+)\s*\{", source):
        attrs = parse_attrs_before(source, match.start())
        if not any("Mutations" in attr and attr.strip().startswith("derive") for attr in attrs):
            continue
        depth, pos = 1, match.end()
        while depth and pos < len(source):
            depth += {"{": 1, "}": -1}.get(source[pos], 0)
            pos += 1
        body = source[match.end() : pos - 1]
        variants = []
        for part in split_top(body):
            part = part.strip()
            vattrs = value_attrs(re.findall(r"#\[(.*?)\]\s*", part, flags=re.S))
            decl = re.sub(r"#\[.*?\]\s*", "", part, flags=re.S).strip()
            vm = re.match(r"(\w+)\s*\(\s*([\w:]+)\s*\)", decl)
            if vm:
                variants.append({"ident": vm.group(1), "payload": vm.group(2), "rename": vattrs.get("rename")})
            elif re.match(r"^\w+$", decl):
                variants.append({"ident": decl, "payload": None, "rename": vattrs.get("rename")})
        out.append({"name": match.group(1), "layout": value_attrs(attrs), "variants": variants})
    return out


def variant_wire(ident, rename, rename_all):
    if rename:
        return rename
    words = [w.lower() for w in re.findall(r"[A-Z][a-z0-9]*|[a-z0-9]+", ident)]
    if rename_all == "camelCase":
        return camel(words)
    if rename_all == "kebab-case":
        return "-".join(words)
    if rename_all == "snake_case":
        return "_".join(words)
    return ident


def schema_kind(name):
    return re.sub(r"^[^\w]+", "", name, flags=re.U).lstrip("️‍")


def inventory():
    roots = []
    for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, NORM)):
        dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated")]
        rel = os.path.relpath(dirpath, REPO)
        if os.path.basename(dirpath) == "🧬️mutations" and "🦀️.rs" in filenames and os.path.basename(os.path.dirname(dirpath)) == "🧬️schema":
            roots.append(rel)
    result = []
    for root in sorted(roots):
        aggregates = parse_aggregate(os.path.join(REPO, root, "🦀️.rs"))
        agg_schema_path = os.path.join(REPO, root, "🔣️.json")
        agg_schema = json.load(open(agg_schema_path, encoding="utf-8")) if os.path.exists(agg_schema_path) else None
        leaves = []
        for entry in sorted(os.listdir(os.path.join(REPO, root))):
            leaf_dir = os.path.join(root, entry)
            descriptor_path = os.path.join(REPO, leaf_dir, "🔣️.json")
            if not os.path.isdir(os.path.join(REPO, leaf_dir)) or not os.path.exists(descriptor_path):
                continue
            descriptor = json.load(open(descriptor_path, encoding="utf-8"))
            if "semanticKind" not in descriptor:
                continue
            variant = descriptor.get("aggregateVariant")
            payload = None
            for aggregate in aggregates:
                for v in aggregate["variants"]:
                    if v["ident"] == variant:
                        payload = v["payload"]
            struct_name = payload.split("::")[-1] if payload else variant
            rust_file, struct = None, None
            for candidate in ("🦀️.rs", "🦠️mutation/🦀️.rs"):
                path = os.path.join(REPO, leaf_dir, candidate)
                if os.path.exists(path):
                    parsed = parse_struct(open(path, encoding="utf-8").read(), struct_name)
                    if parsed:
                        rust_file, struct = os.path.join(leaf_dir, candidate), parsed
                        break
            schema_rel = os.path.join(leaf_dir, descriptor.get("payloadSchema", "🧬️schema/🔣️.json"))
            schema = None
            if os.path.exists(os.path.join(REPO, schema_rel)):
                try:
                    schema = json.load(open(os.path.join(REPO, schema_rel), encoding="utf-8"))
                except json.JSONDecodeError:
                    schema = "INVALID"
            leaves.append({
                "dir": leaf_dir,
                "slug": entry,
                "kind": descriptor["semanticKind"],
                "variant": variant,
                "struct": struct_name,
                "rustFile": rust_file,
                "rust": struct,
                "schemaPath": schema_rel,
                "schemaId": schema.get("$id") if isinstance(schema, dict) else None,
                "schemaProps": list(schema.get("properties", {}).keys()) if isinstance(schema, dict) else None,
                "schemaRequired": schema.get("required") if isinstance(schema, dict) else None,
                "schemaDialect": schema.get("$schema") if isinstance(schema, dict) else None,
            })
        result.append({"root": root, "aggregates": aggregates, "aggregateSchemaId": agg_schema.get("$id") if agg_schema else None, "leaves": leaves})
    return result


def table(inv):
    for group in inv:
        print(f"=== {group['root']}")
        for aggregate in group["aggregates"]:
            print(f"  aggregate {aggregate['name']} layout={aggregate['layout']} variants={len(aggregate['variants'])}")
        print(f"  aggregate schema $id={group['aggregateSchemaId']}")
        for leaf in group["leaves"]:
            rust = leaf["rust"]
            wires = [f["wire"] for f in rust["fields"]] if rust else None
            props = leaf["schemaProps"]
            status = "noRust" if rust is None else ("match" if props is not None and sorted(wires) == sorted(props) else "DRIFT")
            print(f"  {status:6} {leaf['kind']:44} camel={bool(rust and rust['attrs'].get('rename_all'))!s:5} rust={wires} schema={props}")


if __name__ == "__main__":
    inv = inventory()
    if "--json" in sys.argv:
        json.dump(inv, open(sys.argv[sys.argv.index("--json") + 1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    if "--table" in sys.argv:
        table(inv)
