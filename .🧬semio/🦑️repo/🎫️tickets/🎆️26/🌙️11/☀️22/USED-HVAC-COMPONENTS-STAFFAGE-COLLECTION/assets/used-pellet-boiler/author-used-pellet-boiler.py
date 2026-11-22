"""🔥 Authoritative generator of the used 15 kW wood pellet boiler (`used-pellet-boiler`): every hand-chosen number of parts, rig, poses, clips, thermal and structure lives here; the 2D shapes of every scale and view are drawn from the same part list as the 3D solids, so the three views and the volumes agree by construction.

Run: python3 author-used-pellet-boiler.py [areas.json]  (writes 🔣️.json next to this file)
@see ../../brief.md
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""
import json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-pellet-boiler"
PT = 0.144
TOPC = 1.194
TOPL = 1.214


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r5(v):
    """🎯 Rounds a number or nested list to 0.1 mm."""
    if isinstance(v, (list, tuple)):
        return [r5(x) for x in v]
    return round(v, 5) + 0.0


BONES = {}


def bone(id, parent, x, y, z, length, joint, title, mass=None):
    """🦴 Registers a bone (all rest angles 0, so bone-local axes are world axes) and keeps its world origin."""
    ox, oy, oz = (0.0, 0.0, 0.0) if parent is None else BONES[parent]["world"]
    b = {"id": id, "parent": parent, "title": title, "x": r5(x), "y": r5(y), "angleDeg": 0, "lengthM": length, "joint": joint}
    if z:
        b["z"] = r5(z)
    if mass:
        b["mass"] = mass
    BONES[id] = {"def": b, "world": (ox + x, oy + y, oz + z)}


def W(id):
    """🌐 World rest origin of a bone."""
    return BONES[id]["world"]


rev = lambda lo, hi: {"type": "revolute", "minDeg": lo, "maxDeg": hi}
bone("site", None, 0, 0, 0, 0.05, {"type": "fixed"}, T("Yard reference point of the component exchange (fixed root)", "Bezugspunkt im Lager der Bauteilbörse (feste Wurzel)"))
bone("pallet", "site", -0.4, 0, 0, 0.8, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.5}, T("EPAL euro pallet with the two ratchet straps (stays behind when the boiler is lifted)", "EPAL-Europalette mit den zwei Ratschengurten (bleibt beim Anheben zurück)"),
     {"kg": 26.8, "comX": 0.4, "comY": 0.075, "distribution": "uniform", "source": "literature", "reference": "en-13698-1"})
bone("boiler", "site", -0.375, PT, 0, 0.75, {"type": "prismatic", "axisDeg": 90, "minM": -PT, "maxM": 0.5}, T("Pellet boiler with integrated day hopper, drained and empty", "Pelletkessel mit integriertem Tagesbehälter, entleert"),
     {"kg": 255.0, "comX": 0.36, "comY": 0.5, "distribution": "point", "source": "authored-estimate", "reference": "pellet-boiler-datasheets"})
bone("sling", "pallet", 0.65, TOPC, -0.1, 0.05, {"type": "prismatic", "axisDeg": 90, "minM": -1.1, "maxM": 0.5}, T("Two DIN 580 eye bolts of the two-leg chain sling", "Zwei Ringschrauben DIN 580 des zweisträngigen Kettengehänges"),
     {"kg": 2.4, "comX": -0.25, "comY": 0.02, "distribution": "point", "source": "authored-estimate", "reference": "en-818-4"})
bone("sling-front", "sling", 0, 0.033, 0.015, 0.05, rev(-70, 0), T("Front chain leg, hinged in the front eye", "Vorderer Kettenstrang, im vorderen Ring eingehängt"))
bone("sling-back", "sling", -0.5, 0.033, -0.015, 0.05, rev(0, 70), T("Back chain leg, hinged in the back eye", "Hinterer Kettenstrang, im hinteren Ring eingehängt"))
STRAPS = {"r": (-0.475, TOPC), "l": (0.455, TOPL)}
for s, (zc, top) in STRAPS.items():
    side = T("right", "rechts") if s == "r" else T("left", "links")
    L = top + 0.002 - PT
    bone(f"strap-{s}-front", "pallet", 0.777, PT, zc, 0.05, rev(-100, 0), T(f"Strap {side['en']}, front leg hooked under the deck", f"Gurt {side['de']}, vorderes Trum unter dem Deck eingehakt"))
    bone(f"strap-{s}-front-top", f"strap-{s}-front", 0, L, 0, 0.05, rev(-90, 0), T(f"Strap {side['en']}, front half over the top", f"Gurt {side['de']}, vordere Hälfte über die Oberseite"))
    bone(f"strap-{s}-ratchet", f"strap-{s}-front", -0.001, 0.436, 0, 0.05, rev(0, 180), T(f"Ratchet of the {side['en']} strap", f"Ratsche des Gurts {side['de']}"))
    bone(f"strap-{s}-back", "pallet", 0.023, PT, zc, 0.05, rev(0, 100), T(f"Strap {side['en']}, back leg hooked under the deck", f"Gurt {side['de']}, hinteres Trum unter dem Deck eingehakt"))
    bone(f"strap-{s}-back-top", f"strap-{s}-back", 0, L, 0, 0.05, rev(0, 90), T(f"Strap {side['en']}, back half over the top", f"Gurt {side['de']}, hintere Hälfte über die Oberseite"))


def part(id, bone, kind, mat, tone, region, **k):
    """🧩 One physical part in world rest coordinates."""
    return dict(id=id, bone=bone, kind=kind, mat=mat, tone=tone, region=region, **k)


def box(id, bone, mat, tone, region, x, y, z, r=0.0, **k):
    """📦 Axis-aligned box from (x0,x1), (y0,y1), (z0,z1)."""
    return part(id, bone, "box", mat, tone, region, x=x, y=y, z=z, r=r, **k)


def cyl(id, bone, mat, tone, region, axis, c, a, ro, ri=None, **k):
    """🛢 Cylinder (or tube with `ri`) along a world axis; `c` the two other coordinates in (x,y,z) order without the axis, `a` the axial range."""
    return part(id, bone, "cyl", mat, tone, region, axis=axis, c=c, a=a, ro=ro, ri=ri, **k)


def steps(id, bone, mat, tone, region, c, rows, **k):
    """🎂 Stepped vertical revolve: rows of (y0, y1, r)."""
    return part(id, bone, "steps", mat, tone, region, c=c, rows=rows, **k)


def caps(id, bone, mat, tone, region, p0, p1, r, **k):
    """💊 Capsule between two world points."""
    return part(id, bone, "cap", mat, tone, region, p0=p0, p1=p1, rad=r, **k)


def ring(id, bone, mat, tone, region, c, ro, ri, zc, t, **k):
    """⭕ Eye ring in the x-y plane, centre c=(x,y), lateral centre zc, thickness t."""
    return part(id, bone, "ring", mat, tone, region, c=c, ro=ro, ri=ri, zc=zc, t=t, **k)


def circle(cx, cy, r, n=24):
    """🔵 Polygon of a circle."""
    return [[r5(cx + r * math.cos(2 * math.pi * i / n)), r5(cy + r * math.sin(2 * math.pi * i / n))] for i in range(n)]


def pallet_parts(scale):
    """🪵 EPAL 1200 x 800 x 144 mm pallet (EN 13698-1): 3 bottom boards, 9 blocks, 3 cross boards, 5 deck boards; merged per scale."""
    P = []
    rows = [(0.3, 0.4), (-0.0725, 0.0725), (-0.4, -0.3)]
    cols = [(-0.6, -0.455), (-0.0725, 0.0725), (0.455, 0.6)]
    deck = [(0.255, 0.4), (0.11375, 0.21375), (-0.0725, 0.0725), (-0.21375, -0.11375), (-0.4, -0.255)]
    names = ["front", "middle", "back"]
    if scale in (50, 100):
        for i, x in enumerate(rows):
            P.append(box(f"bottom-board-{names[i]}", "pallet", "spruce-weathered", 0.5, "pallet", x, (0, 0.022), (-0.6, 0.6), 0.002 if scale == 50 else 0.0))
        for i, x in enumerate(rows):
            for j, z in enumerate(cols):
                P.append(box(f"block-{names[i]}-{j}", "pallet", "chipboard-block", 0.42, "pallet", x, (0.022, 0.1), z, 0.003 if scale == 50 else 0.0))
        for j, z in enumerate(cols):
            P.append(box(f"cross-board-{j}", "pallet", "spruce-weathered", 0.56, "pallet", (-0.4, 0.4), (0.1, 0.122), z, 0.002 if scale == 50 else 0.0))
        for i, x in enumerate(deck):
            P.append(box(f"deck-board-{i}", "pallet", "spruce-weathered", 0.6, "pallet", x, (0.122, PT), (-0.6, 0.6), 0.002 if scale == 50 else 0.0))
    elif scale == 200:
        for i, x in enumerate(rows):
            P.append(part(f"runner-{names[i]}", "pallet", "runner", "spruce-weathered", 0.46, "pallet", x=x, cols=cols))
        P.append(box("deck", "pallet", "spruce-weathered", 0.58, "pallet", (-0.4, 0.4), (0.1, PT), (-0.6, 0.6)))
    else:
        P.append(box("pallet", "pallet", "spruce-weathered", 0.55, "pallet", (-0.4, 0.4), (0, PT), (-0.6, 0.6)))
    return P


def boiler_parts(scale):
    """🔥 The boiler: base frame on four levelling feet, casing with combustion door, ash box and controls, hopper with lid, cyclone and suction turbine, flue spigot, flue gas fan, capped flow and return, the suction hose remnant."""
    P = []
    b = "boiler"
    if scale in (50, 100, 200):
        for i, (x, z) in enumerate([(0.33, -0.45), (0.33, 0.45), (-0.33, -0.45), (-0.33, 0.45)]):
            P.append(cyl(f"foot-{i}", b, "galvanised-steel", 0.3, "casing", "y", (x, z), (PT, PT + 0.03), 0.02))
        P.append(box("base-frame", b, "galvanised-steel", 0.45, "casing", (-0.36, 0.36), (PT + 0.03, PT + 0.07), (-0.49, 0.49)))
        y0 = PT + 0.07
    else:
        y0 = PT
    rc = {50: 0.01, 100: 0.0, 200: 0.0, 500: 0.0}[scale]
    P.append(box("casing", b, "powder-coat-faded", 0.8, "casing", (-0.375, 0.375), (y0, TOPC), (-0.5, 0.1), rc, layer="sheet"))
    if scale == 500:
        P.append(box("hopper", b, "powder-coat-faded", 0.72, "hopper", (-0.375, 0.375), (y0, TOPL), (0.1, 0.5)))
    else:
        P.append(box("hopper", b, "powder-coat-faded", 0.72, "hopper", (-0.375, 0.375), (y0, TOPC), (0.1, 0.5), rc))
        P.append(box("hopper-lid", b, "powder-coat-faded", 0.66, "hopper", (-0.375, 0.375), (TOPC, TOPL), (0.1, 0.5), 0.004 if scale == 50 else 0.0))
    if scale in (50, 100, 200):
        P.append(box("door", b, "enamel-black", 0.22, "combustion-door", (0.375, 0.405), (0.70, 1.12), (-0.44, 0.04), 0.008 if scale == 50 else 0.0))
    else:
        P.append(box("door", b, "enamel-black", 0.22, "combustion-door", (0.375, 0.45), (0.70, 1.12), (-0.44, 0.04)))
    if scale in (50, 100):
        P.append(caps("door-handle", b, "galvanised-steel", 0.55, "combustion-door", (0.425, 1.05, -0.38), (0.425, 1.05, -0.22), 0.011))
        for i, z in enumerate((-0.37, -0.23)):
            P.append(box(f"door-handle-post-{i}", b, "galvanised-steel", 0.5, "combustion-door", (0.405, 0.42), (1.042, 1.058), (z - 0.008, z + 0.008)))
    if scale == 200:
        P.append(box("door-handle", b, "galvanised-steel", 0.55, "combustion-door", (0.405, 0.436), (1.035, 1.065), (-0.38, -0.22)))
    if scale == 50:
        P.append(cyl("sight-glass", b, "borosilicate", 0.88, "combustion-door", "x", (0.93, -0.2), (0.405, 0.407), 0.025, role="glazing", sheet=True, only=("front",)))
    if scale in (50, 100):
        P.append(box("ash-box", b, "powder-coat-faded", 0.68, "casing", (0.375, 0.395), (0.29, 0.53), (-0.42, 0.02), 0.006 if scale == 50 else 0.0))
        P.append(caps("ash-box-handle", b, "galvanised-steel", 0.5, "casing", (0.41, 0.47, -0.3), (0.41, 0.47, -0.1), 0.009 if scale == 50 else 0.0075))
        for i, z in enumerate((-0.29, -0.11)):
            P.append(box(f"ash-box-handle-post-{i}", b, "galvanised-steel", 0.5, "casing", (0.395, 0.405), (0.462, 0.478), (z - 0.008, z + 0.008)))
        P.append(box("control-panel", b, "polycarbonate", 0.3, "casing", (0.375, 0.39), (1.135, 1.185), (-0.4, 0.02)))
    if scale == 50:
        P.append(box("display", b, "polycarbonate", 0.12, "casing", (0.39, 0.391), (1.147, 1.173), (-0.3, -0.18), texture=True, only=("front",)))
        P.append(box("type-plate", b, "aluminium-plate", 0.9, "casing", (-0.2, -0.05), (0.35, 0.45), (-0.501, -0.5), texture=True, only=("elevation",)))
    fl = {50: 0.008, 100: 0.015}.get(scale, 0.0)
    if fl:
        P.append(cyl("spigot-flange", b, "steel-sooted", 0.32, "flue-spigot", "y", (-0.25, -0.2), (TOPC, TOPC + fl), 0.09))
    P.append(cyl("flue-spigot", b, "steel-sooted", 0.36, "flue-spigot", "y", (-0.25, -0.2), (TOPC + fl, TOPC + 0.12), 0.065, {50: 0.0615, 100: 0.0615, 200: 0.06, 500: 0.058}[scale]))
    if scale in (50, 100):
        P.append(box("fan-housing", b, "cast-aluminium", 0.42, "flue-fan", (-0.435, -0.375), (0.82, 1.02), (-0.22, 0.06), 0.006 if scale == 50 else 0.0))
        P.append(cyl("fan-motor", b, "cast-aluminium", 0.5, "flue-fan", "x", (0.92, -0.08), (-0.505, -0.435), 0.055))
    elif scale == 200:
        P.append(box("fan-housing", b, "cast-aluminium", 0.44, "flue-fan", (-0.435, -0.375), (0.82, 1.02), (-0.22, 0.06)))
        P.append(cyl("fan-motor", b, "cast-aluminium", 0.5, "flue-fan", "x", (0.92, -0.08), (-0.505, -0.435), 0.055))
    else:
        P.append(box("fan-unit", b, "cast-aluminium", 0.44, "flue-fan", (-0.505, -0.375), (0.82, 1.02), (-0.22, 0.06)))
    if scale == 50:
        for n, y in (("flow", 0.98), ("return", 0.62)):
            P.append(cyl(f"{n}-stub", b, "steel-rusty", 0.38, "connections", "x", (y, -0.36), (-0.455, -0.375), 0.01685))
            P.append(cyl(f"{n}-cap", b, "brass-tarnished", 0.52, "connections", "x", (y, -0.36), (-0.48, -0.455), 0.021))
    elif scale in (100, 200):
        for n, y in (("flow", 0.98), ("return", 0.62)):
            P.append(cyl(f"{n}-stub", b, "steel-rusty", 0.4, "connections", "x", (y, -0.36), (-0.48, -0.375), 0.02 if scale == 200 else 0.019))
    if scale in (50, 100, 200):
        P.append(cyl("cyclone", b, "powder-coat-faded", 0.74, "suction-unit", "y", (-0.12, 0.29), (TOPL, 1.384), 0.12))
        P.append(cyl("suction-motor", b, "polymer-housing", 0.4, "suction-unit", "y", (-0.12, 0.29), (1.384, 1.454), 0.07))
        P.append(cyl("cyclone-inlet", b, "powder-coat-faded", 0.6, "suction-unit", "x", (1.33, 0.29), (-0.3, -0.2), 0.03))
        P.append(caps("hose-run", b, "pvc-spiral", 0.64, "suction-hose", (-0.3, 1.33, 0.29), (-0.42, 1.33, 0.29), 0.03))
        P.append(caps("hose-drop", b, "pvc-spiral", 0.64, "suction-hose", (-0.42, 1.33, 0.29), (-0.42, 1.08, 0.29), 0.03))
    else:
        P.append(steps("suction-unit", b, "powder-coat-faded", 0.7, "suction-unit", (-0.12, 0.29), [(TOPL, 1.384, 0.12), (1.384, 1.454, 0.08)]))
    return P


def sling_parts(scale):
    """⛓ The two-leg chain sling: eye bolts DIN 580 M12 on the casing top and two 8 mm chain legs lying on top."""
    P = []
    for n, x, zl, bn in (("front", 0.25, -0.085, "sling-front"), ("back", -0.25, -0.115, "sling-back")):
        if scale == 50:
            P.append(cyl(f"eye-collar-{n}", "sling", "alloy-steel", 0.32, "sling", "y", (x, -0.1), (TOPC, TOPC + 0.008), 0.016))
            P.append(ring(f"eye-ring-{n}", "sling", "alloy-steel", 0.32, "sling", (x, TOPC + 0.033), 0.025, 0.014, -0.1, 0.011))
        elif scale in (100, 200):
            e = 0.02 if scale == 100 else 0.03
            P.append(box(f"eye-{n}", "sling", "alloy-steel", 0.32, "sling", (x - 0.022, x + 0.022), (TOPC, TOPC + 0.05), (-0.1 - e / 2, -0.1 + e / 2)))
        d = -1 if n == "front" else 1
        P.append(caps(f"chain-{n}", bn, "alloy-steel", 0.28, "sling", (x + d * 0.03, TOPC + 0.011, zl), (x + d * 0.45, TOPC + 0.011, zl), 0.011, slender=scale in (200, 500)))
    return P


def strap_parts(scale):
    """🎗 Two 50 mm polyester ratchet straps over the boiler, each in four bands on hinged bones and a ratchet."""
    P = []
    if scale == 500:
        return P
    for s, (zc, top) in STRAPS.items():
        z = (zc - 0.025, zc + 0.025)
        P.append(box(f"strap-{s}-front-leg", f"strap-{s}-front", "polyester-webbing", 0.62, "straps", (0.375, 0.377), (PT, top + 0.002), z, sheet=True))
        P.append(box(f"strap-{s}-front-top", f"strap-{s}-front-top", "polyester-webbing", 0.62, "straps", (0.0, 0.375), (top, top + 0.002), z, sheet=True))
        P.append(box(f"strap-{s}-back-leg", f"strap-{s}-back", "polyester-webbing", 0.62, "straps", (-0.377, -0.375), (PT, top + 0.002), z, sheet=True))
        P.append(box(f"strap-{s}-back-top", f"strap-{s}-back-top", "polyester-webbing", 0.62, "straps", (-0.375, 0.0), (top, top + 0.002), z, sheet=True))
        if scale in (50, 100):
            P.append(box(f"strap-{s}-ratchet", f"strap-{s}-ratchet", "galvanised-steel", 0.36, "straps", (0.377, 0.402), (0.5, 0.66), (zc - 0.03, zc + 0.03)))
    return P


def parts(scale):
    """🧰 Every body part of one scale."""
    return pallet_parts(scale) + boiler_parts(scale) + sling_parts(scale) + strap_parts(scale)


def local(p, bn):
    """📐 World point to bone-local."""
    o = W(bn)
    return [p[0] - o[0], p[1] - o[1], p[2] - o[2]]


def solid_of(p):
    """🧊 The volume solid of a part, in its bone's rest frame."""
    bn = p["bone"]
    o = W(bn)
    s = {"id": p["id"], "bone": bn, "material": p["mat"], "role": p.get("role", "body"), "tone": p["tone"]}
    for f in ("sheet", "slender", "texture"):
        if p.get(f):
            s[f] = True
    k = p["kind"]
    if k == "box":
        (x0, x1), (y0, y1), (z0, z1) = p["x"], p["y"], p["z"]
        s.update(kind="box", size=r5([x1 - x0, y1 - y0, z1 - z0]), transform={"translation": r5([(x0 + x1) / 2 - o[0], (y0 + y1) / 2 - o[1], (z0 + z1) / 2 - o[2]])})
        if p.get("r"):
            s["cornerRadius"] = p["r"]
    elif k == "cyl":
        ax = p["axis"]
        a0, a1 = p["a"]
        c1, c2 = p["c"]
        if ax == "y":
            origin, axis, off = [c1 - o[0], 0.0, c2 - o[2]], [0, 1, 0], o[1]
        elif ax == "x":
            origin, axis, off = [0.0, c1 - o[1], c2 - o[2]], [1, 0, 0], o[0]
        else:
            origin, axis, off = [c1 - o[0], c2 - o[1], 0.0], [0, 0, 1], o[2]
        a0, a1 = a0 - off, a1 - off
        ro, ri = p["ro"], p["ri"]
        prof = [[a0, ri], [a1, ri], [a1, ro], [a0, ro], [a0, ri]] if ri else [[a0, ro], [a1, ro]]
        s.update(kind="revolve", profile=r5(prof), axis=axis, origin=r5(origin))
    elif k == "steps":
        c1, c2 = p["c"]
        prof = []
        for (y0, y1, rr) in p["rows"]:
            prof += [[y0 - o[1], rr], [y1 - o[1], rr]]
        s.update(kind="revolve", profile=r5(prof), axis=[0, 1, 0], origin=r5([c1 - o[0], 0.0, c2 - o[2]]))
    elif k == "cap":
        s.update(kind="capsule", p0=r5(local(p["p0"], bn)), p1=r5(local(p["p1"], bn)), r0=p["rad"], r1=p["rad"])
    elif k == "ring":
        cx, cy = p["c"]
        s.update(kind="extrude", profile=circle(cx - o[0], cy - o[1], p["ro"]), holes=[circle(cx - o[0], cy - o[1], p["ri"])], plane="xy", **{"from": r5(p["zc"] - p["t"] / 2 - o[2]), "to": r5(p["zc"] + p["t"] / 2 - o[2])})
    elif k == "runner":
        x0, x1 = p["x"]
        prof = [[0.0, -0.6], [0.0, 0.6], [0.03, 0.6]]
        for (z0, z1) in reversed(p["cols"]):
            prof += [[0.03, z1], [0.1, z1], [0.1, z0], [0.03, z0]]
        prof += [[0.03, -0.6]]
        clean = []
        for q in prof:
            if not clean or clean[-1] != q:
                clean.append(q)
        clean = [q for i, q in enumerate(clean) if not (i > 0 and q == clean[0])]
        s.update(kind="extrude", profile=r5(dedupe(clean)), plane="yz", **{"from": r5(x0 - o[0]), "to": r5(x1 - o[0])})
    if p.get("tags"):
        s["tags"] = p["tags"]
    return s


