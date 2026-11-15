"""🧍️ Author 09 helpers: build, coarsen and check per-scale volumes of human staffage assets."""
import copy
import json
import math
import os
import subprocess

T = os.path.dirname(os.path.abspath(__file__))
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
SCALES = (50, 100, 200, 500)
FEATURE = {s: 0.15 * s / 1000 for s in SCALES}


def bun():
    base = os.path.expanduser("~/Library/Caches/semio/runtimes/bun")
    return sorted(
        os.path.join(base, d, "bun") for d in os.listdir(base) if os.path.exists(os.path.join(base, d, "bun"))
    )[0]


def r5(x):
    return round(x + 0.0, 5)


def rnd(v):
    if isinstance(v, float):
        return r5(v)
    if isinstance(v, list):
        return [rnd(x) for x in v]
    if isinstance(v, dict):
        return {k: rnd(x) for k, x in v.items()}
    return v


def asset(slug):
    return json.load(open(f"{CAT}/{slug}/🔣️.json"))["asset"]


def seed(slug):
    return json.load(open(f"{T}/author-09-seed/{slug}.json"))["volumes"]


def save(slug, volumes):
    path = f"{T}/volumes/humans/{slug}.json"
    json.dump({"volumes": rnd(volumes)}, open(path, "w"), ensure_ascii=False, separators=(",", ":"))


def run(args, check=False):
    import time
    for attempt in range(40):
        p = subprocess.run([bun()] + args, cwd=T, capture_output=True, text=True)
        out = p.stdout + p.stderr
        if "SyntaxError" not in out[:200] and "Export named" not in out[:300]:
            return out
        time.sleep(15)
    return out


def measurements(slug, scale=None, rules=None):
    args = ["volumes-check.ts", f"humans/{slug}", "--json"]
    if scale:
        args += ["--scale", str(scale)]
    if rules:
        args += ["--rules", rules]
    out = run(args)
    i = out.index("{\n")
    return json.JSONDecoder().raw_decode(out[i:])[0]["measurements"]


def areas(slug, scale, lst="thermal"):
    out = run(["author-09-area.ts", f"humans/{slug}", str(scale), lst])
    rows = {}
    reg = {}
    for line in out.splitlines():
        if line.startswith("REGION "):
            _, k, v = line.split()
            reg[k] = float(v)
        elif " area=" in line:
            rows[line.split()[0]] = line
    return rows, reg


def by_id(solids):
    return {s["id"]: s for s in solids}


def sec(u, v, w, rx, ry, e=2.3, rot=None):
    s = {"at": [u, v, w], "rx": rx, "ry": ry, "exponent": e}
    if rot is not None:
        s["rotation"] = rot
    return s


def loft(id, bone, material, tone, sections, role="body", **extra):
    return {"id": id, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": sections, "capStart": True, "capEnd": True, **extra}


def ellipsoid(id, bone, material, tone, radii, at, role="body", rotation=None, **extra):
    t = {"translation": list(at)}
    if rotation:
        t["rotation"] = rotation
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone, "radii": list(radii), "transform": t, **extra}


