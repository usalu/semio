"""🧰 Authoring kit of the round-3 standing adult: solids are stated by hand in the world of the rest pose (x ahead, y up, z to the figure's left, metres, ground at y = 0) and converted to the bone-local frame (u, v, w) of the bone that carries them.

A loft row is one hand-stated cross-section; `sub` only inserts smooth in-between sections (Catmull-Rom through the stated rows) so that the ruled kernel loft reads as a curved surface.
"""
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated/r3-standing-phone-user")
BOTH = json.load(open(os.path.join(OUT, "frames.json"), encoding="utf-8"))
FRAMES = dict(BOTH["rest"])


class posed:
    """🤸 Inside `with posed():` solids are stated in the world of the default pose (texting) instead of the rest pose; the bone-local result is the same kind of solid."""

    def __enter__(self):
        FRAMES.update(BOTH["pose"])

    def __exit__(self, *_):
        FRAMES.update(BOTH["rest"])
X, Y, Z = (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)


def r5(v):
    return round(v, 5)


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def unit(a):
    n = math.sqrt(dot(a, a)) or 1.0
    return (a[0] / n, a[1] / n, a[2] / n)


def local_point(bone, p):
    f = FRAMES[bone]
    d = (p[0] - f["o"][0], p[1] - f["o"][1], p[2] - f["o"][2])
    return [r5(dot(d, f["u"])), r5(dot(d, f["v"])), r5(dot(d, f["w"]))]


