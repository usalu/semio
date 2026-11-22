"""🛁️ Authoritative chisel of the asset `used-bathtub`: every hand-chosen number of the 170 x 75 cm enamelled steel bath on its adjustable foot set, stored upright on its foot end on a folded moving blanket, and the writer of `🔣️.json` next to this file.

Run: `python3 author-used-bathtub.py` (writes `🔣️.json`, prints the derived masses, areas, extents and clip clearances with a `[DEBUG]` prefix).

Frames. The tub is designed in its FLAT frame (X along the bath, + towards the foot end with the waste, Y up from the bathroom floor with the bath on its feet, Z to the left, metres). The bone `tub` has its origin at the outer lower edge of the rim lip at the foot end (flat X 0.85, Y 0.558) and its axes u = -X, v = -Y, w = Z (the flat frame turned by 180 degrees), so a bone-local point is (0.85 - X, 0.558 - Y, Z). At rest (the stored pose) the bone stands at 100 degrees: the bath rests upright on its foot end on the blanket, leaning back 10 degrees with its upper feet against the depot wall; the pose offset +80 degrees lays it flat for carrying and use.

@see ../../brief.md — the brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the contract
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
G = 9.80665


def T(en, de):
    """🗣️ One bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯️ Rounds a coordinate to the micrometre."""
    return round(v + 0.0, 6)


TH = 0.0023
STEEL_T, ENAMEL_T, GROUND_T = 0.0020, 0.0002, 0.0001
Y_RIM = 0.580
LIP = 0.022
Y_LIP = Y_RIM - LIP
Y_WT = Y_RIM - TH
Y_BO = 0.170
Y_WB = Y_BO + TH
H_W = Y_WT - Y_WB
K_TAPER = 0.8
RIM = (0.85, 0.375, 0.05)
OPEN = (0.805, 0.33, 0.12)
XP, YP = 0.85, Y_LIP
DRAIN_X, DRAIN_R = 0.50, 0.026
OVER_Y, OVER_R = 0.47, 0.026
RAIL_X = (0.40, -0.40)
RAIL_W, RAIL_H, RAIL_T, RAIL_Z = 0.040, 0.020, 0.002, 0.28
Y_RAIL0, Y_RAIL1 = Y_BO - RAIL_H, Y_BO
FOOT_Z = 0.23
Y_CAP = 0.020
STUD_R = 0.005
NUT_R, NUT_H = 0.009, 0.008
Y_LOCK = 0.100
CAP_PROFILE = [(0.0, 0.014), (0.003, 0.018), (0.008, 0.020), (0.020, 0.020)]

BT = 0.018
BLX0, BLL, BLZ = -0.43, 0.50, 0.50
REST_DEG = 100.0
FLAT_DEG = 80.0
WALL_GAP = 0.0

RHO_ES = (STEEL_T * 7850 + (ENAMEL_T + GROUND_T) * 2500) / TH
DENS = {"enamelled-steel": RHO_ES, "galvanised-steel": 7850, "pe-hd-black": 950, "moving-blanket": None}
BLANKET_KG = 1.4
WATER_KG, BATHER_KG = 150.0, 80.0


def s_of(Y):
    """📉️ The taper scale of the deep-drawn wall at height Y (1 at the wall top, 0.8 at the bottom)."""
    return 1 - (1 - K_TAPER) * (Y_WT - Y) / H_W


def xi(Y):
    """↔️ Inner half-length of the bath at height Y."""
    return OPEN[0] * s_of(Y)


def xo(Y):
    """↔️ Outer half-length of the wall at height Y."""
    return (OPEN[0] + TH) * s_of(Y)


def zo(Y):
    """↕️ Outer half-width of the wall at height Y."""
    return (OPEN[1] + TH) * s_of(Y)


def zi(Y):
    """↕️ Inner half-width of the bath at height Y."""
    return OPEN[1] * s_of(Y)


def L(X, Y):
    """📍️ Flat-frame point to the (u, v) of the bones `tub` and `feet`."""
    return (r6(XP - X), r6(YP - Y))


def LV(Y):
    """📏️ The v of a flat height."""
    return r6(YP - Y)


def rr_area(a, b, r):
    """🧮️ Area of a rounded rectangle of half-sides a, b and corner radius r."""
    return 4 * a * b - (4 - math.pi) * r * r


def rr_perimeter(a, b, r):
    """📐️ Perimeter of a rounded rectangle."""
    return 4 * (a - r) + 4 * (b - r) + 2 * math.pi * r


def rr_ring(a, b, r, n, xc=0.0):
    """⭕️ A rounded rectangle about flat (xc, 0) as an extrude profile of the plane zx: points [w, u]."""
    pts = []
    for cx, cz, a0 in ((a - r, b - r, 0), (-(a - r), b - r, 90), (-(a - r), -(b - r), 180), (a - r, -(b - r), 270)):
        for k in range(n + 1):
            t = math.radians(a0 + 90 * k / n)
            X = xc + cx + r * math.cos(t)
            Z = cz + r * math.sin(t)
            pts.append([r6(Z), r6(XP - X)])
    return pts


def circle_ring(X, Z, r, n):
    """🔘️ A circular hole about flat (X, Z) in the plane zx."""
    return [[r6(Z + r * math.sin(2 * math.pi * k / n)), r6(XP - X - r * math.cos(2 * math.pi * k / n))] for k in range(n)]


def ring_extrude(id, bone, outer, holes, y_top, y_bot, material, tone, taper=None, role="body", **kw):
    """🧱️ A horizontal ring or plate of the bath between two flat heights, extruded along v (from the top down), optionally tapered towards the bottom."""
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "plane": "zx", "profile": outer, "from": LV(y_top), "to": LV(y_bot)}
    if holes:
        s["holes"] = holes
    if taper is not None:
        s["taperTo"] = [1, taper]
    s.update(kw)
    return s


def revolve_v(id, bone, X, y_top, Z, profile, material, tone, **kw):
    """🔩️ A turned part about the vertical axis at flat (X, Z), its profile (depth below y_top, radius) running down along v."""
    u, v = L(X, y_top)
    pts = [[0.0, 0.0]] + [[r6(a), r6(r)] for a, r in profile] + [[r6(profile[-1][0]), 0.0]]
    s = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "axis": [0, 1, 0], "origin": [u, v, r6(Z)], "profile": pts}
    s.update(kw)
    return s


def box(id, bone, X0, X1, Y0, Y1, Z0, Z1, material, role="body", tone=0.6, **kw):
    """📦️ An axis-aligned box from flat bounds, in the frame of its bone."""
    u, v = L((X0 + X1) / 2, (Y0 + Y1) / 2)
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [r6(abs(X1 - X0)), r6(abs(Y1 - Y0)), r6(abs(Z1 - Z0))], "transform": {"translation": [u, v, r6((Z0 + Z1) / 2)]}}
    s.update(kw)
    return s


def quat_w(deg):
    """🔄️ The unit quaternion of a turn about the bone axis w."""
    h = math.radians(deg) / 2
    return [0.0, 0.0, r6(math.sin(h)), r6(math.cos(h))]


FOOT_SLOPE = math.degrees(math.atan2((OPEN[0]) * (1 - K_TAPER), H_W))
SIDE_SLOPE = math.degrees(math.atan2((OPEN[1]) * (1 - K_TAPER), H_W))


def wall_disc(id, Y, r_w, r_n, lift, material, tone):
    """🎯️ A flat texture disc lying on the inner face of the foot-end wall at height Y (thin along the wall normal)."""
    X = xi(Y) - lift * math.cos(math.radians(FOOT_SLOPE))
    Yc = Y + lift * math.sin(math.radians(FOOT_SLOPE))
    u, v = L(X, Yc)
    return {"id": id, "kind": "ellipsoid", "bone": "tub", "material": material, "role": "detail", "tone": tone, "texture": True, "radii": [0.0006, r_n, r_w], "transform": {"rotation": quat_w(-FOOT_SLOPE), "translation": [u, v, 0.0]}}


def rails_xy(id, bone, w, h, wall, z, material, tone, hole=True, **kw):
    """🛤️ The two foot rails as rectangular steel tubes across the bath (profile in u, v; along w), repeated from the foot-end rail to the head-end rail."""
    u0, v0 = L(RAIL_X[0] + w / 2, Y_RAIL1)
    u1, v1 = L(RAIL_X[0] - w / 2, Y_RAIL1 - h)
    prof = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]]
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "plane": "xy", "profile": prof, "from": -z, "to": z, "repeat": {"count": 2, "translation": [r6(RAIL_X[0] - RAIL_X[1]), 0, 0]}}
    if hole:
        s["holes"] = [[[r6(u0 + wall), r6(v0 + wall)], [r6(u1 - wall), r6(v0 + wall)], [r6(u1 - wall), r6(v1 - wall)], [r6(u0 + wall), r6(v1 - wall)]]]
    s.update(kw)
    return s


def pi_frame(id, bone, band, leg, width, material, tone):
    """🦿️ A coarse foot frame: rail band across and the two leg pairs down to the floor as one profile in (v, w), extruded along u over the rail width, repeated to the head-end rail."""
    v0 = LV(Y_RAIL1)
    v1 = r6(v0 + band)
    vf = LV(0.0)
    zc0, zc1 = FOOT_Z - leg / 2, FOOT_Z + leg / 2
    prof = [[v0, -RAIL_Z], [v0, RAIL_Z], [v1, RAIL_Z], [v1, zc1], [vf, zc1], [vf, zc0], [v1, zc0], [v1, -zc0], [vf, -zc0], [vf, -zc1], [v1, -zc1], [v1, -RAIL_Z]]
    prof = [[r6(a), r6(b)] for a, b in prof]
    u0 = r6(XP - RAIL_X[0] - width / 2)
    return {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "plane": "yz", "profile": prof, "from": u0, "to": r6(u0 + width), "repeat": {"count": 2, "translation": [r6(RAIL_X[0] - RAIL_X[1]), 0, 0]}}


def arc(cx, cy, r, a0, a1, n):
    """⭕️ Points of a circular arc from angle a0 to a1 (degrees), both ends included."""
    return [(r6(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n))), r6(cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n)))) for k in range(n + 1)]


BLANKET_PROFILE = [(BLL, 0.0), (BLL, BT), (0.009, BT)] + arc(0.009, 0.009, 0.009, 90, 270, 6)[1:-1] + [(0.009, 0.0)]
BLANKET_PROFILE = [[r6(u), r6(v)] for u, v in BLANKET_PROFILE]


def blanket50(id):
    """🧺️ The moving blanket folded to 0.50 x 1.00 m (six layers) with its round fold towards the wall."""
    return {"id": id, "kind": "extrude", "bone": "blanket", "material": "moving-blanket", "role": "body", "tone": 0.45, "plane": "xy", "profile": BLANKET_PROFILE, "from": -BLZ, "to": BLZ, "sheet": True}


def blanket_box(id):
    """🧺️ The folded blanket as a plain sheet at the coarser scales."""
    return {"id": id, "kind": "box", "bone": "blanket", "material": "moving-blanket", "role": "body", "tone": 0.45, "size": [BLL, BT, 2 * BLZ], "transform": {"translation": [BLL / 2, BT / 2, 0.0]}, "sheet": True}


def wall_rings(n):
    """🛁️ The outer and inner top rings of the deep-drawn wall."""
    return rr_ring(OPEN[0] + TH, OPEN[1] + TH, OPEN[2] + TH, n), rr_ring(*OPEN, n)


def bottom_outer(n):
    """⬛️ The outer bottom ring (the wall's outer ring at its foot)."""
    k = K_TAPER
    return rr_ring(k * (OPEN[0] + TH), k * (OPEN[1] + TH), k * (OPEN[2] + TH), n)


def body50():
    """🛁️ The 1:50 body: rim plate, rim lip, the deep-drawn tapered wall, the bottom with the open waste hole, the two foot rails, four studs with welded and lock nuts and PE caps, the traces of use and the folded blanket."""
    n = 8
    wo, wi = wall_rings(n)
    s = [
        ring_extrude("rim", "tub", rr_ring(*RIM, n), [rr_ring(*OPEN, n)], Y_RIM, Y_WT, "enamelled-steel", 0.9, sheet=True),
        ring_extrude("lip", "tub", rr_ring(*RIM, n), [rr_ring(RIM[0] - TH, RIM[1] - TH, RIM[2] - TH, n)], Y_WT, Y_LIP, "enamelled-steel", 0.86),
        ring_extrude("wall", "tub", wo, [wi], Y_WT, Y_WB, "enamelled-steel", 0.82, taper=K_TAPER),
        ring_extrude("bottom", "tub", bottom_outer(n), [circle_ring(DRAIN_X, 0.0, DRAIN_R, 16)], Y_WB, Y_BO, "enamelled-steel", 0.8, sheet=True),
        rails_xy("rails", "feet", RAIL_W, RAIL_H, RAIL_T, RAIL_Z, "galvanised-steel", 0.55),
    ]
    for tag, X in (("foot", RAIL_X[0]), ("head", RAIL_X[1])):
        rep = {"count": 2, "translation": [0, 0, -2 * FOOT_Z]}
        s.append(revolve_v(f"studs-{tag}", "feet", X, Y_RAIL0, FOOT_Z, [(0.0, STUD_R), (Y_RAIL0 - Y_CAP, STUD_R)], "galvanised-steel", 0.5, repeat=rep))
        s.append(revolve_v(f"nuts-{tag}", "feet", X, Y_RAIL0, FOOT_Z, [(0.0, NUT_R), (NUT_H, NUT_R)], "galvanised-steel", 0.45, repeat=rep))
        s.append(revolve_v(f"locknuts-{tag}", "feet", X, Y_LOCK + NUT_H, FOOT_Z, [(0.0, NUT_R), (NUT_H, NUT_R)], "galvanised-steel", 0.45, repeat=rep))
        s.append(revolve_v(f"caps-{tag}", "feet", X, Y_CAP, FOOT_Z, CAP_PROFILE, "pe-hd-black", 0.2, repeat=rep))
    s += [
        wall_disc("overflow", OVER_Y, OVER_R, OVER_R, 0.0007, "rust-chip", 0.08),
        wall_disc("chip", OVER_Y - 0.012, 0.038, 0.046, 0.0003, "rust-chip", 0.3),
        box("silicone", "tub", -0.79, 0.79, Y_RIM, Y_RIM + 0.0008, 0.345, 0.369, "silicone-residue", role="detail", tone=0.7, texture=True),
        dict(box("lip-chip", "tub", -0.37, -0.33, Y_LIP + 0.004, Y_RIM - 0.004, -RIM[1] - 0.0005, -RIM[1] + 0.0005, "rust-chip", role="detail", tone=0.25, texture=True)),
        ring_extrude("scale-ring", "tub", rr_ring(xi(0.405), zi(0.405), OPEN[2] * s_of(0.405), n), [rr_ring(xi(0.405) - 0.0006, zi(0.405) - 0.0006, OPEN[2] * s_of(0.405) - 0.0006, n)], 0.405, 0.399, "limescale", 0.65, role="detail", texture=True),
        blanket50("blanket"),
    ]
    return s


def body100():
    """🪚️ The 1:100 body: rim plate and lip as one 22 mm band over the flange, the tapered wall, the bottom sheet with its waste hole, rails as 40 x 20 x 2 mm tubes open at their ends, each foot as one turned stud of 16 mm with its cap, the overflow mark and the plain blanket."""
    n = 6
    wo, wi = wall_rings(n)
    s = [
        ring_extrude("rim-100", "tub", rr_ring(*RIM, n), [rr_ring(*OPEN, n)], Y_RIM, Y_LIP, "enamelled-steel", 0.88),
        ring_extrude("wall-100", "tub", wo, [wi], Y_WT, Y_WB, "enamelled-steel", 0.82, taper=K_TAPER),
        ring_extrude("bottom-100", "tub", bottom_outer(n), [circle_ring(DRAIN_X, 0.0, DRAIN_R, 12)], Y_WB, Y_BO, "enamelled-steel", 0.8, sheet=True),
        rails_xy("rails-100", "feet", RAIL_W, RAIL_H, RAIL_T, RAIL_Z, "galvanised-steel", 0.55),
    ]
    for tag, X in (("foot", RAIL_X[0]), ("head", RAIL_X[1])):
        s.append(revolve_v(f"feet-100-{tag}", "feet", X, Y_RAIL0, FOOT_Z, [(0.0, 0.008), (Y_RAIL0 - Y_CAP, 0.008), (Y_RAIL0 - Y_CAP + 0.003, 0.018), (Y_RAIL0, 0.020)], "galvanised-steel", 0.45, repeat={"count": 2, "translation": [0, 0, -2 * FOOT_Z]}))
    s += [wall_disc("overflow-100", OVER_Y - 0.006, 0.04, 0.046, 0.0007, "rust-chip", 0.15), blanket_box("blanket-100")]
    return s


def body200():
    """🧱️ The 1:200 body: the rim band (a declared sheet of 22 mm), the tapered wall, the bottom sheet with the waste hole, each foot frame as one rail-and-legs profile at the 30 mm feature, the overflow mark and the blanket sheet."""
    n = 4
    wo, wi = wall_rings(n)
    return [
        ring_extrude("rim-200", "tub", rr_ring(*RIM, n), [rr_ring(*OPEN, n)], Y_RIM, Y_LIP, "enamelled-steel", 0.88, sheet=True),
        ring_extrude("wall-200", "tub", wo, [wi], Y_WT, Y_WB, "enamelled-steel", 0.82, taper=K_TAPER),
        ring_extrude("bottom-200", "tub", bottom_outer(n), [], Y_WB, Y_BO, "enamelled-steel", 0.8, sheet=True),
        pi_frame("feet-200", "feet", 0.030, 0.030, RAIL_W, "galvanised-steel", 0.5),
        wall_disc("overflow-200", OVER_Y - 0.006, 0.04, 0.046, 0.0007, "rust-chip", 0.15),
        blanket_box("blanket-200"),
    ]


def body500():
    """🧱️ The 1:500 body: rim band, tapered wall, plain bottom sheet, each foot frame as one rail-and-legs profile at the 75 mm feature, the blanket sheet."""
    n = 3
    wo, wi = wall_rings(n)
    return [
        ring_extrude("rim-500", "tub", rr_ring(*RIM, n), [rr_ring(*OPEN, n)], Y_RIM, Y_LIP, "enamelled-steel", 0.88, sheet=True),
        ring_extrude("wall-500", "tub", wo, [wi], Y_WT, Y_WB, "enamelled-steel", 0.82, taper=K_TAPER),
        ring_extrude("bottom-500", "tub", bottom_outer(n), [], Y_WB, Y_BO, "enamelled-steel", 0.8, sheet=True),
        pi_frame("feet-500", "feet", 0.09, 0.09, 0.09, "galvanised-steel", 0.5),
        blanket_box("blanket-500"),
    ]


def poly_area(pts):
    """🧮️ Shoelace area of a ring."""
    return abs(sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]))) / 2


