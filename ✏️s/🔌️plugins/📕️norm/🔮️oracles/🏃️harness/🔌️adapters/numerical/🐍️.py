"""🧮️ Numerical-physics adapter on numpy and scipy: reformulates closed forms as integration, root finding and eigenproblems so the reference never reuses the Rust expression."""

import numpy as np
from scipy import integrate, interpolate, linalg, optimize, stats

LIBRARY = "scipy"


def parabola_rectangle(strain, fcd, eps_c2=2.0e-3, eps_cu2=3.5e-3, n=2.0):
    """📈️ EN 1992-1-1 3.1.7(1) design stress of the parabola-rectangle law for compressive strain `strain`."""
    if strain <= 0.0:
        return 0.0
    if strain < eps_c2:
        return fcd * (1.0 - (1.0 - strain / eps_c2) ** n)
    return fcd


def flexure_rectangular(inputs):
    """🏗️ EN 1992-1-1 6.1 singly reinforced rectangle: neutral axis by brentq on integrated force equilibrium, moment about the tension steel."""
    fcd = inputs["alpha_cc"] * inputs["fck"] / inputs["gamma_c"]
    fyd = inputs["fyk"] / inputs["gamma_s"]
    es = inputs.get("Es", 200.0e9)
    eps_cu2 = inputs.get("eps_cu2", 3.5e-3)
    eps_c2 = inputs.get("eps_c2", 2.0e-3)
    n = inputs.get("n", 2.0)
    b, d, a_s = inputs["b"], inputs["d"], inputs["As"]

    def balance(x):
        eps_s = eps_cu2 * (d - x) / x
        c = integrate.quad(lambda z: b * parabola_rectangle(eps_cu2 * (x - z) / x, fcd, eps_c2, eps_cu2, n), 0.0, x, limit=200)[0]
        return c - a_s * min(es * eps_s, fyd)

    x = optimize.brentq(balance, 1e-9, d, xtol=1e-14, rtol=1e-13)
    c_moment = integrate.quad(lambda z: b * parabola_rectangle(eps_cu2 * (x - z) / x, fcd, eps_c2, eps_cu2, n) * (d - z), 0.0, x, limit=200)[0]
    return {"x": x, "M_Rd": c_moment}


def column_chi(inputs):
    """🏛️ EN 1993-1-1 6.3.1.2 reduction factor as the root of the Ayrton-Perry equation (1-chi)(1-lambda^2 chi) = eta chi."""
    lam, alpha = inputs["lambda_bar"], inputs["alpha"]
    if lam <= 0.2:
        return {"chi": 1.0}
    eta = alpha * (lam - 0.2)
    root = optimize.brentq(lambda c: (1.0 - c) * (1.0 - lam * lam * c) - eta * c, 1e-12, 1.0, xtol=1e-15, rtol=1e-14)
    return {"chi": min(root, 1.0)}


def euler_critical_load(inputs):
    """🧱️ Pin-ended column N_cr from the generalised eigenproblem of Euler-Bernoulli elements with consistent geometric stiffness."""
    ei, length, elements = inputs["EI"], inputs["L"], int(inputs.get("elements", 32))
    h = length / elements
    ke = ei / h**3 * np.array([[12, 6 * h, -12, 6 * h], [6 * h, 4 * h * h, -6 * h, 2 * h * h], [-12, -6 * h, 12, -6 * h], [6 * h, 2 * h * h, -6 * h, 4 * h * h]])
    kg = 1.0 / (30.0 * h) * np.array([[36, 3 * h, -36, 3 * h], [3 * h, 4 * h * h, -3 * h, -h * h], [-36, -3 * h, 36, -3 * h], [3 * h, -h * h, -3 * h, 4 * h * h]])
    size = 2 * (elements + 1)
    k, g = np.zeros((size, size)), np.zeros((size, size))
    for e in range(elements):
        s = slice(2 * e, 2 * e + 4)
        k[s, s] += ke
        g[s, s] += kg
    free = [i for i in range(size) if i not in (0, size - 2)]
    values = linalg.eigh(k[np.ix_(free, free)], g[np.ix_(free, free)], eigvals_only=True)
    return {"N_cr": float(values[0])}


def reliability_index(inputs):
    """🎯️ EN 1990 Annex C reliability index beta = -Phi^-1(P_f)."""
    return {"beta": float(-stats.norm.ppf(inputs["pf"]))}


def circular_hollow_section(inputs):
    """⭕️ Thin-walled and thick-walled CHS constants from the exact annulus integrals: A = pi/4 (D^2 - d^2), I = pi/64 (D^4 - d^4), J = 2 I, W_pl = (D^3 - d^3) / 6."""
    outer, inner = inputs["d"], inputs["d"] - 2.0 * inputs["t"]
    area, second = np.pi / 4.0 * (outer**2 - inner**2), np.pi / 64.0 * (outer**4 - inner**4)
    return {"A": area, "Iy": second, "Iz": second, "It": 2.0 * second, "Wel_y": second / (outer / 2.0), "Wel_z": second / (outer / 2.0), "Wpl_y": (outer**3 - inner**3) / 6.0, "Wpl_z": (outer**3 - inner**3) / 6.0}


def flexure_t_section(i):
    """🏛️ EN 1992-1-1 6.1 T-section by strain compatibility: fibre width b_f over h_f, b_w below; neutral axis by brentq on the integrated force balance, moment about the bar group."""
    fcd, fyd, es = i["alpha_cc"] * i["fck"] / i["gamma_c"], i["fyk"] / i["gamma_s"], i.get("Es", 200.0e9)
    eps_cu2, eps_c2, n = i.get("eps_cu2", 3.5e-3), i.get("eps_c2", 2.0e-3), i.get("n", 2.0)
    bf, hf, bw, d, a_s = i["b_f"], i["h_f"], i["b_w"], i["d"], i["As"]

    def width(z):
        return bf if z < hf else bw

    def force(x, lever):
        return integrate.quad(lambda z: width(z) * parabola_rectangle(eps_cu2 * (x - z) / x, fcd, eps_c2, eps_cu2, n) * lever(z), 0.0, x, points=[hf] if hf < x else None, limit=400)[0]

    def balance(x):
        return force(x, lambda z: 1.0) - a_s * min(es * eps_cu2 * (d - x) / x, fyd)

    x = optimize.brentq(balance, 1e-9, d, xtol=1e-14, rtol=1e-13)
    return {"x": x, "M_Rd": force(x, lambda z: d - z)}


def rhs_sharp(i):
    """🔲️ Sharp-cornered rectangular hollow section as the difference of two rectangles: A, I, W_el and W_pl exactly (no corner radii, no mesh)."""
    h, b, t = i["h"], i["b"], i["t"]
    hi, bi = h - 2.0 * t, b - 2.0 * t
    iy, iz = (b * h**3 - bi * hi**3) / 12.0, (h * b**3 - hi * bi**3) / 12.0
    return {"A": b * h - bi * hi, "Iy": iy, "Iz": iz, "Wel_y": iy / (h / 2.0), "Wel_z": iz / (b / 2.0), "Wpl_y": (b * h**2 - bi * hi**2) / 4.0, "Wpl_z": (h * b**2 - hi * bi**2) / 4.0}


def composite_plastic(i):
    """🧩️ EN 1994-1-1 6.2.1.2 plastic resistance moment under sagging bending with full shear connection: the plastic neutral axis is the root of the net axial force (0.85 f_cd on the slab compression zone, f_yd on the steel), moments by exact piece-wise integration of three steel rectangles (flanges and web, no root fillets)."""
    h, b, tf, tw, hc, beff = i["h"], i["b"], i["tf"], i["tw"], i["h_c"], i["b_eff"]
    fyd, fcd = i["f_y"] / i["gamma_a"], 0.85 * i["f_ck"] / i["gamma_c"]
    rects = [(0.0, tf, b), (tf, h - tf, tw), (h - tf, h, b)]
    compression = lambda lo, hi, zs: max(0.0, min(hi, zs) - lo)
    tension = lambda lo, hi, zs: max(0.0, hi - max(lo, zs))
    balance = lambda z: fcd * beff * min(max(z, 0.0), hc) + fyd * sum(w * (compression(lo, hi, z - hc) - tension(lo, hi, z - hc)) for lo, hi, w in rects)
    z = optimize.brentq(balance, 0.0, hc + h, xtol=1e-14, rtol=1e-13)
    zs, c = z - hc, min(z, hc)
    m = fcd * beff * c * (z - c / 2.0)
    for lo, hi, w in rects:
        top = min(hi, zs)
        if zs > lo:
            m += fyd * w * (top - lo) * (zs - (lo + top) / 2.0)
        bottom = max(lo, zs)
        if hi > zs:
            m += fyd * w * (hi - bottom) * ((bottom + hi) / 2.0 - zs)
    return {"z_pna": z, "M_pl_Rd": m}


def combinations(i):
    """⚖️ EN 1990 6.4.3 by exhaustive enumeration: each variable action is tried as the leading one and the maximum is taken, 6.10 / 6.10a / 6.10b with xi, characteristic 6.14b, frequent 6.15b and quasi-permanent 6.16b sums."""
    g, q = sum(i["G_k"]), i["Q_k"]
    p0, p1, p2 = i["psi_0"], i["psi_1"], i["psi_2"]
    gg, gq, xi = i["gamma_G"], i["gamma_Q"], i["xi"]
    idx = range(len(q))
    others = lambda k, p: sum(p[j] * q[j] for j in idx if j != k)
    e610a = gg * g + gq * sum(p0[j] * q[j] for j in idx)
    e610b = max(xi * gg * g + gq * (q[k] + others(k, p0)) for k in idx)
    return {"E_6_10": max(gg * g + gq * (q[k] + others(k, p0)) for k in idx), "E_6_10a": e610a, "E_6_10b": e610b, "E_d": max(e610a, e610b), "E_char": g + max(q[k] + others(k, p0) for k in idx), "E_freq": g + max(p1[k] * q[k] + others(k, p2) for k in idx), "E_qp": g + sum(p2[j] * q[j] for j in idx)}


def form_linear(i):
    """🎯️ EN 1990 Annex C first-order reliability for g = R - E with independent normal R and E: Hasofer-Lind design point by constrained minimisation of |u| in standard normal space (R = mu_R + sigma_R u_1, E = mu_E + sigma_E u_2), sensitivity factors alpha_R = -u_1 / beta and alpha_E = -u_2 / beta, design values R_d = mu_R - alpha_R beta sigma_R and E_d = mu_E - alpha_E beta sigma_E."""
    mr, sr, me, se = i["mu_R"], i["sigma_R"], i["mu_E"], i["sigma_E"]
    g = lambda u: (mr + sr * u[0]) - (me + se * u[1])
    u = optimize.minimize(lambda u: u @ u, np.array([-0.1, 0.1]), constraints=[{"type": "eq", "fun": g}], method="SLSQP", options={"ftol": 1e-16, "maxiter": 500}).x
    beta = float(np.sqrt(u @ u))
    return {"beta": beta, "alpha_R": float(-u[0] / beta), "alpha_E": float(-u[1] / beta), "R_d": mr + sr * float(u[0]), "E_d": me + se * float(u[1]), "P_f": float(stats.norm.cdf(-beta))}


def shear_building_modal(i):
    """🏢️ EN 1998-1 4.3.3 / 4.3.3.1 modal analysis of a shear building: generalized eigenproblem K phi = omega^2 M phi (tri-diagonal storey stiffness), periods T_j, modal participation Gamma_1 = phi^T M r / phi^T M phi and effective modal mass fraction of mode 1."""
    m, k = np.array(i["m"], dtype=float), np.array(i["k"], dtype=float)
    n = len(m)
    stiff = np.zeros((n, n))
    for s in range(n):
        stiff[s, s] += k[s]
        if s + 1 < n:
            stiff[s, s] += k[s + 1]
            stiff[s, s + 1] = stiff[s + 1, s] = -k[s + 1]
    mass = np.diag(m)
    values, vectors = linalg.eigh(stiff, mass)
    omega = np.sqrt(values)
    ones = np.ones(n)
    phi1 = vectors[:, 0] * np.sign(vectors[-1, 0])
    out = {f"T_{j + 1}": float(2.0 * np.pi / omega[j]) for j in range(n)}
    out["Gamma_1"] = float(phi1 @ mass @ ones / (phi1 @ mass @ phi1)) * float(phi1[-1])
    out["m_eff_1_fraction"] = float((phi1 @ mass @ ones) ** 2 / (phi1 @ mass @ phi1) / m.sum())
    return out


def modal_base_shear(i):
    """🌋️ EN 1998-1 4.3.3.3 / 3.2.2.5: generalized eigenproblem of the shear building, effective modal mass of each mode times the design spectrum ordinate S_d(T_j) as modal base shear, combined by SRSS and by CQC with the double sum of the modal correlation rho_jk (xi equal in all modes)."""
    m, k = np.array(i["m"], dtype=float), np.array(i["k"], dtype=float)
    n = len(m)
    stiff = np.zeros((n, n))
    for s in range(n):
        stiff[s, s] += k[s] + (k[s + 1] if s + 1 < n else 0.0)
        if s + 1 < n:
            stiff[s, s + 1] = stiff[s + 1, s] = -k[s + 1]
    mass = np.diag(m)
    values, vectors = linalg.eigh(stiff, mass)
    omega = np.sqrt(values)
    periods = 2.0 * np.pi / omega
    a_g, s_soil, t_b, t_c, t_d, q, beta = (i[x] for x in ("a_g", "S", "T_B", "T_C", "T_D", "q", "beta"))
    plateau = a_g * s_soil * 2.5 / q
    sd = np.array([np.piecewise(t, [t <= t_b, (t > t_b) & (t <= t_c), (t > t_c) & (t <= t_d), t > t_d], [lambda t: a_g * s_soil * (2.0 / 3.0 + t / t_b * (2.5 / q - 2.0 / 3.0)), plateau, lambda t: max(plateau * t_c / t, beta * a_g), lambda t: max(plateau * t_c * t_d / t**2, beta * a_g)]) for t in periods])
    ones = np.ones(n)
    eff = np.array([(vectors[:, j] @ mass @ ones) ** 2 / (vectors[:, j] @ mass @ vectors[:, j]) for j in range(n)])
    modal = eff * sd
    r = omega[None, :] / omega[:, None]
    xi = i["xi"]
    rho = 8.0 * xi**2 * (1.0 + r) * r**1.5 / ((1.0 - r**2) ** 2 + 4.0 * xi**2 * r * (1.0 + r) ** 2)
    out = {f"F_b_{j + 1}": float(modal[j]) for j in range(n)}
    out["V_srss"] = float(np.sqrt(modal @ modal))
    out["V_cqc"] = float(np.sqrt(modal @ rho @ modal))
    return out


