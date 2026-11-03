#!/usr/bin/env python3
"""🪶️ Independent EN 1999 evaluate oracle.

Second implementation written from the clause formulas (EN 1990 6.10a/b, EN 1999-1-1 6.1-6.3, 6.7.4, 8.5, 8.6, 7.2,
EN 1999-1-2, 1-3, 1-4, 1-5), not from the Rust. Numerically it differs on purpose: section properties are integrated
over thin slices, chi is the root of the Ayrton-Perry equation found by bisection, the critical temperature and the
S-N endurance are found by search. Only the standard library is used.
"""
from __future__ import annotations
import importlib.util
import json, math, pathlib, sys

E, G = 70_000e6, 27_000e6


def _limit_rule():
    for parent in pathlib.Path(__file__).resolve().parents:
        path = parent / "🔮️oracles" / "🏃️harness" / "📏️tolerance" / "🐍️.py"
        if path.is_file():
            spec = importlib.util.spec_from_file_location("norm_limit_rule", path)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            return module
    raise FileNotFoundError("oracle tolerance module")


LIMIT = _limit_rule()


def _registry():
    from pathlib import Path
    root = next(p for p in Path(__file__).resolve().parents if (p / "⚖️compliance" / "📜️annex-and-tables").is_dir())
    path = root / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json"
    return {e["id"]: e for e in json.load(open(path, encoding="utf-8"))["entries"]}


NDP = _registry()


def ndp(key, annex="en"):
    e = NDP["ndp.en1999." + key]
    return e["naDe"] if annex == "de" else e["recommended"]


GM1, GM2, GMW, GMFI, GG, GQ, XI = (ndp(k) for k in ("gamma-m1", "gamma-m2", "gamma-mw", "gamma-m-fi", "gamma-g", "gamma-q", "xi"))
DEFL, SIGMA_RATIO = ndp("deflection-ratio"), ndp("stress-ratio")
K_FI = {"RC1": NDP["ndp.en1990.kfi.1"]["recommended"], "RC2": NDP["ndp.en1990.kfi.2"]["recommended"], "RC3": NDP["ndp.en1990.kfi.3"]["recommended"]}
RELIABILITY = {}
SB = 5.67e-8

BANDS = {
    "aw6060-t6": [(0.025, 160e6, 215e6, True, 0.48, 0.56)],
    "aw6061-t6": [(0.025, 240e6, 260e6, True, 0.48, 0.67)],
    "aw6063-t6": [(0.025, 160e6, 195e6, False, 0.41, 0.56)],
    "aw6082-t6": [(0.005, 250e6, 290e6, True, 0.48, 0.64), (0.025, 260e6, 310e6, True, 0.48, 0.60)],
    "aw5083-o": [(0.1, 125e6, 275e6, False, 1.0, 1.0)],
    "aw5083-h111": [(0.1, 125e6, 275e6, False, 1.0, 1.0)],
}
ALIASES = {"aw6060t6": "aw6060-t6", "en-aw-6060-t6": "aw6060-t6", "aw6061t6": "aw6061-t6", "aw6063t6": "aw6063-t6",
           "aw6082t6": "aw6082-t6", "aw5083o": "aw5083-o", "aw5083h111": "aw5083-h111"}


def alloy(designation, t):
    key = designation.lower().replace("_", "-")
    key = ALIASES.get(key, key)
    rows = BANDS.get(key)
    if not rows:
        return None
    for r in rows:
        if t <= r[0] + 1e-12:
            return dict(fo=r[1], fu=r[2], a=r[3], rho_o=r[4], rho_u=r[5])
    r = rows[-1]
    return dict(fo=r[1], fu=r[2], a=r[3], rho_o=r[4], rho_u=r[5])


def alloy_for_section(designation, s):
    ts = [e["thickness"] for e in s.get("elements", [])] + [s["flangeThickness"], s["webThickness"]]
    found = [alloy(designation, t) for t in ts if t > 0]
    found = [f for f in found if f]
    return min(found, key=lambda f: f["fo"]) if found else None


def default_alloy(designation):
    key = designation.lower().replace("_", "-")
    key = ALIASES.get(key, key)
    return alloy(designation, 1.0) if key in BANDS else None


PSI = {"residential": (.7, .5, .3), "A": (.7, .5, .3), "office": (.7, .5, .3), "B": (.7, .5, .3), "selfWeight": (.7, .5, .3),
       "congregation": (.7, .7, .6), "C": (.7, .7, .6), "retail": (.7, .7, .6), "D": (.7, .7, .6),
       "storage": (1.0, .9, .8), "E": (1.0, .9, .8), "snow": (.5, .2, 0.0), "snow_high": (.7, .5, .2),
       "wind": (.6, .2, 0.0), "temperature": (.6, .5, 0.0), "fire": (0.0, 0.0, 0.0)}


def psi(cat):
    return PSI.get(cat, (.4, .3, .2))


def gamma_mf(annex):
    return ndp("gamma-mf", annex)


# ---------------------------------------------------------------- EN 1990
def char_effects(member, a):
    if a.get("source") == "udl":
        L = max(member["length"], 1e-6)
        w = a["gKLine"] + a["qKLine"]
        sup = member.get("support", "simplySupported")
        km, kv = {"cantilever": (0.5, 1.0), "continuous": (1 / 12, 0.5)}.get(sup, (1 / 8, 0.5))
        return [a["nK"], a["vYK"], w * L * kv + a["vZK"], w * L * L * km + a["mYK"], a["mZK"], a["torsionK"], a["localLoadK"]]
    L = max(member["length"], 1e-6)
    w = a["gKLine"] + a["qKLine"]
    km, kv = {"cantilever": (1.0, 1.0), "continuous": (1 / 8, 0.5)}.get(member.get("support", "simplySupported"), (1 / 4, 0.5))
    return [a["nK"], a["vYK"], w * kv + a["vZK"], w * L * km + a["mYK"], a["mZK"], a["torsionK"], a["localLoadK"]]


def vadd(*vs):
    return [sum(c) for c in zip(*vs)]


def vmul(v, f):
    return [x * f for x in v]


def split(member):
    perm = [a for a in member["actions"] if a["kind"] == "permanent"]
    var = [a for a in member["actions"] if a["kind"] not in ("permanent", "fire")]
    g = [0.0] * 7
    for a in perm:
        g = vadd(g, char_effects(member, a))
    return perm, var, g


def uls_combos(member):
    perm, var, g = split(member)
    k_fi = RELIABILITY["k_fi"]
    gamma_g, gamma_q = GG * k_fi, GQ * k_fi
    if not var:
        return [("uls-g", vmul(g, gamma_g))]
    out = []
    for i, lead in enumerate(var):
        comp = [0.0] * 7
        for j, o in enumerate(var):
            if i != j:
                comp = vadd(comp, vmul(char_effects(member, o), gamma_q * psi(o["category"])[0]))
        le = char_effects(member, lead)
        out.append(("uls-610a", vadd(vmul(g, gamma_g), vmul(le, gamma_q * psi(lead["category"])[0]), comp)))
        out.append(("uls-610b", vadd(vmul(g, XI * gamma_g), vmul(le, gamma_q), comp)))
    return out


def sls_combos(member, kind):  # kind: characteristic | frequent | quasiPermanent
    perm, var, g = split(member)
    if not var:
        return [g]
    out = []
    for i, lead in enumerate(var):
        le = char_effects(member, lead)
        p = psi(lead["category"])
        e = vadd(g, le if kind == "characteristic" else vmul(le, p[1] if kind == "frequent" else p[2]))
        for j, o in enumerate(var):
            if i != j:
                po = psi(o["category"])
                e = vadd(e, vmul(char_effects(member, o), po[0] if kind == "characteristic" else po[2]))
        out.append(e)
    return out


def fire_effects(member):
    perm, var, g = split(member)
    for a in var:
        g = vadd(g, vmul(char_effects(member, a), psi(a["category"])[2]))
    return g


def proxy(id_, length, actions):
    L = max(length, 1e-6)
    return {"id": id_, "length": L, "support": "simplySupported", "actions": actions}


# ---------------------------------------------------------------- slices
def haz_width(t):
    return 0.020 if t <= 0.006 else 0.030 if t <= 0.012 else 0.035 if t <= 0.025 else 0.040


def haz_width_for(el):
    return haz_width(el["thickness"]) * (1.5 if el.get("weldProcess", "mig").lower() == "tig" else 1.0) * (1.33 if el.get("weldTJunction") else 1.0)


def behaves_outstand(el):
    ts, d = el.get("edgeStiffenerThickness", 0.0), el.get("edgeStiffenerDepth", 0.0)
    return el["outstand"] and not (ts > 0 and ts >= el["thickness"] and d >= 5 * ts)


def limits(a, outstand, welded):
    if a:
        return {(False, False): (11, 16, 22), (False, True): (9, 13, 18), (True, False): (3, 4.5, 6), (True, True): (2.5, 4, 5)}[(outstand, welded)]
    return {(False, False): (13, 16.5, 18), (False, True): (10, 13.5, 15), (True, False): (3.5, 4.5, 5), (True, True): (3, 3.5, 4)}[(outstand, welded)]


def consts(a, outstand, welded):
    if a and not welded:
        return (10, 24) if outstand else (32, 220)
    if (a and welded) or ((not a) and not welded):
        return (9, 20) if outstand else (29, 198)
    return (8, 16) if outstand else (25, 150)


def rho_c(b_eps, a, outstand, welded):
    if b_eps <= limits(a, outstand, welded)[2]:
        return 1.0
    c1, c2 = consts(a, outstand, welded)
    return min(1.0, max(0.0, c1 / b_eps - c2 / b_eps ** 2))


def eta_of(psi_):
    return 0.7 + 0.3 * psi_ if psi_ > -1 else 0.8 / (1 - psi_)


def cls(b_eps, lim):
    return 1 if b_eps <= lim[0] else 2 if b_eps <= lim[1] else 3 if b_eps <= lim[2] else 4


def is_tube(s):
    return s["kind"] in ("tube", "chs")


def tube_dims(s):
    d = max(s["outerDiameter"], s.get("width", 0.0))
    t = max(s["webThickness"], s.get("flangeThickness", 0.0))
    return d, t


def elem_beta_eps(el, fo, eta):
    return (eta if (not el["outstand"] and "web" in el["id"].lower()) else 1.0) * el["width"] / el["thickness"] / math.sqrt(250e6 / fo) if el["thickness"] > 0 else math.inf


def section_class(s, fo, a, eta):
    if is_tube(s):
        d, t = tube_dims(s)
        return cls(3 * math.sqrt(d / t) / math.sqrt(250e6 / fo), limits(a, False, False))
    return max([cls(elem_beta_eps(e, fo, eta), limits(a, behaves_outstand(e), e["welded"])) for e in s.get("elements", [])] or [1])


def parts_of(el, ctx):
    w = max(el["width"], 0.0)
    rho = rho_c(elem_beta_eps(el, ctx["fo"], ctx["eta"]), ctx["a"], behaves_outstand(el), el["welded"]) if ctx["local"] else 1.0
    if ctx["haz"] is None or not el["welded"]:
        return [(0.0, w, rho)]
    bh = haz_width_for(el)
    wp = el.get("weldPosition", 0.0)
    transverse = el.get("weldDirection", "longitudinal").lower() == "transverse"
    h0 = 0.0 if transverse else min(w, max(0.0, wp - bh))
    h1 = w if transverse else min(w, max(0.0, wp + bh))
    out = []
    if h0 > 1e-12:
        out.append((0.0, h0, rho))
    if h1 > h0:
        out.append((h0, h1, min(rho, ctx["haz"])))
    if w - h1 > 1e-12:
        out.append((h1, w, rho))
    return out or [(0.0, w, rho)]


def find(els, outstand, i):
    lst = [e for e in els if e["outstand"] == outstand]
    return (lst[i] if i < len(lst) else lst[0]) if lst else None


def find_named(els, web, i):
    lst = [e for e in els if ("web" in e["id"].lower()) == web]
    return (lst[i] if i < len(lst) else lst[0]) if lst else None


