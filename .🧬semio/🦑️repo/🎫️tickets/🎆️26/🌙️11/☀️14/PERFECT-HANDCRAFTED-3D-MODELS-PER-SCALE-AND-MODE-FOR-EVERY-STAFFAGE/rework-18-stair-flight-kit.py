"""🪜 Rework 18 stair flight: the real geometry of a straight steel centre-stringer flight (DIN 18065 step rule, HEA 160 stringer, welded tread brackets, base plate and head plate, a balustrade on both open sides) and the writers every script states its parts with.

World frame: x ahead (up the flight), y up, z lateral (left +); the floor of the lower storey is y = 0, the landing slab face is x = RUN.
Bone frame (stringer): origin at the centroid axis of the HEA on the floor, u along the axis up the flight, v its upward normal, w = z.
References: DIN 18065 (2 r + g, nosing lap of open stairs), EN 10365 (HEA 160), EN 338 (C24), EN 10025-2 (S235).
"""
import math

H = 2.9
N = 16
RISE = H / N
GOING = 0.28
LAP = 0.03
DEPTH = GOING + LAP
T_TREAD = 0.04
HALF_W = 0.6
NOSE_R = 0.008
GUARD_H = 0.03
GUARD_T = 0.03
GUARD_SET = 0.03
RUN = N * GOING
M = RISE / GOING
TH = math.atan(M)
C, S = math.cos(TH), math.sin(TH)
PITCH = math.hypot(GOING, RISE)

HEA_H = 0.152
HEA_B = 0.16
HEA_TW = 0.006
HEA_TF = 0.009
HEA_KG_M = 30.4
HEA_AREA_M = 0.906
HEA_A = 0.00388
HEA_R = 0.015
HEA_FILLET = HEA_A / (2 * HEA_B * HEA_TF + (HEA_H - 2 * HEA_TF) * HEA_TW)

CARRIER_T = 0.008
CARRIER_X0 = 0.19
CARRIER_X1 = 0.02
CARRIER_HALF = 0.18
FIN_T = 0.01

DROP = T_TREAD + CARRIER_T
E_AXIS = DROP * C + HEA_H / 2
X0 = E_AXIS / S
BONE_L = (RUN - X0) / C
WALK_U, WALK_V = -X0 * C, X0 * S

BASE_T = 0.015
BASE_X = (0.09, 0.39)
BASE_HALF = 0.13
ANCHOR_X = (0.13, 0.35)
ANCHOR_Z = 0.105
NUT_AF = 0.019
NUT_H = 0.01
BOLT_R = 0.006
BOLT_OUT = 0.01

HEAD_T = 0.015
HEAD_Y = (2.57, 2.85)
HEAD_HALF = 0.13
HEAD_BOLT_Y = (2.60, 2.82)
HEAD_BOLT_OUT = 0.008

RAIL_Z = 0.55
RAIL_R = 0.0212
RAIL_KG_M = 2.55
RAIL_H = 0.929
LOW_R = 0.01065
LOW_KG_M = 1.08
LOW_H = 0.10
BAL_R = 0.006
BAL_KG_M = 0.888
BAL_STEP = 0.105
BAL_PER_BAY = 7
POST_KS = (1, 4, 7, 10, 13, 16)
POST_STEP = 3
FLANGE_R = 0.04
FLANGE_T = 0.008
RAIL_OVER = 0.06

RHO_STEEL = 7850.0
RHO_TIMBER = 420.0


def r5(x):
    return round(x, 5) + 0.0


def local(p):
    """🧭 World (x, y) to stringer-local (u, v)."""
    dx, dy = p[0] - X0, p[1]
    return (dx * C + dy * S, -dx * S + dy * C)


def world(q):
    """🌍 Stringer-local (u, v) to world (x, y)."""
    u, v = q
    return (X0 + u * C - v * S, u * S + v * C)


def local_vec(dx, dy):
    return (dx * C + dy * S, -dx * S + dy * C)


def nose_y(x):
    """📏 Height of the nosing line (through the front edges of the treads) at x."""
    return M * (x + DEPTH)


