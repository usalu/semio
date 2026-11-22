"""🛏️ Authoritative chisel of the asset `used-bedside-table`: every hand-chosen number of the 1970s teak-veneer bedside table on its folded moving blanket, and the writer of `🔣️.json` next to this file.

Run: `python3 author-used-bedside-table.py` (writes `🔣️.json`, prints the masses and centres it derived).

World frame of the staffage (x ahead = the drawer front, y up, z left), metres. Every bone rests at angle 0, so a bone-local point is the world point minus the bone origin.

@see ../../brief.md — the brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the contract
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent


def T(en, de):
    """🗣️ One bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯️ Rounds a coordinate to the micrometre."""
    return round(v + 0.0, 6)


BT = 0.018
BX = 0.3333
BZ = 0.375
LEG_H = 0.2
Y_LEG0 = BT
Y_BOT0, Y_BOT1 = BT + 0.2, BT + 0.216
Y_DIV0, Y_DIV1 = BT + 0.405, BT + 0.421
Y_SIDE1 = BT + 0.531
Y_TOP1 = BT + 0.55
X_BACK, X_CF, X_TF = -0.175, 0.155, 0.175
Z_S, Z_SI = 0.215, 0.199
LEG_X = (-0.135, 0.115)
LEG_Z = 0.175
DF = dict(x0=0.139, x1=0.155, y0=Y_DIV1 + 0.0015, y1=Y_SIDE1 - 0.0015, z=0.1975)
DB = dict(x0=-0.160, x1=0.139, y0=Y_DIV1, y1=Y_DIV1 + 0.090, zo=0.1975, zi=0.1875)
KNOB_Y = (DF["y0"] + DF["y1"]) / 2
KNOB_PROFILE = [(0.0, 0.007), (0.008, 0.007), (0.011, 0.012), (0.016, 0.014), (0.022, 0.013), (0.026, 0.008)]
LEG_PROFILE_50 = [(0.0, 0.0105), (0.003, 0.011), (0.184, 0.0175), (0.186, 0.0195), (0.2, 0.0195)]

ORIGIN = {"ground": (0.0, 0.0), "blanket": (-BX, 0.0), "travel": (0.0, 0.0), "lift": (0.0, 0.0), "carcass": (X_BACK, Y_TOP1), "drawer": (DB["x0"], Y_DIV1)}

DENS = {"teak-veneer-chipboard": 650, "teak-top-faded": 650, "beech-stained": 710, "pine-drawer": 520, "hardboard": 950, "teak-solid": 660, "moving-blanket": None}
BLANKET_KG = 1.4
G = 9.80665


def loc(bone, x, y):
    """📍️ World rest point to the bone-local (u, v) of `bone`."""
    ox, oy = ORIGIN[bone]
    return (r6(x - ox), r6(y - oy))


def box(id, bone, x0, x1, y0, y1, z0, z1, material, role="body", tone=0.6, **kw):
    """📦️ An axis-aligned box solid from world rest bounds, written in the frame of its bone."""
    u, v = loc(bone, (x0 + x1) / 2, (y0 + y1) / 2)
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], "transform": {"translation": [u, v, r6((z0 + z1) / 2)]}}
    s.update(kw)
    return s


def revolve_y(id, bone, x, y, z, profile, material, tone, **kw):
    """🌀️ A turned part about the vertical axis through world (x, y, z): profile of (height above y, radius)."""
    u, v = loc(bone, x, y)
    pts = [[0.0, 0.0]] + [[r6(a), r6(r)] for a, r in profile] + [[r6(profile[-1][0]), 0.0]]
    s = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "axis": [0, 1, 0], "origin": [u, v, r6(z)], "profile": pts}
    s.update(kw)
    return s


def revolve_x(id, bone, x, y, z, profile, material, tone, **kw):
    """🔩️ A turned part about the forward axis starting at world (x, y, z): profile of (length ahead, radius)."""
    u, v = loc(bone, x, y)
    pts = [[0.0, 0.0]] + [[r6(a), r6(r)] for a, r in profile] + [[r6(profile[-1][0]), 0.0]]
    s = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "axis": [1, 0, 0], "origin": [u, v, r6(z)], "profile": pts}
    s.update(kw)
    return s


def box_volume(s):
    """📐️ Volume of a box solid times its repeat count."""
    a, b, c = s["size"]
    return a * b * c * s.get("repeat", {}).get("count", 1)


def revolve_volume(s):
    """🏺️ Volume of an open revolve profile closed through the axis (frusta between profile points)."""
    p = s["profile"][1:-1]
    vol = 0.0
    for (a0, r0), (a1, r1) in zip(p, p[1:]):
        vol += math.pi * (a1 - a0) / 3 * (r0 * r0 + r0 * r1 + r1 * r1)
    return vol * s.get("repeat", {}).get("count", 1)


def poly_area(pts):
    """🧮️ Shoelace area of a ring."""
    return abs(sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]))) / 2


def centre(s):
    """🎯️ The local centre of a solid and its repeat copies."""
    if s["kind"] == "box":
        c = list(s["transform"]["translation"])
    elif s["kind"] == "revolve":
        o = s["origin"]
        ax = s["axis"]
        prof = s["profile"][1:-1]
        num = 0.0
        den = 0.0
        for (a0, r0), (a1, r1) in zip(prof, prof[1:]):
            v = math.pi * (a1 - a0) / 3 * (r0 * r0 + r0 * r1 + r1 * r1)
            num += v * (a0 + a1) / 2
            den += v
        d = num / den
        c = [o[0] + ax[0] * d, o[1] + ax[1] * d, o[2] + ax[2] * d]
    else:
        prof = [tuple(p) for p in s["profile"]]
        a2 = sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(prof, prof[1:] + prof[:1]))
        cx = sum((x0 + x1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(prof, prof[1:] + prof[:1])) / (3 * a2)
        cy = sum((y0 + y1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(prof, prof[1:] + prof[:1])) / (3 * a2)
        c = [cx, cy, (s["from"] + s["to"]) / 2]
    rep = s.get("repeat")
    if rep:
        n = rep["count"]
        t = rep["translation"]
        c = [c[i] + t[i] * (n - 1) / 2 for i in range(3)]
    return c


def arc(cx, cy, r, a0, a1, n):
    """⭕️ Points of a circular arc from angle a0 to a1 (degrees), both ends included."""
    return [(r6(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n))), r6(cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n)))) for k in range(n + 1)]


BLANKET_PROFILE_50 = [(0.6666, 0.0), (0.6666, 0.018), (0.009, 0.018)] + arc(0.009, 0.009, 0.009, 90, 270, 6)[1:-1] + [(0.009, 0.0)]
BLANKET_PROFILE_50 = [[r6(u), r6(v)] for u, v in BLANKET_PROFILE_50]


def blanket_extrude(id, profile, tone, **kw):
    """🧺️ The folded moving blanket as its side profile swept across the full folded width."""
    s = {"id": id, "kind": "extrude", "bone": "blanket", "material": "moving-blanket", "role": "body", "tone": tone, "plane": "xy", "profile": profile, "from": -BZ, "to": BZ, "sheet": True}
    s.update(kw)
    return s


def extrude_volume(s):
    """📏️ Volume of an extrude: profile area minus holes times length."""
    a = poly_area([tuple(p) for p in s["profile"]]) - sum(poly_area([tuple(p) for p in h]) for h in s.get("holes", []))
    return a * (s["to"] - s["from"])


def leg_solids(prefix, profile, tone):
    """🦵️ The rear and front pair of turned legs (each pair one solid repeated across the width)."""
    out = []
    for name, x in (("rear", LEG_X[0]), ("front", LEG_X[1])):
        out.append(revolve_y(f"{prefix}-{name}", "carcass", x, Y_LEG0, LEG_Z, profile, "beech-stained", tone, repeat={"count": 2, "translation": [0, 0, -2 * LEG_Z]}))
    return out


def body50():
    """🪵️ The 1:50 body: every board, the turned legs and knob, the drawer box, the burn and the folded blanket."""
    c = "carcass"
    s = [
        box("top", c, X_BACK, X_TF, Y_SIDE1, Y_TOP1, -Z_S, Z_S, "teak-top-faded", tone=0.84),
        box("sides", c, X_BACK, X_CF, Y_BOT0, Y_SIDE1, -Z_S, -Z_SI, "teak-veneer-chipboard", tone=0.7, repeat={"count": 2, "translation": [0, 0, Z_S + Z_SI]}),
        box("bottom", c, X_BACK + 0.003, X_CF, Y_BOT0, Y_BOT1, -Z_SI, Z_SI, "teak-veneer-chipboard", tone=0.62),
        box("divider", c, X_BACK + 0.003, X_CF, Y_DIV0, Y_DIV1, -Z_SI, Z_SI, "teak-veneer-chipboard", tone=0.62),
        box("back", c, X_BACK, X_BACK + 0.003, Y_BOT0, Y_SIDE1, -Z_SI, Z_SI, "hardboard", tone=0.35, sheet=True),
        *leg_solids("legs", LEG_PROFILE_50, 0.55),
        dict(box("burn", c, 0.081, 0.109, Y_TOP1 - 0.0006, Y_TOP1 + 0.0004, -0.1345, -0.1255, "teak-top-faded", role="detail", tone=0.15, texture=True), kind="ellipsoid"),
    ]
    burn = s[-1]
    burn["radii"] = [0.014, 0.0005, 0.0045]
    burn["transform"]["rotation"] = [0.0, math.sin(math.radians(12.5)), 0.0, math.cos(math.radians(12.5))]
    burn["transform"]["rotation"] = [round(q, 6) for q in burn["transform"]["rotation"]]
    del burn["size"]
    d = "drawer"
    s += [
        box("drawer-front", d, DF["x0"], DF["x1"], DF["y0"], DF["y1"], -DF["z"], DF["z"], "teak-veneer-chipboard", tone=0.66),
        box("drawer-sides", d, DB["x0"], DB["x1"], DB["y0"], DB["y1"], -DB["zo"], -DB["zi"], "pine-drawer", tone=0.78, repeat={"count": 2, "translation": [0, 0, DB["zo"] + DB["zi"]]}),
        box("drawer-back", d, DB["x0"], DB["x0"] + 0.010, DB["y0"] + 0.008, DB["y1"], -DB["zi"], DB["zi"], "pine-drawer", tone=0.78),
        box("drawer-bottom", d, DB["x0"] + 0.010, DB["x1"], DB["y0"] + 0.004, DB["y0"] + 0.008, -DB["zi"], DB["zi"], "hardboard", tone=0.4, sheet=True),
        revolve_x("knob", d, X_CF, KNOB_Y, 0.0, KNOB_PROFILE, "teak-solid", 0.5),
        blanket_extrude("blanket", BLANKET_PROFILE_50, 0.45),
    ]
    return s


def carcass_extrude(id, front_x):
    """🗄️ Sides, bottom and divider as one front-view profile swept from the back panel to the front edges, the open shelf as a hole and the drawer bay open to the top."""
    u0 = r6(X_BACK + 0.003 - ORIGIN["carcass"][0])
    u1 = r6(front_x - ORIGIN["carcass"][0])
    v = lambda y: r6(y - ORIGIN["carcass"][1])
    prof = [[v(Y_BOT0), -Z_S], [v(Y_BOT0), Z_S], [v(Y_SIDE1), Z_S], [v(Y_SIDE1), Z_SI], [v(Y_DIV1), Z_SI], [v(Y_DIV1), -Z_SI], [v(Y_SIDE1), -Z_SI], [v(Y_SIDE1), -Z_S]]
    hole = [[v(Y_BOT1), -Z_SI], [v(Y_DIV0), -Z_SI], [v(Y_DIV0), Z_SI], [v(Y_BOT1), Z_SI]]
    return {"id": id, "kind": "extrude", "bone": "carcass", "material": "teak-veneer-chipboard", "role": "body", "tone": 0.7, "plane": "yz", "profile": prof, "holes": [hole], "from": u0, "to": u1}


def body100():
    """🪚️ The 1:100 body: top with its overhang, sides, bottom and divider as one hollow profile, back sheet, plain tapered legs, drawer front, knob and box boards at the feature, blanket sheet."""
    c = "carcass"
    return [
        box("top-100", c, X_BACK, X_TF, Y_SIDE1, Y_TOP1, -Z_S, Z_S, "teak-top-faded", tone=0.84),
        carcass_extrude("carcass-100", X_CF),
        box("back-100", c, X_BACK, X_BACK + 0.003, Y_BOT0, Y_SIDE1, -Z_S, Z_S, "hardboard", tone=0.35, sheet=True),
        *leg_solids("legs-100", [(0.0, 0.011), (0.2, 0.0185)], 0.55),
        box("drawer-front-100", "drawer", DF["x0"], DF["x1"], DF["y0"], DF["y1"], -DF["z"], DF["z"], "teak-veneer-chipboard", tone=0.66),
        box("drawer-sides-100", "drawer", DB["x0"], DB["x1"], DB["y0"], DB["y1"], -DB["zo"], -DB["zo"] + 0.015, "pine-drawer", tone=0.78, repeat={"count": 2, "translation": [0, 0, 2 * DB["zo"] - 0.015]}),
        box("drawer-back-100", "drawer", DB["x0"], DB["x0"] + 0.015, DB["y0"] + 0.008, DB["y1"], -DB["zo"] + 0.015, DB["zo"] - 0.015, "pine-drawer", tone=0.78),
        box("drawer-bottom-100", "drawer", DB["x0"] + 0.015, DB["x1"], DB["y0"] + 0.004, DB["y0"] + 0.008, -DB["zo"] + 0.015, DB["zo"] - 0.015, "hardboard", tone=0.4, sheet=True),
        revolve_x("knob-100", "drawer", X_CF, KNOB_Y, 0.0, [(0.0, 0.0075), (0.01, 0.0075), (0.016, 0.014), (0.026, 0.0115)], "teak-solid", 0.5),
        box("blanket-100", "blanket", -BX, BX, 0.0, BT, -BZ, BZ, "moving-blanket", tone=0.45, sheet=True),
    ]


def body200():
    """🧱️ The 1:200 body: top flush with the carcass front, the carcass profile, back sheet, legs at the 30 mm feature, the drawer as one block, the blanket as a plain sheet."""
    c = "carcass"
    return [
        box("top-200", c, X_BACK, X_CF, Y_SIDE1, Y_TOP1, -Z_S, Z_S, "teak-top-faded", tone=0.84),
        carcass_extrude("carcass-200", X_CF),
        box("back-200", c, X_BACK, X_BACK + 0.003, Y_BOT0, Y_SIDE1, -Z_S, Z_S, "hardboard", tone=0.35, sheet=True),
        *leg_solids("legs-200", [(0.0, 0.015), (0.2, 0.018)], 0.55),
        box("drawer-200", "drawer", DB["x0"], DF["x1"], DF["y0"], DF["y1"], -DF["z"], DF["z"], "teak-veneer-chipboard", tone=0.66),
        box("blanket-200", "blanket", -BX, BX, 0.0, BT, -BZ, BZ, "moving-blanket", tone=0.45, sheet=True),
    ]


def mass_of(solids):
    """⚖️ Masses and centres per bone from section × density (the blanket from its stated mass)."""
    out = {}
    for s in solids:
        if s.get("texture"):
            continue
        if s["kind"] == "box":
            vol = box_volume(s)
        elif s["kind"] == "revolve":
            vol = revolve_volume(s)
        else:
            vol = extrude_volume(s)
        rho = DENS[s["material"]]
        kg = BLANKET_KG if rho is None else vol * rho
        c = centre(s)
        acc = out.setdefault(s["bone"], [0.0, 0.0, 0.0, 0.0, {}])
        acc[0] += kg
        acc[1] += kg * c[0]
        acc[2] += kg * c[1]
        acc[3] += kg * c[2]
        acc[4][s["id"]] = round(kg, 4)
    return {b: {"kg": a[0], "comX": a[1] / a[0], "comY": a[2] / a[0], "comZ": a[3] / a[0], "parts": a[4]} for b, a in out.items()}


B50 = body50()
MASS = mass_of(B50)
TOTAL_KG = sum(m["kg"] for m in MASS.values())
WB = MASS["blanket"]["kg"] * G


def ellipsoid(id, bone, at, r, material):
    """⚫️ A compact mass marker at a centre of mass."""
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [r6(at[0]), r6(at[1]), r6(at[2])]}, "member": bone, "part": "mass"}


def masses(r):
    """🎯️ The three mass markers of a scale."""
    return [ellipsoid("mass-" + b, b, (MASS[b]["comX"], MASS[b]["comY"], MASS[b]["comZ"]), r if b != "blanket" else min(r, 0.0085), {"carcass": "teak-veneer-chipboard", "drawer": "pine-drawer", "blanket": "moving-blanket"}[b]) for b in ("carcass", "drawer", "blanket")]


def ref(id, of, **kw):
    """🔗️ A thermal or structural reference to a body solid of the same scale."""
    s = {"id": id, "of": of}
    s.update(kw)
    return s


def plates(prefix, h):
    """🔩️ The leg mounting plates with their hanger bolts as joint solids under the bottom panel."""
    out = []
    for name, x in (("rear", LEG_X[0]), ("front", LEG_X[1])):
        out.append(dict(box(f"{prefix}-{name}", "carcass", x - 0.025, x + 0.025, Y_BOT0 - h / 2, Y_BOT0 + h / 2, LEG_Z - 0.025, LEG_Z + 0.025, "teak-veneer-chipboard", role="detail", tone=0.3, repeat={"count": 2, "translation": [0, 0, -2 * LEG_Z]}), member="carcass", part="joint"))
    return out


def structure50():
    """🏗️ The 1:50 load path: boards, legs, drawer box and blanket as members; leg plates, top dowels and drawer runners as joints; three mass markers."""
    st = [ref("m-top", "top", member="carcass", part="member"), ref("m-sides", "sides", member="carcass", part="member"), ref("m-bottom", "bottom", member="carcass", part="member"), ref("m-divider", "divider", member="carcass", part="member"), ref("m-back", "back", member="carcass", part="member"), ref("m-legs-rear", "legs-rear", member="carcass", part="member"), ref("m-legs-front", "legs-front", member="carcass", part="member")]
    st += plates("j-leg-plates", 0.010)
    for name, x in (("rear", X_BACK + 0.04), ("front", X_CF - 0.03)):
        st.append(dict(box(f"j-top-dowels-{name}", "carcass", x - 0.004, x + 0.004, Y_SIDE1 - 0.016, Y_SIDE1 + 0.016, -Z_S + 0.004, -Z_S + 0.012, "beech-stained", role="detail", tone=0.3, repeat={"count": 2, "translation": [0, 0, 2 * Z_S - 0.016]}), member="carcass", part="joint"))
    st += [ref("m-drawer-front", "drawer-front", member="drawer", part="member"), ref("m-drawer-sides", "drawer-sides", member="drawer", part="member"), ref("m-drawer-back", "drawer-back", member="drawer", part="member"), ref("m-drawer-bottom", "drawer-bottom", member="drawer", part="member")]
    st.append(dict(box("j-runners", "drawer", DB["x0"] + 0.01, DB["x1"] - 0.01, DB["y0"], DB["y0"] + 0.008, -DB["zo"], -DB["zi"], "pine-drawer", role="detail", tone=0.3, repeat={"count": 2, "translation": [0, 0, DB["zo"] + DB["zi"]]}), member="drawer", part="joint"))
    st.append(ref("m-blanket", "blanket", member="blanket", part="member"))
    st += masses(0.012)
    return st


def structure100():
    """🏗️ The 1:100 load path at the 15 mm feature."""
    st = [ref("m-top-100", "top-100", member="carcass", part="member"), ref("m-carcass-100", "carcass-100", member="carcass", part="member"), ref("m-back-100", "back-100", member="carcass", part="member"), ref("m-legs-100-rear", "legs-100-rear", member="carcass", part="member"), ref("m-legs-100-front", "legs-100-front", member="carcass", part="member")]
    st += plates("j-leg-plates-100", 0.016)
    st += [ref("m-drawer-front-100", "drawer-front-100", member="drawer", part="member"), ref("m-drawer-sides-100", "drawer-sides-100", member="drawer", part="member"), ref("m-drawer-back-100", "drawer-back-100", member="drawer", part="member"), ref("m-drawer-bottom-100", "drawer-bottom-100", member="drawer", part="member")]
    st.append(dict(box("j-runners-100", "drawer", DB["x0"] + 0.02, DB["x1"] - 0.02, DB["y0"], DB["y0"] + 0.016, -DB["zo"], -DB["zo"] + 0.016, "pine-drawer", role="detail", tone=0.3, repeat={"count": 2, "translation": [0, 0, 2 * DB["zo"] - 0.016]}), member="drawer", part="joint"))
    st.append(ref("m-blanket-100", "blanket-100", member="blanket", part="member"))
    st += [dict(m, id=m["id"] + "-100") for m in masses(0.009)]
    return st


def structure200():
    """🏗️ The 1:200 load path at the 30 mm feature."""
    st = [ref("m-top-200", "top-200", member="carcass", part="member"), ref("m-carcass-200", "carcass-200", member="carcass", part="member"), ref("m-back-200", "back-200", member="carcass", part="member"), ref("m-legs-200-rear", "legs-200-rear", member="carcass", part="member"), ref("m-legs-200-front", "legs-200-front", member="carcass", part="member")]
    st += plates("j-leg-plates-200", 0.030)
    st += [ref("m-drawer-200", "drawer-200", member="drawer", part="member"), ref("m-blanket-200", "blanket-200", member="blanket", part="member")]
    st += [dict(m, id=m["id"] + "-200", radii=[0.016, 0.016, 0.016]) if m["member"] != "blanket" else dict(m, id=m["id"] + "-200", radii=[0.03, 0.009, 0.03]) for m in masses(0.016)]
    return st


def thermal50():
    """🌡️ The 1:50 thermal model: every body solid in its region (the burn is a texture of the top and carries no area)."""
    reg = {"top": "top", "sides": "carcass", "bottom": "carcass", "divider": "carcass", "back": "back-panel", "legs-rear": "legs", "legs-front": "legs", "drawer-front": "drawer", "knob": "drawer", "drawer-sides": "drawer", "drawer-back": "drawer", "drawer-bottom": "drawer", "blanket": "blanket"}
    return [ref("th-" + k, k, region=v) for k, v in reg.items()]


def thermal100():
    """🌡️ The 1:100 thermal model."""
    reg = {"top-100": "top", "carcass-100": "carcass", "back-100": "back-panel", "legs-100-rear": "legs", "legs-100-front": "legs", "drawer-front-100": "drawer", "knob-100": "drawer", "drawer-sides-100": "drawer", "drawer-back-100": "drawer", "drawer-bottom-100": "drawer", "blanket-100": "blanket"}
    return [ref("th-" + k, k, region=v) for k, v in reg.items()]


def thermal200():
    """🌡️ The 1:200 thermal model: every body solid in its region, the drawer block in the drawer region."""
    reg = {"top-200": "top", "carcass-200": "carcass", "back-200": "back-panel", "legs-200-rear": "legs", "legs-200-front": "legs", "drawer-200": "drawer", "blanket-200": "blanket"}
    return [ref("th-" + k, k, region=v) for k, v in reg.items()]


def sh(id, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None):
    """✏️ One 2D shape."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    return s


