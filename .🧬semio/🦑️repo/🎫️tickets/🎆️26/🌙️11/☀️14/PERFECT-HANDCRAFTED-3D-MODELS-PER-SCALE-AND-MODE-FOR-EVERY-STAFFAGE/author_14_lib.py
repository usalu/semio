"""Authoring kit of author 14 (vehicles): drawing queries, region-less solid builders, thermal/structure helpers, writer and area calibration."""
import glob
import json
import math
import os
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import author_14_base as base  # noqa: E402
from author_14_base import path_rotation, qbasis, qz, r, vec, loc, locd, dec, cross3, VERT  # noqa: E402,F401

HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(HERE, "🗑️generated", "author-14")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles"
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def B(id, bone, material, role, tone, size, at=(0, 0, 0), corner=None, **kw):
    return base.box(id, bone, material, role, tone, None, size, at, corner, **kw)


def E(id, bone, material, role, tone, radii, at=(0, 0, 0), **kw):
    return base.ellipsoid(id, bone, material, role, tone, None, radii, at, **kw)


def C(id, bone, material, role, tone, p0, p1, r0, r1=None, lateral=None, **kw):
    return base.capsule(id, bone, material, role, tone, None, p0, p1, r0, r1, lateral, **kw)


def V(id, bone, material, role, tone, profile, axis=(1, 0, 0), origin=(0, 0, 0), lateral=None, sweep=None, **kw):
    return base.revolve(id, bone, material, role, tone, None, profile, axis, origin, lateral, sweep, **kw)


def X(id, bone, material, role, tone, profile, plane, frm, to, holes=None, taper=None, **kw):
    return base.extrude(id, bone, material, role, tone, None, profile, plane, frm, to, holes, taper, **kw)


def L(id, bone, material, role, tone, sections, cap=(True, True), **kw):
    return base.loft(id, bone, material, role, tone, None, sections, cap, **kw)


def slab(id, bone, material, role, tone, a, b, thick, width, corner=0.0, **kw):
    return base.slab(id, bone, material, role, tone, None, a, b, thick, width, corner, **kw)


def flat_tube(id, bone, material, role, tone, a, b, r_v, r_lat, **kw):
    return base.flat_tube(id, bone, material, role, tone, None, a, b, r_v, r_lat, **kw)


def ring(id, bone, material, role, tone, Rc, rt, wc=0.0, n=36, **kw):
    return base.ring(id, bone, material, role, tone, None, Rc, rt, wc, n, **kw)


def tyre_profile(R, hw, shoulder=0.18):
    return base.tyre_ring(R, hw, shoulder=shoulder)


def wheel_stack(prefix, bone, wc, R, hw, rim_r, rim_hw, hub_r, spokes, spoke_r, flip=1, tone_tyre=0.1, rim_mat="alloy", tyre_mat="rubber", shoulder=0.18, disc_r=None, parts=("tyre", "rim", "hub", "spoke")):
    out = []
    o = [0, 0, wc]
    if "tyre" in parts:
        out.append(V(prefix + "-tyre", bone, tyre_mat, "body", tone_tyre, tyre_profile(R, hw, shoulder), axis=(0, 0, 1), origin=o))
    if "rim" in parts:
        out.append(V(prefix + "-rim", bone, rim_mat, "detail", 0.55, [(-rim_hw, rim_r), (rim_hw, rim_r)], axis=(0, 0, 1), origin=o))
    if "disc" in parts and disc_r:
        out.append(V(prefix + "-disc", bone, "cast-iron", "detail", 0.35, [(-0.008, disc_r), (0.008, disc_r)], axis=(0, 0, 1), origin=[0, 0, wc - flip * (rim_hw - 0.02)]))
    if "hub" in parts:
        out.append(V(prefix + "-hub", bone, rim_mat, "detail", 0.7, [(-rim_hw - 0.012, hub_r), (rim_hw + 0.012, hub_r)], axis=(0, 0, 1), origin=o))
    if "spoke" in parts and spokes:
        out.append(C(prefix + "-spoke", bone, rim_mat, "detail", 0.8, (hub_r * 0.6, 0, wc + (rim_hw + spoke_r * 0.5) * flip), (rim_r - 0.012, 0, wc + (rim_hw + spoke_r * 0.5) * flip), spoke_r, spoke_r, repeat={"count": spokes, "rotationAxis": [0, 0, 1], "rotationRad": r(2 * math.pi / spokes)}))
    return out