def dedupe(pts):
    """🧹 Removes collinear and repeated points of a closed ring."""
    out = []
    n = len(pts)
    for i in range(n):
        a, b, c = pts[i - 1], pts[i], pts[(i + 1) % n]
        if a == b:
            continue
        if (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0]) == 0:
            continue
        out.append(b)
    return out


def extents(p):
    """📏 World bounding box of a part as ((x0,x1),(y0,y1),(z0,z1))."""
    k = p["kind"]
    if k == "box":
        return p["x"], p["y"], p["z"]
    if k == "cyl":
        a, (c1, c2), ro = p["a"], p["c"], p["ro"]
        if p["axis"] == "y":
            return (c1 - ro, c1 + ro), a, (c2 - ro, c2 + ro)
        if p["axis"] == "x":
            return a, (c1 - ro, c1 + ro), (c2 - ro, c2 + ro)
        return (c1 - ro, c1 + ro), (c2 - ro, c2 + ro), a
    if k == "steps":
        (c1, c2), rows = p["c"], p["rows"]
        rr = max(r for _, _, r in rows)
        return (c1 - rr, c1 + rr), (rows[0][0], rows[-1][1]), (c2 - rr, c2 + rr)
    if k == "cap":
        r = p["rad"]
        return tuple((min(p["p0"][i], p["p1"][i]) - r, max(p["p0"][i], p["p1"][i]) + r) for i in range(3))
    if k == "ring":
        (cx, cy), ro = p["c"], p["ro"]
        return (cx - ro, cx + ro), (cy - ro, cy + ro), (p["zc"] - p["t"] / 2, p["zc"] + p["t"] / 2)
    if k == "runner":
        return p["x"], (0, 0.1), (-0.6, 0.6)


