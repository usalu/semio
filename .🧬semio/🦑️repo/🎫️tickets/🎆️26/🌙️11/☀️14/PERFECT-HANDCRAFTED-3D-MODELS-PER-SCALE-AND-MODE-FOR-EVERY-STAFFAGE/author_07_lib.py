"""Author 07 helpers: per-scale builders for the humans volumes (body from the scale's own drawings, thermal and structure derived, area fitting)."""
import copy
import glob
import json
import math
import os
import re
import subprocess

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.abspath(os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))
GEN = f"{T}/🗑️generated/author-07"
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
H = 0.7071067811865476
ROT = {"u": [0, -H, 0, H], "v": [-0.5, -0.5, -0.5, 0.5], "w": None}


def r5(x):
    if isinstance(x, float):
        v = round(x, 5)
        return 0.0 if v == 0 else v
    if isinstance(x, list):
        return [r5(i) for i in x]
    if isinstance(x, dict):
        return {k: (v if k == "rotation" else r5(v)) for k, v in x.items()}
    return x


def d(en, de):
    return {"en": en, "de": de}


def stem(i):
    i = re.sub(r"-(l|r)$", "", i)
    i = re.sub(r"-(l|r)-", "-", i)
    return re.sub(r"-?\d+$", "", i)


def rest_frames(data):
    out = {}
    for b in data["rig"]["bones"]:
        px, py, pa = (0.0, 0.0, 0.0) if b["parent"] is None else out[b["parent"]]
        x, y = b.get("x", 0.0), b.get("y", 0.0)
        out[b["id"]] = (px + math.cos(pa) * x - math.sin(pa) * y, py + math.sin(pa) * x + math.cos(pa) * y, pa + math.radians(b["angleDeg"]))
    return out


class Builder:
    """One scale of one asset: body solids with a region map, raw drafts of this scale, thermal and structure lists."""

    def __init__(self, slug, scale, category="humans"):
        self.slug, self.scale, self.category = slug, scale, category
        self.path = glob.glob(f"{CATALOG}/*{category}/{slug}/🔣️.json")[0]
        self.data = json.load(open(self.path))["asset"]
        self.bones = {b["id"]: b for b in self.data["rig"]["bones"]}
        self.materials = {m["id"] for m in self.data["materials"]}
        self.regions = {r["id"]: r for r in self.data["thermal"]["regions"]}
        self.feature = FEATURE[scale]
        raw = f"{GEN}/{slug}.s{scale}.raw.json"
        self.raw = {s["id"]: s for s in json.load(open(raw))["solids"]} if os.path.exists(raw) else {}
        self.body, self.reg, self.thermal, self.structure, self.decisions, self.kfit = [], {}, [], [], [], {}
        self.rep = self.data["representations"][str(scale)]
        self.frames = rest_frames(self.data)

    def L(self, bone):
        return self.bones[bone]["lengthM"]

    def add(self, s, region=None):
        s = r5(copy.deepcopy(s))
        assert s["bone"] in self.bones, s["id"]
        assert s["material"] in self.materials, (s["id"], s["material"])
        assert all(o["id"] != s["id"] for o in self.body), s["id"]
        s.pop("region", None)
        s.pop("regions", None)
        s.pop("maxDenominator", None)
        self.body.append(s)
        regs = region if isinstance(region, (list, tuple)) else ([region] if region else [])
        for g in regs:
            assert g in self.regions, (s["id"], g)
        self.reg[s["id"]] = list(regs)
        return s

    def raw_region(self, sid):
        s = self.raw[sid]
        return s.get("regions") or ([s["region"]] if s.get("region") else [])

    def keep(self, sid, region="raw", **over):
        s = copy.deepcopy(self.raw[sid])
        s.update(over)
        reg = self.raw_region(sid) if region == "raw" else region
        return self.add(s, reg)

    def get(self, sid):
        return next(s for s in self.body if s["id"] == sid)

    def has(self, sid):
        return any(s["id"] == sid for s in self.body)

    def body_out(self):
        return copy.deepcopy(self.body)

    def drop(self, sid):
        self.body = [s for s in self.body if s["id"] != sid]
        self.reg.pop(sid, None)


