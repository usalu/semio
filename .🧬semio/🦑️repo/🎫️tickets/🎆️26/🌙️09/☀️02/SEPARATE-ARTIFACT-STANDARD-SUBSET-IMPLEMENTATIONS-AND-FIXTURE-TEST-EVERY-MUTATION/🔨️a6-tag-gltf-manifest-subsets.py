#!/usr/bin/env python3
"""🏷️ Give every glTF 2.0 mutation manifest entry its real (non-wildcard) subset override, now
that its schema/tests/fixtures physically live under that subset. The oracle contribution itself
stays mounted at ✳️any (its io/oracle/generator machinery is whole-document, not mutation-scoped),
so only `mutations[].subset` changes -- `owningSubsetOf` reads that override ahead of the
manifest's own `subset` field."""
import json

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ORACLE = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json"

with open(f"{TICKET}/🗑️generated/a6-gltf-subset-mapping.json", encoding="utf-8") as f:
    mapping = json.load(f)
by_ascii = {v["ascii"]: v["subset"] for v in mapping.values()}

with open(ORACLE, encoding="utf-8") as f:
    text = f.read()
data = json.loads(text)

mm = data["mutationManifests"]
assert len(mm) == 1, len(mm)
manifest = mm[0]
tagged = 0
for mutation in manifest["mutations"]:
    subset = by_ascii[mutation["id"]]
    # insert "subset" right after "id" for readability
    new_entry = {}
    for k, v in mutation.items():
        new_entry[k] = v
        if k == "id":
            new_entry["subset"] = subset
    mutation.clear()
    mutation.update(new_entry)
    tagged += 1

assert tagged == 120, tagged

with open(ORACLE, "w", encoding="utf-8") as f:
    json.dump(data, f, ensure_ascii=False, indent=2)
    f.write("\n")

print("tagged", tagged, "mutations with explicit subset overrides")
