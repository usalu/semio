"""📚️ EN 1997 independent reference library: textbook formulations written from the target specification, not from the Rust sources.

Every function works in SI units (N, Pa, m, N·m, degrees at the interface). Where a numerical or third-party method can replace a closed form it does:
trial-wedge maximisation for K_a, work-rate integration over a logarithmic spiral for K_p, Boussinesq quadrature for stresses, Student-t inversion by quadrature, regression by `statistics`.
"""
from __future__ import annotations

import math
import statistics

PI = math.pi
R = math.radians


def tand(x):
    return math.tan(R(x))


# ---------------------------------------------------------------- partial factors
A1, A2 = (1.35, 1.5), (1.0, 1.3)
M1 = dict(phi=1.0, c=1.0, cu=1.0, g=1.0)
M2 = dict(phi=1.25, c=1.25, cu=1.4, g=1.0)
R1 = dict(v=1.0, h=1.0, e=1.0, b=1.0, s=1.0, t=1.0, st=1.25, a=1.1)
R2 = dict(v=1.4, h=1.1, e=1.4, b=1.1, s=1.1, t=1.1, st=1.15, a=1.1)
R3 = dict(v=1.0, h=1.0, e=1.0, b=1.0, s=1.0, t=1.0, st=1.1, a=1.0)
R3_DE = dict(R3, a=1.1)
R4PILE = dict(b=1.3, s=1.3, t=1.3, st=1.6)
DE_ACTIONS = {"P": (1.35, 1.5), "T": (1.2, 1.3), "A": (1.1, 1.15)}
DE_BEARING = {"P": 1.4, "T": 1.3, "A": 1.2}
DE_SOIL = {"P": 1.25, "T": 1.15, "A": 1.10}


def situation_key(text):
    t = (text or "bsP").lower().replace("-", "").replace("_", "")
    return "T" if t in ("bst", "transient") else "A" if t in ("bsa", "accidental") else "P"


def approach_key(text):
    t = (text or "da2").lower().replace("-", "").replace("_", "").replace(" ", "").replace("*", "")
    return {"da1geo": "da1c2", "da1c2": "da1c2", "da2": "da2", "da2star": "da2", "da2s": "da2", "da3": "da3", "geo3": "geo3"}.get(t, "da1c1")


def factors(annex, approach, situation):
    ax, ap, sk = annex.lower(), approach_key(approach), situation_key(situation)
    if ax == "en" and sk == "A":
        g, q, m, r = 1.0, 1.0, M1, R1
    elif ax == "de" and ap == "da2":
        g, q = DE_ACTIONS[sk]
        m = M1
        r = dict(v=DE_BEARING[sk], h=1.1, e=DE_BEARING[sk], b=1.1, s=1.1, t=1.1, st=1.15, a=1.1)
    elif ax == "de" and ap == "da3":
        g, q = DE_ACTIONS[sk]
        f = DE_SOIL[sk]
        m, r = dict(phi=f, c=f, cu=f, g=1.0), R3_DE
    elif ax == "de" and ap == "geo3":
        g, q = 1.0, {"P": 1.3, "T": 1.2, "A": 1.1}[sk]
        f = DE_SOIL[sk]
        m, r = dict(phi=f, c=f, cu=f, g=1.0), R3_DE
    elif ap == "da1c1":
        (g, q), m, r = A1, M1, R1
    elif ap == "da1c2":
        (g, q), m = A2, M2
        r = dict(R1, b=R4PILE["b"], s=R4PILE["s"], t=R4PILE["t"], st=R4PILE["st"])
    elif ap == "da2":
        (g, q), m, r = A1, M1, R2
    elif ap == "da3":
        (g, q), m, r = A1, M2, R3
    else:
        (g, q), m, r = A2, M2, R3
    return dict(gamma_g=g, gamma_q=q, gamma_r_v=r["v"], gamma_r_h=r["h"], gamma_r_e=r["e"], gamma_phi=m["phi"], gamma_c=m["c"],
                gamma_cu=m["cu"], gamma_gamma=m["g"], gamma_b=r["b"], gamma_s=r["s"], gamma_t=r["t"], gamma_st=r["st"], gamma_a=r["a"])