def make_strips(s, ctx):
    h, b, tf, tw = s["height"], s["width"], s["flangeThickness"], s["webThickness"]
    hw = max(h - 2 * tf, 0.0)
    els = s.get("elements", [])
    out = []

    def fl(el, root, d, zc, t, width):
        if el is None:
            out.append(dict(y=root + d * width / 2, z=zc, dy=width, dz=t, thin="z", f=1.0, role="F"))
            return
        for s0, s1, f in parts_of(el, ctx):
            out.append(dict(y=root + d * (s0 + s1) / 2, z=zc, dy=s1 - s0, dz=t, thin="z", f=f, role="F"))

    def web(el, yc, zroot, t, height):
        if el is None:
            out.append(dict(y=yc, z=zroot - height / 2, dy=t, dz=height, thin="y", f=1.0, role="W"))
            return
        for s0, s1, f in parts_of(el, ctx):
            out.append(dict(y=yc, z=zroot - (s0 + s1) / 2, dy=t, dz=s1 - s0, thin="y", f=f, role="W"))

    kind = s["kind"]
    if kind in ("box", "rhs"):
        clear = max(b - 2 * tw, 0.0)
        for i, sg in enumerate((1, -1)):
            zc = sg * (h - tf) / 2
            fl(find_named(els, False, i), -clear / 2, 1, zc, tf, clear)
            for side in (-1, 1):
                out.append(dict(y=side * (b - tw) / 2, z=zc, dy=tw, dz=tf, thin="z", f=1.0, role="S"))
        for i, side in enumerate((-1, 1)):
            web(find_named(els, True, i), side * (b - tw) / 2, hw / 2, tw, hw)
    elif kind == "angle":
        t = max(tf, tw)
        out.append(dict(y=t / 2, z=t / 2, dy=t, dz=t, thin="z", f=1.0, role="S"))
        v, hz = find(els, True, 0), find(els, True, 1)
        if v is None:
            out.append(dict(y=t / 2, z=t + (h - t) / 2, dy=t, dz=max(h - t, 0.0), thin="y", f=1.0, role="W"))
        else:
            for s0, s1, f in parts_of(v, ctx):
                out.append(dict(y=t / 2, z=t + (s0 + s1) / 2, dy=t, dz=s1 - s0, thin="y", f=f, role="W"))
        if hz is None:
            out.append(dict(y=t + (b - t) / 2, z=t / 2, dy=max(b - t, 0.0), dz=t, thin="z", f=1.0, role="F"))
        else:
            for s0, s1, f in parts_of(hz, ctx):
                out.append(dict(y=t + (s0 + s1) / 2, z=t / 2, dy=s1 - s0, dz=t, thin="z", f=f, role="F"))
    elif kind == "channel":
        wo = max(b - tw, 0.0)
        for i, sg in enumerate((1, -1)):
            zc = sg * (h - tf) / 2
            fl(find(els, True, i), tw / 2, 1, zc, tf, wo)
            out.append(dict(y=0.0, z=zc, dy=tw, dz=tf, thin="z", f=1.0, role="S"))
        web(find(els, False, 0), 0.0, hw / 2, tw, hw)
    else:
        wo = max((b - tw) / 2, 0.0)
        for i, sg in enumerate((1, -1)):
            zc = sg * (h - tf) / 2
            fl(find(els, True, i), tw / 2, 1, zc, tf, wo)
            fl(find(els, True, i), -tw / 2, -1, zc, tf, wo)
            out.append(dict(y=0.0, z=zc, dy=tw, dz=tf, thin="z", f=1.0, role="S"))
        web(find(els, False, 0), 0.0, hw / 2, tw, hw)
    return out


def integrate(strips, n=120):
    """📐️ Section properties by midpoint integration over n*n cells per strip."""
    cells = []
    for s in strips:
        dy = s["dy"] * (s["f"] if s["thin"] == "y" else 1.0)
        dz = s["dz"] * (s["f"] if s["thin"] == "z" else 1.0)
        if dy <= 0 or dz <= 0:
            continue
        cells.append((s["y"], s["z"], dy, dz, s["role"]))
    A = Sy = Sz = Av = Af = 0.0
    ymin = zmin = math.inf
    ymax = zmax = -math.inf
    for y, z, dy, dz, role in cells:
        a = dy * dz
        A += a
        Sy += a * y
        Sz += a * z
        if role == "W":
            Av += a
        else:
            Af += a
        ymin, ymax = min(ymin, y - dy / 2), max(ymax, y + dy / 2)
        zmin, zmax = min(zmin, z - dz / 2), max(zmax, z + dz / 2)
    if A <= 0:
        return dict(a=0, iy=0, iz=0, wy=0, wz=0, yg=0, zg=0, av=0, af=0)
    yg, zg = Sy / A, Sz / A
    iy = iz = 0.0
    for y, z, dy, dz, role in cells:
        da = dy * dz / (n * n)
        for i in range(n):
            zz = z - dz / 2 + (i + 0.5) * dz / n
            for j in range(n):
                yy = y - dy / 2 + (j + 0.5) * dy / n
                iy += da * (zz - zg) ** 2
                iz += da * (yy - yg) ** 2
    wy = iy / max(zmax - zg, zg - zmin, 1e-12)
    wz = iz / max(ymax - yg, yg - ymin, 1e-12)
    return dict(a=A, iy=iy, iz=iz, wy=wy, wz=wz, yg=yg, zg=zg, av=Av, af=Af)


_cache = {}


def props(s, ctx):
    key = (json.dumps(s, sort_keys=True), ctx["fo"], ctx["a"], round(ctx["eta"], 12), ctx["local"], ctx["haz"])
    if key in _cache:
        return _cache[key]
    if is_tube(s):
        d, t = tube_dims(s)
        di = max(d - 2 * t, 0.0)
        f = rho_c(3 * math.sqrt(d / t) / math.sqrt(250e6 / ctx["fo"]), ctx["a"], False, False) if ctx["local"] else 1.0
        A = math.pi / 4 * (d * d - di * di)
        I = math.pi / 64 * (d ** 4 - di ** 4)
        r = dict(a=A * f, iy=I * f, iz=I * f, wy=2 * I / d * f, wz=2 * I / d * f, yg=0.0, zg=0.0, av=2 * A * f / math.pi, af=2 * A * f / math.pi)
    else:
        r = integrate(make_strips(s, ctx), n=40)
    _cache[key] = r
    return r


def geometry(s):
    h, b, tf, tw = s["height"], s["width"], s["flangeThickness"], s["webThickness"]
    hw = max(h - 2 * tf, 0.0)
    k = s["kind"]
    if k in ("tube", "chs"):
        d, t = tube_dims(s)
        di = max(d - 2 * t, 0.0)
        A = math.pi / 4 * (d * d - di * di)
        I = math.pi / 64 * (d ** 4 - di ** 4)
        return dict(a=A, iy=I, iz=I, it=2 * math.pi * ((d - t) / 2) ** 3 * t, iw=0.0)
    if k in ("box", "rhs"):
        A = 2 * b * tf + 2 * hw * tw
        iy = (b * h ** 3 - max(b - 2 * tw, 0) * hw ** 3) / 12
        iz = (h * b ** 3 - hw * max(b - 2 * tw, 0) ** 3) / 12
        am = max(b - tw, 0) * max(h - tf, 0)
        st = 2 * max(b - tw, 0) / max(tf, 1e-12) + 2 * max(h - tf, 0) / max(tw, 1e-12)
        return dict(a=A, iy=iy, iz=iz, it=4 * am * am / max(st, 1e-12), iw=0.0)
    if k == "angle":
        t = max(tf, tw)
        return dict(a=(b + h - t) * t, iy=t * h ** 3 / 12 + b * t ** 3 / 12, iz=t * b ** 3 / 12 + h * t ** 3 / 12, it=(b + h - t) * t ** 3 / 3, iw=0.0)
    if k == "channel":
        A = 2 * b * tf + hw * tw
        iy = (b * h ** 3 - max(b - tw, 0) * hw ** 3) / 12
        yg = 2 * b * tf * (b / 2 - tw / 2) / max(A, 1e-12)
        iz = 2 * tf * b ** 3 / 12 + hw * tw ** 3 / 12 + 2 * b * tf * (b / 2 - tw / 2 - yg) ** 2 + hw * tw * yg * yg
        bc, hc = max(b - tw / 2, 0), max(h - tf, 0)
        iw = hc * hc * bc ** 3 * tf / 12 * (3 * bc * tf + 2 * hc * tw) / max(6 * bc * tf + hc * tw, 1e-12)
        return dict(a=A, iy=iy, iz=iz, it=(2 * b * tf ** 3 + hw * tw ** 3) / 3, iw=iw)
    A = 2 * b * tf + hw * tw
    iy = (b * h ** 3 - max(b - tw, 0) * hw ** 3) / 12
    iz = 2 * (tf * b ** 3 / 12) + hw * tw ** 3 / 12
    return dict(a=A, iy=iy, iz=iz, it=(2 * b * tf ** 3 + hw * tw ** 3) / 3, iw=tf * b ** 3 * (h - tf) ** 2 / 24)


def plastic(s):
    h, b, tf, tw = s["height"], s["width"], s["flangeThickness"], s["webThickness"]
    hw = max(h - 2 * tf, 0.0)
    k = s["kind"]
    g = props(s, ctx_gross(250e6, True))
    if k in ("tube", "chs"):
        d, t = tube_dims(s)
        w = (d ** 3 - max(d - 2 * t, 0) ** 3) / 6
        return w, w
    if k in ("box", "rhs"):
        return b * tf * (h - tf) + tw * hw * hw / 2, hw * tw * (b - tw) + tf * b * b / 2
    if k == "angle":
        return g["wy"], g["wz"]
    if k == "channel":
        return b * tf * (h - tf) + tw * hw * hw / 4, g["wz"]
    return b * tf * (h - tf) + tw * hw * hw / 4, tf * b * b / 2 + hw * tw * tw / 4


def ctx_gross(fo, a):
    return dict(fo=fo, a=a, eta=1.0, local=False, haz=None)


# ---------------------------------------------------------------- stability
def chi_ap(lam, alpha, lam0):
    """🔁️ Root of the Ayrton-Perry equation (1-chi)(1-chi*lam^2) = alpha*(lam-lam0)*chi in (0,1]."""
    if lam <= lam0:
        return 1.0
    k = alpha * (lam - lam0)
    lo, hi = 0.0, 1.0
    f = lambda x: (1 - x) * (1 - x * lam * lam) - k * x
    if f(1.0) >= 0:
        return 1.0
    for _ in range(80):
        mid = (lo + hi) / 2
        if f(mid) > 0:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def lam_bar(L, I, A, fo):
    if I <= 0 or A <= 0 or L <= 0:
        return 0.0
    ncr = math.pi ** 2 * E * I / L ** 2
    return math.sqrt(A * fo / ncr)


def kappa(k0, lam):
    s = min(1.0, max(0.0, (lam - 0.2) / 1.0))
    k0 = min(1.0, max(0.0, k0))
    return k0 + (1 - k0) * s


def ratio(x, r):
    if abs(x) < 1e-12:
        return 0.0
    if r <= 0:
        return 1e6
    return min(abs(x) / r, 1e6)


def ltb_curve(c):
    return (0.10, 0.6) if c <= 2 else (0.20, 0.4)


def shape_factor(c, wpl, wel, b_eps, lim):
    a12 = min(max(wpl / wel, 1.0), 1.25) if wel > 0 else 1.0
    if c <= 2:
        return a12
    if c == 3:
        span = max(lim[2] - lim[1], 1e-12)
        return min(max(1 + (lim[2] - b_eps) / span * (a12 - 1), 1.0), a12)
    return 1.0


def web_spacings(length, positions):
    """🦴️ Web panel widths between the transverse stiffeners inside the girder and its ends; `[0.0]` (unstiffened) without a stiffener."""
    inner = sorted({round(p, 9) for p in positions if 0.0 < p < length})
    if not inner:
        return [0.0]
    edges = [0.0] + inner + [length]
    return [b - a for a, b in zip(edges, edges[1:])]


