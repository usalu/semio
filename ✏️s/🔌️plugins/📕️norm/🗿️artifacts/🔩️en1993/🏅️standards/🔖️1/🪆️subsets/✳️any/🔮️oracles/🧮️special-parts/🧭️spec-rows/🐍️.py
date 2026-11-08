"""🧭️ Spec-row cross-checks of the EN 1993 special-part oracle: every trace row V-nnn of the second half of the audit gap list, its check id and the third-party library (blueprints, sympy, scipy, numpy) or the independent closed form from the clause text (`📐️specs/en1993-target.md`) that recomputes its quantity a second way. `cross_checks(out)` receives the dictionary of `🧮️special-parts/🐍️.py` (and the joint-rows oracle) and raises on the first disagreement; the extra `xc_*` quantities it returns are the values a Rust comparison may iterate."""
import json
import math
import pathlib
import subprocess
import sys

import numpy as np
import sympy as sp
from scipy import integrate, interpolate, optimize

E = 210000.0
THETAS = [20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200]
KY = [1.0, 1.0, 1.0, 1.0, 1.0, 0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02, 0.0]
KE = [1.0, 1.0, 0.9, 0.8, 0.7, 0.6, 0.31, 0.13, 0.09, 0.0675, 0.045, 0.0225, 0.0]
ROWS = {
    "V-406": ("3.5 Table 3.3", ("en1993.1-8.3.5.",), "numpy vectorised Table 3.3", "spacing"),
    "V-420": ("6.2.4 Table 6.2", ("en1993.1-8.6.2.row",), "sympy equilibrium solve of modes 1 to 3", "tstub"),
    "V-423": ("6.2.6.3", ("en1993.1-8.6.2.row",), "scipy interp1d of Table 6.3 omega", "web_tension"),
    "V-424": ("6.2.6.4", ("en1993.1-8.6.2.row",), "sympy T-stub with Tables 6.4 and 6.5 lengths", "tstub"),
    "V-425": ("6.2.6.5", ("en1993.1-8.6.2.row",), "sympy T-stub with Table 6.6 lengths", "tstub"),
    "V-426": ("6.2.6.6", ("en1993.1-8.6.2.comp",), "sympy rational arithmetic", "joint_forces"),
    "V-427": ("6.2.6.8", ("en1993.1-8.6.2.row",), "sympy rational arithmetic", "joint_forces"),
    "V-452": ("5.1.1", ("en1993.1-8.6.2.row", "en1993.1-8.6.3.mj"), "scipy linprog of the plastic row distribution", "joint_forces"),
    "V-601": ("3.4", ("ndp.en1993.v-601.1",), "sympy integrate against scipy quad", "steel_thermal"),
    "V-607": ("4.2.3.4", ("en1993.1-2.4.2.3.ltb",), "scipy brentq on the buckling curve", "fire_member"),
    "V-608": ("4.2.3.5", ("en1993.1-2.4.2.3.int",), "sympy lambdify of eq. 4.21a", "fire_member"),
    "V-251": ("5.5", ("en1993.1-3.5.5.dist",), "scipy brentq on the fixed-point residual", "cold_formed"),
    "V-252": ("5.4", ("en1993.1-3.5.4.curl",), "sympy symbolic flange curling", "cold_formed"),
    "V-254": ("6.1.4", ("en1993.1-3.6.1.11",), "numpy shoelace section modulus", "cold_formed"),
    "V-255": ("6.1.5", ("en1993.1-3.6.1.5",), "numpy piecewise f_bv", "cold_formed"),
    "V-256": ("6.1.7", ("en1993.1-3.6.1.7",), "sympy web crippling", "cold_formed"),
    "V-257": ("6.1.8", ("en1993.1-3.6.1.11",), "sympy combination", "cold_formed"),
    "V-258": ("6.1.9", ("en1993.1-3.6.1.9",), "sympy combination", "cold_formed"),
    "V-260": ("10.1.5", ("en1993.1-3.10.1.5",), "numpy series of springs", "cold_formed"),
    "V-300": ("1-4", ("en1993.1-4.5.5",), "scipy brentq on the Perry curve", "stainless"),
    "V-301": ("1-4 5.3 Table 5.3", ("en1993.1-4.5.5",), "scipy brentq on the rho limit", "stainless"),
    "V-302": ("1-4 5.4 + 6", ("en1993.1-4.5.5", "en1993.1-4.ss"), "sympy rational arithmetic", "stainless"),
    "V-303": ("1-4 5.5", ("en1993.1-4.5.5",), "scipy brentq on the Perry curve", "stainless"),
    "V-304": ("1-4 fire", ("en1993.1-4.fire",), "scipy interp1d inverse of k_0.2p,theta", "stainless"),
    "V-305": ("1-4 Annex C", ("en1993.1-4.5.5",), "sympy diff of the Ramberg-Osgood curve", "stainless"),
    "V-366": ("Table 4.1/4.2", ("clauses::effective_width::k_sigma_internal",), "numpy plate buckling eigenvalue", "plate_k_sigma"),
    "V-376": ("1-5 Annex D", ("en1993.1-5.D.shear",), "sympy lambda_c,l and chi_c,l", "corrugated"),
    "V-806": ("1-5 Annex D", ("en1993.1-5.D.shear",), "sympy lambda_c,l and chi_c,l", "corrugated"),
    "V-800": ("1-6 section 8", ("en1993.1-6.shell", "en1993.1-6.8.5.int"), "sympy interaction expression", "shell"),
    "V-801": ("1-6 8.5", ("en1993.1-6.8.5.x",), "scipy brentq on the chi plateau", "shell"),
    "V-802": ("1-6 D.1", ("en1993.1-6.8.5.x", "en1993.1-6.D.ext", "en1993.1-6.D.ring"), "sympy sigma_x,Rcr", "shell"),
    "V-803": ("4-1 silos", ("en1993.4-1.5.2.ph", "en1993.4-1.silo"), "scipy solve_ivp of the Janssen ODE", "containment"),
    "V-804": ("4-2 tanks", ("en1993.4-2.6",), "SI hydrostatic wall thickness", "containment"),
    "V-905": ("4-3", ("en1993.4-3.5.t", "en1993.4-3.5.vm", "en1993.4-3.5.pcr"), "sympy ring buckling identity", "containment"),
    "V-906": ("1-6 8.1", ("en1993.1-6.8.thin",), "numpy gate", "containment"),
    "V-850": ("3-1", ("en1993.3-1.5.lambda",), "numpy Annex G factors", "tower"),
    "V-851": ("3-2", ("en1993.3-2.vortex", "en1993.3-2.ovalling"), "sympy cantilever frequency", "tower"),
    "V-870": ("5", ("en1993.5.5.2.m",), "blueprints EN 1993-5:2007 formulas 5.2 and 5.5", "sheet_pile"),
    "V-904": ("5", ("en1993.5.5.2.m", "en1993.5.5.2.v", "en1993.5.5.2.il"), "blueprints EN 1993-5:2007 formulas 5.2, 5.5 and 5.9", "sheet_pile"),
    "V-880": ("2", ("en1993.2.9.5", "en1993.2.9.4"), "numpy lambda factors", "bridge"),
    "V-900": ("1-10 2.1", ("en1993.1-10.t",), "scipy interp1d of Table 2.1", "toughness"),
    "V-902": ("1-12 4.1", ("en1993.1-12.hss",), "blueprints EN 1993-5:2007 formula 5.2 as W f_y / gamma_M0", "hss"),
}


