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


ROT = {"u": [0.5, 0.5, 0.5, 0.5], "v": [-0.5, -0.5, -0.5, 0.5], "w": None}


def loft(id, bone, material, role, tone, region, sections, along="u", cap=(True, True), **kw):
    """Sections are (at, a, b[, exponent]); the path runs along the bone axis `along`; with the kernel's rotation-defined section plane the half sizes a and b lie on the next two axes cyclically: u -> (v, w), v -> (w, u), w -> (u, v)."""
    secs = []
    for s in sections:
        item = {"at": vec(s[0]), "rx": r(s[1]), "ry": r(s[2])}
        if len(s) > 3 and s[3] is not None:
            item["exponent"] = s[3]
        if ROT[along] is not None:
            item["rotation"] = ROT[along]
        secs.append(item)
    return solid(id, "loft", bone, material, role, tone, region, sections=secs, capStart=cap[0], capEnd=cap[1], **kw)


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


def bezier(p0, p1, p2, p3, n=6, first=False):
    out = []
    for k in range(0 if first else 1, n + 1):
        t = k / n
        a, b, c, d = (1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t ** 2, t ** 3
        out.append((a * p0[0] + b * p1[0] + c * p2[0] + d * p3[0], a * p0[1] + b * p1[1] + c * p2[1] + d * p3[1]))
    return out


def arc(cx, cy, rad, a0, a1, n=6):
    return [(cx + rad * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + rad * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]
