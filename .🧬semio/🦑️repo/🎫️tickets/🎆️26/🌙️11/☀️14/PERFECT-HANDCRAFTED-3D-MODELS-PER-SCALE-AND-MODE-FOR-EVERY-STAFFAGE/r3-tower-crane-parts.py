"""🧩 The assemblies of `machinery/tower-crane` per scale (round 3): cross-base with ballast, slewing ring and drives, platform and cab console, operator cab with its glazing, triangular jib, counter-jib, pendants, counterweight, trolley, hook block, concrete skip and the two-fall hoist rope. Each function returns the body, thermal and structure solids of one scale and the 2D shapes of the three viewpoints, stated by hand from the numbers of the kit.

@see ./r3-tower-crane-kit.py
@see ./r3-tower-crane-build.py
"""
import math
import os
from importlib import util

_s = util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-tower-crane-kit.py"))
K = util.module_from_spec(_s)
_s.loader.exec_module(K)

P, box, cyl, rev, ext, r = K.P, K.box, K.cyl, K.rev, K.ext, K.r
STEEL, CONC, MACH, GLASS, ROPE, SKIP = K.STEEL, K.CONC, K.MACH, K.GLASS, K.ROPE, K.SKIP


class Part:
    """📦 Solids and drawings of one assembly at one scale."""

    def __init__(self):
        self.body, self.thermal, self.structure = [], [], []
        self.views = {"elevation": [], "front": [], "plan": []}

    def add(self, other):
        self.body += other.body
        self.thermal += other.thermal
        self.structure += other.structure
        for v in self.views:
            self.views[v] += other.views[v]
        return self

    def ref(self, s, region, **kw):
        self.thermal.append({"id": "th-" + s["id"], "of": s["id"], "region": region, **kw})

    def member(self, s, bone, part="member"):
        self.structure.append({"id": ("member-" if part == "member" else part + "-") + s["id"], "of": s["id"], "member": bone, "part": part})

    def draw(self, shape, views=("elevation", "front", "plan")):
        for v in views:
            self.views[v].append(dict(shape))


