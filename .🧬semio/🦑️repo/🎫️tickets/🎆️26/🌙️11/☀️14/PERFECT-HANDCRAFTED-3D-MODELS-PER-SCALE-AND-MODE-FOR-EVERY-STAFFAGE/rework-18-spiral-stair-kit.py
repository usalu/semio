"""🌀 Rework 18 spiral stair: the real geometry of the stair (fourteen treads with nosing lap and the landing platform as unit 15, hubs, column running on as the landing post, balustrade, helical rail with the two landing guards) and the small writers every script states its parts with.

World frame: x ahead, y up, z lateral (left +); azimuth phi from +x towards +z. Bone frames: column (u = y, v = -x, w = z), tread-k (u = x, v = y - k rise, w = z), balustrade (u = x, v = y, w = z).
References: DIN 18065 (nosing lap, handrail height), MBO section 38 (openings), EN 10219-2 (CHS 168.3 x 6.3, 177.8 x 4.0, 42.4 x 2.6).
"""
import math

RHO = 7850.0
RISE = 2.8 / 15
STEP = 22.5
R_OUT = 0.8
R_CORE = 0.08415
T_CORE = 0.0063
R_HUB = 0.0889
T_HUB = 0.004
R_HUB_IN = R_HUB - T_HUB
PLATE_T = 0.005
PAN = 0.07
LAP = 0.03
BASE_R = 0.2
BASE_T = 0.02
CAP_T = 0.008
TOP = 2.8
R_RAIL = 0.775
RAIL_R = 0.0212
RAIL_T = 0.0026
H_MID = 1.0
BAL_R = 0.006
BAL_OFF = (-7.5, 0.0, 7.5)
NEWEL_PHI = 3.75
TURN_PHI = 315.0
LAND = 0.8
LAND_C = math.sqrt(0.5)
RAIL_LEVEL = RISE + H_MID + RISE * (TURN_PHI - STEP / 2) / STEP
END_T = 0.775
END_PHI = TURN_PHI + math.degrees(END_T / R_RAIL)
INNER_R = 0.025
POST_TOP = RAIL_LEVEL + RAIL_R
LAND_BARS_OUT = 6
LAND_BARS_IN = 5
FIX_R = (0.2, 0.6)
PACK_T = 0.008
BOLT_R = 0.006
POST_R = 0.0212
FLANGE_R = 0.035
FLANGE_T = 0.006
ANCHOR_R = 0.14
STUD_R = 0.008
NUT_AF = 0.024
NUT_H = 0.016
WASHER_T = 0.0


def r5(x):
    return round(x, 5) + 0.0


def phi_mid(k):
    """📐 Azimuth of the centre of tread k in degrees."""
    return STEP / 2 + STEP * (k - 1)


def rail_y(phi):
    """〰️ Height of the handrail centre line above the floor at path angle phi (degrees): 1.0 m above the tread top at the tread centre on the helix, level over the landing (0.907 m above it)."""
    return RISE + H_MID + RISE * (min(phi, TURN_PHI) - phi_mid(1)) / STEP


def land(r, t):
    """🧭 Landing coordinates to plan (x, z): r along the ray at 315 degrees, t along the walking direction there (the ray at 45 degrees)."""
    return (r + t) * LAND_C, (t - r) * LAND_C


def rail_xz(phi):
    """🛤️ Plan point of the outer handrail at path angle phi: the helix of radius 0.775 m up to 315 degrees, then its tangent along the outer landing edge."""
    if phi <= TURN_PHI:
        return polar(R_RAIL, phi)
    return land(R_RAIL, R_RAIL * math.radians(phi - TURN_PHI))


def rail_point(phi):
    x, z = rail_xz(phi)
    return x, rail_y(phi), z


