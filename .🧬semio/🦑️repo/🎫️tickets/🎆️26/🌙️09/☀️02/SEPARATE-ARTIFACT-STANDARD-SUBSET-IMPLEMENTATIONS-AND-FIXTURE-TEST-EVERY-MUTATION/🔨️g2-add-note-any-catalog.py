#!/usr/bin/env python3
"""🔨️ Registers the new note-1-any-document catalog at ✳️any/🧪️oracle/🔣️.json, reusing the
already-manifested note-1-document-mutate capability. Same mechanism as F4's
🔨️f4-add-drawing-any-catalogs.py."""
import json

ROOT = "/Users/ueli/Documents/semio"
ORACLE_JSON = f"{ROOT}/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json"

with open(ORACLE_JSON) as f:
    d = json.load(f)

new_catalog = {
    "id": "note-1-any-document",
    "capability": "note-1-document-mutate",
    "standardDirectoryName": "🔖️1",
    "subsetDirectoryName": "✳️any",
    "kinds": ["rename-note"],
    "vectors": [],
}

existing_ids = {c["id"] for c in d["mutationCatalogs"]}
assert new_catalog["id"] not in existing_ids
d["mutationCatalogs"].append(new_catalog)

with open(ORACLE_JSON, "w") as f:
    json.dump(d, f, indent=2, ensure_ascii=False)
    f.write("\n")

print("registered", new_catalog["id"])