def design_spectrum(i):
    """🌋️ EN 1998-1 3.2.2.2 / 3.2.2.5 spectra by np.interp over the corner periods of the branch laws: the elastic ordinate S_e (eta = 1) and the design ordinate S_d with the lower bound beta a_g, on the T^-1 and T^-2 branches evaluated through log-log interpolation of the corner ordinates (power laws are straight lines in log-log space)."""
    a_g, s, t_b, t_c, t_d, q, beta, t = (i[x] for x in ("a_g", "S", "T_B", "T_C", "T_D", "q", "beta", "T"))

    def loglog(ordinates, corners):
        return float(np.exp(np.interp(np.log(t), np.log(corners), np.log(ordinates))))

    if t <= t_b:
        s_e = float(np.interp(t, [0.0, t_b], [a_g * s, a_g * s * 2.5]))
        s_d = float(np.interp(t, [0.0, t_b], [a_g * s * 2.0 / 3.0, a_g * s * 2.5 / q]))
    elif t <= t_c:
        s_e, s_d = a_g * s * 2.5, a_g * s * 2.5 / q
    elif t <= t_d:
        s_e = loglog([a_g * s * 2.5, a_g * s * 2.5 * t_c / t_d], [t_c, t_d])
        s_d = max(loglog([a_g * s * 2.5 / q, a_g * s * 2.5 * t_c / t_d / q], [t_c, t_d]), beta * a_g)
    else:
        s_e = a_g * s * 2.5 * t_c * t_d / t**2
        s_d = max(a_g * s * 2.5 * t_c * t_d / q / t**2, beta * a_g)
    return {"S_e": s_e, "S_d": s_d}


def yield_line_ridge(edges, length, height, m1, m2):
    """🧱️ Johansen upper bound of the rigid-facet mechanism with a horizontal ridge by virtual work: planar facets w = y / y_r, (H - y) / (H - y_r), x / a, (L - x) / b, each yield line contributing |dg| l (m1 u_x^2 + m2 u_y^2) with dg the jump of the facet gradients and u the unit line direction, fixed edges adding the facet gradient times the edge length, the external work p times the volume under the facets; minimised over (y_r, a, b) by differential evolution and polished by Nelder-Mead."""
    top, bottom, left, right = edges

    def cut(p, q, ga, gb, hog=1.0):
        d = np.array(q, dtype=float) - np.array(p, dtype=float)
        norm = float(np.hypot(*d))
        if norm == 0.0:
            return 0.0
        u = d / norm
        return hog * float(np.hypot(*(np.array(ga) - np.array(gb)))) * norm * (m1 * u[0] ** 2 + m2 * u[1] ** 2)

    def polygon(points, plane):
        pts = np.array(points, dtype=float)
        x, y = pts[:, 0], pts[:, 1]
        cross = x * np.roll(y, -1) - np.roll(x, -1) * y
        area = 0.5 * cross.sum()
        cx, cy = ((x + np.roll(x, -1)) * cross).sum() / (6.0 * area), ((y + np.roll(y, -1)) * cross).sum() / (6.0 * area)
        return abs(area) * plane(cx, cy)

    def pressure(v):
        y_r, a, b = v
        if a + b > length or min(y_r, a, b) <= 0.0 or y_r >= height:
            return 1.0e30
        gb, gt, gl, gr = (0.0, 1.0 / y_r), (0.0, -1.0 / (height - y_r)), (1.0 / a, 0.0), (-1.0 / b, 0.0)
        r1, r2 = (a, y_r), (length - b, y_r)
        work = cut(r1, r2, gb, gt) + cut((0, 0), r1, gb, gl) + cut((0, height), r1, gt, gl) + cut((length, 0), r2, gb, gr) + cut((length, height), r2, gt, gr)
        work += left * cut((0, 0), (0, height), gl, (0, 0)) + right * cut((length, 0), (length, height), gr, (0, 0)) + bottom * cut((0, 0), (length, 0), gb, (0, 0)) + top * cut((0, height), (length, height), gt, (0, 0))
        volume = polygon([(0, 0), (length, 0), r2, r1], lambda x, y: y / y_r) + polygon([(0, height), (length, height), r2, r1], lambda x, y: (height - y) / (height - y_r)) + polygon([(0, 0), (0, height), r1], lambda x, y: x / a) + polygon([(length, 0), (length, height), r2], lambda x, y: (length - x) / b)
        return work / volume

    best = optimize.differential_evolution(pressure, [(1.0e-3 * height, (1.0 - 1.0e-3) * height), (1.0e-3 * length, length), (1.0e-3 * length, length)], tol=1.0e-12, seed=1, polish=False, maxiter=2000, popsize=30)
    polished = optimize.minimize(pressure, best.x, method="Nelder-Mead", options={"xatol": 1.0e-12, "fatol": 1.0e-14, "maxiter": 20000})
    return min(float(best.fun), float(polished.fun))


def yield_line_pressure(i):
    """🧱️ EN 1996-1-1 Annex E mechanics: collapse pressure w of an L x H panel with edge conditions top, bottom, left, right (1 simple, 2 fixed; 0 free is not modelled), m1 across bed joints and m2 across perpendicular joints, the smaller of the horizontal-ridge and the transposed vertical-ridge mechanism."""
    t, b, l, r = (int(i[k]) - 1 for k in ("top", "bottom", "left", "right"))
    horizontal = yield_line_ridge((t, b, l, r), i["l"], i["h"], i["m1"], i["m2"])
    vertical = yield_line_ridge((r, l, b, t), i["h"], i["l"], i["m2"], i["m1"])
    return {"w_Rd": min(horizontal, vertical)}


def yield_line_alpha1(i):
    """🧱️ Stand-in coefficient alpha_1 = m1 / (w l^2) of a panel of unit length l = 1, m1 = 1, m2 = 1 / mu and height h/l, from the collapse pressure of the virtual-work mechanism."""
    w = yield_line_pressure({"top": i["top"], "bottom": i["bottom"], "left": i["left"], "right": i["right"], "l": 1.0, "h": i["h_over_l"], "m1": 1.0, "m2": 1.0 / i["mu"]})["w_Rd"]
    return {"alpha1": 1.0 / w}


def janssen_ode(i):
    """🫙 EN 1991-4 5.2.1.2 Janssen pressures from the slice equilibrium d sigma_v / dz = gamma - (K mu U / A) sigma_v, sigma_v(0) = 0, integrated numerically; p_h = K sigma_v and wall friction p_w = mu p_h."""
    gamma, a, mu, k, depth = i["gamma"], i["a_m"], i["mu"], i["K"], i["depth"]
    rate = k * mu / a
    sol = integrate.solve_ivp(lambda z, s: gamma - rate * s, (0.0, depth), [0.0], rtol=1e-12, atol=1e-9, dense_output=True)
    sigma_v = float(sol.y[0, -1])
    return {"p_vf": sigma_v, "p_hf": k * sigma_v, "p_wf": mu * k * sigma_v, "z_o": 1.0 / rate, "p_ho": gamma * a / mu}


def boussinesq(i):
    """🌍️ Elastic stress increase below a uniform load from Boussinesq's point-load solution 3 q z^3 / (2 pi R^5) integrated numerically over the loaded area: `strip` (per metre, `x` from the strip edge, width b), `rectangle` (corner point) and `circle` (axis point)."""
    q, z, shape = i["q"], i["z"], i["shape"]
    point = lambda x, y: 3.0 * q * z**3 / (2.0 * np.pi * (x * x + y * y + z * z) ** 2.5)
    if shape == "strip":
        line = lambda x: 2.0 * q * z**3 / (np.pi * (x * x + z * z) ** 2)
        edge = i.get("x", 0.0)
        return {"sigma_z": integrate.quad(line, edge - i["width"], edge)[0]}
    if shape == "rectangle":
        return {"sigma_z": integrate.dblquad(lambda y, x: point(x, y), 0.0, i["length"], 0.0, i["width"], epsabs=1e-12, epsrel=1e-12)[0]}
    r = i["radius"]
    return {"sigma_z": integrate.dblquad(lambda t, rho: point(rho * np.cos(t), rho * np.sin(t)) * rho, 0.0, r, 0.0, 2.0 * np.pi, epsabs=1e-12, epsrel=1e-12)[0]}


def tandem_influence(i):
    """🚛️ EN 1991-2 4.3.2 LM1 tandem on a simply supported span from the closed-form influence functions of the simple beam: M(x; xi) = xi (L - x) / L for x >= xi else x (L - xi) / L per unit load, maximised over section x and tandem position on a 5 mm grid; a reaction maximum from R_A = Q (1 - xi / L)."""
    span, q, s = i["span"], i["Q"], i["s"]
    grid = np.arange(0.0, span + 1.0e-9, 0.005)
    pos = np.arange(-s, span + 1.0e-9, 0.005)
    best_m, best_r = 0.0, 0.0
    for a in pos:
        axles = np.array([x for x in (a, a + s) if 0.0 <= x <= span])
        if axles.size == 0:
            continue
        lower = np.where(grid[:, None] >= axles[None, :], axles[None, :] * (span - grid[:, None]) / span, grid[:, None] * (span - axles[None, :]) / span)
        best_m = max(best_m, float(q * lower.sum(axis=1).max()))
        best_r = max(best_r, float(q * (1.0 - axles / span).sum()), float(q * (axles / span).sum()))
    return {"M_max": best_m, "R_max": best_r}


def tandem_section_envelope_influence(i):
    """🌈️ EN 1991-2 4.3.2 LM1 tandem at one section of a simply supported span from the influence lines of the section (bending moment eta_M(xi) and shear eta_V(xi) as piecewise-linear `interp1d` tables with the step of the shear at the section): the tandem is swept on a 1 mm grid of the front axle position and the extremes of the summed ordinates are reported (the shear is evaluated an epsilon left and right of a load standing on the section)."""
    span, q, s, x = i["span"], i["Q"], i["s"], i["x"]
    eps = 1.0e-9
    eta_m = interpolate.interp1d((-1.0e3, 0.0, x, span, span + 1.0e3), (0.0, 0.0, x * (span - x) / span, 0.0, 0.0))
    eta_left = interpolate.interp1d((-1.0e3, -eps, 0.0, x, x + eps, span, span + eps, span + 1.0e3), (0.0, 0.0, 0.0, -x / span, (span - x - eps) / span, 0.0, 0.0, 0.0))
    eta_right = interpolate.interp1d((-1.0e3, -eps, 0.0, x - eps, x, span, span + eps, span + 1.0e3), (0.0, 0.0, 0.0, -(x - eps) / span, (span - x) / span, 0.0, 0.0, 0.0))
    grid = np.arange(0.0, span + s + 1.0e-9, 0.001)
    fronts = np.concatenate([grid, [x, x + s, span, s, 0.0, span + s]])
    rear = np.concatenate([grid - s, [x - s, x, span - s, 0.0, -s, span]])
    m = q * (eta_m(fronts) + eta_m(rear))
    shears = np.concatenate([q * (eta_left(fronts) + eta_left(rear)), q * (eta_right(fronts) + eta_right(rear))])
    return {"M_env": float(m.max()), "V_max": float(max(shears.max(), 0.0)), "V_min": float(min(shears.min(), 0.0))}


def braking_force_clip(i):
    """🛑 EN 1991-2 4.4.1 (2) braking force Q_lk = 0.6 alpha_Q1 (2 Q_1k) + 0.10 alpha_q1 q_1k w_1 L limited to 180 alpha_Q1 kN <= Q_lk <= 900 kN by `numpy.clip`."""
    q = 0.6 * i["alpha_Q1"] * 2.0 * i["Q1"] + 0.10 * i["alpha_q1"] * i["q1"] * i["w1"] * i["L"]
    return {"Q_lk": float(np.clip(q, 180000.0 * i["alpha_Q1"], 900000.0))}


def centrifugal_piecewise(i):
    """🌀 EN 1991-2 4.4.2 Table 4.3 centrifugal force Q_tk of the total tandem weight Q_v: 0.2 Q_v below r = 200 m, 40 Q_v / r up to 1500 m and zero above, by `numpy.piecewise`."""
    r, qv = i["r"], i["Q_v"]
    return {"Q_tk": float(np.piecewise(r, [r < 200.0, (r >= 200.0) & (r <= 1500.0)], [lambda _: 0.2 * qv, lambda v: 40.0 * qv / v, 0.0]))}


