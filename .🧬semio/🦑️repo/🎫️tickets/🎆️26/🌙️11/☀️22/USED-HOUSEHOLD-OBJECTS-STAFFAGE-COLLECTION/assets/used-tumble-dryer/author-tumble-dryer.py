"""🌀 Authoritative chisel of the used heat-pump tumble dryer: states every hand-chosen number and writes `🔣️.json` next to it.

Parts are authored once per scale in world rest coordinates (x ahead = the door side, y up, z left of the dryer); the 3D solids
(bone-local) and the three 2D views (elevation bone-local, front and plan in world rest coordinates) are written from that one table,
so 2D and 3D agree by construction. Texts, regions, poses and clips live in `author_texts.py` beside this file.

@see ../../brief.md
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
@see ../../../USED-HVAC-COMPONENTS-STAFFAGE-COLLECTION/assets/used-brine-water-heat-pump/author-heat-pump.py
"""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import author_texts as TX

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

Y0 = 0.016
YC = Y0 + 0.455
ZH = -0.235
XF = 0.275
XP = 0.285
BONES = {
    "ground": ((0.0, 0.0, 0.0), False),
    "blanket": ((-0.45, 0.0, 0.0), False),
    "lift": ((-0.32, Y0, 0.0), False),
    "unit": ((-0.32, Y0, 0.0), False),
    "door": ((XF, YC, ZH), True),
    "tank": ((-0.05, Y0 + 0.775, 0.2), False),
    "lint-filter": ((0.242, Y0 + 0.287, 0.0), False),
}
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}
FOOT_XZ = [(0.22, -0.25), (0.22, 0.25), (-0.27, -0.25), (-0.27, 0.25)]
FEET = ["foot-front-right", "foot-front-left", "foot-back-right", "foot-back-left"]


def r6(v):
    """🎯️ Rounded to a hundredth of a millimetre."""
    return round(v + 0.0, 5)


def to_local(bone, p):
    """📐️ World rest point to the bone-local frame (u, v, w); the door bone points up, so u = y, v = -x, w = z."""
    (bx, by, bz), up = BONES[bone]
    if up:
        return [r6(p[1] - by), r6(-(p[0] - bx)), r6(p[2] - bz)]
    return [r6(p[0] - bx), r6(p[1] - by), r6(p[2] - bz)]


def axis_local(bone, ax):
    """🧭️ A world axis as a bone-local direction."""
    up = BONES[bone][1]
    if up:
        return {"x": [0, -1, 0], "y": [1, 0, 0], "z": [0, 0, 1]}[ax]
    return {"x": [1, 0, 0], "y": [0, 1, 0], "z": [0, 0, 1]}[ax]


def part(pid, bone, kind, mat, tone, region, views, **kw):
    """🧩️ One part of the parts table."""
    d = dict(id=pid, bone=bone, kind=kind, mat=mat, tone=tone, region=region, views=views)
    d.update(kw)
    return d


def box(pid, bone, mat, tone, region, lo, hi, views="efp", **kw):
    """📦️ An axis-aligned box from its low and high world corners."""
    return part(pid, bone, "box", mat, tone, region, views, lo=list(lo), hi=list(hi), **kw)


def rev(pid, bone, mat, tone, region, ax, c, prof, views="efp", **kw):
    """🥫️ A solid of revolution about a world axis through `c`; `prof` is [world coordinate along the axis, radius]."""
    return part(pid, bone, "rev", mat, tone, region, views, ax=ax, c=list(c), prof=[list(q) for q in prof], **kw)


def disc(pid, bone, mat, tone, region, ax, c, r, a0, a1, views="efp", **kw):
    """💿️ A plain cylinder about a world axis."""
    return rev(pid, bone, mat, tone, region, ax, c, [[a0, 0.0], [a0, r], [a1, r], [a1, 0.0]], views, **kw)


def ring(pid, bone, mat, tone, region, ax, c, loop, views="efp", **kw):
    """⭕️ A ring revolved from a closed loop off the axis."""
    lp = [list(q) for q in loop]
    if lp[0] != lp[-1]:
        lp.append(list(lp[0]))
    return rev(pid, bone, mat, tone, region, ax, c, lp, views, **kw)


AX = {"x": 0, "y": 1, "z": 2}


def is_loop(prof):
    """🔁️ True for a closed loop profile."""
    return prof[0] == prof[-1]


def extent(p):
    """📏️ World bounds of a part."""
    if p["kind"] in ("box", "panel"):
        return list(p["lo"]), list(p["hi"])
    i = AX[p["ax"]]
    rmax = max(r for _, r in p["prof"])
    lo = [p["c"][k] - rmax for k in range(3)]
    hi = [p["c"][k] + rmax for k in range(3)]
    lo[i] = min(a for a, _ in p["prof"])
    hi[i] = max(a for a, _ in p["prof"])
    return lo, hi


