"""🪜 Round 3 stepladder: the real construction of a five-step aluminium platform stepladder stated by hand in world metres at rest (x ahead = the climbing side, y up, z left), the two bone frames, and helpers that turn the parts into solids and 2D shapes.

Reference: household and trade platform stepladder of the five-step class (Hailo L60 StandardLine 8160-507: platform 1.06 m, open height 1.68 m, width 0.48 m, steps 80 mm deep, 150 kg; Krause Corda), EN 131-1 functional sizes.
"""
import math

A_FRONT = 67.5
A_REAR = 74.0
H_TOP = 1.68
PITCH = 0.212
TREADS = 4
H_PLATFORM = PITCH * (TREADS + 1)
STILE = (0.060, 0.022, 0.0015)
RAIL = (0.045, 0.020, 0.0012)
ZF0, ZF1 = 0.216, 0.162
H_HINGE = 1.33
ZR_FOOT = 0.198
TAN_F = math.tan(math.radians(A_FRONT))
SIN_F = math.sin(math.radians(A_FRONT))
COS_F = math.cos(math.radians(A_FRONT))
TAN_R = math.tan(math.radians(A_REAR))
SIN_R = math.sin(math.radians(A_REAR))
COS_R = math.cos(math.radians(A_REAR))
SHOE_F = (0.050, 0.019, 0.012)
SHOE_R = (0.045, 0.016, 0.012)
REAR_RUN = H_HINGE / TAN_F + H_HINGE / TAN_R
XF = round((REAR_RUN - 0.005) / 2, 4)
STILE_Y0 = 0.030
CAP = 0.012
STILE_Y1 = H_TOP - CAP * SIN_F - 0.032 * COS_F
BAR_Y = 1.615
BAR_R = 0.014
TREAD = (0.080, 0.025)
TREAD_FRONT = 0.0485
DECK = (0.250, 0.004, 0.026)
DECK_FRONT = 0.039
PLATFORM_HALF = 0.145
CHEEK = 0.008
AXLE_Y = H_PLATFORM - 0.017
AXLE_R = 0.005
REST_R = 0.011
REST_Y = H_PLATFORM - DECK[1] - REST_R
BRACE = (0.030, 0.015)
BRACE_Y = (0.27, 0.68)
SPREADER_Y = 0.53
LINK = (0.020, 0.008, 0.002)
RIVET_R = 0.006
ALU, STEEL, RUBBER = 2700.0, 7800.0, 1100.0
SIDES = (("l", 1), ("r", -1))


def fx(y):
    """📐 World x of the front stile centre line at height y."""
    return XF - y / TAN_F


def fz(y):
    """↔️ Lateral position of the front stile centre line at height y (the pair converges upwards)."""
    return ZF0 - (ZF0 - ZF1) * y / H_TOP


HX = fx(H_HINGE)
ZR_TOP = fz(H_HINGE) - STILE[1] / 2 - 0.0018 - RAIL[1] / 2


def rx(y):
    """📐 World x of the rear rail centre line at height y."""
    return HX - (H_HINGE - y) / TAN_R


def rz(y):
    """↔️ Lateral position of the rear rail centre line at height y (the pair spreads towards the feet)."""
    return ZR_FOOT - (ZR_FOOT - ZR_TOP) * y / H_HINGE


XR = rx(0.0)
FRONT_LEN = H_TOP / SIN_F
REAR_LEN = H_HINGE / SIN_R
BONES = {
    "front-frame": {"origin": (XF, 0.0), "angle": 180.0 - A_FRONT},
    "rear-frame": {"origin": (HX, H_HINGE), "angle": 180.0 + A_REAR},
}
FRONT, REAR = "front-frame", "rear-frame"
F_DIR = (-COS_F, SIN_F)
F_NRM = (SIN_F, COS_F)
R_DIR = (COS_R, SIN_R)
R_NRM = (SIN_R, -COS_R)
TREAD_Y = [PITCH * (i + 1) for i in range(TREADS)]
REST_X = rx(REST_Y)
AXLE_X = fx(AXLE_Y)
DECK_X1 = fx(H_PLATFORM) + DECK_FRONT
DECK_X0 = DECK_X1 - DECK[0]
SPREADER_X = (fx(SPREADER_Y), rx(SPREADER_Y))
KNUCKLE_X = (SPREADER_X[0] + SPREADER_X[1]) / 2
LINK_ZF = fz(SPREADER_Y) - STILE[1] / 2 - LINK[1] / 2
LINK_ZR = rz(SPREADER_Y) + RAIL[1] / 2 + LINK[1] / 2


