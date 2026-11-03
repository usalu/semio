"""🧮️ SymPy adapter: exact symbolic solutions of the Eurocode equations, evaluated at the case inputs; no numerical root finding."""

import sympy as sp

LIBRARY = "sympy"


def column_chi(i):
    """🏛️ EN 1993-1-1 6.3.1.2: the physical root of the Ayrton-Perry equation (1 - chi)(1 - lambda^2 chi) = alpha (lambda - 0.2) chi, solved symbolically."""
    lam, alpha = sp.Rational(str(i["lambda_bar"])), sp.Rational(str(i["alpha"]))
    if lam <= sp.Rational(1, 5):
        return {"chi": 1.0}
    chi = sp.symbols("chi")
    roots = sp.solve(sp.Eq((1 - chi) * (1 - lam**2 * chi), alpha * (lam - sp.Rational(1, 5)) * chi), chi)
    return {"chi": float(min(r for r in roots if 0 < r <= 1))}


def euler_critical_load(i):
    """🧱️ Pin-ended Euler load from the eigenvalue of y'' + k^2 y = 0 on [0, L] with y(0) = y(L) = 0, k^2 = N / EI: the smallest positive root of sin(kL)."""
    k, length = sp.symbols("k L", positive=True)
    root = sp.solve(sp.sin(k * sp.Rational(str(i["L"]))), k)
    smallest = min(r for r in root if r.is_positive)
    return {"N_cr": float(i["EI"] * smallest**2)}


def reliability_index(i):
    """🎯️ EN 1990 Annex C beta = -Phi^-1(P_f) with the exact inverse error function: Phi^-1(p) = sqrt(2) erfinv(2 p - 1)."""
    pf = sp.Float(str(i["pf"]), 30)
    return {"beta": float(-sp.sqrt(2) * sp.erfinv(2 * pf - 1))}


def design_spectrum(i):
    """🌋️ EN 1998-1 3.2.2.5 design spectrum S_d(T) as a symbolic piecewise function of T (plateau, 1/T branch, 1/T^2 branch, lower bound beta a_g) evaluated exactly at the case period; `Se` is the elastic ordinate 3.2.2.2 with eta = 1."""
    t, ag, s, tb, tc, td, q, beta = sp.symbols("T a_g S T_B T_C T_D q beta", positive=True)
    plateau = ag * s * sp.Rational(5, 2) / q
    floor = beta * ag
    sd = sp.Piecewise(
        (ag * s * (sp.Rational(2, 3) + t / tb * (sp.Rational(5, 2) / q - sp.Rational(2, 3))), t <= tb),
        (plateau, t <= tc),
        (sp.Max(plateau * tc / t, floor), t <= td),
        (sp.Max(plateau * tc * td / t**2, floor), True),
    )
    se = sp.Piecewise((ag * s * (1 + t / tb * (sp.Rational(5, 2) - 1)), t <= tb), (ag * s * sp.Rational(5, 2), t <= tc), (ag * s * sp.Rational(5, 2) * tc / t, t <= td), (ag * s * sp.Rational(5, 2) * tc * td / t**2, True))
    values = {t: sp.Float(str(i["T"]), 30), ag: sp.Float(str(i["a_g"]), 30), s: sp.Float(str(i["S"]), 30), tb: sp.Float(str(i["T_B"]), 30), tc: sp.Float(str(i["T_C"]), 30), td: sp.Float(str(i["T_D"]), 30), q: sp.Float(str(i["q"]), 30), beta: sp.Float(str(i["beta"]), 30)}
    return {"S_d": float(sd.subs(values)), "S_e": float(se.subs(values))}


def form_linear(i):
    """🎯️ Closed form of the linear-normal limit state g = R - E: beta = (mu_R - mu_E) / sqrt(sigma_R^2 + sigma_E^2), alpha_R = sigma_R / sqrt(...), alpha_E = -sigma_E / sqrt(...), P_f = Phi(-beta) = erfc(beta / sqrt 2) / 2, design values R_d = mu_R - alpha_R beta sigma_R, E_d = mu_E - alpha_E beta sigma_E."""
    mr, sr, me, se = (sp.Float(str(i[k]), 30) for k in ("mu_R", "sigma_R", "mu_E", "sigma_E"))
    root = sp.sqrt(sr**2 + se**2)
    beta, ar, ae = (mr - me) / root, sr / root, -se / root
    return {"beta": float(beta), "alpha_R": float(ar), "alpha_E": float(ae), "R_d": float(mr - ar * beta * sr), "E_d": float(me - ae * beta * se), "P_f": float(sp.erfc(beta / sp.sqrt(2)) / 2)}


