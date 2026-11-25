"""🔥️ Ops writer of `used-wall-hung-gas-boiler`: the two lashing straps fold down by up to 209 degrees when they are released and the boiler tilts by 30 degrees while it is lifted off the pallet, but the straps and several parts of the boiler were flat outlines of the rest pose in front and plan (front up to 156 mm below the floor, plan 107 mm longer than the side elevation). Strap feet, legs, tops and ratchets and every boiler part whose flat outline is the rectangle of its side elevation twin become that twin swept over their width; the round knobs and test plugs of the front and the flue adapter of the plan are swept along their own axis over the depth their twin gives; display and label are skins and the seam, the limescale streak and the strap stitching ribbons on the face they lie on: `python3 make/used-wall-hung-gas-boiler.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("used-wall-hung-gas-boiler")
bones = subtree(a, {"boiler", "strap-back-foot", "strap-front-foot"})
frames = rest(a)
OX, OY = frames["boiler"][:2]
done = set()
ops = []


def put_shape(scale, view, sid, new):
    """📌️ Records a manual op of a shape."""
    ops.append(new if "op" in new else op(scale, view, sid, new))
    done.add((scale, view, sid))


for scale in scales(a):
    face = world_box(a, scale, "casing")[1]
    fascia = world_box(a, scale, "fascia")[1] if find(a, scale, "elevation", "fascia") else face
    for s in views(a, scale)["front"]:
        if s["bone"] not in bones:
            continue
        sid = s["id"]
        if "solid" in s:
            continue
        twin = find(a, scale, "elevation", sid)
        if s["kind"] == "ellipse" and twin is not None:
            b = world_box(a, scale, sid)
            put_shape(scale, "front", sid, own_sweep(a, scale, "front", sid, b[0], b[1]))
        elif sid in ("display", "service-label"):
            at = fascia if box(s)[3] < 0.5 else face
            put_shape(scale, "front", sid, own_sweep(a, scale, "front", sid, at, at + 0.002))
        elif sid in ("fascia-seam", "limescale-streak"):
            at = (fascia if sid == "limescale-streak" else face) - OX
            put_shape(scale, "front", sid, sliver(s, [(y - OY, x) for x, y in s["points"]], at, at + 0.002, sweep="u"))
        elif sid.startswith("stitching-"):
            f = frames[s["bone"]]
            (x, y0), (_, y1) = s["points"]
            u = sorted(max(0.0, to_local(f, (f[0] + (y - f[1]) / math.tan(math.radians(f[2])), y))[0]) for y in (y0, y1))
            put_shape(scale, "front", sid, sliver(s, [(u[0], -0.001), (u[1], -0.001)], x - 0.001, x + 0.001))
        elif sid == "adapter":
            b = world_box(a, scale, sid)
            put_shape(scale, "front", sid, own_sweep(a, scale, "front", sid, b[0], b[1]))
    for s in views(a, scale)["plan"]:
        if s["bone"] not in bones or "solid" in s:
            continue
        sid = s["id"]
        twin = find(a, scale, "elevation", sid)
        if twin is None:
            continue
        b = world_box(a, scale, sid)
        if s["kind"] == "ellipse":
            put_shape(scale, "plan", sid, own_sweep(a, scale, "plan", sid, b[2], b[3]))
        elif s["kind"] == "path":
            put_shape(scale, "plan", sid, like(s, d=shift_path(s["d"], -OX), solid=extruded("v", b[2] - OY, b[3] - OY)))
auto = solidify(a, bones, quiet=True)
write("used-wall-hung-gas-boiler", auto + ops)
