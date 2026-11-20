"""🧰 Small solid constructors for the handcrafted courier model (`r3-courier-build.py`): every section, point and radius is stated by the caller; this file only writes the JSON of the six solid kinds, mirrors a left part to the right and shrinks a garment loft to its inner layer shells."""
import copy
import math

STD = [0, -0.70711, 0, 0.70711]
FOOT_A = math.radians(23.807)


def quat(ex, ey):
    """Unit quaternion [x, y, z, w] of the rotation that carries X to ex and Y to ey (orthonormal)."""
    ez = [ex[1] * ey[2] - ex[2] * ey[1], ex[2] * ey[0] - ex[0] * ey[2], ex[0] * ey[1] - ex[1] * ey[0]]
    m = [[ex[0], ey[0], ez[0]], [ex[1], ey[1], ez[1]], [ex[2], ey[2], ez[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [round(v, 5) for v in q]


FOOT_Q = quat([0, 0, 1], [-math.sin(FOOT_A), math.cos(FOOT_A), 0])


def r5(v):
    return round(v, 5)


def S(u, cv, cw, rw, rv, e=2.3):
    """Section across the bone at u: centre (cv sagittal, cw lateral), half width rw (lateral), half depth rv (sagittal)."""
    return {"at": [r5(u), r5(cv), r5(cw)], "rx": r5(rw), "ry": r5(rv), "exponent": e, "rotation": STD}


def F(dx, y, hy, cw, rw, e=2.6):
    """Vertical cross-section of the boot in the foot bone: dx ahead of the ankle, centre height y above the ground, half height hy, half width rw."""
    dy = y - 0.0803
    return {"at": [r5(dx * math.cos(FOOT_A) - dy * math.sin(FOOT_A)), r5(dx * math.sin(FOOT_A) + dy * math.cos(FOOT_A)), r5(cw)], "rx": r5(rw), "ry": r5(hy), "exponent": e, "rotation": FOOT_Q}


def base(id, bone, mat, tone, kind, role="body"):
    return {"id": id, "kind": kind, "bone": bone, "material": mat, "role": role, "tone": tone}


def loft(id, bone, mat, tone, sections, role="body", **kw):
    return {**base(id, bone, mat, tone, "loft", role), "sections": sections, "capStart": True, "capEnd": True, **kw}


def cap(id, bone, mat, tone, p0, p1, r0, r1=None, role="body", **kw):
    return {**base(id, bone, mat, tone, "capsule", role), "p0": [r5(v) for v in p0], "p1": [r5(v) for v in p1], "r0": r0, "r1": r0 if r1 is None else r1, **kw}


def ell(id, bone, mat, tone, at, radii, role="body", rotation=None, **kw):
    t = {"translation": [r5(v) for v in at]}
    if rotation is not None:
        t["rotation"] = rotation
    return {**base(id, bone, mat, tone, "ellipsoid", role), "radii": radii, "transform": t, **kw}


def box(id, bone, mat, tone, at, size, radius=0.0, role="body", **kw):
    s = {**base(id, bone, mat, tone, "box", role), "size": size, "transform": {"translation": [r5(v) for v in at]}, **kw}
    if radius:
        s["cornerRadius"] = radius
    return s


def rev(id, bone, mat, tone, profile, axis, origin, role="body", **kw):
    return {**base(id, bone, mat, tone, "revolve", role), "profile": profile, "axis": axis, "origin": origin, **kw}


def ext(id, bone, mat, tone, profile, plane, lo, hi, role="body", **kw):
    return {**base(id, bone, mat, tone, "extrude", role), "profile": profile, "plane": plane, "from": lo, "to": hi, **kw}


def side(solid, s):
    """The solid of the left side renamed and mirrored across the sagittal plane for side s ('l' keeps it, 'r' mirrors): ids and bones ending in -l change their suffix."""
    out = copy.deepcopy(solid)
    for key in ("id", "bone", "of", "member"):
        if key in out and out[key].endswith("-l") and s == "r":
            out[key] = out[key][:-2] + "-r"
    if s == "l":
        return out
    for sec in out.get("sections", []):
        sec["at"][2] = -sec["at"][2]
        q = sec.get("rotation")
        if q is not None and q != STD:
            sec["rotation"] = [-q[0], -q[1], q[2], q[3]]
    for key in ("p0", "p1"):
        if key in out:
            out[key][2] = -out[key][2]
    t = out.get("transform")
    if t is not None:
        if "translation" in t:
            t["translation"][2] = -t["translation"][2]
        if "rotation" in t:
            q = t["rotation"]
            t["rotation"] = [-q[0], -q[1], q[2], q[3]]
    if "repeat" in out and "translation" in out["repeat"]:
        out["repeat"]["translation"][2] = -out["repeat"]["translation"][2]
    if out["kind"] == "extrude" if "kind" in out else False:
        out["from"], out["to"] = -out["to"], -out["from"]
    return out


def both(solids):
    return [side(x, s) for s in ("l", "r") for x in solids]


def shell(solid, d, id, region, layer):
    """The inner layer shell of a garment loft: the same stated sections, every half extent reduced by the thickness d of the layers outside it."""
    out = copy.deepcopy(solid)
    out["id"] = id
    for sec in out["sections"]:
        sec["rx"] = r5(max(sec["rx"] - d, 0.004))
        sec["ry"] = r5(max(sec["ry"] - d, 0.004))
    out["region"] = region
    out["layer"] = layer
    return out


def of(solid, region, layer=None, **kw):
    out = {"id": "th-" + solid["id"], "of": solid["id"], "region": region}
    if layer is not None:
        out["layer"] = layer
    out.update(kw)
    return out


def mem(solid, member, part="member"):
    return {"id": "st-" + solid["id"], "of": solid["id"], "member": member, "part": part}


def st(solid, member, part="member"):
    return {**solid, "member": member, "part": part}
