import json, os
T = os.path.dirname(os.path.abspath(__file__))
def r(x): return round(x, 5)
def v3(a): return [r(x) for x in a]
def solid(id, kind, bone, material, role, tone, region=None, regions=None, maxD=None, transform=None, repeat=None, **geo):
    s = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    if region: s["region"] = region
    if regions: s["regions"] = regions
    if maxD: s["maxDenominator"] = maxD
    for k, v in geo.items():
        if k in ("size", "radii", "p0", "p1", "axis", "origin"): v = v3(v)
        s[k] = v
    if transform: s["transform"] = {k: ([round(x, 7) for x in v] if k == "rotation" else v3(v)) for k, v in transform.items()}
    if repeat: s["repeat"] = repeat
    return s
def box(id, bone, material, role, tone, size, at, region=None, regions=None, maxD=None, corner=None, rot=None, repeat=None):
    tr = {"translation": at}
    if rot: tr["rotation"] = rot
    g = {"size": size}
    if corner: g["cornerRadius"] = corner
    return solid(id, "box", bone, material, role, tone, region, regions, maxD, tr, repeat, **g)
def cap(id, bone, material, role, tone, p0, p1, r0, r1=None, region=None, regions=None, maxD=None, ls=None, repeat=None, transform=None):
    g = {"p0": p0, "p1": p1, "r0": r0, "r1": r0 if r1 is None else r1}
    if ls is not None: g["lateralScale"] = ls
    return solid(id, "capsule", bone, material, role, tone, region, regions, maxD, transform, repeat, **g)
def ell(id, bone, material, role, tone, radii, at, region=None, regions=None, maxD=None, repeat=None):
    return solid(id, "ellipsoid", bone, material, role, tone, region, regions, maxD, {"translation": at}, repeat, radii=radii)
def ext(id, bone, material, role, tone, profile, plane, f, t, region=None, regions=None, maxD=None, holes=None, repeat=None, transform=None, taper=None):
    g = {"profile": [[r(a), r(b)] for a, b in profile], "plane": plane, "from": r(f), "to": r(t)}
    if holes: g["holes"] = [[[r(a), r(b)] for a, b in h] for h in holes]
    if taper: g["taperTo"] = taper
    return solid(id, "extrude", bone, material, role, tone, region, regions, maxD, transform, repeat, **g)
def rev(id, bone, material, role, tone, profile, axis, origin, region=None, regions=None, maxD=None, ls=None, sweep=None, repeat=None, transform=None):
    g = {"profile": [[r(a), r(b)] for a, b in profile], "axis": axis, "origin": origin}
    if ls is not None: g["lateralScale"] = ls
    if sweep is not None: g["sweepRad"] = sweep
    return solid(id, "revolve", bone, material, role, tone, region, regions, maxD, transform, repeat, **g)
def loft(id, bone, material, role, tone, sections, region=None, regions=None, maxD=None, repeat=None, transform=None, cs=True, ce=True):
    secs = []
    for s in sections:
        d = {"at": v3(s[0]), "rx": r(s[1]), "ry": r(s[2])}
        if len(s) > 3 and s[3] is not None: d["exponent"] = s[3]
        secs.append(d)
    return solid(id, "loft", bone, material, role, tone, region, regions, maxD, transform, repeat, sections=secs, capStart=cs, capEnd=ce)
def write(slug, solids, decisions):
    ids = [s["id"] for s in solids]
    assert len(ids) == len(set(ids)), [i for i in ids if ids.count(i) > 1]
    out = {"volume": {"status": "authored", "solids": solids, "decisions": [{"en": a, "de": b} for a, b in decisions]}}
    p = os.path.join(T, "volumes", "components", slug + ".json")
    with open(p, "w") as f: json.dump(out, f, ensure_ascii=False, indent=1)
    print(slug, len(solids), "solids")
def cyl(id, bone, material, role, tone, base, axis, length, radius, region=None, regions=None, maxD=None, repeat=None):
    return rev(id, bone, material, role, tone, [(0, radius), (length, radius)], axis, base, region, regions, maxD, repeat=repeat)
