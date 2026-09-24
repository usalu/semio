"""🧊️ Sets each gltf manifest row's productionDispatch.variant to the aggregate variant the runtime inventory measured (breach evidence), editing only that field."""
import json, re, sys
rows = json.load(open(sys.argv[1]))
path = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔮️oracles/🔣️.json"
measured = {}
for row in rows:
    if row["id"] == "mutation-variant-mismatch" and row["scope"] == path:
        kind, old, new = re.match(r".*: mutation (\S+) names dispatch variant (\S+) but the runtime reports (\S+)$", row["summary"]).groups()
        measured[kind] = (old, new)
text = open(path, encoding="utf-8").read()
data = json.loads(text)
changed = 0
for manifest in data["mutationManifests"]:
    for mutation in manifest["mutations"]:
        if mutation["id"] in measured:
            old, new = measured[mutation["id"]]
            assert mutation["productionDispatch"]["variant"] == old, mutation["id"]
            mutation["productionDispatch"]["variant"] = new
            changed += 1
indent = 2 if '\n  "' in text[:200] else 1
open(path, "w", encoding="utf-8").write(json.dumps(data, indent=indent, ensure_ascii=False) + "\n")
print(changed, "of", len(measured))