def member_checks(member, s, mat, annex, out, positions=()):
    al = alloy_for_section(mat["designation"], s)
    if al is None:
        out.append({"id": f"en1999.3.2.alloy.{member['id']}", "utilization": 1.0})
        return
    fo, fu, a = al["fo"], al["fu"], al["a"]
    geo = geometry(s)
    lib = {}

    def put(cid, u):
        key = f"{cid}.{member['id']}"
        lib[key] = max(lib.get(key, -1e18), u)

    for name, eff in uls_combos(member):
        n, vy, vz, my, mz, tt, fl = eff
        gross = props(s, ctx_gross(fo, a))
        c = -n / max(gross["a"], 1e-18)
        s1, s2 = c + abs(my) / max(gross["wy"], 1e-18), c - abs(my) / max(gross["wy"], 1e-18)
        smax, smin = max(s1, s2), min(s1, s2)
        eta = eta_of(smin / smax) if smax > 0 else 1.0
        klass = section_class(s, fo, a, eta)
        base = dict(fo=fo, a=a, eta=eta, local=True, haz=None)
        loc_p = props(s, base)
        eff_o = props(s, {**base, "haz": al["rho_o"]})
        haz_o = props(s, {**base, "local": False, "haz": al["rho_o"]})
        haz_u = props(s, {**base, "local": False, "haz": al["rho_u"]})
        has_haz = any(e["welded"] for e in s.get("elements", []))
        # per element classification utilization
        for el in s.get("elements", []):
            b_eps = elem_beta_eps(el, fo, eta)
            lim = limits(a, behaves_outstand(el), el["welded"])
            lib[f"en1999.6.1.4.class.{member['id']}.{el['id']}"] = max(lib.get(f"en1999.6.1.4.class.{member['id']}.{el['id']}", -1e18), b_eps / lim[2])
        nt_yield = haz_o["a"] * fo / GM1
        nt_ult = (haz_u["a"] if has_haz else 0.9 * gross["a"]) * fu / GM2
        nt_rd = min(nt_yield, nt_ult)
        nc_rd = eff_o["a"] * fo / GM1
        nabs, nrd = (n, nt_rd) if n >= 0 else (-n, nc_rd)
        put("en1999.6.2.3.n", nabs / nrd if nrd else 0.0)
        wpl_y, wpl_z = plastic(s)
        if is_tube(s):
            d, t = tube_dims(s)
            gb, glim = 3 * math.sqrt(d / t) / math.sqrt(250e6 / fo), limits(a, False, False)
        else:
            cand = [(elem_beta_eps(e, fo, eta), limits(a, behaves_outstand(e), e["welded"])) for e in s.get("elements", [])]
            gb, glim = max(cand, key=lambda x: x[0] / x[1][2]) if cand else (0.0, (11, 16, 22))
        al_y = 1.0 if has_haz else shape_factor(klass, wpl_y, gross["wy"], gb, glim)
        al_z = 1.0 if has_haz else shape_factor(klass, wpl_z, gross["wz"], gb, glim)
        wy = eff_o["wy"] if has_haz else (loc_p["wy"] if klass == 4 else gross["wy"])
        wz = eff_o["wz"] if has_haz else (loc_p["wz"] if klass == 4 else gross["wz"])
        mry, mrz = al_y * wy * fo / GM1, al_z * wz * fo / GM1
        put("en1999.6.2.5.m", abs(my) / mry if mry else 0.0)
        put("en1999.6.2.5.mz", abs(mz) / mrz if mrz else 0.0)
        hw = max(s["height"] - 2 * s["flangeThickness"], 0.0)
        tw = s["webThickness"]
        vpl_z = haz_o["av"] * fo / (math.sqrt(3) * GM1)
        vpl_y = haz_o["af"] * fo / (math.sqrt(3) * GM1)
        if is_tube(s) or tw <= 0 or hw <= 0:
            rv = 1.0
        else:
            rvs = []
            for a_sp in web_spacings(member["length"], positions):
                ktau = (5.34 + 4 * (hw / a_sp) ** 2 if a_sp >= hw else 4.0 + 5.34 * (hw / a_sp) ** 2) if a_sp > 0 else 5.34
                lw = 0.35 * (hw / tw) * math.sqrt(fo / E) * math.sqrt(5.34 / ktau)
                rvs.append(1.0 if lw <= 0.83 else 0.83 / lw)
            rv = min(rvs)
        vb = rv * vpl_z
        put("en1999.6.2.6.v", abs(vz) / vpl_z if vpl_z else 0.0)
        put("en1999.6.2.6.vy", abs(vy) / vpl_y if vpl_y else 0.0)
        put("en1999.6.7.4.vb", abs(vz) / vb if vb else 0.0)
        wt = torsion_modulus(s)
        tau_t = abs(tt) / wt if wt > 0 else 0.0
        trd = wt * fo / (math.sqrt(3) * GM1)
        put("en1999.6.2.7.t", abs(tt) / trd if abs(tt) >= 1e-12 and trd > 1e-18 else (0.0 if abs(tt) < 1e-12 else 1e6))
        vrt = min(vpl_z, vb) * max(1 - tau_t * math.sqrt(3) * GM1 / fo, 0.0)
        put("en1999.6.2.7.vt", ratio(vz, vrt) if vrt > abs(vz) * 1e-6 else (0.0 if abs(vz) < 1e-12 else 1e6))
        if not is_tube(s):
            webs = 2.0 if s["kind"] in ("box", "rhs") else 1.0
            frd = webs * min(patch_rd(hw, tw, s["width"], s["flangeThickness"], member["bearingLength"], a_sp, fo) for a_sp in web_spacings(member["length"], positions))
            put("en1999.6.7.5.patch", abs(fl) / frd if abs(fl) >= 1e-12 and frd > abs(fl) * 1e-6 else (0.0 if abs(fl) < 1e-12 else 1e6))
        vr = min(vpl_z, vb)
        rho_v = (2 * abs(vz) / vr - 1) ** 2 if vr > 0 and abs(vz) > 0.5 * vr else 0.0
        mv = max(mry - rho_v * haz_o["av"] ** 2 / (4 * max(tw, 1e-9)) * fo / GM1, 0.0)
        put("en1999.6.2.8.mv", abs(my) / mv if mv else (0.0 if abs(my) < 1e-12 else 1e6))
        nm_rd = nt_rd if n >= 0 else nc_rd
        if klass >= 3 or has_haz:
            e_nz = abs(loc_p["zg"] - gross["zg"]) if klass == 4 else 0.0
            e_ny = abs(loc_p["yg"] - gross["yg"]) if klass == 4 else 0.0
            unm = ratio(nabs, nm_rd) + ratio(abs(my) + nabs * e_nz, mry) + ratio(abs(mz) + nabs * e_ny, mrz)
        else:
            psi_ = 1.5 if s["kind"] in ("tube", "chs") else 1.0 if s["kind"] in ("box", "rhs", "angle") else 2.0
            unm = ratio(nabs, nm_rd) ** psi_ + ratio(my, mry) + ratio(mz, mrz)
        put("en1999.6.2.9.nm", unm)
        k = {"continuous": 0.7, "cantilever": 2.0}.get(member.get("support", "simplySupported"), 1.0)
        pick = lambda l: l if l > 0 else k * member["length"]
        lcy, lcz, lct = pick(member["bucklingLengthY"]), pick(member["bucklingLengthZ"]), pick(member["bucklingLengthT"])
        llt = member["ltbLength"] if member["ltbLength"] > 0 else member["length"]
        aloc = loc_p["a"]
        k0 = eff_o["a"] / aloc if has_haz and aloc > 0 else 1.0
        alpha, l0 = (0.20, 0.10) if a else (0.32, 0.0)
        nc = max(-n, 0.0)

        def omega(kc, L):
            if not member["transverseWeld"]:
                return 1.0
            local = min(al["rho_o"] / max(kc, 1e-9), 1.0)
            return 1.0 - (1.0 - local) * abs(math.sin(math.pi * member["transverseWeldOffset"] / L)) if L > 0 else 1.0

        def nb(I, L):
            lam = lam_bar(L, I, aloc, fo)
            kc = kappa(k0, lam) * chi_ap(lam, alpha, l0)
            return lam, omega(kc, L) * kc * aloc * fo / GM1

        lam_z, nbz = nb(geo["iz"], lcz)
        lam_y, nby = nb(geo["iy"], lcy)
        chi_z = chi_ap(lam_z, alpha, l0)
        put("en1999.6.3.1.fb", ratio(nc, nby))
        put("en1999.6.3.1.fb.z", ratio(nc, nbz))
        i0 = (geo["iy"] + geo["iz"]) / max(gross["a"], 1e-18)
        if lct > 0 and i0 > 0:
            ncrt = (G * geo["it"] + math.pi ** 2 * E * geo["iw"] / lct ** 2) / i0
        else:
            ncrt = math.inf
        lam_t = math.sqrt(aloc * fo / ncrt) if math.isfinite(ncrt) and ncrt > 0 else 0.0
        nbt = kappa(k0, lam_t) * chi_ap(lam_t, 0.20 if a else 0.32, 0.20) * aloc * fo / GM1
        put("en1999.6.3.1.tb", ratio(nc, nbt))
        if member["restrainedLtb"] or llt <= 0 or geo["iz"] <= 0:
            mcr = math.inf
        else:
            mcr = member["c1"] * math.pi ** 2 * E * geo["iz"] / llt ** 2 * math.sqrt(max(geo["iw"] / geo["iz"] + llt ** 2 * G * geo["it"] / (math.pi ** 2 * E * geo["iz"]), 0.0))
        lam_lt = math.sqrt(mry * GM1 / mcr) if math.isfinite(mcr) and mcr > 0 else 0.0
        a_lt, l0_lt = ltb_curve(klass)
        chi_lt = chi_ap(lam_lt, a_lt, l0_lt)
        om_lt = omega(chi_lt, llt)
        mb = om_lt * chi_lt * mry
        put("en1999.6.3.2.ltb", ratio(my, mb))
        eta_c, gam_c, xi_c = max(0.8 * chi_z, 1.0), 1.0, max(0.8 * chi_z, 1.0)
        eq59 = ratio(nc, nby) ** eta_c + ratio(my, om_lt * mry) ** gam_c + ratio(mz, mrz) ** xi_c
        eq60 = ratio(nc, nbz) ** eta_c + ratio(my, mb) ** gam_c + ratio(mz, mrz) ** xi_c
        put("en1999.6.3.3.int", max(eq59, eq60))
    # serviceability
    base = dict(fo=fo, a=a, eta=1.0, local=True, haz=None)
    p = props(s, base)
    kw = {"simplySupported": 5 / 48, "continuous": 1 / 32, "cantilever": 0.25}[member.get("support", "simplySupported")]
    ratio_lim = DEFL * (0.5 if member.get("support") == "cantilever" else 1.0)
    for e in sls_combos(member, "quasiPermanent"):
        w = kw * abs(e[3]) * member["length"] ** 2 / (E * p["iy"]) if p["iy"] > 0 else 0.0
        put("en1999.7.2.defl-qp", w / (member["length"] / ratio_lim))
    for e in sls_combos(member, "characteristic"):
        w = kw * abs(e[3]) * member["length"] ** 2 / (E * p["iy"]) if p["iy"] > 0 else 0.0
        put("en1999.7.2.defl", w / (member["length"] / ratio_lim))
        sig = abs(e[0]) / p["a"] + abs(e[3]) / p["wy"] + abs(e[4]) / p["wz"]
        put("en1999.7.2.stress", sig / (SIGMA_RATIO * fo))
    for e in sls_combos(member, "frequent"):
        sig = abs(e[0]) / p["a"] + abs(e[3]) / p["wy"] + abs(e[4]) / p["wz"]
        put("en1999.7.2.sls-freq", sig / (SIGMA_RATIO * fo))
    for k_, v in lib.items():
        out.append({"id": k_, "utilization": v})



