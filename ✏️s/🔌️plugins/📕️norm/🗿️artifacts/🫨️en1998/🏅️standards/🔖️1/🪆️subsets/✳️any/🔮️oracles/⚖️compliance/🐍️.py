#!/usr/bin/env python3
"""🫨️ EN 1998 numeric oracle — spectrum / base shear utilizations (±0.5 % vs Rust)."""
from __future__ import annotations

import json
import math
import sys
from typing import Any


def a_gr_de(s_ap_r: float) -> float:
    return float(s_ap_r) / 2.5


def spectrum_params(combo: str) -> tuple[float, float, float, float]:
    # DIN EN 1998-1/NA Table NA.4
    return {
        "A-R": (1.00, 0.05, 0.20, 2.0),
        "B-R": (1.25, 0.05, 0.25, 2.0),
        "C-R": (1.50, 0.05, 0.30, 2.0),
        "B-T": (1.25, 0.10, 0.30, 2.0),
        "C-T": (1.50, 0.10, 0.40, 2.0),
        "C-S": (0.75, 0.10, 0.50, 2.0),
    }.get(combo, (1.25, 0.05, 0.25, 2.0))


def elastic_se(a_g: float, s: float, tb: float, tc: float, td: float, t: float) -> float:
    eta = 1.0
    if t <= 0:
        return a_g * s
    if t <= tb:
        return a_g * s * (1.0 + t / tb * (2.5 * eta - 1.0))
    if t <= tc:
        return a_g * s * 2.5 * eta
    if t <= td:
        return a_g * s * 2.5 * eta * tc / t
    return a_g * s * 2.5 * eta * tc * td / (t * t)


def psi2(cat: str) -> float:
    c = cat.strip().lower()
    return {
        "a": 0.3,
        "residential": 0.3,
        "b": 0.3,
        "office": 0.3,
        "c": 0.6,
        "congregation": 0.6,
        "d": 0.6,
        "retail": 0.6,
        "e": 0.8,
        "storage": 0.8,
        "f": 0.6,
        "traffic": 0.6,
        "g": 0.3,
        "vehicles": 0.3,
        "h": 0.0,
        "roofs": 0.0,
        "snow": 0.2,
        "wind": 0.0,
    }.get(c, 0.3)


def storey_mass(st: dict, is_roof: bool) -> float:
    phi = 1.0 if is_roof else (0.8 if st.get("correlatedOccupancy", True) else 0.5)
    w = float(st["permanentGkN"])
    for v in st.get("variables", []):
        w += phi * psi2(str(v.get("category", "b"))) * float(v["qkN"])
    return w / 9.81


def evaluate(doc: dict[str, Any]) -> dict[str, Any]:
    site = doc["site"]
    annex = doc.get("annex", "De").lower()
    if annex == "de":
        a_g = a_gr_de(site["sApR"])
        s, tb, tc, td = spectrum_params(site["deGroundCombo"])
    else:
        a_g = float(site["aGr"])
        s, tb, tc, td = 1.2, 0.15, 0.40, 2.0
    checks = []
    for b in doc.get("buildings", []):
        storeys = b["storeys"]
        h = sum(st["heightM"] for st in storeys)
        mass = sum(storey_mass(st, i + 1 == len(storeys)) for i, st in enumerate(storeys))
        ct = float(b.get("ct", 0.075))
        t1 = ct * (h**0.75)
        for sys in b.get("systems", []):
            q = max(1.0, float(sys["q0"]) * float(sys["alphaUOverAlpha1"]) * float(sys["kW"]))
            se = elastic_se(a_g, s, tb, tc, td, t1)
            sd = max(se / q, 0.2 * a_g)
            lam = 0.85 if t1 <= 2 * tc and len(storeys) > 2 else 1.0
            fb = sd * mass * lam
            vrd = float(sys["baseShearResistanceN"])
            checks.append(
                {
                    "id": f"en1998.1.{b['id']}.{sys['id']}.baseShear",
                    "utilization": (fb / vrd) if vrd > 0 else 0.0,
                }
            )
    return {"checks": checks, "spectrum": {"S": s, "TB": tb, "TC": tc, "TD": td, "aG": a_g}}


def cantilever_period(mass: float, height: float, ei: float) -> float:
    lo, hi = 1.0, 3.0
    for _ in range(80):
        mid = (lo + hi) / 2
        if math.cos(mid) * math.cosh(mid) + 1 > 0:
            lo = mid
        else:
            hi = mid
    mu = mass / height
    return 2 * math.pi / (lo**2 * math.sqrt(ei / (mu * height**4)))


def liq_n1_60cs(n: float, fc: float) -> float:
    if fc <= 5:
        a, b = 0.0, 1.0
    elif fc < 35:
        a, b = math.exp(1.76 - 190 / fc**2), 0.99 + fc**1.5 / 1000
    else:
        a, b = 5.0, 1.2
    return a + b * n


def liq_crr(n: float) -> float:
    return 1 / (34 - n) + n / 135 + 50 / (10 * n + 45) ** 2 - 1 / 200


def liq_factor() -> float:
    a_max, z, zw, gamma, mag, n, fc = 0.75, 6.0, 2.0, 18000.0, 6.5, 12.0, 20.0
    sv = gamma * z
    se = sv - 9810.0 * (z - zw)
    rd = 1.0 - 0.00765 * z
    csr = 0.65 * a_max / 9.81 * sv / se * rd
    return liq_crr(liq_n1_60cs(n, fc)) * 10**2.24 / mag**2.56 / csr


RC_MEMBER = {"h": 0.5, "b": 0.3, "d": 0.45, "l_v": 2.0, "n": 300.0e3, "a_s1": 1.8e-3, "a_s2": 0.9e-3, "d_b": 0.02, "f_y": 500.0, "f_c": 30.0, "a_sx": 1.57e-4, "s_h": 0.1, "f_yw": 500.0, "b0": 0.24, "h0": 0.44, "nr": 4.0, "bi": 0.12, "rho_d": 0.0}
ANNEX_F = {"a": 0.92, "b": 1.25, "c": 0.92, "d": 1.25, "e": 0.92, "f": 0.41, "m": 0.96, "k": 0.39, "kp": 0.39, "ct": 2.14, "ctp": 1.81, "cm": 2.57, "cmp": 1.85, "beta": 2.57, "gamma": 4.0}


def section_reference(m: dict[str, float], es: float = 200000.0, ec2: float = 2.0, ecu: float = 3.5, nexp: float = 2.0, esu: float = 45.0, slices: int = 40000) -> dict[str, float]:
    """🧵️ Yield and ultimate point of the rectangular section by fine strip integration (numpy) with brentq on the strain plane and on the first-limit curvature, independent of the Rust kernel; N, mm, MPa, permille, compression positive, top at +z."""
    import numpy as np
    from scipy import optimize

    b, h, d = m["b"] * 1e3, m["h"] * 1e3, m["d"] * 1e3
    fc, fy = m["f_c"], m["f_y"]
    bars = [(m["a_s1"] * 1e6, 0.5 * h - d), (m["a_s2"] * 1e6, d - 0.5 * h)]
    z = -0.5 * h + (np.arange(slices) + 0.5) * h / slices
    dz = h / slices

    def concrete(e):
        e = np.asarray(e, dtype=float)
        return np.where(e <= 0.0, 0.0, np.where(e < ec2, fc * (1.0 - (1.0 - np.minimum(e, ec2) / ec2) ** nexp), fc))

    def forces(em: float, k: float) -> tuple[float, float]:
        sc = concrete(em + k * z)
        n, mo = float(np.sum(sc) * b * dz), float(np.sum(sc * z) * b * dz)
        for area, zb in bars:
            e = em + k * zb
            st = min(fy, max(-fy, es * e / 1000.0)) - float(concrete(e))
            n, mo = n + st * area, mo + st * area * zb
        return n, mo

    def plane(k: float) -> float:
        return optimize.brentq(lambda em: forces(em, k)[0] - m["n"], -200.0, 200.0, xtol=1e-13)

    def ratio(k: float, concrete_limit: float, steel_limit: float) -> float:
        em = plane(k)
        return max((em + k * 0.5 * h) / concrete_limit, max(-(em + k * zb) for _, zb in bars if _ > 0.0) / steel_limit) - 1.0

    def first_limit(concrete_limit: float, steel_limit: float) -> float:
        return optimize.brentq(lambda k: ratio(k, concrete_limit, steel_limit), 1e-9, 4.0 * (ecu + esu) / h, xtol=1e-14)

    ky, ku = first_limit(ec2, fy / es * 1000.0), first_limit(ecu, esu)
    return {"phi_y": ky, "m_y": forces(plane(ky), ky)[1] * 1e-3, "phi_u": ku, "m_u": forces(plane(ku), ku)[1] * 1e-3}