def axle_group_mid_moment_influence(i):
    """🚆️ EN 1991-2 6.3.2 LM71 on a simply supported span from the closed-form mid-span influence function eta(x) = min(x, L - x) / 2: axle loads `Q` at spacing `s` centred at mid-span plus the distributed load `q` integrated numerically with the trapezoidal rule over the span outside the group length (n - 1) s / 2 + `lead`."""
    span, n = i["span"], int(i["n"])
    mid, group = span / 2.0, (n - 1) / 2.0 * i["s"] + i["lead"]
    eta = lambda x: np.minimum(x, span - x) / 2.0
    axles = mid + (np.arange(n) - (n - 1) / 2.0) * i["s"]
    pieces = [(0.0, mid - group), (mid + group, span)]
    udl = sum(float(np.trapezoid(eta(np.linspace(a, b, 200001)), np.linspace(a, b, 200001))) for a, b in pieces if b > a)
    return {"M_mid": float(i["Q"] * eta(axles).sum() + i["q"] * udl)}


def masonry_flexure_numeric(i):
    """🧱️ EN 1996-1-1 6.6.1 reinforced masonry strip by strain compatibility: stress f_d over 0.8 x, eps_mu = 3.5 permille, bilinear steel; neutral axis by brentq on the force balance."""
    fd, fyd, es, b, d, a_s = i["f_d"], i["f_yd"], i.get("Es", 200.0e9), i["b"], i["d"], i["As"]
    eps_mu = 3.5e-3

    def balance(x):
        return fd * b * 0.8 * x - a_s * min(es * eps_mu * (d - x) / x, fyd)

    x = optimize.brentq(balance, 1e-9, d, xtol=1e-14, rtol=1e-13)
    return {"x": x, "M_Rd": a_s * min(es * eps_mu * (d - x) / x, fyd) * (d - 0.4 * x)}


def composite_elastic(i):
    """🧩️ EN 1994-1-1 5.4.2.2 transformed section by the parallel-axis theorem: three steel rectangles (flanges and web, no root fillets) plus the slab divided by n = E_a / E_c; results referred to steel."""
    h, b, tf, tw, hc, beff = i["h"], i["b"], i["tf"], i["tw"], i["h_c"], i["b_eff"]
    n = i["E_a"] / i["E_c"]
    parts = [(b * tf, tf / 2.0, b * tf**3 / 12.0), (tw * (h - 2.0 * tf), h / 2.0, tw * (h - 2.0 * tf) ** 3 / 12.0), (b * tf, h - tf / 2.0, b * tf**3 / 12.0), (beff * hc / n, h + hc / 2.0, beff * hc**3 / 12.0 / n)]
    area = sum(a for a, _, _ in parts)
    centroid = sum(a * z for a, z, _ in parts) / area
    return {"n": n, "A_eff": area, "z_na": centroid, "I_eff": sum(inertia + a * (z - centroid) ** 2 for a, z, inertia in parts)}


def composite_ultimate(i):
    """📉️ EN 1994-1-1 6.2.1.4 non-linear bending resistance of a composite section: the curvature at which the concrete top fibre reaches eps_cu is found by root finding, the neutral axis from N = 0 by a second root finding, strips integrated with numpy; steel bilinear, concrete parabola-rectangle without tension."""
    fyd, fcd, e_a, beff, hc, ys, h, b, tf, tw = (i[k] for k in ("f_yd", "f_cd", "E_a", "b_eff", "h_c", "y_s", "h", "b", "tf", "tw_eff"))
    eps_cu, n_strips = i.get("eps_cu", 3.5e-3), 4000
    def forces(kappa, eps_top):
        yc = (np.arange(n_strips) + 0.5) * hc / n_strips
        ec = eps_top - kappa * yc
        sc = np.array([parabola_rectangle(e, fcd) for e in ec])
        f_c = sc * beff * hc / n_strips
        n_total, m_total = f_c.sum(), -(f_c * yc).sum()
        for top, bottom, width in ((ys, ys + tf, b), (ys + tf, ys + h - tf, tw), (ys + h - tf, ys + h, b)):
            y = top + (np.arange(n_strips) + 0.5) * (bottom - top) / n_strips
            f = np.clip(e_a * (eps_top - kappa * y), -fyd, fyd) * width * (bottom - top) / n_strips
            n_total, m_total = n_total + f.sum(), m_total - (f * y).sum()
        return n_total, m_total
    def top_strain(kappa):
        return optimize.brentq(lambda e: forces(kappa, e)[0], -1.0, 1.0, xtol=1e-14)
    kappa_u = optimize.brentq(lambda k: top_strain(k) - eps_cu, 1e-4, 1.0, xtol=1e-12)
    return {"M_Rd": abs(forces(kappa_u, top_strain(kappa_u))[1]), "kappa_u": kappa_u}


CHECKS = {
    "en1993.section.chs": circular_hollow_section,
    "en1992.6.1.flexure-rect": flexure_rectangular,
    "en1993.6.3.1.chi": column_chi,
    "en1993.euler-ncr": euler_critical_load,
    "en1990.C.beta": reliability_index,
    "en1992.6.1.flexure-t": flexure_t_section,
    "en1993.section.rhs-sharp": rhs_sharp,
    "en1999.section.rhs-sharp": rhs_sharp,
    "en1994.6.2.composite-plastic": composite_plastic,
    "en1994.6.2.1.4.composite-ultimate": composite_ultimate,
    "en1990.6.4.combinations": combinations,
    "en1990.C.form-linear": form_linear,
    "en1998.3.2.2.5.design-spectrum": design_spectrum,
    "en1998.4.3.shear-building-modal": shear_building_modal,
    "en1998.4.3.3.3.modal-base-shear": modal_base_shear,
    "en1991.4.janssen-pressure": janssen_ode,
    "en1997.F.boussinesq": boussinesq,
    "mechanics.lm1-tandem": tandem_influence,
    "mechanics.tandem-section-envelope": tandem_section_envelope_influence,
    "en1991.2.braking": braking_force_clip,
    "en1991.2.centrifugal": centrifugal_piecewise,
    "mechanics.rail-lm71-mid-moment": axle_group_mid_moment_influence,
    "en1996.6.6.1.flexure-reinforced": masonry_flexure_numeric,
    "en1994.5.4.composite-elastic": composite_elastic,
    "en1999.section.chs": circular_hollow_section,
}


def footbridge_loads(i):
    """🚶 EN 1991-2 5.3.2.1 and 5.4 footbridge: q_fk = 2.0 + 120/(L + 30) kN/m2 clipped to 2.5..5.0 and Q_flk = max(10 % of q_fk L b, 10 kN) by `numpy.clip` and `numpy.maximum`."""
    q = float(np.clip(2000.0 + 120000.0 / (i["L"] + 30.0), 2500.0, 5000.0))
    return {"q_fk": q, "Q_flk": float(np.maximum(0.10 * q * i["L"] * i["b"], 10000.0))}


def fatigue_load_models(i):
    """🔁 EN 1991-2 4.6 road fatigue models of lane 1..3: FLM1 axle 0.7 Q_ik and distributed load 0.3 q_ik by `numpy` table lookup, FLM3 axle 120 kN and the vehicle length as the `numpy.sum` of the spacings 1.2, 6.0, 1.2 m."""
    lane = int(i["lane"]) - 1
    return {"axle_flm1": float(0.7 * np.array([300e3, 200e3, 100e3])[lane]), "udl_flm1": float(0.3 * np.array([9000.0, 2500.0, 2500.0])[lane]), "axle_flm3": 120e3, "length_flm3": float(np.sum([1.2, 6.0, 1.2]))}


def support_collision_forces(i):
    """🚛 EN 1991-2 4.7 collision force F_dx on a support at deck level (1000 kN) or on the soffit (500 kN) and F_dy (500 kN, deck level only) by `numpy.where`."""
    soffit = bool(i["soffit"] > 0.5)
    out = {"F_dx": float(np.where(soffit, 500e3, 1000e3))}
    if not soffit:
        out["F_dy"] = 500e3
    return out


def local_fire_hasemi(i):
    """🔥 EN 1991-1-2 Annex C flame length, horizontal flame length, virtual source height and Hasemi heat flux by `numpy.piecewise`."""
    d, q, h, r = i["D"], i["Q"], i["H"], i["r"]
    q_h, q_d = q / (1.11e6 * h ** 2.5), q / (1.11e6 * d ** 2.5)
    l_h = h * (2.9 * q_h ** 0.33 - 1.0)
    z_prime = float(np.piecewise(q_d, [q_d < 1.0, q_d >= 1.0], [lambda t: 2.4 * d * (t ** 0.4 - t ** (2.0 / 3.0)), lambda t: 2.4 * d * (1.0 - t ** 0.4)]))
    y = (r + h + z_prime) / (l_h + h + z_prime)
    flux = float(np.piecewise(y, [y <= 0.3, (y > 0.3) & (y < 1.0), y >= 1.0], [100e3, lambda t: 136.3e3 - 121e3 * t, lambda t: 15e3 * t ** -3.7]))
    return {"L_f": -1.02 * d + 0.0148 * q ** 0.4, "L_h": l_h, "z_prime": z_prime, "flux": flux}


def snow_guard_force(i):
    """🛡️ EN 1991-1-3 6.3 snow guard line force F_s = s b sin(alpha) with `numpy.sin`."""
    return {"F_s": float(i["s"] * i["b"] * np.sin(np.deg2rad(i["alpha_deg"])))}


def rail_derailment_load(i):
    """🚆 EN 1991-2 6.7.1 derailment line loads: situation I 45 kN/m, situation II 1.4 alpha q_vk with q_vk = 80 kN/m, loaded length 20 m and the offset limit 1.5 s with s = 1.435 m, by `numpy.multiply`."""
    return {"q_i": 45000.0, "q_ii": float(np.multiply(1.4, np.multiply(i["alpha"], 80000.0))), "length": 20.0, "offset_limit": float(np.multiply(1.5, 1.435))}


def rail_group_bases(i):
    """🛤️ EN 1991-2 Table 6.11 characteristic bases: the larger of traction min(33 kN/m L, 1000 kN) and braking min(20 kN/m L, 6000 kN), and the nosing force alpha 100 kN, by `numpy.minimum` and `numpy.maximum`."""
    traction = np.minimum(33000.0 * i["span"], 1000000.0)
    braking = np.minimum(20000.0 * i["span"], 6000000.0)
    return {"F_long": float(np.maximum(traction, braking)), "F_nosing": float(np.multiply(i["alpha"], 100000.0))}


def dust_vent_area(i):
    """💥 Bartknecht vent area A = [3.264e-5 p_max K_St p_red^-0.569 + 0.27 (p_stat - 0.1) p_red^-0.5] V^0.753 of EN 14491 (pressures in bar) by `numpy.power`."""
    term_kst = 3.264e-5 * i["p_max_bar"] * i["K_St"] * np.power(i["p_red_bar"], -0.569)
    term_stat = 0.27 * (i["p_stat_bar"] - 0.1) * np.power(i["p_red_bar"], -0.5)
    return {"A_v": float((term_kst + term_stat) * np.power(i["V"], 0.753))}


def hoist_release_factor(i):
    """🪝 EN 1991-3 2.5 dynamic factor of the sudden release phi_3 = 1 - (dm/m)(1 + beta_3) by `numpy.subtract`."""
    return {"phi_3": float(np.subtract(1.0, np.multiply(i["dm_over_m"], 1.0 + i["beta_3"])))}


def rail_dynamic_clip(i):
    """🚆 EN 1991-2 6.4.5.2 dynamic factors Phi_2 = 1.44/(sqrt(L) - 0.2) + 0.82 in 1.00..1.67 and Phi_3 = 2.16/(sqrt(L) - 0.2) + 0.73 in 1.00..2.00 by `numpy.clip`."""
    r = np.sqrt(i["L_phi"]) - 0.2
    return {"Phi_2": float(np.clip(1.44 / r + 0.82, 1.0, 1.67)), "Phi_3": float(np.clip(2.16 / r + 0.73, 1.0, 2.0))}


def bulk_k_mean(i):
    """🌾 EN 1991-4 Annex E mean lateral pressure ratio K_m = 1.1 (1 - sin phi_i) by `numpy.sin` and `numpy.radians`."""
    return {"K_m": float(1.1 * (1.0 - np.sin(np.radians(i["phi_i_deg"]))))}


def machine_unbalance_force(i):
    """⚙️ EN 1991-3 5 equivalent static force F_dyn = m e omega^2 of a rotating unbalance, omega = 2 pi f by `numpy.pi`."""
    return {"F_dyn": float(i["m"] * i["e"] * (2.0 * np.pi * i["f"]) ** 2)}


def dispersal_footprint(i):
    """📐 EN 1991-2 4.3.6 wheel footprint spread at 45 degrees through pavement and half the slab: A = (a + 2 d)(b + 2 d) with d = t_p + t_s/2."""
    d = i["t_p"] + 0.5 * i["t_s"]
    return {"A_foot": float((i["a"] + 2.0 * d) * (i["b"] + 2.0 * d))}


CPE_TABLE = {
    "A": ((0.25, 1.0, 5.0), (-1.2, -1.2, -1.2), (-1.4, -1.4, -1.4)),
    "B": ((0.25, 1.0, 5.0), (-0.8, -0.8, -0.8), (-1.1, -1.1, -1.1)),
    "C": ((0.25, 1.0, 5.0), (-0.5, -0.5, -0.5), (-0.5, -0.5, -0.5)),
    "D": ((0.25, 1.0, 5.0), (0.7, 0.8, 0.8), (1.0, 1.0, 1.0)),
    "E": ((0.25, 1.0, 5.0), (-0.3, -0.5, -0.7), (-0.3, -0.5, -0.7)),
}


