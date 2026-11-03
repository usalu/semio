#!/usr/bin/env python3
"""📚️ Independent EN 1994 catalogue oracle (stdlib only): numeric strip integration, numeric double integration of curvature and RK4 time stepping, written without reference to the Rust closed forms. Prints the spec worked examples E1-01..E1-12 as JSON."""
from __future__ import annotations
import json, math, sys

from pathlib import Path


def registry_number(row: str) -> float:
    """📖 Recommended value of a registry row, the very row the Rust reads."""
    path = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").is_file())
    return float(next(e["recommended"] for e in json.loads(path.read_text(encoding="utf-8"))["entries"] if e["id"] == row))


EA = 210e9
ES = 210e9


def strips(top, bot, width, n):
    h = (bot - top) / n
    return [(top + (i + 0.5) * h, width * h) for i in range(n)]


def ipe300():
    h, b, tf, tw, a = 0.300, 0.150, 0.0107, 0.0071, 5381e-6
    hw = h - 2 * tf
    tw_eff = max(tw, (a - 2 * b * tf) / hw)
    return dict(h=h, b=b, tf=tf, tw=tw, a=a, tw_eff=tw_eff, hw=hw, iy=83.56e-6)


def steel_strips(sec, ys, n=4000):
    out = []
    out += strips(ys, ys + sec["tf"], sec["b"], n)
    out += strips(ys + sec["tf"], ys + sec["h"] - sec["tf"], sec["tw_eff"], n)
    out += strips(ys + sec["h"] - sec["tf"], ys + sec["h"], sec["b"], n)
    return out


def sagging_moment(sec, fyd, ys, beff, hc, fcc, nc):
    st = steel_strips(sec, ys)
    npl = fyd * sum(a for _, a in st)
    nc = min(nc, npl, fcc * beff * hc)
    lo, hi = ys, ys + sec["h"]
    for _ in range(100):
        mid = 0.5 * (lo + hi)
        net = sum(fyd * a * (1 if y < mid else -1) for y, a in st)
        if net < -nc:
            lo = mid
        else:
            hi = mid
    y_na = 0.5 * (lo + hi)
    x = nc / (fcc * beff)
    m = -nc * x / 2
    for y, a in st:
        m += -fyd * a * y if y < y_na else fyd * a * y
    return m, nc, x


def second_moment(sec, ys, beff, hc, n):
    parts = [(a, y, 0.0) for y, a in strips(ys, ys + sec["h"], 0.0, 1)]
    area_s = sec["a"]
    yc_s = ys + sec["h"] / 2
    ac = beff * hc / n
    ybar = (area_s * yc_s + ac * hc / 2) / (area_s + ac)
    i = sec["iy"] + area_s * (yc_s - ybar) ** 2
    for y, a in strips(0.0, hc, beff / n, 2000):
        i += a * (y - ybar) ** 2
    return i, ybar


