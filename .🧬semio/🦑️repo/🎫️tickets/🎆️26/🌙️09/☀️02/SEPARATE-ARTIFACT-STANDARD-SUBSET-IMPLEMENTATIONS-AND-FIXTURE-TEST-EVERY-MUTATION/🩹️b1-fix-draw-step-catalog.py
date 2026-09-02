import json

def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)

def save(path, data):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)
        f.write("\n")

# --- draw: empty the leftover ✳️any wildcard catalog ---
any_path = "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
d = load(any_path)
assert d["mutationManifests"] == [], "✳️any unexpectedly owns manifests"
before_kinds = d["mutationCatalogs"][0]["kinds"] if d["mutationCatalogs"] else []
d["mutationCatalogs"] = []
save(any_path, d)
print("draw ✳️any: cleared leftover catalog, was", len(before_kinds), "kinds")

# --- draw: give each real subset its own catalog id + capability ---
report = []
for subset in ["metadata", "structure", "style", "transform"]:
    path = f"✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️{subset}/🧪️oracle/🔣️.json"
    d = load(path)
    new_id = f"draw-1-{subset}"
    new_cap = f"{new_id}-mutate"
    for c in d["mutationCatalogs"]:
        c["id"] = new_id
        c["capability"] = new_cap
    for m in d["mutationManifests"]:
        for mu in m["mutations"]:
            mu["capability"] = new_cap
    save(path, d)
    kinds = d["mutationCatalogs"][0]["kinds"]
    report.append(("draw", "1", subset, new_id, new_cap, kinds))

for r in report:
    print(r)

# --- step cc6: drop the test-only "no-mutation" sentinel from the catalog vocabulary ---
cc6_path = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🧪️oracle/🔣️.json"
d = load(cc6_path)
cat = d["mutationCatalogs"][0]
assert cat["id"] == "step-ap214-cc6"
before = list(cat["kinds"])
cat["kinds"] = [k for k in cat["kinds"] if k != "no-mutation"]
assert cat["kinds"] != before
save(cc6_path, d)
print("step cc6: kinds", before, "->", cat["kinds"])