def wind_peak_pressure(i):
    """💨️ EN 1991-1-4 4.5 peak velocity pressure in the expanded algebraic form q_p = rho/2 k_r^2 v_b^2 (c_o^2 L^2 + 7 k_I c_o L) with L = ln(z / z_0): the factor (1 + 7 I_v) is absorbed into the logarithm powers, so no intermediate I_v appears."""
    z = max(i["z"], i["z_min"])
    log = np.log(z / i["z0"])
    k_r = 0.19 * (i["z0"] / i.get("z0_II", 0.05)) ** 0.07
    q_p = 0.5 * i["rho"] * k_r**2 * i["v_b"] ** 2 * (i["c_o"] ** 2 * log**2 + 7.0 * i["k_I"] * i["c_o"] * log)
    return {"c_r": k_r * log, "v_m": k_r * log * i["c_o"] * i["v_b"], "I_v": i["k_I"] / (i["c_o"] * log), "q_p": q_p}


def wind_cpe(i):
    """🧭️ EN 1991-1-4 7.2.2 Table 7.1 external pressure coefficient of a vertical wall zone: linear interpolation in h/d through the table break points (`interp1d`), and the log10 interpolation of 7.1.2(2) between the 1 m2 and 10 m2 values."""
    ratios, c10, c1 = CPE_TABLE[i["zone"]]
    v10 = float(interpolate.interp1d(ratios, c10, bounds_error=False, fill_value=(c10[0], c10[-1]))(i["h_d"]))
    v1 = float(interpolate.interp1d(ratios, c1, bounds_error=False, fill_value=(c1[0], c1[-1]))(i["h_d"]))
    weight = float(np.clip(np.log10(i["area"]), 0.0, 1.0))
    return {"c_pe_10": v10, "c_pe_1": v1, "c_pe": v1 + (v10 - v1) * weight}


def snow_shape_mono_pitch(i):
    """❄️ EN 1991-1-3 5.3.2 Table 5.2: mu_1 through the break points of the table (0, 30, 60, 90 degrees) by `interp1d`; mu_2 = 0.8 + 0.8 alpha / 30 capped at 1.6; s = mu_1 C_e C_t s_k."""
    alpha = i["alpha"]
    mu1 = float(interpolate.interp1d((0.0, 30.0, 60.0, 90.0), (0.8, 0.8, 0.0, 0.0))(alpha))
    mu2 = float(interpolate.interp1d((0.0, 30.0, 90.0), (0.8, 1.6, 1.6))(alpha))
    return {"mu_1": mu1, "mu_2": mu2, "s": mu1 * i["C_e"] * i["C_t"] * i["s_k"]}


def snow_drift(i, reading=0):
    """🌨️ EN 1991-1-3 5.3.6 and Annex B drifting behind an obstruction: mu_w = (b_1 + b_2) / (2 h) limited by gamma h / s_k and to [0.8, 4] with the 0.8 floor winning (reading 0, conservative default) or gamma h / s_k capping below 0.8 (reading 1, unverified), drift length l_s = 2 h limited to [5, 15] m."""
    geometric, weight = (i["b1"] + i["b2"]) / (2.0 * i["h"]), i["gamma"] * i["h"] / i["s_k"]
    mu_w = float(np.clip(min(geometric, weight), 0.8, 4.0)) if reading == 0 else min(float(np.clip(geometric, 0.8, 4.0)), weight)
    return {"mu_w": mu_w, "l_s": float(np.clip(2.0 * i["h"], 5.0, 15.0)), "s_peak": mu_w * i["s_k"]}


def snow_drift_gamma_limited(i):
    """🔀 EN 1991-1-3 5.3.6 with gamma h / s_k capping mu_w below 0.8 (NDP reading 1)."""
    return snow_drift(i, reading=1)


def fire_parametric(i):
    """🔥️ EN 1991-1-2 Annex A parametric temperature-time curve from the exponential series: heating up to t_max = max(0.2e-3 q_t,d / O, t_lim) and the linear cooling branch, time in hours internally."""
    gamma_f = ((i["O"] / i["b"]) / (0.04 / 1160.0)) ** 2
    t_lim = i["t_lim"] / 3600.0
    t_max = max(0.2e-3 * i["q_td"] / 1.0e6 / i["O"], t_lim)
    star = lambda hours: hours * gamma_f
    heat = lambda ts: 20.0 + 1325.0 * (1.0 - 0.324 * np.exp(-0.2 * ts) - 0.204 * np.exp(-1.7 * ts) - 0.472 * np.exp(-19.0 * ts))
    ts_max = star(t_max)
    theta_max = heat(ts_max)
    t = i["t"] / 3600.0
    if t <= t_max:
        theta = heat(star(t))
    else:
        x = 1.0 if t_max > t_lim else t_lim * gamma_f / ts_max
        slope = 625.0 if ts_max <= 0.5 else (250.0 * (3.0 - ts_max) if ts_max < 2.0 else 250.0)
        theta = max(theta_max - slope * (star(t) - ts_max * x), 20.0)
    return {"Gamma": gamma_f, "t_max": t_max * 3600.0, "theta_max": float(theta_max), "theta_g": float(theta)}


def fire_iso834(i):
    """🔥️ ISO 834 standard temperature-time curve in the natural-logarithm form 20 + 345 ln(8 t + 1) / ln 10, t in minutes."""
    return {"theta_g": 20.0 + 345.0 * np.log(8.0 * i["t_min"] + 1.0) / np.log(10.0)}


def masonry_phi_m(i):
    """🧱️ EN 1996-1-1 Annex G: reduction factor at mid-height Phi_m = A_1 exp(-u^2 / 2), A_1 = 1 - 2 e_mk / t, u = (lambda - 0.063) / (0.73 - 1.17 e_mk / t), lambda = (h_ef / t) sqrt(f_k / E) with the slenderness evaluated in the order the clause states."""
    ratio = i["e_mk"] / i["t"]
    lam = i["h_ef"] / i["t"] * np.sqrt(i["f_k"] / i["E"])
    u = (lam - 0.063) / (0.73 - 1.17 * ratio)
    return {"lambda": lam, "A_1": 1.0 - 2.0 * ratio, "u": u, "Phi_m": (1.0 - 2.0 * ratio) * np.exp(-0.5 * u * u)}


def masonry_shear(i):
    """🧱️ EN 1996-1-1 3.6.2 and 6.2: compressed length of an eccentrically loaded wall from the no-tension linear stress block (centroid of the triangle at l_c / 3 from the edge, found by brentq on integrated moments), then sigma_d = N / (t l_c), f_vk = min(f_vk0 + 0.4 sigma_d, 0.065 f_b) and V_Rd = f_vk t l_c / gamma_M."""
    length, ecc, n_ed = i["l"], i["e"], i["N_Ed"]
    if ecc <= length / 6.0:
        l_c = length
    else:
        def centroid_gap(lc):
            area = integrate.quad(lambda x: 1.0 - x / lc, 0.0, lc)[0]
            moment = integrate.quad(lambda x: (1.0 - x / lc) * x, 0.0, lc)[0]
            return moment / area - (length / 2.0 - ecc)

        l_c = optimize.brentq(centroid_gap, 1e-6, length, xtol=1e-14)
    sigma_d = n_ed / (i["t"] * l_c)
    f_vk = min(i["f_vk0"] + 0.4 * sigma_d, 0.065 * i["f_b"])
    return {"l_c": l_c, "sigma_d": sigma_d, "f_vk": f_vk, "V_Rd": f_vk * i["t"] * l_c / i["gamma_M"]}


def bearing_factors_annex_d(i):
    """🏗️ EN 1997-1 Annex D.4 drained bearing resistance with N_q from the closed form of the Prandtl solution, shape, base-inclination and load-inclination factors, written as plain numpy expressions."""
    phi, t = np.radians(i["phi"]), np.tan(np.radians(i["phi"]))
    nq = np.exp(np.pi * t) * np.tan(np.pi / 4.0 + phi / 2.0) ** 2
    nc, ng = (nq - 1.0) / t, 2.0 * (nq - 1.0) * t
    ratio = i["B"] / i["L"]
    area = i["B"] * i["L"]
    bq = (1.0 - np.radians(i.get("alpha", 0.0)) * t) ** 2
    bc = bq - (1.0 - bq) / (nc * t)
    sq, sg = 1.0 + ratio * np.sin(phi), 1.0 - 0.3 * ratio
    sc = (sq * nq - 1.0) / (nq - 1.0)
    m = (2.0 + ratio) / (1.0 + ratio)
    base = 1.0 - i["H"] / (i["V"] + area * i["c"] / t)
    iq, ig = base**m, base ** (m + 1.0)
    ic = iq - (1.0 - iq) / (nc * t)
    q_ult = i["c"] * nc * bc * sc * ic + i["q"] * nq * bq * sq * iq + 0.5 * i["gamma"] * i["B"] * ng * bq * sg * ig
    return {"N_q": nq, "N_c": nc, "N_gamma": ng, "s_q": sq, "s_gamma": sg, "s_c": sc, "i_q": iq, "i_gamma": ig, "i_c": ic, "b_q": bq, "q_ult": q_ult, "R_k": q_ult * area}


def newmark_corner(m, n):
    """📐️ Newmark's influence factor of a uniform rectangle corner: sigma_z = q I(m, n) with m = a / z and n = b / z."""
    s = m * m + n * n + 1.0
    first = 2.0 * m * n * np.sqrt(s) / (s + (m * n) ** 2) * (s + 1.0) / s
    return (first + np.arctan2(2.0 * m * n * np.sqrt(s), s - (m * n) ** 2)) / (4.0 * np.pi)


def stress_strain_settlement(i):
    """🏗️ EN 1997-1 Annex F.1 stress-strain method for a uniformly loaded rectangle below its centre: s = integral_0^{z_max} sigma_z(z) / E_m dz with the Boussinesq stress from four Newmark corner factors, integrated by adaptive quadrature."""
    a, b = i["B"] / 2.0, i["L"] / 2.0
    stress = lambda z: 4.0 * i["q"] * newmark_corner(a / z, b / z)
    return {"s": integrate.quad(lambda z: stress(z) / i["E_m"], 1e-9, i["z_max"], limit=200)[0], "sigma_z_mid": stress(i["z_max"] / 2.0)}


def terzaghi_degree(i):
    """⏱️ EN 1997-1 Annex F.2 Terzaghi one-dimensional consolidation: average degree of consolidation U(T_v) = 1 - sum 2 / M^2 exp(-M^2 T_v), M = pi (2k + 1) / 2, and the settlement at time t of an oedometer layer."""
    k = np.arange(0, 2000)
    big_m = np.pi * (2 * k + 1) / 2.0
    t_v = i["c_v"] * i["t"] / i["H_dr"] ** 2
    u = 1.0 - float(np.sum(2.0 / big_m**2 * np.exp(-(big_m**2) * t_v)))
    return {"T_v": t_v, "U": u, "s_t": u * i["s_final"]}


def active_passive_coulomb(phi_deg, delta_deg, side):
    """🧱️ Coulomb horizontal-component coefficients of a vertical wall with level ground: K_a = cos^2 phi / (cos delta (1 + sqrt(sin(phi + delta) sin phi / cos delta))^2), K_p with the sign of delta reversed, times cos delta."""
    phi, delta = np.radians(phi_deg), np.radians(delta_deg)
    if side == "a":
        k = np.cos(phi) ** 2 / (np.cos(delta) * (1.0 + np.sqrt(np.sin(phi + delta) * np.sin(phi) / np.cos(delta))) ** 2)
    else:
        k = np.cos(phi) ** 2 / (np.cos(delta) * (1.0 - np.sqrt(np.sin(phi + delta) * np.sin(phi) / np.cos(delta))) ** 2)
    return k * np.cos(delta)


def embedded_wall_integrals(i, ka, kp):
    """🧱️ Required embedment `t` of an embedded wall by moment equilibrium of the horizontal earth pressures integrated with quad: cantilever about the toe, or free earth support about the prop at depth `prop_depth` below the top; pressures unfactored."""
    h, gamma, q = i["h"], i["gamma"], i["q"]
    active = lambda z: ka * (gamma * z + q)
    passive = lambda z: kp * gamma * z

    def balance(t):
        if i["kind"] == "cantilever":
            ma = integrate.quad(lambda z: active(z) * (h + t - z), 0.0, h + t)[0]
            mp = integrate.quad(lambda z: passive(z) * (t - z), 0.0, t)[0]
        else:
            a = i["prop_depth"]
            ma = integrate.quad(lambda z: active(z) * (z - a), 0.0, h + t)[0]
            mp = integrate.quad(lambda z: passive(z) * (h + z - a), 0.0, t)[0]
        return mp - ma

    return optimize.brentq(balance, 1e-3, 20.0 * h, xtol=1e-12)


def embedded_wall(i):
    """🧱️ Embedded wall embedment with Coulomb earth pressure coefficients evaluated from the closed form."""
    ka, kp = active_passive_coulomb(i["phi"], i["delta_a"], "a"), active_passive_coulomb(i["phi"], i["delta_p"], "p")
    return {"K_ah": ka, "K_ph": kp, "t": embedded_wall_integrals(i, ka, kp)}


def pile_group_efficiency(i):
    """🧱️ EN 1997-1 7.6.2.1 group efficiency by the Converse-Labarre formula E = 1 - atan(d / s) (90 degrees)^-1 ((n - 1) m + (m - 1) n) / (m n) written with arctan in radians, and the group resistance E n m R_c."""
    m, n = i["m"], i["n"]
    theta = np.degrees(np.arctan(i["d"] / i["s"]))
    eff = 1.0 - theta * ((n - 1.0) * m + (m - 1.0) * n) / (90.0 * m * n)
    return {"E_g": eff, "R_group": eff * m * n * i["R_c"]}