def erect(bone, x0, y0, x1, y1):
    """⬛️ An elevation rectangle from world rest corners, in the frame of its bone."""
    u0, v0 = loc(bone, x0, y0)
    return {"kind": "rect", "x": u0, "y": v0, "w": r6(x1 - x0), "h": r6(y1 - y0)}


def epoly(bone, pts, closed=True):
    """🔷️ An elevation polygon from world rest points, in the frame of its bone."""
    return {"kind": "poly", "points": [list(loc(bone, x, y)) for x, y in pts], "closed": closed}


def wrect(a0, b0, a1, b1):
    """⬜️ A front or plan rectangle in world rest coordinates (front: z, y; plan: x, z)."""
    return {"kind": "rect", "x": r6(a0), "y": r6(b0), "w": r6(a1 - a0), "h": r6(b1 - b0)}


def wpoly(pts, closed=True):
    """🔶️ A front or plan polygon in world rest coordinates."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}


def ring_path(rings):
    """〰️ A path of several closed rings."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in ring) + " Z" for ring in rings)}


def leg_outline(cx, profile, y0):
    """🦵️ The side outline of a turned leg about the vertical axis at cx."""
    right = [(cx + r, y0 + a) for a, r in profile]
    left = [(cx - r, y0 + a) for a, r in reversed(profile)]
    return right + left


def knob_outline(x0, yc, profile):
    """🔘️ The side (or top) outline of the turned knob."""
    upper = [(x0 + a, yc + r) for a, r in profile]
    lower = [(x0 + a, yc - r) for a, r in reversed(profile)]
    return [(x0, yc + profile[0][1])] + upper + lower + [(x0, yc - profile[0][1])]


