"""🧺 Round 3, furniture/picnic-table: the measures of the real table, every part of every scale stated by hand in world coordinates, and the helpers that turn a part into its bone-local solid.

The subject: a 1.80 m A-frame picnic table of 45 mm pressure-treated pine. Top: five 132 x 40 mm planks with 10 mm drainage gaps (0.70 m wide, top face at 0.75 m) screwed on two end cleats and a centre cleat. Two A-frames at 0.10 m from the plank ends, each of two 95 x 45 mm legs raked 60 degrees to the ground in one layer and, lapped on their outer faces in a second layer, a 95 x 45 mm cleat under the top and a 95 x 45 mm stretcher at seat-bracket height, clamped by M10 carriage bolts and the M12 pivot bolts. Two 70 x 45 mm braces run from the stretchers up to the centre cleat. Each bench (two 145 x 40 mm planks, 1.50 m, seat at 0.44 m) hangs between the A-frames on two folded 8 mm steel gusset brackets that turn on the pivot bolts through leg and stretcher; a spring plunger on each gusset snaps into an 8 mm bearing plate on the inner leg face, in the seat position and, 90 degrees further down, in the stowed position where the bench hangs plumb between the frames.

World frame: X across the table (the end elevation), Y up, Z along the table (left +). A part is one of: `pz` (a profile in (X, Y) pushed along Z), `px` (a profile in (Z, Y) pushed along X), `rev` (a profile of [axial, radial] turned about a world axis). Bench parts are stated for the right bench (+X) and mirrored for the left one.
"""
import math

from shapely.geometry import Polygon
from shapely.geometry.polygon import orient

PINE, STEEL = "pine-timber", "steel-galvanised"
RHO = {PINE: 500.0, STEEL: 7800.0}

LENGTH, TOP_HALF, TOP_Y0, TOP_Y1 = 1.8, 0.35, 0.71, 0.75
PLANK_W, PLANK_GAP, PLANK_N, EASE = 0.132, 0.010, 5, 0.005
LEG_Z = (0.7775, 0.8225)
BEAM_Z = (0.8225, 0.8675)
FOOT = (0.610, 0.720)
LEG_TOP = (0.200, 0.310)
FOOT_C = 0.665
RAKE = math.radians(60.0)
CLEAT_HALF, CLEAT_Y0, BEVEL = 0.33, 0.615, 0.045
STRETCH_HALF, STRETCH_Y = 0.53, (0.3275, 0.4225)
MID_HALF, MID_Y0, MID_T = 0.31, 0.64, 0.0225
PIVOT = (0.45, 0.375)
TOP_JOINT = (-0.35, 0.71)
BENCH_X, BENCH_Y, BENCH_HALF = (0.01, 0.31), (0.025, 0.065), 0.75
SEAT_W, SEAT_GAP = 0.145, 0.010
PLATE_Z = (0.7695, 0.7775)
GUSSET_Z = (0.7595, 0.7675)
FLANGE_Z = (0.7195, 0.7595)
GUSSET = [(-0.03, 0.025), (0.29, 0.025), (0.29, 0.003), (0.075, -0.045), (-0.01, -0.045), (-0.03, -0.025)]
PLUNGER = (0.05 * math.cos(math.radians(-15)), 0.05 * math.sin(math.radians(-15)))
STOW_HOLE = (0.05 * math.cos(math.radians(-105)), 0.05 * math.sin(math.radians(-105)))
BRACE_A, BRACE_B, BRACE_W, BRACE_T = (0.8225, 0.37), (0.0225, 0.67), 0.07, 0.0225

BONES = {
    "frame": lambda X, Y, Z: (Y, -X, Z),
    "top": lambda X, Y, Z: (X - TOP_JOINT[0], Y - TOP_JOINT[1], Z),
    "bench-r": lambda X, Y, Z: (X - PIVOT[0], Y - PIVOT[1], Z),
    "bench-l": lambda X, Y, Z: (-(X + PIVOT[0]), -(Y - PIVOT[1]), Z),
}
REGION = {"top": ("table-top", "top-plank"), "seat": ("bench-seats", "seat-plank"), "frame": ("a-frame", "frame-member"), "steel": ("hinges", "hinge-plate")}