def pile_resistance(i):
    """🧱️ EN 1997-1 7.6.2.2 characteristic pile resistance: shaft resistance integrated over the soil layers, R_c = q_b A_b + sum q_s,i pi D h_i, then R_c,k = min(R_mean / xi_1, R_min / xi_2) with xi from the Table A.10 break points through `interp1d`."""
    shaft = float(np.sum(np.array(i["q_s"]) * np.pi * i["D"] * np.array(i["h"])))
    base = i["q_b"] * np.pi * i["D"] ** 2 / 4.0
    ns = (1.0, 2.0, 3.0, 4.0, 5.0, 7.0, 10.0)
    xi1 = float(interpolate.interp1d(ns, (1.40, 1.35, 1.33, 1.31, 1.29, 1.27, 1.25))(i["n_profiles"]))
    xi2 = float(interpolate.interp1d(ns, (1.40, 1.27, 1.23, 1.20, 1.15, 1.12, 1.08))(i["n_profiles"]))
    r_mean, r_min = base + shaft, i["R_min_ratio"] * (base + shaft)
    return {"R_c": base + shaft, "xi_1": xi1, "xi_2": xi2, "R_ck": min(r_mean / xi1, r_min / xi2)}


def anchor_resistance(i):
    """⚓️ EN 1997-1 8.5.2 fixed-length pull-out resistance as the integral of the bond stress over the fixed length, R_a = integral pi d tau dz, the horizontal component R_a cos(theta) and the design check utilisation A_d / R_a."""
    r_a = integrate.quad(lambda z: np.pi * i["d"] * i["tau_f"], 0.0, i["l_f"])[0]
    return {"R_a": r_a, "R_a_h": r_a * np.cos(np.radians(i["theta"])), "utilisation": i["a_d"] / r_a}


def vs30_ground_class(i):
    """🌍️ EN 1998-1 3.1.2 Eq. 3.1: v_s,30 = 30 / sum h_i / v_i by travel-time integration of the layer profile on a fine depth grid (`interp` of the step profile), and the ground class from the Table 3.1 limits (A > 800, B > 360, C > 180, D otherwise)."""
    tops = np.concatenate([[0.0], np.cumsum(i["h"])])
    z = np.linspace(0.0, 30.0, 300001)[:-1] + 0.5 * 30.0 / 300000
    layer = np.clip(np.searchsorted(tops, z, side="right") - 1, 0, len(i["v"]) - 1)
    travel = float(np.sum(30.0 / 300000 / np.array(i["v"])[layer]))
    vs30 = 30.0 / travel
    return {"vs30": vs30, "class_index": float(0 if vs30 > 800 else 1 if vs30 > 360 else 2 if vs30 > 180 else 3)}


def nonstructural_force(i):
    """🏢️ EN 1998-1 4.3.5.2 Eq. 4.24: S_a = alpha S (3 (1 + Z / H) / (1 + (1 - T_a / T_1)^2) - 0.5) not below alpha S, F_a = S_a W_a gamma_a / q_a, with alpha = a_g / g."""
    alpha = i["a_g"] / 9.81
    s_a = max(alpha * i["S"] * (3.0 * (1.0 + i["Z"] / i["H"]) / (1.0 + (1.0 - i["T_a"] / i["T_1"]) ** 2) - 0.5), alpha * i["S"])
    return {"S_a": s_a, "F_a": s_a * i["W_a"] * i["gamma_a"] / i["q_a"]}


def sway_imperfection(i):
    """🏢️ EN 1993-1-1 5.3.2(3): phi = phi_0 alpha_h alpha_m with alpha_h = 2 / sqrt(h) in [2/3, 1] and alpha_m = sqrt(0.5 (1 + 1 / m)); the equivalent horizontal force H = phi V."""
    alpha_h = float(np.clip(2.0 / np.sqrt(i["h"]), 2.0 / 3.0, 1.0))
    phi = (1.0 / 200.0) * alpha_h * np.sqrt(0.5 * (1.0 + 1.0 / i["m"]))
    return {"alpha_h": alpha_h, "phi": phi, "H_eq": phi * i["V"]}


def frame_alpha_cr_eigen(i):
    """🏠️ EN 1993-1-1 5.2.1 elastic critical load factor of a single-storey portal as the lowest eigenvalue of K phi = alpha K_G phi with Euler-Bernoulli elements and consistent geometric stiffness (two fixed-base columns, rigid joints, vertical loads V / 2 on each column head); sway factor of 5.2(4) from the first-order drift at the same loads."""
    e, a, h, span = i["E"], i["A"], i["h"], i["span"]
    members = [((0.0, 0.0), (0.0, h), i["I_col"], 0), ((0.0, h), (span, h), i["I_beam"], 1), ((span, 0.0), (span, h), i["I_col"], 2)]
    nodes = {(0.0, 0.0): 0, (0.0, h): 1, (span, h): 2, (span, 0.0): 3}
    ndof = 12
    k, kg_unit = np.zeros((ndof, ndof)), np.zeros((ndof, ndof))
    for p, q_, inertia, idx in members:
        dx, dy = q_[0] - p[0], q_[1] - p[1]
        length = float(np.hypot(dx, dy))
        c, s = dx / length, dy / length
        ea_l, ei = e * a / length, e * inertia
        kl = np.zeros((6, 6))
        kl[np.ix_([0, 3], [0, 3])] = ea_l * np.array([[1.0, -1.0], [-1.0, 1.0]])
        flex = ei / length**3 * np.array([[12, 6 * length, -12, 6 * length], [6 * length, 4 * length**2, -6 * length, 2 * length**2], [-12, -6 * length, 12, -6 * length], [6 * length, 2 * length**2, -6 * length, 4 * length**2]])
        dofs = [1, 2, 4, 5]
        kl[np.ix_(dofs, dofs)] = flex
        gl = np.zeros((6, 6))
        gl[np.ix_(dofs, dofs)] = 1.0 / (30.0 * length) * np.array([[36, 3 * length, -36, 3 * length], [3 * length, 4 * length**2, -3 * length, -(length**2)], [-36, -3 * length, 36, -3 * length], [3 * length, -(length**2), -3 * length, 4 * length**2]])
        rot = np.zeros((6, 6))
        for blk in (0, 3):
            rot[blk : blk + 3, blk : blk + 3] = [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]]
        ia, ib = nodes[p], nodes[q_]
        gdofs = [3 * ia, 3 * ia + 1, 3 * ia + 2, 3 * ib, 3 * ib + 1, 3 * ib + 2]
        k[np.ix_(gdofs, gdofs)] += rot.T @ kl @ rot
        axial = -i["V"] / 2.0 if idx != 1 else 0.0
        kg_unit[np.ix_(gdofs, gdofs)] += rot.T @ (axial * gl) @ rot
    free = [3, 4, 5, 6, 7, 8]
    kf, gf = k[np.ix_(free, free)], kg_unit[np.ix_(free, free)]
    values = [v.real for v in linalg.eig(kf, -gf, right=False) if np.isfinite(v) and abs(v.imag) < 1e-9]
    load = np.zeros(ndof)
    load[3] = i["H"]
    sway = float(np.linalg.solve(kf, load[free])[0])
    return {"alpha_cr_eigen": float(min(v for v in values if v > 0.0)), "alpha_cr_formula": i["H"] / i["V"] * h / sway, "sway": sway}


def miner_damage(i):
    """🔁️ EN 1990 fatigue and EN 1993-1-9 6.2: Palmgren-Miner damage D = sum n_j / N_j with the two-slope S-N curve N = 2e6 (dsigma_C / dsigma)^3 above the constant-amplitude limit dsigma_D = (2 / 5)^(1/3) dsigma_C, N = 5e6 (dsigma_D / dsigma)^5 down to dsigma_L = (5 / 100)^(1/5) dsigma_D, no damage below, evaluated with numpy masks."""
    ds, n = np.array(i["stress_range"], dtype=float), np.array(i["cycles"], dtype=float)
    dc = i["detail_category"]
    dd = (2.0 / 5.0) ** (1.0 / 3.0) * dc
    dl = (5.0 / 100.0) ** (1.0 / 5.0) * dd
    cap = np.where(ds >= dd, 2.0e6 * (dc / ds) ** 3, np.where(ds >= dl, 5.0e6 * (dd / ds) ** 5, np.inf))
    return {"D": float(np.sum(n / cap)), "dsigma_D": dd, "dsigma_L": dl}


def annex_d_kn(i):
    """🧪️ EN 1990 Annex D.7.2 Table D1 coefficients: k_n for unknown coefficient of variation from the Student quantile, k_n = t_{n-1, 0.95} sqrt(1 + 1 / n) (`scipy.stats.t`), for known V from k_n = 1.645 sqrt(1 + 1 / n); characteristic value X_k = m_x (1 - k_n V_x) or m_x - k_n s for the normal case."""
    n = i["n"]
    unknown = float(stats.t.ppf(0.95, df=n - 1) * np.sqrt(1.0 + 1.0 / n))
    known = float(stats.norm.ppf(0.95) * np.sqrt(1.0 + 1.0 / n))
    return {"k_n_unknown": unknown, "k_n_known": known, "X_k": i["mean"] - unknown * i["s"], "X_k_known": i["mean"] - known * i["sigma"]}


def annex_f_membrane(i):
    """🧱️ EN 1992-1-1 Annex F in-plane stress reinforcement from membrane equilibrium: a compression field at angle theta carries F_c; tie forces f_x = n_x + |n_xy| cot theta, f_y = n_y + |n_xy| tan theta (tension positive); the total f_x + f_y is minimised over theta with both ties non-negative (`minimize_scalar`), which reproduces the Annex F cases without evaluating them."""
    nx, ny, nxy = i["n_x"], i["n_y"], abs(i["n_xy"])
    lo = np.arctan(max(-ny / nxy, 0.0) if nxy else 1.0) + 1e-9 if nxy else np.pi / 4.0
    hi = np.arctan(nxy / -nx) - 1e-9 if nxy and nx < 0.0 else np.pi / 2.0 - 1e-9
    if nxy == 0.0:
        return {"f_x": max(nx, 0.0), "f_y": max(ny, 0.0), "theta": np.pi / 4.0}
    total = lambda th: max(nx + nxy / np.tan(th), 0.0) + max(ny + nxy * np.tan(th), 0.0)
    th = optimize.minimize_scalar(total, bounds=(max(lo, 1e-6), min(hi, np.pi / 2.0 - 1e-6)), method="bounded", options={"xatol": 1e-13}).x
    snap = lambda v: 0.0 if v < 1e-7 * (abs(nx) + abs(ny) + nxy) else v
    return {"f_x": snap(nx + nxy / np.tan(th)), "f_y": snap(ny + nxy * np.tan(th)), "theta": float(th)}


def aged_modulus_factor(i):
    """⏳️ EN 1992-2 Annex KK age-adjusted effective modulus E_adj = E / (1 + chi phi): the relaxation function R of the EN 1992-1-1 Annex B creep law follows from the Volterra equation 1 = sum J(t_k, tau_j) d sigma_j (unit imposed strain, J = (1 + phi) / E, midpoint rule), and the aging coefficient is chi = E / (E - R) - 1 / phi (Trost-Bazant)."""
    e, grid = i["E"], np.concatenate([[i["t0"]], np.geomspace(i["t0"] + 0.01, i["t"], 600)])
    beta_fcm = 16.8 / np.sqrt(i["f_cm"] / 1.0e6)

    def phi(t_, tau):
        return i["phi_RH"] * beta_fcm / (0.1 + tau**0.2) * ((t_ - tau) / (i["beta_H"] + (t_ - tau))) ** 0.3

    mid = 0.5 * (grid[1:] + grid[:-1])
    steps = [e]
    for k in range(1, len(grid)):
        acc = (1.0 + phi(grid[k], grid[0])) / e * steps[0]
        for j in range(1, k):
            acc += (1.0 + phi(grid[k], mid[j - 1])) / e * steps[j]
        steps.append((1.0 - acc) / ((1.0 + phi(grid[k], mid[k - 1])) / e))
    relaxation = float(np.sum(steps))
    phi_t = phi(i["t"], i["t0"])
    chi = e / (e - relaxation) - 1.0 / phi_t
    return {"phi": phi_t, "chi": chi, "E_adj": e / (1.0 + chi * phi_t)}


CHECKS |= {
    "en1991.4.wind-peak-pressure": wind_peak_pressure,
    "en1991.7.wind-cpe": wind_cpe,
    "en1991.snow.shape-mono-pitch": snow_shape_mono_pitch,
    "en1991.snow.drift": snow_drift,
    "en1991.snow.drift-gamma-limited": snow_drift_gamma_limited,
    "en1991.fire.parametric-curve": fire_parametric,
    "en1991.fire.iso834": fire_iso834,
    "en1996.G.phi-m": masonry_phi_m,
    "en1996.E.yield-line-pressure": yield_line_pressure,
    "en1996.E.alpha1-standin": yield_line_alpha1,
    "en1996.6.2.shear-wall": masonry_shear,
    "en1997.D.4.bearing-resistance": bearing_factors_annex_d,
    "en1997.F.stress-strain-settlement": stress_strain_settlement,
    "en1997.F.terzaghi-degree": terzaghi_degree,
    "en1997.9.embedded-wall": embedded_wall,
    "en1997.7.pile-group": pile_group_efficiency,
    "en1997.7.pile-resistance": pile_resistance,
    "en1997.8.anchor-resistance": anchor_resistance,
    "en1998.3.1.vs30": vs30_ground_class,
    "en1998.4.3.5.2.nonstructural": nonstructural_force,
    "en1993.5.3.2.sway-imperfection": sway_imperfection,
    "en1993.5.2.alpha-cr-storey": frame_alpha_cr_eigen,
    "en1990.fatigue.miner": miner_damage,
    "en1990.D.7.characteristic-from-tests": annex_d_kn,
    "en1992.F.membrane-reinforcement": annex_f_membrane,
    "en1992.KK.age-adjusted-modulus": aged_modulus_factor,
}