def shape_ring(s):
    k = s["kind"]
    if k in ("poly", "path"):
        pts = s.get("points")
        if pts is None:
            pts = [[float(a), float(b)] for a, b in re.findall(r"[ML]\s*(-?[\d.e-]+)\s+(-?[\d.e-]+)", s["d"])]
        return [tuple(p) for p in pts]
    if k == "ellipse":
        return [(s["cx"] + s["rx"] * math.cos(t * math.pi / 12), s["cy"] + s["ry"] * math.sin(t * math.pi / 12)) for t in range(24)]
    if k == "rect":
        return [(s["x"], s["y"]), (s["x"] + s["w"], s["y"]), (s["x"] + s["w"], s["y"] + s["h"]), (s["x"], s["y"] + s["h"])]
    raise ValueError(k)


def front_halfwidth(B, bone, shape_id, view="front"):
    shape = next((x for x in B.rep["geometry"][view] if x["id"] == shape_id), None)
    if shape is None:
        return None
    ox, oy, ang = B.frames[bone]
    if view == "front":
        ring = [(y, x) for x, y in shape_ring(shape)]
        o, k = oy, math.sin(ang)
    else:
        ring = [(x, y) for x, y in shape_ring(shape)]
        o, k = ox, math.cos(ang)
    ys = [p[0] for p in ring]

    def f(u):
        y = min(max(u * k + o, min(ys) + 1e-4), max(ys) - 1e-4)
        vs = []
        n = len(ring)
        for i in range(n):
            (a0, b0), (a1, b1) = ring[i], ring[(i + 1) % n]
            if (a0 <= y < a1) or (a1 <= y < a0):
                vs.append(b0 + (b1 - b0) * (y - a0) / (a1 - a0))
        return (max(vs) - min(vs)) / 2 if len(vs) >= 2 else 0.0

    return f


def span_at(ring, u):
    vs = []
    n = len(ring)
    for i in range(n):
        (u0, v0), (u1, v1) = ring[i], ring[(i + 1) % n]
        if (u0 <= u < u1) or (u1 <= u < u0):
            vs.append(v0 + (v1 - v0) * (u - u0) / (u1 - u0))
    return (min(vs), max(vs)) if len(vs) >= 2 else None


def pfn(points):
    pts = sorted(points)

    def f(u):
        if u <= pts[0][0]:
            return pts[0][1]
        for (a, x), (c, y) in zip(pts, pts[1:]):
            if u <= c:
                return x + (y - x) * (u - a) / (c - a)
        return pts[-1][1]

    return f


def meta(src, **over):
    m = {k: src[k] for k in ("material", "role", "tone", "bone") if k in src}
    m.update(over)
    return m


def loft_from(raw, lat=None, aspect=None, n=10, exp=2.4, minr=0.004, ends=0.0, id=None, rxfn=None, amax=2.4, **over):
    """Loft along u from the side outline of an xy extrude (ry and v centre from the outline, rx = ry * aspect or rxfn(u))."""
    ring = [tuple(p) for p in raw["profile"]]
    us = [p[0] for p in ring]
    u0, u1 = min(us), max(us)
    w0, w1 = lat if lat else (raw["from"], raw["to"])
    wc, half = (w0 + w1) / 2, (w1 - w0) / 2
    if u1 - u0 < 0.05:
        n, ends = 2, 1.0
    probes = [u0 + (u1 - u0) * (1 - math.cos(math.pi * k / n)) / 2 for k in range(n + 1)]
    spans = []
    for u in probes:
        uu = min(max(u, u0 + 1e-4), u1 - 1e-4)
        spans.append(span_at(ring, uu) or span_at(ring, uu + 1e-4) or span_at(ring, uu - 1e-4))
    rys = [(s[1] - s[0]) / 2 for s in spans]
    rymax = max(rys)
    a = aspect if aspect is not None else half / rymax
    secs = []
    for k, (u, s) in enumerate(zip(probes, spans)):
        ry = max(minr, rys[k])
        if k in (0, n) and ends:
            ry = max(ry, ends * rymax)
        rx = max(minr, ry * a)
        if rxfn is not None:
            rx = max(minr, min(rxfn(u), ry * amax))
        secs.append({"at": [u, (s[0] + s[1]) / 2, wc], "rx": rx, "ry": ry, "exponent": exp, "rotation": ROT["u"]})
    out = {"id": id or raw["id"], "kind": "loft", **meta(raw), "sections": secs, "capStart": True, "capEnd": True}
    out.update(over)
    return out


