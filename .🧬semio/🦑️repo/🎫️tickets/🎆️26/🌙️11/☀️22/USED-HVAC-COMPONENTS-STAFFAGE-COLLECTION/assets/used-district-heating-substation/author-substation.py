"""🏭️ Authoritative generator of the used district heating substation: states every hand-chosen number once and writes the candidate asset document 🔣️.json next to this file.

Run: python3 author-substation.py  (prints the reference masses and the computed region areas)

Frame: world metres, x ahead (along the 0.80 m wall frame), y up, z left (the frame back plane at z +0.18 faces the wall, the equipment stands out towards -z, which is the side the side elevation is seen from).
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
Y0 = 0.10
ZP = -0.05
R20 = 0.01345
R32 = 0.0212
XP1, XP2, XS2, XS1 = 0.10, 0.26, 0.56, 0.70
XPP, XPS = 0.3885, 0.4515
YLO, YHI = 0.60, 1.066
BONES = {
    "bearers": {"o": (0.0, 0.0), "a": 0},
    "carrier": {"o": (0.0, Y0), "a": 0},
    "station": {"o": (0.0, Y0), "a": 0},
    "lever-p1": {"o": (XP1, 0.235), "a": 0},
    "lever-p2": {"o": (XP2, 0.235), "a": 0},
    "lever-s2": {"o": (XS2, 0.465), "a": 180},
    "lever-s1": {"o": (XS1, 0.235), "a": 180},
}


def t(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def r4(v):
    """🎯️ Rounds to a tenth of a millimetre."""
    return round(v + 0.0, 4) + 0.0


def local_xy(bone, x, y):
    """🧭️ World rest (x, y) in the frame of `bone`."""
    b = BONES[bone]
    dx, dy = x - b["o"][0], y - b["o"][1]
    if b["a"] == 180:
        return -dx, -dy
    return dx, dy


def box(id, bone, lo, hi, material, region, tone, **kw):
    """📦️ An axis-aligned box from its world corners."""
    return dict(id=id, bone=bone, kind="box", lo=lo, hi=hi, material=material, region=region, tone=tone, **kw)


def cyl(id, bone, c, axis, r, a0, a1, material, region, tone, **kw):
    """🛢️ A cylinder along a world axis (or an oblique unit vector) from axial position a0 to a1 measured from c."""
    return dict(id=id, bone=bone, kind="cyl", c=c, axis=axis, r=r, a0=a0, a1=a1, material=material, region=region, tone=tone, **kw)


def ushell(id, bone, x0, x1, xi0, xi1, z0, zi, z1, y0, y1, material, region, tone, **kw):
    """🧺️ The rear insulation half-shell: a U section in plan (cavity open to the front) extruded up the exchanger."""
    return dict(id=id, bone=bone, kind="ushell", x0=x0, x1=x1, xi0=xi0, xi1=xi1, z0=z0, zi=zi, z1=z1, y0=y0, y1=y1, material=material, region=region, tone=tone, **kw)


AX = {"x": (1.0, 0.0, 0.0), "y": (0.0, 1.0, 0.0), "z": (0.0, 0.0, 1.0)}


def axis_vec(p):
    """➡️ The unit axis of a cylinder part."""
    a = p["axis"]
    return AX[a] if isinstance(a, str) else a


def samples(p):
    """🔵️ World points on the hull of a part, for projections and bounds."""
    if p["kind"] == "box":
        (x0, y0, z0), (x1, y1, z1) = p["lo"], p["hi"]
        return [(x, y, z) for x in (x0, x1) for y in (y0, y1) for z in (z0, z1)]
    if p["kind"] == "ushell":
        return [(x, y, z) for x in (p["x0"], p["x1"]) for y in (p["y0"], p["y1"]) for z in (p["z0"], p["z1"])]
    a = axis_vec(p)
    helper = (0.0, 0.0, 1.0) if abs(a[2]) < 0.9 else (1.0, 0.0, 0.0)
    e1 = cross(a, helper)
    n1 = math.sqrt(sum(v * v for v in e1))
    e1 = tuple(v / n1 for v in e1)
    e2 = cross(a, e1)
    pts = []
    for s in (p["a0"], p["a1"]):
        for k in range(48):
            ang = 2 * math.pi * k / 48
            pts.append(tuple(p["c"][i] + a[i] * s + p["r"] * (math.cos(ang) * e1[i] + math.sin(ang) * e2[i]) for i in range(3)))
    return pts


def cross(a, b):
    """✖️ The cross product."""
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def hull(points):
    """🪢️ The convex hull of 2D points, counter-clockwise."""
    pts = sorted(set((r4(x), r4(y)) for x, y in points))
    if len(pts) <= 2:
        return pts

    def turn(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for q in pts:
        while len(lower) >= 2 and turn(lower[-2], lower[-1], q) <= 0:
            lower.pop()
        lower.append(q)
    for q in reversed(pts):
        while len(upper) >= 2 and turn(upper[-2], upper[-1], q) <= 0:
            upper.pop()
        upper.append(q)
    return lower[:-1] + upper[:-1]


VIEW_AXES = {"elevation": (0, 1, 2), "front": (2, 1, 0), "plan": (0, 2, 1)}


def depth_order(p, view):
    """🔢️ The painting layer of a part in a view: nearer parts higher (elevation seen from -z, front from +x, plan from +y)."""
    pts = samples(p)
    if view == "elevation":
        return round(2.0 - 4.0 * min(q[2] for q in pts), 3)
    if view == "front":
        return round(1.0 + 2.0 * max(q[0] for q in pts), 3)
    return round(1.0 + 2.0 * max(q[1] for q in pts), 3)


def shape_geometry(p, view):
    """✏️ The 2D geometry of a part in a view: rect, ellipse or hull poly; elevation in the bone frame, front and plan in world rest coordinates."""
    i, j, k = VIEW_AXES[view]
    bone = p["bone"]
    lever = bone.startswith("lever")
    if lever and view != "elevation":
        g = shape_geometry(p, "elevation")
        pts = samples(p)
        g["solid"] = {"kind": "extruded", "sweep": "w", "z0": r4(min(q[2] for q in pts)), "z1": r4(max(q[2] for q in pts))}
        return g

    def place(u, v):
        if view == "elevation":
            return local_xy(bone, u, v)
        return u, v

    if p["kind"] == "box" or p["kind"] == "ushell":
        if p["kind"] == "ushell" and view == "plan":
            ring = [(p["x0"], p["z1"]), (p["x0"], p["z0"]), (p["xi0"], p["z0"]), (p["xi0"], p["zi"]), (p["xi1"], p["zi"]), (p["xi1"], p["z0"]), (p["x1"], p["z0"]), (p["x1"], p["z1"])]
            return {"kind": "poly", "points": [[r4(a), r4(b)] for a, b in ring], "closed": True}
        pts = samples(p)
        a0, a1 = min(q[i] for q in pts), max(q[i] for q in pts)
        b0, b1 = min(q[j] for q in pts), max(q[j] for q in pts)
        c0 = place(a0, b0)
        c1 = place(a1, b1)
        return {"kind": "rect", "x": r4(min(c0[0], c1[0])), "y": r4(min(c0[1], c1[1])), "w": r4(abs(c1[0] - c0[0])), "h": r4(abs(c1[1] - c0[1]))}
    a = axis_vec(p)
    if abs(abs(a[k]) - 1) < 1e-9:
        cx, cy = place(p["c"][i], p["c"][j])
        return {"kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": r4(p["r"]), "ry": r4(p["r"])}
    pts = samples(p)
    if abs(abs(a[i]) - 1) < 1e-9 or abs(abs(a[j]) - 1) < 1e-9:
        a0, a1 = min(q[i] for q in pts), max(q[i] for q in pts)
        b0, b1 = min(q[j] for q in pts), max(q[j] for q in pts)
        c0, c1 = place(a0, b0), place(a1, b1)
        return {"kind": "rect", "x": r4(min(c0[0], c1[0])), "y": r4(min(c0[1], c1[1])), "w": r4(abs(c1[0] - c0[0])), "h": r4(abs(c1[1] - c0[1]))}
    ring = hull([place(q[i], q[j]) for q in pts])
    return {"kind": "poly", "points": [[x, y] for x, y in ring], "closed": True}


def solid_of(p):
    """🧊️ The volume solid of a part in its bone frame."""
    bone = p["bone"]
    base = {"id": p["id"], "bone": bone, "material": p["material"], "role": p.get("role", "body"), "tone": p["tone"]}
    for flag in ("slender", "sheet", "texture", "envelope"):
        if p.get(flag):
            base[flag] = True
    if p["kind"] == "box":
        (x0, y0, z0), (x1, y1, z1) = p["lo"], p["hi"]
        cx, cy = local_xy(bone, (x0 + x1) / 2, (y0 + y1) / 2)
        base.update(kind="box", size=[r4(x1 - x0), r4(y1 - y0), r4(z1 - z0)], transform={"translation": [r4(cx), r4(cy), r4((z0 + z1) / 2)]})
        return base
    if p["kind"] == "ushell":
        ring = [(p["z1"], p["x0"]), (p["z0"], p["x0"]), (p["z0"], p["xi0"]), (p["zi"], p["xi0"]), (p["zi"], p["xi1"]), (p["z0"], p["xi1"]), (p["z0"], p["x1"]), (p["z1"], p["x1"])]
        ox, oy = BONES[bone]["o"]
        base.update(kind="extrude", profile=[[r4(w), r4(u - ox)] for w, u in ring], plane="zx", **{"from": r4(p["y0"] - oy), "to": r4(p["y1"] - oy)})
        return base
    a = axis_vec(p)
    cx, cy = local_xy(bone, p["c"][0], p["c"][1])
    ax = list(a)
    if BONES[bone]["a"] == 180:
        ax = [-a[0], -a[1], a[2]]
    base.update(kind="revolve", profile=[[r4(p["a0"]), r4(p["r"])], [r4(p["a1"]), r4(p["r"])]], axis=[r4(v) for v in ax], origin=[r4(cx), r4(cy), r4(p["c"][2])])
    return base


def area_of(p):
    """📐️ The surface area of a part alone, square metres."""
    if p["kind"] == "box":
        (x0, y0, z0), (x1, y1, z1) = p["lo"], p["hi"]
        a, b, c = x1 - x0, y1 - y0, z1 - z0
        return 2 * (a * b + b * c + a * c)
    if p["kind"] == "ushell":
        h = p["y1"] - p["y0"]
        perim = 2 * (p["x1"] - p["x0"]) + 2 * (p["z1"] - p["z0"]) + 2 * (p["zi"] - p["z0"])
        ring = (p["x1"] - p["x0"]) * (p["z1"] - p["z0"]) - (p["xi1"] - p["xi0"]) * (p["zi"] - p["z0"])
        return perim * h + 2 * ring
    L = p["a1"] - p["a0"]
    return 2 * math.pi * p["r"] * L + 2 * math.pi * p["r"] ** 2


def volume_of(p):
    """🧮️ The volume of a part alone, cubic metres."""
    if p["kind"] == "box":
        (x0, y0, z0), (x1, y1, z1) = p["lo"], p["hi"]
        return (x1 - x0) * (y1 - y0) * (z1 - z0)
    if p["kind"] == "ushell":
        ring = (p["x1"] - p["x0"]) * (p["z1"] - p["z0"]) - (p["xi1"] - p["xi0"]) * (p["zi"] - p["z0"])
        return ring * (p["y1"] - p["y0"])
    return math.pi * p["r"] ** 2 * (p["a1"] - p["a0"])


def zbox(id, bone, x, y, z0, z1, sx, sy, material, region, tone, **kw):
    """📐️ A box centred on (x, y) in elevation, spanning z0..z1."""
    return box(id, bone, (x - sx / 2, y - sy / 2, z0), (x + sx / 2, y + sy / 2, z1), material, region, tone, **kw)


def pipe_run(prefix, x, r, ylo_port, y_port_h, xport, region, scale, tone, ystart):
    """🚰️ One pipe run: vertical riser, horizontal header in the pipe plane and the stub into the exchanger port."""
    rr = r if scale < 200 else max(r, 0.015)
    v_top = y_port_h + rr
    xa, xb = (min(x, xport) - rr, max(x, xport) + rr)
    return [
        cyl(f"{prefix}-riser", "station", (x, 0, ZP), "y", rr, ystart, v_top, "black-steel-pipe", region, tone),
        cyl(f"{prefix}-header", "station", (0, y_port_h, ZP), "x", rr, xa, xb, "black-steel-pipe", region, tone),
        cyl(f"{prefix}-stub", "station", (xport, y_port_h, 0), "z", rr, ZP - rr, 0.02, "black-steel-pipe", region, tone),
    ]


def frame_parts(scale):
    """🪜️ The galvanised wall frame: two SHS 40 x 40 x 2 uprights and three crossbars (1:500: generalised to the 75 mm feature)."""
    if scale == 500:
        return [
            box("frame-upright-left", "station", (0.0, Y0, 0.105), (0.075, Y0 + 1.10, 0.18), "galvanised-steel", "frame", 0.45),
            box("frame-upright-right", "station", (0.725, Y0, 0.105), (0.80, Y0 + 1.10, 0.18), "galvanised-steel", "frame", 0.45),
            box("frame-rail-lower", "station", (0.075, 0.28, 0.105), (0.725, 0.355, 0.18), "galvanised-steel", "frame", 0.45),
            box("frame-rail-top", "station", (0.075, 1.125, 0.105), (0.725, 1.20, 0.18), "galvanised-steel", "frame", 0.45),
        ]
    tone = {50: 0.62, 100: 0.6, 200: 0.58}[scale]
    return [
        box("frame-upright-left", "station", (0.0, Y0, 0.14), (0.04, Y0 + 1.10, 0.18), "galvanised-steel", "frame", tone),
        box("frame-upright-right", "station", (0.76, Y0, 0.14), (0.80, Y0 + 1.10, 0.18), "galvanised-steel", "frame", tone),
        box("frame-rail-lower", "station", (0.04, 0.30, 0.14), (0.76, 0.34, 0.18), "galvanised-steel", "frame", tone - 0.04),
        box("frame-rail-middle", "station", (0.04, 0.70, 0.14), (0.76, 0.74, 0.18), "galvanised-steel", "frame", tone - 0.04),
        box("frame-rail-top", "station", (0.04, 1.16, 0.14), (0.76, 1.20, 0.18), "galvanised-steel", "frame", tone - 0.04),
    ]


def bearer_parts(scale):
    """🪵️ The two weathered squared-timber bearers 100 x 100 x 600 mm the frame stands on in the yard."""
    tone = {50: 0.5, 100: 0.48, 200: 0.46, 500: 0.44}[scale]
    return [
        box("bearer-left", "bearers", (0.0, 0.0, -0.30), (0.10, 0.10, 0.30), "weathered-softwood", "bearers", tone),
        box("bearer-right", "bearers", (0.70, 0.0, -0.30), (0.80, 0.10, 0.30), "weathered-softwood", "bearers", tone - 0.03),
    ]


def exchanger_parts(scale):
    """🔥️ The brazed plate heat exchanger (119 x 526 x 80 mm, 30 plates) in its rear EPP half-shell; the front half-shell is missing."""
    bx0, bx1, by0, by1 = 0.3605, 0.4795, 0.57, 1.096
    if scale == 500:
        return []
    parts = []
    if scale == 200:
        parts.append(box("exchanger-shell", "station", (0.335, 0.545, 0.06), (0.505, 1.121, 0.14), "epp-foam", "shell", 0.15))
        parts.append(box("exchanger", "station", (bx0, by0, 0.02), (bx1, by1, 0.06), "stainless-steel", "exchanger", 0.78))
        return parts
    parts.append(ushell("exchanger-shell", "station", 0.335, 0.505, bx0, bx1, 0.06, 0.10, 0.14, 0.545, 1.121, "epp-foam", "shell", 0.14))
    parts.append(box("exchanger-shell-cap-bottom", "station", (bx0, 0.545, 0.06), (bx1, 0.57, 0.10), "epp-foam", "shell", 0.14))
    parts.append(box("exchanger-shell-cap-top", "station", (bx0, 1.096, 0.06), (bx1, 1.121, 0.10), "epp-foam", "shell", 0.14))
    parts.append(box("exchanger", "station", (bx0, by0, 0.02), (bx1, by1, 0.10), "stainless-steel", "exchanger", 0.8 if scale == 50 else 0.78))
    if scale == 50:
        parts.append(box("exchanger-type-plate", "station", (0.385, 0.95, 0.0185), (0.455, 0.99, 0.02), "aluminium-label", "exchanger", 0.92, texture=True))
        for k, (x, y, r, reg) in enumerate([(XPP, YLO, 0.022, "primary-return"), (XPP, YHI, 0.022, "primary-supply"), (XPS, YLO, 0.028, "secondary-return"), (XPS, YHI, 0.028, "secondary-supply")]):
            parts.append(cyl(f"union-nut-{['p2', 'p1', 's2', 's1'][k]}", "station", (x, y, 0), "z", r, -0.008, 0.02, "brass", reg, 0.55))
    return parts


def pipework(scale):
    """🔧️ The four pipe runs with their fittings: primary supply (strainer, motorised valve), primary return (heat meter, differential pressure controller), secondary return and supply, ball valves, capped stubs and clamps."""
    parts = []
    big = scale >= 200
    tone = {50: 0.3, 100: 0.3, 200: 0.28}[scale]
    ystart = 0.145 if scale == 50 else 0.145
    cap = scale == 50
    y0 = 0.175 if cap else ystart
    parts += pipe_run("primary-supply", XP1, R20, None, YHI, XPP, "primary-supply", scale, tone, y0)
    parts += pipe_run("primary-return", XP2, R20, None, YLO, XPP, "primary-return", scale, tone + 0.02, y0)
    parts += pipe_run("secondary-return", XS2, R32, None, YLO, XPS, "secondary-return", scale, tone + 0.04, y0)
    parts += pipe_run("secondary-supply", XS1, R32, None, YHI, XPS, "secondary-supply", scale, tone + 0.06, y0)
    if cap:
        for name, x, r, reg in [("p1", XP1, R20, "primary-supply"), ("p2", XP2, R20, "primary-return"), ("s2", XS2, R32, "secondary-return"), ("s1", XS1, R32, "secondary-supply")]:
            parts.append(cyl(f"cap-{name}", "station", (x, 0, ZP), "y", r + 0.004, 0.145, 0.178, "malleable-iron", reg, 0.22))
    valves = [("p1", XP1, 0.235, 0.05, 0.07, "primary-supply"), ("p2", XP2, 0.235, 0.05, 0.07, "primary-return"), ("s2", XS2, 0.465, 0.065, 0.09, "secondary-return"), ("s1", XS1, 0.235, 0.065, 0.09, "secondary-supply")]
    for name, x, y, d, h, reg in valves:
        front = ZP - d / 2
        lever_z = (-0.099, -0.091) if d == 0.05 else (-0.107, -0.099)
        if scale == 50:
            parts.append(zbox(f"ball-valve-{name}", "station", x, y, front, ZP + d / 2, d, h, "brass", reg, 0.58))
            parts.append(cyl(f"ball-valve-stem-{name}", "station", (x, y, 0), "z", 0.006, lever_z[1], front, "brass", reg, 0.5))
        elif scale == 100:
            parts.append(zbox(f"ball-valve-{name}", "station", x, y, lever_z[1], ZP + d / 2, d, h, "brass", reg, 0.56))
        else:
            parts.append(zbox(f"ball-valve-{name}", "station", x, y, front, ZP + d / 2, d, h, "brass", reg, 0.54))
        bone = f"lever-{name}"
        length = 0.11 if d == 0.05 else 0.14
        thick = {50: 0.008, 100: 0.015, 200: 0.03}[scale]
        half = 0.01 if scale < 200 else 0.015
        zc = {50: (lever_z[0] + lever_z[1]) / 2, 100: lever_z[1] - 0.0075, 200: front - 0.015}[scale]
        sign = 1 if BONES[bone]["a"] == 0 else -1
        xa, xb = sorted([x - sign * 0.012, x + sign * length])
        parts.append(box(f"lever-{name}", bone, (xa, y - half, zc - thick / 2), (xb, y + half, zc + thick / 2), "coated-steel", "valve-levers", 0.18))
    if scale <= 100:
        for yc, names in [(0.32, [("p1", XP1, R20, "primary-supply"), ("p2", XP2, R20, "primary-return"), ("s2", XS2, R32, "secondary-return"), ("s1", XS1, R32, "secondary-supply")]), (0.72, [("p1-upper", XP1, R20, "primary-supply"), ("s1-upper", XS1, R32, "secondary-supply")])]:
            for name, x, r, reg in names:
                parts.append(cyl(f"clamp-rod-{name}", "station", (x, yc, 0), "z", 0.004, ZP + r, 0.14, "galvanised-steel", "frame", 0.5, slender=True))
                parts.append(zbox(f"clamp-band-{name}", "station", x, yc, ZP - r - 0.004, ZP + r + 0.004, 2 * r + 0.008, 0.015, "galvanised-steel", "frame", 0.52))
    sx = XP1
    parts.append(cyl("strainer-body", "station", (sx, 0, ZP), "y", 0.02, 0.38, 0.46, "brass", "primary-supply", 0.52))
    if scale <= 100:
        d = (math.sqrt(0.5), -math.sqrt(0.5), 0.0)
        parts.append(cyl("strainer-pocket", "station", (sx, 0.43, ZP), d, 0.012, 0.0, 0.062, "brass", "primary-supply", 0.48))
    if scale == 50:
        parts.append(zbox("control-valve-body", "station", XP1, 0.57, -0.08, -0.02, 0.06, 0.10, "cast-iron", "primary-supply", 0.34))
        parts.append(cyl("control-valve-bonnet", "station", (XP1, 0.57, 0), "z", 0.015, -0.10, -0.08, "cast-iron", "primary-supply", 0.4))
        parts.append(zbox("actuator", "station", 0.10, 0.57, -0.24, -0.10, 0.09, 0.09, "abs-housing", "actuator", 0.86))
        parts.append(cyl("actuator-cable", "station", (0.13, 0, -0.21), "y", 0.004, 0.40, 0.525, "pvc-cable", "actuator", 0.1))
    elif scale == 100:
        parts.append(zbox("control-valve-body", "station", XP1, 0.57, -0.08, -0.02, 0.06, 0.10, "cast-iron", "primary-supply", 0.34))
        parts.append(zbox("actuator", "station", 0.10, 0.57, -0.24, -0.08, 0.09, 0.09, "abs-housing", "actuator", 0.85))
    else:
        parts.append(zbox("control-valve-body", "station", XP1, 0.57, -0.08, -0.02, 0.06, 0.10, "cast-iron", "primary-supply", 0.34))
        parts.append(zbox("actuator", "station", 0.10, 0.57, -0.24, -0.08, 0.09, 0.09, "abs-housing", "actuator", 0.84))
    parts.append(zbox("heat-meter-sensor", "station", XP2, 0.415, -0.08, -0.02, 0.05, 0.11, "brass", "primary-return", 0.6))
    parts.append(zbox("dp-controller-body", "station", XP2, 0.54, -0.08, -0.02, 0.06, 0.08, "cast-iron", "primary-return", 0.36))
    if scale == 50:
        parts.append(cyl("dp-controller-spindle", "station", (XP2, 0.54, 0), "z", 0.01, -0.09, -0.08, "stainless-steel", "primary-return", 0.6))
        parts.append(cyl("dp-diaphragm-housing", "station", (XP2, 0.54, 0), "z", 0.07, -0.14, -0.09, "painted-steel", "primary-return", 0.4))
        parts.append(cyl("impulse-tube-run", "station", (0, 0.66, ZP), "x", 0.004, XP1 + R20 - 0.002, 0.234, "copper", "primary-supply", 0.66))
        parts.append(cyl("impulse-tube-drop", "station", (0.23, 0, ZP), "y", 0.004, 0.60, 0.664, "copper", "primary-supply", 0.66))
        parts.append(cyl("impulse-tube-elbow", "station", (0.23, 0.60, 0), "z", 0.004, -0.091, ZP + 0.004, "copper", "primary-supply", 0.66))
    else:
        parts.append(cyl("dp-diaphragm-housing", "station", (XP2, 0.54, 0), "z", 0.07, -0.14, -0.08, "painted-steel", "primary-return", 0.4))
    if scale == 50:
        parts.append(cyl("thermometer-stem", "station", (XS1, 0.90, 0), "z", 0.006, -0.09, ZP - R32 + 0.001, "brass", "secondary-supply", 0.5))
        parts.append(cyl("thermometer-dial", "station", (XS1, 0.90, 0), "z", 0.032, -0.11, -0.09, "painted-steel", "secondary-supply", 0.82))
    else:
        parts.append(cyl("thermometer-dial", "station", (XS1, 0.90, 0), "z", 0.032, -0.11, ZP - R32 + 0.001, "painted-steel", "secondary-supply", 0.8))
    parts.append(box("heat-meter-calculator", "station", (0.14, 0.74, 0.10 if scale < 200 else 0.10), (0.24, 0.86, 0.14), "abs-housing", "meter-unit", 0.88))
    return parts


def parts_of(scale):
    """🧩️ Every part of one scale."""
    if scale == 500:
        return bearer_parts(500) + frame_parts(500) + [
            box("equipment", "station", (0.075, 0.145, -0.10), (0.725, 1.125, 0.105), "black-steel-pipe", "primary-supply", 0.4),
            box("actuator", "station", (0.055, 0.525, -0.24), (0.145, 0.615, -0.10), "abs-housing", "actuator", 0.8),
        ]
    return bearer_parts(scale) + frame_parts(scale) + exchanger_parts(scale) + pipework(scale)


MATERIALS = [
    {"id": "galvanised-steel", "title": t("Hot-dip galvanised steel S235, weathered dull", "Feuerverzinkter Stahl S235, matt bewittert"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "ashrae-2021"},
    {"id": "black-steel-pipe", "title": t("Black steel pipe EN 10255, old red-oxide primer", "Schwarzes Stahlrohr EN 10255, alte Rotgrundierung"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 210000000000, "source": "literature", "reference": "en-10255"},
    {"id": "malleable-iron", "title": t("Malleable cast iron caps, black", "Temperguss-Kappen, schwarz"), "emissivity": 0.8, "solarAbsorptivity": 0.8, "densityKgM3": 7300, "conductivityWmK": 45, "specificHeatJKgK": 460, "source": "literature", "reference": "ashrae-2021"},
    {"id": "stainless-steel", "title": t("Stainless steel 1.4401, copper-brazed, tarnished", "Edelstahl 1.4401, kupfergelötet, angelaufen"), "emissivity": 0.35, "solarAbsorptivity": 0.5, "densityKgM3": 7950, "conductivityWmK": 15, "specificHeatJKgK": 500, "source": "literature", "reference": "ashrae-2021"},
    {"id": "epp-foam", "title": t("Black expanded polypropylene EPP 60 kg/m³", "Schwarzes expandiertes Polypropylen EPP 60 kg/m³"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 60, "conductivityWmK": 0.038, "specificHeatJKgK": 1800, "source": "literature", "reference": "en-iso-10456"},
    {"id": "brass", "title": t("Brass CW617N, tarnished with limescale", "Messing CW617N, angelaufen und verkalkt"), "emissivity": 0.6, "solarAbsorptivity": 0.6, "densityKgM3": 8450, "conductivityWmK": 110, "specificHeatJKgK": 380, "source": "literature", "reference": "ashrae-2021"},
    {"id": "cast-iron", "title": t("Grey cast iron EN-GJL-250, painted, rust at the flanges", "Grauguss EN-GJL-250, lackiert, Rost an den Flanschen"), "emissivity": 0.85, "solarAbsorptivity": 0.8, "densityKgM3": 7200, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "literature", "reference": "ashrae-2021"},
    {"id": "painted-steel", "title": t("Painted sheet steel, faded", "Lackiertes Stahlblech, verblichen"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "source": "literature", "reference": "ashrae-2021"},
    {"id": "abs-housing", "title": t("Light grey ABS housing, yellowed", "Hellgraues ABS-Gehäuse, vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.4, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "ashrae-2021"},
    {"id": "pvc-cable", "title": t("PVC-sheathed control cable, cut", "PVC-Steuerleitung, abgeschnitten"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1400, "conductivityWmK": 0.17, "specificHeatJKgK": 1000, "source": "literature", "reference": "ashrae-2021"},
    {"id": "copper", "title": t("Copper impulse tube 8 x 1, oxidised", "Kupfer-Impulsrohr 8 x 1, oxidiert"), "emissivity": 0.65, "solarAbsorptivity": 0.65, "densityKgM3": 8930, "conductivityWmK": 390, "specificHeatJKgK": 385, "source": "literature", "reference": "ashrae-2021"},
    {"id": "coated-steel", "title": t("Steel lever, red vinyl coating, chipped", "Stahlhebel, rote Vinylbeschichtung, abgeplatzt"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "source": "literature", "reference": "ashrae-2021"},
    {"id": "aluminium-label", "title": t("Aluminium type plate, embossed", "Aluminium-Typenschild, geprägt"), "emissivity": 0.2, "solarAbsorptivity": 0.3, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "source": "literature", "reference": "ashrae-2021"},
    {"id": "weathered-softwood", "title": t("Squared softwood C24, weathered grey", "Kantholz Nadelholz C24, grau bewittert"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 420, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
]

REAL_MASS = {
    "frame-upright-left": 1.10 * 2.31, "frame-upright-right": 1.10 * 2.31,
    "frame-rail-lower": 0.72 * 2.31, "frame-rail-middle": 0.72 * 2.31, "frame-rail-top": 0.72 * 2.31,
    "exchanger": 7.0, "exchanger-shell": 0.26, "exchanger-shell-cap-bottom": 0.02, "exchanger-shell-cap-top": 0.02, "exchanger-type-plate": 0.0,
    "actuator": 0.65, "actuator-cable": 0.03, "control-valve-body": 1.15, "control-valve-bonnet": 0.15,
    "strainer-body": 0.45, "strainer-pocket": 0.15,
    "heat-meter-sensor": 0.75, "heat-meter-calculator": 0.40,
    "dp-controller-body": 1.6, "dp-controller-spindle": 0.1, "dp-diaphragm-housing": 1.4,
    "impulse-tube-run": 0.02, "impulse-tube-drop": 0.01, "impulse-tube-elbow": 0.01,
    "thermometer-stem": 0.05, "thermometer-dial": 0.15,
    "ball-valve-p1": 0.30, "ball-valve-p2": 0.30, "ball-valve-s2": 0.70, "ball-valve-s1": 0.70,
    "ball-valve-stem-p1": 0.02, "ball-valve-stem-p2": 0.02, "ball-valve-stem-s2": 0.03, "ball-valve-stem-s1": 0.03,
    "cap-p1": 0.08, "cap-p2": 0.08, "cap-s2": 0.15, "cap-s1": 0.15,
    "union-nut-p1": 0.08, "union-nut-p2": 0.08, "union-nut-s1": 0.14, "union-nut-s2": 0.14,
    "bearer-left": 0.10 * 0.10 * 0.60 * 420, "bearer-right": 0.10 * 0.10 * 0.60 * 420,
}
PIPE_KG_M = {"primary": 1.56, "secondary": 3.09}
LEVER_KG = {"lever-p1": 0.06, "lever-p2": 0.06, "lever-s2": 0.09, "lever-s1": 0.09}


def mass_of(p):
    """⚖️ The real mass of a part (reference table), kilograms."""
    pid = p["id"]
    if pid in REAL_MASS:
        return REAL_MASS[pid]
    if pid in LEVER_KG:
        return LEVER_KG[pid]
    if pid.startswith("primary-") or pid.startswith("secondary-"):
        L = p["a1"] - p["a0"] - 2 * p["r"] * (0 if pid.endswith("riser") else 1)
        return PIPE_KG_M["primary" if pid.startswith("primary") else "secondary"] * L
    if pid.startswith("clamp-rod"):
        return 0.07
    if pid.startswith("clamp-band"):
        return 0.08
    raise KeyError(pid)


REGIONS_ORDER = ["frame", "bearers", "exchanger", "shell", "primary-supply", "primary-return", "secondary-supply", "secondary-return", "actuator", "meter-unit", "valve-levers"]


def emitter_patch(scale):
    """♨️ The plume outlet patch on the free front half of the exchanger top (D2)."""
    h = FEATURE[scale]
    if scale == 500:
        return {"id": "th-emitter-exchanger-plume", "kind": "box", "bone": "station", "material": "stainless-steel", "role": "body", "tone": 0.8, "size": [0.12, 0.075, 0.075], "transform": {"translation": [0.42, 1.0625, 0.0675]}, "region": "exchanger", "tags": ["exchanger-plume"]}
    return {"id": "th-emitter-exchanger-plume", "kind": "box", "bone": "station", "material": "stainless-steel", "role": "body", "tone": 0.8, "size": [0.10, r4(h), 0.04 if scale < 200 else 0.04], "transform": {"translation": [0.42, r4(1.096 - Y0 + h / 2), 0.04]}, "region": "exchanger", "tags": ["exchanger-plume"]}


def thermal_500():
    """🌡️ The 1:500 thermal model: the equipment mass split into its regions at the 75 mm feature."""
    def b(id, lo, hi, region, material, bone="station", tone=0.5):
        (x0, y0, z0), (x1, y1, z1) = lo, hi
        cx, cy = local_xy(bone, (x0 + x1) / 2, (y0 + y1) / 2)
        return {"id": id, "kind": "box", "bone": bone, "material": material, "role": "body", "tone": tone, "size": [r4(x1 - x0), r4(y1 - y0), r4(z1 - z0)], "transform": {"translation": [r4(cx), r4(cy), r4((z0 + z1) / 2)]}, "region": region}
    return [
        {"id": "th-bearer-left", "of": "bearer-left", "region": "bearers"},
        {"id": "th-bearer-right", "of": "bearer-right", "region": "bearers"},
        {"id": "th-frame-upright-left", "of": "frame-upright-left", "region": "frame"},
        {"id": "th-frame-upright-right", "of": "frame-upright-right", "region": "frame"},
        {"id": "th-frame-rail-lower", "of": "frame-rail-lower", "region": "frame"},
        {"id": "th-frame-rail-top", "of": "frame-rail-top", "region": "frame"},
        b("th-shell", (0.335, 0.545, 0.03), (0.505, 1.121, 0.105), "shell", "epp-foam"),
        b("th-exchanger", (0.36, 0.57, -0.045), (0.48, 1.096, 0.03), "exchanger", "stainless-steel"),
        b("th-primary-supply", (0.075, 0.145, -0.09), (0.15, 1.10, -0.015), "primary-supply", "black-steel-pipe"),
        b("th-primary-return", (0.22, 0.145, -0.09), (0.30, 0.64, -0.015), "primary-return", "black-steel-pipe"),
        b("th-secondary-return", (0.52, 0.145, -0.09), (0.60, 0.64, -0.015), "secondary-return", "black-steel-pipe"),
        b("th-secondary-supply", (0.65, 0.145, -0.09), (0.725, 1.10, -0.015), "secondary-supply", "black-steel-pipe"),
        {"id": "th-actuator", "of": "actuator", "region": "actuator"},
        b("th-meter-unit", (0.15, 0.74, 0.03), (0.25, 0.86, 0.105), "meter-unit", "abs-housing"),
        b("th-valve-levers", (0.15, 0.20, -0.165), (0.225, 0.275, -0.09), "valve-levers", "coated-steel"),
        emitter_patch(500),
    ]


def volume_of_scale(scale, parts):
    """🧱️ The authored volume of one scale: body, thermal and structure lists."""
    body = [solid_of(p) for p in parts]
    if scale == 500:
        thermal = thermal_500()
    else:
        thermal = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in parts if not p.get("texture")]
        thermal.append(emitter_patch(scale))
    frame_ids = [p["id"] for p in parts if p["region"] == "frame" and p["id"].startswith("frame-")]
    structure = [{"id": f"member-{fid}", "of": fid, "member": "station", "part": "member"} for fid in frame_ids]
    structure += [{"id": "member-bearer-left", "of": "bearer-left", "member": "bearers", "part": "member"}, {"id": "member-bearer-right", "of": "bearer-right", "member": "bearers", "part": "member"}]
    jh = {50: 0.008, 100: 0.015, 200: 0.03, 500: 0.075}[scale]
    jw = 0.04 if scale < 500 else 0.075
    for side, x in (("left", jw / 2), ("right", 0.80 - jw / 2)):
        structure.append({"id": f"joint-station-foot-{side}", "kind": "box", "bone": "station", "material": "galvanised-steel", "role": "body", "tone": 0.3, "size": [jw, jh, jw], "transform": {"translation": [r4(x), r4(jh / 2), 0.16 if scale < 500 else 0.1425]}, "member": "station", "part": "joint"})
    mr = {50: 0.015, 100: 0.015, 200: 0.03, 500: 0.04}[scale]
    sm = STATION_MASS
    structure.append({"id": "mass-station", "kind": "ellipsoid", "bone": "station", "material": "galvanised-steel", "role": "body", "tone": 0.5, "radii": [mr, mr, mr], "transform": {"translation": [r4(sm["comX"]), r4(sm["comY"]), 0.04]}, "member": "station", "part": "mass"})
    structure.append({"id": "mass-bearers", "kind": "ellipsoid", "bone": "bearers", "material": "weathered-softwood", "role": "body", "tone": 0.5, "radii": [mr, mr, mr], "transform": {"translation": [0.05, 0.05, 0.0]}, "member": "bearers", "part": "mass"})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[scale]}


def shapes_of(scale, parts):
    """🖊️ The three drawn views of one scale from the same numbers as the solids, plus the detail and texture strokes of the scale."""
    geo = {"elevation": [], "front": [], "plan": []}
    for p in parts:
        if p.get("texture"):
            continue
        for view in geo:
            g = shape_geometry(p, view)
            s = {"id": p["id"], "bone": p["bone"], "role": "body", "z": depth_order(p, view), "filled": True, "tone": p["tone"], "region": p["region"], "material": p["material"]}
            if scale == 500 and p["id"] == "equipment":
                s.pop("region")
                s["regions"] = ["primary-supply", "primary-return", "secondary-supply", "secondary-return", "exchanger", "shell", "meter-unit", "valve-levers"]
            if scale == 200 and not p["id"].startswith(("frame-", "bearer-", "exchanger")):
                s["penMm"] = 0.12
            if p["id"] == "exchanger-shell" and view != "elevation" and scale <= 100:
                s["hatch"] = "insulation"
            s.update(g)
            geo[view].append(s)
    geo["elevation"] += details_elevation(scale)
    geo["front"] += details_front(scale)
    geo["plan"] += details_plan(scale)
    return geo


def line(id, bone, pts, region, role="detail", z=5.0, tone=0.25, closed=False):
    """〰️ An unfilled detail or texture stroke."""
    return {"id": id, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "region": region, "kind": "poly", "points": [[r4(a), r4(b)] for a, b in pts], "closed": closed}


def hole(id, bone, cx, cy, r, region, z=5.0):
    """⭕️ An unfilled circle: a dowel hole, a nail hole."""
    return {"id": id, "bone": bone, "role": "detail", "z": z, "filled": False, "tone": 0.2, "region": region, "kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": r4(r), "ry": r4(r)}


def details_elevation(scale):
    """🔍️ Elevation strokes: wall-dowel holes of the dismantling, the type plate, the cut cable end, the end grain of the bearers."""
    out = []
    if scale >= 200:
        return out
    for side, x in (("left", 0.02), ("right", 0.78)):
        for k, y in enumerate((0.32, 1.18)):
            out.append(hole(f"dowel-hole-{side}-{k}", "station", x, y - Y0, 0.0055, "frame", z=2.0))
    for side, x in (("left", 0.05), ("right", 0.75)):
        out.append(line(f"bearer-end-grain-{side}", "bearers", [(x - 0.035, 0.02), (x - 0.01, 0.045), (x + 0.025, 0.05), (x + 0.04, 0.075)], "bearers", role="texture", z=3.6, tone=0.35))
    if scale == 50:
        out.append({"id": "type-plate", "bone": "station", "role": "detail", "z": 2.3, "filled": False, "tone": 0.3, "region": "exchanger", "kind": "rect", "x": 0.385, "y": r4(0.95 - Y0), "w": 0.07, "h": 0.04})
        out.append(line("type-plate-text", "station", [(0.392, 0.975 - Y0), (0.448, 0.975 - Y0)], "exchanger", role="texture", z=2.31))
        out.append(line("actuator-label", "station", [(0.065, 0.60 - Y0), (0.135, 0.60 - Y0)], "actuator", role="texture", z=3.0))
        out.append(line("cable-cut", "station", [(0.124, 0.40 - Y0), (0.136, 0.405 - Y0)], "actuator", z=2.9))
        out.append({"id": "calculator-display", "bone": "station", "role": "glazing", "z": 1.7, "filled": False, "tone": 0.6, "region": "meter-unit", "kind": "rect", "x": 0.155, "y": r4(0.80 - Y0), "w": 0.07, "h": 0.03})
    return out


def details_front(scale):
    """🔍️ Front strokes: every fifth plate of the exchanger pack, the grain and old nail holes of the bearers."""
    out = []
    if scale >= 200:
        return out
    if scale == 50:
        for k in range(1, 6):
            z = 0.02 + k * 0.08 / 6
            out.append(line(f"plate-line-{k}", "station", [(z, 0.575), (z, 1.091)], "exchanger", role="texture", z=1.85, tone=0.6))
        for side in ("left", "right"):
            for k, zc in enumerate((-0.22, 0.17)):
                out.append(hole(f"nail-hole-{side}-{k}", "bearers", zc, 0.06, 0.004, "bearers", z=1.3 if side == "left" else 2.6))
    for side, zz in (("left", 1.2), ("right", 2.5)):
        out.append(line(f"bearer-grain-{side}", "bearers", [(-0.29, 0.035), (-0.05, 0.03), (0.12, 0.045), (0.29, 0.04)], "bearers", role="texture", z=zz, tone=0.35))
    return out


def details_plan(scale):
    """🔍️ Plan strokes: the plate pack lines on the exchanger top and the saw cuts at the bearer ends."""
    out = []
    if scale != 50:
        return out
    for k in range(1, 6):
        z = 0.02 + k * 0.08 / 6
        out.append(line(f"plate-line-top-{k}", "station", [(0.365, z), (0.475, z)], "exchanger", role="texture", z=3.2, tone=0.6))
    return out


def rep_of(scale, parts):
    """🗺️ The authored representation of one scale."""
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}[scale]
    band = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    hatches = [{"id": "insulation", "title": t("EPP insulation, cut", "EPP-Dämmung, geschnitten"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5 if scale == 50 else 0.4, "penMm": 0.1}] if scale <= 100 else []
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": band[0], "maxDenominator": band[1]}, "pens": pens, "hatches": hatches, "geometry": shapes_of(scale, parts), "decisions": REP_DECISIONS[scale]}


def compute_station_mass(parts):
    """⚖️ The station bone mass and centre from the 1:50 parts (levers excluded)."""
    total = 0.0
    mx = my = mz = 0.0
    for p in parts:
        if p["bone"] != "station":
            continue
        m = mass_of(p)
        pts = samples(p)
        cx = sum(q[0] for q in pts) / len(pts)
        cy = sum(q[1] for q in pts) / len(pts)
        cz = sum(q[2] for q in pts) / len(pts)
        total += m
        mx += m * cx
        my += m * cy
        mz += m * cz
    return {"kg": total, "comX": mx / total, "comY": my / total - Y0, "comZ": mz / total}


P50 = parts_of(50)
STATION_MASS = compute_station_mass(P50)
BEARER_KG = REAL_MASS["bearer-left"] + REAL_MASS["bearer-right"]
TOTAL_KG = STATION_MASS["kg"] + BEARER_KG + sum(LEVER_KG.values())

MINIMAL_STRUCTURE = t("Minimal: the structure list is already the least load path of the station at 1:50 (five frame tubes, two bearers, two foot joints, two mass markers); it cannot lose a member at a coarser scale, so it keeps its count and its triangles while the body and thermal lists simplify.", "Minimal: die Tragwerksliste ist schon bei 1:50 der kleinste Lastpfad der Station (fünf Rahmenrohre, zwei Kanthölzer, zwei Fußgelenke, zwei Massenmarken); bei einem gröberen Maßstab kann sie keinen Stab verlieren, behält daher Anzahl und Dreiecke, während Körper- und Wärmeliste sich vereinfachen.")

VOLUME_DECISIONS = {
    50: [
        t("Body: the wall frame of two SHS 40 x 40 x 2 uprights and three crossbars (1.10 m high, 0.80 m wide) as solid boxes of the outer section; the brazed plate heat exchanger 119 x 526 x 80 mm with its four union nuts and the embossed type plate (texture); its rear EPP half-shell as a U-section extruded up the pack with top and bottom caps, the front half-shell is missing as delivered; four pipe runs (DN 20 primary, DN 32 secondary, EN 10255) as risers, headers in the pipe plane 50 mm in front of the frame and stubs into the ports, each cut and closed with a malleable-iron cap 30 mm above the bearers; four ball valves with stems and their levers on own bones; Y-strainer with its 45-degree screen pocket, the cast-iron control valve with bonnet and the 90 x 90 x 140 mm actuator with its cut cable, the ultrasonic flow sensor, the differential pressure controller with spindle and its 140 mm diaphragm housing, the 8 mm copper impulse tube in three straight runs, the dial thermometer, the heat meter calculator on the middle crossbar; six pipe clamps (band and M8 rod, the rods declared slender); the two 100 x 100 x 600 mm bearers.", "Körper: der Wandrahmen aus zwei Stielen und drei Riegeln MSH 40 x 40 x 2 (1,10 m hoch, 0,80 m breit) als volle Quader des Außenquerschnitts; der gelötete Plattenwärmeübertrager 119 x 526 x 80 mm mit seinen vier Überwurfmuttern und dem geprägten Typenschild (Textur); seine hintere EPP-Halbschale als U-Querschnitt, über die Pakethöhe extrudiert, mit Deckel und Boden, die vordere Halbschale fehlt wie angeliefert; vier Rohrstränge (DN 20 primär, DN 32 sekundär, EN 10255) als Steigleitungen, Querleitungen in der Rohrebene 50 mm vor dem Rahmen und Stutzen in die Anschlüsse, jeweils abgetrennt und 30 mm über den Kanthölzern mit einer Tempergusskappe verschlossen; vier Kugelhähne mit Spindeln und ihren Hebeln auf eigenen Knochen; Schmutzfänger mit seiner 45-Grad-Siebtasche, das Graugussregelventil mit Oberteil und dem Stellantrieb 90 x 90 x 140 mm mit abgeschnittenem Kabel, der Ultraschall-Durchflusssensor, der Differenzdruckregler mit Spindel und seinem 140-mm-Membrangehäuse, das 8-mm-Kupfer-Impulsrohr in drei geraden Stücken, das Zeigerthermometer, das Wärmezähler-Rechenwerk auf dem mittleren Riegel; sechs Rohrschellen (Band und M8-Stange, die Stangen als schlank erklärt); die beiden Kanthölzer 100 x 100 x 600 mm."),
        t("Thermal: every body solid except the type plate is its own thermal surface by reference, grouped into the eleven regions (frame with clamps, bearers, exchanger, shell, the four circuits with their fittings, actuator with cable, meter unit, valve levers); the plume outlet is a 7.5 mm patch on the free front half of the exchanger top.", "Thermisch: jeder Körper außer dem Typenschild ist per Verweis seine eigene thermische Fläche, gruppiert in die elf Regionen (Rahmen mit Schellen, Kanthölzer, Übertrager, Schale, die vier Kreise mit ihren Armaturen, Stellantrieb mit Kabel, Zählerrechenwerk, Kugelhahnhebel); der Auslass der Konvektionsfahne ist ein 7,5-mm-Feld auf der freien vorderen Hälfte der Übertrager-Oberseite."),
        t("Structure: the station member is the frame (five tubes by reference), the bearer member the two timbers; two joint pads where the uprights stand on the bearers carry the prismatic lift joint; mass markers at the centre of the station and of the bearers. The pipework is carried by the frame and is not a member.", "Tragwerk: der Stab der Station ist der Rahmen (fünf Rohre per Verweis), der Stab der Unterlage die beiden Kanthölzer; zwei Gelenkplatten, wo die Stiele auf den Kanthölzern stehen, tragen das prismatische Hubgelenk; Massenmarken im Schwerpunkt der Station und der Kanthölzer. Die Verrohrung wird vom Rahmen getragen und ist kein Stab."),
    ],
    100: [
        t("Body at 1:100 (15 mm feature): frame, bearers, exchanger, shell and pipe runs as at 1:50; the caps are merged into the pipe ends, the union nuts into the port stubs, the ball-valve stems into deeper valve bodies, the control-valve bonnet into a deeper actuator, the spindle into a deeper diaphragm housing, the thermometer stem into its dial; the levers are 15 mm thick; the impulse tube, the cut cable and the type plate are left out (below the feature and not slender enough).", "Körper bei 1:100 (kleinste Einzelheit 15 mm): Rahmen, Kanthölzer, Übertrager, Schale und Rohrstränge wie bei 1:50; die Kappen gehen in den Rohrenden auf, die Überwurfmuttern in den Anschlussstutzen, die Kugelhahnspindeln in tieferen Gehäusen, das Ventiloberteil in einem tieferen Stellantrieb, die Spindel in einem tieferen Membrangehäuse, der Thermometerschaft in seinem Zifferblatt; die Hebel sind 15 mm dick; Impulsrohr, abgeschnittenes Kabel und Typenschild entfallen (unter der Einzelheit und nicht schlank genug)."),
        t("Thermal and structure as at 1:50 on the merged solids; the plume patch is 15 mm thick.", "Thermik und Tragwerk wie bei 1:50 auf den zusammengefassten Körpern; das Fahnenfeld ist 15 mm dick."),
        MINIMAL_STRUCTURE,
    ],
    200: [
        t("Body at 1:200 (30 mm feature): frame tubes and bearers true; the DN 20 runs are thickened to 30 mm, the DN 32 runs stay true; the exchanger is its free front half (40 mm) in front of the shell, which becomes one 80 mm block; ball-valve bodies, strainer body, control valve and actuator, flow sensor, differential pressure controller with its housing, dial and calculator stay as compact bodies; levers, clamps, strainer pocket and every part below 30 mm are left out.", "Körper bei 1:200 (kleinste Einzelheit 30 mm): Rahmenrohre und Kanthölzer wahr; die DN-20-Stränge sind auf 30 mm verdickt, die DN-32-Stränge bleiben wahr; der Übertrager ist seine freie vordere Hälfte (40 mm) vor der Schale, die ein 80-mm-Block wird; Kugelhahngehäuse, Schmutzfänger, Regelventil und Stellantrieb, Durchflusssensor, Differenzdruckregler mit Gehäuse, Zifferblatt und Rechenwerk bleiben kompakte Körper; Hebel, Schellen, Siebtasche und alles unter 30 mm entfallen."),
        t("Thermal by reference on these solids, plume patch 30 mm; structure as at 1:50 with 30 mm joint pads and markers.", "Thermik per Verweis auf diesen Körpern, Fahnenfeld 30 mm; Tragwerk wie bei 1:50 mit 30-mm-Gelenkplatten und Marken."),
        MINIMAL_STRUCTURE,
    ],
    500: [
        t("Body at 1:500 (75 mm feature): the frame as two 75 mm uprights with the lower and top crossbars, the equipment in front of it as one mass 650 x 980 x 205 mm (pipes, fittings, exchanger and shell are below the feature apart), the actuator box standing out of it and the two bearers.", "Körper bei 1:500 (kleinste Einzelheit 75 mm): der Rahmen als zwei 75-mm-Stiele mit unterem und oberem Riegel, die Ausrüstung davor als eine Masse 650 x 980 x 205 mm (Rohre, Armaturen, Übertrager und Schale sind einzeln unter der Einzelheit), der herausstehende Stellantrieb und die beiden Kanthölzer."),
        t("Thermal: the equipment mass is split into 75 mm blocks per region (four circuit columns, exchanger, shell, meter unit, one lever block) so every region keeps a surface; frame, bearers and actuator by reference. Structure: frame and bearers as members by reference, 75 mm joint pads and markers.", "Thermisch: die Ausrüstungsmasse ist in 75-mm-Blöcke je Region aufgeteilt (vier Kreissäulen, Übertrager, Schale, Zählerrechenwerk, ein Hebelblock), damit jede Region eine Fläche behält; Rahmen, Kanthölzer und Stellantrieb per Verweis. Tragwerk: Rahmen und Kanthölzer als Stäbe per Verweis, 75-mm-Gelenkplatten und Marken."),
    ],
}


def decisions(sil, ret, mer, omi, thin, hat, sty, ov):
    """📝️ The eight decision texts of a representation."""
    return {"silhouette": sil, "retained": ret, "merged": mer, "omitted": omi, "thinElements": thin, "hatching": hat, "styles": sty, "overlays": ov}


REP_DECISIONS = {
    50: decisions(
        t("Side elevation from the equipment side: the 0.80 x 1.10 m frame (16 x 22 mm on paper) on its two bearer end faces, the bare plate heat exchanger in the rear half-shell, the four pipe runs with their capped stubs, valves with levers closed across the pipes, strainer, control valve with actuator, flow sensor, differential pressure controller and impulse tube, thermometer and calculator. Front view: the station in depth (0.42 m) on the long side of a bearer; plan: the frame top with everything that stands out of it.", "Seitenansicht von der Ausrüstungsseite: der Rahmen 0,80 x 1,10 m (16 x 22 mm auf Papier) auf den Stirnseiten seiner zwei Kanthölzer, der nackte Plattenwärmeübertrager in der hinteren Halbschale, die vier Rohrstränge mit ihren verkappten Stutzen, Kugelhähne mit quer stehenden (geschlossenen) Hebeln, Schmutzfänger, Regelventil mit Stellantrieb, Durchflusssensor, Differenzdruckregler und Impulsrohr, Thermometer und Rechenwerk. Vorderansicht: die Station in der Tiefe (0,42 m) auf der Längsseite eines Kantholzes; Draufsicht: der Rahmenkopf mit allem, was vorsteht."),
        t("Every fitting at its true place and size, the four caps, the union nuts, the six clamps, the wall-dowel holes in the uprights, the type plate, the cut actuator cable, every fifth plate of the pack.", "Jede Armatur an ihrem wahren Ort und in wahrer Größe, die vier Kappen, die Überwurfmuttern, die sechs Schellen, die Dübellöcher in den Stielen, das Typenschild, das abgeschnittene Stellantriebskabel, jede fünfte Platte des Pakets."),
        t("Tubes, valve bodies and housings are drawn as their outer outlines; the threads and hexagons are not drawn apart.", "Rohre, Ventilgehäuse und Gehäuse sind als ihre Außenkonturen gezeichnet; Gewinde und Sechskante werden nicht einzeln gezeichnet."),
        t("Threads, hexagon flats, screws of the clamps, the sensor cable of the meter and the plastic end plugs of the tubes (below 7.5 mm or hidden).", "Gewinde, Sechskantflächen, Schrauben der Schellen, das Fühlerkabel des Zählers und die Kunststoffstopfen der Rohre (unter 7,5 mm oder verdeckt)."),
        t("The 8 mm impulse tube, the 8 mm cable and the M8 clamp rods are drawn at their true width (0.16 mm) with the body pen.", "Das 8-mm-Impulsrohr, das 8-mm-Kabel und die M8-Schellenstangen sind in wahrer Breite (0,16 mm) mit der Konturfeder gezeichnet."),
        t("The cut faces of the EPP shell in front view and plan carry the 45-degree insulation hatch.", "Die Schnittflächen der EPP-Schale in Vorderansicht und Draufsicht tragen die 45-Grad-Dämmschraffur."),
        t("Outline 0.35 mm, details 0.18 mm, texture 0.1 mm, display glass 0.13 mm.", "Kontur 0,35 mm, Einzelheiten 0,18 mm, Textur 0,1 mm, Displayglas 0,13 mm."),
        t("Each part is its own cell with its region; levers on their own bones.", "Jedes Teil ist eine eigene Zelle mit seiner Region; Hebel auf eigenen Knochen."),
    ),
    100: decisions(
        t("The same three views at half the size (8 x 11 mm): frame, exchanger with shell, pipe runs, valves with levers, strainer, control valve and actuator, sensor, controller and housing, thermometer and calculator.", "Dieselben drei Ansichten in halber Größe (8 x 11 mm): Rahmen, Übertrager mit Schale, Rohrstränge, Kugelhähne mit Hebeln, Schmutzfänger, Regelventil und Stellantrieb, Sensor, Regler und Gehäuse, Thermometer und Rechenwerk."),
        t("Every fitting body, the levers, the clamps, the dowel holes and the bearer grain.", "Jedes Armaturengehäuse, die Hebel, die Schellen, die Dübellöcher und die Maserung der Kanthölzer."),
        t("Caps into the pipe ends, union nuts into the stubs, stems, bonnet and spindle into their bodies.", "Kappen in die Rohrenden, Überwurfmuttern in die Stutzen, Spindeln, Oberteil und Spindel in ihre Gehäuse."),
        t("Impulse tube, cable, type plate and plate lines.", "Impulsrohr, Kabel, Typenschild und Plattenlinien."),
        t("The M8 clamp rods are drawn at their true 8 mm width.", "Die M8-Schellenstangen sind in wahrer Breite von 8 mm gezeichnet."),
        t("Insulation hatch on the shell in front view and plan.", "Dämmschraffur auf der Schale in Vorderansicht und Draufsicht."),
        t("Outline 0.25 mm, details 0.13 mm, texture 0.1 mm.", "Kontur 0,25 mm, Einzelheiten 0,13 mm, Textur 0,1 mm."),
        t("Each part is its own cell.", "Jedes Teil ist eine eigene Zelle."),
    ),
    200: decisions(
        t("Frame, bearers, the exchanger front in front of the shell block, four pipe runs at least 30 mm wide and the compact fitting bodies (4 x 5.5 mm on paper).", "Rahmen, Kanthölzer, die Übertragerfront vor dem Schalenblock, vier mindestens 30 mm breite Rohrstränge und die kompakten Armaturengehäuse (4 x 5,5 mm auf Papier)."),
        t("Ball-valve bodies, strainer, control valve with actuator, sensor, controller with housing, thermometer dial, calculator.", "Kugelhahngehäuse, Schmutzfänger, Regelventil mit Stellantrieb, Sensor, Regler mit Gehäuse, Thermometerzifferblatt, Rechenwerk."),
        t("Shell walls and the rear half of the pack are one block.", "Schalenwände und die hintere Hälfte des Pakets sind ein Block."),
        t("Levers, clamps, strainer pocket, all strokes.", "Hebel, Schellen, Siebtasche, alle Striche."),
        t("The DN 20 runs are drawn 30 mm wide (0.15 mm).", "Die DN-20-Stränge sind 30 mm breit gezeichnet (0,15 mm)."),
        t("No hatch.", "Keine Schraffur."),
        t("Frame, bearers and exchanger outline 0.18 mm; pipework and fittings 0.12 mm, so the 98 mm gaps between the secondary pipes and between the valves still print as white at 1:350.", "Rahmen, Kanthölzer und Übertrager Kontur 0,18 mm; Verrohrung und Armaturen 0,12 mm, damit die 98-mm-Lücken zwischen den Sekundärrohren und zwischen den Armaturen auch bei 1:350 weiß drucken."),
        t("Each part is its own cell.", "Jedes Teil ist eine eigene Zelle."),
    ),
    500: decisions(
        t("The station as frame uprights, lower and top crossbars and the equipment mass in front of them, the actuator standing out, on the two bearers (1.6 x 2.4 mm on paper).", "Die Station als Rahmenstiele, unterer und oberer Riegel und die Ausrüstungsmasse davor, der vorstehende Stellantrieb, auf den beiden Kanthölzern (1,6 x 2,4 mm auf Papier)."),
        t("Overall width, height and depth, the bearers apart.", "Gesamtbreite, -höhe und -tiefe, die Kanthölzer getrennt."),
        t("Pipes, fittings, exchanger and shell are one equipment mass that names all their regions.", "Rohre, Armaturen, Übertrager und Schale sind eine Ausrüstungsmasse, die alle ihre Regionen nennt."),
        t("Every fitting and stroke.", "Jede Armatur und jeder Strich."),
        t("None.", "Keine."),
        t("No hatch.", "Keine Schraffur."),
        t("Outline 0.13 mm.", "Kontur 0,13 mm."),
        t("Frame, equipment, actuator and bearers are separate cells.", "Rahmen, Ausrüstung, Stellantrieb und Kanthölzer sind getrennte Zellen."),
    ),
}


def region_area(region):
    """📐️ The outer area of a region from its 1:50 solids (sum of solid areas less the hidden joints between them)."""
    total = 0.0
    for p in P50:
        if p["region"] == region and not p.get("texture"):
            total += area_of(p)
    return total


def build():
    """🏗️ Assembles the whole asset document."""
    hyd_p = [{"v": 0.5, "supplyTempC": 80, "returnTempC": 45}, {"v": 1, "supplyTempC": 90, "returnTempC": 50}]
    hyd_s = [{"v": 0.5, "supplyTempC": 55, "returnTempC": 42}, {"v": 1, "supplyTempC": 70, "returnTempC": 50}]
    areas = AREAS

    def region(id, title, kind, model, material, L, geom, hc, ref, **extra):
        r = {"id": id, "title": title, "kind": kind, "model": model, "material": material, "areaM2": round(areas[id], 4), "characteristicLengthM": L, "geometry": geom, "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": ref}
        r.update(extra)
        return r

    regions = [
        region("frame", t("Galvanised wall frame with pipe clamps", "Verzinkter Wandrahmen mit Rohrschellen"), "surface", "passive", "galvanised-steel", 0.04, "cylinder", 7300, "en-10219-2"),
        region("bearers", t("Weathered squared-timber bearers", "Bewitterte Kanthölzer"), "surface", "passive", "weathered-softwood", 0.1, "vertical-plate", 24000, "en-iso-10456"),
        region("exchanger", t("Brazed plate heat exchanger, bare front half", "Gelöteter Plattenwärmeübertrager, nackte vordere Hälfte"), "heater", "hydronic", "stainless-steel", 0.526, "vertical-plate", 11000, "bphe-catalogue", ratedPowerW=0, ratedPowerBasis="passive", hydronic={"key": "heating", "rows": hyd_p, "flowPosition": 0.6, "filmM2KW": 0.001}),
        region("shell", t("Rear EPP half-shell of the exchanger", "Hintere EPP-Halbschale des Übertragers"), "surface", "hydronic", "epp-foam", 0.576, "vertical-plate", 2400, "en-iso-10456", layers=[{"id": "shell-epp", "title": t("EPP back wall 40 mm", "EPP-Rückwand 40 mm"), "kind": "insulation", "material": "epp-foam", "thicknessM": 0.04, "resistanceM2KW": 1.05, "heatCapacityJM2K": 4300, "source": "literature", "reference": "en-iso-10456"}], hydronic={"key": "heating", "rows": hyd_p, "flowPosition": 0.6, "filmM2KW": 0.001}),
        region("primary-supply", t("Primary supply: pipe, ball valve, strainer, control valve, impulse tube", "Primärvorlauf: Rohr, Kugelhahn, Schmutzfänger, Regelventil, Impulsrohr"), "heater", "hydronic", "black-steel-pipe", 0.027, "cylinder", 9600, "agfw-fw-515", hydronic={"key": "heating", "rows": hyd_p, "flowPosition": 0, "filmM2KW": 0.002}),
        region("primary-return", t("Primary return: pipe, ball valve, heat meter sensor, differential pressure controller", "Primärrücklauf: Rohr, Kugelhahn, Wärmezähler-Sensor, Differenzdruckregler"), "heater", "hydronic", "black-steel-pipe", 0.027, "cylinder", 9600, "agfw-fw-515", hydronic={"key": "heating", "rows": hyd_p, "flowPosition": 1, "filmM2KW": 0.002}),
        region("secondary-supply", t("Secondary supply: pipe, ball valve, thermometer", "Sekundärvorlauf: Rohr, Kugelhahn, Thermometer"), "heater", "hydronic", "black-steel-pipe", 0.042, "cylinder", 11800, "agfw-fw-515", hydronic={"key": "heating", "rows": hyd_s, "flowPosition": 0, "filmM2KW": 0.002}),
        region("secondary-return", t("Secondary return: pipe and ball valve", "Sekundärrücklauf: Rohr und Kugelhahn"), "heater", "hydronic", "black-steel-pipe", 0.042, "cylinder", 11800, "agfw-fw-515", hydronic={"key": "heating", "rows": hyd_s, "flowPosition": 1, "filmM2KW": 0.002}),
        region("actuator", t("Valve actuator 24 V with cut cable", "Stellantrieb 24 V mit abgeschnittenem Kabel"), "surface", "operating", "abs-housing", 0.09, "vertical-plate", 4400, "actuator-catalogue", operating={"key": "heating", "heatW": 3}, ratedPowerW=5, ratedPowerBasis="electrical-input"),
        region("meter-unit", t("Heat meter calculator, battery powered", "Wärmezähler-Rechenwerk, batteriebetrieben"), "surface", "passive", "abs-housing", 0.12, "vertical-plate", 4400, "en-1434-1"),
        region("valve-levers", t("Ball-valve levers", "Kugelhahnhebel"), "surface", "passive", "coated-steel", 0.02, "vertical-plate", 15000, "ashrae-2021"),
    ]
    stored_regions = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGIONS_ORDER}
    stored_regions["bearers"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.23, "tempC": 12}}
    lifted_regions = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGIONS_ORDER}
    lifted_regions["bearers"] = stored_regions["bearers"]
    wall_regions = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGIONS_ORDER}
    wall_regions["frame"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.18, "tempC": 18}}
    wall_regions["bearers"] = stored_regions["bearers"]
    water = {"id": "water-filling", "kind": "carried", "title": t("Water filling of exchanger and pipes, 5.9 kg", "Wasserfüllung von Übertrager und Rohren, 5,9 kg"), "anchor": "water-centre", "fxN": 0, "fyN": -57.9, "source": "authored-estimate", "reference": "bphe-catalogue"}
    open_levers = {"lever-p1": {"angleDeg": 90}, "lever-p2": {"angleDeg": 90}, "lever-s2": {"angleDeg": -90}, "lever-s1": {"angleDeg": -90}}
    wall_bones = {"carrier": {"dx": 1.1}, "station": {"dy": 0.5}}
    poses = [
        {"id": "stored", "title": t("Stored on bearers", "Auf Kanthölzern gelagert"), "description": t("Drained and capped, the valves closed, the station stands on its two squared-timber bearers in the component exchange yard.", "Entleert und verkappt, die Kugelhähne geschlossen, steht die Station auf ihren zwei Kanthölzern im Lager der Bauteilbörse."), "bones": {}, "ground": "authored", "contacts": ["bearer-left", "bearer-right"], "thermal": {"operating": {"heating": 0}, "regions": stored_regions, "emitters": {"exchanger-plume": 0}}, "loads": []},
        {"id": "lifted", "title": t("Lifted on two round slings", "An zwei Rundschlingen angehoben"), "description": t("Two round slings choked round the top crossbar lift the station 0.30 m clear; the bearers stay on the ground.", "Zwei um den oberen Riegel geschnürte Rundschlingen heben die Station 0,30 m an; die Kanthölzer bleiben am Boden."), "bones": {"station": {"dy": 0.3}}, "ground": "authored", "contacts": ["sling-left", "sling-right"], "thermal": {"operating": {"heating": 0}, "regions": lifted_regions, "emitters": {"exchanger-plume": 0}}, "loads": []},
        {"id": "reinstalled", "title": t("Reinstalled and heating at 60 kW", "Wieder eingebaut, heizt mit 60 kW"), "description": t("Hung on four dowels on a plant-room wall 0.60 m above the floor, filled and connected, valves open: primary 90/50 °C, secondary 70/50 °C at the rated 60 kW.", "An vier Dübeln an einer Technikraumwand 0,60 m über dem Boden aufgehängt, gefüllt und angeschlossen, Kugelhähne offen: primär 90/50 °C, sekundär 70/50 °C bei der Nennleistung von 60 kW."), "bones": {**wall_bones, **open_levers}, "ground": "authored", "contacts": ["wall-upper-left", "wall-upper-right", "wall-lower-left", "wall-lower-right"], "thermal": {"operating": {"heating": 1}, "regions": wall_regions, "emitters": {"exchanger-plume": 1}}, "loads": [water]},
        {"id": "reinstalled-off", "title": t("Reinstalled, shut off", "Wieder eingebaut, abgesperrt"), "description": t("On the wall and filled, the four ball valves closed: the station cools to the room.", "An der Wand und gefüllt, die vier Kugelhähne geschlossen: die Station kühlt auf Raumtemperatur ab."), "bones": dict(wall_bones), "ground": "authored", "contacts": ["wall-upper-left", "wall-upper-right", "wall-lower-left", "wall-lower-right"], "thermal": {"operating": {"heating": 0}, "regions": wall_regions, "emitters": {"exchanger-plume": 0}}, "loads": [water]},
    ]
    clips = [
        {"id": "lift", "title": t("Lift off the bearers", "Von den Kanthölzern abheben"), "description": t("The slings take the load, then the station rises 0.30 m while the bearers stay on the ground.", "Die Schlingen nehmen die Last auf, dann hebt sich die Station um 0,30 m, während die Kanthölzer am Boden bleiben."), "durationS": 6, "fps": 10, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "stored", "ease": "step"}, {"t": 1, "pose": "stored", "ease": "smooth", "bones": {"station": {"dy": 0}}}, {"t": 6, "pose": "lifted", "ease": "smooth"}], "contacts": [{"anchor": "bearer-left", "fromS": 0, "toS": 1}, {"anchor": "bearer-right", "fromS": 0, "toS": 1}, {"anchor": "sling-left", "fromS": 1, "toS": 6}, {"anchor": "sling-right", "fromS": 1, "toS": 6}], "phases": [{"id": "slinging", "fromS": 0, "toS": 1, "title": t("Slings tensioned", "Schlingen gespannt")}, {"id": "hoist", "fromS": 1, "toS": 6, "title": t("Hoist", "Heben")}], "events": [{"t": 1, "kind": "contact-off", "title": t("Frame leaves the bearers", "Rahmen verlässt die Kanthölzer"), "target": "bearer-left"}, {"t": 1, "kind": "contact-on", "title": t("Slings carry the station", "Schlingen tragen die Station"), "target": "sling-left"}], "tracks": []},
        {"id": "commissioning", "title": t("Commissioning after reinstallation", "Inbetriebnahme nach dem Wiedereinbau"), "description": t("The four ball valves are opened, the control valve opens and the exchanger, the pipes and the plume above them warm up to the rated state within a minute and a half.", "Die vier Kugelhähne werden geöffnet, das Regelventil öffnet, und Übertrager, Rohre und die Konvektionsfahne darüber erwärmen sich binnen anderthalb Minuten auf den Nennzustand."), "durationS": 120, "fps": 5, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "smooth"}, {"t": 8, "pose": "reinstalled", "ease": "smooth"}, {"t": 120, "pose": "reinstalled", "ease": "step"}], "contacts": [{"anchor": a, "fromS": 0, "toS": 120} for a in ("wall-upper-left", "wall-upper-right", "wall-lower-left", "wall-lower-right")], "phases": [{"id": "open-valves", "fromS": 0, "toS": 8, "title": t("Open the ball valves", "Kugelhähne öffnen")}, {"id": "warm-up", "fromS": 8, "toS": 120, "title": t("Warm-up", "Aufheizen")}], "events": [{"t": 8, "kind": "operating", "title": t("Control valve opens", "Regelventil öffnet"), "target": "heating"}], "tracks": [{"target": "operating:heating", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 8, "v": 0}, {"t": 90, "v": 1}, {"t": 120, "v": 1}]}, {"target": "emitter:exchanger-plume", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 8, "v": 0}, {"t": 90, "v": 1}, {"t": 120, "v": 1}]}]},
    ]
    sm = STATION_MASS
    rig = {"kind": "mechanical", "root": "bearers", "bones": [
        {"id": "bearers", "parent": None, "title": t("Two squared-timber bearers (stay on the ground)", "Zwei Kanthölzer (bleiben am Boden)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.8, "joint": {"type": "fixed"}, "mass": {"kg": round(BEARER_KG, 3), "comX": 0.4, "comY": 0.05, "distribution": "point", "source": "literature", "reference": "en-338"}, "section": {"areaM2": 0.01, "secondMomentM4": 8.333e-06, "extremeFibreM": 0.05, "material": "weathered-softwood", "title": t("Squared timber 100 x 100 mm", "Kantholz 100 x 100 mm")}},
        {"id": "carrier", "parent": "bearers", "title": t("Yard carrier (massless, moves the station to its new wall)", "Hofträger (masselos, bringt die Station an ihre neue Wand)"), "x": 0, "y": Y0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.5}},
        {"id": "station", "parent": "carrier", "title": t("Substation on its wall frame (lifted by the slings)", "Übergabestation auf ihrem Wandrahmen (von den Schlingen gehoben)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.8, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.6}, "mass": {"kg": round(sm["kg"], 3), "comX": round(sm["comX"], 4), "comY": round(sm["comY"], 4), "distribution": "point", "source": "authored-estimate", "reference": "en-10219-2"}, "section": {"areaM2": 0.000588, "secondMomentM4": 1.388e-07, "extremeFibreM": 0.02, "material": "galvanised-steel", "title": t("Two SHS 40 x 40 x 2 uprights", "Zwei Stiele MSH 40 x 40 x 2")}},
    ]}
    for name in ("p1", "p2", "s2", "s1"):
        b = BONES[f"lever-{name}"]
        rig["bones"].append({"id": f"lever-{name}", "parent": "station", "title": t(f"Ball-valve lever {name.upper()} (across the pipe = closed)", f"Kugelhahnhebel {name.upper()} (quer zum Rohr = geschlossen)"), "x": b["o"][0], "y": round(b["o"][1] - Y0, 4), "angleDeg": b["a"], "lengthM": 0.04, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 90} if b["a"] == 0 else {"type": "revolute", "minDeg": -90, "maxDeg": 0}, "mass": {"kg": LEVER_KG[f"lever-{name}"], "comX": 0.045, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "ball-valve-catalogue"}})
    anchors = [
        {"id": "bearer-left", "bone": "bearers", "x": 0.05, "y": 0, "kind": "contact", "title": t("Left bearer on the yard floor", "Linkes Kantholz auf dem Hofboden"), "support": "contact", "footprintLengthM": 0.1, "footprintWidthM": 0.6, "frictionCoefficient": 0.5},
        {"id": "bearer-right", "bone": "bearers", "x": 0.75, "y": 0, "kind": "contact", "title": t("Right bearer on the yard floor", "Rechtes Kantholz auf dem Hofboden"), "support": "contact", "footprintLengthM": 0.1, "footprintWidthM": 0.6, "frictionCoefficient": 0.5},
        {"id": "sling-left", "bone": "station", "x": 0.12, "y": 1.1, "z": 0.16, "kind": "support", "title": t("Left round sling round the top crossbar", "Linke Rundschlinge um den oberen Riegel"), "support": "pin"},
        {"id": "sling-right", "bone": "station", "x": 0.68, "y": 1.1, "z": 0.16, "kind": "support", "title": t("Right round sling round the top crossbar", "Rechte Rundschlinge um den oberen Riegel"), "support": "pin"},
        {"id": "wall-upper-left", "bone": "station", "x": 0.02, "y": 1.08, "z": 0.18, "kind": "support", "title": t("Upper left wall dowel, the frame hangs on it", "Oberer linker Wanddübel, der Rahmen hängt daran"), "support": "pin"},
        {"id": "wall-upper-right", "bone": "station", "x": 0.78, "y": 1.08, "z": 0.18, "kind": "support", "title": t("Upper right wall dowel", "Oberer rechter Wanddübel"), "support": "pin"},
        {"id": "wall-lower-left", "bone": "station", "x": 0.02, "y": 0.22, "z": 0.18, "kind": "support", "title": t("Lower left wall dowel, presses the frame against the wall", "Unterer linker Wanddübel, drückt den Rahmen an die Wand"), "support": "roller", "normalDeg": 90, "tiltDeg": -90},
        {"id": "wall-lower-right", "bone": "station", "x": 0.78, "y": 0.22, "z": 0.18, "kind": "support", "title": t("Lower right wall dowel", "Unterer rechter Wanddübel"), "support": "roller", "normalDeg": 90, "tiltDeg": -90},
        {"id": "water-centre", "bone": "station", "x": 0.42, "y": 0.73, "z": 0.04, "kind": "load", "title": t("Centre of the water filling", "Schwerpunkt der Wasserfüllung")},
        {"id": "plume-top", "bone": "station", "x": 0.42, "y": round(1.096 - Y0 + 0.004, 4), "z": 0.04, "kind": "emitter", "title": t("Top of the bare exchanger front", "Oberseite der nackten Übertragerfront")},
    ]
    thermal = {
        "environment": {"airTempC": 18, "meanRadiantTempC": 18, "skyTempC": 18, "groundTempC": 15, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [],
        "operating": [{"id": "heating", "title": t("District heat through the station", "Fernwärme durch die Station"), "levels": [{"v": 0, "title": t("Off: valves closed or drained", "Aus: Kugelhähne zu oder entleert")}, {"v": 0.5, "title": t("Part load, primary 80/45 °C, secondary 55/42 °C", "Teillast, primär 80/45 °C, sekundär 55/42 °C")}, {"v": 1, "title": t("Rated 60 kW, primary 90/50 °C, secondary 70/50 °C", "Nennleistung 60 kW, primär 90/50 °C, sekundär 70/50 °C")}]}],
        "regions": regions,
        "emitters": [{"id": "exchanger-plume", "kind": "plume", "title": t("Free-convection plume over the bare exchanger", "Freie Konvektionsfahne über dem nackten Übertrager"), "anchor": "plume-top", "directionDeg": 90, "diameterM": 0.1, "region": "exchanger", "source": "authored-estimate", "reference": "bphe-catalogue"}],
        "assumptions": [
            t("Plant room at 18 °C, still air; the yard poses use the same air for comparison. Rated state: primary 90/50 °C and 0.36 kg/s, secondary 70/50 °C and 0.72 kg/s for 60 kW; part load 80/45 and 55/42 °C (AGFW FW 515 typical connection values).", "Technikraum mit 18 °C, ruhende Luft; die Lagerposen verwenden zum Vergleich dieselbe Luft. Nennzustand: primär 90/50 °C und 0,36 kg/s, sekundär 70/50 °C und 0,72 kg/s für 60 kW; Teillast 80/45 und 55/42 °C (typische Anschlusswerte nach AGFW FW 515)."),
            t("Weathered and aged surfaces: dull galvanising 0.28 instead of 0.07 new, tarnished stainless 0.35 instead of 0.15 polished, limescaled brass 0.6, primed pipe 0.9 (ASHRAE Fundamentals, chapter 4, typical values).", "Bewitterte und gealterte Oberflächen: matte Verzinkung 0,28 statt 0,07 neu, angelaufener Edelstahl 0,35 statt 0,15 poliert, verkalktes Messing 0,6, grundiertes Rohr 0,9 (ASHRAE Fundamentals, Kapitel 4, typische Werte)."),
            t("The pipes are bare: their insulation was cut off during dismantling. The front EPP half-shell of the exchanger is missing; the rear one stays on and its outer face is reached through 40 mm of EPP.", "Die Rohre sind nackt: ihre Dämmung wurde beim Rückbau abgeschnitten. Die vordere EPP-Halbschale des Übertragers fehlt; die hintere bleibt, ihre Außenseite liegt hinter 40 mm EPP."),
            t("Nameplate: only the 24 V actuator draws power (5 VA, counted as 5 W electrical input); the heat exchanger, valves and pipes are fed from the network and are passive; the heat meter runs on its battery.", "Typenschild: nur der 24-V-Stellantrieb nimmt Leistung auf (5 VA, als 5 W elektrische Aufnahme gezählt); Übertrager, Armaturen und Rohre werden vom Netz gespeist und sind passiv; der Wärmezähler läuft mit seiner Batterie."),
        ],
    }
    structure = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
        t("Rigid body on the supports of each pose: two unilateral bearer contacts when stored, two sling pins on the top crossbar when lifted, two upper dowel pins and two lower dowel rollers against the wall when reinstalled. The bearers are part of the rigid body in every pose, so their 5 kg hang in the slings when lifted.", "Starrer Körper auf den Auflagern jeder Pose: zwei einseitige Kantholzkontakte bei der Lagerung, zwei Schlingenbolzen am oberen Riegel beim Anheben, zwei obere Dübel als Festlager und zwei untere Dübel als Rollenlager gegen die Wand nach dem Wiedereinbau. Die Kanthölzer gehören in jeder Pose zum starren Körper, ihre 5 kg hängen daher beim Anheben in den Schlingen."),
        t("Masses drained: frame 2.31 kg/m (SHS 40 x 40 x 2, EN 10219-2), pipes 1.56 and 3.09 kg/m (EN 10255 medium, DN 20 and DN 32), exchanger 7 kg (30 plates, catalogue typical), bearers 420 kg/m³ (C24, EN 338); the 5.9 kg water filling is a load of the reinstalled poses.", "Massen entleert: Rahmen 2,31 kg/m (MSH 40 x 40 x 2, EN 10219-2), Rohre 1,56 und 3,09 kg/m (EN 10255 mittelschwer, DN 20 und DN 32), Übertrager 7 kg (30 Platten, katalogtypisch), Kanthölzer 420 kg/m³ (C24, EN 338); die Wasserfüllung von 5,9 kg ist eine Last der eingebauten Posen."),
        t("The two slings sit on one line along the crossbar, 0.12 m behind the centre of mass in depth: the rigid model carries the weight but not the small moment about that line; in reality the hanging station leans about 7 degrees forward.", "Die beiden Schlingen liegen auf einer Linie entlang des Riegels, 0,12 m hinter dem Schwerpunkt in der Tiefe: das starre Modell trägt das Gewicht, nicht aber das kleine Moment um diese Linie; in Wirklichkeit neigt sich die hängende Station um etwa 7 Grad nach vorn."),
    ], "clinical": False}
    provenance = {"author": "Semio staffage component authors (used HVAC wave)", "method": "handcrafted", "created": "2026-11-22", "references": [
        {"id": "din-4747-1", "citation": "DIN 4747-1:2003 Fernwärmeanlagen, sicherheitstechnische Ausrüstung von Unterstationen, Hausstationen und Hausanlagen (district heating substations: safety equipment, temperature and pressure limitation)."},
        {"id": "agfw-fw-515", "citation": "AGFW FW 515 (2015) Technische Anschlussbedingungen Heizwasser (TAB): typical primary supply 90 to 110 °C, return limit 50 °C, indirect connection through a plate heat exchanger."},
        {"id": "en-1434-1", "citation": "EN 1434-1:2022 Thermal energy meters, part 1: general requirements (qp 1.5 m³/h, flow sensor length 110 mm G3/4B)."},
        {"id": "en-10255", "citation": "EN 10255:2004 Non-alloy steel tubes suitable for welding and threading, medium series: DN 20 26.9 x 2.6 mm 1.56 kg/m, DN 32 42.4 x 3.2 mm 3.09 kg/m."},
        {"id": "en-10219-2", "citation": "EN 10219-2:2006 Cold formed welded structural hollow sections: SHS 40 x 40 x 2.0, A = 2.94 cm², 2.31 kg/m, I = 6.94 cm⁴."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 mean density 420 kg/m³, E0,mean 11 GPa, fm,k 24 MPa."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (softwood 0.13 W/mK, 1600 J/kgK; expanded polymer foams about 0.035 to 0.04 W/mK)."},
        {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4 heat transfer and chapter 33 physical properties: typical emissivities of galvanised, stainless, brass, oxidised copper and painted surfaces (rounded typical values, not table look-ups)."},
        {"id": "bphe-catalogue", "citation": "Brazed plate heat exchanger catalogues of the major manufacturers (plate 119 x 526 mm, port distance 466 x 63 mm, pack depth 13 + 2.24 n mm, about 7 kg empty for 30 plates, hold-up about 1.5 l per side): manufacturer-typical data, no single product."},
        {"id": "actuator-catalogue", "citation": "Catalogues of 24 V modulating valve actuators for district heating control valves DN 15 to DN 25 (about 90 x 90 x 140 mm, 5 VA, 0.6 kg): manufacturer-typical data."},
        {"id": "ball-valve-catalogue", "citation": "Catalogues of full-bore brass ball valves PN 25 with steel lever, DN 20 and DN 32: manufacturer-typical dimensions and masses."},
        {"id": "en-1492-2", "citation": "EN 1492-2:2000+A1:2008 Textile slings, round slings made of man-made fibres for general purpose use (choke hitch mode factor 0.8)."},
    ], "notes": t("A compact indirect district heating substation for 60 kW as removed from a 1990s apartment building: wall frame 0.80 x 1.10 m, bearers 100 x 100 x 600 mm. Drained, the four connections cut and capped, the pipe insulation and the front exchanger half-shell missing, the actuator cable cut, four dowel holes in the uprights.", "Eine kompakte indirekte Fernwärme-Übergabestation für 60 kW, wie sie aus einem Mehrfamilienhaus der 1990er Jahre ausgebaut wurde: Wandrahmen 0,80 x 1,10 m, Kanthölzer 100 x 100 x 600 mm. Entleert, die vier Anschlüsse abgetrennt und verkappt, Rohrdämmung und vordere Übertrager-Halbschale fehlen, das Stellantriebskabel ist abgeschnitten, vier Dübellöcher in den Stielen.")}
    parts = {s: (P50 if s == 50 else parts_of(s)) for s in SCALES}
    asset = {
        "id": "used-district-heating-substation", "category": "components",
        "title": t("Used District Heating Substation", "Gebrauchte Fernwärme-Übergabestation"),
        "summary": t("A dismantled 60 kW compact district heating substation on its galvanised wall frame, standing on two squared-timber bearers: brazed plate heat exchanger in the remaining rear half-shell, differential pressure controller, motorised control valve, ultrasonic heat meter, strainer and four ball valves, drained with the four stubs cut and capped. It can be lifted on two round slings and hung on a new wall, where it transfers district heat at 90/50 °C to a heating circuit at 70/50 °C.", "Eine rückgebaute kompakte Fernwärme-Übergabestation für 60 kW auf ihrem verzinkten Wandrahmen, auf zwei Kanthölzern stehend: gelöteter Plattenwärmeübertrager in der verbliebenen hinteren Halbschale, Differenzdruckregler, motorisches Regelventil, Ultraschall-Wärmezähler, Schmutzfänger und vier Kugelhähne, entleert, die vier Stutzen abgetrennt und verkappt. Sie lässt sich an zwei Rundschlingen anheben und an einer neuen Wand aufhängen, wo sie Fernwärme mit 90/50 °C an einen Heizkreis mit 70/50 °C überträgt."),
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "controls", "district-heating", "substation", "plate-heat-exchanger", "steel", "brass", "timber"],
        "provenance": provenance,
        "dimensions": {"heightM": 1.2, "widthM": 0.8, "depthM": 0.6, "massKg": round(TOTAL_KG, 3)},
        "materials": MATERIALS, "rig": rig,
        "volumes": {str(s): volume_of_scale(s, parts[s]) for s in SCALES},
        "anchors": anchors,
        "representations": {str(s): rep_of(s, parts[s]) for s in SCALES},
        "symbols": [], "poses": poses, "clips": clips, "thermal": thermal, "structure": structure,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


AREA_OVERRIDE = {"frame": 0.758, "bearers": 0.52, "exchanger": 0.228, "shell": 0.367, "primary-supply": 0.164, "primary-return": 0.168, "secondary-supply": 0.205, "secondary-return": 0.114, "actuator": 0.07, "meter-unit": 0.042, "valve-levers": 0.032}
AREAS = {r: AREA_OVERRIDE.get(r, region_area(r)) for r in REGIONS_ORDER}

if __name__ == "__main__":
    doc = build()
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print("[DEBUG] station", {k: round(v, 4) for k, v in STATION_MASS.items()}, "bearers", round(BEARER_KG, 3), "total", round(TOTAL_KG, 3))
    print("[DEBUG] areas", {k: round(v, 4) for k, v in AREAS.items()})
    for s in SCALES:
        v = doc["asset"]["volumes"][str(s)]
        print("[DEBUG] scale", s, "body", len(v["body"]), "thermal", len(v["thermal"]), "structure", len(v["structure"]))