def equ_upl_hyd(annex, situation):
    ax, sk = annex.lower(), situation_key(situation)
    if ax == "de":
        return {
            "P": dict(equ_stb=0.95, equ_dst=1.05, equ_q=1.5, upl_stb=0.9, upl_dst=1.0, upl_q=1.5, hyd_h=1.8, hyd_stb=0.9),
            "T": dict(equ_stb=0.95, equ_dst=1.05, equ_q=1.3, upl_stb=0.9, upl_dst=1.0, upl_q=1.3, hyd_h=1.45, hyd_stb=0.9),
            "A": dict(equ_stb=0.95, equ_dst=1.0, equ_q=1.15, upl_stb=0.95, upl_dst=1.0, upl_q=1.15, hyd_h=1.3, hyd_stb=0.95),
        }[sk]
    if sk == "A":
        return dict(equ_stb=1.0, equ_dst=1.0, equ_q=1.0, upl_stb=1.0, upl_dst=1.0, upl_q=1.0, hyd_h=1.0, hyd_stb=1.0)
    return dict(equ_stb=0.9, equ_dst=1.1, equ_q=1.5, upl_stb=0.9, upl_dst=1.0, upl_q=1.5, hyd_h=1.35, hyd_stb=0.9)


# ---------------------------------------------------------------- earth pressure
def ka_trial_wedge(phi, delta):
    """🧭️ K_a of a vertical wall with level ground by maximising the trial-wedge force over the wedge angle."""
    best = 0.0
    for i in range(1, 18000):
        th = R(0.005 * i)
        if th >= PI / 2:
            break
        w = 0.5 / math.tan(th)
        e = w * math.sin(th - R(phi)) / math.cos(th - R(phi) - R(delta))
        best = max(best, e)
    return 2.0 * best


def ka_coulomb(phi, delta, alpha=0.0, beta=0.0):
    if abs(alpha) < 1e-12 and abs(beta) < 1e-12:
        return ka_trial_wedge(phi, delta)
    p, d, a, b = R(phi), R(delta), R(alpha), R(beta)
    s = math.sqrt(math.sin(p + d) * math.sin(p - b) / (math.cos(a + d) * math.cos(a - b)))
    return math.cos(p - a) ** 2 / (math.cos(a) ** 2 * math.cos(a + d) * (1 + s) ** 2)


def ka_rankine(phi, beta=0.0):
    p, b = R(phi), R(beta)
    r = math.sqrt(max(math.cos(b) ** 2 - math.cos(p) ** 2, 0.0))
    return math.cos(b) * (math.cos(b) - r) / (math.cos(b) + r)


def kp_rankine(phi):
    return math.tan(R(45 + phi / 2)) ** 2


def kp_coulomb(phi, delta, alpha=0.0, beta=0.0):
    p, d, a, b = R(phi), R(delta), R(alpha), R(beta)
    s = math.sqrt(math.sin(p + d) * math.sin(p + b) / (math.cos(d - a) * math.cos(a - b)))
    return math.cos(p + a) ** 2 / (math.cos(a) ** 2 * math.cos(d - a) * (1 - s) ** 2)


def _spiral_exit(xo, yo, tp):
    rb, th0 = math.hypot(xo, yo), math.atan2(-yo, -xo)
    prev, th = 0.0, 0.0
    while th < 7.0:
        th += 0.002
        r = rb * math.exp(th * tp)
        x, y = xo + r * math.cos(th0 + th), yo + r * math.sin(th0 + th)
        if x < -1e-9:
            return None
        if y >= 1.0:
            lo, hi = prev, th
            for _ in range(60):
                mid = 0.5 * (lo + hi)
                ym = yo + rb * math.exp(mid * tp) * math.sin(th0 + mid)
                lo, hi = (lo, mid) if ym >= 1.0 else (mid, hi)
            return hi
        prev = th
    return None


