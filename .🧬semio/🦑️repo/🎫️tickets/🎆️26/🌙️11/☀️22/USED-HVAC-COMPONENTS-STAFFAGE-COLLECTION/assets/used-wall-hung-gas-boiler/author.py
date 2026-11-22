"""🔥 Authoritative generator of the staffage asset `used-wall-hung-gas-boiler`.

Every hand-chosen number of the used 24 kW wall-hung gas condensing combi boiler on its Euro pallet lives here; the script
writes `🔣️.json` next to itself (2D drawings, 3D volumes, thermal and structural models are all derived from one part list
per scale, so the views and the solids agree by construction).

See the report `../../reports/used-wall-hung-gas-boiler.md` for the reference table and the decisions.
"""

import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

T = 0.002
YB = 0.224
PX, PY = -0.183, 0.904
STRAP_Z = (-0.16, 0.16)
STRAP_W = 0.05
FB = (-0.605, 0.150)
CB = (-0.185, YB + 0.70)
FF = (0.605, 0.150)
CF = (0.175, YB + 0.70)
S1 = 0.150 - T
DECK = 0.144
LIFT_DY = 0.24
WALL_DY = 1.0
TILT = -30.0


def r(v, n=5):
    """🔢 Rounds a number for the document."""
    return round(v + 0.0, n)


def tx(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


def ang(p, q):
    """📐 Angle in degrees and length from point p to q."""
    dx, dy = q[0] - p[0], q[1] - p[1]
    return math.degrees(math.atan2(dy, dx)), math.hypot(dx, dy)


AB, LB = ang(FB, CB)
AF, LF = ang(FF, CF)
TOPB = 0.0 - CB[0]
TOPF = CF[0] - 0.0

BONES = {}


def bone(id_, parent, x, y, a, length, joint, title, mass=None, z=None):
    """🦴 Registers a bone (rest placement relative to its parent)."""
    b = {"id": id_, "parent": parent, "title": title, "x": r(x), "y": r(y), "angleDeg": r(a, 4), "lengthM": r(length), "joint": joint}
    if z is not None:
        b["z"] = z
    if mass:
        b["mass"] = mass
    BONES[id_] = b
    return b


def frame(id_):
    """🧭 World rest origin and world angle of a bone."""
    b = BONES[id_]
    if b["parent"] is None:
        return (b["x"], b["y"]), b["angleDeg"]
    (ox, oy), pa = frame(b["parent"])
    c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
    return (ox + c * b["x"] - s * b["y"], oy + s * b["x"] + c * b["y"]), pa + b["angleDeg"]


def to_local(id_, x, y):
    """↩️ World rest point to bone-local (u, v)."""
    (ox, oy), a = frame(id_)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    dx, dy = x - ox, y - oy
    return c * dx + s * dy, -s * dx + c * dy


def to_world(id_, u, v):
    """↪️ Bone-local (u, v) to world rest point."""
    (ox, oy), a = frame(id_)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return ox + c * u - s * v, oy + s * u + c * v


def build_rig():
    """🦴 The rig: pallet root, boiler lift carrier, boiler, bracket, drip and the two folding halves of the strap pair."""
    BONES.clear()
    bone("pallet", None, -0.6, 0.0, 0.0, 1.2, {"type": "fixed"}, tx("Euro pallet with the two squared bearers and the four strap hooks (stays on the ground)", "Europalette mit den zwei Kanthölzern und den vier Zurrhaken (bleibt am Boden)"))
    bone("boiler-lift", "pallet", PX + 0.6, PY, 0.0, 0.05, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 1.2}, tx("Vertical lift of the boiler (crane hook or wall height)", "Senkrechter Hub des Kessels (Kranhaken oder Wandhöhe)"))
    bone("boiler", "boiler-lift", 0.0, 0.0, 0.0, 0.358, {"type": "revolute", "minDeg": -35, "maxDeg": 5}, tx("Boiler casing with heat exchanger, adapter and capped connections, pivoting about the hanging point of its bracket", "Kesselgehäuse mit Wärmeübertrager, Adapter und verschlossenen Anschlüssen, drehbar um den Aufhängepunkt seiner Wandschiene"))
    bone("bracket", "boiler", 0.0, 0.0, 0.0, 0.016, {"type": "fixed"}, tx("Wall-hanging rail taken off the old wall, hooked into the hanging pocket", "Von der alten Wand abgenommene Aufhängeschiene, in die Aufhängetasche eingehängt"))
    du, dv = to_local("boiler", 0.07, YB - 0.07)
    bone("drip", "boiler", du, dv, 0.0, 0.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.4, "maxM": 0}, tx("Condensate drop at the cut drain hose", "Kondensattropfen am abgeschnittenen Ablaufschlauch"))
    bone("strap-back-foot", "pallet", FB[0] + 0.6, FB[1], AB, S1, {"type": "revolute", "minDeg": -5, "maxDeg": 210}, tx("Back strap ends in their hooks at the rear deck edge", "Hintere Gurtenden in ihren Haken an der hinteren Deckkante"))
    bone("strap-back-leg", "strap-back-foot", S1, 0.0, 0.0, LB - S1, {"type": "revolute", "minDeg": -95, "maxDeg": 5}, tx("Back legs of the two ratchet straps", "Hintere Schenkel der zwei Ratschengurte"))
    bone("strap-back-top", "strap-back-leg", LB - S1, 0.0, -AB, TOPB, {"type": "revolute", "minDeg": -5, "maxDeg": 70}, tx("Back halves of the straps over the casing top", "Hintere Hälften der Gurte über der Gehäusedecke"))
    bone("strap-front-foot", "pallet", FF[0] + 0.6, FF[1], AF, S1, {"type": "revolute", "minDeg": -210, "maxDeg": 5}, tx("Front strap ends in their hooks at the front deck edge", "Vordere Gurtenden in ihren Haken an der vorderen Deckkante"))
    bone("strap-front-leg", "strap-front-foot", S1, 0.0, 0.0, LF - S1, {"type": "revolute", "minDeg": -5, "maxDeg": 95}, tx("Front legs of the two ratchet straps", "Vordere Schenkel der zwei Ratschengurte"))
    bone("strap-front-top", "strap-front-leg", LF - S1, 0.0, 180.0 - AF, TOPF, {"type": "revolute", "minDeg": -70, "maxDeg": 5}, tx("Front halves of the straps with the ratchets", "Vordere Hälften der Gurte mit den Ratschen"))


DENS = {"pallet-pine": 550.0, "spruce": 450.0, "steel": 7850.0, "galv": 7850.0}

PARTS = []


def part(id_, bone_, kind, geo, material, tone, region=None, role="body", views=("elevation", "front", "plan"), texture=False, sheet=False, z=None, regions=None, thermal=True, member=None, joint=None, hide2d=()):
    """🧩 One physical part: geometry in the bone frame, its 2D twin per view and its thermal and structural use."""
    p = {"id": id_, "bone": bone_, "kind": kind, "geo": geo, "material": material, "tone": tone, "region": region, "role": role, "views": [v for v in views if v not in hide2d], "texture": texture, "sheet": sheet, "z": z, "regions": regions, "thermal": thermal, "member": member}
    PARTS.append(p)
    return p


def wbox(bone_, x0, x1, y0, y1, z0, z1):
    """📦 A box given by world rest extents, expressed in an axis-parallel bone frame."""
    (ox, oy), a = frame(bone_)
    assert abs(a) < 1e-9
    zb = BONES[bone_].get("z", 0.0)
    return {"c": [(x0 + x1) / 2 - ox, (y0 + y1) / 2 - oy, (z0 + z1) / 2 - zb], "s": [x1 - x0, y1 - y0, z1 - z0]}


def lbox(u0, u1, v0, v1, w0, w1):
    """📦 A box given by bone-local extents."""
    return {"c": [(u0 + u1) / 2, (v0 + v1) / 2, (w0 + w1) / 2], "s": [u1 - u0, v1 - v0, w1 - w0]}


def wcyl(bone_, axis, cx, cy, cz, r_, a0, a1):
    """🛢 A cylinder along a world axis ('x', 'y' or 'z') with its axial range in world coordinates."""
    (ox, oy), _ = frame(bone_)
    off = {"x": ox, "y": oy, "z": 0.0}[axis]
    return {"axis": axis, "c": [cx - ox, cy - oy, cz], "r": r_, "a0": a0 - off, "a1": a1 - off}


def wrev(bone_, cx, cz, prof):
    """🏺 A revolve about a vertical axis through world (cx, cz); profile of (world y, radius) pairs, open or closed."""
    (ox, oy), _ = frame(bone_)
    return {"c": [cx - ox, 0.0, cz], "prof": [(y - oy, rr) for y, rr in prof]}


def box_world_rect(p):
    """🗺 World rest extents (x0, x1, y0, y1, z0, z1) of a part."""
    g = p["geo"]
    b = p["bone"]
    zb = BONES[b].get("z", 0.0)
    if p["kind"] == "box":
        cu, cv, cw = g["c"]
        su, sv, sw = g["s"]
        pts = [to_world(b, cu + du * su / 2, cv + dv * sv / 2) for du in (-1, 1) for dv in (-1, 1)]
        xs, ys = [q[0] for q in pts], [q[1] for q in pts]
        return min(xs), max(xs), min(ys), max(ys), zb + cw - sw / 2, zb + cw + sw / 2
    if p["kind"] == "cyl":
        cu, cv, cw = g["c"]
        rr = g["r"]
        if g["axis"] == "y":
            x, y0 = to_world(b, cu, g["a0"])
            _, y1 = to_world(b, cu, g["a1"])
            return x - rr, x + rr, y0, y1, cw - rr, cw + rr
        if g["axis"] == "x":
            x0, y = to_world(b, g["a0"], cv)
            x1, _ = to_world(b, g["a1"], cv)
            return x0, x1, y - rr, y + rr, cw - rr, cw + rr
        x, y = to_world(b, cu, cv)
        return x - rr, x + rr, y - rr, y + rr, g["a0"], g["a1"]
    if p["kind"] == "rev":
        cu, _, cw = g["c"]
        ys = [to_world(b, cu, v)[1] for v, _ in g["prof"]]
        rm = max(rr for _, rr in g["prof"])
        x, _ = to_world(b, cu, 0)
        return x - rm, x + rm, min(ys), max(ys), cw - rm, cw + rm
    if p["kind"] == "ell":
        cu, cv, cw = g["c"]
        x, y = to_world(b, cu, cv)
        return x - g["r"], x + g["r"], y - g["r"], y + g["r"], cw - g["r"], cw + g["r"]
    raise ValueError(p["kind"])


def solid_of(p):
    """🧊 The 3D solid of a part in its bone frame."""
    g = p["geo"]
    base = {"id": p["id"], "bone": p["bone"], "material": p["material"], "role": p["role"], "tone": p["tone"]}
    if p["kind"] == "box":
        s = dict(base, kind="box", size=[r(v) for v in g["s"]], transform={"translation": [r(v) for v in g["c"]]})
        if g.get("cr"):
            s["cornerRadius"] = g["cr"]
    elif p["kind"] == "cyl":
        ax = {"x": [1, 0, 0], "y": [0, 1, 0], "z": [0, 0, 1]}[g["axis"]]
        cu, cv, cw = g["c"]
        org = {"x": [0, cv, cw], "y": [cu, 0, cw], "z": [cu, cv, 0]}[g["axis"]]
        if "ri" in g:
            prof = [[r(g["a0"]), r(g["ri"])], [r(g["a0"]), r(g["r"])], [r(g["a1"]), r(g["r"])], [r(g["a1"]), r(g["ri"])], [r(g["a0"]), r(g["ri"])]]
        else:
            prof = [[r(g["a0"]), r(g["r"])], [r(g["a1"]), r(g["r"])]]
        s = dict(base, kind="revolve", profile=prof, axis=ax, origin=[r(v) for v in org])
    elif p["kind"] == "rev":
        cu, _, cw = g["c"]
        s = dict(base, kind="revolve", profile=[[r(a), r(b)] for a, b in g["prof"]], axis=[0, 1, 0], origin=[r(cu), 0, r(cw)])
    elif p["kind"] == "ell":
        s = dict(base, kind="ellipsoid", radii=[g["r"]] * 3, transform={"translation": [r(v) for v in g["c"]]})
    if p["texture"]:
        s["texture"] = True
    if p["sheet"]:
        s["sheet"] = True
    return s