def i_rectangles(i):
    """📐️ Doubly symmetric I constants from the three rectangles: A, I_y, I_z, thin-walled I_t = sum b t^3 / 3 and I_w from the flange bending of the two flanges at distance h - t_f (I_w = I_z,flanges (h - t_f)^2 / 4)."""
    h, b, tf, tw = i["h"], i["b"], i["tf"], i["tw"]
    hw = h - 2.0 * tf
    flange_z = 2.0 * tf * b**3 / 12.0
    return {"A": 2.0 * b * tf + hw * tw, "Iy": b * h**3 / 12.0 - (b - tw) * hw**3 / 12.0, "Iz": flange_z + hw * tw**3 / 12.0, "It": (2.0 * b * tf**3 + hw * tw**3) / 3.0, "Iw": flange_z * (h - tf) ** 2 / 4.0}


def chi_by_root(lam, alpha, lam0):
    """📉️ Smaller root of the Ayrton-Perry quadratic lambda^2 chi^2 - (1 + alpha (lambda - lambda_0) + lambda^2) chi + 1 = 0 by brentq on (0, 1], 1 when lambda <= lambda_0."""
    if lam <= lam0:
        return 1.0
    eta = alpha * (lam - lam0)
    f = lambda c: (1.0 - c) * (1.0 - lam * lam * c) - eta * c
    return optimize.brentq(f, 1e-12, min(1.0, 1.0 / lam**2), xtol=1e-15, rtol=1e-14) if lam > 1.0 else optimize.brentq(f, 1e-12, 1.0, xtol=1e-15, rtol=1e-14)


def flexural_buckling_fe(i):
    """🪶️ EN 1999-1-1 6.3.1 with N_cr from the finite-element Euler eigenproblem (32 Hermite elements) and chi from the root of the Ayrton-Perry equation."""
    c = i_rectangles(i)
    n_cr = euler_critical_load({"EI": i["E"] * c["Iz"], "L": i["L"]})["N_cr"]
    lam = float(np.sqrt(c["A"] * i["f_o"] / n_cr))
    chi = chi_by_root(lam, i["alpha"], i["lambda_0"])
    return {"A": c["A"], "Iz": c["Iz"], "N_cr": n_cr, "lambda": lam, "chi": chi, "N_b_Rd": i.get("kappa", 1.0) * chi * c["A"] * i["f_o"] / i["gamma_M1"]}


def hermite_matrices(h):
    """🧩️ Element matrices of cubic Hermite shape functions over length h by four-point Gauss-Legendre quadrature: second-derivative stiffness K2 = int N'' N''^T, first-derivative K1 = int N' N'^T, mass M0 = int N N^T and the coupling C = int N'' N^T."""
    xi, w = np.polynomial.legendre.leggauss(4)
    xi, w = 0.5 * (xi + 1.0), 0.5 * w
    k2, k1, m0, cp = (np.zeros((4, 4)) for _ in range(4))
    for x, wt in zip(xi, w):
        n = np.array([1 - 3 * x**2 + 2 * x**3, h * (x - 2 * x**2 + x**3), 3 * x**2 - 2 * x**3, h * (-(x**2) + x**3)])
        d1 = np.array([-6 * x + 6 * x**2, h * (1 - 4 * x + 3 * x**2), 6 * x - 6 * x**2, h * (-2 * x + 3 * x**2)]) / h
        d2 = np.array([-6 + 12 * x, h * (-4 + 6 * x), 6 - 12 * x, h * (-2 + 6 * x)]) / h**2
        k2 += wt * h * np.outer(d2, d2)
        k1 += wt * h * np.outer(d1, d1)
        m0 += wt * h * np.outer(n, n)
        cp += wt * h * np.outer(d2, n)
    return k2, k1, m0, cp


def ltb_fe(i):
    """🦋️ Elastic critical moment under uniform moment by a Vlasov beam finite element (v, v', phi, phi' per node, cubic Hermite for both fields, 48 elements): K = diag(EI_z K2, EI_w K2 + GI_t K1 + k_s M0) and the geometric coupling M (v'' phi) as C; generalised eigenproblem K x = M_cr G x, the smallest positive eigenvalue with fork supports, then chi_LT with the quadratic root."""
    c = i_rectangles(i)
    n_el = 48
    h = i["L"] / n_el
    k2, k1, m0, cp = hermite_matrices(h)
    ndof = 4 * (n_el + 1)
    kv, kt, g = np.zeros((ndof, ndof)), np.zeros((ndof, ndof)), np.zeros((ndof, ndof))
    ei_z, ei_w, gi_t, ks = i["E"] * c["Iz"], i["E"] * c["Iw"], i["G"] * c["It"], i.get("k_s", 0.0)
    for e in range(n_el):
        s = slice(2 * e, 2 * e + 4)
        kv[s, s] += ei_z * k2
        kt[s, s] += ei_w * k2 + gi_t * k1 + ks * m0
        g[s, s] += cp
    size = 2 * (n_el + 1)
    kfull, gfull = np.zeros((2 * size, 2 * size)), np.zeros((2 * size, 2 * size))
    kfull[:size, :size], kfull[size:, size:] = kv[:size, :size], kt[:size, :size]
    gfull[:size, size:], gfull[size:, :size] = g[:size, :size], g[:size, :size].T
    fixed = {0, size - 2, size, 2 * size - 2}
    free = [d for d in range(2 * size) if d not in fixed]
    values = linalg.eig(kfull[np.ix_(free, free)], gfull[np.ix_(free, free)], right=False)
    m_cr = float(min(v.real for v in values if np.isfinite(v) and abs(v.imag) < 1e-6 * abs(v.real) and v.real > 0.0))
    w_el = c["Iy"] / (i["h"] / 2.0)
    lam = float(np.sqrt(w_el * i["f_o"] / m_cr))
    chi = chi_by_root(lam, i["alpha_LT"], i["lambda_0"])
    return {"Iz": c["Iz"], "It": c["It"], "Iw": c["Iw"], "M_cr": m_cr, "lambda_LT": lam, "chi_LT": chi, "M_b_Rd": chi * w_el * i["f_o"] / i["gamma_M1"]}


def stud_resistance_numeric(i):
    """🔩️ EN 1994-1-1 6.6.3.1 and 6.6.4.2 with alpha from a lookup through (3, 0.8) and (4, 1.0) by `np.interp`, the two failure modes taken with `np.minimum`."""
    d, h = i["d"], i["h_sc"]
    alpha = float(np.interp(h / d, [3.0, 4.0], [0.8, 1.0]))
    steel = 0.8 * i["f_u"] * np.pi * d * d / 4.0 / i["gamma_V"]
    concrete = 0.29 * alpha * d * d * np.sqrt(i["f_ck"] * i["E_cm"]) / i["gamma_V"]
    p_rd = float(np.minimum(steel, concrete))
    k_t = float(np.minimum(0.7 / np.sqrt(i["n_r"]) * i["b_0"] / i["h_p"] * (h / i["h_p"] - 1.0), i["k_t_max"]))
    return {"alpha": alpha, "P_Rd_steel": steel, "P_Rd_concrete": concrete, "P_Rd": p_rd, "k_t": k_t, "P_Rd_sheeting": k_t * p_rd}


def weld_group_numeric(i):
    """🧵️ Box weld group by numerical line integration: I_p = integral r^2 dl by Gauss quadrature along the four sides, and the largest resultant per length by scanning 2000 points on the perimeter."""
    b, d, f, e = i["b"], i["d"], i["F"], i["e"]
    sides = [((-b / 2, -d / 2), (b / 2, -d / 2)), ((b / 2, -d / 2), (b / 2, d / 2)), ((b / 2, d / 2), (-b / 2, d / 2)), ((-b / 2, d / 2), (-b / 2, -d / 2))]
    ip = 0.0
    for (x0, y0), (x1, y1) in sides:
        length = np.hypot(x1 - x0, y1 - y0)
        ip += integrate.quad(lambda s: (x0 + (x1 - x0) * s) ** 2 + (y0 + (y1 - y0) * s) ** 2, 0.0, 1.0)[0] * length
    lw = 2.0 * (b + d)
    peak = 0.0
    for (x0, y0), (x1, y1) in sides:
        for s in np.linspace(0.0, 1.0, 500):
            x, y = x0 + (x1 - x0) * s, y0 + (y1 - y0) * s
            peak = max(peak, float(np.hypot(-f * e * y / ip, f / lw + f * e * x / ip)))
    return {"L_w": lw, "I_p": ip, "f_max": peak}


def ltb_ncci_mcr(i):
    """🦋️ Closed form for the sinusoidal LTB mode used only as a cross-check of the finite element."""
    c = i_rectangles(i)
    a = np.pi / i["L"]
    return {"M_cr": float(np.sqrt(i["E"] * c["Iz"] * (i["E"] * c["Iw"] * a**4 + i["G"] * c["It"] * a**2 + i.get("k_s", 0.0))))}


CHECKS |= {
    "en1999.6.3.1.flexural-buckling": flexural_buckling_fe,
    "en1999.6.3.2.ltb": ltb_fe,
    "en1994.6.4.ltb-restrained": ltb_fe,
    "en1994.6.6.3.stud-resistance": stud_resistance_numeric,
    "en1993.1-8.4.5.weld-group-box": weld_group_numeric,
}


def oedometer_integral(i):
    """🧪️ Primary consolidation settlement of a normally consolidated layer by integrating the compression line e(sigma') = e_0 - C_c log10(sigma' / sigma'_v0) over the stress increase: s = H_0 / (1 + e_0) integral C_c / (sigma' ln 10) d sigma'."""
    s0 = i["sigma_v0"]
    d_e = integrate.quad(lambda sg: i["C_c"] / (sg * np.log(10.0)), s0, s0 + i["d_sigma"])[0]
    return {"s": i["H0"] / (1.0 + i["e0"]) * d_e, "delta_e": d_e}


def pin_bending(i):
    """📌️ Maximum bending moment of a pin in double shear at mid-span: the right half carries the inner-plate pressure F / b over 0 to b / 2 downward and the outer-plate reaction F / (2 a) over b / 2 + c to b / 2 + c + a upward; M = |integral q(x) x dx| by quadrature."""
    f, a, b, c = i["F"], i["a"], i["b"], i["c"]
    outer = integrate.quad(lambda x: f / (2.0 * a) * x, b / 2.0 + c, b / 2.0 + c + a)[0]
    inner = integrate.quad(lambda x: f / b * x, 0.0, b / 2.0)[0]
    return {"M_Ed": abs(outer - inner)}


CHECKS |= {"en1997.F.oedometer-settlement": oedometer_integral, "en1999.8.5.pin-bending-moment": pin_bending}


def anchor_elongation(i):
    """⚓️ Free-length elongation delta = integral N / (E A) dx of the constant axial force."""
    return {"delta": integrate.quad(lambda x: i["a_d"] / (i["E"] * i["A_t"]), 0.0, i["l_free"])[0], "N": i["a_d"]}


def corbel_truss(i):
    """🏛️ Corbel truss by equilibrium of the load joint as a 2 x 2 linear system in the tie tension T and the strut compression S: -T + S a / d + H = 0 and S z / d - F = 0 with d = sqrt(a^2 + z^2), `numpy.linalg.solve`."""
    a, z = i["a"], i["z"]
    d = np.hypot(a, z)
    t, s = np.linalg.solve(np.array([[-1.0, a / d], [0.0, z / d]]), np.array([-i["H"], i["F"]]))
    return {"F_t": t, "C": s, "theta": float(np.arctan2(z, a))}


CHECKS |= {"en1997.8.anchor-elongation": anchor_elongation, "en1992.J.3.corbel-truss": corbel_truss}


def mkappa_linear(i):
    """📉️ EN 1994-1-1 6.2.1.4 moment-curvature point at a small curvature `kappa` with linear-elastic steel and concrete (no tension in the slab): strip integration over slab and the three steel rectangles, the top-fibre strain from N = 0 by brentq, M = -integral sigma y dA about the slab top."""
    e_a, e_c, beff, hc, ys, h, b, tf, tw = (i[k] for k in ("E_a", "E_c", "b_eff", "h_c", "y_s", "h", "b", "tf", "tw"))
    n_strips = 4000

    def forces(kappa, eps_top):
        yc = (np.arange(n_strips) + 0.5) * hc / n_strips
        f_c = np.maximum(e_c * (eps_top - kappa * yc), 0.0) * beff * hc / n_strips
        n_total, m_total = f_c.sum(), -(f_c * yc).sum()
        for top, bottom, width in ((ys, ys + tf, b), (ys + tf, ys + h - tf, tw), (ys + h - tf, ys + h, b)):
            y = top + (np.arange(n_strips) + 0.5) * (bottom - top) / n_strips
            f = e_a * (eps_top - kappa * y) * width * (bottom - top) / n_strips
            n_total, m_total = n_total + f.sum(), m_total - (f * y).sum()
        return n_total, m_total

    eps_top = optimize.brentq(lambda e: forces(i["kappa"], e)[0], -1.0, 1.0, xtol=1e-16)
    return {"M": abs(forces(i["kappa"], eps_top)[1])}


CHECKS |= {"en1994.6.2.1.4.mkappa-elastic": mkappa_linear}