def base_solid(p):
    """🧱️ The common fields of a body solid."""
    s = {"id": p["id"], "bone": p["bone"], "material": p["mat"], "role": "detail" if p.get("texture") else "body", "tone": p["tone"]}
    for flag in ("sheet", "texture", "slender"):
        if p.get(flag):
            s[flag] = True
    return s


def solid_of(p):
    """🧊️ The volume solid of one part."""
    s = base_solid(p)
    bone = p["bone"]
    if p["kind"] == "box":
        lo, hi = p["lo"], p["hi"]
        c = [(a + b) / 2 for a, b in zip(lo, hi)]
        sz = [b - a for a, b in zip(lo, hi)]
        if BONES[bone][1]:
            sz = [sz[1], sz[0], sz[2]]
        s.update(kind="box", size=[r6(v) for v in sz], transform={"translation": to_local(bone, c)})
    elif p["kind"] == "rev":
        i = AX[p["ax"]]
        o = list(p["c"])
        o[i] = BONES[bone][0][i]
        s.update(kind="revolve", profile=[[r6(a - o[i]), r6(r)] for a, r in p["prof"]], axis=axis_local(bone, p["ax"]), origin=to_local(bone, o))
    else:
        (bx, by, bz), _ = BONES[bone]
        lo, hi = p["lo"], p["hi"]
        outer = [[r6(lo[1] - by), r6(lo[2] - bz)], [r6(hi[1] - by), r6(lo[2] - bz)], [r6(hi[1] - by), r6(hi[2] - bz)], [r6(lo[1] - by), r6(hi[2] - bz)]]
        cy, cz, rr = p["hole"]
        hole = [[r6(cy - by + rr * math.cos(2 * math.pi * k / 48)), r6(cz - bz + rr * math.sin(2 * math.pi * k / 48))] for k in range(48)]
        s.update(kind="extrude", profile=outer, holes=[hole], plane="yz", **{"from": r6(lo[0] - bx), "to": r6(hi[0] - bx)})
    return s


def plane_of(view, p):
    """🗺️ World coordinates of a 3D point in the plane of a view: elevation (x, y), front (z, y), plan (x, z)."""
    return {"elevation": (p[0], p[1]), "front": (p[2], p[1]), "plan": (p[0], p[2])}[view]


def to_paper(view, bone, q):
    """📝️ A world plane point of a view to the coordinates of the shape: elevation bone-local, front and plan world."""
    if view != "elevation":
        return [r6(q[0]), r6(q[1])]
    (bx, by, _), up = BONES[bone]
    if up:
        return [r6(q[1] - by), r6(-(q[0] - bx))]
    return [r6(q[0] - bx), r6(q[1] - by)]


def door_local_poly(pts_xy):
    """🚪️ World (x, y) points of a door part as door-local (u, v) points."""
    (bx, by, _), _ = BONES["door"]
    return [[r6(y - by), r6(-(x - bx))] for x, y in pts_xy]


def near(view, lo, hi):
    """👁️ Nearness of a part to the viewer of a view (elevation from -z, front from +x, plan from +y)."""
    return {"elevation": -lo[2], "front": hi[0], "plan": hi[1]}[view]


def rev_across(p, view):
    """🔪️ The outline of a revolved part seen across its axis, as world plane points."""
    i = AX[p["ax"]]
    pa = {"elevation": (0, 1), "front": (2, 1), "plan": (0, 2)}[view]
    along = pa.index(i)
    other = pa[1 - along]
    cc = p["c"][other]
    prof = p["prof"]
    if is_loop(prof):
        amin = min(a for a, _ in prof)
        amax = max(a for a, _ in prof)
        out = p.get("sil") or [[amin, max(r for _, r in prof)], [amax, max(r for _, r in prof)]]
    else:
        out = [q for q in prof if q[1] > 0]
    pts = [(a, cc + r) for a, r in out] + [(a, cc - r) for a, r in reversed(out)]
    if along == 1:
        pts = [(b, a) for a, b in pts]
    return pts


def poly_shape(s, pts):
    """🔷️ A rect when the points are an axis-aligned rectangle, else a closed poly."""
    xs = sorted({q[0] for q in pts})
    ys = sorted({q[1] for q in pts})
    if len(pts) == 4 and len(xs) == 2 and len(ys) == 2:
        s.update(kind="rect", x=xs[0], y=ys[0], w=r6(xs[1] - xs[0]), h=r6(ys[1] - ys[0]))
    else:
        s.update(kind="poly", points=pts, closed=True)
    return s