def capsule(id, bone, material, tone, p0, p1, r0, r1=None, role="body", **extra):
    return {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": list(p0), "p1": list(p1), "r0": r0, "r1": r0 if r1 is None else r1, **extra}


def box(id, bone, material, tone, size, at, role="body", rotation=None, corner=None, **extra):
    t = {"translation": list(at)}
    if rotation:
        t["rotation"] = rotation
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": list(size), "transform": t, **extra}
    if corner:
        s["cornerRadius"] = corner
    return s


def revolve(id, bone, material, tone, profile, axis, origin, role="body", **extra):
    return {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": profile, "axis": list(axis), "origin": list(origin), **extra}


def seclen(secs):
    out = [0.0]
    for a, b in zip(secs, secs[1:]):
        out.append(out[-1] + math.dist(a["at"], b["at"]))
    return out


def _interp(secs, s_list, s):
    for i in range(len(secs) - 1):
        if s_list[i] <= s <= s_list[i + 1] or i == len(secs) - 2:
            span = s_list[i + 1] - s_list[i]
            t = 0 if span == 0 else min(1, max(0, (s - s_list[i]) / span))
            a, b = secs[i], secs[i + 1]
            return [a["at"][k] + (b["at"][k] - a["at"][k]) * t for k in range(3)] + [a["rx"] + (b["rx"] - a["rx"]) * t, a["ry"] + (b["ry"] - a["ry"]) * t]


def dp_sections(secs, tol, fmin):
    """Greedy section selection: keep the section whose omission would err most until every error is under tol."""
    n = len(secs)
    s_list = seclen(secs)
    keep = {0, n - 1}
    while True:
        worst, wi = 0.0, None
        ks = sorted(keep)
        sub = [secs[k] for k in ks]
        sl = [s_list[k] for k in ks]
        for i in range(n):
            if i in keep:
                continue
            p = _interp(sub, sl, s_list[i])
            q = secs[i]
            err = max(abs(p[0] - q["at"][0]), abs(p[1] - q["at"][1]), abs(p[2] - q["at"][2]), abs(p[3] - q["rx"]), abs(p[4] - q["ry"]))
            if err > worst:
                worst, wi = err, i
        if wi is None or worst <= tol:
            break
        keep.add(wi)
    out = []
    for k in sorted(keep):
        s = copy.deepcopy(secs[k])
        s["rx"] = max(s["rx"], fmin)
        s["ry"] = max(s["ry"], fmin)
        out.append(s)
    return out


def coarsen_loft(solid, scale, tol=None, fmin=None, e=None):
    f = FEATURE[scale]
    tol = 0.5 * f if tol is None else tol
    fmin = 0.55 * f if fmin is None else fmin
    s = copy.deepcopy(solid)
    s["sections"] = dp_sections(s["sections"], tol, fmin)
    if e is not None:
        for q in s["sections"]:
            q["exponent"] = e
    return s


def min_radius(solid, fmin):
    s = solid
    if s["kind"] == "ellipsoid":
        s["radii"] = [max(r, fmin) for r in s["radii"]]
    elif s["kind"] == "capsule":
        s["r0"] = max(s["r0"], fmin)
        s["r1"] = max(s["r1"], fmin)
    elif s["kind"] == "loft":
        for q in s["sections"]:
            q["rx"] = max(q["rx"], fmin)
            q["ry"] = max(q["ry"], fmin)
    return s


def shrink(solid, t, new_id, tags=None):
    """The same solid inset by t (thermal layer shell): lofts, ellipsoids, capsules, boxes, revolves and extrudes."""
    s = copy.deepcopy(solid)
    s["id"] = new_id
    k = s["kind"]
    if k == "loft":
        for q in s["sections"]:
            q["rx"] = max(q["rx"] - t, 0.002)
            q["ry"] = max(q["ry"] - t, 0.002)
    elif k == "ellipsoid":
        s["radii"] = [max(r - t, 0.002) for r in s["radii"]]
    elif k == "capsule":
        s["r0"] = max(s["r0"] - t, 0.002)
        s["r1"] = max(s["r1"] - t, 0.002)
    elif k == "box":
        s["size"] = [max(x - 2 * t, 0.004) for x in s["size"]]
    elif k == "revolve":
        s["profile"] = [[a, max(r - t, 0.0)] for a, r in s["profile"]]
    elif k == "extrude":
        pr = s["profile"]
        cx = sum(p[0] for p in pr) / len(pr)
        cy = sum(p[1] for p in pr) / len(pr)
        ext = max(max(abs(p[0] - cx) for p in pr), max(abs(p[1] - cy) for p in pr), 1e-3)
        f = max(0.2, 1 - t / ext)
        s["profile"] = [[cx + (p[0] - cx) * f, cy + (p[1] - cy) * f] for p in pr]
        a, b = s["from"], s["to"]
        m, h = (a + b) / 2, abs(b - a) / 2
        h = max(h - t, 0.002)
        s["from"], s["to"] = m - h, m + h
    s.pop("region", None)
    if tags:
        s["tags"] = tags
    return s


def scaled(solid, k, new_id):
    """A copy of the solid with its cross-section scaled by k about its axis (thermal patch of the right surface)."""
    s = copy.deepcopy(solid)
    s["id"] = new_id
    kind = s["kind"]
    if kind == "loft":
        for q in s["sections"]:
            q["rx"] *= k
            q["ry"] *= k
    elif kind == "ellipsoid":
        s["radii"] = [r * k for r in s["radii"]]
    elif kind == "capsule":
        s["r0"] *= k
        s["r1"] *= k
    elif kind == "box":
        s["size"] = [x * k for x in s["size"]]
        if "cornerRadius" in s:
            s["cornerRadius"] = min(s["cornerRadius"], 0.45 * min(s["size"]))
    elif kind == "revolve":
        s["profile"] = [[a, r * k] for a, r in s["profile"]]
    return s


def ref(id, of, region, layer=None):
    d = {"id": id, "of": of, "region": region}
    if layer:
        d["layer"] = layer
    return d


def thermal_from(asset_, body, spec, scale, factors=None):
    """spec: region -> [body solid id]; layered regions at 1:50 become nested shells (innermost layer first, each inset by the layers outside it)."""
    factors = factors or {}
    regions = {r["id"]: r for r in asset_["thermal"]["regions"]}
    bodies = by_id(body)
    out = []
    for rid, ids in spec.items():
        r = regions[rid]
        layers = r.get("layers", [])
        k = factors.get(rid, 1.0)
        for sid in ids:
            fixed = sid.startswith("=")
            sid = sid.lstrip("=")
            b = bodies[sid]
            k = 1.0 if fixed else factors.get(rid, 1.0)
            top = layers[-1]["id"] if len(layers) > 1 and scale == 50 else None
            if k == 1.0:
                out.append(ref(f"th-{rid}-{sid}", sid, rid, top))
            else:
                c = scaled(b, k, f"th-{rid}-{sid}")
                c["region"] = rid
                if top:
                    c["layer"] = top
                c["role"] = "body"
                min_radius(c, 0.55 * FEATURE[scale])
                if c["kind"] == "box":
                    c["size"] = [max(x, 1.05 * FEATURE[scale]) for x in c["size"]]
                    if "cornerRadius" in c:
                        c["cornerRadius"] = min(c["cornerRadius"], 0.45 * min(c["size"]))
                out.append(c)
            if scale == 50 and len(layers) > 1:
                cum = 0.0
                for layer in reversed(layers[:-1]):
                    cum += next(l for l in layers if l["id"] == layers[layers.index(layer) + 1]["id"]).get("thicknessM") or 0.002
                    c = shrink(b if k == 1.0 else scaled(b, k, "x"), cum, f"th-{rid}-{layer['id']}-{sid}")
                    c["region"] = rid
                    c["layer"] = layer["id"]
                    c["role"] = "body"
                    out.append(c)
    return out


def member_bones(asset_):
    h = asset_["dimensions"]["heightM"]
    return [b["id"] for b in asset_["rig"]["bones"] if b.get("section") or (b.get("lengthM") or 0) >= 0.05 * h]


def sec_at_u(secs, u):
    """Interpolated (v, w, rx, ry, e) of a loft at bone-local u (sections ordered along the path)."""
    pts = sorted(secs, key=lambda q: q["at"][0])
    if u <= pts[0]["at"][0]:
        q = pts[0]
        return q["at"][1], q["at"][2], q["rx"], q["ry"], q.get("exponent", 2)
    if u >= pts[-1]["at"][0]:
        q = pts[-1]
        return q["at"][1], q["at"][2], q["rx"], q["ry"], q.get("exponent", 2)
    for a, b in zip(pts, pts[1:]):
        if a["at"][0] <= u <= b["at"][0]:
            t = 0 if b["at"][0] == a["at"][0] else (u - a["at"][0]) / (b["at"][0] - a["at"][0])
            lerp = lambda x, y: x + (y - x) * t
            return lerp(a["at"][1], b["at"][1]), lerp(a["at"][2], b["at"][2]), lerp(a["rx"], b["rx"]), lerp(a["ry"], b["ry"]), a.get("exponent", 2)


def primary(body, bone):
    cands = [s for s in body if s["bone"] == bone and s.get("role", "body") == "body"]
    lofts = [s for s in cands if s["kind"] == "loft"]
    if lofts:
        return max(lofts, key=lambda s: abs(s["sections"][-1]["at"][0] - s["sections"][0]["at"][0]) + 0.1 * max(q["rx"] + q["ry"] for q in s["sections"]))
    return cands[0] if cands else None


CORE = {"torso": 0.72, "pelvis": 0.72, "head": 0.8, "hand": 0.6, "foot": 0.6, "toe": 0.6}


def core_k(bone, table):
    for key, k in table.items():
        if bone.startswith(key):
            return k
    return table.get("*", 0.6)


ROT_U = [0.7071, 0, 0.7071, 0]


def member_loft(asset_, body, bone, scale, k=None, table=None, n=None):
    b = next(x for x in asset_["rig"]["bones"] if x["id"] == bone)
    L = b["lengthM"]
    p = primary(body, bone)
    f = FEATURE[scale]
    k = k if k is not None else core_k(bone, table or CORE)
    if p is None:
        par = next(x for x in asset_["rig"]["bones"] if x["id"] == b.get("parent"))
        pp = primary(body, par["id"])
        w = sec_at_u(pp["sections"], par["lengthM"])[1] if pp and pp["kind"] == "loft" else 0.0
        r = max(0.012, 0.55 * f)
        return {**capsule(f"member-{bone}", bone, pp["material"] if pp else "skin", 0.5, [0, 0, w], [L, 0, w], r), "member": bone, "part": "member"}
    if p["kind"] == "ellipsoid":
        c = scaled(p, k, f"member-{bone}")
        c["radii"] = [max(x, 0.55 * f) for x in c["radii"]]
        c["role"] = "body"
        c["member"], c["part"] = bone, "member"
        return c
    if p["kind"] != "loft":
        return None
    secs = p["sections"]
    rot = secs[0].get("rotation", ROT_U)
    n = n or (4 if scale == 50 else 3 if scale == 100 else 2)
    out = []
    for i in range(n):
        u = L * i / (n - 1)
        v, w, rx, ry, e = sec_at_u(secs, u)
        out.append(sec(u, v, w, max(rx * k, 0.55 * f), max(ry * k, 0.55 * f), min(e, 3.0), rot))
    return loft(f"member-{bone}", bone, p["material"], 0.5, out, member=bone, part="member")


def joint_solid(asset_, body, bone, scale, k=0.85):
    b = next(x for x in asset_["rig"]["bones"] if x["id"] == bone)
    p = primary(body, bone)
    f = FEATURE[scale]
    if p is None or p["kind"] != "loft":
        return None
    v, w, rx, ry, e = sec_at_u(p["sections"], 0.0)
    par = b.get("parent")
    pp = primary(body, par) if par else None
    r = max(min(max(rx, ry) * k, 0.045 if scale <= 100 else 0.09), 0.55 * f)
    if pp is not None and pp["kind"] == "loft":
        Lp = next(x for x in asset_["rig"]["bones"] if x["id"] == par)["lengthM"]
        _, _, prx, pry, _ = sec_at_u(pp["sections"], Lp)
        r = max(min(r, max(prx, pry) * k), 0.55 * f)
    return ellipsoid(f"joint-{bone}", bone, p["material"], 0.5, [r, r, r], [0, v, w], member=bone, part="joint")


def mass_solid(asset_, body, bone, scale):
    b = next(x for x in asset_["rig"]["bones"] if x["id"] == bone)
    m = b["mass"]
    p = primary(body, bone)
    f = FEATURE[scale]
    cx, cy = m.get("comX", 0), m.get("comY", 0)
    if p is not None and p["kind"] == "loft":
        v, w, rx, ry, _ = sec_at_u(p["sections"], cx)
        cy = v if abs(cy) < 1e-9 else cy
        ru, rv, rw = min(0.45 * b["lengthM"], 0.07), max(0.6 * ry, 0.55 * f), max(0.6 * rx, 0.55 * f)
        return ellipsoid(f"mass-{bone}", bone, p["material"], 0.5, [max(ru, 0.55 * f), rv, rw], [cx, cy, w], member=bone, part="mass")
    return None


def structure_from(asset_, body, scale, joints=None, masses=True, table=None, skip=(), extra=()):
    out = []
    mb = [b for b in member_bones(asset_) if b not in skip]
    total = sum((b.get("mass") or {}).get("kg", 0) for b in asset_["rig"]["bones"])
    for bone in mb:
        s = member_loft(asset_, body, bone, scale, table=table)
        if s:
            out.append(s)
    jb = joints if joints is not None else mb
    for bone in jb:
        b = next(x for x in asset_["rig"]["bones"] if x["id"] == bone)
        if b.get("parent") and bone in mb:
            j = joint_solid(asset_, body, bone, scale)
            if j:
                out.append(j)
    if masses:
        for b in asset_["rig"]["bones"]:
            kg = (b.get("mass") or {}).get("kg", 0)
            if total and kg / total >= 0.10:
                m = mass_solid(asset_, body, b["id"], scale)
                if m:
                    out.append(m)
    out.extend(copy.deepcopy(list(extra)))
    return out


def struct_ref(id, of, member, part="member"):
    return {"id": id, "of": of, "member": member, "part": part}


def emitter(asset_, eid, region, scale, bone, at, direction, tone=0.9, material="skin", length=None):
    e = next(x for x in asset_["thermal"]["emitters"] if x["id"] == eid)
    f = FEATURE[scale]
    d = max(e["diameterM"], 1.25 * f)
    L = length or max(0.02, 2 * f)
    p1 = [at[0] + direction[0] * L, at[1] + direction[1] * L, at[2] + direction[2] * L]
    s = capsule(f"th-{eid}", bone, material, tone, at, p1, d / 2)
    s["region"] = region
    s["tags"] = ["emitter", eid]
    return s


def plume(asset_, eid, region, scale, bone, at, layer=None, material="skin"):
    e = next(x for x in asset_["thermal"]["emitters"] if x["id"] == eid)
    f = FEATURE[scale]
    r = e["diameterM"] / 2
    s = ellipsoid(f"th-{eid}", bone, material, 0.8, [0.0, 0.0, 0.0], at, role="body")
    s["radii"] = [max(0.6 * f, 0.004), r, r]
    s["region"] = region
    s["tags"] = ["emitter", eid]
    if layer:
        s["layer"] = layer
    return s