def rect(x0, x1, y0, y1, r=0.0):
    """▭ Rect geometry."""
    g = {"kind": "rect", "x": r5(x0), "y": r5(y0), "w": r5(x1 - x0), "h": r5(y1 - y0)}
    if r:
        g["r"] = r
    return g


def ell(cx, cy, r):
    """⚪ Circle geometry."""
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(r), "ry": r5(r)}


def views_of(p):
    """🖼 The three 2D geometries (elevation in bone-local u,v; front in world z,y; plan in world x,z) of a part."""
    o = W(p["bone"])
    (x0, x1), (y0, y1), (z0, z1) = extents(p)
    k = p["kind"]
    ex = lambda a, b, c, d, r=0.0: rect(a - o[0], b - o[0], c - o[1], d - o[1], r)
    E = ex(x0, x1, y0, y1, p.get("r", 0.0)) if k == "box" else ex(x0, x1, y0, y1)
    F = rect(z0, z1, y0, y1, p.get("r", 0.0) if k == "box" else 0.0)
    Pl = rect(x0, x1, z0, z1, p.get("r", 0.0) if k == "box" else 0.0)
    if k == "cyl":
        ax, (c1, c2), ro = p["axis"], p["c"], p["ro"]
        if ax == "y":
            Pl = ell(c1, c2, ro)
        elif ax == "x":
            F = ell(c2, c1, ro)
        else:
            E = ell(c1 - o[0], c2 - o[1], ro)
    elif k == "steps":
        (c1, c2), rows = p["c"], p["rows"]
        pts = []
        for (a, b, rr) in rows:
            pts += [[c1 - rr - o[0], a - o[1]], [c1 - rr - o[0], b - o[1]]]
        right = [[2 * (c1 - o[0]) - q[0], q[1]] for q in reversed(pts)]
        E = {"kind": "poly", "points": r5(pts + right), "closed": True}
        fpts = []
        for (a, b, rr) in rows:
            fpts += [[c2 - rr, a], [c2 - rr, b]]
        fright = [[2 * c2 - q[0], q[1]] for q in reversed(fpts)]
        F = {"kind": "poly", "points": r5(fpts + fright), "closed": True}
        Pl = ell(c1, c2, max(r for _, _, r in rows))
    elif k == "cap":
        p0, p1, r = p["p0"], p["p1"], p["rad"]
        cap = lambda a, b, c, d: {"kind": "capsule", "x0": r5(a), "y0": r5(b), "x1": r5(c), "y1": r5(d), "r0": r, "r1": r}
        along = [i for i in range(3) if abs(p0[i] - p1[i]) > 1e-9][0]
        E = cap(p0[0] - o[0], p0[1] - o[1], p1[0] - o[0], p1[1] - o[1]) if along in (0, 1) else ell(p0[0] - o[0], p0[1] - o[1], r)
        F = cap(p0[2], p0[1], p1[2], p1[1]) if along in (1, 2) else ell(p0[2], p0[1], r)
        Pl = cap(p0[0], p0[2], p1[0], p1[2]) if along in (0, 2) else ell(p0[0], p0[2], r)
    elif k == "ring":
        (cx, cy), ro, ri = p["c"], p["ro"], p["ri"]
        lx, ly = cx - o[0], cy - o[1]
        E = {"kind": "path", "d": ring_path(lx, ly, ro, ri)}
    elif k == "runner":
        Pl = rect(x0, x1, -0.6, 0.6)
    return E, F, Pl