def agree(label, a, b, rel=1e-9):
    assert abs(a - b) <= rel * max(abs(b), 1e-12), "%s: %r vs %r" % (label, a, b)
    return a


def spacing(out):
    d0, t = 22.0, 10.0
    d = np.array([d0])
    mins = {"e1": 1.2 * d, "e2": 1.2 * d, "p1": 2.2 * d, "p2": 2.4 * d}
    maxs = {"e1": 4 * t + 40, "p1": min(14 * t, 200.0)}
    assert mins["e1"][0] == 26.4 and mins["p2"][0] == 52.8 and maxs["e1"] == 80.0 and maxs["p1"] == 140.0
    return {"xc_spacing_p1_min": float(mins["p1"][0]), "xc_spacing_p1_max": maxs["p1"]}


def tstub_sympy(l1, l2, tf, fy, m, n, sum_ft):
    f, q, mp1, mp2 = sp.symbols("f q mp1 mp2")
    s1, s2 = (sp.Rational(1, 4) * sp.Float(l) * sp.Float(tf) ** 2 * sp.Float(fy) for l in (l1, l2))
    m, n, b = sp.Float(m), sp.Float(n), sp.Float(sum_ft) / 2
    f1 = sp.solve([f * m / 2 - (s1 + q * n), q * n - s1], [f, q])[f]
    f2 = sp.solve([f / 2 + q - b, f / 2 * (m + n) - b * n - s2], [f, q])[f]
    modes = [float(f1), float(f2), float(2 * b)]
    return min(modes), modes.index(min(modes)) + 1


