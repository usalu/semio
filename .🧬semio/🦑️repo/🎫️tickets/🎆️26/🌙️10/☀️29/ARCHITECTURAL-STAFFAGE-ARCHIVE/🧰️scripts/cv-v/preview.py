import sys, json, os
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
# usage: preview.py <file.json> <rep: orig|500|200|100|50|sym:<id>> <render denominator> <px per paper mm> <out.png> [view: elevation|plan]
file, rep, den, pxmm, out = sys.argv[1], sys.argv[2], float(sys.argv[3]), float(sys.argv[4]), sys.argv[5]
view = sys.argv[6] if len(sys.argv) > 6 else "elevation"
doc = json.load(open(file)); a = doc["asset"]
rig = Rig(a)
if rep == "orig":
    shapes = a["geometry"]["elevation"]; pens = {"body": .25, "detail": .18, "texture": .13, "glazing": .1, "ground": .3}
elif rep.startswith("sym:"):
    sy = [s for s in a["symbols"] if s["id"] == rep[4:]][0]
    shapes = sy["geometry"][view]; pens = sy["pens"]
else:
    r = a["representations"][rep]; shapes = r["geometry"][view]; pens = r["pens"]
bk = [s for s in shapes]
items = []
allx = []; ally = []
sym_mm = rep.startswith("sym:") and sy["units"] == "paper-mm"
for s in sorted(bk, key=lambda s: s["z"]):
    role = s["role"]; pen = s.get("penMm") or pens.get(role)
    if pen is None: continue
    if sym_mm:
        d = shape_world_d(type("R", (), {"to_world": lambda self, b, x, y: (x, y)})(), s)
        d = parse_path(d) if False else d
    else:
        d = shape_world_d(rig, s)
    nums = [float(x) for x in re.findall(r"-?\d+\.?\d*", d)]
    allx += nums[0::2]; ally += nums[1::2]
    fill = "none"; fo = 0
    if s["filled"] and role == "body":
        g = int(255 - 140*(1-s["tone"])**1 if False else 255 - (s["tone"]*150)); fill = f"rgb({g},{g},{g})"; fo = 1.0
    items.append((d, fill, "#111", pen*pxmm, fo))
if sym_mm:
    k = pxmm; x0 = min(allx) - 1; y0 = min(ally) - 1; W = int((max(allx) - x0 + 1)*k); H = int((max(ally) - y0 + 1)*k)
    scale = k
    items = [(d, fl, st, sw, fo) for d, fl, st, sw, fo in items]
    render_svg(items, W, H, x0, y0, scale, out + ".svg"); png(out + ".svg", out); print(out, W, H)
else:
    scale = pxmm*1000/den
    x0 = min(allx) - 0.5*den/1000; y0 = min(min(ally), 0) - 0.5*den/1000
    W = int((max(allx) - x0)*scale + 0.5*den/1000*scale); H = int((max(ally) - y0)*scale + 0.5*den/1000*scale)
    ground = f"M{x0} 0L{max(allx)+1} 0"
    items.append((ground, "none", "#444", pens.get("ground", .3)*pxmm, 0))
    render_svg(items, W, H, x0, y0, scale, out + ".svg"); png(out + ".svg", out); print(out, W, H, "bbox", f(min(allx)), f(max(allx)), f(min(ally)), f(max(ally)))