def shear_building_modal(i):
    """🏢️ Exact characteristic polynomial det(K - omega^2 M) of the shear building with rational stiffness and mass, roots by sympy at 30 digits; periods T_j = 2 pi / omega_j."""
    m, k = [sp.Rational(str(v)) for v in i["m"]], [sp.Rational(str(v)) for v in i["k"]]
    n = len(m)
    stiff = sp.zeros(n, n)
    for s in range(n):
        stiff[s, s] += k[s]
        if s + 1 < n:
            stiff[s, s] += k[s + 1]
            stiff[s, s + 1] = stiff[s + 1, s] = -k[s + 1]
    lam = sp.symbols("lam")
    poly = sp.Poly((stiff - lam * sp.diag(*m)).det(), lam)
    roots = sorted(sp.re(r) for r in poly.nroots(n=30))
    return {f"T_{j + 1}": float(2 * sp.pi / sp.sqrt(r)) for j, r in enumerate(roots)}


def modal_base_shear(i):
    """🌋️ EN 1998-1 4.3.3.3: shear-building modes at 30 digits by the mpmath symmetric eigensolver of M^-1/2 K M^-1/2, effective modal masses, the piecewise design spectrum S_d(T_j) of sympy, SRSS and CQC double sum with rho_jk = 8 xi^2 (1 + r) r^(3/2) / ((1 - r^2)^2 + 4 xi^2 r (1 + r)^2)."""
    mp = sp.mpmath
    mp.mp.dps = 30
    m, k = [mp.mpf(str(v)) for v in i["m"]], [mp.mpf(str(v)) for v in i["k"]]
    n = len(m)
    stiff = mp.zeros(n, n)
    for s in range(n):
        stiff[s, s] += k[s] + (k[s + 1] if s + 1 < n else 0)
        if s + 1 < n:
            stiff[s, s + 1] = stiff[s + 1, s] = -k[s + 1]
    scaled = mp.matrix(n, n)
    for a in range(n):
        for b in range(n):
            scaled[a, b] = stiff[a, b] / mp.sqrt(m[a] * m[b])
    values, vectors = mp.eigsy(scaled)
    order = sorted(range(n), key=lambda j: values[j])
    omega = [mp.sqrt(values[j]) for j in order]
    shapes = [[vectors[a, j] / mp.sqrt(m[a]) for a in range(n)] for j in order]
    t, ag, s_soil, tb, tc, td, q, beta = sp.symbols("T a_g S T_B T_C T_D q beta", positive=True)
    plateau = ag * s_soil * sp.Rational(5, 2) / q
    sd = sp.Piecewise((ag * s_soil * (sp.Rational(2, 3) + t / tb * (sp.Rational(5, 2) / q - sp.Rational(2, 3))), t <= tb), (plateau, t <= tc), (sp.Max(plateau * tc / t, beta * ag), t <= td), (sp.Max(plateau * tc * td / t**2, beta * ag), True))
    values_in = {ag: flo(i["a_g"]), s_soil: flo(i["S"]), tb: flo(i["T_B"]), tc: flo(i["T_C"]), td: flo(i["T_D"]), q: flo(i["q"]), beta: flo(i["beta"])}
    modal = []
    for j in range(n):
        phi = shapes[j]
        eff = sum(m[a] * phi[a] for a in range(n)) ** 2 / sum(m[a] * phi[a] ** 2 for a in range(n))
        period = 2 * mp.pi / omega[j]
        modal.append(eff * mp.mpf(str(sd.subs({**values_in, t: sp.Float(str(period), 30)}))))
    xi = mp.mpf(str(i["xi"]))
    def rho(a, b):
        r = omega[b] / omega[a]
        return 8 * xi**2 * (1 + r) * r ** mp.mpf("1.5") / ((1 - r**2) ** 2 + 4 * xi**2 * r * (1 + r) ** 2)
    out = {f"F_b_{j + 1}": float(modal[j]) for j in range(n)}
    out["V_srss"] = float(mp.sqrt(sum(x * x for x in modal)))
    out["V_cqc"] = float(mp.sqrt(sum(rho(a, b) * modal[a] * modal[b] for a in range(n) for b in range(n))))
    return out


def janssen_ode(i):
    """🫙 Symbolic solution of the Janssen slice equation d sigma_v / dz = gamma - (K mu / a) sigma_v with sigma_v(0) = 0 via dsolve, evaluated at the depth."""
    z = sp.symbols("z", positive=True)
    s = sp.Function("s")
    gamma, a, mu, k = (sp.Rational(str(i[x])) for x in ("gamma", "a_m", "mu", "K"))
    rate = k * mu / a
    sol = sp.dsolve(sp.Eq(s(z).diff(z), gamma - rate * s(z)), s(z), ics={s(0): 0}).rhs
    sigma_v = sol.subs(z, sp.Rational(str(i["depth"])))
    return {"p_vf": float(sigma_v), "p_hf": float(k * sigma_v), "p_wf": float(mu * k * sigma_v), "z_o": float(1 / rate), "p_ho": float(gamma * a / mu)}


def flo(value):
    """🔢️ A 30-digit symbolic float of a case input."""
    return sp.Float(str(value), 30)