def r6(v):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(v + 0.0, 5)


def local(bone, X, Y, Z=0.0):
    """🧭 A world point in the rest frame of a bone."""
    return BONES[bone](X, Y, Z)


def vector(bone, dX, dY, dZ):
    """➡ A world vector in the rest frame of a bone."""
    a, b = BONES[bone](0, 0, 0), BONES[bone](dX, dY, dZ)
    return [r6(b[k] - a[k]) for k in range(3)]


def rect(x0, y0, x1, y1):
    """▭ A rectangle as four corners, counter-clockwise."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def eased(x0, y0, x1, y1, radius=EASE):
    """▢ A plank section with eased edges (three-segment arcs)."""
    pts = []
    for cx, cy, start in ((x0 + radius, y0 + radius, math.pi), (x1 - radius, y0 + radius, 1.5 * math.pi), (x1 - radius, y1 - radius, 0.0), (x0 + radius, y1 - radius, 0.5 * math.pi)):
        for k in range(4):
            a = start + k * math.pi / 6
            pts.append((cx + radius * math.cos(a), cy + radius * math.sin(a)))
    return pts


def leg_x(y):
    """📐 The centre line of the right leg at height y."""
    return FOOT_C - y / math.tan(RAKE)


def on_leg(s, t=0.0):
    """📍 A point of the right leg's inner face: s down along the leg from the pivot, t across it (outwards +)."""
    return (PIVOT[0] + s * math.cos(RAKE) + t * math.sin(RAKE), PIVOT[1] - s * math.sin(RAKE) + t * math.cos(RAKE))


def leg(toe=True):
    """🦵 The right leg in (X, Y): a 95 mm board raked 60 degrees, cut level at the ground and under the planks; `toe` eases the outer toe by 12 mm."""
    foot = [(FOOT[0], 0.0), (FOOT[1] - 0.012, 0.0), (FOOT[1] - 0.006, 0.006 * math.tan(RAKE))] if toe else [(FOOT[0], 0.0), (FOOT[1], 0.0)]
    return foot + [(LEG_TOP[1], TOP_Y0), (LEG_TOP[0], TOP_Y0)]


def cleat(half, y0, bevel=BEVEL):
    """🪵 A cleat under the top in (X, Y) with its lower corners bevelled."""
    return [(-half + bevel, y0), (half - bevel, y0), (half, y0 + bevel), (half, TOP_Y0), (-half, TOP_Y0), (-half, y0 + bevel)]


def stretcher(bevel=0.03):
    """🪵 The stretcher in (X, Y) with plumb ends and bevelled lower corners."""
    y0, y1 = STRETCH_Y
    h = STRETCH_HALF
    if not bevel:
        return rect(-h, y0, h, y1)
    return [(-h + bevel, y0), (h - bevel, y0), (h, y0 + bevel), (h, y1), (-h, y1), (-h, y0 + bevel)]


def brace():
    """📏 The brace at +Z in (Z, Y): a 70 mm strip from the stretcher's inner face up to the centre cleat, plumb cut at both ends."""
    (za, ya), (zb, yb) = BRACE_A, BRACE_B
    slope = (yb - ya) / (zb - za)
    half = BRACE_W / 2 / math.cos(math.atan(abs(slope)))
    return [(za, ya - half), (za, ya + half), (zb, yb + half), (zb, yb - half)]


