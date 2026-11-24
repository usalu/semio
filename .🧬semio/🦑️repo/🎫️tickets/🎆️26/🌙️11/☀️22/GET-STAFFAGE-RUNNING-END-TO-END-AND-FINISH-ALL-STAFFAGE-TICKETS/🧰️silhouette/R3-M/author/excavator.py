"""🦾️ Hand repairs of `excavator` of the third round (after the ops of R2-M), authored against the catalog file as it stood before this round (`SAM_SOURCE`). Every drawn part is an extrusion over the lateral range of its solid on the bone of the solid (the idiom of this asset, so nothing flat rides on a turning bone).
Volume 1:50: the house box reached 120 mm into the counterweight (-2.38 m, the other scales and the drawing end at its face, -2.26 m); it ends at the face, and its folded edges have 20 mm radius (40 mm before; the drawing and the bodies of the other scales have square edges there). Side elevation 1:50: the tongue of the track frame towards the idler has the height of the idler yoke (0.17 m, drawn 0.2 m).
1:50 and 1:100, three views: the two grab rails of the cab (door and front, 30 mm tube: a bar and its two stand-offs each); the two ears of the bucket as two 45 mm plates in place of one block of 430 mm, painted over the 35 mm link plates that lie within their thickness (under them 5 mm of each ear would show on either side, which the print profile refuses); the pin of the bucket link.
1:50, three views: the eye of the stick cylinder rod was one block of 350 mm across the stick; it is the two bosses of 70 mm and the pin through them, as the body has them (and as R2-M drew them at 1:100 and 1:200); the two boom pipes lie on the centre line of their solids (they were drawn 4 mm higher, a seam of one raster cell along 1.4 m against the boom plate) and reach 2 mm into the plate (over the crown of the boom, where the loft of the body sags between its sections, the drawn pipe is 3 mm lower); the barrels of the five cylinders start 60 mm behind their eye as their solids do; the pin of the stick at the boom tip (front, plan); the cab has the square lower front corner of its floor slab and the toolbox the 30 mm corner radius of its box.
1:50, further: the feet of the stick cylinder and of the bucket cylinder were one block of 200 mm each; they are the two 30 mm lugs and the pin (three views; the pin in front and plan); the pin of the boom cylinder rods (front, plan); the glands of the boom cylinders as bodies of revolution (they were boxes, 25 mm fuller at the corners in plan); side elevation: the sprocket as its 0.3 m disc and its 23 teeth (it was a smooth disc of 0.335 m); front and plan: the right mirror arm as three pieces of its 24 mm tube at their own heights (it was one slab 60 mm high).
1:100, plan: the two tip bosses of the boom (in the front elevation they show 10 mm beside the root boss of the stick, which the print profile refuses, and that view agrees without them); front: the pin of the idler link."""
import copy, math, re
from oplib import load, shape, solid, write

ID = "excavator"
a = load(ID)
ops = []
VIEWS = ("elevation", "front", "plan")


def shapes_of(s, role="detail"):
    """🧩️ The extruded shape of one solid: an xy extrude, a revolve about the lateral axis, or an unrotated box."""
    base = {"bone": s["bone"], "role": role, "filled": True, "tone": s.get("tone", 0.4), "material": s["material"]}
    if s["kind"] == "extrude" and s.get("plane") == "xy":
        one = dict(base, kind="path", d="M " + " L ".join(f"{x},{y}" for x, y in s["profile"]) + " Z")
        lo, hi = s["from"], s["to"]
    elif s["kind"] == "revolve" and [abs(v) for v in s["axis"]] == [0, 0, 1]:
        r, n = max(p[1] for p in s["profile"]), max(p[0] for p in s["profile"])
        ox, oy, oz = s["origin"]
        one = dict(base, kind="ellipse", cx=ox, cy=oy, rx=r, ry=r)
        lo, hi = sorted((oz, oz + n * s["axis"][2]))
    else:
        raise SystemExit(f"{s['id']}: kind {s['kind']} not supported")
    return dict(one, id=s["id"], solid={"kind": "extruded", "sweep": "w", "z0": round(lo, 4), "z1": round(hi, 4)})


def listed(scale, view):
    """📜️ The drawn shapes of one view."""
    return a["representations"][scale]["geometry"][view]


def has(scale, view, ident):
    """❓️ Whether a view draws a shape."""
    return any(s["id"] == ident for s in listed(scale, view))


def add(scale, view, after, shp, z):
    """➕️ Adds a shape after another one."""
    ops.append({"op": "shape-add", "scale": scale, "view": view, "after": after, "shape": dict(shp, z=z)})