# ---------------------------------------------------------------- weld runs and free outlines
def apply_runs(member, s, runs):
    """🧵️ The member and section a member's weld runs imply: run flags override the element flags (transverse last), the nearest transverse run is the localized weld."""
    m, sec = dict(member), json.loads(json.dumps(s))
    for r in sorted(runs, key=lambda r: r["direction"] == "transverse"):
        for el in sec.get("elements", []):
            if el["id"] == r["elementId"]:
                el.update(welded=True, weldPosition=max(r["offset"], 0.0), weldProcess=r["process"], weldTJunction=r["tJunction"], weldDirection=r["direction"])
    xs = [max(min(r["start"], member["length"] - r["start"]), 0.0) for r in runs if r["direction"] == "transverse"]
    if xs:
        m["transverseWeld"], m["transverseWeldOffset"] = True, min(xs)
    return m, sec


def ring_groups(vertices, sid):
    rings = {}
    for v in vertices:
        if v["sectionId"] == sid:
            r = rings.setdefault(v["ring"], {"pts": [], "els": []})
            r["pts"].append((v["y"], v["z"]))
            r["els"].append(v["elementId"])
    return rings


def slice_props(rings, n=4000):
    """🔷️ Polygon-with-holes properties by even-odd scan-line integration (independent of the shoelace sums of the Rust)."""
    outer = rings["outer"]["pts"]
    z0, z1 = min(p[1] for p in outer), max(p[1] for p in outer)
    dz = (z1 - z0) / n
    a = sy = sz = iyy = izz = 0.0
    for i in range(n):
        z = z0 + (i + 0.5) * dz
        xs = []
        for r in rings.values():
            pts = r["pts"]
            for k in range(len(pts)):
                (ya, za), (yb, zb) = pts[k], pts[(k + 1) % len(pts)]
                if (za > z) != (zb > z):
                    xs.append(ya + (z - za) * (yb - ya) / (zb - za))
        xs.sort()
        for j in range(0, len(xs) - 1, 2):
            lo, hi = xs[j], xs[j + 1]
            a += (hi - lo) * dz
            sy += (hi * hi - lo * lo) / 2 * dz
            sz += (hi - lo) * z * dz
            iyy += (hi - lo) * z * z * dz
            izz += (hi ** 3 - lo ** 3) / 3 * dz
    yg, zg = sy / a, sz / a
    return dict(a=a, yg=yg, zg=zg, iy=iyy - a * zg * zg, iz=izz - a * yg * yg, o_iy=iyy, o_iz=izz)


def free_effective(s, rings, p, fo, a, eta):
    """🔷️ Strip model: every edge naming a plate element is classified with its length as b; a class-4 edge loses (1-rho)*b*t at its midpoint."""
    klass, gov, lim_gov = 1, 0.0, 22.0
    lost = sy = sz = iy_l = iz_l = 0.0
    els = {e["id"]: e for e in s.get("elements", [])}
    for r in rings.values():
        pts, names = r["pts"], r["els"]
        for i, name in enumerate(names):
            if not name or name not in els:
                continue
            e = {**els[name], "width": math.dist(pts[i], pts[(i + 1) % len(pts)])}
            ln = e["width"]
            b_eps = elem_beta_eps(e, fo, eta)
            lim = limits(a, behaves_outstand(e), e["welded"])
            c = cls(b_eps, lim)
            if c > klass or (c == klass and b_eps / lim[2] > gov / lim_gov):
                gov, lim_gov = b_eps, lim[2]
            klass = max(klass, c)
            da = (1.0 - rho_c(b_eps, a, behaves_outstand(e), e["welded"])) * ln * e["thickness"]
            if da <= 0 or ln <= 0:
                continue
            (ya, za), (yb, zb) = pts[i], pts[(i + 1) % len(pts)]
            ym, zm, cs, sn = (ya + yb) / 2, (za + zb) / 2, (yb - ya) / ln, (zb - za) / ln
            lost += da
            sy += da * ym
            sz += da * zm
            iy_l += da * (zm * zm + (ln * ln * sn * sn + e["thickness"] ** 2 * cs * cs) / 12)
            iz_l += da * (ym * ym + (ln * ln * cs * cs + e["thickness"] ** 2 * sn * sn) / 12)
    ae = p["a"] - lost
    yg, zg = (p["a"] * p["yg"] - sy) / ae, (p["a"] * p["zg"] - sz) / ae
    iy, iz = p["o_iy"] - iy_l - ae * zg * zg, p["o_iz"] - iz_l - ae * yg * yg
    outer = rings["outer"]["pts"]
    zm_, ym_ = max(abs(q[1] - zg) for q in outer), max(abs(q[0] - yg) for q in outer)
    return dict(klass=klass, a=ae, iy=iy, iz=iz, wy=iy / zm_, wz=iz / ym_)


def free_member_checks(member, s, mat, annex, vertices, out):
    al = alloy_for_section(mat["designation"], s)
    if al is None:
        out.append({"id": f"en1999.3.2.alloy.{member['id']}", "utilization": 1.0})
        return
    rings = ring_groups(vertices, s["id"])
    if "outer" not in rings:
        return
    p = slice_props(rings)
    zm_, ym_ = max(abs(q[1] - p["zg"]) for q in rings["outer"]["pts"]), max(abs(q[0] - p["yg"]) for q in rings["outer"]["pts"])
    wy, wz = p["iy"] / zm_, p["iz"] / ym_
    fo, fu, a = al["fo"], al["fu"], al["a"]
    lib = {}

    def put(cid, u):
        key = f"{cid}.{member['id']}"
        lib[key] = max(lib.get(key, -1e18), u)

    for name, eff in uls_combos(member):
        n, vy, vz, my, mz, tt, fl = eff
        c = -n / max(p["a"], 1e-18)
        s1, s2 = c + abs(my) / max(wy, 1e-18), c - abs(my) / max(wy, 1e-18)
        smax, smin = max(s1, s2), min(s1, s2)
        eta = eta_of(smin / smax) if smax > 0 else 1.0
        fe = free_effective(s, rings, p, fo, a, eta)
        nt_rd = min(p["a"] * fo / GM1, 0.9 * p["a"] * fu / GM2)
        nc_rd = fe["a"] * fo / GM1
        nabs, nrd = (n, nt_rd) if n >= 0 else (-n, nc_rd)
        put("en1999.6.2.3.n", nabs / nrd if nrd else 0.0)
        w_y, w_z = (fe["wy"], fe["wz"]) if fe["klass"] == 4 else (wy, wz)
        put("en1999.6.2.5.m", abs(my) / (w_y * fo / GM1) if w_y else 0.0)
        put("en1999.6.2.5.mz", abs(mz) / (w_z * fo / GM1) if w_z else 0.0)
        k = {"continuous": 0.7, "cantilever": 2.0}.get(member.get("support", "simplySupported"), 1.0)
        pick = lambda l: l if l > 0 else k * member["length"]
        alpha, l0 = (0.20, 0.10) if a else (0.32, 0.0)
        nc = max(-n, 0.0)

        def omega(kc, L):
            if not member["transverseWeld"]:
                return 1.0
            local = min(al["rho_o"] / max(kc, 1e-9), 1.0)
            return 1.0 - (1.0 - local) * abs(math.sin(math.pi * member["transverseWeldOffset"] / L)) if L > 0 else 1.0

        for cid, i, L in (("en1999.6.3.1.fb", fe["iy"], pick(member["bucklingLengthY"])), ("en1999.6.3.1.fb.z", fe["iz"], pick(member["bucklingLengthZ"]))):
            lam = lam_bar(L, i, fe["a"], fo)
            kc = kappa(1.0, lam) * chi_ap(lam, alpha, l0)
            put(cid, ratio(nc, omega(kc, L) * kc * fe["a"] * fo / GM1))
    for k_, v in lib.items():
        out.append({"id": k_, "utilization": v})


# ---------------------------------------------------------------- joints
FU = {"4.6": 400e6, "5.6": 500e6, "8.8": 800e6, "10.9": 1000e6, "a2-70": 700e6, "a4-70": 700e6, "rivet-6082": 160e6}
AS = [(8, 36.6), (10, 58.0), (12, 84.3), (14, 115.0), (16, 157.0), (20, 245.0), (24, 353.0), (27, 459.0), (30, 561.0)]


def stress_area(d):
    mm = d * 1e3
    best = min(AS, key=lambda r: abs(r[0] - mm))
    return best[1] * 1e-6 if abs(best[0] - mm) < 0.5 else 0.78 * math.pi / 4 * d * d


def d0_of(d):
    return d + (0.002 if d <= 0.024 else 0.003)


def elastic_group(rows, per_row, p1, p2, V, e):
    """Stiffness method of a rigid plate on equal elastic fasteners (independent of the closed form): solve the 3x3 equilibrium of the plate displacement (ux, uy, theta) for the force (V, 0) acting at the lever arm e from the centroid, return the resultants of all fasteners."""
    pts = [((i - (rows - 1) / 2) * p1, (j - (per_row - 1) / 2) * p2) for i in range(rows) for j in range(per_row)]
    k = [[0.0] * 3 for _ in range(3)]
    for x, y in pts:
        t = (1.0, 0.0, -y), (0.0, 1.0, x)
        for r in range(3):
            for q in range(3):
                k[r][q] += t[0][r] * t[0][q] + t[1][r] * t[1][q]
    rhs = [V, 0.0, -V * e]
    for col in range(3):
        piv = max(range(col, 3), key=lambda r: abs(k[r][col]))
        if abs(k[piv][col]) < 1e-18:
            k[col][col] += 1.0
            continue
        k[col], k[piv] = k[piv], k[col]
        rhs[col], rhs[piv] = rhs[piv], rhs[col]
        for r in range(3):
            if r != col:
                f = k[r][col] / k[col][col]
                k[r] = [a - f * b for a, b in zip(k[r], k[col])]
                rhs[r] -= f * rhs[col]
    u = [rhs[i] / k[i][i] for i in range(3)]
    return [math.hypot(u[0] - u[2] * y, u[1] + u[2] * x) for x, y in pts]


def ic_group(rows, per_row, p1, p2, V, e, mu, exponent):
    """Instantaneous-centre method (Crawford and Kulak curve R/R_ult = (1 - exp(-mu r/r_max))^k) by scipy: the three unknowns (centre x, centre y, scale) of the plate equilibrium are solved without assuming the symmetry axis; returns the resultant of every fastener, rows outermost."""
    from scipy.optimize import least_squares

    pts = [((i - (rows - 1) / 2) * p1, (j - (per_row - 1) / 2) * p2) for i in range(rows) for j in range(per_row)]
    if len(pts) < 2 or e == 0.0:
        return elastic_group(rows, per_row, p1, p2, V, e)

    def units(xc, yc):
        radii = [math.hypot(x - xc, y - yc) for x, y in pts]
        r_max = max(radii)
        out = []
        for (x, y), r in zip(pts, radii):
            w = (1.0 - math.exp(-mu * r / r_max)) ** exponent if r > 1e-15 else 0.0
            out.append((0.0, 0.0) if r <= 1e-15 else (-(y - yc) / r * w, (x - xc) / r * w))
        return out

    def residual(v):
        xc, yc, lam = v
        f = units(xc, yc)
        return [lam * sum(a for a, _ in f) - V, lam * sum(b for _, b in f), lam * sum(x * b - y * a for (x, y), (a, b) in zip(pts, f)) + V * e]

    reach = max(math.hypot(x, y) for x, y in pts)
    for scale in (0.05, 0.3, 1.0, 3.0, 10.0, 30.0):
        sol = least_squares(residual, [0.0, -math.copysign(scale * reach, e), -V / len(pts)], xtol=1e-14, ftol=1e-14, gtol=1e-14)
        if sol.cost < 1e-18 * V * V:
            return [abs(sol.x[2]) * math.hypot(a, b) for a, b in units(sol.x[0], sol.x[1])]
    return elastic_group(rows, per_row, p1, p2, V, e)


def filler_fw(f, parent_fu):
    return {"4043": 190e6, "alsi5": 190e6, "5356": 240e6, "almg5": 240e6, "5183": 270e6}.get(f.lower(), min(parent_fu, 190e6))


