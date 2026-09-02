#!/usr/bin/env python3
"""🏁️ Finish the glTF oracle-manifest side of shard A6's subset split:
1. Give every fixtureManifest entry its real (non-wildcard) target.subset, mirroring the mutation
   overrides already applied, and repair its file paths now that the fixture bytes themselves moved
   to the new subset's own 🧫️fixtures/.
2. Replace the single stale `gltf-2-0-any` mutationCatalog (which claimed all 120 vectors under the
   now-empty ✳️any vocabulary) with eight new minimal oracle contributions, one physically inside
   each new domain subset (`✳️<domain>/🧪️oracle/🔣️.json`), each declaring just that domain's own
   mutationCatalog -- this is what `unregistered-mutation-vocabulary` requires (a contribution whose
   OWNER is the subset directory itself). mutationManifests/fixtureManifests deliberately stay
   centralized at ✳️any (see the shard report for why splitting those would multiply
   `runtime-inventory-missing`, a pre-existing, out-of-scope breach, 1 -> 8)."""
import json

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ANY_ORACLE = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
STANDARDS_ROOT = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets"

with open(f"{TICKET}/🗑️generated/a6-gltf-subset-mapping.json", encoding="utf-8") as f:
    mapping = json.load(f)
by_ascii = {v["ascii"]: v["subset"] for v in mapping.values()}

with open(ANY_ORACLE, encoding="utf-8") as f:
    data = json.load(f)

# --- 1. fixtureManifests: real target.subset + repaired file paths ---
fixed_fixtures = 0
for fixture in data["fixtureManifests"]:
    mutation_id = fixture["mutation"]
    subset = by_ascii[mutation_id]
    fixture["target"]["subset"] = subset
    for file in fixture["files"]:
        old_prefix = "../🧫️fixtures/"
        assert file["path"].startswith(old_prefix), file["path"]
        file["path"] = "../../✳️" + subset + "/🧫️fixtures/" + file["path"][len(old_prefix):]
    fixed_fixtures += 1
assert fixed_fixtures == 120, fixed_fixtures

# --- 2. mutationCatalogs: drop the stale combined one, keep the array empty here ---
old_catalog = data["mutationCatalogs"][0]
assert old_catalog["subsetDirectoryName"] == "✳️any"
old_vectors = old_catalog["vectors"]
old_kinds = old_catalog["kinds"]
data["mutationCatalogs"] = []

with open(ANY_ORACLE, "w", encoding="utf-8") as f:
    json.dump(data, f, ensure_ascii=False, indent=2)
    f.write("\n")
print("updated", ANY_ORACLE)

# --- 3. eight new minimal per-subset oracle contributions ---
vectors_by_subset = {s: [] for s in set(by_ascii.values())}
for vec in old_vectors:
    subset = by_ascii[vec["mutationId"]]
    vectors_by_subset[subset].append(vec)

kinds_by_subset = {s: [] for s in set(by_ascii.values())}
for kind in old_kinds:
    kinds_by_subset[by_ascii[kind]].append(kind)

SCHEMA_REF = data["$schema"]
counts = {}
for subset, vectors in vectors_by_subset.items():
    oracle_dir = f"{STANDARDS_ROOT}/✳️{subset}/🧪️oracle"
    import os
    os.makedirs(oracle_dir, exist_ok=True)
    contribution = {
        "$schema": SCHEMA_REF,
        "schemaVersion": 2,
        "_comment": (
            f"🧩️ This subset's own mutation-catalog contribution to the repository test platform "
            f"-- registers the {subset} slice of the glTF 2.0 mutation vocabulary (moved here from "
            f"the artifact-level ✳️any catalog by shard A6, ticket "
            f"26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION). "
            f"The shared whole-document io/oracle/generator machinery, and every mutation's own "
            f"manifest/fixture record, stay centralized at the artifact's ✳️any substrate; this file "
            f"exists so the {subset} vocabulary directory has a claiming catalog of its own, per "
            f"`unregistered-mutation-vocabulary`."
        ),
        "mutationCatalogs": [
            {
                "id": f"gltf-2-0-{subset}",
                "capability": "gltf-2-0-mutate",
                "standardDirectoryName": "🔖️2.0",
                "subsetDirectoryName": f"✳️{subset}",
                "vectors": vectors,
                "kinds": kinds_by_subset[subset],
            }
        ],
    }
    path = f"{oracle_dir}/🔣️.json"
    with open(path, "w", encoding="utf-8") as f:
        json.dump(contribution, f, ensure_ascii=False, indent=2)
        f.write("\n")
    counts[subset] = len(vectors)
    print("wrote", path, "vectors:", len(vectors), "kinds:", len(kinds_by_subset[subset]))

print(counts, sum(counts.values()))