def blanket_side(profile):
    """🧺️ The side outline of the folded blanket in world rest coordinates."""
    return [(u - BX, v) for u, v in profile]


LEG100 = [(0.0, 0.011), (0.2, 0.0185)]
LEG200 = [(0.0, 0.015), (0.2, 0.018)]
KNOB100 = [(0.0, 0.0075), (0.01, 0.0075), (0.016, 0.014), (0.026, 0.0115)]


def rep50():
    """🖋️ The 1:50 drawing: every board, the veneer grain, the turned legs and knob, the burn, the glass rings, the chip and the scuffs."""
    c, d = "carcass", "drawer"
    el = [
        sh("blanket", "blanket", "body", 1, True, 0.45, epoly("blanket", blanket_side(BLANKET_PROFILE_50)), region="blanket", material="moving-blanket", hatch="felt"),
        sh("blanket-layers", "blanket", "texture", 1.5, False, 0.3, {"kind": "path", "d": f"M {r6(0.03)} 0.006 L 0.6566 0.006 M {r6(0.03)} 0.012 L 0.6566 0.012"}, region="blanket"),
        sh("drawer-box", d, "body", 2, True, 0.78, erect(d, DB["x0"], DB["y0"], DB["x1"], DB["y1"]), region="drawer", material="pine-drawer"),
        sh("drawer-front", d, "body", 2.2, True, 0.66, erect(d, DF["x0"], DF["y0"], DF["x1"], DF["y1"]), region="drawer", material="teak-veneer-chipboard"),
        sh("shelf-back", c, "body", 2.4, True, 0.3, erect(c, X_BACK, Y_BOT0, X_BACK + 0.003, Y_SIDE1), region="back-panel", material="hardboard"),
        sh("leg-rear", c, "body", 3, True, 0.55, epoly(c, leg_outline(LEG_X[0], LEG_PROFILE_50, Y_LEG0)), region="legs", material="beech-stained", hatch="grain-v"),
        sh("leg-front", c, "body", 3, True, 0.55, epoly(c, leg_outline(LEG_X[1], LEG_PROFILE_50, Y_LEG0)), region="legs", material="beech-stained", hatch="grain-v"),
        sh("leg-collars", c, "detail", 3.5, False, 0.3, {"kind": "path", "d": " ".join(f"M {r6(x - 0.0195 - X_BACK)} {r6(Y_LEG0 + 0.186 - Y_TOP1)} L {r6(x + 0.0195 - X_BACK)} {r6(Y_LEG0 + 0.186 - Y_TOP1)}" for x in LEG_X)}, region="legs"),
        sh("leg-scuffs", c, "texture", 3.6, False, 0.3, {"kind": "path", "d": " ".join(f"M {r6(x - 0.009 - X_BACK)} {r6(Y_LEG0 + 0.008 - Y_TOP1)} L {r6(x + 0.006 - X_BACK)} {r6(Y_LEG0 + 0.014 - Y_TOP1)}" for x in LEG_X)}, region="legs"),
        sh("side", c, "body", 4, True, 0.7, erect(c, X_BACK, Y_BOT0, X_CF, Y_SIDE1), region="carcass", material="teak-veneer-chipboard", hatch="veneer"),
        sh("carcass-front", c, "body", 4.2, True, 0.62, erect(c, X_CF - 0.016, Y_BOT0, X_CF, Y_SIDE1), region="carcass", material="teak-veneer-chipboard"),
        sh("top", c, "body", 5, True, 0.84, erect(c, X_BACK, Y_SIDE1, X_TF, Y_TOP1), region="top", material="teak-top-faded"),
        sh("top-front", c, "body", 5.2, True, 0.84, erect(c, X_TF - 0.01, Y_SIDE1, X_TF, Y_TOP1), region="top", material="teak-top-faded"),
        sh("top-chip", c, "detail", 5.5, False, 0.25, epoly(c, [(X_TF - 0.013, Y_TOP1), (X_TF - 0.010, Y_TOP1 - 0.003), (X_TF, Y_TOP1 - 0.003)], closed=False), region="top"),
        sh("knob", d, "body", 6, True, 0.5, epoly(d, knob_outline(X_CF, KNOB_Y, KNOB_PROFILE)), region="drawer", material="teak-solid", pen=0.25),
    ]
    fr = [
        sh("blanket", "blanket", "body", 1, True, 0.45, wrect(-BZ, 0, BZ, BT), region="blanket", material="moving-blanket", hatch="felt"),
        sh("blanket-layers", "blanket", "texture", 1.5, False, 0.3, {"kind": "path", "d": f"M {-BZ + 0.02} 0.006 L {BZ - 0.02} 0.006 M {-BZ + 0.02} 0.012 L {BZ - 0.02} 0.012"}, region="blanket"),
        sh("leg-front", c, "body", 2, True, 0.55, ring_path([leg_outline(z, LEG_PROFILE_50, Y_LEG0) for z in (-LEG_Z, LEG_Z)]), region="legs", material="beech-stained", hatch="grain-v"),
        sh("leg-collars", c, "detail", 2.5, False, 0.3, {"kind": "path", "d": " ".join(f"M {r6(z - 0.0195)} {r6(Y_LEG0 + 0.186)} L {r6(z + 0.0195)} {r6(Y_LEG0 + 0.186)}" for z in (-LEG_Z, LEG_Z))}, region="legs"),
        sh("shelf-back", c, "body", 3, True, 0.3, wrect(-Z_SI, Y_BOT1, Z_SI, Y_SIDE1), region="back-panel", material="hardboard"),
        sh("carcass-front", c, "body", 4, True, 0.62, ring_path([[(-Z_S, Y_BOT0), (-Z_SI, Y_BOT0), (-Z_SI, Y_SIDE1), (-Z_S, Y_SIDE1)], [(Z_SI, Y_BOT0), (Z_S, Y_BOT0), (Z_S, Y_SIDE1), (Z_SI, Y_SIDE1)], [(-Z_SI, Y_BOT0), (Z_SI, Y_BOT0), (Z_SI, Y_BOT1), (-Z_SI, Y_BOT1)], [(-Z_SI, Y_DIV0), (Z_SI, Y_DIV0), (Z_SI, Y_DIV1), (-Z_SI, Y_DIV1)]]), region="carcass", material="teak-veneer-chipboard"),
        sh("drawer-front", d, "body", 5, True, 0.66, wrect(-DF["z"], DF["y0"], DF["z"], DF["y1"]), region="drawer", material="teak-veneer-chipboard", hatch="veneer"),
        sh("drawer-wear", d, "texture", 5.3, False, 0.3, {"kind": "path", "d": f"M -0.06 {r6(DF['y0'] + 0.004)} L 0.07 {r6(DF['y0'] + 0.004)}"}, region="drawer"),
        sh("knob", d, "body", 6, True, 0.5, {"kind": "ellipse", "cx": 0.0, "cy": r6(KNOB_Y), "rx": 0.014, "ry": 0.014}, region="drawer", material="teak-solid"),
        sh("knob-centre", d, "texture", 6.5, False, 0.3, {"kind": "ellipse", "cx": 0.0, "cy": r6(KNOB_Y), "rx": 0.006, "ry": 0.006}, region="drawer"),
        sh("top-front", c, "body", 7, True, 0.84, wrect(-Z_S, Y_SIDE1, Z_S, Y_TOP1), region="top", material="teak-top-faded"),
        sh("top-chip", c, "detail", 7.5, False, 0.25, wpoly([(-Z_S, Y_TOP1 - 0.003), (-Z_S + 0.012, Y_TOP1 - 0.003), (-Z_S + 0.015, Y_TOP1)], closed=False), region="top"),
    ]
    quilt = " ".join(f"M {r6(-BX + 0.02)} {r6(zz)} L {r6(BX - 0.02)} {r6(zz)}" for zz in (-0.25, -0.125, 0.0, 0.125, 0.25))
    pl = [
        sh("blanket", "blanket", "body", 1, True, 0.45, wrect(-BX, -BZ, BX, BZ), region="blanket", material="moving-blanket", hatch="felt"),
        sh("blanket-quilting", "blanket", "texture", 1.5, False, 0.3, {"kind": "path", "d": quilt}, region="blanket"),
        sh("drawer-box", d, "body", 2, True, 0.78, wrect(DB["x0"], -DB["zo"], DB["x1"], DB["zo"]), region="drawer", material="pine-drawer"),
        sh("drawer-front", d, "body", 2.2, True, 0.66, wrect(DF["x0"], -DF["z"], DF["x1"], DF["z"]), region="drawer", material="teak-veneer-chipboard"),
        sh("knob", d, "body", 2.4, True, 0.5, wpoly(knob_outline(X_CF, 0.0, KNOB_PROFILE)), region="drawer", material="teak-solid", pen=0.25),
        sh("top", c, "body", 4, True, 0.84, wrect(X_BACK, -Z_S, X_TF, Z_S), region="top", material="teak-top-faded", hatch="veneer-plan"),
        sh("lamp-shadow", c, "texture", 4.4, False, 0.3, {"kind": "ellipse", "cx": -0.075, "cy": 0.105, "rx": 0.07, "ry": 0.07}, region="top"),
        sh("glass-rings", c, "texture", 4.5, False, 0.3, {"kind": "path", "d": "M 0.098 0.12 A 0.034 0.034 0 1 0 0.03 0.12 A 0.034 0.034 0 1 0 0.098 0.12 Z M 0.124 0.07 A 0.034 0.034 0 1 0 0.056 0.07 A 0.034 0.034 0 1 0 0.124 0.07 Z"}, region="top"),
        sh("burn", c, "detail", 4.6, True, 0.15, {"kind": "ellipse", "cx": 0.095, "cy": -0.13, "rx": 0.014, "ry": 0.0045, "rotationDeg": 25}, region="top", material="teak-top-faded"),
        sh("burn-halo", c, "texture", 4.7, False, 0.3, {"kind": "ellipse", "cx": 0.095, "cy": -0.13, "rx": 0.02, "ry": 0.008, "rotationDeg": 25}, region="top"),
        sh("top-chip", c, "detail", 4.8, False, 0.25, wpoly([(X_TF, -Z_S + 0.015), (X_TF - 0.003, -Z_S + 0.012), (X_TF - 0.003, -Z_S)], closed=False), region="top"),
    ]
    return {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7},
            "hatches": [{"id": "veneer", "title": T("Teak veneer grain", "Teakfurnier-Maserung"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.7, "penMm": 0.13},
                        {"id": "veneer-plan", "title": T("Teak veneer grain across the top", "Teakfurnier-Maserung quer über die Platte"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.9, "penMm": 0.13},
                        {"id": "grain-v", "title": T("Beech grain along the legs", "Buchenmaserung längs der Beine"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.5, "penMm": 0.13},
                        {"id": "felt", "title": T("Needle felt of the moving blanket", "Nadelfilz der Möbeldecke"), "pattern": "dots", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.13}],
            "geometry": {"elevation": el, "front": fr, "plan": pl},
            "decisions": {
                "silhouette": T("Table 7.0 x 11.4 mm on a 13.3 mm folded blanket with its whole construction: the 19 mm top with its 20 mm front overhang, the 16 mm side with its veneer, the inset drawer behind the side, the turned knob under the overhang and two tapered beech legs with their turned collars. Front view: the top edge, both side edges, bottom and divider edges, the drawer front with its 1.5 mm reveal, the knob and the open shelf with the hardboard back behind it. Plan: the sun-faded top with its grain, the knob just showing in front of it, the blanket with its quilting.", "Tisch 7,0 x 11,4 mm auf einer 13,3 mm breiten gefalteten Decke mit seiner ganzen Konstruktion: die 19-mm-Platte mit 20 mm Überstand vorn, die 16-mm-Seite mit Furnier, die eingelassene Schublade hinter der Seite, der gedrechselte Knauf unter dem Überstand und zwei konische Buchenbeine mit gedrechseltem Bund. Vorderansicht: Plattenkante, beide Seitenkanten, Boden- und Zwischenbodenkante, die Schubladenfront mit 1,5 mm Fuge, der Knauf und das offene Fach mit der Hartfaserrückwand dahinter. Grundriss: die sonnengebleichte Platte mit Maserung, der knapp davor sichtbare Knauf, die Decke mit ihrer Steppung."),
                "retained": T("The traces of use: the cigarette burn (28 x 9 mm) with its scorched halo, two glass rings, the darker unfaded circle where a lamp stood, the chipped veneer at the front-right corner, the scuffed feet and the worn lower edge of the drawer front.", "Die Gebrauchsspuren: die Zigarettenbrandstelle (28 x 9 mm) mit versengtem Hof, zwei Glasränder, der dunklere, nicht ausgebleichte Kreis, wo eine Lampe stand, das abgeplatzte Furnier an der vorderen rechten Ecke, die bestoßenen Füße und die abgegriffene Unterkante der Schubladenfront."),
                "merged": T("Left and right legs, sides and drawer sides coincide in the side view; rear and front legs coincide in the front view.", "Linke und rechte Beine, Seiten und Schubladenseiten fallen in der Seitenansicht zusammen; hintere und vordere Beine in der Vorderansicht."),
                "omitted": T("Leg mounting plates and hanger bolts, dowels, nails of the back panel, the drawer bottom grooves and the 6 mm round-over of the top edge (below 7.5 mm).", "Beinanschraubplatten und Stockschrauben, Dübel, Nägel der Rückwand, Nuten des Schubladenbodens und die 6-mm-Rundung der Plattenkante (unter 7,5 mm)."),
                "thinElements": T("The 3 mm hardboard back and the 4 mm drawer bottom keep their gauge as sheets; the blanket is 18 mm (0.36 mm).", "Die 3-mm-Hartfaserrückwand und der 4-mm-Schubladenboden behalten ihre Stärke als Platten; die Decke misst 18 mm (0,36 mm)."),
                "hatching": T("Veneer grain 0.7 mm on the side and the drawer front, 0.9 mm across the top, beech grain 0.5 mm on the legs, felt dots 0.8 mm on the blanket, all with the 0.13 mm pen.", "Furniermaserung 0,7 mm auf Seite und Schubladenfront, 0,9 mm quer über die Platte, Buchenmaserung 0,5 mm auf den Beinen, Filzpunkte 0,8 mm auf der Decke, alle mit dem 0,13-mm-Stift."),
                "styles": T("Faded top 0.84, side 0.7, drawer front 0.66, edges 0.62, legs 0.55, knob 0.5, blanket 0.45, shelf back 0.3 (in the shadow of the open shelf), burn 0.15; the knob is drawn with the 0.25 mm pen so that its 41 mm clearance under the overhang still prints at 1:75.", "Gebleichte Platte 0,84, Seite 0,7, Schubladenfront 0,66, Kanten 0,62, Beine 0,55, Knauf 0,5, Decke 0,45, Rückwand 0,3 (im Schatten des offenen Fachs), Brandstelle 0,15; der Knauf ist mit dem 0,25-mm-Stift gezeichnet, damit sein Abstand von 41 mm unter dem Überstand auch bei 1:75 druckt."),
                "overlays": T("One shape per region and part; the drawer box and the drawer front lie behind the side and under the top and show when the drawer is pulled out.", "Eine Form je Region und Teil; Schubladenkasten und Schubladenfront liegen hinter der Seite und unter der Platte und erscheinen, wenn die Schublade gezogen wird."),
            }}


def rep100():
    """🖋️ The 1:100 drawing: boards as outlines, plain tapered legs, knob in the front view only, the burn as one dot."""
    c, d = "carcass", "drawer"
    el = [
        sh("blanket", "blanket", "body", 1, True, 0.45, erect("blanket", -BX, 0, BX, BT), region="blanket", material="moving-blanket"),
        sh("drawer-box", d, "body", 2, True, 0.78, erect(d, DB["x0"], DB["y0"], DB["x1"], DB["y1"]), region="drawer", material="pine-drawer"),
        sh("drawer-front", d, "body", 2.2, True, 0.66, erect(d, DF["x0"], DF["y0"], DF["x1"], DF["y1"]), region="drawer", material="teak-veneer-chipboard"),
        sh("shelf-back", c, "body", 2.4, True, 0.3, erect(c, X_BACK, Y_BOT0, X_BACK + 0.003, Y_SIDE1), region="back-panel", material="hardboard"),
        sh("leg-rear", c, "body", 3, True, 0.55, epoly(c, leg_outline(LEG_X[0], LEG100, Y_LEG0)), region="legs", material="beech-stained"),
        sh("leg-front", c, "body", 3, True, 0.55, epoly(c, leg_outline(LEG_X[1], LEG100, Y_LEG0)), region="legs", material="beech-stained"),
        sh("side", c, "body", 4, True, 0.72, epoly(c, [(X_BACK, Y_BOT0), (X_CF, Y_BOT0), (X_CF, Y_SIDE1), (X_TF, Y_SIDE1), (X_TF, Y_TOP1), (X_BACK, Y_TOP1)]), regions=["carcass", "top"], material="teak-veneer-chipboard"),
        sh("carcass-front", c, "body", 4.2, True, 0.62, erect(c, X_CF - 0.016, Y_BOT0, X_CF, Y_SIDE1), region="carcass", material="teak-veneer-chipboard"),
        sh("top-front", c, "body", 4.4, True, 0.84, erect(c, X_TF - 0.01, Y_SIDE1, X_TF, Y_TOP1), region="top", material="teak-top-faded"),
        sh("top-line", c, "detail", 4.6, False, 0.3, epoly(c, [(X_BACK, Y_SIDE1), (X_CF, Y_SIDE1)], closed=False), region="top"),
    ]
    fr = [
        sh("blanket", "blanket", "body", 1, True, 0.45, wrect(-BZ, 0, BZ, BT), region="blanket", material="moving-blanket"),
        sh("leg-front", c, "body", 2, True, 0.55, ring_path([leg_outline(z, LEG100, Y_LEG0) for z in (-LEG_Z, LEG_Z)]), region="legs", material="beech-stained"),
        sh("shelf-back", c, "body", 3, True, 0.3, wrect(-Z_SI, Y_BOT1, Z_SI, Y_SIDE1), region="back-panel", material="hardboard"),
        sh("carcass-front", c, "body", 4, True, 0.62, ring_path([[(-Z_S, Y_BOT0), (-Z_SI, Y_BOT0), (-Z_SI, Y_SIDE1), (-Z_S, Y_SIDE1)], [(Z_SI, Y_BOT0), (Z_S, Y_BOT0), (Z_S, Y_SIDE1), (Z_SI, Y_SIDE1)], [(-Z_SI, Y_BOT0), (Z_SI, Y_BOT0), (Z_SI, Y_BOT1), (-Z_SI, Y_BOT1)], [(-Z_SI, Y_DIV0), (Z_SI, Y_DIV0), (Z_SI, Y_DIV1), (-Z_SI, Y_DIV1)]]), region="carcass", material="teak-veneer-chipboard"),
        sh("drawer-front", d, "body", 5, True, 0.66, wrect(-DF["z"], DF["y0"], DF["z"], DF["y1"]), region="drawer", material="teak-veneer-chipboard"),
        sh("knob", d, "detail", 6, True, 0.45, {"kind": "ellipse", "cx": 0.0, "cy": r6(KNOB_Y), "rx": 0.014, "ry": 0.014}, region="drawer", material="teak-solid"),
        sh("top-front", c, "body", 7, True, 0.84, wrect(-Z_S, Y_SIDE1, Z_S, Y_TOP1), region="top", material="teak-top-faded"),
    ]
    pl = [
        sh("blanket", "blanket", "body", 1, True, 0.45, wrect(-BX, -BZ, BX, BZ), region="blanket", material="moving-blanket"),
        sh("drawer-box", d, "body", 2, True, 0.78, wrect(DB["x0"], -DB["zo"], DB["x1"], DB["zo"]), region="drawer", material="pine-drawer"),
        sh("drawer-front", d, "body", 2.2, True, 0.66, wrect(DF["x0"], -DF["z"], DF["x1"], DF["z"]), region="drawer", material="teak-veneer-chipboard"),
        sh("knob", d, "body", 2.4, True, 0.5, wpoly(knob_outline(X_CF, 0.0, KNOB100)), region="drawer", material="teak-solid"),
        sh("top", c, "body", 4, True, 0.84, wrect(X_BACK, -Z_S, X_TF, Z_S), region="top", material="teak-top-faded"),
        sh("burn", c, "detail", 4.6, True, 0.15, {"kind": "ellipse", "cx": 0.095, "cy": -0.13, "rx": 0.016, "ry": 0.008, "rotationDeg": 25}, region="top", material="teak-top-faded"),
    ]
    return {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5},
            "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl},
            "decisions": {
                "silhouette": T("Table 3.5 x 5.7 mm on a 6.7 mm blanket: top with its front overhang and side as one outline with the board line between them, two plain tapered legs, the drawer front and the open shelf in the front view, the top and the knob in plan.", "Tisch 3,5 x 5,7 mm auf einer 6,7 mm breiten Decke: Platte mit Überstand vorn und Seite als ein Umriss mit der Plattenfuge dazwischen, zwei glatte konische Beine, Schubladenfront und offenes Fach in der Vorderansicht, Platte und Knauf im Grundriss."),
                "retained": T("The open shelf with its dark back, the drawer reveal, the 20 mm front overhang and the cigarette burn as one dark dot on the top.", "Das offene Fach mit dunkler Rückwand, die Schubladenfuge, der Überstand von 20 mm vorn und die Zigarettenbrandstelle als ein dunkler Punkt auf der Platte."),
                "merged": T("Side and top in one side outline (regions carcass and top); sides, bottom and divider in one front profile.", "Seite und Platte in einem Seitenumriss (Regionen Korpus und Platte); Seiten, Boden und Zwischenboden in einem Frontprofil."),
                "omitted": T("Leg collars, scuffs, grain, glass rings, the lamp circle and the chip (all under 0.15 mm); the knob in the side view, whose 41 mm clearance under the overhang would print as a 0.03 to 0.3 mm white sliver between 1:75 and 1:150: the front view and the plan carry it.", "Beinbunde, Stoßspuren, Maserung, Glasränder, der Lampenkreis und die Abplatzung (alle unter 0,15 mm); der Knauf in der Seitenansicht, dessen 41-mm-Abstand unter dem Überstand zwischen 1:75 und 1:150 als weißer Spalt von 0,03 bis 0,3 mm drucken würde: Vorderansicht und Grundriss zeigen ihn."),
                "thinElements": T("Top 19 mm, sides 16 mm and drawer front 16 mm are inked by the 0.35 mm pen; the hardboard back keeps its 3 mm as a sheet.", "Platte 19 mm, Seiten 16 mm und Schubladenfront 16 mm trägt der 0,35-mm-Stift; die Hartfaserrückwand behält ihre 3 mm als Platte."),
                "hatching": T("None.", "Keine."),
                "styles": T("Top 0.84, side 0.72, drawer 0.66, edges 0.62, legs 0.55, blanket 0.45, knob 0.45, shelf back 0.3, burn 0.15.", "Platte 0,84, Seite 0,72, Schublade 0,66, Kanten 0,62, Beine 0,55, Decke 0,45, Knauf 0,45, Rückwand 0,3, Brandstelle 0,15."),
                "overlays": T("The side outline names carcass and top; drawer box and front lie hidden behind the side and show when the drawer is out.", "Der Seitenumriss nennt Korpus und Platte; Schubladenkasten und Front liegen verdeckt hinter der Seite und erscheinen bei gezogener Schublade."),
            }}


def rep200():
    """🖋️ The 1:200 drawing: one carcass outline with the top as its band, the drawer as a block, legs at the 30 mm feature, no knob."""
    c, d = "carcass", "drawer"
    el = [
        sh("blanket", "blanket", "body", 1, True, 0.45, erect("blanket", -BX, 0, BX, BT), region="blanket", material="moving-blanket"),
        sh("drawer-block", d, "body", 2, True, 0.66, erect(d, DB["x0"], DF["y0"], DF["x1"], DF["y1"]), region="drawer", material="teak-veneer-chipboard"),
        sh("leg-rear", c, "body", 3, True, 0.55, epoly(c, leg_outline(LEG_X[0], LEG200, Y_LEG0)), region="legs", material="beech-stained"),
        sh("leg-front", c, "body", 3, True, 0.55, epoly(c, leg_outline(LEG_X[1], LEG200, Y_LEG0)), region="legs", material="beech-stained"),
        sh("side", c, "body", 4, True, 0.72, erect(c, X_BACK, Y_BOT0, X_CF, Y_SIDE1), regions=["carcass", "back-panel"], material="teak-veneer-chipboard"),
        sh("top-band", c, "body", 4.5, True, 0.84, erect(c, X_BACK, Y_SIDE1, X_CF, Y_TOP1), region="top", material="teak-top-faded"),
    ]
    fr = [
        sh("blanket", "blanket", "body", 1, True, 0.45, wrect(-BZ, 0, BZ, BT), region="blanket", material="moving-blanket"),
        sh("leg-front", c, "body", 2, True, 0.55, ring_path([leg_outline(z, LEG200, Y_LEG0) for z in (-LEG_Z, LEG_Z)]), region="legs", material="beech-stained"),
        sh("carcass-front", c, "body", 3, True, 0.62, wrect(-Z_S, Y_BOT0, Z_S, Y_SIDE1), region="carcass", material="teak-veneer-chipboard"),
        sh("shelf-back", c, "body", 3.5, True, 0.3, wrect(-Z_SI, Y_BOT1, Z_SI, Y_DIV0), region="back-panel", material="hardboard"),
        sh("drawer-block", d, "body", 4, True, 0.66, wrect(-DF["z"], DF["y0"], DF["z"], DF["y1"]), region="drawer", material="teak-veneer-chipboard"),
        sh("top-band", c, "body", 5, True, 0.84, wrect(-Z_S, Y_SIDE1, Z_S, Y_TOP1), region="top", material="teak-top-faded"),
    ]
    pl = [
        sh("blanket", "blanket", "body", 1, True, 0.45, wrect(-BX, -BZ, BX, BZ), region="blanket", material="moving-blanket"),
        sh("drawer-block", d, "body", 2, True, 0.66, wrect(DB["x0"], -DF["z"], DF["x1"], DF["z"]), region="drawer", material="teak-veneer-chipboard"),
        sh("top-band", c, "body", 4, True, 0.84, wrect(X_BACK, -Z_S, X_CF, Z_S), region="top", material="teak-top-faded"),
    ]
    return {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl},
            "decisions": {
                "silhouette": T("Table 1.8 x 2.8 mm on a 3.3 mm blanket: carcass outline with the top as its band, two legs, the open shelf as a dark field under the drawer block.", "Tisch 1,8 x 2,8 mm auf einer 3,3 mm breiten Decke: Korpusumriss mit der Platte als Band, zwei Beine, das offene Fach als dunkles Feld unter dem Schubladenblock."),
                "retained": T("Top line at 0.57 m, the open shelf, the leg zone and the blanket under it.", "Plattenlinie bei 0,57 m, das offene Fach, die Beinzone und die Decke darunter."),
                "merged": T("Drawer front and box as one block; the top flush with the carcass front (its 20 mm overhang is under the 30 mm feature); the back panel drawn with the side.", "Schubladenfront und -kasten als ein Block; die Platte bündig mit der Korpusfront (ihr Überstand von 20 mm liegt unter dem Merkmal von 30 mm); die Rückwand mit der Seite gezeichnet."),
                "omitted": T("Knob, reveal, collars, grain and every trace of use (all under 0.15 mm at 1:200).", "Knauf, Fuge, Bunde, Maserung und alle Gebrauchsspuren (alle unter 0,15 mm bei 1:200)."),
                "thinElements": T("The 19 mm top band and the 18 mm blanket are carried by the 0.25 mm pen.", "Das 19-mm-Plattenband und die 18-mm-Decke trägt der 0,25-mm-Stift."),
                "hatching": T("None.", "Keine."),
                "styles": T("Top 0.84, side 0.72, drawer 0.66, carcass front 0.62, legs 0.55, blanket 0.45, shelf 0.3.", "Platte 0,84, Seite 0,72, Schublade 0,66, Korpusfront 0,62, Beine 0,55, Decke 0,45, Fach 0,3."),
                "overlays": T("The side names carcass and back panel; the drawer block is the drawer region.", "Die Seite nennt Korpus und Rückwand; der Schubladenblock ist die Region Schublade."),
            }}


