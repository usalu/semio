"""🔀️ S2-NORM WP-6: member order of committed norm wires against their schemas. The Rust wire law compares values, so
member order is pinned only by the TypeScript twin, which writes schema property order. The census reports, per artifact
and schema position, committed objects whose members are key-sorted where the schema is not (`sorted`) and objects in
another order (`other`). `--apply` makes the order declared and canonical in two steps: every named schema record whose
members equal its Rust struct's fields takes the struct's field order (the `ToValue` wire order; `required` follows), and
every committed mutation and snapshot is rewritten in schema order, value and float spelling unchanged.

  python3 🧪️s2-norm-wire-order.py [--apply] [<artifact-dir> ...]
"""

import collections
import json
import os
import re
import sys

ROOT = "/Users/ueli/Documents/semio"
ARTIFACTS = f"{ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts"


def subset(artifact):
    return f"{ARTIFACTS}/{artifact}/🏅️standards/🔖️1/🪆️subsets/✳️any"


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


class Schemas:
    def __init__(self, artifact):
        schema = f"{subset(artifact)}/🧬️schema"
        self.by_id = {}
        paths = [f"{schema}/📸️snapshot/🔣️.json", f"{schema}/🔺️diff/🔣️.json", f"{schema}/🔣️.json", f"{schema}/🧬️mutations/🔣️.json"]
        paths += [f"{schema}/🧬️mutations/{leaf}/🧬️schema/🔣️.json" for leaf in os.listdir(f"{schema}/🧬️mutations") if os.path.exists(f"{schema}/🧬️mutations/{leaf}/🧬️schema/🔣️.json")]
        for path in paths:
            document = load(path)
            self.by_id[document.get("$id")] = document
        self.snapshot = load(paths[0])
        self.aggregate = load(paths[3])

    def resolve(self, node, base):
        while isinstance(node, dict) and "$ref" in node:
            ident, _, pointer = node["$ref"].partition("#")
            document = base if ident == "" else self.by_id.get(ident)
            if document is None:
                return None, base
            target = document
            for key in [part for part in pointer.split("/") if part]:
                target = target.get(key, {}) if isinstance(target, dict) else {}
            node, base = target, document
        return node, base


def walk(schemas, value, node, base, at, found):
    node, base = schemas.resolve(node, base)
    if node is None:
        return
    for key in ("oneOf", "anyOf"):
        if key in node and isinstance(value, dict):
            for branch in node[key]:
                resolved, owner = schemas.resolve(branch, base)
                properties = (resolved or {}).get("properties", {})
                if properties and set(value) <= set(properties) and all(properties[k].get("const", value.get(k)) == value.get(k) for k in properties if "const" in properties[k]):
                    walk(schemas, value, branch, base, at, found)
                    return
            return
    if isinstance(value, dict) and "properties" in node:
        declared = [key for key in node["properties"] if key in value]
        committed = list(value)
        if committed != declared and set(committed) == set(declared):
            kind = "sorted" if committed == sorted(committed) else "other"
            found[(kind, node.get("title") or at)].append(committed)
        for key, item in value.items():
            walk(schemas, item, node["properties"].get(key, {}), base, f"{at}.{key}", found)
    elif isinstance(value, list) and "items" in node:
        for item in value:
            walk(schemas, item, node["items"], base, f"{at}[]", found)


def camel(name):
    head, *rest = name.split("_")
    return head + "".join(part[:1].upper() + part[1:] for part in rest)


def rust_structs(artifact):
    """🦀️ struct name → wire field order (camelCase), from every Rust source of the artifact."""
    found = {}
    for directory, _, names in os.walk(f"{ARTIFACTS}/{artifact}"):
        if "/target" in directory or "📦️packages" in directory:
            continue
        for name in names:
            if name != "🦀️.rs":
                continue
            source = open(os.path.join(directory, name), encoding="utf-8").read()
            for match in re.finditer(r"^pub struct (\w+)\s*\{\n(.*?)^\}", source, re.S | re.M):
                fields = [camel(field) for field in re.findall(r"^\s*pub (\w+)\s*:", match.group(2), re.M)]
                if fields:
                    found.setdefault(match.group(1), fields)
    return found