def plate():
    """🔩 The bearing plate on the right leg's inner face in (X, Y): 88 x 120 mm along the leg, corners cut 8 mm, with the open stow hole of the plunger."""
    s0, s1, t, c = -0.045, 0.075, 0.044, 0.008
    outline = [on_leg(s0 + c, -t), on_leg(s1 - c, -t), on_leg(s1, -t + c), on_leg(s1, t - c), on_leg(s1 - c, t), on_leg(s0 + c, t), on_leg(s0, t - c), on_leg(s0, -t + c)]
    hx, hy = PIVOT[0] + STOW_HOLE[0], PIVOT[1] + STOW_HOLE[1]
    hole = [(hx + 0.0065 * math.cos(k * math.pi / 6), hy + 0.0065 * math.sin(k * math.pi / 6)) for k in range(12)]
    return outline, hole


def bench_world(points):
    """🪑 Bench-local points of the right bench as world points."""
    return [(PIVOT[0] + x, PIVOT[1] + y) for x, y in points]


def part(sid, bone, kind, material, tone, group, views, **more):
    """🧩 One part: `group` binds region and layer, `views` gives the paint order in elevation, front and plan (None: hidden there)."""
    out = {"id": sid, "bone": bone, "kind": kind, "material": material, "tone": tone, "group": group, "views": dict(zip("efp", views))}
    out.update(more)
    return out


def pz(sid, bone, profile, z, material, tone, group, views, **more):
    """⬛ A profile in (X, Y) pushed along Z."""
    return part(sid, bone, "pz", material, tone, group, views, profile=profile, span=z, **more)


def px(sid, bone, profile, x, material, tone, group, views, **more):
    """⬛ A profile in (Z, Y) pushed along X."""
    return part(sid, bone, "px", material, tone, group, views, profile=profile, span=x, **more)


def rev(sid, bone, origin, axis, profile, tone, views, **more):
    """🔘 A turned steel part about a world axis."""
    return part(sid, bone, "rev", STEEL, tone, "steel", views, origin=origin, axis=axis, profile=profile, role="detail", texture=True, **more)


def mirror_x(p):
    """🪞 The left twin of a right-hand part (X to -X)."""
    out = dict(p)
    out["id"] = p["id"].replace("-r", "-l", 1) if "-r" in p["id"] else p["id"] + "-l"
    if p["bone"] == "bench-r":
        out["bone"] = "bench-l"
    if p["kind"] == "pz":
        out["profile"] = [(-x, y) for x, y in reversed(p["profile"])]
        if "holes" in p:
            out["holes"] = [[(-x, y) for x, y in reversed(h)] for h in p["holes"]]
    elif p["kind"] == "rev":
        out["origin"] = (-p["origin"][0], p["origin"][1], p["origin"][2])
        out["axis"] = (-p["axis"][0], p["axis"][1], p["axis"][2])
    if "repeat" in p:
        n, (dx, dy, dz) = p["repeat"]
        out["repeat"] = (n, (-dx, dy, dz))
    return out


def mirror_z(p, suffix="-b"):
    """🪞 The far twin of a part (Z to -Z)."""
    out = dict(p)
    out["id"] = p["id"] + suffix
    if p["kind"] == "pz":
        out["span"] = (-p["span"][1], -p["span"][0])
    elif p["kind"] == "px":
        out["profile"] = [(-z, y) for z, y in reversed(p["profile"])]
    else:
        out["origin"] = (p["origin"][0], p["origin"][1], -p["origin"][2])
        out["axis"] = (p["axis"][0], p["axis"][1], -p["axis"][2])
    if "repeat" in p:
        n, (dx, dy, dz) = p["repeat"]
        out["repeat"] = (n, (dx, dy, -dz))
    return out


DOME = [[-0.0005, 0.012], [0.002, 0.0112], [0.0042, 0.0085], [0.0056, 0.0045], [0.006, 0.0]]
NUT10 = [[0.0, 0.0115], [0.002, 0.0115], [0.002, 0.0085], [0.0095, 0.0085], [0.0095, 0.005], [0.013, 0.005]]
NUT12 = [[0.0, 0.014], [0.0025, 0.014], [0.0025, 0.0105], [0.012, 0.0105], [0.012, 0.006], [0.016, 0.006]]
HEAD12 = [[0.0, 0.0105], [0.0075, 0.0105]]
PLUNGER_BODY = [[0.0, 0.005], [0.014, 0.005], [0.014, 0.009], [0.03, 0.009], [0.03, 0.011], [0.036, 0.011]]