def poly_centroid(pts):
    """🎯️ Centroid of a ring."""
    a2 = sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]))
    cx = sum((x0 + x1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1])) / (3 * a2)
    cy = sum((y0 + y1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1])) / (3 * a2)
    return cx, cy


def volume_centre(s):
    """⚖️ Volume and local centre (u, v, w) of one solid with its repeat copies."""
    if s["kind"] == "box":
        a, b, c = s["size"]
        vol = a * b * c
        cen = list(s["transform"]["translation"])
    elif s["kind"] == "revolve":
        p = s["profile"][1:-1]
        vol, num = 0.0, 0.0
        for (a0, q0), (a1, q1) in zip(p, p[1:]):
            dv = math.pi * (a1 - a0) / 3 * (q0 * q0 + q0 * q1 + q1 * q1)
            vol += dv
            num += dv * (a0 + a1) / 2
        o = s["origin"]
        cen = [o[0], o[1] + num / vol, o[2]]
    elif s["kind"] == "ellipsoid":
        return 0.0, [0, 0, 0]
    else:
        prof = [tuple(q) for q in s["profile"]]
        holes = [[tuple(q) for q in h] for h in s.get("holes", [])]
        ao = poly_area(prof)
        co = poly_centroid(prof)
        area = ao - sum(poly_area(h) for h in holes)
        ca = [co[0] * ao, co[1] * ao]
        for h in holes:
            ch = poly_centroid(h)
            ah = poly_area(h)
            ca[0] -= ch[0] * ah
            ca[1] -= ch[1] * ah
        ca = [ca[0] / area, ca[1] / area]
        ln = s["to"] - s["from"]
        k0, k1 = s.get("taperTo", [1, 1])
        steps = 200
        vol, num = 0.0, 0.0
        for i in range(steps):
            z = (i + 0.5) / steps
            sc = (k0 + (k1 - k0) * z) ** 2
            vol += sc / steps
            num += sc * z / steps
        along = s["from"] + ln * num / vol
        vol = vol * area * ln
        if s["plane"] == "zx":
            cen = [ca[1], along, ca[0]]
        elif s["plane"] == "xy":
            cen = [ca[0], ca[1], along]
        else:
            cen = [along, ca[0], ca[1]]
    rep = s.get("repeat")
    if rep:
        n = rep["count"]
        t = rep.get("translation", [0, 0, 0])
        cen = [cen[i] + t[i] * (n - 1) / 2 for i in range(3)]
        vol *= n
    return vol, cen


def mass_of(solids):
    """⚖️ Masses and centres per bone from section x density (the blanket from its stated mass); texture solids carry no mass."""
    out = {}
    for s in solids:
        if s.get("texture"):
            continue
        vol, c = volume_centre(s)
        rho = DENS[s["material"]]
        kg = BLANKET_KG if rho is None else vol * rho
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
TUB_KG = MASS["tub"]["kg"] + MASS["feet"]["kg"]
RAIL_COM_V = MASS["feet"]["comY"]


def ellipsoid(id, bone, at, r, material, member):
    """⚫️ A compact mass marker at a centre of mass."""
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "radii": [r, r, r] if isinstance(r, float) else r, "transform": {"translation": [r6(at[0]), r6(at[1]), r6(at[2])]}, "member": member, "part": "mass"}


def masses(sfx, r, rb):
    """🎯️ The three mass markers of a scale: shell, foot set, blanket."""
    return [ellipsoid(f"mass-tub{sfx}", "tub", (MASS["tub"]["comX"], MASS["tub"]["comY"], MASS["tub"]["comZ"]), r, "enamelled-steel", "tub"),
            *[ellipsoid(f"mass-feet{sfx}-{tag}", "feet", (XP - X, RAIL_COM_V, 0.0), r, "galvanised-steel", "feet") for tag, X in (("foot", RAIL_X[0]), ("head", RAIL_X[1]))],
            ellipsoid(f"mass-blanket{sfx}", "blanket", (MASS["blanket"]["comX"], MASS["blanket"]["comY"], 0.0), rb, "moving-blanket", "blanket")]


def ref(id, of, **kw):
    """🔗️ A thermal or structural reference to a body solid of the same scale."""
    s = {"id": id, "of": of}
    s.update(kw)
    return s


def pads(id, h, wz):
    """🩹️ The bitumen adhesive pads that join each rail to the bottom, as joint solids straddling the interface."""
    return dict(box(id, "feet", RAIL_X[0] - 0.025, RAIL_X[0] + 0.025, Y_BO - h / 2, Y_BO + h / 2, -wz, wz, "galvanised-steel", role="detail", tone=0.3, repeat={"count": 2, "translation": [r6(RAIL_X[0] - RAIL_X[1]), 0, 0]}), member="feet", part="joint")


def structure50():
    """🏗️ The 1:50 load path: rim, lip, wall and bottom as the shell members of the bath; rails, studs and caps as the members of the foot set; adhesive pads and nuts as joints; three mass markers."""
    st = [ref("m-rim", "rim", member="tub", part="member"), ref("m-lip", "lip", member="tub", part="member"), ref("m-wall", "wall", member="tub", part="member"), ref("m-bottom", "bottom", member="tub", part="member"),
          ref("m-rails", "rails", member="feet", part="member"), pads("j-pads", 0.008, 0.20)]
    for tag in ("foot", "head"):
        st += [ref(f"m-studs-{tag}", f"studs-{tag}", member="feet", part="member"), ref(f"j-nuts-{tag}", f"nuts-{tag}", member="feet", part="joint"), ref(f"j-locknuts-{tag}", f"locknuts-{tag}", member="feet", part="joint"), ref(f"m-caps-{tag}", f"caps-{tag}", member="feet", part="member")]
    st.append(ref("m-blanket", "blanket", member="blanket", part="member"))
    st += masses("", 0.012, 0.008)
    return st


def structure100():
    """🏗️ The 1:100 load path at the 15 mm feature."""
    st = [ref("m-rim-100", "rim-100", member="tub", part="member"), ref("m-wall-100", "wall-100", member="tub", part="member"), ref("m-bottom-100", "bottom-100", member="tub", part="member"),
          ref("m-rails-100", "rails-100", member="feet", part="member"), pads("j-pads-100", 0.016, 0.20)]
    st += [ref(f"m-feet-100-{t}", f"feet-100-{t}", member="feet", part="member") for t in ("foot", "head")]
    st.append(ref("m-blanket-100", "blanket-100", member="blanket", part="member"))
    st += masses("-100", 0.0095, 0.009)
    return st


def structure200():
    """🏗️ The 1:200 load path at the 30 mm feature."""
    st = [ref("m-rim-200", "rim-200", member="tub", part="member"), ref("m-wall-200", "wall-200", member="tub", part="member"), ref("m-bottom-200", "bottom-200", member="tub", part="member"),
          ref("m-feet-200", "feet-200", member="feet", part="member"), pads("j-pads-200", 0.030, 0.20), ref("m-blanket-200", "blanket-200", member="blanket", part="member")]
    st += masses("-200", 0.016, [0.03, 0.009, 0.03])
    return st


def structure500():
    """🏗️ The 1:500 load path at the 75 mm feature."""
    st = [ref("m-rim-500", "rim-500", member="tub", part="member"), ref("m-wall-500", "wall-500", member="tub", part="member"), ref("m-bottom-500", "bottom-500", member="tub", part="member"),
          ref("m-feet-500", "feet-500", member="feet", part="member"), ref("m-blanket-500", "blanket-500", member="blanket", part="member")]
    st += masses("-500", 0.04, [0.08, 0.009, 0.08])
    return st


def shell_refs(sfx, names):
    """🌡️ The enamelled sheet of the bath in its one region `shell`: both faces of rim, lip, wall and bottom exchange with the room, the white cover enamel inside and on the rim, the dark ground coat underneath."""
    return [ref(f"t-{n}{sfx}", f"{n}{sfx}", region="shell") for n in names]


def cap_pads(sfx, w, h):
    """🦶️ The PE caps of the coarse scales as small thermal shells of their own under the merged foot frames."""
    out = []
    for tag, X in (("foot", RAIL_X[0]), ("head", RAIL_X[1])):
        out.append(dict(box(f"t-caps{sfx}-{tag}", "feet", X - w / 2, X + w / 2, 0.0, h, FOOT_Z - w / 2, FOOT_Z + w / 2, "pe-hd-black", tone=0.2, repeat={"count": 2, "translation": [0, 0, -2 * FOOT_Z]}), region="caps", layer="cap-pe"))
    return out


def thermal50():
    """🌡️ The 1:50 thermal model: rim, lip, wall and bottom in the shell region, rails, studs and nuts in the foot set, the caps and the blanket."""
    th = shell_refs("", ["rim", "lip", "wall", "bottom"])
    th.append(ref("t-rails", "rails", region="feet"))
    for tag in ("foot", "head"):
        th += [ref(f"t-studs-{tag}", f"studs-{tag}", region="feet"), ref(f"t-nuts-{tag}", f"nuts-{tag}", region="feet"), ref(f"t-locknuts-{tag}", f"locknuts-{tag}", region="feet"), ref(f"t-caps-{tag}", f"caps-{tag}", region="caps")]
    th.append(ref("t-blanket", "blanket", region="blanket"))
    return th


