"""Authoring helpers for the humans-a volumes: draft access, outline-to-loft conversion, constructors, scale tiers.

Everything produced is explicit data written to volumes/humans/<slug>.json; the helpers only compose it.
"""
import glob
import json
import math
import os
import re

T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CATALOG = os.path.abspath(os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))


H = 0.7071067811865476
ROT = {"u": [0, -H, 0, H], "v": [-0.5, -0.5, -0.5, 0.5], "w": None}


def r4(x):
    if isinstance(x, float):
        v = round(x, 4)
        return 0.0 if v == 0 else v
    if isinstance(x, list):
        return [r4(i) for i in x]
    if isinstance(x, dict):
        return {k: (v if k == "rotation" else r4(v)) for k, v in x.items()}
    return x


class Asset:
    def __init__(self, slug):
        self.slug = slug
        self.path = glob.glob(CATALOG + "/*humans/" + slug + "/🔣️.json")[0]
        self.data = json.load(open(self.path))["asset"]
        self.bones = {b["id"]: b for b in self.data["rig"]["bones"]}
        self.draft = {s["id"]: s for s in json.load(open(f"{T}/🗑️generated/drafts/humans/{slug}.json"))["volume"]["solids"]}
        self.materials = {m["id"] for m in self.data["materials"]}
        self.regions = {r["id"] for r in self.data["thermal"]["regions"]}
        reps = self.data["representations"]
        self.denoms = sorted(int(k) for k, v in reps.items() if "geometry" in v)
        self.rep_ids = {}
        for k, rep in reps.items():
            if "geometry" not in rep:
                continue
            g = rep["geometry"]
            self.rep_ids[int(k)] = {s["id"] for v in ("elevation", "front", "plan") for s in g.get(v, [])}
        self.out = []

    def L(self, bone):
        return self.bones[bone]["lengthM"]

    def tier(self, sid, like=None):
        """Largest preset denominator whose drawing still contains the id stem, None when the coarsest one has it."""
        stems = [stem(sid)] + ([stem(like)] if like else [])
        best = None
        for d in self.denoms:
            if any(stem(i) in stems for i in self.rep_ids[d]):
                best = d
        if best is None:
            return self.denoms[0]
        return None if best == self.denoms[-1] else best

    def add(self, solid, like=None, maxden="auto"):
        if maxden == "auto":
            maxden = self.tier(solid["id"], like or solid["id"])
        if maxden is not None:
            solid["maxDenominator"] = maxden
        assert solid["material"] in self.materials, (solid["id"], solid["material"])
        for k in ("region",):
            if k in solid:
                assert solid[k] in self.regions, (solid["id"], solid[k])
        for k in solid.get("regions", []):
            assert k in self.regions, (solid["id"], k)
        assert solid["bone"] in self.bones, (solid["id"], solid["bone"])
        assert all(o["id"] != solid["id"] for o in self.out), solid["id"]
        self.out.append(r4(solid))
        return solid

    def keep(self, sid, **over):
        s = json.loads(json.dumps(self.draft[sid]))
        s.update(over)
        return self.add(s, like=sid)

    def write(self, decisions):
        d = {"volume": {"status": "authored", "solids": self.out, "decisions": decisions}}
        os.makedirs(f"{T}/volumes/humans", exist_ok=True)
        json.dump(d, open(f"{T}/volumes/humans/{self.slug}.json", "w"), ensure_ascii=False, indent=1)
        print(self.slug, len(self.out), "solids")


def rest_frames(A):
    """Side-projected rest frames: bone id -> (origin x, y, angle rad)."""
    out = {}
    for b in A.data["rig"]["bones"]:
        if b["parent"] is None:
            px, py, pa = 0.0, 0.0, 0.0
        else:
            px, py, pa = out[b["parent"]]
        x, y = b.get("x", 0.0), b.get("y", 0.0)
        ox = px + math.cos(pa) * x - math.sin(pa) * y
        oy = py + math.sin(pa) * x + math.cos(pa) * y
        out[b["id"]] = (ox, oy, pa + math.radians(b["angleDeg"]))
    return out


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


def front_halfwidth(A, bone, front_id, view="front"):
    """Half width of a front/plan shape as a function of the bone-local u (bone not horizontal at rest)."""
    g = A.data["representations"]["50"]["geometry"][view]
    shape = next(x for x in g if x["id"] == front_id)
    ox, oy, ang = rest_frames(A)[bone]
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


def stem(i):
    i = re.sub(r"-(l|r)$", "", i)
    i = re.sub(r"-(l|r)-", "-", i)
    i = re.sub(r"-?\d+$", "", i)
    return i


def meta(src, **over):
    m = {k: src[k] for k in ("material", "role", "tone", "bone") if k in src}
    for k in ("region", "regions"):
        if k in src:
            m[k] = src[k]
    m.update(over)
    return m


