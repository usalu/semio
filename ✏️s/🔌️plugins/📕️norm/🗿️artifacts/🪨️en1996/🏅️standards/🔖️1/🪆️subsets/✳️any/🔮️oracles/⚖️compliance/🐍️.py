#!/usr/bin/env python3
"""🪨️ Independent EN 1996 oracle: scipy/shapely/numpy models and independently keyed tables.

Reads one JSON document on stdin and prints one JSON document:
  {"mode": "catalogue", "cases": [{"id", "entry", "annex", "num", "text", "vec"}]}
  {"mode": "snapshot", "snapshot": {...}, "report": {...}}
No file is read except the sibling tables file and no file is written.
"""
from __future__ import annotations

import importlib.util
import pathlib
import itertools
import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy import integrate, interpolate, optimize
from shapely.geometry import LineString, Polygon

T = json.loads((Path(__file__).parent / "📚️tables.json").read_text())
_FACTORS_PATH = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py").is_file())
_SPEC = importlib.util.spec_from_file_location("action_factors", _FACTORS_PATH)
FACTORS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(FACTORS)
E_S = 200e9
EPS_MU = 0.0035
BLOCK = 0.8
CREEP_FINAL = {"clay": 1.5, "aerated": 1.5, "calcium_silicate": 2.0, "concrete": 2.0}
MAT = {"clay": "clay", "calcium_silicate": "calcium_silicate", "aerated": "aerated", "concrete": "concrete"}


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


def delta(h_m, w_m):
    f = interpolate.RegularGridInterpolator(
        (T["shapeFactor"]["heightsMm"], T["shapeFactor"]["leastDimensionsMm"]), np.array(T["shapeFactor"]["delta"]), bounds_error=False, fill_value=None
    )
    h = min(max(h_m * 1000, 50), 250)
    w = min(max(w_m * 1000, 50), 250)
    return float(f([[h, w]])[0])


def fk_row(annex, mortar, material, group):
    row = T["fk"][annex][mortar]
    return row[material][group - 1], row["alpha"], row["beta"]


def f_k_mpa(annex, material, group, mortar, fb_n, fm):
    k, a, b = fk_row(annex, mortar, material, group)
    fm_eff = min(fm, 20.0)
    if group != 1:
        fm_eff = min(fm_eff, 2 * fb_n)
    return min(k * fb_n**a * fm_eff**b, 0.8 * fb_n)


def mortar_class_mpa(fm):
    return max([c for c in (1.0, 2.5, 5.0, 10.0, 15.0, 20.0) if c <= fm + 1e-9] or [1.0])


def fvk0_mpa(annex, material, mortar, fm):
    t = T["fvk0Mpa"]
    m = mortar_class_mpa(fm)
    if annex == "de":
        d = t["de"]
        if mortar == "tl":
            return d["tl"]
        if mortar == "lw":
            return d["lw"]
        return d["gp"][0] if m >= 15 else d["gp"][1] if m >= 10 else d["gp"][2] if m >= 5 else d["gp"][3]
    row = t["en"]["clay"] if material == "clay" else t["en"]["other"]
    if mortar == "lw":
        return row["lw"]
    if mortar == "tl":
        return row["tl"] if material == "clay" else row["tl"][material]
    return row["gp"][0] if m >= 10 else row["gp"][1] if m >= 2.5 else row["gp"][2]


def fvlt_mpa(annex, group, fb, sigma):
    if annex == "en":
        return 0.065 * fb
    share = T["fvltShare"]["solid"] if group == 1 else T["fvltShare"]["perforated"] if group == 2 else T["fvltShare"]["hollow"]
    fbt = share * fb
    return 0.45 * fbt * math.sqrt(1 + max(sigma, 0) / fbt)


def fxk_mpa(direction, fm, material):
    pair = T["fxkMpa"][material]["fxk%d" % direction]
    return pair[1] if fm >= 5 else pair[0]


def gamma_m(annex, cls, accidental=False):
    if annex == "de" and accidental:
        return T["gammaM"]["deAccidental"]
    return T["gammaM"][annex][int(cls) - 1]


def rho_n(sides, rho2, h, L):
    if sides < 2:
        return 2.0
    if sides == 2:
        return rho2
    if sides == 3:
        return 1.5 * L / h if h > 3.5 * L else max(rho2 / (1 + (rho2 * h / (3 * L)) ** 2), 0.3)
    return 0.5 * L / h if h > 1.15 * L else rho2 / (1 + (rho2 * h / L) ** 2)


def phi_m(slender, emk, t, ke):
    emk = max(emk, 0.05 * t)
    a1 = 1 - 2 * emk / t
    if a1 <= 0:
        return 0.0
    lam = slender / math.sqrt(ke)
    u = (lam - 0.063) / (0.73 - 1.17 * emk / t)
    return a1 * math.exp(-u * u / 2)


def phi_i(e, t, hef):
    e = max(abs(e) + hef / 450, 0.05 * t)
    return min(max(1 - 2 * e / t, 0.0), 1.0)


def beta_c(a1, hc, ab, aef):
    if ab > 0.45 * aef:
        return 1.0
    return max(1.0, min((1 + 0.3 * a1 / hc) * (1.5 - 1.1 * ab / aef), 1.25 + a1 / (2 * hc), 1.5))