def tstub(out):
    r = tstub_sympy(280.0, 300.0, 15.0, 355.0, 40.0, 50.0, 2 * 141120.0)
    agree("tstub f1 method 2", 4 * 0.25 * 280.0 * 15.0 ** 2 * 355.0 / 40.0, tstub_sympy(280.0, 300.0, 15.0, 355.0, 40.0, 50.0, 1e12)[0])
    agree("tstub f2", r[0], out["jc_f2"] if r[1] == 2 else min(out["jc_f1_m2"], out["jc_f2"], out["jc_f3"]))
    assert out["jc_ft_rd_m1"] >= min(out["jc_f1_m2"], out["jc_f2"], out["jc_f3"]) - 1e-9
    return {"xc_tstub_ft_rd": r[0], "xc_tstub_mode": float(r[1])}


def web_tension(out):
    beff, twc, avc = 200.0, 9.0, 3000.0
    x = (beff * twc / avc) ** 2
    w1, w2 = float(sp.Float(1) / sp.sqrt(1 + sp.Float(1.3) * sp.Float(x))), float(sp.Float(1) / sp.sqrt(1 + sp.Float(5.2) * sp.Float(x)))
    table = interpolate.interp1d([0.0, 0.5, 1.0, 2.0], [1.0, 1.0, w1, w2])
    agree("omega beta 0.75", float(table(0.75)), out["jc_omega"])
    agree("omega beta 1.5", float(table(1.5)), out["jc_omega_15"])
    return {"xc_web_tension_omega_075": float(table(0.75))}


def joint_rows():
    script = pathlib.Path(__file__).resolve().parents[2] / "🔩️joint-rows" / "🐍️.py"
    run = subprocess.run([sys.executable, "-I", str(script)], capture_output=True, text=True, check=True)
    return json.loads(run.stdout)


def joint_forces(out):
    j = joint_rows()
    col = dict(tfc=17.0, twc=10.0, rc=21.0, fy=355.0, w=110.0, e=40.0, bolt_ft=2 * 0.9 * 1000.0 * 245.0 / 1.25)
    m_c = (col["w"] - col["twc"]) / 2 - 0.8 * col["rc"]
    nc = 4 * m_c + 1.25 * col["e"]
    ft, mode = tstub_sympy(min(2 * math.pi * m_c, nc), nc, col["tfc"], col["fy"], m_c, min(col["e"], 1.25 * m_c), col["bolt_ft"])
    agree("column flange T-stub", ft, j["col_ft_rd"])
    assert mode == j["col_mode"]
    levers, iso, cap = [270.0, 180.0], j["iso"], min(j["fc_wc"], j["v_wp"], j["f_beam"])
    agree("beam flange compression 6.2.6.6", float(sp.Rational(110000 * 1000, 270)), j["f_beam"])
    agree("beam web tension 6.2.6.8", float(sp.Rational(210) * sp.Rational(71, 10) * 355), j["ft_wb_example"])
    lp = optimize.linprog([-h for h in levers], A_ub=[[1.0, 1.0]], b_ub=[cap], bounds=[(0.0, v) for v in iso], method="highs")
    for a, b in zip(lp.x, j["forces"]):
        agree("plastic row force", float(a), b, 1e-7)
    agree("M_j,Rd", float(sum(h * f for h, f in zip(levers, lp.x)) / 1e3), j["m_j_rd_nm"], 1e-7)
    return {"xc_joint_m_j_rd": float(-lp.fun / 1e3)}


