import json, collections, importlib
from pathlib import Path
root = Path("🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
used = collections.defaultdict(set)
for e in S._manifest()["casts"].values(): used[e["asset"]].add((e["scale"], e["viewpoint"]))
c = collections.Counter(); sym = 0; n = 0; rows = []; roles = collections.Counter()
for f in sorted(root.glob("*/*/🔣️.json")):
    a = json.loads(f.read_text(encoding="utf-8"))["asset"]; n += 1
    reps = a["representations"]; au = sorted(int(k) for k, v in reps.items() if v.get("status") == "authored")
    for k in au: c[k] += 1
    sy = a.get("symbols") or []
    sym += bool(sy)
    if a["id"] in used:
        r50 = reps.get("50", {})
        geo = {g: len(r50.get("geometry", {}).get(g, [])) for g in ("elevation", "front", "plan")}
        for g in r50.get("geometry", {}).get("elevation", []): roles[g.get("role")] += 1
        rows.append((a["id"], au, sorted(used[a["id"]]), len(sy), geo, len(a.get("clips", [])), len(a.get("poses", []))))
print(n, dict(c), "assets with symbols", sym, "assets used by casts", len(used), "roles(elev@50 of used)", dict(roles))
for r in rows[:70]: print(r)