def thermal100():
    """🌡️ The 1:100 thermal model."""
    th = shell_refs("-100", ["rim", "wall", "bottom"])
    th += [ref("t-rails-100", "rails-100", region="feet")]
    for tag, X in (("foot", RAIL_X[0]), ("head", RAIL_X[1])):
        th.append(dict(revolve_v(f"t-studs-100-{tag}", "feet", X, Y_RAIL0, FOOT_Z, [(0.0, 0.008), (Y_RAIL0 - Y_CAP, 0.008)], "galvanised-steel", 0.45, repeat={"count": 2, "translation": [0, 0, -2 * FOOT_Z]}), region="feet"))
        th.append(dict(revolve_v(f"t-caps-100-{tag}", "feet", X, Y_CAP, FOOT_Z, [(0.0, 0.018), (0.003, 0.020), (0.020, 0.020)], "pe-hd-black", 0.2, repeat={"count": 2, "translation": [0, 0, -2 * FOOT_Z]}), region="caps"))
    th.append(ref("t-blanket-100", "blanket-100", region="blanket"))
    return th


def thermal200():
    """🌡️ The 1:200 thermal model: the sheet in the shell region, the foot frames down to the caps, the caps as thin shells, the blanket."""
    th = shell_refs("-200", ["rim", "wall", "bottom"])
    f = pi_frame("t-feet-200", "feet", 0.030, 0.030, RAIL_W, "galvanised-steel", 0.5)
    vc = LV(Y_CAP)
    f["profile"] = [[min(a, vc), b] for a, b in f["profile"]]
    f["region"] = "feet"
    th.append(f)
    th += cap_pads("-200", 0.030, Y_CAP)
    th.append(ref("t-blanket-200", "blanket-200", region="blanket"))
    return th


def thermal500():
    """🌡️ The 1:500 thermal model."""
    th = shell_refs("-500", ["rim", "wall", "bottom"])
    f = pi_frame("t-feet-500", "feet", 0.09, 0.09, 0.09, "galvanised-steel", 0.5)
    vc = LV(Y_CAP)
    f["profile"] = [[min(a, vc), b] for a, b in f["profile"]]
    f["region"] = "feet"
    th.append(f)
    th += cap_pads("-500", 0.040, Y_CAP)
    th.append(ref("t-blanket-500", "blanket-500", region="blanket"))
    return th


def sh(id, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None, solid=None):
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
    if solid:
        s["solid"] = solid
    return s


def lpoly(pts, closed=True):
    """🔷️ A polygon from flat points in the (u, v) frame of the bones `tub` and `feet`."""
    return {"kind": "poly", "points": [list(L(X, Y)) for X, Y in pts], "closed": closed}


def lrect(X0, Y0, X1, Y1):
    """⬛️ A flat rectangle in the (u, v) frame of the bones `tub` and `feet`."""
    return lpoly([(X0, Y0), (X1, Y0), (X1, Y1), (X0, Y1)])