def rect(x0, x1, y0, y1):
    """⬛ A rect geometry."""
    return {"kind": "rect", "x": r(x0), "y": r(y0), "w": r(x1 - x0), "h": r(y1 - y0)}


def poly(pts, closed=True):
    """🔷 A poly geometry."""
    return {"kind": "poly", "points": [[r(a), r(b)] for a, b in pts], "closed": closed}


def shapes_of(p, z):
    """✏️ The 2D shapes of a part: elevation in bone-local (u, v), front in world (z, y), plan in world (x, z), all at rest."""
    out = {}
    g = p["geo"]
    b = p["bone"]
    x0, x1, y0, y1, z0, z1 = box_world_rect(p)
    if p["kind"] == "box":
        cu, cv, _ = g["c"]
        su, sv, _ = g["s"]
        el = rect(cu - su / 2, cu + su / 2, cv - sv / 2, cv + sv / 2)
        if g.get("cr"):
            el["r"] = g["cr"]
    elif p["kind"] == "cyl":
        cu, cv, _ = g["c"]
        rr = g["r"]
        if g["axis"] == "y":
            el = rect(cu - rr, cu + rr, g["a0"], g["a1"])
        elif g["axis"] == "x":
            el = rect(g["a0"], g["a1"], cv - rr, cv + rr)
        else:
            el = {"kind": "ellipse", "cx": r(cu), "cy": r(cv), "rx": rr, "ry": rr}
    elif p["kind"] == "rev":
        cu, _, _ = g["c"]
        el = poly(rev_poly(g["prof"], cu))
    elif p["kind"] == "ell":
        cu, cv, _ = g["c"]
        el = {"kind": "ellipse", "cx": r(cu), "cy": r(cv), "rx": g["r"], "ry": g["r"]}
    if p["kind"] == "cyl" and g["axis"] == "x":
        fr = {"kind": "ellipse", "cx": r((z0 + z1) / 2), "cy": r((y0 + y1) / 2), "rx": g["r"], "ry": g["r"]}
    elif p["kind"] == "rev":
        fr = poly(rev_poly(g["prof"], g["c"][2]))
        fr["points"] = [[q[0], r(q[1] + frame(b)[0][1])] for q in fr["points"]]
    else:
        fr = rect(z0, z1, y0, y1)
    if (p["kind"] == "cyl" and g["axis"] == "y") or p["kind"] == "rev" or p["kind"] == "ell":
        rr = (x1 - x0) / 2
        pl = {"kind": "ellipse", "cx": r((x0 + x1) / 2), "cy": r((z0 + z1) / 2), "rx": r(rr), "ry": r(rr)}
    else:
        pl = rect(x0, x1, z0, z1)
    for view, geo in (("elevation", el), ("front", fr), ("plan", pl)):
        if view not in p["views"]:
            continue
        s = {"id": p["id"], "bone": b, "role": p["role"], "z": z, "filled": True, "tone": p["tone"], "material": p["material"]}
        if p.get("pen"):
            s["penMm"] = p["pen"]
        if p["regions"]:
            s["regions"] = p["regions"]
        elif p["region"]:
            s["region"] = p["region"]
        s.update(geo)
        out[view] = s
    return out


def rev_poly(prof, centre):
    """🏺 Closed outline (lateral position, axial) of a revolve with an open, axially ordered (axial, radius) profile."""
    pts = [q for q in prof]
    right = [(centre + rr, a) for a, rr in pts]
    left = [(centre - rr, a) for a, rr in reversed(pts)]
    return right + left


SCALE = None


def build_parts(scale):
    """🧩 The part list of one scale: the same object, re-modelled for its smallest feature."""
    PARTS.clear()
    fine = scale == 50
    mid = scale == 100
    coarse = scale == 200
    yb = YB
    tb = [(-0.40, -0.255), (-0.21375, -0.11375), (-0.0725, 0.0725), (0.11375, 0.21375), (0.255, 0.40)]
    bx = [(-0.6, -0.455), (-0.0725, 0.0725), (0.455, 0.6)]
    bz = [(-0.40, -0.30), (-0.0725, 0.0725), (0.30, 0.40)]
    if coarse:
        part("deck", "pallet", "box", wbox("pallet", -0.6, 0.6, 0.114, DECK, -0.4, 0.4), "pallet-pine", 0.62, "pallet")
        for i, (xa, xb) in enumerate(bx):
            for j, (za, zb) in enumerate(bz):
                part(f"block-{i}{j}", "pallet", "box", wbox("pallet", xa, xb, 0.03, 0.114, za, zb), "pallet-pine", 0.5, "pallet")
        for j, (za, zb) in enumerate(bz):
            part(f"bottom-board-{j}", "pallet", "box", wbox("pallet", -0.6, 0.6, 0.0, 0.03, za, zb), "pallet-pine", 0.56, "pallet")
    else:
        for i, (za, zb) in enumerate(tb):
            part(f"deck-board-{i}", "pallet", "box", wbox("pallet", -0.6, 0.6, 0.122, DECK, za, zb), "pallet-pine", 0.62 - 0.02 * (i % 2), "pallet")
        for i, (xa, xb) in enumerate(bx):
            part(f"stringer-{i}", "pallet", "box", wbox("pallet", xa, xb, 0.1, 0.122, -0.4, 0.4), "pallet-pine", 0.54, "pallet")
        for i, (xa, xb) in enumerate(bx):
            for j, (za, zb) in enumerate(bz):
                part(f"block-{i}{j}", "pallet", "box", wbox("pallet", xa, xb, 0.022, 0.1, za, zb), "pallet-pine", 0.48, "pallet")
        for j, (za, zb) in enumerate(bz):
            part(f"bottom-board-{j}", "pallet", "box", wbox("pallet", -0.6, 0.6, 0.0, 0.022, za, zb), "pallet-pine", 0.56, "pallet")
    for k, zc in enumerate((-0.21, 0.21)):
        part(f"bearer-{k}", "pallet", "box", wbox("pallet", -0.2, 0.2, DECK, yb, zc - 0.02, zc + 0.02), "spruce", 0.7, "bearers")
    for side, (sx, sgn) in (("back", (-1, -1)), ("front", (1, 1))):
        for k, zc in enumerate(STRAP_Z):
            if fine:
                part(f"hook-{side}-plate-{k}", "pallet", "box", wbox("pallet", *sorted((sgn * 0.598, sgn * 0.612)), 0.108, 0.154, zc - 0.025, zc + 0.025), "steel", 0.25, "straps", role="detail")
                part(f"hook-{side}-lip-{k}", "pallet", "box", wbox("pallet", *sorted((sgn * 0.585, sgn * 0.598)), DECK, 0.154, zc - 0.025, zc + 0.025), "steel", 0.25, "straps", role="detail")
            else:
                hw = 0.015 if mid else 0.03
                part(f"hook-{side}-{k}", "pallet", "box", wbox("pallet", *sorted((sgn * (0.612 - 2 * hw), sgn * 0.612)), 0.108, 0.154, zc - 0.025, zc + 0.025), "steel", 0.25, "straps", role="detail" if mid else "body")
    part("casing", "boiler", "box", dict(wbox("boiler", -0.175, 0.175, yb, yb + 0.70, -0.22, 0.22), cr={50: 0.008, 100: 0.012, 200: 0.02}[scale]), "painted-steel-aged", 0.86, "casing", regions=None if not coarse else ["casing", "controls"])
    if not coarse:
        part("fascia", "boiler", "box", wbox("boiler", 0.175, 0.191, yb + 0.02, yb + 0.105, -0.20, 0.20), "polycarbonate", 0.42, "controls", role="detail")
    if fine:
        for k, zc in enumerate((0.10, 0.145)):
            part(f"knob-{k}", "boiler", "cyl", wcyl("boiler", "x", 0, yb + 0.0625, zc, 0.012, 0.191, 0.203), "polycarbonate", 0.22, "controls", role="detail")
        part("display", "boiler", "box", wbox("boiler", 0.191, 0.1915, yb + 0.045, yb + 0.08, -0.09, 0.01), "display-glass", 0.18, None, role="glazing", texture=True, views=("front",), thermal=False)
        part("service-label", "boiler", "box", wbox("boiler", 0.175, 0.1755, yb + 0.55, yb + 0.62, -0.19, -0.09), "paper-label", 0.97, None, role="texture", texture=True, views=("front",), thermal=False)
        part("type-plate", "boiler", "box", wbox("boiler", -0.12, -0.04, yb + 0.30, yb + 0.36, -0.2205, -0.22), "paper-label", 0.95, None, role="texture", texture=True, views=("elevation",), thermal=False)
    if fine or mid:
        part("adapter-collar", "boiler", "cyl", wcyl("boiler", "y", 0, 0, 0, 0.0625, yb + 0.70, yb + 0.725), "flue-aluminium", 0.8, "flue-adapter")
        part("adapter-air-pipe", "boiler", "cyl", dict(wcyl("boiler", "y", 0, 0, 0, 0.05, yb + 0.725, yb + 0.80), ri=0.048), "flue-aluminium", 0.78, "flue-adapter")
        part("adapter-flue-pipe", "boiler", "cyl", dict(wcyl("boiler", "y", 0, 0, 0, 0.03, yb + (0.725 if fine else 0.70), yb + 0.805), ri=0.0285), "polypropylene", 0.9, "flue-adapter")
    if fine:
        for k, yy in enumerate((yb + 0.745, yb + 0.778)):
            part(f"test-plug-{k}", "boiler", "cyl", wcyl("boiler", "x", 0, yy, 0, 0.007, 0.0485, 0.064), "polypropylene", 0.3, "flue-adapter", role="detail")
    if coarse:
        part("adapter", "boiler", "rev", wrev("boiler", 0, 0, [(yb + 0.70, 0.0625), (yb + 0.725, 0.0625), (yb + 0.725, 0.05), (yb + 0.805, 0.05)]), "flue-aluminium", 0.8, "flue-adapter")
    stubs = [(-0.13, 0.011, 0.015), (-0.065, 0.0075, 0.012), (0.0, 0.011, 0.015), (0.065, 0.0075, 0.012), (0.13, 0.011, 0.015)]
    if fine:
        for k, (zc, rs, rc) in enumerate(stubs):
            part(f"nut-{k}", "boiler", "cyl", wcyl("boiler", "y", -0.03, 0, zc, 0.0165, yb - 0.012, yb), "brass", 0.5, "connections")
            part(f"stub-{k}", "boiler", "cyl", wcyl("boiler", "y", -0.03, 0, zc, rs, yb - 0.045, yb - 0.012), "copper", 0.4, "connections")
            part(f"cap-{k}", "boiler", "cyl", wcyl("boiler", "y", -0.03, 0, zc, rc, yb - 0.065, yb - 0.045), "brass", 0.55, "connections", role="detail")
        part("trap", "boiler", "cyl", wcyl("boiler", "y", 0.07, 0, 0.0, 0.02, yb - 0.05, yb), "polypropylene", 0.88, "condensate-trap")
        part("drain-hose", "boiler", "cyl", wcyl("boiler", "y", 0.07, 0, 0.0, 0.0125, yb - 0.07, yb - 0.05), "pe-hose", 0.75, "condensate-trap", role="detail")
        part("drop", "drip", "ell", {"c": [0.0, 0.006, 0.0], "r": 0.004}, "condensate-water", 0.6, None, role="detail", texture=True, views=(), thermal=False)
    elif mid:
        for k, (zc, rs, rc) in enumerate(stubs):
            part(f"connection-{k}", "boiler", "cyl", wcyl("boiler", "y", -0.03, 0, zc, 0.015, yb - 0.065, yb), "brass", 0.5, "connections")
        part("trap", "boiler", "cyl", wcyl("boiler", "y", 0.07, 0, 0.0, 0.02, yb - 0.07, yb), "polypropylene", 0.88, "condensate-trap")
    else:
        part("connections", "boiler", "box", wbox("boiler", -0.045, -0.015, yb - 0.065, yb, -0.145, 0.145), "brass", 0.5, "connections")
        part("trap", "boiler", "cyl", wcyl("boiler", "y", 0.07, 0, 0.0, 0.02, yb - 0.07, yb), "polypropylene", 0.88, "condensate-trap")
    if not coarse:
        for k, zc in enumerate((-0.15, 0.15)):
            part(f"spacer-{k}", "boiler", "box", wbox("boiler", -0.191, -0.175, yb + 0.02, yb + 0.05, zc - 0.02, zc + 0.02), "epdm", 0.15, "casing", role="detail")
        part("rail", "bracket", "box", wbox("bracket", -0.191, -0.175, 0.859, 0.904, -0.19, 0.19), "galv", 0.55, "bracket")
    else:
        part("rail", "bracket", "box", wbox("bracket", -0.205, -0.175, 0.859, 0.904, -0.19, 0.19), "galv", 0.55, "bracket")
    hb = {50: 0.025, 100: 0.025, 200: 0.03}[scale]
    sheet = True
    for k, zc in enumerate(STRAP_Z):
        w0, w1 = zc - STRAP_W / 2, zc + STRAP_W / 2
        part(f"strap-{k}-back-foot", "strap-back-foot", "box", lbox(0, S1, 0, T, w0, w1), "polyester-webbing", 0.35, "straps", sheet=sheet)
        part(f"strap-{k}-back-leg", "strap-back-leg", "box", lbox(0, LB - S1, 0, T, w0, w1), "polyester-webbing", 0.35, "straps", sheet=sheet)
        part(f"strap-{k}-back-top", "strap-back-top", "box", lbox(0, TOPB, 0, T, w0, w1), "polyester-webbing", 0.35, "straps", sheet=sheet)
        part(f"strap-{k}-front-foot", "strap-front-foot", "box", lbox(0, S1, -T, 0, w0, w1), "polyester-webbing", 0.35, "straps", sheet=sheet)
        part(f"strap-{k}-front-leg", "strap-front-leg", "box", lbox(0, LF - S1, -T, 0, w0, w1), "polyester-webbing", 0.35, "straps", sheet=sheet)
        part(f"strap-{k}-front-top", "strap-front-top", "box", lbox(0, TOPF, -T, 0, w0, w1), "polyester-webbing", 0.35, "straps", sheet=sheet)
        if not coarse:
            part(f"ratchet-{k}", "strap-front-top", "box", lbox(0.03, 0.15, -T - hb, -T, zc - 0.03, zc + 0.03), "steel", 0.3, "straps", role="detail")
    for i, p in enumerate(PARTS):
        p["z"] = i + 1
        if coarse and p["material"] == "polyester-webbing":
            p["pen"] = 0.2
    return PARTS