def deflection_numeric(length, w_over_ei_terms, n=2000):
    """📏️ Deflection at mid-span of a simply supported beam by double numerical integration of the curvature M(x)/EI."""
    h = length / n
    kappa = []
    for i in range(n + 1):
        x = i * h
        kappa.append(sum(w * x * (length - x) / 2 / ei for w, ei in w_over_ei_terms))
    theta = [0.0]
    for i in range(n):
        theta.append(theta[-1] + 0.5 * (kappa[i] + kappa[i + 1]) * h)
    y = [0.0]
    for i in range(n):
        y.append(y[-1] + 0.5 * (theta[i] + theta[i + 1]) * h)
    slope = -y[-1] / length
    return abs(y[n // 2] + slope * (length / 2))


def column_polygon(b, t, fyd, fcd, n=3000):
    h = b / 2
    rects = []
    for sgn in (1, -1):
        for z, a in strips(sgn * (h - t) if sgn > 0 else -h, h if sgn > 0 else -h + t, b, n):
            rects.append((z, a, "s"))
    for z, a in strips(-(h - t), h - t, 2 * t, n):
        rects.append((z, a, "s"))
    for z, a in strips(-(h - t), h - t, b - 2 * t, n):
        rects.append((z, a, "c"))

    def nm(xi):
        nn = mm = 0.0
        for z, a, k in rects:
            if k == "s":
                f = fyd * a * (1 if z > xi else -1)
            elif z > xi:
                f = fcd * a
            else:
                continue
            nn += f
            mm += f * z
        return nn, mm

    npl = nm(-b)[0]
    npm = sum(a for _, a, k in rects if k == "c") * fcd
    lo, hi = -h, h
    for _ in range(80):
        mid = 0.5 * (lo + hi)
        if nm(mid)[0] > 0:
            lo = mid
        else:
            hi = mid
    xi_b = 0.5 * (lo + hi)
    return npl, npm, nm(0.0)[1], nm(xi_b)[1]


def protected_temperature(t_end, am_v, d_p, lam_p, c_p, rho_p):
    def theta_g(t):
        return 20 + 345 * math.log10(8 * t / 60 + 1)

    def ca(th):
        if th < 600:
            return 425 + 0.773 * th - 1.69e-3 * th ** 2 + 2.22e-6 * th ** 3
        if th < 735:
            return 666 + 13002 / (738 - th)
        if th < 900:
            return 545 + 17820 / (th - 731)
        return 650

    phi = c_p * rho_p / (ca(20) * 7850) * d_p * am_v

    def rate(t, th):
        g = theta_g(t)
        dg = (theta_g(t + 0.5) - theta_g(t - 0.5 if t > 0.5 else 0.0)) / (1.0 if t > 0.5 else 0.5)
        r = (lam_p / d_p) * am_v / (ca(th) * 7850) * (g - th) / (1 + phi / 3) - (math.exp(phi / 10) - 1) * dg
        return max(r, 0.0) if g - th > 0 else r

    th, t, dt = 20.0, 0.0, 1.0
    while t < t_end - 1e-9:
        k1 = rate(t, th)
        k2 = rate(t + dt / 2, th + dt / 2 * k1)
        k3 = rate(t + dt / 2, th + dt / 2 * k2)
        k4 = rate(t + dt, th + dt * k3)
        th += dt / 6 * (k1 + 2 * k2 + 2 * k3 + k4)
        t += dt
    return th


def nonlinear_moment(sec, fyd, fcd, ys, beff, hc, eps_cu=3.5e-3, strips_per_part=600):
    """📉️ EN 1994-1-1 6.2.1.4 non-linear resistance: bisection on the curvature for eps_c = eps_cu, bisection on the top strain for N = 0, midpoint strips (parabola-rectangle concrete, bilinear steel)."""
    def stress_c(e):
        return 0.0 if e <= 0 else fcd if e >= 0.002 else fcd * (1 - (1 - e / 0.002) ** 2)
    parts = [(0.0, hc, beff, "c"), (ys, ys + sec["tf"], sec["b"], "s"), (ys + sec["tf"], ys + sec["h"] - sec["tf"], sec["tw_eff"], "s"), (ys + sec["h"] - sec["tf"], ys + sec["h"], sec["b"], "s")]
    def nm(kappa, eps_top):
        n = m = 0.0
        for top, bottom, width, kind in parts:
            dy = (bottom - top) / strips_per_part
            for k in range(strips_per_part):
                y = top + (k + 0.5) * dy
                e = eps_top - kappa * y
                s = stress_c(e) if kind == "c" else max(-fyd, min(fyd, EA * e))
                f = s * width * dy
                n += f
                m -= f * y
        return n, m
    def top_strain(kappa):
        lo, hi = -1.0, 1.0
        for _ in range(60):
            mid = 0.5 * (lo + hi)
            if nm(kappa, mid)[0] < 0:
                lo = mid
            else:
                hi = mid
        return 0.5 * (lo + hi)
    lo, hi = 1e-4, 1.0
    for _ in range(40):
        mid = 0.5 * (lo + hi)
        if top_strain(mid) < eps_cu:
            lo = mid
        else:
            hi = mid
    kappa = 0.5 * (lo + hi)
    return abs(nm(kappa, top_strain(kappa))[1]), kappa


def continuous(spans, loads, ei1, ei2, fraction, cantilever=False, cells=400):
    """🪶️ Flexibility method of a continuous beam with cracked EI2 over `fraction` of each span at internal supports; returns (support moments, peak downward deflection of the last span for a cantilever else the first loaded span)."""
    k = len(spans)

    def ei_at(i, x):
        zone = fraction * spans[i]
        return ei2 if (i > 0 and x < zone - 1e-12) or (i < k - 1 and x > spans[i] - zone + 1e-12) else ei1

    def cell_data(i, ml, mr):
        h = spans[i] / cells
        xs = [(c + 0.5) * h for c in range(cells)]
        kap = [(loads[i] * x * (spans[i] - x) / 2 + ml * (1 - x / spans[i]) + mr * x / spans[i]) / ei_at(i, x) for x in xs]
        return h, xs, kap

    def left_slope(i, ml, mr):
        h, xs, kap = cell_data(i, ml, mr)
        return -sum((spans[i] - x) * q for x, q in zip(xs, kap)) * h / spans[i]

    def turn(i, ml, mr):
        h, xs, kap = cell_data(i, ml, mr)
        return sum(kap) * h

    unknown = [i for i in range(1, k) if not (cantilever and i == k - 1)]

    def supports_for(trial):
        sup = [0.0] * (k + 1)
        for slot, i in enumerate(unknown):
            sup[i] = trial[slot]
        if cantilever:
            sup[k - 1] = -loads[k - 1] * spans[k - 1] ** 2 / 2
        return sup

    def residual(trial):
        sup = supports_for(trial)
        return [left_slope(i - 1, sup[i - 1], sup[i]) + turn(i - 1, sup[i - 1], sup[i]) - left_slope(i, sup[i], sup[i + 1]) for i in unknown]

    n = len(unknown)
    base = residual([0.0] * n)
    cols = []
    for j in range(n):
        unit = [0.0] * n
        unit[j] = 1.0
        r = residual(unit)
        cols.append([r[i] - base[i] for i in range(n)])
    a = [[cols[j][i] for j in range(n)] for i in range(n)]
    b = [-x for x in base]
    for c in range(n):
        piv = max(range(c, n), key=lambda r: abs(a[r][c]))
        a[c], a[piv], b[c], b[piv] = a[piv], a[c], b[piv], b[c]
        for r in range(c + 1, n):
            f = a[r][c] / a[c][c]
            for q in range(c, n):
                a[r][q] -= f * a[c][q]
            b[r] -= f * b[c]
    x = [0.0] * n
    for r in reversed(range(n)):
        x[r] = (b[r] - sum(a[r][q] * x[q] for q in range(r + 1, n))) / a[r][r]
    sup = supports_for(x)
    peak, slope_in = 0.0, 0.0
    for i in range(k):
        h, xs, kap = cell_data(i, sup[i], sup[i + 1])
        theta = slope_in if (cantilever and i == k - 1) else left_slope(i, sup[i], sup[i + 1])
        acc_k = acc_b = 0.0
        own = (k - 1) if cantilever else (1 if k == 3 else 0)
        for c in range(cells + 1):
            xb = c * h
            w = -(theta * xb + xb * acc_k - acc_b)
            if i == own and abs(w) > abs(peak):
                peak = w
            if c < cells:
                acc_k += kap[c] * h
                acc_b += kap[c] * xs[c] * h
        slope_in = theta + sum(kap) * h
    return sup, peak


def encased_polygon():
    """🔺️ Exact piecewise integration of the plastic N-M polygon of a concrete-encased I-section (points B and D)."""
    fyd, fcd0, fsd = 355e6, 0.85 * 30e6 / 1.5, 500e6 / 1.15
    bc = hc = 0.4
    h, b, tw, tf = 0.2, 0.2, 0.0085, 0.013
    as_total, level = 4 * 314.16e-6, 0.4 / 2 - 0.05
    aa = 2 * b * tf + (h - 2 * tf) * tw
    fcd = fcd0 * (bc * hc - aa - as_total) / (bc * hc - aa)
    pieces = [(h / 2 - tf, h / 2, b, 's'), (-h / 2, -h / 2 + tf, b, 's'), (-h / 2 + tf, h / 2 - tf, tw, 's'), (-hc / 2, -h / 2, bc, 'c'), (h / 2, hc / 2, bc, 'c'), (-h / 2, -h / 2 + tf, bc - b, 'c'), (h / 2 - tf, h / 2, bc - b, 'c'), (-h / 2 + tf, h / 2 - tf, bc - tw, 'c')]

    def resultant(yna):
        n = m = 0.0
        for y0, y1, w, kind in pieces:
            lo, hi = max(y0, yna), y1
            if hi > lo:
                f = w * (hi - lo) * (fyd if kind == 's' else fcd)
                n += f
                m += f * 0.5 * (lo + hi)
            if kind == 's':
                lo, hi = y0, min(y1, yna)
                if hi > lo:
                    f = -w * (hi - lo) * fyd
                    n += f
                    m += f * 0.5 * (lo + hi)
        for y in (level, -level):
            f = as_total / 2 * fsd * (1 if y > yna else -1)
            n += f
            m += f * y
        return n, m

    lo, hi = -hc / 2, hc / 2
    for _ in range(100):
        mid = (lo + hi) / 2
        if resultant(mid)[0] > 0:
            lo = mid
        else:
            hi = mid
    return resultant((lo + hi) / 2)[1], resultant(0.0)[1]


def beta_factors(kappa):
    b1 = 1.0 if kappa <= 0.02 else (1 / (1 + 6.4 * kappa ** 2) if kappa <= 0.7 else 1 / (5.9 * kappa))
    b2 = 1.0 if kappa <= 0.02 else (1 / (1 + 6.0 * (kappa - 1 / (2500 * kappa)) + 1.6 * kappa ** 2) if kappa <= 0.7 else 1 / (8.6 * kappa))
    return b1, b2


def column_fire_cells(t_end=3600.0):
    """🔥️ Explicit finite-volume heat conduction of one quadrant of the concrete-encased HE-section (ISO 834, harmonic-mean conductances); returns the temperatures of the cells (0, 0), (0, 9) and (19, 19)."""
    h, b, tw, tf, bc, hc = 0.2, 0.2, 0.0085, 0.013, 0.4, 0.4
    nx = ny = 20
    dx, dy = bc / 2 / nx, hc / 2 / ny
    lim = h / 2 - max(tf, dy / 2)
    steel = [[(((j + 0.5) * dy >= lim and (i + 0.5) * dx <= b / 2 and (j + 0.5) * dy <= h / 2) or ((i + 0.5) * dx <= tw / 2 and (j + 0.5) * dy < lim)) for i in range(nx)] for j in range(ny)]

    def lam_c(t):
        u = min(max(t, 20.0), 1200.0) / 100
        return 1.36 - 0.136 * u + 0.0057 * u * u

    def cp_c(t):
        return 900.0 if t <= 100 else 900 + (t - 100) if t <= 200 else 1000 + (t - 200) / 2 if t <= 400 else 1100.0

    def cp_s(t):
        return 425 + 0.773 * t - 1.69e-3 * t ** 2 + 2.22e-6 * t ** 3 if t < 600 else 666 + 13002 / (738 - t) if t < 735 else 545 + 17820 / (t - 731) if t < 900 else 650.0

    th = [[20.0] * nx for _ in range(ny)]
    t, dt = 0.0, 1.0
    while t < t_end - 1e-9:
        gas = 20 + 345 * math.log10(8 * (t + dt / 2) / 60 + 1)
        lam = [[(54 - 3.33e-2 * th[j][i] if th[j][i] < 800 else 27.3) if steel[j][i] else lam_c(th[j][i]) for i in range(nx)] for j in range(ny)]
        cap = [[(7850 * cp_s(th[j][i])) if steel[j][i] else 2300 * cp_c(th[j][i]) for i in range(nx)] for j in range(ny)]
        nxt = [[0.0] * nx for _ in range(ny)]
        for j in range(ny):
            for i in range(nx):
                q = 0.0
                for (a, c, face, d) in ((i + 1, j, dy, dx), (i - 1, j, dy, dx), (i, j + 1, dx, dy), (i, j - 1, dx, dy)):
                    if 0 <= a < nx and 0 <= c < ny:
                        q += 2 * lam[j][i] * lam[c][a] / (lam[j][i] + lam[c][a]) * (th[c][a] - th[j][i]) / d * face
                flux = 25 * (gas - th[j][i]) + 0.7 * 5.67e-8 * ((gas + 273) ** 4 - (th[j][i] + 273) ** 4)
                if i == nx - 1:
                    q += flux * dy
                if j == ny - 1:
                    q += flux * dx
                nxt[j][i] = th[j][i] + q * dt / (cap[j][i] * dx * dy)
        th = nxt
        t += dt
    return th[0][0], th[9][0], th[19][19]


def end_plate_bearing(fcd, w, h, slab_b, slab_h):
    a0 = w * h
    a1 = max(min(3 * w, slab_b), w) * max(min(3 * h, slab_h), h)
    return min(a0 * fcd * math.sqrt(a1 / a0), 3 * fcd * a0)


def crack_band_station(sup, q, length, k, n=25):
    lo, hi = 0.5 * length, length
    moment = lambda x: q * x * (length - x) / 2 + sup[0] * (1 - x / length) + sup[1] * x / length
    for _ in range(80):
        mid = 0.5 * (lo + hi)
        if moment(mid) > 0:
            lo = mid
        else:
            hi = mid
    x0 = 0.5 * (lo + hi)
    x = x0 + 2 * (length - x0) * k / (n - 1)
    return abs(moment(x if x <= length else 2 * length - x)) / abs(moment(length))


def slab_far_column(ys=0.15, t_end=3600.0, ny=15):
    """🔥️ One column of the slab away from the steel flange (pure one-dimensional conduction from the soffit, ISO 834 with the surface exchange 25 W/m²K plus radiation, 9 W/m²K on the top, half-cell surface resistance); returns the soffit and the middle cell temperature."""
    dy = ys / ny
    th = [20.0] * ny

    def lam(t):
        u = min(max(t, 20.0), 1200.0) / 100
        return 1.36 - 0.136 * u + 0.0057 * u * u

    def cp(t):
        return 900.0 if t <= 100 else 900 + (t - 100) if t <= 200 else 1000 + (t - 200) / 2 if t <= 400 else 1100.0

    def flux(g, t):
        return 25 * (g - t) + 0.7 * 5.67e-8 * ((g + 273) ** 4 - (t + 273) ** 4)

    t = 0.0
    while t < t_end - 1e-9:
        step = min(1.0, t_end - t)
        g = 20 + 345 * math.log10(8 * (t + step / 2) / 60 + 1)
        lm, cap = [lam(x) for x in th], [2300 * cp(x) for x in th]
        nxt = []
        for j in range(ny):
            q = 0.0
            for k in (j - 1, j + 1):
                if 0 <= k < ny:
                    q += 2 * lm[j] * lm[k] / (lm[j] + lm[k]) * (th[k] - th[j]) / dy
            if j == 0:
                d = g - th[0]
                q += 0.0 if abs(d) < 1e-9 else d / (d / flux(g, th[0]) + dy / 2 / lm[0])
            if j == ny - 1:
                q += (20 - th[j]) / (1 / 9.0 + dy / 2 / lm[j])
            nxt.append(th[j] + q * step / (cap[j] * dy))
        th = nxt
        t += step
    return th[0], th[ny // 2]


def ky_table(theta):
    k = [1.0, 1.0, 1.0, 1.0, 1.0, 0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02, 0.0]
    t = min(max(theta, 20.0), 1200.0)
    if t <= 100:
        return 1.0
    i = min(int(t // 100), 11)
    return k[i] + (t - 100 * i) / 100 * (k[i + 1] - k[i])


def endurance(delta_c, n_ref, m, gamma, delta):
    """🔁️ Cycles to failure of the stress range `delta` on the S-N line, by bisection on the resistance curve instead of the closed form."""
    lo, hi = 0.0, 40.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if delta_c * (n_ref / 10 ** mid) ** (1 / m) / gamma > delta:
            lo = mid
        else:
            hi = mid
    return 10 ** ((lo + hi) / 2)


def rebar_resistance(n):
    """🔁️ Design resistance of the reinforcing-steel line at `n` cycles, the stress range whose bisected endurance is `n`."""
    lo, hi = 1e6, 1e9
    for _ in range(200):
        mid = (lo + hi) / 2
        if endurance(162.5e6, 1e6, 9.0, 1.15, mid) > n:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def chi_lt_root(m_pl, mcr, h_over_b, welded):
    """🌀 EN 1993-1-1 6.3.2.2 with Table 6.4 curves a/b (rolled) and c/d (welded), as the smaller root of lambda^2 chi^2 - 2 phi chi + 1 = 0 instead of the closed form 1/(phi + sqrt(phi^2 - lambda^2))."""
    lam = math.sqrt(m_pl / mcr)
    alpha = {(False, True): 0.21, (False, False): 0.34, (True, True): 0.49, (True, False): 0.76}[(welded, h_over_b <= 2.0)]
    phi = 0.5 * (1 + alpha * (lam - 0.2) + lam * lam)
    roots = [(phi - sg * math.sqrt(max(phi * phi - lam * lam, 0.0))) / (lam * lam) for sg in (1, -1)]
    return lam, min(1.0, min(r for r in roots if r > 0))


def patch_loading_a(hw, tw, bf, tf, fyw, fyf, ss, a, gamma):
    """🧷 EN 1993-1-5 section 6 type a: both m2 variants are evaluated and the one consistent with lambda_F > 0.5 is kept."""
    kf = 6 + 2 * (hw / a) ** 2
    fcr = 0.9 * kf * EA * tw ** 3 / hw
    m1 = fyf * bf / (fyw * tw)
    def at(m2):
        ly = min(ss + 2 * tf * (1 + math.sqrt(m1 + m2)), a)
        return ly, math.sqrt(ly * tw * fyw / fcr)
    ly, lam = at(0.0)
    if lam > 0.5:
        ly, lam = at(0.02 * (hw / tf) ** 2)
    chi = min(0.5 / lam, 1.0)
    return chi * ly * tw * fyw / gamma, chi, ly


GA = 81e9
NU = 0.3


def beam_frequency_fd(length, ei, mass, n=400):
    """🌊️ Lowest bending frequency of a simply supported beam: inverse iteration on the finite-difference operator K = -d2/dx2 (the simply supported fourth-order operator is K squared), independent of the closed form pi/(2 L^2) sqrt(EI/m)."""
    h = length / (n + 1)
    x = [1.0] * n
    lam = 1.0
    for _ in range(60):
        rhs = [v * h * h for v in x]
        c, d = [0.0] * n, [0.0] * n
        for i in range(n):
            den = 2.0 + (c[i - 1] if i else 0.0)
            c[i] = -1.0 / den
            d[i] = (rhs[i] + (d[i - 1] if i else 0.0)) / den
        y = [0.0] * n
        y[-1] = d[-1]
        for i in range(n - 2, -1, -1):
            y[i] = d[i] - c[i] * y[i + 1]
        lam = sum(a * a for a in x) / sum(a * b for a, b in zip(x, y))
        norm = math.sqrt(sum(v * v for v in y))
        x = [v / norm for v in y]
    return math.sqrt(ei / mass) * lam / (2 * math.pi)


def flange_inertia_z(sec, n=4000):
    iz = 0.0
    for y, a in strips(-sec["b"] / 2, sec["b"] / 2, 2 * sec["tf"], n):
        iz += a * y * y
    iz += sec["hw"] * sec["tw"] ** 3 / 12
    return iz


def plastic_modulus(sec, n=4000):
    """📐 Plastic section modulus of the doubly symmetric I-section as the first moment of the strips about the mid-height."""
    half = sec["h"] / 2
    rects = [(-half, -half + sec["tf"], sec["b"]), (-half + sec["tf"], half - sec["tf"], sec["tw_eff"]), (half - sec["tf"], half, sec["b"])]
    return sum(abs(y) * a for top, bot, w in rects for y, a in strips(top, bot, w, n))


def uframe_buckling(sec, length, i2, spacing, alpha, fy, welded=False):
    """🦋️ EN 1994-1-1 6.4.2: U-frame stiffness from the slab and web springs in series and Mcr of the compression flange on the elastic torsional restraint, with chi_LT from the quadratic root."""
    k1 = alpha * EA * i2 / spacing
    hs = sec["h"] - sec["tf"]
    k2 = EA * sec["tw"] ** 3 / (4 * (1 - NU * NU) * hs)
    ks = k1 * k2 / (k1 + k2)
    it = (2 * sec["b"] * sec["tf"] ** 3 + sec["hw"] * sec["tw"] ** 3) / 3
    ifz = flange_inertia_z(sec) / 2
    mcr = math.pi / length * math.sqrt((GA * it + ks * length ** 2 / math.pi ** 2) * EA * ifz)
    m_pl = plastic_modulus(sec) * 355e6
    lam, chi = chi_lt_root(m_pl, mcr, sec["h"] / sec["b"], welded)
    return ks, mcr, lam, chi, chi * m_pl


def curvature_deflection(length, kappa, n=2000):
    """📏️ Mid-span deflection of a simply supported beam under the constant curvature kappa by double trapezoidal integration with the end-deflection correction."""
    h = length / n
    theta = [0.0]
    for _ in range(n):
        theta.append(theta[-1] + kappa * h)
    y = [0.0]
    for i in range(n):
        y.append(y[-1] + 0.5 * (theta[i] + theta[i + 1]) * h)
    slope = -y[-1] / length
    return abs(y[n // 2] + slope * (length / 2))


def creep_shrinkage(sec, ys, beff, hc, fck_mpa, phi, psi_l, eps_cs, length, g, q):
    """⏳️ Modular ratios n0, nL, ns from E_cm = 22 ((fck + 8)/10)^0.3 GPa, composite second moments by strip integration and the shrinkage curvature eps_cs (A_c/ns) z / I(ns) integrated twice."""
    ecm = 22e9 * ((fck_mpa + 8) / 10) ** 0.3
    n0 = EA / ecm
    nl = n0 * (1 + psi_l * phi)
    ns = n0 * (1 + 0.55 * phi)
    i0, _ = second_moment(sec, ys, beff, hc, n0)
    il, _ = second_moment(sec, ys, beff, hc, nl)
    is_, yb = second_moment(sec, ys, beff, hc, ns)
    load = deflection_numeric(length, [(g, EA * il), (q, EA * i0)])
    kappa = eps_cs * (beff * hc / ns) * (yb - hc / 2) / is_
    return n0, nl, ns, i0, il, load, curvature_deflection(length, kappa)


def critical_temperature(mu0):
    return 39.19 * math.log(1 / (0.9674 * mu0 ** 3.833) - 1) + 482


def critical_temperature_by_inversion(mu0):
    lo, hi = 400.0, 1100.0
    for _ in range(80):
        mid = 0.5 * (lo + hi)
        if ky_table(mid) > mu0:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)



def vectors():
    sec = ipe300()
    fy = 355e6
    fcc = 17e6
    v = {}
    v["beff_simple"] = 2 * min(8000 / 8, 1500) / 1000
    be = min(8000 / 8, 1500)
    v["beff_end"] = 2 * min(1.0, 0.55 + 0.025 * 8000 / be) * be / 1000
    full, ncf, x = sagging_moment(sec, fy, 0.100, 2.0, 0.100, fcc, 1e30)
    v["mpl_full"] = full
    v["mnl_ipe300"], v["kappa_u_ipe300"] = nonlinear_moment(sec, fy, fcc, 0.100, 2.0, 0.100)
    v["x_full"] = x
    half, _, _ = sagging_moment(sec, fy, 0.100, 2.0, 0.100, fcc, 0.5 * ncf)
    v["mpl_partial_sb"] = half
    ma = 628.4e-6 * fy
    v["mpl_linear"] = ma + (full - ma) * 0.5
    v["eta_min_8m"] = max(1 - (355 / 355) * (0.75 - 0.03 * 8), 0.4)
    d, fu = 0.019, 450e6
    v["prd1"] = 0.8 * fu * math.pi * d * d / 4 / 1.25
    v["prd2"] = 0.29 * 1.0 * d * d * math.sqrt(30e6 * 33e9) / 1.25
    v["prd_sheet"] = min(v["prd1"], v["prd2"]) * min(0.7 * (0.1 / 0.05) * (0.095 / 0.05 - 1), 0.85)
    v["prd_two_per_rib"] = min(v["prd1"], v["prd2"]) * min(0.7 / math.sqrt(2) * (0.1 / 0.05) * (0.095 / 0.05 - 1), 0.70)
    v["stud_gap"] = max(5 * d / 0.60, 0.60 / min(6 * 0.085, 0.8))
    v["asf_req"] = (81656 / 0.150 / 0.200) * 0.100 / (500e6 / 1.15)
    av = 5381e-6 - 2 * 0.150 * 0.0107 + (0.0071 + 2 * 0.015) * 0.0107
    v["vpl"] = av * fy / math.sqrt(3)
    v["rho_shear"] = (2 * 400e3 / v["vpl"] - 1) ** 2
    ia, ic = 121.583e-6, 418.690e-6
    ei = EA * ia + 0.6 * 33e9 * ic
    ncr = math.pi ** 2 * ei / 16
    v["ncr_col"] = ncr
    lam = math.sqrt(5676.9e3 / ncr)
    phi = 0.5 * (1 + 0.21 * (lam - 0.2) + lam * lam)
    v["chi_col"] = 1 / (phi + math.sqrt(phi * phi - lam * lam))
    npl, npm, mmax, mpl = column_polygon(0.2, 0.01, 355e6, 26.667e6)
    v["npl_rhs"], v["npm_rhs"], v["mmax_rhs"], v["mpl_rhs"] = npl, npm, mmax, mpl
    v["mu_1700"] = (npl - 1700e3) / (npl - npm)
    ap, fyp, ep, e, h = 1100e-6, 350e6, 0.020, 0.025, 0.150
    ncf_s = ap * fyp
    xs = ncf_s / (0.85 * 20e6)
    v["slab_full"] = ncf_s * (h - 0.5 * xs - ep + (ep - e))
    nc = 0.6 * ncf_s
    xs = nc / (0.85 * 20e6)
    v["slab_partial"] = nc * (h - 0.5 * xs - ep + (ep - e) * 0.6) + min(1.25 * 4000 * (1 - 0.6), 4000)
    v["slab_mk"] = 1.0 * 0.125 / 1.25 * (200e6 * ap / 1.0 + 0.04e6)
    k_v = min(2.0, 1 + math.sqrt(200 / 125))
    rho_v = min(ap / (0.4 * 0.125), 0.02)
    v_c = 0.18 / 1.5 * k_v * (100 * rho_v * 30) ** (1 / 3)
    v_min = 0.035 * k_v ** 1.5 * math.sqrt(30)
    v["slab_vv"] = max(v_c, v_min) * 1e6 * 0.4 * 0.125
    n0 = EA / 33e9
    nl = n0 * (1 + 1.1 * 2.5)
    i0, _ = second_moment(sec, 0.100, 2.0, 0.100, n0)
    il, _ = second_moment(sec, 0.100, 2.0, 0.100, nl)
    v["i_n0"], v["i_nl"] = i0, il
    v["defl"] = deflection_numeric(8.0, [(6e3, EA * il), (4e3, EA * i0)])
    fct = 2.9e6
    kc = min(1 / (1 + 100 / (2 * 250)) + 0.3, 1.0)
    v["as_min"] = 0.9 * kc * 0.8 * fct * (2000 * 100e-6) / 500e6
    v["iso_60"] = 20 + 345 * math.log10(8 * 60 + 1)
    v["iso_30"] = 20 + 345 * math.log10(8 * 30 + 1)
    v["theta_a_60"] = protected_temperature(3600.0, 150.0, 0.025, 0.12, 1000.0, 800.0)
    v["ky_500"] = 0.78
    q, e1 = 10e3, 210e9 * 6e-4
    sup, defl = continuous([8.0, 8.0], [q, q], e1, 0.5 * e1, 0.15)
    v["cont_end_mb_cracked"], v["cont_end_defl_cracked"] = -sup[1], defl
    v["cont_interior_defl_cracked"] = continuous([6.0, 8.0, 6.0], [q, q, q], e1, 0.5 * e1, 0.15)[1]
    v["cont_cantilever_tip_cracked"] = continuous([8.0, 3.0], [0.0, q], e1, 0.5 * e1, 0.15, cantilever=True)[1]
    v["cont_end_unequal_mb"] = -continuous([8.0, 6.0], [q, q], e1, e1, 0.0)[0][1]
    v["enc_mb"], v["enc_md"] = encased_polygon()
    v["beta1_k01"], v["beta2_k01"] = beta_factors(0.1)
    v["girder_cross_ribbed"], v["girder_cross_solid"], v["girder_as_min"] = 2.5 * 0.022, 4.0 * 0.022, 0.002 * 0.25
    v["delta_sigma_ts_example"] = registry_number("ndp.en1994.1-1.tension-stiffening") * (0.30 * 30.0 ** (2.0 / 3.0) * 1e6) / ((0.02 * 8e-4) / (0.01 * 2e-4) * 0.015)
    v["end_plate_open"] = end_plate_bearing(0.85 * 30e6 / 1.5, 0.2, 0.1, 1.0, 0.25)
    v["end_plate_capped"] = end_plate_bearing(0.85 * 30e6 / 1.5, 0.1, 0.1, 5.0, 5.0)
    v["crack_band_k6"] = crack_band_station(sup, q, 8.0, 6)
    v["fire_core_60"], v["fire_flange_60"], v["fire_corner_60"] = column_fire_cells()
    v["beam_fire_steel_60"] = protected_temperature(3600.0, 150.0, 0.02, 0.12, 1000.0, 800.0)
    v["beam_fire_soffit_60"], v["beam_fire_mid_60"] = slab_far_column()
    theta_120 = protected_temperature(7200.0, 150.0, 0.02, 0.12, 1000.0, 800.0)
    v["beam_fire_mfi_120"] = sagging_moment(sec, 355e6 * ky_table(theta_120), 0.150, 2.4, 0.100, 0.85 * 30e6, 1e30)[0]
    v["fatigue_stud_endurance"] = endurance(90e6, 2e6, 8.0, 1.15, 40e6)
    v["fatigue_stud_damage"] = 5e6 / v["fatigue_stud_endurance"]
    v["fatigue_rebar_damage"] = 2e6 / endurance(162.5e6, 1e6, 9.0, 1.15, 100e6)
    v["fatigue_steel_util"] = 50e6 / (80e6 * (2e6 / 1e7) ** (1 / 3.0) / 1.15)
    fctm = 0.30 * 30.0 ** (2.0 / 3.0) * 1e6
    v["delta_sigma_ts_fatigue_example"] = registry_number("ndp.en1994.2.fatigue-tension-stiffening") * fctm / ((0.02 * 8e-4) / (0.01 * 2e-4) * 0.015)
    per_nm = ES / EA * 0.30 / 8e-4
    v["fatigue_rebar_sigma_max"] = 300e3 * per_nm + v["delta_sigma_ts_fatigue_example"]
    v["fatigue_rebar_util_pass"] = (100e3 * per_nm + v["delta_sigma_ts_fatigue_example"]) / rebar_resistance(2e6)
    v["fatigue_rebar_util_fail"] = (400e3 * per_nm + v["delta_sigma_ts_fatigue_example"]) / rebar_resistance(2e6)
    v["fatigue_rebar_damage_ts"] = 2e6 / endurance(162.5e6, 1e6, 9.0, 1.15, 100e3 * per_nm + v["delta_sigma_ts_fatigue_example"])
    v["chi_lt_rolled_lambda"], v["chi_lt_rolled"] = chi_lt_root(500e3, 1500e3, 1.8, False)
    v["chi_lt_welded"] = chi_lt_root(500e3, 1500e3, 3.0, True)[1]
    v["patch_f_rd"], v["patch_chi"], v["patch_ly"] = patch_loading_a(0.6, 0.008, 0.2, 0.014, 355e6, 355e6, 0.1, 1.0, 1.0)
    v["patch_interaction"] = (0.8 * 150e3 / v["patch_f_rd"] + 300e3 / 450e3) / 1.4
    mass = (10e3 + 0.3 * 15e3) / 9.80665
    v["vibration_mass"] = mass
    v["vibration_f1"] = beam_frequency_fd(8.0, EA * 6e-4, mass)
    v["vibration_utilisation"] = 3.0 / v["vibration_f1"]
    sec_l = ipe300()
    v["ltb_ks"], v["ltb_mcr"], v["ltb_lambda"], v["ltb_chi"], v["ltb_mbrd"] = uframe_buckling(sec_l, 6.0, 4e-5, 3.0, 2.0, 355e6)
    n0c, nlc, nsc, i0c, ilc, load_c, shrink_c = creep_shrinkage(sec_l, 0.100, 2.0, 0.100, 30.0, 2.5, 1.1, 0.3e-3, 8.0, 6e3, 4e3)
    v["creep_ns"], v["creep_i_ns"] = nsc, second_moment(sec_l, 0.100, 2.0, 0.100, nsc)[0]
    v["creep_defl_load"], v["creep_defl_shrinkage"], v["creep_defl_total"] = load_c, shrink_c, load_c + shrink_c
    v["fire_crit_mu05"] = critical_temperature(0.5)
    v["fire_crit_mu03"] = critical_temperature(0.3)
    v["fire_crit_inversion_mu05"] = critical_temperature_by_inversion(0.5)
    v["fire_protected_90"] = protected_temperature(5400.0, 150.0, 0.02, 0.12, 1000.0, 800.0)
    return v


if __name__ == "__main__":
    out = vectors()
    if len(sys.argv) > 1 and sys.argv[1] == "--check":
        assert abs(out["mpl_full"] / 1e3 - 423.9) < 0.3, out["mpl_full"]
        assert abs(out["mpl_linear"] / 1e3 - 323.49) < 0.4
        assert abs(out["mpl_partial_sb"] / 1e3 - 364.35) < 0.8
        assert abs(out["prd_sheet"] / 1e3 - 69.41) < 0.05
        assert abs(out["mmax_rhs"] / 1e3 - 211.85) < 0.3
        assert abs(out["mpl_rhs"] / 1e3 - 206.94) < 0.4
        assert abs(out["slab_full"] / 1e3 - 43.77) < 0.05
        assert abs(out["defl"] * 1e3 - 10.34) < 0.06
        assert abs(out["as_min"] * 1e6 - 835.2) < 0.5
        print("catalogue oracle self-check OK")
    else:
        print(json.dumps(out, sort_keys=True))
