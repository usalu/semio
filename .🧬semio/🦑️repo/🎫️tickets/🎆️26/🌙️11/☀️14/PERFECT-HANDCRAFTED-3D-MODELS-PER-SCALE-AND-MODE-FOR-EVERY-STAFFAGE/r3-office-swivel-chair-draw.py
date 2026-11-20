"""✏ Round 3, furniture/office-swivel-chair: the redrawn 2D shapes of every scale and view (ruling E1). Every filled shape is the true orthographic outline of a named group of that scale's body solids (from `r3-office-swivel-chair-outline.ts`), bound to its bone with the ids, roles and line hierarchy of the previous drawing; wheels are ellipses, detail strokes are placed by hand from the design numbers.

Seat and backrest shapes of the side and front elevations are extruded solids (side: profile swept across the chair; front: outline swept through the part), so the swivel still turns them in every view; plan shapes are flat outlines, which a swivel about the vertical axis turns exactly.
"""
import importlib.util
import json
import math
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("chair_kit_draw", os.path.join(HERE, "r3-office-swivel-chair-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

TOL = {50: 0.0008, 100: 0.0016, 200: 0.003, 500: 0.006}
REGION = {"pa-plastic": "chair-plastic", "steel-painted": "chair-steel", "rubber": "castor-rubber", "fabric-upholstery": "chair-fabric"}
SEAT_TREE = ("seat", "backrest")
CASTORS_ELEVATION = ["back-a", "side-a", "front"]
BACK, SIDE = ["back-a", "back-b"], ["side-a", "side-b"]
ALL = ["front", "side-a", "side-b", "back-a", "back-b"]


def groups(ids):
    """🧺 The named groups of body solids of one scale; a group that has no solid at the scale is absent."""
    g = {}

    def put(name, pattern):
        hit = [i for i in ids if re.fullmatch(pattern, i)]
        if hit:
            g[name] = hit

    for k in ALL:
        put(f"wheel-{k}", rf"castor-{k}-(tyre-.|wheel(-.)?)")
        put(f"rim-{k}", rf"castor-{k}-rim-.")
        put(f"hood-{k}", rf"castor-{k}-(hood|web|head)")
        put(f"neck-{k}", rf"castor-{k}-neck")
        put(f"arm-{k}", rf"arm-{k}")
        put(f"boss-{k}", rf"arm-{k}-boss")
    put("hub", r"hub")
    put("star", r"hub|arm-.*")
    put("sleeve", r"column-sleeve")
    put("collar", r"column-collar")
    put("column", r"column")
    put("cone", r"column-cone")
    put("mechanism", r"seat-(mechanism|plate|hinge|hinge-pin)")
    put("knob", r"seat-knob")
    put("lever", r"seat-lever(-paddle)?")
    put("lock", r"seat-lock-(lever|paddle)")
    put("shell", r"seat-shell")
    put("cushion", r"seat-cushion")
    for side in ("left", "right"):
        put(f"post-{side}", rf"armrest-(strap|post|stem)-{side}")
        put(f"upright-{side}", rf"armrest-(post|stem)-{side}")
        put(f"strap-{side}", rf"armrest-strap-{side}")
        put(f"pad-{side}", rf"armrest-(pad|cap)-{side}")
        put(f"cap-{side}", rf"armrest-cap-{side}")
    put("strut", r"backrest-strut")
    put("foot", r"backrest-strut-foot")
    put("bracket", r"backrest-bracket")
    put("back", r"backrest-(shell|pad)(-left|-right)?")
    put("backpad", r"backrest-pad(-left|-right)?")
    return g


def entry(shape_id, bone, role, z, group, prim="poly", pen=None):
    return {"id": shape_id, "bone": bone, "role": role, "z": z, "group": group, "prim": prim, "pen": pen}


def plan_of(scale, view, g):
    """📋 The shapes of one view in paint order (far to near): id, bone, role, layer and the group of solids each one is the outline of."""
    e = []
    big = scale >= 100
    seat_id = "seat-band" if scale == 500 else "seat-cushion"
    back_id = "backrest" if scale == 500 else "backrest-shell"
    star_id = "base-bar" if scale == 500 else "base-star"

    def castor(k, z, wheel_id, with_boss=True):
        e.append(entry(f"castor-{k}-hood", f"castor-{k}", "body", z + (0.25 if view == "front" else 0), f"hood-{k}"))
        e.append(entry(wheel_id, f"castor-{k}", "body", z + 0.1, f"wheel-{k}", "ellipse" if view == "elevation" else "poly"))
        e.append(entry(f"castor-{k}-hub", f"castor-{k}", "detail", z + 0.2, f"rim-{k}", "ellipse" if view == "elevation" else "poly"))
        e.append(entry(f"arm-{k}-stem", f"arm-{k}", "detail", z + 0.3, f"neck-{k}"))

    def arm(k, z):
        e.append(entry(f"arm-{k}", f"arm-{k}", "body", z, f"arm-{k}"))
        e.append(entry(f"arm-{k}-yoke", f"arm-{k}", "body", z + 0.2, f"boss-{k}"))

    if view == "elevation":
        e.append(entry("armrest-strap-left", "seat", "body", 0.45, "strap-left"))
        e.append(entry("armrest-post-left", "seat", "body", 0.5, "upright-left"))
        e.append(entry("armrest-pad-left", "seat", "body", 0.6, "pad-left"))
        for k in CASTORS_ELEVATION:
            castor(k, 1.0, f"castor-{k}" if big else f"castor-{k}-wheel")
        if big:
            e.append(entry(star_id, "base", "body", 2.5, "star"))
        else:
            for k in CASTORS_ELEVATION:
                arm(k, 2.5)
            e.append(entry("hub", "base", "body", 3.0, "hub"))
        e.append(entry("column" if scale in (50, 500) else "column-rod", "column", "body", 3.2, "column"))
        e.append(entry("column-cone", "column", "body", 3.3, "cone"))
        e.append(entry("column-sleeve", "column-sleeve", "body", 3.4, "sleeve"))
        e.append(entry("column-collar", "column-sleeve", "detail", 3.6, "collar"))
        e.append(entry("backrest-strut-foot", "backrest", "body", 3.79, "foot"))
        e.append(entry("backrest-strut", "backrest", "body", 3.8, "strut"))
        e.append(entry("backrest-bracket", "backrest", "body", 3.85, "bracket"))
        e.append(entry("seat-mechanism", "seat", "body", 3.9, "mechanism"))
        e.append(entry("seat-knob", "seat", "detail", 3.92, "knob"))
        e.append(entry("seat-lever", "seat", "detail", 3.95, "lever"))
        e.append(entry(back_id, "backrest", "body", 4.0, "back"))
        e.append(entry("backrest-pad", "backrest", "detail", 4.05, "backpad"))
        e.append(entry("seat-shell", "seat", "detail", 4.1, "shell"))
        e.append(entry(seat_id, "seat", "body", 4.2, "cushion"))
        e.append(entry("armrest-strap-right", "seat", "body", 4.95, "strap-right"))
        e.append(entry("armrest-post-right", "seat", "body", 5.0, "upright-right"))
        e.append(entry("armrest-pad-right", "seat", "body", 5.1, "pad-right"))
    elif view == "front":
        e.append(entry("backrest-bracket", "backrest", "body", 1.0, "bracket"))
        e.append(entry("backrest-strut-foot", "backrest", "body", 1.05, "foot"))
        e.append(entry("backrest-strut", "backrest", "body", 1.1, "strut"))
        e.append(entry(back_id, "backrest", "body", 1.2, "back"))
        e.append(entry("backrest-pad", "backrest", "detail", 1.3, "backpad"))
        for k in BACK:
            castor(k, 2.0, f"castor-{k}-wheel")
        if scale < 500:
            for k in BACK:
                arm(k, 2.5)
        for k in SIDE:
            castor(k, 3.0, f"castor-{k}-wheel")
        if scale < 500:
            for k in SIDE:
                arm(k, 3.5)
        e.append(entry("column", "column", "body", 4.0, "column"))
        e.append(entry("column-cone", "column", "body", 4.1, "cone"))
        e.append(entry("column-sleeve", "column-sleeve", "body", 4.2, "sleeve"))
        e.append(entry("column-collar", "column-sleeve", "detail", 4.3, "collar"))
        e.append(entry("hub", "base", "body", 4.5, "star" if scale == 500 else "hub") if scale < 500 else entry(star_id, "base", "body", 4.5, "star"))
        e.append(entry("seat-mechanism", "seat", "body", 5.0, "mechanism"))
        e.append(entry("seat-lever", "seat", "detail", 5.1, "lever"))
        e.append(entry("seat-lock-lever", "seat", "detail", 5.1, "lock"))
        e.append(entry("seat-knob", "seat", "detail", 5.2, "knob"))
        e.append(entry("seat-shell", "seat", "detail", 5.4, "shell"))
        e.append(entry(seat_id, "seat", "body", 5.5, "cushion"))
        for side in ("left", "right"):
            e.append(entry(f"armrest-post-{side}", "seat", "body", 6.0, f"post-{side}"))
            e.append(entry(f"armrest-pad-{side}", "seat", "body", 6.1, f"pad-{side}"))
        castor("front", 7.0, "castor-front-wheel")
        if scale < 500:
            arm("front", 7.5)
    else:
        for k in ALL:
            e.append(entry(f"castor-{k}-wheel", f"castor-{k}", "body", 1.0, f"wheel-{k}"))
            e.append(entry(f"castor-{k}-hub", f"castor-{k}", "detail", 1.1, f"rim-{k}"))
            e.append(entry(f"castor-{k}-hood", f"castor-{k}", "body", 1.2, f"hood-{k}"))
        for k in ALL:
            arm(k, 2.5)
        e.append(entry("hub", "base", "body", 3.0, "hub"))
        e.append(entry("seat-mechanism", "seat", "body", 3.5, "mechanism"))
        e.append(entry("seat-knob", "seat", "detail", 3.6, "knob"))
        e.append(entry("seat-lever", "seat", "detail", 3.6, "lever"))
        e.append(entry("seat-lock-lever", "seat", "detail", 3.6, "lock"))
        e.append(entry("backrest-strut-foot", "backrest", "body", 3.79, "foot"))
        e.append(entry("backrest-strut", "backrest", "body", 3.8, "strut"))
        e.append(entry("seat-shell", "seat", "detail", 4.0, "shell"))
        e.append(entry(seat_id, "seat", "body", 4.1, "cushion"))
        for side in ("left", "right"):
            e.append(entry(f"armrest-post-{side}", "seat", "body", 3.9, f"post-{side}"))
            e.append(entry(f"armrest-bracket-{side}", "seat", "body", 5.05, f"cap-{side}"))
            e.append(entry(f"armrest-pad-{side}", "seat", "body", 5.1, f"pad-{side}"))
        e.append(entry("backrest-bracket", "backrest", "body", 6.0, "bracket"))
        e.append(entry(back_id, "backrest", "body", 6.1, "back"))
        e.append(entry("backrest-pad", "backrest", "detail", 6.2, "backpad"))
    return [x for x in e if x["group"] in g]


def r4(v):
    out = round(v, 5)
    return 0.0 if out == 0 else out


def local2(bone, x, y):
    p = K.to_local(bone, (x, y, K.FRAMES[bone][0][2]))
    return [r4(p[0]), r4(p[1])]


def stroke(shape_id, bone, role, z, tone, material, points, solid=None):
    out = {"id": shape_id, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "kind": "poly", "points": [[r4(a), r4(b)] for a, b in points], "closed": False, "region": REGION[material], "material": material}
    if solid:
        out["solid"] = solid
    return out


def details(scale, view):
    """🖋 The detail and texture strokes of the 1:50 drawing, placed by hand on the redrawn forms."""
    if scale != 50:
        return []
    d = []
    if view == "elevation":
        d.append(stroke("hub-ring", "base", "detail", 3.1, 0.3, "pa-plastic", [local2("base", -0.0415, 0.0635), local2("base", 0.0415, 0.0635)]))
        for k in CASTORS_ELEVATION:
            c = math.cos(math.radians(K.AZIMUTH[k]))
            d.append(stroke(f"arm-{k}-rib", f"arm-{k}", "detail", 2.6, 0.3, "pa-plastic", [local2(f"arm-{k}", 0.075 * c, 0.0905), local2(f"arm-{k}", 0.265 * c, 0.0752)]))
        d.append(stroke("column-groove", "column", "detail", 3.25, 0.4, "steel-painted", [local2("column", 0.006, 0.292), local2("column", 0.006, 0.312)]))
        wide = {"kind": "extruded", "sweep": "w", "z0": -0.236, "z1": 0.236}
        d.append(stroke("seat-piping", "seat", "detail", 4.3, 0.3, "fabric-upholstery", [local2("seat", 0.205, 0.4225), local2("seat", -0.185, 0.4245)], wide))
        d.append(stroke("seat-fabric-fold", "seat", "texture", 4.35, 0.3, "fabric-upholstery", [local2("seat", 0.10, 0.4325), local2("seat", -0.08, 0.4335)], {"kind": "extruded", "sweep": "w", "z0": -0.2, "z1": 0.2}))
        for side, z, zz in (("left", 0.65, (0.246, 0.329)), ("right", 5.2, (-0.329, -0.246))):
            d.append(stroke(f"armrest-line-{side}", "seat", "detail", z, 0.3, "pa-plastic", [local2("seat", -0.118, 0.6362), local2("seat", 0.088, 0.6355)], {"kind": "extruded", "sweep": "w", "z0": zz[0], "z1": zz[1]}))
        d.append(stroke("backrest-rim", "backrest", "detail", 4.02, 0.3, "pa-plastic", [local2("backrest", -0.2900, 0.720), local2("backrest", -0.3380, 0.950)], {"kind": "extruded", "sweep": "w", "z0": -0.2, "z1": 0.2}))
        d.append(stroke("backrest-texture", "backrest", "texture", 4.07, 0.3, "fabric-upholstery", [local2("backrest", -0.2610, 0.720), local2("backrest", -0.2960, 0.880)], {"kind": "extruded", "sweep": "w", "z0": -0.19, "z1": 0.19}))
    elif view == "front":
        d.append(stroke("seat-piping-ring", "seat", "detail", 5.6, 0.3, "fabric-upholstery", [[0.0815, -0.205], [0.0815, 0.205]], {"kind": "extruded", "sweep": "v", "z0": -0.2405, "z1": -0.24}))
    else:
        ring = [(-0.196, -0.200), (0.190, -0.214), (0.214, -0.190), (0.214, 0.190), (0.190, 0.214), (-0.196, 0.200)]
        d.append(stroke("seat-piping-ring", "seat", "detail", 4.2, 0.3, "fabric-upholstery", ring))
        d.append(stroke("seat-fabric-fold", "seat", "texture", 4.3, 0.3, "fabric-upholstery", [(-0.10, -0.10), (-0.03, -0.092), (0.03, -0.108), (0.09, -0.10)]))
    return d


def requests(asset):
    out = []
    for scale in (50, 100, 200, 500):
        body = asset["volumes"][str(scale)]["body"]
        g = groups([s["id"] for s in body])
        for view in ("elevation", "front", "plan"):
            for e in plan_of(scale, view, g):
                out.append({"key": f"{scale}|{view}|{e['id']}", "scale": scale, "view": view, "solids": g[e["group"]], "tol": TOL[scale]})
        out.append({"key": f"{scale}|box", "scale": scale, "view": "elevation", "solids": [s["id"] for s in body], "tol": 0.01})
    return out


def shape_of(e, view, scale, hit, body_by_id, group_ids):
    first = body_by_id[group_ids[0]]
    material = first["material"]
    tone = first["tone"]
    ring, box, rings = hit["ring"], hit["box"], hit.get("rings") or [hit["ring"]]
    head = {"id": e["id"], "bone": e["bone"], "role": e["role"], "z": e["z"], "filled": True, "tone": tone}
    tail = {"region": REGION[material], "material": material}
    bone = e["bone"]
    (ox, oy, oz), deg = K.FRAMES[bone]
    if e["prim"] == "ellipse":
        cx, cy = (box[0] + box[3]) / 2, (box[1] + box[4]) / 2
        c = local2(bone, cx, cy)
        return {**head, "kind": "ellipse", "cx": c[0], "cy": c[1], "rx": r4((box[3] - box[0]) / 2), "ry": r4((box[4] - box[1]) / 2), **tail}
    if view == "elevation":
        shape = {**head, **geometry_of([[local2(bone, x, y) for x, y in r] for r in rings]), **tail}
        if bone in SEAT_TREE:
            shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": r4(box[2]), "z1": r4(box[5])}
        return shape
    if view == "front" and bone in SEAT_TREE:
        a = math.radians(deg)
        uy, vx, vy = math.sin(a), -math.sin(a), math.cos(a)
        xm, ym = (box[0] + box[3]) / 2, (box[1] + box[4]) / 2
        if abs(vy) < 1e-9:
            v0, v1 = sorted(((box[0] - ox) * vx, (box[3] - ox) * vx))
            local = [[[(y - oy) / uy, z - oz] for z, y in r] for r in rings]
        else:
            vm = (xm - ox) * vx + (ym - oy) * vy
            v0, v1 = vm - 0.010, vm + 0.010
            local = [[[(y - oy - vm * vy) / uy, z - oz] for z, y in r] for r in rings]
        return {**head, **geometry_of(local), **tail, "solid": {"kind": "extruded", "sweep": "v", "z0": r4(v0), "z1": r4(v1)}}
    if view == "plan" and bone == "backrest":
        a = math.radians(deg)
        ux, vx, vy = math.cos(a), -math.sin(a), math.cos(a)
        vm = ((box[0] + box[3]) / 2 - ox) * vx + ((box[1] + box[4]) / 2 - oy) * vy
        return {**head, **geometry_of([[[(x - ox - vm * vx) / ux, z - oz] for x, z in r] for r in rings]), **tail, "solid": {"kind": "extruded", "sweep": "v", "z0": r4(vm), "z1": r4(vm + 0.0005)}}
    return {**head, **geometry_of(rings), **tail}


def geometry_of(rings):
    """🔗 One ring is a polygon; several rings (two wheels, a part with an opening) are the subpaths of one path."""
    if len(rings) == 1:
        return {"kind": "poly", "points": [[r4(a), r4(b)] for a, b in rings[0]], "closed": True}
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r4(a)} {r4(b)}" for a, b in ring) + " Z" for ring in rings)}


def apply(asset, out_dir):
    os.makedirs(out_dir, exist_ok=True)
    reqs = requests(asset)
    json.dump(reqs, open(os.path.join(out_dir, "outline-requests.json"), "w", encoding="utf-8"), ensure_ascii=False)
    path = os.path.join(out_dir, "outlines.json")
    if not os.path.exists(path):
        return
    hits = json.load(open(path, encoding="utf-8"))
    for scale in (50, 100, 200, 500):
        body = asset["volumes"][str(scale)]["body"]
        by_id = {s["id"]: s for s in body}
        g = groups(list(by_id))
        geometry = {}
        for view in ("elevation", "front", "plan"):
            shapes = []
            for e in plan_of(scale, view, g):
                hit = hits.get(f"{scale}|{view}|{e['id']}")
                if hit is None or len(hit["ring"]) < 3:
                    continue
                shapes.append(shape_of(e, view, scale, hit, by_id, g[e["group"]]))
            shapes += details(scale, view)
            order = sorted(range(len(shapes)), key=lambda i: (shapes[i]["z"], i))
            geometry[view] = [shapes[i] for i in order]
        asset["representations"][str(scale)]["geometry"] = geometry
    box = hits["50|box"]["box"]
    asset["dimensions"] = {"heightM": round(box[4], 2), "widthM": round(box[3] - box[0], 2), "depthM": round(box[5] - box[2], 2), "massKg": 15.4}
    print("box 1:50", box, asset["dimensions"])