def dump(value):
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def reorder_schema(path, structs, report):
    text = open(path, encoding="utf-8").read()
    document = json.loads(text)
    if dump(document) != text:
        report.append(f"skip (not canonical JSON) {os.path.relpath(path, ROOT)}")
        return
    def record(node, name):
        properties = node.get("properties")
        fields = structs.get(name)
        if not isinstance(properties, dict) or not fields:
            return
        if set(fields) != set(properties):
            if set(properties) <= set(fields) or set(fields) <= set(properties):
                report.append(f"members differ {name}: schema-only {sorted(set(properties) - set(fields))} rust-only {sorted(set(fields) - set(properties))}")
            return
        if list(properties) != fields:
            node["properties"] = {field: properties[field] for field in fields}
            if isinstance(node.get("required"), list):
                node["required"] = [field for field in fields if field in node["required"]]
            report.append(f"reordered {name}")
    record(document, document.get("title"))
    for key, node in {**document.get("definitions", {}), **document.get("$defs", {})}.items():
        record(node, node.get("title") or key)
    updated = dump(document)
    if updated != text:
        open(path, "w", encoding="utf-8").write(updated)


def ordered(schemas, value, node, base):
    node, base = schemas.resolve(node, base)
    if node is None:
        return value
    for key in ("oneOf", "anyOf"):
        if key in node and isinstance(value, dict):
            for branch in node[key]:
                resolved, owner = schemas.resolve(branch, base)
                properties = (resolved or {}).get("properties", {})
                if properties and set(value) <= set(properties) and all(properties[k].get("const", value.get(k)) == value.get(k) for k in properties if "const" in properties[k]):
                    return ordered(schemas, value, branch, base)
            return value
    if isinstance(value, dict) and "properties" in node:
        keys = [key for key in node["properties"] if key in value] + [key for key in value if key not in node["properties"]]
        return {key: ordered(schemas, value[key], node["properties"].get(key, {}), base) for key in keys}
    if isinstance(value, list) and "items" in node:
        return [ordered(schemas, item, node["items"], base) for item in value]
    return value


def apply(artifact):
    report = []
    schema = f"{subset(artifact)}/🧬️schema"
    structs = rust_structs(artifact)
    paths = [f"{schema}/📸️snapshot/🔣️.json", f"{schema}/🔺️diff/🔣️.json", f"{schema}/🔣️.json"]
    paths += [f"{schema}/🧬️mutations/{leaf}/🧬️schema/🔣️.json" for leaf in sorted(os.listdir(f"{schema}/🧬️mutations")) if os.path.exists(f"{schema}/🧬️mutations/{leaf}/🧬️schema/🔣️.json")]
    for path in paths:
        reorder_schema(path, structs, report)
    schemas = Schemas(artifact)
    rewritten = 0
    for directory, _, names in os.walk(f"{subset(artifact)}/🧫️fixtures/🧬️mutations"):
        if "🔣️.json" not in names or not any(part in directory for part in ("🦠️mutation", "📸️snapshot")):
            continue
        path = f"{directory}/🔣️.json"
        text = open(path, encoding="utf-8").read()
        value = json.loads(text)
        root = schemas.aggregate if "🦠️mutation" in directory else schemas.snapshot
        updated = dump(ordered(schemas, value, root, root))
        if updated != text:
            open(path, "w", encoding="utf-8").write(updated)
            rewritten += 1
    print(f"== {artifact}: {sum(line.startswith('reordered') for line in report)} schema records reordered, {rewritten} fixtures rewritten")
    for line in report:
        if not line.startswith("reordered"):
            print(f"  {line}")


def main(artifacts):
    for artifact in artifacts:
        schemas = Schemas(artifact)
        found = collections.defaultdict(list)
        files = collections.Counter()
        root = f"{subset(artifact)}/🧫️fixtures/🧬️mutations"
        for directory, _, names in os.walk(root):
            if "🔣️.json" not in names or not any(part in directory for part in ("🦠️mutation", "📸️snapshot")):
                continue
            before = sum(len(rows) for rows in found.values())
            value = load(f"{directory}/🔣️.json")
            if "🦠️mutation" in directory:
                walk(schemas, value, schemas.aggregate, schemas.aggregate, "$", found)
            else:
                walk(schemas, value, schemas.snapshot, schemas.snapshot, "$", found)
            files["deviating" if sum(len(rows) for rows in found.values()) > before else "aligned"] += 1
        print(f"== {artifact} {dict(files)}")
        for (kind, position), rows in sorted(found.items()):
            orders = collections.Counter(tuple(row) for row in rows)
            print(f"  {kind:6} {position}: {len(rows)} objects, {len(orders)} distinct orders; e.g. {list(orders.most_common(1)[0][0])[:8]}")


if __name__ == "__main__":
    chosen = [argument for argument in sys.argv[1:] if argument != "--apply"] or sorted(name for name in os.listdir(ARTIFACTS) if os.path.isdir(f"{subset(name)}/🧫️fixtures"))
    for artifact in chosen:
        apply(artifact) if "--apply" in sys.argv else main([artifact])