def r6(p):
    return [round(c + 0.0, 6) for c in p]


def rd(v):
    return round(v + 0.0, 6)


def frame(bone):
    b = BONES[bone]
    a = math.radians(b["angle"])
    return b["origin"], (math.cos(a), math.sin(a)), (-math.sin(a), math.cos(a))


def local(bone, p):
    """📍 World rest point (x, y[, z]) to the bone-local frame (u, v[, w])."""
    o, u, v = frame(bone)
    dx, dy = p[0] - o[0], p[1] - o[1]
    return [dx * u[0] + dy * u[1], dx * v[0] + dy * v[1]] + ([p[2]] if len(p) > 2 else [])


def ldir(bone, d):
    """🧭 A world direction (x, y, z) in the bone-local frame."""
    _, u, v = frame(bone)
    return [d[0] * u[0] + d[1] * u[1], d[0] * v[0] + d[1] * v[1], d[2]]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def add(a, b, s=1.0):
    return [a[i] + s * b[i] for i in range(3)]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def norm(a):
    return math.sqrt(dot(a, a))


def unit(a):
    n = norm(a)
    return [c / n for c in a]


def quat_from_axes(ex, ey, ez):
    m00, m01, m02 = ex[0], ey[0], ez[0]
    m10, m11, m12 = ex[1], ey[1], ez[1]
    m20, m21, m22 = ex[2], ey[2], ez[2]
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        w, x, y, z = 0.25 * s, (m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        w, x, y, z = (m21 - m12) / s, 0.25 * s, (m01 + m10) / s, (m02 + m20) / s
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        w, x, y, z = (m02 - m20) / s, (m01 + m10) / s, 0.25 * s, (m12 + m21) / s
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2
        w, x, y, z = (m10 - m01) / s, (m02 + m20) / s, (m12 + m21) / s, 0.25 * s
    return [round(x, 6), round(y, 6), round(z, 6), round(w, 6)]


def solid(sid, kind, bone, material, role, tone, **kw):
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(kw)
    return out


def prism(sid, bone, material, role, tone, profile, origin, ex, ey, t0, t1, holes=None, **kw):
    """🧱 A polygon (a, b) in the plane through the world point `origin` spanned by the world directions ex, ey, pushed from t0 to t1 along ex x ey."""
    ex, ey = unit(ex), unit(ey)
    lx, ly = ldir(bone, ex), ldir(bone, ey)
    s = solid(sid, "extrude", bone, material, role, tone, profile=[r6(p) for p in profile], plane="xy", **{"from": rd(t0), "to": rd(t1)}, transform={"translation": r6(local(bone, origin)), "rotation": quat_from_axes(lx, ly, cross(lx, ly))}, **kw)
    if holes:
        s["holes"] = [[r6(p) for p in h] for h in holes]
    return s


def bar(sid, bone, material, role, tone, profile, p0, p1, across, holes=None, **kw):
    """📏 A section (a, b) pushed from the world point p0 to p1; a runs along the world direction `across` made perpendicular to the axis, b completes the frame."""
    d = unit(sub(p1, p0))
    a = unit(add(across, d, -dot(across, d)))
    return prism(sid, bone, material, role, tone, profile, p0, a, cross(d, a), 0.0, norm(sub(p1, p0)), holes, **kw)


def slab(sid, bone, material, role, tone, pts, z0, z1, holes=None, **kw):
    """🍰 A world (x, y) outline pushed along z from z0 to z1."""
    s = solid(sid, "extrude", bone, material, role, tone, profile=[r6(local(bone, p)) for p in pts], plane="xy", **{"from": rd(z0), "to": rd(z1)}, **kw)
    if holes:
        s["holes"] = [[r6(local(bone, p)) for p in h] for h in holes]
    return s


def turned(sid, bone, material, role, tone, profile, origin, axis=(0.0, 0.0, 1.0), **kw):
    """⚙️ A profile [axial, radial] revolved about the world direction `axis` through the world point `origin`."""
    return solid(sid, "revolve", bone, material, role, tone, profile=[r6(p) for p in profile], axis=r6(ldir(bone, axis)), origin=r6(local(bone, origin)), **kw)


def rod(sid, bone, material, role, tone, p0, p1, r, **kw):
    """🥢 A round bar with flat ends between two world points."""
    length = norm(sub(p1, p0))
    return turned(sid, bone, material, role, tone, [[0.0, 0.0], [0.0, r], [length, r], [length, 0.0]], p0, unit(sub(p1, p0)), **kw)


def tube(sid, bone, material, role, tone, p0, p1, r, wall, **kw):
    """🧪 A hollow round tube between two world points (profile written as a loop)."""
    length = norm(sub(p1, p0))
    return turned(sid, bone, material, role, tone, [[0.0, r - wall], [0.0, r], [length, r], [length, r - wall], [0.0, r - wall]], p0, unit(sub(p1, p0)), **kw)


def ball(sid, bone, material, role, tone, radii, centre, **kw):
    return solid(sid, "ellipsoid", bone, material, role, tone, radii=r6(radii), transform={"translation": r6(local(bone, centre))}, **kw)


def block(sid, bone, material, role, tone, size, centre, ex, ey, radius=None, **kw):
    """📦 A box of `size` along the world directions ex, ey, ex x ey, centred on the world point `centre`."""
    ex, ey = unit(ex), unit(ey)
    lx, ly = ldir(bone, ex), ldir(bone, ey)
    s = solid(sid, "box", bone, material, role, tone, size=r6(size), transform={"translation": r6(local(bone, centre)), "rotation": quat_from_axes(lx, ly, cross(lx, ly))}, **kw)
    if radius:
        s["cornerRadius"] = radius
    return s


def rrect(w, h, r, cx=0.0, cy=0.0):
    """⬜ A rectangle w x h with corners rounded by r (three points per corner)."""
    out = []
    for sx, sy, a0 in ((1, 1, 0), (-1, 1, 90), (-1, -1, 180), (1, -1, 270)):
        for t in (0, 45, 90):
            a = math.radians(a0 + t)
            out.append([cx + sx * (w / 2 - r) + r * math.cos(a), cy + sy * (h / 2 - r) + r * math.sin(a)])
    return out


def stile_points(sg, y0, y1):
    """🪜 World end points of a front stile centre line between two heights."""
    return [fx(y0), y0, sg * fz(y0)], [fx(y1), y1, sg * fz(y1)]


def rail_points(sg, y0, y1):
    """🦵 World end points of a rear rail centre line between two heights."""
    return [rx(y0), y0, sg * rz(y0)], [rx(y1), y1, sg * rz(y1)]


def stile_dir(sg):
    a, b = stile_points(sg, 0.0, 1.0)
    return unit(sub(b, a))


def rail_dir(sg):
    a, b = rail_points(sg, 0.0, 1.0)
    return unit(sub(b, a))


FN3 = [F_NRM[0], F_NRM[1], 0.0]
RN3 = [R_NRM[0], R_NRM[1], 0.0]


def tread_profile(y_top, ribs=True, depth=None, thick=None):
    """👣 World (x, y) outline of a tread: extruded box profile with a rounded nose on the climbing side and anti-slip grooves on top."""
    depth = depth or TREAD[0]
    thick = thick or TREAD[1]
    xc = fx(y_top - thick / 2)
    x1 = xc + TREAD_FRONT
    x0 = x1 - depth
    pts = [[x0, y_top - thick], [x1 - 0.007, y_top - thick], [x1, y_top - thick + 0.012], [x1, y_top - 0.004], [x1 - 0.004, y_top]]
    if ribs:
        x = x1 - 0.010
        while x - 0.006 > x0 + 0.006:
            pts += [[x, y_top], [x, y_top - 0.002], [x - 0.006, y_top - 0.002], [x - 0.006, y_top]]
            x -= 0.014
    pts += [[x0, y_top]]
    return pts


def tread_box(y_top, inset):
    """👣 Plain box of the tread for the hollow structural member (inset = 0 the outer skin)."""
    xc = fx(y_top - TREAD[1] / 2)
    x1 = xc + TREAD_FRONT
    x0 = x1 - TREAD[0]
    return [[x0 + inset, y_top - TREAD[1] + inset], [x1 - 0.007 - inset * 0.4, y_top - TREAD[1] + inset], [x1 - inset, y_top - TREAD[1] + 0.012 + inset * 0.4], [x1 - inset, y_top - 0.004 - inset * 0.4], [x1 - 0.004 - inset * 0.4, y_top - inset], [x0 + inset, y_top - inset]]


def tread_half(y_top, thick=None):
    """↔️ Half length of a tread: to 2 mm inside the inner faces of the stiles."""
    return fz(y_top - (thick or TREAD[1]) / 2) - STILE[1] / 2 + 0.002


def deck_profile(ribs=True):
    """🟫 World (x, y) outline of the platform: a ribbed deck with a front and a rear rim, the rear rim hooking behind the rest tube."""
    yt = H_PLATFORM
    yb = yt - DECK[1]
    yr = yt - DECK[2]
    pts = [[DECK_X1, yr + 0.004], [DECK_X1 - 0.003, yr], [DECK_X1 - 0.003, yb], [DECK_X0 + 0.003, yb], [DECK_X0 + 0.003, yr], [DECK_X0, yr], [DECK_X0, yt]]
    if ribs:
        x = DECK_X0 + 0.012
        while x + 0.006 < DECK_X1 - 0.008:
            pts += [[x, yt], [x, yt - 0.002], [x + 0.006, yt - 0.002], [x + 0.006, yt]]
            x += 0.014
    pts += [[DECK_X1 - 0.003, yt], [DECK_X1, yt - 0.003]]
    return pts


def cheek_profile():
    """🟫 World (x, y) outline of a platform side cheek (folded rim) round the axle."""
    yt = H_PLATFORM - 0.002
    yr = H_PLATFORM - DECK[2]
    return [[DECK_X0 + 0.004, yt], [DECK_X0 + 0.004, yr + 0.006], [DECK_X0 + 0.016, yr], [DECK_X1 - 0.02, yr], [DECK_X1 - 0.004, yr + 0.006], [DECK_X1 - 0.004, yt]]


def shoe_profile(front, height=None, wall=0.003, sole=None):
    """👟 World (x, y) outline of a foot shoe: a flat sole with a sleeve round the cut end of the stile or rail."""
    h, _, s = SHOE_F if front else SHOE_R
    h = height or h
    s = sole or s
    if front:
        half = STILE[0] / 2 / SIN_F + wall
        c = fx
        toe, heel = XF + 0.045, XF - 0.043
    else:
        half = RAIL[0] / 2 / SIN_R + wall
        c = rx
        toe, heel = XR + 0.036, XR - 0.040
    return [[heel, 0.0], [toe, 0.0], [toe, s * 0.7], [c(s) + half, s], [c(h) + half, h], [c(h) - half, h], [c(s) - half, s], [heel, s * 0.7]]


def coarse_shoe(front, length, height):
    """🥾 World (x, y) outline of a coarse-scale shoe block: a flat sole of `length` on the ground whose sides lean with the stile or rail it caps."""
    c = fx if front else rx
    x0 = XF if front else XR
    dx = c(height) - c(0.0)
    return [[x0 - length / 2, 0.0], [x0 + length / 2, 0.0], [x0 + dx + length / 2, height], [x0 + dx - length / 2, height]]


def strip(c0, c1, half):
    """▰ World (x, y) outline of a straight strip of half width `half` between two centre points, its ends cut square."""
    d = unit([c1[0] - c0[0], c1[1] - c0[1], 0.0])
    n = (-d[1], d[0])
    return [[c0[0] + n[0] * half, c0[1] + n[1] * half], [c0[0] - n[0] * half, c0[1] - n[1] * half], [c1[0] - n[0] * half, c1[1] - n[1] * half], [c1[0] + n[0] * half, c1[1] + n[1] * half]]


def box_area(w, h, t):
    return w * h - (w - 2 * t) * (h - 2 * t)


def box_inertia(w, h, t):
    """📐 Second moment of a hollow box about the axis across its height h."""
    return (w * h ** 3 - (w - 2 * t) * (h - 2 * t) ** 3) / 12


def parts():
    """⚖️ Every part with its mass (kg) and world centre (x, y), per bone, from the real sections and densities."""
    out = {FRONT: [], REAR: []}
    stile = ALU * box_area(STILE[1], STILE[0], STILE[2]) * (STILE_Y1 - STILE_Y0) / SIN_F
    ym = (STILE_Y0 + STILE_Y1) / 2
    out[FRONT].append(("stiles", 2 * stile, fx(ym), ym))
    out[FRONT].append(("caps", 2 * 0.012, fx(STILE_Y1), STILE_Y1))
    out[FRONT].append(("grab bar", ALU * math.pi * (BAR_R ** 2 - (BAR_R - 0.0015) ** 2) * 2 * fz(BAR_Y), fx(BAR_Y), BAR_Y))
    for y in TREAD_Y:
        length = 2 * tread_half(y)
        out[FRONT].append(("tread", ALU * (box_area(TREAD[0], TREAD[1], 0.0015) + 0.00003) * length + 0.02, fx(y), y - TREAD[1] / 2))
    deck = ALU * (DECK[0] * 2 * PLATFORM_HALF * 0.0025 + 2 * 2 * PLATFORM_HALF * 0.022 * 0.003 + 2 * DECK[0] * 0.024 * 0.004)
    out[FRONT].append(("platform", deck + STEEL * math.pi * AXLE_R ** 2 * 2 * fz(AXLE_Y), (DECK_X0 + DECK_X1) / 2, H_PLATFORM - 0.008))
    link = STEEL * (LINK[0] + 2 * (LINK[1] - LINK[2])) * LINK[2] * ((SPREADER_X[0] - SPREADER_X[1]) / 2 + 0.024)
    out[FRONT].append(("spreader links", 2 * link + 4 * 0.012, (SPREADER_X[0] + KNUCKLE_X) / 2, SPREADER_Y))
    out[FRONT].append(("shoes", 2 * 0.055, XF, 0.02))
    rail = ALU * box_area(RAIL[1], RAIL[0], RAIL[2]) * ((H_HINGE - 0.028) / SIN_R + 0.028)
    out[REAR].append(("rails", 2 * rail, rx(H_HINGE / 2 + 0.01), H_HINGE / 2 + 0.01))
    out[REAR].append(("rest tube", ALU * math.pi * (REST_R ** 2 - (REST_R - 0.0015) ** 2) * 2 * rz(REST_Y), REST_X, REST_Y))
    for y in BRACE_Y:
        out[REAR].append(("brace", ALU * box_area(BRACE[0], BRACE[1], 0.0012) * 2 * rz(y), rx(y), y))
    out[REAR].append(("hinge bolts", 2 * 0.035, HX, H_HINGE))
    out[REAR].append(("spreader links", 2 * link + 2 * 0.012, (SPREADER_X[1] + KNUCKLE_X) / 2, SPREADER_Y))
    out[REAR].append(("shoes", 2 * 0.045, XR, 0.02))
    return out


def masses():
    """⚖️ Bone masses and centres: bone -> (kg, local u, local v, world x, world y)."""
    out = {}
    for bone, items in parts().items():
        kg = sum(p[1] for p in items)
        x = sum(p[1] * p[2] for p in items) / kg
        y = sum(p[1] * p[3] for p in items) / kg
        u, v = local(bone, (x, y))
        out[bone] = (round(kg, 2), round(u, 3), round(v, 3), x, y)
    return out


def sections():
    """📐 Section of each bone: the pair of hollow box profiles bent in the plane of the side elevation."""
    return {
        FRONT: {"areaM2": round(2 * box_area(STILE[1], STILE[0], STILE[2]), 7), "secondMomentM4": float("%.4g" % (2 * box_inertia(STILE[1], STILE[0], STILE[2]))), "extremeFibreM": STILE[0] / 2},
        REAR: {"areaM2": round(2 * box_area(RAIL[1], RAIL[0], RAIL[2]), 7), "secondMomentM4": float("%.4g" % (2 * box_inertia(RAIL[1], RAIL[0], RAIL[2]))), "extremeFibreM": RAIL[0] / 2},
    }


def shape(sid, bone, role, z, filled, tone, kind, **kw):
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update({k: v for k, v in kw.items() if v is not None})
    out["kind"] = kind
    return out


def poly2(sid, bone, role, z, filled, tone, pts, closed=True, **kw):
    out = shape(sid, bone, role, z, filled, tone, "poly", **kw)
    out["points"] = [[round(p[0] + 0.0, 5), round(p[1] + 0.0, 5)] for p in pts]
    out["closed"] = closed
    return out


def ellipse2(sid, bone, role, z, filled, tone, cx, cy, rx_, ry_, **kw):
    out = shape(sid, bone, role, z, filled, tone, "ellipse", **kw)
    out.update({"cx": round(cx, 5), "cy": round(cy, 5), "rx": round(rx_, 5), "ry": round(ry_, 5)})
    return out


def coarse_shoe_z(front, width):
    """👟 Lateral centre of a coarse shoe block: its outer face stays on the real outer face of the shoe."""
    return (fz(0.0) + SHOE_F[1] - width / 2) if front else min(rz(0.0), fz(0.0) + SHOE_F[1] - width / 2)