def shape_of(p, view):
    """✏️ The filled 2D shape of one part in one view."""
    lo, hi = extent(p)
    lo = p.get("dlo") or lo
    hi = p.get("dhi") or hi
    role = p.get("role") or ("texture" if p.get("texture") else "body")
    z = p.get("z" + view[0]) or round(2 + near(view, lo, hi), 4)
    s = {"id": p["id"], "bone": p["bone"], "role": role, "z": z, "filled": True, "tone": p["tone"], "material": p["mat"]}
    if p["region"]:
        s["region"] = p["region"]
    if p.get("hatch") and view in p.get("hatchViews", "p"):
        s["hatch"] = p["hatch"]
    door = BONES[p["bone"]][1]
    if door and view == "front" and p["kind"] == "rev":
        rmax = max(r for _, r in p["prof"])
        (bx, by, bz), _ = BONES["door"]
        cu, cw = p["c"][1] - by, p["c"][2] - bz
        poly_shape(s, [[r6(cu + rmax * math.cos(2 * math.pi * k / 48)), r6(cw + rmax * math.sin(2 * math.pi * k / 48))] for k in range(48)])
        s["solid"] = {"kind": "extruded", "sweep": "v", "z0": r6(-(hi[0] - bx)), "z1": r6(-(lo[0] - bx))}
        return s
    if door and view in ("elevation", "plan"):
        if p["kind"] == "rev":
            wpts = rev_across(p, "elevation")
        else:
            wpts = [(lo[0], lo[1]), (hi[0], lo[1]), (hi[0], hi[1]), (lo[0], hi[1])]
        poly_shape(s, door_local_poly(wpts))
        s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r6(lo[2]), "z1": r6(hi[2])}
        return s
    if p["kind"] == "panel" and view == "front":
        cy, cz, rr = p["hole"]
        d = f"M {r6(lo[2])} {r6(lo[1])} L {r6(hi[2])} {r6(lo[1])} L {r6(hi[2])} {r6(hi[1])} L {r6(lo[2])} {r6(hi[1])} Z M "
        d += " L ".join(f"{r6(cz + rr * math.cos(2 * math.pi * k / 48))} {r6(cy + rr * math.sin(2 * math.pi * k / 48))}" for k in range(48)) + " Z"
        s.update(kind="path", d=d)
        return s
    if p["kind"] == "rev":
        normal = {"elevation": 2, "front": 0, "plan": 1}[view]
        if AX[p["ax"]] == normal:
            cc = to_paper(view, p["bone"], plane_of(view, p["c"]))
            rmax = max(r for _, r in p["prof"])
            s.update(kind="ellipse", cx=cc[0], cy=cc[1], rx=r6(rmax), ry=r6(rmax))
            return s
        if not p.get("dlo") and not p.get("dhi"):
            return poly_shape(s, [to_paper(view, p["bone"], q) for q in rev_across(p, view)])
    a = to_paper(view, p["bone"], plane_of(view, lo))
    b = to_paper(view, p["bone"], plane_of(view, hi))
    return poly_shape(s, [[min(a[0], b[0]), min(a[1], b[1])], [max(a[0], b[0]), min(a[1], b[1])], [max(a[0], b[0]), max(a[1], b[1])], [min(a[0], b[0]), max(a[1], b[1])]])


def blanket_parts(sc):
    """🧺️ The folded moving blanket under the dryer: 1.8 x 1.5 m quilted felt folded to 0.90 x 0.75 m in four layers."""
    flag = {"sheet": True} if sc == 200 else {}
    return [box("blanket", "blanket", "moving-blanket", 0.45, "blanket", (-0.45, 0.0, -0.375), (0.45, Y0, 0.375), hatch="quilt" if sc == 50 else None, hatchViews="efp", **flag)]