def ring_path(cx, cy, ro, ri):
    """🔘 SVG path of an annulus (even-odd hole)."""
    def c(r):
        return f"M {r5(cx + r)} {r5(cy)} A {r} {r} 0 1 0 {r5(cx - r)} {r5(cy)} A {r} {r} 0 1 0 {r5(cx + r)} {r5(cy)} Z"
    return c(ro) + " " + c(ri)


def runner_front(p, o):
    """🪵 Front-view outline of a merged runner (bottom board with its three blocks)."""
    pts = [[-0.6, 0.0], [0.6, 0.0], [0.6, 0.03]]
    for (z0, z1) in reversed(p["cols"]):
        pts += [[z1, 0.03], [z1, 0.1], [z0, 0.1], [z0, 0.03]]
    pts += [[-0.6, 0.03]]
    return {"kind": "poly", "points": r5(dedupe(pts)), "closed": True}


def shapes_of(scale, plist, details):
    """✏️ The geometry of one representation: every part in three views, then the hand-drawn detail strokes."""
    G = {"elevation": [], "front": [], "plan": []}
    nearest = {"elevation": lambda e: -e[2][0], "front": lambda e: e[0][1], "plan": lambda e: e[1][1]}
    rank = {v: {q["id"]: i for i, q in enumerate(sorted(plist, key=lambda q: (nearest[v](extents(q)), q["id"])))} for v in nearest}
    zc = 1.0
    for p in plist:
        E, F, Pl = views_of(p)
        if p["kind"] == "runner":
            F = runner_front(p, W(p["bone"]))
            E = rect(p["x"][0] - W("pallet")[0], p["x"][1] - W("pallet")[0], 0.0, 0.1)
        role = "texture" if p.get("texture") else p.get("role", "body")
        for v, g in (("elevation", E), ("front", F), ("plan", Pl)):
            if p.get("only") and v not in p["only"]:
                continue
            sh = {"id": p["id"], "bone": p["bone"], "role": role, "z": round(1.0 + 0.01 * rank[v][p["id"]], 3), "filled": True, "tone": p["tone"]}
            reg = p.get("regions2d", {}).get(scale) if p.get("regions2d") else None
            if reg:
                sh["regions"] = reg
            else:
                sh["region"] = p["region"]
            sh["material"] = p["mat"]
            sh.update(g)
            G[v].append(sh)
    zc = 1.0 + 0.01 * len(plist)
    for v, lst in details.items():
        for d in lst:
            sh = {"id": d["id"], "bone": d["bone"], "role": d["role"], "z": round(zc, 3), "filled": d.get("filled", False), "tone": d.get("tone", 0.3)}
            if d.get("region"):
                sh["region"] = d["region"]
            sh.update(d["g"])
            G[v].append(sh)
            zc += 0.01
    return G