def rc_reference() -> dict[str, float]:
    """🏛️ EN 1998-3 Annex A: the cracked-section neutral axis is found by equilibrium with a bracketing root finder (independent of the closed form), the chord rotations by the printed expressions."""
    from scipy import optimize

    m, es = RC_MEMBER, 200000.0
    ec = 22000.0 * (m["f_c"] / 10.0) ** 0.3
    eps = m["f_y"] / es
    dp = m["h"] - m["d"]

    def residual(xi: float) -> float:
        phi = eps / ((1.0 - xi) * m["d"])
        x = xi * m["d"]
        return 0.5 * m["b"] * ec * 1e6 * phi * x * x + m["a_s2"] * es * 1e6 * phi * (x - dp) - m["a_s1"] * m["f_y"] * 1e6 - m["n"]

    xi = optimize.brentq(residual, 1e-6, 0.999, xtol=1e-14)
    phi_y = eps / ((1.0 - xi) * m["d"])
    section = section_reference(m)

    def chord_yield(phi: float) -> float:
        return phi * m["l_v"] / 3.0 + 0.0013 * (1.0 + 1.5 * m["h"] / m["l_v"]) + 0.13 * phi * m["d_b"] * m["f_y"] / math.sqrt(m["f_c"])

    theta_y = chord_yield(section["phi_y"])
    alpha = (1 - m["s_h"] / (2 * m["b0"])) * (1 - m["s_h"] / (2 * m["h0"])) * (1 - m["nr"] * m["bi"] ** 2 / (6 * m["b0"] * m["h0"]))
    rho_sx = m["a_sx"] / (m["b"] * m["s_h"])
    nu = m["n"] / (m["b"] * m["h"] * m["f_c"] * 1e6)
    omega = m["a_s1"] / (m["b"] * m["d"]) * m["f_y"] / m["f_c"]
    omega_c = m["a_s2"] / (m["b"] * m["d"]) * m["f_y"] / m["f_c"]
    theta_um = 0.016 / 1.5 * 0.3**nu * (max(0.01, omega_c) / max(0.01, omega) * m["f_c"]) ** 0.225 * (m["l_v"] / m["h"]) ** 0.35 * 25 ** (alpha * rho_sx * m["f_yw"] / m["f_c"]) * 1.25 ** (100 * m["rho_d"])
    return {"rc_xi_y": xi, "rc_phi_y": phi_y, "rc_theta_y": theta_y, "rc_theta_y_elastic": chord_yield(phi_y), "rc_section_phi_y": section["phi_y"], "rc_section_m_y": section["m_y"], "rc_section_phi_u": section["phi_u"], "rc_section_m_u": section["m_u"], "rc_alpha": alpha, "rc_theta_um": theta_um, "rc_stress_ratio": ec * eps * xi / (1 - xi) / m["f_c"]}


def liquefaction_reference() -> dict[str, float]:
    """🌊️ EN 1998-5 Annex B: the stress ratio without depth reduction from groundhog (r_d switched off with the shallow coefficient 0), the Youd et al. ratio with it."""
    from groundhog.soildynamics.liquefaction import cyclicstressratio_youd

    a_max, z, zw, gamma, mag, n, fc = 0.75, 6.0, 2.0, 18.0, 6.5, 12.0, 20.0
    sv, se = gamma * z, gamma * z - 9.81 * (z - zw)
    annex_b = cyclicstressratio_youd(a_max, sv, se, z, mag, rd_factor2=0.0)["CSR [-]"]
    youd = cyclicstressratio_youd(a_max, sv, se, z, mag)["CSR [-]"]
    resistance = liq_crr(liq_n1_60cs(n, fc)) * 10**2.24 / mag**2.56
    return {"liq_csr_annex_b": annex_b, "liq_csr_youd": youd, "liq_factor_annex_b": resistance / annex_b, "liq_factor_youd_check": resistance / youd}


def annex_f_reference() -> dict[str, float]:
    """🏗️ EN 1998-5 Annex F: the surface is typed again, the load factor at the surface and the section limits are found with brentq."""
    from scipy import optimize

    c = ANNEX_F
    a_gs, phi, b_width, n_max = 0.25 * 9.81, 35.0, 2.0, 1.0e6
    f_bar = a_gs / 9.81 / math.tan(math.radians(phi))
    bound = (1 - c["m"] * f_bar ** c["k"]) ** c["kp"]

    def surface(n: float, v: float, mm: float) -> float:
        room = bound - n
        return (1 - c["e"] * f_bar) ** c["ct"] * (c["beta"] * v) ** c["ctp"] / (n ** c["a"] * room ** c["b"]) + (1 - c["f"] * f_bar) ** c["cm"] * (c["gamma"] * mm) ** c["cmp"] / (n ** c["c"] * room ** c["d"]) - 1

    n, v, mm = 0.3e6 / n_max, 0.05e6 / n_max, 0.08e6 / (b_width * n_max)
    lam = optimize.brentq(lambda x: surface(x * n, x * v, x * mm), 1e-9, bound / n * (1 - 1e-9), xtol=1e-14)
    v_limit = optimize.brentq(lambda x: surface(n, x, 0.0), 1e-12, 50.0, xtol=1e-14)
    m_limit = optimize.brentq(lambda x: surface(n, 0.0, x), 1e-12, 50.0, xtol=1e-14)
    return {"annexf_f_bar": f_bar, "annexf_bound": bound, "annexf_surface": surface(n, v, mm), "annexf_util": 1.0 / lam, "annexf_v_limit": v_limit, "annexf_m_limit": m_limit}


def pile_reference() -> dict[str, float]:
    """🏗️ Pile on a Winkler bed: the beam equation (EI times the fourth derivative plus k y equals 0) is solved as a boundary value problem for a pile much longer than 1/beta and compared with the closed forms; the kinematic moment is the Nikolaou et al. expression."""
    import numpy as np
    from scipy.integrate import solve_bvp

    e_p, d, k_h, length, shear = 30.0e9, 0.6, 20.0e6, 40.0, 100.0e3
    ei, k = e_p * math.pi * d**4 / 64.0, k_h * d
    beta = (k / (4 * ei)) ** 0.25

    def solve(fixed: bool) -> tuple[float, float]:
        def rhs(z, y):
            return np.vstack([y[1], y[2], y[3], -k / ei * y[0]])

        def bc(a, b):
            head = a[1] if fixed else a[2]
            return np.array([head, ei * a[3] + shear, b[2], b[3]])

        z = np.linspace(0.0, length, 4001)
        solution = solve_bvp(rhs, bc, z, np.zeros((4, z.size)), tol=1e-10, max_nodes=200000)
        assert solution.success
        moments = ei * solution.sol(np.linspace(0.0, length, 20001))[2]
        return float(np.max(np.abs(moments))), float(solution.sol(0.0)[0])

    free_m, free_y = solve(False)
    fixed_m, fixed_y = solve(True)
    a_gs, gamma, z_i, vs1, vs2 = 0.25 * 9.81, 18000.0, 6.0, 100.0, 300.0
    tau = a_gs / 9.81 * gamma * z_i
    e_1 = 3.0 * gamma / 9.81 * vs1**2
    kinematic = 0.042 * tau * d**3 * (20.0 / d) ** 0.30 * (e_p / e_1) ** 0.65 * (vs2 / vs1) ** 0.50
    return {"pile_beta": beta, "pile_free_mmax": free_m, "pile_free_y0": abs(free_y), "pile_fixed_mmax": fixed_m, "pile_fixed_y0": abs(fixed_y), "pile_kinematic": kinematic}


