"""🧰 Authoring kit of the round-3 worker carrying a box: the rig solved in Python exactly as the product solves it (sagittal angle, then spread about the bone normal, then twist about the bone axis), and solids stated by hand in the world of a pose (x ahead, y up, z to the figure's left, metres, ground at y = 0) converted to the bone-local frame (u, v, w) of the bone that carries them.

A loft row is one hand-stated cross-section; `sub` only inserts smooth in-between sections (Catmull-Rom through the stated rows) so that the ruled kernel loft reads as a curved surface.
"""
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated/r3-worker-carrying-box")
DRAFT = os.path.join(OUT, "asset.json")
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


def comb(a, s, b, t):
    return (a[0] * s + b[0] * t, a[1] * s + b[1] * t, a[2] * s + b[2] * t)


def add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def sub3(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def clamp(v, lo, hi):
    return min(max(v, lo), hi)


def solve(rig, offsets):
    """🦴 The 3D frame of every bone for pose offsets `{bone: {angleDeg, abductionDeg, twistDeg}}`, root at the origin."""
    bones = rig["bones"]
    done = {}
    pending = list(bones)
    while pending:
        rest = []
        for bone in pending:
            if bone["parent"] is not None and bone["parent"] not in done:
                rest.append(bone)
                continue
            parent = done[bone["parent"]] if bone["parent"] else {"o": (0.0, 0.0, 0.0), "u": X, "v": Y, "w": Z}
            off = offsets.get(bone["id"], {})
            joint = bone["joint"]
            angle = clamp(off.get("angleDeg", 0), joint.get("minDeg", -1e9), joint.get("maxDeg", 1e9))
            along = comb(parent["u"], bone["x"], parent["v"], bone["y"])
            side = bone.get("z", 0)
            origin = add(add(parent["o"], along), comb(parent["w"], side, X, 0))
            t = math.radians(bone["angleDeg"] + angle)
            u1 = comb(parent["u"], math.cos(t), parent["v"], math.sin(t))
            v1 = comb(parent["u"], -math.sin(t), parent["v"], math.cos(t))
            p = math.radians(off.get("abductionDeg", 0))
            u2 = comb(u1, math.cos(p), parent["w"], math.sin(p))
            w2 = comb(u1, -math.sin(p), parent["w"], math.cos(p))
            s = math.radians(off.get("twistDeg", 0))
            done[bone["id"]] = {"o": origin, "u": u2, "v": comb(v1, math.cos(s), w2, -math.sin(s)), "w": comb(v1, math.sin(s), w2, math.cos(s))}
        pending = rest
    return done


def anchor_point(asset, frames, anchor_id):
    a = next(x for x in asset["anchors"] if x["id"] == anchor_id)
    f = frames[a["bone"]]
    return add(f["o"], add(comb(f["u"], a["x"], f["v"], a["y"]), comb(f["w"], a.get("z", 0), X, 0)))


def grounded(asset, offsets, contacts):
    """🌍 Frames of a pose with the lowest listed contact anchor on y = 0 (the product's `ground: auto`)."""
    frames = solve(asset["rig"], offsets)
    low = min(anchor_point(asset, frames, c)[1] for c in contacts)
    for f in frames.values():
        f["o"] = (f["o"][0], f["o"][1] - low, f["o"][2])
    return frames


def pose_frames(asset, pose_id=None):
    """🧍 Frames of the named pose (default: the rest pose standing on its soles)."""
    if pose_id is None:
        return grounded(asset, {}, ["contact-l", "contact-r", "heel-l", "heel-r"])
    pose = next(p for p in asset["poses"] if p["id"] == pose_id)
    return grounded(asset, pose["bones"], pose["contacts"])


class Space:
    """🗺 The world of one pose: converts hand-stated world points and directions into bone-local coordinates."""

    def __init__(self, frames):
        self.frames = frames

    def point(self, bone, p):
        f = self.frames[bone]
        d = sub3(p, f["o"])
        return [r5(dot(d, f["u"])), r5(dot(d, f["v"])), r5(dot(d, f["w"]))]

    def direction(self, bone, d):
        f = self.frames[bone]
        return (dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"]))

    def world(self, bone, q):
        f = self.frames[bone]
        return add(f["o"], add(comb(f["u"], q[0], f["v"], q[1]), comb(f["w"], q[2], X, 0)))

    def rotation(self, bone, a, b):
        a, b = unit(a), unit(b)
        n = cross(a, b)
        return quaternion(self.direction(bone, a), self.direction(bone, b), self.direction(bone, n))


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


def flip(p, mirror):
    return (p[0], p[1], -p[2]) if mirror else p


def loft(space, sid, bone, material, tone, rows, axes="y", e=2.0, role="body", sub=0, caps=(True, True), mirror=False, **extra):
    """🎢 A loft whose rows are (s, c1, c2, r1, r2[, e[, yaw]]) along one world axis: axes "y" rows (y, x, z, rz, rx), "x" rows (x, y, z, rz, ry), "z" rows (z, x, y, rx, ry); `yaw` (degrees, "y" lofts only) turns the section about the vertical."""
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
        sections.append({"at": space.point(bone, p), "rx": r5(max(r1, 0.0005)), "ry": r5(max(r2, 0.0005)), "exponent": round(ex, 3), "rotation": space.rotation(bone, a, b)})
    return {**base(sid, "loft", bone, material, tone, role), "sections": sections, "capStart": caps[0], "capEnd": caps[1], **extra}


def free_loft(space, sid, bone, material, tone, rows, e=2.0, role="body", sub=0, caps=(True, True), mirror=False, across=None, **extra):
    """🧶 A loft along a free path: rows are (x, y, z, ra, rb[, e]) with the section normal taken from the path, `ra` along the direction `across` (default: horizontal, perpendicular to the path) made perpendicular to the path, and `rb` completing the frame."""
    pts = refine([list(r) + ([e] if len(r) < 6 else []) for r in rows], sub)
    sgn = -1 if mirror else 1
    sections = []
    for i, (x, y, z, ra, rb, ex) in enumerate(pts):
        q0, q1 = pts[max(i - 1, 0)], pts[min(i + 1, len(pts) - 1)]
        t = unit((q1[0] - q0[0], q1[1] - q0[1], sgn * (q1[2] - q0[2])))
        if across is None:
            a = unit(cross(Y if abs(t[1]) < 0.9 else X, t))
        else:
            want = flip(across, mirror)
            a = unit(sub3(want, comb(t, dot(want, t), X, 0)))
        b = cross(t, a)
        sections.append({"at": space.point(bone, (x, y, sgn * z)), "rx": r5(max(ra, 0.0005)), "ry": r5(max(rb, 0.0005)), "exponent": round(ex, 3), "rotation": space.rotation(bone, a, b)})
    return {**base(sid, "loft", bone, material, tone, role), "sections": sections, "capStart": caps[0], "capEnd": caps[1], **extra}


def local_loft(sid, bone, material, tone, rows, e=2.0, role="body", sub=0, caps=(True, True), mirror=False, **extra):
    """🦵 A loft stated in the frame of its own bone: rows are (u, v, w, rv, rw[, e]) with the section in the (v, w) plane; `mirror` flips w for the right-hand twin."""
    full = [list(r) + ([e] if len(r) < 6 else []) for r in rows]
    sections = []
    for u, v, w, rv, rw, ex in refine(full, sub):
        sections.append({"at": [r5(u), r5(v), r5(-w if mirror else w)], "rx": r5(max(rv, 0.0005)), "ry": r5(max(rw, 0.0005)), "exponent": round(ex, 3), "rotation": quaternion((0, 1, 0), (0, 0, 1), (1, 0, 0))})
    return {**base(sid, "loft", bone, material, tone, role), "sections": sections, "capStart": caps[0], "capEnd": caps[1], **extra}


def capsule(space, sid, bone, material, tone, p0, p1, r0, r1, role="body", mirror=False, **extra):
    return {**base(sid, "capsule", bone, material, tone, role), "p0": space.point(bone, flip(p0, mirror)), "p1": space.point(bone, flip(p1, mirror)), "r0": r5(r0), "r1": r5(r1), **extra}


def axes_of(tilt, yaw, mirror):
    """🧭 World axes of a part tilted `tilt` degrees about z (nose-up positive) and then turned `yaw` degrees about the vertical (towards the left positive)."""
    t, f = math.radians(tilt or 0), math.radians((-(yaw or 0)) if mirror else (yaw or 0))
    ax, ay, az = (math.cos(t), math.sin(t), 0.0), (-math.sin(t), math.cos(t), 0.0), Z
    turn = lambda v: (v[0] * math.cos(f) - v[2] * math.sin(f), v[1], v[0] * math.sin(f) + v[2] * math.cos(f))
    return turn(ax), turn(ay), turn(az)


def ellipsoid(space, sid, bone, material, tone, centre, radii, role="body", mirror=False, tilt=None, yaw=None, **extra):
    """🥚 An ellipsoid with radii along world x, y, z, optionally tilted and turned."""
    ax, ay, az = axes_of(tilt, yaw, mirror)
    q = quaternion(space.direction(bone, ax), space.direction(bone, ay), space.direction(bone, az))
    return {**base(sid, "ellipsoid", bone, material, tone, role), "radii": [r5(radii[0]), r5(radii[1]), r5(radii[2])], "transform": {"translation": space.point(bone, flip(centre, mirror)), "rotation": q}, **extra}


def box(space, sid, bone, material, tone, centre, size, role="body", mirror=False, tilt=None, yaw=None, radius=None, **extra):
    ax, ay, az = axes_of(tilt, yaw, mirror)
    q = quaternion(space.direction(bone, ax), space.direction(bone, ay), space.direction(bone, az))
    out = {**base(sid, "box", bone, material, tone, role), "size": [r5(size[0]), r5(size[1]), r5(size[2])], "transform": {"translation": space.point(bone, flip(centre, mirror)), "rotation": q}, **extra}
    if radius is not None:
        out["cornerRadius"] = radius
    return out


def extrude_in(space, sid, bone, material, tone, origin, ax, ay, profile, depth, holes=None, role="body", **extra):
    """🧇 A polygon (with holes) on the plane through `origin` spanned by the world directions `ax`, `ay`, pushed by `depth` along ax x ay: stated as an extrude on the bone's (u, v) plane and placed by a transform."""
    out = {**base(sid, "extrude", bone, material, tone, role), "plane": "xy", "profile": [[r5(p[0]), r5(p[1])] for p in profile], "from": 0, "to": r5(depth), "transform": {"translation": space.point(bone, origin), "rotation": space.rotation(bone, ax, ay)}, **extra}
    if holes:
        out["holes"] = [[[r5(p[0]), r5(p[1])] for p in hole] for hole in holes]
    return out


def shrink(solid, t, sid, **extra):
    """🧅 The same solid `t` metres smaller all round (an inner layer shell): loft, ellipsoid, capsule radii or box sizes reduced."""
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
    elif out["kind"] == "box":
        out["size"] = [r5(max(s - 2 * t, 0.001)) for s in out["size"]]
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


def th(solid, region, layer=None, tags=None, sid=None):
    """🌡 A thermal solid of its own: `solid` bound to a region (and layer, and tags)."""
    out = dict(json.loads(json.dumps(solid)), region=region)
    if sid:
        out["id"] = sid
    if layer:
        out["layer"] = layer
    if tags:
        out["tags"] = tags
    return out


def st(solid, member, part="member", **extra):
    """🏗 A structural solid: `solid` as a part of the member of bone `member`."""
    return dict(solid, member=member, part=part, **extra)


def draft():
    return json.load(open(DRAFT, encoding="utf-8"))["asset"]
