"""🏛️ Oracle of the annex blocks of the EN 1993 showcase (`annex_showcase`): ring-stiffened shell under external pressure (1-6), corrugated web (1-5 Annex D), orthotropic deck strip (2) and purlin free flange (1-3). The beam on the elastic foundation is solved by finite differences with scipy.linalg.solve_banded (independent of the Fourier series of the clause), the wave inertia of the corrugation by the polygon area moment of a shapely buffer of its centre line (independent of the thin-wall formula) and the deck strip by numerical integration of point-load fixed-end moments with scipy.integrate.quad. Prints the values asserted by the Rust tests; run with the repo venv: python -I 🐍️.py"""
import json
import math

import numpy as np
from scipy.integrate import quad
from scipy.linalg import solve_banded
from shapely.geometry import LineString

E, NU, GM0, GM1, GMF = 210000.0, 0.3, 1.0, 1.1, 1.15


def hoop_ls3(l, r, t, fy, alpha, c_theta):
    omega = l / math.sqrt(r * t)
    cr = 0.92 * E * c_theta / omega * t / r
    lam = math.sqrt(fy / cr)
    lp = math.sqrt(alpha / (1 - 0.6))
    chi = 1.0 if lam <= 0.4 else (1 - 0.6 * (lam - 0.4) / (lp - 0.4) if lam < lp else alpha / lam**2)
    return cr, lam, chi, chi * fy / GM1


def ring(r, t, a_r, i_r, e_r, l_r):
    l_e = 1.56 * math.sqrt(r * t)
    a_s = l_e * t
    c = a_r * e_r / (a_r + a_s)
    inertia = i_r + a_r * (e_r - c) ** 2 + l_e * t**3 / 12 + a_s * c * c
    return inertia, 3 * E * inertia / ((r + c) ** 3 * l_r)


def beam_on_spring_moments(q, L, ei, k, n=800):
    h = L / n
    m = n - 1
    ab = np.zeros((5, m))
    ab[0, 2:] = 1
    ab[1, 1:] = -4
    ab[2, :] = 6
    ab[3, :-1] = -4
    ab[4, :-2] = 1
    ab[2, 0] = ab[2, -1] = 5
    ab *= ei / h**4
    ab[2, :] += k
    w = solve_banded((2, 2), ab, q * np.ones(m))
    full = np.concatenate([[0.0], w, [0.0]])
    return -ei * (full[:-2] - 2 * full[1:-1] + full[2:]) / h**2


def wave_inertia_polygon(a0, a1, d, t):
    pts = [(0.0, d / 2), (a0 / 2, d / 2), (a0 / 2 + a1, -d / 2), (a0 / 2 + a1 + a0, -d / 2), (a0 / 2 + 2 * a1 + a0, d / 2), (a0 + 2 * a1 + a0, d / 2)]
    poly = LineString(pts).buffer(t / 2, cap_style=2, join_style=2, mitre_limit=10)
    xs, ys = poly.exterior.coords.xy
    area = i_x = 0.0
    for (x0, y0), (x1, y1) in zip(zip(xs, ys), zip(xs[1:], ys[1:])):
        cross = x0 * y1 - x1 * y0
        area += cross
        i_x += cross * (y0 * y0 + y0 * y1 + y1 * y1)
    area, i_x = area / 2, i_x / 12
    return abs(i_x)


def corrugated(a0, a1, d, hw, t, fy):
    a2 = math.hypot(a1, d)
    w, s = 2 * (a0 + a1), 2 * (a0 + a2)
    i_wave = t * d * d * (a0 / 2 + a2 / 6)
    bx, bz = E * i_wave / w, E * t**3 / (12 * (1 - NU**2)) * s / w
    tl = 5.34 * math.pi**2 * E / (12 * (1 - NU**2)) * (t / max(a0, a2)) ** 2
    tg = 32.4 / (t * hw**2) * (bx * bz**3) ** 0.25
    ty = fy / math.sqrt(3)
    ll, lg = math.sqrt(ty / tl), math.sqrt(ty / tg)
    chi = min(min(1.15 / (0.9 + ll), 1.0), min(1.5 / (0.5 + lg * lg), 1.0))
    return dict(i_wave=i_wave, polygon_i=wave_inertia_polygon(a0, a1, d, t) * 1.0, bx=bx, bz=bz, tau_l=tl, tau_g=tg, chi=chi, v_rd=chi * ty * hw * t / GM1)