def _polar_moments(xo, yo, tp):
    """📦️ Area and first moment ∫∫(x − x_o)dA of the wedge from the contour integrals ½∮r²dφ and ⅓∮r³cosφ dφ about the pole."""
    te = _spiral_exit(xo, yo, tp)
    if te is None:
        return None
    rb, th0 = math.hypot(xo, yo), math.atan2(-yo, -xo)
    ex, ey = xo + rb * math.exp(te * tp) * math.cos(th0 + te), 1.0

    def unwrap(a, ref):
        while a - ref > PI:
            a -= 2 * PI
        while a - ref < -PI:
            a += 2 * PI
        return a

    ang_a = math.atan2(1.0 - yo, 0.0 - xo)
    ang_b = unwrap(math.atan2(-yo, -xo), ang_a)
    ang_e = unwrap(math.atan2(ey - yo, ex - xo), ang_b + te)

    def piece(radius, phi0, phi1, n=1500):
        h = (phi1 - phi0) / n
        area = mom = 0.0
        for i in range(n + 1):
            ph = phi0 + i * h
            r = radius(ph)
            w = 1 if i in (0, n) else (4 if i % 2 else 2)
            area += w * 0.5 * r * r
            mom += w * r ** 3 * math.cos(ph) / 3.0
        return area * h / 3, mom * h / 3

    wall = lambda ph: (0.0 - xo) / math.cos(ph)
    ground = lambda ph: (1.0 - yo) / math.sin(ph)
    spiral = lambda ph: rb * math.exp((ph - ang_b) * tp)
    try:
        a1, m1 = piece(wall, ang_a, ang_b)
        a2, m2 = piece(spiral, ang_b, ang_b + te)
        a3, m3 = piece(ground, unwrap(ang_e, ang_b + te), ang_a + 0.0 if abs(ang_a - unwrap(ang_e, ang_b + te)) < PI else ang_a)
    except ZeroDivisionError:
        return None
    return a1 + a2 + a3, m1 + m2 + m3


def kp_log_spiral(phi, delta):
    """🪝️ Passive coefficient from the work-rate balance of a rigid body rotating about the pole of a logarithmic spiral, wedge integrals by contour integration about the pole."""
    if delta == 0 or phi <= 0:
        return kp_rankine(max(phi, 0.0))
    delta = min(abs(delta), 2.0 * phi / 3.0)
    tp, d = tand(phi), R(delta)

    def value(xo, yo):
        if yo <= 0.02:
            return 1e9
        res = _polar_moments(xo, yo, tp)
        if res is None:
            return 1e9
        area, mom = res
        den = yo * math.cos(d) + xo * math.sin(d) - math.cos(d) / 3.0
        if den <= 1e-9 or area <= 1e-9 or mom <= 0:
            return 1e9
        return 2.0 * mom / den

    scan = sorted(((value(-8.0 + 8.5 * i / 12, 0.2 + 5.0 * j / 12), (-8.0 + 8.5 * i / 12, 0.2 + 5.0 * j / 12)) for i in range(13) for j in range(13)))
    best = (1e18, None)
    for start in [pt for _, pt in scan[:3]]:
        simplex = [list(start), [start[0] + 0.4, start[1]], [start[0], start[1] + 0.4]]
        fv = [value(*p) for p in simplex]
        for _ in range(90):
            order = sorted(range(3), key=lambda k: fv[k])
            simplex = [simplex[k] for k in order]
            fv = [fv[k] for k in order]
            cen = [(simplex[0][i] + simplex[1][i]) / 2 for i in range(2)]
            refl = [cen[i] + (cen[i] - simplex[2][i]) for i in range(2)]
            fr = value(*refl)
            if fr < fv[0]:
                exp = [cen[i] + 2 * (cen[i] - simplex[2][i]) for i in range(2)]
                fe = value(*exp)
                simplex[2], fv[2] = (exp, fe) if fe < fr else (refl, fr)
            elif fr < fv[1]:
                simplex[2], fv[2] = refl, fr
            else:
                con = [cen[i] + 0.5 * (simplex[2][i] - cen[i]) for i in range(2)]
                fc = value(*con)
                if fc < fv[2]:
                    simplex[2], fv[2] = con, fc
                else:
                    for k in (1, 2):
                        simplex[k] = [simplex[0][i] + 0.5 * (simplex[k][i] - simplex[0][i]) for i in range(2)]
                        fv[k] = value(*simplex[k])
        if fv[0] < best[0]:
            best = (fv[0], simplex[0])
    return best[0]