def lpath(rings):
    """〰️ A path of several flat rings in the (u, v) frame."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{L(X, Y)[0]} {L(X, Y)[1]}" for X, Y in ring) + " Z" for ring in rings)}


def lstroke(segs):
    """➖️ Open strokes from flat segments in the (u, v) frame."""
    return {"kind": "path", "d": " ".join(f"M {L(*a)[0]} {L(*a)[1]} L {L(*b)[0]} {L(*b)[1]}" for a, b in segs)}


def wrect(a0, b0, a1, b1):
    """⬜️ A front or plan rectangle in world rest coordinates."""
    return {"kind": "rect", "x": r6(a0), "y": r6(b0), "w": r6(a1 - a0), "h": r6(b1 - b0)}


def ext(z0, z1):
    """🧊️ The lateral sweep of a front or plan shape."""
    return {"kind": "extruded", "sweep": "w", "z0": r6(z0), "z1": r6(z1)}


def shell_outline():
    """🛁️ The side outline of wall and bottom below the lip in flat points."""
    xb = K_TAPER * (OPEN[0] + TH)
    return [(xo(Y_LIP), Y_LIP), (xb, Y_WB), (xb, Y_BO), (-xb, Y_BO), (-xb, Y_WB), (-xo(Y_LIP), Y_LIP)]


def band_poly(y_top, y_bot):
    """🪜️ The side outline of one horizontal band of the outer wall (the lowest band takes the bottom sheet in)."""
    yb = max(y_bot, Y_WB)
    if yb == y_bot:
        return [(xo(y_top), y_top), (xo(yb), yb), (-xo(yb), yb), (-xo(y_top), y_top)]
    return [(xo(y_top), y_top), (xo(yb), yb), (xo(yb), y_bot), (-xo(yb), y_bot), (-xo(yb), yb), (-xo(y_top), y_top)]


def cavity_poly():
    """🕳️ The side outline of the inside of the bath."""
    return [(xi(Y_WT), Y_WT), (xi(Y_WB), Y_WB), (-xi(Y_WB), Y_WB), (-xi(Y_WT), Y_WT)]


def cap_outline(X, prof):
    """🦶️ The side outline of a turned PE cap."""
    right = [(X + r, Y_CAP - a) for a, r in prof]
    left = [(X - r, Y_CAP - a) for a, r in reversed(prof)]
    return [(X, Y_CAP)] + right + left


def blanket_side():
    """🧺️ The side outline of the folded blanket in its own frame."""
    return {"kind": "poly", "points": [list(p) for p in BLANKET_PROFILE], "closed": True}


def overflow_strip(y0, y1, inset):
    """⭕️ The overflow or its chip as a thin strip lying on the inner face of the foot-end wall."""
    return [(xi(y1), y1), (xi(y0), y0), (xi(y0) - inset, y0), (xi(y1) - inset, y1)]


def solid_views(scale):
    """🧊️ Front and plan shapes of the bath as solids of its side profiles (identical definitions in both views), so the upright stored bath and the flat bath in use both read right."""
    bands = {50: [Y_LIP, 0.457, 0.356, 0.255, Y_BO], 100: [Y_LIP, 0.42, 0.29, Y_BO], 200: [Y_LIP, 0.37, Y_BO], 500: [Y_LIP, 0.35, Y_BO]}[scale]
    fl = OPEN[0] + (RIM[0] - OPEN[0]) * 0.0
    out = [sh("rim-side-l", "tub", "body", 6, True, 0.9, lrect(-RIM[0], Y_LIP, RIM[0], Y_RIM), region="shell", material="enamelled-steel", solid=ext(OPEN[1], RIM[1])),
           sh("rim-side-r", "tub", "body", 6, True, 0.9, lrect(-RIM[0], Y_LIP, RIM[0], Y_RIM), region="shell", material="enamelled-steel", solid=ext(-RIM[1], -OPEN[1])),
           sh("rim-end-foot", "tub", "body", 6, True, 0.9, lrect(fl, Y_LIP, RIM[0], Y_RIM), region="shell", material="enamelled-steel", solid=ext(-OPEN[1], OPEN[1])),
           sh("rim-end-head", "tub", "body", 6, True, 0.9, lrect(-RIM[0], Y_LIP, -fl, Y_RIM), region="shell", material="enamelled-steel", solid=ext(-OPEN[1], OPEN[1]))]
    for i, (ya, yb) in enumerate(zip(bands, bands[1:])):
        out.append(sh(f"wall-band-{i + 1}", "tub", "body", 2 + 0.1 * i, True, 0.3, lpoly(band_poly(ya, yb)), region="shell", material="enamelled-steel", solid=ext(-zo(ya), zo(ya))))
    cb = [Y_WT] + (bands[1:-1] if scale < 500 else []) + [Y_WB]
    for i, (ya, yb) in enumerate(zip(cb, cb[1:])):
        out.append(sh(f"inside-{i + 1}", "tub", "detail", 3 + 0.1 * i, True, 0.62 + 0.04 * i, lpoly([(xi(ya), ya), (xi(yb), yb), (-xi(yb), yb), (-xi(ya), ya)]), region="shell", material="enamel-white-aged", solid=ext(-zi(ya), zi(ya))))
    if scale <= 100:
        out.append(sh("floor", "tub", "detail", 4, True, 0.8, lrect(-xi(Y_WB), Y_WB, xi(Y_WB), Y_WB + 0.004), region="shell", material="enamel-white-aged", solid=ext(-zi(Y_WB), zi(Y_WB))))
        out.append(sh("waste-hole", "tub", "detail", 4.2, True, 0.05, lrect(DRAIN_X - DRAIN_R, Y_WB, DRAIN_X + DRAIN_R, Y_WB + 0.005), region="shell", material="enamel-white-aged", solid=ext(-DRAIN_R, DRAIN_R)))
    if scale <= 100:
        out.append(sh("chip", "tub", "detail", 4.4, True, 0.3, lpoly(overflow_strip(OVER_Y - 0.058, OVER_Y + 0.034, 0.004)), region="shell", material="rust-chip", solid=ext(-0.038, 0.034)))
        out.append(sh("overflow", "tub", "detail", 4.6, True, 0.05, lpoly(overflow_strip(OVER_Y - OVER_R, OVER_Y + OVER_R, 0.005)), region="shell", material="rust-chip", solid=ext(-OVER_R, OVER_R)))
    elif scale == 200:
        out.append(sh("overflow", "tub", "detail", 4.6, True, 0.1, lpoly(overflow_strip(OVER_Y - 0.052, OVER_Y + 0.040, 0.005)), region="shell", material="rust-chip", solid=ext(-0.04, 0.04)))
    if scale == 50:
        out.append(sh("silicone", "tub", "texture", 6.5, False, 0.4, lrect(-0.79, Y_RIM, 0.79, Y_RIM + 0.001), region="shell", solid=ext(0.345, 0.369)))
    for j, X in enumerate(RAIL_X):
        tag = ("foot", "head")[j]
        if scale <= 100:
            out.append(sh(f"rail-{tag}", "feet", "body", 1, True, 0.55, lrect(X - RAIL_W / 2, Y_RAIL0, X + RAIL_W / 2, Y_RAIL1), region="feet", material="galvanised-steel", solid=ext(-RAIL_Z, RAIL_Z)))
            for side, zc in (("l", FOOT_Z), ("r", -FOOT_Z)):
                rs = STUD_R if scale == 50 else 0.008
                out.append(sh(f"stud-{tag}-{side}", "feet", "body", 0.6, True, 0.5, lrect(X - rs, Y_CAP, X + rs, Y_RAIL0), region="feet", material="galvanised-steel", solid=ext(zc - rs, zc + rs)))
                out.append(sh(f"cap-{tag}-{side}", "feet", "body", 0.8, True, 0.2, lpoly(cap_outline(X, CAP_PROFILE)), region="caps", material="pe-hd-black", solid=ext(zc - 0.02, zc + 0.02)))
        else:
            band, leg, wid = (0.030, 0.030, RAIL_W) if scale == 200 else (0.09, 0.09, 0.09)
            out.append(sh(f"rail-{tag}", "feet", "body", 1, True, 0.5, lrect(X - wid / 2, Y_RAIL1 - band, X + wid / 2, Y_RAIL1), region="feet", material="galvanised-steel", solid=ext(-RAIL_Z, RAIL_Z)))
            for side, zc in (("l", FOOT_Z), ("r", -FOOT_Z)):
                out.append(sh(f"leg-{tag}-{side}", "feet", "body", 0.6, True, 0.45, lrect(X - wid / 2, 0.0, X + wid / 2, Y_RAIL1 - band), regions=["feet", "caps"], material="galvanised-steel", solid=ext(zc - leg / 2, zc + leg / 2)))
    return out


def blanket_views(scale):
    """🧺️ Front and plan of the blanket, which never moves, in world rest coordinates."""
    hatch = "felt" if scale == 50 else None
    return ([sh("blanket", "blanket", "body", 0.2, True, 0.45, wrect(-BLZ, 0, BLZ, BT), region="blanket", material="moving-blanket", hatch=hatch)],
            [sh("blanket", "blanket", "body", 0.2, True, 0.45, wrect(BLX0, -BLZ, BLX0 + BLL, BLZ), region="blanket", material="moving-blanket", hatch=hatch)])


def elevation(scale):
    """🖋️ The side elevation of one scale in the (u, v) frame of the bath."""
    el = []
    if scale == 50:
        el.append(sh("blanket", "blanket", "body", 0.2, True, 0.45, blanket_side(), region="blanket", material="moving-blanket", hatch="felt"))
        el.append(sh("blanket-layers", "blanket", "texture", 0.3, False, 0.3, {"kind": "path", "d": f"M 0.03 0.006 L {BLL - 0.01} 0.006 M 0.03 0.012 L {BLL - 0.01} 0.012"}, region="blanket"))
    else:
        el.append(sh("blanket", "blanket", "body", 0.2, True, 0.45, {"kind": "rect", "x": 0.0, "y": 0.0, "w": BLL, "h": BT}, region="blanket", material="moving-blanket"))
    for j, X in enumerate(RAIL_X):
        tag = ("foot", "head")[j]
        if scale <= 100:
            el.append(sh(f"rail-{tag}", "feet", "body", 1, True, 0.55, lrect(X - RAIL_W / 2, Y_RAIL0, X + RAIL_W / 2, Y_RAIL1), region="feet", material="galvanised-steel", hatch="zinc" if scale == 50 else None))
            rs = STUD_R if scale == 50 else 0.008
            el.append(sh(f"stud-{tag}", "feet", "body", 0.6, True, 0.5, lrect(X - rs, Y_CAP, X + rs, Y_RAIL0), region="feet", material="galvanised-steel"))
            if scale == 50:
                el.append(sh(f"nuts-{tag}", "feet", "body", 0.7, True, 0.45, lpath([[(X - NUT_R, Y_RAIL0 - NUT_H), (X + NUT_R, Y_RAIL0 - NUT_H), (X + NUT_R, Y_RAIL0), (X - NUT_R, Y_RAIL0)], [(X - NUT_R, Y_LOCK), (X + NUT_R, Y_LOCK), (X + NUT_R, Y_LOCK + NUT_H), (X - NUT_R, Y_LOCK + NUT_H)]]), region="feet", material="galvanised-steel"))
            el.append(sh(f"cap-{tag}", "feet", "body", 0.8, True, 0.2, lpoly(cap_outline(X, CAP_PROFILE)), region="caps", material="pe-hd-black"))
        else:
            band, wid = (0.030, RAIL_W) if scale == 200 else (0.09, 0.09)
            el.append(sh(f"foot-frame-{tag}", "feet", "body", 1, True, 0.5, lrect(X - wid / 2, 0.0, X + wid / 2, Y_RAIL1), regions=["feet", "caps"], material="galvanised-steel"))
    if scale <= 100:
        el.append(sh("shell", "tub", "body", 2, True, 0.3, lpoly(shell_outline()), region="shell", material="enamelled-steel", hatch="ground" if scale == 50 else None))
        el.append(sh("rim", "tub", "body", 3, True, 0.9, lrect(-RIM[0], Y_LIP, RIM[0], Y_RIM), region="shell", material="enamelled-steel"))
    else:
        out = [(-RIM[0], Y_RIM), (RIM[0], Y_RIM), (RIM[0], Y_LIP)] + shell_outline() + [(-RIM[0], Y_LIP)]
        el.append(sh("shell", "tub", "body", 2, True, 0.4, lpoly(out), region="shell", material="enamelled-steel"))
    if scale == 50:
        el.append(sh("lip-chip", "tub", "detail", 3.5, True, 0.25, lpoly([(-0.37, Y_RIM - 0.004), (-0.335, Y_RIM - 0.006), (-0.33, Y_LIP + 0.004), (-0.36, Y_LIP + 0.006)]), region="shell", material="rust-chip"))
        el.append(sh("dent", "tub", "texture", 3.6, False, 0.3, {"kind": "ellipse", "cx": L(-0.15, 0.33)[0], "cy": L(-0.15, 0.33)[1], "rx": 0.045, "ry": 0.022, "rotationDeg": 0}, region="shell"))
        el.append(sh("rust", "feet", "texture", 3.7, False, 0.3, lstroke([((X - 0.012, Y_RAIL0 + 0.003), (X - 0.004, Y_RAIL0 + 0.016)) for X in RAIL_X] + [((X + 0.006, Y_RAIL0 + 0.004), (X + 0.013, Y_RAIL0 + 0.014)) for X in RAIL_X]), region="feet"))
        el.append(sh("rim-line", "tub", "detail", 3.8, False, 0.3, lstroke([((-RIM[0] + 0.01, Y_RIM - TH), (RIM[0] - 0.01, Y_RIM - TH))]), region="shell"))
    return el


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 500: {"body": 0.2, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def views(scale):
    """🗂️ Elevation, front and plan of one scale."""
    bf, bp = blanket_views(scale)
    fr = bf + solid_views(scale)
    pl = bp + [dict(s) for s in solid_views(scale)]
    return {"elevation": elevation(scale), "front": fr, "plan": pl}


DEC = {
    50: dict(
        silhouette=T("Bath 34 x 34 mm upright on a 10 mm blanket: the 22 mm rim band with its lip, the deep-drawn wall tapering from 1.61 to 1.29 m with its sheet bottom, the two foot rails, four studs with welded and lock nuts and the domed PE caps; front view: the opening with the inside, the floor, the open waste hole and the chipped overflow; plan: rim band, the stepped wall, rails and caps.", "Wanne 34 x 34 mm aufrecht auf einer 10 mm breiten Decke: das 22-mm-Randband mit Abkantung, die tiefgezogene Wand, die sich von 1,61 auf 1,29 m verjüngt, mit Blechboden, die zwei Fußschienen, vier Gewindestangen mit Schweiß- und Kontermuttern und die gewölbten PE-Kappen; Vorderansicht: die Öffnung mit dem Inneren, dem Boden, dem offenen Ablaufloch und dem abgeplatzten Überlauf; Grundriss: Randband, die gestufte Wand, Schienen und Kappen."),
        retained=T("The traces of use: chipped enamel around the open overflow with rust in the chip, the waste hole without its fitting, the limescale line, old silicone along the wall side of the rim, a chip on the rim lip, a dent in the side and rust at the rail ends.", "Die Gebrauchsspuren: abgeplatztes Email um den offenen Überlauf mit Rost im Abplatzer, das Ablaufloch ohne Garnitur, der Kalkrand, alter Silikonrest entlang der Wandseite des Randes, ein Abplatzer an der Randabkantung, eine Delle in der Seite und Rost an den Schienenenden."),
        merged=T("Left and right studs and caps coincide in the side view; rim plate and lip are one band seen from the side.", "Linke und rechte Gewindestangen und Kappen fallen in der Seitenansicht zusammen; Randplatte und Abkantung sind von der Seite gesehen ein Band."),
        omitted=T("Threads, the hex flats of the nuts, the 6 mm fillet between wall and bottom and the bitumen pads under the rails (under 7.5 mm or hidden).", "Gewinde, die Sechskantflächen der Muttern, die 6-mm-Rundung zwischen Wand und Boden und die Bitumenpads unter den Schienen (unter 7,5 mm oder verdeckt)."),
        thinElements=T("The 2.3 mm sheet of rim and bottom keeps its gauge (0.05 mm, inked by the pens); studs 10 mm and nuts 8 mm are drawn true with the 0.5 mm pen.", "Das 2,3-mm-Blech von Rand und Boden behält seine Stärke (0,05 mm, vom Stift getragen); Gewindestangen 10 mm und Muttern 8 mm sind mit dem 0,5-mm-Stift wahr gezeichnet."),
        hatching=T("Stipple of the dark ground coat on the underside, zinc lines on the rails, felt dots on the blanket, all with the 0.13 mm pen.", "Punktierung der dunklen Grundemail an der Unterseite, Zinklinien auf den Schienen, Filzpunkte auf der Decke, alle mit dem 0,13-mm-Stift."),
        styles=T("White enamel rim 0.9, inside 0.62, floor 0.8, ground coat 0.3, rails 0.55, studs 0.5, caps 0.2, blanket 0.45, waste and overflow holes 0.05, chip 0.3.", "Weißes Email am Rand 0,9, Inneres 0,62, Boden 0,8, Grundemail 0,3, Schienen 0,55, Gewindestangen 0,5, Kappen 0,2, Decke 0,45, Ablauf- und Überlaufloch 0,05, Abplatzer 0,3."),
        overlays=T("Front and plan shapes are solids of the side profiles swept across their width, so the upright stored bath and the flat bath in use are both drawn from one definition; the inside, the floor and the holes lie over the shell and are hidden by it where the shell is nearer.", "Vorder- und Grundrissformen sind Körper der Seitenprofile, über ihre Breite gezogen, so werden die aufrecht gelagerte und die liegende Wanne aus einer Definition gezeichnet; Inneres, Boden und Löcher liegen über der Schale und werden von ihr verdeckt, wo die Schale näher ist."),
    ),
    100: dict(
        silhouette=T("Bath 17 x 17 mm upright on a 5 mm blanket: rim band, wall outline, the two rails, studs of 16 mm with their caps; front view: opening, floor, waste hole and chipped overflow.", "Wanne 17 x 17 mm aufrecht auf einer 5 mm breiten Decke: Randband, Wandumriss, die zwei Schienen, Gewindestangen von 16 mm mit Kappen; Vorderansicht: Öffnung, Boden, Ablaufloch und abgeplatzter Überlauf."),
        retained=T("The open waste hole, the chipped overflow and the dark underside against the white rim.", "Das offene Ablaufloch, der abgeplatzte Überlauf und die dunkle Unterseite gegen den weißen Rand."),
        merged=T("Rim plate and lip as one 22 mm band; stud and nuts as one 16 mm stud; three wall bands in front and plan.", "Randplatte und Abkantung als ein 22-mm-Band; Gewindestange und Muttern als eine 16-mm-Stange; drei Wandbänder in Vorderansicht und Grundriss."),
        omitted=T("Nuts, the limescale line, the silicone residue, the lip chip, the dent and the rust (under 0.15 mm).", "Muttern, Kalkrand, Silikonrest, Abplatzer an der Abkantung, Delle und Rost (unter 0,15 mm)."),
        thinElements=T("Bottom sheet 2.3 mm and rim band 22 mm are carried by the 0.35 mm pen.", "Bodenblech 2,3 mm und Randband 22 mm trägt der 0,35-mm-Stift."),
        hatching=T("None.", "Keine."),
        styles=T("Rim 0.9, inside 0.62, floor 0.8, ground coat 0.3, rails 0.55, studs 0.5, caps 0.2, blanket 0.45, holes 0.05.", "Rand 0,9, Inneres 0,62, Boden 0,8, Grundemail 0,3, Schienen 0,55, Gewindestangen 0,5, Kappen 0,2, Decke 0,45, Löcher 0,05."),
        overlays=T("As at 1:50, solids of the side profiles in front and plan.", "Wie bei 1:50, Körper der Seitenprofile in Vorderansicht und Grundriss."),
    ),
    200: dict(
        silhouette=T("Bath 8.5 x 8.5 mm upright: one outline of rim and shell, the two foot frames as solid bars; front view: rim, opening, floor, waste hole and the overflow as one dark spot.", "Wanne 8,5 x 8,5 mm aufrecht: ein Umriss aus Rand und Schale, die zwei Fußgestelle als volle Balken; Vorderansicht: Rand, Öffnung, Boden, Ablaufloch und der Überlauf als ein dunkler Fleck."),
        retained=T("The taper of the wall, the rim overhang, the foot frames and the open inside.", "Die Verjüngung der Wand, der Randüberstand, die Fußgestelle und das offene Innere."),
        merged=T("Rim and shell in one side outline (region shell); each rail with its studs and caps as one frame of 30 mm members; overflow and chip as one spot.", "Rand und Schale in einem Seitenumriss (Region Schale); jede Schiene mit Gewindestangen und Kappen als ein Gestell aus 30-mm-Gliedern; Überlauf und Abplatzer als ein Fleck."),
        omitted=T("Nuts, separate caps and every small trace of use.", "Muttern, getrennte Kappen und jede kleine Gebrauchsspur."),
        thinElements=T("Rim band 22 mm (0.11 mm) and bottom sheet are carried by the 0.25 mm pen.", "Randband 22 mm (0,11 mm) und Bodenblech trägt der 0,25-mm-Stift."),
        hatching=T("None.", "Keine."),
        styles=T("Shell 0.4, rim 0.9, inside 0.62, floor 0.8, frames 0.5, blanket 0.45.", "Schale 0,4, Rand 0,9, Inneres 0,62, Boden 0,8, Gestelle 0,5, Decke 0,45."),
        overlays=T("Solids of the side profiles in front and plan.", "Körper der Seitenprofile in Vorderansicht und Grundriss."),
    ),
    500: dict(
        silhouette=T("Bath 3.4 x 3.4 mm upright: the shell outline with its rim, two foot frames at the 75 mm feature, the opening in the front view.", "Wanne 3,4 x 3,4 mm aufrecht: der Schalenumriss mit Rand, zwei Fußgestelle beim Merkmal von 75 mm, die Öffnung in der Vorderansicht."),
        retained=T("The proportion of rim, tapering shell and feet, and the open inside.", "Das Verhältnis von Rand, sich verjüngender Schale und Füßen sowie das offene Innere."),
        merged=T("Rim and shell in one outline; each foot frame as one Π of 75 mm members; two wall bands in front and plan.", "Rand und Schale in einem Umriss; jedes Fußgestell als ein Π aus 75-mm-Gliedern; zwei Wandbänder in Vorderansicht und Grundriss."),
        omitted=T("Floor, waste and overflow holes and all traces of use (under 0.15 mm).", "Boden, Ablauf- und Überlaufloch und alle Gebrauchsspuren (unter 0,15 mm)."),
        thinElements=T("Rim band and the 18 mm blanket are carried by the 0.18 mm pen.", "Randband und die 18-mm-Decke trägt der 0,18-mm-Stift."),
        hatching=T("None.", "Keine."),
        styles=T("Shell 0.4, rim 0.9, inside 0.62, frames 0.5, blanket 0.45.", "Schale 0,4, Rand 0,9, Inneres 0,62, Gestelle 0,5, Decke 0,45."),
        overlays=T("Solids of the side profiles in front and plan.", "Körper der Seitenprofile in Vorderansicht und Grundriss."),
    ),
}

HATCHES50 = [{"id": "ground", "title": T("Dark ground coat enamel", "Dunkle Grundemail"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13},
             {"id": "zinc", "title": T("Galvanised rail", "Verzinkte Schiene"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.4, "penMm": 0.13},
             {"id": "felt", "title": T("Needle felt of the moving blanket", "Nadelfilz der Möbeldecke"), "pattern": "dots", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.13}]


def rep(scale):
    """🖋️ One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES50 if scale == 50 else [], "geometry": views(scale), "decisions": DEC[scale]}