class Sil:
    """The drawn silhouette of one asset: exact scanline queries over the closed filled shapes (world rings) of a scale and view."""

    def __init__(self, slug):
        d = json.load(open(os.path.join(GEN, slug + ".dump.json")))
        self.dimensions = d["dimensions"]
        self.frames = d["frames"]
        self.d = {int(k): v for k, v in d["scales"].items()}

    def shapes(self, scale, view, sel=None):
        out = self.d[scale][view]
        if sel is None:
            return out
        return [s for s in out if sel(s)]

    @staticmethod
    def _merge(iv):
        iv.sort()
        out = []
        for a, b in iv:
            if out and a <= out[-1][1] + 1e-9:
                out[-1][1] = max(out[-1][1], b)
            else:
                out.append([a, b])
        return [tuple(x) for x in out]

    def _cut(self, rings, c, axis):
        """Even-odd intervals along `1 - axis` at coordinate c of axis (axis 0: vertical line x=c; axis 1: horizontal line y=c)."""
        xs = []
        for ring in rings:
            n = len(ring)
            for i in range(n):
                a, b = ring[i - 1], ring[i]
                if (a[axis] <= c) != (b[axis] <= c):
                    t = (c - a[axis]) / (b[axis] - a[axis])
                    xs.append(a[1 - axis] + t * (b[1 - axis] - a[1 - axis]))
        xs.sort()
        return [(xs[k], xs[k + 1]) for k in range(0, len(xs) - 1, 2)]

    def col(self, scale, view, x, sel=None):
        iv = []
        for s in self.shapes(scale, view, sel):
            iv += [list(p) for p in self._cut(s["rings"], x, 0)]
        return self._merge(iv)

    def row(self, scale, view, y, sel=None):
        iv = []
        for s in self.shapes(scale, view, sel):
            iv += [list(p) for p in self._cut(s["rings"], y, 1)]
        return self._merge(iv)

    def extent(self, scale, view, sel=None):
        pts = [p for s in self.shapes(scale, view, sel) for ring in s["rings"] for p in ring]
        return (min(p[0] for p in pts), max(p[0] for p in pts), min(p[1] for p in pts), max(p[1] for p in pts))

    def ids(self, scale, view):
        return [(s["id"], s["bone"], s["role"]) for s in self.d[scale][view]]


def top(iv):
    return max(b for a, b in iv) if iv else None


def bot(iv):
    return min(a for a, b in iv) if iv else None


class Scale:
    """The three lists and decisions of one scale under construction; thermal and structure helpers keep ids unique."""

    def __init__(self, asset, denominator, K):
        self.asset, self.n, self.K = asset, denominator, K
        self.body, self.thermal, self.structure, self.decisions = [], [], [], []
        self.feature = FEATURE[denominator]

    def f(self, region):
        """Linear calibration factor of a thermal region at this scale (area scales with its square)."""
        return math.sqrt(self.K.get(f"{self.n}:{region}", 1.0))

    def add(self, *solids):
        self.body.extend(s for s in solids if s is not None)

    def th_of(self, body_id, region, layer=None, id=None):
        item = {"id": id or f"th-{body_id}", "of": body_id, "region": region}
        if layer:
            item["layer"] = layer
        self.thermal.append(item)

    def th(self, solid, region, layer=None, tags=None):
        s = dict(solid)
        s["region"] = region
        if layer:
            s["layer"] = layer
        if tags:
            s["tags"] = tags
        self.thermal.append(s)

    def st(self, solid, member, part, tags=None):
        s = dict(solid)
        s["member"] = member
        s["part"] = part
        if tags:
            s["tags"] = tags
        self.structure.append(s)

    def dec(self, en, de):
        self.decisions.append({"en": en, "de": de})

    def authored(self):
        return {"status": "authored", "denominator": self.n, "body": self.body, "thermal": self.thermal, "structure": self.structure, "decisions": self.decisions}


