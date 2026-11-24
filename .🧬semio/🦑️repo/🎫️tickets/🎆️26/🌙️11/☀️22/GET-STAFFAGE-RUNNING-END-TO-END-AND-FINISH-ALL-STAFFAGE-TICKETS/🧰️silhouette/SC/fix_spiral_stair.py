"""🌀️ Repair of `spiral-stair` at 1:50. (1) Side and front elevation: the four anchor studs with their nuts on the base plate (M16, 24 mm above the plate at 99 mm from the axis, seen in pairs) were not drawn (18.8 mm): each visible pair is drawn as its outline. (2) Side and front elevation: every baluster of the body stands on a 22 mm foot with a chamfer, the drawing had the plain 12 mm stroke; the feet decide whether the small openings between tread, baluster and handrail count as openings (49.5 mm where the two differed): the foot of every baluster is drawn. (3) Plan: the two packers and the two bolts that fix the top tread to the landing edge stand out of the tread and were not drawn (68.9 mm): they are drawn from their solids. Idempotent: python3 fix_spiral_stair.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "spiral-stair"
text, doc = sc.load(ID)
C = 0.09899
ANCHOR = [(0.02, -0.01386), (0.036, -0.01386), (0.036, -0.008), (0.042, -0.008), (0.044, -0.006), (0.044, 0.006), (0.042, 0.008), (0.036, 0.008), (0.036, 0.01386), (0.02, 0.01386)]
base = {"bone": "column", "role": "detail", "z": 1.1, "filled": True, "tone": 0.25, "region": "column-steel", "material": "steel", "kind": "poly"}
for sign, name in ((-1, "anchor-a"), (1, "anchor-b")):
    sc.put(sc.shapes(doc, 50, "elevation"), {"id": name, **base, "points": [[h, sc.r6(sign * C + y)] for h, y in ANCHOR], "closed": True}, after="base-plate")
    sc.put(sc.shapes(doc, 50, "front"), {"id": name, **base, "points": [[sc.r6(sign * C + y), h] for h, y in ANCHOR], "closed": True}, after="base-plate")

FOOT = [(-0.011, 0.0), (0.011, 0.0), (0.011, 0.004), (0.006, 0.009), (-0.006, 0.009), (-0.011, 0.004)]
footed = {s["id"][-1] for s in sc.solids(doc, 50) if s["id"].startswith("baluster-") and s["kind"] == "revolve" and s["profile"][:3] == [[0.0, 0.011], [0.004, 0.011], [0.009, 0.006]]}
for view in ("elevation", "front"):
    shapes = sc.shapes(doc, 50, view)
    for stroke in [s for s in shapes if s["id"].startswith("baluster-") and not s["id"].endswith("-foot") and s["id"][-1] in footed and s["kind"] == "poly" and len(s["points"]) == 2]:
        (x0, y0), (x1, y1) = stroke["points"]
        assert x0 == x1 and y0 < y1, stroke["id"]
        foot = {k: v for k, v in stroke.items() if k not in ("points", "closed", "penMm", "filled", "id")}
        foot = {"id": stroke["id"] + "-foot", **foot, "filled": True, "kind": "poly", "points": [[sc.r6(x0 + dx), sc.r6(y0 + dy)] for dx, dy in FOOT], "closed": True}
        sc.put(shapes, foot, after=stroke["id"])

plan = sc.shapes(doc, 50, "plan")
fix = {"bone": "tread-15", "role": "detail", "z": 2.25, "filled": True, "tone": 0.25, "region": "treads", "material": "steel", "kind": "poly"}
packer = sc.solid(doc, 50, "tread-fix-packer")
bolt = sc.solid(doc, 50, "tread-fix-bolt")
for n in range(2):
    dx, dz = packer["repeat"]["translation"][0] * n, packer["repeat"]["translation"][2] * n
    sc.put(plan, {"id": f"tread-fix-packer-{n + 1}", **fix, "points": [[sc.r6(x + dx), sc.r6(z + dz)] for z, x in packer["profile"]], "closed": True}, before="tread-15")
    ox, oz = bolt["origin"][0] + dx, bolt["origin"][2] + dz
    a = 0.7071067811865476
    side = [(t, r) for t, r in bolt["profile"]]
    ring = [(t, r) for t, r in side] + [(t, -r) for t, r in reversed(side)]
    sc.put(plan, {"id": f"tread-fix-bolt-{n + 1}", **fix, "points": [[sc.r6(ox + a * t - a * r), sc.r6(oz + a * t + a * r)] for t, r in ring], "closed": True}, before="tread-15")
sc.save(ID, text, doc)