def surface_v(solid, u, w, front=True):
    secs = solid["sections"]
    for a, b in zip(secs, secs[1:]):
        if a["at"][0] <= u <= b["at"][0]:
            t = (u - a["at"][0]) / (b["at"][0] - a["at"][0])
            break
    else:
        a = b = secs[0] if u < secs[0]["at"][0] else secs[-1]
        t = 0
    lerp = lambda x, y: x + (y - x) * t
    vc, wc = lerp(a["at"][1], b["at"][1]), lerp(a["at"][2], b["at"][2])
    rx, ry, e = lerp(a["rx"], b["rx"]), lerp(a["ry"], b["ry"]), a.get("exponent", 2)
    q = min(1.0, abs(w - wc) / rx)
    dd = ry * (1 - q ** e) ** (1 / e)
    return vc - dd if front else vc + dd


def interp(solid, u):
    secs = solid["sections"]
    for a, b in zip(secs, secs[1:]):
        if a["at"][0] <= u <= b["at"][0]:
            t = (u - a["at"][0]) / (b["at"][0] - a["at"][0])
            break
    else:
        a = b = secs[0] if u < secs[0]["at"][0] else secs[-1]
        t = 0
    lr = lambda x, y: x + (y - x) * t
    return {"v": lr(a["at"][1], b["at"][1]), "w": lr(a["at"][2], b["at"][2]), "rx": lr(a["rx"], b["rx"]), "ry": lr(a["ry"], b["ry"]), "e": a.get("exponent", 2)}


def capsule(id, bone, p0, p1, r0, r1=None, ls=None, **m):
    s = {"id": id, "kind": "capsule", "bone": bone, "p0": list(p0), "p1": list(p1), "r0": r0, "r1": r0 if r1 is None else r1, **m}
    if ls is not None:
        s["lateralScale"] = ls
    return s


def ellipsoid(id, bone, radii, at, rot=None, **m):
    t = {"translation": list(at)}
    if rot:
        t["rotation"] = rot
    return {"id": id, "kind": "ellipsoid", "bone": bone, "radii": list(radii), "transform": t, **m}


def box(id, bone, size, at, corner=None, rot=None, **m):
    t = {"translation": list(at)}
    if rot:
        t["rotation"] = rot
    s = {"id": id, "kind": "box", "bone": bone, "size": list(size), "transform": t, **m}
    if corner:
        s["cornerRadius"] = corner
    return s


def revolve(id, bone, profile, axis=(1, 0, 0), origin=(0, 0, 0), ls=None, **m):
    s = {"id": id, "kind": "revolve", "bone": bone, "profile": profile, "axis": list(axis), "origin": list(origin), **m}
    if ls is not None:
        s["lateralScale"] = ls
    return s


def extrude(id, bone, profile, plane, a, b, **m):
    return {"id": id, "kind": "extrude", "bone": bone, "profile": profile, "plane": plane, "from": a, "to": b, **m}


def loft(id, bone, sections, **m):
    return {"id": id, "kind": "loft", "bone": bone, "sections": sections, "capStart": True, "capEnd": True, **m}


def sec(a, rx, ry, along="u", e=2.4, at=None):
    p = list(at) if at else [0.0, 0.0, 0.0]
    p["uvw".index(along)] = a
    s = {"at": p, "rx": rx, "ry": ry, "exponent": e}
    if ROT[along]:
        s["rotation"] = ROT[along]
    return s


def quat_z(deg):
    a = math.radians(deg) / 2
    return [0, 0, math.sin(a), math.cos(a)]