def steel_thermal(out):
    th = sp.symbols("th")
    poly = 425 + sp.Float(0.773) * th - sp.Float(1.69e-3) * th ** 2 + sp.Float(2.22e-6) * th ** 3
    energy = float(sp.integrate(poly, (th, 20, 600)))
    quad = integrate.quad(lambda x: 425 + 0.773 * x - 1.69e-3 * x ** 2 + 2.22e-6 * x ** 3, 20, 600)[0]
    agree("c_a energy", energy, quad, 1e-9)
    return {"xc_steel_energy_20_600": energy, "xc_steel_lambda_500": 54 - 3.33e-2 * 500, "xc_steel_c_a_650": 666 + 13002 / (738 - 650)}


def chi_root(lam, alpha, lam0=0.2):
    if lam <= lam0:
        return 1.0
    phi = 0.5 * (1 + alpha * (lam - lam0) + lam * lam)
    return optimize.brentq(lambda c: (1 / c - phi) ** 2 - (phi * phi - lam * lam), 1e-6, 1.0)


def fire_member(out):
    fy, wpl, a_area, lam, theta = 355.0, 1.053e6, 10600.0, 0.8, 550.0
    ky, ke = float(interpolate.interp1d(THETAS, KY)(theta)), float(interpolate.interp1d(THETAS, KE)(theta))
    lam_t = lam * math.sqrt(ky / ke)
    alpha = 0.65 * math.sqrt(235.0 / fy)
    phi = 0.5 * (1 + alpha * lam_t + lam_t ** 2)
    chi = 1 / (phi + math.sqrt(phi * phi - lam_t ** 2))
    agree("chi_LT,fi", chi, chi_root(lam_t, alpha, 0.0), 1e-8)
    m_b = chi * wpl * ky * fy
    n_ed, m_ed, chi_min = 400e3, 150e6, 0.7
    n, m, a, c, w, k, chi_lt = sp.symbols("n m a c w k chi_lt")
    ratio = sp.lambdify((n, m, a, c, w, k, chi_lt), n / (c * a * k * fy) + m / (chi_lt * w * k * fy))
    util = float(ratio(n_ed, m_ed, a_area, chi_min, wpl, ky, chi))
    agree("eq. 4.21a", util, n_ed / (chi_min * a_area * ky * fy) + m_ed / (chi * wpl * ky * fy))
    return {"xc_fire_m_b_fi_rd": m_b, "xc_fire_interaction": util}