def yield_line_pressure(code, L, H, m1, m2):
    """🧮️ Johansen work balance on explicit planar facets.

    External work: facet area times the deflection at the facet centroid. Internal work: every shared edge carries m_n * rotation * length with
    m_n = m1 cos^2(phi) + m2 sin^2(phi); every edge of a facet on a fixed support carries the hogging work of the same form. Edge code is top, bottom, left, right with F, S, X.
    """
    edges = dict(zip(("top", "bottom", "left", "right"), code.upper()))

    def pattern(Lx, Hy, ma, mb, ed, transpose):
        # ridge parallel to the x axis; (ma, mb) are the capacities of lines parallel to x and y
        def build(y, al, ar):
            free = lambda k: ed[k] == "F"
            facets = []
            yb = Hy if free("top") else (0.0 if free("bottom") else y)
            if not free("bottom"):
                x0 = al if not free("left") else 0.0
                x1 = Lx - ar if not free("right") else Lx
                facets.append(([(0, 0), (Lx, 0), (x1, yb), (x0, yb)], (0.0, 1.0 / yb), 0.0))
            if not free("top"):
                x0 = al if not free("left") else 0.0
                x1 = Lx - ar if not free("right") else Lx
                facets.append(([(0, Hy), (Lx, Hy), (x1, yb), (x0, yb)], (0.0, -1.0 / (Hy - yb)), Hy / (Hy - yb)))
            if not free("left"):
                pts = [(0, 0), (0, Hy), (al, yb)] if not (free("top") or free("bottom")) else [(0, 0), (0, Hy), (al, Hy if free("top") else 0.0)]
                facets.append((pts, (1.0 / al, 0.0), 0.0))
            if not free("right"):
                pts = [(Lx, 0), (Lx, Hy), (Lx - ar, yb)] if not (free("top") or free("bottom")) else [(Lx, 0), (Lx, Hy), (Lx - ar, Hy if free("top") else 0.0)]
                facets.append((pts, (-1.0 / ar, 0.0), Lx / ar))
            return facets

        def energy(params):
            k = 0
            y = Hy / 2
            if ed["top"] != "F" and ed["bottom"] != "F":
                y = params[k]
                k += 1
            al = ar = 0.0
            if ed["left"] != "F":
                al = params[k]
                k += 1
            if ed["right"] != "F":
                ar = params[k]
            if al + ar > Lx or any(v <= 0 for v in params):
                return 1e30
            facets = build(y, al, ar)
            polys = [Polygon(f[0]) for f in facets]
            if any(p.area <= 1e-12 for p in polys):
                return 1e30
            ext = sum(p.area * (f[1][0] * p.centroid.x + f[1][1] * p.centroid.y + f[2]) for p, f in zip(polys, facets))
            grads = [np.array(f[1]) for f in facets]
            internal = 0.0
            for i in range(len(polys)):
                for j in range(i + 1, len(polys)):
                    sh = polys[i].boundary.intersection(polys[j].boundary)
                    if isinstance(sh, LineString) and sh.length > 1e-12:
                        (xa, ya), (xb, yb2) = sh.coords[0], sh.coords[-1]
                        ph = math.atan2(yb2 - ya, xb - xa) if False else math.atan2(yb2 - ya, xb - xa)
                        internal += (ma * math.cos(ph) ** 2 + mb * math.sin(ph) ** 2) * float(np.linalg.norm(grads[i] - grads[j])) * sh.length
            lines = {"bottom": ((0, 0), (Lx, 0)), "top": ((0, Hy), (Lx, Hy)), "left": ((0, 0), (0, Hy)), "right": ((Lx, 0), (Lx, Hy))}
            for name, seg in lines.items():
                if ed[name] != "X":
                    continue
                line = LineString(seg)
                for p, g in zip(polys, grads):
                    sh = p.boundary.intersection(line)
                    if isinstance(sh, LineString) and sh.length > 1e-12:
                        ph = math.atan2(seg[1][1] - seg[0][1], seg[1][0] - seg[0][0])
                        internal += (ma * math.cos(ph) ** 2 + mb * math.sin(ph) ** 2) * float(np.linalg.norm(g)) * sh.length
            return internal / ext

        if ed["top"] == "F" and ed["bottom"] == "F":
            return math.inf
        lo, hi = [], []
        if ed["top"] != "F" and ed["bottom"] != "F":
            lo.append(1e-3 * Hy)
            hi.append((1 - 1e-3) * Hy)
        for side in ("left", "right"):
            if ed[side] != "F":
                lo.append(1e-3 * Lx)
                hi.append(Lx)
        if not lo:
            return energy([])
        bounds = list(zip(lo, hi))
        r = optimize.differential_evolution(energy, bounds, seed=1, tol=1e-10, maxiter=400, polish=True, popsize=30)
        return float(r.fun)

    horizontal = pattern(L, H, m1, m2, edges, False)
    swapped = {"top": edges["right"], "bottom": edges["left"], "left": edges["bottom"], "right": edges["top"]}
    vertical = pattern(H, L, m2, m1, swapped, True)
    return min(horizontal, vertical)