def rail(ident, x, y0, y1):
    """🪜️ A grab rail of 30 mm tube standing 65 mm off the cab side: the bar and its two stand-offs as three prisms."""
    box = lambda name, xa, ya, xb, yb, za, zb: {"id": f"{ident}{name}", "bone": "upper", "role": "detail", "filled": True, "tone": 0.3, "material": "paint-steel", "kind": "path", "d": f"M {xa},{ya} L {xb},{ya} L {xb},{yb} L {xa},{yb} Z", "solid": {"kind": "extruded", "sweep": "w", "z0": za, "z1": zb}}
    xa, xb = round(x - 0.015, 4), round(x + 0.015, 4)
    return [box("", xa, round(y0 - 0.015, 4), xb, round(y1 + 0.015, 4), 1.41, 1.44), box("-foot", xa, round(y0 - 0.015, 4), xb, round(y0 + 0.015, 4), 1.36, 1.412), box("-head", xa, round(y1 - 0.015, 4), xb, round(y1 + 0.015, 4), 1.36, 1.412)]


ops.append({"op": "solid-set", "scale": "50", "id": "house", "set": {"size": [1.88, 0.96, 2.74], "cornerRadius": 0.02, "transform": {"translation": [-1.32, 0.76, 0]}}})
frame = shape(a, "50", "elevation", "track-frame")
assert "L 1.5,0.35 L 1.9,0.35 L 1.9,0.55 L 1.5,0.55" in frame["d"]
ops.append({"op": "shape-set", "scale": "50", "view": "elevation", "id": "track-frame", "set": {"d": frame["d"].replace("L 1.5,0.35 L 1.9,0.35 L 1.9,0.55 L 1.5,0.55", "L 1.5,0.367 L 1.9,0.362 L 1.9,0.538 L 1.5,0.533")}})
for scale in ("50", "100"):
    for view in VIEWS:
        shapes = listed(scale, view)
        top = max(s.get("z", 0) for s in shapes if s.get("bone") == "upper")
        anchor = next(s["id"] for s in shapes if s.get("bone") == "upper" and s.get("z", 0) == top)
        for piece in rail("cab-grab-door", 0.5, 0.74, 1.56) + rail("cab-grab-front", 1.405, 0.5, 0.98):
            add(scale, view, anchor, piece, round(top + 0.5, 3))
        ear = shape(a, scale, view, "bucket-ear")
        over = round(max(s.get("z", 0) for s in shapes if s.get("bone") in ("bucket", "bucket-link", "idler-link", "bucket-rod")) + 0.5, 3)
        for side in ("right", "left"):
            plate = solid(a, scale, f"bucket-ear-{side}")
            add(scale, view, "bucket-ear", dict(copy.deepcopy(ear), id=f"bucket-ear-{side}", solid={"kind": "extruded", "sweep": "w", "z0": plate["from"], "z1": plate["to"]}), over)
        ops.append({"op": "shape-remove", "scale": scale, "view": view, "id": "bucket-ear"})
        if not has(scale, view, "bucket-link-pin"):
            add(scale, view, "bucket-ear-left", shapes_of(solid(a, scale, "bucket-link-pin")), round(ear["z"] - 0.01, 3))
for view in VIEWS:
    eye = shape(a, "50", view, "stick-rod-eye")
    for ident in ("stick-rod-boss-right", "stick-rod-boss-left", "stick-rod-pin"):
        add("50", view, "stick-rod-eye", shapes_of(solid(a, "50", ident)), eye["z"] if "boss" in ident else round(eye["z"] + 0.01, 3))
    ops.append({"op": "shape-remove", "scale": "50", "view": view, "id": "stick-rod-eye"})
    for ident in ("boom-pipe-right", "boom-pipe-left"):
        if has("50", view, ident) and shape(a, "50", view, ident)["solid"]["sweep"] == "w":
            line = [(sec["at"][0], sec["at"][1]) for sec in solid(a, "50", ident)["sections"]]
            sides = ([], [])
            for k, (x, y) in enumerate(line):
                (ax, ay), (bx, by) = line[max(k - 1, 0)], line[min(k + 1, len(line) - 1)]
                n = math.hypot(bx - ax, by - ay)
                nx, ny = -(by - ay) / n, (bx - ax) / n
                sides[0].append((round(x - nx * 0.019, 4), round(y - ny * 0.019, 4)))
                top = 0.014 if 2 <= k <= 4 else 0.017
                sides[1].append((round(x + nx * top, 4), round(y + ny * top, 4)))
            ops.append({"op": "shape-set", "scale": "50", "view": view, "id": ident, "set": {"d": "M " + " L ".join(f"{x},{y}" for x, y in sides[0] + sides[1][::-1]) + " Z"}})
for view in ("front", "plan"):
    low = min(s.get("z", 0) for s in listed("100", view))
    for ident in ("link-pin",) if view == "front" else ("boom-tip-boss-right", "boom-tip-boss-left"):
        if not has("100", view, ident):
            add("100", view, listed("100", view)[0]["id"], shapes_of(solid(a, "100", ident)), round(low - 0.1, 3))