def landing_outline(lap, inset, r_in, step, inner):
    """⬛ Plan outline (x, z) of the landing: the 0.8 m square between the rays at 315 and 45 degrees, its nosing edge shifted back by the lap, cut by the hub."""
    lo, hi = -lap + inset, LAND - inset
    pts = [(math.sqrt(r_in * r_in - lo * lo), lo), (hi, lo), (hi, hi), (inset, hi), (inset, math.sqrt(r_in * r_in - inset * inset))]
    if inner:
        a0, a1 = math.degrees(math.atan2(pts[4][1], pts[4][0])), math.degrees(math.atan2(lo, pts[0][0]))
        n = max(1, int(math.ceil((a0 - a1) / (step * 2))))
        pts += [(r_in * math.cos(math.radians(a0 + (a1 - a0) * i / n)), r_in * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(1, n)]
    return [land(r, t) for r, t in pts]


def polar(r, phi):
    a = math.radians(phi)
    return r * math.cos(a), r * math.sin(a)


def line_point(a_deg, off, r):
    """📏 Point at radius r on the line parallel to the ray at a_deg, shifted by off towards increasing azimuth."""
    a = math.radians(a_deg)
    s = math.sqrt(r * r - off * off)
    return (s * math.cos(a) - off * math.sin(a), s * math.sin(a) + off * math.cos(a))


def angle_of(p):
    return math.degrees(math.atan2(p[1], p[0]))


def arc_pts(r, a0, a1, step):
    n = max(1, int(math.ceil(abs(a1 - a0) / step - 1e-9)))
    return [polar(r, a0 + (a1 - a0) * i / n) for i in range(n + 1)]


def tread_outline(k, lap=LAP, inset=0.0, r_in=R_HUB_IN, r_out=R_OUT, step=3.0, inner=True):
    """🍕 Plan outline (x, z) of tread k: the 22.5 degree sector whose leading edge (nosing) is shifted back by the lap, inset by `inset` on the three free edges; counter-clockwise."""
    if k == 15:
        return landing_outline(lap, inset, r_in, step, inner)
    a0, a1 = STEP * (k - 1), STEP * k
    o0, o1 = -lap + inset, -inset
    ro = r_out - inset
    p0, p1 = line_point(a0, o0, r_in), line_point(a0, o0, ro)
    p2, p3 = line_point(a1, o1, ro), line_point(a1, o1, r_in)
    b1, b2 = angle_of(p1), angle_of(p2)
    if b2 < b1:
        b2 += 360
    pts = [p0, p1] + arc_pts(ro, b1, b2, step)[1:-1] + [p2, p3]
    if inner:
        c3, c0 = angle_of(p3), angle_of(p0)
        while c3 < c0:
            c3 += 360
        pts += arc_pts(r_in, c3, c0, step * 2)[1:-1]
    return pts


def area_centroid(pts):
    a = cx = cy = 0.0
    for i in range(len(pts)):
        (x0, y0), (x1, y1) = pts[i], pts[(i + 1) % len(pts)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return abs(a), cx / (6 * a), cy / (6 * a)


def perimeter_free(k):
    """🧵 Length of the flange: nosing edge, outer arc and trailing edge of tread k (the hub side has none)."""
    pts = tread_outline(k, inner=False, r_in=R_HUB, step=0.5)
    return sum(math.dist(pts[i], pts[i + 1]) for i in range(len(pts) - 1))


def extent(pts, axis):
    vals = [p[axis] for p in pts]
    return min(vals), max(vals)


def tread_plan_dense(k, lap=LAP):
    return tread_outline(k, lap=lap, r_in=R_HUB, step=0.25)


def hub_span(k):
    """🧱 Height range of the hub sleeve of tread k: from the hub below (or the base plate) to the tread top."""
    return (BASE_T if k == 1 else RISE * (k - 1), RISE * k)


def baluster_feet():
    """🦯 (id, tread, azimuth) of every helix baluster: three per tread at -7.5, 0, +7.5 degrees on treads 1 to 14, the first of tread 1 replaced by the newel."""
    out = []
    for k in range(1, 15):
        for tag, d in zip("abc", BAL_OFF):
            if (k, tag) == (1, "a"):
                continue
            out.append(("baluster-%d%s" % (k, tag), k, phi_mid(k) + d))
    return out


def land_counts(coarse):
    """🔢 Balusters under the outer and the inner landing rail: 6 and 5 as built, 2 and 2 in the generalised infill of 1:200."""
    return (2, 2) if coarse else (LAND_BARS_OUT, LAND_BARS_IN)


def outer_t(i, n):
    return END_T * i / (n + 1)


def inner_t(i, n):
    return R_HUB + (END_T - R_HUB) * i / (n + 1)


def outer_bar_t(i):
    """📏 Station t of landing baluster i (1..6) on the outer guard between the turn post (t = 0) and the end post (t = 0.775): 111 mm centres, 99 mm clear."""
    return END_T * i / (LAND_BARS_OUT + 1)


def inner_bar_t(i):
    """📏 Station t of landing baluster i (1..5) on the inner guard between the column (t = 0.089) and the inner end post (t = 0.775): 114 mm centres, 102 mm clear."""
    return R_HUB + (END_T - R_HUB) * i / (LAND_BARS_IN + 1)


def bars(coarse=False):
    """🦯 Every baluster as (id, x, z, foot height, rail centre height): 41 on the helix, 6 on the outer and 5 on the inner landing guard; coarse = the generalised infill of 1:200, the middle baluster of each tread and two per landing rail."""
    n_out, n_in = land_counts(coarse)
    out = [(i, polar(R_RAIL, p)[0], polar(R_RAIL, p)[1], RISE * k, rail_y(p)) for i, k, p in baluster_feet() if not coarse or i.endswith("b")]
    out += [("baluster-o%d" % i, *land(R_RAIL, outer_t(i, n_out)), TOP, RAIL_LEVEL) for i in range(1, n_out + 1)]
    out += [("baluster-i%d" % i, *land(INNER_R, inner_t(i, n_in)), TOP, RAIL_LEVEL) for i in range(1, n_in + 1)]
    return out


def posts():
    """🪵 The three straight posts as (id, x, z, foot height, top height): newel on tread 1, turn post where the helix meets the landing, end post of the outer landing guard (the inner rail bends down into its own post)."""
    nx, nz = polar(R_RAIL, NEWEL_PHI)
    return [("newel", nx, nz, RISE, rail_y(NEWEL_PHI)), ("newel-turn", *land(R_RAIL, 0), TOP, RAIL_LEVEL), ("end-post", *land(R_RAIL, END_T), TOP, RAIL_LEVEL)]


BEND = 0.06


def inner_rail(flange=0.0):
    """↩️ Centre line of the inner landing rail, one bent tube: level from the column post to the 60 mm bend at the exit, then down as its own post to the landing (or to the top of its flange)."""
    pts = [(R_CORE - 0.004, RAIL_LEVEL), (END_T - BEND, RAIL_LEVEL)]
    pts += [(END_T - BEND + BEND * math.sin(math.radians(a)), RAIL_LEVEL - BEND + BEND * math.cos(math.radians(a))) for a in (22.5, 45, 67.5, 90)]
    pts.append((END_T, TOP + flange))
    return [(land(INNER_R, t)[0], y, land(INNER_R, t)[1]) for t, y in pts]


def rail_path(step=3.75, dome=True):
    """🛤️ Handrail sections (centre, radius) along the helix from the newel to the end post, with domed ends beyond the post axes."""
    span = END_PHI - NEWEL_PHI
    n = int(round(span / step))
    phis = [NEWEL_PHI + span * i / n for i in range(n + 1)]
    phis = sorted(set(phis + [TURN_PHI]))
    secs = [(rail_point(p), RAIL_R) for p in phis]
    if not dome:
        return secs
    per_rad = math.hypot(R_RAIL, RISE / math.radians(STEP))
    ends = []
    for th in (40, 70, 86):
        d = RAIL_R * math.sin(math.radians(th))
        dp = math.degrees(d / per_rad)
        ends.append((dp, RAIL_R * math.cos(math.radians(th))))
    head = [(rail_point(NEWEL_PHI - dp), r) for dp, r in reversed(ends)]
    tail = [(rail_point(END_PHI + math.degrees(RAIL_R * math.sin(math.radians(th)) / R_RAIL)), RAIL_R * math.cos(math.radians(th))) for th in (40, 70, 86)]
    return head + secs + tail


def rail_top():
    return max(c[1] + r for c, r in rail_path(step=0.5))


def rail_length():
    pts = [c for c, _ in rail_path(step=0.25, dome=False)]
    return sum(math.dist(pts[i], pts[i + 1]) for i in range(len(pts) - 1))


def solid(i, kind, bone, material, tone, role="body", **extra):
    return {"id": i, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **extra}


def prism(i, bone, material, tone, profile, plane, lo, hi, role="body", holes=None, **extra):
    s = solid(i, "extrude", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], plane=plane, **{"from": r5(lo), "to": r5(hi)}, **extra)
    if holes:
        s["holes"] = [[[r5(a), r5(b)] for a, b in h] for h in holes]
    return s


def revolve(i, bone, material, tone, profile, axis, origin, role="body", **extra):
    return solid(i, "revolve", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], axis=list(axis), origin=[r5(c) for c in origin], **extra)


