import json, collections

def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)

def save(path, data):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)
        f.write("\n")

report_rows = []  # (artifact, standard, subset, catalog_id, capability, kinds)

def rescope_simple(path, artifact_slug, standard):
    d = load(path)
    changed = False
    for c in d.get("mutationCatalogs", []):
        subset = c["subsetDirectoryName"].lstrip("✳️")
        new_cap = f"{c['id']}-mutate"
        if c.get("capability") != new_cap:
            c["capability"] = new_cap
            changed = True
    for m in d.get("mutationManifests", []):
        subset = m["subset"]
        # find sibling catalog capability for this subset (matching subsetDirectoryName)
        cat = next((c for c in d.get("mutationCatalogs", []) if c["subsetDirectoryName"].lstrip("✳️") == subset), None)
        new_cap = f"{cat['id']}-mutate" if cat else None
        for mu in m["mutations"]:
            if new_cap and mu.get("capability") != new_cap:
                mu["capability"] = new_cap
                changed = True
    if changed:
        save(path, d)
    for c in d.get("mutationCatalogs", []):
        report_rows.append((artifact_slug, standard, c["subsetDirectoryName"].lstrip("✳️"), c["id"], c["capability"], c["kinds"]))
    return changed

# --- note ---
import glob
for f in sorted(glob.glob("✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/*/🪆️subsets/*/🧪️oracle/🔣️.json")):
    d = load(f)
    if d.get("mutationCatalogs"):
        rescope_simple(f, "note", "1")

# --- mathematical ---
for f in sorted(glob.glob("✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/*/🪆️subsets/*/🧪️oracle/🔣️.json")):
    d = load(f)
    if d.get("mutationCatalogs"):
        rescope_simple(f, "mathematical", "1")

# --- sequence ---
for f in sorted(glob.glob("✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/*/🪆️subsets/*/🧪️oracle/🔣️.json")):
    d = load(f)
    if d.get("mutationCatalogs"):
        rescope_simple(f, "sequence", "1")

print("done simple artifacts")
for r in report_rows:
    print(r)