def post_x(k):
    """📍 x of the balustrade post on tread k: the middle of the board."""
    return GOING * k - DEPTH / 2


def rail_y(x, h=RAIL_H):
    """〰️ Height of a rail centre at x: parallel to the nosing line, h above it (the handrail top is 0.95 m above the nosing line, DIN 18065 asks for at least 0.90 m)."""
    return nose_y(x) + h


def rail_span():
    return post_x(POST_KS[0]) - RAIL_OVER, post_x(POST_KS[-1]) + RAIL_OVER


def baluster_x(bay, j):
    """🦯 x of baluster j (1..7) in the bay that starts at post number bay (0..4): 105 mm centres, 93 mm clear (MBO section 38: at most 120 mm)."""
    return post_x(POST_KS[bay]) + BAL_STEP * j


def balustrade_parts():
    """⚖️ (mass, x, y) of every part of ONE balustrade: handrail CHS 42.4 x 2.6, bottom rail CHS 21.3 x 2.3, six posts CHS 42.4 x 2.6 with flanges, 35 balusters of 12 mm."""
    parts = []
    x0, x1 = rail_span()
    parts.append((RAIL_KG_M * (x1 - x0) / C, (x0 + x1) / 2, rail_y((x0 + x1) / 2)))
    a, b = post_x(POST_KS[0]), post_x(POST_KS[-1])
    parts.append((LOW_KG_M * (b - a) / C, (a + b) / 2, rail_y((a + b) / 2, LOW_H)))
    for k in POST_KS:
        x = post_x(k)
        parts.append((RAIL_KG_M * (rail_y(x) - tread_y(k)) + math.pi * FLANGE_R ** 2 * FLANGE_T * RHO_STEEL, x, (rail_y(x) + tread_y(k)) / 2))
    for bay in range(len(POST_KS) - 1):
        for j in range(1, BAL_PER_BAY + 1):
            x = baluster_x(bay, j)
            parts.append((BAL_KG_M * (RAIL_H - LOW_H), x, rail_y(x, (RAIL_H + LOW_H) / 2)))
    return parts


def balustrade_mass():
    """⚖️ Mass and world centre of both balustrades together."""
    parts = balustrade_parts()
    kg = sum(p[0] for p in parts)
    return 2 * kg, (sum(p[0] * p[1] for p in parts) / kg, sum(p[0] * p[2] for p in parts) / kg)


def tread_y(k):
    return RISE * k


def tread_back(k):
    return GOING * k


def line_y(x, drop):
    """📏 Height at x of a line parallel to the pitch line, `drop` metres lower (vertically)."""
    return M * x - drop


def line_x(y, drop):
    return (y + drop) / M


def flange_drops(tf=HEA_TF, h=HEA_H):
    """🔩 Vertical drops below the pitch line of the four HEA faces: top of top flange, its underside, top of bottom flange, underside."""
    return DROP, DROP + tf / C, DROP + (h - tf) / C, DROP + h / C


def band(d_hi, d_lo, y_foot, x_head):
    """🧱 World outline of the strip between two pitch-parallel lines, cut horizontally at y_foot and plumb at x_head."""
    return [(line_x(y_foot, d_hi), y_foot), (line_x(y_foot, d_lo), y_foot), (x_head, line_y(x_head, d_lo)), (x_head, line_y(x_head, d_hi))]


def tread_outline(k, rounded=True, t=T_TREAD, steps=3):
    """🪵 World section of tread k: the board from its nosing (lapping the tread below by LAP) to its back edge at the slab-side riser line, nosing edges rounded."""
    x1 = tread_back(k)
    x0 = x1 - DEPTH
    y1 = tread_y(k)
    y0 = y1 - t
    if not rounded:
        return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
    r = NOSE_R
    pts = [(x1, y0), (x1, y1)]
    for i in range(steps + 1):
        a = math.pi / 2 + (math.pi / 2) * i / steps
        pts.append((x0 + r + r * math.cos(a), y1 - r + r * math.sin(a)))
    for i in range(steps + 1):
        a = math.pi + (math.pi / 2) * i / steps
        pts.append((x0 + r + r * math.cos(a), y0 + r + r * math.sin(a)))
    return pts