def quat_y(deg):
    a = math.radians(deg) / 2
    return [0, math.sin(a), 0, math.cos(a)]


def quat_x(deg):
    a = math.radians(deg) / 2
    return [math.sin(a), 0, 0, math.cos(a)]


def tube_chain(B, id, bone, pts, r, w=0.0, r1=None, **m):
    for i, (p, q) in enumerate(zip(pts, pts[1:])):
        B.add(capsule(f"{id}-{i + 1}" if len(pts) > 2 else id, bone, [p[0], p[1], w], [q[0], q[1], w], r, r1, **{k: v for k, v in m.items() if k != "region"}), m.get("region"))


def ring_wheel(B, id, bone, R, r, ls, n, material, tone, region, role="body", z=0.0):
    th = 2 * math.pi / n
    s = capsule(id, bone, [-R * math.sin(th / 2), R * math.cos(th / 2), z], [R * math.sin(th / 2), R * math.cos(th / 2), z], r, ls=ls, material=material, role=role, tone=tone)
    s["repeat"] = {"count": n, "rotationAxis": [0, 0, 1], "rotationRad": th, "origin": [0, 0, z]}
    B.add(s, region)


def band(B, id, base_id, u0, u1, grow, material, tone, region, role="detail"):
    base = B.get(base_id)
    secs = []
    for u in (u0, (u0 + u1) / 2, u1):
        i = interp(base, u)
        secs.append({"at": [u, i["v"], i["w"]], "rx": i["rx"] + grow, "ry": i["ry"] + grow, "exponent": i["e"], "rotation": ROT["u"]})
    B.add({"id": id, "kind": "loft", "bone": base["bone"], "material": material, "role": role, "tone": tone, "sections": secs, "capStart": True, "capEnd": True}, region)


def mirror_w(s, new_id, new_bone):
    s = copy.deepcopy(s)
    s["id"], s["bone"] = new_id, new_bone
    k = s["kind"]
    if k == "loft":
        for q in s["sections"]:
            q["at"][2] = -q["at"][2]
    elif k == "capsule":
        s["p0"][2] = -s["p0"][2]
        s["p1"][2] = -s["p1"][2]
    elif k in ("ellipsoid", "box"):
        s["transform"]["translation"][2] = -s["transform"]["translation"][2]
    elif k == "extrude":
        s["from"], s["to"] = -s["to"], -s["from"]
    else:
        raise ValueError(k)
    return s


def shift_w(s, dw):
    k = s["kind"]
    if k == "loft":
        for q in s["sections"]:
            q["at"][2] += dw
    elif k == "capsule":
        s["p0"][2] += dw
        s["p1"][2] += dw
    elif k == "extrude":
        s["from"] += dw
        s["to"] += dw
    else:
        t = s.setdefault("transform", {"translation": [0, 0, 0]})
        t.setdefault("translation", [0, 0, 0])
        t["translation"][2] += dw
    return s


def scale_solid(s, k, id=None):
    """The solid with its cross-section scaled by k about its own axis / centre (thermal skin fitting, nested shells)."""
    c = copy.deepcopy(s)
    kind = c["kind"]
    if kind == "loft":
        for q in c["sections"]:
            q["rx"] *= k
            q["ry"] *= k
    elif kind == "capsule":
        c["r0"] *= k
        c["r1"] *= k
    elif kind == "ellipsoid":
        c["radii"] = [x * k for x in c["radii"]]
    elif kind == "box":
        c["size"] = [x * k for x in c["size"]]
        if "cornerRadius" in c:
            c["cornerRadius"] *= k
    elif kind == "revolve":
        c["profile"] = [[a, rad * k] for a, rad in c["profile"]]
    elif kind == "extrude":
        pts = c["profile"]
        cx = sum(p[0] for p in pts) / len(pts)
        cy = sum(p[1] for p in pts) / len(pts)
        c["profile"] = [[cx + (p[0] - cx) * k, cy + (p[1] - cy) * k] for p in pts]
        m = (c["from"] + c["to"]) / 2
        h = (c["to"] - c["from"]) / 2 * k
        c["from"], c["to"] = m - h, m + h
    if id:
        c["id"] = id
    return r5(c)


