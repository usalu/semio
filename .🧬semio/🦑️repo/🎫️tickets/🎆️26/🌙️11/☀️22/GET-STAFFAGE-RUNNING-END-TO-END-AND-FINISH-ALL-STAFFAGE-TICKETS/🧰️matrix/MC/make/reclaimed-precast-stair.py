"""🪜️ Ops writer of `reclaimed-precast-stair`: the flight lies on its soffit at rest and turns by its pitch of 32 degrees when it is lifted and set in place, but every front and plan shape of it was a flat outline of the rest pose (front up to 792 mm off the side elevation, the bearing contacts up to 673 mm off the drawn flight). Every part of the flight becomes its side elevation outline swept over its width: the body (in the front of 1:50 and 1:100 in two parts, behind and before the mortar pat under the sawn nib, so that the pat keeps its place in the paint order), the terrazzo layers (the top landing strip worn down over its walking zone, with the unworn nosing as a cap at either side), the bar stubs and the mortar pat (in the front the strip of it that shows below the nib); the cut bars are round stubs along the flight, the lifting sockets discs on their tread, and the strokes (nib edge, nosings, wear lines, the spalled nosing) slivers at the edge they mark: `python3 make/reclaimed-precast-stair.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("reclaimed-precast-stair")
OX, OY = rest(a)["flight"][:2]
W = 0.55
NOSE, PITCH, SLOPE = (0.21411, 0.2284), 0.33019, 0.625
CUT = 2.975
NIB = (3.09412, 0.04396)


def tread(u):
    """👣️ The height of the tread surface above the bone axis at a position along the flight."""
    k = max(0, int((u - NOSE[0] + 1e-9) // PITCH))
    return NOSE[1] - (u - NOSE[0] - k * PITCH) * SLOPE


def keep(scale, view, sid):
    """♻️ The op that keeps a shape that is a solid already, or None."""
    s = find(a, scale, view, sid)
    return op(scale, view, sid, s) if s is not None and "solid" in s else None


ops = solidify(a, {"flight"}, twins={f"bar-stub-{i}": "bar-stub" for i in range(1, 5)}, force={"core", "terrazzo-00", "mortar-top", "bar-stub-1", "bar-stub-2", "bar-stub-3", "bar-stub-4"}, skip={"terrazzo-09-side-l", "terrazzo-09-side-r", "nib-saw-cut"}, quiet=True)
ops = [o for o in ops if not (o["view"] == "front" and o["id"] in ("core", "terrazzo-09") and o["scale"] in ("50", "100"))]
for scale in ("50", "100"):
    core = shape(a, scale, "elevation", "core")["points"]
    for sid, below in (("core", True), ("nib-saw-cut", False)):
        ops.append(keep(scale, "front", sid) or op(scale, "front", sid, like(shape(a, scale, "front", sid), drop=GEOMETRY, kind="poly", points=clip(core, 0, below, CUT), closed=True, solid=extruded("w", -W, W))))
    layer = shape(a, scale, "elevation", "terrazzo-09")["points"]
    worn = find(a, scale, "front", "terrazzo-09")
    if "solid" in worn:
        ops.append(op(scale, "front", "terrazzo-09", worn))
        ops += [op(scale, "front", f"terrazzo-09-side-{side}", shape(a, scale, "front", f"terrazzo-09-side-{side}")) for side in ("l", "r") if has(a, scale, "front", f"terrazzo-09-side-{side}")]
    elif min(p[1] for p in worn["points"] if p[1] > (box(worn)[2] + box(worn)[3]) / 2) > box(worn)[3] - 1e-6:
        ops.append(op(scale, "front", "terrazzo-09", like(worn, drop=GEOMETRY, kind="poly", points=layer, closed=True, solid=extruded("w", -W, W))))
    else:
        b = box(worn)
        low = min(p[1] for p in worn["points"] if p[1] > (b[2] + b[3]) / 2 and abs(p[0]) < 0.31) - OY
        inner = max(abs(p[0]) for p in worn["points"] if abs(p[0]) < W - 1e-6)
        ops.append(op(scale, "front", "terrazzo-09", like(worn, drop=GEOMETRY, kind="poly", points=clip(layer, 1, True, low), closed=True, solid=extruded("w", -W, W))))
        for side, z0, z1 in (("l", inner, W), ("r", -W, -inner)):
            ops.append(add(scale, "front", "terrazzo-09", like(worn, drop=GEOMETRY, id=f"terrazzo-09-side-{side}", kind="poly", points=clip(layer, 1, False, low), closed=True, solid=extruded("w", z0, z1))))
for i in range(1, 8):
    s = find(a, "50", "front", f"cut-bar-{i}")
    if "solid" in s:
        ops.append(op("50", "front", s["id"], s))
        continue
    v = s["cy"] - OY
    face = 3.05702 + (v + 0.0154) / 0.05936 * 0.0371
    ops.append(op("50", "front", s["id"], like(s, cx=round(v, 6), cy=s["cx"], solid=extruded("u", face - 0.02, face + 0.002))))
pat = find(a, "50", "front", "mortar-top")
ops = [o for o in ops if not (o["scale"] == "50" and o["view"] == "front" and o.get("id") == "mortar-top")]
ops.append(op("50", "front", "mortar-top", pat if pat["kind"] == "poly" else like(pat, drop=GEOMETRY, kind="poly", points=clip(shape(a, "50", "elevation", "mortar-top")["points"], 1, True, -0.0154), closed=True, solid=extruded("w", pat["x"], pat["x"] + pat["w"]))))
edge = keep("200", "front", "nib-line") or op("200", "front", "nib-line", sliver(shape(a, "200", "front", "nib-line"), [NIB], -W, W))
ops.append(edge)
for scale in scales(a):
    for s in views(a, scale)["plan"]:
        if s["bone"] != "flight" or "solid" in s:
            if s["bone"] == "flight" and not any(o["view"] == "plan" and o["scale"] == scale and o["id"] == s["id"] for o in ops):
                ops.append(op(scale, "plan", s["id"], s))
            continue
        if s["id"].startswith("wear-") or s["id"].startswith("nosing-"):
            u = s["points"][0][0] - OX
            z = sorted(p[1] for p in s["points"])
            ops.append(op(scale, "plan", s["id"], sliver(s, [(u, tread(u))], z[0], z[-1])))
        elif s["id"] == "spall-nosing-03":
            z = sorted(p[1] for p in s["points"])
            ops.append(op(scale, "plan", s["id"], sliver(s, [tuple(p) for p in shape(a, scale, "elevation", s["id"])["points"]], z[0], z[-1])))
        elif s["id"].startswith("socket-"):
            u = s["cx"] - OX
            ops.append(op(scale, "plan", s["id"], like(s, cx=round(u, 6), solid=extruded("v", tread(u), tread(u) + 0.002))))
left = [(scale, view, s["id"]) for scale in scales(a) for view in ("front", "plan") for s in views(a, scale)[view] if s["bone"] == "flight" and not any(o.get("scale") == scale and o.get("view") == view and (o.get("id") == s["id"] or o.get("shape", {}).get("id") == s["id"]) for o in ops)]
print("   left flat:", left)
write("reclaimed-precast-stair", ops)