def n2_reference() -> dict[str, float]:
    """🌋️ N2 method: the first mode comes from a generalised eigenproblem, the elastic-plastic curve from the storey capacities and the target displacement from the equal-energy idealisation iterated to a fixed point."""
    import numpy as np
    from scipy.integrate import trapezoid
    from scipy.linalg import eigh

    m = np.array([200.0e3, 200.0e3, 150.0e3])
    k = np.array([80.0e6, 70.0e6, 60.0e6])
    resistance = np.array([900.0e3, 650.0e3, 350.0e3])
    stiffness = np.array([[k[0] + k[1], -k[1], 0.0], [-k[1], k[1] + k[2], -k[2]], [0.0, -k[2], k[2]]])
    values, vectors = eigh(stiffness, np.diag(m))
    shape = vectors[:, 0] / vectors[-1, 0]
    s = np.cumsum((m * shape)[::-1])[::-1]
    lam = np.min(resistance / s)
    d_y = float(np.sum(lam * s / k))
    v_y = float(lam * s[0])
    m_star = float(np.sum(m * shape))
    gamma = m_star / float(np.sum(m * shape**2))
    a_g, soil, tb, tc, td = 1.0, 1.2, 0.15, 0.5, 2.0

    def s_e(t: float) -> float:
        return elastic_se(a_g, soil, tb, tc, td, t)

    d_star, f_star = d_y / gamma, v_y / gamma
    d_m = 1.0e3 * d_star
    for _ in range(60):
        ds = np.array([0.0, d_star, d_m])
        fs = np.array([0.0, f_star, f_star])
        energy = float(trapezoid(fs, ds))
        d_ys = 2.0 * (d_m - energy / f_star)
        t_star = 2.0 * math.pi * math.sqrt(m_star * d_ys / f_star)
        q_u = s_e(t_star) * m_star / f_star
        d_et = s_e(t_star) * (t_star / (2.0 * math.pi)) ** 2
        d_t = d_et if (t_star >= tc or q_u <= 1.0) else d_et / q_u * (1.0 + (q_u - 1.0) * tc / t_star)
        if abs(d_t - d_m) <= 1e-9 * d_m:
            break
        d_m = d_t
    weakest = int(np.argmin(resistance / s))
    drift = float(lam * s[weakest] / k[weakest])
    theta, height = 0.03, 3.2
    roof_target = gamma * d_t
    capacity = d_y - drift + theta * height
    adrs = {"adrs_sa_y": f_star / m_star, "adrs_sa_e": s_e(t_star), "adrs_sd_e": d_et, "adrs_radial_at_dy": (2.0 * math.pi / t_star) ** 2 * d_star}
    return adrs | {"n2_gamma": gamma, "n2_m_star": m_star, "n2_d_y": d_y, "n2_v_y": v_y, "n2_t_star": t_star, "n2_q_u": q_u, "n2_d_t_star": d_t, "n2_period_1": 2 * math.pi / math.sqrt(values[0]), "n2c_weakest": float(weakest), "n2c_weakest_drift": drift, "n2c_roof_target": roof_target, "n2c_roof_capacity": capacity, "n2c_required_theta": (roof_target - (d_y - drift)) / height}


SOIL_LAYERS = [(3.0, 17000.0, 150.0, 0.0, 40.0), (4.0, 18000.0, 120.0, 12.0, 20.0), (5.0, 19000.0, 200.0, 20.0, 10.0)]


def soil_profile_reference() -> dict[str, float]:
    """🪨️ Layered profile: the stresses are the quadrature of the piecewise unit weight, the stress ratio is groundhog with r_d switched off, the resistance is typed again per layer, the site response is the quadrature of 1/v_s."""
    from groundhog.soildynamics.liquefaction import cyclicstressratio_youd
    from scipy import integrate

    a_gs, zw, mag = 0.75, 2.0, 6.5
    tops = [sum(l[0] for l in SOIL_LAYERS[:i]) for i in range(len(SOIL_LAYERS))]
    bottoms = [top + layer[0] for top, layer in zip(tops, SOIL_LAYERS)]
    total_depth = bottoms[-1]

    def index(z: float) -> int:
        return next(i for i, bottom in enumerate(bottoms) if z <= bottom + 1e-12)

    def gamma(z: float) -> float:
        return SOIL_LAYERS[index(min(z, total_depth))][1]

    def v_s(z: float) -> float:
        return SOIL_LAYERS[index(min(z, total_depth))][2]

    def stresses(z: float) -> tuple[float, float]:
        total = integrate.quad(gamma, 0.0, z, points=[b for b in bottoms if b < z], limit=50)[0]
        return total, total - 9810.0 * max(z - zw, 0.0)

    def factor(z: float) -> float:
        total, effective = stresses(z)
        csr = cyclicstressratio_youd(a_gs, total / 1000.0, effective / 1000.0, z, mag, rd_factor2=0.0)["CSR [-]"]
        layer = SOIL_LAYERS[index(z)]
        if layer[3] <= 0:
            return math.inf
        return liq_crr(liq_n1_60cs(layer[3], layer[4])) * 10**2.24 / mag**2.56 / csr

    def governing(i: int) -> float:
        start = max(tops[i] + 1e-6, 0.1)
        return min(factor(start + (bottoms[i] - start) * j / 8.0) for j in range(9))

    travel_30 = integrate.quad(lambda z: 1.0 / v_s(z), 0.0, 30.0, points=bottoms[:-1], limit=50)[0]
    travel = integrate.quad(lambda z: 1.0 / v_s(z), 0.0, total_depth, points=bottoms[:-1], limit=50)[0]
    total_6, effective_6 = stresses(6.0)
    return {"soil_total_6": total_6, "soil_effective_6": effective_6, "soil_factor_6": factor(6.0), "soil_governing_2": governing(1), "soil_governing_3": governing(2), "soil_vs30": 30.0 / travel_30, "soil_period": 4.0 * travel}


def time_history_reference() -> dict[str, float]:
    """📈 Accelerogram set: the mean record spectrum over the elastic spectrum by numpy, lowest over the period range."""
    import numpy as np

    records = np.array([[1.0, 1.1, 0.95, 0.9], [0.9, 1.2, 1.0, 0.8], [1.1, 1.0, 1.05, 1.0]])
    elastic = np.array([1.0, 1.0, 1.0, 1.0])
    return {"th_lowest_mean_ratio": float(np.min(records.mean(axis=0) / elastic))}


def record_samples(seed: int, n: int = 200) -> list[float]:
    """📼 Deterministic integer-derived accelerogram samples in m/s² (exactly representable, no libm in the input)."""
    return [((k * (37 + 2 * seed) + 11 * (seed + 1)) % 101 - 50) / 25.0 for k in range(n)]


def sdof_peak_sa(samples: list[float], dt: float, period: float, xi: float) -> float:
    """🎢 Pseudo-acceleration of the oscillator u'' + 2 xi w u' + w^2 u = -a(t) by scipy.signal.lsim with first-order-hold input (exact for piecewise linear acceleration), peak at the sample instants."""
    import numpy as np
    from scipy import signal

    w = 2.0 * math.pi / period
    system = signal.lti([[0.0, 1.0], [-w * w, -2.0 * xi * w]], [[0.0], [-1.0]], [[1.0, 0.0]], [[0.0]])
    t = np.arange(len(samples)) * dt
    _, u, _ = signal.lsim(system, U=np.array(samples), T=t, interp=True)
    return float(w * w * np.max(np.abs(u)))