def mn_capacity(b, depth, fd, fyd, bars, n_ed):
    """🧱️ Fibre integration of the 0.8 x stress block; neutral axis found with Brent's method."""
    ys = (np.arange(4000) + 0.5) * depth / 4000
    dy = depth / 4000

    def at(c):
        inside = ys <= BLOCK * c
        cm = float(np.sum(inside) * dy * b * fd)
        mm = float(np.sum(inside * (depth / 2 - ys)) * dy * b * fd)
        n, m = cm, mm
        for y, a in bars:
            s = min(max(E_S * EPS_MU * (c - y) / c, -fyd), fyd)
            n += s * a
            m += s * a * (depth / 2 - y)
        return n, m

    lo, hi = 1e-4 * depth, 60 * depth
    if n_ed < at(lo)[0] or n_ed > at(hi)[0]:
        return 0.0
    c = optimize.brentq(lambda c: at(c)[0] - n_ed, lo, hi, xtol=1e-12)
    return at(c)[1]


def fire_mm(rei, material):
    classes = T["fireMm"]["classes"]
    idx = next((i for i, c in enumerate(classes) if rei <= c), len(classes) - 1)
    return T["fireMm"][material][idx] / 1000


def tributary(x, lo, hi):
    """🧭️ Tributary fractions of the wall lines `x` of a flexible diaphragm between its edges `lo` and `hi`: the cell boundaries are the midpoints between neighbouring lines, numpy.diff of the sorted boundaries, ordered back to the input order."""
    order = np.argsort(x)
    xs = np.asarray(x, dtype=float)[order]
    bounds = np.concatenate(([lo], 0.5 * (xs[1:] + xs[:-1]), [hi]))
    out = np.empty(len(xs))
    out[order] = np.diff(bounds) / (hi - lo)
    return out


def bracing(n, v, s=None):
    """🧭️ Distribution of the storey shear over parallel walls: a rigid diaphragm by stiffness and torsion (unknowns u and theta about the centre of stiffness from numpy.linalg.solve), a flexible one (EN 1998-1 4.3.1(4)) by tributary width without torsion, the fractions derived here from the wall lines and the diaphragm edges `edge_lo_m`, `edge_hi_m` when given and otherwise taken from the vector `trib`."""
    length, thick, x, vrd = (np.array(v[k]) for k in ("l_m", "t_m", "x_m", "vrd_n"))
    h, e_mod, v_ed = n["h_m"], n["e_pa"], n["v_ed_n"]
    ratio = T["bracing"]["shearModulusRatio"]
    k = 1.0 / (h**3 / (3 * e_mod * thick * length**3 / 12) + h / (ratio * e_mod * length * thick))
    xc = float(k @ x / k.sum())
    d = x - xc
    if (s or {}).get("diaphragm") == "flexible":
        trib = tributary(x, n["edge_lo_m"], n["edge_hi_m"]) if "edge_lo_m" in n else np.array(v["trib"])
        shares = v_ed * trib
        worst = int(np.argmax(shares / np.maximum(vrd, 1e-9)))
        return {"centre": xc, "governing-wall": float(max(vrd[worst], 1e-9))}
    ecc = n.get("e0_m", 0.0) + 0.05 * n.get("b_m", 0.0)
    stiffness = np.array([[k.sum(), 0.0], [0.0, float(k @ d**2)]])
    u, theta = np.linalg.solve(stiffness, np.array([v_ed, v_ed * ecc])) if stiffness[1, 1] > 0 else (v_ed / k.sum(), 0.0)
    shares = k * u + k * abs(theta) * np.abs(d)
    worst = int(np.argmax(shares / np.maximum(vrd, 1e-9)))
    return {"centre": xc, "governing-wall": float(max(vrd[worst], 1e-9))}


def arch(t, span, fd, samples):
    """🏹️ Three-hinge arch of a wall of thickness t and span: the block depth d from d (t - d) = t^2 / 8 by root finding, the thrust line as a parabola from d/2 at the supports to t - d/2 at mid-span and the lateral capacity from the statics of the hinge thrust N = f_d d."""
    d = float(optimize.brentq(lambda x: x * (t - x) - t * t / 8, 0.0, t / 2))
    rise = t - d
    points = [[span * i / samples, d / 2 + 4 * rise * (i / samples) * (1 - i / samples)] for i in range(samples + 1)]
    return {"block": d, "points": points, "q": 8 * fd * d * rise / span ** 2, "q_catalogue": fd * (t / span) ** 2}


def relocation(at, movable, limits):
    """📍️ Brute force over the millimetre grid: the move of one end of the interval with the largest overshoot that keeps both intervals of the moved support within their limits, the smallest move first."""
    ratios = [(at[k + 1] - at[k]) / limits[k] for k in range(len(limits))]
    k = max(range(len(limits)), key=lambda i: ratios[i])
    if LIMIT.within_limit(ratios[k]):
        return None
    best = None
    for j in (k, k + 1):
        if j in (0, len(at) - 1) or not movable[j]:
            continue
        for step in range(math.floor(at[j - 1] * 1000) + 1, math.ceil(at[j + 1] * 1000)):
            x = step / 1000
            if not at[j - 1] < x < at[j + 1]:
                continue
            moved = list(at)
            moved[j] = x
            if all(moved[i + 1] - moved[i] <= limits[i] + 1e-9 for i in (j - 1, j)):
                distance = abs(x - at[j])
                if best is None or distance < best[2] - 1e-12:
                    best = (j, x, distance)
    return None if best is None else {"index": best[0], "at": best[1]}