def centroid2(pts):
    a = cx = cy = 0.0
    for i in range(len(pts)):
        p, q = pts[i - 1], pts[i]
        w = p[0] * q[1] - q[0] * p[1]
        a += w
        cx += (p[0] + q[0]) * w
        cy += (p[1] + q[1]) * w
    if abs(a) < 1e-12:
        return pts[0]
    return (cx / (3 * a), cy / (3 * a))


def area2(pts):
    return abs(sum(pts[i - 1][0] * pts[i][1] - pts[i][0] * pts[i - 1][1] for i in range(len(pts)))) / 2


def scaled(pts, k, about=None):
    c = about or centroid2(pts)
    return [(c[0] + k * (p[0] - c[0]), c[1] + k * (p[1] - c[1])) for p in pts]


def plate(id, bone, material, tone, outline, plane, at, t, k=1.0, role="detail", about=None, **kw):
    """A flat panel: `outline` (plane coordinates) scaled by `k` in the plane, thickness `t` from `at` to `at + t` (negative t goes the other way)."""
    pts = scaled(outline, k, about)
    return X(id, bone, material, role, tone, pts, plane, at, at + t, **kw)


def stations(xs, f):
    return [f(x) for x in xs]


def hull(id, bone, material, role, tone, sts, e=3.4, **kw):
    """Loft along x through stations (x, y0, y1, halfwidth[, exponent[, zc]]): vertical slices, rx on z, ry on y."""
    secs = []
    for s in sts:
        x, y0, y1, hw = s[0], s[1], s[2], s[3]
        ex = s[4] if len(s) > 4 and s[4] is not None else e
        zc = s[5] if len(s) > 5 else 0.0
        secs.append(((x, (y0 + y1) / 2, zc), hw, (y1 - y0) / 2, ex))
    return L(id, bone, material, role, tone, secs, **kw)


def mirror_z(solid, new_id, bone=None):
    """Mirror a solid across w = 0 (copy with id; translations and p0/p1/origin z flipped; lofts keep the symmetric sections)."""
    s = json.loads(json.dumps(solid))
    s["id"] = new_id
    if bone:
        s["bone"] = bone
    if "transform" in s and "translation" in s["transform"]:
        s["transform"]["translation"][2] *= -1
    for k in ("p0", "p1", "origin"):
        if k in s:
            s[k][2] *= -1
    if "sections" in s:
        for sec in s["sections"]:
            sec["at"][2] *= -1
    if s.get("kind") == "extrude":
        s["from"], s["to"] = -s["to"], -s["from"]
    return s


def write(slug, scales, rep_status):
    out = {}
    for n in (50, 100, 200, 500):
        if rep_status[n] != "authored":
            out[str(n)] = {"status": "unavailable", "denominator": n}
        else:
            sc = scales[n]
            ids = [s["id"] for lst in (sc.body,) for s in lst]
            assert len(ids) == len(set(ids)), (n, "dup body ids", [i for i in ids if ids.count(i) > 1][:4])
            for lst_name in ("thermal", "structure"):
                ids = [s["id"] for s in getattr(sc, lst_name)]
                assert len(ids) == len(set(ids)), (n, lst_name, [i for i in ids if ids.count(i) > 1][:4])
            out[str(n)] = sc.authored()
    path = os.path.join(HERE, "volumes", "vehicles", slug + ".json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volumes": out}, f, ensure_ascii=False, indent=1)
    return path