def unit_parts(sc):
    """🧺️ The dryer cabinet, per scale: feet, base, plinth flap, steel side, back and front panels, top cover, fascia, knob, cable coil."""
    U = []
    f50, f100, f200 = sc == 50, sc == 100, sc == 200
    yt = Y0 + (0.82 if f200 else 0.83)
    if f50:
        for i, (x, z) in enumerate(FOOT_XZ):
            U.append(rev(f"foot-{i}", "unit", "foot-rubber", 0.2, "plinth", "y", (x, 0, z), [[Y0, 0.0], [Y0, 0.02], [Y0 + 0.007, 0.02], [Y0 + 0.007, 0.011], [Y0 + 0.015, 0.011], [Y0 + 0.015, 0.0]], views=("e" if z < 0 else "") + ("f" if x > 0 else "")))
    elif f100:
        for i, (x, z) in enumerate(FOOT_XZ):
            U.append(disc(f"foot-{i}", "unit", "foot-rubber", 0.2, "plinth", "y", (x, 0, z), 0.02, Y0, Y0 + 0.015, views=("e" if z < 0 else "") + ("f" if x > 0 else "")))
    U.append(box("base", "unit", "base-pp", 0.35, "plinth", (-0.32, Y0 if f200 else Y0 + 0.015, -0.3), (XF, Y0 + 0.04, 0.3), views="ef"))
    U.append(box("plinth-flap", "unit", "base-pp", 0.55, "plinth", (XP - max(0.011, FEATURE[sc]), Y0 + 0.04, -0.2992), (XP, Y0 + 0.185, 0.2992), views="ef", ze=1.95))
    U.append(box("side-right", "unit", "enamel-steel", 0.86, "cabinet", (-0.32, Y0 + 0.04, -0.3), (XF, yt, -0.2992), views="ef", sheet=True))
    U[-1]["fbox"] = ((-0.32, Y0 + 0.04, -0.3), (XF, yt, 0.3))
    U.append(box("side-left", "unit", "enamel-steel", 0.86, "cabinet", (-0.32, Y0 + 0.04, 0.2992), (XF, yt, 0.3), views="", sheet=True))
    U.append(box("back", "unit", "enamel-steel", 0.8, "cabinet", (-0.32, Y0 + 0.04, -0.2992), (-0.3192, yt, 0.2992), views="", sheet=True))
    U.append(part("front-panel", "unit", "panel", "enamel-steel", 0.88, "cabinet", "f", lo=[0.2742, Y0 + 0.185, -0.2992], hi=[XF, Y0 + 0.72, 0.2992], hole=(YC, 0.0, 0.18), sheet=True))
    U.append(box("top-cover", "unit", "top-pp", 0.92, "top-cover", (-0.32, yt, -0.3), (XP, Y0 + 0.85, 0.3), views="efp"))
    U.append(box("fascia", "unit", "fascia-abs", 0.78, "fascia", (0.245, Y0 + 0.72, -0.2992), (XP, yt, 0.1), views="ef", ze=1.95))
    if f50:
        U.append(rev("knob", "unit", "fascia-abs", 0.6, "fascia", "x", (0, Y0 + 0.775, -0.06), [[XP, 0.0], [XP, 0.028], [XP + 0.014, 0.028], [XP + 0.022, 0.023], [XP + 0.024, 0.0]], views="efp"))
        U.append(box("display", "unit", "display-glass", 0.12, "fascia", (XP, Y0 + 0.758, -0.25), (XP + 0.0005, Y0 + 0.795, -0.13), views="f", texture=True, sheet=True, zf=4.2))
        U.append(ring("cable-coil", "unit", "pvc-cable", 0.3, "cable", "x", (0, Y0 + 0.62, -0.15), [[-0.345, 0.045], [-0.345, 0.07], [-0.32, 0.07], [-0.32, 0.045]], views="ep"))
    elif f100:
        U.append(disc("knob", "unit", "fascia-abs", 0.6, "fascia", "x", (0, Y0 + 0.775, -0.06), 0.028, XP, XP + 0.022, views="efp"))
        U.append(ring("cable-coil", "unit", "pvc-cable", 0.3, "cable", "x", (0, Y0 + 0.62, -0.15), [[-0.345, 0.04], [-0.345, 0.07], [-0.32, 0.07], [-0.32, 0.04]], views="ep"))
    else:
        U.append(disc("cable-coil", "unit", "pvc-cable", 0.3, "cable", "x", (0, Y0 + 0.62, -0.15), 0.07, -0.35, -0.32, views="ep"))
    U.append(part("drum-mouth", "unit", "box2d", "stainless-steel", 0.12, None, "f", lo=[0.2, YC - 0.18, -0.18], hi=[0.2, YC + 0.18, 0.18], circle=(YC, 0.0, 0.18), nosolid=True, zf=2.15))
    return U