def frame_bolts():
    """🔩 The fixings of one A-frame at +Z that show at 1:50: the domed heads of the M10 carriage bolts on the outer faces of cleat and stretcher lap, their nuts on the inner leg face, and nut and washer of the M12 pivot bolt on the stretcher."""
    out = []
    step = 0.045
    d = (-step * math.cos(RAKE), step * math.sin(RAKE), 0.0)
    y = 0.64
    out.append(rev("cleat-dome-r", "frame", (leg_x(y), y, BEAM_Z[1]), (0, 0, 1), DOME, 0.12, (7, None, None), repeat=(2, d)))
    out.append(rev("cleat-nut-r", "frame", (leg_x(y), y, LEG_Z[0]), (0, 0, -1), NUT10, 0.12, (None, None, None), repeat=(2, d)))
    out.append(rev("pivot-nut-r", "frame", (PIVOT[0], PIVOT[1], BEAM_Z[1]), (0, 0, 1), NUT12, 0.12, (7, None, None)))
    y2 = 0.405
    out.append(rev("lap-nut-r", "frame", (leg_x(y2), y2, BEAM_Z[1]), (0, 0, 1), NUT10, 0.12, (7, None, None)))
    return out


def frame50():
    """🅰 Both A-frames, braces, bearing plates and their fixings at 1:50."""
    near = [
        pz("leg-r", "frame", leg(), LEG_Z, PINE, 0.55, "frame", (3, 3, 3), hatch="grain-v"),
        pz("cleat", "frame", cleat(CLEAT_HALF, CLEAT_Y0), BEAM_Z, PINE, 0.5, "frame", (5, 3.2, None), hatch="grain-h"),
        pz("stretcher", "frame", stretcher(), BEAM_Z, PINE, 0.5, "frame", (5, 3.2, 2.5), hatch="grain-h"),
        px("brace", "frame", brace(), (-BRACE_T, BRACE_T), PINE, 0.46, "frame", (2, 2, None)),
    ]
    outline, hole = plate()
    near.append(pz("plate-r", "frame", outline, PLATE_Z, STEEL, 0.25, "steel", (None, 4, 4), holes=[hole]))
    near += frame_bolts()
    out = []
    for p in near:
        twins = [p] if p["id"] in ("cleat", "stretcher", "brace") else [p, mirror_x(p)]
        for q in twins:
            out += [q, mirror_z(q)]
    return out


def bench50():
    """🪑 The right bench at 1:50: two eased planks, the centre batten, two folded gusset brackets with their flanges, the pivot bolt heads and the spring plungers."""
    (x0, x1), (y0, y1) = BENCH_X, BENCH_Y
    plank = bench_world(eased(x0, y0, x0 + SEAT_W, y1))
    out = [
        pz("seat-r", "bench-r", plank, (-BENCH_HALF, BENCH_HALF), PINE, 0.7, "seat", (2.4, 5, 5), repeat=(2, (SEAT_W + SEAT_GAP, 0, 0)), hatch="grain-h"),
        pz("batten-r", "bench-r", bench_world(rect(x0 + 0.01, y0 - 0.045, x1 - 0.01, y0)), (-0.0225, 0.0225), PINE, 0.5, "seat", (None, 4.5, None)),
    ]
    gusset = pz("gusset-r", "bench-r", bench_world(GUSSET), GUSSET_Z, STEEL, 0.22, "steel", (2.6, 4.6, 4.6))
    flange = pz("flange-r", "bench-r", bench_world(rect(x0, y0 - 0.008, 0.29, y0)), FLANGE_Z, STEEL, 0.25, "steel", (None, 4.7, 4.7))
    head = rev("pivot-head-r", "bench-r", (PIVOT[0], PIVOT[1], GUSSET_Z[0]), (0, 0, -1), HEAD12, 0.12, (None, None, None))
    plunger = rev("plunger-r", "bench-r", (PIVOT[0] + PLUNGER[0], PIVOT[1] + PLUNGER[1], PLATE_Z[0] + 0.004), (0, 0, -1), PLUNGER_BODY, 0.12, (None, None, None))
    for p in (gusset, flange, head, plunger):
        out += [p, mirror_z(p)]
    return out


