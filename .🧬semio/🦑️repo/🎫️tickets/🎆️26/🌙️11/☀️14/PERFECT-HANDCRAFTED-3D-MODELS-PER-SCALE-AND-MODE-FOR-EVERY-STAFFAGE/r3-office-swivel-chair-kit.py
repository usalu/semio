"""🧰 Round 3, furniture/office-swivel-chair: the rig of the chair and the conversion of solids stated in world metres (x ahead, y up, z left, seat-low rest pose) into the bone-local solids of the schema.

Every solid of the chair is stated by hand in world coordinates in `r3-office-swivel-chair-build.py`; this file only carries the bookkeeping: rest frames of the bones, world to bone-local points, section axes to quaternions.
"""
import math

R_PIVOT = 0.30
TRAIL = 0.020
AXLE_Y = 0.025
BASE_Y = 0.090
SEAT_Y = 0.340
PIVOT = (-0.190, 0.365)
BACK_DEG = 12.0
AZIMUTH = {"front": 0.0, "side-a": 72.0, "side-b": -72.0, "back-a": 144.0, "back-b": -144.0}


def rnd(value, digits=5):
    out = round(value, digits)
    return 0.0 if out == 0 else out


def pivot_of(arm):
    a = math.radians(AZIMUTH[arm])
    return (R_PIVOT * math.cos(a), R_PIVOT * math.sin(a))


def text(en, de):
    return {"en": en, "de": de}


