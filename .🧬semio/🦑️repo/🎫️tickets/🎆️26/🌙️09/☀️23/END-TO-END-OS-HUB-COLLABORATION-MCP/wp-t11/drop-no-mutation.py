"""⏸️ `no-mutation` is no runtime leaf (every `NoMutation` variant was dropped); it is the identity
baseline. Removes it from each owner's mutation manifest and catalog kinds, and turns its fixtures into
plain identity fixtures (no mutation/outcome claim). Prints every vector that still names it."""
import json, sys
for path in sys.argv[1:]:
    text = open(path, encoding="utf-8").read()
    data = json.loads(text)
    for manifest in data.get("mutationManifests", []):
        manifest["mutations"] = [m for m in manifest["mutations"] if m["id"] != "no-mutation"]
    for catalog in data.get("mutationCatalogs", []):
        catalog["kinds"] = [k for k in catalog.get("kinds", []) if k != "no-mutation"]
        for vector in catalog.get("vectors", []):
            if "no-mutation" in json.dumps(vector):
                print("  vector still names no-mutation:", catalog["id"], json.dumps(vector, ensure_ascii=False)[:160])
    for fixture in data.get("fixtureManifests", []):
        if fixture.get("mutation") == "no-mutation":
            fixture.pop("mutation"); fixture.pop("outcome", None)
    indent = 2 if '\n  "' in text[:300] else 1
    open(path, "w", encoding="utf-8").write(json.dumps(data, indent=indent, ensure_ascii=False) + "\n")
    print("done", path.split("🗿️artifacts/")[-1])
