"""Helpers that compose explicit VolumeSolid data for the appliances-b authoring scripts; the output is plain JSON that is reviewed solid by solid."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def r(x):
    return round(x, 5)


def vec(v):
    return [r(x) for x in v]


def solid(id, kind, bone, material, role, tone, region=None, maxd=None, translate=None, rotate=None, scale=None, repeat=None, weights=None, **geometry):
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    if isinstance(region, (list, tuple)):
        out["regions"] = list(region)
    elif region is not None:
        out["region"] = region
    if maxd is not None:
        out["maxDenominator"] = maxd
    for key, value in geometry.items():
        out[key] = [vec(p) if isinstance(p, (list, tuple)) else r(p) for p in value] if isinstance(value, (list, tuple)) and key in ("size", "radii", "p0", "p1", "axis", "origin") else value
    transform = {}
    if scale is not None:
        transform["scale"] = vec(scale)
    if rotate is not None:
        transform["rotation"] = vec(rotate)
    if translate is not None:
        transform["translation"] = vec(translate)
    if transform:
        out["transform"] = transform
    if repeat is not None:
        out["repeat"] = repeat
    if weights is not None:
        out["weights"] = weights
    return out


def box(id, bone, material, role, tone, region, size, at=(0, 0, 0), corner=None, **kw):
    extra = {"cornerRadius": r(corner)} if corner else {}
    return solid(id, "box", bone, material, role, tone, region, size=list(size), translate=list(at), **extra, **kw)


def ellipsoid(id, bone, material, role, tone, region, radii, at=(0, 0, 0), **kw):
    return solid(id, "ellipsoid", bone, material, role, tone, region, radii=list(radii), translate=list(at), **kw)


def capsule(id, bone, material, role, tone, region, p0, p1, r0, r1=None, lateral=None, **kw):
    extra = {"lateralScale": lateral} if lateral else {}
    return solid(id, "capsule", bone, material, role, tone, region, p0=list(p0), p1=list(p1), r0=r(r0), r1=r(r0 if r1 is None else r1), **extra, **kw)


def revolve(id, bone, material, role, tone, region, profile, axis=(1, 0, 0), origin=(0, 0, 0), lateral=None, sweep=None, **kw):
    extra = {}
    if lateral:
        extra["lateralScale"] = lateral
    if sweep:
        extra["sweepRad"] = sweep
    return solid(id, "revolve", bone, material, role, tone, region, profile=[[r(a), r(b)] for a, b in profile], axis=list(axis), origin=list(origin), **extra, **kw)


def extrude(id, bone, material, role, tone, region, profile, plane, frm, to, holes=None, taper=None, **kw):
    extra = {}
    if holes:
        extra["holes"] = [[[r(a), r(b)] for a, b in h] for h in holes]
    if taper:
        extra["taperTo"] = list(taper)
    return solid(id, "extrude", bone, material, role, tone, region, profile=[[r(a), r(b)] for a, b in profile], plane=plane, **{"from": r(frm), "to": r(to)}, **extra, **kw)


def _unit(a):
    n = math.sqrt(sum(x * x for x in a)) or 1.0
    return [x / n for x in a]


def _cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


cross3 = _cross


def qbasis(x, y, z):
    m00, m10, m20 = x
    m01, m11, m21 = y
    m02, m12, m22 = z
    t = m00 + m11 + m22
    if t > 0:
        s = math.sqrt(t + 1) * 2
        return [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4]
    if m00 > m11 and m00 > m22:
        s = math.sqrt(1 + m00 - m11 - m22) * 2
        return [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    if m11 > m22:
        s = math.sqrt(1 + m11 - m00 - m22) * 2
        return [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s]
    s = math.sqrt(1 + m22 - m00 - m11) * 2
    return [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s]


def path_rotation(ats, i):
    a = ats[min(i, len(ats) - 2)]
    b = ats[min(i + 1, len(ats) - 1)]
    z = _unit([b[k] - a[k] for k in range(3)])
    d = z[2]
    x = [0 - d * z[0], 0 - d * z[1], 1 - d * z[2]]
    if math.sqrt(sum(c * c for c in x)) < 1e-6:
        d = z[1]
        x = [0 - d * z[0], 1 - d * z[1], 0 - d * z[2]]
    x = _unit(x)
    y = _cross(z, x)
    return [r(c) for c in qbasis(x, y, z)]


def loft(id, bone, material, role, tone, region, sections, cap=(True, True), **kw):
    ats = [s[0] for s in sections]
    secs = []
    for i, s in enumerate(sections):
        item = {"at": vec(s[0]), "rx": r(s[1]), "ry": r(s[2])}
        if len(s) > 3 and s[3] is not None:
            item["exponent"] = s[3]
        item["rotation"] = s[4] if len(s) > 4 and s[4] is not None else path_rotation(ats, i)
        secs.append(item)
    return solid(id, "loft", bone, material, role, tone, region, sections=secs, capStart=cap[0], capEnd=cap[1], **kw)


def qz(angle):
    return [0, 0, r(math.sin(angle / 2)), r(math.cos(angle / 2))]


FRAMES = {}


def frames(slug):
    path = os.path.join(HERE, "..", "..", "🗑️generated", "authoring-vehicles", slug + ".frames.json")
    FRAMES.update(json.load(open(path)))
    return FRAMES


def loc(bone, p):
    f = FRAMES[bone]
    d = [p[k] - f["o"][k] for k in range(3)]
    return [sum(d[k] * f[a][k] for k in range(3)) for a in ("u", "v", "w")]


def locd(bone, p):
    f = FRAMES[bone]
    return [sum(p[k] * f[a][k] for k in range(3)) for a in ("u", "v", "w")]


def tyre_ring(R, hw, inner=None, shoulder=0.18):
    sh = shoulder * R
    return [(-hw, R - 1.5 * sh), (-hw * 0.88, R - 0.35 * sh), (-hw * 0.6, R), (hw * 0.6, R), (hw * 0.88, R - 0.35 * sh), (hw, R - 1.5 * sh)]


def wheel(prefix, bone, material_tyre, region_tyre, wc, R, hw, inner, rim_r, rim_hw, spokes=5, spoke_r=0.012, hub_r=0.045, tone_tyre=0.1, flip=1, rim_mat="alloy", rim_region="brakes", rim_tone=0.55, spoke_mat="alloy", spoke_tone=0.8, maxd=None, shoulder=0.18):
    out = []
    o = [0, 0, wc]
    out.append(revolve(prefix + "-tyre", bone, material_tyre, "body", tone_tyre, region_tyre, tyre_ring(R, hw, shoulder=shoulder), axis=(0, 0, 1), origin=o))
    out.append(revolve(prefix + "-rim", bone, rim_mat, "detail", rim_tone, rim_region, [(-rim_hw, rim_r), (rim_hw, rim_r)], axis=(0, 0, 1), origin=o))
    out.append(revolve(prefix + "-hub", bone, rim_mat, "detail", 0.7, rim_region, [(-rim_hw - 0.012, hub_r), (rim_hw + 0.012, hub_r)], axis=(0, 0, 1), origin=o))
    if spokes:
        out.append(capsule(prefix + "-spoke", bone, spoke_mat, "detail", spoke_tone, rim_region, (hub_r * 0.6, 0, wc + (rim_hw + spoke_r * 0.5) * flip), (rim_r - 0.012, 0, wc + (rim_hw + spoke_r * 0.5) * flip), spoke_r, spoke_r, repeat={"count": spokes, "rotationAxis": [0, 0, 1], "rotationRad": r(2 * math.pi / spokes)}, maxd=maxd))
    return out


def rrect(cx, cy, w, h, n=0, rad=0.0):
    x0, x1, y0, y1 = cx - w / 2, cx + w / 2, cy - h / 2, cy + h / 2
    if not rad:
        return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
    pts = []
    for (px, py, a0) in ((x1 - rad, y0 + rad, -90), (x1 - rad, y1 - rad, 0), (x0 + rad, y1 - rad, 90), (x0 + rad, y0 + rad, 180)):
        for k in range(5):
            a = math.radians(a0 + 90 * k / 4)
            pts.append((px + rad * math.cos(a), py + rad * math.sin(a)))
    return pts


def dec(en, de):
    return {"en": en, "de": de}


def write(slug, solids, decisions):
    ids = [s["id"] for s in solids]
    assert len(ids) == len(set(ids)), "duplicate ids"
    path = os.path.join(HERE, slug + ".json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volume": {"status": "authored", "solids": solids, "decisions": decisions}}, f, ensure_ascii=False, indent=1)
    print("wrote", path, len(solids), "solids")


def ring(id, bone, material, role, tone, region, Rc, rt, wc=0.0, n=36, lateral=None, **kw):
    a = 2 * math.pi / n
    return capsule(id, bone, material, role, tone, region, (Rc, 0, wc), (Rc * math.cos(a), Rc * math.sin(a), wc), rt, rt, repeat={"count": n, "rotationAxis": [0, 0, 1], "rotationRad": r(a)}, **kw)


def arc_pts(c, R, a0, a1, step=10):
    n = max(2, int(abs(a1 - a0) / step) + 1)
    return [(c[0] + R * math.cos(math.radians(a0 + (a1 - a0) * k / (n - 1))), c[1] + R * math.sin(math.radians(a0 + (a1 - a0) * k / (n - 1)))) for k in range(n)]


def arc_strip(c, r_in, r_out, a0, a1, step=10):
    return arc_pts(c, r_out, a0, a1, step) + arc_pts(c, r_in, a0, a1, step)[::-1]


def spokes(prefix, bone, material, tone, region, r_hub, r_rim, w_flange, count, spoke_r, maxd=None, offset_deg=0, lace_deg=0.0):
    out = []
    for sgn, nm in ((1, "l"), (-1, "r")):
        a = math.radians(offset_deg + (180 / count if sgn < 0 else 0))
        b = a + math.radians(lace_deg)
        p0 = (r_hub * math.cos(a), r_hub * math.sin(a), sgn * w_flange)
        p1 = (r_rim * math.cos(b), r_rim * math.sin(b), 0.0)
        out.append(capsule(f"{prefix}-{nm}", bone, material, "detail", tone, region, p0, p1, spoke_r, spoke_r, repeat={"count": count, "rotationAxis": [0, 0, 1], "rotationRad": r(2 * math.pi / count)}, maxd=maxd))
    return out


def flat_tube(id, bone, material, role, tone, region, a, b, r_v, r_lat, **kw):
    """Capsule built on the u axis (so the lateral stretch acts along w) and moved to the segment a-b in the u-v plane at constant w."""
    dx, dy = b[0] - a[0], b[1] - a[1]
    length = math.hypot(dx, dy)
    ang = math.atan2(dy, dx)
    extra = {"lateral": r_lat / r_v} if abs(r_lat - r_v) > 1e-9 else {}
    return capsule(id, bone, material, role, tone, region, (0, 0, 0), (length, 0, 0), r_v, r_v, translate=[a[0], a[1], a[2] if len(a) > 2 else 0], rotate=qz(ang), **extra, **kw)


def slab(id, bone, material, role, tone, region, a, b, thick, width, corner=0.0, **kw):
    """Box slab of length |ab|, thickness `thick` normal to ab in the u-v plane, `width` along w, centred on the midpoint of a-b."""
    dx, dy = b[0] - a[0], b[1] - a[1]
    length = math.hypot(dx, dy)
    ang = math.atan2(dy, dx)
    mid = [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2, (a[2] + b[2]) / 2 if len(a) > 2 else 0]
    extra = {"cornerRadius": r(corner)} if corner else {}
    return solid(id, "box", bone, material, role, tone, region, size=[r(length), r(thick), r(width)], translate=mid, rotate=qz(ang), **extra, **kw)


VERT = [r(c) for c in qbasis([0, 0, 1], [0, -1, 0], [1, 0, 0])]
