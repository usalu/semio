"""🌀 Authoritative chisel of the used brine-to-water heat pump: states every hand-chosen number and writes `🔣️.json` next to it.

Parts are authored once per scale in world rest coordinates (x ahead = the open front of the unit, y up, z left); the 3D solids
(bone-local) and the three 2D views (elevation bone-local, front and plan in world rest coordinates) are both written from that one
table, so 2D and 3D agree by construction.

@see ../../brief.md
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

BONES = {"pallet": (0.0, 0.0), "carrier": (0.0, 0.144), "unit": (0.0, 0.144)}
Y_TOP = 1.294


def t(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def r6(v):
    """🎯️ Rounded to 0.1 mm."""
    return round(v + 0.0, 5)


def part(pid, bone, kind, mat, tone, region, views="efp", **kw):
    """🧩️ One part of the parts table: `box` (c, s), `cyl` (axis, c, r, a0, a1, profile?) or `cap` (p0, p1, r)."""
    d = dict(id=pid, bone=bone, kind=kind, mat=mat, tone=tone, region=region, views=views)
    d.update(kw)
    return d


def box(pid, bone, mat, tone, region, lo, hi, views="efp", **kw):
    """📦️ An axis-aligned box from its low and high world corners."""
    c = [(a + b) / 2 for a, b in zip(lo, hi)]
    s = [b - a for a, b in zip(lo, hi)]
    return part(pid, bone, "box", mat, tone, region, views, c=c, s=s, **kw)


def cyl(pid, bone, mat, tone, region, axis, c, r, a0, a1, views="efp", profile=None, **kw):
    """🥫️ A cylinder along a world axis through `c` (the axial coordinate along `axis` runs from a0 to a1, world)."""
    return part(pid, bone, "cyl", mat, tone, region, views, axis=axis, c=c, r=r, a0=a0, a1=a1, profile=profile, **kw)


def cap(pid, bone, mat, tone, region, p0, p1, r, views="efp", **kw):
    """💊️ A capsule (pipe run) between two world points."""
    return part(pid, bone, "cap", mat, tone, region, views, p0=p0, p1=p1, r=r, **kw)


def pallet_parts(scale):
    """🪵️ The half pallet 800 x 600 x 144 mm (EPAL 6 type) and the two timber chocks, per scale."""
    P = []
    zs3 = [-0.25, 0.0, 0.25]
    xs3 = [-0.35, 0.0, 0.35]
    if scale in (50, 100):
        for i, z in enumerate(zs3):
            P.append(box(f"bottom-board-{i}", "pallet", "softwood", 0.55, "pallet", (-0.4, 0.0, z - 0.05), (0.4, 0.022, z + 0.05), views="ef" if i == 0 else "f"))
        for i, z in enumerate(zs3):
            for j, x in enumerate(xs3):
                P.append(box(f"block-{i}-{j}", "pallet", "softwood", 0.5, "pallet", (x - 0.05, 0.022, z - 0.05), (x + 0.05, 0.1, z + 0.05),
                             views=("e" if i == 0 else "") + ("f" if j == 2 else ""), dhi=(x + 0.05, 0.122, z + 0.05)))
        for j, x in enumerate(xs3):
            P.append(box(f"cross-board-{j}", "pallet", "softwood", 0.58, "pallet", (x - 0.05, 0.1, -0.3), (x + 0.05, 0.122, 0.3), views="f" if j == 2 else ""))
        for i, z in enumerate([-0.25, -0.125, 0.0, 0.125, 0.25]):
            P.append(box(f"deck-board-{i}", "pallet", "softwood", 0.62, "pallet", (-0.4, 0.122, z - 0.05), (0.4, 0.144, z + 0.05), views="e" if i == 0 else ""))
        P.append(box("deck-front", "pallet", "softwood", 0.62, "pallet", (-0.4, 0.122, -0.3), (0.4, 0.144, 0.3), views="f", nosolid=True))
        P.append(box("deck-top", "pallet", "softwood", 0.62, "pallet", (-0.4, 0.122, -0.3), (0.4, 0.144, 0.3), views="p", nosolid=True, hatch="grain" if scale == 50 else None))
    else:
        for i, z in enumerate(zs3):
            for j, x in enumerate(xs3):
                P.append(box(f"block-{i}-{j}", "pallet", "softwood", 0.5, "pallet", (x - 0.05, 0.0, z - 0.05), (x + 0.05, 0.1, z + 0.05), views=("e" if i == 0 else "") + ("f" if j == 2 else "")))
        P.append(box("deck", "pallet", "softwood", 0.6, "pallet", (-0.4, 0.1, -0.3), (0.4, 0.144, 0.3)))
    for nm, x0 in (("chock-front", 0.35), ("chock-back", -0.39)):
        P.append(box(nm, "pallet", "softwood", 0.45, "pallet", (x0, 0.144, -0.26), (x0 + 0.04, 0.184, 0.26)))
    return P


FOOT_XZ = [(0.33, 0.28), (0.33, -0.28), (-0.33, 0.28), (-0.33, -0.28)]
ZH, ZB = -0.199, 0.199
XH = (-0.26, -0.14)
XB = (-0.27, -0.13)
HOSES = [("heat-flow", XH[0], ZH, "heating"), ("heat-return", XH[1], ZH, "heating"), ("brine-out", XB[0], ZB, "brine"), ("brine-in", XB[1], ZB, "brine")]


def unit_parts(scale):
    """🧊️ The heat pump itself, per scale: base frame, casing without its front panel, refrigerant and hydraulic parts, controls, cut hoses."""
    U = []
    fine = scale == 50
    mid = scale == 100
    coarse = scale == 200
    if coarse:
        U.append(box("plinth", "unit", "galvanised-steel", 0.7, "base-frame", (-0.35, 0.144, -0.3), (0.35, 0.229, 0.3), views="ef"))
    else:
        for i, (x, z) in enumerate(FOOT_XZ):
            U.append(cyl(f"foot-{i}", "unit", "rubber", 0.15, "base-frame", "y", (x, 0, z), 0.02, 0.144, 0.169, views=("e" if z < 0 else "") + ("f" if x > 0 else "")))
        U.append(box("frame-left", "unit", "galvanised-steel", 0.7, "base-frame", (-0.35, 0.169, 0.26), (0.35, 0.229, 0.3), views=""))
        U.append(box("frame-right", "unit", "galvanised-steel", 0.7, "base-frame", (-0.35, 0.169, -0.3), (0.35, 0.229, -0.26), views="e"))
        U.append(box("frame-front", "unit", "galvanised-steel", 0.68, "base-frame", (0.31, 0.169, -0.26), (0.35, 0.229, 0.26), views="f", dlo=(0.31, 0.169, -0.3), dhi=(0.35, 0.229, 0.3)))
        U.append(box("frame-back", "unit", "galvanised-steel", 0.68, "base-frame", (-0.35, 0.169, -0.26), (-0.31, 0.229, 0.26), views=""))
    U.append(box("base-plate", "unit", "galvanised-steel", 0.66, "base-frame", (-0.329, 0.229, -0.279), (0.35, 0.233, 0.279), views="", sheet=True))
    if fine:
        U.append(box("side-right-sheet", "unit", "painted-steel", 0.86, "casing", (-0.35, 0.229, -0.3), (0.35, 1.293, -0.299), views="e", sheet=True))
        U.append(box("side-right-lining", "unit", "acoustic-foam", 0.42, "lining", (-0.329, 0.229, -0.299), (0.35, 1.273, -0.279), views="f", dlo=(-0.329, 0.229, -0.3), dhi=(0.35, 1.294, -0.279)))
        U.append(box("side-left-sheet", "unit", "painted-steel", 0.86, "casing", (-0.35, 0.229, 0.299), (0.35, 1.293, 0.3), views="", sheet=True))
        U.append(box("side-left-lining", "unit", "acoustic-foam", 0.42, "lining", (-0.329, 0.229, 0.279), (0.35, 1.273, 0.299), views="f", dhi=(0.35, 1.294, 0.3)))
        U.append(box("back-sheet", "unit", "painted-steel", 0.82, "casing", (-0.35, 0.229, -0.299), (-0.349, 1.293, 0.299), views="", sheet=True))
        U.append(box("back-lining", "unit", "acoustic-foam", 0.4, "lining", (-0.349, 0.229, -0.299), (-0.329, 1.273, 0.299), views="f", dlo=(-0.349, 0.229, -0.279), dhi=(-0.329, 1.273, 0.279)))
        U.append(box("top-sheet", "unit", "painted-steel", 0.9, "casing", (-0.35, 1.293, -0.3), (0.35, 1.294, 0.3), views="p", sheet=True))
        U.append(box("top-lining", "unit", "acoustic-foam", 0.42, "lining", (-0.349, 1.273, -0.299), (0.35, 1.293, 0.299), views="f", dlo=(-0.349, 1.273, -0.279), dhi=(0.35, 1.294, 0.279)))
    else:
        flag = {"sheet": True} if coarse else {}
        U.append(box("side-right", "unit", "casing-panel", 0.86, "casing", (-0.35, 0.229, -0.3), (0.35, 1.294, -0.279), views="e" if coarse else "ef", regions=["casing", "lining"], **flag))
        U.append(box("side-left", "unit", "casing-panel", 0.86, "casing", (-0.35, 0.229, 0.279), (0.35, 1.294, 0.3), views="" if coarse else "f", regions=["casing", "lining"], **flag))
        U.append(box("back", "unit", "casing-panel", 0.4, "casing", (-0.35, 0.229, -0.279), (-0.329, 1.273, 0.279), views="" if coarse else "f", regions=["casing", "lining"], **flag))
        U.append(box("top", "unit", "casing-panel", 0.9, "casing", (-0.35, 1.273, -0.279), (0.35, 1.294, 0.279), views="p" if coarse else "fp", regions=["casing", "lining"], **flag, dlo=(-0.35, 1.273, -0.3), dhi=(0.35, 1.294, 0.3)))
        if coarse:
            U.append(box("casing-front", "unit", "acoustic-foam", 0.4, "casing", (-0.35, 0.229, -0.3), (0.35, 1.294, 0.3), views="f", nosolid=True))
    if not coarse:
        for nm, z0 in (("upright-right", -0.279), ("upright-left", 0.249)):
            U.append(box(nm, "unit", "galvanised-steel", 0.72, "base-frame", (0.32, 0.233, z0), (0.35, 1.273, z0 + 0.03), views="f"))
    U.append(box("compressor-base", "unit", "galvanised-steel", 0.55, "base-frame", (-0.03, 0.233, -0.249), (0.23, 0.265, 0.011), views="" if coarse else "f", dlo=(-0.03, 0.229, -0.249)))
    cr, cz = 0.12, -0.129
    if fine:
        prof = [[0.265, cr], [0.62, cr], [0.635, 0.116], [0.648, 0.104], [0.658, 0.086], [0.665, 0.06], [0.668, 0.0]]
    elif mid:
        prof = [[0.265, cr], [0.625, cr], [0.65, 0.1], [0.668, 0.0]]
    else:
        prof = None
    U.append(cyl("compressor", "unit", "compressor-steel", 0.12, "compressor", "y", (0.1, 0, cz), cr, 0.265, 0.668 if prof else 0.665, views="f", profile=prof, dlo=(-0.02, 0.229, cz - cr) if coarse else None))
    if fine or mid:
        U.append(box("terminal-box", "unit", "polymer", 0.3, "compressor", (0.22, 0.5, cz - 0.04), (0.26, 0.58, cz + 0.04), views="f"))
        U.append(cap("discharge-rise", "unit", "copper", 0.55, "compressor", (0.1, 0.655, cz), (0.1, 0.74, cz), 0.008, views="f"))
        U.append(cap("discharge-turn", "unit", "copper", 0.55, "compressor", (0.1, 0.74, cz), (0.1, 0.74, -0.19), 0.008, views="f"))
        U.append(cap("discharge-run", "unit", "copper", 0.55, "compressor", (0.1, 0.74, -0.19), (-0.12, 0.74, -0.19), 0.008, views="f"))
        U.append(cap("suction-run", "unit", "copper", 0.5, "compressor", (-0.1, 0.45, ZB), (0.1, 0.45, ZB), 0.011, views="f"))
        U.append(cap("suction-drop", "unit", "copper", 0.5, "compressor", (0.1, 0.45, ZB), (0.1, 0.45, -0.014), 0.011, views="f"))
    if fine:
        U.append(cap("liquid-out", "unit", "copper", 0.55, "compressor", (-0.12, 0.36, ZH), (-0.06, 0.36, ZH), 0.005, views="f"))
        U.append(cap("liquid-cross", "unit", "copper", 0.55, "compressor", (-0.06, 0.36, ZH), (-0.06, 0.36, ZB), 0.005, views="f"))
        U.append(cap("liquid-in", "unit", "copper", 0.55, "compressor", (-0.06, 0.36, ZB), (-0.1, 0.36, ZB), 0.005, views="f"))
        U.append(box("expansion-valve", "unit", "brass", 0.4, "compressor", (-0.075, 0.335, 0.055), (-0.045, 0.385, 0.085), views="f"))
    U.append(box("evaporator", "unit", "armaflex", 0.15, "evaporator", (-0.3, 0.29, 0.149), (-0.1, 0.81, 0.249), views="f", dlo=(-0.3, 0.229, -0.009) if coarse else None, layer="insulation"))
    if fine:
        U.append(box("evaporator-plates", "unit", "stainless-steel", 0.6, "evaporator", (-0.287, 0.303, 0.162), (-0.113, 0.797, 0.236), views="", layer="plates"))
    U.append(box("evaporator-bracket", "unit", "galvanised-steel", 0.6, "base-frame", (-0.23, 0.233, 0.169), (-0.17, 0.29, 0.229), views="" if coarse else "f", dlo=(-0.23, 0.229, 0.169)))
    U.append(box("condenser", "unit", "stainless-steel", 0.65, "condenser", (-0.28, 0.33, -0.249), (-0.12, 0.79, -0.149), views="f"))
    U.append(box("condenser-bracket", "unit", "galvanised-steel", 0.6, "base-frame", (-0.23, 0.233, -0.229), (-0.17, 0.33, -0.169), views="" if coarse else "f", dlo=(-0.23, 0.229, -0.229)))
    hr = 0.015 if coarse else 0.014
    rv = "" if coarse else "f"
    U.append(cyl("riser-heat-flow", "unit", "steel-pipe", 0.45, "heating-lines", "y", (XH[0], 0, ZH), hr, 0.79, Y_TOP, views=rv))
    U.append(cyl("riser-heat-return", "unit", "steel-pipe", 0.45, "heating-lines", "y", (XH[1], 0, ZH), hr, 0.79, Y_TOP, views=rv))
    U.append(cyl("riser-brine-out", "unit", "armaflex", 0.2, "brine-lines", "y", (XB[0], 0, ZB), 0.025, 0.81, Y_TOP, views=rv))
    U.append(cyl("riser-brine-in", "unit", "armaflex", 0.2, "brine-lines", "y", (XB[1], 0, ZB), 0.025, 0.81, Y_TOP, views=rv))
    for nm, x, z, r in (("heating-pump", XH[1], ZH, 0.045), ("brine-pump", XB[1], ZB, 0.05)):
        if coarse:
            U.append(cyl(f"{nm}", "unit", "cast-iron", 0.35, "pumps", "x", (0, 0.99, z), 0.05, x - r, 0.06, views=""))
        else:
            U.append(cyl(f"{nm}-housing", "unit", "cast-iron", 0.3, "pumps", "y", (x, 0, z), r, 0.92, 1.06, views="f"))
            U.append(cyl(f"{nm}-motor", "unit", "polymer", 0.55, "pumps", "x", (0, 0.99, z), 0.05, x + r, 0.06, views="f"))
            if fine:
                U.append(box(f"{nm}-terminal", "unit", "polymer", 0.7, "pumps", (-0.03, 1.04, z - 0.03), (0.03, 1.07, z + 0.03), views="f"))
    U.append(box("control-box", "unit", "polymer", 0.8, "controls", (0.22, 1.13, -0.249), (0.32, 1.273, 0.249), views="f"))
    if fine:
        U.append(box("display", "unit", "polymer", 0.15, "controls", (0.32, 1.19, -0.08), (0.322, 1.25, 0.04), views="f", texture=True, sheet=True))
    if coarse:
        for nm, xs, z, rr, reg, m, tn in (("hoses-heating", XH, ZH, 0.025, "heating-lines", "stainless-hose", 0.5), ("hoses-brine", XB, ZB, 0.0325, "brine-lines", "armaflex", 0.2)):
            U.append(box(nm, "unit", m, tn, reg, (xs[0] - rr, Y_TOP, z - rr), (xs[1] + rr, 1.42, z + rr)))
    else:
        for nm, x, z, circ in HOSES:
            heat = circ == "heating"
            reg = "heating-lines" if heat else "brine-lines"
            ru = 0.025 if heat else 0.0325
            U.append(cyl(f"union-{nm}", "unit", "brass", 0.45, reg, "y", (x, 0, z), ru, Y_TOP, 1.314))
            U.append(cyl(f"hose-{nm}", "unit", "stainless-hose" if heat else "armaflex", 0.5 if heat else 0.2, reg, "y", (x, 0, z), 0.0165 if heat else 0.03, 1.314, 1.40))
            U.append(cyl(f"cap-{nm}", "unit", "polymer", 0.3 if heat else 0.25, reg, "y", (x, 0, z), 0.0195 if heat else 0.033, 1.40, 1.42))
    if fine:
        U.append(cyl("cable-stub", "unit", "polymer", 0.2, "controls", "y", (-0.05, 0, 0.0), 0.0075, Y_TOP, 1.36))
        U.append(box("type-plate", "unit", "aluminium", 0.75, "casing", (-0.29, 0.98, -0.3005), (-0.17, 1.05, -0.3), views="e", texture=True, sheet=True))
    return U


def to_local(bone, p):
    """📐️ World rest point to bone-local (all bones rest unrotated)."""
    bx, by = BONES[bone]
    return [r6(p[0] - bx), r6(p[1] - by), r6(p[2])]


AX = {"x": 0, "y": 1, "z": 2}


def solid_of(p):
    """🧱️ The volume solid of one part."""
    base = {"id": p["id"], "bone": p["bone"], "material": p["mat"], "role": "detail" if p.get("texture") else "body", "tone": p["tone"]}
    if p.get("sheet"):
        base["sheet"] = True
    if p.get("texture"):
        base["texture"] = True
    if p.get("slender"):
        base["slender"] = True
    if p["kind"] == "box":
        base.update(kind="box", size=[r6(v) for v in p["s"]], transform={"translation": to_local(p["bone"], p["c"])})
    elif p["kind"] == "cyl":
        ax = p["axis"]
        i = AX[ax]
        c = list(p["c"])
        c[i] = 0.0 if ax != "y" else BONES[p["bone"]][1]
        origin = to_local(p["bone"], c)
        origin[i] = 0.0
        off = BONES[p["bone"]][1] if ax == "y" else 0.0
        prof = p["profile"] or [[p["a0"], p["r"]], [p["a1"], p["r"]]]
        base.update(kind="revolve", profile=[[r6(a - off), r6(r)] for a, r in prof], axis=[1 if ax == "x" else 0, 1 if ax == "y" else 0, 1 if ax == "z" else 0], origin=origin)
    else:
        base.update(kind="capsule", p0=to_local(p["bone"], p["p0"]), p1=to_local(p["bone"], p["p1"]), r0=p["r"], r1=p["r"])
    return base


def view_coords(view, bone, pt):
    """🗺️ Paper-plane coordinates of a world rest point in a view: elevation bone-local (x, y), front (z, y), plan (x, z)."""
    if view == "elevation":
        bx, by = BONES[bone]
        return (pt[0] - bx, pt[1] - by)
    if view == "front":
        return (pt[2], pt[1])
    return (pt[0], pt[2])


def near(view, lo, hi):
    """👁️ Nearness of a part to the viewer of a view (elevation seen from -z, front from +x, plan from +y)."""
    return {"elevation": -lo[2], "front": hi[0], "plan": hi[1]}[view]


def extent(p):
    """📏️ World bounds of a part."""
    if p["kind"] == "box":
        lo = [c - s / 2 for c, s in zip(p["c"], p["s"])]
        hi = [c + s / 2 for c, s in zip(p["c"], p["s"])]
    elif p["kind"] == "cyl":
        i = AX[p["axis"]]
        rmax = max(r for _, r in (p["profile"] or [[0, p["r"]]]))
        lo = [p["c"][k] - rmax for k in range(3)]
        hi = [p["c"][k] + rmax for k in range(3)]
        lo[i], hi[i] = p["a0"], p["a1"]
    else:
        lo = [min(a, b) - p["r"] for a, b in zip(p["p0"], p["p1"])]
        hi = [max(a, b) + p["r"] for a, b in zip(p["p0"], p["p1"])]
    return lo, hi


def shapes_of(p, view):
    """✏️ The filled 2D shape of one part in one view."""
    lo, hi = extent(p)
    lo = p.get("dlo") or lo
    hi = p.get("dhi") or hi
    s = {"id": p["id"], "bone": p["bone"], "role": "detail" if p.get("texture") else "body", "z": round(2 + near(view, lo, hi), 4), "filled": True, "tone": p["tone"], "material": p["mat"]}
    if p.get("regions"):
        s["regions"] = p["regions"]
    elif p["region"]:
        s["region"] = p["region"]
    if p.get("hatch") and view == "plan":
        s["hatch"] = p["hatch"]
    a = view_coords(view, p["bone"], lo)
    b = view_coords(view, p["bone"], hi)
    x0, x1 = sorted((a[0], b[0]))
    y0, y1 = sorted((a[1], b[1]))
    if p["kind"] == "cap":
        q0 = view_coords(view, p["bone"], p["p0"])
        q1 = view_coords(view, p["bone"], p["p1"])
        if abs(q0[0] - q1[0]) < 1e-9 and abs(q0[1] - q1[1]) < 1e-9:
            s.update(kind="ellipse", cx=r6(q0[0]), cy=r6(q0[1]), rx=p["r"], ry=p["r"])
        else:
            s.update(kind="capsule", x0=r6(q0[0]), y0=r6(q0[1]), x1=r6(q1[0]), y1=r6(q1[1]), r0=p["r"], r1=p["r"])
        return s
    if p["kind"] == "cyl":
        ax = p["axis"]
        along = {"elevation": {"x": "u", "y": "v", "z": "n"}, "front": {"z": "u", "y": "v", "x": "n"}, "plan": {"x": "u", "z": "v", "y": "n"}}[view][ax]
        if along == "n":
            cc = view_coords(view, p["bone"], p["c"])
            rmax = max(r for _, r in (p["profile"] or [[0, p["r"]]]))
            s.update(kind="ellipse", cx=r6(cc[0]), cy=r6(cc[1]), rx=rmax, ry=rmax)
            return s
        if p["profile"]:
            cc = view_coords(view, p["bone"], p["c"])
            off = BONES[p["bone"]][1] if (view == "elevation" and ax == "y") else 0.0
            right = [[r6(cc[0] + r), r6(a - off)] for a, r in p["profile"]]
            left = [[r6(cc[0] - r), r6(a - off)] for a, r in reversed(p["profile"]) if r > 0]
            pts = right + left
            s.update(kind="poly", points=pts, closed=True)
            return s
    s.update(kind="rect", x=r6(x0), y=r6(y0), w=r6(x1 - x0), h=r6(y1 - y0))
    return s


def strokes(scale):
    """〰️ Detail strokes per view: panel seams, fold lines, louvre slots, screw rows, old marks."""
    E, F, P = [], [], []
    if scale == 200:
        return E, F, P
    d = 0.144
    E.append({"id": "front-fold", "bone": "unit", "role": "detail", "z": 3.5, "filled": False, "tone": 0.4, "kind": "poly", "points": [[0.335, 0.09], [0.335, 1.146]], "closed": False})
    E.append({"id": "rear-fold", "bone": "unit", "role": "detail", "z": 3.5, "filled": False, "tone": 0.4, "kind": "poly", "points": [[-0.335, 0.09], [-0.335, 1.146]], "closed": False})
    E.append({"id": "lid-seam", "bone": "unit", "role": "detail", "z": 3.5, "filled": False, "tone": 0.4, "kind": "poly", "points": [[-0.35, 1.129], [0.35, 1.129]], "closed": False})
    if scale == 50:
        for k, y in enumerate([0.25, 0.55, 0.85]):
            E.append({"id": f"hook-slot-{k}", "bone": "unit", "role": "detail", "z": 3.6, "filled": False, "tone": 0.3, "kind": "poly", "points": [[0.342, y], [0.342, y + 0.03]], "closed": False})
        E.append({"id": "type-plate-text", "bone": "unit", "role": "detail", "z": 3.7, "filled": False, "tone": 0.3, "kind": "path", "d": "M -0.28 0.89 L -0.19 0.89 M -0.28 0.875 L -0.21 0.875 M -0.28 0.86 L -0.2 0.86"})
        E.append({"id": "scuff", "bone": "unit", "role": "detail", "z": 3.6, "filled": False, "tone": 0.5, "kind": "path", "d": "M 0.05 0.16 L 0.16 0.19 M 0.08 0.14 L 0.2 0.175"})
        F.append({"id": "back-seam", "bone": "unit", "role": "detail", "z": 1.7, "filled": False, "tone": 0.35, "kind": "poly", "points": [[-0.279, 0.75], [0.279, 0.75]], "closed": False})
        F.append({"id": "control-display-frame", "bone": "unit", "role": "detail", "z": 2.8, "filled": False, "tone": 0.3, "kind": "poly", "points": [[-0.1, 1.18], [0.06, 1.18], [0.06, 1.26], [-0.1, 1.26], [-0.1, 1.18]], "closed": False})
        F.append({"id": "compressor-weld", "bone": "unit", "role": "detail", "z": 2.5, "filled": False, "tone": 0.6, "kind": "poly", "points": [[-0.24, 0.6], [0.0, 0.6]], "closed": False})
        F.append({"id": "limescale", "bone": "unit", "role": "detail", "z": 2.3, "filled": False, "tone": 0.75, "kind": "path", "d": "M -0.2 0.76 L -0.12 0.74 M -0.19 0.72 L -0.11 0.7"})
        P.append({"id": "lid-seam", "bone": "unit", "role": "detail", "z": 3.5, "filled": False, "tone": 0.4, "kind": "poly", "points": [[0.335, -0.29], [0.335, 0.29]], "closed": False})
        for k, z in enumerate([-0.2, 0.0, 0.2]):
            P.append({"id": f"screw-{k}", "bone": "unit", "role": "detail", "z": 3.6, "filled": False, "tone": 0.3, "kind": "ellipse", "cx": -0.33, "cy": z, "rx": 0.004, "ry": 0.004})
    return E, F, P


def split_wall(p):
    """🧅️ A merged casing wall split into its outer 1 mm sheet and the lining behind it, for the layered thermal list of the coarse scales."""
    i = min(range(3), key=lambda k: p["s"][k])
    lo = [c - h / 2 for c, h in zip(p["c"], p["s"])]
    hi = [c + h / 2 for c, h in zip(p["c"], p["s"])]
    outer_hi = abs(hi[i]) > abs(lo[i]) if i != 1 else True
    out = []
    for lid in ("lining", "sheet"):
        l2, h2 = list(lo), list(hi)
        if (lid == "sheet") == outer_hi:
            l2[i] = hi[i] - 0.001
        else:
            h2[i] = lo[i] + 0.001
        if lid == "lining":
            if outer_hi:
                h2[i] = hi[i] - 0.001
            else:
                l2[i] = lo[i] + 0.001
        out.append((lid, box(p["id"], p["bone"], "painted-steel" if lid == "sheet" else "acoustic-foam", p["tone"], "casing", l2, h2)))
    return out


def build_scale(scale):
    """🏗️ Body, thermal and structure lists and the three drawings of one authored scale."""
    parts = pallet_parts(scale) + unit_parts(scale)
    solid_parts = [p for p in parts if not p.get("nosolid")]
    body = [solid_of(p) for p in solid_parts]
    with_layers = scale == 50
    thermal = []
    for p in solid_parts:
        if p["region"] == "casing" and not with_layers and p["kind"] == "box":
            for lid, sub in split_wall(p):
                ts = dict(solid_of(sub), id=f"th-{p['id']}-{lid}", region="casing" if lid == "sheet" else "lining")
                if min(sub["s"]) < {100: 0.015, 200: 0.03}[scale]:
                    ts["sheet"] = True
                thermal.append(ts)
            continue
        if p["region"] in HIDDEN_REGIONS:
            ts = dict(solid_of(p), id=f"th-{p['id']}", region=p["region"])
        else:
            ts = {"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]}
        if with_layers and p.get("layer"):
            ts["layer"] = p["layer"]
        thermal.append(ts)
    ph = {50: 0.01, 100: 0.015, 200: 0.03}[scale]
    top = 0.668 if scale != 200 else 0.665
    thermal.append({"id": "th-compressor-plume", "kind": "revolve", "bone": "unit", "material": "compressor-steel", "role": "detail", "tone": 0.5, "profile": [[r6(top - 0.144), 0.06], [r6(top - 0.144 + ph), 0.06]], "axis": [0, 1, 0], "origin": [0.1, 0.0, -0.129], "region": "compressor", "tags": ["compressor-plume"]})
    structure = []
    if scale in (50, 100):
        for i in range(5):
            structure.append({"id": f"member-deck-{i}", "of": f"deck-board-{i}", "member": "pallet", "part": "member"})
        for j in range(3):
            structure.append({"id": f"joint-cross-{j}", "of": f"cross-board-{j}", "member": "pallet", "part": "joint"})
        for i in range(3):
            for j in (range(3) if scale == 50 else [1]):
                structure.append({"id": f"joint-block-{i}-{j}", "of": f"block-{i}-{j}", "member": "pallet", "part": "joint"})
        for i in range(3):
            structure.append({"id": f"support-runner-{i}", "of": f"bottom-board-{i}", "member": "pallet", "part": "support", "anchor": f"pallet-runner-{i}"})
    else:
        structure.append({"id": "member-deck", "of": "deck", "member": "pallet", "part": "member"})
        for i in range(3):
            for j in range(3):
                if j == 1:
                    structure.append({"id": f"support-block-{i}-{j}", "of": f"block-{i}-{j}", "member": "pallet", "part": "support", "anchor": f"pallet-runner-{i}"})
                else:
                    structure.append({"id": f"joint-block-{i}-{j}", "of": f"block-{i}-{j}", "member": "pallet", "part": "joint"})
    structure.append({"id": "joint-carrier", "kind": "box", "bone": "carrier", "material": "galvanised-steel", "role": "body", "tone": 0.3, "size": [0.06, 0.03 if scale == 200 else 0.016, 0.06],
                      "transform": {"translation": [0.0, -0.015 if scale == 200 else -0.008, 0.0]}, "member": "carrier", "part": "joint"})
    if scale == 200:
        structure.append({"id": "member-frame", "of": "plinth", "member": "unit", "part": "member"})
        for i, nm in enumerate(FEET):
            x, z = FOOT_XZ[i]
            structure.append({"id": f"joint-{nm}", "kind": "box", "bone": "unit", "material": "galvanised-steel", "role": "body", "tone": 0.3, "size": [0.04, 0.03, 0.04], "transform": {"translation": [x, 0.015, z]}, "member": "unit", "part": "joint"})
        for nm, x in (("sling-front", 0.33), ("sling-back", -0.33)):
            structure.append({"id": f"joint-{nm}", "kind": "box", "bone": "unit", "material": "galvanised-steel", "role": "body", "tone": 0.3, "size": [0.04, 0.03, 0.52], "transform": {"translation": [x, 0.055, 0.0]}, "member": "unit", "part": "joint"})
    else:
        structure.append({"id": "member-frame-left", "of": "frame-left", "member": "unit", "part": "member"})
        structure.append({"id": "member-frame-right", "of": "frame-right", "member": "unit", "part": "member"})
        structure.append({"id": "joint-sling-front", "of": "frame-front", "member": "unit", "part": "joint"})
        structure.append({"id": "joint-sling-back", "of": "frame-back", "member": "unit", "part": "joint"})
        for i, nm in enumerate(FEET):
            structure.append({"id": f"joint-{nm}", "of": f"foot-{i}", "member": "unit", "part": "joint"})
    mr = {50: 0.03, 100: 0.04, 200: 0.06}[scale]
    structure.append({"id": "mass-unit", "kind": "ellipsoid", "bone": "unit", "material": "galvanised-steel", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr], "transform": {"translation": [UNIT_COM[0], UNIT_COM[1], 0.0]}, "member": "unit", "part": "mass"})
    structure.append({"id": "mass-pallet", "kind": "ellipsoid", "bone": "pallet", "material": "softwood", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr], "transform": {"translation": [PALLET_COM[0], PALLET_COM[1], 0.0]}, "member": "pallet", "part": "mass"})
    geom = {"elevation": [], "front": [], "plan": []}
    for p in parts:
        for v, key in (("e", "elevation"), ("f", "front"), ("p", "plan")):
            if v in p["views"]:
                geom[key].append(shapes_of(p, key))
    for key in geom:
        for k, sh in enumerate(geom[key]):
            sh["z"] = round(sh["z"] + k * 1e-4, 5)
    E, F, P = strokes(scale)
    geom["elevation"] += E
    geom["front"] += F
    geom["plan"] += P
    return parts, body, thermal, structure, geom


UNIT_COM = (0.0, 0.40)
PALLET_COM = (0.0, 0.08)


def mat(mid, en, de, em, sa, rho, k, c, src="literature", ref="incropera-2018", **kw):
    """🧪️ One material."""
    m = {"id": mid, "title": t(en, de), "emissivity": em, "solarAbsorptivity": sa, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c}
    m.update(kw)
    m.update(source=src, reference=ref)
    return m


MATERIALS = [
    mat("softwood", "Weathered softwood (pallet pine, strength class C16)", "Verwittertes Nadelholz (Palettenkiefer, Festigkeitsklasse C16)", 0.9, 0.65, 470, 0.13, 1600, ref="en-338", elasticModulusPa=8.0e9, strengthPa=16.0e6),
    mat("galvanised-steel", "Weathered hot-dip galvanised steel sheet", "Verwittertes feuerverzinktes Stahlblech", 0.28, 0.6, 7850, 50, 460, ref="en-10346", elasticModulusPa=210e9, strengthPa=235e6),
    mat("painted-steel", "Faded white powder-coated steel sheet", "Ausgebleichtes weiß pulverbeschichtetes Stahlblech", 0.9, 0.35, 7850, 50, 460),
    mat("casing-panel", "Casing panel, 1 mm powder-coated steel with 20 mm acoustic lining (merged)", "Gehäuseblech, 1 mm pulverbeschichteter Stahl mit 20 mm Schalldämmung (zusammengefasst)", 0.9, 0.35, 402, 0.037, 470, src="authored-estimate"),
    mat("acoustic-foam", "Polyurethane acoustic lining foam", "Schalldämmschaum aus Polyurethan", 0.9, 0.8, 30, 0.035, 1400, ref="en-iso-10456"),
    mat("compressor-steel", "Black painted compressor shell steel", "Schwarz lackierter Verdichtermantel aus Stahl", 0.9, 0.9, 7850, 50, 460),
    mat("copper", "Oxidised copper tube", "Oxidiertes Kupferrohr", 0.6, 0.7, 8930, 390, 385),
    mat("brass", "Tarnished brass union", "Angelaufene Messingverschraubung", 0.4, 0.6, 8500, 120, 380),
    mat("armaflex", "Black elastomeric pipe insulation", "Schwarze Elastomer-Rohrdämmung", 0.9, 0.9, 65, 0.036, 1500, ref="en-iso-10456"),
    mat("stainless-steel", "Brazed stainless-steel plate heat exchanger", "Gelöteter Edelstahl-Plattenwärmeübertrager", 0.4, 0.5, 7900, 15, 500),
    mat("steel-pipe", "Painted steel pipe", "Lackiertes Stahlrohr", 0.85, 0.7, 7850, 50, 460),
    mat("cast-iron", "Cast-iron pump housing", "Pumpengehäuse aus Grauguss", 0.8, 0.7, 7200, 50, 460),
    mat("polymer", "Grey ABS and polycarbonate housing", "Graues Gehäuse aus ABS und Polycarbonat", 0.9, 0.5, 1100, 0.2, 1400),
    mat("rubber", "Rubber foot pad", "Gummifuß", 0.92, 0.9, 1200, 0.2, 2000),
    mat("stainless-hose", "Corrugated stainless-steel hose", "Edelstahl-Wellschlauch", 0.4, 0.5, 7900, 15, 500),
    mat("aluminium", "Anodised aluminium type plate", "Eloxiertes Aluminium-Typenschild", 0.2, 0.3, 2700, 200, 900),
]

HEAT_ROWS = [{"v": 1, "supplyTempC": 35, "returnTempC": 30}]
BRINE_ROWS = [{"v": 1, "supplyTempC": 0, "returnTempC": -3}]


def layer(lid, en, de, kind, m, th, r, hc, src="authored-estimate", ref="incropera-2018"):
    """🧶️ One thermal layer."""
    return {"id": lid, "title": t(en, de), "kind": kind, "material": m, "thicknessM": th, "resistanceM2KW": r, "heatCapacityJM2K": hc, "source": src, "reference": ref}


AREAS = {"casing": 5.0, "lining": 4.8, "base-frame": 1.6, "compressor": 0.42, "evaporator": 0.35, "condenser": 0.27, "heating-lines": 0.12, "brine-lines": 0.2, "pumps": 0.26, "controls": 0.28, "pallet": 1.95}


HIDDEN = t("Inside the unit behind the closed right casing panel, so the side elevation cannot show it; the front elevation draws it through the open front.", "Im Gerät hinter dem geschlossenen rechten Gehäuseblech, daher in der Seitenansicht nicht sichtbar; die Vorderansicht zeichnet es durch die offene Front.")


HIDDEN_REGIONS = {"compressor", "evaporator", "condenser", "pumps", "controls", "lining"}


def regions():
    """🌡️ The thermal regions with their layers, models and nameplates."""
    R = []
    R.append({"id": "casing", "title": t("Casing panels, front panel removed (outer and inner face)", "Gehäusebleche, Frontblech abgenommen (Außen- und Innenseite)"), "kind": "surface", "model": "passive", "material": "painted-steel",
              "areaM2": AREAS["casing"], "characteristicLengthM": 1.0, "geometry": "vertical-plate", "skyViewFactor": 0.3,
              "layers": [layer("sheet", "Powder-coated steel sheet 1 mm", "Pulverbeschichtetes Stahlblech 1 mm", "metal", "painted-steel", 0.001, 0.00002, 3610)],
              "heatCapacityJM2K": 3610, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "en-iso-10456"})
    R.append({"id": "lining", "title": t("Acoustic lining glued inside the casing", "Innen ins Gehäuse geklebte Schalldämmung"), "kind": "surface", "model": "passive", "material": "acoustic-foam",
              "hidden": HIDDEN, "areaM2": AREAS["lining"], "characteristicLengthM": 1.0, "geometry": "vertical-plate", "skyViewFactor": 0.1,
              "layers": [layer("foam", "Polyurethane acoustic foam 20 mm", "Polyurethan-Schalldämmschaum 20 mm", "insulation", "acoustic-foam", 0.02, 0.57, 840, ref="en-iso-10456")],
              "heatCapacityJM2K": 840, "source": "authored-estimate", "reference": "en-iso-10456"})
    R.append({"id": "base-frame", "title": t("Galvanised base frame, base plate, uprights, brackets and rubber feet", "Verzinkter Grundrahmen, Bodenblech, Stützen, Konsolen und Gummifüße"), "kind": "surface", "model": "passive", "material": "galvanised-steel",
              "areaM2": AREAS["base-frame"], "characteristicLengthM": 0.3, "geometry": "vertical-plate", "skyViewFactor": 0.1,
              "layers": [layer("steel", "Galvanised steel 3 mm", "Verzinkter Stahl 3 mm", "metal", "galvanised-steel", 0.003, 0.00006, 10830)], "heatCapacityJM2K": 10830, "source": "authored-estimate", "reference": "en-10346"})
    R.append({"id": "compressor", "title": t("Scroll compressor with refrigerant lines (R410A recovered in storage)", "Scrollverdichter mit Kältemittelleitungen (R410A im Lager abgesaugt)"), "kind": "engine", "model": "operating", "material": "compressor-steel",
              "hidden": HIDDEN, "areaM2": AREAS["compressor"], "characteristicLengthM": 0.24, "geometry": "cylinder", "skyViewFactor": 0.1,
              "layers": [layer("shell", "Compressor shell steel 4 mm", "Verdichtermantel Stahl 4 mm", "metal", "compressor-steel", 0.004, 0.00008, 14440)], "heatCapacityJM2K": 14440,
              "operating": {"key": "compressor", "heatW": 110}, "ratedPowerW": 2300, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "en-14511"})
    R.append({"id": "evaporator", "title": t("Evaporator plate heat exchanger in its insulation (brine side)", "Verdampfer-Plattenwärmeübertrager in seiner Dämmung (Soleseite)"), "kind": "surface", "model": "hydronic", "material": "armaflex",
              "hidden": HIDDEN, "areaM2": AREAS["evaporator"], "characteristicLengthM": 0.52, "geometry": "vertical-plate", "skyViewFactor": 0.1,
              "layers": [layer("plates", "Brazed stainless plate pack, 0.4 mm plates", "Gelötetes Edelstahl-Plattenpaket, Platten 0,4 mm", "metal", "stainless-steel", 0.004, 0.0003, 15800),
                         layer("insulation", "Elastomeric insulation 13 mm", "Elastomerdämmung 13 mm", "insulation", "armaflex", 0.013, 0.36, 1270, ref="en-iso-10456")], "heatCapacityJM2K": 17070,
              "hydronic": {"key": "brine-circuit", "rows": BRINE_ROWS, "flowPosition": 0.5, "filmM2KW": 0.004}, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "vdi-4640"})
    R.append({"id": "condenser", "title": t("Condenser plate heat exchanger (heating side), limescale traces", "Verflüssiger-Plattenwärmeübertrager (Heizungsseite), Kalkspuren"), "kind": "heater", "model": "hydronic", "material": "stainless-steel",
              "hidden": HIDDEN, "areaM2": AREAS["condenser"], "characteristicLengthM": 0.46, "geometry": "vertical-plate", "skyViewFactor": 0.1, "layers": [], "heatCapacityJM2K": 9000,
              "hydronic": {"key": "heating-circuit", "rows": HEAT_ROWS, "flowPosition": 0.3, "filmM2KW": 0.003}, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "en-14511"})
    R.append({"id": "heating-lines", "title": t("Heating risers, unions and cut flexible hoses", "Heizungssteigleitungen, Verschraubungen und abgeschnittene Flexschläuche"), "kind": "surface", "model": "hydronic", "material": "steel-pipe",
              "areaM2": AREAS["heating-lines"], "characteristicLengthM": 0.028, "geometry": "cylinder", "skyViewFactor": 0.2, "layers": [], "heatCapacityJM2K": 8000,
              "hydronic": {"key": "heating-circuit", "rows": HEAT_ROWS, "flowPosition": 0.5, "filmM2KW": 0.002}, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "en-14511"})
    R.append({"id": "brine-lines", "title": t("Insulated brine risers, unions and cut flexible hoses", "Gedämmte Solesteigleitungen, Verschraubungen und abgeschnittene Flexschläuche"), "kind": "surface", "model": "hydronic", "material": "armaflex",
              "areaM2": AREAS["brine-lines"], "characteristicLengthM": 0.05, "geometry": "cylinder", "skyViewFactor": 0.2,
              "layers": [layer("insulation", "Elastomeric insulation 13 mm", "Elastomerdämmung 13 mm", "insulation", "armaflex", 0.013, 0.36, 1270, ref="en-iso-10456")], "heatCapacityJM2K": 1270,
              "hydronic": {"key": "brine-circuit", "rows": BRINE_ROWS, "flowPosition": 0.5, "filmM2KW": 0.004}, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "vdi-4640"})
    R.append({"id": "pumps", "title": t("Brine and heating circulators", "Sole- und Heizungsumwälzpumpe"), "kind": "engine", "model": "operating", "material": "cast-iron",
              "hidden": HIDDEN, "areaM2": AREAS["pumps"], "characteristicLengthM": 0.1, "geometry": "cylinder", "skyViewFactor": 0.1,
              "layers": [layer("housing", "Cast iron 5 mm", "Grauguss 5 mm", "metal", "cast-iron", 0.005, 0.0001, 16560)], "heatCapacityJM2K": 16560,
              "operating": {"key": "pumps", "heatW": 45}, "ratedPowerW": 160, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "en-16297"})
    R.append({"id": "controls", "title": t("Switch box with controller and display, cut supply cable", "Schaltkasten mit Regler und Display, abgeschnittenes Netzkabel"), "kind": "surface", "model": "operating", "material": "polymer",
              "hidden": HIDDEN, "areaM2": AREAS["controls"], "characteristicLengthM": 0.17, "geometry": "vertical-plate", "skyViewFactor": 0.2,
              "layers": [layer("housing", "ABS housing 3 mm", "ABS-Gehäuse 3 mm", "polymer", "polymer", 0.003, 0.015, 4620)], "heatCapacityJM2K": 4620,
              "operating": {"key": "controls", "heatW": 12}, "ratedPowerW": 15, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "incropera-2018"})
    R.append({"id": "pallet", "title": t("Half pallet and chocks, weathered softwood", "Halbpalette und Keile, verwittertes Nadelholz"), "kind": "surface", "model": "passive", "material": "softwood",
              "areaM2": AREAS["pallet"], "characteristicLengthM": 0.1, "geometry": "horizontal-plate-up", "skyViewFactor": 0.3, "layers": [], "heatCapacityJM2K": 16500,
              "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "en-iso-13786"})
    return R


OPERATING = [
    {"id": "compressor", "title": t("Compressor", "Verdichter"), "levels": [{"v": 0, "title": t("Off", "Aus")}, {"v": 1, "title": t("Rated, B0/W35 (10.2 kW heat, 2.3 kW input)", "Nennbetrieb, B0/W35 (10,2 kW Wärme, 2,3 kW Aufnahme)")}]},
    {"id": "pumps", "title": t("Brine and heating circulators", "Sole- und Heizungsumwälzpumpe"), "levels": [{"v": 0, "title": t("Off", "Aus")}, {"v": 1, "title": t("Running (160 W)", "In Betrieb (160 W)")}]},
    {"id": "controls", "title": t("Controller", "Regler"), "levels": [{"v": 0, "title": t("Disconnected", "Abgeklemmt")}, {"v": 1, "title": t("On (15 W)", "An (15 W)")}]},
    {"id": "heating-circuit", "title": t("Heating water through the condenser", "Heizwasser durch den Verflüssiger"), "levels": [{"v": 0, "title": t("Drained or still", "Entleert oder ruhend")}, {"v": 1, "title": t("Design flow 35/30 degC", "Auslegungsdurchfluss 35/30 °C")}]},
    {"id": "brine-circuit", "title": t("Brine through the evaporator", "Sole durch den Verdampfer"), "levels": [{"v": 0, "title": t("Drained or still", "Entleert oder ruhend")}, {"v": 1, "title": t("Design flow 0/-3 degC", "Auslegungsdurchfluss 0/-3 °C")}]},
]

REGION_IDS = ["casing", "lining", "base-frame", "compressor", "evaporator", "condenser", "heating-lines", "brine-lines", "pumps", "controls", "pallet"]


def binding(levels, plume):
    """🌬️ A thermal binding of every region and emitter."""
    regs = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGION_IDS}
    regs["pallet"] = {"exposedFraction": 0.85, "sunlitFraction": 0, "contact": {"fraction": 0.15, "tempC": 15}}
    regs["base-frame"] = {"exposedFraction": 0.95, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 15}}
    return {"operating": levels, "regions": regs, "emitters": {"compressor-plume": plume}}


OFF = {"compressor": 0, "pumps": 0, "controls": 0, "heating-circuit": 0, "brine-circuit": 0}
RUN = {"compressor": 1, "pumps": 1, "controls": 1, "heating-circuit": 1, "brine-circuit": 1}
STANDBY = {"compressor": 0, "pumps": 0, "controls": 1, "heating-circuit": 0, "brine-circuit": 0}
RUNNERS = ["pallet-runner-0", "pallet-runner-1", "pallet-runner-2"]
FEET = ["foot-front-left", "foot-front-right", "foot-back-left", "foot-back-right"]
SLINGS = ["sling-front", "sling-back"]
FILL = {"id": "media-fill", "kind": "carried", "title": t("Heating water and brine filled again, 11 kg", "Heizwasser und Sole wieder gefüllt, 11 kg"), "bone": "unit", "x": -0.1, "y": 0.45, "fxN": 0, "fyN": -107.9, "source": "authored-estimate", "reference": "en-14511"}
REINSTALLED = {"carrier": {"dx": 1.0}, "unit": {"dy": -0.144}}

POSES = [
    {"id": "stored", "title": t("Stored on the half pallet", "Auf der Halbpalette gelagert"),
     "description": t("Drained of heating water and brine, refrigerant recovered, unions capped: the unit stands on its four rubber feet on a half pallet between two nailed chocks, the front panel taken off so that buyers see the compressor, the plate heat exchangers and the circulators.",
                      "Heizwasser und Sole abgelassen, Kältemittel abgesaugt, Verschraubungen verschlossen: das Gerät steht auf seinen vier Gummifüßen zwischen zwei angenagelten Keilen auf einer Halbpalette, das Frontblech ist abgenommen, damit Käufer Verdichter, Plattenwärmeübertrager und Umwälzpumpen sehen."),
     "bones": {}, "ground": "authored", "contacts": RUNNERS, "thermal": binding(OFF, 0), "loads": []},
    {"id": "lifted", "title": t("Lifted off the pallet", "Von der Palette gehoben"),
     "description": t("Two round slings under the front and back channels of the base frame lift the 132 kg unit 0.3 m off the pallet; pallet and chocks stay on the ground.",
                      "Zwei Rundschlingen unter dem vorderen und hinteren Profil des Grundrahmens heben das 132 kg schwere Gerät 0,3 m von der Palette; Palette und Keile bleiben am Boden."),
     "bones": {"unit": {"dy": 0.3}}, "ground": "authored", "contacts": RUNNERS + SLINGS, "thermal": binding(OFF, 0), "loads": []},
    {"id": "reinstalled", "title": t("Reinstalled and heating (B0/W35)", "Wieder eingebaut und heizend (B0/W35)"),
     "description": t("Back in a plant room beside its empty pallet, refilled and connected with new hoses: the compressor runs at its rated point B0/W35, brine enters the evaporator at 0 degC and leaves at -3 degC, the heating water leaves the condenser at 35 degC and returns at 30 degC.",
                      "Wieder in einem Technikraum neben seiner leeren Palette, neu befüllt und mit neuen Schläuchen angeschlossen: der Verdichter läuft im Nennpunkt B0/W35, die Sole tritt mit 0 °C in den Verdampfer ein und mit -3 °C aus, das Heizwasser verlässt den Verflüssiger mit 35 °C und kehrt mit 30 °C zurück."),
     "bones": REINSTALLED, "ground": "authored", "contacts": RUNNERS + FEET, "thermal": binding(RUN, 1), "loads": [FILL]},
    {"id": "reinstalled-off", "title": t("Reinstalled, standby", "Wieder eingebaut, Bereitschaft"),
     "description": t("Reinstalled and filled, the controller is on and waits for heat demand: compressor and circulators stand still, both circuits are at room temperature.",
                      "Wieder eingebaut und befüllt, der Regler ist an und wartet auf Wärmebedarf: Verdichter und Umwälzpumpen stehen still, beide Kreise haben Raumtemperatur."),
     "bones": REINSTALLED, "ground": "authored", "contacts": RUNNERS + FEET, "thermal": binding(STANDBY, 0), "loads": [FILL]},
]

CLIPS = [
    {"id": "lift", "title": t("Lift off the pallet", "Von der Palette heben"),
     "description": t("A hoist takes the unit up on two round slings under its base frame and lifts it 0.3 m clear of the pallet and chocks within six seconds.",
                      "Ein Hebezeug nimmt das Gerät an zwei Rundschlingen unter dem Grundrahmen auf und hebt es innerhalb von sechs Sekunden 0,3 m über Palette und Keile."),
     "durationS": 6, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in RUNNERS + SLINGS],
     "phases": [{"id": "lift", "fromS": 0, "toS": 6, "title": t("Lifting", "Heben")}],
     "events": [{"t": 0, "kind": "contact-off", "title": t("Feet leave the pallet deck", "Füße verlassen das Palettendeck"), "target": "unit"}, {"t": 6, "kind": "motion", "title": t("Unit hangs clear", "Gerät hängt frei"), "target": "unit"}],
     "tracks": []},
    {"id": "start-up", "title": t("Start-up after reinstallation", "Inbetriebnahme nach dem Wiedereinbau"),
     "description": t("The reinstalled unit receives heat demand: the circulators start after 15 s, the compressor after 30 s; the brine cools to its design temperatures within five minutes and the heating water warms to 35/30 degC within ten minutes.",
                      "Das wieder eingebaute Gerät erhält Wärmebedarf: die Umwälzpumpen starten nach 15 s, der Verdichter nach 30 s; die Sole kühlt innerhalb von fünf Minuten auf ihre Auslegungstemperaturen ab und das Heizwasser erwärmt sich innerhalb von zehn Minuten auf 35/30 °C."),
     "durationS": 900, "fps": 1, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 30, "pose": "reinstalled", "ease": "step"}, {"t": 900, "pose": "reinstalled", "ease": "step"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 900} for a in RUNNERS + FEET],
     "phases": [{"id": "standby", "fromS": 0, "toS": 15, "title": t("Standby", "Bereitschaft")}, {"id": "pumps", "fromS": 15, "toS": 30, "title": t("Circulators pre-run", "Pumpenvorlauf")},
                {"id": "warm-up", "fromS": 30, "toS": 600, "title": t("Warm-up", "Aufheizen")}, {"id": "steady", "fromS": 600, "toS": 900, "title": t("Steady operation", "Beharrungsbetrieb")}],
     "events": [{"t": 15, "kind": "operating", "title": t("Circulators start", "Umwälzpumpen starten"), "target": "pumps"}, {"t": 30, "kind": "operating", "title": t("Compressor starts", "Verdichter startet"), "target": "compressor"}],
     "tracks": [{"target": "operating:controls", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 900, "v": 1}]},
                {"target": "operating:pumps", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 15, "v": 1}, {"t": 900, "v": 1}]},
                {"target": "operating:compressor", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 1}, {"t": 900, "v": 1}]},
                {"target": "operating:brine-circuit", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 330, "v": 1}, {"t": 900, "v": 1}]},
                {"target": "operating:heating-circuit", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 600, "v": 1}, {"t": 900, "v": 1}]},
                {"target": "emitter:compressor-plume", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 600, "v": 1}, {"t": 900, "v": 1}]}]},
]


def anchors():
    """📍️ Ground contacts of the pallet, the four feet, the two sling points and the plume source."""
    A = []
    for i, z in enumerate([-0.25, 0.0, 0.25]):
        A.append({"id": f"pallet-runner-{i}", "bone": "pallet", "x": 0, "y": 0, "z": z, "kind": "contact", "title": t(f"Pallet runner {i + 1} on the ground", f"Palettenkufe {i + 1} am Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.8, "footprintWidthM": 0.1, "frictionCoefficient": 0.5})
    names = {"foot-front-left": ("Front left rubber foot", "Gummifuß vorne links"), "foot-front-right": ("Front right rubber foot", "Gummifuß vorne rechts"), "foot-back-left": ("Back left rubber foot", "Gummifuß hinten links"), "foot-back-right": ("Back right rubber foot", "Gummifuß hinten rechts")}
    for nm, (x, z) in zip(FEET, FOOT_XZ):
        A.append({"id": nm, "bone": "unit", "x": x, "y": 0, "z": z, "kind": "contact", "title": t(*names[nm]), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.04, "frictionCoefficient": 0.6})
    A.append({"id": "sling-front", "bone": "unit", "x": 0.33, "y": 0.055, "z": 0, "kind": "support", "title": t("Front round sling under the front channel of the base frame", "Vordere Rundschlinge unter dem vorderen Profil des Grundrahmens"), "support": "pin", "footprintLengthM": 0.04, "footprintWidthM": 0.52})
    A.append({"id": "sling-back", "bone": "unit", "x": -0.33, "y": 0.055, "z": 0, "kind": "support", "title": t("Back round sling under the back channel of the base frame", "Hintere Rundschlinge unter dem hinteren Profil des Grundrahmens"), "support": "pin", "footprintLengthM": 0.04, "footprintWidthM": 0.52})
    A.append({"id": "compressor-top", "bone": "unit", "x": 0.1, "y": 0.524, "z": -0.129, "kind": "emitter", "title": t("Dome of the compressor", "Kuppel des Verdichters")})
    return A


PALLET_KG = round(470 * (3 * 0.8 * 0.1 * 0.022 + 9 * 0.1 * 0.1 * 0.078 + 3 * 0.6 * 0.1 * 0.022 + 5 * 0.8 * 0.1 * 0.022 + 2 * 0.04 * 0.04 * 0.5), 2)
UNIT_KG = 132.0
RIG = {"kind": "mechanical", "root": "pallet", "bones": [
    {"id": "pallet", "parent": None, "title": t("Half pallet with chocks", "Halbpalette mit Keilen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.4, "joint": {"type": "fixed"},
     "mass": {"kg": PALLET_KG, "comX": PALLET_COM[0], "comY": PALLET_COM[1], "distribution": "point", "source": "authored-estimate", "reference": "en-338"},
     "section": {"areaM2": 0.011, "secondMomentM4": 4.44e-7, "extremeFibreM": 0.011, "material": "softwood", "title": t("Five deck boards 100 x 22 mm", "Fünf Deckbretter 100 x 22 mm")}},
    {"id": "carrier", "parent": "pallet", "title": t("Placement carrier (massless, moves the unit beside the pallet)", "Platzierungsträger (masselos, setzt das Gerät neben die Palette)"), "x": 0, "y": 0.144, "angleDeg": 0, "lengthM": 0,
     "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.2}},
    {"id": "unit", "parent": "carrier", "title": t("Heat pump on its base frame", "Wärmepumpe auf ihrem Grundrahmen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.35,
     "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.144, "maxM": 0.6},
     "mass": {"kg": UNIT_KG, "comX": UNIT_COM[0], "comY": UNIT_COM[1], "distribution": "point", "source": "authored-estimate", "reference": "manufacturer-typical"},
     "section": {"areaM2": 0.00084, "secondMomentM4": 4.98e-7, "extremeFibreM": 0.03, "material": "galvanised-steel", "title": t("Two base-frame channels C 60 x 40 x 3", "Zwei Grundrahmenprofile C 60 x 40 x 3")}},
]}

REFS = [
    {"id": "manufacturer-typical", "citation": "Data sheets of 10 kW indoor brine-to-water heat pumps of the 2008 to 2018 generation (Stiebel Eltron WPF 10, Viessmann Vitocal 300-G, NIBE F1145-10 class): casing about 600 x 1150 x 650 to 700 mm, 120 to 150 kg, about 10 kW heating output at B0/W35 with about 2.2 to 2.4 kW electrical input, scroll compressor, two brazed plate heat exchangers, integrated brine and heating circulators; values rounded to the class, not those of one product."},
    {"id": "en-14511", "citation": "EN 14511-2:2018 Air conditioners, liquid chilling packages and heat pumps, test conditions: brine/water rating point B0/W35 (brine 0 degC in, heating water 35 degC out, 5 K spread on the heating side, 3 K on the brine side)."},
    {"id": "vdi-4640", "citation": "VDI 4640 Blatt 2:2019 Thermal use of the underground, ground source heat pump systems (brine design temperatures around 0 degC, monoethylene glycol 25 %)."},
    {"id": "en-378", "citation": "EN 378-4:2016 Refrigerating systems and heat pumps, operation, maintenance, repair and recovery: refrigerant recovered before dismantling, circuit left with a holding charge of dry nitrogen."},
    {"id": "en-16297", "citation": "EN 16297-1:2012 Pumps, glandless circulators, energy efficiency index; typical electrical input of brine and heating circulators of 10 kW heat pumps 60 to 100 W each."},
    {"id": "epal-half-pallet", "citation": "EPAL half pallet (EPAL 6) 800 x 600 x 144 mm with nine blocks, three bottom boards, three stringer boards and five deck boards of 22 mm; construction after UIC 435-2 and EN 13698-1:2003 for the Euro pallet; mass about 9.5 to 12 kg."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C16 mean density 370 kg/m3 dry (pallet pine at 15 to 20 % moisture about 470 kg/m3), E0,mean 8 GPa, fm,k 16 MPa."},
    {"id": "en-10346", "citation": "EN 10346:2015 Continuously hot-dip coated steel flat products (galvanised base frame of S235 class steel); weathered zinc emissivity about 0.25 to 0.3."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (polyurethane foam 0.035 W/mK, elastomeric foam 0.036 W/mK, softwood 0.13 W/mK, 1600 J/kgK; steel 50 W/mK, 450 J/kgK)."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics (effective storage depth of exposed timber faces)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of painted, galvanised, oxidised and polished metals, convection from cylinders and plates)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (line groups)."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on cuts and sections (timber grain hatch)."},
]

PENS = {50: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}
HATCHES = {50: [{"id": "grain", "title": t("Timber, grain along the boards", "Holz, Faser längs der Bretter"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}], 100: [], 200: []}

DECISIONS = {
    50: {
        "silhouette": t("Side elevation 16 x 28.4 mm on paper: the closed right casing panel with its folds, the type plate and the old hook slots of the front panel, the galvanised base frame on two rubber feet, the four cut hose stubs with their caps above the lid, the half pallet with its blocks, boards and chocks below. Front elevation 12 x 28.4 mm: the open front shows the black scroll compressor on its mounting base, the insulated evaporator and the condenser behind, both circulators with their motors facing the viewer, the risers, the switch box at the top and the lined casing around it. Plan 16 x 12 mm: the lid with its seam and screws, the hoses with caps and the deck boards and chocks around it.",
                            "Seitenansicht 16 x 28,4 mm auf Papier: das geschlossene rechte Gehäuseblech mit Kanten, Typenschild und den alten Hakenschlitzen des Frontblechs, der verzinkte Grundrahmen auf zwei Gummifüßen, oben die vier abgeschnittenen Schlauchstutzen mit Kappen, unten die Halbpalette mit Klötzen, Brettern und Keilen. Vorderansicht 12 x 28,4 mm: die offene Front zeigt den schwarzen Scrollverdichter auf seinem Sockel, dahinter den gedämmten Verdampfer und den Verflüssiger, beide Umwälzpumpen mit den Motoren zum Betrachter, die Steigleitungen, oben den Schaltkasten und ringsum das gedämmte Gehäuse. Grundriss 16 x 12 mm: der Deckel mit Fuge und Schrauben, die Schläuche mit Kappen und ringsum Deckbretter und Keile."),
        "retained": t("Every board, block and chock of the pallet; feet, frame channels, base plate, uprights and brackets; the casing as 1 mm sheet with its 20 mm lining; compressor with dome, terminal box, discharge, suction and liquid lines and the expansion valve; both plate heat exchangers; both circulators with housing, motor and terminal box; four risers, four unions, four hoses with caps; switch box with display; cut supply cable; type plate.",
                          "Jedes Brett, jeder Klotz und Keil der Palette; Füße, Rahmenprofile, Bodenblech, Stützen und Konsolen; das Gehäuse als 1-mm-Blech mit seiner 20-mm-Dämmung; Verdichter mit Kuppel, Klemmenkasten, Druck-, Saug- und Flüssigkeitsleitung und Expansionsventil; beide Plattenwärmeübertrager; beide Umwälzpumpen mit Gehäuse, Motor und Klemmenkasten; vier Steigleitungen, vier Verschraubungen, vier Schläuche mit Kappen; Schaltkasten mit Display; abgeschnittenes Netzkabel; Typenschild."),
        "merged": t("Plate packs, ports and the inner wiring are inside their housings; the brine hose insulation and its core are two cylinders.", "Plattenpakete, Anschlüsse und die innere Verdrahtung liegen in ihren Gehäusen; Dämmung und Kern der Soleschläuche sind zwei Zylinder."),
        "omitted": t("Screws of the pallet, nails of the chocks, cable ties and the inner contactors of the switch box.", "Schrauben der Palette, Nägel der Keile, Kabelbinder und die inneren Schütze des Schaltkastens."),
        "thinElements": t("The 1 mm sheets and the 4 mm base plate are declared sheets; the front view draws each sheet with its lining as one 21 mm strip.", "Die 1-mm-Bleche und das 4-mm-Bodenblech sind erklärte Bleche; die Vorderansicht zeichnet jedes Blech mit seiner Dämmung als einen 21-mm-Streifen."),
        "hatching": t("Grain lines on the deck boards in plan (DIN ISO 128-50).", "Faserlinien auf den Deckbrettern im Grundriss (DIN ISO 128-50)."),
        "styles": t("Outline 0.25 mm, details 0.13 mm; the black compressor and insulation dark, the faded casing and the galvanised steel light, the weathered pallet mid grey.", "Kontur 0,25 mm, Details 0,13 mm; schwarzer Verdichter und Dämmung dunkel, ausgebleichtes Gehäuse und verzinkter Stahl hell, verwitterte Palette mittelgrau."),
        "overlays": t("Every part is its own cell so that the thermal view colours casing, compressor, both circuits and the pallet apart.", "Jedes Teil ist eine eigene Zelle, damit die Thermoansicht Gehäuse, Verdichter, beide Kreise und die Palette getrennt einfärbt."),
    },
    100: {
        "silhouette": t("Side elevation 8 x 14.2 mm: casing panel, base frame on its feet, hose stubs and the pallet with its fork openings. Front 6 x 14.2 mm: compressor, heat exchangers, circulators, risers and switch box inside the lined casing. Plan 8 x 6 mm: lid, hoses and the deck strips around it.",
                            "Seitenansicht 8 x 14,2 mm: Gehäuseblech, Grundrahmen auf Füßen, Schlauchstutzen und die Palette mit ihren Gabeleinfahrten. Vorderansicht 6 x 14,2 mm: Verdichter, Wärmeübertrager, Umwälzpumpen, Steigleitungen und Schaltkasten im gedämmten Gehäuse. Grundriss 8 x 6 mm: Deckel, Schläuche und die Deckstreifen ringsum."),
        "retained": t("Pallet boards and blocks, chocks, feet, frame, casing panels as 21 mm walls, compressor with terminal box, discharge and suction lines, both heat exchangers, both circulators, risers, unions, hoses and caps, switch box.",
                          "Palettenbretter und Klötze, Keile, Füße, Rahmen, Gehäusebleche als 21-mm-Wände, Verdichter mit Klemmenkasten, Druck- und Saugleitung, beide Wärmeübertrager, beide Umwälzpumpen, Steigleitungen, Verschraubungen, Schläuche und Kappen, Schaltkasten."),
        "merged": t("Sheet and lining are one 21 mm wall; the brine hose core is merged into its insulation.", "Blech und Dämmung sind eine 21-mm-Wand; der Kern der Soleschläuche ist in seine Dämmung einbezogen."),
        "omitted": t("Liquid line and expansion valve (10 mm), pump terminal boxes, display, cable stub, type plate and the hatch.", "Flüssigkeitsleitung und Expansionsventil (10 mm), Klemmenkästen der Pumpen, Display, Kabelstummel, Typenschild und die Schraffur."),
        "thinElements": t("The 4 mm base plate stays a declared sheet.", "Das 4-mm-Bodenblech bleibt ein erklärtes Blech."),
        "hatching": t("No hatch at this scale.", "Keine Schraffur in diesem Maßstab."),
        "styles": t("Outline 0.25 mm, three detail lines 0.13 mm.", "Kontur 0,25 mm, drei Detaillinien 0,13 mm."),
        "overlays": t("One cell per part as at 1:50.", "Eine Zelle je Teil wie bei 1:50."),
    },
    200: {
        "silhouette": t("Side elevation 4 x 7.1 mm: the casing block on its plinth, the four hose stubs as two bars and the pallet as deck and blocks. Front 3 x 7.1 mm: the open casing with the compressor, the two heat exchangers, the two pump motors and the switch box as masses. Plan 4 x 3 mm: lid, hoses and the deck strips.",
                            "Seitenansicht 4 x 7,1 mm: der Gehäuseblock auf seinem Sockel, die vier Schlauchstutzen und die Palette als Deck und Klötze. Vorderansicht 3 x 7,1 mm: das offene Gehäuse mit Verdichter, den zwei Wärmeübertragern, den zwei Pumpenmotoren und dem Schaltkasten als Massen. Grundriss 4 x 3 mm: Deckel, Schläuche und die Deckstreifen."),
        "retained": t("Pallet deck, nine blocks, chocks, plinth, casing walls, compressor, both heat exchangers with brackets, risers, both pumps, switch box, four hose stubs.", "Palettendeck, neun Klötze, Keile, Sockel, Gehäusewände, Verdichter, beide Wärmeübertrager mit Konsolen, Steigleitungen, beide Pumpen, Schaltkasten, vier Schlauchstutzen."),
        "merged": t("Deck and stringer boards are one 44 mm deck, bottom boards are merged into the blocks, feet and frame into an 85 mm plinth, pump housing and motor into one drum, union, hose and cap into one stub.",
                        "Deck- und Querbretter sind ein 44-mm-Deck, die Bodenbretter sind in die Klötze einbezogen, Füße und Rahmen in einen 85-mm-Sockel, Pumpengehäuse und Motor in eine Trommel, Verschraubung, Schlauch und Kappe in einen Stutzen."),
        "omitted": t("Refrigerant lines, terminal boxes, all detail strokes.", "Kältemittelleitungen, Klemmenkästen, alle Detailstriche."),
        "thinElements": t("The 21 mm casing walls and the 4 mm base plate are declared sheets.", "Die 21-mm-Gehäusewände und das 4-mm-Bodenblech sind erklärte Bleche."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm only.", "Nur Kontur 0,18 mm."),
        "overlays": t("One cell per mass.", "Eine Zelle je Masse."),
    },
}

VOL_DECISIONS = {
    50: [t("Body: every part as it is: the half pallet board by board with its nine blocks and two nailed chocks, the unit on four rubber feet and a galvanised channel frame, the casing as 1 mm sheets with 20 mm polyurethane lining open at the front, the scroll compressor revolved with its dome, both plate heat exchangers, both circulators, the risers, unions, cut hoses with their caps, the switch box and the cut supply cable.",
           "Körper: jedes Teil, wie es ist: die Halbpalette Brett für Brett mit ihren neun Klötzen und zwei angenagelten Keilen, das Gerät auf vier Gummifüßen und einem verzinkten Profilrahmen, das Gehäuse als 1-mm-Bleche mit 20 mm Polyurethan-Dämmung, vorne offen, der Scrollverdichter mit Kuppel gedreht, beide Plattenwärmeübertrager, beide Umwälzpumpen, die Steigleitungen, Verschraubungen, abgeschnittenen Schläuche mit Kappen, der Schaltkasten und das abgeschnittene Netzkabel."),
         t("Thermal: every body solid is a thermal solid of its region; the casing is two nested layers, lining inside the sheet. Structure: deck boards are the pallet member, stringer boards and blocks its joints, the bottom boards the supports at the three runner contacts; the side channels are the member of the unit, the front and back channels the sling supports, the feet the floor supports.",
           "Thermik: jeder Körper ist ein Thermokörper seiner Region; das Gehäuse besteht aus zwei verschachtelten Lagen, die Dämmung innen am Blech. Tragwerk: die Deckbretter sind das Palettenglied, Querbretter und Klötze seine Knoten, die Bodenbretter die Auflager an den drei Kufenkontakten; die Seitenprofile sind das Glied des Geräts, vorderes und hinteres Profil die Schlingenauflager, die Füße die Bodenauflager.")],
    100: [t("Body: a new model for the 15 mm feature: sheet and lining are one 21 mm wall, the dome is coarser, liquid line, expansion valve, pump terminal boxes, display, cable stub and type plate are left out, the brine hose core is merged into its insulation.",
            "Körper: ein neues Modell für das Merkmal von 15 mm: Blech und Dämmung sind eine 21-mm-Wand, die Kuppel ist gröber, Flüssigkeitsleitung, Expansionsventil, Klemmenkästen der Pumpen, Display, Kabelstummel und Typenschild entfallen, der Kern der Soleschläuche ist in die Dämmung einbezogen."),
          t("Thermal and structure as at 1:50 on the solids of this scale; the casing is one solid per wall.", "Thermik und Tragwerk wie bei 1:50 auf den Körpern dieses Maßstabs; das Gehäuse ist ein Körper je Wand.")],
    200: [t("Body: a new abstraction for the 30 mm feature: deck and stringer boards are one 44 mm deck on nine 100 mm blocks, feet and frame one 85 mm plinth, the compressor a plain drum, each pump one drum, each connection one stub; the refrigerant lines and terminal boxes are below the feature and left out; the casing walls are declared 21 mm sheets.",
            "Körper: eine neue Abstraktion für das Merkmal von 30 mm: Deck- und Querbretter sind ein 44-mm-Deck auf neun Klötzen von 100 mm, Füße und Rahmen ein 85-mm-Sockel, der Verdichter eine einfache Trommel, jede Pumpe eine Trommel, jeder Anschluss ein Stutzen; Kältemittelleitungen und Klemmenkästen liegen unter dem Merkmal und entfallen; die Gehäusewände sind erklärte 21-mm-Bleche."),
          t("Structure: the deck is the pallet member, the three middle blocks the supports, the others joints; the plinth is the member of the unit with support pads at the feet and sling points.", "Tragwerk: das Deck ist das Palettenglied, die drei mittleren Klötze die Auflager, die übrigen Knoten; der Sockel ist das Glied des Geräts mit Auflagerklötzen an Füßen und Schlingenpunkten.")],
}

UNAVAILABLE_500 = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
                   "reason": t("At 1:500 the stored heat pump is 1.6 mm by 2.8 mm on paper, under the 3 mm a recognisable drawing needs, and its largest inner parts (compressor 240 mm, pumps 100 mm) are 0.48 and 0.2 mm, at or below the 0.2 mm office-laser minimum; draw it at 1:200 or larger.",
                               "Bei 1:500 ist die gelagerte Wärmepumpe 1,6 mm mal 2,8 mm groß auf Papier, unter den 3 mm, die eine erkennbare Zeichnung braucht, und ihre größten inneren Teile (Verdichter 240 mm, Pumpen 100 mm) messen 0,48 und 0,2 mm, an oder unter dem Mindestmerkmal von 0,2 mm des Büro-Laserdrucks; bei 1:200 oder größer zeichnen."),
                   "limitingFeature": t("Circulators of 100 mm", "Umwälzpumpen von 100 mm"), "limitingSizeM": 0.1,
                   "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}


def bbox2(s):
    """📦️ Paper-plane bounds of a filled 2D shape."""
    k = s["kind"]
    if k == "rect":
        return s["x"], s["y"], s["x"] + s["w"], s["y"] + s["h"]
    if k == "ellipse":
        return s["cx"] - s["rx"], s["cy"] - s["ry"], s["cx"] + s["rx"], s["cy"] + s["ry"]
    if k == "capsule":
        r = max(s["r0"], s["r1"])
        return min(s["x0"], s["x1"]) - r, min(s["y0"], s["y1"]) - r, max(s["x0"], s["x1"]) + r, max(s["y0"], s["y1"]) + r
    xs = [q[0] for q in s["points"]]
    ys = [q[1] for q in s["points"]]
    return min(xs), min(ys), max(xs), max(ys)


def check_print(sc, geom):
    """🖨️ Near-miss gaps and thin features per view against the print profile at the band end (bounds-based, conservative)."""
    end = RANGES[sc][1]
    pen = PENS[sc]["body"]
    g = (0.15 + pen) * end / 1000
    fmin = max(0.0, 0.2 - pen) * end / 1000
    bad = 0
    for view, shapes in geom.items():
        fs = [(s["id"], bbox2(s), s["bone"], s["z"]) for s in shapes if s["filled"]]
        for sid, b, _, _ in fs:
            if min(b[2] - b[0], b[3] - b[1]) < fmin - 1e-9 and not sid.startswith(("type-plate", "display")):
                print(f"[DEBUG] 1:{sc} {view} feature {sid} {min(b[2] - b[0], b[3] - b[1]) * 1000:.1f} mm < {fmin * 1000:.1f}")
                bad += 1
        for i in range(len(fs)):
            for j in range(i + 1, len(fs)):
                a, b = fs[i][1], fs[j][1]
                if fs[i][2] != fs[j][2] or fs[i][3] != fs[j][3]:
                    continue
                dx = max(0.0, max(a[0], b[0]) - min(a[2], b[2]))
                dy = max(0.0, max(a[1], b[1]) - min(a[3], b[3]))
                d = math.hypot(dx, dy)
                if 1e-6 < d < g - 1e-9:
                    print(f"[DEBUG] 1:{sc} {view} gap {fs[i][0]} + {fs[j][0]} {d * 1000:.1f} mm < {g * 1000:.1f}")
                    bad += 1
    return bad


def main():
    """✍️ Writes the asset document."""
    reps, vols = {}, {}
    for sc in (50, 100, 200):
        parts, body, thermal, structure, geom = build_scale(sc)
        check_print(sc, geom)
        reps[str(sc)] = {"status": "authored", "denominator": sc, "validRange": {"minDenominator": RANGES[sc][0], "maxDenominator": RANGES[sc][1]}, "pens": PENS[sc], "hatches": HATCHES[sc], "geometry": geom, "decisions": DECISIONS[sc]}
        vols[str(sc)] = {"status": "authored", "denominator": sc, "body": body, "thermal": thermal, "structure": structure, "decisions": VOL_DECISIONS[sc]}
    reps["500"] = UNAVAILABLE_500
    vols["500"] = {"status": "unavailable", "denominator": 500}
    asset = {
        "id": "used-brine-water-heat-pump", "category": "components",
        "title": t("Used Brine-to-Water Heat Pump", "Gebrauchte Sole-Wasser-Wärmepumpe"),
        "summary": t("A dismantled 10 kW indoor ground-source heat pump of 600 x 1150 x 700 mm and 132 kg as a reuse dealer offers it: drained, refrigerant recovered, flexible hoses cut and capped, front panel taken off so the scroll compressor, the two plate heat exchangers and both circulators show; it stands on its rubber feet between two chocks on a half pallet, can be lifted off on two slings and, reinstalled, delivers 10.2 kW at B0/W35 from 2.3 kW of compressor input.",
                     "Eine ausgebaute 10-kW-Sole-Wasser-Wärmepumpe für innen von 600 x 1150 x 700 mm und 132 kg, wie sie ein Wiederverwendungshändler anbietet: entleert, Kältemittel abgesaugt, Flexschläuche abgeschnitten und verschlossen, Frontblech abgenommen, sodass Scrollverdichter, die zwei Plattenwärmeübertrager und beide Umwälzpumpen sichtbar sind; sie steht auf ihren Gummifüßen zwischen zwei Keilen auf einer Halbpalette, lässt sich an zwei Schlingen abheben und liefert wieder eingebaut 10,2 kW bei B0/W35 aus 2,3 kW Verdichterleistung."),
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "heat-pump", "ground-source", "brine-water", "scroll-compressor", "plate-heat-exchanger", "pallet", "steel"],
        "provenance": {"author": "Semio staffage component authors (used HVAC wave, author 06)", "method": "handcrafted", "created": "2026-11-22", "references": REFS,
                       "notes": t("Unit 600 mm wide, 1150 mm high on its feet, 700 mm deep, 132 kg dry; half pallet 800 x 600 x 144 mm, %.1f kg with chocks; dimensions, masses and ratings are class-typical authored estimates, nothing is measured." % PALLET_KG,
                                  "Gerät 600 mm breit, 1150 mm hoch auf seinen Füßen, 700 mm tief, 132 kg trocken; Halbpalette 800 x 600 x 144 mm, %s kg mit Keilen; Abmessungen, Massen und Leistungen sind klassentypische Schätzwerte, nichts ist gemessen." % ("%.1f" % PALLET_KG).replace(".", ","))},
        "dimensions": {"heightM": 1.42, "widthM": 0.8, "depthM": 0.6, "massKg": round(PALLET_KG + UNIT_KG, 2)},
        "materials": MATERIALS, "rig": RIG, "anchors": anchors(), "representations": reps, "symbols": [], "poses": POSES, "clips": CLIPS,
        "thermal": {"environment": {"airTempC": 15, "meanRadiantTempC": 15, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
                    "passengers": [], "operating": OPERATING, "regions": regions(),
                    "emitters": [{"id": "compressor-plume", "kind": "plume", "title": t("Warm air rising from the compressor dome", "Warme Luft, die von der Verdichterkuppel aufsteigt"), "anchor": "compressor-top", "directionDeg": 90, "diameterM": 0.2, "region": "compressor", "source": "authored-estimate", "reference": "incropera-2018"}],
                    "assumptions": [t("A plant room or dealer hall at 15 degC: stored and on standby every surface is at room temperature; reinstalled at B0/W35 the compressor shell releases about 110 W, the circulators 45 W and the controller 12 W, the heating side runs 35/30 degC and the brine side 0/-3 degC.",
                                      "Ein Technikraum oder eine Händlerhalle bei 15 °C: gelagert und in Bereitschaft haben alle Flächen Raumtemperatur; wieder eingebaut bei B0/W35 gibt der Verdichtermantel etwa 110 W ab, die Umwälzpumpen 45 W und der Regler 12 W, die Heizungsseite läuft mit 35/30 °C und die Soleseite mit 0/-3 °C."),
                                    t("The nameplate counts electrical input only (compressor 2.3 kW, circulators 160 W, controller 15 W); the 10.2 kW heating output is not power of the asset. No backup heater is fitted.",
                                      "Das Typenschild zählt nur die elektrische Aufnahme (Verdichter 2,3 kW, Umwälzpumpen 160 W, Regler 15 W); die Heizleistung von 10,2 kW ist keine Leistung des Assets. Ein Heizstab ist nicht eingebaut.")]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": RUNNERS,
                      "assumptions": [t("Stored, the 132 kg unit stands with its four rubber feet on the deck boards, which span 0.25 m between the stringer boards and blocks; the three bottom boards bear on the ground. Lifted, two round slings under the front and back base-frame channels carry the unit; reinstalled, the four feet carry it and 11 kg of heating water and brine.",
                                        "Gelagert steht das 132 kg schwere Gerät mit vier Gummifüßen auf den Deckbrettern, die 0,25 m zwischen Querbrettern und Klötzen spannen; die drei Bodenbretter liegen am Boden auf. Gehoben tragen zwei Rundschlingen unter dem vorderen und hinteren Grundrahmenprofil das Gerät; wieder eingebaut tragen es die vier Füße mit 11 kg Heizwasser und Sole."),
                                      t("Only the three pallet runners are permanent supports; each pose names the further contacts it uses (the four feet on the pallet deck or the floor, the two slings when lifted). The solver treats the asset as one rigid assembly, so in the lifted and reinstalled poses the pallet left on the floor shares the reactions with the slings or feet in proportion to stiffness, not as a separate body.", "Nur die drei Palettenkufen sind ständige Auflager; jede Pose nennt die weiteren Kontakte, die sie nutzt (die vier Füße auf dem Palettendeck oder dem Boden, die zwei Schlingen beim Heben). Der Löser behandelt das Asset als eine starre Baugruppe, die am Boden gebliebene Palette teilt daher in der gehobenen und der eingebauten Pose die Reaktionen mit Schlingen oder Füßen nach Steifigkeit, nicht als eigener Körper."),
                                      t("The casing panels are cladding; the load path is the galvanised base frame, which carries compressor, heat exchangers, pumps and casing.", "Die Gehäusebleche sind Verkleidung; der Lastweg ist der verzinkte Grundrahmen, der Verdichter, Wärmeübertrager, Pumpen und Gehäuse trägt.")],
                      "clinical": False},
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(f"[DEBUG] wrote {OUT}")


if __name__ == "__main__":
    main()