def wind_peak_pressure(i):
    """💨️ EN 1991-1-4 4.3.2 and 4.5 evaluated symbolically in the order of the clauses: c_r = k_r ln(z / z_0), v_m = c_r c_o v_b, I_v = k_I / (c_o ln(z / z_0)), q_p = (1 + 7 I_v) rho v_m^2 / 2."""
    z, z0, z_min, v_b, rho = (flo(i[k]) for k in ("z", "z0", "z_min", "v_b", "rho"))
    c_o, k_i = flo(i["c_o"]), flo(i["k_I"])
    k_r = sp.Rational(19, 100) * (z0 / flo(i.get("z0_II", 0.05))) ** sp.Rational(7, 100)
    log = sp.log(sp.Max(z, z_min) / z0)
    c_r = k_r * log
    v_m = c_r * c_o * v_b
    i_v = k_i / (c_o * log)
    return {"c_r": float(c_r), "v_m": float(v_m), "I_v": float(i_v), "q_p": float((1 + 7 * i_v) * rho * v_m**2 / 2)}


def wind_cpe(i):
    """🧭️ EN 1991-1-4 7.2.2 Table 7.1 vertical walls as symbolic piecewise-linear functions of h/d through the printed break points and the log10 area interpolation of 7.1.2(2)."""
    hd, area = flo(i["h_d"]), flo(i["area"])
    pts = {"A": ((-1.2,) * 3, (-1.4,) * 3), "B": ((-0.8,) * 3, (-1.1,) * 3), "C": ((-0.5,) * 3, (-0.5,) * 3), "D": ((0.7, 0.8, 0.8), (1.0, 1.0, 1.0)), "E": ((-0.3, -0.5, -0.7), (-0.3, -0.5, -0.7))}
    x = (sp.Rational(1, 4), 1, 5)

    def piece(values):
        a, b, c = (sp.Rational(str(v)) for v in values)
        return sp.Piecewise((a, hd <= x[0]), (a + (b - a) * (hd - x[0]) / (x[1] - x[0]), hd <= x[1]), (b + (c - b) * (hd - x[1]) / (x[2] - x[1]), hd <= x[2]), (c, True))

    c10, c1 = (piece(v) for v in pts[i["zone"]])
    weight = sp.Min(sp.Max(sp.log(area, 10), 0), 1)
    return {"c_pe_10": float(c10), "c_pe_1": float(c1), "c_pe": float(c1 + (c10 - c1) * weight)}


def snow_shape_mono_pitch(i):
    """❄️ EN 1991-1-3 5.3.2 Table 5.2 as symbolic piecewise functions of the roof pitch alpha."""
    a = flo(i["alpha"])
    mu1 = sp.Piecewise((sp.Rational(4, 5), a <= 30), (sp.Rational(4, 5) * (60 - a) / 30, a < 60), (0, True))
    mu2 = sp.Piecewise((sp.Rational(4, 5) + sp.Rational(4, 5) * a / 30, a <= 30), (sp.Rational(8, 5), True))
    return {"mu_1": float(mu1), "mu_2": float(mu2), "s": float(mu1 * flo(i["C_e"]) * flo(i["C_t"]) * flo(i["s_k"]))}


def snow_drift(i, reading=0):
    """🌨️ EN 1991-1-3 5.3.6: mu_w = Max(Min((b_1 + b_2) / (2 h), 4, gamma h / s_k), 0.8) (reading 0, floor wins) or Min(Max((b_1 + b_2) / (2 h), 0.8), 4, gamma h / s_k) (reading 1) and l_s = Min(Max(2 h, 5), 15)."""
    h, b1, b2, gamma, sk = (flo(i[k]) for k in ("h", "b1", "b2", "gamma", "s_k"))
    if reading == 0:
        mu_w = sp.Max(sp.Min((b1 + b2) / (2 * h), 4, gamma * h / sk), sp.Rational(4, 5))
    else:
        mu_w = sp.Min(sp.Max((b1 + b2) / (2 * h), sp.Rational(4, 5)), 4, gamma * h / sk)
    return {"mu_w": float(mu_w), "l_s": float(sp.Min(sp.Max(2 * h, 5), 15)), "s_peak": float(mu_w * sk)}


def snow_drift_gamma_limited(i):
    """🔀 EN 1991-1-3 5.3.6 with gamma h / s_k capping mu_w below 0.8 (NDP reading 1)."""
    return snow_drift(i, reading=1)


def fire_parametric(i):
    """🔥️ EN 1991-1-2 Annex A.1 to A.3 with exact rational coefficients: Gamma = ((O / b) / (0.04 / 1160))^2, t* = Gamma t, theta_g = 20 + 1325 (1 - 0.324 e^{-0.2 t*} - 0.204 e^{-1.7 t*} - 0.472 e^{-19 t*}), cooling by the three branches of A.11 in t*_max."""
    gamma_f = (flo(i["O"]) / flo(i["b"]) * sp.Rational(1160, 1) / sp.Rational(4, 100)) ** 2
    t_lim = flo(i["t_lim"]) / 3600
    t_max = sp.Max(sp.Rational(2, 10000) * flo(i["q_td"]) / 10**6 / flo(i["O"]), t_lim)
    heat = lambda ts: 20 + 1325 * (1 - sp.Rational(324, 1000) * sp.exp(-ts / 5) - sp.Rational(204, 1000) * sp.exp(-sp.Rational(17, 10) * ts) - sp.Rational(472, 1000) * sp.exp(-19 * ts))
    ts_max, t = gamma_f * t_max, flo(i["t"]) / 3600
    theta_max = heat(ts_max)
    x = sp.Piecewise((1, t_max > t_lim), (t_lim * gamma_f / ts_max, True))
    slope = sp.Piecewise((625, ts_max <= sp.Rational(1, 2)), (250 * (3 - ts_max), ts_max < 2), (250, True))
    theta = sp.Piecewise((heat(gamma_f * t), t <= t_max), (sp.Max(theta_max - slope * (gamma_f * t - ts_max * x), 20), True))
    return {"Gamma": float(gamma_f), "t_max": float(t_max * 3600), "theta_max": float(theta_max), "theta_g": float(theta)}