def poly(pts, closed=False):
    """〰️ Poly geometry."""
    return {"kind": "poly", "points": r5(pts), "closed": closed}


def det(id, bone, role, g, filled=False, tone=0.3, region=None):
    """🖊 A hand-drawn detail stroke."""
    return dict(id=id, bone=bone, role=role, g=g, filled=filled, tone=tone, region=region)


def details(scale):
    """🔎 The hand-drawn line work of one scale: seams, door gap, hinges, labels, louvres, nails, brand marks, soot, rust and the cut hose end; fewer and coarser towards 1:500."""
    ob = W("boiler")
    op = W("pallet")
    bx = lambda x: x - ob[0]
    by = lambda y: y - ob[1]
    E, F, P = [], [], []
    if scale == 500:
        return {"elevation": E, "front": F, "plan": P}
    E.append(det("side-seam", "boiler", "detail", poly([[bx(-0.375), by(0.7)], [bx(0.375), by(0.7)]])))
    F.append(det("hopper-joint", "boiler", "detail", poly([[0.1, PT + 0.07], [0.1, TOPC]])))
    P.append(det("hopper-joint", "boiler", "detail", poly([[-0.375, 0.1], [0.375, 0.1]])))
    F.append(det("door-hinge-line", "boiler", "detail", poly([[0.03, 0.72], [0.03, 1.10]])))
    if scale in (50, 100):
        F.append(det("ash-box-gap", "boiler", "detail", poly([[-0.42, 0.29], [0.02, 0.29], [0.02, 0.53], [-0.42, 0.53]], True)))
        E.append(det("fan-louvres", "boiler", "detail", poly([[bx(-0.435), by(0.86)], [bx(-0.435), by(0.98)]])))
        P.append(det("lid-hinge", "boiler", "detail", poly([[-0.355, 0.12], [-0.355, 0.48]])))
        F.append(det("hopper-level-window", "boiler", "detail", poly([[0.27, 0.45], [0.33, 0.45], [0.33, 0.95], [0.27, 0.95]], True)))
    if scale == 50:
        for i, y in enumerate((0.37, 0.39, 0.41, 0.43)):
            E.append(det(f"type-plate-line-{i}", "boiler", "texture", poly([[bx(-0.19), by(y)], [bx(-0.07 if i else -0.11), by(y)]])))
        E.append(det("rust-streak-side", "boiler", "texture", poly([[bx(0.05), by(0.24)], [bx(0.07), by(0.30)], [bx(0.12), by(0.27)], [bx(0.16), by(0.33)]]), tone=0.25))
        E.append(det("cut-hose-end", "boiler", "texture", poly([[bx(-0.45), by(1.083)], [bx(-0.39), by(1.077)]]), tone=0.2))
        for i, y in enumerate((1.12, 1.17, 1.22, 1.27)):
            E.append(det(f"hose-rib-{i}", "boiler", "texture", poly([[bx(-0.45), by(y)], [bx(-0.39), by(y + 0.01)]])))
        for i, x in enumerate((-0.33, -0.36, -0.39)):
            E.append(det(f"hose-run-rib-{i}", "boiler", "texture", poly([[bx(x), by(1.30)], [bx(x + 0.01), by(1.36)]])))
        E.append(det("spigot-soot", "boiler", "texture", poly([[bx(-0.315), by(TOPC + 0.10)], [bx(-0.25), by(TOPC + 0.106)], [bx(-0.185), by(TOPC + 0.10)]]), tone=0.1))
        E.append(det("stub-limescale-flow", "boiler", "texture", poly([[bx(-0.44), by(0.966)], [bx(-0.42), by(0.994)]]), tone=0.85))
        E.append(det("stub-limescale-return", "boiler", "texture", poly([[bx(-0.44), by(0.606)], [bx(-0.42), by(0.634)]]), tone=0.85))
        F.append(det("door-label", "boiler", "texture", poly([[-0.12, 0.76], [0.0, 0.76], [0.0, 0.82], [-0.12, 0.82]], True), tone=0.6))
        F.append(det("door-label-text", "boiler", "texture", poly([[-0.11, 0.79], [-0.01, 0.79]])))
        F.append(det("door-hinge-top", "boiler", "detail", poly([[0.03, 1.06], [0.05, 1.06]])))
        F.append(det("door-hinge-bottom", "boiler", "detail", poly([[0.03, 0.76], [0.05, 0.76]])))
        F.append(det("panel-buttons", "boiler", "texture", poly([[-0.14, 1.16], [-0.06, 1.16]])))
        F.append(det("faded-hopper-sticker", "boiler", "texture", poly([[0.15, 1.0], [0.24, 1.0], [0.24, 1.08], [0.15, 1.08]], True), tone=0.55))
        P.append(det("motor-grille", "boiler", "texture", poly(circle(-0.12, 0.29, 0.045, 16), True)))
        P.append(det("cyclone-lid-seam", "boiler", "texture", poly(circle(-0.12, 0.29, 0.1, 24), True)))
        P.append(det("casing-top-seam", "boiler", "detail", poly([[0.15, -0.5], [0.15, 0.1]])))
        for j in range(3):
            E.append(det(f"epal-mark-{j}", "pallet", "texture", poly(circle([0.35, 0.0, -0.35][j] - op[0], 0.061 - op[1], 0.018, 12), True)))
        for i, x in enumerate((0.3275, 0.16375, 0.0, -0.16375, -0.3275)):
            P.append(det(f"nails-{i}", "pallet", "texture", poly([[x, -0.53], [x, -0.52]])))
        F.append(det("strap-stitch-r", "strap-r-front", "texture", poly([[-0.495, 0.60], [-0.455, 0.60]])))
        F.append(det("strap-stitch-l", "strap-l-front", "texture", poly([[0.435, 0.60], [0.475, 0.60]])))
        P.append(det("chain-links-front", "sling-front", "texture", poly([[-0.17, -0.085], [0.2, -0.085]])))
        P.append(det("chain-links-back", "sling-back", "texture", poly([[-0.2, -0.115], [0.17, -0.115]])))
    return {"elevation": E, "front": F, "plan": P}