def rig():
    """🦴 The corrected rig: hub centre at the arm axis, castor joints at the wheel axles beside the symmetry plane, seat joint at the top of the gas spring, backrest joint at the hinge of the mechanism."""
    bones = [
        {"id": "base", "parent": None, "title": text("Five-arm base hub", "Nabe des Fünfarmfußes"), "x": 0, "y": BASE_Y, "angleDeg": 0, "lengthM": 0.1, "joint": {"type": "fixed"}, "mass": {"kg": 1.8, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"}},
    ]
    arms = [("front", "Front arm", "Vorderer Arm"), ("side-a", "Side arm A (projected)", "Seitenarm A (projiziert)"), ("side-b", "Side arm B (projected)", "Seitenarm B (projiziert)"), ("back-a", "Back arm A (projected)", "Hinterer Arm A (projiziert)"), ("back-b", "Back arm B (projected)", "Hinterer Arm B (projiziert)")]
    for key, en, de in arms:
        px, _ = pivot_of(key)
        bones.append({"id": f"arm-{key}", "parent": "base", "title": text(en, de), "x": 0, "y": 0, "angleDeg": 180 if px < 0 else 0, "lengthM": rnd(abs(px), 4), "joint": {"type": "fixed"}})
    castors = [("front", "Front castor", "Vordere Rolle"), ("side-a", "Side castor A", "Seitenrolle A"), ("side-b", "Side castor B", "Seitenrolle B"), ("back-a", "Back castor A", "Hintere Rolle A"), ("back-b", "Back castor B", "Hintere Rolle B")]
    for key, en, de in castors:
        px, pz = pivot_of(key)
        back = px < 0
        dx, dy = px - TRAIL, AXLE_Y - BASE_Y
        bone = {"id": f"castor-{key}", "parent": f"arm-{key}", "title": text(en, de), "x": rnd(-dx if back else dx, 4), "y": rnd(-dy if back else dy, 4)}
        if abs(pz) > 1e-9:
            bone["z"] = rnd(pz, 4)
        bone.update({"angleDeg": 180 if back else 0, "lengthM": 0.02, "joint": {"type": "revolute"}, "mass": {"kg": 0.15, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
        bones.append(bone)
    bones += [
        {"id": "column-sleeve", "parent": "base", "title": text("Gas-spring standpipe", "Standrohr der Gasfeder"), "x": 0, "y": 0, "angleDeg": 90, "lengthM": 0.195, "joint": {"type": "fixed"}},
        {"id": "column", "parent": "base", "title": text("Gas-spring piston tube", "Kolbenrohr der Gasfeder"), "x": 0, "y": 0, "angleDeg": 90, "lengthM": rnd(SEAT_Y - BASE_Y), "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.1}, "mass": {"kg": 1.6, "comX": 0.125, "comY": 0, "distribution": "uniform", "source": "authored-estimate"}},
        {"id": "seat", "parent": "column", "title": text("Seat on its mechanism", "Sitz auf seiner Mechanik"), "x": rnd(SEAT_Y - BASE_Y), "y": 0, "angleDeg": 0, "lengthM": 0.105, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0}, "mass": {"kg": 8.2, "comX": 0.086, "comY": 0.007, "distribution": "point", "source": "authored-estimate"}},
        {"id": "backrest", "parent": "seat", "title": text("Backrest on its carrier", "Rückenlehne auf ihrem Träger"), "x": rnd(PIVOT[1] - SEAT_Y), "y": rnd(-PIVOT[0]), "angleDeg": BACK_DEG, "lengthM": 0.645, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 20}, "mass": {"kg": 3.05, "comX": 0.345, "comY": 0.011, "distribution": "point", "source": "authored-estimate"}},
    ]
    return {"kind": "mechanical", "root": "base", "bones": bones}


def rest_frames(rig_value):
    """🧭 Rest frame of every bone: origin (x, y, z) and the world angle of u in degrees."""
    frames = {}
    by_id = {b["id"]: b for b in rig_value["bones"]}

    def solve(bone_id):
        if bone_id in frames:
            return frames[bone_id]
        b = by_id[bone_id]
        if b["parent"] is None:
            frames[bone_id] = ((b["x"], b["y"], b.get("z", 0.0)), b["angleDeg"])
            return frames[bone_id]
        (ox, oy, oz), deg = solve(b["parent"])
        c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
        frames[bone_id] = ((ox + c * b["x"] - s * b["y"], oy + s * b["x"] + c * b["y"], oz + b.get("z", 0.0)), deg + b["angleDeg"])
        return frames[bone_id]

    for b in rig_value["bones"]:
        solve(b["id"])
    return frames


FRAMES = rest_frames(rig())


def to_local(bone, p):
    (ox, oy, oz), deg = FRAMES[bone]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    dx, dy = p[0] - ox, p[1] - oy
    return [rnd(c * dx + s * dy), rnd(-s * dx + c * dy), rnd(p[2] - oz)]


def dir_local(bone, d):
    _, deg = FRAMES[bone]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [c * d[0] + s * d[1], -s * d[0] + c * d[1], d[2]]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(sum(t * t for t in a))
    return [t / n for t in a]


def quat(x_axis, y_axis):
    """🧮 Unit quaternion [x, y, z, w] of the rotation that takes the frame axes x, y, z to x_axis, y_axis and their cross product."""
    x = unit(x_axis)
    z = unit(cross(x, y_axis))
    y = cross(z, x)
    m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]]
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0:
        k = math.sqrt(trace + 1.0) * 2
        q = [(m[2][1] - m[1][2]) / k, (m[0][2] - m[2][0]) / k, (m[1][0] - m[0][1]) / k, 0.25 * k]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        k = math.sqrt(1.0 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [0.25 * k, (m[0][1] + m[1][0]) / k, (m[0][2] + m[2][0]) / k, (m[2][1] - m[1][2]) / k]
    elif m[1][1] > m[2][2]:
        k = math.sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / k, 0.25 * k, (m[1][2] + m[2][1]) / k, (m[0][2] - m[2][0]) / k]
    else:
        k = math.sqrt(1.0 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / k, (m[1][2] + m[2][1]) / k, 0.25 * k, (m[1][0] - m[0][1]) / k]
    if q[3] < 0:
        q = [-t for t in q]
    return [rnd(t, 6) for t in q]


def common(solid_id, bone, material, role, tone):
    return {"id": solid_id, "bone": bone, "material": material, "role": role, "tone": tone}


def loft(solid_id, bone, material, role, tone, sections, exponent=None):
    """🧵 Loft through sections stated in the world: (at, rx, ry, rx axis, ry axis[, exponent])."""
    out = []
    for s in sections:
        at, rx, ry, ax, ay = s[:5]
        e = s[5] if len(s) > 5 else exponent
        row = {"at": to_local(bone, at), "rx": rnd(rx), "ry": rnd(ry)}
        if e is not None:
            row["exponent"] = e
        row["rotation"] = quat(dir_local(bone, ax), dir_local(bone, ay))
        out.append(row)
    head = common(solid_id, bone, material, role, tone)
    return {"id": head["id"], "kind": "loft", **{k: v for k, v in head.items() if k != "id"}, "sections": out, "capStart": True, "capEnd": True}


def revolve(solid_id, bone, material, role, tone, origin, axis, profile):
    """🌀 Full revolve about a world axis through a world point; the profile is [axial, radial]."""
    head = common(solid_id, bone, material, role, tone)
    return {"id": head["id"], "kind": "revolve", **{k: v for k, v in head.items() if k != "id"}, "profile": [[rnd(a), rnd(r)] for a, r in profile], "axis": [rnd(t, 6) for t in dir_local(bone, axis)], "origin": to_local(bone, origin)}


def placed(solid, bone, x_axis, y_axis, at):
    """📌 Places a solid stated in its own frame: its x and y axes point along the given world axes, its origin sits at the world point."""
    solid["transform"] = {"rotation": quat(dir_local(bone, x_axis), dir_local(bone, y_axis)), "translation": to_local(bone, at)}
    return solid


def patch(solid_id, bone, material, role, tone, origin, axis, toward, sweep_deg, profile):
    """🪭 A revolve of less than a full turn about a world axis, its sweep centred on the world direction `toward` (perpendicular to the axis)."""
    half = math.radians(sweep_deg) / 2
    d = unit(axis)
    n = unit(toward)
    side = cross(n, d)
    x_axis = [math.cos(half) * n[i] + math.sin(half) * side[i] for i in range(3)]
    head = common(solid_id, bone, material, role, tone)
    solid = {"id": head["id"], "kind": "revolve", **{k: v for k, v in head.items() if k != "id"}, "profile": [[rnd(a), rnd(r)] for a, r in profile], "axis": [0, 1, 0], "origin": [0, 0, 0], "sweepRad": rnd(2 * half, 6)}
    return placed(solid, bone, x_axis, d, origin)


def extrude_xy(solid_id, bone, material, role, tone, points, z0, z1):
    """🏗 A side profile stated in world (x, y) pushed along z from z0 to z1."""
    head = common(solid_id, bone, material, role, tone)
    oz = FRAMES[bone][0][2]
    return {"id": head["id"], "kind": "extrude", **{k: v for k, v in head.items() if k != "id"}, "profile": [to_local(bone, (x, y, oz))[:2] for x, y in points], "plane": "xy", "from": rnd(z0 - oz), "to": rnd(z1 - oz)}


def box(solid_id, bone, material, role, tone, centre, size, corner=None, x_axis=(1, 0, 0), y_axis=(0, 1, 0)):
    """📦 A box with its edges along the given world axes (default the world axes), size along them and their cross product."""
    head = common(solid_id, bone, material, role, tone)
    solid = {"id": head["id"], "kind": "box", **{k: v for k, v in head.items() if k != "id"}, "size": [rnd(t) for t in size]}
    if corner is not None:
        solid["cornerRadius"] = corner
    return placed(solid, bone, x_axis, y_axis, centre)


def ellipsoid(solid_id, bone, material, role, tone, centre, radii, x_axis=(1, 0, 0), y_axis=(0, 1, 0)):
    head = common(solid_id, bone, material, role, tone)
    solid = {"id": head["id"], "kind": "ellipsoid", **{k: v for k, v in head.items() if k != "id"}, "radii": [rnd(t) for t in radii]}
    return placed(solid, bone, x_axis, y_axis, centre)


def capsule(solid_id, bone, material, role, tone, p0, p1, r0, r1):
    head = common(solid_id, bone, material, role, tone)
    return {"id": head["id"], "kind": "capsule", **{k: v for k, v in head.items() if k != "id"}, "p0": to_local(bone, p0), "p1": to_local(bone, p1), "r0": rnd(r0), "r1": rnd(r1)}