def fire_iso834(i):
    """🔥️ ISO 834: 20 + 345 log10(8 t + 1) with sympy's base-10 logarithm."""
    return {"theta_g": float(20 + 345 * sp.log(8 * flo(i["t_min"]) + 1, 10))}


def masonry_phi_m(i):
    """🧱️ EN 1996-1-1 Annex G.1 as a symbolic expression of (h_ef / t, e_mk / t, f_k / E), evaluated once."""
    hef_t, e_t, ratio = flo(i["h_ef"]) / flo(i["t"]), flo(i["e_mk"]) / flo(i["t"]), flo(i["f_k"]) / flo(i["E"])
    lam = hef_t * sp.sqrt(ratio)
    u = (lam - sp.Rational(63, 1000)) / (sp.Rational(73, 100) - sp.Rational(117, 100) * e_t)
    a1 = 1 - 2 * e_t
    return {"lambda": float(lam), "A_1": float(a1), "u": float(u), "Phi_m": float(a1 * sp.exp(-(u**2) / 2))}


def masonry_shear(i):
    """🧱️ EN 1996-1-1 6.2 with the compressed length in closed form, l_c = l for e <= l / 6 and l_c = 3 (l / 2 - e) beyond (no-tension triangle), then the shear strength and resistance."""
    length, ecc, t, n = (flo(i[k]) for k in ("l", "e", "t", "N_Ed"))
    l_c = sp.Piecewise((length, ecc <= length / 6), (3 * (length / 2 - ecc), True))
    sigma = n / (t * l_c)
    f_vk = sp.Min(flo(i["f_vk0"]) + sp.Rational(4, 10) * sigma, sp.Rational(65, 1000) * flo(i["f_b"]))
    return {"l_c": float(l_c), "sigma_d": float(sigma), "f_vk": float(f_vk), "V_Rd": float(f_vk * t * l_c / flo(i["gamma_M"]))}


def bearing_factors_annex_d(i):
    """🏗️ EN 1997-1 Annex D.4 drained bearing resistance as one symbolic expression in (phi, B / L, H / V, c, q, gamma)."""
    phi = flo(i["phi"]) * sp.pi / 180
    t = sp.tan(phi)
    nq = sp.exp(sp.pi * t) * sp.tan(sp.pi / 4 + phi / 2) ** 2
    nc, ng = (nq - 1) / t, 2 * (nq - 1) * t
    ratio, area = flo(i["B"]) / flo(i["L"]), flo(i["B"]) * flo(i["L"])
    bq = (1 - flo(i.get("alpha", 0.0)) * sp.pi / 180 * t) ** 2
    bc = bq - (1 - bq) / (nc * t)
    sq, sg = 1 + ratio * sp.sin(phi), 1 - sp.Rational(3, 10) * ratio
    sc = (sq * nq - 1) / (nq - 1)
    m = (2 + ratio) / (1 + ratio)
    base = 1 - flo(i["H"]) / (flo(i["V"]) + area * flo(i["c"]) / t)
    iq, ig = base**m, base ** (m + 1)
    ic = iq - (1 - iq) / (nc * t)
    q_ult = flo(i["c"]) * nc * bc * sc * ic + flo(i["q"]) * nq * bq * sq * iq + flo(i["gamma"]) * flo(i["B"]) / 2 * ng * bq * sg * ig
    return {"N_q": float(nq), "N_c": float(nc), "N_gamma": float(ng), "s_q": float(sq), "s_gamma": float(sg), "s_c": float(sc), "i_q": float(iq), "i_gamma": float(ig), "i_c": float(ic), "b_q": float(bq), "q_ult": float(q_ult), "R_k": float(q_ult * area)}


def coulomb_coefficients(phi_deg, delta_deg, side):
    """🧱️ Coulomb horizontal coefficient of a vertical wall with level ground in symbolic form."""
    phi, delta = flo(phi_deg) * sp.pi / 180, flo(delta_deg) * sp.pi / 180
    root = sp.sqrt(sp.sin(phi + delta) * sp.sin(phi) / sp.cos(delta))
    return sp.cos(phi) ** 2 / (sp.cos(delta) * (1 + (root if side == "a" else -root)) ** 2) * sp.cos(delta)