def vol(den, body, thermal, structure, decisions):
    """🧊️ One authored scale volume."""
    return {"status": "authored", "denominator": den, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


VOLUMES = {
    "50": vol(50, B50, thermal50(), structure50(), [
        T("Body for the 7.5 mm feature: the 1.70 x 0.75 m rim plate of 2.3 mm enamelled sheet (a declared sheet) with its 22 mm lip, the deep-drawn wall as one tapered ring (the inside narrows from 1.61 x 0.66 m at the rim to 1.29 x 0.53 m at the floor, the sheet thinning towards the bottom as deep drawing thins it), the 2.3 mm bottom with the open 52 mm waste hole, two galvanised 40 x 20 x 2 mm rail tubes, four M10 studs with welded and lock nuts and the domed 40 mm PE caps, and the moving blanket folded to 0.50 x 1.00 m with its round fold.", "Körper für das 7,5-mm-Merkmal: die Randplatte 1,70 x 0,75 m aus 2,3-mm-emailliertem Blech (erklärte Platte) mit 22-mm-Abkantung, die tiefgezogene Wand als ein verjüngter Ring (das Innere verengt sich von 1,61 x 0,66 m am Rand auf 1,29 x 0,53 m am Boden, das Blech wird zum Boden hin dünner, wie Tiefziehen es ausdünnt), der 2,3-mm-Boden mit dem offenen 52-mm-Ablaufloch, zwei verzinkte Rechteckrohre 40 x 20 x 2 mm, vier M10-Gewindestangen mit Schweiß- und Kontermuttern und die gewölbten 40-mm-PE-Kappen sowie die auf 0,50 x 1,00 m gefaltete Möbeldecke mit runder Falte."),
        T("Traces of use as texture solids on the surfaces: the open overflow and the chipped enamel around it on the foot-end wall, the limescale line 0.23 m above the floor, the silicone residue on the wall side of the rim and the chip on the rim lip.", "Gebrauchsspuren als Texturkörper auf den Oberflächen: der offene Überlauf und das abgeplatzte Email darum an der Fußendwand, der Kalkrand 0,23 m über dem Boden, der Silikonrest auf der Wandseite des Randes und der Abplatzer an der Randabkantung."),
        T("Thermal: rim, lip, wall and bottom by reference in the region shell, whose both faces exchange with the room; rails, studs and nuts in the foot set, the caps and the blanket by reference. Structure: rim, lip, wall and bottom as the shell members of the bath, rails, studs and caps as the members of the foot set, adhesive pads and nuts as joints, three mass markers; the contacts of the poses carry it.", "Thermik: Rand, Abkantung, Wand und Boden über Verweise in der Region Schale, deren beide Seiten mit dem Raum austauschen; Schienen, Gewindestangen und Muttern im Fußgestell, Kappen und Decke über Verweise. Tragwerk: Rand, Abkantung, Wand und Boden als Schalenglieder der Wanne, Schienen, Gewindestangen und Kappen als Glieder des Fußgestells, Klebepads und Muttern als Gelenke, drei Massenmarken; die Kontakte der Posen tragen."),
    ]),
    "100": vol(100, body100(), thermal100(), structure100(), [
        T("Body for the 15 mm feature: rim plate and lip as one 22 mm band over the flange, the tapered wall and the bottom sheet with its waste hole as at 1:50, the rails as 40 x 20 x 2 mm tubes open at their ends, each foot as one turned 16 mm stud with its cap, the overflow and its chip as one texture spot and the blanket as a plain 18 mm sheet.", "Körper für das 15-mm-Merkmal: Randplatte und Abkantung als ein 22-mm-Band über dem Flansch, die verjüngte Wand und das Bodenblech mit Ablaufloch wie bei 1:50, die Schienen als an den Enden offene Rohre 40 x 20 x 2 mm, jeder Fuß als eine gedrehte 16-mm-Stange mit Kappe, Überlauf und Abplatzer als ein Texturfleck und die Decke als glatte 18-mm-Platte."),
        T("Merged against 1:50: nuts into the studs, limescale, silicone and lip chip left out. Thermal: rim band, wall and bottom in the shell region, studs and caps as their own turned solids, rails and blanket by reference. Structure: rim band, wall, bottom, rails and feet as members, the adhesive pads as joints, masses 19 mm.", "Gegenüber 1:50 zusammengefasst: Muttern in die Gewindestangen, Kalkrand, Silikon und Abplatzer an der Abkantung weggelassen. Thermik: Randband, Wand und Boden in der Region Schale, Gewindestangen und Kappen als eigene Drehkörper, Schienen und Decke über Verweise. Tragwerk: Randband, Wand, Boden, Schienen und Füße als Glieder, die Klebepads als Gelenke, Massen 19 mm."),
    ]),
    "200": vol(200, body200(), thermal200(), structure200(), [
        T("Body for the 30 mm feature: the 22 mm rim band as a declared sheet, the tapered wall, the bottom sheet with its waste hole, each rail with its two feet as one frame profile of 30 mm members over the 40 mm rail width, the overflow spot and the blanket sheet.", "Körper für das 30-mm-Merkmal: das 22-mm-Randband als erklärte Platte, die verjüngte Wand, das Bodenblech mit Ablaufloch, jede Schiene mit ihren zwei Füßen als ein Gestellprofil aus 30-mm-Gliedern über die 40-mm-Schienenbreite, der Überlauffleck und die Deckenplatte."),
        T("Thermal: rim band, wall and bottom in the shell region, the foot frames down to the caps and the caps as thin shells of their own; structure: rim band, wall, bottom and the frames as members, pads as joints, masses 32 mm.", "Thermik: Randband, Wand und Boden in der Region Schale, die Fußgestelle bis zu den Kappen und die Kappen als eigene dünne Schalen; Tragwerk: Randband, Wand, Boden und Gestelle als Glieder, Pads als Gelenke, Massen 32 mm."),
    ]),
    "500": vol(500, body500(), thermal500(), structure500(), [
        T("Body for the 75 mm feature: the rim band (a declared sheet), the tapered wall (its 0.41 m height is above the feature, so the vessel stays hollow), the plain bottom sheet without the 52 mm waste hole, each rail with its two feet as one frame of 75 mm members and the blanket sheet.", "Körper für das 75-mm-Merkmal: das Randband (erklärte Platte), die verjüngte Wand (ihre Höhe von 0,41 m liegt über dem Merkmal, so bleibt das Gefäß hohl), das glatte Bodenblech ohne das 52-mm-Ablaufloch, jede Schiene mit ihren zwei Füßen als ein Gestell aus 75-mm-Gliedern und die Deckenplatte."),
        T("Thermal: rim band, wall and bottom in the shell region, frames and the caps as thin shells; structure: rim band, wall, bottom and frames as members, masses 80 mm.", "Thermik: Randband, Wand und Boden in der Region Schale, Gestelle und die Kappen als dünne Schalen; Tragwerk: Randband, Wand, Boden und Gestelle als Glieder, Massen 80 mm."),
    ]),
}


def mat(id, en, de, eps, alpha, rho, k, c, E=None, f=None, source="literature", reference="iso-10456-2007"):
    """🧱️ One material."""
    m = {"id": id, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha}
    if rho is not None:
        m["densityKgM3"] = round(rho, 1)
    m.update({"conductivityWmK": k, "specificHeatJKgK": c})
    if E is not None:
        m["elasticModulusPa"] = E
    if f is not None:
        m["strengthPa"] = f
    m.update({"source": source, "reference": reference})
    return m


BLANKET_RHO = round(BLANKET_KG / volume_centre(B50[-1])[0], 1)
C_ES = round((STEEL_T * 7850 * 470 + (ENAMEL_T + GROUND_T) * 2500 * 840) / (TH * RHO_ES))
K_ES = round(TH / (STEEL_T / 50 + (ENAMEL_T + GROUND_T) / 1.0), 2)
MATERIALS = [
    mat("enamelled-steel", "Enamelled steel sheet, 2.0 mm DC04EK with enamel both sides", "Emailliertes Stahlblech, 2,0 mm DC04EK, beidseitig emailliert", 0.92, 0.3, RHO_ES, K_ES, C_ES, 2.1e11, 1.8e8, "literature", "en-10209-2013"),
    mat("enamel-white-aged", "White cover enamel, dulled by scouring and limescale", "Weiße Deckemail, durch Scheuern und Kalk stumpf geworden", 0.92, 0.32, 2500, 1.0, 840, None, None, "authored-estimate", "incropera-2018"),
    mat("galvanised-steel", "Hot-dip galvanised steel, weathered dull", "Feuerverzinkter Stahl, stumpf verwittert", 0.28, 0.65, 7850, 50, 470, 2.1e11, 2.35e8, "literature", "en-10346-2015"),
    mat("pe-hd-black", "Black polyethylene foot cap", "Schwarze Polyethylen-Fußkappe", 0.92, 0.93, 950, 0.45, 1900, 1.0e9, 2.5e7, "literature", "incropera-2018"),
    mat("rust-chip", "Chipped enamel with rust in the bare steel", "Abgeplatztes Email mit Rost im blanken Stahl", 0.85, 0.85, 5200, 30, 650, None, None, "authored-estimate", "iso-4532-1991"),
    mat("limescale", "Limescale deposit (calcium carbonate)", "Kalkablagerung (Calciumcarbonat)", 0.92, 0.45, 2700, 2.2, 840, None, None, "authored-estimate", "incropera-2018"),
    mat("silicone-residue", "Old sanitary silicone residue", "Alter Sanitärsilikonrest", 0.9, 0.4, 1100, 0.2, 1300, None, None, "authored-estimate", "incropera-2018"),
    mat("moving-blanket", "Quilted cotton-polyester moving blanket, faded blue", "Gesteppte Möbeldecke aus Baumwolle und Polyester, verblasstes Blau", 0.88, 0.75, BLANKET_RHO, 0.045, 1300, None, None, "authored-estimate", "moving-blanket-type"),
]


def layer(id, en, de, kind, material, t, R, C):
    """🧶️ One single layer of a region with its resistance and areal heat capacity."""
    return {"id": id, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": R, "heatCapacityJM2K": C, "source": "literature", "reference": "iso-10456-2007"}


HALF_STEEL_C = STEEL_T / 2 * 7850 * 470
AREA = json.loads((HERE / "areas.json").read_text()) if (HERE / "areas.json").exists() else {"shell": 5.27, "feet": 0.272, "caps": 0.018, "blanket": 1.054}


def region(id, en, de, material, geometry, L_, sky, lay, extra=None):
    """🔥️ One passive thermal region."""
    r = {"id": id, "title": T(en, de), "kind": "surface", "model": "passive", "material": material, "areaM2": AREA[id], "characteristicLengthM": L_, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        r.update(extra)
    return r


REGIONS = [
    region("shell", "Enamelled steel shell: white inside and rim, dark ground coat underneath", "Emaillierte Stahlschale: innen und am Rand weiß, unten dunkle Grundemail", "enamel-white-aged", "vertical-plate", 0.41, 0.4,
           layer("shell-sheet", "Steel sheet 2.0 mm with 0.3 mm of enamel", "Stahlblech 2,0 mm mit 0,3 mm Email", "metal", "enamel-white-aged", TH, round(STEEL_T / 50 + (ENAMEL_T + GROUND_T) / 1.0, 6), round(2 * HALF_STEEL_C + (ENAMEL_T + GROUND_T) * 2500 * 840)), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
    region("feet", "Galvanised foot rails, studs and nuts", "Verzinkte Fußschienen, Gewindestangen und Muttern", "galvanised-steel", "cylinder", 0.02, 0.2,
           layer("rail-steel", "Steel tube wall 2 mm", "Stahlrohrwand 2 mm", "metal", "galvanised-steel", 0.002, round(0.002 / 50, 6), round(0.002 * 7850 * 470))),
    region("caps", "PE foot caps", "PE-Fußkappen", "pe-hd-black", "horizontal-plate-down", 0.04, 0.1,
           layer("cap-pe", "Polyethylene cap 20 mm", "Polyethylenkappe 20 mm", "polymer", "pe-hd-black", 0.02, round(0.02 / 0.45, 5), round(0.01 * 950 * 1900))),
    region("blanket", "Folded moving blanket", "Gefaltete Möbeldecke", "moving-blanket", "horizontal-plate-up", 0.5, 0.6,
           layer("blanket-felt", "Six felt layers 18 mm", "Sechs Filzlagen 18 mm", "clothing", "moving-blanket", BT, round(BT / 0.045, 4), round(BLANKET_RHO * 1300 * min(BT, math.sqrt(0.045 / (BLANKET_RHO * 1300) * 3600))))),
]


def rot(a, u, v):
    """🔄️ A bone-local point turned by `a` degrees in the sagittal plane."""
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return (u * c - v * s, u * s + v * c)


ANCHORS = []
for side, z in (("l", 0.3), ("r", -0.3)):
    ANCHORS.append({"id": f"pad-{side}", "bone": "blanket", "x": r6(-BLX0), "y": 0.0, "z": z, "kind": "contact", "title": T(f"Blanket under the {'left' if side == 'l' else 'right'} half of the lip", f"Decke unter der {'linken' if side == 'l' else 'rechten'} Hälfte der Abkantung"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.05, "footprintWidthM": 0.3, "frictionCoefficient": 0.6})
for side, z in (("l", FOOT_Z), ("r", -FOOT_Z)):
    u, v = L(RAIL_X[1], 0.0)
    ANCHORS.append({"id": f"wall-{side}", "bone": "feet", "x": u, "y": v, "z": z, "kind": "contact", "title": T(f"Upper {'left' if side == 'l' else 'right'} foot cap against the depot wall", f"Obere {'linke' if side == 'l' else 'rechte'} Fußkappe an der Lagerwand"), "support": "contact", "normalDeg": 0, "footprintLengthM": 0.04, "footprintWidthM": 0.04, "frictionCoefficient": 0.4})
FEET = []
for tag, X in (("foot", RAIL_X[0]), ("head", RAIL_X[1])):
    for side, z in (("l", FOOT_Z), ("r", -FOOT_Z)):
        u, v = L(X, 0.0)
        FEET.append(f"cap-{tag}-{side}")
        ANCHORS.append({"id": f"cap-{tag}-{side}", "bone": "feet", "x": u, "y": v, "z": z, "kind": "contact", "title": T(f"{'Foot-end' if tag == 'foot' else 'Head-end'} {'left' if side == 'l' else 'right'} foot cap", f"{'Fußseitige' if tag == 'foot' else 'Kopfseitige'} {'linke' if side == 'l' else 'rechte'} Fußkappe"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.04, "frictionCoefficient": 0.5})
ANCHORS += [
    {"id": "grip-foot", "bone": "tub", "x": 0.0, "y": 0.0, "z": 0.0, "kind": "support", "title": T("Hands of the first carrier under the lip at the foot end", "Hände des ersten Trägers unter der Abkantung am Fußende"), "support": "pin"},
    {"id": "grip-head", "bone": "tub", "x": r6(2 * RIM[0]), "y": 0.0, "z": 0.0, "kind": "support", "title": T("Hands of the second carrier under the lip at the head end", "Hände des zweiten Trägers unter der Abkantung am Kopfende"), "support": "pin"},
    {"id": "water", "bone": "tub", "x": r6(XP), "y": LV(Y_WB), "z": 0.0, "kind": "load", "title": T("Bath water on the floor of the bath", "Badewasser auf dem Wannenboden"), },
    {"id": "bather-seat", "bone": "tub", "x": r6(XP + 0.30), "y": LV(Y_WB), "z": 0.0, "kind": "load", "title": T("Bather sitting on the floor of the bath", "Badende Person auf dem Wannenboden"), },
    {"id": "bather-back", "bone": "tub", "x": r6(XP + xi(0.40)), "y": LV(0.40), "z": 0.0, "kind": "load", "title": T("Bather's back against the sloped head end", "Rücken der badenden Person an der Schräge des Kopfendes"), },
]

BONES = [
    {"id": "ground", "parent": None, "title": T("Depot floor reference", "Bodenbezug im Lager"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "blanket", "parent": "ground", "title": T("Folded moving blanket (stays behind)", "Gefaltete Möbeldecke (bleibt zurück)"), "x": BLX0, "y": 0, "angleDeg": 0, "lengthM": BLL, "joint": {"type": "fixed"}, "mass": {"kg": round(MASS["blanket"]["kg"], 5), "comX": round(MASS["blanket"]["comX"], 5), "comY": round(MASS["blanket"]["comY"], 5), "distribution": "point", "source": "authored-estimate", "reference": "moving-blanket-type"}},
    {"id": "travel", "parent": "ground", "title": T("Carrying travel along the floor", "Tragweg über den Boden"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.5}},
    {"id": "lift", "parent": "travel", "title": T("Lift of the carrying hands", "Hub der tragenden Hände"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.9}},
    {"id": "tub", "parent": "lift", "title": T("Bath shell (pivot at the lower edge of the lip at the foot end)", "Wannenschale (Drehpunkt an der Unterkante der Abkantung am Fußende)"), "x": 0, "y": BT, "angleDeg": REST_DEG, "lengthM": r6(2 * RIM[0]), "joint": {"type": "revolute", "minDeg": 0, "maxDeg": FLAT_DEG}, "mass": {"kg": round(MASS["tub"]["kg"], 5), "comX": round(MASS["tub"]["comX"], 5), "comY": round(MASS["tub"]["comY"], 5), "distribution": "point", "source": "authored-estimate", "reference": "en-10209-2013"}},
    {"id": "feet", "parent": "tub", "title": T("Adjustable foot set glued under the bottom", "Verstellbares Fußgestell unter dem Boden verklebt"), "x": r6(XP - RAIL_X[0]), "y": LV(Y_RAIL1), "angleDeg": 0, "lengthM": r6(RAIL_X[0] - RAIL_X[1]), "joint": {"type": "fixed"}, "mass": {"kg": round(MASS["feet"]["kg"], 5), "comX": 0.0, "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "en-10346-2015"}},
]
FEET_ORIGIN = (XP - RAIL_X[0], LV(Y_RAIL1))


def to_feet(s):
    """🦶️ Re-expresses a solid or shape authored in the tub frame in the frame of the bone `feet` (same axes, origin at the top of the foot-end rail)."""
    du, dv = FEET_ORIGIN
    if "kind" in s and s.get("bone") == "feet":
        if s["kind"] == "revolve":
            s["origin"] = [r6(s["origin"][0] - du), r6(s["origin"][1] - dv), s["origin"][2]]
        elif s["kind"] in ("box", "ellipsoid"):
            t = s["transform"]["translation"]
            s["transform"]["translation"] = [r6(t[0] - du), r6(t[1] - dv), t[2]]
        elif s["kind"] == "extrude":
            if s["plane"] == "xy":
                s["profile"] = [[r6(a - du), r6(b - dv)] for a, b in s["profile"]]
                s["holes"] = [[[r6(a - du), r6(b - dv)] for a, b in h] for h in s.get("holes", [])] or s.pop("holes", None) or []
                if not s["holes"]:
                    s.pop("holes")
            elif s["plane"] == "yz":
                s["profile"] = [[r6(a - dv), b] for a, b in s["profile"]]
                s["from"], s["to"] = r6(s["from"] - du), r6(s["to"] - du)
        elif s["kind"] == "poly":
            s["points"] = [[r6(a - du), r6(b - dv)] for a, b in s["points"]]
        elif s["kind"] == "path":
            toks = s["d"].split()
            out, i = [], 0
            while i < len(toks):
                if toks[i] in ("M", "L"):
                    out += [toks[i], f"{r6(float(toks[i + 1]) - du)}", f"{r6(float(toks[i + 2]) - dv)}"]
                    i += 3
                else:
                    out.append(toks[i])
                    i += 1
            s["d"] = " ".join(out)
    return s


for scale_vol in VOLUMES.values():
    for lst in ("body", "thermal", "structure"):
        for s in scale_vol[lst]:
            to_feet(s)
for b in BONES:
    if b["id"] == "feet":
        b["mass"]["comX"] = r6(MASS["feet"]["comX"] - FEET_ORIGIN[0])
        b["mass"]["comY"] = r6(MASS["feet"]["comY"] - FEET_ORIGIN[1])
for a in ANCHORS:
    if a["bone"] == "feet":
        a["x"], a["y"] = r6(a["x"] - FEET_ORIGIN[0]), r6(a["y"] - FEET_ORIGIN[1])


def body_points(solids):
    """📌️ Sample points (u, v, w, bone) of the body solids for extents and clearances."""
    pts = []
    for s in solids:
        if s.get("texture"):
            continue
        b = s["bone"]
        du, dv = (FEET_ORIGIN if b == "feet" else (0, 0))
        reps = [(0, 0, 0)]
        if "repeat" in s:
            t = s["repeat"].get("translation", [0, 0, 0])
            reps = [tuple(t[i] * k for i in range(3)) for k in range(s["repeat"]["count"])]
        local = []
        if s["kind"] == "extrude":
            k0, k1 = s.get("taperTo", [1, 1])
            prof = s["profile"]
            cx = sum(p[0] for p in prof) / len(prof)
            cy = sum(p[1] for p in prof) / len(prof)
            for depth, kk in ((s["from"], k0), (s["to"], k1)):
                for a, bb in prof:
                    a2, b2 = cx + (a - cx) * kk, cy + (bb - cy) * kk
                    if s["plane"] == "zx":
                        local.append((b2, depth, a2))
                    elif s["plane"] == "xy":
                        local.append((a2, b2, depth))
                    else:
                        local.append((depth, a2, b2))
        elif s["kind"] == "revolve":
            o = s["origin"]
            for ax, rad in s["profile"]:
                for ang in range(0, 360, 45):
                    local.append((o[0] + rad * math.cos(math.radians(ang)), o[1] + ax, o[2] + rad * math.sin(math.radians(ang))))
        elif s["kind"] == "box":
            t = s["transform"]["translation"]
            a, bb, c = s["size"]
            for i in (-1, 1):
                for j in (-1, 1):
                    for k in (-1, 1):
                        local.append((t[0] + i * a / 2, t[1] + j * bb / 2, t[2] + k * c / 2))
        for (u, v, w) in local:
            for ru, rv, rw in reps:
                pts.append((u + ru + du, v + rv + dv, w + rw, b))
    return pts


P50 = body_points(B50)


def world(p, theta=0.0, travel=0.0, lift=0.0):
    """🌍️ A sample point in world metres for a tub offset `theta`, travel and lift."""
    u, v, w, b = p
    if b == "blanket":
        return (BLX0 + u, v, w)
    x, y = rot(REST_DEG + theta, u, v)
    return (travel + x, lift + BT + y, w)


def extents(theta=0.0, travel=0.0, lift=0.0, tub_only=False):
    """📦️ The world bounding box of the 1:50 body in one configuration."""
    W = [world(p, theta, travel, lift) for p in P50 if not (tub_only and p[3] == "blanket")]
    return [min(q[i] for q in W) for i in range(3)], [max(q[i] for q in W) for i in range(3)]


def lowest(theta, travel, lift):
    """⬇️ The lowest point of the bath (without the blanket) and whether it is over the blanket."""
    m = min((world(p, theta, travel, lift) for p in P50 if p[3] != "blanket"), key=lambda q: q[1])
    return m


LO, HI = extents()
LIFT_FLAT = r6(Y_LIP - BT)
CARRY_LIFT = r6(0.76 - BT)
TRAVEL_USE = 2.0
CARRY_TRAVEL = 1.0


def tip_lift(theta, margin=0.05):
    """🤸️ The lift that keeps the bath clear of floor and blanket while it tips by `theta` about its foot-end lip."""
    m = lowest(theta, 0.0, 0.0)
    need = BT + margin - m[1]
    return r6(max(0.0, need))


TIP1, TIP2 = 30.0, 60.0
LIFT1, LIFT2 = tip_lift(TIP1), tip_lift(TIP2)


def catmull(keys, t):
    """📈️ Catmull-Rom Hermite through (t, value) keys with zero end tangents, as the clip ease `smooth` reads them."""
    ts = [k[0] for k in keys]
    vs = [k[1] for k in keys]
    n = len(keys)
    for i in range(n - 1):
        if ts[i] <= t <= ts[i + 1]:
            break
    m = []
    for j in range(n):
        if j == 0 or j == n - 1:
            m.append(0.0)
        else:
            m.append((vs[j + 1] - vs[j - 1]) / (ts[j + 1] - ts[j - 1]))
    h = ts[i + 1] - ts[i]
    s = (t - ts[i]) / h
    h00, h10, h01, h11 = 2 * s ** 3 - 3 * s ** 2 + 1, s ** 3 - 2 * s ** 2 + s, -2 * s ** 3 + 3 * s ** 2, s ** 3 - s ** 2
    return h00 * vs[i] + h10 * h * m[i] + h01 * vs[i + 1] + h11 * h * m[i + 1]


CARRY_KEYS_T = [0.0, 0.9, 1.7, 2.4, 3.6]
CARRY_THETA = [0.0, TIP1, TIP2, FLAT_DEG, FLAT_DEG]
CARRY_LIFTS = [0.0, LIFT1, LIFT2, CARRY_LIFT, CARRY_LIFT]
CARRY_TRAV = [0.0, 0.0, 0.0, 0.25, CARRY_TRAVEL]


def carry_clearance():
    """🔍️ The smallest height of the bath above the floor (and above the blanket over it) along the sampled carry clip."""
    worst = (9, 0)
    for i in range(0, 361):
        t = 3.6 * i / 360
        th = catmull(list(zip(CARRY_KEYS_T, CARRY_THETA)), t)
        lf = catmull(list(zip(CARRY_KEYS_T, CARRY_LIFTS)), t)
        tr = catmull(list(zip(CARRY_KEYS_T, CARRY_TRAV)), t)
        for p in P50:
            if p[3] == "blanket":
                continue
            x, y, z = world(p, th, tr, lf)
            floor = BT if (BLX0 - 0.01 <= x <= BLX0 + BLL + 0.01 and abs(z) <= BLZ + 0.01) else 0.0
            if i > 3 and y - floor < worst[0]:
                worst = (y - floor, t)
    return worst


SET_KEYS_T = [0.0, 2.0, 3.0]
SET_LIFTS = [CARRY_LIFT, r6(LIFT_FLAT + 0.01), LIFT_FLAT]
SET_TRAV = [CARRY_TRAVEL, TRAVEL_USE, TRAVEL_USE]


def regions_binding(enamel=None, wet=0.0, blanket=0.45, caps=None):
    """🌬️ The thermal binding of every region of a pose."""
    e = {"exposedFraction": 1, "sunlitFraction": 0, "wettedness": wet}
    if enamel:
        e["contact"] = {"fraction": enamel[0], "tempC": enamel[1]}
    c = {"exposedFraction": 1, "sunlitFraction": 0}
    if caps:
        c["contact"] = {"fraction": caps, "tempC": 21}
    return {"regions": {"shell": e, "feet": {"exposedFraction": 1, "sunlitFraction": 0}, "caps": c, "blanket": {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": blanket, "tempC": 18}}}, "emitters": {}}


FLOOR_UNDER_BLANKET = {"id": "floor-under-blanket", "kind": "imposed", "title": T("Floor reaction under the blanket, which lies on its own", "Bodenreaktion unter der Decke, die für sich liegt"), "bone": "blanket", "x": round(MASS["blanket"]["comX"], 5), "y": round(MASS["blanket"]["comY"], 5), "fxN": 0, "fyN": round(WB, 3), "source": "literature", "reference": "en-1991-1-1"}
USE_LOADS = [
    {"id": "water-load", "kind": "imposed", "title": T(f"Bath water, {WATER_KG:.0f} l at 38 degC", f"Badewasser, {WATER_KG:.0f} l bei 38 degC"), "anchor": "water", "fxN": 0, "fyN": round(-WATER_KG * G, 3), "source": "authored-estimate", "reference": "en-1991-1-1"},
    {"id": "bather-seat-load", "kind": "occupant", "title": T("Bather on the floor of the bath, 60 kg", "Badende Person auf dem Wannenboden, 60 kg"), "anchor": "bather-seat", "fxN": 0, "fyN": round(-60 * G, 3), "source": "authored-estimate", "reference": "din-33402-2-2020"},
    {"id": "bather-back-load", "kind": "occupant", "title": T("Bather's back on the head end, 20 kg", "Rücken der badenden Person am Kopfende, 20 kg"), "anchor": "bather-back", "fxN": 0, "fyN": round(-20 * G, 3), "source": "authored-estimate", "reference": "din-33402-2-2020"},
]
CAPS_CONTACT = 0.33

POSES = [
    {"id": "stored", "title": T("Stored upright on its foot end", "Hochkant auf dem Fußende gelagert"), "description": T("Emptied and cleaned, waste fitting removed, standing upright on the lip of its foot end on a folded moving blanket in the reuse depot and leaning back 10 degrees with its two upper foot caps against the depot wall, the opening facing the aisle.", "Geleert und gereinigt, Ablaufgarnitur ausgebaut, hochkant auf der Abkantung des Fußendes auf einer gefalteten Möbeldecke im Wiederverwendungslager stehend und um 10 Grad zurückgelehnt, die zwei oberen Fußkappen an der Lagerwand, die Öffnung zum Gang."), "bones": {}, "ground": "authored", "contacts": ["pad-l", "pad-r", "wall-l", "wall-r"], "thermal": regions_binding(), "loads": [FLOOR_UNDER_BLANKET]},
    {"id": "carried", "title": T("Carried flat by two people", "Liegend von zwei Personen getragen"), "description": T("Two carriers hold the lip at both ends with their hands at 0.76 m, the bath level with its rim up and its caps 0.2 m above the floor; the blanket stays behind.", "Zwei Träger halten die Abkantung an beiden Enden mit den Händen auf 0,76 m, die Wanne waagerecht mit dem Rand nach oben und den Kappen 0,2 m über dem Boden; die Decke bleibt zurück."), "bones": {"travel": {"dx": CARRY_TRAVEL}, "lift": {"dy": CARRY_LIFT}, "tub": {"angleDeg": FLAT_DEG}}, "ground": "authored", "contacts": ["grip-foot", "grip-head"], "thermal": regions_binding(), "loads": [FLOOR_UNDER_BLANKET]},
    {"id": "in-use", "title": T("Filled, with a bather", "Gefüllt, mit badender Person"), "description": T(f"Back on its four feet in a bathroom, rim at 0.58 m, filled with {WATER_KG:.0f} l of water at 38 degC and a bather of 80 kg leaning against the sloped head end.", f"Wieder auf seinen vier Füßen in einem Bad, Rand auf 0,58 m, gefüllt mit {WATER_KG:.0f} l Wasser von 38 degC und einer badenden Person von 80 kg, an die Schräge des Kopfendes gelehnt."), "bones": {"travel": {"dx": TRAVEL_USE}, "lift": {"dy": LIFT_FLAT}, "tub": {"angleDeg": FLAT_DEG}}, "ground": "authored", "contacts": FEET, "thermal": regions_binding((0.33, 38.0), 1.0, caps=CAPS_CONTACT), "loads": [FLOOR_UNDER_BLANKET] + USE_LOADS},
    {"id": "installed-empty", "title": T("Installed, empty", "Aufgestellt, leer"), "description": T("Standing on its four feet in the bathroom, levelled, dry and empty.", "Auf seinen vier Füßen im Bad stehend, ausgerichtet, trocken und leer."), "bones": {"travel": {"dx": TRAVEL_USE}, "lift": {"dy": LIFT_FLAT}, "tub": {"angleDeg": FLAT_DEG}}, "ground": "authored", "contacts": FEET, "thermal": regions_binding(caps=CAPS_CONTACT), "loads": [FLOOR_UNDER_BLANKET]},
    {"id": "tip-key", "key": True, "title": T("Tipping down", "Abkippen"), "description": T(f"The carriers tip the bath {TIP1:.0f} degrees down from upright while lifting its foot end off the blanket.", f"Die Träger kippen die Wanne um {TIP1:.0f} Grad aus der Senkrechten und heben dabei das Fußende von der Decke."), "bones": {"lift": {"dy": LIFT1}, "tub": {"angleDeg": TIP1}}, "ground": "authored", "contacts": ["grip-foot", "grip-head"], "thermal": regions_binding(), "loads": [FLOOR_UNDER_BLANKET]},
    {"id": "tilt-key", "key": True, "title": T("Nearly level", "Fast waagerecht"), "description": T(f"Tipped {TIP2:.0f} degrees and lifted, the head end coming down.", f"Um {TIP2:.0f} Grad gekippt und angehoben, das Kopfende senkt sich."), "bones": {"lift": {"dy": LIFT2}, "tub": {"angleDeg": TIP2}}, "ground": "authored", "contacts": ["grip-foot", "grip-head"], "thermal": regions_binding(), "loads": [FLOOR_UNDER_BLANKET]},
    {"id": "level-key", "key": True, "title": T("Level and lifted", "Waagerecht und angehoben"), "description": T("Level at carrying height, the first steps away from the blanket.", "Waagerecht in Tragehöhe, die ersten Schritte weg von der Decke."), "bones": {"travel": {"dx": 0.25}, "lift": {"dy": CARRY_LIFT}, "tub": {"angleDeg": FLAT_DEG}}, "ground": "authored", "contacts": ["grip-foot", "grip-head"], "thermal": regions_binding(), "loads": [FLOOR_UNDER_BLANKET]},
    {"id": "set-down-key", "key": True, "title": T("Lowering in the bathroom", "Absenken im Bad"), "description": T("Carried into place and lowered until the caps are 10 mm above the floor.", "An den Platz getragen und abgesenkt, bis die Kappen 10 mm über dem Boden sind."), "bones": {"travel": {"dx": TRAVEL_USE}, "lift": {"dy": SET_LIFTS[1]}, "tub": {"angleDeg": FLAT_DEG}}, "ground": "authored", "contacts": ["grip-foot", "grip-head"], "thermal": regions_binding(), "loads": [FLOOR_UNDER_BLANKET]},
]

CLIPS = [
    {"id": "carry", "title": T("Tip it down and carry it off", "Abkippen und wegtragen"), "description": T("Two carriers take the lip at both ends, tip the upright bath down while lifting its foot end off the blanket, bring it level at hand height and carry it away; the blanket stays on the depot floor.", "Zwei Träger fassen die Abkantung an beiden Enden, kippen die aufrechte Wanne ab und heben dabei das Fußende von der Decke, bringen sie in Handhöhe waagerecht und tragen sie fort; die Decke bleibt auf dem Lagerboden."), "durationS": 3.6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.9, "pose": "tip-key", "ease": "smooth"}, {"t": 1.7, "pose": "tilt-key", "ease": "smooth"}, {"t": 2.4, "pose": "level-key", "ease": "smooth"}, {"t": 3.6, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": "pad-l", "fromS": 0, "toS": 0.2}, {"anchor": "pad-r", "fromS": 0, "toS": 0.2}, {"anchor": "wall-l", "fromS": 0, "toS": 0.1}, {"anchor": "wall-r", "fromS": 0, "toS": 0.1}, {"anchor": "grip-foot", "fromS": 0, "toS": 3.6}, {"anchor": "grip-head", "fromS": 0, "toS": 3.6}],
     "phases": [{"id": "tip", "fromS": 0, "toS": 1.7, "title": T("Take hold and tip down", "Fassen und abkippen")}, {"id": "level", "fromS": 1.7, "toS": 2.4, "title": T("Bring level", "Waagerecht bringen")}, {"id": "carry", "fromS": 2.4, "toS": 3.6, "title": T("Carry", "Tragen")}],
     "events": [{"t": 0.1, "kind": "contact-off", "title": T("Caps leave the wall", "Kappen verlassen die Wand")}, {"t": 0.2, "kind": "contact-off", "title": T("Lip leaves the blanket", "Abkantung verlässt die Decke")}, {"t": 2.4, "kind": "motion", "title": T("Level at hand height", "Waagerecht in Handhöhe")}], "tracks": []},
    {"id": "bath", "title": T("Run a bath, soak and drain", "Bad einlassen, baden und ablassen"), "description": T(f"In use: the bath fills with {WATER_KG:.0f} l of water at 38 degC, the bather lies in it, then the water drains and the enamel dries.", f"In Gebrauch: die Wanne füllt sich mit {WATER_KG:.0f} l Wasser von 38 degC, die Person liegt darin, dann läuft das Wasser ab und das Email trocknet."), "durationS": 12.0, "fps": 25, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "installed-empty", "ease": "smooth"}, {"t": 4.0, "pose": "in-use", "ease": "smooth"}, {"t": 8.0, "pose": "in-use", "ease": "smooth"}, {"t": 12.0, "pose": "installed-empty", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 12.0} for a in FEET],
     "phases": [{"id": "fill", "fromS": 0, "toS": 4.0, "title": T("Fill and get in", "Füllen und einsteigen")}, {"id": "soak", "fromS": 4.0, "toS": 8.0, "title": T("Soak", "Baden")}, {"id": "drain", "fromS": 8.0, "toS": 12.0, "title": T("Get out and drain", "Aussteigen und ablassen")}],
     "events": [{"t": 4.0, "kind": "motion", "title": T("Filled to the overflow line", "Bis zur Überlauflinie gefüllt")}, {"t": 12.0, "kind": "motion", "title": T("Drained and dry", "Abgelassen und trocken")}],
     "tracks": [{"target": "region:shell:wettedness", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 4.0, "v": 1}, {"t": 9.0, "v": 1}, {"t": 12.0, "v": 0}]}]},
    {"id": "set-down", "title": T("Set it down in the bathroom", "Im Bad absetzen"), "description": T("Carried on into the bathroom, lowered level onto its four feet and let go.", "Weiter ins Bad getragen, waagerecht auf seine vier Füße abgesenkt und losgelassen."), "durationS": 3.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "carried", "ease": "smooth"}, {"t": 2.0, "pose": "set-down-key", "ease": "smooth"}, {"t": 3.0, "pose": "installed-empty", "ease": "smooth"}],
     "contacts": [{"anchor": "grip-foot", "fromS": 0, "toS": 2.9}, {"anchor": "grip-head", "fromS": 0, "toS": 2.9}] + [{"anchor": a, "fromS": 2.6, "toS": 3.0} for a in FEET],
     "phases": [{"id": "walk", "fromS": 0, "toS": 2.0, "title": T("Walk in", "Hineintragen")}, {"id": "lower", "fromS": 2.0, "toS": 3.0, "title": T("Lower and let go", "Absenken und loslassen")}],
     "events": [{"t": 2.6, "kind": "contact-on", "title": T("Caps touch the floor", "Kappen berühren den Boden")}, {"t": 2.9, "kind": "contact-off", "title": T("Hands let go", "Hände lassen los")}], "tracks": []},
]


def world_poly(pts):
    """🌍️ Flat points of the stored bath as world points (x, y) for the symbol."""
    out = []
    for X, Y in pts:
        u, v = L(X, Y)
        x, y = rot(REST_DEG, u, v)
        out.append([r6(x), r6(y + BT)])
    return out


SYM_OUT = [(-RIM[0], Y_RIM), (RIM[0], Y_RIM), (RIM[0], Y_LIP)] + shell_outline() + [(-RIM[0], Y_LIP)]
SYMBOL = {"id": "used-bathtub-symbol", "title": T("Bath upright on its blanket, true extent", "Wanne hochkant auf ihrer Decke, wahre Ausdehnung"), "description": T("The stored bath in all three views: 1.70 x 0.75 m standing on its foot end and leaning back 10 degrees, rim, tapering shell and foot frames, on the folded 0.50 x 1.00 m blanket.", "Die gelagerte Wanne in allen drei Ansichten: 1,70 x 0,75 m auf dem Fußende stehend und 10 Grad zurückgelehnt, Rand, sich verjüngende Schale und Fußgestelle, auf der gefalteten Decke 0,50 x 1,00 m."),
          "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100, 50], "pens": PENS[200], "hatches": [],
          "geometry": {
              "elevation": [sh("blanket", "ground", "body", 1, True, 0.45, wrect(BLX0, 0, BLX0 + BLL, BT)), sh("bath", "ground", "body", 2, True, 0.8, {"kind": "poly", "points": world_poly(SYM_OUT), "closed": True})] + [sh(f"feet-{i}", "ground", "body", 3, True, 0.5, {"kind": "poly", "points": world_poly([(X - 0.02, 0.0), (X + 0.02, 0.0), (X + 0.02, Y_RAIL1), (X - 0.02, Y_RAIL1)]), "closed": True}) for i, X in enumerate(RAIL_X)],
              "front": [sh("blanket", "ground", "body", 1, True, 0.45, wrect(-BLZ, 0, BLZ, BT)), sh("bath", "ground", "body", 2, True, 0.8, wrect(-RIM[1], BT, RIM[1], r6(HI[1]))), sh("opening", "ground", "detail", 3, True, 0.6, wrect(-OPEN[1], r6(BT + 0.045 * math.sin(math.radians(80))), OPEN[1], r6(HI[1] - 0.045)))],
              "plan": [sh("blanket", "ground", "body", 1, True, 0.45, wrect(BLX0, -BLZ, BLX0 + BLL, BLZ)), sh("bath", "ground", "body", 2, True, 0.8, wrect(r6(LO[0]), -RIM[1], 0.0217, RIM[1]))],
          },
          "dimensionalNote": T("Model metres at real size in the frame of the asset (x ahead to the opening of the stored bath, lateral positive to the left, y up): the side view is seen from the right, the front view from ahead, the top view from above; all three show the stored pose.", "Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn zur Öffnung der gelagerten Wanne, seitlich positiv nach links, y nach oben): die Seitenansicht blickt von rechts, die Vorderansicht von vorn, die Draufsicht von oben; alle drei zeigen die gelagerte Pose.")}

REFERENCES = [
    {"id": "en-14516-2015", "citation": "EN 14516:2015+A1:2020 Baths for domestic purposes (classification, dimensional tolerances, load resistance, durability and cleanability of baths of every material, enamelled steel included)."},
    {"id": "en-232-2003", "citation": "EN 232:2003 Baths - Connecting dimensions (overall sizes of rectangular baths such as 1700 x 750 mm, the 52 mm holes of the waste outlet and the overflow)."},
    {"id": "en-10209-2013", "citation": "EN 10209:2013 Cold rolled low carbon steel flat products for vitreous enamelling (grades DC04EK and DC06EK for deep-drawn enamelled parts; yield strength of DC04EK at most 220 N/mm2, taken as 180 N/mm2)."},
    {"id": "iso-4532-1991", "citation": "ISO 4532:1991 Vitreous and porcelain enamels - Determination of the resistance of enamelled articles to impact (the chipping of enamel by impact, as at the overflow and the rim lip)."},
    {"id": "en-10346-2015", "citation": "EN 10346:2015 Continuously hot-dip coated steel flat products for cold forming (zinc-coated steel of the foot rails; steel S235: 235 N/mm2, 210 kN/mm2)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel 7800 kg/m3, 50 W/mK, 450 J/kgK; glass 2500 kg/m3, 1.0 W/mK, 750-840 J/kgK; polyethylene, textiles)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (total emissivity of porcelain enamel and glazes 0.90-0.93, of weathered galvanised steel about 0.25-0.3, of polyethylene 0.9; solar absorptance of white and dark painted and enamelled surfaces)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (steel 77-78.5 kN/m3, water 10 kN/m3; self-weight as volume x density)."},
    {"id": "din-18011-2020", "citation": "DIN 18011:2020-04 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (placing area of a bath 1.70 x 0.75 m and the movement area in front of it)."},
    {"id": "neufert-2019", "citation": "Neufert, E., Kister, J. (2019). Bauentwurfslehre, 42nd ed., Springer Vieweg (bathrooms: baths 1.60-1.80 x 0.70-0.80 m, rim height 0.55-0.60 m above the floor, inner depth about 0.40 m)."},
    {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte (grip axis of the hanging arm of standing adults about 0.72-0.78 m, taken as 0.76 m for the carrying hands; body sizes of an adult bather)."},
    {"id": "steel-bath-type", "citation": "Type data of West German enamelled steel baths of 1975-1995 as sanitary wholesale catalogues offered them: rectangular 170 x 75 cm, deep-drawn from 2.0-3.5 mm enamelling steel, 400-420 mm deep, both ends sloped, waste at the foot end, 25-50 kg, about 180-240 l to the overflow, on a glued foot set of two galvanised rails with four M10 or M8 adjustable studs and PE caps (rim 0.55-0.60 m); no single product, no brand."},
    {"id": "moving-blanket-type", "citation": "Type data of removal blankets (Möbeldecken, Umzugsdecken) as removal-equipment suppliers offer them: quilted needle felt of recycled cotton and polyester, 150 x 200 cm, 400-600 g/m2 (1.2-2.2 kg), about 3 mm thick per layer; no single product, no brand."},
]

CAPACITY_L = 1000 * rr_area(*OPEN) * H_W / (1 - K_TAPER) * (s_of(OVER_Y) ** 3 - K_TAPER ** 3) / 3

ASSET = {
    "id": "used-bathtub", "category": "objects",
    "title": T("Used Enamelled Steel Bathtub", "Gebrauchte Stahlemaille-Badewanne"),
    "summary": T(f"A 1.70 x 0.75 m enamelled steel bath of the 1980s, deep-drawn from 2 mm sheet, 0.41 m deep, {TUB_KG:.1f} kg with its adjustable foot set, enamel chipped around the open overflow, waste fitting removed, limescale line and old silicone on the rim; stored upright on its foot end on a moving blanket against the depot wall, tipped down and carried flat by two people, back in a bathroom filled with {WATER_KG:.0f} l of warm water and a bather.", f"Eine Stahlemaille-Badewanne der 1980er-Jahre, 1,70 x 0,75 m, aus 2-mm-Blech tiefgezogen, 0,41 m tief, {TUB_KG:.1f} kg mit verstellbarem Fußgestell, Email um den offenen Überlauf abgeplatzt, Ablaufgarnitur ausgebaut, Kalkrand und alter Silikonrest auf dem Rand; hochkant auf dem Fußende auf einer Möbeldecke an der Lagerwand gelagert, abgekippt und liegend von zwei Personen getragen, wieder in einem Bad gefüllt mit {WATER_KG:.0f} l warmem Wasser und einer badenden Person."),
    "tags": ["used", "household", "second-hand", "bathroom", "bathtub", "bath", "enamel", "steel", "sanitary", "1980s", "interior"],
    "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                   "notes": T(f"West German rectangular bath of the 1980s type, taken out at a flat clearance with its foot set still glued on. Shell deep-drawn from 2.0 mm DC04EK enamelling steel with 0.2 mm white cover enamel inside and 0.1 mm dark ground coat outside (2.3 mm), rim 1.70 x 0.75 m with a 45 mm flange and a 22 mm lip, inside 1.61 x 0.66 m at the rim narrowing to 1.29 x 0.53 m at the floor, 0.41 m deep, both ends sloped {FOOT_SLOPE:.0f} degrees and the long sides {SIDE_SLOPE:.0f} degrees; 52 mm waste hole at the foot end and 52 mm overflow 0.11 m below the rim, both fittings removed; about {CAPACITY_L:.0f} l to the overflow. Foot set of two galvanised 40 x 20 x 2 mm rail tubes 0.56 m long, four M10 studs with welded and lock nuts and domed 40 mm PE caps, rim at 0.58 m. Traces of use: enamel chipped around the overflow with rust in the chip, a limescale line, old silicone on the wall side of the rim, a chip on the lip, a dent in the side, rust at the rail ends; nothing that stops it holding water. Stored upright on its foot end on a 150 x 200 cm moving blanket folded to 0.50 x 1.00 m, which stays behind. Masses from the modelled 1:50 volumes x density: shell {MASS['tub']['kg']:.2f} kg, foot set {MASS['feet']['kg']:.2f} kg, blanket {MASS['blanket']['kg']:.2f} kg.", f"Westdeutsche Rechteckwanne vom Typ der 1980er-Jahre, bei einer Haushaltsauflösung mit noch angeklebtem Fußgestell ausgebaut. Schale aus 2,0-mm-Emaillierstahl DC04EK tiefgezogen, innen 0,2 mm weiße Deckemail, außen 0,1 mm dunkle Grundemail (2,3 mm), Rand 1,70 x 0,75 m mit 45-mm-Flansch und 22-mm-Abkantung, innen 1,61 x 0,66 m am Rand, zum Boden auf 1,29 x 0,53 m verjüngt, 0,41 m tief, beide Enden um {FOOT_SLOPE:.0f} Grad und die Längsseiten um {SIDE_SLOPE:.0f} Grad geneigt; 52-mm-Ablaufloch am Fußende und 52-mm-Überlauf 0,11 m unter dem Rand, beide Garnituren ausgebaut; etwa {CAPACITY_L:.0f} l bis zum Überlauf. Fußgestell aus zwei verzinkten Rechteckrohren 40 x 20 x 2 mm, 0,56 m lang, vier M10-Gewindestangen mit Schweiß- und Kontermuttern und gewölbten 40-mm-PE-Kappen, Rand auf 0,58 m. Gebrauchsspuren: Email um den Überlauf abgeplatzt mit Rost im Abplatzer, ein Kalkrand, alter Silikonrest auf der Wandseite des Randes, ein Abplatzer an der Abkantung, eine Delle in der Seite, Rost an den Schienenenden; nichts, was das Wasserhalten hindert. Hochkant auf dem Fußende auf einer Möbeldecke 150 x 200 cm gelagert, auf 0,50 x 1,00 m gefaltet, die zurückbleibt. Massen aus den 1:50-Volumen x Dichte: Schale {MASS['tub']['kg']:.2f} kg, Fußgestell {MASS['feet']['kg']:.2f} kg, Decke {MASS['blanket']['kg']:.2f} kg.")},
    "dimensions": {"heightM": round(HI[1] - LO[1], 4), "widthM": round(HI[0] - LO[0], 4), "depthM": round(HI[2] - LO[2], 4), "massKg": round(TOTAL_KG, 5)},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "ground", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {"50": rep(50), "100": rep(100), "200": rep(200), "500": rep(500)},
    "symbols": [SYMBOL],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.55, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 300, "diffuseFraction": 0.2, "pressurePa": 101325}, "operating": [], "regions": REGIONS, "emitters": [], "passengers": [],
                "assumptions": [
                    T("The 2.3 mm sheet is 2.0 mm steel (7850 kg/m3, 50 W/mK, 470 J/kgK) with 0.3 mm of enamel (2500 kg/m3, 1.0 W/mK, 840 J/kgK) after ISO 10456, one thin layer of tiny resistance and an areal heat capacity of about 8 kJ/m2K: the shell follows the water within seconds. Its area counts both faces, the inside and the underside, which both exchange with the room.", "Das 2,3-mm-Blech ist 2,0 mm Stahl (7850 kg/m3, 50 W/mK, 470 J/kgK) mit 0,3 mm Email (2500 kg/m3, 1,0 W/mK, 840 J/kgK) nach ISO 10456, eine dünne Schicht mit winzigem Widerstand und einer flächenbezogenen Wärmekapazität von etwa 8 kJ/m2K: die Schale folgt dem Wasser binnen Sekunden. Ihre Fläche zählt beide Seiten, Innenseite und Unterseite, die beide mit dem Raum austauschen."),
                    T("Aged surfaces: the white cover enamel, dulled by scouring powder and limescale, absorbs 0.32 of the sun instead of about 0.25 when new; the dark ground coat of the underside about 0.85, which the one region of the sheet does not separate because no sun reaches the underside in any pose (both coats emit about 0.9); weathered galvanising emits 0.28 instead of about 0.05 when bright (authored estimates from the ranges in Incropera).", "Gealterte Oberflächen: die weiße Deckemail, durch Scheuerpulver und Kalk stumpf, absorbiert 0,32 der Sonne statt etwa 0,25 im Neuzustand; die dunkle Grundemail der Unterseite etwa 0,85, die die eine Region des Blechs nicht trennt, weil in keiner Pose Sonne an die Unterseite gelangt (beide Schichten strahlen etwa 0,9 ab); verwitterte Verzinkung strahlt 0,28 statt etwa 0,05 im blanken Zustand ab (Schätzwerte aus den Bereichen bei Incropera)."),
                    T("In use 0.33 of the shell area (the inside of floor and walls up to the water line) is in contact with water at 38 degC and the surface is wet; the caps stand with a third of their area on a 21 degC tiled floor; in the depot the blanket lies with 0.45 of its area on an 18 degC floor.", "In Gebrauch ist 0,33 der Schalenfläche (die Innenseite von Boden und Wänden bis zur Wasserlinie) in Kontakt mit Wasser von 38 degC und die Oberfläche nass; die Kappen stehen mit einem Drittel ihrer Fläche auf einem 21 degC warmen Fliesenboden; im Lager liegt die Decke mit 0,45 ihrer Fläche auf einem 18 degC warmen Boden."),
                    T("No element draws power: the bath is a passive object, 0 W stated explicitly on the shell region; the water is filled from the house supply and is a load and a contact temperature, not part of the asset.", "Kein Element nimmt Leistung auf: die Wanne ist ein passives Objekt, 0 W ausdrücklich an der Region Schale angegeben; das Wasser kommt aus der Hausleitung und ist eine Last und eine Kontakttemperatur, nicht Teil des Assets."),
                ]},
    "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": [],
                  "assumptions": [
                      T("One rigid body on the contacts of each pose: stored, the lip stands on the blanket (friction 0.6) and the two upper caps lean on the depot wall (horizontal contact, friction 0.4), which takes about 0.3 of the weight horizontally; carried, the hands of two carriers under the lip at both ends are pins; in use, the four caps stand on the floor.", "Ein starrer Körper auf den Kontakten jeder Pose: gelagert steht die Abkantung auf der Decke (Reibung 0,6) und die zwei oberen Kappen lehnen an der Lagerwand (waagerechter Kontakt, Reibung 0,4), die etwa 0,3 des Gewichts waagerecht aufnimmt; getragen sind die Hände zweier Träger unter der Abkantung an beiden Enden Gelenklager; in Gebrauch stehen die vier Kappen auf dem Boden."),
                      T(f"The blanket ({MASS['blanket']['kg']:.2f} kg) belongs to the asset but lies on its own: its floor reaction is entered as an upward load at its centre in every pose, so the carriers lift the bath alone ({TUB_KG:.1f} kg = {TUB_KG * G:.0f} N).", f"Die Decke ({MASS['blanket']['kg']:.2f} kg) gehört zum Asset, liegt aber für sich: ihre Bodenreaktion ist in jeder Pose als aufwärts gerichtete Last in ihrer Mitte angesetzt, so heben die Träger nur die Wanne ({TUB_KG:.1f} kg = {TUB_KG * G:.0f} N)."),
                      T("Members: the enamelled DC04EK shell (EN 10209, 2.0 mm steel, E 210 kN/mm2, yield taken as 180 N/mm2) as rim, lip, wall and bottom; the galvanised S235 rail tubes and M10 studs of the foot set (EN 10346) glued to the bottom with bitumen pads; masses are section x density of the 1:50 solids.", "Glieder: die emaillierte DC04EK-Schale (EN 10209, 2,0 mm Stahl, E 210 kN/mm2, Streckgrenze mit 180 N/mm2 angesetzt) als Rand, Abkantung, Wand und Boden; die verzinkten S235-Rechteckrohre und M10-Gewindestangen des Fußgestells (EN 10346), mit Bitumenpads unter den Boden geklebt; Massen sind Querschnitt x Dichte der 1:50-Körper."),
                      T(f"In use: {WATER_KG:.0f} l of water on the floor of the bath and a bather of 80 kg, 60 kg on the floor and 20 kg against the sloped head end (authored estimates; together with the bath about {TUB_KG + WATER_KG + 80:.0f} kg on four caps of 40 mm, the case EN 14516 asks a bath to carry without lasting deformation).", f"In Gebrauch: {WATER_KG:.0f} l Wasser auf dem Wannenboden und eine badende Person von 80 kg, 60 kg auf dem Boden und 20 kg an der Schräge des Kopfendes (Schätzwerte; zusammen mit der Wanne etwa {TUB_KG + WATER_KG + 80:.0f} kg auf vier Kappen von 40 mm, der Fall, den EN 14516 eine Wanne ohne bleibende Verformung tragen lässt)."),
                  ], "clinical": False},
    "volumes": VOLUMES,
    "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

for scale_rep in ASSET["representations"].values():
    for view in scale_rep["geometry"].values():
        for s in view:
            if s["bone"] == "feet":
                to_feet(s)

ORDER = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]
DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: ASSET[k] for k in ORDER}}

if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(DOC, ensure_ascii=False, indent=1) + "\n")
    for b, m in MASS.items():
        print(f"[DEBUG] {b}: {m['kg']:.4f} kg com ({m['comX']:.4f}, {m['comY']:.4f}, {m['comZ']:.4f}) parts {m['parts']}")
    print(f"[DEBUG] total {TOTAL_KG:.4f} kg; tub+feet {TUB_KG:.3f} kg; blanket density {BLANKET_RHO}; rho_es {RHO_ES:.1f} c {C_ES} k {K_ES}")
    print(f"[DEBUG] stored extents lo {[round(q, 4) for q in LO]} hi {[round(q, 4) for q in HI]}")
    print(f"[DEBUG] flat in use extents {[[round(q, 4) for q in e] for e in extents(FLAT_DEG, TRAVEL_USE, LIFT_FLAT, True)]}")
    print(f"[DEBUG] carried extents {[[round(q, 4) for q in e] for e in extents(FLAT_DEG, CARRY_TRAVEL, CARRY_LIFT, True)]}")
    print(f"[DEBUG] tip lifts {LIFT1} {LIFT2}; carry clearance (m, t) {carry_clearance()}")
    print(f"[DEBUG] slopes foot {FOOT_SLOPE:.2f} side {SIDE_SLOPE:.2f}; capacity to overflow {CAPACITY_L:.0f} l")
    tub = MASS["tub"]
    print(f"[DEBUG] stored COM world {rot(REST_DEG, tub['comX'], tub['comY'])}")
