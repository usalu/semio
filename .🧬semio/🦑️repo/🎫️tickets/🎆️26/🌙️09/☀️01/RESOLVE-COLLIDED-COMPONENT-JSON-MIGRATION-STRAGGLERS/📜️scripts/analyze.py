import json, subprocess, sys, os

SEMIO = "/Users/ueli/Documents/semio"
os.chdir(SEMIO)

with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/component_dirs.txt") as f:
    dirs = [l.strip()[2:] for l in f if l.strip()]  # strip leading "./"

APPROVED_VERBS = {"add","append","apply","bind","change","clear","commit","connect","create","delete",
"disconnect","drag","duplicate","edit","extract","finish","fix","flatten","group","inline","insert",
"merge","move","remove","rename","reorder","replace","resize","restore","rotate","scale","seal","set",
"split","start","switch","toggle","unbind","unflatten","ungroup","update"}

def kebab(s):
    out = []
    for i, c in enumerate(s):
        if c.isupper() and i > 0:
            out.append('-')
        out.append(c.lower())
    return ''.join(out)

results = []
anomalies = []

for d in dirs:
    jpath = os.path.join(d, "🔣️.json")
    cpath = os.path.join(d, "🔣️component.json")
    if not os.path.exists(jpath) or not os.path.exists(cpath):
        anomalies.append({"dir": d, "issue": "missing one of the two files now"})
        continue
    try:
        j = json.load(open(jpath, encoding="utf-8"))
        c = json.load(open(cpath, encoding="utf-8"))
    except Exception as e:
        anomalies.append({"dir": d, "issue": f"json parse error: {e}"})
        continue

    j_surfaces = len(j.get("requiredLanguageSurfaces", []))
    c_surfaces = len(c.get("requiredLanguageSurfaces", []))
    j_sparse = j.get("binaryTag") is None and j.get("textOpcode") is None
    c_sparse = c.get("binaryTag") is None and c.get("textOpcode") is None

    # heuristic: component.json wins unless it's the sparse one
    if c_sparse and not j_sparse:
        winner = "json"
        flag = "REVERSED"
    elif c_surfaces < j_surfaces and c_sparse:
        winner = "json"
        flag = "REVERSED"
    else:
        winner = "component"
        flag = None

    entry = {
        "dir": d,
        "winner": winner,
        "j_surfaces": j_surfaces, "c_surfaces": c_surfaces,
        "j_sparse": j_sparse, "c_sparse": c_sparse,
        "j_keys": sorted(j.keys()), "c_keys": sorted(c.keys()),
    }
    if flag:
        entry["flag"] = flag
        anomalies.append(entry)
    results.append(entry)

print(f"Total dirs: {len(dirs)}")
print(f"Processed: {len(results)}")
print(f"Anomalies/reversed: {len([r for r in results if r.get('flag')])}")
print(f"Missing-file or parse-error issues: {len([a for a in anomalies if 'issue' in a])}")

# key-set uniformity check
keysets_j = set(tuple(r["j_keys"]) for r in results)
keysets_c = set(tuple(r["c_keys"]) for r in results)
print(f"Distinct .json key-sets: {len(keysets_j)}")
print(f"Distinct component.json key-sets: {len(keysets_c)}")
for ks in keysets_j:
    print("  .json keyset:", ks)
for ks in keysets_c:
    print("  component.json keyset:", ks)

with open("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RESOLVE-COLLIDED-COMPONENT-JSON-MIGRATION-STRAGGLERS/🗑️generated/analysis.json", "w", encoding="utf-8") as f:
    json.dump({"results": results, "anomalies": anomalies}, f, ensure_ascii=False, indent=2)