def ring_of(solid):
    assert solid["kind"] == "extrude" and solid["plane"] == "xy", solid["id"]
    return solid["profile"]


def span_at(ring, u):
    vs = []
    n = len(ring)
    for i in range(n):
        (u0, v0), (u1, v1) = ring[i], ring[(i + 1) % n]
        if (u0 <= u < u1) or (u1 <= u < u0):
            vs.append(v0 + (v1 - v0) * (u - u0) / (u1 - u0))
    return (min(vs), max(vs)) if len(vs) >= 2 else None


def loft_from(draft, lat=None, aspect=None, n=12, exp=2.4, minr=0.004, ends=0.0, id=None, ring=None, ramp="auto", rxfn=None, amax=2.4, **over):
    """Loft along u from the side outline of an xy extrude: ry and the v centre follow the outline, rx = ry * aspect.

    lat = (w0, w1) lateral span (default the extrude's from/to). When the span is much wider than the section is deep the part is
    a slanted tube: the section is round and its centre ramps across the span (ramp +1 rises with u, -1 falls, 0 none, auto by the -l/-r suffix).
    """
    ring = ring or ring_of(draft)
    us = [p[0] for p in ring]
    u0, u1 = min(us), max(us)
    w0, w1 = lat if lat else (draft["from"], draft["to"])
    wc, half = (w0 + w1) / 2, (w1 - w0) / 2
    if u1 - u0 < 0.05:
        n, ends = 2, 1.0
    probes = [u0 + (u1 - u0) * (1 - math.cos(math.pi * k / n)) / 2 for k in range(n + 1)]
    spans = []
    for k, u in enumerate(probes):
        uu = min(max(u, u0 + 1e-4), u1 - 1e-4)
        s = span_at(ring, uu) or span_at(ring, uu + 1e-4) or span_at(ring, uu - 1e-4)
        spans.append(s)
    rys = [(s[1] - s[0]) / 2 for s in spans]
    rymax = max(rys)
    slanted = ramp != 0 and aspect is None and half > 1.3 * rymax and re.sub(r"-(l|r)$", "", draft["bone"]) in ("arm", "upper-arm", "forearm", "hand")
    if slanted:
        if ramp == "auto":
            ramp = -1 if draft["id"].endswith("-r") or draft["bone"].endswith("-r") else 1
        ra = rys[1] if n > 2 else rys[0]
        rb = rys[-2] if n > 2 else rys[-1]
        ca, cb = (w0 + ra, w1 - rb) if ramp > 0 else (w1 - ra, w0 + rb)
    a = 1.0 if slanted else (aspect if aspect is not None else half / rymax)
    secs = []
    for k, (u, s) in enumerate(zip(probes, spans)):
        ry = max(minr, rys[k])
        if k in (0, n) and ends:
            ry = max(ry, ends * rymax)
        rx = max(minr, ry * a)
        if rxfn is not None:
            rx = max(minr, min(rxfn(u), ry * amax))
        w = (ca + (cb - ca) * (u - u0) / (u1 - u0)) if slanted else wc
        if slanted:
            w = min(max(w, w0 + rx), w1 - rx)
        secs.append({"at": [u, (s[0] + s[1]) / 2, w], "rx": rx, "ry": ry, "exponent": exp, "rotation": ROT["u"]})
    out = {"id": id or draft["id"], "kind": "loft", **meta(draft), "sections": secs, "capStart": True, "capEnd": True}
    out.update(over)
    return out


def surface_v(solid, u, w, front=True):
    """The v coordinate of a loft's surface at height u and lateral w (front = the -v side)."""
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
    d = ry * (1 - q ** e) ** (1 / e)
    return vc - d if front else vc + d


def capsule(id, bone, p0, p1, r0, r1=None, ls=None, **m):
    s = {"id": id, "kind": "capsule", "bone": bone, "p0": p0, "p1": p1, "r0": r0, "r1": r0 if r1 is None else r1, **m}
    if ls is not None:
        s["lateralScale"] = ls
    return s


def ellipsoid(id, bone, radii, at, rot=None, **m):
    t = {"translation": at}
    if rot:
        t["rotation"] = rot
    return {"id": id, "kind": "ellipsoid", "bone": bone, "radii": radii, "transform": t, **m}


def box(id, bone, size, at, corner=None, rot=None, **m):
    t = {"translation": at}
    if rot:
        t["rotation"] = rot
    s = {"id": id, "kind": "box", "bone": bone, "size": size, "transform": t, **m}
    if corner:
        s["cornerRadius"] = corner
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
    """Loft section: `a` is the position along the travel axis; rx lies on w (u, v travel) or u (w travel), ry on v (u travel), u (v travel) or v (w travel)."""
    p = list(at) if at else [0.0, 0.0, 0.0]
    p["uvw".index(along)] = a
    s = {"at": p, "rx": rx, "ry": ry, "exponent": e}
    if ROT[along]:
        s["rotation"] = ROT[along]
    return s