def rep500():
    """🚫️ 1:500 is not drawn: the whole table is about one millimetre of paper."""
    return {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
            "reason": T("The bedside table is 0.35 x 0.43 x 0.57 m = 0.7 x 0.9 x 1.1 mm on paper at 1:500, under 3 mm: the drawer (0.11 m = 0.21 mm), the open shelf, the legs (22-39 mm = 0.04-0.08 mm) and the 18 mm blanket would merge into one anonymous speck.", "Der Nachttisch misst 0,35 x 0,43 x 0,57 m = 0,7 x 0,9 x 1,1 mm auf dem Papier bei 1:500, unter 3 mm: Schublade (0,11 m = 0,21 mm), offenes Fach, Beine (22-39 mm = 0,04-0,08 mm) und die 18-mm-Decke würden zu einem namenlosen Fleck verschmelzen."),
            "limitingFeature": T("Turned legs, 22-39 mm (0.04-0.08 mm on paper)", "Gedrechselte Beine, 22-39 mm (0,04-0,08 mm auf dem Papier)"),
            "limitingSizeM": 0.022, "alternatives": [{"kind": "scale", "denominator": 200}]}


def vol(den, body, thermal, structure, decisions):
    """🧊️ One authored scale volume."""
    return {"status": "authored", "denominator": den, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


VOLUMES = {
    "50": vol(50, B50, thermal50(), structure50(), [
        T("Body for the 7.5 mm feature: the 19 mm faded top with its 20 mm front overhang, two 16 mm veneered chipboard sides, bottom and divider (16 mm), the 3 mm hardboard back (sheet), four turned beech legs (22 mm at the foot, 35 mm under the collar, 39 mm collar) as two repeated revolves, the inset drawer (16 mm front, 10 mm pine sides and back, 4 mm hardboard bottom as a sheet) with its turned teak knob, and the folded 150 x 200 cm moving blanket (six layers, 18 mm) with its round fold at the back.", "Körper für das 7,5-mm-Merkmal: die 19-mm-gebleichte Platte mit 20 mm Überstand vorn, zwei 16-mm-furnierte Spanplattenseiten, Boden und Zwischenboden (16 mm), die 3-mm-Hartfaserrückwand (Platte), vier gedrechselte Buchenbeine (22 mm am Fuß, 35 mm unter dem Bund, 39 mm Bund) als zwei wiederholte Drehkörper, die eingelassene Schublade (16-mm-Front, 10-mm-Kiefernseiten und -rückwand, 4-mm-Hartfaserboden als Platte) mit gedrechseltem Teakknauf und die gefaltete Möbeldecke 150 x 200 cm (sechs Lagen, 18 mm) mit runder Falte hinten."),
        T("Traces of use as geometry where they reach the feature and as texture where they do not: the cigarette burn is a 28 x 9 mm texture solid 0.6 mm into the top; rings, lamp circle, chip and scuffs are texture strokes of the drawing.", "Gebrauchsspuren als Geometrie, wo sie das Merkmal erreichen, und als Textur, wo nicht: die Zigarettenbrandstelle ist ein 28 x 9 mm großer Texturkörper 0,6 mm tief in der Platte; Glasränder, Lampenkreis, Abplatzung und Stoßspuren sind Texturstriche der Zeichnung."),
        T("Thermal: every body solid in its region; the drawer box belongs to the drawer region and lies in the closed drawer bay (the 1.5 mm reveals are below the quarter feature and close it), so only the front and knob count as its outer area. Structure: boards, legs, drawer parts and blanket as members along their bones, leg plates, top dowels and the drawer runners as joints, three mass markers; no permanent support: the pose contacts (blanket pads, feet, grip) carry it.", "Thermik: jeder Körper in seiner Region; der Schubladenkasten gehört zur Region Schublade und liegt im geschlossenen Schubladenfach (die 1,5-mm-Fugen liegen unter dem Viertelmerkmal und schließen es), daher zählen nur Front und Knauf als ihre Außenfläche. Tragwerk: Platten, Beine, Schubladenteile und Decke als Stäbe entlang ihrer Knochen, Beinplatten, Plattendübel und Schubladenlaufleisten als Gelenke, drei Massenmarken; kein dauerndes Auflager: die Kontakte der Posen (Deckenpunkte, Füße, Griff) tragen."),
    ]),
    "100": vol(100, body100(), thermal100(), structure100(), [
        T("Body for the 15 mm feature: the top with its overhang, sides, bottom and divider as one front profile swept to the front edge with the open shelf as a hole and the drawer bay open, the back sheet across the full width, plain tapered legs (22 to 37 mm), the drawer front, the knob, the drawer box of sides and back thickened from 10 to the 15 mm feature with its 4 mm bottom sheet, and the blanket as a plain 18 mm sheet.", "Körper für das 15-mm-Merkmal: die Platte mit Überstand, Seiten, Boden und Zwischenboden als ein bis zur Vorderkante gezogenes Frontprofil mit dem offenen Fach als Loch und offenem Schubladenfach, die Rückwand als Platte über die ganze Breite, glatte konische Beine (22 bis 37 mm), Schubladenfront, Knauf, der Schubladenkasten aus von 10 auf das Merkmal von 15 mm verdickten Seiten und Rückwand mit 4-mm-Bodenplatte und die Decke als glatte 18-mm-Platte."),
        T("Merged against 1:50: leg collars and the burn texture; the knob stays in 3D (26 x 28 mm) although the side view leaves it to the front view and plan (white-gap rule between 1:75 and 1:150).", "Gegenüber 1:50 zusammengefasst: Beinbunde und die Brandtextur; der Knauf bleibt im 3D (26 x 28 mm), obwohl die Seitenansicht ihn Vorderansicht und Grundriss überlässt (Weißspaltregel zwischen 1:75 und 1:150)."),
        T("Thermal and structure reference the body: carcass profile, top, back, legs, drawer front and knob, drawer box in the drawer region, blanket; leg plates and runners as 16 mm joints, masses 18 mm.", "Thermik und Tragwerk verweisen auf den Körper: Korpusprofil, Platte, Rückwand, Beine, Schubladenfront und Knauf, Schubladenkasten in der Region Schublade, Decke; Beinplatten und Laufleisten als 16-mm-Gelenke, Massen 18 mm."),
    ]),
    "200": vol(200, body200(), thermal200(), structure200(), [
        T("Body for the 30 mm feature: top flush with the carcass front (the 20 mm overhang is under the feature), the carcass profile with the open shelf, the back sheet, legs from 30 to 36 mm, the drawer front and box as one block without the knob, and the blanket as an 18 mm sheet.", "Körper für das 30-mm-Merkmal: Platte bündig mit der Korpusfront (der Überstand von 20 mm liegt unter dem Merkmal), das Korpusprofil mit offenem Fach, die Rückwand als Platte, Beine von 30 bis 36 mm, Schubladenfront und -kasten als ein Block ohne Knauf und die Decke als 18-mm-Platte."),
        T("Thermal: the drawer block is the drawer region; structure: top, carcass, back, legs, drawer block and blanket as members, leg plates 30 mm, masses 32 mm.", "Thermik: der Schubladenblock ist die Region Schublade; Tragwerk: Platte, Korpus, Rückwand, Beine, Schubladenblock und Decke als Stäbe, Beinplatten 30 mm, Massen 32 mm."),
    ]),
    "500": {"status": "unavailable", "denominator": 500},
}


def mat(id, en, de, eps, alpha, rho, k, c, E=None, f=None, source="literature", reference="iso-10456-2007"):
    """🧱️ One material."""
    m = {"id": id, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha}
    if rho is not None:
        m["densityKgM3"] = rho
    m.update({"conductivityWmK": k, "specificHeatJKgK": c})
    if E is not None:
        m["elasticModulusPa"] = E
    if f is not None:
        m["strengthPa"] = f
    m.update({"source": source, "reference": reference})
    return m


BLANKET_RHO = round(BLANKET_KG / extrude_volume(B50[-1]), 1)
MATERIALS = [
    mat("teak-veneer-chipboard", "Teak-veneered chipboard P2, aged nitro lacquer", "Teakfurnierte Spanplatte P2, gealterter Nitrolack", 0.9, 0.7, 650, 0.13, 1700, 1.6e9, 11e6, "literature", "en-312-2010"),
    mat("teak-top-faded", "Teak-veneered chipboard top, sun-bleached and worn", "Teakfurnierte Spanplattenplatte, sonnengebleicht und abgenutzt", 0.88, 0.58, 650, 0.13, 1700, 1.6e9, 11e6, "authored-estimate", "incropera-2018"),
    mat("beech-stained", "Turned beech, stained teak colour and lacquered", "Gedrechselte Buche, teakfarben gebeizt und lackiert", 0.9, 0.72, 710, 0.16, 1600, 14e9, 35e6, "literature", "wood-handbook-2021"),
    mat("pine-drawer", "Pine drawer boards, unfinished", "Kiefernholz der Schublade, unbehandelt", 0.9, 0.45, 520, 0.13, 1600, 11e9, 24e6, "literature", "en-338-2016"),
    mat("hardboard", "Hardboard HB 3 mm, raw back", "Hartfaserplatte HB 3 mm, rohe Rückseite", 0.92, 0.65, 950, 0.18, 1700, 4.0e9, 30e6, "literature", "en-622-2-2004"),
    mat("teak-solid", "Solid teak knob, oiled and handled", "Knauf aus massivem Teak, geölt und abgegriffen", 0.9, 0.7, 660, 0.17, 1600, 10e9, 70e6, "literature", "wood-handbook-2021"),
    mat("moving-blanket", "Quilted cotton-polyester moving blanket, faded blue", "Gesteppte Möbeldecke aus Baumwolle und Polyester, verblasstes Blau", 0.88, 0.75, BLANKET_RHO, 0.045, 1300, None, None, "authored-estimate", "moving-blanket-type"),
]


def layer(id, en, de, kind, material, t, k, rho, c):
    """🧶️ One single layer of a region with its resistance and areal heat capacity rho c min(t, sqrt(alpha 3600 s))."""
    depth = min(t, math.sqrt(k / (rho * c) * 3600))
    return {"id": id, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / k, 6), "heatCapacityJM2K": round(rho * c * depth), "source": "literature", "reference": "iso-10456-2007"}