def ellipsoid(i, bone, material, tone, radii, at, role="detail", **extra):
    return solid(i, "ellipsoid", bone, material, tone, role, radii=[r5(r) for r in radii], transform={"translation": [r5(c) for c in at]}, **extra)


def loft(i, bone, material, tone, secs, role="body", **extra):
    return solid(i, "loft", bone, material, tone, role, sections=[{"at": [r5(c) for c in at], "rx": r5(r), "ry": r5(r)} for at, r in secs], capStart=True, capEnd=True, **extra)


def circle(r, n, phase=0.0):
    return [(r * math.cos(2 * math.pi * i / n + phase), r * math.sin(2 * math.pi * i / n + phase)) for i in range(n)]


def hexagon(af):
    return circle(af / math.sqrt(3), 6)


def zx(pts):
    """↔️ Plan points (x, z) as an extrude profile on the zx plane (w, u) of a tread or balustrade bone."""
    return [(z, x) for x, z in pts]


def spin(s, count, step_deg, rise):
    s["repeat"] = {"count": count, "rotationAxis": [0, 1, 0], "rotationRad": r5(-math.radians(step_deg)), "translation": [0, r5(rise), 0]}
    return s


def th_of(i, of, region):
    return {"id": i, "of": of, "region": region}


def st_of(i, of, member, part="member", anchor=None):
    out = {"id": i, "of": of, "member": member, "part": part}
    if anchor:
        out["anchor"] = anchor
    return out