def local_dir(bone, d):
    f = FRAMES[bone]
    return (dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"]))


def quaternion(c0, c1, c2):
    m00, m10, m20 = c0
    m01, m11, m21 = c1
    m02, m12, m22 = c2
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        q = ((m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s)
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        q = (0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s)
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        q = ((m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s)
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2
        q = ((m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s)
    n = math.sqrt(sum(x * x for x in q))
    return [r5(x / n) for x in q]


def local_rotation(bone, a, b):
    a, b = unit(a), unit(b)
    n = cross(a, b)
    return quaternion(local_dir(bone, a), local_dir(bone, b), local_dir(bone, n))


def catmull(p0, p1, p2, p3, t):
    return 0.5 * ((2 * p1) + (-p0 + p2) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (-p0 + 3 * p1 - 3 * p2 + p3) * t * t * t)


def refine(rows, sub):
    if sub <= 0 or len(rows) < 3:
        return [list(r) for r in rows]
    out = []
    n = len(rows)
    for i in range(n - 1):
        p0, p1, p2, p3 = rows[max(i - 1, 0)], rows[i], rows[i + 1], rows[min(i + 2, n - 1)]
        out.append(list(p1))
        for k in range(1, sub + 1):
            t = k / (sub + 1)
            row = []
            for c in range(len(p1)):
                v = catmull(p0[c], p1[c], p2[c], p3[c], t)
                lo, hi = min(p1[c], p2[c]), max(p1[c], p2[c])
                span = hi - lo
                row.append(min(max(v, lo - 0.15 * span), hi + 0.15 * span))
            out.append(row)
    out.append(list(rows[-1]))
    return out


def base(sid, kind, bone, material, tone, role):
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def loft(sid, bone, material, tone, rows, axes, e=2.0, role="body", sub=0, caps=(True, True), mirror=False, **extra):
    """🎢 A loft whose rows are (s, c1, c2, r1, r2[, e[, yaw]]) along one world axis: axes "y" rows (y, x, z, rz, rx), "x" rows (x, y, z, rz, ry), "z" rows (z, x, y, rx, ry); `yaw` (degrees, "y" lofts only) turns the section about the vertical so that its second radius points from ahead towards the left."""
    full = [list(r) + ([e] if len(r) < 6 else []) + ([0.0] if len(r) < 7 else []) for r in rows]
    sections = []
    for s, c1, c2, r1, r2, ex, yaw in refine(full, sub):
        if axes == "y":
            f = math.radians(-yaw if mirror else yaw)
            p, a, b = (c1, s, -c2 if mirror else c2), (-math.sin(f), 0.0, math.cos(f)), (math.cos(f), 0.0, math.sin(f))
        elif axes == "x":
            p, a, b = (s, c1, -c2 if mirror else c2), Z, Y
        else:
            p, a, b = (c1, c2, -s if mirror else s), X, Y
        sections.append({"at": local_point(bone, p), "rx": r5(max(r1, 0.0005)), "ry": r5(max(r2, 0.0005)), "exponent": round(ex, 3), "rotation": local_rotation(bone, a, b)})
    return {**base(sid, "loft", bone, material, tone, role), "sections": sections, "capStart": caps[0], "capEnd": caps[1], **extra}


def free_loft(sid, bone, material, tone, rows, e=2.0, role="body", sub=0, caps=(True, True), mirror=False, **extra):
    """🧶 A loft along a free path: rows are (x, y, z, ra, rb) with the section normal taken from the path, `a` the horizontal direction across the path and `b` completing the frame."""
    pts = refine([list(r) for r in rows], sub)
    sections = []
    for i, (x, y, z, ra, rb) in enumerate(pts):
        q0, q1 = pts[max(i - 1, 0)], pts[min(i + 1, len(pts) - 1)]
        sgn = -1 if mirror else 1
        t = unit((q1[0] - q0[0], q1[1] - q0[1], sgn * (q1[2] - q0[2])))
        ref = Y if abs(t[1]) < 0.9 else X
        a = unit(cross(ref, t))
        b = cross(t, a)
        sections.append({"at": local_point(bone, (x, y, sgn * z)), "rx": r5(ra), "ry": r5(rb), "exponent": e, "rotation": local_rotation(bone, a, b)})
    return {**base(sid, "loft", bone, material, tone, role), "sections": sections, "capStart": caps[0], "capEnd": caps[1], **extra}


def capsule(sid, bone, material, tone, p0, p1, r0, r1, role="body", mirror=False, **extra):
    m = (lambda p: (p[0], p[1], -p[2])) if mirror else (lambda p: p)
    return {**base(sid, "capsule", bone, material, tone, role), "p0": local_point(bone, m(p0)), "p1": local_point(bone, m(p1)), "r0": r5(r0), "r1": r5(r1), **extra}


def ellipsoid(sid, bone, material, tone, centre, radii, role="body", mirror=False, tilt=None, **extra):
    """🥚 An ellipsoid with radii along world x, y, z, optionally tilted by `tilt` degrees about world z (nose-up positive)."""
    c = (centre[0], centre[1], -centre[2]) if mirror else centre
    ax, ay = X, Y
    if tilt is not None:
        t = math.radians(tilt)
        ax, ay = (math.cos(t), math.sin(t), 0.0), (-math.sin(t), math.cos(t), 0.0)
    q = quaternion(local_dir(bone, ax), local_dir(bone, ay), local_dir(bone, Z))
    return {**base(sid, "ellipsoid", bone, material, tone, role), "radii": [r5(radii[0]), r5(radii[1]), r5(radii[2])], "transform": {"translation": local_point(bone, c), "rotation": q}, **extra}


def box(sid, bone, material, tone, centre, size, role="body", mirror=False, tilt=None, radius=None, **extra):
    c = (centre[0], centre[1], -centre[2]) if mirror else centre
    ax, ay = X, Y
    if tilt is not None:
        t = math.radians(tilt)
        ax, ay = (math.cos(t), math.sin(t), 0.0), (-math.sin(t), math.cos(t), 0.0)
    q = quaternion(local_dir(bone, ax), local_dir(bone, ay), local_dir(bone, Z))
    out = {**base(sid, "box", bone, material, tone, role), "size": [r5(size[0]), r5(size[1]), r5(size[2])], "transform": {"translation": local_point(bone, c), "rotation": q}, **extra}
    if radius is not None:
        out["cornerRadius"] = radius
    return out


def both(make):
    """👯 Builds the left and the right twin of a part: `make(side, mirror)` returns a list of solids."""
    return make("l", False) + make("r", True)


def shrink(solid, t, sid, **extra):
    """🧅 The same solid `t` metres smaller all round (an inner layer shell): loft radii, ellipsoid radii or capsule radii reduced."""
    out = json.loads(json.dumps(solid))
    out["id"] = sid
    if out["kind"] == "loft":
        for section in out["sections"]:
            section["rx"] = r5(max(section["rx"] - t, 0.0005))
            section["ry"] = r5(max(section["ry"] - t, 0.0005))
    elif out["kind"] == "ellipsoid":
        out["radii"] = [r5(max(r - t, 0.0005)) for r in out["radii"]]
    elif out["kind"] == "capsule":
        out["r0"], out["r1"] = r5(max(out["r0"] - t, 0.0005)), r5(max(out["r1"] - t, 0.0005))
    else:
        raise SystemExit("no shell for " + out["kind"])
    out.update(extra)
    return out


def ref(of, region, layer=None, sid=None):
    """🔗 A thermal reference to the body solid `of`."""
    out = {"id": sid or "th-" + of, "of": of, "region": region}
    if layer:
        out["layer"] = layer
    return out


def th(solid, region, layer=None, tags=None):
    """🌡 A thermal solid of its own: `solid` bound to a region (and layer, and tags)."""
    out = dict(solid, region=region)
    if layer:
        out["layer"] = layer
    if tags:
        out["tags"] = tags
    return out


def st(solid, member, part="member", **extra):
    """🦴 A structural solid: `solid` as a part of the member of bone `member`."""
    return dict(solid, member=member, part=part, **extra)


def sphere(sid, bone, material, tone, centre, r, role="body", mirror=False, **extra):
    return ellipsoid(sid, bone, material, tone, centre, (r, r, r), role=role, mirror=mirror, **extra)


def rib_rows(y_back, drop, half_width, spine_x, front_x, radius, to=0.0, steps=10):
    """🎗 The path of one rib as free-loft rows: from its vertebra back and outwards to the rib angle, then a half ellipse in plan round the flank towards the sternum (angle `to`, 0 = at the sternum), sinking by `drop` from back to front."""
    back = spine_x - 0.030
    depth, xc = (front_x - back) / 2, (front_x + back) / 2
    rows = [(spine_x - 0.004, y_back + 0.004, 0.016, radius, radius)]
    start = math.pi - 0.42
    for k in range(steps + 1):
        a = start + (to - start) * k / steps
        z = half_width * math.sin(a) ** 0.85 + 0.014 * (1 - math.sin(a)) * (1 if a < math.pi / 2 else 0)
        rows.append((xc + depth * math.cos(a), y_back - drop * (1 + math.cos(a)) / 2, z, radius, radius))
    return rows


def face_rows(skull, edge):
    """🎭 The bare part of the head as loft rows: per stated height the skull section (rows of height, centre x, 0, half breadth, half depth) ahead of the hair edge `edge` (x of the hair line at that height), 0.4 mm proud of the skull solid so that the face is the exposed surface."""
    rows = []
    for y, back in edge:
        k = max(i for i, row in enumerate(skull) if row[0] >= y)
        a, b = skull[k], skull[min(k + 1, len(skull) - 1)]
        t = 0 if a[0] == b[0] else (a[0] - y) / (a[0] - b[0])
        cx, rz, rx = (a[i] + (b[i] - a[i]) * t for i in (1, 3, 4))
        back = max(back, cx - rx)
        front = cx + rx
        mid, half = (front + back) / 2, (front - back) / 2
        width = rz * math.sqrt(max(1 - ((mid - cx) / rx) ** 2, 0.02))
        rows.append((y, mid + 0.0004, 0, width + 0.0004, half))
    return rows


def to_pose(bone, p):
    """🧭 A point of the rest world, carried by `bone`, in the world of the default pose."""
    r, q = BOTH["rest"][bone], BOTH["pose"][bone]
    d = [p[i] - r["o"][i] for i in range(3)]
    loc = [dot(d, r[k]) for k in "uvw"]
    return tuple(round(q["o"][i] + loc[0] * q["u"][i] + loc[1] * q["v"][i] + loc[2] * q["w"][i], 4) for i in range(3))


def to_rest(bone, p):
    """🧭 A point of the default-pose world, carried by `bone`, in the world of the rest pose."""
    r, q = BOTH["rest"][bone], BOTH["pose"][bone]
    d = [p[i] - q["o"][i] for i in range(3)]
    loc = [dot(d, q[k]) for k in "uvw"]
    return tuple(round(r["o"][i] + loc[0] * r["u"][i] + loc[1] * r["v"][i] + loc[2] * r["w"][i], 4) for i in range(3))


UNDER = 0.026
PHONE_LENGTH, PHONE_WIDTH, PHONE_THICK = 0.147, 0.0715, 0.0078
GRIP_ANGLE = math.radians(40)
HAND_U, HAND_V = (math.cos(GRIP_ANGLE), math.sin(GRIP_ANGLE), 0.0), (math.sin(GRIP_ANGLE), -math.cos(GRIP_ANGLE), 0.0)
WRIST = (-0.105, -0.076, -0.024)


def grip_dir(d, hand="r"):
    """🧲 A direction of the grip frame (a along the phone from its lower to its upper edge, b across it towards the figure's left, n out of the screen) in the world of the default pose; the left hand is the mirror image in b."""
    f = BOTH["pose"]["phone"]
    b = -d[1] if hand == "l" else d[1]
    return tuple(d[0] * f["u"][i] - b * f["v"][i] + d[2] * f["w"][i] for i in range(3))


def grip(p, hand="r"):
    """🤲 A point of the grip frame (origin at the centre of the phone) in the world of the default pose; for the left hand mirrored in b and `UNDER` metres further behind the phone, where the left hand cradles the fingers of the right."""
    f = BOTH["pose"]["phone"]
    d = grip_dir((p[0] + PHONE_LENGTH / 2, p[1], p[2] - (UNDER if hand == "l" else 0.0)), hand)
    return tuple(f["o"][i] + d[i] for i in range(3))


def on_hand(s, t=0.0, n=0.0):
    """✋ A point of the grip frame given along the right hand: `s` from the wrist towards the knuckles, `t` towards the thumb side, `n` towards the palm side (out of the palm, towards the phone)."""
    return tuple(WRIST[i] + s * HAND_U[i] + t * HAND_V[i] + (n if i == 2 else 0.0) for i in range(3))


def frame_loft(sid, bone, material, tone, rows, e=2.0, role="body", caps=(True, True), **extra):
    """🧵 A loft through sections stated one by one: rows are (centre, axis a, axis b, ra, rb[, exponent]) in the current world."""
    sections = []
    for row in rows:
        c, a, b, ra, rb = row[:5]
        sections.append({"at": local_point(bone, c), "rx": r5(ra), "ry": r5(rb), "exponent": row[5] if len(row) > 5 else e, "rotation": local_rotation(bone, a, b)})
    return {**base(sid, "loft", bone, material, tone, role), "sections": sections, "capStart": caps[0], "capEnd": caps[1], **extra}


def ellipsoid_axes(sid, bone, material, tone, centre, a, b, radii, role="body", **extra):
    """🥚 An ellipsoid with its first two semi-axes along the directions `a` and `b` of the current world."""
    a, b = unit(a), unit(b)
    q = quaternion(local_dir(bone, a), local_dir(bone, b), local_dir(bone, cross(a, b)))
    return {**base(sid, "ellipsoid", bone, material, tone, role), "radii": [r5(r) for r in radii], "transform": {"translation": local_point(bone, centre), "rotation": q}, **extra}


def rounded_plate(sid, bone, material, tone, u0, u1, half, radius, w0, w1, role="body", inset=0.0, **extra):
    """📱 A plate in the frame of its own bone: a rectangle from `u0` to `u1` along the bone and `half` to either side, corners rounded with `radius`, from `w0` to `w1` across."""
    pts = []
    u0, u1, half, radius = u0 + inset, u1 - inset, half - inset, max(radius - inset, 0.001)
    for cu, cv, start in ((u1 - radius, half - radius, 0), (u0 + radius, half - radius, 90), (u0 + radius, -half + radius, 180), (u1 - radius, -half + radius, 270)):
        for k in range(5):
            t = math.radians(start + 22.5 * k)
            pts.append([r5(cu + radius * math.cos(t)), r5(cv + radius * math.sin(t))])
    return {**base(sid, "extrude", bone, material, tone, role), "profile": pts, "plane": "xy", "from": r5(w0), "to": r5(w1), **extra}


HAIRLINE = ((0.100, 1.662), (-0.058, 1.552))
HAIR_CAP = ((0.018, 1.632), (0.103, 0.086, 0.081))


def hairline_x(y):
    """💇 The x of the hairline plane at height y (rest pose): the hair is pulled back, its edge runs from the top of the forehead over the ears to the nape."""
    (x0, y0), (x1, y1) = HAIRLINE
    return x0 + (x1 - x0) * (y - y0) / (y1 - y0)


def hair_cap(sid, bone, material, tone, fractions, grow=0.0, **extra):
    """💇 The cap of pulled-back hair: the stated ellipsoid `HAIR_CAP` (8 mm of hair over the skull) behind and above the stated hairline plane `HAIRLINE`, as a loft through its slices parallel to that plane; `fractions` are the positions of the slices between the hairline (0) and the far pole of the cap (1); `grow` thickens the hair where a coarser scale merges it into a fuller mass."""
    (x0, y0), (x1, y1) = HAIRLINE
    (cx, cy), (a, b, c) = HAIR_CAP
    a, b, c = a + grow, b + grow, c + grow
    e = unit((x0 - x1, y0 - y1, 0.0))
    n = (-e[1], e[0], 0.0)
    reach = math.sqrt((n[0] * a) ** 2 + (n[1] * b) ** 2) - ((x0 - cx) * n[0] + (y0 - cy) * n[1])
    rows = []
    for f in fractions:
        t = reach * f
        dx, dy = x0 + t * n[0] - cx, y0 + t * n[1] - cy
        A = (e[0] / a) ** 2 + (e[1] / b) ** 2
        B = 2 * (dx * e[0] / a ** 2 + dy * e[1] / b ** 2)
        C = (dx / a) ** 2 + (dy / b) ** 2
        k = max(1 - C + B * B / (4 * A), 1e-6)
        s0 = -B / (2 * A)
        rows.append(((x0 + t * n[0] + s0 * e[0], y0 + t * n[1] + s0 * e[1], 0.0), Z, e, c * math.sqrt(k), math.sqrt(k / A)))
    return frame_loft(sid, bone, material, tone, rows, **extra)