def net_width_paths(width, d0, lines, gauge, stagger):
    """🕳️ Net width by brute force: every subset of the hole lines is one path (holes of every second line offset by `stagger` along the load), Σ d0 is deducted and s²/(4p) added for each diagonal; the minimum over all paths."""
    import itertools
    best = width
    for k in range(1, lines + 1):
        for subset in itertools.combinations(range(lines), k):
            w = width - k * d0
            for j, m in zip(subset, subset[1:]):
                s = abs((stagger if m % 2 else 0.0) - (stagger if j % 2 else 0.0))
                w += s * s / (4 * (m - j) * gauge)
            best = min(best, w)
    return best


def connection_checks(c, mat, member, annex, out):
    if member is None:
        return
    al = default_alloy(mat["designation"])
    if al is None:
        out.append({"id": f"en1999.3.2.alloy.{c['id']}", "utilization": 1.0})
        return
    al = alloy(mat["designation"], c["bolts"]["plateThickness"]) or al
    kind = c["kind"].lower()
    if kind not in ("bolted", "welded", "combined", "pinned"):
        return
    src = member if not c["actions"] else proxy(c["id"], 1.0, c["actions"])
    lib = {}

    def put(cid, u):
        key = f"{cid}.{c['id']}"
        lib[key] = max(lib.get(key, -1e18), u)

    b, w = c["bolts"], c["welds"]
    for name, e in uls_combos(src):
        n, vy, vz, my, mz = e[:5]
        V, M = math.hypot(vy, vz), math.hypot(my, mz)
        if kind == "pinned" and FU.get(b["material"].lower(), 0) > 0:
            fup = FU[b["material"].lower()]
            d = b["diameter"]
            npin = max(b["rows"] * b["boltsPerRow"], 1)
            fed = math.hypot(V, abs(n)) / npin
            fvrd = 0.6 * fup * math.pi / 4 * d * d / GM2
            fbrd = 1.5 * b["plateThickness"] * d * al["fo"] / GM1
            med = fed * b["plateThickness"] / 8
            mrd = 1.5 * math.pi * d ** 3 / 32 * 0.8 * fup / GM2
            uv, um = 0.5 * fed / max(fvrd, 1e-9), med / max(mrd, 1e-9)
            put("en1999.8.5.pin.shear", uv)
            put("en1999.8.5.pin.bearing", fed / max(fbrd, 1e-9))
            put("en1999.8.5.pin.bend", um)
            put("en1999.8.5.pin.int", um * um + uv * uv)
        if kind in ("bolted", "combined") and FU.get(b["material"].lower(), 0) > 0:
            grade = b["material"].lower()
            nb = max(b["rows"] * b["boltsPerRow"], 1)
            lj = max(b["rows"] - 1, 0) * b["pitch"]
            blf = 1.0 if lj <= 15 * b["diameter"] else min(1.0, max(0.75, 1 - (lj - 15 * b["diameter"]) / (200 * b["diameter"])))
            d = b["diameter"]
            d0 = d0_of(d)
            fub = FU[grade]
            if grade.startswith("rivet-"):
                fv = 0.6 * fub * math.pi / 4 * d0 ** 2 / GM2 * blf
            else:
                fv = (0.5 if grade == "10.9" else 0.6) * fub * stress_area(d) / GM2 * blf
            ab = max(0.0, min(b["edgeDistance"] / (3 * d0), b["pitch"] / (3 * d0) - 0.25, fub / al["fu"], 1.0))
            k1 = max(0.0, min(2.8 * b["edgeDistance"] / d0 - 1.7, 2.5))
            fb = k1 * ab * al["fu"] * d * b["plateThickness"] / GM2
            ft = 0.9 * fub * stress_area(d) / GM2
            cnt, arm = (b["rows"], b["pitch"]) if b["rows"] > 1 else (b["boltsPerRow"], b["gauge"])
            per = nb / max(cnt, 1)
            z2 = sum((i - (cnt - 1) / 2) ** 2 * arm * arm for i in range(cnt)) * per
            zmax = (cnt - 1) / 2 * arm
            fm = M * zmax / z2 if z2 > 0 else M / max(arm, 1e-6)
            fved = max((ic_group(b["rows"], b["boltsPerRow"], b["pitch"], b["gauge"], V, c.get("loadEccentricity", 0.0), ndp("fastener-ic.curve-mu"), ndp("fastener-ic.curve-exponent")) if c.get("forceMethod", "elastic") == "instantaneousCentre" else elastic_group(b["rows"], b["boltsPerRow"], b["pitch"], b["gauge"], V, c.get("loadEccentricity", 0.0))))
            fted = max(n, 0.0) / nb + fm
            frd = min(fv, fb)
            put("en1999.8.5.bolt", fved / max(frd, 1e-9))
            if not grade.startswith("rivet-"):
                put("en1999.8.5.tension", fted / max(ft, 1e-9))
                put("en1999.8.5.vt", fved / max(frd, 1e-9) + fted / (1.4 * max(ft, 1e-9)))
                bp = 0.6 * math.pi * 1.5 * d * b["plateThickness"] * al["fu"] / GM2
                put("en1999.8.5.punch", fted / max(bp, 1e-9))
            put("en1999.8.5.pitch", 2.2 * d0 / max(b["pitch"], 1e-9))
            put("en1999.8.5.edge", 1.2 * d0 / max(b["edgeDistance"], 1e-9))
            put("en1999.8.5.gauge", ndp("table-8-2.p2-over-d0") * d0 / max(b["gauge"], 1e-9) if b["boltsPerRow"] > 1 else 0.0)
            put("en1999.8.5.emax", b["edgeDistance"] / (4 * b["plateThickness"] + 0.040))
            if b["rows"] > 1:
                put("en1999.8.5.pmax", b["pitch"] / max(min(14 * b["plateThickness"], 0.2), 1e-9))
            nrow = max(b["rows"], 1)
            ant = max((b["edgeDistance"] - 0.5 * d0) * b["plateThickness"], 0.0)
            anv = max((b["edgeDistance"] + (nrow - 1) * b["pitch"] - (nrow - 0.5) * d0) * b["plateThickness"], 0.0)
            blk = al["fu"] * ant / GM2 + al["fo"] * anv / (math.sqrt(3) * GM1)
            put("en1999.8.5.block", V / max(blk, 1e-9))
            lines = b["boltsPerRow"]
            plate_w = max(lines - 1, 0) * b["gauge"] + 2 * b["edgeDistance"]
            net_w = net_width_paths(plate_w, d0, lines, b["gauge"], c.get("holeStagger", 0.0))
            n_rd = min(plate_w * b["plateThickness"] * al["fo"] / GM1, 0.9 * net_w * b["plateThickness"] * al["fu"] / GM2)
            put("en1999.8.5.net", V / max(n_rd, 1e-9))
        if kind in ("welded", "combined"):
            pt = max(b["plateThickness"], w["throat"] / 0.7)
            band = alloy(mat["designation"], pt) or al
            fw = filler_fw(w["fillerAlloy"], band["fu"])
            metal = fw * w["throat"] / (max(w["betaW"], 0.01) * GMW)
            haz = band["rho_u"] * band["fu"] * (w["throat"] / 0.7) / (math.sqrt(3) * GM2)
            L = max(w["length"], 1e-9)
            fed = math.hypot(abs(n) / L + 6 * M / (L * L), V / L)
            put("en1999.8.6.weld", fed / max(min(metal, haz), 1e-9))
            put("en1999.8.6.length", max(8 * w["throat"], 0.04) / L)
            bh = w["hazExtent"] if w["hazExtent"] > 0 else haz_width(pt)
            tp = b["plateThickness"]
            if w["hazExtent"] > 0:
                ag = tp * L
                ah = min(2 * bh * tp, ag)
                ao, au = ag - ah * (1 - band["rho_o"]), ag - ah * (1 - band["rho_u"])
            else:
                ao, au = band["rho_o"] * tp * L, band["rho_u"] * tp * L
            nhaz = min(ao * band["fo"] / GM1, au * band["fu"] / GM2)
            put("en1999.8.6.haz", (abs(n) + V) / max(nhaz, 1e-9))
    for e in sls_combos(src, "frequent"):
        n, vy, vz, my, mz = e[:5]
        V, M = math.hypot(vy, vz), math.hypot(my, mz)
        cap = math.inf
        if kind in ("bolted", "combined"):
            grade = b["material"].lower()
            if FU.get(grade, 0) > 0:
                d = b["diameter"]
                d0 = d0_of(d)
                fub = FU[grade]
                nb = max(b["rows"] * b["boltsPerRow"], 1)
                lj = max(b["rows"] - 1, 0) * b["pitch"]
                blf = 1.0 if lj <= 15 * d else min(1.0, max(0.75, 1 - (lj - 15 * d) / (200 * d)))
                if grade.startswith("rivet-"):
                    fv = 0.6 * fub * math.pi / 4 * d0 ** 2 / GM2 * blf
                else:
                    fv = (0.5 if grade == "10.9" else 0.6) * fub * stress_area(d) / GM2 * blf
                ab = max(0.0, min(b["edgeDistance"] / (3 * d0), b["pitch"] / (3 * d0) - 0.25, fub / al["fu"], 1.0))
                k1 = max(0.0, min(2.8 * b["edgeDistance"] / d0 - 1.7, 2.5))
                fb = k1 * ab * al["fu"] * d * b["plateThickness"] / GM2
                cap = min(cap, nb * min(fv, fb))
        if kind == "pinned" and FU.get(b["material"].lower(), 0) > 0:
            npin = max(b["rows"] * b["boltsPerRow"], 1)
            cap = min(cap, 2 * npin * 0.6 * FU[b["material"].lower()] * math.pi / 4 * b["diameter"] ** 2 / GM2)
        if kind in ("welded", "combined"):
            fw = filler_fw(w["fillerAlloy"], al["fu"])
            metal = fw * w["throat"] / (max(w["betaW"], 0.01) * GMW)
            haz = al["rho_u"] * al["fu"] * (w["throat"] / 0.7) / (math.sqrt(3) * GM2)
            cap = min(cap, min(metal, haz) * w["length"])
        if math.isfinite(cap):
            key = f"en1999.8.sls-freq.{c['id']}"
            lib[key] = max(lib.get(key, -1e18), (V + abs(n)) / max(cap, 1e-9))
    for k_, v in lib.items():
        out.append({"id": k_, "utilization": v})


# ---------------------------------------------------------------- fire
FIRE = [(th, NDP[f"ndp.en1999.fire.k-0.2proof.{th}"]["recommended"], NDP[f"ndp.en1999.fire.k-e.{th}"]["recommended"]) for th in (20, 50, 100, 150, 200, 250, 300, 350, 400, 550)]


def fire_k(theta, col):
    if theta <= FIRE[0][0]:
        return FIRE[0][col]
    for a_, b_ in zip(FIRE, FIRE[1:]):
        if theta <= b_[0]:
            t = (theta - a_[0]) / (b_[0] - a_[0])
            return a_[col] + t * (b_[col] - a_[col])
    return FIRE[-1][col]


def gas(t):
    return 20 + 345 * math.log10(8 * t / 60 + 1)


def member_temperature(amv, duration, th0):
    th, t = th0, 0.0
    while t < duration - 1e-9:
        step = min(5.0, duration - t)
        tn = t + step
        g_ = gas(tn)
        hnet = 25 * (g_ - th) + 0.3 * SB * ((g_ + 273) ** 4 - (th + 273) ** 4)
        th += amv / ((0.41 * th + 903) * 2700) * hnet * step
        t = tn
    return th


def protected_temperature(amv, duration, th0, ins):
    """🧱 Protected member by explicit stepping (5 s) of the insulation formula with the registry divisors k1, k2; no thickness is the bare member."""
    if ins["thickness"] <= 0:
        return member_temperature(amv, duration, th0)
    k1, k2 = ndp("fire-protected-denominator"), ndp("fire-protected-exponent")
    th, t = max(th0, 20.0), 0.0
    while t < duration - 1e-9:
        step = min(5.0, duration - t)
        g0, g1 = gas(t), gas(t + step)
        ca = 0.41 * th + 903
        phi = ins["specificHeat"] * ins["density"] / (ca * 2700) * ins["thickness"] * amv
        rise = ins["conductivity"] / ins["thickness"] * amv / (ca * 2700) * (g1 - th) / (1 + phi / k1) * step - (math.exp(phi / k2) - 1) * (g1 - g0)
        th += max(rise, 0.0) if g1 > g0 else rise
        t += step
    return th