def lift_offsets():
    """🎚 Bone offsets of the released straps lying on the ground and of the boiler for the lifted and reinstalled poses."""
    released = {
        "strap-back-foot": {"angleDeg": r(270.0 - AB - 360.0 + 360.0, 4)},
        "strap-back-leg": {"angleDeg": -90.0},
        "strap-back-top": {"angleDeg": r(AB, 4)},
        "strap-front-foot": {"angleDeg": r(-90.0 - AF, 4)},
        "strap-front-leg": {"angleDeg": 90.0},
        "strap-front-top": {"angleDeg": r(AF - 180.0, 4)},
    }
    opened = {
        "strap-back-foot": {"angleDeg": r(180.0 - AB, 4)},
        "strap-back-top": {"angleDeg": r(AB, 4)},
        "strap-front-foot": {"angleDeg": r(-AF, 4)},
        "strap-front-top": {"angleDeg": r(AF - 180.0, 4)},
    }
    return released, opened


def lowest_lifted():
    """📏 Lowest world height of the casing corners and connections after the lift (for choosing LIFT_DY)."""
    pts = [(-0.175, YB), (0.191, YB), (-0.03, YB - 0.065), (0.07, YB - 0.07), (0.175, YB)]
    c, s = math.cos(math.radians(TILT)), math.sin(math.radians(TILT))
    out = []
    for x, y in pts:
        dx, dy = x - PX, y - PY
        out.append(PY + LIFT_DY + s * dx + c * dy)
    return min(out)




MATERIALS = [
    ("painted-steel-aged", "Yellowed white powder-coated steel sheet", "Vergilbtes weiß pulverbeschichtetes Stahlblech", 0.9, 0.38, 7850, 50, 460, None, None, "authored-estimate", "incropera-2018"),
    ("polycarbonate", "Grey polycarbonate fascia and knobs", "Graue Polycarbonat-Blende und Knöpfe", 0.9, 0.5, 1200, 0.2, 1200, None, None, "literature", "iso-10456-2007"),
    ("display-glass", "Display window", "Displayfenster", 0.9, 0.1, 2500, 1.0, 840, None, None, "literature", "iso-10456-2007"),
    ("paper-label", "Faded paper service label and type plate", "Verblichener Papier-Wartungsaufkleber und Typenschild", 0.9, 0.3, 800, 0.1, 1300, None, None, "authored-estimate", "incropera-2018"),
    ("flue-aluminium", "White painted aluminium air pipe of the concentric adapter", "Weiß lackiertes Aluminium-Luftrohr des konzentrischen Adapters", 0.85, 0.35, 2700, 160, 900, None, None, "literature", "incropera-2018"),
    ("polypropylene", "Polypropylene flue pipe, condensate trap and test plugs", "Polypropylen-Abgasrohr, Kondensatsiphon und Messstopfen", 0.92, 0.4, 905, 0.22, 1900, None, None, "literature", "iso-10456-2007"),
    ("brass", "Tarnished brass connection nuts and caps", "Angelaufene Messing-Anschlussmuttern und Kappen", 0.6, 0.6, 8500, 110, 380, None, None, "literature", "incropera-2018"),
    ("copper", "Oxidised copper pipe stubs, cut", "Oxidierte, abgeschnittene Kupferrohrstutzen", 0.65, 0.65, 8930, 390, 385, None, None, "literature", "incropera-2018"),
    ("pe-hose", "Corrugated PE condensate hose, cut", "Abgeschnittener PE-Wellschlauch des Kondensatablaufs", 0.92, 0.5, 950, 0.4, 2000, None, None, "literature", "iso-10456-2007"),
    ("condensate-water", "Acidic condensate water", "Saures Kondensatwasser", 0.96, 0.1, 1000, 0.6, 4180, None, None, "literature", "incropera-2018"),
    ("epdm", "Black EPDM wall spacers", "Schwarze EPDM-Wandabstandshalter", 0.92, 0.9, 1150, 0.25, 2000, None, None, "literature", "iso-10456-2007"),
    ("galv", "Dull galvanised steel hanging rail", "Matt verzinkte Aufhängeschiene aus Stahl", 0.28, 0.6, 7850, 50, 460, 210e9, 235e6, "literature", "incropera-2018"),
    ("steel", "Zinc-plated steel strap hooks and ratchets", "Verzinkte Stahl-Zurrhaken und Ratschen", 0.3, 0.6, 7850, 50, 460, 210e9, 355e6, "literature", "incropera-2018"),
    ("stainless-steel", "Stainless steel heat exchanger", "Edelstahl-Wärmeübertrager", 0.4, 0.5, 7900, 15, 500, 193e9, 205e6, "literature", "incropera-2018"),
    ("burner-mesh", "FeCrAl knitted premix burner surface", "FeCrAl-Gestrick der Vormischbrennerfläche", 0.6, 0.5, 7200, 15, 500, None, None, "authored-estimate", "incropera-2018"),
    ("pallet-pine", "Weathered grey pine and spruce of a used Euro pallet", "Vergrautes Kiefern- und Fichtenholz einer gebrauchten Europalette", 0.9, 0.65, 550, 0.13, 1600, 11e9, 24e6, "literature", "en-13698-1"),
    ("spruce", "Sawn spruce squared bearer", "Gesägtes Fichten-Kantholz", 0.9, 0.55, 450, 0.12, 1600, 11e9, 24e6, "literature", "en-338-2016"),
    ("polyester-webbing", "Faded orange polyester lashing webbing", "Ausgeblichenes orangefarbenes Polyester-Zurrgurtband", 0.88, 0.6, 1380, 0.15, 1200, 8e9, 400e6, "literature", "en-12195-2"),
]

REFERENCES = [
    ("en-15502-1", "EN 15502-1:2021 Gas-fired heating boilers - Part 1: General requirements and tests (heat input, casing temperature, flue gas, condensate of condensing boilers)."),
    ("en-15502-2-1", "EN 15502-2-1:2022 Gas-fired central heating boilers - Part 2-1: Specific standard for type C appliances and type B2, B3 and B5 appliances of a nominal heat input not exceeding 1000 kW (room-sealed wall-hung condensing boilers with concentric flue)."),
    ("en-13203-1", "EN 13203-1:2015 Gas-fired domestic appliances producing hot water - Part 1: Assessment of performance of hot water deliveries (combination boilers, DHW heat input above the space-heating input)."),
    ("combi-data-sheets", "Manufacturer data sheets of 24 to 26 kW wall-hung gas condensing combination boilers of the 2005-2015 generation, e.g. Vaillant ecoTEC plus VUW (720 x 440 x 338 mm), Viessmann Vitodens 100-W (700 x 400 x 350 mm), Buderus Logamax plus GB172-24K (840 x 440 x 350 mm): casings 700 to 840 mm high, 400 to 440 mm wide, 338 to 350 mm deep, dry weights 33 to 44 kg, heat input 19 to 20 kW for heating and 24 to 26 kW for hot water, electrical input 70 to 110 W, concentric flue 60/100 mm on top, five connections and the condensate trap below (read 2026-11; the asset is the assigned 440 x 700 x 350 mm of this class, not one model)."),
    ("en-14471", "EN 14471:2013+A1:2015 Chimneys - System chimneys with plastic flue liners (polypropylene flue pipe of 60 mm inside the 100 mm aluminium air pipe of a concentric boiler adapter)."),
    ("en-13698-1", "EN 13698-1:2003 Pallet production specification - Part 1: Construction specification for 800 mm x 1200 mm flat wooden pallets (Euro pallet: 1200 x 800 x 144 mm, 22 mm deck and bottom boards, 78 mm blocks, nine blocks, three stringer boards, about 25 kg at 22 % moisture)."),
    ("en-338-2016", "EN 338:2016 Structural timber - Strength classes (C24: E0,mean 11 GPa, f_m,k 24 MPa, mean density 420 kg/m3 at 12 % moisture)."),
    ("en-12195-2", "EN 12195-2:2000 Load restraining on road vehicles - Safety - Part 2: Web lashing made from man-made fibres (50 mm polyester ratchet straps, LC 2500 daN, J-hooks)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity and heat capacity of timber, polymers, rubber)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity and absorptivity of painted, oxidised, galvanised and polished metals, properties of metals)."),
    ("turns-2012", "Turns, S. R. (2012). An Introduction to Combustion: Concepts and Applications, 3rd ed., McGraw-Hill (natural-gas flue gas volume and water vapour at excess air 1.2)."),
]


