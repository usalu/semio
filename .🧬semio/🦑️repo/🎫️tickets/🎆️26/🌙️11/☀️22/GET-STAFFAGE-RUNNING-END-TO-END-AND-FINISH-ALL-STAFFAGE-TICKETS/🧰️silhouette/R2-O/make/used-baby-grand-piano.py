"""🎹️ Writes the repair ops of `used-baby-grand-piano` (ops/used-baby-grand-piano.json) from the document of its generator before this repair.

The two ratchet straps are flat webbing of 50 mm. They were `slender` solids of 3 mm: the guard draws a slender solid as a stroke along the centre line it finds in the mesh, and a flat band has no such line (the stroke wandered 26 mm to both sides and ran 28 mm past both ends, below the floor). The webbing is a thin sheet: it is modelled 2.5 mm thick (the gauge of 50 mm lashing webbing, a twentieth of its width) and declared `sheet`, so every view compares its true outline; the drawn bands get the same gauge.
The skid board and its carpet have two hand holes of 35 x 120 mm that the plan drew closed: their plan outline, a prism swept through the thickness, carries the holes as one ring with slits of no width (seen from above the holes are open, seen from the side the board is closed).
The collars of the three legs lay 0.5 mm clear of the edges of the skid, a hairline the drawing showed open and the body closed (48.8 mm): the collars are 65.5 mm long and touch the board, in the body and in the three views.
The worn patch on the lid edge was a rectangle of 80 x 200 mm that reached up to 21 mm past the curved edge of the lid, a patch of finish in the air: it is cut to the outline of the lid and reaches further in from the edge so that it keeps its 0.016 m2, in the body and in the three views of every scale."""
import json, os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, write

SLUG, GAUGE = "used-baby-grand-piano", 0.0025
a = asset(SLUG)
ops = []
r6 = lambda v: round(v, 6)
thin = lambda v: r6(v * GAUGE / 0.003) if abs(abs(v) - 0.0015) < 1e-9 else v
for scale in ("50", "100", "200", "500"):
    for s in a["volumes"][scale]["body"]:
        if not (s["id"].startswith("strap-") and s.get("slender")):
            continue
        change = {"sheet": True}
        if s["plane"] == "xy":
            change["profile"] = [[p[0], thin(p[1])] for p in s["profile"]]
        else:
            change["to"] = GAUGE
        ops.append({"op": "solid-set", "scale": scale, "id": s["id"], "set": change})
        ops.append({"op": "set", "path": ["volumes", scale, "body", {"id": s["id"]}, "slender"]})
    for view, shapes in a["representations"][scale]["geometry"].items():
        for s in shapes:
            if not s["id"].startswith("strap-") or s.get("kind") != "poly" or "solid" not in s:
                continue
            if s["solid"]["sweep"] == "w" and any(abs(abs(p[1]) - 0.0015) < 1e-9 for p in s["points"]):
                ops.append({"op": "shape-set", "scale": scale, "view": view, "id": s["id"], "set": {"points": [[p[0], thin(p[1])] for p in s["points"]]}})
            elif s["solid"]["sweep"] == "v" and abs(s["solid"]["z1"] - 0.003) < 1e-9:
                ops.append({"op": "shape-set", "scale": scale, "view": view, "id": s["id"], "set": {"solid": {**s["solid"], "z1": GAUGE}}})
for scale in ("50", "100"):
    for sid in ("skid-board", "skid-carpet"):
        solid = next((s for s in a["volumes"][scale]["body"] if s["id"] == sid), None)
        shape = next((s for s in a["representations"][scale]["geometry"]["plan"] if s["id"] == sid), None)
        if solid is None or shape is None or not solid.get("holes"):
            continue
        xs, zs = [p[0] for p in shape["points"]], [p[1] for p in shape["points"]]
        x0, x1, z0, z1 = min(xs), max(xs), min(zs), max(zs)
        low, high = sorted(([[p[1], p[0]] for p in h] for h in solid["holes"]), key=lambda h: h[0][1])
        def loop(hole, edge):
            """🕳️ From an end edge straight to the middle of the near side of a hole, around the hole and back."""
            i = min(range(len(hole)), key=lambda k: abs(hole[k][1] - edge) + abs(hole[(k + 1) % len(hole)][1] - edge))
            turned = hole[i + 1:] + hole[:i + 1]
            middle = [(hole[i][0] + hole[(i + 1) % len(hole)][0]) / 2, hole[i][1]]
            return [[middle[0], edge], middle] + turned + [middle, [middle[0], edge]]
        ring = [[x0, z0]] + loop(low, z0) + [[x1, z0], [x1, z1]] + loop(high, z1)[::-1] + [[x0, z1]]
        ops.append({"op": "shape-set", "scale": scale, "view": "plan", "id": sid, "set": {"points": [[r6(p[0]), r6(p[1])] for p in ring]}})
for scale in ("50", "100", "200", "500"):
    for sid in ("leg-bass-collar", "leg-treble-collar", "leg-tail-collar"):
        solid = next((s for s in a["volumes"][scale]["body"] if s["id"] == sid), None)
        if solid is None or solid.get("from") != 0.0:
            continue
        ops.append({"op": "solid-set", "scale": scale, "id": sid, "set": {"from": -0.0005}})
        for view, shapes in a["representations"][scale]["geometry"].items():
            for s in shapes:
                if s["id"] == sid and s.get("solid", {}).get("z0") == 0.0:
                    ops.append({"op": "shape-set", "scale": scale, "view": view, "id": sid, "set": {"solid": {**s["solid"], "z0": -0.0005}}})
    lid = next(s for s in a["volumes"][scale]["body"] if s["id"] == "lid")
    patch = next((s for s in a["volumes"][scale]["body"] if s["id"] == "wear-lid-edge"), None)
    if patch is None:
        continue
    zs, xs = [p[0] for p in patch["profile"]], [p[1] for p in patch["profile"]]

    def within(z_low):
        """✂️ The lid inside the rectangle of the patch from `z_low` to its outer side (Sutherland-Hodgman, the rectangle clips)."""
        cut = lid["profile"]
        for axis, sign, bound in ((0, 1, z_low), (0, -1, max(zs)), (1, 1, min(xs)), (1, -1, max(xs))):
            out = []
            for i, p in enumerate(cut):
                q = cut[(i + 1) % len(cut)]
                dp, dq = sign * (p[axis] - bound), sign * (q[axis] - bound)
                if dp >= 0:
                    out.append(p)
                if (dp > 0 and dq < 0) or (dp < 0 and dq > 0):
                    k = dp / (dp - dq)
                    out.append([p[0] + k * (q[0] - p[0]), p[1] + k * (q[1] - p[1])])
            cut = out
        return cut

    area = lambda ring: abs(sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))) / 2
    target = (max(zs) - min(zs)) * (max(xs) - min(xs))
    low, high = min(p[0] for p in lid["profile"]), min(zs)
    for _ in range(40):
        middle = (low + high) / 2
        low, high = (middle, high) if area(within(middle)) > target else (low, middle)
    cut = [[r6(p[0]), r6(p[1])] for p in within(r6(low))]
    ops.append({"op": "solid-set", "scale": scale, "id": "wear-lid-edge", "set": {"profile": cut}})
    for view, shapes in a["representations"][scale]["geometry"].items():
        if any(s["id"] == "wear-lid-edge" for s in shapes):
            ops.append({"op": "shape-set", "scale": scale, "view": view, "id": "wear-lid-edge", "set": {"points": [[p[1], p[0]] for p in cut]}})
write(SLUG, ops)
print(f"{SLUG}: {len(ops)} ops")