def door_parts(sc):
    """🚪️ The porthole door on its vertical hinge: yellowed ring, domed glass, hinge block."""
    D = []
    if sc == 50:
        D.append(ring("door-ring", "door", "door-abs", 0.72, "door", "x", (0, YC, 0.0), [[XF, 0.155], [XF, 0.23], [0.3, 0.23], [0.31, 0.217], [0.31, 0.17], [0.302, 0.155]],
                      sil=[[XF, 0.23], [0.3, 0.23], [0.31, 0.217]], zf=2.31))
        D.append(rev("door-glass", "door", "door-glass", 0.25, "door", "x", (0, YC, 0.0), [[0.3, 0.0], [0.3, 0.155], [XF, 0.155], [0.245, 0.13], [0.225, 0.0]], views="f", role="glazing", zf=2.32))
        D.append(box("door-hinge", "door", "hinge-zinc", 0.4, "door", (XF, YC - 0.05, -0.245), (0.29, YC + 0.05, -0.225), views=""))
    elif sc == 100:
        D.append(ring("door-ring", "door", "door-abs", 0.72, "door", "x", (0, YC, 0.0), [[XF, 0.155], [XF, 0.23], [0.31, 0.23], [0.31, 0.155]], zf=2.31))
        D.append(rev("door-glass", "door", "door-glass", 0.25, "door", "x", (0, YC, 0.0), [[0.3, 0.0], [0.3, 0.155], [XF, 0.155], [0.23, 0.0]], views="f", role="glazing", zf=2.32))
        D.append(box("door-hinge", "door", "hinge-zinc", 0.4, "door", (XF, YC - 0.05, -0.25), (0.29, YC + 0.05, -0.225), views=""))
    else:
        D.append(disc("door-ring", "door", "door-abs", 0.72, "door", "x", (0, YC, 0.0), 0.23, XF, 0.31, zf=2.31))
        D.append(rev("door-glass", "door", "door-glass", 0.25, "door", "x", (0, YC, 0.0), [[0.31, 0.0], [0.31, 0.15], [0.27, 0.15], [0.23, 0.0]], views="f", role="glazing", zf=2.32))
    return D


def tank_parts(sc):
    """🫙️ The condensate tank drawer at the top left of the fascia."""
    if sc == 200:
        return [box("tank-front", "tank", "tank-pp", 0.7, "tank", (-0.05, Y0 + 0.72, 0.1), (XP, Y0 + 0.82, 0.2992), views="efp", ze=1.92, zp=2.84)]
    return [box("tank-front", "tank", "tank-pp", 0.7, "tank", (0.245, Y0 + 0.72, 0.1), (XP, Y0 + 0.83, 0.2992), views="efp", ze=1.92, zp=2.84),
            box("tank-body", "tank", "tank-pp", 0.62, "tank", (-0.05, Y0 + 0.73, 0.11), (0.245, Y0 + 0.82, 0.29), views="ep", ze=1.91, zp=2.83)]


def filter_parts(sc):
    """🧹️ The fluff filter in the bottom of the door aperture."""
    if sc == 200:
        return []
    return [box("lint-filter", "lint-filter", "filter-pp", 0.55, "door", (0.21, Y0 + 0.277, -0.1), (0.274, Y0 + 0.297, 0.1), views="f", zf=2.16)]


def hidden_parts(sc):
    """🔥️ Drum and heat-pump module: real solids inside the closed cabinet, in the thermal list only."""
    H = []
    if sc == 50:
        H.append(rev("drum", "unit", "stainless-steel", 0.5, "drum", "x", (0, YC, 0.0), [[-0.29, 0.0], [-0.29, 0.25], [-0.27, 0.27], [0.18, 0.27], [0.2, 0.22], [0.2, 0.0]]))
        H.append(disc("compressor", "unit", "compressor-steel", 0.2, "heat-pump", "y", (-0.22, 0, -0.2), 0.055, Y0 + 0.045, Y0 + 0.175))
        H.append(box("heat-exchangers", "unit", "aluminium-fin", 0.6, "heat-pump", (-0.1, Y0 + 0.045, -0.12), (0.2, Y0 + 0.165, 0.22)))
        H.append(disc("drum-motor", "unit", "motor-steel", 0.3, "heat-pump", "x", (0, Y0 + 0.11, 0.2), 0.055, -0.31, -0.16))
    elif sc == 100:
        H.append(disc("drum", "unit", "stainless-steel", 0.5, "drum", "x", (0, YC, 0.0), 0.27, -0.29, 0.2))
        H.append(disc("compressor", "unit", "compressor-steel", 0.2, "heat-pump", "y", (-0.22, 0, -0.2), 0.055, Y0 + 0.045, Y0 + 0.175))
        H.append(box("heat-exchangers", "unit", "aluminium-fin", 0.6, "heat-pump", (-0.14, Y0 + 0.045, -0.12), (0.2, Y0 + 0.165, 0.255)))
    else:
        H.append(disc("drum", "unit", "stainless-steel", 0.5, "drum", "x", (0, YC, 0.0), 0.27, -0.29, 0.2))
        H.append(box("heat-pump-module", "unit", "aluminium-fin", 0.6, "heat-pump", (-0.3, Y0 + 0.045, -0.26), (0.2, Y0 + 0.175, 0.26)))
    return H