def top50():
    """🍽 The top at 1:50: five eased planks and the centre cleat."""
    x0 = -TOP_HALF
    return [
        pz("top-plank", "top", eased(x0, TOP_Y0, x0 + PLANK_W, TOP_Y1), (-LENGTH / 2, LENGTH / 2), PINE, 0.7, "top", (6, 6, 6), repeat=(PLANK_N, (PLANK_W + PLANK_GAP, 0, 0)), hatch="grain-h"),
        pz("top-cleat", "top", cleat(MID_HALF, MID_Y0, 0.03), (-MID_T, MID_T), PINE, 0.5, "frame", (None, 3.1, None)),
    ]


def both(parts):
    """👯 Right bench parts and their left twins."""
    return parts + [mirror_x(p) for p in parts]


def parts50():
    """🔍 1:50, smallest feature 7.5 mm: the whole construction."""
    return top50() + frame50() + both(bench50())


def parts100():
    """🔎 1:100, smallest feature 15 mm: square-edged planks, the frames with their laps and braces, each bracket as a 15 mm web that takes up the bearing plate, with its flange at the 15 mm floor."""
    x0 = -TOP_HALF
    (bx0, bx1), (by0, by1) = BENCH_X, BENCH_Y
    out = [
        pz("top-plank", "top", rect(x0, TOP_Y0, x0 + PLANK_W, TOP_Y1), (-LENGTH / 2, LENGTH / 2), PINE, 0.7, "top", (6, 6, 6), repeat=(PLANK_N, (PLANK_W + PLANK_GAP, 0, 0))),
        pz("top-cleat", "top", rect(-MID_HALF, MID_Y0, MID_HALF, TOP_Y0), (-MID_T, MID_T), PINE, 0.5, "frame", (None, 3.1, None)),
    ]
    near = [
        pz("leg-r", "frame", leg(False), LEG_Z, PINE, 0.55, "frame", (3, 3, 3)),
        pz("cleat", "frame", cleat(CLEAT_HALF, CLEAT_Y0), BEAM_Z, PINE, 0.5, "frame", (5, 3.2, None)),
        pz("stretcher", "frame", stretcher(0), BEAM_Z, PINE, 0.5, "frame", (5, 3.2, 2.5)),
        px("brace", "frame", brace(), (-BRACE_T, BRACE_T), PINE, 0.46, "frame", (2, 2, None)),
    ]
    for p in near:
        for q in ([p] if p["id"] != "leg-r" else [p, mirror_x(p)]):
            out += [q, mirror_z(q)]
    bench = [
        pz("seat-r", "bench-r", bench_world(rect(bx0, by0, bx0 + SEAT_W, by1)), (-BENCH_HALF, BENCH_HALF), PINE, 0.7, "seat", (2.4, 5, 5), repeat=(2, (SEAT_W + SEAT_GAP, 0, 0))),
        pz("batten-r", "bench-r", bench_world(rect(bx0 + 0.01, by0 - 0.045, bx1 - 0.01, by0)), (-0.0225, 0.0225), PINE, 0.5, "seat", (None, 4.5, None)),
    ]
    web = pz("bracket-r", "bench-r", bench_world([(-0.03, 0.025), (0.29, 0.025), (0.29, 0.005), (0.075, -0.045), (-0.03, -0.045)]), (LEG_Z[0] - 0.015, LEG_Z[0]), STEEL, 0.22, "steel", (2.6, 4.6, 4.6))
    flange = pz("flange-r", "bench-r", bench_world(rect(bx0, by0 - 0.015, 0.29, by0)), (LEG_Z[0] - 0.055, LEG_Z[0] - 0.015), STEEL, 0.25, "steel", (None, 4.7, 4.7))
    for p in (web, flange):
        bench += [p, mirror_z(p)]
    return out + both(bench)


