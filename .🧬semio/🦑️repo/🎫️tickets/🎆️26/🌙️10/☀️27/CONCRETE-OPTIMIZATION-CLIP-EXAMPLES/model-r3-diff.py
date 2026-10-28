import json, sys, math
b = json.load(open("🗑️generated/model-r3/before.json", encoding="utf-8"))
a = json.load(open("🗑️generated/model-r3/after.json", encoding="utf-8"))
old_keys = set(next(iter(b["designs"].values())))
changed = {}
for k, v in b["designs"].items():
    n = a["designs"][k]
    diffs = {key: (v[key], n[key]) for key in old_keys if not math.isclose(v[key], n[key], rel_tol=1e-12, abs_tol=1e-12)}
    if diffs:
        changed[k] = diffs
print("designs", len(b["designs"]), "changed", len(changed))
thetas = sorted({float(k.split()[0]) for k in changed})
print("thetas changed:", thetas)
print("south (180) changed:", [k for k in changed if float(k.split()[0]) == 180.0])
print("d=0 changed:", [k for k in changed if float(k.split()[2]) == 0.0])
scene = set(a["scene"])
print("scene designs", len(scene), "changed in scene set:", [k for k in changed if k in scene])
print("sun equal:", a["sun"] == b["sun"], a["state"] == b["state"])
for k in list(changed)[:int(sys.argv[1]) if len(sys.argv) > 1 else 15]:
    print(k, {key: (round(x, 4), round(y, 4)) for key, (x, y) in changed[k].items() if key in ("shading_summer", "shading_winter", "cooling", "heating", "electricity")})
json.dump(changed, open("🗑️generated/model-r3/changed.json", "w", encoding="utf-8"), indent=1)