def horizontal_reference() -> dict[str, float]:
    """🧱 Diaphragm resistance by quadrature of the shear diagram, overturning by brentq, accelerogram spectra by lsim, spectrum surface by the closed forms of EN 1998-1 3.2.2.2."""
    import numpy as np
    from scipy import integrate, optimize

    f_i, gamma_d, span, depth, t_m, tau, opening, area, f_d_mpa = 500.0e3, 1.3, 24.0, 16.0, 0.2, 1.0, 2.4, 0.01, 435.0
    force = gamma_d * f_i
    shear = lambda x: force * (0.5 - x / span)
    moment = lambda x: integrate.quad(shear, 0.0, x)[0]
    out = {
        "dia_f_d": force,
        "dia_v_ed": shear(0.0),
        "dia_v_rd": tau * 1.0e6 * t_m * (depth - opening),
        "dia_t_req": shear(0.0) / (tau * 1.0e6 * (depth - opening)),
        "dia_shear_flow": shear(0.0) / (depth - opening),
        "dia_m_mid": moment(span / 2.0),
        "dia_n_mid": moment(span / 2.0) / depth,
        "dia_n_quarter": moment(span / 4.0) / depth,
        "dia_n_rd": area * f_d_mpa * 1.0e6,
        "dia_a_req": moment(span / 2.0) / depth / (f_d_mpa * 1.0e6),
    }
    forces, elevations = np.array([100.0e3, 200.0e3, 300.0e3]), np.array([3.5, 7.0, 10.5])
    weight, base, tie, factor = 1.0e6, 4.0, 1.0e5, 1.0
    m_ov = float(np.dot(forces, elevations))
    stab = lambda b: factor * weight * b / 2.0 + tie
    out |= {
        "ov_moment": m_ov,
        "ov_stabilising": stab(base),
        "ov_base_req": optimize.brentq(lambda b: stab(b) - m_ov, 0.0, 100.0),
        "ov_tie_req": m_ov - factor * weight * base / 2.0,
    }
    dt, xi = 0.02, 0.05
    records = [record_samples(seed) for seed in (0, 1, 2)]
    out |= {
        "th_pga_0": max(abs(a) for a in records[0]),
        "th_sa_0_t0p1": sdof_peak_sa(records[0], dt, 0.1, xi),
        "th_sa_0_t0p5": sdof_peak_sa(records[0], dt, 0.5, xi),
        "th_sa_1_t1p0": sdof_peak_sa(records[1], dt, 1.0, xi),
        "th_sa_0_t0p5_xi2": sdof_peak_sa(records[0], dt, 0.5, 0.02),
    }
    a_g, s, tb, tc, td, t1 = 1.5, 1.2, 0.15, 0.5, 2.0, 0.6
    periods = [0.2 * t1 + (2.0 * t1 - 0.2 * t1) * i / 18 for i in range(19)]
    scales = (1.0, 1.2, 0.8)
    spectra = [[sdof_peak_sa([a * k for a in rec], dt, p, xi) for p in periods] for rec, k in zip(records, scales)]
    mean = np.mean(np.array(spectra), axis=0)
    elastic = np.array([elastic_se(a_g, s, tb, tc, td, p) for p in periods])
    ratio = mean / elastic
    mean_zero = float(np.mean([max(abs(a) for a in rec) * k for rec, k in zip(records, scales)]))
    out |= {
        "th_mean_zero": mean_zero,
        "th_lowest_ratio": float(np.min(ratio)),
        "th_lowest_period": periods[int(np.argmin(ratio))],
        "th_scale_to_accept": max(a_g * s / mean_zero, 0.9 / float(np.min(ratio)), 1.0),
    }

    def eta(xi_percent: float) -> float:
        return max(math.sqrt(10.0 / (5.0 + xi_percent)), 0.55)

    def surface(xi_percent: float, t: float) -> float:
        e, base = eta(xi_percent), 0.6 * 1.2
        if t <= 0.15:
            return base * (1 + t / 0.15 * (2.5 * e - 1))
        if t <= 0.5:
            return base * 2.5 * e
        if t <= 2.0:
            return base * 2.5 * e * 0.5 / t
        return base * 2.5 * e * 0.5 * 2.0 / t**2

    out |= {"surface_xi2_t0p3": surface(2.0, 0.3), "surface_xi5_t1": surface(5.0, 1.0), "surface_xi30_t0p1": surface(30.0, 0.1), "surface_xi5_t3": surface(5.0, 3.0), "surface_eta_xi30": eta(30.0)}
    return out


def ground_reference() -> dict[str, float]:
    """🌍️ Table 3.1 ground types by numpy: time-averaged v_s,30, N_SPT and c_u over layers and the class by `numpy.digitize` with the boundary in the lower class (A=0, B=1, C=2, D=3, E=4, S1=5, S2=6)."""
    import numpy as np

    def average(layers: list[tuple[float, float]]) -> float:
        h, x = np.array([l[0] for l in layers]), np.array([l[1] for l in layers])
        return float(h.sum() / np.sum(h / x))

    def by(value: float, edges: list[float]) -> int:
        return int(3 - np.digitize(value, edges, right=True))

    vs = average([(10.0, 200.0), (20.0, 400.0)])
    n = average([(5.0, 10.0), (25.0, 40.0)])
    cu = average([(10.0, 50.0), (20.0, 100.0)])
    return {
        "gt_vs_layers": vs,
        "gt_n_layers": n,
        "gt_cu_layers": cu,
        "gt_class_vs_layers": by(vs, [180.0, 360.0, 800.0]),
        "gt_class_vs_900": by(900.0, [180.0, 360.0, 800.0]),
        "gt_class_vs_800": by(800.0, [180.0, 360.0, 800.0]),
        "gt_class_vs_360": by(360.0, [180.0, 360.0, 800.0]),
        "gt_class_vs_120": by(120.0, [180.0, 360.0, 800.0]),
        "gt_class_n_layers": by(n, [15.0, 50.0]),
        "gt_class_n_15": by(15.0, [15.0, 50.0]),
        "gt_class_n_60": by(60.0, [15.0, 50.0]),
        "gt_class_cu_layers": by(cu, [70.0, 250.0]),
        "gt_class_cu_250": by(250.0, [70.0, 250.0]),
        "gt_class_cu_60": by(60.0, [70.0, 250.0]),
        "gt_depth_continued": average([(10.0, 200.0), (15.0, 300.0), (5.0, 300.0)]),
        "gt_depth_cut": average([(20.0, 200.0), (10.0, 400.0)]),
        "gt_class_alluvium": 4,
        "gt_class_soft_clay": 5,
        "gt_class_liquefiable": 6,
    }


def masonry_reference() -> dict[str, float]:
    """🧱 Simple masonry wall-area densities by numpy: the wall rows (length, thickness, axis angle, storey row id with "" = every storey) split by the dominant axis component per direction and storey row over the floor area, and the governing minimum over both directions and the storey rows."""
    import numpy as np

    rows = [(6.0, 0.3, 0.0, "s1"), (4.0, 0.3, 0.0, "s1"), (5.0, 0.3, math.pi / 2, "s1"), (3.0, 0.3, math.pi / 2, ""), (2.0, 0.25, 0.3, "s2")]
    length, thickness, angle = np.array([r[:3] for r in rows]).T
    storey = np.array([r[3] for r in rows])
    area, runs_y = 80.0, np.abs(np.sin(angle)) > np.abs(np.cos(angle))

    def density(along_y: bool, level: int) -> float:
        mask = ((storey == "") | (storey == f"s{level}")) & (runs_y == along_y)
        return float(np.sum(length[mask] * thickness[mask]) / area)

    return {
        "mw_x_1": density(False, 1),
        "mw_y_1": density(True, 1),
        "mw_x_2": density(False, 2),
        "mw_y_2": density(True, 2),
        "mw_governing_1": min(density(False, 1), density(True, 1)),
        "mw_governing_2": min(density(a, level) for a in (False, True) for level in (1, 2)),
    }


def cantilever_reference() -> dict[str, float]:
    """🗼 Uniform cantilever modes: the roots of 1 + cos x cosh x = 0 by brentq, the tip-normalised shape and the participation by adaptive quadrature."""
    from scipy import integrate, optimize

    roots = [optimize.brentq(lambda x: 1.0 + math.cos(x) * math.cosh(x), a, b) for a, b in ((1.5, 2.5), (4.0, 5.0), (7.0, 8.0))]

    def shape(n: int, x: float) -> float:
        b = roots[n - 1]
        sigma = (math.cosh(b) + math.cos(b)) / (math.sinh(b) + math.sin(b))
        raw = lambda t: math.cosh(b * t) - math.cos(b * t) - sigma * (math.sinh(b * t) - math.sin(b * t))
        return raw(x) / raw(1.0)

    out = {f"cantilever_root_{n}": roots[n - 1] for n in (1, 2, 3)}
    for n in (1, 2, 3):
        out[f"cantilever_shape_{n}_half"] = shape(n, 0.5)
        out[f"cantilever_participation_{n}"] = integrate.quad(lambda x: shape(n, x), 0, 1)[0] / integrate.quad(lambda x: shape(n, x) ** 2, 0, 1)[0]
        out[f"cantilever_period_ratio_{n}"] = (roots[0] / roots[n - 1]) ** 2
    return out


def _cqc(values: list[float], periods: list[float], xi: float = 0.05) -> float:
    w = [2.0 * math.pi / t for t in periods]

    def rho(i: int, j: int) -> float:
        r = w[j] / w[i]
        return 8 * xi**2 * (1 + r) * r**1.5 / ((1 - r**2) ** 2 + 4 * xi**2 * r * (1 + r) ** 2)

    return math.sqrt(sum(rho(i, j) * values[i] * values[j] for i in range(len(values)) for j in range(len(values))))


def _design_sd(a_g: float, s: float, tb: float, tc: float, td: float, q: float, t: float) -> float:
    base = a_g * s
    if t <= tb:
        return base * (2 / 3 + t / tb * (2.5 / q - 2 / 3))
    if t <= tc:
        return base * 2.5 / q
    return max(base * 2.5 / q * (tc / t if t <= td else tc * td / t**2), 0.2 * a_g)