def strokes(sc):
    """〰️ Detail strokes and texture marks per view: seams, dents, scratches, faded print, tape, grille slots, stickers."""
    E, F, P = [], [], []

    def ln(lst, sid, bone, pts, z, tone=0.4, role="detail", closed=False):
        lst.append({"id": sid, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed})

    def pa(lst, sid, bone, d, z, tone=0.45, role="detail"):
        lst.append({"id": sid, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "kind": "path", "d": d})

    ux = lambda x: x + 0.32
    if sc in (50, 100):
        ln(E, "front-edge", "unit", [(ux(XF), 0.04), (ux(XF), 0.83)], 3.5)
        ln(E, "top-seam", "unit", [(0.0, 0.83), (ux(XP), 0.83)], 3.5, 0.35)
        ln(F, "plinth-seam", "unit", [(-0.2992, Y0 + 0.185), (0.2992, Y0 + 0.185)], 3.5)
        ln(F, "fascia-seam", "unit", [(-0.2992, Y0 + 0.72), (0.2992, Y0 + 0.72)], 3.5)
        for k, y in enumerate([0.07, 0.085, 0.1] if sc == 50 else [0.075, 0.1]):
            ln(F, f"grille-slot-{k}", "unit", [(-0.1, Y0 + y), (0.1, Y0 + y)], 3.6, 0.3)
        F.append(door_mark("tape", "texture", 0.92, (-0.018, 0.018), (0.17, 0.295), (-0.036, -0.035), 4.5, "packing-tape"))
        ln(F, "tank-grip", "tank", [(0.16, Y0 + 0.735), (0.24, Y0 + 0.735)], 4.0, 0.3)
        ln(P, "rear-rim", "unit", [(-0.3, -0.3), (-0.3, 0.3)], 3.5, 0.35)
        P.append({"id": "price-sticker", "bone": "unit", "role": "texture", "z": 4.5, "filled": True, "tone": 0.97, "kind": "rect", "x": 0.05, "y": 0.12, "w": 0.1, "h": 0.06, "material": "packing-tape"})
        pa(P, "top-scratches", "unit", "M -0.2 -0.15 L -0.05 -0.1 M -0.18 -0.18 L -0.02 -0.135 M 0.1 0.05 L 0.2 0.0", 3.6, 0.5)
    if sc == 50:
        pa(E, "side-dent", "unit", "M 0.13 0.25 L 0.16 0.235 L 0.2 0.232 L 0.23 0.245", 3.6, 0.5)
        pa(E, "corner-scuffs", "unit", "M 0.01 0.06 L 0.08 0.09 M 0.02 0.1 L 0.07 0.12 M 0.55 0.05 L 0.59 0.08", 3.6, 0.55)
        ln(E, "label-residue", "unit", [(0.1, 0.55), (0.22, 0.55), (0.22, 0.63), (0.1, 0.63)], 3.6, 0.6, closed=True)
        pa(E, "coil-tape", "unit", "M -0.025 0.585 L 0.0 0.585 M -0.025 0.655 L 0.0 0.655", 3.7, 0.9)
        for k in range(8):
            a = 2 * math.pi * k / 8 + 0.3
            ln(F, f"knob-tick-{k}", "unit", [(-0.06 + 0.033 * math.cos(a), Y0 + 0.775 + 0.033 * math.sin(a)), (-0.06 + 0.04 * math.cos(a), Y0 + 0.775 + 0.04 * math.sin(a))], 4.1, 0.55)
        for k, z in enumerate([0.03, 0.06]):
            F.append({"id": f"button-{k}", "bone": "unit", "role": "detail", "z": 4.1, "filled": False, "tone": 0.5, "kind": "ellipse", "cx": z, "cy": r6(Y0 + 0.775), "rx": 0.009, "ry": 0.009})
        pa(F, "flap-scuffs", "unit", "M -0.22 0.075 L -0.16 0.09 M 0.12 0.07 L 0.2 0.082 M -0.05 0.064 L 0.02 0.07", 3.7, 0.55)
        pa(F, "faded-print", "unit", "M -0.28 0.8 L -0.26 0.8 M -0.1 0.81 L -0.08 0.81", 4.1, 0.75)
        F.append(door_mark("handle-recess", "detail", 0.3, (-0.06, 0.06), (0.2, 0.214), (-0.036, -0.035), 4.0, "door-abs"))
        F.append(door_mark("glass-lint", "detail", 0.7, (-0.135, -0.118), (-0.08, 0.09), (-0.026, -0.025), 4.0, "door-glass"))
        P.append({"id": "cup-ring", "bone": "unit", "role": "detail", "z": 3.7, "filled": False, "tone": 0.55, "kind": "ellipse", "cx": -0.1, "cy": 0.12, "rx": 0.04, "ry": 0.04})
        ln(P, "adhesive-residue", "unit", [(-0.25, -0.05), (-0.17, -0.05), (-0.17, 0.02), (-0.25, 0.02)], 3.7, 0.6, closed=True)
    return E, F, P


