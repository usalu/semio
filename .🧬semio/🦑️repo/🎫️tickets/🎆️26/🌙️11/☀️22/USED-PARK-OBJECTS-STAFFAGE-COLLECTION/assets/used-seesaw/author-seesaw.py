"""🎢️ Hand-chosen numbers and frame helpers of the used timber beam seesaw: one place for every dimension, bone frame and conversion from
world (x ahead, y up, z left) or beam-frame (s along the beam, t along its normal from the axle) coordinates into bone-local (u, v, w).

@see ../../brief.md — the wave brief
@see ../../reports/used-seesaw.md — reference table and the decisions these numbers follow"""
import json
import math
import os

TH = 10.0
th = math.radians(TH)
CT, ST = math.cos(th), math.sin(th)
P = (0.0, 0.42)
E = 0.16
BT0, BT1 = 0.06, 0.26
BW = 0.07
BL = 1.75
CH = 0.02
SEAT_S = (1.25, 1.65)
SEAT_T = (0.26, 0.305)
SEAT_W = 0.15
HS = 1.12
HPLATE_S, HPLATE_T, HPLATE_W = 0.06, 0.008, 0.05
HPOST_R = 0.0212
GRIP_T = 0.605
GRIP_R = 0.01685
GRIP_W = 0.18
WORN = (0.04, 0.16)
CHEEK_Z = (0.070, 0.078)
CHEEK_S = 0.11
CHEEK_TOP = 0.20
CHEEK_R = 0.05
BOLT_S, BOLT_T, BOLT_R = 0.07, 0.13, 0.012
BOLT_Z = (0.078, 0.088)
AXLE_R, AXLE_Z = 0.015, 0.093
NUT_R, NUT_Z = 0.023, 0.078
HOUSING_RO, HOUSING_RI, HOUSING_W = 0.03015, 0.0155, 0.065
LUG_X, LUG_LOW, LUG_HIGH = 0.05, 0.405, 0.432
TOPPLATE = (0.38, 0.39, 0.06)
WEB = (0.39, 0.40, 0.02, 0.06)
POST_Y = (0.04, 0.38)
POST_H, POST_T = 0.05, 0.004
BASE = (0.025, 0.04, 0.13)
ANCHOR_XZ, ANCHOR_R, ANCHOR_H = 0.095, 0.012, 0.015
CROWN = (0.0, 0.025, 0.225)
TX = 1.50
RO, RI, TREAD = 0.317, 0.1905, 0.012
TW, SIDEWALL = 0.0975, 0.010
YC = P[1] + (BT0 - RO - TX * ST) / CT

RHO = {"larch-weathered": 590.0, "larch-fresh": 590.0, "steel-galvanised": 7850.0, "steel-painted": 7850.0, "steel-worn": 7850.0, "rubber-tyre": 1200.0, "concrete": 2400.0}

FRAMES = {
    "ground": ((0.0, 0.0), 0.0),
    "footing": ((0.0, 0.0), 0.0),
    "post": ((0.0, CROWN[1]), 90.0),
    "lift": (P, 0.0),
    "beam": (P, TH + 90.0),
    "beam-fore": ((P[0] - E * ST, P[1] + E * CT), TH),
    "beam-aft": ((P[0] - E * ST, P[1] + E * CT), TH + 180.0),
    "tyre-fore": ((TX, 0.0), 0.0),
    "tyre-aft": ((-TX, 0.0), 0.0),
}


def r6(x):
    """🔢️ Rounds to 0.1 µm and clears negative zero."""
    v = round(x, 6)
    return 0.0 if v == 0 else v


def bw(s, t):
    """🧭️ World point of the beam frame point (s along the beam, t along its normal) at rest."""
    return (P[0] + s * CT - t * ST, P[1] + s * ST + t * CT)


def loc(bone, x, y):
    """📍️ Bone-local (u, v) of a world point at rest."""
    (ox, oy), phi = FRAMES[bone]
    c, s = math.cos(math.radians(phi)), math.sin(math.radians(phi))
    dx, dy = x - ox, y - oy
    return (r6(c * dx + s * dy), r6(-s * dx + c * dy))


def bl(bone, s, t):
    """🪵️ Bone-local (u, v) of a beam frame point."""
    return loc(bone, *bw(s, t))


def ldir(bone, dx, dy):
    """➡️ Bone-local direction of a world direction."""
    _, phi = FRAMES[bone]
    c, s = math.cos(math.radians(phi)), math.sin(math.radians(phi))
    return (r6(c * dx + s * dy), r6(-s * dx + c * dy))


def nrm():
    """⬆️ World direction of the beam normal at rest."""
    return (-ST, CT)


def arc(cx, cy, r, a0, a1, n):
    """🌙️ Points of a circular arc from angle a0 to a1 (radians), both ends included."""
    return [(cx + r * math.cos(a0 + (a1 - a0) * k / n), cy + r * math.sin(a0 + (a1 - a0) * k / n)) for k in range(n + 1)]


def circle(cx, cy, r, n=24):
    """⭕️ A closed polygon around a circle."""
    return [(r6(cx + r * math.cos(2 * math.pi * k / n)), r6(cy + r * math.sin(2 * math.pi * k / n))) for k in range(n)]


