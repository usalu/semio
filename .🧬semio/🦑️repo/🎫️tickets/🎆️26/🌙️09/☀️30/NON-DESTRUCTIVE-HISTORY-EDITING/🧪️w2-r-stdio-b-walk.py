#!/usr/bin/env python3
"""🔭️ W2-R stdio-b: walks every in-scope stdio mutation leaf payload schema the way the manifest reader
(`mutationInputDefs`, `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`) does and lists EVERY input (top-level and nested)
that still lacks a label or option labels, with the file and JSON path of the node that has to carry the
`x-semio-ui` annotation, the resolved value shape, and whether the input is a recursion back-edge (its value
re-enters an object already being expanded, which the reader cannot survive once every field is labelled).

    python3 🧪️w2-r-stdio-b-walk.py > findings.json
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
CATALOG = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"
GLOSSARY = "🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json"
MINE = set("📇️inventory 📸️jpg 🧾️json ☁️las 📝️md 🎵️mp3 🎥️mp4 🗽️obj 📖️pdf 🧱️ply 📷️png 📽️pptx 🧿️semio 📐️step 🔺️stl 🎨️svg 🖼️tiff 📑️tsv 🔤️txt 🔊️wav 📕️xlsx 📰️xml 🎒️zip".split())
TYPES = ("string", "integer", "number", "boolean", "object", "array")


def load(path):
    try:
        with open(os.path.join(REPO, path), encoding="utf-8") as handle:
            return json.load(handle)
    except (OSError, ValueError):
        return None


def in_scope(path):
    parts = path.split("/")
    return len(parts) > 4 and parts[2] == "🗄️stdio" and parts[3] == "🗿️artifacts" and parts[4] in MINE


def index_documents(catalog):
    paths = {scope["path"] for scope in catalog.values()}
    documents = {}

    def walk(directory):
        try:
            entries = list(os.scandir(os.path.join(REPO, directory)))
        except OSError:
            return
        for entry in entries:
            path = directory + "/" + entry.name
            if entry.is_dir() and path not in paths:
                walk(path)
            elif entry.is_file() and entry.name.endswith(".json"):
                document = load(path)
                if isinstance(document, dict) and isinstance(document.get("$id"), str):
                    documents[document["$id"]] = (path, document)

    for path in paths:
        walk(path)
    return documents


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
    return named[0] if len(named) == 1 and named[0] in TYPES else None


def inferred_reference(key, many):
    suffix = next((s for s in (("Ids", "_ids") if many else ("Id", "_id")) if key.endswith(s)), None)
    if suffix is None:
        return None
    stem = key[: len(key) - len(suffix)]
    if stem.startswith("new"):
        if stem[3:] == "":
            return None
        if stem[3:4].isupper():
            stem = stem[3:]
    return stem[0].lower() + stem[1:] if re.match(r"^[A-Za-z][A-Za-z0-9_]*$", stem) else None


def parent_of(file, path, leaf_path):
    for index in range(len(path) - 1, -1, -1):
        if path[index] in ("$defs", "definitions") and index + 1 < len(path):
            return path[index + 1]
    if file == leaf_path:
        return "leaf:" + re.sub(r"^[^a-z]*", "", file.split("/")[-3])
    return "doc:" + file.split("/")[-2]


def subset_of(file):
    parts = file.split("/")
    return re.sub(r"^[^a-z0-9]*", "", parts[parts.index("🪆️subsets") + 1]) if "🪆️subsets" in parts else None


class Resolved:
    def __init__(self, document, file, path, node, ui, site):
        self.document, self.file, self.path, self.node, self.ui, self.site = document, file, path, node, ui, site


class Walker:
    def __init__(self, leaf_path, leaf, documents, glossary, everything=False):
        self.leaf_path, self.leaf, self.documents, self.glossary, self.everything = leaf_path, leaf, documents, glossary, everything
        self.found = []
        self.trail = []
        self.guards = set()

    def target(self, document, reference):
        head, _, fragment = reference.partition("#")
        owner = head if head else document
        if owner is None:
            file, current = self.leaf_path, self.leaf
        elif owner in self.documents:
            file, current = self.documents[owner]
        else:
            raise LookupError("refUnresolved " + reference)
        path = []
        for segment in [part.replace("~1", "/").replace("~0", "~") for part in fragment.split("/")[1:]]:
            if isinstance(current, dict):
                current = current.get(segment)
            elif isinstance(current, list) and segment.isdigit():
                current = current[int(segment)]
            else:
                current = None
            path.append(segment)
            if current is None:
                raise LookupError("refUnresolved " + reference)
        return owner, file, path, current

    def resolve(self, document, file, path, node):
        ui, site = {}, (file, list(path))
        for _ in range(32):
            if not isinstance(node, dict):
                return Resolved(document, file, path, {}, ui, site)
            if isinstance(node.get("x-semio-ui"), dict):
                for key, value in node["x-semio-ui"].items():
                    ui.setdefault(key, value)
            if isinstance(node.get("$ref"), str):
                document, file, path, node = self.target(document, node["$ref"])
                continue
            name = "oneOf" if isinstance(node.get("oneOf"), list) else "anyOf" if isinstance(node.get("anyOf"), list) else None
            if name is not None:
                concrete = [(i, b) for i, b in enumerate(node[name]) if not (isinstance(b, dict) and b.get("type") == "null" and len(b) == 1)]
                if len(concrete) == 1 and len(concrete) < len(node[name]):
                    path, node = path + [name, str(concrete[0][0])], concrete[0][1]
                    continue
            return Resolved(document, file, path, node, ui, site)
        raise LookupError("malformed ref chain")

    def shape(self, resolved):
        node = resolved.node
        shape = {"type": kind_of(node)}
        for key in ("enum", "minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "minItems", "maxItems", "minLength", "maxLength", "pattern", "description", "title", "default", "format"):
            if key in node:
                shape[key] = node[key]
        if shape["type"] == "array" and "items" in node:
            try:
                items = self.resolve(resolved.document, resolved.file, resolved.path + ["items"], node["items"])
                shape["itemType"] = kind_of(items.node)
                if "enum" in items.node:
                    shape["itemEnum"] = items.node["enum"]
            except LookupError:
                shape["itemType"] = None
        return shape

    def record(self, problem, pointer, key, resolved, extra=None):
        file, path = resolved.site
        entry = {"problem": problem, "leaf": self.leaf_path, "pointer": pointer, "key": key, "site": {"file": file, "path": path}, "parent": parent_of(file, path, self.leaf_path), "subset": subset_of(file) or subset_of(self.leaf_path), "shape": self.shape(resolved), "annotated": sorted(resolved.ui)}
        entry.update(extra or {})
        self.found.append(entry)

    def options(self, resolved, pointer, key):
        values = resolved.node.get("enum")
        if not isinstance(values, list):
            return
        labels = resolved.ui.get("options") if isinstance(resolved.ui.get("options"), dict) else {}
        missing = [v for v in values if isinstance(v, str) and v not in labels and v not in self.glossary]
        if missing:
            self.record("optionLabelMissing", pointer, key, resolved, {"missing": missing})

    def expand(self, resolved, pointer, stack):
        identity = (resolved.file, "/".join(resolved.path))
        entered = next((depth for known, depth in stack if known == identity), None)
        if entered is not None:
            open_inputs = [step for step in self.trail[entered:] if not step[1]]
            if open_inputs:
                self.guards.add(open_inputs[-1][0])
            return False
        self.fields(resolved, pointer, stack + [(identity, len(self.trail))])
        return True

    def item(self, resolved, pointer, key, stack):
        node = resolved.node
        if "const" in node:
            return True
        kind = kind_of(node)
        if kind == "string":
            self.options(resolved, pointer, key)
        elif kind == "object" and ("properties" in node or "allOf" in node):
            return self.expand(resolved, pointer, stack)
        elif kind == "array" and "items" in node:
            return self.item(self.resolve(resolved.document, resolved.file, resolved.path + ["items"], node["items"]), pointer + "/-", key, stack)
        return True

    def value(self, key, resolved, role, widget, pointer, stack):
        node = resolved.node
        kind = kind_of(node)
        items = self.resolve(resolved.document, resolved.file, resolved.path + ["items"], node["items"]) if kind == "array" and "items" in node else None
        item_kind = kind_of(items.node) if items else None
        reference_shaped = kind == "string" or (kind == "array" and item_kind == "string")
        explicit = "ref" in resolved.ui or widget == "reference" or role == "target"
        inferred = inferred_reference(key, kind == "array") if role is None and widget is None and reference_shaped and "enum" not in node else None
        if explicit or inferred is not None:
            return True
        if kind == "string":
            self.options(resolved, pointer, key)
        elif kind == "object" and ("properties" in node or "allOf" in node):
            return self.expand(resolved, pointer, stack)
        elif kind == "array" and items is not None:
            fixed = node.get("minItems") == node.get("maxItems") and isinstance(node.get("minItems"), int) and 2 <= node["minItems"] <= 4
            if not (widget == "vector" or (item_kind in ("integer", "number") and fixed)):
                return self.item(items, pointer + "/-", key, stack)
        return True

    def input(self, key, resolved, pointer, stack):
        role = resolved.ui.get("role")
        if role == "discriminator" or "const" in resolved.node:
            return
        labelled = "label" in resolved.ui or key in self.glossary
        if not labelled:
            self.record("labelMissing", pointer, key, resolved)
        elif self.everything:
            self.record("input", pointer, key, resolved)
        self.trail.append((json.dumps(resolved.site), labelled))
        try:
            self.value(key, resolved, role, resolved.ui.get("widget"), pointer, stack)
        finally:
            self.trail.pop()

    def members(self, resolved, found):
        found.append(resolved)
        for index, member in enumerate(resolved.node.get("allOf", []) if isinstance(resolved.node.get("allOf"), list) else []):
            self.members(self.resolve(resolved.document, resolved.file, resolved.path + ["allOf", str(index)], member), found)
        return found

    def fields(self, resolved, pointer, stack):
        seen = set()
        for member in self.members(resolved, []):
            for key, child in (member.node.get("properties") or {}).items():
                if key in seen:
                    continue
                seen.add(key)
                child_pointer = pointer + "/" + key.replace("~", "~0").replace("/", "~1")
                try:
                    self.input(key, self.resolve(member.document, member.file, member.path + ["properties", key], child), child_pointer, stack)
                except LookupError as error:
                    self.found.append({"problem": str(error).split(" ")[0], "leaf": self.leaf_path, "pointer": child_pointer, "key": key, "detail": str(error)})

    def run(self):
        try:
            payload = self.resolve(None, self.leaf_path, [], self.leaf)
        except LookupError as error:
            self.found.append({"problem": str(error).split(" ")[0], "leaf": self.leaf_path, "pointer": "", "detail": str(error)})
            return
        if "properties" not in payload.node and "allOf" not in payload.node:
            if "oneOf" in payload.node or "anyOf" in payload.node:
                self.found.append({"problem": "malformed", "leaf": self.leaf_path, "pointer": ""})
            return
        self.fields(payload, "", [((payload.file, "/".join(payload.path)), 0)])
        for entry in self.found:
            if entry.get("problem") == "labelMissing":
                entry["backEdge"] = json.dumps([entry["site"]["file"], entry["site"]["path"]]) in self.guards


def findings(everything=False):
    catalog = load(CATALOG)["scopes"]
    documents = index_documents(catalog)
    glossary = load(GLOSSARY)["labels"]
    report = []
    for scope_id, scope in sorted(catalog.items()):
        if scope.get("level") != "mutation-leaf" or not in_scope(scope["path"]):
            continue
        leaf_path = scope["path"] + "/" + scope["formats"].get("🔣️jsonschema", "🔣️.json")
        walker = Walker(leaf_path, load(leaf_path), documents, glossary, everything)
        walker.run()
        for entry in walker.found:
            entry["scope"] = scope_id
            entry["artifact"] = leaf_path.split("/")[4]
            report.append(entry)
    return report


if __name__ == "__main__":
    json.dump(findings(), sys.stdout, indent=1, ensure_ascii=False)