def door_mark(sid, role, tone, us, zs, vs, z, material):
    """🩹️ A filled mark on the door (tape, grip recess, lint film): a thin plate on the door bone, swept along its normal so it turns with the door."""
    w0, w1 = zs[0] - ZH, zs[1] - ZH
    return {"id": sid, "bone": "door", "role": role, "z": z, "filled": True, "tone": tone, "material": material, "kind": "rect", "x": us[0], "y": r6(w0), "w": r6(us[1] - us[0]), "h": r6(w1 - w0),
            "solid": {"kind": "extruded", "sweep": "v", "z0": vs[0], "z1": vs[1]}}


def special_shapes(p, view):
    """🎯️ Shapes the generic outline does not draw: the side-panel cell of the front view and the drum mouth."""
    if p["id"] == "side-right" and view == "front":
        lo, hi = p["fbox"]
        return {"id": p["id"], "bone": "unit", "role": "body", "z": 2.2, "filled": True, "tone": p["tone"], "material": p["mat"], "region": "cabinet", "kind": "rect",
                "x": r6(lo[2]), "y": r6(lo[1]), "w": r6(hi[2] - lo[2]), "h": r6(hi[1] - lo[1])}
    if p["id"] == "drum-mouth" and view == "front":
        cy, cz, rr = p["circle"]
        return {"id": p["id"], "bone": "unit", "role": "body", "z": p["zf"], "filled": True, "tone": p["tone"], "material": p["mat"], "kind": "ellipse", "cx": cz, "cy": r6(cy), "rx": rr, "ry": rr}
    return None


def build_scale(sc):
    """🏗️ Body, thermal and structure lists and the three drawings of one authored scale."""
    parts = blanket_parts(sc) + unit_parts(sc) + door_parts(sc) + tank_parts(sc) + filter_parts(sc)
    solid_parts = [p for p in parts if not p.get("nosolid")]
    body = [solid_of(p) for p in solid_parts]
    thermal = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in solid_parts]
    for p in hidden_parts(sc):
        thermal.append(dict(solid_of(p), id=f"th-{p['id']}", region=p["region"]))
    f = FEATURE[sc]
    vent = box("plinth-vent", "unit", "base-pp", 0.5, "plinth", (XP, Y0 + 0.06, -0.1), (XP + f, Y0 + 0.09, 0.1))
    thermal.append(dict(solid_of(vent), id="th-plinth-vent", region="plinth", tags=["plinth-vent"]))
    structure = structure_of(sc, f)
    geom = {"elevation": [], "front": [], "plan": []}
    for p in parts:
        for v, key in (("e", "elevation"), ("f", "front"), ("p", "plan")):
            sp = special_shapes(p, key)
            if sp:
                geom[key].append(sp)
            elif v in p["views"]:
                geom[key].append(shape_of(p, key))
    for key in geom:
        for k, sh in enumerate(geom[key]):
            sh["z"] = round(sh["z"] + k * 1e-4, 5)
    E, F, P = strokes(sc)
    geom["elevation"] += E
    geom["front"] += F
    geom["plan"] += P
    return parts, body, thermal, structure, geom


def mass_marker(mid, bone, com, r, mat):
    """⚖️ A centre-of-mass marker solid."""
    return {"id": mid, "kind": "ellipsoid", "bone": bone, "material": mat, "role": "detail", "tone": 0.5, "radii": [r, r, r], "transform": {"translation": [com[0], com[1], 0.0]}, "member": bone, "part": "mass"}


