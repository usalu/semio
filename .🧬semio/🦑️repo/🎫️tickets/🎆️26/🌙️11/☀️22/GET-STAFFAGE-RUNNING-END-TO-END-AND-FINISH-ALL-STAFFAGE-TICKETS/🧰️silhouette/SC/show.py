"""🔎️ Compact listing of the drawn shapes (id, kind, role, bounding box) and the body solids (id, kind, bone, key numbers) of a component: show.py <id> <scale> [view|body]… [--grep text]"""
import json, re, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components"
args = sys.argv[1:]
grep = None
if "--grep" in args:
    i = args.index("--grep"); grep = args[i + 1]; del args[i:i + 2]
a = json.load(open(args[0] if args[0].endswith(".json") else f"{CAT}/{args[0]}/🔣️.json"))["asset"]
scale = args[1]
what = args[2:] or ["elevation", "front", "plan", "body"]
def bbox(g):
    k = g["kind"]
    if k == "rect": return (g["x"], g["y"], g["x"] + g["w"], g["y"] + g["h"])
    if k in ("circle",): return (g["cx"] - g["r"], g["cy"] - g["r"], g["cx"] + g["r"], g["cy"] + g["r"])
    if k == "ellipse": return (g["cx"] - g["rx"], g["cy"] - g["ry"], g["cx"] + g["rx"], g["cy"] + g["ry"])
    if k == "poly": p = g["points"]
    elif k == "line": p = [[g["x1"], g["y1"]], [g["x2"], g["y2"]]]
    elif k == "path":
        n = [float(x) for x in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", g["d"])]; p = list(zip(n[0::2], n[1::2]))
    else: return None
    return (min(q[0] for q in p), min(q[1] for q in p), max(q[0] for q in p), max(q[1] for q in p))
r = a["representations"][scale]
print(a["id"], "dims", a["dimensions"], "sections", r.get("sections"), "pens", r.get("pens"))
print("bones", [(b["id"], b.get("parent"), b.get("origin") or b.get("head") or "") for b in a["rig"]["bones"]][:14])
for v in what:
    if v == "body":
        for b in a["volumes"][scale]["body"]:
            s = json.dumps({k: x for k, x in b.items() if k not in ("material", "role", "tone", "tags")})
            if grep is None or grep in s: print(" V", s[:int(__import__('os').environ.get('L', 300))])
        continue
    for g in r["geometry"][v]:
        s = f" {v[0].upper()} {g['id']} {g['kind']} {g['role']} bone={g.get('bone')} filled={g.get('filled')} closed={g.get('closed')} z={g.get('z')} pen={g.get('penMm')} " + (("bbox " + " ".join(f"{x:.4f}" for x in bbox(g))) if bbox(g) else "") + (f" tf={json.dumps(g['transform'])}" if "transform" in g else "")
        if grep is None or grep in s: print(s)