def lognormal_gumbel(i):
    """🎲️ Frozen scipy distributions of R (lognormal) and E (Gumbel for maxima) from mean and standard deviation."""
    cov = i["sigma_R"] / i["mu_R"]
    s = np.sqrt(np.log(1.0 + cov * cov))
    r = stats.lognorm(s, scale=i["mu_R"] / np.sqrt(1.0 + cov * cov))
    scale = i["sigma_E"] * np.sqrt(6.0) / np.pi
    e = stats.gumbel_r(loc=i["mu_E"] - np.euler_gamma * scale, scale=scale)
    return r, e


def form_nonnormal_sqp(i):
    """🎯️ FORM of g = R - E (R lognormal, E Gumbel) in standard normal space: the iso-probabilistic transform x = F^-1(Phi(u)) of each variable and the design point as the minimum of |u| on g = 0 by SLSQP; also the exact beta = -Phi^-1(P_f) with P_f = integral f_E(e) F_R(e) de by quadrature."""
    r, e = lognormal_gumbel(i)
    to_x = lambda dist, u: dist.ppf(stats.norm.cdf(u))
    result = optimize.minimize(lambda u: u @ u, np.array([-1.0, 1.0]), constraints=[{"type": "eq", "fun": lambda u: to_x(r, u[0]) - to_x(e, u[1])}], method="SLSQP", options={"ftol": 1e-14, "maxiter": 500})
    pf = integrate.quad(lambda x: e.pdf(x) * r.cdf(x), e.ppf(1e-12), e.ppf(1.0 - 1e-12), limit=400)[0]
    return {"beta": float(np.sqrt(result.fun)), "R_d": float(to_x(r, result.x[0])), "E_d": float(to_x(e, result.x[1])), "beta_exact": float(-stats.norm.ppf(pf))}


def form_nonnormal_rackwitz(i):
    """🎯️ FORM of g = R - E by the Rackwitz-Fiessler iteration in physical space: at the current point each non-normal variable is replaced by the normal with the same cdf and pdf (sigma' = pdf(Phi^-1(cdf)) / pdf, mu' = x - Phi^-1(cdf) sigma'), the linear limit state gives beta and the next point."""
    r, e = lognormal_gumbel(i)
    x = np.array([i["mu_R"], i["mu_E"]], dtype=float)
    for _ in range(200):
        parts = []
        for dist, value in ((r, x[0]), (e, x[1])):
            z = stats.norm.ppf(dist.cdf(value))
            sigma = stats.norm.pdf(z) / dist.pdf(value)
            parts.append((value - z * sigma, sigma))
        (mr, sr), (me, se) = parts
        beta = (mr - me) / np.hypot(sr, se)
        alpha_r, alpha_e = sr / np.hypot(sr, se), -se / np.hypot(sr, se)
        new = np.array([mr - alpha_r * beta * sr, me - alpha_e * beta * se])
        if np.max(np.abs(new - x)) < 1e-10 * max(1.0, np.max(np.abs(x))):
            x = new
            break
        x = new
    return {"beta": float(beta), "R_d": float(x[0]), "E_d": float(x[1])}


CHECKS |= {"en1990.C.form-nonnormal": form_nonnormal_sqp, "en1990.C.form-nonnormal-rf": form_nonnormal_rackwitz, "en1993.5.2.alpha-cr-eigen": frame_alpha_cr_eigen}


def settlement_pairs(i):
    """📏️ EN 1997-1 Annex H serviceability pairs: the stress-strain settlement of each footing (`stress_strain_settlement` of this adapter), the angular distortion between the first two footings and the gradient of the least-squares settlement plane (numpy.linalg.lstsq) through the footing positions."""
    import numpy as np

    s = np.array([stress_strain_settlement({"B": i["B"], "L": i["L"], "q": q, "E_m": i["E_m"], "z_max": i["z_max"]})["s"] for q in i["q"]])
    xy = np.array(i["xy"], dtype=float)
    design = np.column_stack([np.ones(len(s)), xy[:, 0], xy[:, 1]])
    plane = np.linalg.lstsq(design, s, rcond=None)[0]
    return {"s_1": float(s[0]), "beta_12": float(abs(s[1] - s[0]) / np.hypot(*(xy[1] - xy[0]))), "tilt": float(np.hypot(plane[1], plane[2]))}


CHECKS |= {"en1997.H.settlement-pairs": settlement_pairs}


def interaction_stress(q, a, b, px, py, z):
    """📐️ Vertical stress at offset (px, py) from the centre of a rectangle of half sides a and b: signed Newmark corner factors, I(-m, n) = -I(m, n)."""
    corner = lambda u, v: np.sign(u) * np.sign(v) * newmark_corner(abs(u) / z, abs(v) / z)
    return q * (corner(a - px, b - py) - corner(-a - px, b - py) - corner(a - px, -b - py) + corner(-a - px, -b - py))


def settlement_interaction(i):
    """🤝 EN 1997-1 Annex F.1 and H: the stress-strain settlement of each of several identical footings at its own centre with the stress of every other footing superposed (scipy.integrate.quad over the depth), the angular distortion between the first two footings and the gradient of the least-squares settlement plane (numpy.linalg.lstsq)."""
    a, b = i["B"] / 2.0, i["L"] / 2.0
    xy = np.array(i["xy"], dtype=float)
    q = i["q"]

    def at(receiver, loaded):
        px, py = xy[receiver] - xy[loaded]
        return integrate.quad(lambda z: interaction_stress(q[loaded], a, b, px, py, z) / i["E_m"], 1e-9, i["z_max"], limit=200)[0]

    s = np.array([sum(at(k, j) for j in range(len(q))) for k in range(len(q))])
    alone = at(0, 0)
    design = np.column_stack([np.ones(len(s)), xy[:, 0], xy[:, 1]])
    plane = np.linalg.lstsq(design, s, rcond=None)[0]
    return {"s_1": float(s[0]), "s_1_alone": float(alone), "beta_12": float(abs(s[1] - s[0]) / np.hypot(*(xy[1] - xy[0]))), "tilt": float(np.hypot(plane[1], plane[2]))}


CHECKS |= {"en1997.H.settlement-interaction": settlement_interaction}


def pile_lateral_winkler(i):
    """🪵️ EN 1997-1 7.7 and 7.8 lateral head deflection, maximum moment and elastic critical load of a pile on a Winkler foundation: `solve_bvp` of EI y'''' + k_h D y = 0 (head load H, free tip) for y and M = -EI y'', `eigh` of the finite-difference beam for the pinned critical load."""
    ei, k_h, d, length, head = i["EI"], i["k_h"], i["D"], i["L"], i["H"]
    ks = k_h * d
    x = np.linspace(0.0, length, 600)
    solution = integrate.solve_bvp(lambda _, s: np.vstack([s[1], s[2], s[3], -ks / ei * s[0]]), lambda a, b: np.array([a[2], a[3] - head / ei, b[2], b[3]]), x, np.zeros((4, x.size)), tol=1e-12, max_nodes=200000)
    assert solution.status == 0
    depth = np.linspace(0.0, length, 6001)
    nodes = 1200
    h, n = length / nodes, nodes - 1
    second = (np.diag(-2.0 * np.ones(n)) + np.diag(np.ones(n - 1), 1) + np.diag(np.ones(n - 1), -1)) / h**2
    geometric = (np.diag(2.0 * np.ones(n)) - np.diag(np.ones(n - 1), 1) - np.diag(np.ones(n - 1), -1)) / h
    critical = linalg.eigh(ei * (second @ second.T) * h + ks * h * np.eye(n), geometric, eigvals_only=True, subset_by_index=[0, 0])[0]
    return {"y_0": float(solution.sol(0.0)[0]), "M_max": float(np.max(np.abs(ei * solution.sol(depth)[2]))), "N_cr": float(critical)}


def mononobe_okabe_wedge(i):
    """🌋 EN 1998-5 Annex E thrust of the active wedge behind a vertical wall: the force polygon (weight with the vertical factor, inertia k_h W towards the wall, wall reaction at delta, plane reaction at phi) maximised over the plane angle with `minimize_scalar`, for both signs of k_v; the more onerous sign is reported with its wedge angle."""
    gamma, h, phi, delta, kh, kv = i["gamma"], i["H"], np.radians(i["phi"]), np.radians(i["delta"]), i["k_h"], i["k_v"]
    best = None
    for sign in (1.0, -1.0):
        vertical = 1.0 - sign * kv

        def thrust(rho, vertical=vertical):
            a, b = -np.sin(rho) + np.tan(phi) * np.cos(rho), np.cos(rho) + np.tan(phi) * np.sin(rho)
            return 0.5 * gamma * h * h / np.tan(rho) * (kh * b - vertical * a) / (np.cos(delta) * b - np.sin(delta) * a)

        found = optimize.minimize_scalar(lambda rho: -thrust(rho), bounds=(np.radians(1.0), np.radians(89.0)), method="bounded", options={"xatol": 1e-12})
        if best is None or -found.fun > best[0]:
            best = (-float(found.fun), float(np.degrees(found.x)), vertical)
    thrust_max, angle, vertical = best
    return {"E_ae": thrust_max, "K_ae": thrust_max / (0.5 * gamma * vertical * h * h), "rho_deg": angle}


def mononobe_okabe_water(i):
    """🌊 EN 1998-5 Annex E wall thrust with retained water: the dry layer at the angle of k_h, the submerged layer at the angle of k_h gamma_inertia / (gamma_sat - gamma_w) (dry weight for a permeable backfill, saturated weight otherwise), each coefficient from the maximised force polygon, plus the hydrostatic water thrust and, for a permeable backfill, the Westergaard thrust 7/12 k_h gamma_w H_w^2 from the integral of 7/8 k_h gamma_w sqrt(H_w z)."""
    gamma, h, wet, gamma_sat, kh, permeable = i["gamma"], i["H"], i["H_w"], i["gamma_sat"], i["k_h"], i["permeable"] > 0.5
    gamma_w = 9810.0
    effective, dry = gamma_sat - gamma_w, h - wet
    k = lambda kh_eff: mononobe_okabe_wedge({"gamma": 1.0, "H": 1.0, "phi": i["phi"], "delta": 0.0, "k_h": kh_eff, "k_v": 0.0})["K_ae"]
    inertia = gamma if permeable else gamma_sat
    soil = 0.5 * gamma * dry**2 * k(kh) + k(kh * inertia / effective) * (gamma * dry * wet + 0.5 * effective * wet**2)
    pressure = lambda z: 7.0 / 8.0 * kh * gamma_w * np.sqrt(wet * z)
    dynamic = integrate.quad(pressure, 0.0, wet)[0] if permeable else 0.0
    static = integrate.quad(lambda z: gamma_w * z, 0.0, wet)[0]
    return {"H_soil": soil, "H_static": static, "H_dynamic": dynamic, "H_total": soil + static + dynamic}


def slope_slices(h, beta, xc, yc, rad, gamma, n):
    """⛰️ The n slices of a circle in a dry slope (crest at x = 0, toe at h / tan beta), evaluated on the same 4001-point scan for the extent of the sliding mass as the kernel: width, weight, base angle and centroid height."""
    tb = np.tan(np.radians(beta))
    run = h / tb
    ground = lambda x: np.where(x <= 0.0, h, np.where(x >= run, 0.0, h - x * tb))
    xs = np.linspace(xc - rad, xc + rad, 4001)
    lower = yc - np.sqrt(np.maximum(rad * rad - (xs - xc) ** 2, 0.0))
    inside = xs[lower < ground(xs)]
    if inside.size == 0 or inside[-1] - inside[0] < 1e-6:
        return None
    xa, xb = inside[0], inside[-1]
    width = (xb - xa) / n
    x = xa + (np.arange(n) + 0.5) * width
    base = yc - np.sqrt(np.maximum(rad * rad - (x - xc) ** 2, 0.0))
    height = np.maximum(ground(x) - base, 0.0)
    return {"b": width, "w": gamma * height * width, "alpha": np.arcsin((xc - x) / rad), "ycg": base + 0.5 * height}


def slope_bishop(sl, yc, rad, tan_phi, c, kh, share, sign):
    """⛰️ Simplified Bishop factor under F_H = k_h W and F_V = sign share F_H at the slice centroids: the root of the moment equilibrium by brentq between the pole of the slice denominators and 1e4, `None` without a driving moment or a bracketed root."""
    fh = kh * sl["w"]
    wv = sl["w"] + sign * share * fh
    drive = np.sum(wv * np.sin(sl["alpha"]) + fh * (yc - sl["ycg"]) / rad)
    if drive <= 0.0:
        return None
    residual = lambda f: np.sum((c * sl["b"] + np.maximum(wv, 0.0) * tan_phi) / (np.cos(sl["alpha"]) * (1.0 + np.tan(sl["alpha"]) * tan_phi / f))) / drive - f
    pole = max(0.0, -float(np.min(np.tan(sl["alpha"]))) * tan_phi)
    low, high = pole * (1.0 + 1e-9) + 1e-9, 1.0e4
    if residual(low) * residual(high) > 0.0:
        return None
    return float(optimize.brentq(residual, low, high, xtol=1e-13))


def slope_least(i, xc, yc, rad, kh):
    """⛰️ The least Bishop factor of both vertical signs of one circle, `None` when the circle develops no sliding mass."""
    sl = slope_slices(i["H"], i["beta"], xc, yc, rad, i["gamma"], 40)
    if sl is None:
        return None
    tan_phi, c = np.tan(np.radians(i["phi"])) / i["gamma_phi"], i["c"] / i["gamma_c"]
    factors = [slope_bishop(sl, yc, rad, tan_phi, c, kh, i["share"], sign) for sign in (1.0, -1.0)]
    return None if None in factors else min(factors)


