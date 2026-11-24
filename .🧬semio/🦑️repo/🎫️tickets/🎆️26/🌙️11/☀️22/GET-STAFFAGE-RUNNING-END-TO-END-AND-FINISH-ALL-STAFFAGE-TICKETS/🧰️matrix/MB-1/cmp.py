"""🔬️ Short listing of the drawn shapes of views per band (id, kind and role initials, bone, fill, bounding box in mm), equal consecutive rows folded: cmp.py <id> <views e,f,p> <scales 50,100> [--max n] [--vol]"""
import json, re, sys, glob
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
args = sys.argv[1:]
mx = 40
if "--max" in args:
    i = args.index("--max"); mx = int(args[i + 1]); del args[i:i + 2]
vol = "--vol" in args
if vol: args.remove("--vol")
a = json.load(open(glob.glob(f"{CATALOG}/*/{args[0]}/🔣️.json")[0]))["asset"]
names = {"e": "elevation", "f": "front", "p": "plan"}
def bbox(g):
    k = g["kind"]
    if k == "rect": return (g["x"], g["y"], g["x"] + g["w"], g["y"] + g["h"])
    if k == "ellipse": return (g["cx"] - g["rx"], g["cy"] - g["ry"], g["cx"] + g["rx"], g["cy"] + g["ry"])
    if k == "capsule":
        r = max(g["r0"], g["r1"]); return (min(g["x0"], g["x1"]) - r, min(g["y0"], g["y1"]) - r, max(g["x0"], g["x1"]) + r, max(g["y0"], g["y1"]) + r)
    if k == "poly": p = g["points"]
    elif k == "path":
        n = [float(x) for x in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", g["d"])]; p = list(zip(n[0::2], n[1::2]))
    elif k == "member": p = [g["from"], g["to"]] if isinstance(g.get("from"), list) else None
    else: p = None
    if not p: return None
    p = [(q["x"], q["y"]) if isinstance(q, dict) else q for q in p]
    return (min(q[0] for q in p), min(q[1] for q in p), max(q[0] for q in p), max(q[1] for q in p))
print(a["id"], a["dimensions"], "default", a["presentation"]["defaultPose"], "poses", [p["id"] for p in a["poses"]])
print("bones", " ".join(f"{b['id']}<{b.get('parent')}@{b.get('x',0)},{b.get('y',0)},{b.get('z',0)}/{b.get('angleDeg',0)}" for b in a["rig"]["bones"])[:900])
for s in args[2].split(","):
    r = a["representations"][s]
    if r.get("status") != "authored": print(s, r.get("status")); continue
    print(f"== 1:{s} range {r.get('validRange')} sections {json.dumps(r.get('sections'))[:120] if r.get('sections') else None}")
    for v in args[1].split(","):
        rows = []
        for g in r["geometry"][names[v]]:
            b = bbox(g)
            rows.append(f"{g['id']} {g['kind'][:2]}/{g['role'][:2]} {g['bone']} {'F' if g.get('filled') else 'o'}{'S' if 'solid' in g else ''} z{g.get('z')} " + (" ".join(f"{x*1000:.0f}" for x in b) if b else "?"))
        print(f" {v}: {len(rows)} shapes")
        for row in rows[:mx]: print("   " + row)
    if vol:
        for b in a["volumes"][s]["body"]:
            print("   V " + json.dumps({k: x for k, x in b.items() if k not in ("material", "role", "tone", "tags")})[:230])
