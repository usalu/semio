#!/usr/bin/env python3
"""🔭️ W2-R tail: walks every mutation leaf payload schema under the tail group's plugins the way the framework reader
(`mutationInputDefs`, `🛂️manifest/🟦️.ts`) does, and lists EVERY input that lacks a label or option labels (the lint stops at
the first refusal per top-level input) together with the annotation site: the document file and the JSON path of the
property node that declares the input. Uncatalogued leaves are walked too. Output: JSON lines on stdout."""
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
CATALOG = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"
GLOSSARY = "🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json"
ROOTS = ["✒️writer", "➗️mathematical", "🌊️flow", "🌍️gis", "🌿️vcs", "🎞️animate", "🎪️demonstrator", "🎬️sequence", "🏭️process", "💠️lowpoly", "💡️reasoning", "📖️playbook", "📜️imperative", "🔱️trinity", "🕸️dag", "🪐️space", "🪵️sourcing"]
EXCLUDE = ("window.",)


def load(path):
    try:
        with open(os.path.join(REPO, path), encoding="utf-8") as handle:
            return json.load(handle)
    except Exception:
        return None


def main():
    catalog = load(CATALOG)["scopes"]
    glossary = load(GLOSSARY)["labels"]
    documents = {}
    files = {}
    for scope in catalog.values():
        base = os.path.join(REPO, scope["path"])
        if not os.path.isdir(base):
            continue
        for directory, _, names in os.walk(base):
            for name in names:
                if name.endswith(".json"):
                    rel = os.path.relpath(os.path.join(directory, name), REPO)
                    if rel in files:
                        continue
                    document = load(rel)
                    files[rel] = document
                    if isinstance(document, dict) and isinstance(document.get("$id"), str):
                        documents.setdefault(document["$id"], rel)
    leaves = {v["path"]: k for k, v in catalog.items() if v.get("level") == "mutation-leaf"}
    for root in ROOTS:
        for directory, subdirs, names in os.walk(os.path.join(REPO, "✏️s/🔌️plugins", root)):
            subdirs[:] = [d for d in subdirs if d not in ("node_modules", "target", "🗑️generated")]
            rel = os.path.relpath(directory, REPO)
            parts = rel.split("/")
            if parts[-1] == "🧬️schema" and len(parts) > 2 and parts[-3] == "🧬️mutations" and "🔣️.json" in names:
                leaves.setdefault(rel, None)
    for path, scope in sorted(leaves.items()):
        if not any(path.startswith("✏️s/🔌️plugins/%s/" % root) for root in ROOTS):
            continue
        if scope is not None and scope.startswith(EXCLUDE):
            continue
        walk_leaf(path + "/🔣️.json", scope, documents, glossary)