def folded_band(t):
    """🪗 World outline of the 1:500 flight: the 16 treads and the closed risers generalised to one folded band of thickness t (vertical under the treads, horizontal behind the risers), standing on the floor at the first nosing."""
    nose = [tread_back(k) - DEPTH for k in range(1, N + 1)]
    top = [(nose[0], 0.0)]
    for k in range(1, N + 1):
        top += [(nose[k - 1], tread_y(k)), (nose[k] if k < N else RUN, tread_y(k))]
    under = [(RUN, H - t)]
    for k in range(N, 0, -1):
        under += [(nose[k - 1] + t, tread_y(k) - t), (nose[k - 1] + t, tread_y(k - 1) - t if k > 1 else 0.0)]
    return top + under


def guard_outline(k):
    """🧒 World section of the guard batten of tread k: 30 x 30 mm of timber screwed under the board 30 mm behind its nosing, directly behind the back edge of the tread below, so the open riser of 141 mm is 111 mm clear (at most 120 mm where children are expected)."""
    x0 = tread_back(k) - DEPTH + GUARD_SET
    y1 = tread_y(k) - T_TREAD
    return [(x0, y1 - GUARD_H), (x0 + GUARD_T, y1 - GUARD_H), (x0 + GUARD_T, y1), (x0, y1)]


def carrier_outline(k):
    x1 = tread_back(k) - CARRIER_X1
    x0 = tread_back(k) - CARRIER_X0
    y1 = tread_y(k) - T_TREAD
    return [(x0, y1 - CARRIER_T), (x1, y1 - CARRIER_T), (x1, y1), (x0, y1)]


def fin_outline(k, top_drop=DROP, foot_y=BASE_T):
    """📐 World outline of the bracket fin of tread k: under the carrier (or the tread at coarse scales) down to the top flange, at the foot down to the base plate."""
    x1 = tread_back(k) - CARRIER_X1
    x0 = tread_back(k) - CARRIER_X0
    yt = tread_y(k) - top_drop
    pts = [(x0, yt), (x1, yt), (x1, line_y(x1, DROP))]
    xs = line_x(foot_y, DROP)
    if x0 < xs:
        pts += [(xs, foot_y), (x0, foot_y)]
    else:
        pts.append((x0, line_y(x0, DROP)))
    return pts


def rect(x0, y0, x1, y1):
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def root_fillet(i, material, tone, top, left, steps=4):
    """🌙 One of the four 15 mm root radii of the HEA between web and flange: the concave quarter section (v, w) run along the axis between the foot cut and the head cut, ending square where the whole section is still inside both cuts."""
    sv = 1.0 if top else -1.0
    sw = 1.0 if left else -1.0
    vf = sv * (HEA_H / 2 - HEA_TF)
    ww = sw * HEA_TW / 2
    cv, cw = vf - sv * HEA_R, ww + sw * HEA_R
    pts = [(vf, ww)] + [(cv + sv * HEA_R * math.sin(math.pi / 2 * k / steps), cw - sw * HEA_R * math.cos(math.pi / 2 * k / steps)) for k in range(steps + 1)]
    if sv * sw < 0:
        pts.reverse()
    vs = (vf, vf - sv * HEA_R)
    u0 = max((BASE_T - v * C) / S for v in vs)
    u1 = min((RUN - HEAD_T - X0 + v * S) / C for v in vs)
    return solid(i, "extrude", material, tone, "body", profile=[[r5(a), r5(b)] for a, b in pts], plane="yz", **{"from": r5(u0), "to": r5(u1)})