def perimeter(s):
    k, h, b, tw = s["kind"], s["height"], s["width"], s["webThickness"]
    if k in ("tube", "chs"):
        return math.pi * max(s["outerDiameter"], b)
    if k in ("box", "rhs", "angle"):
        return 2 * (h + b)
    return 2 * h + 4 * b - 2 * tw


def fire_field(s, fo, a, th0, duration, theta_uniform):
    """🧊️ Strength retained with the strip temperatures (section factor 2/t per strip) against the uniform member temperature: ratios for N, M_y and M_z."""
    if is_tube(s):
        return 1.0, 1.0, 1.0
    st = make_strips(s, ctx_gross(fo, a))
    if not st:
        return 1.0, 1.0, 1.0
    thetas = [member_temperature(2.0 / max(min(q["dy"], q["dz"]), 1e-6), duration, th0) for q in st]
    areas = [q["dy"] * q["dz"] for q in st]
    tot = sum(areas)
    yg = sum(w * q["y"] for w, q in zip(areas, st)) / tot
    zg = sum(w * q["z"] for w, q in zip(areas, st)) / tot
    ku = fire_k(theta_uniform, 1)

    def ratio_of(weight):
        den = sum(w * weight(q) for w, q in zip(areas, st))
        num = sum(fire_k(th, 1) * w * weight(q) for th, w, q in zip(thetas, areas, st))
        return 1.0 if ku < 1e-6 or den <= 0 else num / den / ku
    return ratio_of(lambda q: 1.0), ratio_of(lambda q: abs(q["z"] - zg)), ratio_of(lambda q: abs(q["y"] - yg))


def fire_checks(f, member, s, mat, out, ins=None):
    al = alloy_for_section(mat["designation"], s)
    if al is None:
        out.append({"id": f"en1999.1-2.fire.{f['id']}", "utilization": 1.0})
        return
    fo, a = al["fo"], al["a"]
    geo = geometry(s)
    amv = perimeter(s) / geo["a"] if geo["a"] > 0 else 0.0
    protected = ins is not None and ins["thickness"] > 0
    theta_calc = protected_temperature(amv, max(f["durationS"], 0.0), max(f["thetaA"], 20.0), ins) if protected else member_temperature(amv, max(f["durationS"], 0.0), max(f["thetaA"], 20.0))
    theta = theta_calc
    r_n, r_my, r_mz = (1.0, 1.0, 1.0) if protected else fire_field(s, fo, a, max(f["thetaA"], 20.0), max(f["durationS"], 0.0), theta_calc)
    e = fire_effects(member)
    n, my, mz = e[0], e[3], e[4]
    nc = max(-n, 0.0)
    base = dict(fo=fo, a=a, eta=1.0, local=True, haz=None)
    loc_p = props(s, base)
    eff = props(s, {**base, "haz": al["rho_o"]})
    klass = section_class(s, fo, a, 1.0)

    def res(th, rn=1.0, rmy=1.0, rmz=1.0):
        k, ke = fire_k(th, 1), max(fire_k(th, 2), 1e-6)
        sc = math.sqrt(k / ke)
        kk = {"continuous": 0.7, "cantilever": 2.0}.get(member.get("support", "simplySupported"), 1.0)
        pick = lambda l: l if l > 0 else kk * member["length"]
        alpha, l0 = (0.20, 0.10) if a else (0.32, 0.0)
        chi = min(chi_ap(lam_bar(pick(member["bucklingLengthY"]), geo["iy"], loc_p["a"], fo) * sc, alpha, l0),
                  chi_ap(lam_bar(pick(member["bucklingLengthZ"]), geo["iz"], loc_p["a"], fo) * sc, alpha, l0))
        my_r = eff["wy"] * k * fo / GMFI * rmy
        llt = member["ltbLength"] if member["ltbLength"] > 0 else member["length"]
        if member["restrainedLtb"] or llt <= 0 or geo["iz"] <= 0:
            lam = 0.0
        else:
            mcr = member["c1"] * math.pi ** 2 * E * geo["iz"] / llt ** 2 * math.sqrt(max(geo["iw"] / geo["iz"] + llt ** 2 * G * geo["it"] / (math.pi ** 2 * E * geo["iz"]), 0.0))
            lam = math.sqrt(eff["wy"] * fo / mcr) * sc
        a_lt, l0_lt = ltb_curve(klass)
        return dict(nt=k * eff["a"] * fo / GMFI * rn, nc=k * eff["a"] * fo / GMFI * rn, my=my_r, mz=eff["wz"] * k * fo / GMFI * rmz,
                    nb=chi * k * loc_p["a"] * fo / GMFI * rn, mb=chi_ap(lam, a_lt, l0_lt) * my_r)

    def usum(r):
        return ratio(max(n, 0.0), r["nt"]) + ratio(nc, r["nc"]) + ratio(my, r["my"]) + ratio(mz, r["mz"])

    r = res(theta, r_n, r_my, r_mz)
    mu0 = usum(res(20.0))
    lo, hi = 20.0, 550.0
    if mu0 >= 1:
        th_cr = 20.0
    elif mu0 <= 0:
        th_cr = 550.0
    else:
        for _ in range(80):
            mid = (lo + hi) / 2
            if fire_k(mid, 1) > mu0:
                lo = mid
            else:
                hi = mid
        th_cr = (lo + hi) / 2
    out.append({"id": f"en1999.1-2.fire.{f['id']}", "utilization": usum(r)})
    out.append({"id": f"en1999.1-2.fire.buckling.{f['id']}", "utilization": ratio(nc, r["nb"])})
    out.append({"id": f"en1999.1-2.fire.ltb.{f['id']}", "utilization": ratio(my, r["mb"])})
    out.append({"id": f"en1999.1-2.fire.theta-cr.{f['id']}", "utilization": theta / max(th_cr, 1.0)})


# ---------------------------------------------------------------- fatigue
NC, ND, NL = 2e6, 5e6, 1e8


def detail_dc(cat):
    tok = cat.strip()
    if tok.endswith("-weld"):
        tok = tok[:-5]
    try:
        v = float(tok)
    except ValueError:
        return None
    return v * 1e6 if v in (14, 16, 18, 20, 22, 25, 28, 32, 36, 40, 45, 50, 56, 63, 71, 80, 90, 100, 112, 125) else None


def endurance(ds, dc, m1, m2):
    """📈️ N(delta-sigma) of the S-N curve found by segment search."""
    if ds <= 0:
        return math.inf
    dd = dc * (NC / ND) ** (1 / m1)
    dl = dd * (ND / NL) ** (1 / m2)
    if ds >= dd:
        return NC * (dc / ds) ** m1
    if ds < dl:
        return math.inf
    return ND * (dd / ds) ** m2


def strength_at(dc, m1, m2, n):
    dd = dc * (NC / ND) ** (1 / m1)
    if n <= ND:
        return dc * (NC / n) ** (1 / m1)
    if n >= NL:
        return dd * (ND / NL) ** (1 / m2)
    return dd * (ND / n) ** (1 / m2)


def fatigue_checks(fd, member, annex, out):
    dc = detail_dc(fd["detailCategory"])
    if dc is None or fd["m1"] <= 0 or (member is None and fd["memberId"]):
        return
    m1 = fd["m1"]
    m2 = fd["m2"] if fd["m2"] > 0 else m1 + 2
    gmf = gamma_mf(annex)
    n_allow = endurance(fd["deltaSigmaEd"] * 1.0 * gmf, dc, m1, m2)
    d = 0.0 if math.isinf(n_allow) else fd["nCycles"] / n_allow
    rd = strength_at(dc, m1, m2, fd["nCycles"]) / gmf
    out.append({"id": f"en1999.1-3.fat.{fd['id']}", "utilization": d})
    out.append({"id": f"en1999.1-3.fat.range.{fd['id']}", "utilization": fd["deltaSigmaEd"] / max(rd, 1e-9)})


def fatigue_spectra(details, members, annex, out):
    """📊️ Palmgren-Miner sum of the rows of one member and one detail category (the first row names the spectrum and gives the S-N curve)."""
    groups = {}
    for i, fd in enumerate(details):
        if fd["memberId"]:
            groups.setdefault((fd["memberId"], fd["detailCategory"]), []).append(fd)
    gmf = gamma_mf(annex)
    for rows in groups.values():
        first = rows[0]
        dc = detail_dc(first["detailCategory"])
        if len(rows) < 2 or dc is None or first["m1"] <= 0 or first["memberId"] not in members:
            continue
        m1 = first["m1"]
        m2 = first["m2"] if first["m2"] > 0 else m1 + 2
        total = 0.0
        for fd in rows:
            n_allow = endurance(fd["deltaSigmaEd"] * gmf, dc, m1, m2)
            total += 0.0 if math.isinf(n_allow) else fd["nCycles"] / n_allow
        out.append({"id": f"en1999.1-3.fat.miner.{first['id']}", "utilization": total})


def category_checks(categories, details, members, annex, out):
    """🎯️ Palmgren-Miner sum of one fatigue category: the rows of its member and detail category are its blocks, the slopes of the site override the block slopes."""
    gmf = gamma_mf(annex)
    for c in categories:
        rows = [d for d in details if d["memberId"] == c["memberId"] and d["detailCategory"] == c["detailCategory"]]
        dc = detail_dc(c["detailCategory"])
        if c["memberId"] not in members or dc is None or not rows:
            continue
        m1 = c["m1"] if c["m1"] > 0 else rows[0]["m1"]
        if m1 <= 0:
            continue
        m2 = c["m2"] if c["m2"] > 0 else (rows[0]["m2"] if c["m1"] <= 0 and rows[0]["m2"] > 0 else m1 + 2)
        total = 0.0
        for fd in rows:
            n_allow = endurance(fd["deltaSigmaEd"] * gmf, dc, m1, m2)
            total += 0.0 if math.isinf(n_allow) else fd["nCycles"] / n_allow
        out.append({"id": f"en1999.1-3.fat.category.{c['id']}", "utilization": total})


TRANSVERSE = (6.0, 2.0, 0.9, 0.5, 0.02)


def patch_checks(loads, members, sections, materials, out, stiffeners=()):
    """⬇️ Transverse web load of §6.7.5 with the recalled structure constants (unverified); end loads and sheets stay unevaluated."""
    k0, k1, c_cr, chi0, m2f = TRANSVERSE
    for ld in loads:
        m = members.get(ld["subjectId"])
        s = sections.get(m["sectionId"]) if m else None
        mat = materials.get(m["materialId"]) if m else None
        if not (m and s and mat) or ld["position"] == "end" or s["kind"] in ("chs", "free"):
            continue
        tw, tf, bf = s["webThickness"], s["flangeThickness"], s["width"]
        hw = ld["webHeight"] if ld["webHeight"] > 0 else s["height"] - 2 * tf
        al = alloy_for_section(mat["designation"], s)
        if al is None or min(tw, tf, hw) <= 0:
            continue
        fo = al["fo"]
        ss = ld["bearingLength"] if ld["bearingLength"] > 0 else m["bearingLength"]
        m1 = fo * bf / (fo * tw)
        frd = math.inf
        for a in web_spacings(m["length"], [v["position"] for v in stiffeners if v["girderId"] == m["id"]]):
            kf = k0 + k1 * (hw / a) ** 2 if a > 0 else k0
            fcr = c_cr * kf * E * tw ** 3 / hw

            def reach(m2):
                l = ss + 2 * tf * (1 + math.sqrt(m1 + m2))
                return min(l, a) if a > 0 else l

            lam = lambda l: math.sqrt(l * tw * fo / fcr)
            ly = reach(0.0)
            if lam(ly) > 0.5:
                ly = reach(m2f * (hw / tf) ** 2)
            chi = min(chi0 / lam(ly), 1.0)
            frd = min(frd, fo * chi * ly * tw / GM1)
        out.append({"id": f"en1999.6.7.5.web.{ld['id']}", "utilization": abs(ld["load"]) / frd})


