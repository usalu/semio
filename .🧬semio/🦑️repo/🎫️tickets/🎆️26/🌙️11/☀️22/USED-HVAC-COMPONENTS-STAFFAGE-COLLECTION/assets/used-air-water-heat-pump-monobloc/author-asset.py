"""🌀 Author script of the used air-to-water monobloc heat pump: states every hand-chosen number and writes `🔣️.json` next to it.

World frame of the staffage (x ahead = the fan face, y up, z left), stored pose = rest pose. Every bone rests with angle 0, so a
bone-local point is the world point minus the bone origin, and every solid and every 2D shape is written from one parts table per scale.

@see ../../brief.md
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

BONES = {"yard": (0.0, 0.0, 0.0), "pallet": (-0.40, 0.0, 0.0), "console": (-0.30, 0.144, 0.0), "unit": (-0.30, 0.444, 0.0), "fan": (0.15, 1.144, -0.17)}
FAN_Y, FAN_Z = 1.144, -0.17
PALLET_X_BOTTOM = [(-0.40, -0.30), (-0.0725, 0.0725), (0.30, 0.40)]
PALLET_Z_ROWS = [(-0.60, -0.455), (-0.0725, 0.0725), (0.455, 0.60)]
PALLET_X_TOP = [(-0.40, -0.255), (-0.21375, -0.11375), (-0.0725, 0.0725), (0.11375, 0.21375), (0.255, 0.40)]
STAND_Z = [-0.40, 0.40]


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯️ Rounds a number to micrometres for the document."""
    return round(v + 0.0, 6)


def loc(bone, x, y, z):
    """📍️ The bone-local point of a world point at rest."""
    o = BONES[bone]
    return [r6(x - o[0]), r6(y - o[1]), r6(z - o[2])]


def base(sid, bone, mat, role, tone, **extra):
    """🧱️ The common members of a solid."""
    d = {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone}
    d.update(extra)
    return d