for view in VIEWS:
    for foot, lugs, pin in (("stick-cyl-foot", ("stick-cyl-lug-right", "stick-cyl-lug-left"), "stick-cyl-pin"), ("bucket-cyl-foot", ("bucket-cyl-lug-right", "bucket-cyl-lug-left"), "bucket-cyl-pin")):
        old = shape(a, "50", view, foot)
        for ident in lugs:
            add("50", view, foot, dict(shapes_of(solid(a, "50", ident)), tone=old["tone"]), old["z"])
        if view != "elevation":
            add("50", view, foot, shapes_of(solid(a, "50", pin)), round(old["z"] - 0.01, 3))
        ops.append({"op": "shape-remove", "scale": "50", "view": view, "id": foot})
    if view != "elevation":
        eye = shape(a, "50", view, "boom-rod-eye")
        add("50", view, "boom-rod-eye", shapes_of(solid(a, "50", "boom-rod-pin")), round(eye["z"] - 0.01, 3))
    for ident in ("boom-cyl-gland", "boom-cyl-gland-left", "boom-cyl-gland-right"):
        if has("50", view, ident):
            g = copy.deepcopy(shape(a, "50", view, ident))
            lateral = round((g["solid"]["z0"] + g["solid"]["z1"]) / 2, 4)
            g.pop("d")
            g.update(kind="poly", points=[[1.55, 0], [1.62, 0], [1.62, 0.0845], [1.55, 0.0845]], closed=True, solid={"kind": "revolved", "axisBone": "boom-cyl", "axis": "u", "axisX": 0, "axisZ": lateral, "mirror": True})
            ops.append({"op": "shape", "scale": "50", "view": view, "id": ident, "shape": g})
    for s in listed("50", view):
        if s["id"].startswith(("stick-cyl-barrel", "bucket-cyl-barrel", "boom-cyl-barrel")) and s.get("kind") == "member":
            ops.append({"op": "shape-set", "scale": "50", "view": view, "id": s["id"], "set": {"from": dict(s["from"], x=0.06)}})
    if view != "elevation":
        add("50", view, "boom-tip-boss", shapes_of(solid(a, "50", "stick-pin")), round(shape(a, "50", view, "boom-tip-boss")["z"] - 0.01, 3))
    for s in listed("50", view):
        if "L 1.39,0.3 A 0.05 0.05 0 0 1 1.44,0.35" in s.get("d", ""):
            ops.append({"op": "shape-set", "scale": "50", "view": view, "id": s["id"], "set": {"d": s["d"].replace("L 1.39,0.3 A 0.05 0.05 0 0 1 1.44,0.35", "L 1.44,0.3")}})
    if has("50", view, "toolbox") and shape(a, "50", view, "toolbox")["solid"]["sweep"] == "w":
        ops.append({"op": "shape-set", "scale": "50", "view": view, "id": "toolbox", "set": {"d": "M 0.75,0.28 L 1.39,0.28 A 0.03 0.03 0 0 1 1.42,0.31 L 1.42,0.89 A 0.03 0.03 0 0 1 1.39,0.92 L 0.75,0.92 Z"}})
ops.append({"op": "shape-set", "scale": "50", "view": "elevation", "id": "sprocket", "set": {"rx": 0.3, "ry": 0.3}})
teeth = []
for k in range(23):
    c, sn = math.cos(k * 0.2732), math.sin(k * 0.2732)
    corners = [(-1.825 + (0.32 + dx) * c - dy * sn, 0.47 + (0.32 + dx) * sn + dy * c) for dx, dy in ((-0.035, -0.0225), (0.035, -0.0225), (0.035, 0.0225), (-0.035, 0.0225))]
    teeth.append("M " + " L ".join(f"{round(x, 4)},{round(y, 4)}" for x, y in corners) + " Z")
disc = shape(a, "50", "elevation", "sprocket")
add("50", "elevation", "sprocket", {"id": "sprocket-teeth", "bone": "undercarriage", "role": "detail", "filled": True, "tone": disc["tone"], "kind": "path", "d": " ".join(teeth), **({"material": disc["material"]} if "material" in disc else {})}, round(disc["z"] - 0.01, 3))
for view in ("front", "plan"):
    arm = shape(a, "50", view, "mirror-arm-right")

    def leg(ident, p, q, v0, v1):
        """🪞️ One straight piece of the 24 mm mirror arm as a slab swept through its height."""
        (px, pz), (qx, qz) = p, q
        n = math.hypot(qx - px, qz - pz)
        ox, oz = -(qz - pz) / n * 0.012, (qx - px) / n * 0.012
        pts = [(px + ox, pz + oz), (qx + ox, qz + oz), (qx - ox, qz - oz), (px - ox, pz - oz)]
        return dict(copy.deepcopy(arm), id=ident, d="M " + " L ".join(f"{round(x, 4)},{round(z, 4)}" for x, z in pts) + " Z", solid={"kind": "extruded", "sweep": "v", "z0": v0, "z1": v1})

    ops.append({"op": "shape", "scale": "50", "view": view, "id": "mirror-arm-right", "shape": leg("mirror-arm-right", (0.7, -1.3), (0.862, -1.3608), 1.588, 1.612)})
    add("50", view, "mirror-arm-right", leg("mirror-arm-right-2", (0.858, -1.359), (0.902, -1.381), 1.568, 1.612), arm["z"])
    add("50", view, "mirror-arm-right-2", leg("mirror-arm-right-3", (0.898, -1.379), (0.94, -1.4), 1.548, 1.592), arm["z"])
write(ID, ops)