WEB_STIFFENER = {"strip": 15.0, "short": 1.5, "long": 0.75, "aspect": math.sqrt(2.0), "length": 0.75}


def stiffener_section(tw, width, thickness, two, strip, n=4000):
    """🦴️ Area, centroid offset and second moment about the centroidal axis along the web of the web strip plus the flat bar(s), integrated over thin slices across the thickness direction (not the closed form of the Rust)."""
    pieces = [(-tw / 2, tw / 2, strip), (tw / 2, tw / 2 + width, thickness)] + ([(-tw / 2 - width, -tw / 2, thickness)] if two else [])
    slices = []
    for lo, hi, breadth in pieces:
        dy = (hi - lo) / n
        slices += [(lo + (k + 0.5) * dy, breadth * dy) for k in range(n)]
    area = sum(a for _, a in slices)
    yc = sum(y * a for y, a in slices) / area
    return area, yc, sum(a * (y - yc) ** 2 for y, a in slices) + sum(breadth * (hi - lo) ** 3 / 12 / n ** 2 for lo, hi, breadth in pieces)


def stiffener_checks(stiffeners, members, sections, materials, out):
    """🦴️ Transverse web stiffeners (EN 1999-1-1 6.7.6 structure with the recalled EN 1993-1-5 9.1 and 9.3.3 constants, unverified): stiffness I_st,min / I_st and strength N_Ed / N_b,Rd of the stiffener with its web strip as a strut."""
    k = WEB_STIFFENER
    for st in stiffeners:
        m = members.get(st["girderId"])
        s = sections.get(m["sectionId"]) if m else None
        mat = materials.get(m["materialId"]) if m else None
        if not (m and s and mat) or s["kind"] in ("chs", "tube", "free") or st["sides"] not in ("one", "two"):
            continue
        tw, tf = s["webThickness"], s["flangeThickness"]
        hw = s["height"] - 2 * tf
        al = alloy_for_section(mat["designation"], s)
        x = st["position"]
        if al is None or min(tw, tf, hw) <= 0 or not 0 < x < m["length"]:
            continue
        fo = al["fo"]
        eps = math.sqrt(250e6 / fo)
        others = [o["position"] for o in stiffeners if o["girderId"] == st["girderId"]]
        before = max([v for v in others if v < x] + [0.0])
        after = min([v for v in others if v > x] + [m["length"]])
        a_min, a_max = min(x - before, after - x), max(x - before, after - x)
        area, yc, inertia = stiffener_section(tw, st["width"], st["thickness"], st["sides"] == "two", 2 * k["strip"] * eps * tw + st["thickness"])
        i_min = k["short"] * hw ** 3 * tw ** 3 / a_min ** 2 if a_min / hw < k["aspect"] else k["long"] * hw * tw ** 3
        out.append({"id": f"en1999.6.7.6.web-stiffness.{st['id']}", "utilization": i_min / inertia})
        combos = uls_combos(m)
        v_ed = max(abs(e[2]) for _, e in combos)
        f_loc = max(abs(e[6]) for _, e in combos)
        kt = 5.34 + 4 * (hw / a_max) ** 2 if a_max >= hw else 4.0 + 5.34 * (hw / a_max) ** 2
        lw = 0.35 * (hw / tw) * math.sqrt(fo / E) * math.sqrt(5.34 / kt)
        rho_v = 1.0 if lw <= 0.83 else 0.83 / lw
        n_ed = max(0.0, v_ed - rho_v * hw * tw * fo / (math.sqrt(3.0) * GM1)) + f_loc
        alpha, lam0 = (0.20, 0.10) if al["a"] else (0.32, 0.0)
        chi = chi_ap(lam_bar(k["length"] * hw, inertia, area, fo), alpha, lam0)
        out.append({"id": f"en1999.6.7.6.web-strength.{st['id']}", "utilization": n_ed / (chi * area * fo / GM1)})


# ---------------------------------------------------------------- 1-4, 1-5
def rib_sections(rib, t, k, n=2000):
    """🪗 Rib per pitch by slicing the mid-lines of the plates into n strips each: area, centroid above the bottom mid-line, second moment with the exact own inertia of every strip and the outer-fibre moduli."""
    p, top, bot, h = rib
    proj = (p - top - bot) / 2
    s_w = math.hypot(h, proj)
    cos_w, sin_w = proj / s_w, h / s_w
    strips = []
    for length, ti, cos_a, sin_a, z0, z1 in ((top, k[0] * t, 1.0, 0.0, h, h), (bot, k[2] * t, 1.0, 0.0, 0.0, 0.0), (s_w, k[1] * t, cos_w, sin_w, 0.0, h), (s_w, k[1] * t, cos_w, sin_w, 0.0, h)):
        ds = length / n
        for i in range(n):
            z = z0 + (z1 - z0) * (i + 0.5) / n
            strips.append((ti * ds, z, (ds * ti ** 3 * cos_a ** 2 + ti * ds ** 3 * sin_a ** 2) / 12))
    area = sum(a for a, _, _ in strips)
    zc = sum(a * z for a, z, _ in strips) / area
    inertia = sum(a * (z - zc) ** 2 + own for a, z, own in strips)
    return area, zc, inertia, inertia / (h - zc + t / 2), inertia / (zc + t / 2)


def rib_effective(rib, t, fo, a, welded):
    """🪗 Effective rib under a sagging moment: fixed point of the thickness factors of the compressed flange (b/t) and the web (η·s_w/t, η from the stress ratio of its mid-line ends)."""
    p, top, bot, h = rib
    s_w = math.hypot(h, (p - top - bot) / 2)
    eps = math.sqrt(250e6 / fo)
    k = [1.0, 1.0, 1.0]
    for _ in range(60):
        _, zc, _, _, _ = rib_sections(rib, t, k, 400)
        psi_ = -zc / (h - zc) if h - zc > 1e-12 else -1.0
        nk = [rho_c(top / t / eps, a, False, welded), rho_c(eta_of(psi_) * s_w / t / eps, a, False, welded), 1.0]
        done = max(abs(x - y) for x, y in zip(nk, k)) < 1e-12
        k = nk
        if done:
            break
    return k, rib_sections(rib, t, k)


def rib_resistances(rib, t, fo, a, welded):
    """🪗 Per pitch: M_Rd sagging and hogging, effective inertia, N_Rd and V_Rd of the rib (unverified web shear rule of the registry rows)."""
    p, top, bot, h = rib
    s_w = math.hypot(h, (p - top - bot) / 2)
    eps = math.sqrt(250e6 / fo)
    out = {}
    for sense, r in ("sag", rib), ("hog", (p, bot, top, h)):
        _, (area, zc, inertia, wt, wb) = rib_effective(r, t, fo, a, welded)
        out[sense] = (min(wt, wb) * fo / GM1, inertia)
    rho = lambda b_: rho_c(b_ / t / eps, a, False, welded)
    out["n"] = t * (rho(top) * top + rho(bot) * bot + 2 * rho(s_w) * s_w) * fo / GM1
    lam = ndp("sheet-web-shear-factor") * s_w / t * math.sqrt(fo / E)
    rho_v = 1.0 if lam <= ndp("sheet-web-shear-limit") else ndp("sheet-web-shear-limit") / lam
    out["v"] = 2 * h * t * rho_v * fo / (math.sqrt(3) * GM1)
    return out


def sheet_checks(sh, mat, out, profile=None):
    al = alloy(mat["designation"], sh["thickness"])
    if al is None:
        out.append({"id": f"en1999.3.2.alloy.{sh['id']}", "utilization": 1.0})
        return
    fo, a = al["fo"], al["a"]
    b, t = sh["width"], sh["thickness"]
    p = proxy(sh["id"], sh["span"], sh["actions"])
    lib = {}
    rib = None
    if profile and profile["top"] > 0 and profile["bottom"] > 0 and profile["height"] > 0 and profile["pitch"] >= profile["top"] + profile["bottom"]:
        rib = (profile["pitch"], profile["top"], profile["bottom"], profile["height"])
        rr = rib_resistances(rib, t, fo, a, sh["welded"])
        scale = b / rib[0]

    def put(cid, u):
        key = f"en1999.1-4.{cid}.{sh['id']}"
        lib[key] = max(lib.get(key, -1e18), u)

    for name, e in uls_combos(p):
        n, m, v = abs(e[0]), math.hypot(e[3], e[4]), math.hypot(e[1], e[2])
        rh = al["rho_o"] if sh["welded"] else 1.0
        if rib is None:
            be = b / t / math.sqrt(250e6 / fo)
            lim = limits(a, False, sh["welded"])
            rho = rho_c(be, a, False, sh["welded"])
            nrd = rho * b * t * rh * fo / GM1
            mrd = rh * b * t * t / 6 * fo / GM1
            vrd = b * t * fo / (math.sqrt(3) * GM1)
        else:
            widest = max(rib[1], rib[2], math.hypot(rib[3], (rib[0] - rib[1] - rib[2]) / 2))
            be = widest / t / math.sqrt(250e6 / fo)
            lim = limits(a, False, sh["welded"])
            nrd = rr["n"] * scale * rh
            mrd = min(rr["sag"][0], rr["hog"][0]) * scale * rh
            vrd = rr["v"] * scale
        put("axial", n / nrd)
        put("bend", m / mrd)
        put("shear", v / vrd)
        put("nm", n / nrd + m / mrd)
        put("local", be / lim[2])
    wk = abs(sum(a_["gKLine"] + a_["qKLine"] for a_ in sh["actions"] if a_["kind"] != "fire"))
    inertia = b * t ** 3 / 12 if rib is None else min(rr["sag"][1], rr["hog"][1]) * scale
    put("defl", 5 * wk * sh["span"] ** 4 / (384 * E * inertia) / (sh["span"] / DEFL))
    for k_, v in lib.items():
        out.append({"id": k_, "utilization": v})


def shell_checks(sl, mat, out):
    al = alloy(mat["designation"], sl["thickness"])
    if al is None:
        out.append({"id": f"en1999.3.2.alloy.{sl['id']}", "utilization": 1.0})
        return
    fo = al["fo"] * (al["rho_o"] if sl["welded"] else 1.0)
    r, t, L = sl["radius"], sl["thickness"], sl["length"]
    p = proxy(sl["id"], L, sl["actions"])
    lib = {}

    def put(cid, u):
        key = f"en1999.1-5.{cid}.{sl['id']}"
        lib[key] = max(lib.get(key, -1e18), u)

    w = L / math.sqrt(r * t)
    if w < 1.7:
        cx = max(1.36 - 1.83 / w + 2.07 / w ** 2, 0.1)
    elif w <= 0.5 * r / t:
        cx = 1.0
    else:
        cx = max(1 + 0.2 * (1 - 2 * w / (r / t)), 0.6)
    sx_cr = 0.605 * E * cx * t / r
    st_cr = 0.92 * E * (t / r) / w
    tau_cr = 0.75 * E * (t / r) / math.sqrt(w)
    dw = math.sqrt(r / t) * t / {"A": 40.0, "C": 16.0}.get(sl["fabricationClass"], 25.0)
    ax = 0.62 / (1 + 1.91 * (dw / t) ** 1.44)

    def chi(lam, alpha, lam0, beta=0.6, eta=1.0):
        if lam <= lam0:
            return 1.0
        lp = math.sqrt(alpha / (1 - beta))
        if lam >= lp:
            return alpha / lam ** 2
        return 1 - beta * ((lam - lam0) / (lp - lam0)) ** eta

    cxv = chi(math.sqrt(fo / sx_cr), ax, 0.2)
    ctv = chi(math.sqrt(fo / st_cr), 0.65, 0.4)
    cuv = chi(math.sqrt(fo / (math.sqrt(3) * tau_cr)), 0.65, 0.4)
    rx, rt, ru = cxv * fo / GM1, ctv * fo / GM1, cuv * fo / (math.sqrt(3) * GM1)
    for name, e in uls_combos(p):
        sx, st, tau = abs(e[0]), math.hypot(e[3], e[4]), math.hypot(e[1], e[2])
        put("buckle", sx / rx)
        put("ring", st / rt)
        put("shear", tau / ru)
        kx, kt, ku = 1.25 + 0.75 * cxv, 1.25 + 0.75 * ctv, 1.75 + 0.25 * cuv
        ki = (cxv * ctv) ** 2
        ax_, at_ = sx / rx, st / rt
        put("int", max(ax_ ** kx - ki * ax_ * at_ + at_ ** kt + (tau / ru) ** ku, 0.0))
    for k_, v in lib.items():
        out.append({"id": k_, "utilization": v})