def material(m):
    """🧱 A material entry."""
    id_, en, de, eps, alpha, rho, k, cp, e, f, src, ref = m
    out = {"id": id_, "title": tx(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp}
    if e:
        out["elasticModulusPa"] = e
        out["strengthPa"] = f
    out["source"] = src
    out["reference"] = ref
    return out


def area_of(p):
    """📐 Total surface of one part (m²)."""
    g = p["geo"]
    if p["kind"] == "box":
        a, b, c = g["s"]
        return 2 * (a * b + b * c + c * a)
    if p["kind"] == "cyl":
        L = g["a1"] - g["a0"]
        if "ri" in g:
            return 2 * math.pi * (g["r"] + g["ri"]) * L + 2 * math.pi * (g["r"] ** 2 - g["ri"] ** 2)
        return 2 * math.pi * g["r"] * L + 2 * math.pi * g["r"] ** 2
    if p["kind"] == "rev":
        pr = g["prof"]
        s = math.pi * pr[0][1] ** 2 + math.pi * pr[-1][1] ** 2
        for (a0, r0), (a1, r1) in zip(pr, pr[1:]):
            if a0 == a1:
                s += math.pi * abs(r0 ** 2 - r1 ** 2)
            else:
                s += 2 * math.pi * r0 * (a1 - a0)
        return s
    if p["kind"] == "ell":
        return 4 * math.pi * g["r"] ** 2
    return 0.0


def contact_area(p, q):
    """🤝 Face contact between two parts of the same region (axis-parallel boxes and coaxial cylinder stacks)."""
    if p["bone"] != q["bone"]:
        return 0.0
    A, B = box_world_rect(p), box_world_rect(q)
    if p["kind"] == "box" and q["kind"] == "box" and abs(frame(p["bone"])[1]) < 1e-9:
        tot = 0.0
        for ax in range(3):
            lo, hi = 2 * ax, 2 * ax + 1
            if abs(A[hi] - B[lo]) < 1e-9 or abs(B[hi] - A[lo]) < 1e-9:
                o = 1.0
                for bx in range(3):
                    if bx == ax:
                        continue
                    ov = min(A[2 * bx + 1], B[2 * bx + 1]) - max(A[2 * bx], B[2 * bx])
                    o *= max(0.0, ov)
                tot += o
        return tot
    if p["kind"] == "cyl" and q["kind"] == "cyl" and p["geo"]["axis"] == q["geo"]["axis"] == "y":
        if abs(p["geo"]["c"][0] - q["geo"]["c"][0]) < 1e-9 and abs(p["geo"]["c"][2] - q["geo"]["c"][2]) < 1e-9:
            if abs(p["geo"]["a1"] - q["geo"]["a0"]) < 1e-9 or abs(q["geo"]["a1"] - p["geo"]["a0"]) < 1e-9:
                rp, rq = p["geo"]["r"], q["geo"]["r"]
                ip, iq = p["geo"].get("ri", 0.0), q["geo"].get("ri", 0.0)
                return math.pi * (min(rp, rq) ** 2 - max(ip, iq) ** 2)
    return 0.0


def region_areas(parts):
    """🌡 Outer area per region of a part list: sum of part surfaces minus both faces of every contact of the same region."""
    out = {}
    th = [p for p in parts if p["thermal"] and p["region"]]
    for p in th:
        out[p["region"]] = out.get(p["region"], 0.0) + area_of(p)
    for i, p in enumerate(th):
        for q in th[i + 1:]:
            if p["region"] == q["region"]:
                out[p["region"]] -= 2 * contact_area(p, q)
    return out


HIDDEN = {
    "burner": ("burner-mesh", "x", 0.04, 0.06, 0.10, YB + 0.47),
    "heat-exchanger": ("stainless-steel", "x", 0.115, -0.11, 0.06, YB + 0.47),
}


def hidden_solids(scale):
    """🫥 The hidden thermal parts inside the casing: the stainless heat-exchanger coil pack and the premix burner in front of it."""
    out = []
    for reg, (mat, ax, rad, a0, a1, yc) in HIDDEN.items():
        g = wcyl("boiler", ax, 0, yc, 0.0, rad, a0, a1)
        cu, cv, cw = g["c"]
        out.append({"id": f"th-{reg}", "bone": "boiler", "material": mat, "role": "body", "tone": 0.5, "kind": "revolve", "profile": [[r(g["a0"]), rad], [r(g["a1"]), rad]], "axis": [1, 0, 0], "origin": [0, r(cv), 0], "region": reg, "layer": {"burner": "mesh", "heat-exchanger": "wall"}[reg]})
    return out


def hidden_areas():
    """🫥 Full mesh areas of the hidden parts."""
    return {reg: 2 * math.pi * rad * (a1 - a0) + 2 * math.pi * rad * rad for reg, (_, _, rad, a0, a1, _) in HIDDEN.items()}


LAYER = {
    "casing": ("sheet", "Powder-coated steel sheet 0.8 mm", "Pulverbeschichtetes Stahlblech 0,8 mm", "metal", "painted-steel-aged", 0.0008, 1.6e-05, 3700),
    "controls": ("panel", "Polycarbonate fascia 2.5 mm", "Polycarbonat-Blende 2,5 mm", "polymer", "polycarbonate", 0.0025, 0.0125, 3000),
    "flue-adapter": ("pipe", "Aluminium air pipe 1 mm", "Aluminium-Luftrohr 1 mm", "metal", "flue-aluminium", 0.001, 6e-06, 2400),
    "connections": ("tube", "Copper and brass wall 1.5 mm", "Kupfer- und Messingwand 1,5 mm", "metal", "brass", 0.0015, 1.4e-05, 4800),
    "condensate-trap": ("body", "Polypropylene wall 2.5 mm", "Polypropylenwand 2,5 mm", "polymer", "polypropylene", 0.0025, 0.0114, 4300),
    "bracket": ("rail", "Galvanised steel 2 mm", "Verzinkter Stahl 2 mm", "metal", "galv", 0.002, 4e-05, 7200),
    "pallet": ("boards", "Pallet timber 22 mm", "Palettenholz 22 mm", "timber", "pallet-pine", 0.022, 0.169, 19360),
    "bearers": ("timber", "Spruce bearer 40 mm", "Fichtenholz 40 mm", "timber", "spruce", 0.04, 0.333, 28800),
    "straps": ("webbing", "Polyester webbing 2 mm", "Polyester-Gurtband 2 mm", "polymer", "polyester-webbing", 0.002, 0.0133, 3300),
    "burner": ("mesh", "FeCrAl knit 1 mm", "FeCrAl-Gestrick 1 mm", "metal", "burner-mesh", 0.001, 7e-05, 3600),
    "heat-exchanger": ("wall", "Stainless steel wall 1 mm", "Edelstahlwand 1 mm", "metal", "stainless-steel", 0.001, 6.7e-05, 3950),
}


def regions_doc(areas):
    """🔥 The thermal regions with their materials, layers, operating heat and nameplates."""
    spec = [
        ("casing", "Casing hood and wall spacers", "Gehäusehaube und Wandabstandshalter", "surface", "operating", "vertical-plate", 0.70, 0.4, {"key": "burner", "heatW": 60}, None, None),
        ("controls", "Fascia with display and knobs", "Blende mit Display und Knöpfen", "surface", "operating", "vertical-plate", 0.085, 0.4, {"key": "controls", "heatW": 4}, 85, "electrical-input"),
        ("flue-adapter", "Concentric flue adapter 60/100", "Konzentrischer Abgasadapter 60/100", "surface", "operating", "cylinder", 0.1, 0.6, {"key": "burner", "heatW": 12}, None, None),
        ("connections", "Capped connection stubs", "Verschlossene Anschlussstutzen", "surface", "operating", "cylinder", 0.03, 0.3, {"key": "burner", "heatW": 15}, None, None),
        ("condensate-trap", "Condensate trap and cut drain hose", "Kondensatsiphon und abgeschnittener Ablaufschlauch", "water", "operating", "cylinder", 0.04, 0.3, {"key": "burner", "heatW": 3}, None, None),
        ("bracket", "Wall-hanging rail", "Aufhängeschiene", "surface", "passive", "vertical-plate", 0.045, 0.3, None, None, None),
        ("pallet", "Euro pallet", "Europalette", "surface", "passive", "horizontal-plate-up", 0.8, 0.8, None, None, None),
        ("bearers", "Squared timber bearers", "Kanthölzer", "surface", "passive", "horizontal-plate-up", 0.08, 0.5, None, None, None),
        ("straps", "Ratchet straps with hooks", "Ratschengurte mit Haken", "surface", "passive", "vertical-plate", 0.05, 0.8, None, None, None),
        ("burner", "Premix burner", "Vormischbrenner", "heater", "operating", "cylinder", 0.08, 0.0, {"key": "burner", "heatW": 3}, 24000, "fuel-input"),
        ("heat-exchanger", "Stainless heat exchanger coil pack", "Edelstahl-Wärmeübertragerpaket", "surface", "passive", "cylinder", 0.23, 0.0, None, None, None),
    ]
    out = []
    for id_, en, de, kind, model, geom, lc, svf, op, rated, basis in spec:
        lid, len_, lde, lkind, lmat, th, res, hc = LAYER[id_]
        reg = {"id": id_, "title": tx(en, de), "kind": kind, "model": model, "material": lmat, "areaM2": r(areas[id_], 4), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf,
               "layers": [{"id": lid, "title": tx(len_, lde), "kind": lkind, "material": lmat, "thicknessM": th, "resistanceM2KW": res, "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "iso-10456-2007"}],
               "heatCapacityJM2K": hc}
        if op:
            reg["operating"] = op
        if rated is not None:
            reg["ratedPowerW"] = rated
            reg["ratedPowerBasis"] = basis
        reg["source"] = "authored-estimate"
        reg["reference"] = {"burner": "en-15502-1", "controls": "combi-data-sheets"}.get(id_, "incropera-2018")
        if id_ == "burner":
            reg["hidden"] = tx("The cylindrical premix burner of 80 mm sits inside the closed casing in front of the coil pack; it is not drawn.", "Der zylindrische Vormischbrenner von 80 mm sitzt im geschlossenen Gehäuse vor dem Wärmeübertragerpaket; er wird nicht gezeichnet.")
        if id_ == "heat-exchanger":
            reg["hidden"] = tx("The stainless coil pack of 230 mm diameter and 170 mm depth fills the upper half of the closed casing; it is not drawn.", "Das Edelstahl-Wendelpaket von 230 mm Durchmesser und 170 mm Tiefe füllt die obere Hälfte des geschlossenen Gehäuses; es wird nicht gezeichnet.")
        out.append(reg)
    return out


FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}


def cube(id_, bone_, u, v, w, f, mat, extra):
    """🧊 A feature-sized cube marker solid for joints and supports."""
    s = {"id": id_, "bone": bone_, "material": mat, "role": "body", "tone": 0.4, "kind": "box", "size": [f, f, f], "transform": {"translation": [r(u), r(v), r(w)]}}
    s.update(extra)
    return s


CONTACTS = [("pallet-contact-back-right", 0.0725, -0.35), ("pallet-contact-back-left", 0.0725, 0.35), ("pallet-contact-front-right", 1.1275, -0.35), ("pallet-contact-front-left", 1.1275, 0.35)]


def masses():
    """⚖️ Bone masses from section times density (timber, steel, webbing) and the catalogue dry weight of the boiler."""
    build_parts(50)
    pal = [p for p in PARTS if p["bone"] == "pallet"]
    m = 0.0
    mx = my = 0.0
    for p in pal:
        x0, x1, y0, y1, z0, z1 = box_world_rect(p)
        vol = (x1 - x0) * (y1 - y0) * (z1 - z0)
        mm = vol * DENS[{"pallet-pine": "pallet-pine", "spruce": "spruce", "steel": "steel"}[p["material"]]]
        if p["material"] == "steel":
            mm *= 0.5
        m += mm
        mx += mm * ((x0 + x1) / 2 + 0.6)
        my += mm * (y0 + y1) / 2
    web = 0.06 * 2
    return {
        "pallet": (m, mx / m, my / m, "point"),
        "boiler": (36.0, 0.19, -0.33, "point"),
        "bracket": (0.55, -0.0, -0.0225, "point"),
        "strap-back-foot": (web * S1, S1 / 2, 0.0, "uniform"),
        "strap-back-leg": (web * (LB - S1), (LB - S1) / 2, 0.0, "uniform"),
        "strap-back-top": (web * TOPB, TOPB / 2, 0.0, "uniform"),
        "strap-front-foot": (web * S1, S1 / 2, 0.0, "uniform"),
        "strap-front-leg": (web * (LF - S1), (LF - S1) / 2, 0.0, "uniform"),
        "strap-front-top": (web * TOPF + 2 * 0.6, 0.09, -0.015, "point"),
    }


def thermal_list(scale, parts):
    """🌡 The thermal list of a scale: references to the body solids of every region, the hidden parts and the flue-gas outlet."""
    out = []
    for p in parts:
        if p["thermal"] and p["region"]:
            out.append({"id": f"th-{p['id']}", "of": p["id"], "region": p["region"], "layer": LAYER[p["region"]][0]})
    if scale == 200:
        g = wbox("boiler", 0.175, 0.205, YB + 0.02, YB + 0.105, -0.2, 0.2)
        out.append({"id": "th-controls-patch", "bone": "boiler", "material": "polycarbonate", "role": "body", "tone": 0.42, "kind": "box", "size": [r(v) for v in g["s"]], "transform": {"translation": [r(v) for v in g["c"]]}, "region": "controls", "layer": "panel"})
    out.extend(hidden_solids(scale))
    top = YB + (0.805 if scale == 50 else 0.805)
    h = {50: 0.008, 100: 0.015, 200: 0.03}[scale]
    rr = {50: 0.028, 100: 0.028, 200: 0.03}[scale]
    gy0 = to_local("boiler", 0, top - h)[1]
    gy1 = to_local("boiler", 0, top)[1]
    ou = to_local("boiler", 0, 0)[0]
    out.append({"id": "th-flue-gas", "bone": "boiler", "material": "polypropylene", "role": "detail", "tone": 0.5, "kind": "revolve", "profile": [[r(gy0), rr], [r(gy1), rr]], "axis": [0, 1, 0], "origin": [r(ou), 0, 0], "region": "flue-adapter", "tags": ["flue-gas"]})
    return out


def structure_list(scale, parts):
    """🏗 The structural list of a scale: timber, rail and webbing members, the chassis sheets, joints, supports and masses."""
    f = FEATURE[scale]
    out = []
    for p in parts:
        if p["bone"] == "pallet" and p["material"] in ("pallet-pine", "spruce", "steel"):
            out.append({"id": f"member-{p['id']}", "of": p["id"], "member": "pallet", "part": "member"})
        if p["bone"].startswith("strap-") and p["material"] in ("polyester-webbing", "steel"):
            out.append({"id": f"member-{p['id']}", "of": p["id"], "member": p["bone"], "part": "member"})
        if p["bone"] == "bracket":
            out.append({"id": f"member-{p['id']}", "of": p["id"], "member": "bracket", "part": "member"})
    tsheet = 0.001
    g = wbox("boiler", -0.175, -0.175 + tsheet, YB, YB + 0.70, -0.21, 0.21)
    out.append({"id": "member-chassis-back", "bone": "boiler", "material": "galv", "role": "body", "tone": 0.7, "kind": "box", "size": [r(v) for v in g["s"]], "transform": {"translation": [r(v) for v in g["c"]]}, "sheet": True, "member": "boiler", "part": "member"})
    g = wbox("boiler", -0.175, 0.165, YB, YB + tsheet, -0.21, 0.21)
    out.append({"id": "member-chassis-bottom", "bone": "boiler", "material": "galv", "role": "body", "tone": 0.7, "kind": "box", "size": [r(v) for v in g["s"]], "transform": {"translation": [r(v) for v in g["c"]]}, "sheet": True, "member": "boiler", "part": "member"})
    out.append(cube("joint-hanging-point", "boiler", 0.0, -f / 2, 0.0, f, "galv", {"member": "boiler", "part": "joint"}))
    for side in ("back", "front"):
        for k, zc in enumerate(STRAP_Z):
            out.append(cube(f"joint-{side}-hook-{k}", f"strap-{side}-foot", 0.0, 0.0, zc, f, "steel", {"member": f"strap-{side}-foot", "part": "joint"}))
            out.append(cube(f"joint-{side}-leg-{k}", f"strap-{side}-leg", 0.0, 0.0, zc, f, "polyester-webbing", {"member": f"strap-{side}-leg", "part": "joint"}))
            out.append(cube(f"joint-{side}-corner-{k}", f"strap-{side}-top", 0.0, 0.0, zc, f, "polyester-webbing", {"member": f"strap-{side}-top", "part": "joint"}))
    for aid, u, w in CONTACTS:
        out.append({"id": f"support-{aid}", "bone": "pallet", "material": "pallet-pine", "role": "body", "tone": 0.3, "kind": "box", "size": [0.1, f, 0.1], "transform": {"translation": [r(u), r(f / 2), r(w)]}, "member": "pallet", "part": "support", "anchor": aid})
    ms = masses()
    build_parts(scale)
    for b, (kg, cx, cy, dist) in ms.items():
        if kg < 0.5:
            continue
        out.append({"id": f"mass-{b}", "bone": b, "material": "steel", "role": "detail", "tone": 0.5, "kind": "ellipsoid", "radii": [f / 2] * 3, "transform": {"translation": [r(cx), r(cy), 0.0]}, "member": b, "part": "mass"})
    return out


def ring_path(cx, cy, ro, ri):
    """⭕ An annulus as an even-odd path."""
    def circ(rr):
        return f"M {r(cx + rr)} {r(cy)} A {rr} {rr} 0 1 0 {r(cx - rr)} {r(cy)} A {rr} {rr} 0 1 0 {r(cx + rr)} {r(cy)} Z"
    return {"kind": "path", "d": circ(ro) + " " + circ(ri)}


def strokes(scale):
    """〰 Texture and detail strokes of a scale: casing seams, the hood joint, the pallet stamp and the strap stitching."""
    out = []
    if scale == 50:
        ub0, vb0 = to_local("boiler", -0.15, YB + 0.005)
        ub1, vb1 = to_local("boiler", -0.15, YB + 0.695)
        out.append(("elevation", {"id": "hood-joint", "bone": "boiler", "role": "texture", "z": 900, "filled": False, "tone": 0.35, "region": "casing", "kind": "poly", "points": [[r(ub0), r(vb0)], [r(ub1), r(vb1)]], "closed": False}))
        out.append(("front", {"id": "fascia-seam", "bone": "boiler", "role": "texture", "z": 901, "filled": False, "tone": 0.35, "region": "casing", "kind": "poly", "points": [[-0.215, r(YB + 0.115)], [0.215, r(YB + 0.115)]], "closed": False}))
        out.append(("front", {"id": "limescale-streak", "bone": "boiler", "role": "texture", "z": 902, "filled": False, "tone": 0.55, "region": "casing", "kind": "poly", "points": [[0.02, r(YB + 0.005)], [0.03, r(YB + 0.04)], [0.025, r(YB + 0.075)]], "closed": False}))
        out.append(("front", {"id": "pallet-stamp", "bone": "pallet", "role": "texture", "z": 903, "filled": False, "tone": 0.3, "region": "pallet", "kind": "poly", "points": [[-0.04, 0.04], [0.04, 0.04], [0.04, 0.085], [-0.04, 0.085]], "closed": True}))
        for k, zc in enumerate(STRAP_Z):
            out.append(("front", {"id": f"stitching-{k}", "bone": "strap-front-leg", "role": "texture", "z": 904 + k, "filled": False, "tone": 0.2, "region": "straps", "kind": "poly", "points": [[r(zc), 0.25], [r(zc), 0.85]], "closed": False}))
    if scale == 100:
        out.append(("front", {"id": "fascia-seam", "bone": "boiler", "role": "texture", "z": 901, "filled": False, "tone": 0.35, "region": "casing", "kind": "poly", "points": [[-0.215, r(YB + 0.115)], [0.215, r(YB + 0.115)]], "closed": False}))
    return out


PENS = {
    50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
    100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
    200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": 0.1, "ground": 0.25},
}
RANGE = {50: (20, 75), 100: (75, 150), 200: (150, 350)}


def representation(scale, parts):
    """🗺 The authored 2D representation of a scale, every filled shape the twin of one solid."""
    geo = {"elevation": [], "front": [], "plan": []}
    for p in parts:
        for view, s in shapes_of(p, p["z"]).items():
            if view == "plan" and p["id"].startswith(("nut-", "stub-", "cap-", "trap", "drain-hose", "connection")):
                continue
            if view == "plan" and p["kind"] == "cyl" and p["geo"]["axis"] == "y" and "ri" in p["geo"]:
                x0, x1, _, _, z0, z1 = box_world_rect(p)
                keep = {k: v for k, v in s.items() if k not in ("kind", "rx", "ry", "cx", "cy")}
                keep.update(ring_path((x0 + x1) / 2, (z0 + z1) / 2, p["geo"]["r"], p["geo"]["ri"]))
                s = keep
            geo[view].append(s)
    for view, s in strokes(scale):
        geo[view].append(s)
    lo, hi = RANGE[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": geo, "decisions": REP_DECISIONS[scale]}


REP_DECISIONS = {
    50: {
        "silhouette": tx("Side elevation of the boiler side (350 mm deep, 700 mm high) standing on two 40 x 80 mm bearers on the 1200 x 800 x 144 mm Euro pallet, the two 50 mm ratchet straps running from the hooks at both deck ends over the casing top, the concentric 60/100 adapter on top and the five capped connections and the trap below; front elevation of the 440 mm wide front with fascia, display and knobs; plan of the deck with its five boards, the strap legs and the adapter rings.", "Seitenansicht der Kesselseite (350 mm tief, 700 mm hoch) auf zwei Kanthölzern 40 x 80 mm auf der Europalette 1200 x 800 x 144 mm, die zwei 50-mm-Ratschengurte von den Haken an beiden Deckenden über die Gehäusedecke, oben der konzentrische Adapter 60/100, unten die fünf verschlossenen Anschlüsse und der Siphon; Vorderansicht der 440 mm breiten Front mit Blende, Display und Knöpfen; Draufsicht auf das Deck mit seinen fünf Brettern, die Gurtschenkel und die Adapterringe."),
        "retained": tx("Every board, block and stringer of the pallet with its 41 mm deck gaps, both bearers, the 8 mm rounded casing edges, the 16 mm fascia, two knobs of 24 mm, the display, the type plate and the service label, the adapter collar, air pipe, flue pipe and two test plugs, five nuts, stubs and caps, the trap and the cut hose, the two EPDM spacers, the hanging rail, both straps leg by leg with their hooks and ratchets.", "Jedes Brett, jeder Klotz und jedes Querbrett der Palette mit den 41-mm-Deckfugen, beide Kanthölzer, die mit 8 mm gerundeten Gehäusekanten, die 16-mm-Blende, zwei Knöpfe von 24 mm, Display, Typenschild und Wartungsaufkleber, Adapterkragen, Luftrohr, Abgasrohr und zwei Messstopfen, fünf Muttern, Stutzen und Kappen, Siphon und abgeschnittener Schlauch, die zwei EPDM-Abstandshalter, die Aufhängeschiene, beide Gurte Schenkel für Schenkel mit Haken und Ratschen."),
        "merged": tx("The two straps lie in one plane each and share their four folding bones; in the side elevation the near and the far strap coincide.", "Die zwei Gurte liegen je in einer Ebene und teilen sich ihre vier Faltknochen; in der Seitenansicht fallen der nahe und der ferne Gurt zusammen."),
        "omitted": tx("Pallet nails, the screw holes of the rail and the internal hydraulics; the hidden burner and heat exchanger are thermal parts only.", "Palettennägel, die Schraublöcher der Schiene und die innere Hydraulik; verborgener Brenner und Wärmeübertrager sind nur Thermikteile."),
        "thinElements": tx("The 2 mm webbing is drawn as a filled band of its true thickness; the hood joint, the fascia seam, a limescale streak, the pallet stamp and the strap stitching are texture strokes.", "Das 2-mm-Gurtband ist als gefülltes Band in wahrer Dicke gezeichnet; Haubenfuge, Blendenfuge, eine Kalkspur, der Palettenstempel und die Gurtnaht sind Texturstriche."),
        "hatching": tx("No hatches: timber, steel and webbing read by their tones.", "Keine Schraffuren: Holz, Stahl und Gurtband lesen sich über ihre Tonwerte."),
        "styles": tx("Body pen 0.35 for every part, detail 0.18 for fascia, knobs, caps, hooks and ratchets, texture 0.1, glazing 0.13 for the display.", "Körperstift 0,35 für jedes Teil, Detail 0,18 für Blende, Knöpfe, Kappen, Haken und Ratschen, Textur 0,1, Verglasung 0,13 für das Display."),
        "overlays": tx("Thermal view: casing, adapter, connections and trap warm when reinstalled and firing; the pallet, bearers and straps stay at room temperature.", "Thermikansicht: Gehäuse, Adapter, Anschlüsse und Siphon erwärmen sich im wieder eingebauten Brennbetrieb; Palette, Kanthölzer und Gurte bleiben auf Raumtemperatur."),
    },
    100: {
        "silhouette": tx("The same pallet, bearers, boiler and straps at 1:100: pallet boards and blocks individually (gaps 41 mm and more), casing with 12 mm rounded edges, fascia as one 16 mm step, adapter collar with air and flue pipe, five merged connection cylinders of 30 mm, trap, spacers, rail, straps and ratchets.", "Dieselbe Palette, Kanthölzer, Kessel und Gurte bei 1:100: Palettenbretter und Klötze einzeln (Fugen ab 41 mm), Gehäuse mit 12 mm gerundeten Kanten, Blende als eine 16-mm-Stufe, Adapterkragen mit Luft- und Abgasrohr, fünf zusammengefasste Anschlusszylinder von 30 mm, Siphon, Abstandshalter, Schiene, Gurte und Ratschen."),
        "retained": tx("Pallet construction, bearers, casing, fascia, adapter outline, connection row, trap, rail, both straps with hooks and ratchets.", "Palettenaufbau, Kanthölzer, Gehäuse, Blende, Adapterumriss, Anschlussreihe, Siphon, Schiene, beide Gurte mit Haken und Ratschen."),
        "merged": tx("Nut, copper stub and cap of each connection are one 30 mm cylinder; the flue pipe of the adapter runs down to the casing through the collar; hook plate and lip are one 30 mm block.", "Mutter, Kupferstutzen und Kappe jedes Anschlusses sind ein Zylinder von 30 mm; das Abgasrohr des Adapters läuft durch den Kragen bis zum Gehäuse; Hakenblech und Lippe sind ein Block von 30 mm."),
        "omitted": tx("Knobs (12 mm proud), test plugs (14 mm), display, labels, the drain hose and the condensate drop.", "Knöpfe (12 mm vorstehend), Messstopfen (14 mm), Display, Aufkleber, Ablaufschlauch und Kondensattropfen."),
        "thinElements": tx("Webbing as a filled 2 mm band; the fascia seam as a texture stroke.", "Gurtband als gefülltes 2-mm-Band; die Blendenfuge als Texturstrich."),
        "hatching": tx("No hatches.", "Keine Schraffuren."),
        "styles": tx("Body pen 0.25, detail 0.13, texture 0.1.", "Körperstift 0,25, Detail 0,13, Textur 0,1."),
        "overlays": tx("Thermal view as at 1:50 with merged connection and adapter regions.", "Thermikansicht wie bei 1:50 mit zusammengefassten Anschluss- und Adapterregionen."),
    },
    200: {
        "silhouette": tx("At 1:200 (6.1 mm long on paper) the pallet is a 30 mm deck plate on nine blocks and three 30 mm bottom boards, the bearers stand under the casing, the boiler is one rounded block with the stepped adapter, a 30 mm connection row and the trap below and a 30 mm rail behind; the straps run as 2 mm bands over the top to 30 mm hooks.", "Bei 1:200 (6,1 mm lang auf Papier) ist die Palette eine 30-mm-Deckplatte auf neun Klötzen und drei 30-mm-Bodenbrettern, die Kanthölzer stehen unter dem Gehäuse, der Kessel ist ein gerundeter Block mit gestuftem Adapter, darunter eine 30-mm-Anschlussreihe und der Siphon und dahinter eine 30-mm-Schiene; die Gurte laufen als 2-mm-Bänder über die Decke zu 30-mm-Haken."),
        "retained": tx("Pallet openings in both directions, bearers, casing, adapter, connection row, trap, rail, both straps.", "Palettenöffnungen in beiden Richtungen, Kanthölzer, Gehäuse, Adapter, Anschlussreihe, Siphon, Schiene, beide Gurte."),
        "merged": tx("Deck boards and stringers into one 30 mm plate (the 41 mm deck gaps would close at 1:350); the five connections into one 30 x 65 x 290 mm row; the fascia into the casing outline, which carries the controls region; the rail is drawn at the 30 mm feature.", "Deckbretter und Querbretter zu einer 30-mm-Platte (die 41-mm-Deckfugen liefen bei 1:350 zu); die fünf Anschlüsse zu einer Reihe 30 x 65 x 290 mm; die Blende in den Gehäuseumriss, der die Regelungsregion trägt; die Schiene ist auf das 30-mm-Merkmal gezeichnet."),
        "omitted": tx("Knobs, fascia step, plugs, spacers, ratchets, labels, seams and the drop.", "Knöpfe, Blendenstufe, Stopfen, Abstandshalter, Ratschen, Aufkleber, Fugen und Tropfen."),
        "thinElements": tx("Webbing as a filled 2 mm band at the body pen.", "Gurtband als gefülltes 2-mm-Band mit dem Körperstift."),
        "hatching": tx("No hatches.", "Keine Schraffuren."),
        "styles": tx("Body pen 0.18, detail 0.13 for the hooks.", "Körperstift 0,18, Detail 0,13 für die Haken."),
        "overlays": tx("Thermal view with the controls region merged into the casing shape.", "Thermikansicht mit der Regelungsregion im Gehäuseumriss."),
    },
}

VOL_DECISIONS = {
    50: [
        tx("Body: the Euro pallet board by board (five 22 mm deck boards with 41 mm gaps, three stringers, nine 78 mm blocks, three bottom boards), two 40 x 80 mm spruce bearers, the 350 x 700 x 440 mm casing with 8 mm rounded edges, the 16 mm fascia with two knobs and the display, the concentric adapter as collar, hollow aluminium air pipe and hollow PP flue pipe with two test plugs, five connections as nut, cut copper stub and cap, the trap with its cut hose and a condensate drop, two EPDM spacers, the 16 x 45 x 380 mm rail and both ratchet straps as 2 mm webbing sheets on their six folding segments with hooks and ratchets.", "Körper: die Europalette Brett für Brett (fünf 22-mm-Deckbretter mit 41-mm-Fugen, drei Querbretter, neun 78-mm-Klötze, drei Bodenbretter), zwei Fichten-Kanthölzer 40 x 80 mm, das Gehäuse 350 x 700 x 440 mm mit 8 mm gerundeten Kanten, die 16-mm-Blende mit zwei Knöpfen und Display, der konzentrische Adapter als Kragen, hohles Aluminium-Luftrohr und hohles PP-Abgasrohr mit zwei Messstopfen, fünf Anschlüsse als Mutter, abgeschnittener Kupferstutzen und Kappe, der Siphon mit abgeschnittenem Schlauch und ein Kondensattropfen, zwei EPDM-Abstandshalter, die Schiene 16 x 45 x 380 mm und beide Ratschengurte als 2-mm-Gurtbandflächen auf ihren sechs Faltsegmenten mit Haken und Ratschen."),
        tx("Thermal: every body part of a region by reference, the hidden heat exchanger (230 x 170 mm) and premix burner (80 x 40 mm) inside the closed casing, the flue-gas outlet patch in the mouth of the flue pipe.", "Thermik: jedes Körperteil einer Region als Verweis, der verborgene Wärmeübertrager (230 x 170 mm) und der Vormischbrenner (80 x 40 mm) im geschlossenen Gehäuse, die Abgas-Austrittsfläche in der Mündung des Abgasrohrs."),
        tx("Structure: pallet timbers, bearers and hooks as members of the pallet, the 1 mm galvanised back and bottom sheets of the chassis as the boiler's load path to the bearers and to the hanging point, the rail, the webbing segments and ratchets as tension members, joints at the hanging point and at every strap hinge, contact supports under the four corner blocks, masses at the centres.", "Struktur: Palettenhölzer, Kanthölzer und Haken als Stäbe der Palette, das 1-mm-Rück- und Bodenblech des Chassis als Lastweg des Kessels zu den Kanthölzern und zum Aufhängepunkt, die Schiene, die Gurtsegmente und Ratschen als Zugglieder, Gelenke am Aufhängepunkt und an jedem Gurtgelenk, Kontaktauflager unter den vier Eckklötzen, Massen in den Schwerpunkten."),
    ],
    100: [
        tx("Body re-modelled for 15 mm: pallet unchanged in construction (all parts at least 22 mm), casing edges rounded 12 mm, fascia without knobs or display, adapter as collar and two hollow pipes without the test plugs, each connection one 30 mm cylinder, the trap with the hose merged, hooks as one 30 mm block, straps and ratchets kept, the drop left out.", "Körper für 15 mm neu modelliert: Palette im Aufbau unverändert (alle Teile mindestens 22 mm), Gehäusekanten mit 12 mm gerundet, Blende ohne Knöpfe und Display, Adapter als Kragen und zwei hohle Rohre ohne Messstopfen, jeder Anschluss ein Zylinder von 30 mm, Siphon mit Schlauch zusammengefasst, Haken als ein Block von 30 mm, Gurte und Ratschen erhalten, Tropfen weggelassen."),
        tx("Thermal: references to the regions' body parts, the hidden parts and a 15 mm flue-gas outlet patch.", "Thermik: Verweise auf die Körperteile der Regionen, die verborgenen Teile und eine 15-mm-Abgas-Austrittsfläche."),
        tx("Structure as at 1:50 with 15 mm joints, supports and mass markers.", "Struktur wie bei 1:50 mit 15-mm-Gelenken, -Auflagern und -Massenmarken."),
    ],
    200: [
        tx("Body re-modelled for 30 mm: the pallet as a 30 mm deck plate on nine 84 mm blocks and three 30 mm bottom boards (openings 84 mm high in both directions), bearers, casing with 20 mm rounded edges, stepped adapter, one connection row block, trap, the rail generalised to the 30 mm feature, straps as 2 mm sheets with 30 mm hooks; fascia, knobs, plugs, spacers and ratchets merged or left out.", "Körper für 30 mm neu modelliert: die Palette als 30-mm-Deckplatte auf neun 84-mm-Klötzen und drei 30-mm-Bodenbrettern (Öffnungen 84 mm hoch in beiden Richtungen), Kanthölzer, Gehäuse mit 20 mm gerundeten Kanten, gestufter Adapter, ein Anschlussreihen-Block, Siphon, die Schiene auf das 30-mm-Merkmal verallgemeinert, Gurte als 2-mm-Flächen mit 30-mm-Haken; Blende, Knöpfe, Stopfen, Abstandshalter und Ratschen zusammengefasst oder weggelassen."),
        tx("Thermal: references, a 30 mm controls patch on the front where the fascia is, hidden parts, a 30 mm flue-gas outlet.", "Thermik: Verweise, eine 30-mm-Regelungsfläche vorn an der Stelle der Blende, verborgene Teile, ein 30-mm-Abgasaustritt."),
        tx("Structure as at 1:50 with 30 mm joints, supports and mass markers.", "Struktur wie bei 1:50 mit 30-mm-Gelenken, -Auflagern und -Massenmarken."),
    ],
}


def anchors_doc():
    """📍 Supports, the emitter and the load point."""
    out = []
    names = {"back-right": tx("Back right corner block on the ground", "Hinterer rechter Eckklotz am Boden"), "back-left": tx("Back left corner block on the ground", "Hinterer linker Eckklotz am Boden"), "front-right": tx("Front right corner block on the ground", "Vorderer rechter Eckklotz am Boden"), "front-left": tx("Front left corner block on the ground", "Vorderer linker Eckklotz am Boden")}
    for aid, u, w in CONTACTS:
        out.append({"id": aid, "bone": "pallet", "x": u, "y": 0.0, "z": w, "kind": "support", "title": names[aid.replace("pallet-contact-", "")], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.145, "footprintWidthM": 0.1, "frictionCoefficient": 0.5})
    out.append({"id": "lifting-point", "bone": "bracket", "x": 0.0, "y": 0.0, "z": 0.0, "kind": "support", "title": tx("Shackle of the crane sling in the centre slot of the rail", "Schäkel des Krangehänges im Mittelschlitz der Schiene"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.03, "footprintWidthM": 0.03})
    out.append({"id": "wall-bracket", "bone": "bracket", "x": -0.008, "y": -0.0225, "z": 0.0, "kind": "support", "title": tx("Rail screwed to the new wall", "An die neue Wand geschraubte Schiene"), "support": "pin", "normalDeg": 0, "footprintLengthM": 0.045, "footprintWidthM": 0.38})
    u, v = to_local("boiler", -0.191, YB + 0.035)
    out.append({"id": "wall-rest", "bone": "boiler", "x": r(u), "y": r(v), "z": 0.0, "kind": "support", "title": tx("EPDM spacers of the lower back against the wall", "EPDM-Abstandshalter der unteren Rückseite an der Wand"), "support": "roller", "normalDeg": 0, "footprintLengthM": 0.03, "footprintWidthM": 0.34, "frictionCoefficient": 0.3})
    u, v = to_local("boiler", 0.0, YB + 0.805)
    out.append({"id": "flue-outlet", "bone": "boiler", "x": r(u), "y": r(v), "z": 0.0, "kind": "emitter", "title": tx("Mouth of the 60 mm flue pipe of the adapter", "Mündung des 60-mm-Abgasrohrs des Adapters")})
    out.append({"id": "water-fill", "bone": "boiler", "x": 0.19, "y": -0.33, "z": 0.0, "kind": "load", "title": tx("Centre of the 2.5 l water content when refilled", "Mitte der 2,5 l Wasserinhalt nach dem Befüllen")})
    return out


REGION_IDS = ["casing", "controls", "flue-adapter", "connections", "condensate-trap", "bracket", "pallet", "bearers", "straps", "burner", "heat-exchanger"]


def binding(burner, controls, flue, ground_contact=True):
    """🌬 The thermal binding of a pose: every region and the flue emitter."""
    regs = {}
    for rid in REGION_IDS:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if rid == "pallet" and ground_contact:
            b = {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.1, "tempC": 18}}
        if rid in ("burner", "heat-exchanger"):
            b = {"exposedFraction": 0, "sunlitFraction": 0}
        regs[rid] = b
    return {"operating": {"burner": burner, "controls": controls}, "regions": regs, "emitters": {"flue-gas": flue}}


PALLET_CONTACTS = [c[0] for c in CONTACTS]


def poses_doc():
    """🧍 Stored, lifted, reinstalled, reinstalled-off and the key poses of the lift."""
    released, opened = lift_offsets()
    lifted = dict(released)
    lifted["boiler-lift"] = {"dy": LIFT_DY}
    lifted["boiler"] = {"angleDeg": TILT}
    clear = dict(released)
    clear["boiler-lift"] = {"dy": 0.06}
    wall = dict(released)
    wall["boiler-lift"] = {"dy": WALL_DY}
    water = {"id": "water-fill", "kind": "carried", "title": tx("Water content of the refilled boiler, 2.5 l", "Wasserinhalt des wieder befüllten Kessels, 2,5 l"), "anchor": "water-fill", "fxN": 0, "fyN": -24.5, "source": "authored-estimate", "reference": "combi-data-sheets"}
    return [
        {"id": "stored", "title": tx("Stored on the pallet", "Auf der Palette gelagert"), "description": tx("Drained and capped, the boiler stands upright on two bearers on a Euro pallet, held by two ratchet straps over its top; the wall rail hangs in its pocket and the adapter stays on.", "Entleert und verschlossen steht der Kessel aufrecht auf zwei Kanthölzern auf einer Europalette, gehalten von zwei Ratschengurten über seine Decke; die Wandschiene hängt in ihrer Tasche, der Adapter bleibt aufgesteckt."), "bones": {}, "ground": "authored", "contacts": PALLET_CONTACTS, "thermal": binding(0, 0, 0), "loads": []},
        {"id": "straps-open", "key": True, "title": tx("Straps swung open", "Gurte aufgeschwenkt"), "description": tx("In-between key of the lift: the ratchets are released and both strap halves swing out over the pallet ends.", "Zwischenschlüssel des Anhebens: die Ratschen sind gelöst und beide Gurthälften schwenken über die Palettenenden hinaus."), "bones": opened, "ground": "authored", "contacts": PALLET_CONTACTS, "thermal": binding(0, 0, 0), "loads": []},
        {"id": "straps-down", "key": True, "title": tx("Straps laid down", "Gurte abgelegt"), "description": tx("In-between key of the lift: the released straps hang from their hooks over the pallet ends and lie on the floor.", "Zwischenschlüssel des Anhebens: die gelösten Gurte hängen an ihren Haken über die Palettenenden und liegen am Boden."), "bones": dict(released), "ground": "authored", "contacts": PALLET_CONTACTS, "thermal": binding(0, 0, 0), "loads": []},
        {"id": "lift-clear", "key": True, "title": tx("Lifted clear of the bearers", "Von den Kanthölzern abgehoben"), "description": tx("In-between key of the lift: the sling takes the weight at the rail and lifts the boiler 60 mm clear of the bearers before it swings.", "Zwischenschlüssel des Anhebens: das Gehänge übernimmt das Gewicht an der Schiene und hebt den Kessel 60 mm von den Kanthölzern ab, bevor er ausschwingt."), "bones": clear, "ground": "authored", "contacts": PALLET_CONTACTS + ["lifting-point"], "thermal": binding(0, 0, 0), "loads": []},
        {"id": "lifted", "title": tx("Lifted on the crane sling", "Am Krangehänge angehoben"), "description": tx("The boiler hangs from the shackle in the centre slot of its rail: with its centre of mass 190 mm in front of the rail it swings 30 degrees front-down until the mass hangs under the hook; pallet, bearers and the released straps stay on the floor.", "Der Kessel hängt am Schäkel im Mittelschlitz seiner Schiene: mit seinem Schwerpunkt 190 mm vor der Schiene schwingt er 30 Grad mit der Front nach unten, bis die Masse unter dem Haken hängt; Palette, Kanthölzer und die gelösten Gurte bleiben am Boden."), "bones": lifted, "ground": "authored", "contacts": PALLET_CONTACTS + ["lifting-point"], "thermal": binding(0, 0, 0), "loads": []},
        {"id": "reinstalled", "title": tx("Reinstalled and firing", "Wieder eingebaut und in Betrieb"), "description": tx("Hung on its rail on a new wall with the lower edge 1.22 m above the floor, refilled and connected, the boiler heats at 19.2 kW: flow 55 and return 30 degrees C, the casing and adapter release about 90 W, the flue gas leaves the adapter at about 40 K above the room and condensate drips from the trap.", "An seiner Schiene an einer neuen Wand mit der Unterkante 1,22 m über dem Boden aufgehängt, befüllt und angeschlossen, heizt der Kessel mit 19,2 kW: Vorlauf 55 und Rücklauf 30 Grad C, Gehäuse und Adapter geben etwa 90 W ab, das Abgas verlässt den Adapter etwa 40 K über Raumtemperatur und aus dem Siphon tropft Kondensat."), "bones": wall, "ground": "authored", "contacts": PALLET_CONTACTS + ["wall-bracket", "wall-rest"], "thermal": binding(0.8, 1, 1), "loads": [water]},
        {"id": "reinstalled-off", "title": tx("Reinstalled, switched off", "Wieder eingebaut, ausgeschaltet"), "description": tx("The reinstalled and refilled boiler on the wall before it is switched on: no flame, cold casing, no flue gas.", "Der wieder eingebaute und befüllte Kessel an der Wand vor dem Einschalten: keine Flamme, kaltes Gehäuse, kein Abgas."), "bones": dict(wall), "ground": "authored", "contacts": PALLET_CONTACTS + ["wall-bracket", "wall-rest"], "thermal": binding(0, 0, 0), "loads": [water]},
    ]


def clips_doc():
    """🎞 The lift from the pallet and the firing cycle on the wall with condensate drops."""
    lift = {"id": "lift", "title": tx("Lift off the pallet", "Abheben von der Palette"), "description": tx("The two ratchets are released, the strap halves swing out and drop over the pallet ends to the floor, the sling lifts the boiler 60 mm clear of the bearers and then 240 mm up, where it swings 30 degrees front-down under the hook.", "Die zwei Ratschen werden gelöst, die Gurthälften schwenken aus und fallen über die Palettenenden auf den Boden, das Gehänge hebt den Kessel 60 mm von den Kanthölzern ab und dann 240 mm hoch, wo er 30 Grad mit der Front nach unten unter den Haken schwingt."),
            "durationS": 14, "fps": 10, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "stored", "ease": "linear"}, {"t": 3, "pose": "straps-open", "ease": "linear"}, {"t": 5, "pose": "straps-down", "ease": "smooth"}, {"t": 7, "pose": "lift-clear", "ease": "smooth"}, {"t": 12, "pose": "lifted", "ease": "smooth"}, {"t": 14, "pose": "lifted", "ease": "step"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 14} for a in PALLET_CONTACTS] + [{"anchor": "lifting-point", "fromS": 5, "toS": 14}],
            "phases": [{"id": "release", "fromS": 0, "toS": 5, "title": tx("Release the straps", "Gurte lösen")}, {"id": "lift-off", "fromS": 5, "toS": 7, "title": tx("Lift off the bearers", "Von den Kanthölzern abheben")}, {"id": "hoist", "fromS": 7, "toS": 14, "title": tx("Hoist and swing under the hook", "Hochziehen und unter den Haken schwingen")}],
            "events": [{"t": 0.5, "kind": "motion", "title": tx("Ratchets opened", "Ratschen geöffnet"), "target": "strap-front-top"}, {"t": 5, "kind": "contact-on", "title": tx("Sling takes the weight", "Gehänge übernimmt das Gewicht"), "target": "lifting-point"}],
            "tracks": [{"target": "operating:burner", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 14, "v": 0}]}, {"target": "operating:controls", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 14, "v": 0}]}, {"target": "emitter:flue-gas", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 14, "v": 0}]}]}
    keys = [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 8, "pose": "reinstalled", "ease": "step"}]
    for t0 in (20, 32):
        keys.append({"t": t0, "pose": "reinstalled", "ease": "in", "bones": {"drip": {"dy": 0}}})
        keys.append({"t": t0 + 0.6, "pose": "reinstalled", "ease": "step", "bones": {"drip": {"dy": -0.4}}})
        keys.append({"t": t0 + 0.8, "pose": "reinstalled", "ease": "step", "bones": {"drip": {"dy": 0}}})
    keys.append({"t": 40, "pose": "reinstalled", "ease": "step"})
    wall_contacts = PALLET_CONTACTS + ["wall-bracket", "wall-rest"]
    firing = {"id": "firing", "title": tx("Ignition and firing on the wall", "Zünden und Brennbetrieb an der Wand"), "description": tx("The reinstalled boiler is switched on: after the pre-purge the burner lights at 8 s and heats at 19.2 kW, the casing warms, the flue gas leaves the adapter and two condensate drops fall from the cut drain hose.", "Der wieder eingebaute Kessel wird eingeschaltet: nach der Vorspülung zündet der Brenner bei 8 s und heizt mit 19,2 kW, das Gehäuse erwärmt sich, das Abgas verlässt den Adapter und zwei Kondensattropfen fallen aus dem abgeschnittenen Ablaufschlauch."),
              "durationS": 40, "fps": 5, "loop": False, "rootMotion": "none", "keys": keys,
              "contacts": [{"anchor": a, "fromS": 0, "toS": 40} for a in wall_contacts],
              "phases": [{"id": "pre-purge", "fromS": 0, "toS": 8, "title": tx("Fan pre-purge", "Gebläse-Vorspülung")}, {"id": "firing", "fromS": 8, "toS": 40, "title": tx("Firing at 19.2 kW", "Brennbetrieb mit 19,2 kW")}],
              "events": [{"t": 8, "kind": "ignition", "title": tx("Burner lights", "Brenner zündet"), "target": "burner"}, {"t": 20, "kind": "thermal", "title": tx("Condensate drop falls", "Kondensattropfen fällt"), "target": "condensate-trap"}, {"t": 32, "kind": "thermal", "title": tx("Condensate drop falls", "Kondensattropfen fällt"), "target": "condensate-trap"}],
              "tracks": [{"target": "operating:burner", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 8, "v": 0.8}, {"t": 40, "v": 0.8}]}, {"target": "operating:controls", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1, "v": 1}, {"t": 40, "v": 1}]}, {"target": "emitter:flue-gas", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 8, "v": 1}, {"t": 40, "v": 1}]}]}
    return [lift, firing]