def rep_status(slug):
    d = json.load(open(os.path.join(CATALOG, slug, "🔣️.json")))["asset"]
    return {int(k): v["status"] for k, v in d["representations"].items()}


def areas(slug):
    p = subprocess.run([BUN, os.path.join(HERE, "author-14-area.ts"), "vehicles/" + slug], capture_output=True, text=True, cwd=HERE)
    try:
        return json.loads(p.stdout)
    except Exception:
        sys.stderr.write(p.stdout + p.stderr)
        raise


def exposed(slug, scale=None):
    """Exposed thermal area per (scale, region) from the product check findings: {(scale, region): (area, target)} for the failing ones; passing regions are absent."""
    args = [BUN, os.path.join(HERE, "volumes-check.ts"), "vehicles/" + slug, "--json", "--rules", "volumes-thermal-coverage"]
    p = subprocess.run(args, capture_output=True, text=True, cwd=HERE)
    import re
    out = {}
    for f in json.loads(p.stdout).get("findings", []):
        m = re.search(r'region "([^"]+)" at 1:(\d+) is ([0-9.]+) m\S* against areaM2 ([0-9.]+)', json.dumps(f.get("message", f)) if not isinstance(f.get("message"), str) else f["message"])
        if m:
            out[(int(m.group(2)), m.group(1))] = (float(m.group(3)), float(m.group(4)))
    return out


def calibrate(slug, build, passes=4):
    """Rebuild until the exposed thermal area of every region matches areaM2 (K = area factor of the own, non-reference solids per `scale:region`)."""
    K = {}
    st = rep_status(slug)
    for i in range(passes + 1):
        scales = build(K)
        write(slug, scales, st)
        if i == passes:
            break
        bad = exposed(slug)
        if not bad:
            break
        for (sc, region), (area, tgt) in bad.items():
            if area > 0.001:
                K[f"{sc}:{region}"] = K.get(f"{sc}:{region}", 1.0) * (tgt / area) ** 0.95
    return K


def region_tunable(sc, region):
    return any(s.get("region") == region and "of" not in s for s in sc.thermal)


def check(slug, extra=()):
    p = subprocess.run([BUN, os.path.join(HERE, "volumes-check.ts"), "vehicles/" + slug, "--quiet", *extra], capture_output=True, text=True, cwd=HERE)
    return p.stdout


def render(slug, scale=None):
    args = [BUN, os.path.join(HERE, "volumes-render.ts"), "vehicles/" + slug]
    if scale:
        args += ["--scale", str(scale)]
    subprocess.run(args, capture_output=True, text=True, cwd=HERE)


def skin(sc, region, layers, pieces, k=None, material=None, tone=0.5):
    """Nested sheet layers (innermost first, (layer id, thickness m, material)) of one region over `pieces` [(name, bone, plane, outline, at, sign)]:
    the outermost layer lies on the outer face `at` and the inner layers follow inwards (sign +1: the body is below `at` along the push axis, -1: above); outlines scale by sqrt(K) in the plane."""
    f = sc.f(region) if k is None else k
    for name, bone, plane, outline, at, sign in pieces:
        off = 0.0
        for lid, t, mat in reversed(layers):
            lo, hi = (at - sign * off - sign * t, at - sign * off)
            a, b = (lo, hi) if lo < hi else (hi, lo)
            solid = X(f"th-{region}-{name}-{lid}" if len(layers) > 1 else f"th-{region}-{name}", bone, mat, "texture" if t < sc.feature else "detail", 0.5, scaled(outline, f), plane, a, b)
            sc.th(solid, region, lid if len(layers) > 1 else None)
            off += t


def proud(solid, new_id, d):
    """Copy of a loft with its cross sections thickened by d on the thin axis (outer face stands proud of the body surface)."""
    s = json.loads(json.dumps(solid))
    s["id"] = new_id
    for sec in s["sections"]:
        sec["ry"] = r(sec["ry"] + d)
    return s
