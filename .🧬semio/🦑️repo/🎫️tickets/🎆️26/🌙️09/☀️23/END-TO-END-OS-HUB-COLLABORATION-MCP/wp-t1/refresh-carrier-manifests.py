"""🔄️ Copies regenerated `files` digests and `generator` provenance from a generator's manifest output into the owner registry, by fixture id."""
import json, sys
registry_path, produced_path = sys.argv[1], sys.argv[2]
registry = json.load(open(registry_path, encoding="utf-8"))
produced = {entry["id"]: entry for entry in json.load(open(produced_path, encoding="utf-8"))}
seen = set()
for entry in registry.get("fixtureManifests", []):
    fresh = produced.get(entry["id"])
    if fresh is None: continue
    assert [f["path"] for f in entry["files"]] == [f["path"] for f in fresh["files"]], entry["id"]
    entry["files"], entry["generator"] = fresh["files"], fresh["generator"]
    seen.add(entry["id"])
missing = sorted(set(produced) - seen)
assert not missing, f"generator produced ids the registry does not hold: {missing}"
open(registry_path, "w", encoding="utf-8").write(json.dumps(registry, ensure_ascii=False, indent=2) + "\n")
print(f"{len(seen)} manifest(s) refreshed")