def sbox(sid, bone, lo, hi, member, part_kind, mat="enamel-steel", role="body", **kw):
    """🔩️ A structure-only box from world corners."""
    c = [(a + b) / 2 for a, b in zip(lo, hi)]
    sz = [b - a for a, b in zip(lo, hi)]
    if BONES[bone][1]:
        sz = [sz[1], sz[0], sz[2]]
    s = {"id": sid, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": 0.3, "size": [r6(v) for v in sz], "transform": {"translation": to_local(bone, c)}, "member": member, "part": part_kind}
    s.update(kw)
    return s


def structure_of(sc, f):
    """🏗️ The load path of one scale: blanket pad, side panels and base of the cabinet, feet, door hinge, tank rail, masses."""
    S = [{"id": "m-blanket", "of": "blanket", "member": "blanket", "part": "member"}]
    pad = max(0.006, f)
    S.append(sbox("support-blanket-floor", "blanket", (-0.3, 0.0, -0.25), (0.3, pad, 0.25), "blanket", "support", mat="moving-blanket", role="ground", anchor="blanket-floor"))
    for nm in ("side-right", "side-left", "base"):
        S.append({"id": f"m-{nm}", "of": nm, "member": "unit", "part": "member"})
    for i, (x, z) in enumerate(FOOT_XZ):
        if sc == 200:
            S.append(sbox(f"j-{FEET[i]}", "unit", (x - 0.02, Y0, z - 0.02), (x + 0.02, Y0 + 0.03, z + 0.02), "unit", "joint", mat="foot-rubber"))
        else:
            S.append({"id": f"j-{FEET[i]}", "of": f"foot-{i}", "member": "unit", "part": "joint"})
    S.append({"id": "m-door", "of": "door-ring", "member": "door", "part": "member"})
    hv = max(0.02, f)
    S.append(sbox("j-door-hinge", "door", (XF - hv / 2, YC - 0.05, -0.2992), (XF + hv / 2, YC + 0.05, -0.215), "door", "joint", mat="hinge-zinc"))
    if sc == 200:
        S.append({"id": "m-tank", "of": "tank-front", "member": "tank", "part": "member"})
    else:
        S.append({"id": "m-tank", "of": "tank-body", "member": "tank", "part": "member"})
        S.append(sbox("j-tank-rail", "tank", (-0.05, Y0 + 0.76, 0.28), (0.245, Y0 + 0.79, 0.2992), "tank", "joint", mat="tank-pp"))
    mr = f / 2
    S.append(mass_marker("mass-unit", "unit", TX.UNIT_COM, mr, "enamel-steel"))
    S.append(mass_marker("mass-door", "door", TX.DOOR_COM, mr, "door-abs"))
    S.append(mass_marker("mass-tank", "tank", TX.TANK_COM, mr, "tank-pp"))
    S.append(mass_marker("mass-blanket", "blanket", TX.BLANKET_COM, mr, "moving-blanket"))
    return S


def bbox2(s):
    """📦️ Paper-plane bounds of a filled 2D shape."""
    k = s["kind"]
    if k == "rect":
        return s["x"], s["y"], s["x"] + s["w"], s["y"] + s["h"]
    if k == "ellipse":
        return s["cx"] - s["rx"], s["cy"] - s["ry"], s["cx"] + s["rx"], s["cy"] + s["ry"]
    if k == "path":
        nums = [float(t) for t in s["d"].replace("M", " ").replace("L", " ").replace("Z", " ").split()]
        xs, ys = nums[0::2], nums[1::2]
        return min(xs), min(ys), max(xs), max(ys)
    xs = [q[0] for q in s["points"]]
    ys = [q[1] for q in s["points"]]
    return min(xs), min(ys), max(xs), max(ys)


def report(sc, body, thermal, structure, geom):
    """🧾️ Counts and extents of one scale for the report."""
    ext = {}
    for view, shapes in geom.items():
        bs = [bbox2(s) for s in shapes if s["filled"] and s["bone"] not in ("door",) and s["role"] != "texture"]
        ext[view] = tuple(round(v, 4) for v in (min(b[0] for b in bs), min(b[1] for b in bs), max(b[2] for b in bs), max(b[3] for b in bs)))
    print(f"[DEBUG] 1:{sc} body {len(body)} thermal {len(thermal)} structure {len(structure)} shapes " + " ".join(f"{k}={len(v)}" for k, v in geom.items()) + f" extents {ext}")


def main():
    """✍️ Writes the asset document."""
    reps, vols = {}, {}
    for sc in (50, 100, 200):
        parts, body, thermal, structure, geom = build_scale(sc)
        report(sc, body, thermal, structure, geom)
        reps[str(sc)] = {"status": "authored", "denominator": sc, "validRange": {"minDenominator": TX.RANGES[sc][0], "maxDenominator": TX.RANGES[sc][1]}, "pens": TX.PENS[sc], "hatches": TX.HATCHES[sc], "geometry": geom, "decisions": TX.DECISIONS[sc]}
        vols[str(sc)] = {"status": "authored", "denominator": sc, "body": body, "thermal": thermal, "structure": structure, "decisions": TX.VOL_DECISIONS[sc]}
    reps["500"] = TX.UNAVAILABLE_500
    vols["500"] = {"status": "unavailable", "denominator": 500}
    asset = TX.asset_shell()
    asset["representations"] = reps
    asset["volumes"] = vols
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print(f"[DEBUG] wrote {OUT}")


if __name__ == "__main__":
    main()