def shrink(s, dist, id=None, floor=0.002):
    c = copy.deepcopy(s)
    kind = c["kind"]
    if kind == "loft":
        for q in c["sections"]:
            q["rx"] = max(floor, q["rx"] - dist)
            q["ry"] = max(floor, q["ry"] - dist)
    elif kind == "capsule":
        c["r0"] = max(floor, c["r0"] - dist)
        c["r1"] = max(floor, c["r1"] - dist)
    elif kind == "ellipsoid":
        c["radii"] = [max(floor, x - dist) for x in c["radii"]]
    elif kind == "box":
        c["size"] = [max(2 * floor, x - 2 * dist) for x in c["size"]]
        if "cornerRadius" in c:
            c["cornerRadius"] = max(0, c["cornerRadius"] - dist)
    elif kind == "revolve":
        c["profile"] = [[a, max(floor if rad > 0 else 0.0, rad - dist)] for a, rad in c["profile"]]
    else:
        return None
    if id:
        c["id"] = id
    return r5(c)


def min_extent_rough(s):
    k = s["kind"]
    if k == "loft":
        return 2 * min(min(q["rx"], q["ry"]) for q in s["sections"])
    if k == "capsule":
        return 2 * min(s["r0"], s["r1"])
    if k == "ellipsoid":
        return 2 * min(s["radii"])
    if k == "box":
        return min(s["size"])
    return 1.0


def thermal_from_body(B, kfit=None, exclude=None, extra=None, drop=()):
    """Thermal list from the region map of the body: reference when k = 1, else a skin copy scaled by k; at 1:50 multi-layer regions are nested shells (outer = skin, inner = shrunk)."""
    kfit = kfit or {}
    exclude = exclude or {}
    out = []
    for s in B.body:
        for region in B.reg.get(s["id"], []):
            if s["id"] in exclude.get(region, ()):
                continue
            r = B.regions[region]
            layers = r.get("layers", [])
            multi = len(layers) > 1 and B.scale == 50
            tid = f"th-{s['id']}" if len(B.reg[s["id"]]) == 1 else f"th-{region}-{s['id']}"
            k = kfit.get(region, 1.0)
            base = s if abs(k - 1) < 1e-3 else scale_solid(s, k, id=s["id"])
            ref = abs(k - 1) < 1e-3

            def emit(sid, solid, layer):
                if sid in drop:
                    return
                if ref and solid is s:
                    e = {"id": sid, "of": s["id"], "region": region}
                else:
                    e = {k2: v for k2, v in solid.items() if k2 != "id"}
                    e = {"id": sid, **e, "region": region}
                if layer:
                    e["layer"] = layer
                out.append(e)

            if not multi:
                emit(tid, base, layers[-1]["id"] if len(layers) == 1 else None)
                continue
            thick = [l.get("thicknessM") or 0.0 for l in layers]
            for j, l in enumerate(layers):
                outside = sum(thick[j + 1:])
                if j == len(layers) - 1:
                    emit(f"{tid}-{l['id']}", base, l["id"])
                    continue
                sh = shrink(base, outside) if outside > 0 else base
                if sh is None:
                    continue
                emit(f"{tid}-{l['id']}", sh, l["id"])
    out.extend(extra or [])
    return out


def areas(slug, scale):
    txt = run_ts("author-07-areas.ts", f"humans/{slug}", str(scale))
    return json.loads(txt.strip().splitlines()[-1])[str(scale)]


def check_json(slug, scale):
    r = subprocess.run([bun(), f"{T}/volumes-check.ts", f"humans/{slug}", "--scale", str(scale), "--json"], cwd=T, capture_output=True, text=True)
    return json.loads(r.stdout[r.stdout.index("{"):])


def feature_offenders(slug, scale, lst):
    out = []
    for f in check_json(slug, scale)["findings"]:
        if f.get("rule") == "volumes-feature" and f.get("list") == lst:
            m = re.search(r'Solid "([^"]+)"', f["message"]["en"] if isinstance(f["message"], dict) else f["message"])
            if m:
                out.append(m.group(1))
    return out