def arch(ro, ri, yc, n=36):
    """🌈️ The part above the ground (y >= 0) of the ring between radii ri and ro about (0, yc): an arch, as a simple polygon."""
    ao = math.asin(-yc / ro)
    ai = math.asin(-yc / ri)
    outer = arc(0.0, yc, ro, ao, math.pi - ao, n)
    inner = arc(0.0, yc, ri, math.pi - ai, ai, max(8, n * 2 // 3))
    return [(r6(x), r6(max(0.0, y))) for x, y in outer + inner]


def cheek_profile(n=12):
    """🛡️ Beam frame outline (s, t) of a bearing cheek: a plate 0.22 m wide down the beam side that narrows to a half round around the axle."""
    pts = [(CHEEK_S, CHEEK_TOP), (-CHEEK_S, CHEEK_TOP), (-CHEEK_S, BT0)]
    pts += arc(0.0, 0.0, CHEEK_R, math.pi, 2 * math.pi, n)
    pts += [(CHEEK_S, BT0)]
    return pts


def beam_profile(chamfer):
    """🪵️ Beam-fore local outline (u from the beam centre to the end, v about the centre line) of one beam half with chamfered end edges."""
    h = (BT1 - BT0) / 2
    if chamfer <= 0:
        return [(0.0, -h), (BL, -h), (BL, h), (0.0, h)]
    return [(0.0, -h), (BL - chamfer, -h), (BL, -h + chamfer), (BL, h - chamfer), (BL - chamfer, h), (0.0, h)]


def housing_lower():
    """🔩️ World outline (x, y) of the lower bearing half-shell with its two bolting lugs (bore 31 mm, outside 60.3 mm)."""
    y0 = P[1]
    xo = math.sqrt(HOUSING_RO ** 2 - (y0 - LUG_LOW) ** 2)
    a0 = math.atan2(LUG_LOW - y0, xo)
    pts = [(HOUSING_RI, y0), (LUG_X, y0), (LUG_X, LUG_LOW), (xo, LUG_LOW)]
    pts += arc(0.0, y0, HOUSING_RO, a0, -math.pi - a0, 16)[1:-1]
    pts += [(-xo, LUG_LOW), (-LUG_X, LUG_LOW), (-LUG_X, y0), (-HOUSING_RI, y0)]
    pts += arc(0.0, y0, HOUSING_RI, -math.pi, 0.0, 10)[1:-1]
    return [(r6(x), r6(y)) for x, y in pts]


def housing_cap():
    """🧢️ World outline (x, y) of the bearing cap that lifts out with the axle (upper half-shell and its lugs)."""
    y0 = P[1]
    xo = math.sqrt(HOUSING_RO ** 2 - (LUG_HIGH - y0) ** 2)
    a0 = math.atan2(LUG_HIGH - y0, xo)
    pts = [(-HOUSING_RI, y0), (-LUG_X, y0), (-LUG_X, LUG_HIGH), (-xo, LUG_HIGH)]
    pts += arc(0.0, y0, HOUSING_RO, math.pi - a0, a0, 16)[1:-1]
    pts += [(xo, LUG_HIGH), (LUG_X, LUG_HIGH), (LUG_X, y0), (HOUSING_RI, y0)]
    pts += arc(0.0, y0, HOUSING_RI, 0.0, math.pi, 10)[1:-1]
    return [(r6(x), r6(y)) for x, y in pts]


def contact_s():
    """🛞️ Beam frame s (negative: aft) of the point of the beam underside that touches the aft tyre at rest."""
    tx, ty = -TX - RO * ST, YC + RO * CT
    qx, qy = bw(0.0, BT0)
    return (tx - qx) * CT + (ty - qy) * ST


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def pts(seq):
    """📐️ Rounded point list."""
    return [[r6(x), r6(y)] for x, y in seq]


def poly(seq, closed=True):
    """✏️ A poly geometry."""
    return {"kind": "poly", "points": pts(seq), "closed": closed}


def ell(cx, cy, r):
    """🔘️ A circle geometry."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(r), "ry": r6(r)}


def wrect(bone, x0, y0, x1, y1):
    """🟫️ Bone-local outline of a world rectangle of the side elevation."""
    return [loc(bone, x0, y0), loc(bone, x1, y0), loc(bone, x1, y1), loc(bone, x0, y1)]


def brect(bone, s0, s1, t0, t1):
    """🟧️ Bone-local outline of a beam frame rectangle."""
    return [bl(bone, s0, t0), bl(bone, s1, t0), bl(bone, s1, t1), bl(bone, s0, t1)]


def bpoly(bone, seq):
    """🟠️ Bone-local outline of a beam frame polygon."""
    return [bl(bone, s, t) for s, t in seq]


def sh(sid, bone, role, z, filled, tone, geom, region=None, material=None, hatch=None, pen=None, solid=None, regions=None):
    """🖊️ One 2D shape."""
    d = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if pen is not None:
        d["penMm"] = pen
    if region:
        d["region"] = region
    if regions:
        d["regions"] = regions
    if material:
        d["material"] = material
    if hatch:
        d["hatch"] = hatch
    if solid:
        d["solid"] = solid
    d.update(geom)
    return d


def sw(z0, z1):
    """↔️ A lateral extrusion of an elevation profile for front and plan."""
    return {"kind": "extruded", "sweep": "w", "z0": r6(z0), "z1": r6(z1)}


MASS = {}


def account(bone, kg, cu, cv):
    """⚖️ Adds a 1:50 part to the mass of its bone (local centre)."""
    m, mu, mv = MASS.get(bone, (0.0, 0.0, 0.0))
    MASS[bone] = (m + kg, mu + kg * cu, mv + kg * cv)


def area(seq):
    """📏️ Signed area and centroid of a polygon."""
    a = cx = cy = 0.0
    n = len(seq)
    for i in range(n):
        x0, y0 = seq[i]
        x1, y1 = seq[(i + 1) % n]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return abs(a), cx / (6 * a), cy / (6 * a)


COUNTING = [False]


def solid(sid, bone, material, tone, kind, body, role="body", mass=None, **flags):
    """🧊️ One volume solid; `mass` = (volume m³, centre u, centre v) adds it to the bone mass while the 1:50 body is built."""
    d = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    d.update(body)
    for k, v in flags.items():
        if v is not None:
            d[k] = v
    if COUNTING[0] and mass is not None:
        account(bone, mass[0] * RHO[material], mass[1], mass[2])
    return d


def box(sid, bone, material, tone, c, size, corner=None, **kw):
    """📦️ A box centred at the local point c = (u, v, w)."""
    body = {"size": [r6(x) for x in size]}
    if corner:
        body["cornerRadius"] = corner
    if any(abs(x) > 1e-9 for x in c):
        body["transform"] = {"translation": [r6(x) for x in c]}
    return solid(sid, bone, material, tone, "box", body, mass=(size[0] * size[1] * size[2], c[0], c[1]), **kw)


def ext(sid, bone, material, tone, profile, z0, z1, holes=None, **kw):
    """🧱️ A profile in the bone (u, v) plane pushed laterally from z0 to z1."""
    a, cx, cy = area(profile)
    body = {"profile": pts(profile), "plane": "xy", "from": r6(z0), "to": r6(z1)}
    vol = a
    if holes:
        body["holes"] = [pts(h) for h in holes]
        vol -= sum(area(h)[0] for h in holes)
    return solid(sid, bone, material, tone, "extrude", body, mass=(vol * (z1 - z0), cx, cy), **kw)


def ext_up(sid, bone, material, tone, profile_vw, u0, u1, holes=None, **kw):
    """🏛️ A profile in the bone (v, w) plane pushed along u from u0 to u1."""
    a, cx, cy = area(profile_vw)
    body = {"profile": pts(profile_vw), "plane": "yz", "from": r6(u0), "to": r6(u1)}
    vol = a
    if holes:
        body["holes"] = [pts(h) for h in holes]
        vol -= sum(area(h)[0] for h in holes)
    return solid(sid, bone, material, tone, "extrude", body, mass=(vol * (u1 - u0), (u0 + u1) / 2, cx), **kw)


def cyl(sid, bone, material, tone, origin, axis, length, r, ri=None, wall=None, **kw):
    """🥫️ A cylinder (or a tube with inner radius ri) of `length` from the local `origin` along the local `axis`; `wall` is the wall of a capped tube whose closed cavity the solid envelope hides (its mass is the tube section's)."""
    if ri:
        prof = [[0.0, r6(ri)], [0.0, r6(r)], [r6(length), r6(r)], [r6(length), r6(ri)], [0.0, r6(ri)]]
        vol = math.pi * (r * r - ri * ri) * length
    else:
        prof = [[0.0, 0.0], [0.0, r6(r)], [r6(length), r6(r)], [r6(length), 0.0]]
        vol = math.pi * (r * r - (r - wall) ** 2 if wall else r * r) * length
    cu = origin[0] + axis[0] * length / 2
    cv = origin[1] + axis[1] * length / 2
    body = {"profile": prof, "axis": [r6(a) for a in axis], "origin": [r6(o) for o in origin]}
    return solid(sid, bone, material, tone, "revolve", body, mass=(vol, cu, cv), **kw)


def rev(sid, bone, material, tone, origin, axis, profile, **kw):
    """🏺️ A turned part from an open [axial, radial] profile (closed through the axis)."""
    vol = 0.0
    for (a0, r0), (a1, r1) in zip(profile, profile[1:]):
        vol += math.pi * (a1 - a0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3
    L = max(p[0] for p in profile)
    body = {"profile": [[r6(a), r6(r)] for a, r in profile], "axis": [r6(a) for a in axis], "origin": [r6(o) for o in origin]}
    return solid(sid, bone, material, tone, "revolve", body, mass=(abs(vol), origin[0] + axis[0] * L / 2, origin[1] + axis[1] * L / 2), **kw)


def ref(sid, of, **label):
    """🔗️ A reference to a body solid of the same scale."""
    d = {"id": sid, "of": of}
    d.update(label)
    return d


GALV, PAINT, WORNS, LARCH, FRESH, RUBBER, CONC = "steel-galvanised", "steel-painted", "steel-worn", "larch-weathered", "larch-fresh", "rubber-tyre", "concrete"
TONE = {"footing": 0.72, "galv": 0.5, "galv-dark": 0.4, "beam": 0.58, "seat-old": 0.5, "seat-new": 0.82, "paint": 0.3, "worn": 0.66, "tyre": 0.14}
REGION_OF_MATERIAL = {CONC: "footing-concrete", GALV: "bearing-galvanised", LARCH: "beam-larch", RUBBER: "tyre-rubber", PAINT: "handle-paint", WORNS: "grip-worn"}
SIDES = (("fore", 1.0), ("aft", -1.0))


def cs_u():
    """🛞️ Local u (on beam-aft or beam-fore) of the tyre contact point of the beam underside."""
    return -contact_s()


def body50(count=False):
    """🔍️ The 1:50 body: every part the seesaw has at 7.5 mm, with its true sections; `count` adds the parts to the bone masses."""
    COUNTING[0] = count
    b = []
    b.append(box("footing-crown", "footing", CONC, TONE["footing"], (0, CROWN[1] / 2, 0), (2 * CROWN[2], CROWN[1], 2 * CROWN[2])))
    b.append(box("base-plate", "post", GALV, TONE["galv"], ((BASE[0] + BASE[1]) / 2 - CROWN[1], 0, 0), (BASE[1] - BASE[0], 2 * BASE[2], 2 * BASE[2])))
    for k, (sx, sz) in enumerate(((1, 1), (-1, 1), (-1, -1), (1, -1))):
        u, v = loc("post", sx * ANCHOR_XZ, BASE[1])
        b.append(rev(f"anchor-nut-{'abcd'[k]}", "post", GALV, TONE["galv-dark"], (u, v, sz * ANCHOR_XZ), (1, 0, 0), [(0, 0), (0, ANCHOR_R), (0.013, ANCHOR_R), (0.013, 0.008), (0.02, 0.008), (0.02, 0)]))
    sq = [(-POST_H, -POST_H), (POST_H, -POST_H), (POST_H, POST_H), (-POST_H, POST_H)]
    hole = [(-POST_H + POST_T, -POST_H + POST_T), (POST_H - POST_T, -POST_H + POST_T), (POST_H - POST_T, POST_H - POST_T), (-POST_H + POST_T, POST_H - POST_T)]
    b.append(ext_up("post-tube", "post", GALV, TONE["galv"], sq, POST_Y[0] - CROWN[1], POST_Y[1] - CROWN[1], holes=[hole]))
    b.append(box("top-plate", "post", GALV, TONE["galv"], ((TOPPLATE[0] + TOPPLATE[1]) / 2 - CROWN[1], 0, 0), (TOPPLATE[1] - TOPPLATE[0], 2 * TOPPLATE[2], 2 * TOPPLATE[2])))
    b.append(box("housing-web", "post", GALV, TONE["galv"], ((WEB[0] + WEB[1]) / 2 - CROWN[1], 0, 0), (WEB[1] - WEB[0], 2 * WEB[2], 2 * WEB[3])))
    b.append(ext("housing-lower", "post", GALV, TONE["galv"], [loc("post", x, y) for x, y in housing_lower()], -HOUSING_W, HOUSING_W))
    b.append(ext("bearing-cap", "lift", GALV, TONE["galv"], [loc("lift", x, y) for x, y in housing_cap()], -HOUSING_W, HOUSING_W))
    b.append(rev("grease-nipple", "lift", GALV, TONE["galv-dark"], (0, HOUSING_RO - 0.003, 0), (0, 1, 0), [(0, 0), (0, 0.0055), (0.011, 0.0055), (0.011, 0.004), (0.019, 0.004), (0.019, 0)]))
    b.append(cyl("axle", "lift", GALV, TONE["galv-dark"], (0, 0, -AXLE_Z), (0, 0, 1), 2 * AXLE_Z, AXLE_R))
    b.append(cyl("axle-nut-left", "lift", GALV, TONE["galv-dark"], (0, 0, NUT_Z), (0, 0, 1), AXLE_Z - NUT_Z, NUT_R, ri=AXLE_R))
    b.append(cyl("axle-nut-right", "lift", GALV, TONE["galv-dark"], (0, 0, -AXLE_Z), (0, 0, 1), AXLE_Z - NUT_Z, NUT_R, ri=AXLE_R))
    cheek = bpoly("beam", cheek_profile())
    bore = [bl("beam", x, y) for x, y in circle(0, 0, AXLE_R, 16)]
    b.append(ext("cheek-left", "beam", GALV, TONE["galv"], cheek, CHEEK_Z[0], CHEEK_Z[1], holes=[bore]))
    b.append(ext("cheek-right", "beam", GALV, TONE["galv"], cheek, -CHEEK_Z[1], -CHEEK_Z[0], holes=[bore]))
    for side, sg in SIDES:
        u, v = bl("beam", sg * BOLT_S, BOLT_T)
        b.append(cyl(f"bolt-{side}-left", "beam", GALV, TONE["galv-dark"], (u, v, BOLT_Z[0]), (0, 0, 1), BOLT_Z[1] - BOLT_Z[0], BOLT_R))
        b.append(cyl(f"bolt-{side}-right", "beam", GALV, TONE["galv-dark"], (u, v, -BOLT_Z[1]), (0, 0, 1), BOLT_Z[1] - BOLT_Z[0], BOLT_R))
    for side, sg in SIDES:
        bone = f"beam-{side}"
        b.append(ext(f"beam-{side}-body", bone, LARCH, TONE["beam"], beam_profile(CH), -BW, BW))
        su, sv = bl(bone, sg * sum(SEAT_S) / 2, sum(SEAT_T) / 2)
        b.append(box(f"seat-{side}", bone, FRESH if side == "fore" else LARCH, TONE["seat-new"] if side == "fore" else TONE["seat-old"], (su, sv, 0), (SEAT_S[1] - SEAT_S[0], SEAT_T[1] - SEAT_T[0], 2 * SEAT_W), corner=0.006))
        pu, pv = bl(bone, sg * HS, BT1 + HPLATE_T / 2)
        b.append(box(f"handle-{side}-plate", bone, PAINT, TONE["paint"], (pu, pv, 0), (2 * HPLATE_S, HPLATE_T, 2 * HPLATE_W)))
        ou, ov = bl(bone, sg * HS, BT1 + HPLATE_T)
        ax = ldir(bone, *nrm())
        b.append(cyl(f"handle-{side}-post", bone, PAINT, TONE["paint"], (ou, ov, 0), (ax[0], ax[1], 0), GRIP_T - BT1 - HPLATE_T, HPOST_R, wall=0.0032))
        gu, gv = bl(bone, sg * HS, GRIP_T)
        for name, z0, z1, mat in (("end-right", -GRIP_W, -WORN[1], PAINT), ("worn-right", -WORN[1], -WORN[0], WORNS), ("middle", -WORN[0], WORN[0], PAINT), ("worn-left", WORN[0], WORN[1], WORNS), ("end-left", WORN[1], GRIP_W, PAINT)):
            b.append(cyl(f"grip-{side}-{name}", bone, mat, TONE["worn"] if mat == WORNS else TONE["paint"], (gu, gv, z0), (0, 0, 1), z1 - z0, GRIP_R, wall=0.0026))
    for side, _ in SIDES:
        bone = f"tyre-{side}"
        b.append(ext(f"tyre-{side}-tread", bone, RUBBER, TONE["tyre"], arch(RO, RO - TREAD, YC), -TW, TW))
        b.append(ext(f"tyre-{side}-wall-left", bone, RUBBER, TONE["tyre"], arch(RO - TREAD, RI, YC), TW - SIDEWALL, TW))
        b.append(ext(f"tyre-{side}-wall-right", bone, RUBBER, TONE["tyre"], arch(RO - TREAD, RI, YC), -TW, -TW + SIDEWALL))
    COUNTING[0] = False
    return b


def region_of(s):
    """🗺️ The thermal region of a body solid."""
    if s["id"].startswith("seat-"):
        return "seat-fore-replaced" if s["id"] == "seat-fore" else "seat-aft-weathered"
    return REGION_OF_MATERIAL[s["material"]]


def thermal_of(body):
    """🌡️ Every body solid by reference in its region."""
    return [ref(f"th-{s['id']}", s["id"], region=region_of(s)) for s in body]


def rgn(material, sid=""):
    """🏷️ Region and material keywords of a shape."""
    if sid.startswith("seat-"):
        return {"region": "seat-fore-replaced" if sid == "seat-fore" else "seat-aft-weathered", "material": FRESH if sid == "seat-fore" else LARCH}
    return {"region": REGION_OF_MATERIAL[material], "material": material}


def nut_t(cx, base):
    """🔩️ Side outline of an anchor nut with its bolt end (x across, y up)."""
    return [(cx - ANCHOR_R, base), (cx + ANCHOR_R, base), (cx + ANCHOR_R, base + 0.013), (cx + 0.008, base + 0.013), (cx + 0.008, base + 0.02), (cx - 0.008, base + 0.02), (cx - 0.008, base + 0.013), (cx - ANCHOR_R, base + 0.013)]


def nipple_t(cx, base):
    """🛢️ Side outline of the grease nipple standing on the bearing cap (x across, y up)."""
    return [(cx - 0.0055, base), (cx + 0.0055, base), (cx + 0.0055, base + 0.011), (cx + 0.004, base + 0.011), (cx + 0.004, base + 0.019), (cx - 0.004, base + 0.019), (cx - 0.004, base + 0.011), (cx - 0.0055, base + 0.011)]


def tyre_front(cz):
    """🛞️ Tread face of a half-buried tyre seen from ahead: 195 mm wide, shoulders rounded by 25 mm."""
    h = YC + RO
    q = arc(cz + TW - 0.025, h - 0.025, 0.025, 0.0, math.pi / 2, 4) + arc(cz - TW + 0.025, h - 0.025, 0.025, math.pi / 2, math.pi, 4)
    return [(cz - TW, 0.0), (cz + TW, 0.0)] + q


def grip_segments(gw):
    """✋️ Grip segments (name, z0, z1, material): painted ends and middle, hands worn bare between."""
    return (("end-right", -gw, -WORN[1], PAINT), ("worn-right", -WORN[1], -WORN[0], WORNS), ("middle", -WORN[0], WORN[0], PAINT), ("worn-left", WORN[0], WORN[1], WORNS), ("end-left", WORN[1], gw, PAINT))


def beam_assembly_shapes(view, scale):
    """🪵️ Shapes of the rocking beam (beam halves, seats, handles, cheeks, bolts) for one view of 1:50 or 1:100: the elevation draws the
    bone-local outlines, front and plan the same profiles as lateral extrusions, so all three views follow the beam in every pose."""
    out = []
    elev = view == "elevation"
    zb = {"elevation": 0, "front": 0, "plan": 0}[view]
    for side, sg in SIDES:
        bone = f"beam-{side}"
        zo = (2.0 if view != "front" else (2.0 if side == "fore" else 1.2))
        out.append(sh(f"beam-{side}-body", bone, "body", zo, True, TONE["beam"], poly(beam_profile(CH if scale == 50 else 0.0)), hatch="larch-grain" if elev else None, solid=None if elev else sw(-BW, BW), **rgn(LARCH)))
        mat = FRESH if side == "fore" else LARCH
        out.append(sh(f"seat-{side}", bone, "body", zo + 0.2, True, TONE["seat-new"] if side == "fore" else TONE["seat-old"], poly(brect(bone, sg * SEAT_S[0], sg * SEAT_S[1], *SEAT_T)), hatch="larch-grain" if (elev and side == "aft") else None, solid=None if elev else sw(-SEAT_W, SEAT_W), **rgn(mat, f"seat-{side}")))
        pt = HPLATE_T if scale == 50 else HPLATE100
        out.append(sh(f"handle-{side}-plate", bone, "body", zo + 0.3, True, TONE["paint"], poly(brect(bone, sg * HS - HPLATE_S, sg * HS + HPLATE_S, BT1, BT1 + pt)), solid=None if elev else sw(-HPLATE_W, HPLATE_W), **rgn(PAINT)))
        post0 = BT1 + pt
        out.append(sh(f"handle-{side}-post", bone, "body", zo + 0.35, True, TONE["paint"], poly(brect(bone, sg * HS - HPOST_R, sg * HS + HPOST_R, post0, GRIP_T)), solid=None if elev else sw(-HPOST_R, HPOST_R), **rgn(PAINT)))
        gu, gv = bl(bone, sg * HS, GRIP_T)
        if elev:
            out.append(sh(f"grip-{side}-end-right", bone, "body", zo + 0.4, True, TONE["paint"], ell(gu, gv, GRIP_R), regions=["handle-paint", "grip-worn"], material=PAINT))
        else:
            for name, z0, z1, m in grip_segments(GRIP_W):
                out.append(sh(f"grip-{side}-{name}", bone, "body", zo + 0.4, True, TONE["worn"] if m == WORNS else TONE["paint"], poly(circle(gu, gv, GRIP_R, 16)), solid=sw(z0, z1), **rgn(m)))
    cheek = bpoly("beam", cheek_profile())
    zc = 2.6 if view != "plan" else 2.5
    if elev:
        out.append(sh("cheek-right", "beam", "body", zc, True, TONE["galv"], poly(cheek), **rgn(GALV)))
    else:
        out.append(sh("cheek-left", "beam", "body", zc, True, TONE["galv"], poly(cheek), solid=sw(*CHEEK_Z), **rgn(GALV)))
        out.append(sh("cheek-right", "beam", "body", zc, True, TONE["galv"], poly(cheek), solid=sw(-CHEEK_Z[1], -CHEEK_Z[0]), **rgn(GALV)))
    if scale == 50:
        for side, sg in SIDES:
            u, v = bl("beam", sg * BOLT_S, BOLT_T)
            if elev:
                out.append(sh(f"bolt-{side}-right", "beam", "body", 2.7, True, TONE["galv-dark"], ell(u, v, BOLT_R), **rgn(GALV)))
            else:
                out.append(sh(f"bolt-{side}-left", "beam", "body", 2.7, True, TONE["galv-dark"], poly(circle(u, v, BOLT_R, 12)), solid=sw(*BOLT_Z), **rgn(GALV)))
                out.append(sh(f"bolt-{side}-right", "beam", "body", 2.7, True, TONE["galv-dark"], poly(circle(u, v, BOLT_R, 12)), solid=sw(-BOLT_Z[1], -BOLT_Z[0]), **rgn(GALV)))
    return out


def traces_elevation(scale):
    """🧾️ Traces of use drawn on the side elevation: shrinkage checks along the greyed beam, a sprayed tag and sticker residue on the aft half,
    the hollow sat into the old aft seat with chewing-gum spots, white rust on the post, the sun-cracked tyre walls and the grease smear."""
    out = []
    d = {"detail": "detail", "texture": "texture"}
    out.append(sh("beam-fore-check-1", "beam-fore", "detail", 2.05, False, 0.3, poly([(0.18, 0.035), (0.46, 0.03), (0.71, 0.038), (1.02, 0.031)], False), **rgn(LARCH)))
    out.append(sh("beam-aft-check-1", "beam-aft", "detail", 2.05, False, 0.3, poly([(0.25, -0.03), (0.6, -0.036), (0.98, -0.028), (1.4, -0.034)], False), **rgn(LARCH)))
    if scale == 50:
        out.append(sh("beam-fore-check-2", "beam-fore", "detail", 2.05, False, 0.3, poly([(0.95, -0.045), (1.22, -0.05), (1.6, -0.043)], False), **rgn(LARCH)))
        out.append(sh("beam-aft-end-checks", "beam-aft", "detail", 2.05, False, 0.3, poly([(1.74, -0.06), (1.66, -0.02), (1.72, 0.03)], False), **rgn(LARCH)))
        tag = [(0.42, 0.04), (0.47, -0.05), (0.52, 0.045), (0.57, -0.04), (0.6, 0.02), (0.66, -0.05), (0.7, 0.035), (0.76, -0.02)]
        out.append(sh("beam-aft-tag", "beam-aft", "texture", 2.06, False, 0.15, poly(tag, False), **rgn(LARCH)))
        out.append(sh("beam-aft-sticker", "beam-aft", "detail", 2.06, False, 0.35, poly([(0.92, -0.02), (1.0, -0.02), (1.0, 0.035), (0.92, 0.035)]), **rgn(LARCH)))
        u0, v0 = bl("beam-aft", -1.33, SEAT_T[1] - 0.004)
        u1, v1 = bl("beam-aft", -1.45, SEAT_T[1] - 0.011)
        u2, v2 = bl("beam-aft", -1.57, SEAT_T[1] - 0.004)
        out.append(sh("seat-aft-hollow", "beam-aft", "detail", 2.25, False, 0.3, poly([(u0, v0), (u1, v1), (u2, v2)], False), **rgn(LARCH, "seat-aft")))
        out.append(sh("post-white-rust", "post", "texture", 1.31, False, 0.6, poly([loc("post", -0.03, 0.08), loc("post", -0.01, 0.15), loc("post", -0.035, 0.24), loc("post", 0.0, 0.31)], False), **rgn(GALV)))
        out.append(sh("post-sticker", "post", "detail", 1.31, False, 0.4, poly(wrect("post", -0.03, 0.2, 0.03, 0.26)), **rgn(GALV)))
        for side, _ in SIDES:
            out.append(sh(f"tyre-{side}-tread-edge", f"tyre-{side}", "detail", 1.05, False, 0.3, poly(arc(0.0, YC, RO - TREAD, math.asin(-YC / (RO - TREAD)) + 0.05, math.pi - math.asin(-YC / (RO - TREAD)) - 0.05, 24), False), **rgn(RUBBER)))
            out.append(sh(f"tyre-{side}-cracks", f"tyre-{side}", "texture", 1.06, False, 0.4, poly([(-0.2, 0.06), (-0.17, 0.1), (-0.21, 0.13), (-0.16, 0.16)], False), **rgn(RUBBER)))
    else:
        for side, _ in SIDES:
            out.append(sh(f"tyre-{side}-tread-edge", f"tyre-{side}", "detail", 1.05, False, 0.3, poly(arc(0.0, YC, RO - 0.03, math.asin(-YC / (RO - 0.03)) + 0.08, math.pi - math.asin(-YC / (RO - 0.03)) - 0.08, 16), False), **rgn(RUBBER)))
    return out


def pedestal_elevation(scale):
    """🏗️ Side elevation of the pedestal (footing crown, base plate, anchor nuts, post, head and bearing) at 1:50 or 1:100."""
    out = [sh("footing-crown", "footing", "body", 1.1, True, TONE["footing"], poly(wrect("footing", -CROWN[2], 0, CROWN[2], CROWN[1])), hatch="concrete" if scale == 50 else None, **rgn(CONC))]
    out.append(sh("base-plate", "post", "body", 1.2, True, TONE["galv"], poly(wrect("post", -BASE[2], BASE[0], BASE[2], BASE[1])), **rgn(GALV)))
    if scale == 50:
        out.append(sh("anchor-nut-c", "post", "body", 1.25, True, TONE["galv-dark"], poly([loc("post", x, y) for x, y in nut_t(-ANCHOR_XZ, BASE[1])]), **rgn(GALV)))
        out.append(sh("anchor-nut-d", "post", "body", 1.25, True, TONE["galv-dark"], poly([loc("post", x, y) for x, y in nut_t(ANCHOR_XZ, BASE[1])]), **rgn(GALV)))
    out.append(sh("post-tube", "post", "body", 1.3, True, TONE["galv"], poly(wrect("post", -POST_H, POST_Y[0], POST_H, POST_Y[1])), **rgn(GALV)))
    if scale == 50:
        out.append(sh("top-plate", "post", "body", 1.35, True, TONE["galv"], poly(wrect("post", -TOPPLATE[2], TOPPLATE[0], TOPPLATE[2], TOPPLATE[1])), **rgn(GALV)))
        out.append(sh("housing-web", "post", "body", 1.36, True, TONE["galv"], poly(wrect("post", -WEB[2], WEB[0], WEB[2], WEB[1])), **rgn(GALV)))
        out.append(sh("housing-lower", "post", "body", 1.4, True, TONE["galv"], poly([loc("post", x, y) for x, y in housing_lower()]), **rgn(GALV)))
        out.append(sh("bearing-cap", "lift", "body", 1.4, True, TONE["galv"], poly([loc("lift", x, y) for x, y in housing_cap()]), **rgn(GALV)))
        out.append(sh("grease-nipple", "lift", "body", 1.45, True, TONE["galv-dark"], poly([loc("lift", x, y) for x, y in nipple_t(0.0, P[1] + HOUSING_RO - 0.003)]), **rgn(GALV)))
        out.append(sh("axle-nut-right", "lift", "body", 2.8, True, TONE["galv-dark"], ell(0, 0, NUT_R), **rgn(GALV)))
        out.append(sh("axle-end", "lift", "detail", 2.85, False, 0.2, ell(0, 0, AXLE_R), **rgn(GALV)))
    else:
        out.append(sh("post-head", "post", "body", 1.35, True, TONE["galv"], poly(wrect("post", -TOPPLATE[2], TOPPLATE[0], TOPPLATE[2], WEB[1])), **rgn(GALV)))
        out.append(sh("housing-lower", "post", "body", 1.4, True, TONE["galv"], poly([loc("post", x, y) for x, y in [(HOUSING_RO, P[1])] + arc(0, P[1], HOUSING_RO, 0, -math.pi, 12)[1:]]), **rgn(GALV)))
        out.append(sh("bearing-cap", "lift", "body", 1.4, True, TONE["galv"], poly([loc("lift", x, y) for x, y in arc(0, P[1], HOUSING_RO, 0, math.pi, 12)]), **rgn(GALV)))
        out.append(sh("axle", "lift", "body", 2.8, True, TONE["galv-dark"], ell(0, 0, NUT_R), **rgn(GALV)))
    for side, _ in SIDES:
        out.append(sh(f"tyre-{side}", f"tyre-{side}", "body", 1.0, True, TONE["tyre"], poly(arch(RO, RI, YC)), **rgn(RUBBER)))
    return out


def rect_pts(a0, b0, a1, b1):
    """▭️ Corner points of an axis-parallel rectangle in paper axes."""
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def pedestal_front_plan(view, scale):
    """🏗️ Front elevation (lateral, height) and plan (ahead, lateral) of footing, base plate, post, head, bearing and tyres at 1:50 or 1:100."""
    out = []
    fr = view == "front"
    ux = math.sqrt(RO ** 2 - YC ** 2)

    def R(sid, bone, z, tone, front_rect, plan_rect, mat=GALV):
        g = front_rect if fr else plan_rect
        out.append(sh(sid, bone, "body", z, True, tone, poly(rect_pts(*g)), **rgn(mat)))

    R("footing-crown", "footing", 0.5, TONE["footing"], (-CROWN[2], 0, CROWN[2], CROWN[1]), (-CROWN[2], -CROWN[2], CROWN[2], CROWN[2]), CONC)
    R("base-plate", "post", 0.6, TONE["galv"], (-BASE[2], BASE[0], BASE[2], BASE[1]), (-BASE[2], -BASE[2], BASE[2], BASE[2]))
    if scale == 50:
        if fr:
            for k in "ad":
                cz = ANCHOR_XZ if k == "a" else -ANCHOR_XZ
                out.append(sh(f"anchor-nut-{k}", "post", "body", 0.65, True, TONE["galv-dark"], poly(nut_t(cz, BASE[1])), **rgn(GALV)))
        else:
            for k, (sx, sz) in zip("abcd", ((1, 1), (-1, 1), (-1, -1), (1, -1))):
                out.append(sh(f"anchor-nut-{k}", "post", "body", 0.65, True, TONE["galv-dark"], ell(sx * ANCHOR_XZ, sz * ANCHOR_XZ, ANCHOR_R), **rgn(GALV)))
    R("post-tube", "post", 0.7, TONE["galv"], (-POST_H, POST_Y[0], POST_H, POST_Y[1]), (-POST_H, -POST_H, POST_H, POST_H))
    if scale == 50:
        R("top-plate", "post", 0.75, TONE["galv"], (-TOPPLATE[2], TOPPLATE[0], TOPPLATE[2], TOPPLATE[1]), (-TOPPLATE[2], -TOPPLATE[2], TOPPLATE[2], TOPPLATE[2]))
        R("housing-web", "post", 0.76, TONE["galv"], (-WEB[3], WEB[0], WEB[3], WEB[1]), (-WEB[2], -WEB[3], WEB[2], WEB[3]))
    else:
        R("post-head", "post", 0.75, TONE["galv"], (-TOPPLATE[2], TOPPLATE[0], TOPPLATE[2], WEB[1]), (-TOPPLATE[2], -TOPPLATE[2], TOPPLATE[2], TOPPLATE[2]))
    lug = LUG_X if scale == 50 else HOUSING_RO
    R("housing-lower", "post", 0.8, TONE["galv"], (-HOUSING_W, P[1] - HOUSING_RO, HOUSING_W, P[1]), (-lug, -HOUSING_W, lug, HOUSING_W))
    R("bearing-cap", "lift", 0.85, TONE["galv"], (-HOUSING_W, P[1], HOUSING_W, P[1] + HOUSING_RO), (-lug, -HOUSING_W, lug, HOUSING_W))
    if scale == 50:
        if fr:
            out.append(sh("grease-nipple", "lift", "body", 0.9, True, TONE["galv-dark"], poly(nipple_t(0.0, P[1] + HOUSING_RO - 0.003)), **rgn(GALV)))
        else:
            out.append(sh("grease-nipple", "lift", "body", 2.9, True, TONE["galv-dark"], ell(0, 0, 0.0055), **rgn(GALV)))
        R("axle", "lift", 0.82, TONE["galv-dark"], (-AXLE_Z, P[1] - AXLE_R, AXLE_Z, P[1] + AXLE_R), (-AXLE_R, -AXLE_Z, AXLE_R, AXLE_Z))
        R("axle-nut-left", "lift", 2.8, TONE["galv-dark"], (NUT_Z, P[1] - NUT_R, AXLE_Z, P[1] + NUT_R), (-NUT_R, NUT_Z, NUT_R, AXLE_Z))
        R("axle-nut-right", "lift", 2.8, TONE["galv-dark"], (-AXLE_Z, P[1] - NUT_R, -NUT_Z, P[1] + NUT_R), (-NUT_R, -AXLE_Z, NUT_R, -NUT_Z))
    else:
        R("axle", "lift", 2.8, TONE["galv-dark"], (-AXLE_Z, P[1] - NUT_R, AXLE_Z, P[1] + NUT_R), (-NUT_R, -AXLE_Z, NUT_R, AXLE_Z))
    for side, sg in SIDES:
        z = (3.0 if side == "fore" else 0.4) if fr else 0.4
        g = poly(tyre_front(0.0)) if fr else poly(rect_pts(sg * TX - ux, -TW, sg * TX + ux, TW))
        out.append(sh(f"tyre-{side}", f"tyre-{side}", "body", z, True, TONE["tyre"], g, **rgn(RUBBER)))
        if scale == 50 and not fr:
            out.append(sh(f"tyre-{side}-tread-edge", f"tyre-{side}", "detail", 0.45, False, 0.3, poly([(sg * TX - ux + 0.02, -TW + 0.012), (sg * TX + ux - 0.02, -TW + 0.012)], False), **rgn(RUBBER)))
            out.append(sh(f"tyre-{side}-tread-edge-left", f"tyre-{side}", "detail", 0.45, False, 0.3, poly([(sg * TX - ux + 0.02, TW - 0.012), (sg * TX + ux - 0.02, TW - 0.012)], False), **rgn(RUBBER)))
    return out


def semi(r, up, n=12):
    """🌗️ World outline of a half disc of the bearing about the axle (upper or lower half)."""
    y0 = P[1]
    a = arc(0, y0, r, 0, math.pi, n) if up else arc(0, y0, r, 0, -math.pi, n)
    return a


def body100():
    """🔎️ The 1:100 body (feature 15 mm): nuts, nipple, bolts and plates under 15 mm merged or left out, the bearing as two half discs."""
    b = [box("footing-crown", "footing", CONC, TONE["footing"], (0, CROWN[1] / 2, 0), (2 * CROWN[2], CROWN[1], 2 * CROWN[2]))]
    b.append(box("base-plate", "post", GALV, TONE["galv"], ((BASE[0] + BASE[1]) / 2 - CROWN[1], 0, 0), (BASE[1] - BASE[0], 2 * BASE[2], 2 * BASE[2])))
    sq = [(-POST_H, -POST_H), (POST_H, -POST_H), (POST_H, POST_H), (-POST_H, POST_H)]
    hole = [(-POST_H + POST_T, -POST_H + POST_T), (POST_H - POST_T, -POST_H + POST_T), (POST_H - POST_T, POST_H - POST_T), (-POST_H + POST_T, POST_H - POST_T)]
    b.append(ext_up("post-tube", "post", GALV, TONE["galv"], sq, POST_Y[0] - CROWN[1], POST_Y[1] - CROWN[1], holes=[hole]))
    b.append(box("post-head", "post", GALV, TONE["galv"], ((TOPPLATE[0] + WEB[1]) / 2 - CROWN[1], 0, 0), (WEB[1] - TOPPLATE[0], 2 * TOPPLATE[2], 2 * TOPPLATE[2])))
    b.append(ext("housing-lower", "post", GALV, TONE["galv"], [loc("post", x, y) for x, y in semi(HOUSING_RO, False)], -HOUSING_W, HOUSING_W))
    b.append(ext("bearing-cap", "lift", GALV, TONE["galv"], [loc("lift", x, y) for x, y in semi(HOUSING_RO, True)], -HOUSING_W, HOUSING_W))
    b.append(cyl("axle", "lift", GALV, TONE["galv-dark"], (0, 0, -AXLE_Z), (0, 0, 1), 2 * AXLE_Z, NUT_R))
    cheek = bpoly("beam", cheek_profile(8))
    b.append(ext("cheek-left", "beam", GALV, TONE["galv"], cheek, CHEEK_Z[0], CHEEK_Z[1], sheet=True))
    b.append(ext("cheek-right", "beam", GALV, TONE["galv"], cheek, -CHEEK_Z[1], -CHEEK_Z[0], sheet=True))
    for side, sg in SIDES:
        bone = f"beam-{side}"
        b.append(ext(f"beam-{side}-body", bone, LARCH, TONE["beam"], beam_profile(0.0), -BW, BW))
        su, sv = bl(bone, sg * sum(SEAT_S) / 2, sum(SEAT_T) / 2)
        b.append(box(f"seat-{side}", bone, FRESH if side == "fore" else LARCH, TONE["seat-new"] if side == "fore" else TONE["seat-old"], (su, sv, 0), (SEAT_S[1] - SEAT_S[0], SEAT_T[1] - SEAT_T[0], 2 * SEAT_W)))
        pu, pv = bl(bone, sg * HS, BT1 + HPLATE100 / 2)
        b.append(box(f"handle-{side}-plate", bone, PAINT, TONE["paint"], (pu, pv, 0), (2 * HPLATE_S, HPLATE100, 2 * HPLATE_W)))
        ou, ov = bl(bone, sg * HS, BT1 + HPLATE100)
        ax = ldir(bone, *nrm())
        b.append(cyl(f"handle-{side}-post", bone, PAINT, TONE["paint"], (ou, ov, 0), (ax[0], ax[1], 0), GRIP_T - BT1 - HPLATE100, HPOST_R))
        gu, gv = bl(bone, sg * HS, GRIP_T)
        for name, z0, z1, mat in grip_segments(GRIP_W):
            b.append(cyl(f"grip-{side}-{name}", bone, mat, TONE["worn"] if mat == WORNS else TONE["paint"], (gu, gv, z0), (0, 0, 1), z1 - z0, GRIP_R))
    for side, _ in SIDES:
        b.append(ext(f"tyre-{side}", f"tyre-{side}", RUBBER, TONE["tyre"], arch(RO, RI, YC, 24), -TW, TW))
    return b


def body200():
    """🔭️ The 1:200 body (feature 30 mm): footing crown and base plate one 40 mm slab, the post head one 40 mm block, the bearing one drum."""
    b = [box("footing-slab", "footing", CONC, TONE["footing"], (0, BASE[1] / 2, 0), (2 * CROWN[2], BASE[1], 2 * CROWN[2]))]
    b.append(box("post-tube", "post", GALV, TONE["galv"], ((POST_Y[0] + POST_Y[1]) / 2 - CROWN[1], 0, 0), (POST_Y[1] - POST_Y[0], 2 * POST_H, 2 * POST_H)))
    b.append(box("post-head", "post", GALV, TONE["galv"], ((TOPPLATE[0] + P[1]) / 2 - CROWN[1], 0, 0), (P[1] - TOPPLATE[0], 2 * TOPPLATE[2], 2 * HOUSING_W)))
    b.append(cyl("bearing-drum", "lift", GALV, TONE["galv-dark"], (0, 0, -AXLE_Z), (0, 0, 1), 2 * AXLE_Z, HOUSING_RO))
    cheek = bpoly("beam", cheek_profile(6))
    b.append(ext("cheek-left", "beam", GALV, TONE["galv"], cheek, CHEEK_Z[0], CHEEK_Z[1], sheet=True))
    b.append(ext("cheek-right", "beam", GALV, TONE["galv"], cheek, -CHEEK_Z[1], -CHEEK_Z[0], sheet=True))
    for side, sg in SIDES:
        bone = f"beam-{side}"
        b.append(box(f"beam-{side}-body", bone, LARCH, TONE["beam"], (BL / 2, 0, 0), (BL, BT1 - BT0, 2 * BW)))
        su, sv = bl(bone, sg * sum(SEAT_S) / 2, sum(SEAT_T) / 2)
        b.append(box(f"seat-{side}", bone, FRESH if side == "fore" else LARCH, TONE["seat-new"] if side == "fore" else TONE["seat-old"], (su, sv, 0), (SEAT_S[1] - SEAT_S[0], SEAT_T[1] - SEAT_T[0], 2 * SEAT_W)))
        ou, ov = bl(bone, sg * HS, BT1)
        ax = ldir(bone, *nrm())
        b.append(cyl(f"handle-{side}-post", bone, PAINT, TONE["paint"], (ou, ov, 0), (ax[0], ax[1], 0), GRIP_T - BT1, HPOST_R))
        gu, gv = bl(bone, sg * HS, GRIP_T)
        for name, z0, z1, mat in (("worn-right", -GRIP_W, -WORN[0], WORNS), ("middle", -WORN[0], WORN[0], PAINT), ("worn-left", WORN[0], GRIP_W, WORNS)):
            b.append(cyl(f"grip-{side}-{name}", bone, mat, TONE["worn"] if mat == WORNS else TONE["paint"], (gu, gv, z0), (0, 0, 1), z1 - z0, GRIP_R))
    for side, _ in SIDES:
        b.append(ext(f"tyre-{side}", f"tyre-{side}", RUBBER, TONE["tyre"], arch(RO, RI, YC, 16), -TW, TW))
    return b


F500 = 0.075
HPLATE100 = 0.015


def body500():
    """🛰️ The 1:500 body (feature 75 mm): a 75 mm footing slab, the post, the bearing as one hub block, the beam with 75 mm seat pads, T handles of 75 mm bars, the two tyre arches."""
    b = [box("footing-slab", "footing", CONC, TONE["footing"], (0, F500 / 2, 0), (2 * CROWN[2], F500, 2 * CROWN[2]))]
    top = P[1] - CHEEK_R
    b.append(box("post-tube", "post", GALV, TONE["galv"], ((F500 + top) / 2 - CROWN[1], 0, 0), (top - F500, 2 * POST_H, 2 * POST_H)))
    b.append(ext("hub-block", "beam", GALV, TONE["galv"], bpoly("beam", cheek_profile(6)), -CHEEK_Z[1], CHEEK_Z[1]))
    h = F500 / 2
    for side, sg in SIDES:
        bone = f"beam-{side}"
        b.append(box(f"beam-{side}-body", bone, LARCH, TONE["beam"], (BL / 2, 0, 0), (BL, BT1 - BT0, 2 * BW)))
        su, sv = bl(bone, sg * sum(SEAT_S) / 2, BT1 + h)
        b.append(box(f"seat-{side}", bone, FRESH if side == "fore" else LARCH, TONE["seat-new"] if side == "fore" else TONE["seat-old"], (su, sv, 0), (SEAT_S[1] - SEAT_S[0], F500, 2 * SEAT_W)))
        g5 = GRIP_T + GRIP_R - h
        pu, pv = bl(bone, sg * HS, (BT1 + g5 - h) / 2)
        b.append(box(f"handle-{side}-post", bone, PAINT, TONE["paint"], (pu, pv, 0), (F500, g5 - h - BT1, F500)))
        gu, gv = bl(bone, sg * HS, g5)
        for name, z0, z1, mat in (("worn-right", -GRIP_W, -WORN[0], WORNS), ("middle", -WORN[0], WORN[0], PAINT), ("worn-left", WORN[0], GRIP_W, WORNS)):
            b.append(box(f"grip-{side}-{name}", bone, mat, TONE["worn"] if mat == WORNS else TONE["paint"], (gu, gv, (z0 + z1) / 2), (F500, F500, z1 - z0)))
    for side, _ in SIDES:
        b.append(ext(f"tyre-{side}", f"tyre-{side}", RUBBER, TONE["tyre"], arch(RO, RI, YC, 12), -TW, TW))
    return b


def coarse_geometry(scale, view):
    """🗺️ The 2D drawing of one view at 1:200 or 1:500, drawn to the same numbers as the solids of that scale."""
    out = []
    el, fr = view == "elevation", view == "front"
    ux = math.sqrt(RO ** 2 - YC ** 2)
    f = F500 if scale == 500 else BASE[1]
    if el:
        out.append(sh("footing-slab", "footing", "body", 1.1, True, TONE["footing"], poly(wrect("footing", -CROWN[2], 0, CROWN[2], f)), **rgn(CONC)))
    elif fr:
        out.append(sh("footing-slab", "footing", "body", 0.5, True, TONE["footing"], poly(rect_pts(-CROWN[2], 0, CROWN[2], f)), **rgn(CONC)))
    else:
        out.append(sh("footing-slab", "footing", "body", 0.5, True, TONE["footing"], poly(rect_pts(-CROWN[2], -CROWN[2], CROWN[2], CROWN[2])), **rgn(CONC)))
    ptop = POST_Y[1] if scale == 200 else P[1] - CHEEK_R
    if el:
        out.append(sh("post-tube", "post", "body", 1.3, True, TONE["galv"], poly(wrect("post", -POST_H, f, POST_H, ptop)), **rgn(GALV)))
    elif fr:
        out.append(sh("post-tube", "post", "body", 0.7, True, TONE["galv"], poly(rect_pts(-POST_H, f, POST_H, ptop)), **rgn(GALV)))
    else:
        out.append(sh("post-tube", "post", "body", 0.7, True, TONE["galv"], poly(rect_pts(-POST_H, -POST_H, POST_H, POST_H)), **rgn(GALV)))
    if scale == 200:
        if el:
            out.append(sh("post-head", "post", "body", 1.35, True, TONE["galv"], poly(wrect("post", -TOPPLATE[2], TOPPLATE[0], TOPPLATE[2], P[1])), **rgn(GALV)))
            out.append(sh("bearing-drum", "lift", "body", 2.8, True, TONE["galv-dark"], ell(0, 0, HOUSING_RO), **rgn(GALV)))
        elif fr:
            out.append(sh("post-head", "post", "body", 0.75, True, TONE["galv"], poly(rect_pts(-HOUSING_W, TOPPLATE[0], HOUSING_W, P[1])), **rgn(GALV)))
            out.append(sh("bearing-drum", "lift", "body", 2.8, True, TONE["galv-dark"], poly(rect_pts(-AXLE_Z, P[1] - HOUSING_RO, AXLE_Z, P[1] + HOUSING_RO)), **rgn(GALV)))
        else:
            out.append(sh("post-head", "post", "body", 0.75, True, TONE["galv"], poly(rect_pts(-TOPPLATE[2], -HOUSING_W, TOPPLATE[2], HOUSING_W)), **rgn(GALV)))
            out.append(sh("bearing-drum", "lift", "body", 2.8, True, TONE["galv-dark"], poly(rect_pts(-HOUSING_RO, -AXLE_Z, HOUSING_RO, AXLE_Z)), **rgn(GALV)))
        cheek = bpoly("beam", cheek_profile(6))
        if el:
            out.append(sh("cheek-right", "beam", "body", 2.6, True, TONE["galv"], poly(cheek), **rgn(GALV)))
        else:
            out.append(sh("cheek-left", "beam", "body", 2.6, True, TONE["galv"], poly(cheek), solid=sw(*CHEEK_Z), **rgn(GALV)))
            out.append(sh("cheek-right", "beam", "body", 2.6, True, TONE["galv"], poly(cheek), solid=sw(-CHEEK_Z[1], -CHEEK_Z[0]), **rgn(GALV)))
    else:
        cheek = bpoly("beam", cheek_profile(6))
        out.append(sh("hub-block", "beam", "body", 2.6, True, TONE["galv"], poly(cheek), solid=None if el else sw(-CHEEK_Z[1], CHEEK_Z[1]), **rgn(GALV)))
    h = F500 / 2
    for side, sg in SIDES:
        bone = f"beam-{side}"
        zo = 2.0 if not fr else (2.0 if side == "fore" else 1.2)
        out.append(sh(f"beam-{side}-body", bone, "body", zo, True, TONE["beam"], poly(beam_profile(0.0)), solid=None if el else sw(-BW, BW), **rgn(LARCH)))
        st1 = SEAT_T[1] if scale == 200 else BT1 + F500
        mat = FRESH if side == "fore" else LARCH
        out.append(sh(f"seat-{side}", bone, "body", zo + 0.2, True, TONE["seat-new"] if side == "fore" else TONE["seat-old"], poly(brect(bone, sg * SEAT_S[0], sg * SEAT_S[1], BT1, st1)), solid=None if el else sw(-SEAT_W, SEAT_W), **rgn(mat, f"seat-{side}")))
        pr = HPOST_R if scale == 200 else h
        g5 = GRIP_T if scale == 200 else GRIP_T + GRIP_R - h
        ptop2 = GRIP_T if scale == 200 else g5 - h
        out.append(sh(f"handle-{side}-post", bone, "body", zo + 0.35, True, TONE["paint"], poly(brect(bone, sg * HS - pr, sg * HS + pr, BT1, ptop2)), solid=None if el else sw(-pr, pr), **rgn(PAINT)))
        gu, gv = bl(bone, sg * HS, GRIP_T)
        gprof = circle(gu, gv, GRIP_R, 12) if scale == 200 else brect(bone, sg * HS - h, sg * HS + h, g5 - h, g5 + h)
        if el:
            out.append(sh(f"grip-{side}-worn-right", bone, "body", zo + 0.4, True, TONE["worn"], poly(gprof), regions=["grip-worn", "handle-paint"], material=WORNS))
        elif scale == 500:
            out.append(sh(f"grip-{side}", bone, "body", zo + 0.4, True, TONE["worn"], poly(gprof), solid=sw(-GRIP_W, GRIP_W), regions=["grip-worn", "handle-paint"], material=WORNS))
        else:
            for name, z0, z1, m in (("worn-right", -GRIP_W, -WORN[0], WORNS), ("middle", -WORN[0], WORN[0], PAINT), ("worn-left", WORN[0], GRIP_W, WORNS)):
                out.append(sh(f"grip-{side}-{name}", bone, "body", zo + 0.4, True, TONE["worn"] if m == WORNS else TONE["paint"], poly(gprof), solid=sw(z0, z1), **rgn(m)))
        if el and scale == 200:
            out.append(sh(f"beam-{side}-check", bone, "detail", 2.05, False, 0.3, poly([(0.3, 0.02 * sg), (0.8, 0.026 * sg), (1.3, 0.018 * sg)], False), **rgn(LARCH)))
    for side, sg in SIDES:
        if el:
            g = poly(arch(RO, RI, YC, 16 if scale == 200 else 12))
            z = 1.0
        elif fr:
            g = poly(tyre_front(0.0)) if scale == 200 else poly(rect_pts(-TW, 0, TW, YC + RO))
            z = 3.0 if side == "fore" else 0.4
        else:
            g = poly(rect_pts(sg * TX - ux, -TW, sg * TX + ux, TW))
            z = 0.4
        out.append(sh(f"tyre-{side}", f"tyre-{side}", "body", z, True, TONE["tyre"], g, **rgn(RUBBER)))
    return out


FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def structure_list(scale, body):
    """🏗️ The structural idealisation of one scale: members along post, hub and both beam halves, joints at the rig joints, one support solid
    per permanent support anchor (the tyre stops and the slings are switched supports without a solid of their own), a mass marker at the centre of mass of every bone that carries mass."""
    F = FEATURE[scale]
    ids = {s["id"] for s in body}
    out = []
    ptop = {50: TOPPLATE[1], 100: WEB[1], 200: P[1], 500: P[1] - CHEEK_R}[scale]
    pbot = {50: POST_Y[0], 100: POST_Y[0], 200: BASE[1], 500: F500}[scale]
    out.append(box("member-post", "post", GALV, 0.5, ((pbot + ptop) / 2 - CROWN[1], 0, 0), (ptop - pbot, 2 * POST_H, 2 * POST_H)) | {"member": "post", "part": "member"})
    if "hub-block" in ids:
        out.append(ref("member-hub", "hub-block", member="beam", part="member"))
    else:
        out.append(ref("member-hub-left", "cheek-left", member="beam", part="member"))
        out.append(ref("member-hub-right", "cheek-right", member="beam", part="member"))
    for side, _ in SIDES:
        out.append(ref(f"member-beam-{side}", f"beam-{side}-body", member=f"beam-{side}", part="member"))
    if "base-plate" in ids:
        out.append(ref("joint-post-base", "base-plate", member="post", part="joint"))
    else:
        out.append(box("joint-post-base", "post", GALV, 0.2, (pbot - CROWN[1] + F / 2, 0, 0), (F, 0.12, 0.12)) | {"member": "post", "part": "joint"})
    if "housing-lower" in ids:
        out.append(ref("joint-bearing", "housing-lower", member="post", part="joint"))
    elif "post-head" in ids:
        out.append(ref("joint-bearing", "post-head", member="post", part="joint"))
    if "axle" in ids:
        out.append(ref("joint-axle", "axle", member="beam", part="joint"))
    elif "bearing-drum" in ids:
        out.append(ref("joint-axle", "bearing-drum", member="beam", part="joint"))
    else:
        out.append(solid("joint-axle", "beam", GALV, 0.2, "ellipsoid", {"radii": [CHEEK_R, CHEEK_R, CHEEK_Z[1]]}) | {"member": "beam", "part": "joint"})
    r = max(F / 2 + 0.001, 0.03)
    if scale == 500:
        out.append(solid("joint-beam-centre", "beam", GALV, 0.2, "box", {"size": [2 * r, 2 * r, 2 * BW], "transform": {"translation": [E, 0.0, 0.0]}}) | {"member": "beam", "part": "joint"})
    else:
        out.append(solid("joint-beam-centre", "beam", GALV, 0.2, "ellipsoid", {"radii": [r, r, BW], "transform": {"translation": [E, 0.0, 0.0]}}) | {"member": "beam", "part": "joint"})
    out.append(ref("support-footing", "footing-crown" if "footing-crown" in ids else "footing-slab", member="footing", part="support", anchor="footing-anchor"))
    ux = math.sqrt(RO ** 2 - YC ** 2)
    for side, sg in SIDES:
        out.append(box(f"support-tyre-{side}", f"tyre-{side}", RUBBER, 0.15, (0, F / 2, 0), (2 * ux, F, 2 * TW)) | {"member": f"tyre-{side}", "part": "support", "anchor": f"tyre-{side}-ground"})
    for bone, (kg, cu, cv) in BONE_MASS.items():
        rm = max(F / 2 + 0.001, 0.025)
        if bone == "footing":
            rm = min(rm, {50: CROWN[1], 100: CROWN[1], 200: BASE[1], 500: F500}[scale] / 2)
        if scale == 500:
            out.append(solid(f"mass-{bone}", bone, GALV, 0.2, "box", {"size": [2 * rm, 2 * rm, 2 * rm], "transform": {"translation": [r6(cu), r6(cv), 0.0]}}, role="detail") | {"member": bone, "part": "mass"})
        else:
            out.append(solid(f"mass-{bone}", bone, GALV, 0.2, "ellipsoid", {"radii": [rm, rm, rm], "transform": {"translation": [r6(cu), r6(cv), 0.0]}}, role="detail") | {"member": bone, "part": "mass"})
    return out


SLING_U = 0.90
BONE_MASS = {}


def masses():
    """⚖️ Bone masses from the 1:50 solids (section x density) and the catalogue mass of the tyres, whose buried half is not drawn."""
    MASS.clear()
    body50(True)
    for bone, (m, mu, mv) in MASS.items():
        BONE_MASS[bone] = (round(m, 2), mu / m, mv / m)


REFERENCES = [
    {"id": "en-1176-1", "citation": "EN 1176-1:2017 Playground equipment and surfacing - Part 1: General safety requirements and test methods (finger and foot entrapment, handgrip diameters, free height of fall, impact area, foundations below the playing surface or with rounded edges)."},
    {"id": "en-1176-6", "citation": "EN 1176-6:2017 Playground equipment and surfacing - Part 6: Additional specific safety requirements and test methods for rocking equipment (type 1 single-pivot seesaw: inclination of the beam limited to 20 degrees, damping under the seats, handholds for each user, ground clearance under the seats)."},
    {"id": "en-1176-7", "citation": "EN 1176-7:2020 Playground equipment and surfacing - Part 7: Guidance on installation, inspection, maintenance and operation (routine, operational and annual main inspection; lubrication and wear of bearings and moving parts, replacement of worn components)."},
    {"id": "en-1177", "citation": "EN 1177:2018 Impact attenuating playground surfacing - Methods of test for determination of impact attenuation (turf or topsoil accepted up to a critical fall height of 1.0 m)."},
    {"id": "din-18034", "citation": "DIN 18034:2020-10 Spielplaetze und Freiraeume zum Spielen - Anforderungen fuer Planung, Bau und Betrieb (municipal playground planning and operation in Germany)."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber - Strength classes (C24: bending strength 24 MPa, mean modulus of elasticity parallel 11 GPa, characteristic density 350 kg/m3)."},
    {"id": "en-350", "citation": "EN 350:2016 Durability of wood and wood-based products - Testing and classification of the durability to biological agents (European larch heartwood durability class 3-4 in ground contact)."},
    {"id": "wagenfuehr-2007", "citation": "Wagenfuehr, R. (2007). Holzatlas, 6th ed., Carl Hanser, Munich (European larch, Larix decidua: mean density about 590 kg/m3 at 12-15 % moisture, heartwood reddish brown, weathering to silver grey)."},
    {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (SHS 100 x 100 x 4: 11.7 kg/m, A = 14.9 cm2, I = 226 cm4; CHS 42.4 x 3.2: 3.09 kg/m; CHS 33.7 x 2.6: 1.99 kg/m)."},
    {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels - Part 2: Non-alloy structural steels (S235JR plates and flats, yield strength 235 MPa, E = 210 GPa)."},
    {"id": "en-iso-1461", "citation": "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles (zinc coatings of posts, plates and fasteners; dulling to a grey zinc patina in service)."},
    {"id": "etrto-2020", "citation": "ETRTO Standards Manual 2020, passenger car tyres (195/65 R15: section width 195 mm, overall diameter 635 mm, rim diameter 381 mm; a tyre of this size weighs about 8 kg)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: Densities, self-weight, imposed loads for buildings (Annex A: plain concrete 24 kN/m3, steel 77 kN/m3)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (softwood 500 kg/m3: 0.13 W/mK, 1600 J/kgK; concrete 2400 kg/m3: 2.0 W/mK, 1000 J/kgK; steel 50 W/mK, 450 J/kgK; hard rubber 1200 kg/m3: 0.17 W/mK, 1400 J/kgK)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.11 and A.12: total emissivity and solar absorptivity of oxidised and galvanised steel, paints, wood, concrete and rubber)."},
    {"id": "iso-7730-2005", "citation": "ISO 7730:2005 Ergonomics of the thermal environment - Analytical determination and interpretation of thermal comfort (outer clothing surface temperatures of seated persons about 28-32 degC)."},
    {"id": "din-33402-2", "citation": "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values (body masses of children aged 6 to 9 years, about 20 to 30 kg)."},
]

MATERIALS = [
    {"id": LARCH, "title": T("Weathered larch, silver grey and checked", "Verwitterte Lärche, silbergrau und rissig"), "emissivity": 0.9, "solarAbsorptivity": 0.72, "densityKgM3": 590, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "authored-estimate", "reference": "wagenfuehr-2007"},
    {"id": FRESH, "title": T("Fresh larch of the replaced seat board", "Frische Lärche des ersetzten Sitzbretts"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 590, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "authored-estimate", "reference": "wagenfuehr-2007"},
    {"id": GALV, "title": T("Hot-dip galvanised steel, dull zinc patina", "Feuerverzinkter Stahl, matte Zinkpatina"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": PAINT, "title": T("Steel tube painted moss green (RAL 6005)", "Stahlrohr moosgrün lackiert (RAL 6005)"), "emissivity": 0.9, "solarAbsorptivity": 0.88, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": WORNS, "title": T("Grip steel worn bare by hands, lightly rusted", "Von Händen blank gegriffener Stahl, leicht angerostet"), "emissivity": 0.45, "solarAbsorptivity": 0.7, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": RUBBER, "title": T("Old car tyre rubber, sun-cracked", "Altreifengummi, sonnenrissig"), "emissivity": 0.92, "solarAbsorptivity": 0.92, "densityKgM3": 1200, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"},
    {"id": CONC, "title": T("Concrete C20/25 of the point foundation, weathered", "Beton C20/25 des Punktfundaments, verwittert"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 2400, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 30000000000, "source": "literature", "reference": "iso-10456-2007"},
]


def rig():
    """🦴️ The rig: footing and post fixed, the axle that lifts out vertically, the beam hub rocking about the axle, the two beam halves and the two tyre dampers."""
    bm = {k: v for k, v in BONE_MASS.items()}

    def mass(bone, src="literature", refid="en-10219-2"):
        kg, cu, cv = bm[bone]
        return {"kg": kg, "comX": r6(cu), "comY": r6(cv), "distribution": "point", "source": src, "reference": refid}

    beam_sec = {"areaM2": 0.028, "secondMomentM4": 9.333e-05, "extremeFibreM": 0.1, "material": LARCH, "title": T("Larch beam 140 x 200 mm (C24, EN 338)", "Lärchenbalken 140 x 200 mm (C24, EN 338)")}
    return {"kind": "mechanical", "root": "ground", "bones": [
        {"id": "ground", "parent": None, "title": T("Ground frame", "Bezugsrahmen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "fixed"}},
        {"id": "footing", "parent": "ground", "title": T("Crown of the point foundation", "Kopf des Punktfundaments"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "fixed"}, "mass": mass("footing", "literature", "en-1991-1-1")},
        {"id": "post", "parent": "ground", "title": T("Bearing post SHS 100 x 100 x 4 with base plate and lower bearing shell", "Lagerpfosten Vierkantrohr 100 x 100 x 4 mit Fußplatte und unterer Lagerschale"), "x": 0, "y": CROWN[1], "angleDeg": 90, "lengthM": r6(P[1] - CROWN[1]), "joint": {"type": "fixed"}, "mass": mass("post"), "section": {"areaM2": 0.00149, "secondMomentM4": 2.26e-06, "extremeFibreM": 0.05, "material": GALV, "title": T("SHS 100 x 100 x 4 (EN 10219-2)", "Vierkantrohr 100 x 100 x 4 (EN 10219-2)")}},
        {"id": "lift", "parent": "ground", "title": T("Axle with bearing cap (lifts out with the beam)", "Achse mit Lagerdeckel (wird mit dem Balken ausgehoben)"), "x": 0, "y": P[1], "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.35}, "mass": mass("lift", "literature", "en-10025-2")},
        {"id": "beam", "parent": "lift", "title": T("Beam hub: bearing cheeks rocking about the axle", "Balkennabe: um die Achse kippende Lagerwangen"), "x": 0, "y": 0, "angleDeg": r6(TH + 90), "lengthM": E, "joint": {"type": "revolute", "minDeg": r6(-2 * TH), "maxDeg": 0}, "mass": mass("beam", "literature", "en-10025-2"), "section": {"areaM2": 0.00352, "secondMomentM4": 1.42e-05, "extremeFibreM": 0.11, "material": GALV, "title": T("Two cheek plates 220 x 8 mm (S235JR)", "Zwei Wangenbleche 220 x 8 mm (S235JR)")}},
        {"id": "beam-fore", "parent": "beam", "title": T("Fore beam half with the replaced seat and its handle", "Vordere Balkenhälfte mit ersetztem Sitz und Griff"), "x": E, "y": 0, "angleDeg": -90, "lengthM": BL, "joint": {"type": "fixed"}, "mass": mass("beam-fore", "authored-estimate", "wagenfuehr-2007"), "section": beam_sec},
        {"id": "beam-aft", "parent": "beam", "title": T("Aft beam half with the old seat and its handle", "Hintere Balkenhälfte mit altem Sitz und Griff"), "x": E, "y": 0, "angleDeg": 90, "lengthM": BL, "joint": {"type": "fixed"}, "mass": mass("beam-aft", "authored-estimate", "wagenfuehr-2007"), "section": beam_sec},
        {"id": "tyre-fore", "parent": "ground", "title": T("Fore tyre damper, half buried", "Vorderer Reifenpuffer, halb eingegraben"), "x": TX, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "fixed"}, "mass": mass("tyre-fore", "literature", "etrto-2020")},
        {"id": "tyre-aft", "parent": "ground", "title": T("Aft tyre damper, half buried", "Hinterer Reifenpuffer, halb eingegraben"), "x": -TX, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "fixed"}, "mass": mass("tyre-aft", "literature", "etrto-2020")},
    ]}


def anchors():
    """📍️ Supports (foundation, tyre bedding, the two tyre stops of the beam, the two lifting slings) and load points (seats, handles, grease nipple)."""
    ux = math.sqrt(RO ** 2 - YC ** 2)
    a = [{"id": "footing-anchor", "bone": "footing", "x": 0, "y": 0, "kind": "support", "title": T("Point foundation of the post (anchored in the ground)", "Punktfundament des Pfostens (im Boden verankert)"), "support": "fixed", "footprintLengthM": 2 * CROWN[2], "footprintWidthM": 2 * CROWN[2]}]
    for side, sg in SIDES:
        a.append({"id": f"tyre-{side}-ground", "bone": f"tyre-{side}", "x": 0, "y": 0, "kind": "support", "title": T(f"Bedding of the {side} tyre in the soil", f"Bettung des {'vorderen' if side == 'fore' else 'hinteren'} Reifens im Boden"), "support": "contact", "footprintLengthM": r6(2 * ux), "footprintWidthM": 2 * TW, "frictionCoefficient": 0.6})
    for side, sg in SIDES:
        under = -sg * (BT1 - BT0) / 2
        a.append({"id": f"beam-{side}-stop", "bone": f"beam-{side}", "x": r6(cs_u()), "y": r6(under), "kind": "support", "title": T(f"Where the {side} beam end lands on its tyre", f"Wo das {'vordere' if side == 'fore' else 'hintere'} Balkenende auf seinem Reifen aufsetzt"), "support": "contact", "normalDeg": r6(90 + TH if side == "aft" else 90 - TH), "footprintLengthM": 0.1, "footprintWidthM": 2 * BW, "frictionCoefficient": 0.8})
    for side, sg in SIDES:
        under = -sg * (BT1 - BT0) / 2
        a.append({"id": f"sling-{side}", "bone": f"beam-{side}", "x": SLING_U, "y": r6(-under), "kind": "load", "title": T(f"Lifting sling round the {side} beam half (depot crane)", f"Hebeband um die {'vordere' if side == 'fore' else 'hintere'} Balkenhälfte (Kran des Bauhofs)")})
    for side, sg in SIDES:
        u, v = bl(f"beam-{side}", sg * sum(SEAT_S) / 2, SEAT_T[1])
        a.append({"id": f"seat-{side}-point", "bone": f"beam-{side}", "x": u, "y": v, "kind": "load", "title": T(f"Sitting point of the {side} seat", f"Sitzpunkt des {'vorderen' if side == 'fore' else 'hinteren'} Sitzes")})
        u, v = bl(f"beam-{side}", sg * HS, GRIP_T)
        a.append({"id": f"grip-{side}-point", "bone": f"beam-{side}", "x": u, "y": v, "kind": "load", "title": T(f"Hand hold of the {side} grip", f"Griffpunkt des {'vorderen' if side == 'fore' else 'hinteren'} Griffs")})
    a.append({"id": "grease-nipple", "bone": "lift", "x": 0, "y": r6(HOUSING_RO + 0.016), "kind": "attachment", "title": T("Grease nipple of the bearing cap (DIN 71412)", "Schmiernippel des Lagerdeckels (DIN 71412)")})
    return a


AREA = {"beam-larch": 2.43, "seat-fore-replaced": 0.295, "seat-aft-weathered": 0.295, "handle-paint": 0.173, "grip-worn": 0.058, "bearing-galvanised": 0.54, "tyre-rubber": 1.137, "footing-concrete": 0.45}
EXPOSED = {"beam-larch": 0.72, "seat-fore-replaced": 0.81, "seat-aft-weathered": 0.81, "handle-paint": 0.82, "grip-worn": 0.88, "bearing-galvanised": 0.76, "tyre-rubber": 0.96, "footing-concrete": 1}


def layer(lid, en, de, kind, material, t):
    """🧶️ One region layer: thickness, resistance t / lambda and the areal heat capacity of the depth that one hour of heating reaches."""
    m = {x["id"]: x for x in MATERIALS}[material]
    rho, cp, lam = m["densityKgM3"], m["specificHeatJKgK"], m["conductivityWmK"]
    depth = min(t, math.sqrt(lam / (rho * cp) * 3600))
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / lam), "heatCapacityJM2K": round(rho * cp * depth), "source": "literature", "reference": "iso-10456-2007"}


def regions():
    """🌡️ Every exposed surface as a region with its weathered material."""
    def R(rid, en, de, mat, kind, lay, L, geom, sky, ref_id="incropera-2018"):
        return {"id": rid, "title": T(en, de), "kind": "surface", "model": "passive", "material": mat, "areaM2": AREA[rid], "characteristicLengthM": L, "geometry": geom, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": ref_id}
    return [
        R("beam-larch", "Greyed and checked larch beam", "Vergrauter, rissiger Lärchenbalken", LARCH, "surface", layer("larch", "Larch, the depth one hour of sun reaches (22 mm of 140)", "Lärche, die Tiefe, die eine Stunde Sonne erreicht (22 mm von 140)", "timber", LARCH, 0.07), 0.2, "horizontal-plate-up", 0.8),
        R("seat-fore-replaced", "Replaced fore seat board, fresh larch", "Ersetztes vorderes Sitzbrett, frische Lärche", FRESH, "surface", layer("board", "Larch board 45 mm", "Lärchenbrett 45 mm", "timber", FRESH, 0.045), 0.3, "horizontal-plate-up", 0.9),
        R("seat-aft-weathered", "Old aft seat board, sat hollow and greyed", "Altes hinteres Sitzbrett, durchgesessen und vergraut", LARCH, "surface", layer("board", "Larch board 45 mm", "Lärchenbrett 45 mm", "timber", LARCH, 0.045), 0.3, "horizontal-plate-up", 0.9),
        R("handle-paint", "Painted handle posts, plates and grip ends", "Lackierte Griffstützen, Platten und Griffenden", PAINT, "surface", layer("tube", "Steel tube wall 3.2 mm", "Stahlrohrwand 3,2 mm", "metal", PAINT, 0.0032), 0.042, "cylinder", 0.6),
        R("grip-worn", "Grip bands worn bare by hands", "Von Händen blank gegriffene Griffbänder", WORNS, "surface", layer("tube", "Steel tube wall 2.6 mm", "Stahlrohrwand 2,6 mm", "metal", WORNS, 0.0026), 0.034, "cylinder", 0.6),
        R("bearing-galvanised", "Galvanised post, plates, bearing, cheeks and fasteners", "Verzinkter Pfosten, Platten, Lager, Wangen und Verbindungsmittel", GALV, "surface", layer("plate", "Galvanised steel 6 mm (4 mm tube to 15 mm plate)", "Verzinkter Stahl 6 mm (4-mm-Rohr bis 15-mm-Platte)", "metal", GALV, 0.006), 0.1, "vertical-plate", 0.4),
        R("tyre-rubber", "Two half-buried car tyres, sun-cracked", "Zwei halb eingegrabene Autoreifen, sonnenrissig", RUBBER, "surface", layer("rubber", "Tyre tread rubber 12 mm", "Laufflächengummi 12 mm", "polymer", RUBBER, 0.012), 0.63, "cylinder", 0.6, "iso-10456-2007"),
        R("footing-concrete", "Concrete crown of the foundation laid bare by erosion", "Durch Erosion freigelegter Betonkopf des Fundaments", CONC, "surface", layer("concrete", "Concrete, depth of one hour of heating", "Beton, Tiefe einer Stunde Erwärmung", "masonry", CONC, 0.1), 0.45, "horizontal-plate-up", 0.7, "iso-10456-2007"),
    ]


SUN = {"beam-larch": 0.6, "seat-fore-replaced": 0.9, "seat-aft-weathered": 0.9, "handle-paint": 0.55, "grip-worn": 0.55, "bearing-galvanised": 0.25, "tyre-rubber": 0.6, "footing-concrete": 0.45}


def binding(sit=None, lifted=False):
    """🌞️ The thermal binding of a pose: every region exposed, sun shares from the summer afternoon, seats under seated children in contact."""
    r = {}
    for rid, s in SUN.items():
        b = {"exposedFraction": EXPOSED[rid], "sunlitFraction": s}
        if lifted and rid in ("bearing-galvanised", "footing-concrete"):
            b["sunlitFraction"] = 0.5
        if sit and rid in sit:
            b = {"exposedFraction": EXPOSED[rid], "sunlitFraction": 0.45, "contact": {"fraction": 0.5, "tempC": 31}}
        if rid == "footing-concrete":
            b["contact"] = {"fraction": 0.6, "tempC": 18}
        r[rid] = b
    return {"regions": r, "emitters": {}}


def child(lid, side, kg, s_seat, en, de):
    """🧒️ A seated child as an occupant load on its seat."""
    u, v = bl(f"beam-{side}", s_seat, SEAT_T[1])
    return {"id": lid, "kind": "occupant", "title": T(en, de), "bone": f"beam-{side}", "x": u, "y": v, "fxN": 0, "fyN": r6(-kg * 9.80665), "source": "authored-estimate", "reference": "din-33402-2"}


SUPPORTS_SERVICE = ["footing-anchor", "tyre-fore-ground", "tyre-aft-ground"]


def sling_loads():
    """🏗️ The crane hook as applied lifting forces: each round sling at 0.90 m from the pivot carries half the weight of the rocking part (beam, cheeks, axle and cap), whose centre of mass lies on the pivot line."""
    half = r6(sum(BONE_MASS[b][0] for b in ("lift", "beam", "beam-fore", "beam-aft")) * 9.80665 / 2)
    return [{"id": f"sling-{side}", "kind": "equipment", "title": T(f"Round sling round the {side} beam half, half of the rocking part", f"Rundschlinge um die {'vordere' if side == 'fore' else 'hintere'} Balkenhälfte, die Hälfte des Wippteils"), "anchor": f"sling-{side}", "fxN": 0, "fyN": half, "source": "authored-estimate", "reference": "en-1176-7"} for side in ("fore", "aft")]


def poses():
    """🧍️ In service (empty, aft end resting on its tyre), in use (two children, fore end down), balanced (two children level) and maintenance (beam lifted out on two slings, the bearing open for greasing)."""
    return [
        {"id": "in-service", "title": T("In service, empty", "Im Betrieb, leer"), "description": T("The empty seesaw as it stands in the park: the beam rests 10 degrees nose up on the aft tyre, because its centre of mass lies 0.16 m above the axle and tips it to whichever end is lower.", "Die leere Wippe, wie sie im Park steht: der Balken liegt mit 10 Grad Neigung auf dem hinteren Reifen auf, weil sein Schwerpunkt 0,16 m über der Achse liegt und ihn zum jeweils tieferen Ende kippt."), "bones": {}, "ground": "auto", "contacts": SUPPORTS_SERVICE + ["beam-aft-stop"], "thermal": binding(), "loads": []},
        {"id": "in-use", "title": T("In use, fore end down", "In Benutzung, vorderes Ende unten"), "description": T("Two children ride the seesaw: the heavier child (28 kg) has come down on the fore tyre and is about to push off with the feet, the lighter one (25 kg) sits up on the aft seat holding the worn grip.", "Zwei Kinder wippen: das schwerere Kind (28 kg) ist auf dem vorderen Reifen aufgesetzt und stößt sich gleich mit den Füßen ab, das leichtere (25 kg) sitzt oben auf dem hinteren Sitz und hält den abgegriffenen Griff."), "bones": {"beam": {"angleDeg": r6(-2 * TH)}}, "ground": "auto", "contacts": SUPPORTS_SERVICE + ["beam-fore-stop"], "thermal": binding(sit=("seat-fore-replaced", "seat-aft-weathered")), "loads": [child("child-fore", "fore", 28, SEAT_S[0] + 0.2, "Child on the fore seat (28 kg)", "Kind auf dem vorderen Sitz (28 kg)"), child("child-aft", "aft", 25, -(SEAT_S[0] + 0.2), "Child on the aft seat (25 kg)", "Kind auf dem hinteren Sitz (25 kg)")]},
        {"id": "balanced", "title": T("Balanced, beam level", "Ausbalanciert, Balken waagerecht"), "description": T("The two children balance the beam level in the air: the heavier child has moved 0.155 m towards the pivot so that 28 kg x 1.295 m equals 25 kg x 1.45 m; neither end touches its tyre.", "Die beiden Kinder halten den Balken waagerecht in der Luft: das schwerere Kind ist 0,155 m zum Drehpunkt gerückt, sodass 28 kg x 1,295 m gleich 25 kg x 1,45 m ist; kein Ende berührt seinen Reifen."), "bones": {"beam": {"angleDeg": r6(-TH)}}, "ground": "auto", "contacts": list(SUPPORTS_SERVICE), "thermal": binding(sit=("seat-fore-replaced", "seat-aft-weathered")), "loads": [child("child-fore", "fore", 28, 1.295, "Child on the fore seat, moved inwards (28 kg)", "Kind auf dem vorderen Sitz, nach innen gerückt (28 kg)"), child("child-aft", "aft", 25, -(SEAT_S[0] + 0.2), "Child on the aft seat (25 kg)", "Kind auf dem hinteren Sitz (25 kg)")]},
        {"id": "maintenance", "title": T("Maintenance: beam lifted out, bearing greased", "Wartung: Balken ausgehoben, Lager abgeschmiert"), "description": T("The depot crew has unbolted the bearing cap and lifts the whole rocking part (beam, cheeks, axle and cap) 0.30 m out of the lower bearing shell on two round slings at 0.90 m either side of the pivot; the post, its foundation and the tyres stay behind while the shell and the bushings are cleaned and packed with fresh grease.", "Der Bauhof hat den Lagerdeckel gelöst und hebt das ganze kippende Teil (Balken, Wangen, Achse und Deckel) an zwei Rundschlingen 0,90 m beidseits des Drehpunkts 0,30 m aus der unteren Lagerschale; Pfosten, Fundament und Reifen bleiben stehen, während Schale und Buchsen gereinigt und mit frischem Fett gefüllt werden."), "bones": {"beam": {"angleDeg": r6(-TH)}, "lift": {"dy": 0.3}}, "ground": "auto", "contacts": SUPPORTS_SERVICE, "thermal": binding(lifted=True), "loads": sling_loads() + [{"id": "grease-gun", "kind": "equipment", "title": T("Grease gun pressed onto the lower bearing shell (60 N)", "Fettpresse auf die untere Lagerschale gedrückt (60 N)"), "bone": "post", "x": r6(P[1] - CROWN[1]), "y": 0.0, "fxN": 0, "fyN": -60, "source": "authored-estimate", "reference": "en-1176-7"}]},
    ]


def clips():
    """🎞️ The see-saw ride of two children (looping) and the maintenance lift."""
    d, hold = 2.6, 0.3
    sv = [
        {"t": 0, "pose": "in-use", "ease": "linear", "bones": {"beam": {"angleDeg": r6(-2 * TH)}}},
        {"t": hold, "pose": "in-use", "ease": "smooth", "bones": {"beam": {"angleDeg": r6(-2 * TH)}}},
        {"t": d / 2, "pose": "in-use", "ease": "linear", "bones": {"beam": {"angleDeg": 0}}},
        {"t": d / 2 + hold, "pose": "in-use", "ease": "smooth", "bones": {"beam": {"angleDeg": 0}}},
        {"t": d, "pose": "in-use", "ease": "linear", "bones": {"beam": {"angleDeg": r6(-2 * TH)}}},
    ]
    full = [{"anchor": a, "fromS": 0, "toS": d} for a in SUPPORTS_SERVICE]
    seesaw = {"id": "see-saw", "title": T("See-sawing", "Wippen"), "description": T("Two children ride for one period of 2.6 s: the fore end rests 0.3 s on its tyre while the child there pushes off, the beam swings 20 degrees in 1.0 s, the aft end lands on its tyre and rests 0.3 s while the other child pushes off, and back.", "Zwei Kinder wippen eine Periode von 2,6 s: das vordere Ende liegt 0,3 s auf seinem Reifen, während das Kind dort sich abstößt, der Balken schwingt in 1,0 s um 20 Grad, das hintere Ende setzt auf seinem Reifen auf und ruht 0,3 s, während das andere Kind sich abstößt, und zurück."), "durationS": d, "fps": 30, "loop": True, "rootMotion": "none", "keys": sv, "contacts": full + [{"anchor": "beam-fore-stop", "fromS": 0, "toS": hold}, {"anchor": "beam-aft-stop", "fromS": d / 2, "toS": d / 2 + hold}], "phases": [{"id": "fore-push", "fromS": 0, "toS": hold, "title": T("Fore child pushes off", "Vorderes Kind stößt sich ab")}, {"id": "swing-up", "fromS": hold, "toS": d / 2, "title": T("Fore end rises", "Vorderes Ende steigt")}, {"id": "aft-push", "fromS": d / 2, "toS": d / 2 + hold, "title": T("Aft child pushes off", "Hinteres Kind stößt sich ab")}, {"id": "swing-down", "fromS": d / 2 + hold, "toS": d, "title": T("Fore end comes down", "Vorderes Ende senkt sich")}], "events": [{"t": hold, "kind": "contact-off", "title": T("Fore end leaves its tyre", "Vorderes Ende verlässt seinen Reifen"), "target": "beam-fore-stop"}, {"t": d / 2, "kind": "contact-on", "title": T("Aft end lands on its tyre", "Hinteres Ende setzt auf seinem Reifen auf"), "target": "beam-aft-stop"}, {"t": d / 2 + hold, "kind": "contact-off", "title": T("Aft end leaves its tyre", "Hinteres Ende verlässt seinen Reifen"), "target": "beam-aft-stop"}], "tracks": []}
    m = 12.0
    mk = [
        {"t": 0, "pose": "in-service", "ease": "smooth"},
        {"t": 3, "pose": "maintenance", "ease": "smooth", "bones": {"beam": {"angleDeg": r6(-TH)}, "lift": {"dy": 0}}},
        {"t": 7, "pose": "maintenance", "ease": "linear"},
        {"t": m, "pose": "maintenance", "ease": "linear"},
    ]
    maint = {"id": "maintenance", "title": T("Lifting the beam out for greasing", "Ausheben des Balkens zum Abschmieren"), "description": T("The crane takes the beam on two slings and turns it level in 3 s (the aft end leaves its tyre), then lifts the rocking part 0.30 m out of the opened bearing in 4 s and holds it while the shell is greased.", "Der Kran nimmt den Balken an zwei Schlingen auf und dreht ihn in 3 s waagerecht (das hintere Ende verlässt seinen Reifen), hebt dann das kippende Teil in 4 s 0,30 m aus dem geöffneten Lager und hält es, während die Schale abgeschmiert wird."), "durationS": m, "fps": 25, "loop": False, "rootMotion": "none", "keys": mk, "contacts": [{"anchor": a, "fromS": 0, "toS": m} for a in SUPPORTS_SERVICE] + [{"anchor": "beam-aft-stop", "fromS": 0, "toS": 0.4}], "phases": [{"id": "level", "fromS": 0, "toS": 3, "title": T("Beam turned level on the slings", "Balken an den Schlingen waagerecht gedreht")}, {"id": "lift", "fromS": 3, "toS": 7, "title": T("Lifted out of the bearing", "Aus dem Lager gehoben")}, {"id": "grease", "fromS": 7, "toS": m, "title": T("Bearing shell cleaned and greased", "Lagerschale gereinigt und abgeschmiert")}], "events": [{"t": 0.4, "kind": "contact-off", "title": T("Aft end leaves its tyre", "Hinteres Ende verlässt seinen Reifen"), "target": "beam-aft-stop"}, {"t": 7, "kind": "load-on", "title": T("Grease gun applied", "Fettpresse angesetzt"), "target": "grease-gun"}], "tracks": [{"target": "load:grease-gun", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 7, "v": 1}, {"t": m, "v": 1}]}]}
    return [seesaw, maint]


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def hatches(scale):
    """▤️ Larch grain along the beam and concrete stipple."""
    if scale == 50:
        return [{"id": "larch-grain", "title": T("Larch grain", "Lärchenmaserung"), "pattern": "lines", "angleDeg": TH, "spacingMm": 0.8, "penMm": 0.13}, {"id": "concrete", "title": T("Concrete", "Beton"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13}]
    if scale == 100:
        return [{"id": "larch-grain", "title": T("Larch grain", "Lärchenmaserung"), "pattern": "lines", "angleDeg": TH, "spacingMm": 0.9, "penMm": 0.1}]
    return []


def plan_traces(scale):
    """🧾️ Traces of use seen from above: the hollow and the gum spots of the old seat, the fresh board's screw rows, the checks along the beam top."""
    if scale != 50:
        return []
    out = []
    u0, v0 = bl("beam-aft", -1.31, SEAT_T[1])
    out.append(sh("seat-aft-hollow-plan", "beam-aft", "detail", 2.25, False, 0.3, poly([(u0, v0), (u0 - 0.28, v0)], False), solid=sw(-0.09, 0.09), **rgn(LARCH, "seat-aft")))
    out.append(sh("beam-fore-top-check", "beam-fore", "detail", 2.05, False, 0.3, poly([bl("beam-fore", 0.2, BT1), bl("beam-fore", 0.9, BT1)], False), solid=sw(0.015, 0.02), **rgn(LARCH)))
    out.append(sh("beam-aft-top-check", "beam-aft", "detail", 2.05, False, 0.3, poly([bl("beam-aft", -0.25, BT1), bl("beam-aft", -0.95, BT1)], False), solid=sw(-0.025, -0.02), **rgn(LARCH)))
    return out


def geometry(scale):
    """🖼️ The three views of one scale."""
    if scale in (50, 100):
        return {"elevation": pedestal_elevation(scale) + beam_assembly_shapes("elevation", scale) + traces_elevation(scale),
                "front": pedestal_front_plan("front", scale) + beam_assembly_shapes("front", scale),
                "plan": pedestal_front_plan("plan", scale) + beam_assembly_shapes("plan", scale) + plan_traces(scale)}
    return {v: coarse_geometry(scale, v) for v in ("elevation", "front", "plan")}


REP_DECISIONS = {
    50: {
        "silhouette": T("Side elevation 72 x 25 mm: the 3.5 m beam lying 10 degrees nose up on the aft tyre, with its chamfered ends, the fresh fore seat board and the greyed aft one, two T handles on base plates, the bearing cheeks with two through bolts and the axle nut, the split bearing with its lugs, cap and grease nipple on the square post, the base plate with two anchor nuts on the eroded foundation crown and the two half-buried tyres. Front view: the beam end, both seat boards, the grips with their worn bands, cheeks and bolt heads either side of the beam, axle and nuts, bearing, post, base plate and crown, the fore tyre face in front. Plan: beam, seats, grips, cheeks, axle nuts and bolt heads beside the beam, the corners of base plate and crown, the four anchor nuts and the two tyre tops.", "Seitenansicht 72 x 25 mm: der 3,5 m lange Balken liegt 10 Grad mit der Nase nach oben auf dem hinteren Reifen, mit gefasten Enden, dem frischen vorderen und dem vergrauten hinteren Sitzbrett, zwei T-Griffen auf Fußplatten, den Lagerwangen mit zwei Durchgangsschrauben und der Achsmutter, dem geteilten Lager mit Laschen, Deckel und Schmiernippel auf dem Vierkantpfosten, der Fußplatte mit zwei Ankermuttern auf dem freigespülten Fundamentkopf und den beiden halb eingegrabenen Reifen. Vorderansicht: das Balkenende, beide Sitzbretter, die Griffe mit ihren abgegriffenen Bändern, Wangen und Schraubenköpfe beidseits des Balkens, Achse und Muttern, Lager, Pfosten, Fußplatte und Kopf, vorn die Lauffläche des vorderen Reifens. Draufsicht: Balken, Sitze, Griffe, Wangen, Achsmuttern und Schraubenköpfe neben dem Balken, die Ecken von Fußplatte und Kopf, die vier Ankermuttern und die beiden Reifenscheitel."),
        "retained": T("Every part of 7.5 mm and more: bolt heads (24 x 10 mm), anchor nuts, the 11 mm grease nipple, the 20 mm grip end caps, the 8 mm cheeks and handle plates, the 12 mm tyre tread band.", "Jedes Teil ab 7,5 mm: Schraubenköpfe (24 x 10 mm), Ankermuttern, der 11-mm-Schmiernippel, die 20-mm-Griffenden, die 8-mm-Wangen und Griffplatten, das 12-mm-Laufflächenband der Reifen."),
        "merged": T("Left and right cheek, bolt head and axle nut coincide in the side elevation and are drawn once (the near, right one); the two anchor nuts behind the post are hidden.", "Linke und rechte Wange, Schraubenkopf und Achsmutter fallen in der Seitenansicht zusammen und sind einmal gezeichnet (die nahe, rechte); die zwei Ankermuttern hinter dem Pfosten sind verdeckt."),
        "omitted": T("Bushings inside the bearing, washers, screw heads of the seat boards (6 mm), the buried half of the tyres and the buried body of the point foundation.", "Buchsen im Lager, Unterlegscheiben, Schraubenköpfe der Sitzbretter (6 mm), die eingegrabene Hälfte der Reifen und der eingegrabene Körper des Punktfundaments."),
        "thinElements": T("Shrinkage checks of the beam and seat, the sat-in hollow of the old seat and the tread edge of the tyres are 0.25 mm detail lines; the sprayed tag on the aft beam half, white rust on the post and the tyre cracks are 0.13 mm texture.", "Schwindrisse von Balken und Sitz, die durchgesessene Mulde des alten Sitzes und die Laufflächenkante der Reifen sind 0,25-mm-Detaillinien; das gesprühte Tag auf der hinteren Balkenhälfte, Weißrost am Pfosten und die Reifenrisse sind 0,13-mm-Textur."),
        "hatching": T("Larch grain lines along the beam and the old seat at 0.8 mm; concrete stipple on the foundation crown.", "Lärchenmaserung längs des Balkens und des alten Sitzes mit 0,8 mm; Betonpunktierung auf dem Fundamentkopf."),
        "styles": T("Body pen 0.5, detail 0.25, texture 0.13; tones: greyed larch 0.58, fresh board 0.82, old board 0.5, moss-green paint 0.3, worn steel 0.66, zinc 0.5 and 0.4, tyres 0.14, concrete 0.72.", "Körperstift 0,5, Detail 0,25, Textur 0,13; Töne: vergraute Lärche 0,58, frisches Brett 0,82, altes Brett 0,5, moosgrüner Lack 0,3, blanker Stahl 0,66, Zink 0,5 und 0,4, Reifen 0,14, Beton 0,72."),
        "overlays": T("Beam = beam-larch, fore board = seat-fore-replaced, aft board = seat-aft-weathered, posts, plates and grip ends = handle-paint, hand bands = grip-worn, all zinc parts = bearing-galvanised, tyres = tyre-rubber, crown = footing-concrete.", "Balken = beam-larch, vorderes Brett = seat-fore-replaced, hinteres Brett = seat-aft-weathered, Stützen, Platten und Griffenden = handle-paint, Handbänder = grip-worn, alle Zinkteile = bearing-galvanised, Reifen = tyre-rubber, Kopf = footing-concrete."),
    },
    100: {
        "silhouette": T("Side elevation 36 x 12 mm: the tilted beam with both seat boards and T handles, the cheeks round the axle, the bearing halves on the post head, base plate and crown, the two tyre arches. Front and plan as at 1:50 without nuts, bolt heads and nipple.", "Seitenansicht 36 x 12 mm: der geneigte Balken mit beiden Sitzbrettern und T-Griffen, die Wangen um die Achse, die Lagerhälften auf dem Pfostenkopf, Fußplatte und Kopf, die beiden Reifenbögen. Vorder- und Draufsicht wie 1:50 ohne Muttern, Schraubenköpfe und Nippel."),
        "retained": T("Beam, seats, handle posts and grips with the worn bands, cheeks, bearing halves, axle, post, base plate, crown, tyres.", "Balken, Sitze, Griffstützen und Griffe mit den abgegriffenen Bändern, Wangen, Lagerhälften, Achse, Pfosten, Fußplatte, Kopf, Reifen."),
        "merged": T("Top plate and web are one 20 mm post head; axle and axle nuts one 46 mm drum end; the 8 mm handle plates and their welds are one 15 mm plate.", "Kopfplatte und Steg sind ein 20-mm-Pfostenkopf; Achse und Achsmuttern ein 46-mm-Zapfen; die 8-mm-Griffplatten mit ihren Schweißnähten sind eine 15-mm-Platte."),
        "omitted": T("Bolt heads, anchor nuts and the grease nipple (under 15 mm), the bore of the bearing, beam end chamfers, tag, sticker and gum spots.", "Schraubenköpfe, Ankermuttern und Schmiernippel (unter 15 mm), die Lagerbohrung, Fasen der Balkenenden, Tag, Aufkleber und Kaugummiflecken."),
        "thinElements": T("One shrinkage check per beam half and the tyre tread edge as 0.18 mm detail lines; the 8 mm cheeks stay true as declared thin plates.", "Ein Schwindriss je Balkenhälfte und die Laufflächenkante als 0,18-mm-Detaillinien; die 8-mm-Wangen bleiben als erklärte dünne Bleche wahr."),
        "hatching": T("Larch grain on the beam at 0.9 mm.", "Lärchenmaserung auf dem Balken mit 0,9 mm."),
        "styles": T("Body pen 0.35, detail 0.18; tones as at 1:50.", "Körperstift 0,35, Detail 0,18; Töne wie 1:50."),
        "overlays": T("Regions as at 1:50.", "Regionen wie 1:50."),
    },
    200: {
        "silhouette": T("Side elevation 18 x 6 mm: the tilted beam with seats and handles, the cheeks, the bearing drum on the post head, the post on its 40 mm slab and the two tyre arches; front and plan to the same parts.", "Seitenansicht 18 x 6 mm: der geneigte Balken mit Sitzen und Griffen, die Wangen, die Lagertrommel auf dem Pfostenkopf, der Pfosten auf seiner 40-mm-Platte und die beiden Reifenbögen; Vorder- und Draufsicht mit denselben Teilen."),
        "retained": T("Beam halves, seat boards, handle posts, grips in three bands, cheeks, drum, post and head, slab, tyres.", "Balkenhälften, Sitzbretter, Griffstützen, Griffe in drei Bändern, Wangen, Trommel, Pfosten und Kopf, Platte, Reifen."),
        "merged": T("Foundation crown and base plate are one 40 mm slab; top plate, web and lower shell one 40 mm head; cap, axle and nuts one 60 mm bearing drum.", "Fundamentkopf und Fußplatte sind eine 40-mm-Platte; Kopfplatte, Steg und untere Schale ein 40-mm-Kopf; Deckel, Achse und Muttern eine 60-mm-Lagertrommel."),
        "omitted": T("All fasteners, lugs and the nipple; traces of use except one check per beam half.", "Alle Verbindungsmittel, Laschen und der Nippel; Gebrauchsspuren bis auf einen Riss je Balkenhälfte."),
        "thinElements": T("Checks 0.13 mm; the cheeks are drawn at their true 8 mm beside the beam.", "Risse 0,13 mm; die Wangen sind in ihren wahren 8 mm neben dem Balken gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.25, detail 0.13; tones as at 1:50.", "Körperstift 0,25, Detail 0,13; Töne wie 1:50."),
        "overlays": T("Regions as at 1:50; the slab carries footing-concrete.", "Regionen wie 1:50; die Platte trägt footing-concrete."),
    },
    500: {
        "silhouette": T("Side elevation 7.2 x 2.5 mm: the tilted beam with 75 mm seat pads, two T handles of 75 mm bars, one hub block for cheeks and bearing on the post, the 75 mm slab and the two tyre arches.", "Seitenansicht 7,2 x 2,5 mm: der geneigte Balken mit 75-mm-Sitzpolstern, zwei T-Griffe aus 75-mm-Stäben, ein Nabenblock für Wangen und Lager auf dem Pfosten, die 75-mm-Platte und die beiden Reifenbögen."),
        "retained": T("Beam, seats, handles with worn bands, hub, post, slab, tyres.", "Balken, Sitze, Griffe mit abgegriffenen Bändern, Nabe, Pfosten, Platte, Reifen."),
        "merged": T("Cheeks, bolts, bearing and axle are one 156 mm hub block in the cheek outline; seats are generalised to 75 mm pads, handle posts and grips to 75 mm bars; crown and base plate to a 75 mm slab.", "Wangen, Schrauben, Lager und Achse sind ein 156-mm-Nabenblock im Wangenumriss; Sitze sind zu 75-mm-Polstern, Griffstützen und Griffe zu 75-mm-Stäben, Kopf und Fußplatte zu einer 75-mm-Platte verallgemeinert."),
        "omitted": T("All fasteners and every trace of use smaller than 75 mm.", "Alle Verbindungsmittel und jede Gebrauchsspur unter 75 mm."),
        "thinElements": T("None: every part is at least 75 mm.", "Keine: jedes Teil ist mindestens 75 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.18; tones as at 1:50.", "Körperstift 0,18; Töne wie 1:50."),
        "overlays": T("Regions as at 1:50; the hub block carries bearing-galvanised.", "Regionen wie 1:50; der Nabenblock trägt bearing-galvanised."),
    },
}


def representation(scale):
    """🗺️ One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": hatches(scale), "geometry": geometry(scale), "decisions": REP_DECISIONS[scale]}


VOL_DECISIONS = {
    50: [T("Body 1:50 (feature 7.5 mm), every part to its true section: the 3.5 m larch beam 140 x 200 mm as two extruded halves with 20 mm end chamfers, two 400 x 300 x 45 mm seat boards (the fore one replaced in fresh larch), T handles of CHS 42.4 on 8 mm plates with CHS 33.7 grips in five bands (painted ends and middle, two bands worn bare by hands), two 8 mm cheeks with an axle bore and two M16 through bolts, the 30 mm axle with nuts, the split bearing of 60.3 mm (lower shell with lugs on the post, cap with lugs and grease nipple on the axle), the hollow SHS 100 x 100 x 4 post with top plate and web, the 15 mm base plate with four anchor nuts on the 25 mm foundation crown, and two half-buried 195/65 R15 tyres as tread band and side walls open at the bead.", "Körper 1:50 (Merkmal 7,5 mm), jedes Teil in seinem wahren Querschnitt: der 3,5 m lange Lärchenbalken 140 x 200 mm als zwei extrudierte Hälften mit 20-mm-Endfasen, zwei Sitzbretter 400 x 300 x 45 mm (das vordere in frischer Lärche ersetzt), T-Griffe aus Rohr 42,4 auf 8-mm-Platten mit Griffrohren 33,7 in fünf Bändern (lackierte Enden und Mitte, zwei von Händen blank gegriffene Bänder), zwei 8-mm-Wangen mit Achsbohrung und zwei M16-Durchgangsschrauben, die 30-mm-Achse mit Muttern, das geteilte Lager 60,3 mm (untere Schale mit Laschen auf dem Pfosten, Deckel mit Laschen und Schmiernippel auf der Achse), der hohle Vierkantpfosten 100 x 100 x 4 mit Kopfplatte und Steg, die 15-mm-Fußplatte mit vier Ankermuttern auf dem 25-mm-Fundamentkopf und zwei halb eingegrabene Reifen 195/65 R15 als Laufband und Seitenwände, am Wulst offen."),
         T("Thermal: every body solid by reference in its region; one layer per region, no emitters, no hidden regions; the grip bands worn bare are their own region because bare steel absorbs and emits differently from the paint.", "Thermik: jeder Körper als Referenz in seiner Region; eine Schicht je Region, keine Emitter, keine verdeckten Regionen; die blank gegriffenen Griffbänder sind eine eigene Region, weil blanker Stahl anders absorbiert und abstrahlt als der Lack."),
         T("Structure: the post member is the SHS from base plate to top plate, the hub member the two cheeks, the beam members the two halves; joints at the base plate, the lower bearing shell and the axle and a node at the beam centre; a fixed support at the foundation, contact pads under the tyres and at both tyre stops of the beam, pin pads at the two sling points; mass markers at the centres of mass of footing, post, axle, hub, both beam halves and both tyres (whole tyre centre).", "Tragwerk: das Pfostenglied ist das Vierkantrohr von Fuß- bis Kopfplatte, das Nabenglied die beiden Wangen, die Balkenglieder die beiden Hälften; Gelenke an Fußplatte, unterer Lagerschale und Achse und ein Knoten in Balkenmitte; ein Festlager am Fundament, Kontaktpolster unter den Reifen und an beiden Reifenanschlägen des Balkens, Gelenkpolster an den beiden Schlingenpunkten; Massenmarken in den Schwerpunkten von Fundamentkopf, Pfosten, Achse, Nabe, beiden Balkenhälften und beiden Reifen (Reifenmitte).")],
    100: [T("Body 1:100 (feature 15 mm), authored anew: the beam halves without the 20 mm chamfers, seat boards, handle posts on 15 mm plates with the five grip bands (20 mm painted ends, worn bands, painted middle), the cheeks as declared 8 mm sheets without the bore, the bearing as two solid 60 mm half discs on a 20 mm post head (top plate and web), the axle with its nuts as one 46 mm cylinder, the hollow post, base plate and crown, each tyre one arch band; bolts, nuts and nipple (under 15 mm) are left out.", "Körper 1:100 (Merkmal 15 mm), neu erstellt: die Balkenhälften ohne die 20-mm-Fasen, Sitzbretter, Griffstützen auf 15-mm-Platten mit den fünf Griffbändern (20-mm-Lackenden, abgegriffene Bänder, lackierte Mitte), die Wangen als erklärte 8-mm-Bleche ohne Bohrung, das Lager als zwei volle 60-mm-Halbscheiben auf einem 20-mm-Pfostenkopf (Kopfplatte und Steg), Achse mit Muttern als ein 46-mm-Zylinder, der hohle Pfosten, Fußplatte und Kopf, jeder Reifen ein Bogenband; Schrauben, Muttern und Nippel (unter 15 mm) entfallen."),
          T("Thermal: every body solid by reference in its region.", "Thermik: jeder Körper als Referenz in seiner Region."),
          T("Structure: as at 1:50 with the members, joints, supports and masses sized for 15 mm.", "Tragwerk: wie 1:50, Glieder, Gelenke, Lager und Massen für 15 mm bemessen.")],
    200: [T("Body 1:200 (feature 30 mm), authored anew: crown and base plate one 40 mm slab, the post solid, top plate, web and lower shell one 40 mm head, cap, axle and nuts one 60 mm bearing drum, the cheeks still as declared 8 mm sheets (they are what the side elevation shows of the bearing), beam halves as boxes, seat boards, handle posts and three grip bands, tyre arches.", "Körper 1:200 (Merkmal 30 mm), neu erstellt: Kopf und Fußplatte eine 40-mm-Platte, der Pfosten voll, Kopfplatte, Steg und untere Schale ein 40-mm-Kopf, Deckel, Achse und Muttern eine 60-mm-Lagertrommel, die Wangen weiter als erklärte 8-mm-Bleche (sie sind, was die Seitenansicht vom Lager zeigt), Balkenhälften als Quader, Sitzbretter, Griffstützen und drei Griffbänder, Reifenbögen."),
          T("Thermal: every body solid by reference in its region; the slab carries the concrete region, the post the zinc.", "Thermik: jeder Körper als Referenz in seiner Region; die Platte trägt die Betonregion, der Pfosten das Zink."),
          T("Structure: the post member runs from the slab to the axle, joints are the post foot block, the head and the drum; supports and masses sized for 30 mm.", "Tragwerk: das Pfostenglied reicht von der Platte bis zur Achse, Gelenke sind Pfostenfußblock, Kopf und Trommel; Lager und Massen für 30 mm bemessen.")],
    500: [T("Body 1:500 (feature 75 mm), authored anew: a 75 mm slab, the 100 mm post up to the hub, one hub block of the cheek outline 156 mm wide that stands for cheeks, bolts, bearing and axle, the beam halves, 75 mm seat pads, handle posts and grips as 75 mm bars (grips in three bands), the tyre arches.", "Körper 1:500 (Merkmal 75 mm), neu erstellt: eine 75-mm-Platte, der 100-mm-Pfosten bis zur Nabe, ein Nabenblock im Wangenumriss von 156 mm Breite für Wangen, Schrauben, Lager und Achse, die Balkenhälften, 75-mm-Sitzpolster, Griffstützen und Griffe als 75-mm-Stäbe (Griffe in drei Bändern), die Reifenbögen."),
          T("Thermal: every body solid by reference in its region.", "Thermik: jeder Körper als Referenz in seiner Region."),
          T("Structure: post member, the hub block as hub member, the beam halves; the axle joint is a 100 mm node in the hub; supports and masses sized for 75 mm.", "Tragwerk: Pfostenglied, der Nabenblock als Nabenglied, die Balkenhälften; das Achsgelenk ist ein 100-mm-Knoten in der Nabe; Lager und Massen für 75 mm bemessen.")],
}


def volume(scale, body):
    """🧊️ One authored volume."""
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_of(body), "structure": structure_list(scale, body), "decisions": VOL_DECISIONS[scale]}


def document():
    """📄️ The whole asset document."""
    masses()
    ux = math.sqrt(RO ** 2 - YC ** 2)
    height = bw(HS, GRIP_T)[1] + GRIP_R
    total = round(sum(m for m, _, _ in BONE_MASS.values()), 1)
    thermal = {"environment": {"airTempC": 28, "meanRadiantTempC": 32, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325},
               "operating": [], "emitters": [], "regions": regions(), "passengers": [],
               "assumptions": [T("Summer afternoon in a municipal park: 28 degC air, 800 W/m2 sun, 1.5 m/s breeze; every surface passive, no emitters, no rated element (0 W). The dark moss-green handle paint (absorptance 0.88) and the black tyres (0.92) heat most; the silver-grey weathered larch (0.72, fresh larch 0.55) gets hotter than the replaced board beside it, and the bare grip bands absorb nearly as much as the paint but radiate far less (emissivity 0.45 against 0.9), so they are the hottest thing a child grips.", "Sommernachmittag in einem städtischen Park: 28 degC Luft, 800 W/m2 Sonne, 1,5 m/s Brise; alle Flächen passiv, keine Emitter, kein bemessenes Element (0 W). Der dunkle moosgrüne Lack der Griffe (Absorptionsgrad 0,88) und die schwarzen Reifen (0,92) erwärmen sich am stärksten; die silbergrau verwitterte Lärche (0,72, frische Lärche 0,55) wird heißer als das ersetzte Brett daneben, und die blanken Griffbänder absorbieren fast so viel wie der Lack, strahlen aber viel weniger ab (Emissionsgrad 0,45 statt 0,9), sodass sie das Heißeste sind, was ein Kind anfasst."),
                               T("Weathered surfaces differ from new ones: grey weathered larch 0.72 against 0.55 fresh, dull zinc patina 0.28 / 0.65 against bright zinc about 0.05 / 0.4, rubbed steel 0.45 / 0.7; these are authored estimates from the tables of Incropera et al. for oxidised metals, paints and wood. Heat capacities rho*cp*min(thickness, sqrt(a*3600 s)) with ISO 10456 values.", "Verwitterte Flächen unterscheiden sich von neuen: grau verwitterte Lärche 0,72 gegen 0,55 frisch, matte Zinkpatina 0,28 / 0,65 gegen blankes Zink etwa 0,05 / 0,4, blank geriebener Stahl 0,45 / 0,7; das sind Schätzwerte nach den Tabellen von Incropera et al. für oxidierte Metalle, Lacke und Holz. Wärmekapazitäten rho*cp*min(Dicke, sqrt(a*3600 s)) mit Werten nach ISO 10456."),
                               T("Seated children cover half of each seat with clothing at about 31 degC (ISO 7730) and shade it; the foundation crown sits in the soil with 60 % of its surface (underside and most of its edges) in contact at 18 degC. The exposed shares of each pose are those of the 1:50 model (seats, cheeks and plates cover parts of the beam).", "Sitzende Kinder bedecken jeden Sitz zur Hälfte mit Kleidung bei etwa 31 degC (ISO 7730) und beschatten ihn; der Fundamentkopf sitzt mit 60 % seiner Fläche (Unterseite und der größte Teil der Kanten) im Boden bei 18 degC. Die freien Anteile jeder Pose sind die des 1:50-Modells (Sitze, Wangen und Platten decken Teile des Balkens ab).")]}
    structure = {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": list(SUPPORTS_SERVICE),
                 "assumptions": [T("The post stands in a concrete point foundation (fixed support; the buried block of about 0.4 x 0.4 x 0.6 m is not part of the asset mass). In service the rocking part rests on the axle and on the aft tyre (contact); in use on the axle and on the tyre the low end has landed on; balanced only on the axle. The bearing is a pin in reality: the solver treats each pose as an instantaneously rigid body, so an unbalanced pose is carried by the tyre stop, and the balanced pose is set so that 28 kg x 1.295 m equals 25 kg x 1.45 m.", "Der Pfosten steht in einem Beton-Punktfundament (Festlager; der eingegrabene Block von etwa 0,4 x 0,4 x 0,6 m gehört nicht zur Masse des Assets). Im Betrieb liegt das kippende Teil auf der Achse und auf dem hinteren Reifen (Kontakt); in Benutzung auf der Achse und dem Reifen, auf dem das tiefe Ende aufgesetzt hat; ausbalanciert nur auf der Achse. Das Lager ist in Wirklichkeit ein Gelenk: der Löser behandelt jede Pose als momentan starren Körper, daher trägt der Reifenanschlag eine unausgeglichene Pose, und die ausbalancierte Pose ist so gesetzt, dass 28 kg x 1,295 m gleich 25 kg x 1,45 m ist."),
                                 T(f"Members: larch beam 140 x 200 mm C24 (A = 0.028 m2, I = 9.33e-5 m4) as two cantilevers from the hub, two 8 mm S235 cheeks from the axle to the beam centre line (0.16 m), SHS 100 x 100 x 4 post. Loads are the self-weights (section x density: {total:.1f} kg in all), seated children of 25 and 28 kg (DIN 33402-2 body masses of six- to nine-year-olds) and, in maintenance, the two slings of the depot crane at 0.90 m either side of the pivot, applied as their lifting forces (0.38 kN each, together the 0.76 kN weight of the rocking part) because supports of the archive are springs of equal stiffness that would share the weight with the foundation and the tyres, and a 60 N grease gun; the EN 1176-1 design load combinations are not reproduced, these are service loads.", f"Glieder: Lärchenbalken 140 x 200 mm C24 (A = 0,028 m2, I = 9,33e-5 m4) als zwei Kragarme von der Nabe, zwei 8-mm-Wangen S235 von der Achse zur Balkenachse (0,16 m), Vierkantpfosten 100 x 100 x 4. Lasten sind die Eigengewichte (Querschnitt x Dichte: zusammen {str(total).replace('.', ',')} kg), sitzende Kinder von 25 und 28 kg (Körpermassen Sechs- bis Neunjähriger nach DIN 33402-2) und in der Wartung die beiden Schlingen des Bauhofkrans 0,90 m beidseits des Drehpunkts, angesetzt als ihre Hubkräfte (je 0,38 kN, zusammen die 0,76 kN Gewicht des Wippteils), weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das Gewicht mit Fundament und Reifen teilen würden, und eine Fettpresse mit 60 N; die Bemessungskombinationen nach EN 1176-1 werden nicht nachgebildet, es sind Gebrauchslasten."),
                                 T("Geometry per EN 1176-6 type 1: the tyres stop the beam at 10 degrees either way (limit 20 degrees), the highest seat is 0.97 m above the ground (free height of fall under 1.0 m, so turf is an admissible surface per EN 1177), the beam ends overhang the tyres so that feet are not trapped under a landing seat.", "Geometrie nach EN 1176-6 Typ 1: die Reifen begrenzen den Balken auf 10 Grad je Seite (Grenze 20 Grad), der höchste Sitz liegt 0,97 m über dem Boden (freie Fallhöhe unter 1,0 m, also ist Rasen nach EN 1177 als Boden zulässig), die Balkenenden kragen über die Reifen aus, sodass Füße unter einem aufsetzenden Sitz nicht eingeklemmt werden.")],
                 "clinical": False}
    asset = {
        "id": "used-seesaw", "category": "objects",
        "title": T("Used Seesaw", "Gebrauchte Wippe"),
        "summary": T("A municipal playground seesaw after many summers: a 3.5 m larch beam of 140 x 200 mm rocks on a greased split bearing on a galvanised square post, two half-buried car tyres stop it at 10 degrees either way, two T handles of painted steel tube have their grips worn bare where hands hold them; the beam has greyed silver and checked, carries a sprayed tag, the aft seat board is sat hollow and the fore one has been replaced in fresh larch.", "Eine städtische Spielplatzwippe nach vielen Sommern: ein 3,5 m langer Lärchenbalken 140 x 200 mm kippt auf einem abgeschmierten geteilten Lager auf einem verzinkten Vierkantpfosten, zwei halb eingegrabene Autoreifen begrenzen ihn auf 10 Grad je Seite, zwei T-Griffe aus lackiertem Stahlrohr sind dort blank gegriffen, wo Hände sie halten; der Balken ist silbergrau vergraut und rissig und trägt ein gesprühtes Tag, das hintere Sitzbrett ist durchgesessen, das vordere in frischer Lärche ersetzt."),
        "tags": ["used", "park", "public-space", "play", "seesaw", "rocking", "larch", "timber", "galvanised-steel", "tyre", "playground"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                       "notes": T("Catalogue-typical single-pivot timber beam seesaw of German municipal playgrounds (EN 1176-6 type 1). Beam 3.50 m, larch 140 x 200 mm; axle 0.42 m above the ground, beam centre line 0.16 m above the axle; seats 400 x 300 x 45 mm at 1.25-1.65 m from the pivot, top 0.725 m above the ground when level; T handles at 1.12 m, grip centre 0.30 m above the seat top, grip CHS 33.7 (EN 1176-1 handgrip range); tyre dampers 195/65 R15 at 1.50 m, buried to 0.21 m so that the beam underside lands on them at exactly 10 degrees. Rest pose is the empty seesaw lying on the aft tyre, so every pose angle is an offset of the hub from it (in use -20 degrees, level -10 degrees). Masses are section x density of the 1:50 parts (larch 590 kg/m3, steel 7850, concrete 2400), the tyres are their visible upper half (3.7 kg of a tyre of about 8 kg, ETRTO) because their buried half, like the buried block of the point foundation, lies below the ground and is not part of the asset; all parameters are literature values or authored estimates, nothing is measured.", "Kataloguebliche Einpunkt-Balkenwippe aus Holz städtischer Spielplätze in Deutschland (EN 1176-6 Typ 1). Balken 3,50 m, Lärche 140 x 200 mm; Achse 0,42 m über dem Boden, Balkenachse 0,16 m über der Achse; Sitze 400 x 300 x 45 mm in 1,25-1,65 m vom Drehpunkt, Oberkante waagerecht 0,725 m über dem Boden; T-Griffe bei 1,12 m, Griffmitte 0,30 m über der Sitzfläche, Griffrohr 33,7 (Griffbereich nach EN 1176-1); Reifenpuffer 195/65 R15 bei 1,50 m, so tief eingegraben (0,21 m frei), dass die Balkenunterseite genau bei 10 Grad aufsetzt. Ruhelage ist die leere Wippe auf dem hinteren Reifen, jede Posenneigung ist also ein Versatz der Nabe davon (Benutzung -20 Grad, waagerecht -10 Grad). Massen sind Querschnitt x Dichte der 1:50-Teile (Lärche 590 kg/m3, Stahl 7850, Beton 2400), die Reifen sind ihre sichtbare obere Hälfte (3,7 kg eines Reifens von etwa 8 kg, ETRTO), weil ihre eingegrabene Hälfte wie der eingegrabene Block des Punktfundaments unter dem Boden liegt und nicht zum Asset gehört; alle Parameter sind Literaturwerte oder Schätzungen, nichts ist gemessen.")},
        "dimensions": {"heightM": round(height, 3), "widthM": round(2 * (TX + ux), 3), "depthM": round(2 * CROWN[2], 3), "massKg": total},
        "materials": MATERIALS, "rig": rig(), "anchors": anchors(),
        "representations": {str(s): representation(s) for s in (500, 200, 100, 50)},
        "symbols": [], "poses": poses(), "clips": clips(), "thermal": thermal, "structure": structure,
        "volumes": {str(s): volume(s, f()) for s, f in ((50, body50), (100, body100), (200, body200), (500, body500))},
        "presentation": {"defaultPose": "in-service", "defaultClip": "see-saw", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🔣️.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    a = doc["asset"]
    print("[DEBUG] wrote", out, a["dimensions"], {k: len(v["body"]) for k, v in a["volumes"].items()})