def st(s, member, part="member", anchor=None):
    s["member"] = member
    s["part"] = part
    if anchor:
        s["anchor"] = anchor
    return s


def th(s, region):
    s["region"] = region
    return s


def text(en, de):
    return {"en": en, "de": de}


def masses():
    """⚖️ Masses and centres of the real assembly: column (core, base plate, cap, anchors), each tread (plate, flange, hub), balustrade (rail, posts, balusters)."""
    core = math.pi * (R_CORE ** 2 - (R_CORE - T_CORE) ** 2) * (POST_TOP - BASE_T) * RHO
    plate = (math.pi * BASE_R ** 2 - 4 * math.pi * 0.009 ** 2) * BASE_T * RHO
    cap = math.pi * R_CORE ** 2 * CAP_T * RHO
    nut = 4 * (3 * math.sqrt(3) / 2 * (NUT_AF / math.sqrt(3)) ** 2 * NUT_H) * RHO
    col_kg = core + plate + cap + nut
    col_com = (core * (BASE_T + POST_TOP) / 2 + plate * BASE_T / 2 + cap * (POST_TOP + CAP_T / 2) + nut * 0.028) / col_kg
    treads = {}
    for k in range(1, 16):
        a, cx, cz = area_centroid(tread_plan_dense(k))
        m_plate = a * PLATE_T * RHO
        m_flange = perimeter_free(k) * (PAN - PLATE_T) * PLATE_T * RHO
        fpts = tread_outline(k, inner=False, r_in=R_HUB, step=0.5)
        fl = [((fpts[i][0] + fpts[i + 1][0]) / 2, (fpts[i][1] + fpts[i + 1][1]) / 2, math.dist(fpts[i], fpts[i + 1])) for i in range(len(fpts) - 1)]
        L = sum(w for _, _, w in fl)
        fx, fz = sum(x * w for x, _, w in fl) / L, sum(z * w for _, z, w in fl) / L
        y0, y1 = hub_span(k)
        m_hub = math.pi * (R_HUB ** 2 - R_HUB_IN ** 2) * (y1 - y0) * RHO
        kg = m_plate + m_flange + m_hub
        x = (m_plate * cx + m_flange * fx) / kg
        z = (m_plate * cz + m_flange * fz) / kg
        v = (m_plate * (-PLATE_T / 2) + m_flange * (-(PAN + PLATE_T) / 2) + m_hub * ((y0 + y1) / 2 - RISE * k)) / kg
        treads[k] = {"kg": kg, "x": x, "z": z, "v": v, "plate": m_plate, "flange": m_flange, "hub": m_hub}
    tube = math.pi * (RAIL_R ** 2 - (RAIL_R - RAIL_T) ** 2) * RHO
    rail_m = tube * rail_length()
    parts = []
    pts = [c for c, _ in rail_path(step=0.25, dome=False)]
    for i in range(len(pts) - 1):
        m = tube * math.dist(pts[i], pts[i + 1])
        parts.append((m, [(pts[i][j] + pts[i + 1][j]) / 2 for j in range(3)]))
    for _, x, z, y0, y1 in bars():
        parts.append((math.pi * BAL_R ** 2 * (y1 - y0) * RHO, [x, (y0 + y1) / 2, z]))
    for _, x, z, y0, y1 in posts():
        parts.append((tube * (y1 - y0) + math.pi * FLANGE_R ** 2 * FLANGE_T * RHO, [x, (y0 + y1) / 2, z]))
    ip = inner_rail()
    for a, b in zip(ip, ip[1:]):
        parts.append((tube * math.dist(a, b), [(a[j] + b[j]) / 2 for j in range(3)]))
    parts.append((math.pi * FLANGE_R ** 2 * FLANGE_T * RHO, list(ip[-1])))
    bal_kg = sum(m for m, _ in parts)
    bal_com = [sum(m * c[j] for m, c in parts) / bal_kg for j in range(3)]
    return {"column": (col_kg, col_com), "treads": treads, "balustrade": (bal_kg, bal_com), "rail": rail_m}


if __name__ == "__main__":
    M = masses()
    print("[DEBUG] column", M["column"])
    print("[DEBUG] tread 1", M["treads"][1])
    print("[DEBUG] tread 8", M["treads"][8])
    print("[DEBUG] balustrade", M["balustrade"], "rail", M["rail"], "length", rail_length())
    print("[DEBUG] total", M["column"][0] + sum(t["kg"] for t in M["treads"].values()) + M["balustrade"][0])
    print("[DEBUG] rail top", rail_top(), "balusters", len(bars()), "landing", tread_outline(15, inner=False), "extent x", extent(tread_plan_dense(15), 0))
    a, cx, cz = area_centroid(tread_plan_dense(3))
    print("[DEBUG] tread area", a, "free perimeter", perimeter_free(3))
    print("[DEBUG] nosing clearance at rail", [rail_y(STEP * j - math.degrees(math.asin(LAP / R_RAIL))) + RAIL_R - RISE * (j + 1) for j in (1, 7, 13)])