def fit_thermal(B, structure_fn=None, exclude=None, extra=None, tol=0.06, rounds=5, limit=None):
    """Writes body + thermal (+ structure) of one scale and iterates the per-region skin factor until the outer-layer area is within tol of areaM2."""
    kfit = {}
    drop = set()
    limit = limit or (0.15 if B.scale == 50 else 0.3)
    for rnd in range(rounds):
        B.thermal = thermal_from_body(B, kfit, exclude, extra, drop)
        write_source(B.slug, B.scale, B.body_out(), B.thermal, B.structure or [{"id": "tmp", "of": B.body[0]["id"], "member": B.body[0]["bone"], "part": "member"}], B.decisions or [d("x", "x")])
        got = areas(B.slug, B.scale)
        worst = 0
        for region, v in got.items():
            tgt, a = v["target"], v["area"]
            if not tgt or a <= 0:
                continue
            dev = abs(a / tgt - 1)
            if dev > tol:
                kfit[region] = max(0.3, min(2.5, kfit.get(region, 1.0) * math.sqrt(tgt / a)))
                worst = max(worst, dev)
        if worst == 0:
            bad = [i for i in feature_offenders(B.slug, B.scale, "thermal") if i not in drop and not i.startswith("th-emitter")]
            if not bad:
                break
            drop.update(bad)
    B.kfit = kfit
    B.thermal = thermal_from_body(B, kfit, exclude, extra, drop)
    return kfit


def write_source(slug, scale, body, thermal, structure, decisions, category="humans"):
    path = f"{T}/volumes/{category}/{slug}.json"
    doc = json.load(open(path))
    v = doc["volumes"][str(scale)]
    assert v["status"] == "authored", (slug, scale)
    v["body"], v["thermal"], v["structure"], v["decisions"] = body, thermal, structure, decisions
    json.dump(doc, open(path, "w"), ensure_ascii=False, indent=1)


BUN = None


def bun():
    global BUN
    if BUN is None:
        BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
    return BUN


def run_ts(script, *args):
    r = subprocess.run([bun(), f"{T}/{script}", *args], cwd=T, capture_output=True, text=True)
    return r.stdout + r.stderr


HUMAN = ("pelvis", "torso", "neck", "head", "arm", "upper-arm", "forearm", "hand", "thigh", "shank", "foot", "toe")
EXP = {"head": 2.0, "hair": 2.0, "hat": 2.0, "helmet": 2.0, "torso": 2.5, "hip": 2.5, "neck": 2.0, "collar": 2.2, "backpack": 3.2}
NSEC = {50: 10, 100: 7, 200: 5, 500: 3}


def is_human_bone(b):
    return re.sub(r"-(l|r)$", "", b) in HUMAN


def convertible(s):
    if s["kind"] != "extrude" or s["plane"] != "xy" or "holes" in s or "transform" in s or "repeat" in s:
        return False
    if s["role"] == "body":
        return True
    us = [p[0] for p in s["profile"]]
    return (max(us) - min(us)) > 2 * 0.0075 and (s["to"] - s["from"]) > 0.03


def humanize(B, skip=(), bones=None, aspects=None, exps=None, lats=None, front=None, n=None, minext=None):
    aspects, exps, lats, front = aspects or {}, exps or {}, lats or {}, front or {}
    n = n or NSEC[B.scale]
    minext = B.feature if minext is None else minext
    for sid, s in B.raw.items():
        if sid in skip:
            continue
        if bones is None and not is_human_bone(s["bone"]):
            continue
        if bones is not None and s["bone"] not in bones:
            continue
        reg = B.raw_region(sid)
        if convertible(s):
            kw = {}
            if sid in front:
                fn = front_halfwidth(B, s["bone"], front[sid][0], front[sid][1] if len(front[sid]) > 1 else "front")
                if fn:
                    kw["rxfn"] = fn
            lo = loft_from(s, lat=lats.get(sid), aspect=aspects.get(sid), n=n, exp=exps.get(sid, EXP.get(stem(sid), 2.2)), minr=max(0.004, minext / 2 * 0.9), **kw)
            B.add(lo, reg)
        elif s["kind"] != "extrude":
            B.add(copy.deepcopy(s), reg)