def embedded_wall(i):
    """🧱️ Embedded wall embedment as the positive real root of the polynomial moment balance of the linear earth pressure diagrams (cantilever about the toe, free earth support about the prop), solved symbolically."""
    t, z = sp.symbols("t z", positive=True)
    ka, kp = coulomb_coefficients(i["phi"], i["delta_a"], "a"), coulomb_coefficients(i["phi"], i["delta_p"], "p")
    h, gamma, q = flo(i["h"]), flo(i["gamma"]), flo(i["q"])
    active, passive = ka * (gamma * z + q), kp * gamma * z
    if i["kind"] == "cantilever":
        ma = sp.integrate(active * (h + t - z), (z, 0, h + t))
        mp = sp.integrate(passive * (t - z), (z, 0, t))
    else:
        a = flo(i["prop_depth"])
        ma = sp.integrate(active * (z - a), (z, 0, h + t))
        mp = sp.integrate(passive * (h + z - a), (z, 0, t))
    roots = [r for r in sp.Poly(sp.expand(mp - ma), t).nroots(n=30) if abs(sp.im(r)) < 1e-20 and sp.re(r) > 0]
    return {"K_ah": float(ka), "K_ph": float(kp), "t": float(max(sp.re(r) for r in roots))}


def pile_group_efficiency(i):
    """🧱️ Converse-Labarre efficiency with sympy's atan and exact degree conversion."""
    m, n, d, s = (flo(i[k]) for k in ("m", "n", "d", "s"))
    eff = 1 - sp.atan(d / s) * 180 / sp.pi * ((n - 1) * m + (m - 1) * n) / (90 * m * n)
    return {"E_g": float(eff), "R_group": float(eff * m * n * flo(i["R_c"]))}


def vs30_ground_class(i):
    """🌍️ EN 1998-1 Eq. 3.1 with exact rational layer thicknesses and velocities, and the Table 3.1 class from nested Piecewise limits."""
    hs, vs = [flo(h) for h in i["h"]], [flo(v) for v in i["v"]]
    depth, travel = sp.Integer(0), sp.Integer(0)
    for h, v in zip(hs, vs):
        used = sp.Min(h, 30 - depth)
        travel += sp.Max(used, 0) / v
        depth += h
    vs30 = 30 / travel
    cls = sp.Piecewise((0, vs30 > 800), (1, vs30 > 360), (2, vs30 > 180), (3, True))
    return {"vs30": float(vs30), "class_index": float(cls)}


def nonstructural_force(i):
    """🏢️ EN 1998-1 4.3.5.2 Eq. 4.24 as one symbolic expression."""
    alpha = flo(i["a_g"]) / sp.Float("9.81", 30)
    s_a = sp.Max(alpha * flo(i["S"]) * (3 * (1 + flo(i["Z"]) / flo(i["H"])) / (1 + (1 - flo(i["T_a"]) / flo(i["T_1"])) ** 2) - sp.Rational(1, 2)), alpha * flo(i["S"]))
    return {"S_a": float(s_a), "F_a": float(s_a * flo(i["W_a"]) * flo(i["gamma_a"]) / flo(i["q_a"]))}


def sway_imperfection(i):
    """🏢️ EN 1993-1-1 5.3.2(3) symbolic."""
    h, m, v = flo(i["h"]), flo(i["m"]), flo(i["V"])
    alpha_h = sp.Min(sp.Max(2 / sp.sqrt(h), sp.Rational(2, 3)), 1)
    phi = alpha_h * sp.sqrt((1 + 1 / m) / 2) / 200
    return {"alpha_h": float(alpha_h), "phi": float(phi), "H_eq": float(phi * v)}


def i_constants(i):
    """📐️ Thin-walled doubly symmetric I constants from rectangles: A, I_y exact, I_z exact, I_t = sum b t^3 / 3 and I_w = t_f b^3 (h - t_f)^2 / 24."""
    h, b, tf, tw = (flo(i[k]) for k in ("h", "b", "tf", "tw"))
    hw = h - 2 * tf
    return {"A": 2 * b * tf + hw * tw, "Iy": b * h**3 / 12 - (b - tw) * hw**3 / 12, "Iz": 2 * tf * b**3 / 12 + hw * tw**3 / 12, "It": (2 * b * tf**3 + hw * tw**3) / 3, "Iw": tf * b**3 * (h - tf) ** 2 / 24}


def buckling_curve(lam, alpha, lam0):
    """📉️ Reduction factor chi = 1 / (phi + sqrt(phi^2 - lambda^2)) with phi = (1 + alpha (lambda - lambda_0) + lambda^2) / 2, capped at 1."""
    phi = (1 + alpha * (lam - lam0) + lam**2) / 2
    return sp.Min(1, 1 / (phi + sp.sqrt(phi**2 - lam**2)))


def flexural_buckling(i):
    """🪶️ EN 1999-1-1 6.3.1 and EN 1993-1-1 6.3.1: N_cr = pi^2 E I_z / L^2 in closed form, lambda = sqrt(A f_o / N_cr), chi and N_b,Rd = kappa chi A f_o / gamma_M1."""
    c = i_constants(i)
    n_cr = sp.pi**2 * flo(i["E"]) * c["Iz"] / flo(i["L"]) ** 2
    lam = sp.sqrt(c["A"] * flo(i["f_o"]) / n_cr)
    chi = buckling_curve(lam, flo(i["alpha"]), flo(i["lambda_0"]))
    return {"A": float(c["A"]), "Iz": float(c["Iz"]), "N_cr": float(n_cr), "lambda": float(lam), "chi": float(chi), "N_b_Rd": float(flo(i.get("kappa", 1.0)) * chi * c["A"] * flo(i["f_o"]) / flo(i["gamma_M1"]))}


