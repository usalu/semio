#!/usr/bin/env python3
"""🚶️ W2-R stdio-a: a Python mirror of the manifest reader's traversal (`mutationInputDefs`, `🛂️manifest/🟦️.ts`) that
enumerates every mutation input of every leaf payload schema in scope — nested object fields, array item records and
root-union variants — with the schema node that carries its `x-semio-ui` (the "site"). Imported by the annotator and
runnable alone: `python3 🧪️w2-r-stdio-a-walk.py [--missing]` prints the inventory as JSON lines."""
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
BASE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
ARTIFACTS = ["📼️avi", "💬️bcf", "💾️binary", "🪟️bmp", "🏃️commands", "🛂️contract", "📊️csv", "🗜️deflate", "📜️docx", "🖊️dwg", "🖋️dxf", "🌦️epw", "🎞️gif", "🧊️gltf", "🕸️graph", "🌐️html", "🏗️ifc"]
CATALOG = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"
GLOSSARY = "🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json"
INPUT_TYPES = {"string", "integer", "number", "boolean", "object", "array"}


def load(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return json.load(handle)


def catalog_scopes():
    return load(CATALOG)["scopes"]


def leaf_paths():
    """🍃️ Every catalogued mutation leaf schema file under the group's artifacts, sorted."""
    out = []
    for scope in catalog_scopes().values():
        if scope.get("level") == "mutation-leaf" and any(scope["path"].startswith(BASE + artifact + "/") for artifact in ARTIFACTS):
            out.append(scope["path"] + "/" + scope["formats"].get("🔣️jsonschema", "🔣️.json"))
    return sorted(out)


def document_index():
    """🗂️ `$id` → repo-relative file, over every JSON file inside the catalogued scope directories (the lint's resolver)."""
    scopes = catalog_scopes()
    scope_paths = {scope["path"] for scope in scopes.values()}
    index = {}

    def visit(directory):
        try:
            entries = list(os.scandir(os.path.join(REPO, directory)))
        except FileNotFoundError:
            return
        for entry in entries:
            path = directory + "/" + entry.name
            if entry.is_dir() and path not in scope_paths:
                visit(path)
            elif entry.is_file() and entry.name.endswith(".json"):
                try:
                    document = load(path)
                except Exception:
                    continue
                if isinstance(document, dict) and isinstance(document.get("$id"), str):
                    index[document["$id"]] = path

    for path in scope_paths:
        visit(path)
    return index


def glossary():
    return load(GLOSSARY)["labels"]


def input_type(node):
    kind = node.get("type")
    if isinstance(kind, str):
        named = [kind]
    elif isinstance(kind, list):
        named = [name for name in kind if isinstance(name, str) and name != "null"]
    elif "properties" in node:
        named = ["object"]
    elif "items" in node:
        named = ["array"]
    elif isinstance(node.get("enum"), list) and node["enum"] and all(isinstance(value, str) for value in node["enum"]):
        named = ["string"]
    else:
        named = []
    return named[0] if len(named) == 1 and named[0] in INPUT_TYPES else None


class Walker:
    """🚶️ Mirrors `inputSchemaReader`: resolve (ref chain + nullable unwrap + ui merge), fields over allOf, variants, recursion."""

    def __init__(self, index, labels):
        self.index = index
        self.labels = labels
        self.cache = {}
        self.stack = []

    def document(self, file):
        if file not in self.cache:
            self.cache[file] = load(file)
        return self.cache[file]

    def target(self, file, reference):
        hash_at = reference.find("#")
        ident = reference if hash_at < 0 else reference[:hash_at]
        fragment = "" if hash_at < 0 else reference[hash_at + 1:]
        owner = file if ident == "" else self.index.get(ident)
        if owner is None:
            raise LookupError("refUnresolved " + reference)
        current = self.document(owner)
        path = []
        for segment in fragment.split("/")[1:]:
            segment = segment.replace("~1", "/").replace("~0", "~")
            if isinstance(current, dict):
                current = current.get(segment)
                path.append(segment)
            elif isinstance(current, list) and segment.isdigit() and int(segment) < len(current):
                current = current[int(segment)]
                path.append(int(segment))
            else:
                current = None
            if current is None:
                raise LookupError("refUnresolved " + reference)
        return owner, tuple(path), current

    def resolve(self, file, path, node):
        ui = {}
        chain = []
        for _ in range(32):
            if not isinstance(node, dict):
                return {"file": file, "path": path, "node": {}, "ui": ui, "chain": chain}
            chain.append((file, path))
            annotation = node.get("x-semio-ui")
            if isinstance(annotation, dict):
                for key, value in annotation.items():
                    ui.setdefault(key, value)
            if isinstance(node.get("$ref"), str):
                file, path, node = self.target(file, node["$ref"])
                continue
            union = node.get("oneOf") if isinstance(node.get("oneOf"), list) else node.get("anyOf") if isinstance(node.get("anyOf"), list) else None
            if union is not None:
                key = "oneOf" if isinstance(node.get("oneOf"), list) else "anyOf"
                concrete = [(at, branch) for at, branch in enumerate(union) if not (isinstance(branch, dict) and branch.get("type") == "null" and len(branch) == 1)]
                if len(concrete) == 1 and len(concrete) < len(union):
                    path = path + (key, concrete[0][0])
                    node = concrete[0][1]
                    continue
            return {"file": file, "path": path, "node": node, "ui": ui, "chain": chain}
        raise RuntimeError("ref chain too deep")

    def members(self, obj, found):
        found.append(obj)
        for at, member in enumerate(obj["node"].get("allOf", []) if isinstance(obj["node"].get("allOf"), list) else []):
            self.members(self.resolve(obj["file"], obj["path"] + ("allOf", at), member), found)
        return found

    def fields(self, obj, pointer, out, context):
        anchor = (obj["file"], tuple(obj["path"]))
        if anchor in self.stack:
            out.append({"recursive": True, "pointer": pointer, "key": None, "target": anchor})
            return
        self.stack.append(anchor)
        try:
            self.fields_of(obj, pointer, out, context)
        finally:
            self.stack.pop()

    def fields_of(self, obj, pointer, out, context):
        seen = set()
        for member in self.members(obj, []):
            properties = member["node"].get("properties")
            if not isinstance(properties, dict):
                continue
            required = member["node"].get("required", [])
            for key, prop in properties.items():
                if key in seen:
                    continue
                seen.add(key)
                site = (member["file"], member["path"] + ("properties", key))
                resolved = self.resolve(member["file"], site[1], prop)
                self.input(key, resolved, site, pointer + "/" + key.replace("~", "~0").replace("/", "~1"), out, context, key in required)

    def input(self, key, resolved, site, pointer, out, context, required):
        node = resolved["node"]
        ui = resolved["ui"]
        if ui.get("role") == "discriminator" or "const" in node:
            return
        kind = input_type(node)
        record = {
            "pointer": pointer,
            "key": key,
            "site": site,
            "target": (resolved["file"], resolved["path"]),
            "type": kind,
            "enum": node.get("enum"),
            "format": node.get("format"),
            "bounds": {name: node[name] for name in ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "minLength", "maxLength", "minItems", "maxItems", "pattern") if name in node},
            "ui": ui,
            "labelled": "label" in ui or key in self.labels,
            "glossary": key in self.labels,
            "context": context,
            "required": required,
            "description": node.get("description"),
            "title": node.get("title"),
        }
        out.append(record)
        if kind == "object" and ("properties" in node or "allOf" in node):
            self.fields(resolved, pointer, out, context)
        elif kind == "array" and "items" in node:
            items = self.resolve(resolved["file"], resolved["path"] + ("items",), node["items"])
            record["itemsType"] = input_type(items["node"])
            record["itemsEnum"] = items["node"].get("enum")
            record["itemsSite"] = (resolved["file"], resolved["path"] + ("items",))
            record["itemsUi"] = items["ui"]
            self.item(items, pointer + "/-", out, context)

    def item(self, items, pointer, out, context):
        node = items["node"]
        if "const" in node:
            return
        kind = input_type(node)
        if kind == "object" and ("properties" in node or "allOf" in node):
            self.fields(items, pointer, out, context)
        elif kind == "array" and "items" in node:
            self.item(self.resolve(items["file"], items["path"] + ("items",), node["items"]), pointer + "/-", out, context)

    def leaf(self, file):
        out = []
        root = self.document(file)
        payload = self.resolve(file, (), root)
        node = payload["node"]
        if "properties" not in node and "allOf" not in node:
            union_key = "oneOf" if isinstance(node.get("oneOf"), list) else "anyOf" if isinstance(node.get("anyOf"), list) else None
            if union_key is not None:
                for at, branch in enumerate(node[union_key]):
                    member = self.resolve(payload["file"], payload["path"] + (union_key, at), branch)
                    discriminators = []
                    for composed in self.members(member, []):
                        for key, prop in (composed["node"].get("properties") or {}).items():
                            resolved = self.resolve(composed["file"], composed["path"] + ("properties", key), prop)
                            if isinstance(resolved["node"].get("const"), str):
                                discriminators.append((key, resolved["node"]["const"], (composed["file"], composed["path"] + ("properties", key)), resolved["ui"]))
                    out.append({"variant": True, "branchSite": (payload["file"], payload["path"] + (union_key, at)), "memberUi": member["ui"], "discriminators": discriminators, "pointer": "", "key": None})
                    self.fields(member, "", out, "variant:%s" % ([value for _, value, _, _ in discriminators] or [at])[0])
            return out
        self.fields(payload, "", out, "root")
        return out


def main():
    walker = Walker(document_index(), glossary())
    only_missing = "--missing" in sys.argv
    for file in leaf_paths():
        try:
            records = walker.leaf(file)
        except LookupError as error:
            print(json.dumps({"leaf": file, "error": str(error)}, ensure_ascii=False))
            continue
        for record in records:
            if only_missing and (record.get("variant") or record.get("recursive") or record["labelled"]):
                continue
            print(json.dumps({"leaf": file, **record}, ensure_ascii=False, default=list))


if __name__ == "__main__":
    main()
