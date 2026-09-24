"""🪆️ One owner per mutation (T5 §6, the gif 89a pattern): the subset whose directory holds the leaves
dispatches them, so its manifest declares every such row with the schema's `subset` override naming the
sibling that semantically owns it; the sibling's own mutationManifests are removed. Sibling catalogs,
cases and fixtures stay where they are. Variant spellings are reduced to the aggregate variant the
runtime reports, and payloadSchema paths are re-rooted to the base contribution.

Usage: adopt-sibling-rows.py <subsets-dir> <base-subset-dir> <sibling-subset-dir>..."""
import json, os, sys
root, base, siblings = sys.argv[1], sys.argv[2], sys.argv[3:]

def load(subset):
    path = os.path.join(root, subset, "🔮️oracles", "🔣️.json")
    text = open(path, encoding="utf-8").read()
    return path, text, json.loads(text)

def save(path, text, data):
    indent = 2 if '\n  "' in text[:300] else 1
    open(path, "w", encoding="utf-8").write(json.dumps(data, indent=indent, ensure_ascii=False) + "\n")

base_path, base_text, base_data = load(base)
(base_manifest,) = base_data["mutationManifests"]
owned = {m["id"] for m in base_manifest["mutations"]}
for sibling in siblings:
    path, text, data = load(sibling)
    manifests = data.pop("mutationManifests", [])
    for manifest in manifests:
        for row in manifest["mutations"]:
            assert row["id"] not in owned, row["id"]
            moved = {"id": row["id"], "capability": row["capability"], "subset": row.get("subset", manifest["subset"])}
            moved.update({k: v for k, v in row.items() if k not in ("id", "capability", "subset")})
            moved["payloadSchema"] = moved["payloadSchema"].replace(f"../../{base}/", "../")
            variant = moved["productionDispatch"].get("variant")
            if variant is not None:
                moved["productionDispatch"]["variant"] = variant.split("::")[-1]
            base_manifest["mutations"].append(moved)
            owned.add(row["id"])
            print(f"  {sibling} -> {base}: {row['id']} ({moved['payloadSchema']})")
    save(path, text, data)
save(base_path, base_text, base_data)