def deltoid(B, arm_solid_id, scale=1.0, region=None):
    a = B.get(arm_solid_id)
    s0, s1 = a["sections"][0], a["sections"][1]
    r = max(s0["ry"], s1["ry"]) * 1.04 * scale
    side = "l" if arm_solid_id.endswith("l") or a["bone"].endswith("-l") else "r"
    B.add(ellipsoid(f"shoulder-{side}", a["bone"], [r, r, r], [s1["at"][0] * 0.55, s1["at"][1], s1["at"][2]], material=a["material"], role=a["role"], tone=a["tone"]), region if region is not None else B.reg.get(arm_solid_id))


def load_seed(B, drop=()):
    seed = json.load(open(f"{T}/author-07-seed/{B.slug}.json"))
    for s in seed["body"]:
        if s["id"] in drop:
            continue
        B.add(s, seed["regions"].get(s["id"]))
    return seed


def member_bones_of(slug, scale):
    out = run_ts("author-07-members.ts", f"humans/{slug}")
    for line in out.splitlines():
        parts = line.split()
        if len(parts) > 3 and parts[1] == str(scale):
            return parts[3].split("|")[0].strip().split(",")
    return []


def solid_extent_u(s):
    if s["kind"] == "loft":
        us = [q["at"][0] for q in s["sections"]]
        return min(us), max(us)
    return None


def profile_of(B, bone):
    """The dominant loft of a bone: (u0, u1, section function u -> (v, w, rx, ry))."""
    best = None
    for s in B.body:
        if s["bone"] == bone and s["kind"] == "loft" and s["role"] == "body":
            u0, u1 = solid_extent_u(s)
            if best is None or u1 - u0 > best[1] - best[0]:
                best = (u0, u1, s)
    if best is None:
        for s in B.body:
            if s["bone"] == bone and s["kind"] == "loft":
                u0, u1 = solid_extent_u(s)
                if best is None or u1 - u0 > best[1] - best[0]:
                    best = (u0, u1, s)
    if best is None:
        return None
    u0, u1, s = best
    return u0, u1, (lambda u: tuple(interp(s, min(max(u, u0), u1))[k] for k in ("v", "w", "rx", "ry")))


def human_member(B, bone, frac=0.45, id=None, u_from=None, u_to=None, rmin=None, **meta):
    """A load-bearing member along a human bone: a capsule on the dominant loft's centre line with a core section (frac of the body half section)."""
    p = profile_of(B, bone)
    if p is None:
        return None
    u0, u1, f = p
    a = max(u0, 0.0) if u_from is None else u_from
    b = min(u1, B.L(bone)) if u_to is None else u_to
    if b - a < 0.02:
        a, b = u0, u1
    va, wa, rxa, rya = f(a + 0.1 * (b - a))
    vb, wb, rxb, ryb = f(b - 0.1 * (b - a))
    ra = max(rmin or B.feature * 0.55, frac * min(rxa, rya))
    rb = max(rmin or B.feature * 0.55, frac * min(rxb, ryb))
    return {"id": id or f"member-{bone}", "kind": "capsule", "bone": bone, "p0": [a, va, wa], "p1": [b, vb, wb], "r0": ra, "r1": rb, "member": bone, "part": "member", **meta}


def joint_at(B, bone, f=1.15, id=None, material=None, tone=0.5, role="body"):
    """A joint solid at the origin of a bone (its rig joint to the parent) sized from the member core."""
    m = human_member(B, bone)
    if m is None:
        return None
    r = max(m["r0"] * f, B.feature * 0.6)
    p = m["p0"]
    return {"id": id or f"joint-{bone}", "kind": "ellipsoid", "bone": bone, "radii": [r, r, r], "transform": {"translation": [0.0, p[1], p[2]]}, "member": bone, "part": "joint", "material": m.get("material", material), "role": role, "tone": tone}


