"""🖥 Round 3, appliances/tower-server: the measures of the real pedestal server and the helpers that turn world coordinates into bone-local solids.

World frame: x through the depth (front bezel at +0.225, rear panel at -0.225), y up, z across the width (the removable side cover at -0.1, which the product side elevation looks at; the fixed wall carrying the mainboard at +0.1). The corrected `case` bone stands upright from the floor (angle 90 degrees), so bone-local u = world y, v = -world x, w = world z.
"""
import math

REAR, PAN, CHASSIS_FRONT, FRONT = -0.225, -0.213, 0.200, 0.225
FOOT, TOP_UNDER, TOP = 0.012, 0.440, 0.450
HALF = 0.100
COVER_IN = -0.090
SHEET = 0.0008
WALL_IN = HALF - SHEET
FLOOR_IN = FOOT + SHEET
FOOT_X, FOOT_Z, FOOT_L, FOOT_W = 0.180, 0.075, 0.040, 0.030
GRILLE = (-0.195, -0.035, 0.210, 0.340)
LATCH = (0.050, 0.120, 0.385, 0.415)
PSU_X, PSU_Y, PSU_Z = (-0.2245, -0.0845), (0.352, 0.438), (-0.075, 0.075)
PSU_TOP = (-0.205, -0.105, -0.085, 0.085)
FAN = 0.120
REAR_FAN = (-0.200, 0.285, -0.025)
CPU_FAN = (-0.085, 0.275, -0.003)
FRONT_FAN = (0.180, 0.200, 0.0)
IO = (0.045, 0.0895, 0.186, 0.345)
SLOT_Z = (-0.015, 0.085)
SLOT_TOP, SLOT_PITCH, SLOT_H, SLOTS = 0.170, 0.02032, 0.0125, 7
MESH = (-0.080, 0.080, 0.050, 0.350)
BAY = (-0.074, 0.074, 0.385, 0.427)
BOARD_X, BOARD_Y = (-0.211, 0.033), (0.045, 0.350)
BOARD_Z = (WALL_IN - 0.0081, WALL_IN - 0.0065)
BUTTON = (-0.060, 0.368)
STEEL, ABS, RUBBER = "chassis-steel", "bezel-abs", "rubber-foot"
FACE = {"+x": "-v", "-x": "+v", "+y": "+u", "-y": "-u", "+z": "+w", "-z": "-w"}
ALL_FACES = ["+u", "-u", "+v", "-v", "+w", "-w"]
MASSES = [
    ("chassis", 4.9, 0.0, 0.225),
    ("bezel", 0.4, 0.2125, 0.231),
    ("psu", 1.7, -0.1545, 0.395),
    ("board", 1.6, -0.095, 0.230),
    ("drives", 1.5, 0.110, 0.070),
    ("optical", 0.7, 0.126, 0.406),
    ("fans", 0.2, -0.010, 0.243),
]


def centre_of_mass():
    """⚖ The centre of mass (world x, y) of the component table, rounded to 5 mm; the masses add up to the declared 11 kg."""
    total = sum(m for _, m, _, _ in MASSES)
    x = sum(m * cx for _, m, cx, _ in MASSES) / total
    y = sum(m * cy for _, m, _, cy in MASSES) / total
    return round(total, 3), round(x / 0.005) * 0.005, round(y / 0.005) * 0.005


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(value + 0.0, 5)


def uvw(x, y, z):
    """🧭 A world point in the frame of the upright bone."""
    return [r6(y), r6(-x), r6(z)]


def faces(*world):
    """🪞 World face names as bone-local section faces."""
    return [FACE[name] for name in world]