AREA = {"top": 0.234, "carcass": 0.76, "back-panel": 0.24, "drawer": 0.461, "legs": 0.080, "blanket": 1.05}


def region(id, en, de, material, geometry, L, sky, lay, extra=None):
    """🔥️ One passive thermal region."""
    r = {"id": id, "title": T(en, de), "kind": "surface", "model": "passive", "material": material, "areaM2": AREA[id], "characteristicLengthM": L, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        r.update(extra)
    return r


REGIONS = [
    region("top", "Sun-faded veneered top", "Sonnengebleichte furnierte Platte", "teak-top-faded", "horizontal-plate-up", 0.35, 0.8, layer("top-board", "Chipboard 19 mm", "Spanplatte 19 mm", "timber", "teak-top-faded", 0.019, 0.13, 650, 1700), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
    region("carcass", "Veneered carcass and open shelf", "Furnierter Korpus und offenes Fach", "teak-veneer-chipboard", "vertical-plate", 0.33, 0.4, layer("carcass-board", "Chipboard 16 mm", "Spanplatte 16 mm", "timber", "teak-veneer-chipboard", 0.016, 0.13, 650, 1700)),
    region("back-panel", "Hardboard back", "Hartfaserrückwand", "hardboard", "vertical-plate", 0.33, 0.3, layer("back-sheet", "Hardboard 3 mm", "Hartfaser 3 mm", "timber", "hardboard", 0.003, 0.18, 950, 1700)),
    region("drawer", "Drawer front, knob and box", "Schubladenfront, Knauf und Kasten", "teak-veneer-chipboard", "vertical-plate", 0.107, 0.4, layer("drawer-board", "Chipboard 16 mm", "Spanplatte 16 mm", "timber", "teak-veneer-chipboard", 0.016, 0.13, 650, 1700)),
    region("legs", "Turned beech legs", "Gedrechselte Buchenbeine", "beech-stained", "cylinder", 0.03, 0.3, layer("leg-wood", "Beech, half diameter 15 mm", "Buche, halber Durchmesser 15 mm", "timber", "beech-stained", 0.015, 0.16, 710, 1600)),
    region("blanket", "Folded moving blanket", "Gefaltete Möbeldecke", "moving-blanket", "horizontal-plate-up", 0.667, 0.6, layer("blanket-felt", "Six felt layers 18 mm", "Sechs Filzlagen 18 mm", "clothing", "moving-blanket", 0.018, 0.045, BLANKET_RHO, 1300)),
]

C = "carcass"
ANCHORS = []
for name, x in (("rear", LEG_X[0]), ("front", LEG_X[1])):
    for side, z in (("l", LEG_Z), ("r", -LEG_Z)):
        u, v = loc(C, x, Y_LEG0)
        ANCHORS.append({"id": f"foot-{name}-{side}", "bone": C, "x": u, "y": v, "z": z, "kind": "contact", "title": T(f"{'Rear' if name == 'rear' else 'Front'} {'left' if side == 'l' else 'right'} foot", f"{'Hinterer' if name == 'rear' else 'Vorderer'} {'linker' if side == 'l' else 'rechter'} Fuß"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.022, "footprintWidthM": 0.022, "frictionCoefficient": 0.4})
for name, x in (("rear", LEG_X[0]), ("front", LEG_X[1])):
    for side, z in (("l", LEG_Z), ("r", -LEG_Z)):
        u, v = loc("blanket", x, 0.0)
        ANCHORS.append({"id": f"pad-{name}-{side}", "bone": "blanket", "x": u, "y": v, "z": z, "kind": "contact", "title": T(f"Blanket under the {name} {'left' if side == 'l' else 'right'} foot", f"Decke unter dem {'hinteren' if name == 'rear' else 'vorderen'} {'linken' if side == 'l' else 'rechten'} Fuß"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.05, "footprintWidthM": 0.05, "frictionCoefficient": 0.6})
GRIP = loc(C, X_TF, Y_TOP1 - 0.0095)
ANCHORS += [
    {"id": "grip", "bone": C, "x": GRIP[0], "y": GRIP[1], "z": 0.0, "kind": "support", "title": T("Carrying hand at the front edge of the top", "Tragende Hand an der Vorderkante der Platte"), "support": "fixed"},
    {"id": "lamp", "bone": C, "x": r6(-0.075 - X_BACK), "y": 0.0, "z": 0.105, "kind": "load", "title": T("Bedside lamp on the top", "Nachttischlampe auf der Platte")},
    {"id": "books", "bone": C, "x": r6(0.04 - X_BACK), "y": 0.0, "z": -0.09, "kind": "load", "title": T("Books on the top", "Bücher auf der Platte")},
    {"id": "glass", "bone": C, "x": r6(0.11 - X_BACK), "y": 0.0, "z": 0.1, "kind": "load", "title": T("Glass of water", "Wasserglas")},
    {"id": "drawer-contents", "bone": "drawer", "x": 0.15, "y": 0.008, "z": 0.0, "kind": "load", "title": T("Contents of the drawer", "Inhalt der Schublade")},
    {"id": "knob-pull", "bone": "drawer", "x": r6(X_CF + 0.026 - DB["x0"]), "y": r6(KNOB_Y - Y_DIV1), "z": 0.0, "kind": "load", "title": T("Hand pulling the knob", "Hand am Knauf")},
]

BONES = [
    {"id": "ground", "parent": None, "title": T("Depot floor reference", "Bodenbezug im Lager"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "blanket", "parent": "ground", "title": T("Folded moving blanket (stays behind)", "Gefaltete Möbeldecke (bleibt zurück)"), "x": -BX, "y": 0, "angleDeg": 0, "lengthM": 0.6666, "joint": {"type": "fixed"}, "mass": {"kg": round(MASS["blanket"]["kg"], 5), "comX": round(MASS["blanket"]["comX"], 5), "comY": round(MASS["blanket"]["comY"], 5), "distribution": "point", "source": "authored-estimate", "reference": "moving-blanket-type"}},
    {"id": "travel", "parent": "ground", "title": T("Carrying travel along the floor", "Tragweg über den Boden"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.0}},
    {"id": "lift", "parent": "travel", "title": T("Lift of the carrying hand", "Hub der tragenden Hand"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.03, "maxM": 0.3}},
    {"id": "carcass", "parent": "lift", "title": T("Carcass with top and legs (pivot at the rear top edge)", "Korpus mit Platte und Beinen (Drehpunkt an der hinteren Plattenkante)"), "x": X_BACK, "y": Y_TOP1, "angleDeg": 0, "lengthM": 0.35, "joint": {"type": "revolute", "minDeg": -15, "maxDeg": 30}, "mass": {"kg": round(MASS["carcass"]["kg"], 5), "comX": round(MASS["carcass"]["comX"], 5), "comY": round(MASS["carcass"]["comY"], 5), "distribution": "point", "source": "authored-estimate", "reference": "en-312-2010"}},
    {"id": "drawer", "parent": "carcass", "title": T("Drawer on wooden runners", "Schublade auf Holzlaufleisten"), "x": r6(DB["x0"] - X_BACK), "y": r6(Y_DIV1 - Y_TOP1), "angleDeg": 0, "lengthM": r6(X_CF - DB["x0"]), "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.26}, "mass": {"kg": round(MASS["drawer"]["kg"], 5), "comX": round(MASS["drawer"]["comX"], 5), "comY": round(MASS["drawer"]["comY"], 5), "distribution": "point", "source": "authored-estimate", "reference": "en-312-2010"}},
]


def carried_offsets(grip_y=0.76, tilt=12.0, dx=0.30):
    """🤲️ The lift that brings the grip to the hand height after the carcass tilts about its rear top edge, and the clearance of the lowest foot."""
    a = math.radians(tilt)
    gx, gy = GRIP
    gy_rot = gx * math.sin(a) + gy * math.cos(a)
    dy = grip_y - Y_TOP1 - gy_rot
    feet = []
    for x in LEG_X:
        fu, fv = loc(C, x, Y_LEG0)
        feet.append(Y_TOP1 + dy + fu * math.sin(a) + fv * math.cos(a))
    return round(dy, 4), min(feet)


LIFT, FOOT_Y = carried_offsets()


def regions_binding(top_sun, drawer=0.2, blanket_contact=0.45):
    """🌬️ The thermal binding of every region of a pose."""
    return {"regions": {"top": {"exposedFraction": 1, "sunlitFraction": top_sun}, "carcass": {"exposedFraction": 1, "sunlitFraction": 0}, "back-panel": {"exposedFraction": 1, "sunlitFraction": 0}, "drawer": {"exposedFraction": drawer, "sunlitFraction": 0}, "legs": {"exposedFraction": 1, "sunlitFraction": 0}, "blanket": {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": blanket_contact, "tempC": 18}}}, "emitters": {}}


FLOOR_UNDER_BLANKET = {"id": "floor-under-blanket", "kind": "imposed", "title": T("Floor reaction under the blanket, which lies on its own", "Bodenreaktion unter der Decke, die für sich liegt"), "bone": "blanket", "x": round(MASS["blanket"]["comX"], 5), "y": round(MASS["blanket"]["comY"], 5), "fxN": 0, "fyN": round(WB, 3), "source": "literature", "reference": "en-1991-1-1"}
IN_USE_LOADS = [
    {"id": "lamp-load", "kind": "equipment", "title": T("Bedside lamp, 1.8 kg", "Nachttischlampe, 1,8 kg"), "anchor": "lamp", "fxN": 0, "fyN": round(-1.8 * G, 3), "source": "authored-estimate", "reference": "en-16122-2012"},
    {"id": "books-load", "kind": "equipment", "title": T("Books, 1.5 kg", "Bücher, 1,5 kg"), "anchor": "books", "fxN": 0, "fyN": round(-1.5 * G, 3), "source": "authored-estimate", "reference": "en-16122-2012"},
    {"id": "glass-load", "kind": "equipment", "title": T("Glass of water, 0.35 kg", "Wasserglas, 0,35 kg"), "anchor": "glass", "fxN": 0, "fyN": round(-0.35 * G, 3), "source": "authored-estimate", "reference": "en-16122-2012"},
    {"id": "drawer-load", "kind": "equipment", "title": T("Drawer contents, 0.8 kg", "Schubladeninhalt, 0,8 kg"), "anchor": "drawer-contents", "fxN": 0, "fyN": round(-0.8 * G, 3), "source": "authored-estimate", "reference": "en-16122-2012"},
]
FEET = ["foot-rear-l", "foot-rear-r", "foot-front-l", "foot-front-r"]
PADS = ["pad-rear-l", "pad-rear-r", "pad-front-l", "pad-front-r"]

POSES = [
    {"id": "stored", "title": T("Stored on the blanket", "Auf der Decke gelagert"), "description": T("Emptied and cleaned, standing on a folded moving blanket in the reuse depot, drawer pushed shut.", "Geleert und gereinigt, auf einer gefalteten Möbeldecke im Wiederverwendungslager, Schublade zugeschoben."), "bones": {}, "ground": "authored", "contacts": PADS, "thermal": regions_binding(0.0), "loads": [FLOOR_UNDER_BLANKET]},
    {"id": "carried", "title": T("Carried in one hand", "Mit einer Hand getragen"), "description": T(f"Held by the front edge of the top at {0.76:.2f} m hand height, tilted 12 degrees back so the drawer stays shut, lowest foot {FOOT_Y:.2f} m above the floor; the blanket stays behind.", f"An der Vorderkante der Platte in {0.76:.2f} m Handhöhe gehalten, 12 Grad nach hinten geneigt, damit die Schublade zu bleibt, tiefster Fuß {FOOT_Y:.2f} m über dem Boden; die Decke bleibt zurück.").copy(), "bones": {"travel": {"dx": 0.3}, "lift": {"dy": LIFT}, "carcass": {"angleDeg": 12}}, "ground": "authored", "contacts": ["grip"], "thermal": regions_binding(0.0), "loads": [FLOOR_UNDER_BLANKET]},
    {"id": "in-use", "title": T("Beside the bed again", "Wieder neben dem Bett"), "description": T("Set down on the bedroom floor beside a bed with a lamp, books and a glass on the top and a few things in the drawer; a window sun patch lies on the top.", "Auf den Schlafzimmerboden neben ein Bett gestellt, mit Lampe, Büchern und Glas auf der Platte und ein paar Dingen in der Schublade; ein Fensterlichtfleck liegt auf der Platte."), "bones": {"travel": {"dx": 0.8}, "lift": {"dy": -BT}}, "ground": "authored", "contacts": FEET, "thermal": regions_binding(0.3), "loads": [FLOOR_UNDER_BLANKET] + IN_USE_LOADS},
    {"id": "drawer-open", "title": T("Drawer pulled out", "Schublade gezogen"), "description": T("In use with the drawer pulled 0.20 m out on its wooden runners by the knob.", "In Gebrauch, die Schublade am Knauf 0,20 m auf ihren Holzlaufleisten herausgezogen."), "bones": {"travel": {"dx": 0.8}, "lift": {"dy": -BT}, "drawer": {"dx": 0.2}}, "ground": "authored", "contacts": FEET, "thermal": regions_binding(0.3, 0.55), "loads": [FLOOR_UNDER_BLANKET] + IN_USE_LOADS + [{"id": "pull", "kind": "live", "title": T("Pull at the knob, 12 N", "Zug am Knauf, 12 N"), "anchor": "knob-pull", "fxN": 12, "fyN": 0, "source": "authored-estimate", "reference": "en-16122-2012"}]},
    {"id": "lift-off-key", "key": True, "title": T("Lifting off the blanket", "Abheben von der Decke"), "description": T("The hand has taken the top and lifts the table clear of the blanket.", "Die Hand hat die Platte gefasst und hebt den Tisch von der Decke ab."), "bones": {"lift": {"dy": 0.03}, "carcass": {"angleDeg": 2}}, "ground": "authored", "contacts": ["grip"], "thermal": regions_binding(0.0), "loads": [FLOOR_UNDER_BLANKET]},
    {"id": "set-down-key", "key": True, "title": T("Lowering at the bedside", "Absenken am Bett"), "description": T("Carried to the bedside, levelled and lowered to just above the floor.", "Ans Bett getragen, waagerecht gestellt und bis knapp über den Boden abgesenkt."), "bones": {"travel": {"dx": 0.8}, "lift": {"dy": -0.005}}, "ground": "authored", "contacts": ["grip"], "thermal": regions_binding(0.0), "loads": [FLOOR_UNDER_BLANKET]},
]

CLIPS = [
    {"id": "carry", "title": T("Carry it off the blanket", "Von der Decke wegtragen"), "description": T("One hand takes the front edge of the top, lifts the table off the blanket, lets it tilt back and carries it away; the blanket stays on the depot floor.", "Eine Hand fasst die Vorderkante der Platte, hebt den Tisch von der Decke, lässt ihn nach hinten kippen und trägt ihn fort; die Decke bleibt auf dem Lagerboden."), "durationS": 2.4, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.5, "pose": "lift-off-key", "ease": "smooth"}, {"t": 1.8, "pose": "carried", "ease": "smooth"}, {"t": 2.4, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 0.2} for a in PADS] + [{"anchor": "grip", "fromS": 0, "toS": 2.4}],
     "phases": [{"id": "grip", "fromS": 0, "toS": 0.5, "title": T("Take hold and lift", "Fassen und anheben")}, {"id": "swing", "fromS": 0.5, "toS": 1.8, "title": T("Tilt back and walk", "Zurückkippen und gehen")}, {"id": "carry", "fromS": 1.8, "toS": 2.4, "title": T("Carry", "Tragen")}],
     "events": [{"t": 0.2, "kind": "contact-off", "title": T("Feet leave the blanket", "Füße verlassen die Decke")}, {"t": 1.8, "kind": "motion", "title": T("Hanging from the hand", "Hängt an der Hand")}], "tracks": []},
    {"id": "drawer-pull", "title": T("Pull the drawer and push it back", "Schublade ziehen und zurückschieben"), "description": T("In use: the drawer is pulled 0.20 m out by its knob, held and pushed back.", "In Gebrauch: die Schublade wird am Knauf 0,20 m herausgezogen, gehalten und zurückgeschoben."), "durationS": 3.0, "fps": 30, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}, {"t": 1.0, "pose": "drawer-open", "ease": "smooth"}, {"t": 2.0, "pose": "drawer-open", "ease": "smooth"}, {"t": 3.0, "pose": "in-use", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 3.0} for a in FEET],
     "phases": [{"id": "pull", "fromS": 0, "toS": 1.0, "title": T("Pull", "Ziehen")}, {"id": "hold", "fromS": 1.0, "toS": 2.0, "title": T("Open", "Offen")}, {"id": "push", "fromS": 2.0, "toS": 3.0, "title": T("Push back", "Zurückschieben")}],
     "events": [{"t": 1.0, "kind": "motion", "title": T("Drawer fully out", "Schublade ganz offen")}, {"t": 3.0, "kind": "motion", "title": T("Drawer shut", "Schublade zu")}],
     "tracks": [{"target": "region:drawer:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.2}, {"t": 1.0, "v": 0.55}, {"t": 2.0, "v": 0.55}, {"t": 3.0, "v": 0.2}]}]},
    {"id": "set-down", "title": T("Set it down beside the bed", "Neben dem Bett abstellen"), "description": T("Carried on to the bedside, levelled, lowered onto its feet and let go.", "Weiter ans Bett getragen, waagerecht gestellt, auf die Füße abgesenkt und losgelassen."), "durationS": 3.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "carried", "ease": "smooth"}, {"t": 2.2, "pose": "set-down-key", "ease": "smooth"}, {"t": 3.0, "pose": "in-use", "ease": "smooth"}],
     "contacts": [{"anchor": "grip", "fromS": 0, "toS": 2.9}] + [{"anchor": a, "fromS": 2.6, "toS": 3.0} for a in FEET],
     "phases": [{"id": "walk", "fromS": 0, "toS": 2.2, "title": T("Walk and level", "Gehen und ausrichten")}, {"id": "lower", "fromS": 2.2, "toS": 3.0, "title": T("Lower and let go", "Absenken und loslassen")}],
     "events": [{"t": 2.6, "kind": "contact-on", "title": T("Feet touch the floor", "Füße berühren den Boden")}, {"t": 2.9, "kind": "contact-off", "title": T("Hand lets go", "Hand lässt los")}], "tracks": []},
]

SYMBOL = {"id": "used-bedside-table-symbol", "title": T("Bedside table on its blanket, true extent", "Nachttisch auf seiner Decke, wahre Ausdehnung"), "description": T("The stored bedside table in all three views: top 0.35 x 0.43 m at 0.57 m, carcass, open shelf, drawer, two legs of each pair, and the folded 0.67 x 0.75 m blanket under it.", "Der gelagerte Nachttisch in allen drei Ansichten: Platte 0,35 x 0,43 m auf 0,57 m, Korpus, offenes Fach, Schublade, je zwei Beine und die gefaltete Decke 0,67 x 0,75 m darunter."),
          "semantics": "true-extent", "units": "model-m", "denominators": [200, 100, 50], "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [],
          "geometry": {
              "elevation": [sh("blanket", "ground", "body", 1, True, 0.45, wrect(-BX, 0, BX, BT)), sh("table", "ground", "body", 2, True, 0.8, wpoly([(X_BACK, Y_TOP1), (X_TF, Y_TOP1), (X_TF, Y_SIDE1), (X_CF, Y_SIDE1), (X_CF, Y_BOT0), (LEG_X[1] + 0.0185, Y_BOT0), (LEG_X[1] + 0.011, BT), (LEG_X[1] - 0.011, BT), (LEG_X[1] - 0.0185, Y_BOT0), (LEG_X[0] + 0.0185, Y_BOT0), (LEG_X[0] + 0.011, BT), (LEG_X[0] - 0.011, BT), (LEG_X[0] - 0.0185, Y_BOT0), (X_BACK, Y_BOT0)])), sh("drawer-line", "ground", "detail", 3, False, 0.3, wpoly([(X_CF, Y_DIV1), (X_TF - 0.005, Y_DIV1)], closed=False))],
              "front": [sh("blanket", "ground", "body", 1, True, 0.45, wrect(-BZ, 0, BZ, BT)), sh("table", "ground", "body", 2, True, 0.8, wpoly([(-Z_S, Y_TOP1), (Z_S, Y_TOP1), (Z_S, Y_BOT0), (LEG_Z + 0.0185, Y_BOT0), (LEG_Z + 0.011, BT), (LEG_Z - 0.011, BT), (LEG_Z - 0.0185, Y_BOT0), (-LEG_Z + 0.0185, Y_BOT0), (-LEG_Z + 0.011, BT), (-LEG_Z - 0.011, BT), (-LEG_Z - 0.0185, Y_BOT0), (-Z_S, Y_BOT0)])), sh("shelf", "ground", "detail", 3, True, 0.3, wrect(-Z_SI, Y_BOT1, Z_SI, Y_DIV0)), sh("drawer", "ground", "detail", 3.5, False, 0.3, wrect(-DF["z"], DF["y0"], DF["z"], DF["y1"]))],
              "plan": [sh("blanket", "ground", "body", 1, True, 0.45, wrect(-BX, -BZ, BX, BZ)), sh("top", "ground", "body", 2, True, 0.8, wrect(X_BACK, -Z_S, X_TF, Z_S))],
          },
          "dimensionalNote": T("Model metres at real size in the frame of the asset (x ahead to the drawer front, lateral positive to the left, y up): the side view is seen from the right, the front view from ahead, the top view from above; all three show the stored pose on the blanket.", "Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn zur Schubladenfront, seitlich positiv nach links, y nach oben): die Seitenansicht blickt von rechts, die Vorderansicht von vorn, die Draufsicht von oben; alle drei zeigen die gelagerte Pose auf der Decke.")}

REFERENCES = [
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values (particleboard 600-700 kg/m3 about 0.13-0.14 W/mK and 1700 J/kgK, fibreboard, hardwood and softwood timber)."},
    {"id": "en-312-2010", "citation": "EN 312:2010 Particleboards - Specifications (type P2, boards for interior fitments including furniture in dry conditions: bending strength 11 N/mm2 and modulus of elasticity 1600 N/mm2 for 13-20 mm boards)."},
    {"id": "en-622-2-2004", "citation": "EN 622-2:2004 Fibreboards - Specifications - Part 2: Requirements for hardboards (HB, density at least 900 kg/m3, bending strength above 30 N/mm2 for boards up to 3.5 mm)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (C24 softwood: 24 N/mm2 bending, 11 kN/mm2 modulus, 420 kg/m3 characteristic density; D35 hardwood: 35 N/mm2, 12 kN/mm2)."},
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service (air-dry density and elastic modulus of European beech about 710 kg/m3 and 14 GPa, teak about 660 kg/m3, Scots pine about 520 kg/m3)."},
    {"id": "en-16122-2012", "citation": "EN 16122:2012 Domestic and non-domestic storage furniture - Test methods for the determination of strength, durability and stability (loads on tops and shelves, extension elements pulled out, stability with extension elements open)."},
    {"id": "en-14749-2016", "citation": "EN 14749:2016 Domestic and kitchen storage units and worktops - Safety requirements and test methods (stability of storage furniture with extension elements open and loaded)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (self-weight as the product of volume and density)."},
    {"id": "din-18011-2020", "citation": "DIN 18011:2020-04 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (placing areas of beds and bedside tables, 0.4-0.5 m wide and 0.35-0.4 m deep, beside the bed head)."},
    {"id": "neufert-2019", "citation": "Neufert, E., Kister, J. (2019). Bauentwurfslehre, 42nd ed., Springer Vieweg (bedroom furniture: bedside tables about 0.40-0.50 m wide, 0.35-0.40 m deep, top at mattress height 0.50-0.60 m)."},
    {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte (grip axis height of the hanging arm of standing adults about 0.72-0.78 m, taken as 0.76 m for the carrying hand)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (total emissivity of lacquered wood 0.88-0.94 and of textiles 0.85-0.90; solar absorptance of dark and light wood surfaces)."},
    {"id": "furniture-1970s-type", "citation": "Type data of West German bedroom furniture programmes of 1970-1979 as mail-order and furniture-store catalogues offered them: bedside tables of teak- or walnut-veneered chipboard on four turned tapered legs, 40-48 cm wide, 30-38 cm deep, 50-58 cm high, one drawer over an open shelf, 7-11 kg; no single product, no brand."},
    {"id": "moving-blanket-type", "citation": "Type data of removal blankets (Möbeldecken, Umzugsdecken) as removal-equipment suppliers offer them: quilted needle felt of recycled cotton and polyester, 150 x 200 cm or 180 x 200 cm, 400-600 g/m2 (1.2-2.2 kg), about 3 mm thick per layer; no single product, no brand."},
]

ASSET = {
    "id": "used-bedside-table", "category": "furniture",
    "title": T("Used Bedside Table", "Gebrauchter Nachttisch"),
    "summary": T(f"A 1970s teak-veneered chipboard bedside table, 0.35 x 0.43 x 0.55 m and {MASS['carcass']['kg'] + MASS['drawer']['kg']:.1f} kg, with one drawer over an open shelf on four turned beech legs, sun-faded top with a cigarette burn, glass rings and a chipped corner; stored on a folded moving blanket, carried in one hand by the front edge of the top, back beside a bed with lamp and books, drawer pulled.", f"Ein Nachttisch der 1970er-Jahre aus teakfurnierter Spanplatte, 0,35 x 0,43 x 0,55 m und {MASS['carcass']['kg'] + MASS['drawer']['kg']:.1f} kg, mit einer Schublade über einem offenen Fach auf vier gedrechselten Buchenbeinen, sonnengebleichter Platte mit Zigarettenbrandstelle, Glasrändern und abgeplatzter Ecke; auf einer gefalteten Möbeldecke gelagert, mit einer Hand an der Vorderkante der Platte getragen, wieder neben einem Bett mit Lampe und Büchern, Schublade gezogen."),
    "tags": ["used", "household", "second-hand", "bedroom", "bedside-table", "furniture", "teak", "veneer", "chipboard", "beech", "drawer", "1970s", "interior"],
    "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                   "notes": T(f"West German bedside table of the 1970s type, emptied and cleaned at a flat clearance. Carcass of 16 mm teak-veneered chipboard P2 (sides 0.33 x 0.331 m, bottom and divider between them), 19 mm top flush with the sides and 20 mm proud at the front, 3 mm hardboard back, drawer bay 110 mm over an open shelf 189 mm clear; inset drawer with a 16 mm front, 10 mm pine sides and back and a 4 mm hardboard bottom on wooden runners, a turned teak knob; four turned beech legs 200 mm long, 22 mm at the foot and 35 mm under a 39 mm collar, on mounting plates. Traces of use: sun-faded top (lighter, lower solar absorptance) with the darker circle of a lamp base, a cigarette burn 28 x 9 mm near the front right, two glass rings, chipped veneer at the front-right corner, scuffed feet, a worn lower drawer edge; nothing that stops it working. Stored on a 150 x 200 cm moving blanket folded to 0.67 x 0.75 m (six layers, 18 mm), which stays behind when it is carried. Masses from the modelled 1:50 volumes x density: carcass {MASS['carcass']['kg']:.2f} kg, drawer {MASS['drawer']['kg']:.2f} kg, blanket {MASS['blanket']['kg']:.2f} kg.", f"Westdeutscher Nachttisch vom Typ der 1970er-Jahre, bei einer Haushaltsauflösung geleert und gereinigt. Korpus aus 16-mm-teakfurnierter Spanplatte P2 (Seiten 0,33 x 0,331 m, Boden und Zwischenboden dazwischen), 19-mm-Platte bündig mit den Seiten und vorn 20 mm überstehend, 3-mm-Hartfaserrückwand, Schubladenfach 110 mm über einem offenen Fach mit 189 mm lichter Höhe; eingelassene Schublade mit 16-mm-Front, 10-mm-Kiefernseiten und -rückwand und 4-mm-Hartfaserboden auf Holzlaufleisten, gedrechselter Teakknauf; vier gedrechselte Buchenbeine, 200 mm lang, 22 mm am Fuß und 35 mm unter einem 39-mm-Bund, auf Anschraubplatten. Gebrauchsspuren: sonnengebleichte Platte (heller, geringere Solarabsorption) mit dem dunkleren Kreis eines Lampenfußes, eine Zigarettenbrandstelle 28 x 9 mm vorn rechts, zwei Glasränder, abgeplatztes Furnier an der vorderen rechten Ecke, bestoßene Füße, eine abgegriffene untere Schubladenkante; nichts, was den Gebrauch hindert. Gelagert auf einer Möbeldecke 150 x 200 cm, auf 0,67 x 0,75 m gefaltet (sechs Lagen, 18 mm), die beim Tragen zurückbleibt. Massen aus den 1:50-Volumen x Dichte: Korpus {MASS['carcass']['kg']:.2f} kg, Schublade {MASS['drawer']['kg']:.2f} kg, Decke {MASS['blanket']['kg']:.2f} kg.")},
    "dimensions": {"heightM": round(Y_TOP1, 4), "widthM": round(2 * BX, 4), "depthM": round(2 * BZ, 4), "massKg": round(TOTAL_KG, 5)},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "ground", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {"50": rep50(), "100": rep100(), "200": rep200(), "500": rep500()},
    "symbols": [SYMBOL],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 300, "diffuseFraction": 0.2, "pressurePa": 101325}, "operating": [], "regions": REGIONS, "emitters": [], "passengers": [],
                "assumptions": [
                    T("Veneered chipboard after ISO 10456 (650 kg/m3, 0.13 W/mK, 1700 J/kgK); areal heat capacity rho c min(t, sqrt(alpha 3600 s)): the 16 and 19 mm boards take part fully, the 39 mm legs to their half diameter.", "Furnierte Spanplatte nach ISO 10456 (650 kg/m3, 0,13 W/mK, 1700 J/kgK); flächenbezogene Wärmekapazität rho c min(t, sqrt(alpha 3600 s)): die 16- und 19-mm-Platten nehmen voll teil, die 39-mm-Beine bis zum halben Durchmesser."),
                    T("Aged surfaces: the sun-bleached top absorbs less (0.58) than the darkened teak lacquer of the carcass (0.70), and its worn lacquer emits a little less (0.88 against 0.90); authored estimates from the range of lacquered wood in Incropera.", "Gealterte Oberflächen: die sonnengebleichte Platte absorbiert weniger (0,58) als der nachgedunkelte Teaklack des Korpus (0,70), und ihr abgenutzter Lack strahlt etwas weniger ab (0,88 statt 0,90); Schätzwerte aus dem Bereich lackierten Holzes nach Incropera."),
                    T("The drawer region is the drawer front with its knob and the pine box behind it; the box sits in the drawer bay closed by 1.5 mm reveals, so only the front and knob are outer area (areaM2); pulled 0.20 m out, its sides add about 0.05 m2 of exchange that this passive model does not count.", "Die Region Schublade ist die Schubladenfront mit Knauf und dem Kiefernkasten dahinter; der Kasten sitzt im durch 1,5-mm-Fugen geschlossenen Schubladenfach, daher sind nur Front und Knauf Außenfläche (areaM2); 0,20 m herausgezogen kommen etwa 0,05 m2 Seitenfläche hinzu, die dieses passive Modell nicht zählt."),
                    T("Reference case: a bedroom at 20 degC with still air and a 300 W/m2 window sun patch on the top in use; in the depot no sun; the blanket lies with 0.45 of its surface on an 18 degC floor.", "Referenzfall: ein Schlafzimmer bei 20 degC mit ruhender Luft und einem Fensterlichtfleck von 300 W/m2 auf der Platte in Gebrauch; im Lager keine Sonne; die Decke liegt mit 0,45 ihrer Oberfläche auf einem 18 degC warmen Boden."),
                    T("No electrical element: the table is passive furniture, 0 W stated explicitly on the top region; the lamp on it is a load, not part of the asset.", "Kein elektrisches Element: der Tisch ist passives Möbel, 0 W ausdrücklich an der Region Platte angegeben; die Lampe darauf ist eine Last, nicht Teil des Assets."),
                ]},
    "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": [],
                  "assumptions": [
                      T("One rigid body on the contacts of each pose: stored, the four feet stand on the blanket and the blanket pads under them carry the table to the floor; carried, the hand at the front edge of the top is a fixed support (it clamps the 19 mm top between thumb and fingers and takes the tilting moment); in use, the four feet stand on the floor.", "Ein starrer Körper auf den Kontakten jeder Pose: gelagert stehen die vier Füße auf der Decke, und die Deckenpunkte darunter tragen den Tisch zum Boden; getragen ist die Hand an der Vorderkante der Platte ein festes Auflager (sie klemmt die 19-mm-Platte zwischen Daumen und Fingern und nimmt das Kippmoment auf); in Gebrauch stehen die vier Füße auf dem Boden."),
                      T(f"The blanket ({MASS['blanket']['kg']:.2f} kg) belongs to the asset but lies on its own: its floor reaction is entered as an upward load at its centre in every pose, so the hand carries the table alone ({MASS['carcass']['kg'] + MASS['drawer']['kg']:.2f} kg = {(MASS['carcass']['kg'] + MASS['drawer']['kg']) * G:.1f} N) and the blanket is never lifted with it.", f"Die Decke ({MASS['blanket']['kg']:.2f} kg) gehört zum Asset, liegt aber für sich: ihre Bodenreaktion ist in jeder Pose als aufwärts gerichtete Last in ihrer Mitte angesetzt, so trägt die Hand nur den Tisch ({MASS['carcass']['kg'] + MASS['drawer']['kg']:.2f} kg = {(MASS['carcass']['kg'] + MASS['drawer']['kg']) * G:.1f} N), und die Decke wird nie mit angehoben."),
                      T("Members: 16 and 19 mm chipboard P2 (EN 312: 11 N/mm2, 1600 N/mm2), turned beech legs (D35 after EN 338) on screwed mounting plates, pine drawer boards (C24), hardboard back and drawer bottom (EN 622-2). Masses are section x density of the 1:50 solids.", "Stäbe: 16- und 19-mm-Spanplatte P2 (EN 312: 11 N/mm2, 1600 N/mm2), gedrechselte Buchenbeine (D35 nach EN 338) auf geschraubten Anschraubplatten, Kiefernholz der Schublade (C24), Hartfaserrückwand und -boden (EN 622-2). Massen sind Querschnitt x Dichte der 1:50-Körper."),
                      T("In use: lamp 1.8 kg, books 1.5 kg, glass 0.35 kg and 0.8 kg in the drawer (authored estimates, far below the shelf and top test loads of EN 16122); with the drawer pulled 0.20 m and a 12 N pull at the knob the table stays standing (stability with extension elements open after EN 14749 and EN 16122).", "In Gebrauch: Lampe 1,8 kg, Bücher 1,5 kg, Glas 0,35 kg und 0,8 kg in der Schublade (Schätzwerte, weit unter den Prüflasten für Platten und Böden nach EN 16122); bei 0,20 m gezogener Schublade und 12 N Zug am Knauf bleibt der Tisch stehen (Standsicherheit mit geöffneten Auszügen nach EN 14749 und EN 16122)."),
                  ], "clinical": False},
    "volumes": VOLUMES,
    "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

ORDER = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]
DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: ASSET[k] for k in ORDER}}

if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(DOC, ensure_ascii=False, indent=1) + "\n")
    for b, m in MASS.items():
        print(f"[DEBUG] {b}: {m['kg']:.4f} kg com ({m['comX']:.4f}, {m['comY']:.4f}, {m['comZ']:.4f}) parts {m['parts']}")
    print(f"[DEBUG] total {TOTAL_KG:.4f} kg, blanket density {BLANKET_RHO}, carried lift {LIFT}, lowest foot {FOOT_Y:.4f} m")
