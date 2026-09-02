#!/usr/bin/env python3
"""🩹️ Second repair pass on the glTF oracle catalogs:
1. Restore a `gltf-2-0-any` catalog inside ✳️any's own contribution, scoped to ✳️any, so the
   pre-existing artifact-root feature (`🧪️tests/mutate-gltf-2-0/🥒️.feature`, tagged
   `@mutations-gltf-2-0-any`, exercising 7 kinds) stays claimed exactly as before. Its `vectors` is
   now empty -- none of those mutation directories still live under ✳️any.
2. Give each of the 8 new per-subset catalogs a non-empty, honest `kinds` (= all of that subset's own
   vectors), so the contribution is well-formed. None of these are claimed by an existing feature
   (that would need real new Cucumber scenarios per subset -- out of this shard's scope, and shared
   with 15+ other artifacts already carrying the same honest gap) -- see the shard report."""
import json, os

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ANY_ORACLE = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
STANDARDS_ROOT = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets"

with open(f"{TICKET}/🗑️generated/a6-gltf-subset-mapping.json", encoding="utf-8") as f:
    mapping = json.load(f)
domains = sorted(set(v["subset"] for v in mapping.values()))

ORIGINAL_KINDS = [
    "bind-node-child", "bind-scene-root-node", "change-material-alpha-mode",
    "change-material-double-sided", "create-scene", "unbind-node-child", "unbind-scene-root-node",
]

with open(ANY_ORACLE, encoding="utf-8") as f:
    data = json.load(f)

assert data["mutationCatalogs"] == []
data["mutationCatalogs"] = [
    {
        "id": "gltf-2-0-any",
        "capability": "gltf-2-0-mutate",
        "standardDirectoryName": "🔖️2.0",
        "subsetDirectoryName": "✳️any",
        "vectors": [],
        "kinds": ORIGINAL_KINDS,
    }
]
with open(ANY_ORACLE, "w", encoding="utf-8") as f:
    json.dump(data, f, ensure_ascii=False, indent=2)
    f.write("\n")
print("restored gltf-2-0-any catalog at", ANY_ORACLE)

for subset in domains:
    path = f"{STANDARDS_ROOT}/✳️{subset}/🧪️oracle/🔣️.json"
    with open(path, encoding="utf-8") as f:
        contribution = json.load(f)
    catalog = contribution["mutationCatalogs"][0]
    kinds = sorted({vec["mutationId"] for vec in catalog["vectors"]})
    assert len(kinds) > 0
    catalog["kinds"] = kinds
    with open(path, "w", encoding="utf-8") as f:
        json.dump(contribution, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print("repaired", path, "kinds:", len(kinds))