def k0(phi, ocr=1.0, beta=0.0):
    return (1 - math.sin(R(phi))) * math.sqrt(max(ocr, 1.0)) * (1 + math.sin(R(beta)))


def k_ae(phi, delta, psi, beta, kh, kv):
    th = math.atan2(kh, 1 - kv)
    p, d, a, b = R(phi), R(delta), R(psi), R(beta)
    if p - th - b < 0:
        raise ValueError("no limit equilibrium")
    s = math.sqrt(max(math.sin(p + d) * math.sin(p - th - b), 0.0) / (math.cos(d + a + th) * math.cos(b - a)))
    return math.cos(p - th - a) ** 2 / (math.cos(th) * math.cos(a) ** 2 * math.cos(d + a + th) * (1 + s) ** 2)


# ---------------------------------------------------------------- bearing
def nq(phi):
    return (1 + math.sin(R(phi))) / (1 - math.sin(R(phi))) * math.exp(PI * tand(phi))


def nc(phi):
    return PI + 2 if abs(phi) < 1e-9 else (nq(phi) - 1) / tand(phi)


def ng(phi):
    return 2 * (nq(phi) - 1) * tand(phi)


def eff_dims(b, l, eb, el):
    bb, ll = max(b - 2 * abs(eb), 1e-6), max(l - 2 * abs(el), 1e-6)
    return (bb, ll) if bb <= ll else (ll, bb)


def bearing_drained(b, l, d, alpha, v, hb, hl, eb, el, phi, c, g_below, q_over, depth=False):
    bb, ll = eff_dims(b, l, eb, el)
    if b - 2 * abs(eb) > l - 2 * abs(el):
        hb, hl = hl, hb
    a = bb * ll
    t = tand(phi)
    ratio = bb / ll
    q, c_, g_ = nq(phi), nc(phi), ng(phi)
    sq = 1 + ratio * math.sin(R(phi))
    sg = 1 - 0.3 * ratio
    sc = (sq * q - 1) / (q - 1)
    bq = (1 - R(alpha) * t) ** 2
    bc = bq - (1 - bq) / (c_ * t)
    mb, ml = (2 + ratio) / (1 + ratio), (2 + 1 / ratio) / (1 + 1 / ratio)
    h = math.hypot(hb, hl)
    cos2 = (hl / h) ** 2 if h > 0 else 1.0
    m = ml * cos2 + mb * (1 - cos2)
    tt = 1 - h / (v + a * c / t)
    if tt < 0:
        raise ValueError("H too large")
    iq, ig = tt ** m, tt ** (m + 1)
    ic = iq - (1 - iq) / (c_ * t)
    dq, dc = 1.0, 1.0
    if depth:
        dq = 1 + 2 * t * (1 - math.sin(R(phi))) ** 2 * math.atan(d / bb)
        dc = dq - (1 - dq) / (c_ * t)
    ra = c * c_ * bc * sc * ic * dc + q_over * q * bq * sq * iq * dq + 0.5 * g_below * bb * g_ * bq * sg * ig
    return dict(nq=q, nc=c_, ng=g_, sq=sq, sc=sc, sg=sg, iq=iq, ic=ic, ig=ig, bq=bq, b_eff=bb, l_eff=ll, r_over_a=ra, r_k=ra * a)