def geometry(case):
    kind = case["kind"]
    if kind == "arch":
        return arch(case["t_m"], case["span_m"], case["fd_pa"], case["samples"])
    if kind == "relocation":
        return relocation(case["at"], case["movable"], case["limits"])
    if kind == "remedy-m43":
        n = case["num"]
        at = [0.0] + case["pos_m"] + [n["l_m"]]
        side = max(n["col_a_side_m"], math.ceil(0.02 / n["col_a_other_m"] * 1000 - 1e-9) / 1000)
        return {"side": side, "move": relocation(at, [False] + [True] * len(case["pos_m"]) + [False], [4.0] * (len(at) - 1))}
    if kind == "remedy-m51":
        n = case["num"]
        s_max = n["anchor_rk_n"] / 1.5 / (n["w_ed_pa"] * n["h_trib_m"])
        at = [0.0] + case["pos_m"] + [n["l_m"]]
        limits = [s_max / 2] + [s_max] * (len(at) - 3) + [s_max / 2]
        return {"s_max": s_max, "move": relocation(at, [False] + [True] * len(case["pos_m"]) + [False], limits)}
    if kind == "remedy-m44":
        n = case["num"]
        required_area = (n["n_ed_n"] + n["p0_n"]) / n["fd_pa"]
        return {"thickness": math.ceil(n["t_m"] * required_area / n["a_m2"] * 1000 - 1e-9) / 1000}
    if kind == "remedy-m50":
        n = case["num"]
        share = n["t1_m"] ** 3 / (n["t1_m"] ** 3 + n["t2_m"] ** 3)
        return {"resistance": math.ceil(n["w_ed_pa"] * share / n["n_per_m2"] - 1e-9)}
    if kind == "yield":
        return {"pressure": yield_line_pressure(case["edges"], case["l_m"], case["h_m"], case["m1"], case["m2"])}
    if kind == "annex-e-classes":
        order = "FSX".index
        pairs = ["".join(sorted(p, key=order)) for p in itertools.combinations_with_replacement("FSX", 2)]
        classes = sorted(a + b for a in pairs for b in pairs)
        canonical = {code: "".join(sorted(code[:2], key=order)) + "".join(sorted(code[2:], key=order)) for code in case["codes"]}
        return {"classes": classes, "canonical": canonical}
    if kind == "annex-e-grid":
        mus = sorted({c[0] for c in case["cells"]})
        hls = sorted({c[1] for c in case["cells"]})
        lookup = {(c[0], c[1]): c[2] for c in case["cells"]}
        if len(lookup) != len(mus) * len(hls):
            return {"value": None}
        grid = np.array([[lookup[(m, h)] for h in hls] for m in mus])
        f = interpolate.RegularGridInterpolator((mus, hls), grid, method="linear", bounds_error=False, fill_value=None)
        m, h = case["mu"], case["hl"]
        inside = mus[0] <= m <= mus[-1] and hls[0] <= h <= hls[-1]
        return {"value": float(f([[m, h]])[0]) if inside else None}
    if kind == "annex-e-standin":
        w = yield_line_pressure(case["edges"], 1.0, case["hl"], 1.0, 1.0 / case["mu"])
        return {"alpha": 1.0 / w}
    if kind == "chase-limit":
        limits = chase_limits(case["orientation"], case["t_m"], case.get("length_m", float("inf")), case.get("top_m", float("inf")), case.get("storey_m", 1.0))
        return {"limit": None} if limits is None else {"limit": limits[0], "width": limits[1]}
    raise ValueError(kind)


def chase_limits(kind, t, length, top, storey):
    """🕳️ Table 8.1 (vertical chases and recesses) and Table 8.2 (horizontal chases) of EN 1996-1-1 8.6.2 from the independently keyed `chase` section of the table file: (depth, width or None) in metres, None below the tabulated wall thickness."""
    c = T["chase"]
    if t * 1000 + 1e-6 < c["tLowerMm"]:
        return None
    band = next((i for i, upper in enumerate(c["bandUpperMm"]) if t * 1000 <= upper + 1e-6), len(c["bandUpperMm"]))
    if kind == "horizontal":
        column = c["horizontalDepthShortMm"] if length * 1000 <= c["shortLengthMm"] + 1e-6 else c["horizontalDepthUnlimitedMm"]
        return column[band] / 1000, None
    depth, width = c["verticalDepthMm"][band] / 1000, c["verticalWidthMm"][band] / 1000
    low = c["lowThird"]
    if kind == "vertical" and t * 1000 + 1e-6 >= low["tMinMm"] and top <= low["heightRatio"] * storey + 1e-9:
        depth, width = max(depth, low["depthMm"] / 1000), max(width, low["widthMm"] / 1000)
    return depth, width


def chase_gap(kind_a, kind_b, wider, longer):
    """📐️ Required horizontal distance of two chases or recesses (Tables 8.1 and 8.2 notes)."""
    c = T["chase"]
    if kind_a == kind_b == "horizontal":
        return c["horizontalSpacingFactor"] * longer
    if "recess" in (kind_a, kind_b):
        return c["recessSpacingFactor"] * wider
    return c["verticalSpacingMm"] / 1000


def chase_opening_gap(kind, width):
    """🚪️ Required horizontal distance of a chase or recess to an opening."""
    c = T["chase"]
    return c["horizontalOpeningMm"] / 1000 if kind == "horizontal" else c["recessSpacingFactor"] * width if kind == "recess" else c["verticalSpacingMm"] / 1000