def deck(q, c, l, t):
    w = q / c**2
    cc = min(c, l)
    lo, hi = (l - cc) / 2, (l + cc) / 2
    m_end = quad(lambda x: w * x * (l - x) ** 2 / l**2, lo, hi)[0]
    m_free = w * cc * (l / 4 - cc / 8)
    return m_end, m_free - m_end, 6 * m_end / t**2


r_ls3 = hoop_ls3(1500.0, 1000.0, 8.0, 355.0, 0.65, 1.0)
inertia, p_cr_ring = ring(1000.0, 8.0, 1500.0, 2.0e6, 80.0, 1500.0)
cor = corrugated(140.0, 40.0, 50.0, 1000.0, 2.0, 355.0)
m_end, m_sag, sigma_hog = deck(60000.0, 400 + 2 * 70 + 14.0, 300.0, 14.0)
delta_e2 = (400000 / 480000) * 1.0 * 1.2 * sigma_hog
t_f, b_f, c_f, hw = 2.0, 60.0, 20.0, 200.0
side = c_f + hw / 5
area = t_f * (b_f + side)
yc = t_f * (c_f - hw / 5) * b_f / 2 / area
i_fz = t_f * b_f**3 / 12 + t_f * side * (b_f / 2) ** 2 - area * yc * yc
cd = 1 / (1 / 4000.0 + 1 / (E * t_f**3 / (4 * (1 - NU**2) * hw)))
k = cd / hw**2
moments = beam_on_spring_moments(0.15 * 2.5, 6000.0, E * i_fz, k)
stations = moments[9:400:10]
m_fz = float(np.max(np.abs(stations)))
m_fz_all = float(np.max(np.abs(moments)))
assert len(stations) == 40 and abs(m_fz_all - m_fz) < 0.01 * m_fz_all
m_fz_rd = i_fz / (b_f / 2 + abs(yc)) * 350.0 / GM0
def stiffened_plate(a, b, t, fy, sigma_ed, n, area, inertia, offset):
    """📐️ Navier solution of the simply supported orthotropic plate with the stiffeners smeared over the width (independent of the printed Annex A formula)."""
    a_p, a_s = b * t, n * area
    shift = a_s * offset / (a_p + a_s)
    i_whole = b * t**3 / 12 + a_p * shift**2 + n * inertia + a_s * (offset - shift) ** 2
    gamma, delta = i_whole / (b * t**3 / 10.92), a_s / a_p
    k = min((gamma * (m * b / a) ** 2 + 2 + (a / (m * b)) ** 2) / (1 + delta) for m in range(1, 9))
    sigma_e = np.pi**2 * E * t**2 / (12 * (1 - NU**2) * b**2)
    lam = (fy / (k * sigma_e)) ** 0.5
    rho = 1.0 if lam <= 0.673 else (lam - 0.22) / lam**2
    return dict(gamma=gamma, delta=delta, k=k, sigmaCr=k * sigma_e, lam=lam, rho=rho, sigmaRd=rho * fy / GM1, utilisation=sigma_ed / (rho * fy / GM1))


stiff = stiffened_plate(3000.0, 1500.0, 10.0, 355.0, 100.0, 3, 1000.0, 1000.0 * 100.0**2 / 12, 55.0)
result = {
    "stiffened": stiff,
    "ringLs3": {"sigmaRd": r_ls3[3], "utilisation": 62.5 / r_ls3[3]},
    "ringInstability": {"inertia": inertia, "pCr": p_cr_ring, "utilisation": 1.5 / p_cr_ring},
    "corrugated": {**cor, "utilisation": 30000.0 / cor["v_rd"]},
    "deck": {"mHogging": m_end, "mSagging": m_sag, "sigmaHogging": sigma_hog, "deltaE2": delta_e2, "utilisation": delta_e2 * GMF / 71.0},
    "purlin": {"k": k, "peakFreeFlange": m_fz, "freeFlangeResistance": m_fz_rd, "utilisation": 2.5 * 6000.0**2 / 8 / 15.0e6 + m_fz / m_fz_rd},
}
assert abs(cor["polygon_i"] - cor["i_wave"]) < 0.02 * cor["i_wave"], (cor["polygon_i"], cor["i_wave"])
print(json.dumps(result, indent=2))