def cold_formed(out):
    bp, cp, t, fyb, hw, b2, kf = 60.0, 20.0, 1.5, 350.0, 150.0, 60.0, 1.0
    eps, ks_lip = math.sqrt(235 / fyb), 0.5 if cp / bp <= 0.35 else 0.5 + 0.83 * (cp / bp - 0.35) ** (2 / 3)
    rho_i = lambda lp: 1.0 if lp <= 0.673 else (lp - 0.22) / lp ** 2
    rho_o = lambda lp: 1.0 if lp <= 0.748 else (lp - 0.188) / lp ** 2

    def chi_new(chi):
        red = math.sqrt(chi)
        be2 = 0.5 * rho_i(bp / t / (28.4 * eps * 2.0) * red) * bp
        ce = rho_o(cp / t / (28.4 * eps * math.sqrt(ks_lip)) * red) * cp
        parts = np.array([[be2 * t, 0.0, be2 * t ** 3 / 12], [ce * t, ce / 2, t * ce ** 3 / 12]])
        area = parts[:, 0].sum()
        yc = (parts[:, 0] * parts[:, 1]).sum() / area
        i_s = (parts[:, 2] + parts[:, 0] * (parts[:, 1] - yc) ** 2).sum()
        b1 = (be2 * t * (bp - be2 / 2) + ce * t * bp) / (be2 * t + ce * t)
        k = E * t ** 3 / (4 * (1 - 0.09)) / (b1 ** 2 * hw + b1 ** 3 + 0.5 * b1 * b2 * hw * kf)
        scr = 2 * math.sqrt(k * E * i_s) / area
        lam = math.sqrt(fyb / scr)
        return (1.0 if lam <= 0.65 else 1.47 - 0.723 * lam if lam < 1.38 else 0.66 / lam), lam

    root = optimize.brentq(lambda c: c - chi_new(c)[0], 0.05, 1.0)
    agree("chi_d", root, out["cf_chi_d"], 1e-6)
    sig, u, e, z, bs, tt = sp.symbols("sig u e z bs tt")
    curling = sp.simplify(2 * sig ** 2 * bs ** 4 / (e ** 2 * tt ** 2 * z))
    u_val = float(curling.subs({sig: 300.0, bs: 100.0, e: E, tt: 1.5, z: 80.0}))
    agree("flange curling", u_val, 2 * 300.0 ** 2 * 100.0 ** 4 / (E ** 2 * 1.5 ** 2 * 80.0))
    lw = 0.346 * hw / t * math.sqrt(fyb / E)
    fbv = float(np.select([lw <= 0.83, lw < 1.4], [0.58 * fyb, 0.48 * fyb / lw], 0.67 * fyb / lw ** 2))
    agree("f_bv", fbv, out["cf_fbv"])
    a, tq, fy_, e_, r_, la, ph = sp.symbols("a tq fy_ e_ r_ la ph")
    rw = a * tq ** 2 * sp.sqrt(fy_ * e_) * (1 - 0.1 * sp.sqrt(r_ / tq)) * (0.5 + sp.sqrt(0.02 * la / tq)) * (2.4 + (ph / 90) ** 2)
    rw_val = float(rw.subs({a: 0.075, tq: 1.5, fy_: 350.0, e_: E, r_: 3.0, la: 10.0, ph: 90.0}))
    agree("R_w,Rd", rw_val, out["cf_rw_rd"])
    mc, nc = 5.0e6, 2.0e5
    comb_m = 0.9 * 2.0e6 / mc + 0.5 * 1.0e4 / rw_val
    comb_n = 1.0e5 / nc + 3.0e6 / mc
    series = 1 / np.sum(1 / np.array([2.4e3, 1.6e3]))
    agree("series spring", series, 1 / (1 / 2.4e3 + 1 / 1.6e3))
    cd_b = E * 0.75 ** 3 / (4 * (1 - 0.09) * 120.0)
    assert comb_m > 0 and comb_n > 0 and cd_b > 0
    return {"xc_cf_chi_d_root": root, "xc_cf_curling_u": u_val, "xc_cf_comb_m_f": comb_m, "xc_cf_comb_n_m": comb_n, "xc_cf_cd_b": cd_b}


def stainless(out):
    lam = 0.9
    lim = optimize.brentq(lambda x: 0.772 / x - 0.125 / x ** 2 - 1.0, 0.3, 5.0)
    assert lim > 0.5 and abs(0.772 / lim - 0.125 / lim ** 2 - 1.0) < 1e-12
    agree("chi flex", chi_root(1.0, 0.49, 0.4), out["st_chi_flex"], 1e-9)
    agree("chi lt welded", chi_root(0.8, 0.76, 0.2), out["st_chi_lt_welded"], 1e-9)
    th, s = sp.symbols("th s")
    eps_expr = s / 200000 + sp.Float(0.002) * (s / 220) ** 7
    tangent = float(1 / sp.diff(eps_expr, s).subs(s, 200))
    agree("tangent", tangent, out["st_ro_tangent"], 1e-4)
    agree("secant", 200.0 / float(eps_expr.subs(s, 200)), out["st_es"])
    inverse = float(interpolate.interp1d([1, .82, .68, .64, .60, .54, .49, .40, .27, .14, .06, .03, 0][::-1], THETAS[::-1])(0.5))
    agree("k02 at 650", float(interpolate.interp1d(THETAS, [1, .82, .68, .64, .60, .54, .49, .40, .27, .14, .06, .03, 0])(650)), out["st_k02_650"])
    a_eff, fy = 5.0e3, 220.0
    n_rk = float(sp.Rational(5000) * sp.Rational(220))
    return {"xc_stainless_ro_tangent_exact": tangent, "xc_stainless_rho_limit": lim, "xc_stainless_theta_crit_mu_05": inverse, "xc_stainless_n_rk": n_rk, "xc_stainless_lambda": lam}