def bridge_reference() -> dict[str, float]:
    """🌉 Bridge on pier and bearing springs: cantilever stiffness 3EI/H^3 in series with the bearings, rigid-deck period, and a deck beam on the springs assembled from Hermite beam elements and solved by scipy.linalg.eigh (lumped masses), CQC-combined support forces and displacements, seat length."""
    import numpy as np
    from scipy import linalg

    e_pier, b, h, height = 30000.0e6, 2.0, 1.5, 8.0
    k_long = 3 * e_pier * (b * h**3 / 12) / height**3
    k_trans = 3 * e_pier * (h * b**3 / 12) / height**3
    bearing_pier, bearing_abutment = 2 * 5.0e7, 2.0e7
    series = lambda kp, kb: 1 / (1 / kp + 1 / kb)
    xs = [0.0, 10.0, 30.0, 40.0]
    k_l = [bearing_abutment, series(k_long, bearing_pier), series(k_long, bearing_pier), bearing_abutment]
    k_t = [bearing_abutment, series(k_trans, bearing_pier), series(k_trans, bearing_pier), bearing_abutment]
    mass, length, ei = 9.0e5, 40.0, 5.0e11
    a_g, s, tb, tc, td, q = 0.6, 1.25, 0.05, 0.25, 2.0, 1.5
    out = {"br_k_pier_long": k_long, "br_k_pier_trans": k_trans, "br_k_support_long": k_l[1], "br_k_total_long": sum(k_l)}
    period_long = 2 * math.pi * math.sqrt(mass / sum(k_l))
    force = mass * _design_sd(a_g, s, tb, tc, td, q, period_long)
    out |= {"br_t_long": period_long, "br_v_long": force, "br_f_pier_long": force * k_l[1] / sum(k_l), "br_u_long": elastic_se(a_g, s, tb, tc, td, period_long) / (2 * math.pi / period_long) ** 2}
    key = sorted(set(xs + [0.0, length]))
    nodes = [key[0]]
    for a, c in zip(key[:-1], key[1:]):
        n = max(1, math.ceil((c - a) / (length / 20)))
        nodes += [a + (c - a) * i / n for i in range(1, n + 1)]
    n = len(nodes)
    stiffness = np.zeros((2 * n, 2 * n))
    for i in range(n - 1):
        l = nodes[i + 1] - nodes[i]
        stiffness[2 * i:2 * i + 4, 2 * i:2 * i + 4] += ei / l**3 * np.array([[12, 6 * l, -12, 6 * l], [6 * l, 4 * l * l, -6 * l, 2 * l * l], [-12, -6 * l, 12, -6 * l], [6 * l, 2 * l * l, -6 * l, 4 * l * l]])
    index = [int(np.argmin([abs(x - y) for y in nodes])) for x in xs]
    for i, k in zip(index, k_t):
        stiffness[2 * i, 2 * i] += k
    lumps = np.array([mass / length * ((nodes[min(i + 1, n - 1)] - nodes[max(i - 1, 0)]) / 2) for i in range(n)])
    t_dofs, r_dofs = list(range(0, 2 * n, 2)), list(range(1, 2 * n, 2))
    condensed = stiffness[np.ix_(t_dofs, t_dofs)] - stiffness[np.ix_(t_dofs, r_dofs)] @ np.linalg.solve(stiffness[np.ix_(r_dofs, r_dofs)], stiffness[np.ix_(r_dofs, t_dofs)])
    w2, vectors = linalg.eigh(condensed, np.diag(lumps))
    order = np.argsort(w2)[:4]
    periods, gammas, ratios, shapes = [], [], [], []
    for j in order:
        psi = vectors[:, j]
        linear, quadratic = float(psi @ lumps), float(psi @ (lumps * psi))
        periods.append(2 * math.pi / math.sqrt(w2[j]))
        gammas.append(linear / quadratic)
        ratios.append(linear**2 / quadratic / lumps.sum())
        peak = psi[int(np.argmax(np.abs(psi)))]
        shapes.append(psi / peak)
    forces = [[k_t[i] * gammas[m] * (vectors[:, order[m]][index[i]]) * _design_sd(a_g, s, tb, tc, td, q, periods[m]) / (2 * math.pi / periods[m]) ** 2 for m in range(4)] for i in range(4)]
    disp = [[gammas[m] * vectors[:, order[m]][index[i]] * elastic_se(a_g, s, tb, tc, td, periods[m]) / (2 * math.pi / periods[m]) ** 2 for m in range(4)] for i in range(4)]
    out |= {f"br_t_trans_{m + 1}": periods[m] for m in range(3)} | {f"br_meff_trans_{m + 1}": ratios[m] for m in range(2)}
    out |= {"br_meff_trans_sum": sum(ratios), "br_f_pier_trans": _cqc(forces[1], periods), "br_f_abutment_trans": _cqc(forces[0], periods), "br_u_pier_trans": _cqc(disp[1], periods), "br_v_trans": _cqc([sum(forces[i][m] for i in range(4)) for m in range(4)], periods)}
    d_g = 0.025 * a_g * s * tc * td
    out |= {"br_d_g": d_g, "br_seat": 0.4 + 2.0 * d_g + out["br_u_long"], "br_m_ed_pier": out["br_f_pier_long"] * height, "br_v_cd": 1.35 * 6.0e6 / height}
    out |= bridge_action_reference(a_g)
    return out


def bridge_action_reference(a_g: float) -> dict[str, float]:
    """🌉 EN 1998-2 action rules (BR-01, 02, 03, 05, 06) from the repository spec with plain arithmetic and itertools: gamma_I class III, the vertical spectrum at three periods, q = 3.5 lambda(alpha_s), the largest permutation of the component combination and the cracked pier stiffness."""
    from itertools import permutations
    avg = 0.7 * 1.3 * a_g
    vertical = lambda t: avg * (1 + t / 0.05 * 2) if t <= 0.05 else avg * 3 if t <= 0.15 else avg * 3 * 0.15 / t if t <= 1.0 else avg * 3 * 0.15 / t**2
    q_of = lambda alpha: None if alpha < 1.0 else 3.5 if alpha >= 3.0 else 3.5 * math.sqrt(alpha / 3.0)
    ex, ey, ez = 5.0e6, -2.0e6, 1.0e6
    combined = max(abs(m[0]) + 0.3 * abs(m[1]) + 0.3 * abs(m[2]) for m in permutations((ex, ey, ez)))
    k_gross = 3 * 30000.0e6 * (2.0 * 1.5**3 / 12.0) / 8.0**3
    return {"br_gamma_i_iii": 1.3, "br_s_ve_005": vertical(0.05), "br_s_ve_010": vertical(0.10), "br_s_ve_050": vertical(0.5), "br_s_ve_200": vertical(2.0), "br_alpha_s": 8.0 / 2.0, "br_q_alpha_2": q_of(2.0), "br_q_alpha_5": q_of(5.0), "br_q_alpha_1": q_of(1.0), "br_combined": combined, "br_k_cracked_half": 0.5 * k_gross}


def tank_reference() -> dict[str, float]:
    """🛢 Rigid circular tank: wall pressure of the impulsive mode (Veletsos series with modified Bessel functions of scipy.special) and of the first sloshing mode, their wall resultants by quadrature, hoop stress and sloshing wave."""
    from scipy import integrate, special

    height, radius, rho, a_i, a_c = 9.0, 5.0, 649.0, 1.875, 0.2
    eps = 1.8411837813406593

    def impulsive(z: float, theta: float = 0.0) -> float:
        total = 0.0
        for n in range(1, 13):
            lam = (2 * n - 1) * math.pi / 2
            total += (-1) ** (n - 1) / lam**2 * special.iv(1, lam * radius / height) / special.ivp(1, lam * radius / height) * math.cos(lam * z / height)
        return 2 * rho * height * a_i * total * math.cos(theta)

    def convective(z: float, theta: float = 0.0) -> float:
        return 2 * rho * radius / (eps * eps - 1) * math.cosh(eps * z / radius) / math.cosh(eps * height / radius) * a_c * math.cos(theta)

    mass = rho * math.pi * radius**2 * height
    out = {"tk_p_imp_0": impulsive(0.0), "tk_p_imp_mid": impulsive(height / 2), "tk_p_imp_top": impulsive(height), "tk_p_imp_45": impulsive(0.0, math.pi / 4), "tk_p_conv_top": convective(height), "tk_p_conv_0": convective(0.0), "tk_p_hydro_0": rho * 9.81 * height}
    out["tk_mi_series"] = math.pi * radius * integrate.quad(lambda z: impulsive(z), 0, height)[0] / (a_i * mass)
    out["tk_mc_series"] = math.pi * radius * integrate.quad(lambda z: convective(z), 0, height)[0] / (a_c * mass)
    out["tk_mc_closed"] = 2 * radius * math.tanh(eps * height / radius) / (height * eps * (eps * eps - 1))
    out["tk_bessel_i1_5"] = float(special.iv(1, 5.0))
    out["tk_bessel_i1_12"] = float(special.iv(1, 12.0))
    out["tk_ratio_5"] = float(special.iv(1, 5.0) / special.ivp(1, 5.0))
    out["tk_j1_1p2"] = float(special.jv(1, 1.2))
    out["tk_eta_half"] = 0.84 * radius * a_c / 9.81 * special.jv(1, eps * 0.5) / special.jv(1, eps) * math.cos(math.pi / 3)
    out["tk_slosh"] = 0.84 * radius * a_c / 9.81
    out["tk_hoop_mpa"] = 63.5e3 * radius / 0.01 / 1.0e6
    out["tk_t_req"] = 63.5e3 * radius / (200.0e6)
    return out