def catalogue(case):
    n, s, v, annex = case.get("num", {}), case.get("text", {}), case.get("vec", {}), case["annex"]
    mat = s.get("material", "clay")
    group = int(n.get("group", 1))
    mortar = s.get("mortar", "gp")
    gm = n.get("gamma_m", gamma_m(annex, n.get("class", 1), s.get("situation") in ("accidental", "transient", "seismic")))
    e = case["entry"]
    out = {}
    if e == "M-01":
        d = delta(n["unit_h_m"], n["unit_w_m"])
        out = {"delta": d, "fb": d * n["fb_decl_pa"]}
    elif e == "M-02":
        fk = f_k_mpa(annex, mat, group, mortar, n["fb_pa"] / 1e6, n["fm_pa"] / 1e6) * 1e6
        out = {"fk": fk, "fd": fk / gm}
    elif e == "M-03":
        fb, sg = n["fb_pa"] / 1e6, n.get("sigma_pa", 0) / 1e6
        f0 = fvk0_mpa(annex, mat, mortar, n["fm_pa"] / 1e6) * (0.5 if s.get("head_joint") == "unfilled" else 1.0)
        out = {"fvk": min(f0 + 0.4 * sg, fvlt_mpa(annex, group, fb, sg)) * 1e6}
    elif e == "M-04":
        fm = n["fm_pa"] / 1e6
        x1, x2 = fxk_mpa(1, fm, mat) * 1e6, fxk_mpa(2, fm, mat) * 1e6
        out = {"fxk1": x1, "fxk2": x2, "mu": x1 / x2}
    elif e == "M-06":
        young = n["fk_pa"] * (1000 if annex == "en" else 950)
        out = {"e": young, "e-lt": young / (1 + CREEP_FINAL[mat])}
    elif e == "M-10":
        rho2 = n.get("rho2", 0.75 if n.get("bearing_m", 0) >= 2 * n["t_m"] / 3 else 1.0)
        r = rho_n(int(n["sides"]), rho2, n["h_m"], n.get("l_m", 1e9))
        out = {"rho-n": r, "hef": r * n["h_m"]}
    elif e == "M-11":
        te = (n["t1_m"] ** 3 + n["t2_m"] ** 3) ** (1 / 3) if "t1_m" in n else n["t_m"]
        out = {"slenderness": n["hef_m"] / te}
    elif e == "M-12":
        t, hef = n["t_m"], n["hef_m"]
        fd = n["fd_pa"] if "fd_pa" in n else None
        for key, nk, mk in (("n-top", "n_top_n", "m_top_nm"), ("n-bottom", "n_bot_n", "m_bot_nm")):
            ei = abs(n.get(mk, 0) / n[nk] + n.get("ehe_m", 0)) + hef / 450
            phi = min(max(1 - 2 * max(ei, 0.05 * t) / t, 0), 1)
            out[key] = phi * t * fd
    elif e == "M-13":
        t, hef = n["t_m"], n["hef_m"]
        nm = n["n_mid_n"]
        et = n.get("m_top_nm", 0) / n.get("n_top_n", nm)
        eb = n.get("m_bot_nm", 0) / n.get("n_bot_n", nm)
        em = abs(0.5 * (abs(et) + abs(eb)) + n.get("m_mid_nm", 0) / nm + n.get("m_hm_nm", 0) / nm + hef / 450)
        ek = 0.002 * n.get("phi_inf", 1.5) * hef / t * math.sqrt(t * em)
        emk = max(em + ek, 0.05 * t)
        out = {"emk": emk, "phi-m": phi_m(hef / t, emk, t, n.get("k_e", 1000 if annex == "en" else 950)), "n-mid": phi_m(hef / t, emk, t, n.get("k_e", 1000 if annex == "en" else 950)) * t * n["fd_pa"]}
    elif e == "M-16":
        t, hef, nm = n["t_m"], n["hef_m"], n["n_mid_n"]
        ke = n.get("k_e", 1000 if annex == "en" else 950)

        def mid_capacity(m_hm):
            em = abs(m_hm / nm + hef / 450)
            emk = max(em + 0.002 * n.get("phi_inf", 1.5) * hef / t * math.sqrt(t * em), 0.05 * t)
            return phi_m(hef / t, emk, t, ke)

        phi_with, phi_without = mid_capacity(n["w_ed_pa"] * n["h_m"] ** 2 / 8.0), mid_capacity(0.0)
        out = {"phi-reduction": phi_with / max(phi_without, 1e-9), "n-mid-wind": max(phi_with * t * n["fd_pa"], 1e-9)}
    elif e == "M-33":
        out = {"we": n["qp_pa"] * n["cpe"]}
        if all(k in n for k in ("k0", "gamma_soil_n_m3", "he_m")):
            e0 = n["k0"] * n["gamma_soil_n_m3"] * n["he_m"]
            out.update({"e0": e0, "earth-resultant": integrate.quad(lambda z: n["k0"] * n["gamma_soil_n_m3"] * z, 0.0, n["he_m"])[0]})
    elif e == "M-05":
        fbd = n.get("fbok_pa", 2.0e6) / gm
        out = {"lb": n["diameter_m"] * n["fyd_pa"] / (4.0 * fbd)}
        if "lb_avail_m" in n:
            out["lb-avail"] = n["lb_avail_m"]
    elif e == "M-07":
        out = {"gamma-m": gm, "gamma-s": 1.15, "gamma-m-fire": 1.0}
    elif e == "M-52":
        out = {"spacing": {"clay": 12.0, "calcium_silicate": 8.0, "concrete": 6.0, "aerated": 6.0}[mat]}
    elif e == "M-60":
        out = {}
        expo = s["exposure"]
        mortar_need = {"mx1": 1.0, "mx2": 2.5, "mx3": 5.0, "mx4": 10.0, "mx5": float("inf")}[expo]
        if expo == "mx5" or (expo == "mx4" and mat == "aerated"):
            out["admissible"] = 1.0
        else:
            out["mortar"] = n["fm_pa"]
        if "frost_class" in n:
            out["frost"] = max(n["frost_class"], 1e-9)
        if "cover_m" in n:
            out["cover"] = n["cover_m"]
    elif e == "M-14":
        b = beta_c(n.get("a1_m", 0), n["hc_m"], n["ab_m2"], n.get("lefm_m", 1.0) * n["t_m"])
        out = {"beta": b, "n-rdc": b * n["ab_m2"] * n["fd_pa"]}
    elif e == "M-20":
        l, t = n["l_m"], n["t_m"]
        ecc = n.get("m_ed_nm", 0) / n["n_ed_n"]
        lc = min(max(3 * (l / 2 - ecc), 0), l)
        sg = n["n_ed_n"] / (lc * t) / 1e6
        fvk = min(fvk0_mpa(annex, mat, mortar, n["fm_pa"] / 1e6) + 0.4 * sg, fvlt_mpa(annex, group, n["fb_pa"] / 1e6, sg)) * 1e6
        out = {"lc": lc, "v-rd": fvk / gm * t * lc}
        if "flange_t_m" in n:
            tf, rule = n["flange_t_m"], min(0.5 * n["web_spacing_m"], 6 * n["flange_t_m"], 0.2 * n["height_total_m"])
            ov = [min(rule, max(x, 0.0)) for x in v["flange_len_m"]] if "flange_len_m" in v else [rule] * int(n.get("flanges", 2))
            a_web, a_f = l * t, sum(ov) * tf
            yc = (a_web * l / 2 + a_f * (l - tf / 2)) / (a_web + a_f)
            out.update({"flange-overhang": sum(ov) / len(ov), "flanged-area": a_web + a_f, "flanged-i": t * l**3 / 12 + a_web * (yc - l / 2) ** 2 + sum(o * tf**3 / 12 + o * tf * (l - tf / 2 - yc) ** 2 for o in ov)})
    elif e == "M-30":
        w, h, t = n["w_ed_pa"], n["h_m"], n["t_m"]
        c = {"cantilever": 2, "fixed": 12}.get(s.get("support", "pinned"), 8)
        me = w * h * h / c if c != 2 else w * h * h / 2
        out = {"m-rd": (n["fxk1_pa"] / gm + n.get("sigma_d_pa", 0)) * t * t / 6}
    elif e == "M-31":
        t = n["t_m"]
        z = t * t / 6
        m1 = (n["fxk1_pa"] / gm + n.get("sigma_d_pa", 0)) * z
        m2 = n["fxk2_pa"] / gm * z
        w_rd = yield_line_pressure(s.get("edges", "SSSS"), n["l_m"], n["h_m"], m1, m2)
        out = {"w-rd": w_rd, "alpha1": m1 / (w_rd * n["l_m"] ** 2)}
        mu = (n["fxk1_pa"] / gm + n.get("sigma_d_pa", 0)) / (n["fxk2_pa"] / gm)
        w_unit = yield_line_pressure(s.get("edges", "SSSS"), 1.0, n["h_m"] / n["l_m"], 1.0, 1.0 / mu)
        out["annex-e"] = n["alpha1"] if "alpha1" in n else 1.0 / w_unit
        if "alpha1" in n:
            out["annex-e-m-ed"] = m1
    elif e == "M-70":
        out = {}
        if "chase_width_m" in n and "chase_length_m" in n:
            out["chase-area"] = n["chase_width_m"] * n["chase_length_m"]
        if "chase_depth_m" in n and "t_m" in n:
            limits = chase_limits(s.get("orientation", "vertical"), n["t_m"], n.get("chase_length_m", float("inf")), float("inf"), 1.0)
            out["chase"] = max(limits[0] if limits else 0.0, 1e-9)
            if limits and limits[1] is not None and "chase_width_m" in n:
                out["chase-width"] = limits[1]
    elif e == "M-32":
        # equivalent compressive block depth x* solves 8 f_d x (t - x) / l^2 = f_d t^2 / l^2 (three-hinge arch statics)
        t = n["t_m"]
        x = optimize.brentq(lambda x: 8 * x * (t - x) - t * t, 0, t / 2)
        out = {"q-lat": 8 * n["fd_pa"] * x * (t - x) / n["la_m"] ** 2}
    elif e == "M-34":
        out = {"n-min": n["rho_e_n_m3"] * n["h_m"] * n["he_m"] ** 2 / (20 * n["t_m"]), "n-max": 0.45 * n["t_m"] * n["fd_pa"]}
    elif e == "M-40":
        b, d, a_s, fyd, fd = n["b_m"], n["d_m"], n["as_m2"], n.get("fyd_pa", 500e6 / 1.15), n["fd_pa"]
        x = optimize.brentq(lambda x: fd * b * x - a_s * fyd, 0, d)
        z = min(d - x / 2, 0.95 * d)
        out = {"z": z, "m-rd": min(a_s * fyd * z, 0.4 * fd * b * d * d)}
    elif e == "M-41":
        b, d, fd = n["b_m"], n["d_m"], n["fd_pa"]
        enh = min(max(2 * d / n["av_m"], 1), 2) if "av_m" in n else 1.0
        v1 = n["fvk_pa"] / gm * b * d * enh
        v2 = 0.9 * n["asw_m2"] * n.get("fyd_pa", 500e6 / 1.15) * d / n["s_m"] if "asw_m2" in n else 0.0
        out = {"v-rd1": v1, "v-rd2": v2, "v-rd": min(v1 + v2, 0.25 * fd * b * d)}
    elif e == "M-42":
        bars = list(zip(v["bar_y_m"], v["bar_a_m2"]))
        out = {"m-rd-n": mn_capacity(n["b_m"], n["l_m"], n["fd_pa"], n.get("fyd_pa", 500e6 / 1.15), bars, n.get("n_ed_n", 0))}
    elif e == "M-44":
        a, p0, ap = n["a_m2"], n["p0_n"], n["ap_m2"]
        loss = n.get("phi_inf", 1.5) * (p0 / a) * n["ep_pa"] / n["e_pa"] + n.get("eps_cs", 0) * n["ep_pa"] + n.get("rho_relax", 0) * p0 / ap
        out = {"p-eff": p0 - loss * ap}
    elif e == "M-50":
        share = n["t1_m"] ** 3 / (n["t1_m"] ** 3 + n["t2_m"] ** 3) if "t1_m" in n else 0
        out = {"tie-force": n["w_ed_pa"] * share / n["n_per_m2"]} if "w_ed_pa" in n else {}
    elif e == "M-81":
        lam = n["hef_m"] / n["t_m"]
        p1 = max(0.85 - 0.0011 * lam * lam, 0)
        ls = n.get("span_m", 0)
        p2 = 1.0 if (annex == "en" or ls <= 4.2) else min(0.9, 1.6 - ls / 6)
        out = {"phi-s": min(p1, p2), "n-rd": min(p1, p2) * n["t_m"] * n["fk_pa"] / gm}
    elif e == "M-15":
        pier_phi = n.get("phi", 0.9)
        out = {"pier-compression": pier_phi * n["pier_l_m"] * n["t_m"] * n["fd_pa"]}
        if "opening_w_m" in n:
            out["lintel-load"] = n.get("density", 1800) * 9.81 * n["t_m"] * n["opening_w_m"] ** 2 / 4
            out["lintel-moment"] = out["lintel-load"] * n["opening_w_m"] / 6
    elif e == "M-21":
        out = bracing(n, v, s)
    elif e == "M-43":
        out = {"col-area": n["col_a_m2"], "col-reinforcement": n["col_as_m2"]}
        if "v_ed_n" in n:
            out["shear"] = n["fvk_pa"] / gm * n["l_m"] * n["t_m"]
    elif e == "M-45":
        f_bd = T["bond"]["fbokMPa"] * 1e6 / n.get("gamma_m", 1.5)
        out = {"lb": n["diameter_m"] * n.get("fyd_pa", 500e6 / 1.15) / (4 * f_bd), "anchorage": n["provided_m"]}
    elif e == "M-51":
        out = {"anchor": n["anchor_rk_n"] / n.get("gamma_m", 1.5)}
        if "ring_tie_n" in n:
            out["ring-tie"] = n["ring_tie_n"]
    elif e == "M-61":
        limit = {"cantilever": 7.0, "continuous": 26.0}.get(s.get("support", "simple"), 20.0)
        out = {"l-over-d": limit * (310e6 / n["sigma_s_pa"] if "sigma_s_pa" in n else 1.0)}
        if "deflection_m" in n:
            out["lintel-deflection"] = n["span_m"] / n.get("limit_ratio", 500.0)
    elif e == "M-80":
        out = {"height-limit": 12 * n["t_m"] if n["t_m"] >= 0.24 else 2.75, "thickness-limit": n["t_m"]}
    elif e == "M-82":
        out = {"lc-min": n["lc_m"], "v-rd": (n["fvk0_pa"] + 0.4 * n.get("sigma_d_pa", 0)) / gm * n["t_m"] * n["lc_m"]}
    elif e == "M-83":
        fxd = n["fxk1_pa"] / gm + n.get("sigma_d_pa", 0)
        out = {"wind-panel": fxd * n["t_m"] ** 2 / 6}
        if n.get("basement", 0) > 0:
            out["basement-n-max"] = 0.45 * n["t_m"] * n["fd_pa"]
    elif e == "M-91":
        t_red = max(n["t_m"] - 2 * n["a_theta_m"], 0.0)
        out = {"t-red": t_red, "n-rd-fi": n.get("phi", 1.0) * n["k_theta"] * n["fd_pa"] * t_red * n["l_m"]}
    elif e == "M-90":
        mu = n.get("mu_fi", 1.0)
        out = {"t-min": fire_mm(int(n["rei"]), mat)}
    elif e == "M-95":
        lo, hi = {"unreinforced": (1.5, 2.5), "confined": (2.0, 3.0), "reinforced": (2.5, 3.0)}[s["system"]]
        out = {"q-min": lo, "q-max": hi}
    return {k: float(x) for k, x in out.items()}