REGIONS_500 = {"casing": ["casing", "connections"], "hopper": ["hopper", "straps"], "suction-unit": ["suction-unit", "suction-hose"], "pallet": ["pallet", "straps"], "chain-front": ["sling"], "chain-back": ["sling"]}


def representation(scale, decisions):
    """🗺 One authored representation."""
    plist = parts(scale)
    if scale == 500:
        for p in plist:
            if p["id"] in REGIONS_500:
                p["regions2d"] = {500: REGIONS_500[p["id"]]}
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
            100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
            200: {"body": 0.2, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
            500: {"body": 0.2, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": [], "geometry": shapes_of(scale, plist, details(scale)), "decisions": decisions}


def ref(id, of, **k):
    """🔗 A reference solid."""
    d = {"id": id, "of": of}
    d.update(k)
    return d


def own(p, **k):
    """🧱 An own solid of a part spec with labels."""
    s = solid_of(p)
    s.update(k)
    return s


def thermal_list(scale, plist):
    """🌡 The thermal model of one scale: every body surface by reference, the insulation shell at 1:50, the tagged flue gas spigot, own sheets where the body generalises a region away."""
    L = []
    for p in plist:
        if p.get("texture"):
            continue
        extra = {}
        if p["region"] == "casing" and scale == 50:
            extra["layer"] = "sheet"
        L.append(ref("th-" + p["id"], p["id"], region=p["region"], **extra))
    L.append(own(cyl("flue-gas", "boiler", "steel-sooted", 0.5, "flue-spigot", "y", (-0.25, -0.2), (TOPC + 0.12, TOPC + {50: 0.17, 100: 0.18, 200: 0.19, 500: 0.2}[scale]), 0.04, role="detail", tags=["flue-gas"]), id="th-flue-gas", region="flue-spigot"))
    if scale == 50:
        L.append(own(box("insulation", "boiler", "mineral-wool", 0.9, "casing", (-0.37, 0.37), (PT + 0.075, TOPC - 0.005), (-0.495, 0.095)), id="th-insulation", region="casing", layer="insulation"))
    if scale == 500:
        b = "boiler"
        L.append(own(box("connections", b, "steel-rusty", 0.4, "connections", (-0.378, -0.375), (0.6, 1.0), (-0.42, -0.3), sheet=True), id="th-connections", region="connections"))
        L.append(own(box("hose", b, "pvc-spiral", 0.64, "suction-hose", (-0.423, -0.42), (1.06, 1.36), (0.25, 0.33), sheet=True), id="th-hose", region="suction-hose"))
        for s, (zc, top) in STRAPS.items():
            for n, xx in (("front", (0.375, 0.377)), ("back", (-0.377, -0.375))):
                L.append(own(box(f"strap-{s}-{n}", f"strap-{s}-{n}", "polyester-webbing", 0.62, "straps", xx, (0.3, 1.1), (zc - 0.0375, zc + 0.0375), sheet=True), id=f"th-strap-{s}-{n}", region="straps"))
    return L


def structure_list(scale, plist):
    """🏗 The load path of one scale: runners as supports, blocks as joints, deck and cross boards as the pallet member, the boiler feet as joints into the casing member, the eye bolts and chains of the sling, and the three centres of mass."""
    S = []
    ids = {p["id"]: p for p in plist}
    anchors = {"front": "pallet-runner-front", "middle": "pallet-runner-middle", "back": "pallet-runner-back"}
    for p in plist:
        i = p["id"]
        if i.startswith("bottom-board-") or i.startswith("runner-"):
            S.append(ref("st-" + i, i, member="pallet", part="support", anchor=anchors[i.split("-")[-1]]))
        elif i.startswith("block-") or i.startswith("cross-board-"):
            S.append(ref("st-" + i, i, member="pallet", part="joint" if i.startswith("block-") else "member"))
        elif i.startswith("deck") or i == "pallet":
            S.append(ref("st-" + i, i, member="pallet", part="member"))
        elif i.startswith("foot-"):
            S.append(ref("st-" + i, i, member="boiler", part="joint"))
        elif i in ("base-frame", "casing", "hopper"):
            S.append(ref("st-" + i, i, member="boiler", part="member"))
        elif i.startswith("eye"):
            S.append(ref("st-" + i, i, member="sling", part="joint"))
        elif i.startswith("chain-"):
            S.append(ref("st-" + i, i, member=p["bone"], part="member"))
    if scale == 500:
        for n, x in (("front", (0.3, 0.4)), ("middle", (-0.0725, 0.0725)), ("back", (-0.4, -0.3))):
            S.append(own(box(f"support-{n}", "pallet", "spruce-weathered", 0.5, "pallet", x, (0.0, 0.075), (-0.6, 0.6)), id=f"st-support-{n}", member="pallet", part="support", anchor=anchors[n]))
    mm = {50: 0.012, 100: 0.016, 200: 0.02, 500: 0.04}[scale]
    for mid, bn, c in (("pallet", "pallet", (0.0, 0.075, 0.0)), ("boiler", "boiler", (-0.015, PT + 0.5, -0.1)), ("sling", "sling", (0.0, TOPC + 0.011, -0.1))):
        o = W(bn)
        S.append({"id": f"mass-{mid}", "kind": "ellipsoid", "bone": bn, "material": "alloy-steel", "role": "detail", "tone": 0.5, "radii": [mm, mm, mm], "transform": {"translation": r5([c[0] - o[0], c[1] - o[1], c[2] - o[2]])}, "member": bn, "part": "mass"})
    return S


def volume(scale, decisions):
    """🧊 The authored volume of one scale."""
    plist = parts(scale)
    body = [solid_of(p) for p in plist]
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, plist), "structure": structure_list(scale, plist), "decisions": decisions}