def seismic_slope(i):
    """⛰️ EN 1998-5 4.1.3.3 pseudo-static slope: k_h = factor alpha S S_T from the design acceleration, the M2 strengths, the least Bishop factor of both vertical signs for the given circle and for the grid of circles (centres x from -h/2 to the toe plus h/2 and z from 0.8 h to 2.4 h, radii 0.85 to 1.3 times the toe distance)."""
    h = i["H"]
    kh = i["factor"] * i["a_g"] / 9.81 * i["S"] * i["S_T"]
    run = h / np.tan(np.radians(i["beta"]))
    grid = [slope_least(i, xc, yc, np.hypot(xc - run, yc) * scale, kh) for xc in -0.5 * h + (run + h) * np.arange(9) / 8.0 for yc in h * (0.8 + 1.6 * np.arange(7) / 6.0) for scale in (0.85, 1.0, 1.15, 1.3)]
    critical = min(f for f in grid if f is not None)
    given = slope_least(i, i["xc"], i["yc"], i["R"], kh)
    return {"F_static": slope_least(i, i["xc"], i["yc"], i["R"], 0.0), "F_given": given, "F_critical": critical, "F_s": min(given, critical), "k_h": kh}


def tie_beam_force(i):
    """🔗 EN 1998-5 5.4.1.2 tie force N = coefficient alpha S N_Ed with alpha = a_g / g, the steel resistance A_s f_yd and their ratio, evaluated on `numpy` arrays of the ground type coefficients so that the ground type is looked up by index."""
    coefficients = np.array([0.3, 0.4, 0.6])
    n_tie = float(coefficients[int(i["ground_index"])] * i["a_g"] / 9.81 * i["S"] * i["N_Ed"])
    n_rd = float(i["A_s"] * i["f_yd"])
    return {"N_tie": n_tie, "N_Rd": n_rd, "utilisation": n_tie / n_rd}


CHECKS |= {"en1997.7.pile-lateral-winkler": pile_lateral_winkler, "en1997.9.mononobe-okabe": mononobe_okabe_wedge, "en1998.5.mononobe-okabe": mononobe_okabe_wedge, "en1998.5.wall-water": mononobe_okabe_water, "en1998.5.seismic-slope": seismic_slope, "en1998.5.tie-beam": tie_beam_force}


def bridge_wind_force(i):
    """🌉 EN 1991-1-4 8.3 horizontal, vertical and longitudinal wind force per metre of a bridge deck: d_tot from the larger of the parapet and the traffic increment, c_f,x0 by `interp1d` through the two plateaus (registry values, unverified), forces as q_p c d."""
    parapet = {0: 0.0, 1: 2.0 * i["open_add"], 2: 2.0 * i["d1"]}[int(i["parapet_kind"])]
    traffic = {0: 0.0, 1: i["road_add"], 2: i["rail_add"]}[int(i["traffic_kind"])]
    d_tot = i["d"] + max(parapet, traffic)
    c_fx0 = float(interpolate.interp1d((i["r_narrow"], i["r_wide"]), (i["cfx_narrow"], i["cfx_wide"]), bounds_error=False, fill_value=(i["cfx_narrow"], i["cfx_wide"]))(i["b"] / d_tot))
    f_x = i["q_p"] * c_fx0 * d_tot
    return {"d_tot": d_tot, "c_fx0": c_fx0, "F_x": f_x, "F_z": i["q_p"] * i["cfz"] * i["b"], "F_y": i["longitudinal"] * f_x}


def structural_factor_annex_b(i):
    """🌀 EN 1991-1-4 6.3 and Annex B structural factor c_s c_d: the admittance functions R_h and R_b by `quad` of 2 int_0^1 (1 - u) exp(-2 eta u) du instead of their closed form, the peak factor k_p from the up-crossing rate."""
    z_s = max(0.6 * i["h"], i["z_min"])
    alpha = 0.67 + 0.05 * np.log(i["z0"])
    length = i["L_t"] * (z_s / i["z_t"]) ** alpha
    b2 = 1.0 / (1.0 + 0.9 * ((i["b"] + i["h"]) / length) ** 0.63)
    k_r = 0.19 * (i["z0"] / i["z0_II"]) ** 0.07
    v_m = k_r * np.log(z_s / i["z0"]) * i["v_b"]
    f_l = i["n1"] * length / v_m
    s_l = 6.8 * f_l / (1.0 + 10.2 * f_l) ** (5.0 / 3.0)
    admittance = lambda eta: 1.0 if eta <= 0.0 else 2.0 * integrate.quad(lambda u: (1.0 - u) * np.exp(-2.0 * eta * u), 0.0, 1.0)[0]
    r2 = np.pi**2 / (2.0 * i["delta"]) * s_l * admittance(4.6 * i["h"] / length * f_l) * admittance(4.6 * i["b"] / length * f_l)
    nu = max(i["n1"] * np.sqrt(r2 / (b2 + r2)), i["nu_min"])
    root = np.sqrt(2.0 * np.log(nu * i["T"]))
    k_p = max(root + 0.6 / root, 3.0)
    i_v = 1.0 / np.log(z_s / i["z0"])
    return {"z_s": z_s, "L": length, "B2": b2, "S_L": s_l, "R2": r2, "nu": nu, "k_p": k_p, "c_s_c_d": (1.0 + 2.0 * k_p * i_v * np.sqrt(b2 + r2)) / (1.0 + 7.0 * i_v)}


def vortex_shedding_onset(i):
    """🌪️ EN 1991-1-4 Annex E critical velocity as the root of the shedding frequency St v / b = n_1 (`brentq`), the Scruton number and the Reynolds number at that velocity."""
    v_crit = optimize.brentq(lambda v: i["St"] * v / i["b"] - i["n1"], 1.0e-6, 1.0e4)
    return {"v_crit": v_crit, "Sc": 2.0 * i["delta_s"] * i["m_e"] / (i["rho"] * i["b"] ** 2), "v_onset": i["onset"] * i["v_m"], "Re": i["b"] * v_crit / i["nu"]}


def impact_spring_peak(i):
    """💥 EN 1991-1-7 Annex C equivalent static force of a mass hitting a linear spring: the peak spring force of m x'' + k x = 0 with x(0) = 0 and x'(0) = v from `solve_ivp`, and the kinetic energy."""
    omega = np.sqrt(i["k"] / i["m"])
    grid = np.linspace(0.0, np.pi / omega, 4001)
    solution = integrate.solve_ivp(lambda t, y: [y[1], -(i["k"] / i["m"]) * y[0]], (0.0, grid[-1]), [0.0, i["v"]], t_eval=grid, rtol=1e-11, atol=1e-13)
    return {"F": float(i["k"] * np.max(solution.y[0])), "E": 0.5 * i["m"] * i["v"] ** 2}


def gas_explosion_pressure(i):
    """🏠 EN 1991-1-7 Annex D design pressure p_d (kN/m2) of the formulas D.1 and D.2 (registry constants, unverified) for the vent ratio A_v/V; the larger governs."""
    ratio = i["a_v"] / i["V"]
    d1 = i["base"] + i["p_v"]
    d2 = i["base"] + i["p_v"] / 2.0 + i["coefficient"] / ratio**2
    return {"p_d1": d1, "p_d2": d2, "p_d": float(np.maximum(d1, d2)), "ratio": ratio}


def bridge_temperature_actions(i):
    """🌡️ EN 1991-1-5 6.1 bridge temperature actions: the uniform ranges from the shade temperatures and the deck shifts, the curvature as the slope of the strain alpha T(z) fitted to the linear profile (`polyfit`), the end rotation from the bending boundary value problem w'' = kappa, w(0) = w(L) = 0 (`solve_bvp`), the movement alpha dT L and the governing combination."""
    dt_con = i["t_0"] - (i["t_min"] + i["shift_min"])
    dt_exp = i["t_max"] + i["shift_max"] - i["t_0"]
    m = max(i["dtm_heat"] * i["ksur_heat"], i["dtm_cool"] * i["ksur_cool"])
    z = np.linspace(0.0, i["h"], 11)
    kappa = float(np.polyfit(z, i["alpha"] * m * (z / i["h"] - 0.5), 1)[0])
    mesh = np.linspace(0.0, i["L"], 41)
    bvp = integrate.solve_bvp(lambda x, y: np.vstack([y[1], np.full_like(x, kappa)]), lambda ya, yb: np.array([ya[0], yb[0]]), mesh, np.zeros((2, mesh.size)), tol=1e-10)
    n = max(dt_con, dt_exp)
    return {"dt_con": dt_con, "dt_exp": dt_exp, "kappa": kappa, "theta": float(abs(bvp.sol(0.0)[1])), "movement": i["alpha"] * (dt_con + dt_exp) * i["L"], "combined": max(n + i["omega_m"] * m, i["omega_n"] * n + m)}


def special_vehicle_footprint(i):
    """🚛 EN 1991-2 Annex A special vehicle designation total/axle in kN: the number of axle lines as the integer quotient, the length from the line spacing and the weight check n times the axle line."""
    total, axle = (float(part) for part in i["designation"].split("/"))
    lines = int(round(total / axle))
    return {"total": total * 1000.0, "axle": axle * 1000.0, "lines": float(lines), "length": (lines - 1) * i["spacing"], "sum": lines * axle * 1000.0}


def roof_step_drift(i):
    """❄️ EN 1991-1-3 5.3.6 roof step: mu_w = (b_1 + b_2) / (2 h) limited by gamma h / s_k and then to [0.8, cap]; mu_s the registry share of mu_1 of the upper pitch above 15 degrees; l_s = 2 h within [5, 15] m."""
    mu_w = float(np.clip(min((i["b1"] + i["b2"]) / (2.0 * i["h"]), i["gamma"] * i["h"] / i["s_k"]), 0.8, i["cap"]))
    mu_1 = float(interpolate.interp1d((0.0, 30.0, 60.0, 90.0), (0.8, 0.8, 0.0, 0.0))(i["pitch_upper"]))
    return {"mu_w": mu_w, "mu_s": i["share"] * mu_1 if i["pitch_upper"] > 15.0 else 0.0, "l_s": float(np.clip(2.0 * i["h"], 5.0, 15.0))}


def tandem_travel_envelope(i):
    """🌉 EN 1991-2 4.3.2 tandem travel over a simple span: the mid-span moment and left reaction of two axles one spacing apart from piecewise-linear influence lines (`interp1d`, a step at the support for the reaction); the sum is piecewise linear in the front axle position, so its maximum is the largest value at the breakpoints."""
    span, spacing, axle = i["span"], i["spacing"], i["axle"]
    eta_m = interpolate.interp1d((-1.0e3, 0.0, span / 2.0, span, span + 1.0e3), (0.0, 0.0, span / 4.0, 0.0, 0.0))
    eta_r = interpolate.interp1d((-1.0e3, -1.0e-12, 0.0, span, span + 1.0e-12, span + 1.0e3), (0.0, 0.0, 1.0, 0.0, 0.0, 0.0))
    cuts = sorted({0.0, spacing, span / 2.0, span / 2.0 + spacing, span, span + spacing})
    return {"m_env": max(axle * (float(eta_m(x)) + float(eta_m(x - spacing))) for x in cuts), "r_env": max(axle * (float(eta_r(x)) + float(eta_r(x - spacing))) for x in cuts)}


def snow_profile_mu(i):
    """❄️ EN 1991-1-3 5.3.6 drift triangle along a roof section: mu rises or falls linearly between the base value and the peak over the drift length (`np.interp`), the base elsewhere."""
    start, length, base, peak, x = i["start"], i["length"], i["base"], i["peak"], i["x"]
    knots = (start, start + length)
    values = (base, max(peak, base)) if i["rising"] > 0.5 else (max(peak, base), base)
    inside = knots[0] - 1e-9 <= x <= knots[1] + 1e-9
    return {"mu": float(np.interp(x, knots, values)) if inside else base}


def wind_net_pressure(i):
    """🌬️ EN 1991-1-4 7.2.9 net pressure of a zone: w = q_p (c_pe - c_pi) c_s c_d by a `polyval` of the coefficient polynomial in (c_pe - c_pi)."""
    return {"w": float(np.polyval([i["qp"] * i["factor"], 0.0], i["cpe"] - i["cpi"]))}

CHECKS |= {
    "en1991.8.bridge-wind-force": bridge_wind_force,
    "en1991.2.tandem-travel-envelope": tandem_travel_envelope,
    "en1991.snow.profile-mu": snow_profile_mu,
    "en1991.wind.net-pressure": wind_net_pressure,
    "en1991.6.structural-factor": structural_factor_annex_b,
    "en1991.6.vortex-shedding": vortex_shedding_onset,
    "en1991.7.impact-force": impact_spring_peak,
    "en1991.7.gas-explosion": gas_explosion_pressure,
    "en1991.5.bridge-temperature": bridge_temperature_actions,
    "en1991.2.special-vehicle": special_vehicle_footprint,
    "en1991.snow.roof-step-drift": roof_step_drift,
    "en1991.2.footbridge-load": footbridge_loads,
    "en1991.2.fatigue-load-model": fatigue_load_models,
    "en1991.2.support-collision": support_collision_forces,
    "en1991.1-2.local-fire-hasemi": local_fire_hasemi,
    "en1991.1-3.snow-guard": snow_guard_force,
    "en1991.2.rail-dynamic": rail_dynamic_clip,
    "en1991.2.rail-derailment-load": rail_derailment_load,
    "en1991.2.rail-group-bases": rail_group_bases,
    "en1991.4.dust-vent-area": dust_vent_area,
    "en1991.3.hoist-release": hoist_release_factor,
    "en1991.4.bulk-k-mean": bulk_k_mean,
    "en1991.3.machine-force": machine_unbalance_force,
    "en1991.2.dispersal-footprint": dispersal_footprint,
}