def norm(s):
    return str(s).lower().replace("_", "").replace("-", "")


def snapshot(doc):
    """📐️ Compression, mid-height and simplified utilisation of every wall by brute-force enumeration of the EN 1990 combinations."""
    annex = norm(doc.get("annex", "de"))
    cls = int(str(doc.get("masonryClass", "class1"))[-1])
    res = {}
    for w in doc["walls"]:
        t, h, L = w["thicknessM"], w["heightM"], w["lengthM"]
        material = {"clay": "clay", "calciumsilicate": "calcium_silicate", "aerated": "aerated", "concrete": "concrete"}[norm(w["unitMaterial"])]
        group = int(str(w["unitGroup"])[-1])
        mortar = {"generalpurpose": "gp", "thinlayer": "tl", "lightweight": "lw"}[norm(w["mortarType"])]
        fm = (w["mortarStrengthPa"] or 0) / 1e6
        d = delta(w["unitHeightM"], min(w["unitWidthM"], w["unitLengthM"]))
        fk = f_k_mpa(annex, material, group, mortar, w["fBPa"] / 1e6 * d, fm) * 1e6
        accidental = norm(doc.get("designSituation")) in ("accidental", "transient", "seismic")
        gm = gamma_m(annex, cls, accidental)
        fd = fk / gm
        l_ef = max(L - sum(o["widthM"] for o in w["openings"]), 1e-3)
        rho2 = 0.75 if w["slabBearingDepthM"] >= 2 * t / 3 else 1.0
        hef = rho_n(int(w["supportSides"]), rho2, h, L) * h
        lam = hef / t
        area = l_ef * t
        sw = w["densityKgM3"] * 9.81 * max(t * L * h - sum(o["widthM"] * o["heightM"] * t for o in w["openings"]), 0)
        keff = 1000 if annex == "en" else 950
        best = {"top": 0, "mid": 0, "bottom": 0}
        simp = 0.0
        span_max = max([lc["slabSpanM"] for lc in w["loadCases"]] or [0.0])
        phi2_simple = 1.0 if (annex == "en" or span_max <= 4.2) else min(0.9, 1.6 - min(span_max, 6) / 6)
        for lc in w["loadCases"]:
            g = lc["gKSlabN"] + sum(c["forceN"] for c in lc["concentrated"])
            qi = lc["qKImposedPa"] * lc["tributaryAreaM2"]
            qs = lc["qKSnowPa"] * lc["tributaryAreaM2"]
            fac = FACTORS.ActionFactors(annex, reliability_class=int(str(doc["reliabilityClass"])[-1]))
            gg, gq = fac.gamma_g_sup(), fac.gamma_q()
            psi = fac.psi(str(lc["imposedCategory"])[0].upper())
            psi_wind, psi_snow = fac.psi("wind")[0], fac.psi("snow")[0]
            wk = abs(lc["qPWindPa"] * lc["cPe"])
            combos = []
            for lead in ("imp", "snow"):
                if lead == "snow" and qs == 0:
                    continue
                q = gq * (qi + psi_snow * qs) if lead == "imp" else gq * (qs + psi[0] * qi)
                wv = gq * wk if lead == "imp" else gq * psi_wind * wk
                combos.append((gg * g + q, gg * (g + sw / 2) + q, gg * (g + sw) + q, wv))
            nt, nm, nb, wv = max(combos, key=lambda c: c[2])
            nfloor = max(g - sum(c["forceN"] for c in lc["concentrated"]) + qi + qs, 0)
            share = min(max(nfloor / max(nt, nm), 0), 1)
            etheta = min(min(lc["slabSpanM"], 6) / 25 * share, 0.05 * t) if annex == "de" else 0.0
            phi2 = 1.0 if (annex == "en" or lc["slabSpanM"] <= 4.2) else min(0.9, 1.6 - min(lc["slabSpanM"], 6) / 6)
            e_top = abs(w["eccentricityTopM"] + abs(t / 2 - min(max(w["slabBearingDepthM"], 0), t) / 2) + etheta)
            e_bot = abs(w["eccentricityBottomM"])
            ehm = wv * h * h / 8 * l_ef / max(nm, 1.0)
            em = 0.5 * (e_top + e_bot) + hef / 450 + ehm
            ek = 0.002 * w["phiInfinity"] * hef / t * math.sqrt(t * abs(em))
            emk = max(abs(em + ek), 0.05 * t)
            caps = {
                "top": (nt, phi_i(e_top, t, hef) * phi2),
                "mid": (nm, phi_m(lam, emk, t, keff)),
                "bottom": (nb, phi_i(e_bot, t, hef) * phi2),
            }
            worst = max(caps.values(), key=lambda c: c[0] / max(c[1], 1e-9))
            best["top"] = max(best["top"], worst[0] / max(worst[1] * fd * area, 1e-9))
            best["mid"] = max(best["mid"], caps["mid"][0] / max(caps["mid"][1] * fd * area, 1e-9))
            simp = max(simp, nb)
        res[w["id"]] = {
            "compression": best["top"],
            "mid": best["mid"],
            "slenderness": lam / 27,
            "simplified": simp / max(min(max(0.85 - 0.0011 * lam * lam, 0), phi2_simple) * fd * area, 1e-9),
        }
    return res


def main():
    doc = json.loads(sys.stdin.read())
    if doc["mode"] == "catalogue":
        print(json.dumps({"ok": True, "results": {c["id"]: catalogue(c) for c in doc["cases"]}}))
        return 0
    if doc["mode"] == "geometry":
        print(json.dumps({"ok": True, "results": {c["id"]: geometry(c) for c in doc["cases"]}}))
        return 0
    expected = snapshot(doc["snapshot"])
    print(json.dumps({"ok": True, "expected": expected}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
