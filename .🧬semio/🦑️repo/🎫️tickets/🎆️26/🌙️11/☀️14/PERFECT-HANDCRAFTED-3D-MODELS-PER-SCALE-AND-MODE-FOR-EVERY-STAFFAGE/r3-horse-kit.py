"""🧰 Round 3 horse: authoring kit. Every station of every solid is stated by hand in the rest-pose world of the rig (x ahead, y up from the ground, z lateral with the far, left side positive) and carried into the rest frame of its bone here; nothing in this file guesses a form.

Loaded by r3-horse-model.py through importlib (the file name carries the asset prefix).
"""
import json
import math

FILE = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/horse/🔣️.json"
RAD = math.pi / 180


def rest_frames(asset):
    """🦴 Rest frames of all bones in the sagittal plane: id -> (ox, oy, angleDeg), the ground at y = 0 under the fore hoof sole."""
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    out = {}

    def solve(i):
        if i in out:
            return out[i]
        b = bones[i]
        if b["parent"] is None:
            out[i] = (b["x"], b["y"], b["angleDeg"])
        else:
            px, py, pa = solve(b["parent"])
            c, s = math.cos(pa * RAD), math.sin(pa * RAD)
            out[i] = (px + b["x"] * c - b["y"] * s, py + b["x"] * s + b["y"] * c, pa + b["angleDeg"])
        return out[i]

    for i in bones:
        solve(i)
    a = next(x for x in asset["anchors"] if x["id"] == "contact-fn")
    ox, oy, ang = out[a["bone"]]
    ground = oy + a["x"] * math.sin(ang * RAD) + a["y"] * math.cos(ang * RAD)
    return {i: (x, y - ground, t) for i, (x, y, t) in out.items()}


ASSET = json.load(open(FILE, encoding="utf-8"))["asset"]
F = rest_frames(ASSET)
LEN = {b["id"]: b["lengthM"] for b in ASSET["rig"]["bones"]}


def r5(x):
    v = round(x, 5)
    return 0.0 if v == 0 else v


def local(bone, x, y):
    """📍 World (x, y) in the rest frame (u, v) of a bone."""
    ox, oy, t = F[bone]
    c, s = math.cos(t * RAD), math.sin(t * RAD)
    dx, dy = x - ox, y - oy
    return dx * c + dy * s, -dx * s + dy * c


def world(bone, u, v):
    """🌍 Bone-local (u, v) in the rest world."""
    ox, oy, t = F[bone]
    c, s = math.cos(t * RAD), math.sin(t * RAD)
    return ox + u * c - v * s, oy + u * s + v * c


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return (aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz)


def qz(deg):
    return (0.0, 0.0, math.sin(deg * RAD / 2), math.cos(deg * RAD / 2))


Q0 = (0.5, 0.5, 0.5, 0.5)


def section_rotation(phi_local, roll=0.0):
    """🔄 Quaternion of a loft section whose normal lies in the sagittal plane at `phi_local` degrees from the bone axis: rx is the sagittal radius, ry the lateral one; `roll` turns the section about its normal."""
    q = qmul(qmul(qz(phi_local), Q0), qz(roll))
    return [r5(c) for c in q]