def lateral_torsional_buckling(i):
    """🦋️ Elastic critical moment of a simply supported I-beam under uniform moment with continuous torsional restraint k_s: the 2 x 2 determinant of the sinusoidal mode equations EI_z a^4 A - M a^2 B = 0 and -M a^2 A + (EI_w a^4 + GI_t a^2 + k_s) B = 0 solved for M (a = pi / L), then lambda_LT = sqrt(W f / M_cr), chi_LT and M_b,Rd."""
    c = i_constants(i)
    m, a = sp.symbols("M a", positive=True)
    ei_z, ei_w, gi_t, ks = flo(i["E"]) * c["Iz"], flo(i["E"]) * c["Iw"], flo(i["G"]) * c["It"], flo(i.get("k_s", 0.0))
    det = sp.Matrix([[ei_z * a**4, -m * a**2], [-m * a**2, ei_w * a**4 + gi_t * a**2 + ks]]).det()
    m_cr = sp.solve(det, m)[-1].subs(a, sp.pi / flo(i["L"]))
    w_el = c["Iy"] / (flo(i["h"]) / 2)
    lam = sp.sqrt(w_el * flo(i["f_o"]) / m_cr)
    chi = buckling_curve(lam, flo(i["alpha_LT"]), flo(i["lambda_0"]))
    return {"Iz": float(c["Iz"]), "It": float(c["It"]), "Iw": float(c["Iw"]), "M_cr": float(m_cr), "lambda_LT": float(lam), "chi_LT": float(chi), "M_b_Rd": float(chi * w_el * flo(i["f_o"]) / flo(i["gamma_M1"]))}


def stud_resistance(i):
    """🔩️ EN 1994-1-1 6.6.3.1 stud resistance: P_Rd = min(0.8 f_u pi d^2 / 4, 0.29 alpha d^2 sqrt(f_ck E_cm)) / gamma_V with alpha = 0.2 (h_sc / d + 1) for 3 <= h_sc / d <= 4 and 1 above, and the sheeting reduction k_t = 0.7 / sqrt(n_r) b_0 / h_p (h_sc / h_p - 1) <= k_t,max of 6.6.4.2."""
    d, h = flo(i["d"]), flo(i["h_sc"])
    alpha = sp.Piecewise((sp.Rational(1, 5) * (h / d + 1), h / d <= 4), (1, True))
    steel = sp.Rational(8, 10) * flo(i["f_u"]) * sp.pi * d**2 / 4 / flo(i["gamma_V"])
    concrete = sp.Rational(29, 100) * alpha * d**2 * sp.sqrt(flo(i["f_ck"]) * flo(i["E_cm"])) / flo(i["gamma_V"])
    p_rd = sp.Min(steel, concrete)
    k_t = sp.Min(sp.Rational(7, 10) / sp.sqrt(flo(i["n_r"])) * flo(i["b_0"]) / flo(i["h_p"]) * (h / flo(i["h_p"]) - 1), flo(i["k_t_max"]))
    return {"alpha": float(alpha), "P_Rd_steel": float(steel), "P_Rd_concrete": float(concrete), "P_Rd": float(p_rd), "k_t": float(k_t), "P_Rd_sheeting": float(k_t * p_rd)}


def weld_group_box(i):
    """🧵️ EN 1993-1-8 4.5.3.2 box weld group per unit throat: L_w and I_p by symbolic line integrals of r^2 along the four sides, force F at eccentricity e, resultant per length at the corner by the vector sum of the direct and torsional parts."""
    b, d, f, e = (flo(i[k]) for k in ("b", "d", "F", "e"))
    x, y = sp.symbols("x y")
    ip = 2 * sp.integrate(x**2 + (d / 2) ** 2, (x, -b / 2, b / 2)) + 2 * sp.integrate((b / 2) ** 2 + y**2, (y, -d / 2, d / 2))
    lw = 2 * (b + d)
    torque = f * e
    direct = f / lw
    tx, ty = -torque * (d / 2) / ip, torque * (b / 2) / ip
    return {"L_w": float(lw), "I_p": float(ip), "f_max": float(sp.sqrt(tx**2 + (direct + ty) ** 2))}


def corbel_truss(i):
    """🏛️ EN 1992-1-1 6.5 corbel strut-and-tie by joint equilibrium: with a vertical load F at distance a from the column face and lever arm z, tie F_t = F a / z + H_Ed and strut C = F / sin(theta) with tan(theta) = z / a."""
    f, a, z, h = (flo(i[k]) for k in ("F", "a", "z", "H"))
    theta = sp.atan(z / a)
    return {"F_t": float(f * a / z + h), "C": float(f / sp.sin(theta)), "theta": float(theta)}