if __name__ == "__main__":
    from author_texts import ASSET_TEXTS, REP_DECISIONS, VOL_DECISIONS
    from author_physics import materials, thermal, poses, clips, anchors, structure, references, sling_angles
    areas = json.load(open(sys.argv[1])) if len(sys.argv) > 1 else {}
    rig = {"kind": "mechanical", "root": "site", "bones": [BONES[k]["def"] for k in BONES]}
    asset = {
        "id": SLUG, "category": "components",
        "title": T("Used Wood Pellet Boiler", "Gebrauchter Pelletkessel"),
        "summary": ASSET_TEXTS["summary"],
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "pellet-boiler", "boiler", "biomass", "wood-pellets", "hopper", "flue", "sheet-steel", "pallet", "ratchet-strap", "chain-sling"],
        "provenance": {"author": "Semio staffage component authors (used HVAC wave, author 07)", "method": "handcrafted", "created": "2026-11-22", "references": references(), "notes": ASSET_TEXTS["notes"]},
        "dimensions": {"heightM": 1.454, "widthM": 0.941, "depthM": 1.2, "massKg": 284.2},
        "materials": materials(),
        "rig": rig,
        "volumes": {str(s): volume(s, VOL_DECISIONS[s]) for s in (50, 100, 200, 500)},
        "anchors": anchors(W),
        "representations": {str(s): representation(s, REP_DECISIONS[s]) for s in (50, 100, 200, 500)},
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal(areas),
        "structure": structure(),
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print("[DEBUG] written", sum(len(v["body"]) for v in asset["volumes"].values()), "body solids")
