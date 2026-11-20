"""🪑 Round 3, furniture/park-bench: the measures of the real bench and the helpers that turn world coordinates into bone-local solids and drawing frames.

The subject: a 1.80 m public bench with armrests, four 100 x 45 mm oak seat slats (10 mm drainage gaps) on a seat raked 5 degrees, three 80 x 35 mm oak back slats on back posts raked 8 degrees (seat-to-back angle 103 degrees), carried by two one-piece cast end frames of I-section (40 mm flange, 12 mm web) at 0.12 m from the slat ends; armrest 0.18 m above the seat front, carriage bolts M8 through every slat into the frame flange.

World frame: x along the bench from its left end, y up, z lateral (the sitter faces -z, front of the seat at -z). Every measure is metres. The frame profile lies in the plane (z, y) and is constructed from hand-stated member strips (their centre lines and widths), united and filleted with shapely (`staffage-oracles/geometry`, run this file with that python).
"""
import math

from shapely.geometry import LineString, Polygon
from shapely.ops import unary_union

LENGTH, DEPTH, HEIGHT = 1.8, 0.6, 0.85
FRAME_X = (0.12, 1.68)
FLANGE, WEB, RIM = 0.04, 0.010, 0.009
STEEL, OAK = "steel-painted", "oak-timber"
RHO = {STEEL: 7850.0, OAK: 700.0}

SEAT_RAKE, BACK_RAKE = math.radians(5.0), math.radians(8.0)
D = (math.cos(SEAT_RAKE), -math.sin(SEAT_RAKE))
N = (math.sin(SEAT_RAKE), math.cos(SEAT_RAKE))
E = (math.sin(BACK_RAKE), math.cos(BACK_RAKE))
M = (-math.cos(BACK_RAKE), math.sin(BACK_RAKE))

NOSE = (-0.240, 0.405)
SEAT_W, SEAT_T, SEAT_GAP, SEAT_N = 0.100, 0.045, 0.010, 4
SEAT_S = [(-0.0 + k * (SEAT_W + SEAT_GAP), k * (SEAT_W + SEAT_GAP) + SEAT_W) for k in range(SEAT_N)]
POST_FOOT = (0.205, 0.410)
POST_W = 0.036
BACK_W, BACK_T = 0.080, 0.035
BACK_S = [(0.080, 0.160), (0.245, 0.325), (0.357, 0.437)]
POST_TOP = 0.437
RAIL_D = 0.045
FRONT_Z, FRONT_W = -0.268, 0.040
REAR_FOOT_Z = 0.245
ARM_Y0, ARM_Y1 = 0.595, 0.630
ARM_FRONT = -0.298
BRACE_Y, BRACE_H = 0.150, 0.030
PAD_H, PAD_L, PAD_X = 0.016, 0.070, 0.070
SLAT_R = 0.006


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(value + 0.0, 5)


def add(p, v, s=1.0):
    """➕ p + s v in the plane (z, y)."""
    return (p[0] + s * v[0], p[1] + s * v[1])


def seat_point(s, h=0.0):
    """💺 A point of the seat: s along the rake from the nose's lower front edge, h up from the slat underside."""
    return add(add(NOSE, D, s), N, h)


def post_front(t):
    """🪵 A point of the post's front face, t along the post from its reference point at seat height."""
    return add(POST_FOOT, E, t)


def post_back(t):
    """🪵 A point of the post's back face."""
    return add(post_front(t), M, -POST_W)


def strip(points, width, cap=2):
    """▬ A member strip of the casting: a polyline of (z, y) points widened to `width` (flat ends)."""
    return LineString(points).buffer(width / 2, cap_style=cap, join_style=2)


def quad(points):
    """⬛ A member stated by its corners."""
    return Polygon(points)


def frame_parts(detail=True):
    """🦴 The hand-stated members of one end frame in (z, y): front leg with arm post, seat rail, back post, rear leg, lower brace, arm band; `detail` keeps the 1:50/1:100 widths, else the coarse 1:200 casting."""
    fz = FRONT_Z
    front = quad([(fz - 0.025, 0.012), (fz + 0.025, 0.012), (fz + FRONT_W / 2, 0.10), (fz + FRONT_W / 2, ARM_Y1 - 0.004), (fz - FRONT_W / 2, ARM_Y1 - 0.004), (fz - FRONT_W / 2, 0.10)])
    rail_top0 = seat_point(-(NOSE[0] - (fz + 0.004)) / D[0])
    rail_top1 = add(rail_top0, D, 0.47)
    rail = quad([rail_top0, rail_top1, add(rail_top1, N, -RAIL_D), add(rail_top0, N, -RAIL_D)])
    post = quad([post_front(-0.075), post_front(POST_TOP), post_back(POST_TOP), post_back(-0.075)])
    junction = add(post_back(-0.06), M, 0.022)
    rear = strip([junction, (REAR_FOOT_Z, 0.012)], 0.045 if detail else 0.05)
    rear_foot = quad([(REAR_FOOT_Z - 0.025, 0.012), (REAR_FOOT_Z + 0.025, 0.012), (REAR_FOOT_Z + 0.022, 0.06), (REAR_FOOT_Z - 0.022, 0.06)])
    brace = quad([(fz, BRACE_Y - BRACE_H / 2), (REAR_FOOT_Z - 0.01, BRACE_Y - BRACE_H / 2), (REAR_FOOT_Z - 0.016, BRACE_Y + BRACE_H / 2), (fz, BRACE_Y + BRACE_H / 2)])
    t_arm = (ARM_Y1 - POST_FOOT[1]) / E[1]
    arm = quad([(fz - FRONT_W / 2, ARM_Y0), (post_front(t_arm)[0] + 0.012, ARM_Y0), (post_front(t_arm)[0] + 0.012, ARM_Y1), (fz - FRONT_W / 2, ARM_Y1)])
    return [front, rail, post, rear, rear_foot, brace, arm]