def human_structure(B, members, joints=(), masses=(), material=None, tone=0.45, frac=0.45, extra=()):
    out = []
    for bone in members:
        m = human_member(B, bone, frac=frac)
        if m is None:
            continue
        m["material"] = material or next(s["material"] for s in B.body if s["bone"] == bone)
        m["role"], m["tone"] = "body", tone
        out.append(m)
    for bone in joints:
        j = joint_at(B, bone, material=material, tone=tone)
        if j:
            j["material"] = material or next(s["material"] for s in B.body if s["bone"] == bone)
            out.append(j)
    for bone in masses:
        p = profile_of(B, bone)
        if p is None:
            continue
        u0, u1, f = p
        b = B.bones[bone]
        cx = (b.get("mass") or {}).get("comX", (u0 + u1) / 2)
        cx = min(max(cx, u0 + 0.2 * (u1 - u0)), u1 - 0.2 * (u1 - u0))
        v, w, rx, ry = f(cx)
        ru = max(B.feature, min(0.3 * (u1 - u0), 0.7 * max(rx, ry)))
        out.append({"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "radii": [ru, max(B.feature * 0.6, 0.65 * ry), max(B.feature * 0.6, 0.65 * rx)], "transform": {"translation": [cx, v, w]}, "member": bone, "part": "mass", "material": material or next(s["material"] for s in B.body if s["bone"] == bone), "role": "body", "tone": tone})
    out.extend(extra)
    return [r5(s) for s in out]


def anchor(B, aid):
    return next(a for a in B.data["anchors"] if a["id"] == aid)


def emitter_solid(B, emitter_id, anchor_id, diameter, region, flat=False, direction_u=1.0):
    a = anchor(B, anchor_id)
    r = max(diameter / 2, B.feature * 0.6)
    lift = 0.0 if not flat else 2.2 * B.feature
    mat = next(s["material"] for s in B.body if s["bone"] == a["bone"])
    if flat:
        radii = [B.feature * 0.6, r, r]
    else:
        radii = [r, r, r]
    return r5({"id": f"th-emitter-{emitter_id}", "kind": "ellipsoid", "bone": a["bone"], "radii": radii, "transform": {"translation": [a["x"] - lift, a["y"], 0.0]}, "material": mat, "role": "detail", "tone": 0.5, "region": region, "tags": [emitter_id]})


def contact_pad(B, aid, bone_member, length, width, y_ground=None, material="steel", tone=0.3):
    a = anchor(B, aid)
    t = max(B.feature, 0.01)
    return r5({"id": f"support-{aid}", "kind": "box", "bone": a["bone"], "size": [max(length, B.feature), t, max(width, B.feature)], "transform": {"translation": [a["x"], a["y"] + (t / 2 if y_ground is None else y_ground), 0.0]}, "member": bone_member, "part": "support", "anchor": aid, "material": material, "role": "body", "tone": tone})


def snap(B, ids="all", opts=""):
    """Writes the body, lets the TS tool fit loft sections laterally to the posed front/plan drawings, reloads the body."""
    doc = json.load(open(f"{T}/volumes/{B.category}/{B.slug}.json"))
    v = doc["volumes"][str(B.scale)]
    v["body"] = B.body_out()
    if not v.get("thermal"):
        v["thermal"] = [{"id": "tmp", "of": B.body[0]["id"], "region": next(iter(B.regions))}]
        v["structure"] = [{"id": "tmp", "of": B.body[0]["id"], "member": B.body[0]["bone"], "part": "member"}]
        v["decisions"] = [d("x", "x")]
    json.dump(doc, open(f"{T}/volumes/{B.category}/{B.slug}.json", "w"), ensure_ascii=False, indent=1)
    out = run_ts("author-07-snap.ts", f"{B.category}/{B.slug}", str(B.scale), ids if isinstance(ids, str) else ",".join(ids), opts)
    new = json.load(open(f"{T}/volumes/{B.category}/{B.slug}.json"))["volumes"][str(B.scale)]["body"]
    byid = {s["id"]: s for s in new}
    for s in B.body:
        if s["id"] in byid:
            s["sections"] = byid[s["id"]]["sections"] if s["kind"] == "loft" else s.get("sections")
    return out
