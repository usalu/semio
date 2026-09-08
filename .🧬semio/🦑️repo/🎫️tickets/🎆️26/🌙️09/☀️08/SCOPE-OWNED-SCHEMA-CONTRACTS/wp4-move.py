#!/usr/bin/env python3
"""🧬 WP4 helper — migrate a fixture-owned JSON Schema into an owner module `$defs` export.

Usage:
  wp4-move.py inspect <schema.json>
  wp4-move.py move <schema.json> <module-dir> <ExportId> [--module-id ID] [--title TITLE] [--keep]

`move` reads the fixture schema, converts it to draft-07, strips its `$schema`/`$id`, and writes it
as `$defs.<ExportId>` of `<module-dir>/🔣️.json` (creating the module file when absent). The source
file is deleted unless `--keep` is given.
"""
import json
import os
import sys
from collections import OrderedDict

DRAFT07 = "http://json-schema.org/draft-07/schema#"


def to_draft07(node):
    """♻️ 2020-12 → draft-07: tuple validation, and drop keywords draft-07 does not know."""
    if isinstance(node, list):
        return [to_draft07(item) for item in node]
    if not isinstance(node, dict):
        return node
    out = OrderedDict()
    prefix = node.get("prefixItems")
    for key, value in node.items():
        if key == "prefixItems":
            out["items"] = [to_draft07(item) for item in value]
            continue
        if key == "items" and prefix is not None:
            out["additionalItems"] = False if value is False else to_draft07(value)
            continue
        if key in ("unevaluatedProperties", "unevaluatedItems", "$anchor", "$dynamicRef", "$dynamicAnchor"):
            continue
        if key == "$schema":
            continue
        out[key] = to_draft07(value)
    return out


def strip_self_schema(node):
    """🧹 Remove the `$schema` self-reference property that only pointed at the deleted sibling."""
    if not isinstance(node, dict):
        return node
    props = node.get("properties")
    if isinstance(props, dict) and "$schema" in props:
        const = props["$schema"].get("const") if isinstance(props["$schema"], dict) else None
        if isinstance(const, str) and const.startswith("./"):
            del props["$schema"]
            if isinstance(node.get("required"), list):
                node["required"] = [name for name in node["required"] if name != "$schema"]
    return node


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle, object_pairs_hook=OrderedDict)


def dump(path, value):
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def main():
    action = sys.argv[1]
    if action == "inspect":
        doc = load(sys.argv[2])
        print(json.dumps({"$schema": doc.get("$schema"), "$id": doc.get("$id"), "title": doc.get("title"),
                          "keys": list(doc.keys()), "$defs": list(doc.get("$defs", {}).keys())},
                         ensure_ascii=False, indent=2))
        return
    if action != "move":
        raise SystemExit("unknown action")
    source, module_dir, export = sys.argv[2], sys.argv[3], sys.argv[4]
    rest = sys.argv[5:]
    module_id = rest[rest.index("--module-id") + 1] if "--module-id" in rest else None
    title = rest[rest.index("--title") + 1] if "--title" in rest else None
    keep = "--keep" in rest

    doc = strip_self_schema(to_draft07(load(source)))
    nested = doc.pop("$defs", None)
    doc.pop("$id", None)
    if title:
        doc["title"] = title
    elif "title" not in doc:
        doc["title"] = export

    module_path = os.path.join(module_dir, "🔣️.json")
    if os.path.exists(module_path):
        module = load(module_path)
    else:
        os.makedirs(module_dir, exist_ok=True)
        if not module_id:
            raise SystemExit("--module-id is required to create a new module")
        module = OrderedDict([("$schema", DRAFT07), ("$id", module_id), ("$defs", OrderedDict())])
    module.setdefault("$schema", DRAFT07)
    module["$schema"] = DRAFT07
    if module_id:
        module["$id"] = module_id
    defs = module.setdefault("$defs", OrderedDict())
    if nested:
        for name, value in nested.items():
            key = name if name[:1].isupper() else export + name[:1].upper() + name[1:]
            defs[key] = value
            doc = json.loads(json.dumps(doc).replace('#/$defs/%s"' % name, '#/$defs/%s"' % key))
    defs[export] = doc
    # keep $defs last
    module["$defs"] = defs
    ordered = OrderedDict((k, v) for k, v in module.items() if k != "$defs")
    ordered["$defs"] = defs
    dump(module_path, ordered)
    if not keep:
        os.remove(source)
    print("moved %s -> %s#/$defs/%s" % (source, module_path, export))


if __name__ == "__main__":
    main()