def box(sid, bone, mat, role, tone, x0, x1, y0, y1, z0, z1, **extra):
    """📦️ A box given by its world bounds at rest."""
    c = loc(bone, (x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2)
    d = base(sid, bone, mat, role, tone, **extra)
    d.update({"kind": "box", "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], "transform": {"translation": c}})
    return d


def cyl(sid, bone, mat, role, tone, axis, a0, a1, r, p, q, **extra):
    """🛢️ A solid cylinder along world `axis` from `a0` to `a1` of radius `r` through the cross position (p, q)."""
    d = base(sid, bone, mat, role, tone, **extra)
    if axis == "x":
        o = loc(bone, 0, p, q)
        o[0] = 0.0
        prof = [[r6(a0 - BONES[bone][0]), 0], [r6(a0 - BONES[bone][0]), r], [r6(a1 - BONES[bone][0]), r], [r6(a1 - BONES[bone][0]), 0]]
        vec = [1, 0, 0]
    else:
        o = loc(bone, p, 0, q)
        o[1] = 0.0
        prof = [[r6(a0 - BONES[bone][1]), 0], [r6(a0 - BONES[bone][1]), r], [r6(a1 - BONES[bone][1]), r], [r6(a1 - BONES[bone][1]), 0]]
        vec = [0, 1, 0]
    d.update({"kind": "revolve", "profile": prof, "axis": vec, "origin": o})
    return d


def ring(sid, bone, mat, role, tone, a0, a1, r0, r1, **extra):
    """⭕️ A ring about the fan axis (world x through the fan centre) as a closed-loop revolve."""
    d = base(sid, bone, mat, role, tone, **extra)
    ox = BONES[bone][0]
    o = loc(bone, 0, FAN_Y, FAN_Z)
    o[0] = 0.0
    u0, u1 = r6(a0 - ox), r6(a1 - ox)
    d.update({"kind": "revolve", "profile": [[u0, r0], [u0, r1], [u1, r1], [u1, r0], [u0, r0]], "axis": [1, 0, 0], "origin": o})
    return d


def disc_profile(a0, a1, r):
    """💿️ The open revolve profile of a disc."""
    return [[a0, 0], [a0, r], [a1, r], [a1, 0]]


def ext(sid, bone, mat, role, tone, plane, profile, frm, to, holes=None, **extra):
    """🧊️ An extrude in bone-local profile coordinates."""
    d = base(sid, bone, mat, role, tone, **extra)
    d.update({"kind": "extrude", "plane": plane, "profile": [[r6(a), r6(b)] for a, b in profile], "from": r6(frm), "to": r6(to)})
    if holes:
        d["holes"] = [[[r6(a), r6(b)] for a, b in h] for h in holes]
    return d


def circle_pts(ca, cb, r, n=48):
    """🔵️ A polygon of `n` points on a circle."""
    return [(ca + r * math.cos(2 * math.pi * k / n), cb + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def ref(sid, of, **extra):
    """🔗️ A reference to a body solid."""
    d = {"id": sid, "of": of}
    d.update(extra)
    return d


def shp(sid, bone, role, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None):
    """✏️ A 2D shape; its paint order `z` is set by its place in the view list."""
    d = {"id": sid, "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone}
    if region:
        d["region"] = region
    if regions:
        d["regions"] = regions
    if material:
        d["material"] = material
    if hatch:
        d["hatch"] = hatch
    if pen:
        d["penMm"] = pen
    d.update(geom)
    return d


def erect(bone, x0, x1, y0, y1):
    """📐️ An elevation rectangle from world bounds."""
    o = BONES[bone]
    return {"kind": "rect", "x": r6(x0 - o[0]), "y": r6(y0 - o[1]), "w": r6(x1 - x0), "h": r6(y1 - y0)}


def wrect(a0, a1, b0, b1):
    """📐️ A front (z, y) or plan (x, z) rectangle in world rest coordinates."""
    return {"kind": "rect", "x": r6(a0), "y": r6(b0), "w": r6(a1 - a0), "h": r6(b1 - b0)}


def epoly(bone, pts, closed=True):
    """🔷️ An elevation polygon from world points."""
    o = BONES[bone]
    return {"kind": "poly", "points": [[r6(x - o[0]), r6(y - o[1])] for x, y in pts], "closed": closed}


def wpoly(pts, closed=True):
    """🔷️ A front or plan polygon in world rest coordinates."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}


def ell(cx, cy, r):
    """⭕️ A circle in world rest coordinates."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(r), "ry": r6(r)}


def ring_d(cx, cy, r):
    """〰️ The path of one circle ring."""
    return f"M {r6(cx + r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx - r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx + r)} {r6(cy)} Z"


def rect_d(a0, a1, b0, b1):
    """〰️ The path of one rectangle ring."""
    return f"M {r6(a0)} {r6(b0)} L {r6(a1)} {r6(b0)} L {r6(a1)} {r6(b1)} L {r6(a0)} {r6(b1)} Z"


def epath_poly(bone, outer, hole):
    """〰️ An elevation path of an outer polygon with one polygon hole, from world points."""
    o = BONES[bone]
    ring_of = lambda pts: "M " + " L ".join(f"{r6(x - o[0])} {r6(y - o[1])}" for x, y in pts) + " Z"
    return {"kind": "path", "d": ring_of(outer) + " " + ring_of(hole)}


def number(lst):
    """🔢️ Sets the paint order of a view list: later is nearer."""
    for k, s in enumerate(lst):
        s["z"] = k + 1
    return lst


def stand_profile(top):
    """🦿️ The side profile of one console stand in console-local (u, v): foot plate and trapezoid web up to `top`."""
    return [(0, 0), (0.60, 0), (0.60, 0.04), (0.54, 0.04), (0.50, top), (0.10, top), (0.06, 0.04), (0, 0.04)]


STAND_HOLE = [(0.16, 0.08), (0.44, 0.08), (0.42, 0.22), (0.18, 0.22)]


def world_stand(pts):
    """🌍️ Console-local stand points in world (x, y)."""
    return [(x + BONES["console"][0], y + BONES["console"][1]) for x, y in pts]


def blade_polys():
    """🌀️ The front outlines of the three pitched blades: radial 55..255 mm, chord 110 mm at 25 degrees (projected 99.7 mm)."""
    half = 0.055 * math.cos(math.radians(25)) + 0.0025 * math.sin(math.radians(25))
    out = []
    for k in range(3):
        t = 2 * math.pi * k / 3
        pts = []
        for v, w in [(0.055, -half), (0.255, -half * 0.8), (0.255, half * 0.8), (0.055, half)]:
            vv = v * math.cos(t) - w * math.sin(t)
            ww = v * math.sin(t) + w * math.cos(t)
            pts.append((FAN_Z + ww, FAN_Y + vv))
        out.append(pts)
    return out


def blade_solid(sid, thick):
    """🌀️ The three pitched fan blades as one repeated sheet."""
    q = [0, r6(math.sin(math.radians(12.5))), 0, r6(math.cos(math.radians(12.5)))]
    d = base(sid, "fan", "pp-gf", "detail", 0.2, sheet=True)
    d.update({"kind": "box", "size": [thick, 0.2, 0.11], "transform": {"rotation": q, "translation": [0.035, 0.155, 0]}, "repeat": {"count": 3, "rotationAxis": [1, 0, 0], "rotationRad": r6(2 * math.pi / 3), "origin": [0, 0, 0]}})
    return d


def hub_solid(sid):
    """🔩️ The fan hub on the motor shaft."""
    d = base(sid, "fan", "pp-gf", "detail", 0.15)
    d.update({"kind": "revolve", "profile": [[0, 0], [0, 0.06], [0.05, 0.06], [0.07, 0.035], [0.07, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0]})
    return d


def front_panel(sid, x0, hole_r, region_tone=0.62):
    """🪟️ The front panel of the fan section with the fan opening, from `x0` to the front face x 0.26."""
    prof = [(0.06, -0.48), (1.24, -0.48), (1.24, 0.56), (0.06, 0.56)]
    hole = [(0.70 + b, FAN_Z + a) for a, b in circle_pts(0, 0, hole_r)]
    return ext(sid, "unit", "powder-coat-faded", "body", region_tone, "yz", prof, x0 + 0.30, 0.56, holes=[hole])


COIL_L = [(0.20, 0.0), (0.20, 0.10), (-0.48, 0.10), (-0.48, 0.50), (-0.56, 0.50), (-0.56, 0.0)]
COIL_L_100 = [(0.20, 0.0), (0.20, 0.10), (-0.48, 0.10), (-0.48, 0.56), (-0.56, 0.56), (-0.56, 0.0)]


def scale50():
    """🔍️ The 1:50 model: every board of the pallet, the console with lightening holes, mounts and cut anchor bolts, the casing panel by panel, the grille with its wires, motor, hub and blades, capped stubs, spigot, type plate and the crushed fin patch."""
    body, th, st = [], [], []
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        body.append(box(f"bottom-board-{k + 1}", "pallet", "pallet-softwood", "body", 0.5, a, b, 0, 0.022, -0.60, 0.60))
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        s = box(f"blocks-{k + 1}", "pallet", "pallet-softwood", "body", 0.45, a, b, 0.022, 0.100, -0.60, -0.455)
        s["repeat"] = {"count": 3, "translation": [0, 0, 0.5275]}
        body.append(s)
    for k, (a, b) in enumerate(PALLET_Z_ROWS):
        body.append(box(f"stringer-{k + 1}", "pallet", "pallet-softwood", "body", 0.5, -0.40, 0.40, 0.100, 0.122, a, b))
    for k, (a, b) in enumerate(PALLET_X_TOP):
        body.append(box(f"top-board-{k + 1}", "pallet", "pallet-softwood", "body", 0.55, a, b, 0.122, 0.144, -0.60, 0.60))
    for side, z in (("r", -0.40), ("l", 0.40)):
        body.append(ext(f"stand-{side}", "console", "galvanised-steel", "body", 0.4, "xy", stand_profile(0.27), z - 0.03, z + 0.03, holes=[STAND_HOLE]))
        m = cyl(f"mounts-{side}", "console", "epdm-rubber", "detail", 0.15, "y", 0.414, 0.444, 0.025, -0.18, z)
        m["repeat"] = {"count": 2, "translation": [0.36, 0, 0]}
        body.append(m)
        b = cyl(f"anchor-bolts-{side}", "console", "galvanised-steel", "detail", 0.3, "y", 0.184, 0.209, 0.006, -0.27, z)
        b["repeat"] = {"count": 2, "translation": [0.54, 0, 0]}
        body.append(b)
    body += [
        box("base-pan", "unit", "galvanised-steel", "body", 0.35, -0.30, 0.26, 0.444, 0.504, -0.56, 0.56),
        box("compressor-casing", "unit", "powder-coat-faded", "body", 0.6, -0.30, 0.25, 0.504, 1.684, 0.20, 0.56),
        ext("coil", "unit", "aluminium-fins", "detail", 0.3, "zx", COIL_L, 0.06, 1.24),
        box("front-post", "unit", "powder-coat-faded", "body", 0.6, 0.20, 0.26, 0.504, 1.684, -0.56, -0.48),
        front_panel("front-panel", 0.25, 0.30),
        box("top-cover", "unit", "powder-coat-faded", "body", 0.55, -0.31, 0.27, 1.684, 1.744, -0.57, 0.57),
        ring("grille-ring", "unit", "coated-wire", "body", 0.2, 0.26, 0.30, 0.30, 0.33),
    ]
    for k, r in enumerate((0.11, 0.18, 0.25)):
        body.append(ring(f"grille-hoop-{k + 1}", "unit", "coated-wire", "detail", 0.2, 0.296, 0.30, r - 0.002, r + 0.002, slender=True))
    sp = base("grille-spokes", "unit", "coated-wire", "detail", 0.2, slender=True)
    sp.update({"kind": "capsule", "p0": loc("unit", 0.298, FAN_Y + 0.05, FAN_Z), "p1": loc("unit", 0.298, FAN_Y + 0.30, FAN_Z), "r0": 0.002, "r1": 0.002, "repeat": {"count": 8, "rotationAxis": [1, 0, 0], "rotationRad": r6(math.pi / 4), "origin": loc("unit", 0.298, FAN_Y, FAN_Z)}})
    body.append(sp)
    cap = base("grille-cap", "unit", "coated-wire", "detail", 0.2)
    cap.update({"kind": "revolve", "profile": disc_profile(0.58, 0.60, 0.05), "axis": [1, 0, 0], "origin": loc("unit", 0.30, FAN_Y, FAN_Z)})
    cap["origin"][0] = 0.0
    body.append(cap)
    body += [
        cyl("fan-motor", "unit", "painted-steel", "detail", 0.1, "x", 0.05, 0.15, 0.09, FAN_Y, FAN_Z),
        box("motor-bracket", "unit", "galvanised-steel", "detail", 0.35, 0.03, 0.05, 0.504, 1.684, -0.19, -0.15),
        cyl("water-stub-flow", "unit", "copper-oxidised", "detail", 0.45, "x", -0.36, -0.30, 0.021, 0.624, 0.46),
        cyl("water-stub-return", "unit", "copper-oxidised", "detail", 0.45, "x", -0.36, -0.30, 0.021, 0.624, 0.32),
        cyl("cap-flow", "unit", "pe-cap", "detail", 0.25, "x", -0.385, -0.36, 0.026, 0.624, 0.46),
        cyl("cap-return", "unit", "pe-cap", "detail", 0.25, "x", -0.385, -0.36, 0.026, 0.624, 0.32),
        cyl("condensate-spigot", "unit", "pe-cap", "detail", 0.25, "y", 0.414, 0.444, 0.012, 0.0, -0.20),
        box("type-plate", "unit", "aluminium-fins", "texture", 0.8, 0.26, 0.262, 1.40, 1.48, 0.30, 0.42, texture=True),
        box("crushed-fins", "unit", "aluminium-fins", "texture", 0.15, -0.10, 0.06, 0.62, 0.84, -0.562, -0.560, texture=True),
        hub_solid("fan-hub"),
        blade_solid("fan-blades", 0.005),
    ]
    regions = {"bottom-board": "pallet", "blocks": "pallet", "stringer": "pallet", "top-board": "pallet", "stand": "console", "mounts": "console", "anchor-bolts": "console", "base-pan": "base-pan", "compressor-casing": "compressor-compartment", "type-plate": "casing", "coil": "coil", "crushed-fins": "coil", "front-post": "casing", "front-panel": "casing", "motor-bracket": "casing", "top-cover": "top-cover", "grille": "fan", "fan-motor": "fan", "fan-hub": "fan", "fan-blades": "fan", "water-stub": "water-connections", "cap": "water-connections", "condensate-spigot": "base-pan"}
    th = thermal_refs(body, regions)
    th += emitters(0.008, 0.414)
    st = [ref(f"m-{s['id']}", s["id"], member="pallet", part="member") for s in body if s["bone"] == "pallet"]
    st += [ref("m-stand-r", "stand-r", member="console", part="member"), ref("m-stand-l", "stand-l", member="console", part="member")]
    st += [ref("j-mounts-r", "mounts-r", member="console", part="joint"), ref("j-mounts-l", "mounts-l", member="console", part="joint")]
    st += posts(0.04)
    st += [ref("m-base-pan", "base-pan", member="unit", part="member"), ref("m-top-cover", "top-cover", member="unit", part="member"), ref("m-motor-bracket", "motor-bracket", member="unit", part="member"), ref("m-fan-motor", "fan-motor", member="unit", part="member")]
    st += [ref("m-fan-hub", "fan-hub", member="fan", part="member"), bearing(0.012)]
    st += masses(0.03)
    return body, th, st


def thermal_refs(body, regions):
    """🌡️ The thermal references of a body list by the region of each solid's id prefix."""
    out = []
    for s in body:
        key = next(k for k in sorted(regions, key=len, reverse=True) if s["id"].startswith(k))
        out.append(ref(f"th-{s['id']}", s["id"], region=regions[key]))
    return out


def emitters(t, drip_top):
    """💨️ The outlet patches of the three emitters, `t` thick, the drip outlet ending at `drip_top`."""
    jet = base("outlet-cold-air-jet", "unit", "coated-wire", "detail", 0.2, tags=["cold-air-jet"], region="fan")
    jet.update({"kind": "revolve", "profile": disc_profile(r6(0.60 - t), 0.60, 0.28), "axis": [1, 0, 0], "origin": [0, 0.70, FAN_Z]})
    plume = box("outlet-defrost-plume", "unit", "aluminium-fins", "detail", 0.3, -0.30 - t, -0.30, 1.00, 1.60, -0.45, 0.05, tags=["defrost-plume"], region="coil")
    drip_r = max(0.012, t / 2)
    drip = cyl("outlet-condensate-drip", "unit", "pe-cap", "detail", 0.25, "y", drip_top - t, drip_top, drip_r, 0.0, -0.20, tags=["condensate-drip"], region="base-pan")
    return [jet, plume, drip]


def posts(s):
    """🏛️ The four corner posts of the casing frame, `s` square."""
    out = []
    for nm, (x0, x1), (z0, z1) in (("rear-right", (-0.30, -0.30 + s), (-0.56, -0.56 + s)), ("rear-left", (-0.30, -0.30 + s), (0.56 - s, 0.56)), ("front-right", (0.26 - s, 0.26), (-0.56, -0.56 + s)), ("front-left", (0.26 - s, 0.26), (0.56 - s, 0.56))):
        out.append(box(f"post-{nm}", "unit", "galvanised-steel", "body", 0.4, x0, x1, 0.504, 1.684, z0, z1, member="unit", part="member"))
    return out


def bearing(r):
    """⚙️ The fan bearing between motor and hub."""
    d = base("j-fan-bearing", "fan", "painted-steel", "body", 0.2, member="fan", part="joint")
    d.update({"kind": "revolve", "profile": [[-0.02, 0], [-0.02, r], [0.01, r], [0.01, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0]})
    return d


def masses(r):
    """⚖️ The mass markers at the centres of mass of the three loaded bones."""
    out = []
    for bone, (x, y, w) in (("pallet", (0.40, 0.075, 0)), ("console", (0.30, 0.05, -0.40)), ("unit", (0.26, 0.52, 0))):
        d = base(f"mass-{bone}", bone, "galvanised-steel", "body", 0.2, member=bone, part="mass")
        d.update({"kind": "ellipsoid", "radii": [r, r, r], "transform": {"translation": [x, y, w]}})
        out.append(d)
    return out


def scale100():
    """🔍️ The 1:100 model: stringer and blocks of each row merged into one runner, console stands without mounts and bolts reaching the base pan, caps merged into the stubs, no grille wires, type plate or fin patch."""
    body = []
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        body.append(box(f"bottom-board-{k + 1}", "pallet", "pallet-softwood", "body", 0.5, a, b, 0, 0.022, -0.60, 0.60))
    runner = [(0, 0.122), (0.80, 0.122), (0.80, 0.022), (0.70, 0.022), (0.70, 0.100), (0.4725, 0.100), (0.4725, 0.022), (0.3275, 0.022), (0.3275, 0.100), (0.10, 0.100), (0.10, 0.022), (0, 0.022)]
    for k, (a, b) in enumerate(PALLET_Z_ROWS):
        body.append(ext(f"runner-{k + 1}", "pallet", "pallet-softwood", "body", 0.48, "xy", runner, a, b))
    for k, (a, b) in enumerate(PALLET_X_TOP):
        body.append(box(f"top-board-{k + 1}", "pallet", "pallet-softwood", "body", 0.55, a, b, 0.122, 0.144, -0.60, 0.60))
    for side, z in (("r", -0.40), ("l", 0.40)):
        body.append(ext(f"stand-{side}", "console", "galvanised-steel", "body", 0.4, "xy", stand_profile(0.30), z - 0.03, z + 0.03, holes=[STAND_HOLE]))
    body += [
        box("base-pan", "unit", "galvanised-steel", "body", 0.35, -0.30, 0.26, 0.444, 0.504, -0.56, 0.56),
        box("compressor-casing", "unit", "powder-coat-faded", "body", 0.6, -0.30, 0.25, 0.504, 1.684, 0.20, 0.56),
        ext("coil", "unit", "aluminium-fins", "detail", 0.3, "zx", COIL_L, 0.06, 1.24),
        box("front-post", "unit", "powder-coat-faded", "body", 0.6, 0.20, 0.26, 0.504, 1.684, -0.56, -0.48),
        front_panel("front-panel", 0.245, 0.30),
        box("top-cover", "unit", "powder-coat-faded", "body", 0.55, -0.31, 0.27, 1.684, 1.744, -0.57, 0.57),
        ring("grille-ring", "unit", "coated-wire", "body", 0.2, 0.26, 0.30, 0.30, 0.33),
        cyl("fan-motor", "unit", "painted-steel", "detail", 0.1, "x", 0.05, 0.15, 0.09, FAN_Y, FAN_Z),
        box("motor-bracket", "unit", "galvanised-steel", "detail", 0.35, 0.03, 0.05, 0.504, 1.684, -0.19, -0.15),
        cyl("water-stub-flow", "unit", "copper-oxidised", "detail", 0.45, "x", -0.385, -0.30, 0.026, 0.624, 0.46),
        cyl("water-stub-return", "unit", "copper-oxidised", "detail", 0.45, "x", -0.385, -0.30, 0.026, 0.624, 0.32),
        cyl("condensate-spigot", "unit", "pe-cap", "detail", 0.25, "y", 0.414, 0.444, 0.012, 0.0, -0.20),
        hub_solid("fan-hub"),
        blade_solid("fan-blades", 0.005),
    ]
    regions = {"bottom-board": "pallet", "runner": "pallet", "top-board": "pallet", "stand": "console", "base-pan": "base-pan", "compressor-casing": "compressor-compartment", "coil": "coil", "front-post": "casing", "front-panel": "casing", "motor-bracket": "casing", "top-cover": "top-cover", "grille": "fan", "fan-motor": "fan", "fan-hub": "fan", "fan-blades": "fan", "water-stub": "water-connections", "condensate-spigot": "base-pan"}
    th = thermal_refs(body, regions) + emitters(0.015, 0.414)
    st = [ref(f"m-{s['id']}", s["id"], member="pallet", part="member") for s in body if s["bone"] == "pallet"]
    st += [ref("m-stand-r", "stand-r", member="console", part="member"), ref("m-stand-l", "stand-l", member="console", part="member")]
    for side, z in (("r", -0.40), ("l", 0.40)):
        st.append(box(f"j-mount-pads-{side}", "console", "epdm-rubber", "body", 0.15, -0.205, -0.155, 0.429, 0.444, z - 0.025, z + 0.025, member="console", part="joint", repeat={"count": 2, "translation": [0.36, 0, 0]}))
    st += posts(0.04)
    st += [ref("m-base-pan", "base-pan", member="unit", part="member"), ref("m-top-cover", "top-cover", member="unit", part="member"), ref("m-motor-bracket", "motor-bracket", member="unit", part="member"), ref("m-fan-motor", "fan-motor", member="unit", part="member")]
    st += [ref("m-fan-hub", "fan-hub", member="fan", part="member"), bearing(0.015)]
    st += masses(0.03)
    return body, th, st


def scale200():
    """🔍️ The 1:200 model: pallet as deck and three bearers, solid trapezoid stands, the casing as one box under its cover, the grille as a disc, the stubs as one block, no fan, motor or spigot."""
    body = [box("pallet-deck", "pallet", "pallet-softwood", "body", 0.55, -0.40, 0.40, 0.100, 0.144, -0.60, 0.60)]
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        body.append(box(f"bearer-{k + 1}", "pallet", "pallet-softwood", "body", 0.48, a, b, 0, 0.100, -0.60, 0.60))
    for side, z in (("r", -0.40), ("l", 0.40)):
        body.append(ext(f"stand-{side}", "console", "galvanised-steel", "body", 0.4, "xy", stand_profile(0.30), z - 0.03, z + 0.03))
    body += [
        box("casing", "unit", "powder-coat-faded", "body", 0.6, -0.30, 0.26, 0.444, 1.684, -0.56, 0.56),
        box("top-cover", "unit", "powder-coat-faded", "body", 0.55, -0.31, 0.27, 1.684, 1.744, -0.57, 0.57),
        cyl("grille-disc", "unit", "coated-wire", "body", 0.2, "x", 0.26, 0.30, 0.33, FAN_Y, FAN_Z),
        box("water-stubs", "unit", "copper-oxidised", "detail", 0.45, -0.385, -0.30, 0.594, 0.654, 0.29, 0.49),
    ]
    th = [ref("th-pallet-deck", "pallet-deck", region="pallet")] + [ref(f"th-bearer-{k + 1}", f"bearer-{k + 1}", region="pallet") for k in range(3)]
    th += [ref("th-stand-r", "stand-r", region="console"), ref("th-stand-l", "stand-l", region="console")]
    th += [
        box("th-base-pan", "unit", "galvanised-steel", "body", 0.35, -0.30, 0.26, 0.444, 0.504, -0.56, 0.56, region="base-pan"),
        box("th-compressor-casing", "unit", "powder-coat-faded", "body", 0.6, -0.30, 0.23, 0.504, 1.684, 0.20, 0.56, region="compressor-compartment"),
        ext("th-coil", "unit", "aluminium-fins", "detail", 0.3, "zx", COIL_L_100, 0.06, 1.24, region="coil"),
        ext("th-front-panel", "unit", "powder-coat-faded", "body", 0.6, "yz", [(0.06, -0.48), (1.24, -0.48), (1.24, 0.56), (0.06, 0.56)], 0.53, 0.56, holes=[[(0.70 + b, FAN_Z + a) for a, b in circle_pts(0, 0, 0.27)]], region="casing"),
        cyl("th-fan", "unit", "painted-steel", "detail", 0.1, "x", 0.05, 0.22, 0.09, FAN_Y, FAN_Z, region="fan"),
        ref("th-top-cover", "top-cover", region="top-cover"),
        ring("th-grille", "unit", "coated-wire", "body", 0.2, 0.26, 0.30, 0.27, 0.33, region="fan"),
        ref("th-water-stubs", "water-stubs", region="water-connections"),
    ]
    th += emitters(0.03, 0.444)
    st = [ref("m-pallet-deck", "pallet-deck", member="pallet", part="member")] + [ref(f"m-bearer-{k + 1}", f"bearer-{k + 1}", member="pallet", part="member") for k in range(3)]
    st += [ref("m-stand-r", "stand-r", member="console", part="member"), ref("m-stand-l", "stand-l", member="console", part="member")]
    st += posts(0.05)
    st += [box("m-base-frame", "unit", "galvanised-steel", "body", 0.35, -0.30, 0.26, 0.444, 0.504, -0.56, 0.56, member="unit", part="member"), box("m-top-frame", "unit", "galvanised-steel", "body", 0.35, -0.30, 0.26, 1.684, 1.744, -0.56, 0.56, member="unit", part="member")]
    st += masses(0.03)
    return body, th, st


def scale500():
    """🔍️ The 1:500 model: pallet as one slab, two 75 mm stands, the unit as one block from the base pan to the cover."""
    body = [box("pallet", "pallet", "pallet-softwood", "body", 0.5, -0.40, 0.40, 0, 0.144, -0.60, 0.60)]
    for side, z in (("r", -0.40), ("l", 0.40)):
        body.append(box(f"stand-{side}", "console", "galvanised-steel", "body", 0.4, -0.30, 0.30, 0.144, 0.444, z - 0.0375, z + 0.0375))
    body.append(box("unit-block", "unit", "powder-coat-faded", "body", 0.6, -0.30, 0.26, 0.444, 1.744, -0.56, 0.56))
    th = [ref("th-pallet", "pallet", region="pallet"), ref("th-stand-r", "stand-r", region="console"), ref("th-stand-l", "stand-l", region="console")]
    th += [
        box("th-base-pan", "unit", "galvanised-steel", "body", 0.35, -0.30, 0.26, 0.444, 0.519, -0.56, 0.56, region="base-pan"),
        box("th-compressor-casing", "unit", "powder-coat-faded", "body", 0.6, -0.30, 0.185, 0.519, 1.669, 0.18, 0.56, region="compressor-compartment"),
        ext("th-coil", "unit", "aluminium-fins", "detail", 0.3, "zx", [(0.18, 0.0), (0.18, 0.10), (-0.48, 0.10), (-0.48, 0.56), (-0.56, 0.56), (-0.56, 0.0)], 0.075, 1.225, region="coil"),
        ext("th-front-panel", "unit", "powder-coat-faded", "body", 0.6, "yz", [(0.075, -0.48), (1.225, -0.48), (1.225, 0.56), (0.075, 0.56)], 0.485, 0.56, holes=[[(0.70 + b, FAN_Z + a) for a, b in circle_pts(0, 0, 0.30)]], region="casing"),
        ring("th-grille", "unit", "coated-wire", "body", 0.2, 0.185, 0.26, 0.22, 0.30, region="fan"),
        cyl("th-fan", "unit", "painted-steel", "detail", 0.1, "x", 0.035, 0.185, 0.0375, FAN_Y, FAN_Z, region="fan"),
        box("th-top-cover", "unit", "powder-coat-faded", "body", 0.55, -0.30, 0.26, 1.669, 1.744, -0.56, 0.56, region="top-cover"),
        box("th-water-stubs", "unit", "copper-oxidised", "detail", 0.45, -0.375, -0.30, 0.5865, 0.6615, 0.29, 0.49, region="water-connections"),
    ]
    th += emitters(0.075, 0.444)
    st = [ref("m-pallet", "pallet", member="pallet", part="member"), ref("m-stand-r", "stand-r", member="console", part="member"), ref("m-stand-l", "stand-l", member="console", part="member")]
    st += posts(0.08)
    st += [box("m-base-frame", "unit", "galvanised-steel", "body", 0.35, -0.30, 0.26, 0.444, 0.519, -0.56, 0.56, member="unit", part="member"), box("m-top-frame", "unit", "galvanised-steel", "body", 0.35, -0.30, 0.26, 1.669, 1.744, -0.56, 0.56, member="unit", part="member")]
    st += masses(0.04)
    return body, th, st


def views50():
    """✏️ The 1:50 drawings: true views of the 1:50 model."""
    el, fr, pl = [], [], []
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        el.append(shp(f"bottom-board-{k + 1}", "pallet", "body", True, 0.5, erect("pallet", a, b, 0, 0.022), region="pallet", material="pallet-softwood"))
        el.append(shp(f"blocks-{k + 1}", "pallet", "body", True, 0.45, erect("pallet", a, b, 0.022, 0.100), region="pallet", material="pallet-softwood"))
    el.append(shp("stringer-1", "pallet", "body", True, 0.5, erect("pallet", -0.40, 0.40, 0.100, 0.122), region="pallet", material="pallet-softwood"))
    for k, (a, b) in enumerate(PALLET_X_TOP):
        el.append(shp(f"top-board-{k + 1}", "pallet", "body", True, 0.55, erect("pallet", a, b, 0.122, 0.144), region="pallet", material="pallet-softwood"))
    el.append(shp("condensate-spigot", "unit", "detail", True, 0.25, erect("unit", -0.012, 0.012, 0.414, 0.444), region="base-pan", material="pe-cap"))
    el.append(shp("stand-r", "console", "body", True, 0.4, epath_poly("console", world_stand(stand_profile(0.27)), world_stand(STAND_HOLE)), region="console", material="galvanised-steel"))
    for x in (-0.18, 0.18):
        el.append(shp(f"mounts-r{'-front' if x > 0 else '-rear'}", "console", "detail", True, 0.15, erect("console", x - 0.025, x + 0.025, 0.414, 0.444), region="console", material="epdm-rubber"))
    for x in (-0.27, 0.27):
        el.append(shp(f"anchor-bolt-r{'-front' if x > 0 else '-rear'}", "console", "detail", True, 0.3, erect("console", x - 0.006, x + 0.006, 0.184, 0.209), region="console", material="galvanised-steel"))
    el += [
        shp("water-stub-return", "unit", "detail", True, 0.45, erect("unit", -0.36, -0.30, 0.603, 0.645), region="water-connections", material="copper-oxidised"),
        shp("cap-return", "unit", "detail", True, 0.25, erect("unit", -0.385, -0.36, 0.598, 0.650), region="water-connections", material="pe-cap"),
        shp("base-pan", "unit", "body", True, 0.35, erect("unit", -0.30, 0.26, 0.444, 0.504), region="base-pan", material="galvanised-steel"),
        shp("casing-frame", "unit", "body", True, 0.6, erect("unit", -0.30, 0.26, 0.504, 1.684), regions=["casing", "compressor-compartment"], material="powder-coat-faded"),
        shp("coil", "unit", "detail", True, 0.3, erect("unit", -0.30, 0.20, 0.504, 1.684), region="coil", material="aluminium-fins", hatch="fins"),
        shp("crushed-fins", "unit", "texture", True, 0.15, erect("unit", -0.10, 0.06, 0.62, 0.84), region="coil", material="aluminium-fins"),
        shp("front-post", "unit", "body", True, 0.6, erect("unit", 0.20, 0.26, 0.504, 1.684), region="casing", material="powder-coat-faded"),
        shp("top-cover", "unit", "body", True, 0.55, erect("unit", -0.31, 0.27, 1.684, 1.744), region="top-cover", material="powder-coat-faded"),
        shp("grille-ring", "unit", "body", True, 0.2, erect("unit", 0.26, 0.30, FAN_Y - 0.33, FAN_Y + 0.33), region="fan", material="coated-wire"),
    ]
    fr.append(shp("bottom-board-3", "pallet", "body", True, 0.5, wrect(-0.60, 0.60, 0, 0.022), region="pallet", material="pallet-softwood"))
    for k, (a, b) in enumerate(PALLET_Z_ROWS):
        fr.append(shp(f"blocks-front-{k + 1}", "pallet", "body", True, 0.45, wrect(a, b, 0.022, 0.100), region="pallet", material="pallet-softwood"))
        fr.append(shp(f"stringer-{k + 1}", "pallet", "body", True, 0.5, wrect(a, b, 0.100, 0.122), region="pallet", material="pallet-softwood"))
    fr.append(shp("top-board-5", "pallet", "body", True, 0.55, wrect(-0.60, 0.60, 0.122, 0.144), region="pallet", material="pallet-softwood"))
    for side, z in (("r", -0.40), ("l", 0.40)):
        fr.append(shp(f"stand-{side}", "console", "body", True, 0.4, wrect(z - 0.03, z + 0.03, 0.144, 0.414), region="console", material="galvanised-steel"))
        fr.append(shp(f"mounts-{side}", "console", "detail", True, 0.15, wrect(z - 0.025, z + 0.025, 0.414, 0.444), region="console", material="epdm-rubber"))
    fr += [
        shp("condensate-spigot", "unit", "detail", True, 0.25, wrect(-0.212, -0.188, 0.414, 0.444), region="base-pan", material="pe-cap"),
        shp("coil-behind-grille", "unit", "detail", True, 0.25, ell(FAN_Z, FAN_Y, 0.30), region="coil", material="aluminium-fins", hatch="fins"),
        shp("fan-motor", "unit", "detail", True, 0.1, ell(FAN_Z, FAN_Y, 0.09), region="fan", material="painted-steel"),
    ]
    for k, pts in enumerate(blade_polys()):
        fr.append(shp(f"fan-blade-{k + 1}", "fan", "detail", True, 0.2, wpoly(pts), region="fan", material="pp-gf"))
    fr += [
        shp("fan-hub", "fan", "detail", True, 0.15, ell(FAN_Z, FAN_Y, 0.06), region="fan", material="pp-gf"),
        shp("front-panel", "unit", "body", True, 0.62, {"kind": "path", "d": rect_d(-0.48, 0.56, 0.504, 1.684) + " " + ring_d(FAN_Z, FAN_Y, 0.30)}, region="casing", material="powder-coat-faded"),
        shp("base-pan", "unit", "body", True, 0.35, wrect(-0.56, 0.56, 0.444, 0.504), region="base-pan", material="galvanised-steel"),
        shp("front-post", "unit", "body", True, 0.6, wrect(-0.56, -0.48, 0.504, 1.684), region="casing", material="powder-coat-faded"),
        shp("type-plate", "unit", "texture", True, 0.8, wrect(0.30, 0.42, 1.40, 1.48), region="casing", material="aluminium-fins"),
        shp("service-seam", "unit", "texture", False, 0.4, {"kind": "path", "d": "M 0.20 0.504 L 0.20 1.684"}, region="casing"),
        shp("grille-ring", "unit", "body", True, 0.2, {"kind": "path", "d": ring_d(FAN_Z, FAN_Y, 0.33) + " " + ring_d(FAN_Z, FAN_Y, 0.30)}, region="fan", material="coated-wire"),
    ]
    for k, r in enumerate((0.11, 0.18, 0.25)):
        fr.append(shp(f"grille-hoop-{k + 1}", "unit", "detail", False, 0.2, ell(FAN_Z, FAN_Y, r), region="fan", material="coated-wire", pen=0.13))
    spokes = " ".join(f"M {r6(FAN_Z + 0.05 * math.sin(k * math.pi / 4))} {r6(FAN_Y + 0.05 * math.cos(k * math.pi / 4))} L {r6(FAN_Z + 0.30 * math.sin(k * math.pi / 4))} {r6(FAN_Y + 0.30 * math.cos(k * math.pi / 4))}" for k in range(8))
    fr += [
        shp("grille-spokes", "unit", "detail", False, 0.2, {"kind": "path", "d": spokes}, region="fan", material="coated-wire", pen=0.13),
        shp("grille-cap", "unit", "detail", True, 0.2, ell(FAN_Z, FAN_Y, 0.05), region="fan", material="coated-wire"),
        shp("top-cover", "unit", "body", True, 0.55, wrect(-0.57, 0.57, 1.684, 1.744), region="top-cover", material="powder-coat-faded"),
    ]
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        pl.append(shp(f"bottom-board-{k + 1}", "pallet", "body", True, 0.5, wrect(a, b, -0.60, 0.60), region="pallet", material="pallet-softwood"))
    for k, (a, b) in enumerate(PALLET_Z_ROWS):
        pl.append(shp(f"stringer-{k + 1}", "pallet", "body", True, 0.5, wrect(-0.40, 0.40, a, b), region="pallet", material="pallet-softwood"))
    for k, (a, b) in enumerate(PALLET_X_TOP):
        pl.append(shp(f"top-board-{k + 1}", "pallet", "body", True, 0.55, wrect(a, b, -0.60, 0.60), region="pallet", material="pallet-softwood"))
    for side, z in (("r", -0.40), ("l", 0.40)):
        pl.append(shp(f"stand-{side}", "console", "body", True, 0.4, wrect(-0.30, 0.30, z - 0.03, z + 0.03), region="console", material="galvanised-steel"))
    pl += [
        shp("cap-flow", "unit", "detail", True, 0.25, wrect(-0.385, -0.36, 0.434, 0.486), region="water-connections", material="pe-cap"),
        shp("cap-return", "unit", "detail", True, 0.25, wrect(-0.385, -0.36, 0.294, 0.346), region="water-connections", material="pe-cap"),
        shp("water-stub-flow", "unit", "detail", True, 0.45, wrect(-0.36, -0.30, 0.439, 0.481), region="water-connections", material="copper-oxidised"),
        shp("water-stub-return", "unit", "detail", True, 0.45, wrect(-0.36, -0.30, 0.299, 0.341), region="water-connections", material="copper-oxidised"),
        shp("grille-ring", "unit", "body", True, 0.2, wrect(0.26, 0.30, FAN_Z - 0.33, FAN_Z + 0.33), region="fan", material="coated-wire"),
        shp("top-cover", "unit", "body", True, 0.55, wrect(-0.31, 0.27, -0.57, 0.57), region="top-cover", material="powder-coat-faded"),
        shp("cover-fold", "unit", "texture", False, 0.4, {"kind": "path", "d": "M -0.29 -0.55 L 0.25 -0.55 L 0.25 0.55 L -0.29 0.55 Z"}, region="top-cover"),
    ]
    return number(el), number(fr), number(pl)


def views100():
    """✏️ The 1:100 drawings: runners for stringer and blocks, stands to the pan, merged caps, grille without wires."""
    el, fr, pl = [], [], []
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        el.append(shp(f"bottom-board-{k + 1}", "pallet", "body", True, 0.5, erect("pallet", a, b, 0, 0.022), region="pallet", material="pallet-softwood"))
    runner = [(0, 0.122), (0.80, 0.122), (0.80, 0.022), (0.70, 0.022), (0.70, 0.100), (0.4725, 0.100), (0.4725, 0.022), (0.3275, 0.022), (0.3275, 0.100), (0.10, 0.100), (0.10, 0.022), (0, 0.022)]
    el.append(shp("runner-1", "pallet", "body", True, 0.48, {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in runner], "closed": True}, region="pallet", material="pallet-softwood"))
    for k, (a, b) in enumerate(PALLET_X_TOP):
        el.append(shp(f"top-board-{k + 1}", "pallet", "body", True, 0.55, erect("pallet", a, b, 0.122, 0.144), region="pallet", material="pallet-softwood"))
    el += [
        shp("stand-r", "console", "body", True, 0.4, epath_poly("console", world_stand(stand_profile(0.30)), world_stand(STAND_HOLE)), region="console", material="galvanised-steel"),
        shp("water-stub-return", "unit", "detail", True, 0.45, erect("unit", -0.385, -0.30, 0.598, 0.650), region="water-connections", material="copper-oxidised"),
        shp("base-pan", "unit", "body", True, 0.35, erect("unit", -0.30, 0.26, 0.444, 0.504), region="base-pan", material="galvanised-steel"),
        shp("casing-frame", "unit", "body", True, 0.6, erect("unit", -0.30, 0.26, 0.504, 1.684), regions=["casing", "compressor-compartment"], material="powder-coat-faded"),
        shp("coil", "unit", "detail", True, 0.3, erect("unit", -0.30, 0.20, 0.504, 1.684), region="coil", material="aluminium-fins", hatch="fins"),
        shp("front-post", "unit", "body", True, 0.6, erect("unit", 0.20, 0.26, 0.504, 1.684), region="casing", material="powder-coat-faded"),
        shp("top-cover", "unit", "body", True, 0.55, erect("unit", -0.31, 0.27, 1.684, 1.744), region="top-cover", material="powder-coat-faded"),
        shp("grille-ring", "unit", "body", True, 0.2, erect("unit", 0.26, 0.30, FAN_Y - 0.33, FAN_Y + 0.33), region="fan", material="coated-wire"),
    ]
    fr.append(shp("bottom-board-3", "pallet", "body", True, 0.5, wrect(-0.60, 0.60, 0, 0.022), region="pallet", material="pallet-softwood"))
    for k, (a, b) in enumerate(PALLET_Z_ROWS):
        fr.append(shp(f"runner-{k + 1}", "pallet", "body", True, 0.48, wrect(a, b, 0.022, 0.122), region="pallet", material="pallet-softwood"))
    fr.append(shp("top-board-5", "pallet", "body", True, 0.55, wrect(-0.60, 0.60, 0.122, 0.144), region="pallet", material="pallet-softwood"))
    for side, z in (("r", -0.40), ("l", 0.40)):
        fr.append(shp(f"stand-{side}", "console", "body", True, 0.4, wrect(z - 0.03, z + 0.03, 0.144, 0.444), region="console", material="galvanised-steel"))
    fr += [
        shp("condensate-spigot", "unit", "detail", True, 0.25, wrect(-0.212, -0.188, 0.414, 0.444), region="base-pan", material="pe-cap"),
        shp("coil-behind-grille", "unit", "detail", True, 0.25, ell(FAN_Z, FAN_Y, 0.30), region="coil", material="aluminium-fins", hatch="fins"),
        shp("fan-motor", "unit", "detail", True, 0.1, ell(FAN_Z, FAN_Y, 0.09), region="fan", material="painted-steel"),
    ]
    for k, pts in enumerate(blade_polys()):
        fr.append(shp(f"fan-blade-{k + 1}", "fan", "detail", True, 0.2, wpoly(pts), region="fan", material="pp-gf"))
    fr += [
        shp("fan-hub", "fan", "detail", True, 0.15, ell(FAN_Z, FAN_Y, 0.06), region="fan", material="pp-gf"),
        shp("front-panel", "unit", "body", True, 0.62, {"kind": "path", "d": rect_d(-0.48, 0.56, 0.504, 1.684) + " " + ring_d(FAN_Z, FAN_Y, 0.30)}, region="casing", material="powder-coat-faded"),
        shp("base-pan", "unit", "body", True, 0.35, wrect(-0.56, 0.56, 0.444, 0.504), region="base-pan", material="galvanised-steel"),
        shp("front-post", "unit", "body", True, 0.6, wrect(-0.56, -0.48, 0.504, 1.684), region="casing", material="powder-coat-faded"),
        shp("grille-ring", "unit", "body", True, 0.2, {"kind": "path", "d": ring_d(FAN_Z, FAN_Y, 0.33) + " " + ring_d(FAN_Z, FAN_Y, 0.30)}, region="fan", material="coated-wire"),
        shp("top-cover", "unit", "body", True, 0.55, wrect(-0.57, 0.57, 1.684, 1.744), region="top-cover", material="powder-coat-faded"),
    ]
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        pl.append(shp(f"bottom-board-{k + 1}", "pallet", "body", True, 0.5, wrect(a, b, -0.60, 0.60), region="pallet", material="pallet-softwood"))
    for k, (a, b) in enumerate(PALLET_Z_ROWS):
        pl.append(shp(f"runner-{k + 1}", "pallet", "body", True, 0.48, wrect(-0.40, 0.40, a, b), region="pallet", material="pallet-softwood"))
    for k, (a, b) in enumerate(PALLET_X_TOP):
        pl.append(shp(f"top-board-{k + 1}", "pallet", "body", True, 0.55, wrect(a, b, -0.60, 0.60), region="pallet", material="pallet-softwood"))
    for side, z in (("r", -0.40), ("l", 0.40)):
        pl.append(shp(f"stand-{side}", "console", "body", True, 0.4, wrect(-0.30, 0.30, z - 0.03, z + 0.03), region="console", material="galvanised-steel"))
    pl += [
        shp("water-stub-flow", "unit", "detail", True, 0.45, wrect(-0.385, -0.30, 0.434, 0.486), region="water-connections", material="copper-oxidised"),
        shp("water-stub-return", "unit", "detail", True, 0.45, wrect(-0.385, -0.30, 0.294, 0.346), region="water-connections", material="copper-oxidised"),
        shp("grille-ring", "unit", "body", True, 0.2, wrect(0.26, 0.30, FAN_Z - 0.33, FAN_Z + 0.33), region="fan", material="coated-wire"),
        shp("top-cover", "unit", "body", True, 0.55, wrect(-0.31, 0.27, -0.57, 0.57), region="top-cover", material="powder-coat-faded"),
    ]
    return number(el), number(fr), number(pl)


UNIT_REGIONS_200 = ["casing", "compressor-compartment", "coil", "base-pan"]
UNIT_REGIONS_500 = ["casing", "compressor-compartment", "coil", "base-pan", "fan", "top-cover", "water-connections"]


def views200():
    """✏️ The 1:200 drawings: deck and bearers, solid stands, casing, cover, grille disc and the stub block."""
    el, fr, pl = [], [], []
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        el.append(shp(f"bearer-{k + 1}", "pallet", "body", True, 0.48, erect("pallet", a, b, 0, 0.100), region="pallet", material="pallet-softwood"))
    el += [
        shp("pallet-deck", "pallet", "body", True, 0.55, erect("pallet", -0.40, 0.40, 0.100, 0.144), region="pallet", material="pallet-softwood"),
        shp("stand-r", "console", "body", True, 0.4, epoly("console", world_stand(stand_profile(0.30))), region="console", material="galvanised-steel"),
        shp("water-stubs", "unit", "detail", True, 0.45, erect("unit", -0.385, -0.30, 0.594, 0.654), region="water-connections", material="copper-oxidised"),
        shp("casing", "unit", "body", True, 0.6, erect("unit", -0.30, 0.26, 0.444, 1.684), regions=UNIT_REGIONS_200, material="powder-coat-faded"),
        shp("top-cover", "unit", "body", True, 0.55, erect("unit", -0.31, 0.27, 1.684, 1.744), region="top-cover", material="powder-coat-faded"),
        shp("grille-disc", "unit", "body", True, 0.2, erect("unit", 0.26, 0.30, FAN_Y - 0.33, FAN_Y + 0.33), region="fan", material="coated-wire"),
    ]
    fr += [
        shp("bearer-3", "pallet", "body", True, 0.48, wrect(-0.60, 0.60, 0, 0.100), region="pallet", material="pallet-softwood"),
        shp("pallet-deck", "pallet", "body", True, 0.55, wrect(-0.60, 0.60, 0.100, 0.144), region="pallet", material="pallet-softwood"),
        shp("stand-r", "console", "body", True, 0.4, wrect(-0.43, -0.37, 0.144, 0.444), region="console", material="galvanised-steel"),
        shp("stand-l", "console", "body", True, 0.4, wrect(0.37, 0.43, 0.144, 0.444), region="console", material="galvanised-steel"),
        shp("casing", "unit", "body", True, 0.6, wrect(-0.56, 0.56, 0.444, 1.684), regions=UNIT_REGIONS_200, material="powder-coat-faded"),
        shp("casing-joint", "unit", "detail", False, 0.4, {"kind": "path", "d": "M 0.20 0.504 L 0.20 1.684 M -0.56 0.504 L 0.56 0.504"}, region="casing"),
        shp("grille-disc", "unit", "body", True, 0.2, ell(FAN_Z, FAN_Y, 0.33), region="fan", material="coated-wire"),
        shp("fan-ring", "unit", "detail", False, 0.2, ell(FAN_Z, FAN_Y, 0.20), region="fan", material="coated-wire"),
        shp("top-cover", "unit", "body", True, 0.55, wrect(-0.57, 0.57, 1.684, 1.744), region="top-cover", material="powder-coat-faded"),
    ]
    pl += [
        shp("pallet-deck", "pallet", "body", True, 0.55, wrect(-0.40, 0.40, -0.60, 0.60), region="pallet", material="pallet-softwood"),
        shp("stand-r", "console", "body", True, 0.4, wrect(-0.30, 0.30, -0.43, -0.37), region="console", material="galvanised-steel"),
        shp("stand-l", "console", "body", True, 0.4, wrect(-0.30, 0.30, 0.37, 0.43), region="console", material="galvanised-steel"),
        shp("water-stubs", "unit", "detail", True, 0.45, wrect(-0.385, -0.30, 0.29, 0.49), region="water-connections", material="copper-oxidised"),
        shp("grille-disc", "unit", "body", True, 0.2, wrect(0.26, 0.30, FAN_Z - 0.33, FAN_Z + 0.33), region="fan", material="coated-wire"),
        shp("top-cover", "unit", "body", True, 0.55, wrect(-0.31, 0.27, -0.57, 0.57), region="top-cover", material="powder-coat-faded"),
    ]
    return number(el), number(fr), number(pl)


def views500():
    """✏️ The 1:500 drawings: pallet slab, two stands and the unit block with the fan opening as one stroke."""
    el = [
        shp("pallet", "pallet", "body", True, 0.5, erect("pallet", -0.40, 0.40, 0, 0.144), region="pallet", material="pallet-softwood"),
        shp("stand-r", "console", "body", True, 0.4, erect("console", -0.30, 0.30, 0.144, 0.444), region="console", material="galvanised-steel"),
        shp("unit-block", "unit", "body", True, 0.6, erect("unit", -0.30, 0.26, 0.444, 1.744), regions=UNIT_REGIONS_500, material="powder-coat-faded"),
    ]
    fr = [
        shp("pallet", "pallet", "body", True, 0.5, wrect(-0.60, 0.60, 0, 0.144), region="pallet", material="pallet-softwood"),
        shp("stand-r", "console", "body", True, 0.4, wrect(-0.4375, -0.3625, 0.144, 0.444), region="console", material="galvanised-steel"),
        shp("stand-l", "console", "body", True, 0.4, wrect(0.3625, 0.4375, 0.144, 0.444), region="console", material="galvanised-steel"),
        shp("unit-block", "unit", "body", True, 0.6, wrect(-0.56, 0.56, 0.444, 1.744), regions=UNIT_REGIONS_500, material="powder-coat-faded"),
        shp("fan-opening", "unit", "detail", False, 0.2, ell(FAN_Z, FAN_Y, 0.30), region="fan", material="coated-wire"),
    ]
    pl = [
        shp("pallet", "pallet", "body", True, 0.5, wrect(-0.40, 0.40, -0.60, 0.60), region="pallet", material="pallet-softwood"),
        shp("unit-block", "unit", "body", True, 0.6, wrect(-0.30, 0.26, -0.56, 0.56), regions=UNIT_REGIONS_500, material="powder-coat-faded"),
    ]
    return number(el), number(fr), number(pl)


exec(open(os.path.join(HERE, "author-text.py"), encoding="utf-8").read())


def build():
    """🏗️ Assembles the asset document and writes it."""
    vols, reps = {}, {}
    builders = {50: (scale50, views50), 100: (scale100, views100), 200: (scale200, views200), 500: (scale500, views500)}
    for den, (vb, rb) in builders.items():
        body, th, st = vb()
        el, fr, pl = rb()
        vols[str(den)] = {"status": "authored", "denominator": den, "body": body, "thermal": th, "structure": st, "decisions": VOLUME_DECISIONS[den]}
        reps[str(den)] = {"status": "authored", "denominator": den, "validRange": VALID[den], "pens": PENS[den], "hatches": HATCHES[den], "geometry": {"elevation": el, "front": fr, "plan": pl}, "decisions": REP_DECISIONS[den]}
    asset = {
        "id": "used-air-water-heat-pump-monobloc",
        "category": "components",
        "title": T("Used Air-to-Water Monobloc Heat Pump", "Gebrauchte Luft-Wasser-Monoblock-Wärmepumpe"),
        "summary": SUMMARY,
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "heat-pump", "monobloc", "air-source", "outdoor-unit", "pallet", "steel", "aluminium", "fan", "defrost"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": 1.744, "widthM": 0.8, "depthM": 1.2, "massKg": 184.6},
        "materials": MATERIALS,
        "rig": RIG,
        "volumes": vols,
        "anchors": ANCHORS,
        "representations": reps,
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": THERMAL,
        "structure": STRUCTURE,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print("[DEBUG] wrote", OUT, {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in vols.items()})


if __name__ == "__main__":
    build()