def poly_area_centroid(pts):
    a = cx = cy = 0.0
    for i in range(len(pts)):
        (x0, y0), (x1, y1) = pts[i], pts[(i + 1) % len(pts)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return abs(a), cx / (6 * a), cy / (6 * a)


def hexagon(af):
    rr = af / math.sqrt(3)
    return [(rr * math.cos(math.pi / 6 + i * math.pi / 3), rr * math.sin(math.pi / 6 + i * math.pi / 3)) for i in range(6)]


def masses():
    """⚖️ Mass and world centre of every part of the real flight: HEA stringer, base and head plate with anchors, 16 brackets (fin and carrier) and 16 C24 treads."""
    parts = []
    d = flange_drops()
    for lo, hi, w in ((d[0], d[1], HEA_B), (d[1], d[2], HEA_TW), (d[2], d[3], HEA_B)):
        a, cx, cy = poly_area_centroid(band(lo, hi, BASE_T, RUN - HEAD_T))
        parts.append(("stringer", a * w * RHO_STEEL * HEA_FILLET, cx, cy))
    hea = sum(p[1] for p in parts)
    a, cx, cy = poly_area_centroid(rect(BASE_X[0], 0, BASE_X[1], BASE_T))
    parts.append(("base-plate", a * 2 * BASE_HALF * RHO_STEEL, cx, cy))
    a, cx, cy = poly_area_centroid(rect(RUN - HEAD_T, HEAD_Y[0], RUN, HEAD_Y[1]))
    parts.append(("head-plate", a * 2 * HEAD_HALF * RHO_STEEL, cx, cy))
    nut = 3 * math.sqrt(3) / 2 * (NUT_AF / math.sqrt(3)) ** 2 * NUT_H * RHO_STEEL
    for x in ANCHOR_X:
        parts.append(("anchor", 2 * nut, x, BASE_T + NUT_H / 2))
    for y in HEAD_BOLT_Y:
        parts.append(("anchor", 2 * nut, RUN - HEAD_T - NUT_H / 2, y))
    for k in range(1, N + 1):
        a, cx, cy = poly_area_centroid(carrier_outline(k))
        parts.append(("carrier", a * 2 * CARRIER_HALF * RHO_STEEL, cx, cy))
        a, cx, cy = poly_area_centroid(fin_outline(k))
        parts.append(("fin", a * FIN_T * RHO_STEEL, cx, cy))
        a, cx, cy = poly_area_centroid(tread_outline(k, rounded=False))
        parts.append(("tread", a * 2 * HALF_W * RHO_TIMBER, cx, cy))
        a, cx, cy = poly_area_centroid(guard_outline(k))
        parts.append(("guard", a * 2 * HALF_W * RHO_TIMBER, cx, cy))
    flight_kg = sum(p[1] for p in parts)
    flight_com = (sum(p[1] * p[2] for p in parts) / flight_kg, sum(p[1] * p[3] for p in parts) / flight_kg)
    bkg, bcom = balustrade_mass()
    parts.append(("balustrade", bkg, bcom[0], bcom[1]))
    kg = sum(p[1] for p in parts)
    cx = sum(p[1] * p[2] for p in parts) / kg
    cy = sum(p[1] * p[3] for p in parts) / kg
    by = {}
    for name, m, _, _ in parts:
        by[name] = by.get(name, 0) + m
    return {"kg": kg, "com": (cx, cy), "comLocal": local((cx, cy)), "by": by, "heaKg": hea, "heaLengthM": hea / HEA_KG_M, "flightKg": flight_kg, "flightCom": flight_com, "flightComLocal": local(flight_com), "balustradeKg": bkg, "balustradeCom": bcom}


def solid(i, kind, material, tone, role="body", **extra):
    return {"id": i, "kind": kind, "bone": "stringer", "material": material, "role": role, "tone": tone, **extra}


def prism(i, material, tone, world_pts, lo, hi, role="body", **extra):
    """🧊 An extrude of a world (x, y) outline across the flight (w from lo to hi), stated in the stringer frame."""
    return solid(i, "extrude", material, tone, role, profile=[[r5(a), r5(b)] for a, b in (local(p) for p in world_pts)], plane="xy", **{"from": r5(lo), "to": r5(hi)}, **extra)


def quat_world():
    """🔄 The rotation that turns a part authored along world axes into the stringer frame."""
    return [0.0, 0.0, r5(math.sin(-TH / 2)), r5(math.cos(-TH / 2))]


def hex_up(i, material, tone, at, af, y0, y1, role="detail", **extra):
    """🔩 A hexagon nut with a vertical axis at world (x, z), from height y0 to y1."""
    x, z = at
    u, v = local((x, 0.0))
    return solid(i, "extrude", material, tone, role, profile=[[r5(a), r5(b)] for a, b in ((p[0], p[1]) for p in hexagon(af))], plane="zx", **{"from": r5(y0), "to": r5(y1)}, transform={"rotation": quat_world(), "translation": [r5(u), r5(v), r5(z)]}, **extra)


def hex_ahead(i, material, tone, at, af, x0, x1, role="detail", **extra):
    """🔩 A hexagon nut with its axis along world x at world (y, z), from x0 to x1."""
    y, z = at
    u, v = local((x0, y))
    return solid(i, "extrude", material, tone, role, profile=[[r5(a), r5(b)] for a, b in ((p[1], p[0]) for p in hexagon(af))], plane="yz", **{"from": 0.0, "to": r5(x1 - x0)}, transform={"rotation": quat_world(), "translation": [r5(u), r5(v), r5(z)]}, **extra)


def rod(i, material, tone, p0, p1, radius, role="detail", **extra):
    """📍 A round rod between two world points (x, y, z)."""
    a, b = local(p0[:2]), local(p1[:2])
    du, dv, dw = b[0] - a[0], b[1] - a[1], p1[2] - p0[2]
    ln = math.sqrt(du * du + dv * dv + dw * dw)
    return solid(i, "revolve", material, tone, role, profile=[[0.0, r5(radius)], [r5(ln), r5(radius)]], axis=[r5(du / ln), r5(dv / ln), r5(dw / ln)], origin=[r5(a[0]), r5(a[1]), r5(p0[2])], **extra)


def ellipsoid(i, material, tone, radii, at_world, role="detail", **extra):
    u, v = local(at_world)
    return solid(i, "ellipsoid", material, tone, role, radii=[r5(r) for r in radii], transform={"translation": [r5(u), r5(v), 0.0]}, **extra)


def step(count, dx=GOING, dy=RISE):
    u, v = local_vec(dx, dy)
    return {"count": count, "translation": [r5(u), r5(v), 0.0]}


def text(en, de):
    return {"en": en, "de": de}


def th_of(i, of, region, faces=None):
    out = {"id": i, "of": of, "region": region}
    if faces:
        out["sectionFaces"] = faces
    return out


def st_of(i, of, part="member", anchor=None):
    out = {"id": i, "of": of, "member": "stringer", "part": part}
    if anchor:
        out["anchor"] = anchor
    return out


def st(s, part="member", anchor=None):
    s["member"] = "stringer"
    s["part"] = part
    if anchor:
        s["anchor"] = anchor
    return s


if __name__ == "__main__":
    Mx = masses()
    print("[DEBUG] rise", RISE, "going", GOING, "2r+g", 2 * RISE + GOING, "slope", math.degrees(TH))
    print("[DEBUG] axis offset", E_AXIS, "X0", X0, "bone L", BONE_L, "pitch L", math.hypot(RUN, H))
    print("[DEBUG] masses", {k: round(v, 2) for k, v in Mx["by"].items()}, "total", round(Mx["kg"], 2))
    print("[DEBUG] com world", Mx["com"], "local", Mx["comLocal"], "HEA length", Mx["heaLengthM"])
    print("[DEBUG] foot cut", line_x(BASE_T, DROP), line_x(BASE_T, flange_drops()[3]), "head", line_y(RUN - HEAD_T, DROP), line_y(RUN - HEAD_T, flange_drops()[3]))
    print("[DEBUG] open riser", RISE - T_TREAD, "fin heights", [round(tread_y(1) - DROP - line_y(tread_back(1) - x, DROP), 4) for x in (CARRIER_X1, CARRIER_X0)])
    print("[DEBUG] tip world", world((BONE_L, 0)), "walker origin world", world((WALK_U, WALK_V)))


if __name__ == "__main__":
    m = masses()
    print("[DEBUG] total", m["kg"], "com", m["com"], "local", m["comLocal"])
    print("[DEBUG] flight", m["flightKg"], m["flightComLocal"], "balustrade", m["balustradeKg"], m["balustradeCom"], "post height", rail_y(post_x(1)) - tread_y(1), "mid baluster x", baluster_x(2, 4))