def base(sid, kind, material, tone, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": "case", "material": material, "role": role, "tone": tone}


def box(sid, material, tone, x, y, z, radius=0.0, role="body", **more):
    """📦 A box between the world extents x, y, z (pairs)."""
    out = base(sid, "box", material, tone, role)
    out["size"] = [r6(y[1] - y[0]), r6(x[1] - x[0]), r6(z[1] - z[0])]
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": uvw((x[0] + x[1]) / 2, (y[0] + y[1]) / 2, (z[0] + z[1]) / 2)}
    out.update(more)
    return out


def up(sid, material, tone, profile_xz, y, holes=(), role="body", **more):
    """⬆ A prism standing up: a world profile of (x, z) points pushed from y[0] to y[1]."""
    out = base(sid, "extrude", material, tone, role)
    out.update({"profile": [[r6(-x), r6(z)] for x, z in profile_xz], "plane": "yz", "from": r6(y[0]), "to": r6(y[1])})
    if holes:
        out["holes"] = [[[r6(-x), r6(z)] for x, z in ring] for ring in holes]
    out.update(more)
    return out


def alongx(sid, material, tone, profile_zy, x, holes=(), role="body", **more):
    """↔ A prism through the depth: a world profile of (z, y) points pushed from x[0] to x[1]."""
    out = base(sid, "extrude", material, tone, role)
    out.update({"profile": [[r6(z), r6(y)] for z, y in profile_zy], "plane": "zx", "from": r6(-x[1]), "to": r6(-x[0])})
    if holes:
        out["holes"] = [[[r6(z), r6(y)] for z, y in ring] for ring in holes]
    out.update(more)
    return out


def alongz(sid, material, tone, profile_xy, z, holes=(), role="body", **more):
    """↘ A prism across the width: a world profile of (x, y) points pushed from z[0] to z[1]."""
    out = base(sid, "extrude", material, tone, role)
    out.update({"profile": [[r6(y), r6(-x)] for x, y in profile_xy], "plane": "xy", "from": r6(z[0]), "to": r6(z[1])})
    if holes:
        out["holes"] = [[[r6(y), r6(-x)] for x, y in ring] for ring in holes]
    out.update(more)
    return out


def turned_x(sid, material, tone, profile, origin, role="body", **more):
    """⭕ A revolve of an (axial, radial) profile about the world x axis through a world origin; axial runs towards the front."""
    out = base(sid, "revolve", material, tone, role)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": [0, -1, 0], "origin": uvw(*origin)})
    out.update(more)
    return out


def turned_y(sid, material, tone, profile, origin, role="body", **more):
    """🔘 A revolve of an (axial, radial) profile about the vertical through a world origin; axial runs upwards."""
    out = base(sid, "revolve", material, tone, role)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": [1, 0, 0], "origin": uvw(*origin)})
    out.update(more)
    return out


def capsule(sid, material, tone, p0, p1, radius, role="body", **more):
    """💊 A round bar between two world points."""
    out = base(sid, "capsule", material, tone, role)
    out.update({"p0": uvw(*p0), "p1": uvw(*p1), "r0": radius, "r1": radius})
    out.update(more)
    return out


def half_turn(solid, axis, origin):
    """🔃 A second copy turned half way round a world axis ('x', 'y' or 'z') through a world origin."""
    solid["repeat"] = {"count": 2, "rotationAxis": {"x": [0, -1, 0], "y": [1, 0, 0], "z": [0, 0, 1]}[axis], "rotationRad": r6(math.pi), "origin": uvw(*origin)}
    return solid


def step(solid, count, dx=0.0, dy=0.0, dz=0.0):
    """➡ Copies a solid `count` times along a world step."""
    solid["repeat"] = {"count": count, "translation": [r6(dy), r6(-dx), r6(dz)]}
    return solid


def rect(a0, a1, b0, b1):
    """⬜ A counter-clockwise rectangle."""
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def rounded(a0, a1, b0, b1, radius, steps=3):
    """🔲 A counter-clockwise rectangle with one corner radius."""
    if radius <= 0:
        return rect(a0, a1, b0, b1)
    points = []
    for ca, cb, start in ((a0 + radius, b0 + radius, 180), (a1 - radius, b0 + radius, 270), (a1 - radius, b1 - radius, 0), (a0 + radius, b1 - radius, 90)):
        for k in range(steps + 1):
            angle = math.radians(start + 90 * k / steps)
            points.append((ca + radius * math.cos(angle), cb + radius * math.sin(angle)))
    return points


def circle(ca, cb, radius, steps=24):
    """⚪ A counter-clockwise polygon of a circle."""
    return [(ca + radius * math.cos(2 * math.pi * k / steps), cb + radius * math.sin(2 * math.pi * k / steps)) for k in range(steps)]


def hexes(a0, a1, b0, b1, flats, pitch, margin=0.004):
    """⬡ The hexagonal holes of a perforated field: `flats` across the flats at `pitch` in staggered rows, only whole holes that keep `margin` to the edge of the field; the real 5 mm honeycomb generalised to what the scale can show."""
    out = []
    radius = flats / math.sqrt(3)
    rise = pitch * math.sqrt(3) / 2
    rows = int((b1 - b0 - 2 * margin - 2 * radius) / rise + 1e-9)
    b_start = (b0 + b1) / 2 - rows * rise / 2
    for j in range(rows + 1):
        cb = b_start + j * rise
        cols = int((a1 - a0 - 2 * margin - flats - (pitch / 2 if j % 2 else 0)) / pitch + 1e-9)
        a_start = (a0 + a1) / 2 - cols * pitch / 2
        for i in range(cols + 1):
            ca = a_start + i * pitch
            out.append([(r6(ca + radius * math.sin(math.radians(60 * k))), r6(cb + radius * math.cos(math.radians(60 * k)))) for k in range(6)])
    return out


def ring_slots(ca, cb, bands, spoke, spokes=4, steps=6):
    """🌀 The punched slots of a stamped fan guard: between `spokes` radial bars of half width `spoke` every (inner, outer) band is open."""
    out = []
    turn = 2 * math.pi / spokes
    for inner, outer in bands:
        for s in range(spokes):
            a_in, a_out = math.asin(spoke / inner), math.asin(spoke / outer)
            ring = []
            for k in range(steps + 1):
                angle = s * turn + math.pi / 4 + a_out + (turn - 2 * a_out) * k / steps
                ring.append((r6(ca + outer * math.cos(angle)), r6(cb + outer * math.sin(angle))))
            for k in range(steps + 1):
                angle = s * turn + math.pi / 4 + turn - a_in - (turn - 2 * a_in) * k / steps
                ring.append((r6(ca + inner * math.cos(angle)), r6(cb + inner * math.sin(angle))))
            out.append(ring)
    return out


def rig(old):
    """🦴 The corrected rig: the case bone stands upright from the floor under the middle of the case to its top, its mass at the centre of the component table."""
    total, cx, cy = centre_of_mass()
    bone = dict(old["bones"][0], x=0, y=0, angleDeg=90, lengthM=TOP)
    bone["mass"] = dict(bone["mass"], kg=total, comX=r6(cy), comY=r6(-cx))
    return dict(old, bones=[bone])