def main():
    """🚀 Builds every scale, the models and writes the asset document."""
    build_rig()
    ms = masses()
    for b, (kg, cx, cy, dist) in ms.items():
        BONES[b]["mass"] = {"kg": r(kg, 4), "comX": r(cx, 4), "comY": r(cy, 4), "distribution": dist, "source": "authored-estimate" if b == "boiler" else "literature", "reference": {"pallet": "en-13698-1", "boiler": "combi-data-sheets", "bracket": "combi-data-sheets"}.get(b, "en-12195-2")}
    total = sum(v[0] for v in ms.values())
    reps, vols = {}, {}
    areas = None
    for scale in (50, 100, 200):
        parts = build_parts(scale)
        if scale == 50:
            areas = region_areas(parts)
            areas.update(hidden_areas())
        reps[str(scale)] = representation(scale, parts)
        body = [solid_of(p) for p in parts]
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, build_parts(scale)), "decisions": VOL_DECISIONS[scale]}
    reps["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": tx("At 1:500 the whole component on its pallet is 2.4 mm long and 2.1 mm high on paper, under the 3 mm a recognisable drawing needs; the boiler alone is 0.7 by 1.4 mm and its straps, rail and connections are 0.004 to 0.1 mm. Draw it at 1:200 or larger.", "Bei 1:500 ist das ganze Bauteil mit Palette 2,4 mm lang und 2,1 mm hoch auf Papier, unter den 3 mm, die eine erkennbare Zeichnung braucht; der Kessel allein misst 0,7 mal 1,4 mm, Gurte, Schiene und Anschlüsse 0,004 bis 0,1 mm. Bei 1:200 oder größer zeichnen."), "limitingFeature": tx("Whole component of 1.22 m on its pallet", "Ganzes Bauteil von 1,22 m mit Palette"), "limitingSizeM": 1.224, "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}
    vols["500"] = {"status": "unavailable", "denominator": 500}
    build_parts(50)
    xs = [box_world_rect(p) for p in PARTS if not p["texture"]]
    ext = (min(q[0] for q in xs), max(q[1] for q in xs), min(q[2] for q in xs), max(q[3] for q in xs), min(q[4] for q in xs), max(q[5] for q in xs))
    print("[DEBUG] extents", [r(v, 4) for v in ext], "mass", r(total, 3))
    print("[DEBUG] areas", {k: r(v, 4) for k, v in areas.items()})
    thermal = {"environment": {"airTempC": 18, "meanRadiantTempC": 18, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
               "passengers": [],
               "operating": [{"id": "burner", "title": tx("Burner heat input", "Brennerbelastung"), "levels": [{"v": 0, "title": tx("Off", "Aus")}, {"v": 0.8, "title": tx("Space heating 19.2 kW", "Heizbetrieb 19,2 kW")}, {"v": 1, "title": tx("Hot water 24 kW", "Warmwasser 24 kW")}]}, {"id": "controls", "title": tx("Fan, pump and controls", "Gebläse, Pumpe und Regelung"), "levels": [{"v": 0, "title": tx("Off", "Aus")}, {"v": 1, "title": tx("On (85 W rated)", "An (85 W Nennaufnahme)")}]}],
               "regions": regions_doc(areas),
               "emitters": [{"id": "flue-gas", "kind": "flue", "title": tx("Flue gas leaving the 60 mm flue pipe of the adapter", "Abgas aus dem 60-mm-Abgasrohr des Adapters"), "anchor": "flue-outlet", "directionDeg": 90, "diameterM": 0.057, "operatingKey": "burner", "volumeFlowM3S": 0.0092, "temperatureRiseK": 40, "vapourKgPerKg": 0.07, "source": "authored-estimate", "reference": "turns-2012"}],
               "assumptions": [tx("Stored, the boiler is drained and capped and every surface is at the 18 degrees C of the hall. Reinstalled at 19.2 kW heat input (0.8 of the 24 kW hot-water rating) the heating water runs at 55/30 degrees C; only the surface losses reach the room: casing 60 W, adapter 12 W, connections 15 W, trap 3 W, fascia 4 W, burner window 3 W, about 0.5 % of the input. The flue gas (0.0092 m3/s at 24 kW, 40 K above the room after condensation) is shown at the adapter mouth although a room-sealed boiler leads it to the outside.", "Gelagert ist der Kessel entleert und verschlossen, jede Fläche hat die 18 Grad C der Halle. Wieder eingebaut mit 19,2 kW Belastung (0,8 der 24 kW Warmwasser-Nennbelastung) läuft das Heizwasser mit 55/30 Grad C; nur die Oberflächenverluste erreichen den Raum: Gehäuse 60 W, Adapter 12 W, Anschlüsse 15 W, Siphon 3 W, Blende 4 W, Brennerfenster 3 W, etwa 0,5 % der Belastung. Das Abgas (0,0092 m3/s bei 24 kW, nach der Kondensation 40 K über dem Raum) ist an der Adaptermündung gezeigt, obwohl ein raumluftunabhängiger Kessel es nach außen führt."),
                               tx("Nameplates: the burner states its 24 kW fuel input (hot water, Hi), the fan, pump and controls their 85 W electrical input; the pallet, bearers, straps, rail and heat exchanger draw nothing.", "Typenschild: der Brenner nennt seine 24 kW Brennstoffbelastung (Warmwasser, Hi), Gebläse, Pumpe und Regelung ihre 85 W elektrische Aufnahme; Palette, Kanthölzer, Gurte, Schiene und Wärmeübertrager nehmen nichts auf."),
                               tx("Used surfaces: the powder coating has yellowed (solar absorptance 0.38 instead of about 0.3 new), the brass and copper are tarnished and oxidised (emissivity 0.6 to 0.65 instead of 0.03 to 0.1 polished), the pallet timber is weathered grey (absorptance 0.65) and the zinc of the rail is dull (0.28); all are authored estimates from the tables of Incropera.", "Gebrauchte Oberflächen: die Pulverbeschichtung ist vergilbt (solarer Absorptionsgrad 0,38 statt etwa 0,3 neu), Messing und Kupfer sind angelaufen und oxidiert (Emissionsgrad 0,6 bis 0,65 statt 0,03 bis 0,1 poliert), das Palettenholz ist grau verwittert (Absorptionsgrad 0,65) und das Zink der Schiene matt (0,28); alles Schätzwerte nach den Tabellen von Incropera.")]}
    structure = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": PALLET_CONTACTS,
                 "assumptions": [tx("Stored: the pallet stands on the floor with its four corner blocks as contact supports; the boiler bears through the folded bottom edge of its chassis on the two bearers, which bear on the deck boards, stringers, blocks and bottom boards; the ratchet straps are pretensioned tension members between the hooks at the deck ends.", "Gelagert: die Palette steht mit ihren vier Eckklötzen als Kontaktauflager am Boden; der Kessel lagert mit der gekanteten Unterkante seines Chassis auf den zwei Kanthölzern, die auf Deckbrettern, Querbrettern, Klötzen und Bodenbrettern lagern; die Ratschengurte sind vorgespannte Zugglieder zwischen den Haken an den Deckenden."),
                                 tx("Lifted and reinstalled: the pose adds its own supports, the shackle of the sling in the centre slot of the rail (pin) or the rail screwed to the wall (pin) and the EPDM spacers against the wall (roller, horizontal). The product solves the whole asset as one rigid body on all active supports, so the pallet left on the floor and the boiler share one equilibrium; the reactions of the floor and of the hook or wall are therefore an idealisation of two separate bodies.", "Angehoben und wieder eingebaut: die Pose fügt ihre eigenen Auflager hinzu, den Schäkel des Gehänges im Mittelschlitz der Schiene (Gelenk) oder die an die Wand geschraubte Schiene (Gelenk) und die EPDM-Abstandshalter an der Wand (Rollenlager, waagrecht). Das Produkt löst das ganze Objekt als einen starren Körper auf allen aktiven Auflagern, die am Boden gebliebene Palette und der Kessel teilen daher ein Gleichgewicht; die Reaktionen von Boden und Haken oder Wand sind deshalb eine Idealisierung zweier getrennter Körper."),
                                 tx(f"Masses: Euro pallet, bearers and hooks {ms['pallet'][0]:.1f} kg from their volumes (pallet timber 550 kg/m3 at 22 % moisture, spruce 450 kg/m3, hooks as bent 7850 kg/m3 strip), boiler 36.0 kg dry (catalogue class of 24 kW combis), rail 0.55 kg, webbing 0.06 kg/m and two ratchets of 0.6 kg; total {total:.2f} kg.", f"Massen: Europalette, Kanthölzer und Haken {ms['pallet'][0]:.1f} kg aus ihren Volumina (Palettenholz 550 kg/m3 bei 22 % Feuchte, Fichte 450 kg/m3, Haken als gebogenes Band 7850 kg/m3), Kessel 36,0 kg trocken (Katalogklasse der 24-kW-Kombigeräte), Schiene 0,55 kg, Gurtband 0,06 kg/m und zwei Ratschen zu 0,6 kg; zusammen {total:.2f} kg.".replace(".", ",", 0))],
                 "clinical": False}
    asset = {
        "id": "used-wall-hung-gas-boiler", "category": "components",
        "title": tx("Used Wall-Hung Gas Condensing Boiler", "Gebrauchtes Gas-Brennwert-Wandgerät"),
        "summary": tx("A dismantled 24 kW wall-hung gas condensing combi boiler of 440 x 700 x 350 mm as a component exchange offers it: drained, its five connections cut and capped, the concentric 60/100 flue adapter still on top and the wall rail taken off the old wall hooked into its pocket, standing on two bearers on a Euro pallet under two ratchet straps. It is lifted on a crane sling from its rail and swings 30 degrees under the hook, and reinstalled on a new wall it fires at 19.2 kW with its flue plume and dripping condensate.", "Ein ausgebautes 24-kW-Gas-Brennwert-Kombiwandgerät von 440 x 700 x 350 mm, wie eine Bauteilbörse es anbietet: entleert, seine fünf Anschlüsse abgeschnitten und verschlossen, oben noch der konzentrische Abgasadapter 60/100 und die von der alten Wand abgenommene Schiene in ihrer Tasche, auf zwei Kanthölzern auf einer Europalette unter zwei Ratschengurten. Es wird am Krangehänge an seiner Schiene angehoben und schwingt 30 Grad unter den Haken, und an einer neuen Wand wieder eingebaut brennt es mit 19,2 kW mit Abgasfahne und tropfendem Kondensat."),
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "plumbing", "gas-boiler", "condensing-boiler", "combi-boiler", "wall-hung", "steel", "pallet", "ratchet-strap", "flue"],
        "provenance": {"author": "Semio staffage component authors (used HVAC wave, author 02)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": a, "citation": b} for a, b in REFERENCES],
                       "notes": tx("Dimensions and masses follow the reference table of the authoring report: casing 440 x 700 x 350 mm and 36 kg dry from the data sheets of the 24 kW combi class, Euro pallet 1200 x 800 x 144 mm after EN 13698-1, bearers 40 x 80 x 400 mm, 50 mm webbing after EN 12195-2. Every value is literature or an authored estimate; nothing is measured.", "Maße und Massen folgen der Referenztabelle des Autorenberichts: Gehäuse 440 x 700 x 350 mm und 36 kg trocken aus den Datenblättern der 24-kW-Kombiklasse, Europalette 1200 x 800 x 144 mm nach EN 13698-1, Kanthölzer 40 x 80 x 400 mm, 50-mm-Gurtband nach EN 12195-2. Jeder Wert ist Literatur oder Schätzwert; nichts ist gemessen.")},
        "dimensions": {"heightM": r(ext[3] - ext[2], 4), "widthM": r(ext[1] - ext[0], 4), "depthM": r(ext[5] - ext[4], 4), "massKg": r(total, 3)},
        "materials": [material(m) for m in MATERIALS],
        "rig": {"kind": "mechanical", "root": "pallet", "bones": list(BONES.values())},
        "volumes": vols,
        "anchors": anchors_doc(),
        "representations": reps,
        "symbols": [],
        "poses": poses_doc(),
        "clips": clips_doc(),
        "thermal": thermal,
        "structure": structure,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print("[DEBUG] wrote", OUT, os.path.getsize(OUT))


if __name__ == "__main__":
    main()
