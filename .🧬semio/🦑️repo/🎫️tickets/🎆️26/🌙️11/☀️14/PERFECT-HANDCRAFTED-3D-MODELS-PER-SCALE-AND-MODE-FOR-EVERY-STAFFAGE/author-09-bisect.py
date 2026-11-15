import sys, json, copy
sys.path.insert(0, ".")
from author_09_lib import *
slug, scale = sys.argv[1], sys.argv[2]
d = json.load(open(f"{T}/volumes/humans/{slug}.json"))
st = d["volumes"][scale]["structure"]
groups = {}
for s in st:
    groups.setdefault(s["id"].split("-")[0] + ":" + s.get("part", ""), []).append(s["id"])
groups["all-members-arm"] = [s["id"] for s in st if "arm" in s["id"] or "hand" in s["id"]]
for g, ids in groups.items():
    e = copy.deepcopy(d)
    e["volumes"][scale]["structure"] = [s for s in st if s["id"] not in ids] or st[:1]
    p = f"{T}/🗑️generated/author09-bisect.json"
    json.dump(e, open(p, "w"))
    out = run(["volumes-check.ts", f"humans/{slug}", "--scale", scale, "--rules", "volumes-structure-coverage", "--source", p, "--json"])
    if "{\n" not in out:
        print(g, "ERR", out[:200]); continue
    i = out.index("{\n")
    ms = json.JSONDecoder().raw_decode(out[i:])[0]["measurements"]
    v = [m["value"] for m in ms if m["name"] == "outsideBodyM:front"]
    print(g, ids[:3], v)