def tower_reference() -> dict[str, float]:
    """🗼 Cantilever on a rotational base spring: the roots of the 3x3 frequency determinant by brentq (exact), and the same discretisation as the kernel (24 Hermite elements, lumped masses) solved by scipy.linalg.eigh with effective modal masses."""
    import numpy as np
    from scipy import linalg, optimize

    ei, mass, height = 2.0e10, 1.0e5, 45.0
    mu = mass / height

    def determinant(beta: float, kappa: float) -> float:
        c, sn, ch, sh = math.cos(beta), math.sin(beta), math.cosh(beta), math.sinh(beta)
        return float(np.linalg.det(np.array([[-2 * beta, -kappa, -kappa], [-(c + ch), -sn, sh], [sn - sh, -c, ch]])))

    def exact_periods(k_theta: float) -> list[float]:
        kappa = k_theta * height / ei if k_theta > 0 else 1.0e12
        grid = np.linspace(0.05, 12, 3000)
        values = [determinant(x, kappa) for x in grid]
        roots = [optimize.brentq(determinant, grid[i], grid[i + 1], args=(kappa,)) for i in range(len(grid) - 1) if values[i] * values[i + 1] < 0]
        return [2 * math.pi / (r * r) * math.sqrt(mu * height**4 / ei) for r in roots[:3]]

    def discretised(k_theta: float, elements: int = 24):
        h = height / elements
        nodes = elements + 1
        k = np.zeros((2 * nodes, 2 * nodes))
        element = ei / h**3 * np.array([[12, 6 * h, -12, 6 * h], [6 * h, 4 * h * h, -6 * h, 2 * h * h], [-12, -6 * h, 12, -6 * h], [6 * h, 2 * h * h, -6 * h, 4 * h * h]])
        for i in range(elements):
            k[2 * i:2 * i + 4, 2 * i:2 * i + 4] += element
        if k_theta > 0:
            k[1, 1] += k_theta
        lumps = np.array([mass / elements * (0.5 if i in (0, elements) else 1.0) for i in range(nodes)])
        free_translation = list(range(2, 2 * nodes, 2))
        rotations = [1] + list(range(3, 2 * nodes, 2)) if k_theta > 0 else list(range(3, 2 * nodes, 2))
        condensed = k[np.ix_(free_translation, free_translation)] - k[np.ix_(free_translation, rotations)] @ np.linalg.solve(k[np.ix_(rotations, rotations)], k[np.ix_(rotations, free_translation)])
        m = lumps[1:]
        w2, vectors = linalg.eigh(condensed, np.diag(m))
        periods, ratios, gammas = [], [], []
        for j in range(3):
            psi = vectors[:, j]
            tip = psi[-1]
            shape = psi / tip
            linear, quadratic = float(shape @ m), float(shape @ (m * shape))
            periods.append(2 * math.pi / math.sqrt(w2[j]))
            gammas.append(linear / quadratic)
            ratios.append(float(psi @ m) ** 2 / float(psi @ (m * psi)) / lumps.sum())
        return periods, ratios, gammas

    out = {}
    for label, k_theta in (("fixed", 0.0), ("spring", 1.0e10)):
        exact = exact_periods(k_theta)
        periods, ratios, gammas = discretised(k_theta)
        out |= {f"tw_{label}_exact_{j + 1}": exact[j] for j in range(3)} | {f"tw_{label}_t_{j + 1}": periods[j] for j in range(3)} | {f"tw_{label}_meff_{j + 1}": ratios[j] for j in range(3)} | {f"tw_{label}_gamma_{j + 1}": gammas[j] for j in range(3)}
    out["tw_lengthening"] = out["tw_spring_t_1"] / out["tw_fixed_t_1"]
    return out


WALL_PLAN_ROWS = [(1.0, 2.0, 6.0, 0.0), (9.0, 1.0, 5.0, math.pi / 2), (2.0, 10.0, 4.0, 0.0), (3.0, 3.0, 4.0, 0.5)]


def wall_stiffness(length: float, thickness: float, height: float, poisson: float = 0.25) -> float:
    """🧱 Cantilever wall tip stiffness for E = 1 and G = E / (2 (1 + nu)) from the virtual work integrals of bending and shear (scipy quad); nu = 0.25 gives G = 0.4 E."""
    from scipy import integrate
    bending = integrate.quad(lambda x: (height - x) ** 2 / (thickness * length**3 / 12.0), 0.0, height)[0]
    shear = integrate.quad(lambda x: 1.0 / (thickness * length / (2.0 * (1.0 + poisson))), 0.0, height)[0]
    return 1.0 / (bending + shear)


def wall_plan_reference() -> dict[str, float]:
    """🌀 Rigid diaphragm stiffness centre and torsional radii of positioned walls: 3 x 3 assembly by numpy, the centre as the least squares zero of the translation-rotation coupling (scipy)."""
    import numpy as np
    from scipy import optimize
    height, thickness = 3.2, 0.3
    k = np.zeros((3, 3))
    for x0, y0, length, direction in WALL_PLAN_ROWS:
        xc, yc = x0 + 0.5 * length * math.cos(direction), y0 + 0.5 * length * math.sin(direction)
        a = np.array([math.cos(direction), math.sin(direction), xc * math.sin(direction) - yc * math.cos(direction)])
        k += wall_stiffness(length, thickness, height) * np.outer(a, a)

    def coupling(p: np.ndarray) -> np.ndarray:
        shift = np.array([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [p[1], -p[0], 1.0]])
        moved = shift @ k @ shift.T
        return moved[2, :2]

    centre = optimize.least_squares(coupling, np.array([5.0, 5.0]), xtol=1e-14, ftol=1e-14, gtol=1e-14).x
    shift = np.array([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [centre[1], -centre[0], 1.0]])
    moved = shift @ k @ shift.T
    return {
        "wall_plan_k_l6": wall_stiffness(6.0, thickness, height),
        "wall_plan_k_l6_nu02": wall_stiffness(6.0, thickness, height, 0.2),
        "wall_plan_cs_x": float(centre[0]),
        "wall_plan_cs_y": float(centre[1]),
        "wall_plan_k_x": float(k[0, 0]),
        "wall_plan_k_y": float(k[1, 1]),
        "wall_plan_r_x": math.sqrt(moved[2, 2] / k[1, 1]),
        "wall_plan_r_y": math.sqrt(moved[2, 2] / k[0, 0]),
    }


CRACKED_WALL_FACTORS = (0.5, 1.0, 1.0, 0.5)