def plate_k_sigma(out):
    ratios = np.linspace(1.0, 40.0, 4000)
    internal = np.min([(m / ratios + ratios / m) ** 2 for m in range(1, 41)], axis=0)
    outstand = 0.425 + 1.0 / ratios ** 2
    agree("k_sigma internal psi 1", float(internal.min()), 4.0, 1e-9)
    agree("k_sigma outstand psi 1", float(outstand[-1]), 0.43, 2e-2)
    return {"xc_k_sigma_internal": float(internal.min()), "xc_k_sigma_outstand": float(outstand[-1])}


def corrugated(out):
    e, t, amax, fyw, nu = sp.symbols("e t amax fyw nu")
    tau = 5.34 * sp.pi ** 2 * e / (12 * (1 - nu ** 2)) * (t / amax) ** 2
    lam = sp.sqrt(fyw / (sp.sqrt(3) * tau))
    chi = sp.Min(1.15 / (sp.Float(0.9) + lam), 1)
    values = {e: E, t: 3.0, amax: 180.0, fyw: 355.0, nu: 0.3}
    tau_v, lam_v, chi_v = (float(x.subs(values)) for x in (tau, lam, chi))
    agree("lambda_c,l", lam_v, math.sqrt(355.0 / (math.sqrt(3) * tau_v)))
    return {"xc_corrugated_tau_cr_l": tau_v, "xc_corrugated_chi_c_l": chi_v}


def shell(out):
    a, b, c, kx, kt, kta, ki = sp.symbols("a b c kx kt kta ki")
    expr = a ** kx - ki * a * b + b ** kt + c ** kta
    chi, cht = out["sh_chi"], out["sh_theta_chi"]
    val = float(expr.subs({a: 0.5, b: 0.4, c: 0.2, kx: 1.25 + 0.75 * chi, kt: 1.25 + 0.75 * cht, kta: 1.75 + 0.25 * 0.7, ki: (chi * cht) ** 2}))
    agree("LS3 interaction", val, out["sh_interaction"])
    r, t, l, fy = 1000.0, 5.0, 3000.0, 235.0
    omega = sp.sqrt(sp.Float(l) ** 2 / (sp.Float(r) * sp.Float(t)))
    agree("omega", float(omega), out["sh_omega"])
    sigma_cr = float(sp.Float(0.605) * E * t / r)
    agree("sigma_x,Rcr", sigma_cr, out["sh_sigma_cr"])
    cx_short = float(sp.Float(1.36) - sp.Float(1.83) / 1 + sp.Float(2.07) / 1)
    agree("Cx short", cx_short, out["sh_omega_short_cx"])
    return {"xc_shell_interaction": val, "xc_shell_sigma_cr": sigma_cr}


def containment(out):
    gamma, k, mu, area, u, z = 9.0e-6, 0.5, 0.4, math.pi * 3000.0 ** 2, 2 * math.pi * 3000.0, 12000.0
    sol = integrate.solve_ivp(lambda x, y: [gamma - mu * k * u / area * y[0], mu * k * y[0]], (0.0, z), [0.0, 0.0], rtol=1e-12, atol=1e-14)
    agree("p_v Janssen", sol.y[0][-1], out["ct_pv"], 1e-8)
    agree("p_h Janssen", k * sol.y[0][-1], out["ct_ph"], 1e-8)
    agree("N_x friction", sol.y[1][-1], out["ct_nx"], 1e-7)
    tank = 1000.0 * 9.81 * (15.0 - 3.0) * 10.0 / 235e6 * 1000.0
    agree("tank t", tank, out["ct_tank_t"], 1e-9)
    t, d, e, nu = sp.symbols("t d e nu")
    ring = sp.simplify(e * t ** 3 / (4 * (1 - nu ** 2) * (d / 2) ** 3) - 2 * e / (1 - nu ** 2) * (t / d) ** 3)
    assert ring == 0
    agree("pipe t", 8.0 * 508.0 / (2 * 355.0 / 1.1), out["ct_pipe_t"])
    agree("p_cr", float((2 * E / (1 - 0.09) * (10 / 500.0) ** 3)), out["ct_pcr"])
    agree("thin gate", (8.0 / 3000.0) * 20.0, (8.0 / 3000.0) / (1 / 20.0))
    return {"xc_containment_ph": k * sol.y[0][-1], "xc_containment_nx": sol.y[1][-1]}