def parts200():
    """🗺 1:200, smallest feature 30 mm: the top as one slab, each bench as one board, the frames member by member, each bracket as a 30 mm block between board end and leg."""
    (bx0, bx1), (by0, by1) = BENCH_X, BENCH_Y
    out = [pz("top-slab", "top", rect(-TOP_HALF, TOP_Y0, TOP_HALF, TOP_Y1), (-LENGTH / 2, LENGTH / 2), PINE, 0.7, "top", (6, 6, 6))]
    near = [
        pz("leg-r", "frame", leg(False), LEG_Z, PINE, 0.55, "frame", (3, 3, 3)),
        pz("cleat", "frame", rect(-CLEAT_HALF, CLEAT_Y0, CLEAT_HALF, TOP_Y0), BEAM_Z, PINE, 0.5, "frame", (5, 3.2, None)),
        pz("stretcher", "frame", stretcher(0), BEAM_Z, PINE, 0.5, "frame", (5, 3.2, 2.5)),
        px("brace", "frame", brace(), (-BRACE_T, BRACE_T), PINE, 0.46, "frame", (2, 2, None)),
    ]
    for p in near:
        for q in ([p] if p["id"] != "leg-r" else [p, mirror_x(p)]):
            out += [q, mirror_z(q)]
    bench = [pz("seat-r", "bench-r", bench_world(rect(bx0, by0, bx1, by1)), (-BENCH_HALF, BENCH_HALF), PINE, 0.7, "seat", (2.4, 5, 5))]
    bracket = pz("bracket-r", "bench-r", bench_world([(-0.03, 0.025), (0.29, 0.025), (0.29, -0.005), (0.06, -0.045), (-0.03, -0.045)]), (LEG_Z[0] - 0.03, LEG_Z[0]), STEEL, 0.22, "steel", (2.6, 4.6, 4.6))
    bench += [bracket, mirror_z(bracket)]
    return out + both(bench)


def a_outline():
    """🅰 The 1:500 A-frame in (X, Y): both legs and the stretcher as one outline."""
    right = Polygon(leg(False))
    left = Polygon([(-x, y) for x, y in leg(False)])
    shape = right.union(left).union(Polygon(stretcher(0)))
    return [(round(x, 5), round(y, 5)) for x, y in list(orient(shape, 1.0).exterior.coords)[:-1]]


def parts500():
    """🌍 1:500, smallest feature 75 mm: the top with its cleats as one 75 mm slab, each A-frame as one 90 mm plate of legs and stretcher, each bench as a 75 mm board, each pair of brackets as 75 mm blocks under the board ends."""
    (bx0, bx1), (by0, by1) = BENCH_X, BENCH_Y
    out = [pz("top-slab", "top", rect(-TOP_HALF, TOP_Y1 - 0.075, TOP_HALF, TOP_Y1), (-LENGTH / 2, LENGTH / 2), PINE, 0.7, "top", (6, 6, 6))]
    a = pz("a-frame", "frame", a_outline(), (LEG_Z[0], BEAM_Z[1]), PINE, 0.5, "frame", (3, 3, 3))
    out += [a, mirror_z(a)]
    bench = [pz("seat-r", "bench-r", bench_world(rect(bx0, by1 - 0.075, bx1, by1)), (-BENCH_HALF, BENCH_HALF), PINE, 0.7, "seat", (2.4, 5, 5))]
    block = pz("bracket-r", "bench-r", bench_world(rect(bx0, by1 - 0.15, bx0 + 0.075, by1 - 0.075)), (BENCH_HALF - 0.075, BENCH_HALF), STEEL, 0.22, "steel", (2.6, 4.6, None))
    bench += [block, mirror_z(block)]
    return out + both(bench)