def shape(kind, sid, bone, role, z, filled, tone, region=None, material=None, solid=None, **geom):
    s = {"kind": kind}
    s.update({k: (P(*v) if isinstance(v, list) and v and not isinstance(v[0], list) else ([P(*q) for q in v] if isinstance(v, list) else (r(v) if isinstance(v, float) else v))) for k, v in geom.items()})
    s.update({"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone})
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    if solid:
        s["solid"] = solid
    return s


def ex(z0, z1):
    return {"kind": "extruded", "sweep": "w", "z0": r(z0), "z1": r(z1)}


def rv(bone, axisX, axisZ=0.0):
    return {"kind": "revolved", "axisBone": bone, "axis": "v", "axisX": r(axisX), "axisZ": r(axisZ)}


def rects(rs):
    return " ".join(f"M {r(x0)},{r(y0)} L {r(x1)},{r(y0)} L {r(x1)},{r(y1)} L {r(x0)},{r(y1)} Z" for x0, y0, x1, y1 in rs)


def circle(cx, cy, rad, n=16):
    return [[r(cx + rad * math.cos(2 * math.pi * k / n)), r(cy + rad * math.sin(2 * math.pi * k / n))] for k in range(n)]


def circles(cs, n=16):
    out = []
    for cx, cy, rad in cs:
        pts = circle(cx, cy, rad, n)
        out.append("M " + " L ".join(f"{x},{y}" for x, y in pts) + " Z")
    return " ".join(out)


def base(sc):
    """🧱 Cross-base: two crossing 7 m box beams 0.6 x 0.6 m on four 0.7 m pads, four concrete ballast blocks 2.2 x 0.8 x 2.4 m (10.6 t each) on the arms, the mast-foot section between them."""
    p = Part()
    B = "base"
    pads = sc == 50
    y0 = 0.1 if pads else 0.0
    beams = [box("cross-base-x", B, [0, (y0 + 0.7) / 2, 0], [7.0, 0.7 - y0, 0.6], tone=0.45, cornerRadius=0.01 if sc == 50 else 0.0), box("cross-base-z", B, [0, (y0 + 0.7) / 2, 0], [0.6, 0.7 - y0, 7.0], tone=0.45, cornerRadius=0.01 if sc == 50 else 0.0)]
    for s in beams:
        p.body.append(s)
        p.ref(s, "lattice")
        p.member(s, B)
    if pads:
        for i, (x, z) in enumerate(((3.15, 0), (-3.15, 0), (0, 3.15), (0, -3.15))):
            s = box(f"base-pad-{i + 1}", B, [x, 0.05, z], [0.7, 0.1, 0.7], tone=0.35)
            p.body.append(s)
            p.ref(s, "lattice")
    blocks = [("x-pos", [2.4, 1.1, 0], [2.2, 0.8, 2.4]), ("x-neg", [-2.4, 1.1, 0], [2.2, 0.8, 2.4]), ("z-pos", [0, 1.1, 2.4], [2.4, 0.8, 2.2]), ("z-neg", [0, 1.1, -2.4], [2.4, 0.8, 2.2])]
    for name, c, size in blocks:
        s = box(f"ballast-{name}", B, c, size, mat=CONC, tone=0.35, cornerRadius={50: 0.02, 100: 0.02, 200: 0.0, 500: 0.0}[sc])
        p.body.append(s)
        p.ref(s, "ballast")
    foot = box("mast-foot", B, [0, 0.85, 0], [2.2, 0.3, 2.2], tone=0.5, cornerRadius=0.01 if sc <= 100 else 0.0)
    p.body.append(foot)
    p.ref(foot, "lattice")
    p.member(foot, B)
    p.structure.append(K.solid("base-fixed-pad", B, "extrude", CONC, "body", 0.4, profile=[P(*q) for q in [[-0.3, -3.5], [0.3, -3.5], [0.3, -0.3], [3.5, -0.3], [3.5, 0.3], [0.3, 0.3], [0.3, 3.5], [-0.3, 3.5], [-0.3, 0.3], [-3.5, 0.3], [-3.5, -0.3], [-0.3, -0.3]]], plane="zx", **{"from": 0.0, "to": r(max(K.FEAT[sc], 0.05))}, member=B, part="support", anchor="base-fixed"))
    p.structure.append(K.solid("mass-ballast", B, "ellipsoid", CONC, "body", 0.4, radii=P(1.0, 0.35, 1.0), transform={"translation": P(0, 1.1, 0)}, member=B, part="mass"))
    elev = [
        shape("rect", "cross-base", B, "body", 1, True, 0.45, "lattice", STEEL, x=-3.5, y=y0, w=7.0, h=0.7 - y0),
        shape("path", "ballast-slab", B, "body", 2, True, 0.35, "ballast", CONC, d=rects([(-3.5, 0.7, -1.3, 1.5), (-1.2, 0.7, 1.2, 1.5), (1.3, 0.7, 3.5, 1.5)])),
    ]
    if pads:
        elev.append(shape("path", "base-pads", B, "detail", 1.1, True, 0.3, "lattice", STEEL, d=rects([(-3.5, 0, -2.8, 0.1), (-0.35, 0, 0.35, 0.1), (2.8, 0, 3.5, 0.1)])))
    for s in elev:
        p.draw(s, ("elevation", "front"))
    cross = [[-0.3, -3.5], [0.3, -3.5], [0.3, -0.3], [3.5, -0.3], [3.5, 0.3], [0.3, 0.3], [0.3, 3.5], [-0.3, 3.5], [-0.3, 0.3], [-3.5, 0.3], [-3.5, -0.3], [-0.3, -0.3]]
    p.draw(shape("poly", "cross-base", B, "body", 1, True, 0.45, "lattice", STEEL, points=cross, closed=True), ("plan",))
    p.draw(shape("path", "ballast-slab", B, "body", 2, True, 0.35, "ballast", CONC, d=rects([(1.3, -1.2, 3.5, 1.2), (-3.5, -1.2, -1.3, 1.2), (-1.2, 1.3, 1.2, 3.5), (-1.2, -3.5, 1.2, -1.3)])), ("plan",))
    p.draw(shape("rect", "mast-foot", B, "detail", 1.5, True, 0.55, "lattice", STEEL, x=-1.1, y=-1.1, w=2.2, h=2.2), ("plan",))
    return p


def slewing(sc):
    """⚙️ Slewing unit below the tower head: a 2.2 m slewing ring, the 5.8 m platform girder, the console that carries the cab and, from 1:200 on, the two vertical slewing gear motors behind the tower head."""
    p = Part()
    B = "slewing"
    ring = rev("slew-ring", B, [0, 0, 0], [[0, 1.1], [0.5, 1.1]], mat=MACH, role="detail", tone=0.3)
    p.body.append(ring)
    p.ref(ring, "slew-drive", sectionFaces=["-v", "+v"])
    p.structure.append(K.solid("joint-slew-bearing", "turntable", "revolve", STEEL, "body", 0.4, profile=[[0.0, 1.0], [0.5, 1.0]], axis=[1.0, 0.0, 0.0], origin=[0.0, 0.0, 0.0], member="turntable", part="joint"))
    plat = box("slew-platform", B, [0.5, 0.85, 0], [5.8, 0.7, 2.4], mat=MACH, tone=0.5, cornerRadius=0.03 if sc <= 100 else 0.0)
    p.body.append(plat)
    p.ref(plat, "house")
    p.member(plat, B)
    cons = []
    if sc <= 200:
        for i, x in enumerate((1.55, 2.9)):
            cons.append(box(f"cab-console-{i + 1}", B, [x, 1.0, -1.72], [0.16, 0.4, 1.06], mat=MACH, tone=0.45))
    else:
        cons.append(box("cab-console", B, [2.225, 1.0, -1.72], [1.55, 0.4, 1.06], mat=MACH, tone=0.45))
    for s in cons:
        p.body.append(s)
        p.ref(s, "house")
    if sc <= 200:
        for s, z in (("l", 0.6), ("r", -0.6)):
            prof = [[0, 0.16], [0.62, 0.16], [0.62, 0.1], [0.72, 0.1], [0.72, 0.0]] if sc == 50 else [[0, 0.16], [0.72, 0.16]]
            m = rev(f"slew-motor-{s}", B, [-1.9, 1.2, z], prof, mat=MACH, role="detail", tone=0.25)
            p.body.append(m)
            p.ref(m, "slew-drive")
            p.draw(shape("rect", f"slew-motor-{s}", B, "detail", 3.25, True, 0.25, "slew-drive", MACH, rv(B, -1.9, z), x=-2.06, y=1.2, w=0.32, h=0.72))
    p.draw(shape("rect", "slew-ring", B, "detail", 2.1, True, 0.3, "slew-drive", MACH, rv(B, 0.0), x=-1.1, y=0.0, w=2.2, h=0.5))
    p.draw(shape("rect", "slew-platform", B, "body", 2, True, 0.5, "house", MACH, ex(-1.2, 1.2), x=-2.4, y=0.5, w=5.8, h=0.7))
    p.draw(shape("rect", "cab-console", B, "detail", 2.05, True, 0.45, "house", MACH, ex(-2.25, -1.19), x=1.47, y=0.8, w=1.51, h=0.4))
    return p


def cab(sc):
    """🪟 Operator cab beside the tower head, outboard of the trolley track: 1.85 m long, 1.4 m wide, 2.1 m high on the console, front face raked forward by 0.3 m over its glazed height, a side window in the outer wall; 33 mm panels (3 mm steel skin outside, 30 mm insulated lining inside) at 1:50 and 1:100, one solid shell from 1:200."""
    p = Part()
    B = "slewing"
    x0, x1, y0, y1, z0, z1, t = K.CAB_X0, K.CAB_X1, K.CAB_Y0, K.CAB_Y1, K.CAB_Z0, K.CAB_Z1, K.WALL
    fx = K.front_x
    side = K.CAB_PROFILE
    win = K.CAB_WINDOW
    gy0, gy1 = y0 + t, K.CAB_SLOPE_Y1
    glass = [[fx(gy0) - 0.02, gy0], [fx(gy0), gy0], [fx(gy1), gy1], [fx(gy1) - 0.02, gy1]]
    if sc <= 100:
        panels = [
            ("cab-floor", ext("cab-floor", B, [[x0, y0], [x1, y0], [fx(y0 + t), y0 + t], [x0, y0 + t]], "xy", z0, z1, mat=MACH, tone=0.55), "-v"),
            ("cab-roof", box("cab-roof", B, [(x0 + K.CAB_TOPX) / 2, y1 - t / 2, (z0 + z1) / 2], [K.CAB_TOPX - x0, t, z1 - z0], mat=MACH, tone=0.6), "+v"),
            ("cab-rear", box("cab-rear", B, [x0 + t / 2, (y0 + y1) / 2, (z0 + z1) / 2], [t, y1 - y0, z1 - z0], mat=MACH, tone=0.55), "-u"),
            ("cab-header", box("cab-header", B, [K.CAB_TOPX - t / 2, (gy1 + y1 - t) / 2, (z0 + z1) / 2], [t, y1 - t - gy1, z1 - z0 - 2 * t], mat=MACH, tone=0.55), "+u"),
            ("cab-wall-in", ext("cab-wall-in", B, side, "xy", z1 - t, z1, mat=MACH, tone=0.55), "+w"),
            ("cab-wall-out", ext("cab-wall-out", B, side, "xy", z0, z0 + t, mat=MACH, tone=0.6, holes=[[P(*q) for q in win]]), "-w"),
        ]
        for _, s, _ in panels:
            p.body.append(s)
        gf = ext("cab-glass-front", B, glass, "xy", z0 + t, z1 - t, mat=GLASS, role="glazing", tone=0.85)
        gs = ext("cab-glass-side", B, win, "xy", z0 + 0.008, z0 + 0.028, mat=GLASS, role="glazing", tone=0.85)
        p.body += [gf, gs]
        if sc == 100:
            for _, s, face in panels:
                p.ref(s, "cab", sectionFaces=[INNER[face]])
            p.ref(gf, "cab-glazing", sectionFaces=["-u"])
            p.ref(gs, "cab-glazing", sectionFaces=["+w"])
        else:
            for name, s, face in panels:
                p.thermal += layered(s, face)
            p.thermal += glazing_layers()
    else:
        s = ext("cab", B, side, "xy", z0, z1, mat=MACH, tone=0.55)
        gf = ext("cab-glass-front", B, [[fx(gy0), gy0], [fx(gy0) + 0.03, gy0], [fx(gy1) + 0.03, gy1], [fx(gy1), gy1]], "xy", z0 + 0.04, z1 - 0.04, mat=GLASS, role="glazing", tone=0.85)
        p.body += [s, gf]
        p.ref(s, "cab")
        p.ref(gf, "cab-glazing")
        if sc == 200:
            gs = ext("cab-glass-side", B, win, "xy", z0 - 0.03, z0 + 0.01, mat=GLASS, role="glazing", tone=0.85)
            p.body.append(gs)
            p.ref(gs, "cab-glazing")
    p.draw(shape("poly", "cab", B, "body", 3.4, True, 0.6, "cab", MACH, ex(z0, z1), points=side, closed=True))
    if sc <= 200:
        p.draw(shape("poly", "cab-glass", B, "glazing", 6, True, 0.85, "cab-glazing", GLASS, ex(z0 - (0.03 if sc == 200 else 0.0), z0 + 0.028), points=win, closed=True), ("elevation",))
    gview = glass if sc <= 100 else [[fx(gy0), gy0], [fx(gy0) + 0.03, gy0], [fx(gy1) + 0.03, gy1], [fx(gy1), gy1]]
    p.draw(shape("poly", "cab-glass-front", B, "glazing", 6.1, True, 0.85, "cab-glazing", GLASS, ex(z0 + t, z1 - t), points=gview, closed=True))
    return p


INNER = {"-v": "+v", "+v": "-v", "-u": "+u", "+u": "-u", "-w": "+w", "+w": "-w"}


def layered(s, face):
    """🧅 The two layers of one 33 mm cab panel at 1:50: the 3 mm steel skin on the outer face and the 30 mm lining inside it, as solids of their own."""
    out = []
    ax = {"u": 0, "v": 1, "w": 2}[face[1]]
    sign = 1 if face[0] == "+" else -1
    for layer, a, b in (("lining", 0.0, 0.03), ("steel", 0.03, 0.033)):
        q = dict(s)
        q = {k: v for k, v in s.items() if k not in ("id",)}
        q["id"] = f"th-{s['id']}-{layer}"
        if s["kind"] == "box":
            size = list(s["size"])
            c = list(s["transform"]["translation"])
            lo = c[ax] - size[ax] / 2
            hi = c[ax] + size[ax] / 2
            if sign > 0:
                n0, n1 = lo + a, lo + b
            else:
                n0, n1 = hi - b, hi - a
            size[ax] = r(n1 - n0)
            c[ax] = r((n0 + n1) / 2)
            q["size"] = size
            q["transform"] = {"translation": c}
            q.pop("cornerRadius", None)
        elif face[1] == "w":
            lo, hi = s["from"], s["to"]
            n0, n1 = (lo + a, lo + b) if sign > 0 else (hi - b, hi - a)
            q["from"], q["to"] = r(n0), r(n1)
        else:
            prof = s["profile"]
            ys = sorted({p[1] for p in prof})
            lo, hi = ys[0], ys[-1]
            n0, n1 = (lo + a, lo + b) if sign > 0 else (hi - b, hi - a)
            q["profile"] = [[r(K.CAB_X0), r(n0)], [r(K.front_x(n0)), r(n0)], [r(K.front_x(n1)), r(n1)], [r(K.CAB_X0), r(n1)]]
        q["region"] = "cab"
        q["layer"] = layer
        if layer == "steel":
            q["sectionFaces"] = [INNER[face]]
        q["tone"] = 0.6 if layer == "steel" else 0.45
        out.append(q)
    return out


def glazing_layers():
    """🔲 The two insulating glass units of the cab at 1:50 as three sheets each, interior first (4 mm pane, 12 mm gap, 4 mm pane): the raked windscreen offset along its normal and the side window in the outer wall."""
    B = "slewing"
    fx = K.front_x
    t = K.WALL
    gy0, gy1 = K.CAB_Y0 + t, K.CAB_SLOPE_Y1
    out = []
    n = (1.95, -0.3)
    ln = math.hypot(*n)
    nx, ny = n[0] / ln, n[1] / ln
    off = 0.02
    for layer, a, b in (("pane-in", 0.0, 0.004), ("gap", 0.004, 0.016), ("pane-out", 0.016, 0.02)):
        sa, sb = a - off, b - off
        prof = [[fx(gy0) + sa * nx, gy0 + sa * ny], [fx(gy0) + sb * nx, gy0 + sb * ny], [fx(gy1) + sb * nx, gy1 + sb * ny], [fx(gy1) + sa * nx, gy1 + sa * ny]]
        out.append(ext(f"th-glass-front-{layer}", B, prof, "xy", K.CAB_Z0 + t, K.CAB_Z1 - t, mat=GLASS, role="glazing", tone=0.85, region="cab-glazing", layer=layer, **({"sectionFaces": ["-u"]} if layer == "pane-out" else {})))
    for layer, a, b in (("pane-in", 0.024, 0.028), ("gap", 0.012, 0.024), ("pane-out", 0.008, 0.012)):
        out.append(ext(f"th-glass-side-{layer}", B, K.CAB_WINDOW, "xy", K.CAB_Z0 + a, K.CAB_Z0 + b, mat=GLASS, role="glazing", tone=0.85, region="cab-glazing", layer=layer, **({"sectionFaces": ["+w"]} if layer == "pane-out" else {})))
    return out


def lattice(bars, bone, sc, joints):
    """🕸️ Bars of a lattice as body solids, lattice thermal references, structural members, and as member shapes in the viewpoints that see them distinctly."""
    p = Part()
    for b in bars:
        s = b.solid()
        p.body.append(s)
        p.ref(s, b.region)
        if b.region == "lattice":
            p.member(s, bone)
        for v in b.views:
            p.views[v].append(b.shape(sc))
    p.structure += joints
    return p


def jib(sc):
    if sc == 500:
        p = Part()
        tri = [[0.81, 0.0], [-0.81, -0.7], [-0.81, 0.7]]
        env = ext("jib-envelope", "jib", tri, "yz", K.JIB_HEEL_TOP, K.JIB_TIP, tone=0.5, envelope=True)
        p.body.append(env)
        p.ref(env, "lattice")
        for b in (K.Bar("jib-chord", "jib", [K.JIB_HEEL_TOP, 0.74, 0], [K.JIB_TIP, 0.74, 0], d=0.14), K.Bar("jib-rail-l", "jib", [K.JIB_HEEL_BOT, -0.74, 0.6], [K.JIB_TIP, -0.74, 0.6], d=0.13), K.Bar("jib-rail-r", "jib", [K.JIB_HEEL_BOT, -0.74, -0.6], [K.JIB_TIP, -0.74, -0.6], d=0.13)):
            s = b.solid()
            s.pop("slender", None)
            s["id"] = "member-" + s["id"]
            s.update({"member": "jib", "part": "member"})
            p.structure.append(s)
        p.structure.append(box("joint-jib-root", "jib", [K.JIB_HEEL_BOT, 0, 0], [0.3, 1.3, 1.1], tone=0.4, member="jib", part="joint"))
        p.draw(shape("rect", "jib-envelope", "jib", "body", 3.5, True, 0.75, "lattice", STEEL, ex(-0.7, 0.7), x=K.JIB_HEEL_TOP, y=-0.81, w=K.JIB_TIP - K.JIB_HEEL_TOP, h=1.62))
    else:
        joints = [box("joint-jib-heel", "jib", [K.JIB_HEEL_BOT + 0.09, -0.74, 0], [0.16, 0.16, 1.32], tone=0.4, member="jib", part="joint"), box("joint-jib-heel-top", "jib", [K.JIB_HEEL_TOP + 0.05, 0.74, 0], [0.16, 0.16, 0.2], tone=0.4, member="jib", part="joint")]
        p = lattice(K.jib_bars(sc), "jib", sc, joints)
    tip = rev("jib-tip-sheave", "jib", [K.JIB_TIP, 0, -0.08], [[0, 0.3], [0.16, 0.3]], axis=(0, 0, 1), tone=0.3, role="detail")
    p.body.append(tip)
    p.ref(tip, "lattice")
    p.draw(shape("ellipse", "jib-tip-sheave", "jib", "detail", 3.3, True, 0.3, None, STEEL, ex(-0.08, 0.08), cx=K.JIB_TIP, cy=0.0, rx=0.3, ry=0.3))
    return p


def counter_jib(sc):
    if sc == 500:
        p = Part()
        env = box("counter-jib-envelope", "counter-jib", [(K.CJ_HEEL + K.CJ_TAIL) / 2, 0, 0], [K.CJ_TAIL - K.CJ_HEEL, 1.4, 1.4], tone=0.5, envelope=True)
        p.body.append(env)
        p.ref(env, "lattice")
        for s_, (y, z) in (("tl", (-0.64, 0.64)), ("tr", (-0.64, -0.64)), ("bl", (0.64, 0.64)), ("br", (0.64, -0.64))):
            s = K.Bar(f"member-counter-chord-{s_}", "counter-jib", [K.CJ_HEEL, y, z], [K.CJ_TAIL, y, z], d=0.12).solid()
            s.pop("slender", None)
            s.update({"member": "counter-jib", "part": "member"})
            p.structure.append(s)
        p.structure.append(box("joint-counter-root", "counter-jib", [K.CJ_HEEL + 0.2, 0, 0], [0.3, 1.2, 1.2], tone=0.4, member="counter-jib", part="joint"))
        p.draw(shape("rect", "counter-jib-envelope", "counter-jib", "body", 1.8, True, 0.75, "lattice", STEEL, ex(-0.7, 0.7), x=K.CJ_HEEL, y=-0.7, w=K.CJ_TAIL - K.CJ_HEEL, h=1.4))
        return p
    joints = [rev(name, "counter-jib", [K.CJ_HEEL + 0.01, 0.64, z], [[0, 0.036], [0.12, 0.036]],
                  axis=(1, 0, 0), tone=0.4, member="counter-jib", part="joint")
              for name, z in (("joint-counter-heel", 0.64), ("joint-counter-heel-right", -0.64))]
    return lattice(K.counter_bars(sc), "counter-jib", sc, joints)


def pendants(sc):
    p = Part()
    for b in K.pendant_bars():
        s = b.solid()
        s["role"] = "detail"
        s["tone"] = 0.3
        p.body.append(s)
        p.ref(s, "pendants")
        for v in ("elevation", "front", "plan"):
            m = b.shape(sc, z=4, tone=0.3, role="detail")
            p.views[v].append(m)
    return p


def counterweight(sc):
    """⚖️ Counterweight: four vertical concrete slabs 0.4 x 2.1 x 2.4 m (5 t each, 20 t) hung side by side from a steel suspension beam under the counter-jib tail; one block with its beam from 1:100 on."""
    p = Part()
    B = "counterweight"
    if sc == 50:
        slabs = [box(f"counterweight-slab-{i + 1}", B, [-0.6 + 0.4 * i, 1.05, 0], [0.4, 2.1, 2.4], mat=CONC, tone=0.3, cornerRadius=0.01) for i in range(4)]
    else:
        slabs = [box("counterweight-slab", B, [0, 1.05, 0], [1.6, 2.1, 2.4], mat=CONC, tone=0.3)]
    for s in slabs:
        p.body.append(s)
        p.ref(s, "counterweight")
    beam = box("counterweight-beam", B, [0, 2.16, 0], [1.7, 0.12, 2.5], tone=0.45)
    p.body.append(beam)
    p.ref(beam, "lattice")
    p.member(beam, B)
    p.structure.append(K.solid("mass-counterweight", B, "ellipsoid", CONC, "body", 0.4, radii=P(0.6, 0.8, 0.9), transform={"translation": P(0, 1.05, 0)}, member=B, part="mass"))
    p.draw(shape("rect", "counterweight-slabs", B, "body", 3.3, True, 0.3, "counterweight", CONC, ex(-1.2, 1.2), x=-0.8, y=0.0, w=1.6, h=2.1))
    p.draw(shape("rect", "counterweight-beam", B, "detail", 3.35, True, 0.45, "lattice", STEEL, ex(-1.25, 1.25), x=-0.85, y=2.1, w=1.7, h=0.12))
    if sc == 50:
        p.draw(shape("path", "counterweight-joints", B, "texture", 3.4, False, 0.2, None, CONC, ex(-1.2, 1.2), d="M -0.4,0 L -0.4,2.1 M 0,0 L 0,2.1 M 0.4,0 L 0.4,2.1"), ("elevation", "plan"))
    return p


def trolley(sc):
    """🛒 Trolley: two 40 mm side plates outside the bottom chords, four 250 mm wheels running on the outer halves of the chords, two cross beams below the chords, two 400 mm rope sheaves 0.8 m apart between cheek plates (the two falls of the hoist rope drop 0.4 m apart); the wheels merge into the plates at 1:200, a block at 1:500."""
    p = Part()
    B = "trolley"
    th = "trolley"
    ss = []
    if sc == 500:
        ss.append(box("trolley-frame", B, [0, -0.04, 0], [1.4, 0.92, 1.46], mat=MACH, tone=0.4))
        p.draw(shape("rect", "trolley-frame", B, "body", 4, True, 0.4, th, MACH, ex(-0.73, 0.73), x=-0.7, y=-0.5, w=1.4, h=0.92))
    else:
        for s_, z in (("r", -0.71), ("l", 0.71)):
            ss.append(box(f"trolley-plate-{s_}", B, [0, 0.1, z], [1.4, 0.64, 0.04], mat=MACH, tone=0.4))
        for i, x in enumerate((-0.45, 0.45)):
            ss.append(box(f"trolley-beam-{i + 1}", B, [x, -0.145, 0], [0.12, 0.15, 1.46], mat=MACH, tone=0.35))
        if sc <= 100:
            for s_, z in (("r", -0.63), ("l", 0.63)):
                for i, x in enumerate((-0.5, 0.5)):
                    ss.append(rev(f"trolley-wheel-{s_}-{i + 1}", B, [x, 0.255, z - 0.025], [[0, 0.125], [0.05, 0.125]], axis=(0, 0, 1), mat=MACH, role="detail", tone=0.2))
        ss.append(box("trolley-cheeks", B, [0, -0.37, 0], [1.0, 0.3, 0.12], mat=MACH, tone=0.35))
        for i, x in enumerate((-0.4, 0.4)):
            ss.append(rev(f"trolley-sheave-{i + 1}", B, [x, -0.45, -0.03], [[0, 0.2], [0.06, 0.2]], axis=(0, 0, 1), mat=STEEL, role="detail", tone=0.3))
        p.draw(shape("rect", "trolley-frame", B, "body", 4, True, 0.4, th, MACH, ex(-0.73, -0.69), x=-0.7, y=-0.22, w=1.4, h=0.64))
        p.draw(shape("rect", "trolley-frame-far", B, "body", 3.9, True, 0.4, th, MACH, ex(0.69, 0.73), x=-0.7, y=-0.22, w=1.4, h=0.64), ("front", "plan"))
        p.draw(shape("path", "trolley-beams", B, "detail", 3.95, True, 0.35, th, MACH, ex(-0.73, 0.73), d=rects([(-0.51, -0.22, -0.39, -0.07), (0.39, -0.22, 0.51, -0.07)])), ("front", "plan"))
        p.draw(shape("rect", "trolley-cheeks", B, "detail", 4.05, True, 0.35, th, MACH, ex(-0.06, 0.06), x=-0.5, y=-0.52, w=1.0, h=0.3))
        if sc <= 100:
            for s_, z in (("r", -0.63), ("l", 0.63)):
                p.draw(shape("path", f"trolley-wheels-{s_}", B, "detail", 3.92, True, 0.2, th, MACH, ex(z - 0.025, z + 0.025), d=circles([(-0.5, 0.255, 0.125), (0.5, 0.255, 0.125)])), ("front", "plan"))
        p.draw(shape("path", "trolley-sheave", B, "detail", 4.2, True, 0.3, th, STEEL, ex(-0.03, 0.03), d=circles([(-0.4, -0.45, 0.2), (0.4, -0.45, 0.2)])))
    for s in ss:
        p.body.append(s)
        p.ref(s, th)
    p.structure.append(K.solid("joint-trolley", B, "box", STEEL, "body", 0.4, size=[1.4, 0.14, 1.36], transform={"translation": [0.0, 0.06, 0.0]}, member=B, part="joint"))
    p.structure.append(K.solid("mass-trolley", B, "ellipsoid", MACH, "body", 0.4, radii=P(0.45, 0.25, 0.45), transform={"translation": P(0, -0.12, 0)}, member=B, part="mass"))
    return p


HOOK = None


def block(sc, hook):
    """🪝 Hook block: two side plates with a rounded head 0.6 m wide, a 440 mm sheave between them (the two falls arrive 0.4 m apart), a crosshead below and the forged hook; one rounded block from 1:200."""
    p = Part()
    B = "hoist-block"
    arc = [[r(0.3 * math.cos(math.pi * k / 8)), r(-0.2 + 0.3 * math.sin(math.pi * k / 8))] for k in range(9)]
    plate = [[0.3, -0.45]] + arc + [[-0.3, -0.45]]
    ss = []
    if sc <= 100:
        for s_, z in (("r", -0.075), ("l", 0.075)):
            ss.append(ext(f"block-plate-{s_}", B, plate, "xy", z - 0.015, z + 0.015, mat=SKIP, tone=0.3))
        ss.append(rev("block-sheave", B, [0, -0.2, -0.04], [[0, 0.2], [0.08, 0.2]], axis=(0, 0, 1), mat=STEEL, role="detail", tone=0.5))
        ss.append(box("block-crosshead", B, [0, -0.5, 0], [0.4, 0.1, 0.24], mat=SKIP, tone=0.3))
    else:
        ss.append(ext("block-body", B, plate + [], "xy", -0.09, 0.09, mat=SKIP, tone=0.3))
        if sc == 200:
            ss.append(box("block-crosshead", B, [0, -0.5, 0], [0.4, 0.1, 0.24], mat=SKIP, tone=0.3))
    h = dict(hook)
    if sc == 500:
        h = K.solid("hook", B, "capsule", SKIP, "detail", 0.2, p0=[0.03, -0.5, 0.0], p1=[0.03, -1.15, 0.0], r0=0.06, r1=0.06)
    ss.append(h)
    for s in ss:
        p.body.append(s)
        p.ref(s, "skip")
    p.structure.append({"id": "joint-hoist-block", "of": "block-sheave" if sc <= 100 else "block-body", "member": B, "part": "joint"})
    p.draw(shape("poly", "block-body", B, "body", 4.5, True, 0.3, "skip", SKIP, ex(-0.09, 0.09), points=plate, closed=True))
    if sc == 500:
        p.draw(shape("capsule", "hook", B, "detail", 4.45, True, 0.2, "skip", SKIP, ex(-0.06, 0.06), x0=0.03, y0=-0.5, x1=0.03, y1=-1.15, r0=0.06, r1=0.06))
    if sc <= 200:
        p.draw(shape("rect", "block-crosshead", B, "body", 4.4, True, 0.3, "skip", SKIP, ex(-0.12, 0.12), x=-0.2, y=-0.55, w=0.4, h=0.1))
    return p


def skip(sc):
    """🪣 Concrete skip of 1.0 m3: a cylindrical upper part 1.36 m across and 0.45 m deep, a cone down to the 0.36 m discharge spout with its gate, four legs to stand on and a bail that hangs in the hook; 6 mm walls open at the top at 1:50 and 1:100, a closed body from 1:200."""
    p = Part()
    B = "skip"
    outer = [[0.0, 0.68], [-0.45, 0.68], [-0.95, 0.18], [-1.0, 0.18]]
    if sc <= 100:
        prof = outer + [[-1.0, 0.174], [-0.952, 0.174], [-0.452, 0.674], [0.0, 0.674], [0.0, 0.68]]
    else:
        prof = outer + [[-1.0, 0.0], [0.0, 0.0]]
    body = rev("skip-body", B, [0, 0, 0], [[a, b] for a, b in prof], axis=(0, -1, 0), mat=SKIP, tone=0.45)
    body["profile"] = [[r(-a), r(b)] for a, b in prof]
    ss = [body]
    if sc == 50:
        ss.append(rev("skip-rim", B, [0, -0.05, 0], [[0, 0.7], [0.05, 0.7], [0.05, 0.672], [0, 0.672], [0, 0.7]], mat=SKIP, tone=0.4, role="detail"))
    ss.append(box("skip-gate", B, [0, -1.075, 0], [0.42, 0.15, 0.42], mat=SKIP, tone=0.25))
    bail_d = {50: 0.05, 100: 0.05, 200: 0.05, 500: 0.075}[sc]
    by = 0.36 if sc < 500 else 0.33
    bars = [K.Bar("bail-bar", B, [0.05, by, -0.76], [0.05, by, 0.76], d=bail_d, region="skip", mat=SKIP), K.Bar("bail-leg-r", B, [0.05, by, -0.74], [0.05, -0.25, -0.74], d=bail_d, region="skip", mat=SKIP), K.Bar("bail-leg-l", B, [0.05, by, 0.74], [0.05, -0.25, 0.74], d=bail_d, region="skip", mat=SKIP)]
    if sc <= 100:
        for i, (x, z) in enumerate(((0.44, 0.44), (-0.44, 0.44), (-0.44, -0.44), (0.44, -0.44))):
            bars.append(K.Bar(f"skip-leg-{i + 1}", B, [x, -0.4, z], [x, -1.15, z], d=0.06, region="skip", mat=SKIP))
    for b in bars:
        s = b.solid()
        s.pop("slender", None)
        if norm_len(b) / b.d >= 20.5:
            s["slender"] = True
        s["role"] = "detail"
        ss.append(s)
        p.structure.append({"id": "member-" + s["id"], "of": s["id"], "member": B, "part": "member"})
        for v in ("elevation", "front", "plan"):
            p.views[v].append(b.shape(sc, z=4.6, tone=0.3, role="detail"))
    if sc <= 100:
        for i in range(2):
            ss.append(box(f"skip-lug-{'rl'[i]}", B, [0.025, -0.25, (-1, 1)[i] * 0.705], [0.12, 0.12, 0.07], mat=SKIP, tone=0.3, role="detail"))
    for s in ss:
        p.body.append(s)
        p.ref(s, "skip")
    p.structure.append(K.solid("mass-skip", B, "ellipsoid", SKIP, "body", 0.4, radii=[0.5, 0.35, 0.5], transform={"translation": [0.0, -0.45, 0.0]}, member=B, part="mass"))
    outline = [[0.68, 0.0], [0.68, -0.45], [0.18, -0.95], [0.18, -1.0], [-0.18, -1.0], [-0.18, -0.95], [-0.68, -0.45], [-0.68, 0.0]]
    p.draw(shape("poly", "skip-body", B, "body", 4.4, True, 0.45, "skip", SKIP, rv(B, 0.0), points=outline, closed=True))
    p.draw(shape("rect", "skip-gate", B, "detail", 4.5, True, 0.25, "skip", SKIP, ex(-0.21, 0.21), x=-0.21, y=-1.15, w=0.42, h=0.15))
    return p


def norm_len(b):
    return K.norm(K.sub(b.b, b.a))


def ropes(sc):
    """🧶 Hoist rope, reeved in two falls: each of the eight telescoping rope pieces is two 14 mm strands 0.4 m apart, the falls from the two trolley sheaves to the block sheave; one strand on the axis at 1:500."""
    p = Part()
    for n in range(1, 9):
        B = f"rope-{n}"
        xs = (-K.STRAND_X, K.STRAND_X) if sc < 500 else (0.0,)
        for i, x in enumerate(xs):
            sid = f"rope-{n}" if len(xs) == 1 else f"rope-{n}-{'ab'[i]}"
            s = cyl(sid, B, [x, 0, 0], [x, -5.75, 0], K.ROPE_R, mat=ROPE, role="detail", tone=0.2)
            p.body.append(s)
            p.ref(s, "rope")
            p.member(s, B)
        pen = r(max(2 * K.ROPE_R * 1000 / sc, 0.13), 3)
        d = rects([(x - K.ROPE_R, -5.75, x + K.ROPE_R, 0.0) for x in xs])
        line = shape("path", f"rope-{n}-line", B, "detail", 3, False, 0.3, "rope", ROPE, ex(-K.ROPE_R, K.ROPE_R), d=d)
        line["penMm"] = pen
        p.draw(line, ("elevation",))
        fr = shape("path", f"rope-{n}-line", B, "detail", 3, False, 0.3, "rope", ROPE, ex(-K.ROPE_R, K.ROPE_R), d="M 0,0 L 0,-5.75")
        fr["penMm"] = pen
        p.draw(fr, ("front",))
    return p


def winch_house_thermal(sc):
    """🏠 The winch house skin as the thermal model sees it: the closed 4.0 x 1.8 x 1.4 m house, whose heat leaves through its outer faces (the enclosed air inside is no outer surface, C31); the body keeps the real 7.5 mm walls."""
    return [box("th-winch-house", "winch", [0, 0.9, 0], [4.0, 1.8, 1.4], mat=MACH, tone=0.55, region="winch-house")]


def winch_skids(sc):
    """🛷 Two skid channels under the winch house that carry it on the counter-jib top chords (the house stood 50 mm above the chords before)."""
    p = Part()
    B = "winch"
    for s_, z in (("r", -0.64), ("l", 0.64)):
        s = box(f"winch-skid-{s_}", B, [0, -0.025, z], [4.0, 0.06 if sc < 500 else 0.075, 0.14], tone=0.4)
        if sc == 500:
            s["transform"]["translation"][1] = -0.0325
        p.body.append(s)
        p.ref(s, "winch-house")
    p.draw(shape("rect", "winch-skids", B, "detail", 3.2, True, 0.4, "winch-house", STEEL, ex(-0.71, 0.71), x=-2.0, y=-0.06 if sc < 500 else -0.07, w=4.0, h=0.06 if sc < 500 else 0.07), ("elevation", "front"))
    return p


def mast(sc):
    """🗼 The lattice mast at 1:50, 1:100 and 1:200: every chord, horizontal and diagonal at its real section (the lattice is what the crane is at every one of these scales); at 1:50 the rung ladder inside it is drawn as texture; 1:500 keeps the envelope prism."""
    p = lattice(K.mast_bars(), "mast", sc, [])
    if sc == 50:
        rungs = " ".join(f"M -0.2,{r(0.5 + 0.3 * k)} L 0.2,{r(0.5 + 0.3 * k)}" for k in range(125))
        p.draw(shape("path", "mast-ladder", "mast", "texture", 1.7, False, 0.3, None, STEEL, d="M 0.2,-0.55 L 38.8,-0.55"), ("elevation",))
        p.draw(shape("path", "mast-ladder", "mast", "texture", 1.7, False, 0.3, None, STEEL, d="M -0.2,0.2 L -0.2,38.8 M 0.2,0.2 L 0.2,38.8 " + rungs), ("front",))
    return p