def loft(sid, bone, stations, material="short-coat", tone=0.62, role="body", exp=2.2, caps=(True, True), mirror=1, extra=None):
    """🧵 A loft from hand-set stations (x, y, z, rs, rl[, phi[, exponent[, roll]]]) in the rest world: rs is the radius in the sagittal plane, rl the lateral radius, phi the world angle of the section normal (default: the path tangent in the sagittal plane). `mirror` -1 writes the near-side twin."""
    out = []
    n = len(stations)
    for i, st in enumerate(stations):
        x, y, z, rs, rl = st[:5]
        phi = st[5] if len(st) > 5 and st[5] is not None else None
        e = st[6] if len(st) > 6 and st[6] is not None else exp
        roll = st[7] if len(st) > 7 else 0.0
        if phi is None:
            a, b = stations[max(0, i - 1)], stations[min(n - 1, i + 1)]
            phi = math.atan2(b[1] - a[1], b[0] - a[0]) / RAD
        u, v = local(bone, x, y)
        sec = {"at": [r5(u), r5(v), r5(z * mirror)], "rx": r5(rs), "ry": r5(rl), "exponent": e, "rotation": section_rotation(phi - F[bone][2], roll * mirror)}
        out.append(sec)
    solid = {"id": sid, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": out, "capStart": caps[0], "capEnd": caps[1]}
    if extra:
        solid.update(extra)
    return solid


def along(bone, rows, z=0.0):
    """📏 Stations along a bone from rows (t, v, rs, rl[, z[, exponent]]): t metres from the joint along the bone, v the offset normal to it; the sections are normal to the bone."""
    out = []
    for row in rows:
        t, v, rs, rl = row[:4]
        zz = row[4] if len(row) > 4 and row[4] is not None else z
        e = row[5] if len(row) > 5 else None
        x, y = world(bone, t, v)
        out.append((x, y, zz, rs, rl, F[bone][2], e))
    return out


def ellipsoid(sid, bone, centre, radii, material="short-coat", tone=0.62, role="body", angle=0.0, mirror=1, extra=None):
    """🥚 An ellipsoid at a world centre (x, y, z) with radii (along `angle` in the sagittal plane, normal to it, lateral)."""
    u, v = local(bone, centre[0], centre[1])
    solid = {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone, "radii": [r5(radii[0]), r5(radii[1]), r5(radii[2])], "transform": {"translation": [r5(u), r5(v), r5(centre[2] * mirror)], "rotation": [r5(c) for c in qz(angle - F[bone][2])]}}
    if extra:
        solid.update(extra)
    return solid


def capsule(sid, bone, p0, p1, r0, r1, material="short-coat", tone=0.62, role="body", mirror=1, lateral=None, extra=None):
    """💊 A capsule between two world points (x, y, z)."""
    a, b = local(bone, p0[0], p0[1]), local(bone, p1[0], p1[1])
    solid = {"id": sid, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": [r5(a[0]), r5(a[1]), r5(p0[2] * mirror)], "p1": [r5(b[0]), r5(b[1]), r5(p1[2] * mirror)], "r0": r5(r0), "r1": r5(r1)}
    if lateral is not None:
        solid["lateralScale"] = lateral
    if extra:
        solid.update(extra)
    return solid


def plate(sid, bone, outline, z0, z1, material="short-coat", tone=0.62, role="body", mirror=1, extra=None):
    """🪵 A flat plate: a world outline [(x, y), ...] in the sagittal plane pushed laterally from z0 to z1."""
    pts = [[r5(c) for c in local(bone, x, y)] for x, y in outline]
    lo, hi = sorted((z0 * mirror, z1 * mirror))
    solid = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": pts, "plane": "xy", "from": r5(lo), "to": r5(hi)}
    if extra:
        solid.update(extra)
    return solid


def hoop(sid, bone, centre, radius, half_width, bar, tilt=0.0, sweep=None, material="short-coat", tone=0.62, extra=None):
    """⭕ A rib hoop: an elliptic ring about the trunk axis through a world centre (x, y), `radius` its vertical half depth, `half_width` its lateral half width, `bar` = (along the axis, radial) half sizes of the rib section, tilted by `tilt` degrees (the lower end swept back); `sweep` degrees opens the ring at the bottom."""
    a, t = bar
    profile = [[-a, radius - t], [a, radius - t], [a, radius + t], [-a, radius + t], [-a, radius - t]]
    u, v = local(bone, centre[0], centre[1])
    q = qz(-tilt)
    solid = {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": [[r5(p[0]), r5(p[1])] for p in profile], "axis": [1, 0, 0], "origin": [0, 0, 0], "lateralScale": r5(half_width / radius)}
    if sweep is not None:
        solid["sweepRad"] = r5(sweep * RAD)
        q = qmul(q, (math.sin(-sweep * RAD / 4), 0.0, 0.0, math.cos(-sweep * RAD / 4)))
    solid["transform"] = {"translation": [r5(u), r5(v), 0.0], "rotation": [r5(c) for c in q]}
    if extra:
        solid.update(extra)
    return solid


SIDES = (("far", "f", 1), ("near", "n", -1))