PARTS = {"50": parts50, "100": parts100, "200": parts200, "500": parts500}


def ring(bone, points):
    """⭕ World (X, Y) points as bone-local (u, v), counter-clockwise."""
    pts = [local(bone, x, y)[:2] for x, y in points]
    return [[r6(u), r6(v)] for u, v in list(orient(Polygon(pts), 1.0).exterior.coords)[:-1]]


def solid(p):
    """🧱 The bone-local volume solid of a part."""
    bone = p["bone"]
    out = {"id": p["id"], "kind": "extrude" if p["kind"] != "rev" else "revolve", "bone": bone, "material": p["material"], "role": p.get("role", "body"), "tone": p["tone"]}
    if p["kind"] == "pz":
        out.update({"profile": ring(bone, p["profile"]), "plane": "xy", "from": r6(p["span"][0]), "to": r6(p["span"][1])})
        if "holes" in p:
            out["holes"] = [list(reversed(ring(bone, h))) for h in p["holes"]]
    elif p["kind"] == "px":
        zy = list(orient(Polygon(p["profile"]), 1.0).exterior.coords)[:-1]
        if bone == "frame":
            out.update({"profile": [[r6(z), r6(y)] for z, y in zy], "plane": "zx", "from": r6(-p["span"][1]), "to": r6(-p["span"][0])})
        else:
            oy = local(bone, 0, 0)[1]
            ox = local(bone, 0, 0)[0]
            out.update({"profile": [[r6(y + oy), r6(z)] for z, y in zy], "plane": "yz", "from": r6(p["span"][0] + ox), "to": r6(p["span"][1] + ox)})
    else:
        out.update({"profile": [[r6(a), r6(b)] for a, b in p["profile"]], "axis": vector(bone, *p["axis"]), "origin": [r6(v) for v in local(bone, *p["origin"])]})
    if p.get("texture"):
        out["texture"] = True
    if "repeat" in p:
        out["repeat"] = {"count": p["repeat"][0], "translation": vector(bone, *p["repeat"][1])}
    return out


def volume(p):
    """⚖ Volume (m3) and world centroid of a part with its copies (turned parts: their swept cylinder is neglected, they are weighed as hardware)."""
    if p["kind"] == "rev":
        return 0.0, (0.0, 0.0, 0.0)
    poly = Polygon(p["profile"], p.get("holes"))
    n, step = p.get("repeat", (1, (0, 0, 0)))
    v = poly.area * (p["span"][1] - p["span"][0]) * n
    c, mid = poly.centroid, (p["span"][0] + p["span"][1]) / 2
    shift = [(n - 1) / 2 * s for s in step]
    if p["kind"] == "pz":
        return v, (c.x + shift[0], c.y + shift[1], mid + shift[2])
    return v, (mid + shift[0], c.y + shift[1], c.x + shift[2])


HARDWARE_KG = {"frame": 1.2, "top": 0.4, "bench-r": 0.7, "bench-l": 0.7}


def masses():
    """⚖ Mass (kg) and world centre of mass of every bone from the 1:50 parts (pine 500, steel 7800 kg/m3) plus bolts, screws and plungers as hardware at the centre."""
    acc = {}
    for p in parts50():
        v, c = volume(p)
        m = v * RHO[p["material"]]
        a = acc.setdefault(p["bone"], [0.0, 0.0, 0.0, 0.0])
        a[0] += m
        for k in range(3):
            a[k + 1] += m * c[k]
    return {b: (round(a[0] + HARDWARE_KG[b], 1), a[1] / a[0], a[2] / a[0], a[3] / a[0]) for b, a in acc.items()}


if __name__ == "__main__":
    for scale, make in PARTS.items():
        ps = make()
        print(scale, len(ps), sum(p.get("repeat", (1,))[0] for p in ps))
    for bone, m in masses().items():
        print(bone, [round(v, 4) for v in m])
    print("brace", brace(), "plate", plate()[0][:2], "plunger", PLUNGER, "stow", STOW_HOLE)