def walk_leaf(leaf_path, scope, documents, glossary):
    cache = {}

    def document(rel):
        if rel not in cache:
            cache[rel] = load(rel)
        return cache[rel]

    def emit(**row):
        print(json.dumps({"leaf": leaf_path, "scope": scope, **row}, ensure_ascii=False))

    def target(doc, reference):
        hash_at = reference.find("#")
        ident = reference if hash_at < 0 else reference[:hash_at]
        fragment = "" if hash_at < 0 else reference[hash_at + 1:]
        owner = doc if ident == "" else documents.get(ident)
        if owner is None:
            return None
        current = document(owner)
        path = []
        for segment in [s.replace("~1", "/").replace("~0", "~") for s in fragment.split("/")[1:]]:
            if isinstance(current, dict) and segment in current:
                current = current[segment]
            elif isinstance(current, list) and segment.isdigit() and int(segment) < len(current):
                current = current[int(segment)]
                segment = int(segment)
            else:
                return None
            path.append(segment)
        return owner, path, current

    def resolve(doc, path, node):
        ui = {}
        for _ in range(32):
            if not isinstance(node, dict):
                return doc, path, {}, ui
            for key, value in (node.get("x-semio-ui") or {}).items():
                ui.setdefault(key, value)
            if isinstance(node.get("$ref"), str):
                found = target(doc, node["$ref"])
                if found is None:
                    return None
                doc, path, node = found
                continue
            union = node.get("oneOf") if isinstance(node.get("oneOf"), list) else node.get("anyOf") if isinstance(node.get("anyOf"), list) else None
            key = "oneOf" if isinstance(node.get("oneOf"), list) else "anyOf"
            if union is not None:
                concrete = [(i, b) for i, b in enumerate(union) if not (isinstance(b, dict) and b.get("type") == "null" and len(b) == 1)]
                if len(concrete) == 1 and len(concrete) < len(union):
                    path = path + [key, concrete[0][0]]
                    node = concrete[0][1]
                    continue
            return doc, path, node, ui
        return None

    def kind_of(node):
        kind = node.get("type")
        if isinstance(kind, str):
            named = [kind]
        elif isinstance(kind, list):
            named = [k for k in kind if isinstance(k, str) and k != "null"]
        elif "properties" in node:
            named = ["object"]
        elif "items" in node:
            named = ["array"]
        elif isinstance(node.get("enum"), list) and node["enum"] and all(isinstance(v, str) for v in node["enum"]):
            named = ["string"]
        else:
            named = []
        return named[0] if len(named) == 1 else None

    def options(site, resolved, pointer):
        doc, path, node, ui = resolved
        values = node.get("enum")
        if not isinstance(values, list):
            return
        labels = ui.get("options") or {}
        missing = [v for v in values if isinstance(v, str) and v not in labels and v not in glossary]
        if missing:
            emit(code="optionLabelMissing", pointer=pointer, site=site, enum=values, missing=missing, enum_site=[doc, path])

    def value(key, site, resolved, pointer):
        doc, path, node, ui = resolved
        kind = kind_of(node)
        if kind == "string":
            options(site, resolved, pointer)
        elif kind == "object":
            if "properties" in node or "allOf" in node:
                fields(resolved, pointer)
        elif kind == "array" and "items" in node:
            items_site = [doc, path + ["items"]]
            inner = resolve(doc, path + ["items"], node["items"])
            if inner is None:
                emit(code="refUnresolved", pointer=pointer + "/-", site=items_site)
                return
            item(items_site, inner, pointer + "/-")

    def item(site, resolved, pointer):
        doc, path, node, ui = resolved
        if "const" in node:
            return
        kind = kind_of(node)
        if kind == "string":
            options(site, resolved, pointer)
        elif kind == "object" and ("properties" in node or "allOf" in node):
            fields(resolved, pointer)
        elif kind == "array" and "items" in node:
            inner = resolve(doc, path + ["items"], node["items"])
            if inner is not None:
                item([doc, path + ["items"]], inner, pointer + "/-")

    def members(resolved, found, depth=0):
        found.append(resolved)
        doc, path, node, _ = resolved
        for index, member in enumerate(node.get("allOf") or []):
            inner = resolve(doc, path + ["allOf", index], member)
            if inner is not None and depth < 32:
                members(inner, found, depth + 1)
        return found

    def fields(resolved, pointer):
        seen = set()
        for member in members(resolved, []):
            doc, path, node, _ = member
            for key, child in (node.get("properties") or {}).items():
                if key in seen:
                    continue
                seen.add(key)
                child_pointer = pointer + "/" + key.replace("~", "~0").replace("/", "~1")
                site = [doc, path + ["properties", key]]
                inner = resolve(doc, path + ["properties", key], child)
                if inner is None:
                    emit(code="refUnresolved", pointer=child_pointer, site=site)
                    continue
                _, _, leaf_node, ui = inner
                if ui.get("role") == "discriminator" or "const" in leaf_node:
                    continue
                if "label" not in ui and key not in glossary:
                    emit(code="labelMissing", pointer=child_pointer, key=key, site=site, type=kind_of(leaf_node), node={k: v for k, v in leaf_node.items() if k in ("type", "enum", "description", "title", "minimum", "maximum", "exclusiveMinimum", "format", "minItems", "maxItems", "default")}, resolved=[inner[0], inner[1]])
                value(key, site, inner, child_pointer)

    root = resolve(leaf_path, [], document(leaf_path))
    if root is None:
        emit(code="refUnresolved", pointer="", site=[leaf_path, []])
        return
    node = root[2]
    if "properties" not in node and "allOf" not in node:
        if "oneOf" in node or "anyOf" in node:
            emit(code="malformed", pointer="", site=[leaf_path, []])
        return
    fields(root, "")


if __name__ == "__main__":
    sys.exit(main())
