"""🗄️ Authoritative author of the staffage asset `used-wall-unit`: a 1980s oak-look living-room wall unit (Schrankwand) of three bodies, dismantled and stored on moving blankets.

Every hand-chosen number of the asset is stated here; run `python3 author-used-wall-unit.py` to rewrite `🔣️.json` next to this file.
The geometry kit (rig frames, solid and shape builders) is `author-geometry.py` in the same folder.

@see ../../brief.md — the author brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the staffage contract
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "author-geometry.py"))
kit = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(kit)
R, Rig, Kit, Draw, clean, ref = kit.R, kit.Rig, kit.Kit, kit.Draw, kit.clean, kit.ref
rect_pts, rounded_strip, circle = kit.rect_pts, kit.rounded_strip, kit.circle


def T(en, de):
    return {"en": en, "de": de}


BL = 0.012
D = 0.48
HW = 0.45
IN = 0.431
Y_PL, Y_B1, Y_T0, Y_T1, Y_TOP = 0.08, 0.099, 1.921, 1.94, 2.0
X_BK0, X_BK1 = 0.008, 0.011
X_PL0, X_PL1 = 0.43, 0.449
X_F0, X_F1, X_SLAB = 0.481, 0.5, 0.492
LEAF = 0.4485
ZC = {"linen": 0.9, "centre": 0.0, "tv": -0.9}
FLAP_PIVOT = (0.5, 0.44)
STAY_Q = (0.481, 0.64)
STAY_P = (0.32, 0.62)
STAY_Z = 0.424
STAY_L = 0.43
STAY_R = 0.003
PLAN_CUT = 0.55
FLAP_KNOB_Y = 0.832
HANDLE_50 = [(0.172, 0.492), (0.182, 0.492), (0.182, 0.51), (0.268, 0.51), (0.268, 0.492), (0.278, 0.492), (0.278, 0.52), (0.172, 0.52)]
KNOB = [(0, 0), (0, 0.008), (0.008, 0.007), (0.013, 0.012), (0.018, 0.015), (0.023, 0.013), (0.025, 0.007), (0.025, 0)]
KNOB_S = [(a * 0.75, r * 0.75) for a, r in KNOB]
KNOB_100 = [(0, 0), (0, 0.015), (0.025, 0.015), (0.025, 0)]
CORNICE_50 = [(0.46, 1.94), (0.5, 1.94), (0.5, 1.946), (0.51, 1.95), (0.518, 1.958), (0.523, 1.968), (0.526, 1.978), (0.53, 1.982), (0.53, 2.0), (0.46, 2.0)]
CORNICE_100 = [(0.46, 1.94), (0.5, 1.94), (0.515, 1.955), (0.53, 1.97), (0.53, 2.0), (0.46, 2.0)]
DENSITY = {"oak-decor-chipboard": 650, "oak-decor-mdf": 750, "hardboard": 900, "float-glass": 2500, "mirror-glass": 2500, "beech-knob": 700, "chrome-steel": 7800, "zinc-plated-steel": 7800, "moving-blanket": 345, "lamp-glass": 2500}


def w(body, x, y, z=0.0):
    return (x, y + BL, z + ZC[body])


def wp(body, pts):
    return [(x, y + BL) for x, y in pts]


rig = Rig()
rig.add("ground", (0, 0, 0), 0)
rig.add("blankets", (-0.03, 0, 0), 0)
for b in ("linen", "tv"):
    rig.add(f"{b}-body", (0, BL, ZC[b]), 0)
rig.add("centre-slide", (0, BL, 0), 0)
rig.add("centre-lift", (0, BL, 0), 0)
rig.add("centre-body", (0, BL, 0), 0)
rig.add("flap", (FLAP_PIVOT[0], FLAP_PIVOT[1] + BL, 0), 90)
PSI0 = math.degrees(math.atan2(STAY_P[1] - STAY_Q[1], STAY_P[0] - STAY_Q[0]))
for s, z in (("p", STAY_Z), ("m", -STAY_Z)):
    rig.add(f"stay-{s}", (STAY_Q[0], STAY_Q[1] + BL, z), PSI0)
rig.add("drawer-1", (0.06, 0.10 + BL, 0), 0)
rig.add("drawer-2", (0.06, 0.28 + BL, 0), 0)
DOORS = {
    "linen-door-p": ("linen", 0.082, 1.938, +1), "linen-door-m": ("linen", 0.082, 1.938, -1),
    "centre-door-p": ("centre", 0.86, 1.938, +1), "centre-door-m": ("centre", 0.86, 1.938, -1),
    "tv-door-lp": ("tv", 0.082, 0.789, +1), "tv-door-lm": ("tv", 0.082, 0.789, -1),
    "tv-door-up": ("tv", 1.361, 1.938, +1), "tv-door-um": ("tv", 1.361, 1.938, -1),
}
for d, (b, y0, y1, side) in DOORS.items():
    rig.add(d, (X_F0, y0 + BL, ZC[b] + side * LEAF), 0)

K = Kit(rig)
G = Draw(rig)
S = Draw.shape


def leaf_z(body, side):
    return (ZC[body] + 0.0015, ZC[body] + LEAF) if side > 0 else (ZC[body] - LEAF, ZC[body] - 0.0015)


def flap_angle_q(theta):
    t = math.radians(theta)
    rx, ry = STAY_Q[0] - FLAP_PIVOT[0], STAY_Q[1] - FLAP_PIVOT[1]
    return (FLAP_PIVOT[0] + rx * math.cos(t) - ry * math.sin(t), FLAP_PIVOT[1] + rx * math.sin(t) + ry * math.cos(t))


def stay_offset(theta):
    qx, qy = flap_angle_q(theta)
    psi = math.degrees(math.atan2(STAY_P[1] - qy, STAY_P[0] - qx))
    return R((psi - PSI0 - theta + 180) % 360 - 180)


def flap_bones(theta):
    off = stay_offset(theta)
    return {"flap": {"angleDeg": R(theta)}, "stay-p": {"angleDeg": off}, "stay-m": {"angleDeg": off}}


#region 🪵 Parts of the 1:50 model
def carcass_50(body):
    """🪵 The boards of one body at 1:50 in world-rest coordinates."""
    bone = f"{body}-body"
    z = ZC[body]
    out = []
    c = lambda sid, x0, x1, y0, y1, z0, z1, mat="oak-decor-chipboard", role="body", tone=0.55, **f: out.append(K.box(f"{body}-{sid}", bone, mat, role, tone, x0, x1, y0 + BL, y1 + BL, z0 + z, z1 + z, **f))
    c("side-p", 0, D, 0, Y_T1, IN, HW)
    c("side-m", 0, D, 0, Y_T1, -HW, -IN)
    c("bottom", X_BK1, D, Y_PL, Y_B1, -IN, IN)
    c("top", X_BK1, D, Y_T0, Y_T1, -IN, IN)
    c("plinth", X_PL0, X_PL1, 0, Y_PL, -IN, IN, tone=0.35)
    if body == "tv":
        hole = [(y + BL, zz + z) for y, zz in circle(0.95, 0.0, 0.03, 24)]
        out.append(K.yzring("tv-back", bone, "hardboard", "body", 0.8, [(Y_PL + BL, -IN + z), (Y_PL + BL, IN + z), (Y_T1 + BL, IN + z), (Y_T1 + BL, -IN + z)], [hole], X_BK0, X_BK1, sheet=True))
    else:
        c("back", X_BK0, X_BK1, Y_PL, Y_T1, -IN, IN, mat="hardboard", tone=0.8, sheet=True)
    out.append(K.xprof(f"{body}-cornice", bone, "oak-decor-mdf", "body", 0.45, wp(body, CORNICE_50), -HW + z, HW + z))
    if body == "linen":
        c("shelves", 0.031, 0.47, 0.46, 0.479, -0.43, 0.43, tone=0.6, repeat={"count": 4, "translation": [0, 0.36, 0]})
    if body == "centre":
        c("bar-bottom", X_BK1, D, 0.44, 0.459, -IN, IN)
        c("bar-top", X_BK1, D, 0.86, 0.879, -IN, IN)
        c("mirror", X_BK1, 0.015, 0.459, 0.86, -IN, IN, mat="mirror-glass", role="detail", tone=0.9, sheet=True)
        c("bar-shelf", 0.02, 0.44, 0.655, 0.661, -0.43, 0.43, mat="float-glass", role="glazing", tone=0.9, sheet=True)
        c("display-shelves", 0.03, 0.44, 1.22, 1.226, -0.43, 0.43, mat="float-glass", role="glazing", tone=0.9, sheet=True, repeat={"count": 2, "translation": [0, 0.35, 0]})
        c("guides", STAY_P[0] - 0.012, STAY_P[0] + 0.012, STAY_P[1] - 0.012, STAY_P[1] + 0.012, 0.421, IN, mat="zinc-plated-steel", role="detail", tone=0.3, repeat={"count": 2, "translation": [0, 0, -0.852]})
    if body == "tv":
        c("niche-bottom", X_BK1, D, 0.78, 0.799, -IN, IN)
        c("niche-top", X_BK1, D, 1.35, 1.369, -IN, IN)
        c("shelf-lower", 0.031, 0.47, 0.43, 0.449, -0.43, 0.43, tone=0.6)
        c("shelf-upper", 0.031, 0.47, 1.64, 1.659, -0.43, 0.43, tone=0.6)
    return out


def routed_front(sid, bone, z0, z1, y0, y1, frame):
    """🚪 A routed MDF front: 11 mm slab and the 8 mm proud frame of the routed frame-and-panel look."""
    outer = [(y0, z0), (y0, z1), (y1, z1), (y1, z0)]
    hole = [(y0 + frame, z0 + frame), (y0 + frame, z1 - frame), (y1 - frame, z1 - frame), (y1 - frame, z0 + frame)]
    return [
        K.yzring(f"{sid}-slab", bone, "oak-decor-mdf", "body", 0.5, outer, None, X_F0, X_SLAB),
        K.yzring(f"{sid}-frame", bone, "oak-decor-mdf", "body", 0.45, outer, [hole], X_SLAB, X_F1),
    ]


def fronts_50():
    out = []
    for d, (b, y0, y1, side) in DOORS.items():
        z0, z1 = leaf_z(b, side)
        Y0, Y1 = y0 + BL, y1 + BL
        if b == "centre":
            outer = [(Y0, z0), (Y0, z1), (Y1, z1), (Y1, z0)]
            hole = [(Y0 + 0.055, z0 + 0.055), (Y0 + 0.055, z1 - 0.055), (Y1 - 0.055, z1 - 0.055), (Y1 - 0.055, z0 + 0.055)]
            out.append(K.yzring(f"{d}-frame", d, "oak-decor-mdf", "body", 0.45, outer, [hole], X_F0, X_F1))
            out.append(K.box(f"{d}-glass", d, "float-glass", "glazing", 0.9, 0.488, X_SLAB, Y0 + 0.055, Y1 - 0.055, z0 + 0.055, z1 - 0.055, sheet=True))
            out.append(K.knob(f"{d}-knob", d, "beech-knob", "detail", 0.3, X_F1, 1.10 + BL, ZC[b] + side * 0.03, KNOB_S))
            continue
        out += routed_front(d, d, z0, z1, Y0, Y1, 0.07)
        knob_y = {"linen": 1.0, "tv": 0.70 if y0 < 0.5 else 1.45}[b]
        if d != "tv-door-um":
            out.append(K.knob(f"{d}-knob", d, "beech-knob", "detail", 0.3, X_F1, knob_y + BL, ZC[b] + side * 0.0415, KNOB))
    for n, (y0, y1, by0) in {1: (0.082, 0.258, 0.100), 2: (0.261, 0.437, 0.280)}.items():
        bone = f"drawer-{n}"
        out += routed_front(bone, bone, -LEAF, LEAF, y0 + BL, y1 + BL, 0.04)
        out.append(K.box(f"{bone}-bottom", bone, "hardboard", "detail", 0.75, 0.06, X_F0, by0 + BL, by0 + 0.004 + BL, -0.418, 0.418, sheet=True))
        out.append(K.xzring(f"{bone}-box", bone, "oak-decor-chipboard", "detail", 0.65, rect_pts(0.06, X_F0, -0.418, 0.418), [rect_pts(0.072, 0.469, -0.406, 0.406)], by0 + 0.004 + BL, by0 + 0.124 + BL))
        mid = by0 + 0.064 + BL
        out.append(K.box(f"{bone}-runners", bone, "zinc-plated-steel", "detail", 0.3, 0.07, 0.47, mid - 0.0085, mid + 0.0085, 0.418, IN, repeat={"count": 2, "translation": [0, 0, -0.849]}))
        ky = (y0 + y1) / 2 + BL
        out.append(K.knob(f"{bone}-knob-m", bone, "beech-knob", "detail", 0.3, X_SLAB, ky, -0.225, KNOB))
        if n == 1:
            out.append(K.knob(f"{bone}-knob-p", bone, "beech-knob", "detail", 0.3, X_SLAB, ky, 0.225, KNOB))
        else:
            u = HANDLE_50
            out.append(K.xzring("drawer-2-handle", bone, "chrome-steel", "detail", 0.85, [(x, z) for z, x in u], None, ky - 0.005, ky + 0.005))
    fy0, fy1 = 0.44 + BL, 0.857 + BL
    outer = [(fy0, -LEAF), (fy0, LEAF), (fy1, LEAF), (fy1, -LEAF)]
    hole = [(fy0 + 0.05, -LEAF + 0.05), (fy0 + 0.05, LEAF - 0.05), (fy1 - 0.05, LEAF - 0.05), (fy1 - 0.05, -LEAF + 0.05)]
    out.append(K.yzring("flap-slab", "flap", "oak-decor-mdf", "body", 0.5, outer, None, X_F0, X_SLAB))
    out.append(K.yzring("flap-frame", "flap", "oak-decor-mdf", "body", 0.45, outer, [hole], X_SLAB, X_F1))
    out.append(K.knob("flap-knob", "flap", "beech-knob", "detail", 0.3, X_F1, FLAP_KNOB_Y + BL, 0.0, KNOB))
    for s, z in (("p", STAY_Z), ("m", -STAY_Z)):
        q = (STAY_Q[0], STAY_Q[1] + BL, z)
        dirx, diry = math.cos(math.radians(PSI0)), math.sin(math.radians(PSI0))
        out.append(K.capsule(f"stay-{s}-rod", f"stay-{s}", "zinc-plated-steel", "detail", 0.3, q, (q[0] + STAY_L * dirx, q[1] + STAY_L * diry, z), STAY_R, slender=True))
    return out


def blankets(scale):
    pts = rounded_strip(-0.03, 0.53, 0.0, BL) if scale == 50 else rect_pts(-0.03, 0.53, 0.0, BL)
    return K.xprof(f"blankets-{scale}" if scale != 50 else "blankets", "blankets", "moving-blanket", "body", 0.35, pts, -1.35, -0.45, sheet=(scale != 50), repeat={"count": 3, "translation": [0, 0, 0.9]})


def lamp(scale):
    r = {50: 0.013, 100: 0.0135, 200: 0.015, 500: 0.0375}[scale]
    if scale == 500:
        return K.box("lamp-500", "centre-body", "lamp-glass", "detail", 0.95, 0.30, 0.375, 0.86 - 0.075 + BL, 0.86 + BL, -0.15, 0.15)
    return K.capsule(f"lamp-{scale}", "centre-body", "lamp-glass", "detail", 0.95, (0.40, 0.86 - r - 0.006 + BL, -0.13), (0.40, 0.86 - r - 0.006 + BL, 0.13), r)
#endregion


body50 = carcass_50("linen") + carcass_50("centre") + carcass_50("tv") + fronts_50() + [blankets(50)]
ids50 = {s["id"]: s for s in body50}

#region ⚖️ Masses from the 1:50 parts
MASS = {}
for s in body50:
    m = s["_vol"] * DENSITY[s["material"]]
    e = MASS.setdefault(s["bone"], [0.0, 0.0, 0.0])
    e[0] += m
    e[1] += m * s["_c"][0]
    e[2] += m * s["_c"][1]
LAMP_KG = 0.03
MASS["centre-body"][0] += LAMP_KG
BONE_MASS = {}
for bone, (m, mx, my) in MASS.items():
    cx, cy = mx / m, my / m
    u, v, _ = rig.loc(bone, cx, cy, 0)
    BONE_MASS[bone] = {"kg": round(m, 3), "comX": R(u), "comY": R(v)}
TOTAL_KG = round(sum(v["kg"] for v in BONE_MASS.values()), 3)
BODY_KG = {b: round(sum(v["kg"] for k, v in BONE_MASS.items() if k == f"{b}-body" or k.startswith(f"{b}-door")), 2) for b in ("linen", "tv")}
BODY_KG["centre"] = round(sum(v["kg"] for k, v in BONE_MASS.items() if k in ("centre-body", "flap", "stay-p", "stay-m", "drawer-1", "drawer-2", "centre-door-p", "centre-door-m")), 2)
#endregion


#region 🌡️🏗️ Lists of 1:50
BODIES = ("linen", "centre", "tv")
FIXED = {"linen": [], "centre": ["bar-bottom", "bar-top"], "tv": ["niche-bottom", "niche-top"]}


def thermal_fronts_50():
    """🌡️ Fronts of the thermal model at 1:50 with the 3 mm running gaps closed (the interiors are not ventilated in the model)."""
    out = []
    for d, (b, y0, y1, side) in DOORS.items():
        z0, z1 = (ZC[b], ZC[b] + LEAF) if side > 0 else (ZC[b] - LEAF, ZC[b])
        if b == "centre":
            Y0, Y1 = 0.8585 + BL, y1 + BL
            gz0, gz1 = leaf_z(b, side)
            hole = [(y1 + BL - 0.055, gz0 + 0.055), (y1 + BL - 0.055, gz1 - 0.055), (y0 + BL + 0.055, gz1 - 0.055), (y0 + BL + 0.055, gz0 + 0.055)]
            out.append({**K.yzring(f"th-{d}", d, "oak-decor-mdf", "body", 0.45, [(Y0, z0), (Y0, z1), (Y1, z1), (Y1, z0)], [hole], X_F0, X_F1), "region": "fronts"})
            continue
        out.append({**K.box(f"th-{d}", d, "oak-decor-mdf", "body", 0.5, X_F0, X_F1, y0 + BL, y1 + BL, z0, z1), "region": "fronts"})
    out.append({**K.box("th-drawer-1", "drawer-1", "oak-decor-mdf", "body", 0.5, X_F0, X_F1, 0.082 + BL, 0.2595 + BL, -LEAF, LEAF), "region": "fronts"})
    out.append({**K.box("th-drawer-2", "drawer-2", "oak-decor-mdf", "body", 0.5, X_F0, X_F1, 0.2595 + BL, 0.4385 + BL, -LEAF, LEAF), "region": "fronts"})
    out.append({**K.box("th-flap", "flap", "oak-decor-mdf", "body", 0.5, X_F0, X_F1, 0.4385 + BL, 0.8585 + BL, -LEAF, LEAF), "region": "fronts"})
    return out


def thermal_50():
    out = []
    for b in BODIES:
        for p in ["side-p", "side-m", "bottom", "top", "plinth"] + FIXED[b]:
            out.append(ref(f"th-{b}-{p}", f"{b}-{p}", region="carcass"))
        out.append(ref(f"th-{b}-cornice", f"{b}-cornice", region="cornice"))
        out.append(ref(f"th-{b}-back", f"{b}-back", region="back-panel"))
    out.append(ref("th-blankets", "blankets", region="blankets"))
    out += [ref(f"th-{d}-glass", f"{d}-glass", region="glazing") for d in ("centre-door-p", "centre-door-m")]
    out += thermal_fronts_50()
    out.append({**lamp(50), "region": "bar-lamp"})
    return out


def com_world(bone):
    m = BONE_MASS[bone]
    ox, oy, oz = rig.origin(bone)
    return (ox + m["comX"], oy + m["comY"], oz)


def supports_and_masses(scale, pad, radius):
    out = []
    for b in ("linen", "tv"):
        out.append({**K.box(f"support-plinth-{b}", f"{b}-body", "oak-decor-chipboard", "ground", 0.2, 0, D, BL, BL + pad, ZC[b] - HW, ZC[b] + HW), "member": f"{b}-body", "part": "support", "anchor": f"plinth-{b}"})
    for b in BODIES:
        out.append({**K.ellipsoid(f"mass-{b}", f"{b}-body", "oak-decor-chipboard", "detail", 0.3, com_world(f"{b}-body"), (radius, radius, radius)), "member": f"{b}-body", "part": "mass"})
    return out


def flap_hinges(size):
    h = size / 2
    return {**K.box(f"hinges-{int(size * 1000)}", "flap", "zinc-plated-steel", "detail", 0.25, X_F1 - size, X_F1, FLAP_PIVOT[1] + BL - h, FLAP_PIVOT[1] + BL + h, 0.30, 0.36, repeat={"count": 2, "translation": [0, 0, -0.66]}), "member": "flap", "part": "joint"}


def stay_pins(size):
    out = []
    for s, z in (("p", STAY_Z), ("m", -STAY_Z)):
        zz = (z - size / 2, z + size / 2) if size < 0.012 else ((IN - size, IN) if z > 0 else (-IN, -IN + size))
        out.append({**K.box(f"pin-{s}-{int(size * 1000)}", f"stay-{s}", "zinc-plated-steel", "detail", 0.25, X_F0 - size, X_F0, STAY_Q[1] + BL - size / 2, STAY_Q[1] + BL + size / 2, zz[0], zz[1]), "member": f"stay-{s}", "part": "joint"})
    return out


def structure_50():
    out = []
    for b in BODIES:
        for p in ["side-p", "side-m", "top", "bottom", "back"] + FIXED[b]:
            out.append(ref(f"m-{b}-{p}", f"{b}-{p}", member=f"{b}-body", part="member"))
    out.append(ref("m-blankets", "blankets", member="blankets", part="member"))
    out.append(ref("m-flap", "flap-slab", member="flap", part="member"))
    for s in ("p", "m"):
        out.append(ref(f"m-stay-{s}", f"stay-{s}-rod", member=f"stay-{s}", part="member"))
    for n in (1, 2):
        out.append(ref(f"m-drawer-{n}", f"drawer-{n}-box", member=f"drawer-{n}", part="member"))
        out.append(ref(f"j-drawer-{n}-runners", f"drawer-{n}-runners", member=f"drawer-{n}", part="joint"))
    out.append(flap_hinges(0.012))
    out += stay_pins(0.008)
    out.append(ref("j-guides", "centre-guides", member="centre-body", part="joint"))
    out += supports_and_masses(50, 0.0075, 0.03)
    return out
#endregion


#region ✏️ Drawings of 1:50
def knob_elev(bone, x0, y, prof):
    upper = [(x0 + a, y + r) for a, r in prof[1:-1]]
    lower = [(x0 + a, y - r) for a, r in reversed(prof[1:-1])]
    return G.epoly(bone, upper + lower)


def knob_plan(x0, z, prof):
    upper = [(x0 + a, z + r) for a, r in prof[1:-1]]
    lower = [(x0 + a, z - r) for a, r in reversed(prof[1:-1])]
    return Draw.poly(upper + lower)


def knob_spots():
    """📍 Every knob of the unit at 1:50: (shape id, bone, y world, z world, profile)."""
    out = []
    for d, (b, y0, y1, side) in DOORS.items():
        if b == "centre":
            out.append((f"{d}-knob", d, 1.10 + BL, ZC[b] + side * 0.03, KNOB_S))
        elif d != "tv-door-um":
            out.append((f"{d}-knob", d, {"linen": 1.0, "tv": 0.70 if y0 < 0.5 else 1.45}[b] + BL, ZC[b] + side * 0.0415, KNOB))
    for n, (y0, y1) in {1: (0.082, 0.258), 2: (0.261, 0.437)}.items():
        ky = (y0 + y1) / 2 + BL
        out.append((f"drawer-{n}-knob-m", f"drawer-{n}", ky, -0.225, KNOB))
        if n == 1:
            out.append((f"drawer-{n}-knob-p", f"drawer-{n}", ky, 0.225, KNOB))
    out.append(("flap-knob", "flap", FLAP_KNOB_Y + BL, 0.0, KNOB))
    return out


GROUP = {"linen": 0, "centre": 10, "tv": 20}


def elevation_50():
    out = []
    for b in BODIES:
        bone, g = f"{b}-body", GROUP[b]
        out.append(S(f"{b}-back", bone, "body", g + 1, True, 0.8, G.erect(bone, X_BK0, X_BK1, Y_PL + BL, Y_T1 + BL), region="back-panel", material="hardboard"))
        out.append(S(f"{b}-side", bone, "body", g + 2, True, 0.55, G.erect(bone, 0, D, BL, Y_T1 + BL), region="carcass", material="oak-decor-chipboard", pen=0.25))
        out.append(S(f"{b}-cornice", bone, "body", g + 3, True, 0.45, G.epoly(bone, wp(b, CORNICE_50)), region="cornice", material="oak-decor-mdf", pen=0.25))
    for d, (b, y0, y1, side) in DOORS.items():
        g = GROUP[b] + 4 + (0.5 if side < 0 else 0)
        out.append(S(d, d, "body", g, True, 0.5, G.erect(d, X_F0, X_F1, y0 + BL, y1 + BL), region="fronts", material="oak-decor-mdf", pen=0.25))
        if b == "centre":
            out.append(S(f"{d}-glass", d, "glazing", g - 0.2, True, 0.9, G.erect(d, 0.488, X_SLAB, y0 + BL + 0.055, y1 + BL - 0.055), region="glazing", material="float-glass"))
    for n, (y0, y1, by0) in {1: (0.082, 0.258, 0.100), 2: (0.261, 0.437, 0.280)}.items():
        bone = f"drawer-{n}"
        out.append(S(f"{bone}-box", bone, "detail", 13, True, 0.65, G.erect(bone, 0.06, X_F0, by0 + BL, by0 + 0.124 + BL), material="oak-decor-chipboard"))
        out.append(S(bone, bone, "body", 14, True, 0.5, G.erect(bone, X_F0, X_F1, y0 + BL, y1 + BL), region="fronts", material="oak-decor-mdf", pen=0.25))
        if n == 2:
            ky = (y0 + y1) / 2 + BL
            out.append(S("drawer-2-handle", bone, "detail", 16, True, 0.85, G.erect(bone, X_SLAB, 0.52, ky - 0.005, ky + 0.005), material="chrome-steel"))
    for s in ("p", "m"):
        out.append(S(f"stay-{s}-rod", f"stay-{s}", "detail", 12.5, True, 0.3, Draw.rect(0, STAY_L, -STAY_R, STAY_R), material="zinc-plated-steel"))
    out.append(S("flap", "flap", "body", 14.2, True, 0.5, G.erect("flap", X_F0, X_F1, 0.44 + BL, 0.857 + BL), region="fronts", material="oak-decor-mdf", pen=0.25))
    for sid, bone, y, z, prof in knob_spots():
        out.append(S(sid, bone, "detail", 30, True, 0.3, knob_elev(bone, X_SLAB if bone.startswith("drawer") else X_F1, y, prof), material="beech-knob"))
    out.append(S("blankets", "blankets", "body", 31, True, 0.35, G.epoly("blankets", rounded_strip(-0.03, 0.53, 0, BL)), region="blankets", material="moving-blanket"))
    out.append(S("blanket-fold", "blankets", "texture", 31.5, False, 0.2, G.epoly("blankets", [(-0.024, BL / 2), (0.524, BL / 2)], closed=False)))
    tv = "tv-body"
    out.append(S("tv-faded", tv, "detail", 22.2, True, 0.66, G.erect(tv, 0, D, 1.22 + BL, Y_T1 + BL), region="carcass", material="oak-decor-chipboard"))
    out.append(S("tv-shadow-line", tv, "texture", 22.3, False, 0.3, G.epoly(tv, [(0.0, 1.22 + BL), (D, 1.22 + BL)], closed=False)))
    out.append(S("tv-sticker", tv, "detail", 22.4, True, 0.95, G.erect(tv, 0.33, 0.39, 1.46 + BL, 1.495 + BL)))
    out.append(S("tv-chip", tv, "detail", 22.4, True, 0.8, G.epoly(tv, [(0.462, BL), (D, BL), (D, 0.022 + BL)])))
    for k, (a, b2) in enumerate([((0.40, 0.03), (0.45, 0.05)), ((0.42, 0.07), (0.47, 0.082)), ((0.35, 0.02), (0.40, 0.026))]):
        out.append(S(f"tv-scuff-{k + 1}", tv, "texture", 22.5, False, 0.3, G.epoly(tv, [(a[0], a[1] + BL), (b2[0], b2[1] + BL)], closed=False)))
    return out


FLAP_PROFILE = [(0, 0), (0.417, 0), (0.417, 0.019), (0, 0.019)]
FLAP_KNOB = [(FLAP_KNOB_Y - 0.44 - 0.015, -0.025), (FLAP_KNOB_Y - 0.44 + 0.015, -0.025), (FLAP_KNOB_Y - 0.44 + 0.015, 0), (FLAP_KNOB_Y - 0.44 - 0.015, 0)]


def flap_solid(z0, z1):
    return {"kind": "extruded", "sweep": "w", "z0": R(z0), "z1": R(z1)}


def pierced(z0, z1, y0, y1, zc, yc, rad):
    """🕳️ A rectangle of the front elevation with a round hole through it (even-odd path): the panel around the cable hole of the television body."""
    return {"kind": "path", "d": f"M {z0:.4f} {y0:.4f} L {z1:.4f} {y0:.4f} L {z1:.4f} {y1:.4f} L {z0:.4f} {y1:.4f} Z M {zc + rad:.4f} {yc:.4f} A {rad} {rad} 0 1 0 {zc - rad:.4f} {yc:.4f} A {rad} {rad} 0 1 0 {zc + rad:.4f} {yc:.4f} Z"}


def front_50():
    """🪟 The front elevation of 1:50. The 60 mm cable hole goes through the back of the television body and nothing stands behind it, so the body is open there: the outline shape of that body and its back are drawn with the hole and the hole itself as an unfilled circle, not as a dark disc on a closed panel (silhouette repair SF of 2026-11-23, `catalog-silhouette` 80.6 mm)."""
    out = []
    for b in BODIES:
        bone, zc = f"{b}-body", ZC[b]
        out.append(S(f"{b}-side", bone, "body", 1, True, 0.55, pierced(zc - HW, zc + HW, BL, Y_T1 + BL, zc, 0.95 + BL, 0.03) if b == "tv" else Draw.rect(zc - HW, zc + HW, BL, Y_T1 + BL), region="carcass", material="oak-decor-chipboard", pen=0.25))
        out.append(S(f"{b}-plinth", bone, "body", 1.2, True, 0.3, Draw.rect(zc - IN, zc + IN, BL, Y_PL + BL), region="carcass", material="oak-decor-chipboard"))
        for k in range(3):
            z = zc - 0.3 + 0.25 * k
            out.append(S(f"{b}-plinth-scuff-{k + 1}", bone, "texture", 1.3, False, 0.2, Draw.poly([(z, 0.02 + BL + 0.01 * k), (z + 0.09, 0.03 + BL + 0.008 * k)], closed=False)))
        out.append(S(f"{b}-cornice", bone, "body", 6, True, 0.45, Draw.rect(zc - HW, zc + HW, Y_T1 + BL, Y_TOP + BL), region="cornice", material="oak-decor-mdf", pen=0.25))
        for k, y in enumerate((1.95, 1.968, 1.982)):
            out.append(S(f"{b}-cornice-line-{k + 1}", bone, "texture", 6.1, False, 0.2, Draw.poly([(zc - HW, y + BL), (zc + HW, y + BL)], closed=False)))
    c = "centre-body"
    out.append(S("centre-back", c, "body", 1.5, True, 0.8, Draw.rect(-IN, IN, 0.879 + BL, Y_T0 + BL), region="back-panel", material="hardboard"))
    out.append(S("centre-mirror", c, "detail", 1.5, True, 0.9, Draw.rect(-IN, IN, 0.459 + BL, 0.86 + BL), material="mirror-glass"))
    out.append(S("centre-bar-shelf", c, "glazing", 1.7, True, 0.85, Draw.rect(-0.43, 0.43, 0.655 + BL, 0.661 + BL), material="float-glass"))
    out.append(S("centre-bar-bottom", c, "body", 1.8, True, 0.55, Draw.rect(-IN, IN, 0.44 + BL, 0.459 + BL), region="carcass", material="oak-decor-chipboard"))
    out.append(S("centre-bar-top", c, "body", 1.8, True, 0.55, Draw.rect(-IN, IN, 0.86 + BL, 0.879 + BL), region="carcass", material="oak-decor-chipboard"))
    for k, y in enumerate((1.22, 1.57)):
        out.append(S(f"centre-display-shelf-{k + 1}", c, "detail", 5.3, False, 0.3, Draw.poly([(-0.43, y + BL), (0.43, y + BL)], closed=False), material="float-glass"))
    tv = "tv-body"
    zc = ZC["tv"]
    out.append(S("tv-back", tv, "body", 1.5, True, 0.8, pierced(zc - IN, zc + IN, 0.799 + BL, 1.35 + BL, zc, 0.95 + BL, 0.03), region="back-panel", material="hardboard"))
    out.append(S("tv-cable-hole", tv, "detail", 1.6, False, 0.1, Draw.ellipse(zc, 0.95 + BL, 0.03, 0.03)))
    out.append(S("tv-tv-mark", tv, "texture", 1.7, False, 0.3, Draw.rect(zc - 0.30, zc + 0.30, 0.799 + BL, 1.28 + BL)))
    out.append(S("tv-niche-bottom", tv, "body", 2, True, 0.55, Draw.rect(zc - IN, zc + IN, 0.78 + BL, 0.799 + BL), region="carcass", material="oak-decor-chipboard"))
    out.append(S("tv-niche-top", tv, "body", 2, True, 0.55, Draw.rect(zc - IN, zc + IN, 1.35 + BL, 1.369 + BL), region="carcass", material="oak-decor-chipboard"))
    for d, (b, y0, y1, side) in DOORS.items():
        z0, z1 = leaf_z(b, side)
        Y0, Y1 = y0 + BL, y1 + BL
        if b == "centre":
            out.append(S(f"{d}-glass", d, "glazing", 4.8, True, 0.9, Draw.rect(z0 + 0.055, z1 - 0.055, Y0 + 0.055, Y1 - 0.055), region="glazing", material="float-glass"))
            out.append(S(d, d, "body", 5, True, 0.45, Draw.path([rect_pts(z0, z1, Y0, Y1), rect_pts(z0 + 0.055, z1 - 0.055, Y0 + 0.055, Y1 - 0.055)]), region="fronts", material="oak-decor-mdf", pen=0.25))
            out.append(S(f"{d}-knob", d, "detail", 7, True, 0.3, Draw.ellipse(ZC[b] + side * 0.03, 1.10 + BL, 0.011, 0.011), material="beech-knob"))
            continue
        out.append(S(d, d, "body", 5, True, 0.5, Draw.rect(z0, z1, Y0, Y1), region="fronts", material="oak-decor-mdf", pen=0.25, hatch="grain-v"))
        out.append(S(f"{d}-routing", d, "detail", 5.1, False, 0.3, Draw.rect(z0 + 0.07, z1 - 0.07, Y0 + 0.07, Y1 - 0.07)))
    out.append(S("centre-door-p-sticker", "centre-door-p", "detail", 5.5, True, 0.95, Draw.rect(0.25, 0.31, 1.30 + BL, 1.335 + BL)))
    out.append(S("tv-door-lm-chip", "tv-door-lm", "detail", 5.2, True, 0.8, Draw.poly([(-1.3485, 0.082 + BL), (-1.3285, 0.082 + BL), (-1.3485, 0.094 + BL)])))
    out.append(S("linen-door-p-chip", "linen-door-p", "detail", 5.2, True, 0.8, Draw.poly([(1.3485, 0.082 + BL), (1.3485, 0.097 + BL), (1.3305, 0.082 + BL)])))
    out.append(S("tv-door-um-screw-hole", "tv-door-um", "detail", 5.2, True, 0.1, Draw.ellipse(-0.9 - 0.0415, 1.45 + BL, 0.003, 0.003)))
    for sid, bone, y, z, prof in knob_spots():
        if bone == "flap" or bone.startswith("centre-door"):
            continue
        out.append(S(sid, bone, "detail", 7, True, 0.3, Draw.ellipse(z, y, 0.015, 0.015), material="beech-knob"))
    for n, (y0, y1) in {1: (0.082, 0.258), 2: (0.261, 0.437)}.items():
        bone = f"drawer-{n}"
        out.append(S(bone, bone, "body", 5, True, 0.5, Draw.rect(-LEAF, LEAF, y0 + BL, y1 + BL), region="fronts", material="oak-decor-mdf", pen=0.25, hatch="grain-h"))
        out.append(S(f"{bone}-routing", bone, "detail", 5.1, False, 0.3, Draw.rect(-LEAF + 0.04, LEAF - 0.04, y0 + BL + 0.04, y1 + BL - 0.04)))
    ky = (0.261 + 0.437) / 2 + BL
    out.append(S("drawer-2-handle", "drawer-2", "detail", 7, True, 0.85, Draw.rect(0.172, 0.278, ky - 0.005, ky + 0.005), material="chrome-steel"))
    out.append(S("flap", "flap", "body", 5, True, 0.5, Draw.poly(FLAP_PROFILE), region="fronts", material="oak-decor-mdf", pen=0.25, solid=flap_solid(-LEAF, LEAF)))
    out.append(S("flap-knob", "flap", "detail", 7, True, 0.3, Draw.poly(FLAP_KNOB), material="beech-knob", solid=flap_solid(-0.015, 0.015)))
    for k, zc in enumerate((-0.9, 0.0, 0.9)):
        out.append(S(f"blanket-{k + 1}", "blankets", "body", 6.5, True, 0.35, Draw.rect(zc - HW, zc + HW, 0, BL), region="blankets", material="moving-blanket"))
    return out


def door_section(z0, z1, frame):
    return [(X_F0, z0), (X_F1, z0), (X_F1, z0 + frame), (X_SLAB, z0 + frame), (X_SLAB, z1 - frame), (X_F1, z1 - frame), (X_F1, z1), (X_F0, z1)]


def plan_50():
    out = []
    for k, zc in enumerate((-0.9, 0.0, 0.9)):
        out.append(S(f"blanket-{k + 1}", "blankets", "body", 0.5, True, 0.35, Draw.rect(-0.03, 0.53, zc - HW, zc + HW), region="blankets", material="moving-blanket"))
        out.append(S(f"blanket-{k + 1}-fold", "blankets", "texture", 0.6, False, 0.2, Draw.poly([(-0.024, zc - HW), (-0.024, zc + HW)], closed=False)))
    for b in BODIES:
        bone, zc = f"{b}-body", ZC[b]
        out.append(S(f"{b}-bottom", bone, "body", 1, True, 0.6, Draw.rect(X_BK1, D, zc - IN, zc + IN), region="carcass", material="oak-decor-chipboard"))
        if b == "linen":
            out.append(S("linen-shelves", bone, "body", 1.5, True, 0.65, Draw.rect(0.031, 0.47, zc - 0.43, zc + 0.43), region="carcass", material="oak-decor-chipboard"))
        if b == "tv":
            out.append(S("tv-shelf-lower", bone, "body", 1.5, True, 0.65, Draw.rect(0.031, 0.47, zc - 0.43, zc + 0.43), region="carcass", material="oak-decor-chipboard"))
        if b == "centre":
            out.append(S("centre-bar-bottom", bone, "body", 1.5, True, 0.65, Draw.rect(X_BK1, D, -IN, IN), region="carcass", material="oak-decor-chipboard"))
            out.append(S("centre-mirror", bone, "detail", 5, True, 0.85, Draw.rect(X_BK1, 0.015, -IN, IN), material="mirror-glass"))
        out.append(S(f"{b}-back", bone, "body", 4.9, True, 0.7, Draw.rect(X_BK0, X_BK1, zc - IN - 0.006, zc + IN + 0.006), region="back-panel", material="hardboard", pen=0.25))
        for s, (z0, z1) in (("p", (zc + IN, zc + HW)), ("m", (zc - HW, zc - IN))):
            out.append(S(f"{b}-side-{s}", bone, "body", 5, True, 0.3, Draw.rect(0, D, z0, z1), region="carcass", material="oak-decor-chipboard", pen=0.25))
    for d in ("linen-door-p", "linen-door-m", "tv-door-lp", "tv-door-lm"):
        b, y0, y1, side = DOORS[d]
        z0, z1 = leaf_z(b, side)
        out.append(S(d, d, "body", 5, True, 0.3, Draw.poly(door_section(z0, z1, 0.07)), region="fronts", material="oak-decor-mdf", pen=0.25))
    out.append(S("flap", "flap", "body", 5, True, 0.3, Draw.poly(FLAP_PROFILE), region="fronts", material="oak-decor-mdf", pen=0.25, solid=flap_solid(-LEAF, LEAF)))
    out.append(S("drawer-2-knob-m", "drawer-2", "detail", 6, True, 0.3, knob_plan(X_SLAB, -0.225, KNOB), material="beech-knob"))
    out.append(S("drawer-2-handle", "drawer-2", "detail", 6, True, 0.85, Draw.poly([(x, z) for z, x in HANDLE_50]), material="chrome-steel"))
    return out
#endregion


#region 🧱 1:100 — 15 mm feature
def carcass_100(body):
    bone, z = f"{body}-body", ZC[body]
    out = []
    c = lambda sid, x0, x1, y0, y1, z0, z1, mat="oak-decor-chipboard", role="body", tone=0.55, **f: out.append(K.box(f"{body}-{sid}", bone, mat, role, tone, x0, x1, y0 + BL, y1 + BL, z0 + z, z1 + z, **f))
    c("side-p", 0, D, 0, Y_T1, IN, HW)
    c("side-m", 0, D, 0, Y_T1, -HW, -IN)
    c("bottom", X_BK1, D, Y_PL, Y_B1, -IN, IN)
    c("top", X_BK1, D, Y_T0, Y_T1, -IN, IN)
    c("plinth", X_PL0, X_PL1, 0, Y_PL, -IN, IN, tone=0.35)
    if body == "tv":
        hole = [(y + BL, zz + z) for y, zz in circle(0.95, 0.0, 0.03, 16)]
        out.append(K.yzring("tv-back", bone, "hardboard", "body", 0.8, [(Y_PL + BL, -IN + z), (Y_PL + BL, IN + z), (Y_T1 + BL, IN + z), (Y_T1 + BL, -IN + z)], [hole], X_BK0, X_BK1, sheet=True))
    else:
        c("back", X_BK0, X_BK1, Y_PL, Y_T1, -IN, IN, mat="hardboard", tone=0.8, sheet=True)
    out.append(K.xprof(f"{body}-cornice", bone, "oak-decor-mdf", "body", 0.45, wp(body, CORNICE_100), -HW + z, HW + z))
    if body == "linen":
        c("shelves", 0.031, 0.47, 0.46, 0.479, -0.43, 0.43, tone=0.6, repeat={"count": 4, "translation": [0, 0.36, 0]})
    if body == "centre":
        c("bar-bottom", X_BK1, D, 0.44, 0.459, -IN, IN)
        c("bar-top", X_BK1, D, 0.86, 0.879, -IN, IN)
        c("mirror", X_BK1, 0.015, 0.459, 0.86, -IN, IN, mat="mirror-glass", role="detail", tone=0.9, sheet=True)
        c("bar-shelf", 0.02, 0.44, 0.655, 0.661, -0.43, 0.43, mat="float-glass", role="glazing", tone=0.9, sheet=True)
        c("display-shelves", 0.03, 0.44, 1.22, 1.226, -0.43, 0.43, mat="float-glass", role="glazing", tone=0.9, sheet=True, repeat={"count": 2, "translation": [0, 0.35, 0]})
    if body == "tv":
        c("niche-bottom", X_BK1, D, 0.78, 0.799, -IN, IN)
        c("niche-top", X_BK1, D, 1.35, 1.369, -IN, IN)
        c("shelf-lower", 0.031, 0.47, 0.43, 0.449, -0.43, 0.43, tone=0.6)
        c("shelf-upper", 0.031, 0.47, 1.64, 1.659, -0.43, 0.43, tone=0.6)
    return out


def fronts_100():
    out = []
    for d, (b, y0, y1, side) in DOORS.items():
        z0, z1 = leaf_z(b, side)
        Y0, Y1 = y0 + BL, y1 + BL
        if b == "centre":
            outer = [(Y0, z0), (Y0, z1), (Y1, z1), (Y1, z0)]
            hole = [(Y0 + 0.055, z0 + 0.055), (Y0 + 0.055, z1 - 0.055), (Y1 - 0.055, z1 - 0.055), (Y1 - 0.055, z0 + 0.055)]
            out.append(K.yzring(f"{d}-frame", d, "oak-decor-mdf", "body", 0.45, outer, [hole], X_F0, X_F1))
            out.append(K.box(f"{d}-glass", d, "float-glass", "glazing", 0.9, 0.488, X_SLAB, Y0 + 0.055, Y1 - 0.055, z0 + 0.055, z1 - 0.055, sheet=True))
            out.append(K.knob(f"{d}-knob", d, "beech-knob", "detail", 0.3, X_F1, 1.10 + BL, ZC[b] + side * 0.03, [(a * 0.75, r * 0.75) for a, r in KNOB_100]))
            continue
        out.append(K.box(f"{d}-slab", d, "oak-decor-mdf", "body", 0.5, X_F0, X_F1, Y0, Y1, z0, z1))
        if d != "tv-door-um":
            out.append(K.knob(f"{d}-knob", d, "beech-knob", "detail", 0.3, X_F1, {"linen": 1.0, "tv": 0.70 if y0 < 0.5 else 1.45}[b] + BL, ZC[b] + side * 0.0415, KNOB_100))
    for n, (y0, y1, by0) in {1: (0.082, 0.258, 0.100), 2: (0.261, 0.437, 0.280)}.items():
        bone = f"drawer-{n}"
        out.append(K.box(f"{bone}-front", bone, "oak-decor-mdf", "body", 0.5, X_F0, X_F1, y0 + BL, y1 + BL, -LEAF, LEAF))
        out.append(K.box(f"{bone}-box", bone, "oak-decor-chipboard", "detail", 0.65, 0.06, X_F0, by0 + BL, by0 + 0.124 + BL, -0.43, 0.43))
        ky = (y0 + y1) / 2 + BL
        out.append(K.knob(f"{bone}-knob-m", bone, "beech-knob", "detail", 0.3, X_F1, ky, -0.225, KNOB_100))
        if n == 1:
            out.append(K.knob(f"{bone}-knob-p", bone, "beech-knob", "detail", 0.3, X_F1, ky, 0.225, KNOB_100))
        else:
            out.append(K.box("drawer-2-handle", bone, "chrome-steel", "detail", 0.85, X_F1, 0.528, ky - 0.0075, ky + 0.0075, 0.172, 0.278))
    out.append(K.box("flap-slab", "flap", "oak-decor-mdf", "body", 0.5, X_F0, X_F1, 0.44 + BL, 0.857 + BL, -LEAF, LEAF))
    out.append(K.knob("flap-knob", "flap", "beech-knob", "detail", 0.3, X_F1, FLAP_KNOB_Y + BL, 0.0, KNOB_100))
    for s, z in (("p", STAY_Z), ("m", -STAY_Z)):
        q = (STAY_Q[0], STAY_Q[1] + BL, z)
        dirx, diry = math.cos(math.radians(PSI0)), math.sin(math.radians(PSI0))
        out.append(K.capsule(f"stay-{s}-rod", f"stay-{s}", "zinc-plated-steel", "detail", 0.3, q, (q[0] + STAY_L * dirx, q[1] + STAY_L * diry, z), STAY_R, slender=True))
    return out


body100 = carcass_100("linen") + carcass_100("centre") + carcass_100("tv") + fronts_100() + [blankets(100)]


def thermal_100():
    out = []
    for b in BODIES:
        for p in ["side-p", "side-m", "bottom", "top", "plinth"] + FIXED[b]:
            out.append(ref(f"th-{b}-{p}", f"{b}-{p}", region="carcass"))
        out.append(ref(f"th-{b}-cornice", f"{b}-cornice", region="cornice"))
        out.append(ref(f"th-{b}-back", f"{b}-back", region="back-panel"))
    out.append(ref("th-blankets", "blankets-100", region="blankets"))
    for d, (b, y0, y1, side) in DOORS.items():
        if b == "centre":
            out.append(ref(f"th-{d}-glass", f"{d}-glass", region="glazing"))
            out.append(ref(f"th-{d}", f"{d}-frame", region="fronts"))
        else:
            out.append(ref(f"th-{d}", f"{d}-slab", region="fronts"))
    out += [ref(f"th-drawer-{n}", f"drawer-{n}-front", region="fronts") for n in (1, 2)]
    out.append(ref("th-flap", "flap-slab", region="fronts"))
    out.append({**lamp(100), "region": "bar-lamp"})
    return out


def structure_100():
    out = []
    for b in BODIES:
        for p in ["side-p", "side-m", "top", "bottom", "back"] + FIXED[b]:
            out.append(ref(f"m-{b}-{p}", f"{b}-{p}", member=f"{b}-body", part="member"))
    out.append(ref("m-blankets", "blankets-100", member="blankets", part="member"))
    out.append(ref("m-flap", "flap-slab", member="flap", part="member"))
    for s in ("p", "m"):
        out.append(ref(f"m-stay-{s}", f"stay-{s}-rod", member=f"stay-{s}", part="member"))
    for n in (1, 2):
        out.append(ref(f"m-drawer-{n}", f"drawer-{n}-box", member=f"drawer-{n}", part="member"))
    out.append(flap_hinges(0.015))
    out += stay_pins(0.015)
    out += supports_and_masses(100, 0.015, 0.035)
    return out


def elevation_100():
    out = []
    for b in BODIES:
        bone, g = f"{b}-body", GROUP[b]
        out.append(S(f"{b}-back", bone, "body", g + 1, True, 0.8, G.erect(bone, X_BK0, X_BK1, Y_PL + BL, Y_T1 + BL), region="back-panel", material="hardboard"))
        out.append(S(f"{b}-side", bone, "body", g + 2, True, 0.55, G.erect(bone, 0, D, BL, Y_T1 + BL), region="carcass", material="oak-decor-chipboard", pen=0.18))
        out.append(S(f"{b}-cornice", bone, "body", g + 3, True, 0.45, G.epoly(bone, wp(b, CORNICE_100)), region="cornice", material="oak-decor-mdf", pen=0.18))
    for d, (b, y0, y1, side) in DOORS.items():
        g = GROUP[b] + 4 + (0.5 if side < 0 else 0)
        out.append(S(d, d, "body", g, True, 0.5, G.erect(d, X_F0, X_F1, y0 + BL, y1 + BL), region="fronts", material="oak-decor-mdf", pen=0.18))
        if b == "centre":
            out.append(S(f"{d}-glass", d, "glazing", g - 0.2, True, 0.9, G.erect(d, 0.488, X_SLAB, y0 + BL + 0.055, y1 + BL - 0.055), region="glazing", material="float-glass"))
    for n, (y0, y1, by0) in {1: (0.082, 0.258, 0.100), 2: (0.261, 0.437, 0.280)}.items():
        bone = f"drawer-{n}"
        out.append(S(f"{bone}-box", bone, "detail", 13, True, 0.65, G.erect(bone, 0.06, X_F0, by0 + BL, by0 + 0.124 + BL), material="oak-decor-chipboard"))
        out.append(S(bone, bone, "body", 14, True, 0.5, G.erect(bone, X_F0, X_F1, y0 + BL, y1 + BL), region="fronts", material="oak-decor-mdf", pen=0.18))
        if n == 2:
            ky = (y0 + y1) / 2 + BL
            out.append(S("drawer-2-handle", bone, "detail", 16, True, 0.85, G.erect(bone, X_F1, 0.528, ky - 0.0075, ky + 0.0075), material="chrome-steel"))
    for s in ("p", "m"):
        out.append(S(f"stay-{s}-rod", f"stay-{s}", "detail", 12.5, True, 0.3, Draw.rect(0, STAY_L, -STAY_R, STAY_R), material="zinc-plated-steel"))
    out.append(S("flap", "flap", "body", 14.2, True, 0.5, G.erect("flap", X_F0, X_F1, 0.44 + BL, 0.857 + BL), region="fronts", material="oak-decor-mdf", pen=0.18))
    for sid, bone, y, z, prof in knob_spots():
        r = 0.01125 if bone.startswith("centre-door") else 0.015
        a = 0.01875 if bone.startswith("centre-door") else 0.025
        out.append(S(sid, bone, "detail", 30, True, 0.3, G.erect(bone, X_F1, X_F1 + a, y - r, y + r), material="beech-knob"))
    out.append(S("blankets", "blankets", "body", 31, True, 0.35, G.erect("blankets", -0.03, 0.53, 0, BL), region="blankets", material="moving-blanket"))
    tv = "tv-body"
    out.append(S("tv-faded", tv, "detail", 22.2, True, 0.66, G.erect(tv, 0, D, 1.22 + BL, Y_T1 + BL), region="carcass", material="oak-decor-chipboard"))
    out.append(S("tv-sticker", tv, "detail", 22.4, True, 0.95, G.erect(tv, 0.33, 0.39, 1.46 + BL, 1.495 + BL)))
    return out


def front_100():
    out = []
    for b in BODIES:
        bone, zc = f"{b}-body", ZC[b]
        out.append(S(f"{b}-side", bone, "body", 1, True, 0.55, Draw.rect(zc - HW, zc + HW, BL, Y_T1 + BL), region="carcass", material="oak-decor-chipboard", pen=0.18))
        out.append(S(f"{b}-plinth", bone, "body", 1.2, True, 0.3, Draw.rect(zc - IN, zc + IN, BL, Y_PL + BL), region="carcass", material="oak-decor-chipboard"))
        out.append(S(f"{b}-cornice", bone, "body", 6, True, 0.45, Draw.rect(zc - HW, zc + HW, Y_T1 + BL, Y_TOP + BL), region="cornice", material="oak-decor-mdf", pen=0.18))
        out.append(S(f"{b}-cornice-line", bone, "texture", 6.1, False, 0.2, Draw.poly([(zc - HW, 1.97 + BL), (zc + HW, 1.97 + BL)], closed=False)))
    c = "centre-body"
    out.append(S("centre-back", c, "body", 1.5, True, 0.8, Draw.rect(-IN, IN, 0.879 + BL, Y_T0 + BL), region="back-panel", material="hardboard"))
    out.append(S("centre-mirror", c, "detail", 1.5, True, 0.9, Draw.rect(-IN, IN, 0.459 + BL, 0.86 + BL), material="mirror-glass"))
    out.append(S("centre-bar-bottom", c, "body", 1.8, True, 0.55, Draw.rect(-IN, IN, 0.44 + BL, 0.459 + BL), region="carcass", material="oak-decor-chipboard"))
    out.append(S("centre-bar-top", c, "body", 1.8, True, 0.55, Draw.rect(-IN, IN, 0.86 + BL, 0.879 + BL), region="carcass", material="oak-decor-chipboard"))
    for k, y in enumerate((1.22, 1.57)):
        out.append(S(f"centre-display-shelf-{k + 1}", c, "detail", 5.3, False, 0.3, Draw.poly([(-0.43, y + BL), (0.43, y + BL)], closed=False), material="float-glass"))
    tv, zc = "tv-body", ZC["tv"]
    out.append(S("tv-back", tv, "body", 1.5, True, 0.8, Draw.rect(zc - IN, zc + IN, 0.799 + BL, 1.35 + BL), region="back-panel", material="hardboard"))
    out.append(S("tv-cable-hole", tv, "detail", 1.6, True, 0.1, Draw.ellipse(zc, 0.95 + BL, 0.03, 0.03)))
    out.append(S("tv-niche-bottom", tv, "body", 2, True, 0.55, Draw.rect(zc - IN, zc + IN, 0.78 + BL, 0.799 + BL), region="carcass", material="oak-decor-chipboard"))
    out.append(S("tv-niche-top", tv, "body", 2, True, 0.55, Draw.rect(zc - IN, zc + IN, 1.35 + BL, 1.369 + BL), region="carcass", material="oak-decor-chipboard"))
    for d, (b, y0, y1, side) in DOORS.items():
        z0, z1 = leaf_z(b, side)
        Y0, Y1 = y0 + BL, y1 + BL
        if b == "centre":
            out.append(S(f"{d}-glass", d, "glazing", 4.8, True, 0.9, Draw.rect(z0 + 0.055, z1 - 0.055, Y0 + 0.055, Y1 - 0.055), region="glazing", material="float-glass"))
            out.append(S(d, d, "body", 5, True, 0.45, Draw.path([rect_pts(z0, z1, Y0, Y1), rect_pts(z0 + 0.055, z1 - 0.055, Y0 + 0.055, Y1 - 0.055)]), region="fronts", material="oak-decor-mdf", pen=0.18))
            out.append(S(f"{d}-knob", d, "detail", 7, True, 0.3, Draw.ellipse(ZC[b] + side * 0.03, 1.10 + BL, 0.01125, 0.01125), material="beech-knob"))
            continue
        out.append(S(d, d, "body", 5, True, 0.5, Draw.rect(z0, z1, Y0, Y1), region="fronts", material="oak-decor-mdf", pen=0.18))
        out.append(S(f"{d}-routing", d, "detail", 5.1, False, 0.3, Draw.rect(z0 + 0.07, z1 - 0.07, Y0 + 0.07, Y1 - 0.07)))
    out.append(S("centre-door-p-sticker", "centre-door-p", "detail", 5.5, True, 0.95, Draw.rect(0.25, 0.31, 1.30 + BL, 1.335 + BL)))
    for sid, bone, y, z, prof in knob_spots():
        if bone == "flap" or bone.startswith("centre-door"):
            continue
        out.append(S(sid, bone, "detail", 7, True, 0.3, Draw.ellipse(z, y, 0.015, 0.015), material="beech-knob"))
    for n, (y0, y1) in {1: (0.082, 0.258), 2: (0.261, 0.437)}.items():
        bone = f"drawer-{n}"
        out.append(S(bone, bone, "body", 5, True, 0.5, Draw.rect(-LEAF, LEAF, y0 + BL, y1 + BL), region="fronts", material="oak-decor-mdf", pen=0.18))
    ky = (0.261 + 0.437) / 2 + BL
    out.append(S("drawer-2-handle", "drawer-2", "detail", 7, True, 0.85, Draw.rect(0.172, 0.278, ky - 0.0075, ky + 0.0075), material="chrome-steel"))
    out.append(S("flap", "flap", "body", 5, True, 0.5, Draw.poly(FLAP_PROFILE), region="fronts", material="oak-decor-mdf", pen=0.18, solid=flap_solid(-LEAF, LEAF)))
    out.append(S("flap-knob", "flap", "detail", 7, True, 0.3, Draw.poly(FLAP_KNOB), material="beech-knob", solid=flap_solid(-0.015, 0.015)))
    for k, zc in enumerate((-0.9, 0.0, 0.9)):
        out.append(S(f"blanket-{k + 1}", "blankets", "body", 6.5, True, 0.35, Draw.rect(zc - HW, zc + HW, 0, BL), region="blankets", material="moving-blanket"))
    return out


def plan_top(scale, top_depth, cornice_from, pen):
    """🗺️ The plain top view of the coarser scales: carcass tops, the cornice strips, the blankets showing behind."""
    out = []
    for k, zc in enumerate((-0.9, 0.0, 0.9)):
        out.append(S(f"blanket-{k + 1}", "blankets", "body", 0.5, True, 0.35, Draw.rect(-0.03, 0.53, zc - HW, zc + HW), region="blankets", material="moving-blanket"))
    for b in BODIES:
        bone, zc = f"{b}-body", ZC[b]
        out.append(S(f"{b}-top", bone, "body", 2, True, 0.55, Draw.rect(0, top_depth, zc - HW, zc + HW), region="carcass", material="oak-decor-chipboard", pen=pen))
        out.append(S(f"{b}-cornice", bone, "body", 3, True, 0.45, Draw.rect(cornice_from, 0.53, zc - HW, zc + HW), region="cornice", material="oak-decor-mdf", pen=pen))
    return out
#endregion


#region 🧱 1:200 and 1:500 — carcass tubes, fronts at the feature
COARSE = {
    200: {"t": 0.03, "top": Y_T1, "hole_top": Y_T0, "cornice": (0.46, Y_T1), "glass": (0.483, 0.487), "frame": 0.055, "pen": 0.13, "pad": 0.03, "mass": 0.06, "front_top": 1.938},
    500: {"t": 0.075, "top": 1.925, "hole_top": 1.906, "cornice": (0.455, 1.925), "glass": (0.46, 0.464), "frame": 0.075, "pen": 0.18, "pad": 0.075, "mass": 0.075, "front_top": 1.925},
}
BLANKET_DRAWN = {200: BL, 500: 0.022}
HOLES = {"linen": [(Y_B1, None)], "centre": [(Y_B1, 0.44), (0.459, 0.86), (0.879, None)], "tv": [(Y_B1, 0.78), (0.799, 1.35), (1.369, None)]}
BOARDS = {"linen": [], "centre": [("bar-bottom", 0.44, 0.459), ("bar-top", 0.86, 0.879)], "tv": [("niche-bottom", 0.78, 0.799), ("niche-top", 1.35, 1.369)]}


def coarse_body(scale):
    p = COARSE[scale]
    xf = X_F1 - p["t"]
    out = []
    for b in BODIES:
        bone, zc = f"{b}-body", ZC[b]
        outer = [(BL, zc - HW), (BL, zc + HW), (p["top"] + BL, zc + HW), (p["top"] + BL, zc - HW)]
        holes = [[(y0 + BL, zc - IN), (y0 + BL, zc + IN), ((y1 or p["hole_top"]) + BL, zc + IN), ((y1 or p["hole_top"]) + BL, zc - IN)] for y0, y1 in HOLES[b]]
        out.append(K.yzring(f"{b}-carcass", bone, "oak-decor-chipboard", "body", 0.55, outer, holes, 0.003, xf))
        back = [(BL, zc - HW), (BL, zc + HW), (p["top"] + BL, zc + HW), (p["top"] + BL, zc - HW)]
        if b == "tv":
            hole = [(y + BL, zz + zc) for y, zz in circle(0.95, 0.0, 0.03, 12)]
            out.append(K.yzring("tv-back", bone, "hardboard", "body", 0.8, back, [hole] if scale == 200 else None, 0.0, 0.003, sheet=True))
        else:
            out.append(K.yzring(f"{b}-back", bone, "hardboard", "body", 0.8, back, None, 0.0, 0.003, sheet=True))
        cx, cy = p["cornice"]
        out.append(K.box(f"{b}-cornice", bone, "oak-decor-mdf", "body", 0.45, cx, 0.53, cy + BL, Y_TOP + BL, zc - HW, zc + HW))
    for d, (b, y0, y1, side) in DOORS.items():
        z0, z1 = leaf_z(b, side)
        Y0, Y1 = y0 + BL, min(y1, p["front_top"]) + BL
        if b == "centre":
            f = p["frame"]
            outer = [(Y0, z0), (Y0, z1), (Y1, z1), (Y1, z0)]
            hole = [(Y0 + f, z0 + f), (Y0 + f, z1 - f), (Y1 - f, z1 - f), (Y1 - f, z0 + f)]
            out.append(K.yzring(f"{d}-frame", d, "oak-decor-mdf", "body", 0.45, outer, [hole], xf, X_F1))
            out.append(K.box(f"{d}-glass", d, "float-glass", "glazing", 0.9, p["glass"][0], p["glass"][1], Y0 + f, Y1 - f, z0 + f, z1 - f, sheet=True))
            continue
        out.append(K.box(f"{d}-slab", d, "oak-decor-mdf", "body", 0.5, xf, X_F1, Y0, Y1, z0, z1))
    for n, (y0, y1, by0) in {1: (0.082, 0.261, 0.100), 2: (0.261, 0.44, 0.280)}.items():
        bone = f"drawer-{n}"
        out.append(K.box(f"{bone}-front", bone, "oak-decor-mdf", "body", 0.5, xf, X_F1, y0 + BL, y1 + BL, -LEAF, LEAF))
        out.append(K.box(f"{bone}-box", bone, "oak-decor-chipboard", "detail", 0.65, 0.06, xf, by0 + BL, by0 + 0.124 + BL, -0.428 if scale == 200 else -0.42, 0.428 if scale == 200 else 0.42))
    out.append(K.box("flap-slab", "flap", "oak-decor-mdf", "body", 0.5, xf, X_F1, 0.44 + BL, 0.86 + BL, -LEAF, LEAF))
    if scale == 200:
        out.append(K.box("centre-mirror", "centre-body", "mirror-glass", "detail", 0.9, 0.003, 0.007, 0.459 + BL, 0.86 + BL, -IN, IN, sheet=True))
        out.append(K.box("centre-bar-shelf", "centre-body", "float-glass", "glazing", 0.9, 0.02, 0.44, 0.655 + BL, 0.661 + BL, -0.43, 0.43, sheet=True))
        out.append(K.box("centre-display-shelves", "centre-body", "float-glass", "glazing", 0.9, 0.03, 0.44, 1.22 + BL, 1.226 + BL, -0.43, 0.43, sheet=True, repeat={"count": 2, "translation": [0, 0.35, 0]}))
    out.append(blankets(scale))
    return out


def coarse_thermal(scale):
    out = []
    for b in BODIES:
        out.append(ref(f"th-{b}-carcass", f"{b}-carcass", region="carcass"))
        out.append(ref(f"th-{b}-cornice", f"{b}-cornice", region="cornice"))
        out.append(ref(f"th-{b}-back", f"{b}-back", region="back-panel"))
    out.append(ref("th-blankets", f"blankets-{scale}", region="blankets"))
    for d, (b, y0, y1, side) in DOORS.items():
        if b == "centre":
            out.append(ref(f"th-{d}-glass", f"{d}-glass", region="glazing"))
            out.append(ref(f"th-{d}", f"{d}-frame", region="fronts"))
        else:
            out.append(ref(f"th-{d}", f"{d}-slab", region="fronts"))
    out += [ref(f"th-drawer-{n}", f"drawer-{n}-front", region="fronts") for n in (1, 2)]
    out.append(ref("th-flap", "flap-slab", region="fronts"))
    out.append({**lamp(scale), "region": "bar-lamp"})
    return out


def coarse_structure(scale):
    p = COARSE[scale]
    xf = X_F1 - p["t"]
    out = []
    for b in BODIES:
        bone, zc = f"{b}-body", ZC[b]
        m = {"member": bone, "part": "member"}
        out.append({**K.box(f"m-{b}-side-p", bone, "oak-decor-chipboard", "body", 0.55, 0.003, xf, BL, p["top"] + BL, zc + IN, zc + HW, sheet=True), **m})
        out.append({**K.box(f"m-{b}-side-m", bone, "oak-decor-chipboard", "body", 0.55, 0.003, xf, BL, p["top"] + BL, zc - HW, zc - IN, sheet=True), **m})
        out.append({**K.box(f"m-{b}-bottom", bone, "oak-decor-chipboard", "body", 0.55, 0.003, xf, Y_PL + BL, Y_B1 + BL, zc - IN, zc + IN, sheet=True), **m})
        out.append({**K.box(f"m-{b}-top", bone, "oak-decor-chipboard", "body", 0.55, 0.003, xf, p["hole_top"] + BL, p["top"] + BL, zc - IN, zc + IN, sheet=True), **m})
        for sid, y0, y1 in BOARDS[b]:
            out.append({**K.box(f"m-{b}-{sid}", bone, "oak-decor-chipboard", "body", 0.55, 0.003, xf, y0 + BL, y1 + BL, zc - IN, zc + IN, sheet=True), **m})
        out.append(ref(f"m-{b}-back", f"{b}-back", **m))
    out.append(ref("m-blankets", f"blankets-{scale}", member="blankets", part="member"))
    out.append(ref("m-flap", "flap-slab", member="flap", part="member"))
    for n in (1, 2):
        out.append(ref(f"m-drawer-{n}", f"drawer-{n}-box", member=f"drawer-{n}", part="member"))
    for s, z in (("p", STAY_Z), ("m", -STAY_Z)):
        q = (STAY_Q[0], STAY_Q[1] + BL, z)
        dirx, diry = math.cos(math.radians(PSI0)), math.sin(math.radians(PSI0))
        out.append({**K.capsule(f"m-stay-{s}", f"stay-{s}", "zinc-plated-steel", "detail", 0.3, q, (q[0] + STAY_L * dirx, q[1] + STAY_L * diry, z), STAY_R, slender=True), "member": f"stay-{s}", "part": "member"})
    if scale == 200:
        out.append(flap_hinges(0.03))
    out += supports_and_masses(scale, p["pad"], p["mass"])
    return out


def coarse_elevation(scale):
    p = COARSE[scale]
    xf, pen = X_F1 - p["t"], p["pen"]
    out = []
    for b in BODIES:
        bone, g = f"{b}-body", GROUP[b]
        out.append(S(f"{b}-back", bone, "body", g + 1, True, 0.8, G.erect(bone, 0, 0.003, BL, p["top"] + BL), region="back-panel", material="hardboard"))
        out.append(S(f"{b}-side", bone, "body", g + 2, True, 0.55, G.erect(bone, 0, X_F1, BL, p["top"] + BL), region="carcass", material="oak-decor-chipboard", pen=pen))
        cx, cy = p["cornice"]
        out.append(S(f"{b}-cornice", bone, "body", g + 3, True, 0.45, G.erect(bone, cx, 0.53, cy + BL, Y_TOP + BL), region="cornice", material="oak-decor-mdf", pen=pen))
    for d in ("centre-door-p", "centre-door-m"):
        b, y0, y1, side = DOORS[d]
        Y1, f = min(y1, p["front_top"]) + BL, p["frame"]
        out.append(S(f"{d}-glass", d, "glazing", GROUP[b] + 1.5, True, 0.9, G.erect(d, p["glass"][0], p["glass"][1], y0 + BL + f, Y1 - f), region="glazing", material="float-glass"))
    for n, (y0, y1, by0) in {1: (0.082, 0.261, 0.100), 2: (0.261, 0.44, 0.280)}.items():
        bone = f"drawer-{n}"
        out.append(S(f"{bone}-box", bone, "body", 13, True, 0.65, G.erect(bone, 0.06, xf, by0 + BL, by0 + 0.124 + BL), material="oak-decor-chipboard"))
        out.append(S(bone, bone, "body", 14, True, 0.5, G.erect(bone, xf, X_F1, y0 + BL, y1 + BL), region="fronts", material="oak-decor-mdf", pen=pen))
    out.append(S("flap", "flap", "body", 14.2, True, 0.5, G.erect("flap", xf, X_F1, 0.44 + BL, 0.86 + BL), region="fronts", material="oak-decor-mdf", pen=pen))
    out.append(S("blankets", "blankets", "body", 31, True, 0.35, G.erect("blankets", -0.03, 0.53, 0, BLANKET_DRAWN[scale]), region="blankets", material="moving-blanket"))
    if scale == 200:
        out.append(S("tv-faded", "tv-body", "detail", 22.2, True, 0.66, G.erect("tv-body", 0, X_F1, 1.22 + BL, p["top"] + BL), region="carcass", material="oak-decor-chipboard"))
    return out


def coarse_front(scale):
    p = COARSE[scale]
    pen, f = p["pen"], p["frame"]
    out = []
    for b in BODIES:
        bone, zc = f"{b}-body", ZC[b]
        out.append(S(f"{b}-side", bone, "body", 1, True, 0.55, Draw.rect(zc - HW, zc + HW, BL, p["top"] + BL), region="carcass", material="oak-decor-chipboard", pen=pen))
        out.append(S(f"{b}-plinth", bone, "body", 1.2, True, 0.3, Draw.rect(zc - IN, zc + IN, BL, Y_PL + BL), region="carcass", material="oak-decor-chipboard"))
        out.append(S(f"{b}-cornice", bone, "body", 6, True, 0.45, Draw.rect(zc - HW, zc + HW, p["cornice"][1] + BL, Y_TOP + BL), region="cornice", material="oak-decor-mdf", pen=pen))
    out.append(S("centre-back", "centre-body", "body", 1.5, True, 0.8, Draw.rect(-IN, IN, 0.879 + BL, p["hole_top"] + BL), region="back-panel", material="hardboard"))
    tv, zc = "tv-body", ZC["tv"]
    out.append(S("tv-back", tv, "body", 1.5, True, 0.8, Draw.rect(zc - IN, zc + IN, 0.799 + BL, 1.35 + BL), region="back-panel", material="hardboard"))
    out.append(S("tv-niche-bottom", tv, "body", 2, True, 0.55, Draw.rect(zc - IN, zc + IN, 0.78 + BL, 0.799 + BL), region="carcass", material="oak-decor-chipboard"))
    for d, (b, y0, y1, side) in DOORS.items():
        z0, z1 = leaf_z(b, side)
        Y0, Y1 = y0 + BL, min(y1, p["front_top"]) + BL
        if b == "centre":
            out.append(S(f"{d}-glass", d, "glazing", 4.8, True, 0.9, Draw.rect(z0 + f, z1 - f, Y0 + f, Y1 - f), region="glazing", material="float-glass"))
            out.append(S(d, d, "body", 5, True, 0.45, Draw.path([rect_pts(z0, z1, Y0, Y1), rect_pts(z0 + f, z1 - f, Y0 + f, Y1 - f)]), region="fronts", material="oak-decor-mdf", pen=pen))
            continue
        out.append(S(d, d, "body", 5, True, 0.5, Draw.rect(z0, z1, Y0, Y1), region="fronts", material="oak-decor-mdf", pen=pen))
    for n, (y0, y1) in {1: (0.082, 0.261), 2: (0.261, 0.44)}.items():
        out.append(S(f"drawer-{n}", f"drawer-{n}", "body", 5, True, 0.5, Draw.rect(-LEAF, LEAF, y0 + BL, y1 + BL), region="fronts", material="oak-decor-mdf", pen=pen))
    out.append(S("flap", "flap", "body", 5, True, 0.5, Draw.poly([(0, 0), (0.42, 0), (0.42, p["t"]), (0, p["t"])]), region="fronts", material="oak-decor-mdf", pen=pen, solid=flap_solid(-LEAF, LEAF)))
    for k, zc in enumerate((-0.9, 0.0, 0.9)):
        out.append(S(f"blanket-{k + 1}", "blankets", "body", 6.5, True, 0.35, Draw.rect(zc - HW, zc + HW, 0, BLANKET_DRAWN[scale]), region="blankets", material="moving-blanket"))
    return out
#endregion


#region 🧾 Asset data
SRC = "authored-estimate"
MATERIALS = [
    {"id": "oak-decor-chipboard", "title": T("Chipboard P2, 19 mm, oak-decor paper foil, aged", "Spanplatte P2, 19 mm, Eichendekor-Papierfolie, gealtert"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": 650, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 1.6e9, "strengthPa": 11e6, "source": "literature", "reference": "en-312-2010"},
    {"id": "oak-decor-mdf", "title": T("MDF, 19 mm, routed and foiled in oak decor (fronts, cornice)", "MDF, 19 mm, gefräst und eichendekorfoliert (Fronten, Kranz)"), "emissivity": 0.9, "solarAbsorptivity": 0.68, "densityKgM3": 750, "conductivityWmK": 0.13, "specificHeatJKgK": 1700, "elasticModulusPa": 2.5e9, "strengthPa": 20e6, "source": "literature", "reference": "en-622-5-2009"},
    {"id": "hardboard", "title": T("Hardboard HB, 3 and 4 mm (backs, drawer bottoms)", "Hartfaserplatte HB, 3 und 4 mm (Rückwände, Schubkastenböden)"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 900, "conductivityWmK": 0.18, "specificHeatJKgK": 1700, "elasticModulusPa": 3.0e9, "strengthPa": 30e6, "source": "literature", "reference": "en-622-2-2004"},
    {"id": "float-glass", "title": T("Float glass, 4 and 6 mm (doors, shelves)", "Floatglas, 4 und 6 mm (Türen, Böden)"), "emissivity": 0.84, "solarAbsorptivity": 0.08, "densityKgM3": 2500, "conductivityWmK": 1.0, "specificHeatJKgK": 750, "elasticModulusPa": 70e9, "strengthPa": 45e6, "source": "literature", "reference": "en-iso-10456-2007"},
    {"id": "mirror-glass", "title": T("Mirror glass, 4 mm (bar back)", "Spiegelglas, 4 mm (Barrückwand)"), "emissivity": 0.84, "solarAbsorptivity": 0.15, "densityKgM3": 2500, "conductivityWmK": 1.0, "specificHeatJKgK": 750, "source": "literature", "reference": "en-iso-10456-2007"},
    {"id": "beech-knob", "title": T("Turned beech knobs, stained", "Gedrechselte Buchenknöpfe, gebeizt"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 700, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "source": "literature", "reference": "en-iso-10456-2007"},
    {"id": "chrome-steel", "title": T("Chromed steel handle (replacement)", "Verchromter Stahlgriff (Ersatz)"), "emissivity": 0.08, "solarAbsorptivity": 0.35, "densityKgM3": 7800, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "en-iso-10456-2007"},
    {"id": "zinc-plated-steel", "title": T("Zinc-plated steel fittings (flap stays, runners, hinges)", "Verzinkte Stahlbeschläge (Klappenhalter, Laufschienen, Bänder)"), "emissivity": 0.25, "solarAbsorptivity": 0.5, "densityKgM3": 7800, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "en-iso-10456-2007"},
    {"id": "moving-blanket", "title": T("Moving blanket, cotton-polyester nonwoven, folded to 12 mm", "Möbeldecke, Baumwoll-Polyester-Vlies, auf 12 mm gefaltet"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 345, "conductivityWmK": 0.045, "specificHeatJKgK": 1350, "source": SRC, "reference": "moving-blanket"},
    {"id": "lamp-glass", "title": T("Soda-lime glass tube of the line lamp", "Kalk-Natron-Glasrohr der Linienlampe"), "emissivity": 0.9, "solarAbsorptivity": 0.1, "densityKgM3": 2500, "conductivityWmK": 1.0, "specificHeatJKgK": 750, "source": "literature", "reference": "iec-60061-1"},
]
REFERENCES = [
    ("en-312-2010", "EN 312:2010 Particleboards - Specifications (type P2, boards for interior fitments including furniture in dry conditions: 13-20 mm bending strength 11 N/mm2, modulus of elasticity 1600 N/mm2; 600-680 kg/m3)."),
    ("en-622-5-2009", "EN 622-5:2009 Fibreboards - Specifications - Part 5: Requirements for dry process boards (MDF): 19 mm boards 700-780 kg/m3, bending strength 20 N/mm2, modulus of elasticity 2200-2500 N/mm2."),
    ("en-622-2-2004", "EN 622-2:2004 Fibreboards - Specifications - Part 2: Requirements for hardboards (HB, 3-4 mm, about 900 kg/m3)."),
    ("en-iso-10456-2007", "EN ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity, density and specific heat of wood-based panels, fibreboard, glass, steel and timber)."),
    ("en-16122-2012", "EN 16122:2012 Domestic and non-domestic storage furniture - Test methods for the determination of strength, durability and stability (shelf, drop-front flap and drawer tests; shelf test load 1.5 kg/dm2 for domestic storage)."),
    ("en-14749-2016", "EN 14749:2016 Furniture - Domestic and kitchen storage units and worktops - Safety requirements and test methods (stability of tall units with doors and flaps open, wall fixing of units taller than 600 mm)."),
    ("en-1991-1-1-2002", "EN 1991-1-1:2002 Eurocode 1 - Actions on structures - General actions: densities, self-weight and imposed loads of buildings (category A floors, Annex A densities of stored materials)."),
    ("iec-60061-1", "IEC 60061-1 Lamp caps and holders (cap S14s of the tubular line lamp) with catalogue data of 300 mm line lamps 35 W, tube diameter 26-30 mm."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties of wood, glass and metals, free convection on plates)."),
    ("wall-unit-catalogues-1980s", "Living-room wall units ('Wohnwand', 'Schrankwand') of German mail-order and furniture-store catalogues 1980-1989: about 270 x 197-200 x 48-55 cm, three 90 cm elements in oak-look decor, glass display cabinet, bar compartment with mirror back and light, drawers, TV niche, profiled cornice, 200-260 kg; catalogue-typical values of the type, no single product."),
    ("fittings-catalogues", "Furniture fittings catalogues (drop-front flap stays with brake, 35 mm concealed cup hinges, roller drawer runners 12.7 mm, flap hinges for drop fronts, System 32 hole rows): catalogue-typical dimensions."),
    ("moving-blanket", "Removal-trade moving blankets (Möbeldecken) of about 180 x 200 cm and 500-600 g/m2 cotton-polyester nonwoven, folded under carcass furniture in the depot; authored-estimate."),
    ("din-68871", "DIN 68871:2002 Furniture - Terms (Korpus, Sockel, Kranz, Klappe, Vitrine, Barfach): the furniture vocabulary used for the parts."),
]
TAGS = ["used", "household", "second-hand", "living", "wall-unit", "storage", "chipboard", "oak-decor", "display-cabinet", "bar-compartment", "1980s", "interior"]


def bone(bid, parent, title, x, y, angle, length, joint, z=None, section=None):
    b = {"id": bid, "parent": parent, "title": title, "x": R(x), "y": R(y), "angleDeg": R(angle), "lengthM": length, "joint": joint}
    if z is not None:
        b["z"] = R(z)
    if bid in BONE_MASS:
        m = BONE_MASS[bid]
        b["mass"] = {"kg": m["kg"], "comX": m["comX"], "comY": m["comY"], "distribution": "point", "source": SRC, "reference": "en-312-2010"}
    if section:
        b["section"] = section
    return b


STAY_LIMIT = math.ceil(stay_offset(-90) + 1)
DOOR_TITLES = {
    "linen-door-p": T("Linen cabinet door, left leaf seen from the front", "Wäscheschranktür, vorn links gesehener Flügel"),
    "linen-door-m": T("Linen cabinet door, right leaf seen from the front", "Wäscheschranktür, vorn rechts gesehener Flügel"),
    "centre-door-p": T("Glass display door, left leaf", "Glastür der Vitrine, linker Flügel"),
    "centre-door-m": T("Glass display door, right leaf", "Glastür der Vitrine, rechter Flügel"),
    "tv-door-lp": T("TV body lower door, left leaf", "Untere Tür des TV-Teils, linker Flügel"),
    "tv-door-lm": T("TV body lower door, right leaf", "Untere Tür des TV-Teils, rechter Flügel"),
    "tv-door-up": T("TV body upper door, left leaf", "Obere Tür des TV-Teils, linker Flügel"),
    "tv-door-um": T("TV body upper door, right leaf (knob lost)", "Obere Tür des TV-Teils, rechter Flügel (Knopf verloren)"),
}
DOOR_SPREAD = {"type": "revolute", "minDeg": 0, "maxDeg": 0}
BONES = [
    bone("ground", None, T("Depot floor", "Lagerboden"), 0, 0, 0, 0, {"type": "fixed"}),
    bone("blankets", "ground", T("Three folded moving blankets", "Drei gefaltete Möbeldecken"), -0.03, 0, 0, 0.56, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.9}),
    bone("linen-body", "ground", T("Linen cabinet body (seen from the front on the right)", "Wäscheschrankteil (von vorn rechts)"), 0, BL, 0, D, {"type": "prismatic", "axisDeg": 90, "minM": -BL, "maxM": 0}, z=ZC["linen"]),
    bone("centre-slide", "ground", T("Centre body carried forward", "Mittelteil nach vorn getragen"), 0, BL, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.6}),
    bone("centre-lift", "centre-slide", T("Centre body lifted", "Mittelteil angehoben"), 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -BL, "maxM": 0.35}),
    bone("centre-body", "centre-lift", T("Centre body: drawers, bar, display cabinet (tilts about its rear bottom edge)", "Mittelteil: Schubkästen, Bar, Vitrine (kippt um die hintere Unterkante)"), 0, 0, 0, D, {"type": "revolute", "minDeg": 0, "maxDeg": 35}),
    bone("tv-body", "ground", T("TV body (seen from the front on the left)", "TV-Teil (von vorn links)"), 0, BL, 0, D, {"type": "prismatic", "axisDeg": 90, "minM": -BL, "maxM": 0}, z=ZC["tv"]),
    bone("flap", "centre-body", T("Bar flap hinged at its outer bottom edge", "Barklappe, an der äußeren Unterkante angeschlagen"), FLAP_PIVOT[0], FLAP_PIVOT[1], 90, 0.417, {"type": "revolute", "minDeg": -90, "maxDeg": 0}, section={"areaM2": R(0.897 * 0.019), "secondMomentM4": 5.127e-07, "extremeFibreM": 0.0095, "material": "oak-decor-mdf", "title": T("Flap 897 x 19 mm MDF", "Klappe 897 x 19 mm MDF")}),
]
for s, z in (("p", STAY_Z), ("m", -STAY_Z)):
    BONES.append(bone(f"stay-{s}", "flap", T(f"Flap stay {'left' if s == 'p' else 'right'} (rod sliding in its brake guide)", f"Klappenhalter {'links' if s == 'p' else 'rechts'} (Stange, gleitet in der Bremsführung)"), STAY_Q[1] - FLAP_PIVOT[1], FLAP_PIVOT[0] - STAY_Q[0], (PSI0 - 90 + 180) % 360 - 180, STAY_L, {"type": "revolute", "minDeg": 0, "maxDeg": STAY_LIMIT}, z=z, section={"areaM2": 2.827e-05, "secondMomentM4": 6.362e-11, "extremeFibreM": 0.003, "material": "zinc-plated-steel", "title": T("Steel rod 6 mm", "Stahlstange 6 mm")}))
for n, y in ((1, 0.10), (2, 0.28)):
    BONES.append(bone(f"drawer-{n}", "centre-body", T(f"Drawer {n} on roller runners", f"Schubkasten {n} auf Rollenauszügen"), 0.06, y, 0, 0.44, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.36}))
for d, (b, y0, y1, side) in DOORS.items():
    BONES.append(bone(d, f"{b}-body", DOOR_TITLES[d], X_F0, y0, 0, 0.019, DOOR_SPREAD, z=side * LEAF))
assert abs(sum(b["mass"]["kg"] for b in BONES if "mass" in b) - TOTAL_KG) < 1e-6

ANCHORS = []
for b in BODIES:
    ANCHORS.append({"id": f"plinth-{b}", "bone": f"{b}-body", "x": 0.24, "y": 0.0, "kind": "contact", "title": T(f"{ {'linen': 'Linen cabinet', 'centre': 'Centre body', 'tv': 'TV body'}[b]} standing on its side panels and plinth", f"{ {'linen': 'Wäscheschrankteil', 'centre': 'Mittelteil', 'tv': 'TV-Teil'}[b]} steht auf Seiten und Sockel"), "support": "contact", "normalDeg": 90, "footprintLengthM": D, "footprintWidthM": 2 * HW, "frictionCoefficient": 0.4})
ANCHORS.append({"id": "grip-low", "bone": "centre-body", "x": 0.46, "y": 0.0, "kind": "support", "title": T("Lower mover's grip under the front bottom edge", "Griff des unteren Trägers unter der vorderen Unterkante"), "support": "pin"})
ANCHORS.append({"id": "grip-high", "bone": "centre-body", "x": 0.03, "y": Y_T1, "kind": "support", "title": T("Upper mover's grip at the rear top edge", "Griff des oberen Trägers an der hinteren Oberkante"), "support": "roller", "normalDeg": 90})
#endregion


#region 🌡️ Thermal model
AREA = {"carcass": 7.15, "cornice": 0.603, "fronts": 4.46, "glazing": 0.673, "back-panel": 5.4, "blankets": 3.01, "bar-lamp": 0.0233}


def layer(lid, title, kind, mat, thick, k, rho, cp, src="literature", refid="en-iso-10456-2007"):
    return {"id": lid, "title": title, "kind": kind, "material": mat, "thicknessM": thick, "resistanceM2KW": R(thick / k), "heatCapacityJM2K": round(rho * cp * thick), "source": src, "reference": refid}


REGIONS = [
    {"id": "carcass", "title": T("Carcass: side panels, top, bottom and fixed boards, plinths, open TV niche", "Korpus: Seiten, Deckel, Böden und feste Fachböden, Sockel, offene TV-Nische"), "kind": "surface", "model": "passive", "material": "oak-decor-chipboard", "areaM2": AREA["carcass"], "characteristicLengthM": 0.48, "geometry": "vertical-plate", "skyViewFactor": 0.3, "layers": [layer("carcass-board", T("Chipboard P2 19 mm with aged decor foil", "Spanplatte P2 19 mm mit gealterter Dekorfolie"), "timber", "oak-decor-chipboard", 0.019, 0.14, 650, 1700, refid="en-312-2010")], "heatCapacityJM2K": round(650 * 1700 * 0.019), "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": SRC, "reference": "en-iso-10456-2007"},
    {"id": "cornice", "title": T("Profiled cornice", "Profilierter Kranz"), "kind": "surface", "model": "passive", "material": "oak-decor-mdf", "areaM2": AREA["cornice"], "characteristicLengthM": 0.06, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [layer("cornice-mdf", T("MDF profile 60 x 70 mm, effective depth 19 mm", "MDF-Profil 60 x 70 mm, wirksame Tiefe 19 mm"), "timber", "oak-decor-mdf", 0.019, 0.13, 750, 1700, refid="en-622-5-2009")], "heatCapacityJM2K": round(750 * 1700 * 0.019), "source": SRC, "reference": "en-iso-10456-2007"},
    {"id": "fronts", "title": T("Fronts: doors, drawer fronts, bar flap, glass door frames", "Fronten: Türen, Schubkastenblenden, Barklappe, Glastürrahmen"), "kind": "surface", "model": "passive", "material": "oak-decor-mdf", "areaM2": AREA["fronts"], "characteristicLengthM": 0.45, "geometry": "vertical-plate", "skyViewFactor": 0.4, "layers": [layer("front-mdf", T("Routed MDF 19 mm with decor foil", "Gefräste MDF 19 mm mit Dekorfolie"), "timber", "oak-decor-mdf", 0.019, 0.13, 750, 1700, refid="en-622-5-2009")], "heatCapacityJM2K": round(750 * 1700 * 0.019), "source": SRC, "reference": "en-iso-10456-2007"},
    {"id": "glazing", "title": T("Glass panes of the display doors", "Glasscheiben der Vitrinentüren"), "kind": "glazing", "model": "passive", "material": "float-glass", "areaM2": AREA["glazing"], "characteristicLengthM": 0.34, "geometry": "vertical-plate", "skyViewFactor": 0.4, "layers": [layer("pane", T("Float glass 4 mm", "Floatglas 4 mm"), "glass", "float-glass", 0.004, 1.0, 2500, 750)], "heatCapacityJM2K": round(2500 * 750 * 0.004), "source": SRC, "reference": "en-iso-10456-2007"},
    {"id": "back-panel", "title": T("Hardboard backs", "Hartfaserrückwände"), "kind": "surface", "model": "passive", "material": "hardboard", "areaM2": AREA["back-panel"], "characteristicLengthM": 0.9, "geometry": "vertical-plate", "skyViewFactor": 0.3, "layers": [layer("back-hardboard", T("Hardboard 3 mm", "Hartfaser 3 mm"), "timber", "hardboard", 0.003, 0.18, 900, 1700, refid="en-622-2-2004")], "heatCapacityJM2K": round(900 * 1700 * 0.003), "source": SRC, "reference": "en-iso-10456-2007"},
    {"id": "blankets", "title": T("Folded moving blankets", "Gefaltete Möbeldecken"), "kind": "surface", "model": "passive", "material": "moving-blanket", "areaM2": AREA["blankets"], "characteristicLengthM": 0.56, "geometry": "horizontal-plate-up", "skyViewFactor": 0.2, "layers": [layer("blanket-felt", T("Folded nonwoven 12 mm", "Gefaltetes Vlies 12 mm"), "clothing", "moving-blanket", 0.012, 0.045, 345, 1350, src=SRC, refid="moving-blanket")], "heatCapacityJM2K": round(345 * 1350 * 0.012), "source": SRC, "reference": "moving-blanket"},
    {"id": "bar-lamp", "title": T("Line lamp of the bar compartment, 35 W", "Linienlampe des Barfachs, 35 W"), "kind": "lamp", "model": "operating", "material": "lamp-glass", "areaM2": AREA["bar-lamp"], "characteristicLengthM": 0.026, "geometry": "cylinder", "skyViewFactor": 0.0, "layers": [layer("lamp-tube", T("Soda-lime glass tube 0.8 mm", "Kalk-Natron-Glasrohr 0,8 mm"), "glass", "lamp-glass", 0.0008, 1.0, 2500, 750, refid="iec-60061-1")], "heatCapacityJM2K": round(2500 * 750 * 0.0008), "operating": {"key": "bar-light", "heatW": 35}, "ratedPowerW": 35, "ratedPowerBasis": "electrical-input", "hidden": T("Inside the closed bar compartment under its top board; it is seen only while the flap is open.", "Im geschlossenen Barfach unter dessen Deckboden; sichtbar nur bei offener Klappe."), "source": SRC, "reference": "iec-60061-1"},
]
ENVIRONMENT = {"airTempC": 16, "meanRadiantTempC": 15, "relativeHumidity": 0.55, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325}


EXPOSED = {"carcass": 0.75, "cornice": 0.92, "fronts": 0.94, "glazing": 0.97, "back-panel": 0.98, "blankets": 1.0, "bar-lamp": 1.0}


def binding(lamp_on=0, in_use=False):
    reg = {r["id"]: {"exposedFraction": EXPOSED[r["id"]], "sunlitFraction": 0.0} for r in REGIONS}
    reg["blankets"] = {"exposedFraction": 1.0, "sunlitFraction": 0.0, "contact": {"fraction": 0.5 if in_use else 0.47, "tempC": 16}}
    if in_use:
        reg["back-panel"] = {"exposedFraction": 0.3, "sunlitFraction": 0.0}
    return {"operating": {"bar-light": lamp_on}, "regions": reg, "emitters": {}}
#endregion


#region 🧍 Poses, loads and clips
def load(lid, en, de, bone_id, x, y, kg, ref_id="en-16122-2012"):
    return {"id": lid, "kind": "imposed", "title": T(en, de), "bone": bone_id, "x": R(x), "y": R(y), "fxN": 0.0, "fyN": R(-kg * 9.80665), "source": SRC, "reference": ref_id}


CONTENTS = [load(f"linen-shelf-{k + 1}", f"Linen and tableware on shelf {k + 1} (12 kg)", f"Wäsche und Geschirr auf Boden {k + 1} (12 kg)", "linen-body", 0.25, 0.479 + 0.36 * k, 12) for k in range(4)]
CONTENTS += [
    load("linen-floor", "Linen on the cabinet floor (10 kg)", "Wäsche auf dem Schrankboden (10 kg)", "linen-body", 0.25, Y_B1, 10),
    load("drawer-1-contents", "Cutlery and table linen in drawer 1 (6 kg)", "Besteck und Tischwäsche in Schubkasten 1 (6 kg)", "drawer-1", 0.21, 0.004, 6),
    load("drawer-2-contents", "Candles, games and papers in drawer 2 (6 kg)", "Kerzen, Spiele und Papiere in Schubkasten 2 (6 kg)", "drawer-2", 0.21, 0.004, 6),
    load("bar-bottles", "Bottles in the bar compartment (8 kg)", "Flaschen im Barfach (8 kg)", "centre-body", 0.25, 0.459, 8),
    load("display-floor", "Glassware on the display floor (4 kg)", "Gläser auf dem Vitrinenboden (4 kg)", "centre-body", 0.25, 0.879, 4),
    load("display-shelf-1", "Glassware on glass shelf 1 (3 kg)", "Gläser auf Glasboden 1 (3 kg)", "centre-body", 0.23, 1.226, 3),
    load("display-shelf-2", "Glassware on glass shelf 2 (3 kg)", "Gläser auf Glasboden 2 (3 kg)", "centre-body", 0.23, 1.576, 3),
    load("tv-set", "CRT television of 63 cm in the niche (25 kg)", "Röhrenfernseher 63 cm in der Nische (25 kg)", "tv-body", 0.27, 0.799, 25, "wall-unit-catalogues-1980s"),
    load("tv-shelf-lower", "Video recorder and tapes on the lower shelf (10 kg)", "Videorekorder und Kassetten auf dem unteren Boden (10 kg)", "tv-body", 0.25, 0.449, 10),
    load("tv-shelf-upper", "Photo albums on the upper shelf (6 kg)", "Fotoalben auf dem oberen Boden (6 kg)", "tv-body", 0.25, 1.659, 6),
    load("tv-floor", "Records on the lower cabinet floor (8 kg)", "Schallplatten auf dem unteren Schrankboden (8 kg)", "tv-body", 0.25, Y_B1, 8),
]
FLAP_LOAD = load("glasses-on-flap", "Bottle and glasses on the open flap (5 kg)", "Flasche und Gläser auf der offenen Klappe (5 kg)", "flap", 0.21, 0.019, 5)
IN_USE_BONES = {"linen-body": {"dy": -BL}, "tv-body": {"dy": -BL}, "centre-lift": {"dy": -BL}, "blankets": {"dx": 0.62}}
CARRIED_BONES = {"centre-slide": {"dx": 1.45}, "centre-lift": {"dy": 0.20}, "centre-body": {"angleDeg": 25}}
STAND = ["plinth-linen", "plinth-centre", "plinth-tv"]
POSES = [
    {"id": "stored", "title": T("Stored in the depot", "Im Lager abgestellt"), "description": T("Taken apart at the flat clearance: the three bodies stand unbolted side by side, each on a moving blanket folded to 12 mm; drawers and flap closed, shelves inside.", "Bei der Haushaltsauflösung zerlegt: die drei Teile stehen unverschraubt nebeneinander, jedes auf einer auf 12 mm gefalteten Möbeldecke; Schubkästen und Klappe zu, Böden drin."), "bones": {}, "ground": "authored", "contacts": STAND, "thermal": binding(), "loads": []},
    {"id": "carried", "title": T("Centre body carried out", "Mittelteil wird hinausgetragen"), "description": T("Two movers carry the 99 kg centre body 1.45 m out of the row, tilted back by 25 degrees and lifted 0.20 m: one grips under the front bottom edge, the other the rear top edge; the blankets stay on the floor.", "Zwei Träger tragen das 99 kg schwere Mittelteil 1,45 m aus der Reihe, um 25 Grad nach hinten gekippt und 0,20 m angehoben: einer greift unter die vordere Unterkante, der andere an die hintere Oberkante; die Decken bleiben am Boden."), "bones": CARRIED_BONES, "ground": "authored", "contacts": ["plinth-linen", "plinth-tv", "grip-low", "grip-high"], "thermal": binding(), "loads": []},
    {"id": "in-use", "title": T("In use in a living room", "In Gebrauch im Wohnzimmer"), "description": T("Set up again in a living room: the bodies stand on the floor, the blankets pulled out lie in front, the shelves hold linen, tableware, glassware, a CRT television, a video recorder and records.", "Im Wohnzimmer wieder aufgestellt: die Teile stehen auf dem Boden, die herausgezogenen Decken liegen davor, die Böden tragen Wäsche, Geschirr, Gläser, einen Röhrenfernseher, einen Videorekorder und Schallplatten."), "bones": IN_USE_BONES, "ground": "authored", "contacts": STAND, "thermal": binding(in_use=True), "loads": CONTENTS},
    {"id": "bar-open", "title": T("Bar flap open, light on", "Barklappe offen, Licht an"), "description": T("The flap lies horizontal on its two stays, the line lamp lights the mirrored bar, a bottle and glasses stand on the flap.", "Die Klappe liegt waagerecht auf ihren zwei Haltern, die Linienlampe beleuchtet die verspiegelte Bar, Flasche und Gläser stehen auf der Klappe."), "bones": {**IN_USE_BONES, **flap_bones(-90)}, "ground": "authored", "contacts": STAND, "thermal": binding(1, True), "loads": CONTENTS + [FLAP_LOAD]},
    {"id": "drawer-out", "title": T("Drawer pulled out", "Schubkasten herausgezogen"), "description": T("The upper drawer, the one with the replaced chrome handle, is pulled out 0.30 m on its roller runners.", "Der obere Schubkasten, der mit dem ersetzten Chromgriff, ist 0,30 m auf seinen Rollenauszügen herausgezogen."), "bones": {**IN_USE_BONES, "drawer-2": {"dx": 0.30}}, "ground": "authored", "contacts": STAND, "thermal": binding(in_use=True), "loads": CONTENTS},
]


def flap_keys(times, thetas, pose="in-use"):
    return [{"t": t, "pose": pose, "ease": "smooth", "bones": {**IN_USE_BONES, **flap_bones(th)}} for t, th in zip(times, thetas)]


OPEN = [-15, -30, -45, -60, -75]
CLIPS = [
    {"id": "carry", "title": T("Carry the centre body out", "Mittelteil hinaustragen"), "description": T("The movers tilt the centre body back on its rear bottom edge, lift it off its blanket and carry it 1.45 m out of the row; the blanket stays behind.", "Die Träger kippen das Mittelteil auf seiner hinteren Unterkante nach hinten, heben es von seiner Decke und tragen es 1,45 m aus der Reihe; die Decke bleibt liegen."), "durationS": 4.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.2, "pose": "stored", "ease": "smooth", "bones": {"centre-body": {"angleDeg": 25}}}, {"t": 2.0, "pose": "stored", "ease": "smooth", "bones": {"centre-body": {"angleDeg": 25}, "centre-lift": {"dy": 0.20}}}, {"t": 4.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": "plinth-linen", "fromS": 0, "toS": 4.0}, {"anchor": "plinth-tv", "fromS": 0, "toS": 4.0}, {"anchor": "plinth-centre", "fromS": 0, "toS": 0.6}, {"anchor": "grip-high", "fromS": 0.2, "toS": 4.0}, {"anchor": "grip-low", "fromS": 0.6, "toS": 4.0}],
     "phases": [{"id": "tilt", "fromS": 0, "toS": 1.2, "title": T("Tilting back", "Nach hinten kippen")}, {"id": "lift", "fromS": 1.2, "toS": 2.0, "title": T("Lifting off the blanket", "Von der Decke heben")}, {"id": "carry-out", "fromS": 2.0, "toS": 4.0, "title": T("Carrying out", "Hinaustragen")}],
     "events": [{"t": 0.2, "kind": "contact-on", "title": T("Upper mover grips the top edge", "Oberer Träger greift die Oberkante"), "target": "grip-high"}, {"t": 0.6, "kind": "contact-off", "title": T("Front of the plinth leaves the blanket", "Sockelvorderkante verlässt die Decke"), "target": "plinth-centre"}, {"t": 2.0, "kind": "motion", "title": T("Walking out", "Hinausgehen")}],
     "tracks": []},
    {"id": "open-bar-flap", "title": T("Open the bar flap", "Barklappe öffnen"), "description": T("The flap is pulled down to horizontal on its braked stays; the line lamp switches on as the flap leaves its contact switch, stays on while the flap is open and goes off as it closes.", "Die Klappe wird an ihren gebremsten Haltern bis waagerecht heruntergezogen; die Linienlampe schaltet ein, sobald die Klappe den Kontaktschalter freigibt, bleibt bei offener Klappe an und geht beim Schließen aus."), "durationS": 5.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}] + flap_keys([0.55, 0.8, 1.05, 1.3, 1.55], OPEN) + [{"t": 1.8, "pose": "bar-open", "ease": "smooth"}, {"t": 3.0, "pose": "bar-open", "ease": "smooth"}] + flap_keys([3.25, 3.5, 3.75, 4.0, 4.25], list(reversed(OPEN))) + [{"t": 4.5, "pose": "in-use", "ease": "smooth"}, {"t": 5.0, "pose": "in-use", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 5.0} for a in STAND],
     "phases": [{"id": "opening", "fromS": 0, "toS": 1.8, "title": T("Opening", "Öffnen")}, {"id": "open", "fromS": 1.8, "toS": 3.0, "title": T("Open", "Offen")}, {"id": "closing", "fromS": 3.0, "toS": 4.5, "title": T("Closing", "Schließen")}],
     "events": [{"t": 0.55, "kind": "operating", "title": T("Bar light on", "Barlicht an"), "target": "bar-light"}, {"t": 4.4, "kind": "operating", "title": T("Bar light off", "Barlicht aus"), "target": "bar-light"}],
     "tracks": [{"target": "operating:bar-light", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 0.55, "v": 1}, {"t": 4.4, "v": 0}, {"t": 5.0, "v": 0}]}]},
    {"id": "pull-drawer", "title": T("Pull the drawer", "Schubkasten aufziehen"), "description": T("The upper drawer is pulled out 0.30 m on its roller runners and pushed back.", "Der obere Schubkasten wird 0,30 m auf seinen Rollenauszügen herausgezogen und zurückgeschoben."), "durationS": 3.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}, {"t": 1.0, "pose": "drawer-out", "ease": "smooth"}, {"t": 2.0, "pose": "drawer-out", "ease": "smooth"}, {"t": 3.0, "pose": "in-use", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 3.0} for a in STAND],
     "phases": [{"id": "pull", "fromS": 0, "toS": 1.0, "title": T("Pulling", "Aufziehen")}, {"id": "push", "fromS": 2.0, "toS": 3.0, "title": T("Pushing back", "Zuschieben")}],
     "events": [{"t": 1.0, "kind": "motion", "title": T("Drawer at its stop", "Schubkasten am Anschlag")}],
     "tracks": []},
]
#endregion


#region 🏗️ Structure and hand checks
CENTRE_BONES = ("centre-body", "flap", "stay-p", "stay-m", "drawer-1", "drawer-2", "centre-door-p", "centre-door-m")
_cm = sum(BONE_MASS[b]["kg"] for b in CENTRE_BONES)
_cx = sum(BONE_MASS[b]["kg"] * com_world(b)[0] for b in CENTRE_BONES) / _cm
_cy = sum(BONE_MASS[b]["kg"] * (com_world(b)[1] - BL) for b in CENTRE_BONES) / _cm


def _tilt(x, y, deg=25):
    t = math.radians(deg)
    return (x * math.cos(t) - y * math.sin(t), x * math.sin(t) + y * math.cos(t))


_low, _high, _com = _tilt(0.46, 0.0), _tilt(0.03, Y_T1), _tilt(_cx, _cy)
W_CENTRE = _cm * 9.80665
R_HIGH = W_CENTRE * (_com[0] - _low[0]) / (_high[0] - _low[0])
R_LOW = W_CENTRE - R_HIGH
TIP_RESTORE = W_CENTRE * (D - _cx)
TIP_FLAP = 20 * 9.80665 * (X_F1 + 0.417 - D)
STRUCTURE = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["plinth-linen", "plinth-tv"], "clinical": False, "assumptions": [
    T(f"Each body is a rigid carcass of 19 mm chipboard P2 (EN 312: 11 N/mm2, 1600 N/mm2) braced by its 3 mm hardboard back; it stands on the floor edges of its two side panels and its plinth (footprint 0.48 x 0.90 m, friction 0.4 on the blanket). Masses are section x density of the 1:50 parts: linen body {BODY_KG['linen']} kg, centre body {BODY_KG['centre']} kg, TV body {BODY_KG['tv']} kg, blankets {BONE_MASS['blankets']['kg']} kg, together {TOTAL_KG} kg.",
      f"Jedes Teil ist ein starrer Korpus aus 19-mm-Spanplatte P2 (EN 312: 11 N/mm2, 1600 N/mm2), ausgesteift durch die 3-mm-Hartfaserrückwand; es steht auf den Bodenkanten seiner zwei Seiten und seinem Sockel (Aufstandsfläche 0,48 x 0,90 m, Reibung 0,4 auf der Decke). Massen sind Querschnitt x Dichte der 1:50-Teile: Wäscheschrankteil {BODY_KG['linen']} kg, Mittelteil {BODY_KG['centre']} kg, TV-Teil {BODY_KG['tv']} kg, Decken {BONE_MASS['blankets']['kg']} kg, zusammen {TOTAL_KG} kg."),
    T(f"Carried: the solver treats the asset as one rigid body, so the grips share one equilibrium with the floor contacts of the two standing bodies. By hand, with the centre body tilted 25 degrees: weight {W_CENTRE:.0f} N at x = {_com[0]:.3f} m from the pivot, lower grip at {_low[0]:.3f} m carries {R_LOW:.0f} N, upper grip at {_high[0]:.3f} m carries {R_HIGH:.0f} N (about {R_LOW / 9.81:.0f} and {R_HIGH / 9.81:.0f} kg per mover).",
      f"Getragen: der Löser behandelt das Asset als einen starren Körper, die Griffe teilen sich also ein Gleichgewicht mit den Bodenkontakten der zwei stehenden Teile. Von Hand, mit dem um 25 Grad gekippten Mittelteil: Gewicht {W_CENTRE:.0f} N bei x = {_com[0]:.3f} m vom Drehpunkt, der untere Griff bei {_low[0]:.3f} m trägt {R_LOW:.0f} N, der obere bei {_high[0]:.3f} m {R_HIGH:.0f} N (je Träger etwa {R_LOW / 9.81:.0f} und {R_HIGH / 9.81:.0f} kg)."),
    T(f"Stability after EN 14749 for the unbolted centre body with the flap open: 20 kg on the flap edge tilt it about the front edge of its side panels with {TIP_FLAP:.0f} N m against a restoring {TIP_RESTORE:.0f} N m of its own weight; bolted to its neighbours and fixed to the wall, as EN 14749 asks for units above 600 mm, it is safe. The open flap is a cantilever on its two hinges held by two 6 mm steel stays.",
      f"Standsicherheit nach EN 14749 für das unverschraubte Mittelteil bei offener Klappe: 20 kg an der Klappenkante kippen es um die Vorderkante seiner Seiten mit {TIP_FLAP:.0f} N m gegen rückstellende {TIP_RESTORE:.0f} N m aus Eigengewicht; mit den Nachbarteilen verschraubt und an der Wand befestigt, wie EN 14749 es für Möbel über 600 mm fordert, ist es sicher. Die offene Klappe kragt an ihren zwei Bändern aus und wird von zwei 6-mm-Stahlhaltern gehalten."),
    T("In use the contents are imposed point loads at the shelf centres: 12 kg per linen shelf, 25 kg for the CRT television in the niche, 3 to 8 kg elsewhere; against the EN 16122 shelf test level of 1.5 kg/dm2 (57 kg on a 0.44 x 0.86 m shelf) they are 20 to 45 percent of it. The long-term creep sag of the shelves (below 5 mm) is not modelled.",
      "In Gebrauch ist der Inhalt als Punktlast in Bodenmitte aufgebracht: 12 kg je Wäscheboden, 25 kg für den Röhrenfernseher in der Nische, sonst 3 bis 8 kg; gegenüber der Fachbodenprüflast der EN 16122 von 1,5 kg/dm2 (57 kg auf einem Boden 0,44 x 0,86 m) sind das 20 bis 45 Prozent. Die Langzeitkriechdurchbiegung der Böden (unter 5 mm) ist nicht modelliert."),
]}
THERMAL_ASSUMPTIONS = [
    T("Reference case: unheated reuse depot hall, 16 degC air, 15 degC surroundings, 55 percent humidity, still air, no sun. Passive chipboard, MDF and hardboard after EN ISO 10456; heat capacity of one board depth.", "Referenzfall: unbeheizte Halle eines Wiederverwendungslagers, 16 °C Luft, 15 °C Umgebung, 55 Prozent Feuchte, ruhige Luft, keine Sonne. Passive Span-, MDF- und Hartfaserplatte nach EN ISO 10456; Wärmekapazität einer Plattenstärke."),
    T("Aged surfaces: the oak-decor paper foil of the carcass is sun-bleached on the right side; its solar absorptance is taken as 0.62 against about 0.72 of a new dark oak decor (authored-estimate), emissivity 0.9 is unchanged.", "Gealterte Oberflächen: die Eichendekor-Papierfolie des Korpus ist an der rechten Seite sonnengebleicht; ihr solarer Absorptionsgrad ist mit 0,62 statt etwa 0,72 eines neuen dunklen Eichendekors angesetzt (Schätzwert), der Emissionsgrad 0,9 bleibt."),
    T("The only powered element is the 35 W line lamp of the bar (S14s, 300 mm), switched by the flap; its 35 W end as heat inside the bar compartment. Furniture parts are passive, 0 W, stated on the carcass.", "Einziges elektrisches Element ist die 35-W-Linienlampe der Bar (S14s, 300 mm), von der Klappe geschaltet; ihre 35 W werden im Barfach zu Wärme. Die Möbelteile sind passiv, 0 W, am Korpus angegeben."),
    T("Thermal model: the 3 mm running gaps between the fronts are closed, the closed cabinets are not ventilated; the open TV niche and the open plinth spaces behind the recessed plinth boards exchange heat with the room.", "Thermisches Modell: die 3-mm-Fugen zwischen den Fronten sind geschlossen, die geschlossenen Fächer werden nicht belüftet; die offene TV-Nische und die offenen Sockelräume hinter den zurückgesetzten Sockelblenden tauschen Wärme mit dem Raum."),
]
#endregion


#region 🖨️ Representations and volumes
PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": 0.13, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES_50 = [{"id": "grain-v", "title": T("Oak decor grain, vertical", "Eichendekor-Maserung senkrecht"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.6, "penMm": 0.1}, {"id": "grain-h", "title": T("Oak decor grain, horizontal", "Eichendekor-Maserung waagerecht"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}]
REP_DECISIONS = {
    50: {"silhouette": T("Wall unit 54 x 11 mm in front, 11 x 40 mm in side view: three 0.90 m bodies of 19 mm boards on 12 mm folded blankets, the cornice profile 60 mm high standing 30 mm proud of the fronts. Side elevation: the right side panel of the TV body with every knob of the unit standing 25 mm proud. Plan: section B-B at 0.55 m through the cabinets: cut side panels, backs, the linen and lower TV doors with their routed stiles and the closed bar flap, below them shelves, bottoms, the drawer knobs and the replaced handle.", "Schrankwand 54 x 11 mm in der Vorder-, 11 x 40 mm in der Seitenansicht: drei 0,90-m-Teile aus 19-mm-Platten auf 12 mm gefalteten Decken, das 60 mm hohe Kranzprofil steht 30 mm vor den Fronten. Seitenansicht: die rechte Seite des TV-Teils mit allen 25 mm vorstehenden Knöpfen. Grundriss: Schnitt B-B auf 0,55 m durch die Fächer: geschnittene Seiten, Rückwände, Wäscheschrank- und untere TV-Türen mit gefrästen Friesen und die geschlossene Barklappe, darunter Böden, Schubkastenknöpfe und der Ersatzgriff."),
         "retained": T("Board thicknesses, the 8 mm routed frame relief of every front, the glass doors with 55 mm frames, the drop flap on its outer hinge and its two stays, both drawers with their boxes, the open TV niche with its 60 mm cable hole, the missing knob, the replaced chrome handle, the sun-bleached upper side, the price label and the chipped foil.", "Plattenstärken, das 8 mm erhabene Fräsprofil jeder Front, die Glastüren mit 55-mm-Rahmen, die Klappe am äußeren Band mit zwei Haltern, beide Schubkästen mit Kästen, die offene TV-Nische mit 60-mm-Kabeldurchlass, der fehlende Knopf, der Chrom-Ersatzgriff, die sonnengebleichte obere Seite, das Preisetikett und die abgeplatzte Folie."),
         "merged": T("Edge banding into the boards, the stays' brake guides into one block each.", "Kanten in die Platten, die Bremsführungen der Halter zu je einem Block."),
         "omitted": T("Hinge cups, cam fittings, shelf pins, the 32 mm hole rows, screws.", "Topfbänder, Exzenterverbinder, Bodenträger, 32-mm-Lochreihen, Schrauben."),
         "thinElements": T("The 3 mm backs (0.06 mm), the 4 mm glass and the 12 mm blankets are drawn at true size and read by tone; the stays 6 mm.", "Die 3-mm-Rückwände (0,06 mm), das 4-mm-Glas und die 12-mm-Decken sind in wahrer Größe gezeichnet und wirken durch Ton; die Halter 6 mm."),
         "hatching": T("Oak decor grain on the solid fronts, vertical on doors, horizontal on drawer fronts.", "Eichendekor-Maserung auf den Vollholzfronten, senkrecht auf Türen, waagerecht auf Schubkastenblenden."),
         "styles": T("Carcass 0.55, cornice and frames 0.45, fronts 0.5, plinth 0.3, backs 0.8, glass 0.9, blankets 0.35, cut faces in plan 0.3; board outlines 0.25 mm.", "Korpus 0,55, Kranz und Rahmen 0,45, Fronten 0,5, Sockel 0,3, Rückwände 0,8, Glas 0,9, Decken 0,35, Schnittflächen im Grundriss 0,3; Plattenkonturen 0,25 mm."),
         "overlays": T("One shape per board, front, knob and blanket, bound to the regions carcass, cornice, fronts, glazing, back-panel and blankets.", "Eine Form je Platte, Front, Knopf und Decke, gebunden an die Regionen carcass, cornice, fronts, glazing, back-panel und blankets.")},
    100: {"silhouette": T("Wall unit 27 x 5.6 mm in front: the three bodies, a stepped cornice, slab fronts with the routed frame as a line, cylindrical knobs.", "Schrankwand 27 x 5,6 mm in der Vorderansicht: die drei Teile, ein gestufter Kranz, Plattenfronten mit dem Fräsrahmen als Linie, zylindrische Knöpfe."),
          "retained": T("Board thicknesses, glass doors and shelves, the flap and its stays, both drawers, the open niche with its cable hole, the missing knob, the replaced handle, the faded side and the label.", "Plattenstärken, Glastüren und Glasböden, die Klappe mit Haltern, beide Schubkästen, die offene Nische mit Kabeldurchlass, der fehlende Knopf, der Ersatzgriff, die verblichene Seite und das Etikett."),
          "merged": T("The routed relief of the fronts into their slabs, the cornice cove into two steps, drawer boxes into blocks.", "Das Fräsprofil der Fronten in ihre Platten, die Kranzkehle in zwei Stufen, Schubkastenkästen zu Blöcken."),
          "omitted": T("Scuffs, chips, the stay guides, the grain hatch.", "Kratzer, Abplatzer, die Halterführungen, die Maserung."),
          "thinElements": T("Backs, glass and blankets at true size, read by tone.", "Rückwände, Glas und Decken in wahrer Größe, durch Ton lesbar."),
          "hatching": T("None.", "Keine."),
          "styles": T("As 1:50, outlines 0.18 mm.", "Wie 1:50, Konturen 0,18 mm."),
          "overlays": T("One shape per board and front, bound to the six drawn regions.", "Eine Form je Platte und Front, gebunden an die sechs gezeichneten Regionen.")},
    200: {"silhouette": T("Wall unit 13.5 x 2.8 mm in front: three carcass boxes, the cornice band, fronts drawn 30 mm deep, glass doors, the open niche. Plan: the three tops with the cornice strip and the blankets behind.", "Schrankwand 13,5 x 2,8 mm in der Vorderansicht: drei Korpuskästen, das Kranzband, 30 mm tiefe Fronten, Glastüren, die offene Nische. Grundriss: die drei Oberseiten mit Kranzstreifen und den Decken dahinter."),
          "retained": T("The three bodies, the plinth band, doors, drawers and flap as separate fronts, the display glass, the niche and the faded side.", "Die drei Teile, das Sockelband, Türen, Schubkästen und Klappe als eigene Fronten, das Vitrinenglas, die Nische und die verblichene Seite."),
          "merged": T("Boards into one carcass per body, the cornice profile into a 60 mm band.", "Platten zu einem Korpus je Teil, das Kranzprofil zu einem 60-mm-Band."),
          "omitted": T("Knobs, the handle, the routed lines, the label, the stays.", "Knöpfe, Griff, Fräslinien, Etikett, Halter."),
          "thinElements": T("Fronts are drawn at the 30 mm feature; backs, glass and blankets thin.", "Fronten sind mit dem 30-mm-Merkmal gezeichnet; Rückwände, Glas und Decken dünn."),
          "hatching": T("None.", "Keine."),
          "styles": T("Carcass 0.55, fronts 0.5, cornice 0.45, outlines 0.13 mm.", "Korpus 0,55, Fronten 0,5, Kranz 0,45, Konturen 0,13 mm."),
          "overlays": T("One shape per body, front and blanket.", "Eine Form je Teil, Front und Decke.")},
    500: {"silhouette": T("Wall unit 5.4 x 1.1 mm in front: three boxes with 75 mm fronts, the cornice band, the niche and the glass; side and plan as outlines.", "Schrankwand 5,4 x 1,1 mm in der Vorderansicht: drei Kästen mit 75-mm-Fronten, das Kranzband, die Nische und das Glas; Seite und Grundriss als Umriss."),
          "retained": T("Overall 2.70 x 0.56 x 2.01 m, the three bodies, the bar flap and both drawers as fronts, the niche.", "Gesamt 2,70 x 0,56 x 2,01 m, die drei Teile, Barklappe und beide Schubkästen als Fronten, die Nische."),
          "merged": T("Boards into one carcass per body, fronts to the 75 mm feature, the cornice into a 75 mm band.", "Platten zu einem Korpus je Teil, Fronten auf das 75-mm-Merkmal, der Kranz zu einem 75-mm-Band."),
          "omitted": T("Knobs, the handle, shelves, mirror, stays, every trace of use.", "Knöpfe, Griff, Böden, Spiegel, Halter, alle Gebrauchsspuren."),
          "thinElements": T("Backs, glass and blankets stay thin and read by tone only.", "Rückwände, Glas und Decken bleiben dünn und wirken nur durch Ton."),
          "hatching": T("None.", "Keine."),
          "styles": T("Carcass 0.55, fronts 0.5, outlines 0.18 mm.", "Korpus 0,55, Fronten 0,5, Konturen 0,18 mm."),
          "overlays": T("One shape per body, front and blanket.", "Eine Form je Teil, Front und Decke.")},
}
VOL_DECISIONS = {
    50: [T("Body 1:50 is the unit as built: per body two 19 mm side panels to the floor, bottom, top and fixed boards between them, the 3 mm hardboard back 8 mm in front of the rear edge, the plinth board recessed 51 mm, the profiled cornice 60 x 70 mm; adjustable shelves inside; every door and drawer front an 11 mm slab with the 8 mm proud routed frame; glass doors with 55 mm frames and 4 mm panes; two drawers with boxes, 4 mm bottoms and roller runners; the bar flap on its outer bottom hinge with two 6 mm stays through their brake guides; mirror back and glass shelf in the bar; two glass shelves in the display; turned knobs, the replaced chrome D-handle, the missing knob of the upper TV door, the 60 mm cable hole; three blankets folded to 12 mm with rounded folds.", "Der Körper 1:50 ist die Schrankwand, wie sie gebaut ist: je Teil zwei 19-mm-Seiten bis zum Boden, Boden, Deckel und feste Böden dazwischen, die 3-mm-Hartfaserrückwand 8 mm vor der Hinterkante, die 51 mm zurückgesetzte Sockelblende, der profilierte Kranz 60 x 70 mm; verstellbare Böden innen; jede Tür- und Schubkastenfront eine 11-mm-Platte mit 8 mm erhabenem Fräsrahmen; Glastüren mit 55-mm-Rahmen und 4-mm-Scheiben; zwei Schubkästen mit Kästen, 4-mm-Böden und Rollenauszügen; die Barklappe an ihrem äußeren Unterkantenband mit zwei 6-mm-Haltern durch ihre Bremsführungen; Spiegelrückwand und Glasboden in der Bar; zwei Glasböden in der Vitrine; gedrechselte Knöpfe, der Chrom-Ersatzbügelgriff, der fehlende Knopf der oberen TV-Tür, der 60-mm-Kabeldurchlass; drei auf 12 mm gefaltete Decken mit runden Falzen."),
         T("Thermal 1:50: the boards are their own surfaces (references); the fronts are own solids that close the 3 mm running gaps, so the closed cabinets are sealed; the glass panes are the glazing, the 35 W line lamp a hidden solid under the bar top. Structure 1:50: the carcass boards are the members of each body, the flap slab, the stays, the drawer boxes and the blankets theirs; flap hinges, stay pins, runners and guides are joints; the plinths of the two end bodies are the supports, the centre body stands on its contact; masses at the centres of the three bodies.", "Thermik 1:50: die Platten sind ihre eigenen Oberflächen (Verweise); die Fronten sind eigene Körper, die die 3-mm-Fugen schließen, sodass die geschlossenen Fächer dicht sind; die Glasscheiben sind die Verglasung, die 35-W-Linienlampe ein verborgener Körper unter dem Bardeckel. Tragwerk 1:50: die Korpusplatten sind die Stäbe jedes Teils, Klappenplatte, Halter, Schubkastenkästen und Decken die ihren; Klappenbänder, Halterbolzen, Auszüge und Führungen sind Gelenke; die Sockel der zwei Endteile sind die Auflager, das Mittelteil steht auf seinem Kontakt; Massen in den Schwerpunkten der drei Teile.")],
    100: [T("Body 1:100, authored for 15 mm: boards as at 1:50, fronts as plain 19 mm slabs (the 8 mm relief is below the feature), knobs as 30 x 25 mm cylinders, the chrome handle as a block, drawer boxes as blocks, the cornice with two steps, guides and drawer bottoms left out; glass, mirror, backs and blankets declared thin sheets.", "Körper 1:100, für 15 mm gebaut: Platten wie bei 1:50, Fronten als glatte 19-mm-Platten (das 8-mm-Relief liegt unter dem Merkmal), Knöpfe als Zylinder 30 x 25 mm, der Chromgriff als Block, Schubkastenkästen als Blöcke, der Kranz mit zwei Stufen, Führungen und Schubkastenböden entfallen; Glas, Spiegel, Rückwände und Decken als dünne Bleche erklärt."),
          T("Thermal 1:100 references the body (the 3 mm gaps are below a quarter feature and closed by the flood). Structure 1:100: the boards, flap, stays, drawer blocks and blankets as members, 15 mm hinge and pin joints, supports and masses.", "Thermik 1:100 verweist auf den Körper (die 3-mm-Fugen liegen unter einem Viertel Merkmal und sind für die Flutung zu). Tragwerk 1:100: Platten, Klappe, Halter, Schubkastenblöcke und Decken als Stäbe, 15-mm-Band- und Bolzengelenke, Auflager und Massen.")],
    200: [T("Body 1:200, authored for 30 mm: each carcass is one 19 mm tube of its side panels and boards with a hole per compartment (judged by its overall extents), closed at the back by the hardboard sheet; fronts are 30 mm slabs, the glass doors 30 mm frames with the pane; the drawers are front and box blocks; the cornice a 60 x 70 mm band; mirror, bar shelf and display shelves stay as thin sheets; knobs, handle, stays and the hidden shelves are left out.", "Körper 1:200, für 30 mm gebaut: jeder Korpus ist ein 19-mm-Rohr aus Seiten und Böden mit einem Loch je Fach (nach seinen Gesamtmaßen beurteilt), hinten von der Hartfaserplatte geschlossen; Fronten sind 30-mm-Platten, die Glastüren 30-mm-Rahmen mit Scheibe; die Schubkästen Front- und Kastenblöcke; der Kranz ein Band 60 x 70 mm; Spiegel, Barboden und Vitrinenböden bleiben dünne Bleche; Knöpfe, Griff, Halter und die verdeckten Böden entfallen."),
          T("Thermal 1:200 references the tubes, fronts, glass, backs, cornice and blankets. Structure 1:200: the side panels and boards as 19 mm declared sheets, the backs, the flap, the drawer blocks, the stays as 6 mm rods, the blankets, 30 mm hinge joints, supports and masses.", "Thermik 1:200 verweist auf Rohre, Fronten, Glas, Rückwände, Kranz und Decken. Tragwerk 1:200: Seiten und Böden als erklärte 19-mm-Bleche, die Rückwände, die Klappe, die Schubkastenblöcke, die Halter als 6-mm-Stangen, die Decken, 30-mm-Bandgelenke, Auflager und Massen.")],
    500: [T("Body 1:500, authored for 75 mm: the carcass tubes end 75 mm behind the front plane, the fronts are 75 mm slabs (doors, drawers, flap, glass door frames), the cornice a 75 x 75 mm band on a carcass 15 mm lower, the backs, the display glass and the blankets thin sheets; mirror and glass shelves are left out. The model is minimal: fewer parts would lose the flap and the drawers that move.", "Körper 1:500, für 75 mm gebaut: die Korpusrohre enden 75 mm hinter der Frontebene, die Fronten sind 75-mm-Platten (Türen, Schubkästen, Klappe, Glastürrahmen), der Kranz ein Band 75 x 75 mm auf einem 15 mm niedrigeren Korpus, Rückwände, Vitrinenglas und Decken dünne Bleche; Spiegel und Glasböden entfallen. Das Modell ist minimal: weniger Teile verlören Klappe und Schubkästen, die sich bewegen."),
          T("Thermal 1:500 references the body; the lamp is a 75 mm block in the bar. Structure 1:500: side panels and boards as declared 19 mm sheets, backs, flap, drawers, stays, blankets, supports and masses; the hinges are below the feature and merged into the flap member.", "Thermik 1:500 verweist auf den Körper; die Lampe ist ein 75-mm-Block in der Bar. Tragwerk 1:500: Seiten und Böden als erklärte 19-mm-Bleche, Rückwände, Klappe, Schubkästen, Halter, Decken, Auflager und Massen; die Bänder liegen unter dem Merkmal und gehen im Klappenstab auf.")],
}


def representation(scale, elevation, front, plan, sections=None, hatches=None):
    r = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": RANGES[scale][0], "maxDenominator": RANGES[scale][1]}, "pens": PENS[scale], "hatches": hatches or [], "geometry": {"elevation": elevation, "front": front, "plan": plan}}
    if sections:
        r["sections"] = sections
    r["decisions"] = REP_DECISIONS[scale]
    return r


def volume(scale, body, thermal, structure):
    return {"status": "authored", "denominator": scale, "body": [clean(s) for s in body], "thermal": [clean(s) for s in thermal], "structure": [clean(s) for s in structure], "decisions": VOL_DECISIONS[scale]}


REPRESENTATIONS = {
    "50": representation(50, elevation_50(), front_50(), plan_50(), {"plan": {"plane": PLAN_CUT, "label": T("B-B", "B-B")}}, HATCHES_50),
    "100": representation(100, elevation_100(), front_100(), plan_top(100, 0.46, 0.46, 0.18)),
    "200": representation(200, coarse_elevation(200), coarse_front(200), plan_top(200, 0.46, 0.46, 0.13)),
    "500": representation(500, coarse_elevation(500), coarse_front(500), plan_top(500, 0.455, 0.455, 0.18)),
}
VOLUMES = {
    "50": volume(50, body50, thermal_50(), structure_50()),
    "100": volume(100, body100, thermal_100(), structure_100()),
    "200": volume(200, coarse_body(200), coarse_thermal(200), coarse_structure(200)),
    "500": volume(500, coarse_body(500), coarse_thermal(500), coarse_structure(500)),
}
SYMBOL = {"id": "used-wall-unit-symbol", "title": T("Wall unit symbol, true extent", "Schrankwandsymbol, wahre Ausdehnung"), "description": T("The stored wall unit in all three views: three bodies 0.90 x 0.53 x 2.00 m side by side on 12 mm blankets, the fronts facing ahead, the cornice standing 30 mm proud.", "Die gelagerte Schrankwand in allen drei Ansichten: drei Teile 0,90 x 0,53 x 2,00 m nebeneinander auf 12-mm-Decken, die Fronten nach vorn, der Kranz 30 mm vorstehend."), "semantics": "true-extent", "units": "model-m", "dimensionalNote": T("Model metres at real size in the frame of the asset (x ahead, lateral positive to the left, y up): the side view is seen from the right, the front view from ahead, the top view from above; all three show the stored default pose. The backs lie at x = 0, the fronts face ahead (+x).", "Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn, seitlich positiv nach links, y nach oben): die Seitenansicht blickt von rechts, die Vorderansicht von vorn, die Draufsicht von oben; alle drei zeigen die gelagerte Standardpose. Die Rückwände liegen bei x = 0, die Fronten weisen nach vorn (+x)."), "denominators": [200, 100, 50], "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [],
          "geometry": {"elevation": [S("unit", "ground", "body", 1, True, 0.85, Draw.poly([(-0.03, 0), (0.53, 0), (0.53, Y_TOP + BL), (0.46, Y_TOP + BL), (0.46, Y_T1 + BL), (-0.03 + 0.03, Y_T1 + BL), (0, BL), (-0.03, BL)]))],
                       "front": [S(f"body-{k + 1}", "ground", "body", 1, True, 0.85, Draw.rect(zc - HW, zc + HW, 0, Y_TOP + BL)) for k, zc in enumerate((-0.9, 0.0, 0.9))],
                       "plan": [S("unit", "ground", "body", 1, True, 0.85, Draw.rect(-0.03, 0.53, -1.35, 1.35))] + [S(f"joint-{k + 1}", "ground", "detail", 2, False, 0.5, Draw.poly([(0, z), (0.53, z)], closed=False)) for k, z in enumerate((-0.45, 0.45))]}}
#endregion


#region 📄 Document
ASSET = {
    "id": "used-wall-unit", "category": "furniture",
    "title": T("Used Living Room Wall Unit", "Gebrauchte Schrankwand"),
    "summary": T("A 1980s oak-look living-room wall unit of three 0.90 m bodies, 2.70 x 0.53 x 2.00 m and 254 kg, taken apart at a flat clearance and stored on folded moving blankets: a two-door linen cabinet, a centre body with two drawers, a mirrored bar behind a drop flap whose 35 W light switches on as the flap opens and a glass display cabinet, and a TV body with an open niche. The sun has bleached the right side, a knob is lost, a drawer handle was replaced and a price label sticks to the glass.",
                 "Eine Schrankwand der 1980er Jahre in Eichenoptik aus drei 0,90-m-Teilen, 2,70 x 0,53 x 2,00 m und 254 kg, bei einer Haushaltsauflösung zerlegt und auf gefalteten Möbeldecken gelagert: ein zweitüriger Wäscheschrank, ein Mittelteil mit zwei Schubkästen, einer verspiegelten Bar hinter einer Klappe, deren 35-W-Licht beim Öffnen angeht, und einer Glasvitrine, sowie ein TV-Teil mit offener Nische. Die Sonne hat die rechte Seite ausgebleicht, ein Knopf fehlt, ein Schubkastengriff wurde ersetzt und auf dem Glas klebt ein Preisetikett."),
    "tags": TAGS,
    "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES],
                   "notes": T(f"Catalogue-typical 1980s wall unit of three 0.90 m elements, 2.70 m wide, 0.48 m carcass depth and 0.50 m to the fronts, 2.00 m high with the cornice, in 19 mm chipboard P2 with oak-decor foil, MDF fronts routed to a frame-and-panel look, 3 mm hardboard backs. Masses are section x density of the modelled parts ({TOTAL_KG} kg with the blankets); no parameter is measured. Traces of use: sun-bleached right side panel with its shadow line, chipped foil at the plinth corner and two door corners, scuffed plinths, the knob of the upper right TV door lost (screw hole left), a drawer knob replaced by a chrome D-handle, an old price label on the glass, a lighter rectangle where the television stood.",
                              f"Katalogtypische Schrankwand der 1980er Jahre aus drei 0,90-m-Elementen, 2,70 m breit, 0,48 m Korpustiefe und 0,50 m bis zu den Fronten, 2,00 m hoch mit Kranz, aus 19-mm-Spanplatte P2 mit Eichendekorfolie, MDF-Fronten mit gefrästem Rahmen-Füllungs-Bild, 3-mm-Hartfaserrückwänden. Massen sind Querschnitt x Dichte der modellierten Teile ({TOTAL_KG} kg mit den Decken); kein Parameter ist gemessen. Gebrauchsspuren: sonnengebleichte rechte Seite mit Schattenkante, abgeplatzte Folie an der Sockelecke und an zwei Türecken, verkratzte Sockel, der Knopf der oberen rechten TV-Tür verloren (Schraubloch bleibt), ein Schubkastenknopf durch einen Chrom-Bügelgriff ersetzt, ein altes Preisetikett auf dem Glas, ein helleres Rechteck, wo der Fernseher stand.")},
    "dimensions": {"heightM": R(Y_TOP + BL), "widthM": 0.56, "depthM": 2.7, "massKg": TOTAL_KG},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "ground", "bones": BONES},
    "anchors": ANCHORS,
    "representations": REPRESENTATIONS,
    "symbols": [SYMBOL],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": {"environment": ENVIRONMENT, "passengers": [], "operating": [{"id": "bar-light", "title": T("Bar light", "Barlicht"), "levels": [{"v": 0, "title": T("Off (flap closed)", "Aus (Klappe zu)")}, {"v": 1, "title": T("On, 35 W (flap open)", "An, 35 W (Klappe offen)")}]}], "regions": REGIONS, "emitters": [], "assumptions": THERMAL_ASSUMPTIONS},
    "structure": STRUCTURE,
    "volumes": VOLUMES,
    "presentation": {"defaultPose": "stored", "defaultClip": "open-bar-flap", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}
DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}

if __name__ == "__main__":
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(DOC, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(f"[DEBUG] wrote 🔣️.json: {TOTAL_KG} kg, solids per scale", {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in VOLUMES.items()})
#endregion
