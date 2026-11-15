"""Builders shared by the author-32 generators (city-car, delivery-van, city-bus): plain solids, shells of body solids, thermal references, structure members."""
import copy, math, json
from author_32_lib import *

def rr(x):
    return round(x, 5)

def _c(id, bone, material, role, tone):
    return {"id": id, "bone": bone, "material": material, "role": role, "tone": tone}

def box(id, bone, mat, role, tone, size, at, corner=0.0, rot=None):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "box", "size": [rr(v) for v in size], "transform": {"translation": [rr(v) for v in at]}})
    if rot: s["transform"]["rotation"] = rot
    if corner: s["cornerRadius"] = corner
    return s

def cap(id, bone, mat, role, tone, p0, p1, r0, r1=None, lateral=None, slender=False):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "capsule", "p0": [rr(v) for v in p0], "p1": [rr(v) for v in p1], "r0": rr(r0), "r1": rr(r0 if r1 is None else r1)})
    if lateral: s["lateralScale"] = lateral
    if slender: s["slender"] = True
    return s

def ell(id, bone, mat, role, tone, radii, at, rot=None):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "ellipsoid", "radii": [rr(v) for v in radii], "transform": {"translation": [rr(v) for v in at]}})
    if rot: s["transform"]["rotation"] = rot
    return s

def rev(id, bone, mat, role, tone, profile, origin, axis=(0, 0, 1)):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "revolve", "profile": [[rr(a), rr(b)] for a, b in profile], "axis": list(axis), "origin": [rr(v) for v in origin]})
    return s

def loft(id, bone, mat, role, tone, sections, caps=True):
    s = _c(id, bone, mat, role, tone)
    s.update({"kind": "loft", "sections": sections, "capStart": caps, "capEnd": caps})
    return s

def sec(at, rx, ry, e=2, rot=None):
    o = {"at": [rr(v) for v in at], "rx": rr(rx), "ry": rr(ry), "exponent": e}
    if rot: o["rotation"] = rot
    return o

def ref(id, of, region, layer=None, faces=None, tags=None):
    s = {"id": id, "of": of, "region": region}
    if layer: s["layer"] = layer
    if faces: s["sectionFaces"] = faces
    if tags: s["tags"] = tags
    return s

def shell(solid, t, id, region, layer, floor=0.002):
    """the solid inset by t (loft sections, box size): a nested layer shell"""
    s = copy.deepcopy(solid)
    s["id"] = id; s["region"] = region; s["layer"] = layer
    for k in ("part", "member", "tags", "sectionFaces"): s.pop(k, None)
    if s["kind"] == "loft":
        for q in s["sections"]:
            q["rx"] = rr(max(floor, q["rx"] - t)); q["ry"] = rr(max(floor, q["ry"] - t))
    elif s["kind"] == "box":
        s["size"] = [rr(max(2 * floor, v - 2 * t)) for v in s["size"]]
        if s.get("cornerRadius"):
            s["cornerRadius"] = rr(min(max(s["cornerRadius"] - t, 0), min(s["size"]) / 2 - 1e-4))
            if s["cornerRadius"] <= 0: del s["cornerRadius"]
    elif s["kind"] == "capsule":
        s["r0"] = rr(max(floor, s["r0"] - t)); s["r1"] = rr(max(floor, s["r1"] - t))
    else:
        raise ValueError(s["kind"])
    s["tone"] = max(0.0, s.get("tone", 0.5) - 0.1)
    return s

def th(solid, region, layer=None, tags=None):
    s = copy.deepcopy(solid)
    s["region"] = region
    if layer: s["layer"] = layer
    if tags: s["tags"] = tags
    return s

def st(solid, member, part, anchor=None):
    s = copy.deepcopy(solid)
    s["member"] = member; s["part"] = part
    if anchor: s["anchor"] = anchor
    return s

def body_of(v, id):
    return next(s for s in v["body"] if s["id"] == id)

def probe(asset, scale, d, thermal=None, structure=None):
    dd = copy.deepcopy(d)
    if thermal is not None: dd["volumes"][str(scale)]["thermal"] = thermal
    if structure is not None: dd["volumes"][str(scale)]["structure"] = structure
    return dd

def show(asset, scale, d, rules="volumes-thermal-coverage,volumes-structure-coverage"):
    p = scratch(asset, d, 'show')
    out = run(['author-32-probe.ts', asset, '--scale', str(scale), '--source', p, '--owned', '--rules', rules])
    return out