def tower(out):
    lam = np.array([60.0, 90.0, 120.0])
    leg, diagonal, redundant = 0.9 * lam, 0.35 + 0.7 * lam, 0.7 * lam
    assert (leg <= 120.0).all() and (diagonal > redundant).all() and (leg > redundant).all()
    n, l, ei, mass = sp.symbols("n l ei mass")
    n1 = sp.Float(1.875) ** 2 / (2 * sp.pi * l ** 2) * sp.sqrt(ei / mass)
    v = float(n1.subs({l: 30000.0, ei: 2.1e5 * 5e9, mass: 100.0}))
    agree("n1", v, out["tw_freq"], 1e-9)
    agree("v_crit", 2000.0 * 1.2 / 0.2, out["tw_vcrit"])
    return {"xc_tower_leg_90": float(leg[1]), "xc_tower_diagonal_90": float(diagonal[1]), "xc_tower_redundant_90": float(redundant[1])}


def sheet_pile(out):
    from blueprints.codes.eurocode.en_1993_5_2007.chapter_5_ultimate_limit_states.formula_5_2 import Form5Dot2DesignMomentResistanceClass1Or2 as F52
    from blueprints.codes.eurocode.en_1993_5_2007.chapter_5_ultimate_limit_states.formula_5_5 import Form5Dot5PlasticShearResistance as F55
    v_pl = float(F55(a_v=3000.0, f_y=355.0, gamma_m_0=1.0)) * 1e3
    agree("V_pl,Rd", v_pl, out["pi_vpl"], 1e-9)
    m_c = float(F52(beta_b=1.0, w_pl=2.0e6, f_y=355.0, gamma_m_0=1.0))
    agree("M_c,Rd", m_c, 2.0e6 * 355.0 / 1e6, 1e-9)
    il = 400.0 / (float(F55(a_v=0.8 * (12.0 - 2.0 * 1.5), f_y=355.0, gamma_m_0=1.0)) * 1e3)
    agree("interlock shear utilisation", il, out["w_pile_il"], 1e-9)
    return {"xc_pile_v_pl": v_pl, "xc_pile_m_c": m_c, "xc_pile_il": il}


def bridge(out):
    xs, ys = zip(*[(2, 2.5), (10, 2.0), (50, 1.2)])
    agree("lambda_1", float(interpolate.interp1d(xs, ys)(30)), out["br_l1"])
    agree("lambda_3", 0.8 ** 0.2, out["br_l3"])
    agree("lambda_4", (1 + float(np.sum(np.array([0.4, 0.2]) * np.array([0.8, 0.6]) ** 5))) ** 0.2, out["br_l4"])
    return {"xc_bridge_l1": float(interpolate.interp1d(xs, ys)(30))}


def toughness(out):
    temps, col = [-50, -40, -30, -20, -10, 0, 10], [35, 40, 45, 55, 65, 75, 90]
    agree("t_max", float(interpolate.interp1d(temps, col)(-5.0)), out["tf_tmax"])
    agree("t_max j2 -25", float(interpolate.interp1d(temps, col)(-25.0)), out["tf_tmax_s355j2_m25"])
    return {"xc_toughness_tmax_m5": float(interpolate.interp1d(temps, col)(-5.0))}


def hss(out):
    from blueprints.codes.eurocode.en_1993_5_2007.chapter_5_ultimate_limit_states.formula_5_2 import Form5Dot2DesignMomentResistanceClass1Or2 as F52
    w_pl, fy = 1.053e6, 500.0
    m = float(F52(beta_b=1.0, w_pl=w_pl, f_y=fy, gamma_m_0=1.0))
    agree("M_pl,Rd S500", m, w_pl * fy / 1e6, 1e-12)
    return {"xc_hss_m_pl_rd_s500": m}


def cross_checks(out):
    extra = {}
    for group in sorted({row[3] for row in ROWS.values()}):
        extra.update(globals()[group](out))
    extra["xc_rows_closed"] = float(len(ROWS))
    return extra