CHECKS = {
    "en1990.C.beta": reliability_index,
    "en1993.6.3.1.chi": column_chi,
    "en1993.euler-ncr": euler_critical_load,
    "en1998.3.2.2.5.design-spectrum": design_spectrum,
    "en1990.C.form-linear": form_linear,
    "en1998.4.3.shear-building-modal": shear_building_modal,
    "en1998.4.3.3.3.modal-base-shear": modal_base_shear,
    "en1991.4.janssen-pressure": janssen_ode,
}


def membrane_reinforcement(i):
    """🧱️ EN 1992-1-1 Annex F.1 cases in closed form: f_tdx = n_x + |n_xy|, f_tdy = n_y + |n_xy| when both are positive; otherwise the compressed direction carries a strut and the other tie is n + n_xy^2 / |n|."""
    nx, ny, t = flo(i["n_x"]), flo(i["n_y"]), sp.Abs(flo(i["n_xy"]))
    fx, fy = nx + t, ny + t
    if fx < 0:
        fx, fy = sp.Integer(0), ny + t**2 / sp.Abs(nx)
    elif fy < 0:
        fx, fy = nx + t**2 / sp.Abs(ny), sp.Integer(0)
    return {"f_x": float(fx), "f_y": float(fy)}


CHECKS |= {
    "en1992.F.membrane-reinforcement": membrane_reinforcement,
    "en1991.4.wind-peak-pressure": wind_peak_pressure,
    "en1991.7.wind-cpe": wind_cpe,
    "en1991.snow.shape-mono-pitch": snow_shape_mono_pitch,
    "en1991.snow.drift": snow_drift,
    "en1991.snow.drift-gamma-limited": snow_drift_gamma_limited,
    "en1991.fire.parametric-curve": fire_parametric,
    "en1991.fire.iso834": fire_iso834,
    "en1996.G.phi-m": masonry_phi_m,
    "en1996.6.2.shear-wall": masonry_shear,
    "en1997.D.4.bearing-resistance": bearing_factors_annex_d,
    "en1997.9.embedded-wall": embedded_wall,
    "en1997.7.pile-group": pile_group_efficiency,
    "en1998.3.1.vs30": vs30_ground_class,
    "en1998.4.3.5.2.nonstructural": nonstructural_force,
    "en1993.5.3.2.sway-imperfection": sway_imperfection,
    "en1999.6.3.1.flexural-buckling": flexural_buckling,
    "en1999.6.3.2.ltb": lateral_torsional_buckling,
    "en1994.6.4.ltb-restrained": lateral_torsional_buckling,
    "en1994.6.6.3.stud-resistance": stud_resistance,
    "en1993.1-8.4.5.weld-group-box": weld_group_box,
    "en1992.J.3.corbel-truss": corbel_truss,
}


def bridge_wind_force(i):
    """🌉 EN 1991-1-4 8.3 bridge wind force per metre: c_f,x0 as the exact `Piecewise` line between the registry plateaus (unverified), d_tot with `Max`, forces symbolic in q_p, b and d_tot."""
    q, b, d, d1, r0, r1 = (sp.Rational(str(i[k])) for k in ("q_p", "b", "d", "d1", "r_narrow", "r_wide"))
    c0, c1 = sp.Rational(str(i["cfx_narrow"])), sp.Rational(str(i["cfx_wide"]))
    parapet = {0: sp.Integer(0), 1: 2 * sp.Rational(str(i["open_add"])), 2: 2 * d1}[int(i["parapet_kind"])]
    traffic = {0: sp.Integer(0), 1: sp.Rational(str(i["road_add"])), 2: sp.Rational(str(i["rail_add"]))}[int(i["traffic_kind"])]
    d_tot = d + sp.Max(parapet, traffic)
    r = sp.symbols("r")
    c_fx0 = sp.Piecewise((c0, r <= r0), (c1, r >= r1), (c0 + (c1 - c0) * (r - r0) / (r1 - r0), True)).subs(r, b / d_tot)
    f_x = q * c_fx0 * d_tot
    return {"d_tot": float(d_tot), "c_fx0": float(c_fx0), "F_x": float(f_x), "F_z": float(q * sp.Rational(str(i["cfz"])) * b), "F_y": float(sp.Rational(str(i["longitudinal"])) * f_x)}


def vortex_shedding_onset(i):
    """🌪️ EN 1991-1-4 Annex E: the critical velocity solved symbolically from St v / b = n_1, the Scruton number, the onset velocity and the Reynolds number."""
    v = sp.symbols("v", positive=True)
    b, st, n1 = (sp.Rational(str(i[k])) for k in ("b", "St", "n1"))
    v_crit = sp.solve(sp.Eq(st * v / b, n1), v)[0]
    return {"v_crit": float(v_crit), "Sc": float(2 * sp.Rational(str(i["delta_s"])) * sp.Rational(str(i["m_e"])) / (sp.Rational(str(i["rho"])) * b**2)), "v_onset": float(sp.Rational(str(i["onset"])) * sp.Rational(str(i["v_m"]))), "Re": float(b * v_crit / sp.Rational(str(i["nu"])))}