def ring_checks(rings, shells, out):
    """⭕ Ring stiffeners (EN 1999-1-5 5, unverified width factor and margin): position inside the shell and flexural stability w_Ed against w_cr = 3·E·I/r³ of the ring with its effective shell strip b_e = factor·sqrt(r·t)."""
    factor = ndp("ring-stiffener.effective-width-factor")
    margin = ndp("ring-stiffener.required-inertia-factor")
    gamma_r = 1.0 if margin is None else margin
    for ring in rings:
        sl = shells.get(ring["shellId"])
        if sl is None:
            continue
        r, t, length = sl["radius"], sl["thickness"], sl["length"]
        x = ring["x"]
        inside = 0 < x < length
        out.append({"id": f"en1999.1-5.stiffener-position.{ring['id']}", "utilization": abs(x) / max(length, 1e-9), "status": "pass" if inside else "fail"})
        b_e = (0.0 if factor is None else factor) * math.sqrt(r * t)
        a_s, a_b = b_e * t, ring["depth"] * ring["thickness"]
        z_b = 0.5 * t + 0.5 * ring["depth"]
        z_c = a_b * z_b / max(a_s + a_b, 1e-30)
        inertia = b_e * t ** 3 / 12 + a_s * z_c ** 2 + ring["thickness"] * ring["depth"] ** 3 / 12 + a_b * (z_b - z_c) ** 2
        at = sorted({o["x"] for o in rings if o["shellId"] == sl["id"] and 0 < o["x"] < length})
        before = max([v for v in at if v < x] + [0.0])
        after = min([v for v in at if v > x] + [length])
        sigma = max(math.hypot(e[3], e[4]) for _, e in uls_combos(proxy(sl["id"], length, sl["actions"])))
        w_ed = sigma * t / max(r, 1e-9) * 0.5 * (after - before)
        w_cr = 3 * E * inertia / max(r ** 3, 1e-18)
        u = gamma_r * w_ed / max(w_cr, 1e-9)
        out.append({"id": f"en1999.1-5.stiffener.{ring['id']}", "utilization": u, "status": ("unverifiedBasis" if LIMIT.within_limit(u) else "fail")})


def torsion_modulus(s):
    tf, tw = s["flangeThickness"], s["webThickness"]
    k = s["kind"]
    if k in ("tube", "chs"):
        d = max(s["outerDiameter"], s.get("width", 0.0))
        t = max(tw, tf)
        return 2 * math.pi * ((d - t) / 2) ** 2 * t
    if k in ("box", "rhs"):
        return 2 * max(s["width"] - tw, 0.0) * max(s["height"] - tf, 0.0) * min(tf, tw)
    return geometry(s)["it"] / max(tf, tw, 1e-12)


def patch_rd(hw, tw, bf, tf, ss, a_sp, fo):
    if hw <= 0 or tw <= 0:
        return 0.0
    kf = 6.0 + (2.0 * (hw / a_sp) ** 2 if a_sp > 0 else 0.0)
    fcr = 0.9 * kf * E * tw ** 3 / hw
    m1 = bf / tw
    cap = a_sp if a_sp > 0 else math.inf

    def ly(m2):
        return min(ss + 2 * tf * (1 + math.sqrt(m1 + m2)), cap)

    l = ly(0.0)
    lf = math.sqrt(l * tw * fo / fcr)
    if lf > 0.5:
        l = ly(0.02 * (hw / max(tf, 1e-12)) ** 2)
        lf = math.sqrt(l * tw * fo / fcr)
    chi = min(0.5 / max(lf, 1e-9), 1.0)
    return fo * chi * l * tw / GM1


def bolt_shear(grade, d, gm2):
    f = FU[grade.lower()]
    if grade.lower().startswith("rivet-"):
        return 0.6 * f * math.pi / 4 * d0_of(d) ** 2 / gm2
    return (0.5 if grade == "10.9" else 0.6) * f * stress_area(d) / gm2


def bolt_bearing(e1, p1, e2, d, t, fu, fub, gm2):
    d0 = d0_of(d)
    ab = max(0.0, min(e1 / (3 * d0), p1 / (3 * d0) - 0.25, fub / fu, 1.0))
    return max(0.0, min(2.8 * e2 / d0 - 1.7, 2.5)) * ab * fu * d * t / gm2


def bolt_tension(grade, d, gm2):
    return 0.9 * FU[grade.lower()] * stress_area(d) / gm2


def sn_corner_d(dc, m1):
    return dc * (NC / ND) ** (1 / m1)


def sn_corner_l(dc, m1, m2):
    return sn_corner_d(dc, m1) * (ND / NL) ** (1 / m2)


def damage_block(ds, n, dc, m1, m2, gff, gmf):
    e = endurance(ds * gff * gmf, dc, m1, m2)
    return 0.0 if math.isinf(e) else n / e


def sigma_x_cr(r, t, L):
    w = L / math.sqrt(r * t)
    cx = max(1.36 - 1.83 / w + 2.07 / w ** 2, 0.1) if w < 1.7 else 1.0 if w <= 0.5 * r / t else max(1 + 0.2 * (1 - 2 * w / (r / t)), 0.6)
    return 0.605 * E * cx * t / r


VECTORS = {
    "rho_c": lambda b, a, o, w: rho_c(b, a, o, w),
    "n_cr": lambda i, L: math.pi ** 2 * E * i / L ** 2,
    "chi": chi_ap,
    "m_cr": lambda c1, iz, iw, it, L: c1 * math.pi ** 2 * E * iz / L ** 2 * math.sqrt(iw / iz + L ** 2 * G * it / (math.pi ** 2 * E * iz)),
    "bolt_shear": bolt_shear,
    "bolt_bearing": bolt_bearing,
    "bolt_tension": bolt_tension,
    "delta_sigma_d": sn_corner_d,
    "delta_sigma_l": sn_corner_l,
    "cycles": endurance,
    "damage": damage_block,
    "fire_step": lambda th0, amv, dur: member_temperature(amv, dur, th0) - th0,
    "sigma_x_rcr": sigma_x_cr,
    "haz_width": haz_width,
    "shear_rho": lambda r, fo, hw, a: (lambda lw: 1.0 if lw <= 0.83 else 0.83 / lw)(0.35 * r * math.sqrt(fo / E) * math.sqrt(5.34 / ((5.34 + 4 * (hw / a) ** 2 if a >= hw else 4.0 + 5.34 * (hw / a) ** 2) if a > 0 else 5.34))),
    "haz_width_for": lambda t, process, tj: haz_width_for({"thickness": t, "weldProcess": process, "weldTJunction": tj}),
    "patch_rd": patch_rd,
    "stress_area": stress_area,
}


def run_vectors(path):
    """🧪️ Evaluates the language-agnostic worked examples with this implementation; returns the list of mismatches."""
    failures = []
    for v in json.load(open(path, encoding="utf-8"))["vectors"]:
        got = VECTORS[v["fn"]](*v["args"])
        if abs(got - v["expected"]) > v["relTol"] * max(abs(v["expected"]), 1e-12):
            failures.append(f"{v['id']}: expected {v['expected']} got {got}")
    return failures


def evaluate(doc):
    annex = doc.get("annex", "en")
    RELIABILITY["k_fi"] = K_FI[doc["reliabilityClass"]]
    secs = {s["id"]: s for s in doc.get("sections", [])}
    mats = {m["id"]: m for m in doc.get("materials", [])}
    mems = {m["id"]: m for m in doc.get("members", [])}
    out = []
    runs = doc.get("weldRuns", [])
    vertices = doc.get("outlineVertices", [])
    for m in doc.get("members", []):
        s, mat = secs.get(m["sectionId"]), mats.get(m["materialId"])
        if s and mat and s["kind"] == "free":
            free_member_checks(m, s, mat, annex, vertices, out)
        elif s and mat:
            rm, rs = apply_runs(m, s, [r for r in runs if r["memberId"] == m["id"]])
            member_checks(rm, rs, mat, annex, out, [v["position"] for v in doc.get("transverseStiffeners", []) if v["girderId"] == m["id"]])
    for c in doc.get("connections", []):
        mat = mats.get(c["materialId"])
        if mat:
            connection_checks(c, mat, mems.get(c["memberId"]), annex, out)
    for f in doc.get("fireScenarios", []):
        m = mems.get(f["memberId"])
        s = secs.get(m["sectionId"]) if m else None
        mat = mats.get(m["materialId"]) if m else None
        if m and s and mat:
            fire_checks(f, m, s, mat, out, next((q for q in doc.get("fireInsulations", []) if q["fireId"] == f["id"]), None))
    for fd in doc.get("fatigueDetails", []):
        fatigue_checks(fd, mems.get(fd["memberId"]), annex, out)
    categories = doc.get("fatigueCategories", [])
    keyed = {(c["memberId"], c["detailCategory"]) for c in categories}
    fatigue_spectra([d for d in doc.get("fatigueDetails", []) if (d["memberId"], d["detailCategory"]) not in keyed], mems, annex, out)
    category_checks(categories, doc.get("fatigueDetails", []), mems, annex, out)
    patch_checks(doc.get("patchLoads", []), mems, secs, mats, out, doc.get("transverseStiffeners", []))
    stiffener_checks(doc.get("transverseStiffeners", []), mems, secs, mats, out)
    for sh in doc.get("coldFormed", []):
        mat = mats.get(sh["materialId"])
        if mat:
            profile = next(({"pitch": q["pitch"], "top": q["topFlange"], "bottom": q["bottomFlange"], "height": q["webHeight"]} for q in doc.get("sheetProfiles", []) if q["sheetId"] == sh["id"]), None)
            sheet_checks(sh, mat, out, profile)
    fire_ids = {c["id"] for c in doc.get("fireScenarios", [])}
    for q in doc.get("fireInsulations", []):
        if q["fireId"] not in fire_ids:
            out.append({"id": f"en1999.1-2.insulation-ref.{q['id']}", "utilization": 1.0, "status": "fail"})
        else:
            ok = q["thickness"] >= 0 and (q["thickness"] <= 0 or all(q[k] > 0 for k in ("conductivity", "specificHeat", "density")))
            out.append({"id": f"en1999.1-2.insulation.{q['id']}", "utilization": 0.0 if ok else 2.0, "status": "pass" if ok else "fail"})
    sheet_ids = {c["id"] for c in doc.get("coldFormed", [])}
    for q in doc.get("sheetProfiles", []):
        if q["sheetId"] not in sheet_ids:
            out.append({"id": f"en1999.1-4.profile-ref.{q['id']}", "utilization": 1.0, "status": "fail"})
        else:
            ok = q["topFlange"] > 0 and q["bottomFlange"] > 0 and q["webHeight"] > 0 and q["pitch"] >= q["topFlange"] + q["bottomFlange"]
            out.append({"id": f"en1999.1-4.profile.{q['id']}", "utilization": (q["topFlange"] + q["bottomFlange"]) / max(q["pitch"], 1e-9), "status": "pass" if ok else "fail"})
    for sl in doc.get("shells", []):
        mat = mats.get(sl["materialId"])
        if mat:
            shell_checks(sl, mat, out)
    ring_checks(doc.get("ringStiffeners", []), {s_["id"]: s_ for s_ in doc.get("shells", [])}, out)
    return {"checks": out}


if __name__ == "__main__":
    if len(sys.argv) > 2 and sys.argv[1] == "--vectors":
        bad = run_vectors(sys.argv[2])
        print(json.dumps({"failures": bad}))
        sys.exit(1 if bad else 0)
    args = [a for a in sys.argv[1:] if a != "--json"]
    doc = json.load(open(args[0], encoding="utf-8") if args else sys.stdin)
    print(json.dumps(evaluate(doc)))
