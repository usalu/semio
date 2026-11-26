"""🖐️ Trial of the asset-side recipe for the hands of `person-with-guide-dog` at 1:50: the elevation draws the palm on `hand-<s>` (the plate clipped at the length of the palm in front and plan) and every finger on its own bone as the stadium of its volume capsule, as the front and the plan already draw them: guide-hands.py <catalog json> <out json> [<clip u, default 0.0998>]"""
import copy, json, math, sys
src, out = sys.argv[1], sys.argv[2]
clip = float(sys.argv[3]) if len(sys.argv) > 3 else 0.0998
only = sys.argv[4] if len(sys.argv) > 4 else 'all'
doc = json.load(open(src))
asset = doc["asset"]
geo = asset["representations"]["50"]["geometry"]
body = {s["id"]: s for s in asset["volumes"]["50"]["body"]}

def clipx(points, x0):
    out = []
    for i, p in enumerate(points):
        q = points[(i + 1) % len(points)]
        pin, qin = p[0] <= x0, q[0] <= x0
        if pin: out.append(p)
        if pin != qin:
            t = (x0 - p[0]) / (q[0] - p[0])
            out.append([x0, p[1] + t * (q[1] - p[1])])
    return out

def stadium(length, r0, r1, n=8):
    pts = []
    for k in range(n + 1): a = math.pi - math.pi / 2 * k / n; pts.append([r0 * math.cos(a), r0 * math.sin(a)])
    for k in range(n + 1): a = math.pi / 2 - math.pi / 2 * k / n; pts.append([length + r1 * math.cos(a), r1 * math.sin(a)])
    return [[round(x, 5), round(y, 5)] for x, y in pts]

new = []
template = next(s for s in geo["front"] if s["id"] == "hand-l-finger-1")
for s in geo["elevation"]:
    if s["id"] in ("hand-l", "hand-r"):
        s = copy.deepcopy(s); s["points"] = [[round(a, 5), round(b, 5)] for a, b in clipx(s["points"], clip)]
    new.append(s)
for side in ("l", "r"):
    for f in ("finger-%d%s" % (i, suf) for i in (1, 2, 3, 4) for suf in ("", "-mid", "-tip") if only != "no%d" % i and not only.startswith("keep") or only == "keep%d" % i) if only != "thumbs" else []:
        sid = f"hand-{side}-{f}"
        v = body.get(sid)
        shape = next((s for s in geo["front"] if s["id"] == sid), None)
        if v is None or shape is None: continue
        d = {k: x for k, x in template.items() if k not in ("points", "solid")}; d["solid"] = {"kind": "revolved", "axisBone": shape["bone"], "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
        d.update(id=sid, bone=shape["bone"], z=1.5)
        d["points"] = stadium(v["p1"][0], v["r0"], v["r1"])
        new.append(d)
    for f in ("thumb", "thumb-mid", "thumb-tip") if only != "fingers" else []:
        sid = f"hand-{side}-{f}"
        v = body.get(sid); shape = next((s for s in geo["front"] if s["id"] == sid), None)
        if v is None or shape is None: continue
        d = {k: x for k, x in template.items() if k not in ("points", "solid")}; d["solid"] = {"kind": "revolved", "axisBone": shape["bone"], "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
        d.update(id=sid, bone=shape["bone"], z=1.5, points=stadium(v["p1"][0], v["r0"], v["r1"]))
        new.append(d)
geo["elevation"] = new
json.dump(doc, open(out, "w"), ensure_ascii=False, indent=1)
print("elevation shapes", len(new))