def cracked_reference() -> dict[str, float]:
    """🪨️ Cracked stiffness (EN 1998-1 4.3.1(6), factor 0.5 unverified): the generalised eigenproblem of a shear building with the gross and the halved storey stiffnesses (scipy eigh) gives the period ratio 1/sqrt(0.5), and walls with different factors move the stiffness centre of the 3 x 3 diaphragm assembly while the least squares zero is found by scipy."""
    import numpy as np
    from scipy import optimize
    from scipy.linalg import eigh

    m = np.array([200.0e3, 200.0e3, 150.0e3])
    k = np.array([80.0e6, 70.0e6, 60.0e6])

    def first_period(stiffness: np.ndarray) -> float:
        matrix = np.array([[stiffness[0] + stiffness[1], -stiffness[1], 0.0], [-stiffness[1], stiffness[1] + stiffness[2], -stiffness[2]], [0.0, -stiffness[2], stiffness[2]]])
        return 2 * math.pi / math.sqrt(eigh(matrix, np.diag(m), eigvals_only=True)[0])

    height, thickness = 3.2, 0.3
    assembled = np.zeros((3, 3))
    for (x0, y0, length, direction), factor in zip(WALL_PLAN_ROWS, CRACKED_WALL_FACTORS):
        xc, yc = x0 + 0.5 * length * math.cos(direction), y0 + 0.5 * length * math.sin(direction)
        a = np.array([math.cos(direction), math.sin(direction), xc * math.sin(direction) - yc * math.cos(direction)])
        assembled += factor * wall_stiffness(length, thickness, height) * np.outer(a, a)

    def coupling(p: np.ndarray) -> np.ndarray:
        shift = np.array([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [p[1], -p[0], 1.0]])
        return (shift @ assembled @ shift.T)[2, :2]

    centre = optimize.least_squares(coupling, np.array([5.0, 5.0]), xtol=1e-14, ftol=1e-14, gtol=1e-14).x
    shift = np.array([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [centre[1], -centre[0], 1.0]])
    moved = shift @ assembled @ shift.T
    return {
        "cracked_t1_gross": first_period(k),
        "cracked_t1_half": first_period(0.5 * k),
        "cracked_t1_ratio": first_period(0.5 * k) / first_period(k),
        "cracked_wall_cs_x": float(centre[0]),
        "cracked_wall_cs_y": float(centre[1]),
        "cracked_wall_k_x": float(assembled[0, 0]),
        "cracked_wall_k_y": float(assembled[1, 1]),
        "cracked_wall_r_x": math.sqrt(moved[2, 2] / assembled[1, 1]),
        "cracked_wall_r_y": math.sqrt(moved[2, 2] / assembled[0, 0]),
    }


JOINT_ROWS = [((5.0e6, 4.0e6), (2.5e6, 1.5e6)), ((3.0e6, 3.0e6), (2.5e6, 2.5e6)), ((4.0e6, 3.5e6), (2.0e6, 0.5e6))]


def joint_reference() -> dict[str, float]:
    """🔩 Strong column weak beam at three joints (EN 1998-1 4.4.2.3): every joint lists the resistances of the columns above and below and of the beams left and right [N m]; numpy sums them per joint, applies gamma_Rd = 1.3 to the beams and returns the ratio, the demand, the utilisation and the largest beam resistance that still satisfies the rule."""
    import numpy as np
    rows = np.array(JOINT_ROWS)
    columns, beams = rows[:, 0, :].sum(axis=1), rows[:, 1, :].sum(axis=1)
    demand = 1.3 * beams
    out = {}
    for i in range(len(JOINT_ROWS)):
        out |= {f"joint_sum_rc_{i}": float(columns[i]), f"joint_sum_rb_{i}": float(beams[i]), f"joint_ratio_{i}": float(columns[i] / demand[i]), f"joint_demand_{i}": float(demand[i]), f"joint_utilisation_{i}": float(demand[i] / columns[i]), f"joint_max_beam_{i}": float(columns[i] / 1.3)}
    return out


def exact_t1(masses: list[float], stiffness: list[float]) -> float:
    """🎻 Fundamental period of a shear building from the generalised eigenproblem K x = w² M x (scipy eigh); the Rayleigh frequency of the weight-proportional load lies above the exact one, its period just below."""
    import numpy as np
    from scipy.linalg import eigh
    n = len(masses)
    k = np.zeros((n, n))
    for i, ki in enumerate(stiffness):
        k[i, i] += ki
        if i + 1 < n:
            k[i + 1, i + 1] += ki
            k[i, i + 1] -= ki
            k[i + 1, i] -= ki
    w2 = eigh(k, np.diag(masses), eigvals_only=True)
    return 2.0 * math.pi / math.sqrt(w2[0])


def tie_reference() -> dict[str, float]:
    """🔗 EN 1998-5 5.4.1.2 tie beams: N = coefficient alpha S N_Ed with 0.3, 0.4, 0.6 for ground types B, C, D, the steel resistance A_s f_yd by sympy, the load factor at the resistance by scipy brentq; independent closed form, no library covers the clause."""
    import sympy
    from scipy import optimize

    alpha, s_soil, n_ed, a_s, f_yd = sympy.symbols("alpha S N_Ed A_s f_yd", positive=True)
    force = alpha * s_soil * n_ed
    values = {"B": 0.3, "C": 0.4, "D": 0.6}
    out = {f"tie_force_{t.lower()}": float(force.subs({alpha: c, s_soil: 1.2, n_ed: 2.0e6})) for t, c in values.items()}
    resistance = float((a_s * f_yd).subs({a_s: 6.0e-4, f_yd: 435.0e6}))
    out["tie_util_c"] = out["tie_force_c"] / resistance
    out["tie_load_factor_c"] = optimize.brentq(lambda x: x * out["tie_force_c"] - resistance, 0.0, 10.0)
    return out


def silo_reference() -> dict[str, float]:
    """🏭 EN 1998-4 3 silo: N_Ed = m (g + 0.9 a_g) against N_Rd and F_h = m S_d(T) against V_Rd with the filled mass of the storage psi_E; independent closed form, no library covers the clause."""
    import sympy

    m, g, a_g, s_d = sympy.symbols("m g a_g S_d", positive=True)
    point = {m: 1.5e6 / 9.81, g: 9.81, a_g: 0.6, s_d: 0.6 * 1.25 * 2.5 / 1.5}
    return {"silo_n_ed": float((m * (g + 0.9 * a_g)).subs(point)), "silo_f_h": float((m * s_d).subs(point))}