def bearing_undrained(b, l, d, alpha, v, hb, hl, eb, el, cu, q_over):
    bb, ll = eff_dims(b, l, eb, el)
    a = bb * ll
    h = math.hypot(hb, hl)
    if h > a * cu:
        raise ValueError("H too large")
    bc = 1 - 2 * R(alpha) / (PI + 2)
    sc = 1 + 0.2 * bb / ll
    ic = 0.5 * (1 + math.sqrt(1 - h / (a * cu)))
    ra = (PI + 2) * cu * bc * sc * ic + q_over
    return dict(nq=1.0, nc=PI + 2, ng=0.0, sq=1.0, sc=sc, sg=1.0, iq=1.0, ic=ic, ig=1.0, bq=1.0, b_eff=bb, l_eff=ll, r_over_a=ra, r_k=ra * a)


def bearing_din4017(b, l, v, hb, hl, eb, el, phi, c, g_below, q_over):
    bb, ll = eff_dims(b, l, eb, el)
    t = tand(phi)
    nd = nq(phi)
    nb = (nd - 1) * t
    ratio = bb / ll
    sd, sb = 1 + ratio * math.sin(R(phi)), 1 - 0.3 * ratio
    sc = (sd * nd - 1) / (nd - 1)
    td = math.hypot(hb, hl) / v
    if td >= 1:
        raise ValueError("H/V")
    m = (2 + ratio) / (1 + ratio)
    idd, ib = (1 - 0.7 * td) ** (m + 1), (1 - td) ** (m + 1)
    ic = (idd * nd - 1) / (nd - 1)
    ra = c * nc(phi) * sc * ic + q_over * nd * sd * idd + g_below * bb * nb * sb * ib
    return dict(nq=nd, nc=nc(phi), ng=2 * nb, sq=sd, sc=sc, sg=sb, iq=idd, ic=ic, ig=ib, bq=1.0, b_eff=bb, l_eff=ll, r_over_a=ra, r_k=ra * bb * ll)


# ---------------------------------------------------------------- stresses and settlement (numerical Boussinesq)
def boussinesq_point(q, x, y, z):
    r = math.sqrt(x * x + y * y + z * z)
    return 3.0 * q * z ** 3 / (2.0 * PI * r ** 5)


def sigma_z_circle(sigma0, a, z, n=400):
    total, dr = 0.0, a / n
    for i in range(n):
        r = (i + 0.5) * dr
        total += sigma0 * 3.0 * z ** 3 / (r * r + z * z) ** 2.5 * r * dr
    return total


def sigma_z_rect_centre(sigma0, b, l, z, n=240):
    dx, dy = b / n, l / n
    total = 0.0
    for i in range(n):
        x = -b / 2 + (i + 0.5) * dx
        for j in range(n):
            y = -l / 2 + (j + 0.5) * dy
            total += boussinesq_point(sigma0 * dx * dy, x, y, z)
    return total


def sigma_z_strip_centre(sigma0, b, z, n=2000):
    total, dx = 0.0, b / n
    for i in range(n):
        x = -b / 2 + (i + 0.5) * dx
        total += sigma0 * dx * 2.0 * z ** 3 / (PI * (x * x + z * z) ** 2)
    return total


# ---------------------------------------------------------------- statistics
def t95(nu):
    try:
        from scipy import stats  # third-party reference when installed

        return float(stats.t.ppf(0.95, nu))
    except Exception:
        pass

    def pdf(x):
        return math.exp(math.lgamma((nu + 1) / 2) - math.lgamma(nu / 2)) / math.sqrt(nu * PI) * (1 + x * x / nu) ** (-(nu + 1) / 2)

    def cdf(x, n=4000):
        h = x / n
        s = pdf(0) + pdf(x)
        for i in range(1, n):
            s += (4 if i % 2 else 2) * pdf(i * h)
        return 0.5 + s * h / 3

    lo, hi = 0.0, 50.0
    for _ in range(60):
        mid = 0.5 * (lo + hi)
        lo, hi = (mid, hi) if cdf(mid) < 0.95 else (lo, mid)
    return 0.5 * (lo + hi)