def impact_spring_peak(i):
    """💥 EN 1991-1-7 Annex C: energy balance 1/2 m v^2 = 1/2 F^2 / k solved for the positive force F, and the kinetic energy."""
    f = sp.symbols("F", positive=True)
    m, v, k = (sp.Rational(str(i[x])) for x in ("m", "v", "k"))
    return {"F": float(sp.solve(sp.Eq(m * v**2 / 2, f**2 / (2 * k)), f)[0]), "E": float(m * v**2 / 2)}


def gas_explosion_pressure(i):
    """🏠 EN 1991-1-7 Annex D: p_d1 and p_d2 exactly in rationals with `Max` as the governing value."""
    p_v, ratio = sp.Rational(str(i["p_v"])), sp.Rational(str(i["a_v"])) / sp.Rational(str(i["V"]))
    d1 = sp.Rational(str(i["base"])) + p_v
    d2 = sp.Rational(str(i["base"])) + p_v / 2 + sp.Rational(str(i["coefficient"])) / ratio**2
    return {"p_d1": float(d1), "p_d2": float(d2), "p_d": float(sp.Max(d1, d2)), "ratio": float(ratio)}


def roof_step_drift(i):
    """❄️ EN 1991-1-3 5.3.6: mu_w as `Min`/`Max` of the exact rational limits (floor last), mu_s with a `Piecewise` mu_1, l_s = 2 h within [5, 15]."""
    h, b1, b2, gamma, s_k, cap = (sp.Rational(str(i[k])) for k in ("h", "b1", "b2", "gamma", "s_k", "cap"))
    mu_w = sp.Max(sp.Rational(4, 5), sp.Min(cap, sp.Min((b1 + b2) / (2 * h), gamma * h / s_k)))
    alpha = sp.Rational(str(i["pitch_upper"]))
    mu_1 = sp.Piecewise((sp.Rational(4, 5), alpha <= 30), (sp.Rational(4, 5) * (60 - alpha) / 30, alpha < 60), (0, True))
    mu_s = sp.Rational(str(i["share"])) * mu_1 if alpha > 15 else sp.Integer(0)
    return {"mu_w": float(mu_w), "mu_s": float(mu_s), "l_s": float(sp.Max(5, sp.Min(15, 2 * h)))}


def tandem_travel_envelope(i):
    """🌉 EN 1991-2 4.3.2 tandem travel: the mid-span moment is piecewise linear in the front axle position, so its maximum sits at a breakpoint; evaluated exactly as `Max` of the influence sums at the breakpoints (closed form Q (L - s) / 2 for L >= 2 s)."""
    span, spacing, axle = (sp.Rational(str(i[k])) for k in ("span", "spacing", "axle"))
    x = sp.Symbol("x", real=True)
    eta_m = lambda t: sp.Piecewise((0, t < 0), (t / 2, t <= span / 2), ((span - t) / 2, t <= span), (0, True))
    eta_r = lambda t: sp.Piecewise((0, t < 0), (1 - t / span, t <= span), (0, True))
    cuts = (0, spacing, span / 2, span / 2 + spacing, span, span + spacing)
    moment = axle * (eta_m(x) + eta_m(x - spacing))
    reaction = axle * (eta_r(x) + eta_r(x - spacing))
    return {"m_env": float(sp.Max(*[moment.subs(x, c) for c in cuts])), "r_env": float(sp.Max(*[reaction.subs(x, c) for c in cuts]))}


def snow_profile_mu(i):
    """❄️ EN 1991-1-3 5.3.6 drift triangle along a roof section as a `Piecewise` linear ramp between the base value and the peak."""
    start, length, base, peak, x = (sp.Rational(str(i[k])) for k in ("start", "length", "base", "peak", "x"))
    top = sp.Max(peak, base)
    t = (x - start) / length
    mu = sp.Piecewise((base, x < start), (base + (top - base) * (t if i["rising"] > 0.5 else 1 - t), x <= start + length), (base, True))
    return {"mu": float(mu)}


def wind_net_pressure(i):
    """🌬️ EN 1991-1-4 7.2.9 net pressure w = q_p (c_pe - c_pi) c_s c_d by symbolic substitution."""
    qp, cpe, cpi, factor = sp.symbols("qp cpe cpi factor")
    return {"w": float((qp * (cpe - cpi) * factor).subs({qp: flo(i["qp"]), cpe: flo(i["cpe"]), cpi: flo(i["cpi"]), factor: flo(i["factor"])}))}

CHECKS |= {
    "en1991.8.bridge-wind-force": bridge_wind_force,
    "en1991.2.tandem-travel-envelope": tandem_travel_envelope,
    "en1991.snow.profile-mu": snow_profile_mu,
    "en1991.wind.net-pressure": wind_net_pressure,
    "en1991.6.vortex-shedding": vortex_shedding_onset,
    "en1991.7.impact-force": impact_spring_peak,
    "en1991.7.gas-explosion": gas_explosion_pressure,
    "en1991.snow.roof-step-drift": roof_step_drift,
}