def frame_profile(detail=True, fillet=0.008):
    """🔩 The outline of one cast end frame in (z, y): the united members with cast fillets in the inner corners and eased outer corners."""
    shape = unary_union(frame_parts(detail))
    return shape.buffer(fillet, join_style=1).buffer(-2 * fillet, join_style=1).buffer(fillet, join_style=1).simplify(0.0004)


def rings(poly):
    """⭕ The exterior and the interiors of a polygon as point lists (exterior counter-clockwise, holes clockwise)."""
    from shapely.geometry.polygon import orient
    p = orient(poly, 1.0)
    return [list(p.exterior.coords)[:-1]], [list(r.coords)[:-1] for r in p.interiors]


def i_section(profile):
    """🧱 The I-section of the casting: the web (profile shrunk by the rim) and the flange rims around the outline and every opening."""
    core = profile.buffer(-RIM, join_style=1).simplify(0.0004)
    rims = profile.difference(core)
    pieces = [rims] if rims.geom_type == "Polygon" else list(rims.geoms)
    return core, pieces


def base(sid, kind, bone, material, tone, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def end_extrude(sid, bone, poly, half, tone, role="body", material=STEEL):
    """🔩 A slab of the casting in the frame plane: the (z, y) polygon with its holes, from -half to +half across the frame (bone v = -world x)."""
    ext, holes = rings(poly)
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(z), r6(y)] for z, y in ext[0]], "plane": "zx", "from": r6(-half), "to": r6(half)})
    if holes:
        out["holes"] = [[[r6(z), r6(y)] for z, y in h] for h in holes]
    return out


def along(sid, bone, oy, section, tone, material=OAK, x=(0.0, LENGTH), role="body"):
    """➡ A slat: a world (z, y) section pushed along the bench from x[0] to x[1] on a bone at height oy."""
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(y - oy), r6(z)] for z, y in section], "plane": "yz", "from": r6(x[0]), "to": r6(x[1])})
    return out


def slat_section(origin, along_v, up_v, width, thick, radius):
    """▭ The rounded rectangle of a slat section in (z, y): corner `origin` (lower front), `width` along along_v, `thick` along up_v, edges eased by `radius` (three-segment arcs)."""
    if radius <= 0:
        return [origin, add(origin, along_v, width), add(add(origin, along_v, width), up_v, thick), add(origin, up_v, thick)]
    pts = []
    corners = [(radius, radius, math.pi), (width - radius, radius, 1.5 * math.pi), (width - radius, thick - radius, 0.0), (radius, thick - radius, 0.5 * math.pi)]
    for a, b, start in corners:
        for k in range(4):
            ang = start + k * math.pi / 6
            s, h = a + radius * math.cos(ang), b + radius * math.sin(ang)
            pts.append(add(add(origin, along_v, s), up_v, h))
    return pts


def seat_slats(radius=SLAT_R):
    """💺 The four seat slat sections (front first)."""
    return [slat_section(seat_point(s0), D, N, s1 - s0, SEAT_T, radius) for s0, s1 in SEAT_S]


def back_slats(radius=SLAT_R):
    """🪑 The three back slat sections (lowest first), lying on the posts' front faces."""
    out = []
    for t0, t1 in BACK_S:
        origin = add(post_front(t0), M, BACK_T)
        out.append(slat_section(origin, E, (-M[0], -M[1]), t1 - t0, BACK_T, radius))
    return out


def centroid(points):
    """⊙ The area centroid of a polygon given as points."""
    return Polygon(points).centroid.coords[0]


def seat_y():
    """📏 The height of the seat bone: the mean centre height of the seat slats (the beam line of the seat)."""
    return round(sum(centroid(s)[1] for s in seat_slats()) / SEAT_N, 3)


def back_y():
    """📏 The height of the backrest bone: the mean centre height of the back slats."""
    return round(sum(centroid(s)[1] for s in back_slats()) / len(BACK_S), 3)


def arm_reach():
    """🦾 The rear end of the armrest: the back post front face at armrest height, 12 mm into the post."""
    t = ((ARM_Y0 + ARM_Y1) / 2 - POST_FOOT[1]) / E[1]
    return post_front(t)[0] + 0.012


if __name__ == "__main__":
    prof = frame_profile()
    core, rims = i_section(prof)
    print("profile", prof.geom_type, round(prof.area, 4), prof.bounds, "holes", len(prof.interiors))
    print("core", core.geom_type, round(core.area, 4), "rims", len(rims), [round(r.area, 4) for r in rims])
    print("seat y", seat_y(), "back y", back_y(), "arm reach", arm_reach())
    for s in seat_slats():
        print("seat", [round(c, 4) for c in Polygon(s).bounds])
    for s in back_slats():
        print("back", [round(c, 4) for c in Polygon(s).bounds])
    print("post top front", post_front(POST_TOP), "back", post_back(POST_TOP))