def formulas() -> dict[str, float]:
    """📘️ Independent reference values: closed forms and a characteristic polynomial for the two storey shear building."""
    a_g, s, tb, tc, td = 0.6, 1.25, 0.05, 0.25, 2.0

    def sd(q: float, t: float) -> float:
        base = a_g * s
        if t <= tb:
            return base * (2 / 3 + t / tb * (2.5 / q - 2 / 3))
        if t <= tc:
            return base * 2.5 / q
        return max(base * 2.5 / q * (tc / t if t <= td else tc * td / t**2), 0.2 * a_g)

    def sve(t: float) -> float:
        avg = 0.9 * a_g
        if t <= 0.05:
            return avg * (1 + t / 0.05 * 2)
        if t <= 0.15:
            return avg * 3
        return avg * 3 * (0.15 / t if t <= 1.0 else 0.15 / t**2)

    def rho(xi: float, wi: float, wj: float) -> float:
        r = wj / wi
        return 8 * xi**2 * (1 + r) * r**1.5 / ((1 - r**2) ** 2 + 4 * xi**2 * r * (1 + r) ** 2)

    pi = math.pi
    w = [2 * pi / 0.5, 2 * pi / 0.45]
    modal = [3.0, 4.0]
    cqc = math.sqrt(sum(rho(0.05, w[i], w[j]) * modal[i] * modal[j] for i in range(2) for j in range(2)))
    m, k = 1.0e5, 4.0e7
    lam1, lam2 = (3 - math.sqrt(5)) / 2 * k / m, (3 + math.sqrt(5)) / 2 * k / m
    alpha = 0.06 * 1.25 * (3 * (1 + 8 / 12) / (1 + (1 - 0.2 / 0.5) ** 2) - 0.5)
    masses, k_storey, g = [4.0e5] * 4, 1.2e9, 9.81
    deflection, run = [], 0.0
    for j in range(4):
        run += sum(masses[j:]) * g / k_storey
        deflection.append(run)
    rayleigh = 2 * pi * math.sqrt(sum(m * d * d for m, d in zip(masses, deflection)) / (g * sum(m * d for m, d in zip(masses, deflection))))
    phi_ = math.radians(30.0)
    theta_ = math.atan(0.2)
    kae = math.cos(phi_ - theta_) ** 2 / (math.cos(theta_) ** 2 * (1 + math.sqrt(math.sin(phi_) * math.sin(phi_ - theta_) / math.cos(theta_))) ** 2)
    def mo_active(phi_deg, theta):
        phi = math.radians(phi_deg)
        return math.cos(phi - theta) ** 2 / (math.cos(theta) ** 2 * (1 + math.sqrt(math.sin(phi) * math.sin(phi - theta) / math.cos(theta))) ** 2)

    gamma_w, gamma_d, gamma_sat, kh_w, wet, height = 9810.0, 18000.0, 20000.0, 0.2, 3.0, 4.0
    dry_h, eff = height - wet, gamma_sat - gamma_w
    k_dry = mo_active(32.0, math.atan(kh_w))
    soil_permeable = 0.5 * gamma_d * dry_h**2 * k_dry + mo_active(32.0, math.atan(gamma_d / eff * kh_w)) * (gamma_d * dry_h * wet + 0.5 * eff * wet**2)
    soil_low = 0.5 * gamma_d * dry_h**2 * k_dry + mo_active(32.0, math.atan(gamma_sat / eff * kh_w)) * (gamma_d * dry_h * wet + 0.5 * eff * wet**2)
    hydrostatic = 0.5 * gamma_w * wet**2
    westergaard = 7 / 12 * kh_w * gamma_w * wet**2
    sliding_friction = max(2.0e7 - 40000.0 * 200.0, 0.0) * math.tan(math.radians(25.0)) / 1.0
    return {
        "rayleigh_4_storeys": rayleigh,
        "exact_t1_4_storeys": exact_t1([4.0e5] * 4, [k_storey] * 4),
        "psi_e_retail_storey": 1.0 * 0.6,
        "psi_e_office_storey": 0.8 * 0.3,
        "kae_kh_0_2": kae,
        "wall_thrust_permeable": soil_permeable + hydrostatic + westergaard,
        "wall_thrust_low_permeability": soil_low + hydrostatic,
        "westergaard_thrust": westergaard,
        "sliding_friction_n": sliding_friction,
        "sd_below_tb_q39": 0.6 * 1.25 * (2 / 3 + 0.02 / 0.05 * (2.5 / 3.9 - 2 / 3)),
        "sd_floor_gamma_1_4": max(0.6 * 1.4 * 1.25 * 2.5 / 3.0 * 0.25 * 2.0 / 9.0, 0.2 * 0.6 * 1.4),
        "sd_q3_t0_02": sd(3, 0.02),
        "sd_q3_t0_15": sd(3, 0.15),
        "sd_q3_t0_6": sd(3, 0.6),
        "sd_q3_t3": sd(3, 3.0),
        "sve_t0_03": sve(0.03),
        "sve_t0_1": sve(0.1),
        "sve_t0_5": sve(0.5),
        "sve_t2": sve(2.0),
        "delta_x5_le10": 1 + 0.6 * 5 / 10,
        "drift_util": 0.012 * 0.5 / (0.0075 * 3.5),
        "cqc_rho_0_9": rho(0.05, 1.0, 0.9),
        "srss_3_4": 5.0,
        "cqc_two_modes": cqc,
        "envelope": max(100 + 0.3 * 50, 0.3 * 100 + 50),
        "cantilever_period": cantilever_period(5.0e5, 45.0, 2.0e10),
        "liq_rd_6": 1.0 - 0.00765 * 6.0,
        "liq_rd_15": 1.174 - 0.0267 * 15.0,
        "liq_msf_6_5": 10**2.24 / 6.5**2.56,
        "liq_n1_60cs_fc20": liq_n1_60cs(12.0, 20.0),
        "liq_crr_12": liq_crr(12.0),
        "liq_factor": liq_factor(),
        "pier_shear": 1.35 * 4.0e6 / 8.0,
        "timber_util": max(14.0 / 12.0, 8 * 14.0 / 100.0),
        "masonry_util": max(3.6 / 0.3 / 12.0, 0.24 / 0.3),
        "vs30_two_layers": 30 / (10 / 200 + 20 / 400),
        "slenderness_util_30_10": (30 / 10) / 4,
        "envelope_util_0_06": 0.06 / 0.05,
        "setback_util_0_3": 0.3 / 0.20,
        "strong_column_ratio": 1.0e6 / (1.3 * 6.0e5),
        "two_storey_t1": 2 * pi / math.sqrt(lam1),
        "two_storey_t2": 2 * pi / math.sqrt(lam2),
        "two_storey_meff_sum": 2 * m,
        "fa": max(alpha, 0.06 * 1.25) * 1000.0 * 1.0 / 2.0,
    } | rc_reference() | liquefaction_reference() | annex_f_reference() | pile_reference() | n2_reference() | cantilever_reference() | soil_profile_reference() | time_history_reference() | tie_reference() | silo_reference() | cracked_reference() | wall_plan_reference() | joint_reference() | horizontal_reference() | ground_reference() | masonry_reference() | bridge_reference() | tank_reference() | tower_reference()


SPEC_ROWS = {
    "SEI-06": {"checks": [".timeHistory."], "library": "scipy.signal.lsim", "reference": "horizontal_reference", "keys": ("th_",)},
    "SEI-48": {"checks": [".diaphragm"], "library": "scipy.integrate.quad", "reference": "horizontal_reference", "keys": ("dia_",)},
    "SEI-52": {"checks": [".timberConnection", ".masonryWall", ".masonry.simple"], "library": "numpy", "reference": "masonry_reference", "keys": ("mw_", "timber_util", "masonry_util")},
    "GE-06": {"checks": [".annexF"], "library": "scipy.optimize.brentq", "reference": "annex_f_reference", "keys": ("annexf_",)},
    "GE-08": {"checks": [".tie", ".displacement"], "library": "closed form + sympy + scipy.optimize.brentq (EN 1998-5 5.4.1.2)", "reference": "tie_reference", "keys": ("tie_",)},
    "BR-04": {"checks": [".analysis.massParticipation"], "library": "scipy.linalg.eigh", "reference": "bridge_reference", "keys": ("br_t_trans_", "br_meff_trans_")},
    "BR-05": {"checks": [".action.stiffness"], "library": "concreteproperties + sectionproperties (🌉️pier-stiffness)", "reference": "bridge_action_reference", "keys": ("br_k_cracked_half", "br_k_pier_")},
    "AS-04": {"checks": [".chordRotation.derivation"], "library": "scipy.optimize.brentq", "reference": "rc_reference", "keys": ("rc_theta_y", "rc_phi_y", "rc_xi_y")},
    "AS-05": {"checks": [".chordRotation.derivation", ".chordRotation"], "library": "scipy.optimize.brentq", "reference": "rc_reference", "keys": ("rc_theta_um", "rc_alpha")},
    "ST-01": {"checks": [".wall", ".anchor"], "library": "closed form + sympy (EN 1998-4 3)", "reference": "silo_reference", "keys": ("silo_",)},
    "TW-03": {"checks": [".modal.massParticipation", ".modal.overturning"], "library": "scipy.linalg.eigh + scipy.optimize.brentq", "reference": "tower_reference", "keys": ("tw_",)},
}


def spec_rows() -> dict[str, dict[str, Any]]:
    """🧾 Spec row id -> check id fragment -> third-party library -> the reference values (keys of `--formulas`) that carry the cross-check; a row with no key would fail loudly."""
    values = formulas()
    out = {}
    for row, spec in SPEC_ROWS.items():
        keys = sorted(k for k in values if k.startswith(spec["keys"]))
        assert keys, f"{row}: no reference value for {spec['keys']}"
        assert all(math.isfinite(values[k]) for k in keys), row
        out[row] = {"checks": spec["checks"], "library": spec["library"], "reference": spec["reference"], "values": {k: values[k] for k in keys}}
    return out


def main() -> None:
    if "--spec-rows" in sys.argv:
        json.dump(spec_rows(), sys.stdout)
        return
    if "--formulas" in sys.argv:
        json.dump(formulas(), sys.stdout)
        return
    if "--json" in sys.argv:
        doc = json.load(sys.stdin)
        json.dump(evaluate(doc), sys.stdout)
        return
    if "--combo-table" in sys.argv:
        out = {}
        for combo in ["A-R", "B-R", "C-R", "B-T", "C-T", "C-S"]:
            s, tb, tc, td = spectrum_params(combo)
            se = elastic_se(0.6, s, tb, tc, td, 0.25)
            out[combo] = {"S": s, "TB": tb, "TC": tc, "TD": td, "Se025": se}
        json.dump(out, sys.stdout)
        return
    se = elastic_se(0.6, 1.25, 0.05, 0.25, 2.0, 0.25)
    assert abs(se - 1.875) < 1e-9, se
    s_cs, _, _, _ = spectrum_params("C-S")
    assert abs(s_cs - 0.75) < 1e-12, s_cs
    print("ok", se)


if __name__ == "__main__":
    main()
