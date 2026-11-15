import json, os, subprocess, sys, math
T = os.path.dirname(os.path.abspath(__file__))
BUN = subprocess.run("ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1", shell=True, capture_output=True, text=True).stdout.strip()

def _t(translation=None, rotation=None):
    t = {}
    if translation is not None and any(translation): t["translation"] = [round(x, 6) for x in translation]
    if rotation is not None: t["rotation"] = rotation
    return {"transform": t} if t else {}

def base(id, kind, bone, material, role, tone, **kw):
    d = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    d.update(kw)
    return d

def box(id, bone, mat, role, tone, size, at=(0, 0, 0), r=None, rot=None, **kw):
    d = base(id, "box", bone, mat, role, tone, size=[round(x, 6) for x in size])
    if r: d["cornerRadius"] = r
    d.update(_t(at, rot)); d.update(kw); return d

def ell(id, bone, mat, role, tone, radii, at=(0, 0, 0), rot=None, **kw):
    d = base(id, "ellipsoid", bone, mat, role, tone, radii=[round(x, 6) for x in radii])
    d.update(_t(at, rot)); d.update(kw); return d

def cap(id, bone, mat, role, tone, p0, p1, r0, r1=None, lat=None, **kw):
    d = base(id, "capsule", bone, mat, role, tone, p0=list(p0), p1=list(p1), r0=r0, r1=r0 if r1 is None else r1)
    if lat: d["lateralScale"] = lat
    d.update(kw); return d

def rev(id, bone, mat, role, tone, profile, axis=(0, 0, 1), origin=(0, 0, 0), lat=None, at=None, rot=None, **kw):
    d = base(id, "revolve", bone, mat, role, tone, profile=[[round(a, 6), round(b, 6)] for a, b in profile], axis=list(axis), origin=list(origin))
    if lat: d["lateralScale"] = lat
    d.update(_t(at, rot)); d.update(kw); return d

def ext(id, bone, mat, role, tone, profile, plane, a, b, holes=None, taper=None, at=None, rot=None, **kw):
    d = base(id, "extrude", bone, mat, role, tone, profile=[[round(p, 6), round(q, 6)] for p, q in profile], plane=plane)
    d["from"] = a; d["to"] = b
    if holes: d["holes"] = [[[round(p, 6), round(q, 6)] for p, q in h] for h in holes]
    if taper: d["taperTo"] = list(taper)
    d.update(_t(at, rot)); d.update(kw); return d

def loft(id, bone, mat, role, tone, sections, capS=True, capE=True, **kw):
    d = base(id, "loft", bone, mat, role, tone, sections=sections)
    d["capStart"] = capS; d["capEnd"] = capE
    d.update(kw); return d

def sec(at, rx, ry, n=2, rot=None):
    s = {"at": [round(x, 6) for x in at], "rx": rx, "ry": ry}
    if n != 2: s["exponent"] = n
    if rot is not None: s["rotation"] = rot
    return s

def rrect(w, h, r, n=4):
    pts = []
    for cx, cy, a0 in ((w/2-r, h/2-r, 0), (-w/2+r, h/2-r, 90), (-w/2+r, -h/2+r, 180), (w/2-r, -h/2+r, 270)):
        for k in range(n+1):
            a = math.radians(a0 + 90*k/n)
            pts.append((cx + r*math.cos(a), cy + r*math.sin(a)))
    return pts

def circle(r, n=24, cx=0, cy=0):
    return [(cx + r*math.cos(2*math.pi*k/n), cy + r*math.sin(2*math.pi*k/n)) for k in range(n)]

def th(id, of, region, layer=None):
    d = {"id": id, "of": of, "region": region}
    if layer: d["layer"] = layer
    return d

def tho(solid, region, layer=None):
    solid = dict(solid); solid["region"] = region
    if layer: solid["layer"] = layer
    return solid

def st(solid, member, part):
    solid = dict(solid); solid["member"] = member; solid["part"] = part
    return solid

def stof(id, of, member, part):
    return {"id": id, "of": of, "member": member, "part": part}

def scale_vol(den, body, thermal, structure, en, de):
    return {"status": "authored", "denominator": den, "body": body, "thermal": thermal, "structure": structure,
            "decisions": [{"en": e, "de": d} for e, d in zip(en, de)]}

def unavailable(den): return {"status": "unavailable", "denominator": den}

def _supports(asset):
    cat, slug = asset.split("/")
    base = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
    d = [x for x in os.listdir(base) if x.endswith(cat)][0]
    return json.load(open(f"{base}/{d}/{slug}/🔣️.json"))["asset"]["structure"]["supports"]

def write(asset, volumes):
    sup = _supports(asset)
    for v in volumes.values():
        for x in v.get("structure", []):
            if x.get("part") == "support":
                m = [a for a in sup if a in x["id"]]
                if m: x["anchor"] = max(m, key=len)
    p = os.path.join(T, "volumes", asset + ".json")
    cur = json.load(open(p))
    for k, v in volumes.items(): cur["volumes"][k] = v
    json.dump(cur, open(p, "w"), ensure_ascii=False, indent=1)

def run(cmd, asset, *a):
    out = subprocess.run([BUN, f"volumes-{cmd}.ts", asset, *a], cwd=T, capture_output=True, text=True)
    return out.stdout + out.stderr

def set_area(asset, region, new):
    cat, slug = asset.split("/")
    base = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
    d = [x for x in os.listdir(base) if x.endswith(cat)][0]
    path = f"{base}/{d}/{slug}/🔣️.json"
    t = open(path).read()
    i = t.index('"thermal": {')
    j = t.index(f'"id": "{region}",', t.index('"regions": [', i))
    k = t.index('"areaM2":', j)
    e = t.index(",", k)
    old = t[k + 10:e].strip()
    if float(old) == new: return
    open(path, "w").write(t[:k] + f'"areaM2": {new}' + t[e:])
    open(os.path.join(T, "author-13-areafix.log"), "a").write(f"{asset} {region} {old} -> {new}\n")

def fit_areas(asset, scale="50", skip=(), limit=0.15):
    import re
    out = run("check", asset, "--scale", scale, "--rules", "volumes-thermal-coverage")
    cat, slug = asset.split("/")
    base = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
    d = [x for x in os.listdir(base) if x.endswith(cat)][0]
    a = json.load(open(f"{base}/{d}/{slug}/🔣️.json"))["asset"]
    decl = {r["id"]: r["areaM2"] for r in a["thermal"]["regions"]}
    for m in re.finditer(r"!areaDeviation:([a-z0-9-]+)=([0-9.e-]+)<=", out):
        rid, dev = m.group(1), float(m.group(2))
        if rid in skip: continue
        for cand in (round(decl[rid] * (1 + dev), 5), round(decl[rid] * (1 - dev), 5)):
            if cand <= 0: continue
            set_area(asset, rid, cand)
            o2 = run("check", asset, "--scale", scale, "--rules", "volumes-thermal-coverage")
            if f"!areaDeviation:{rid}=" not in o2: break
            set_area(asset, rid, decl[rid])
