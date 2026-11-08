"""🔮️ Independent oracle for the EN 1993 special-part clause functions (1-3, 1-4, 1-5, 1-6, 1-8 components, 1-10, 1-11, 2, 3, 4, 5, 6): scipy and numpy formulations that differ from the Rust code (fixed-point solver, linear programming for base plates, numerical quadrature for Janssen, interpolation by numpy)."""
import importlib.util
import json
import math
import pathlib

import numpy as np
from scipy import integrate, optimize

E = 210000.0


def chi_ap(lam, alpha, lam0=0.2):
    if lam <= lam0:
        return 1.0
    phi = 0.5 * (1 + alpha * (lam - lam0) + lam * lam)
    return optimize.brentq(lambda c: 1 / c - phi - math.sqrt(phi * phi - lam * lam), 1e-6, 1.0)


def rho_int(lp, psi=1.0, k=None):
    return 1.0 if lp <= 0.5 + math.sqrt(0.085 - 0.055 * psi) else min(1.0, (lp - 0.055 * (3 + psi)) / lp ** 2)


def rho_out(lp):
    return 1.0 if lp <= 0.748 else min(1.0, (lp - 0.188) / lp ** 2)


def cold_formed():
    bp, cp, t, fyb, hw, b2, kf = 60.0, 20.0, 1.5, 350.0, 150.0, 60.0, 1.0
    eps = math.sqrt(235 / fyb)
    ratio = cp / bp
    ks_lip = 0.5 if ratio <= 0.35 else 0.5 + 0.83 * (ratio - 0.35) ** (2 / 3)

    def step(chi):
        sig = chi * fyb
        red = math.sqrt(sig / fyb)
        lam_f = bp / t / (28.4 * eps * 2.0) * red
        lam_l = cp / t / (28.4 * eps * math.sqrt(ks_lip)) * red
        be2 = 0.5 * rho_int(lam_f) * bp
        ce = rho_out(lam_l) * cp
        a = be2 * t + ce * t
        yc = (ce * t * ce / 2) / a
        i = be2 * t ** 3 / 12 + be2 * t * yc ** 2 + t * ce ** 3 / 12 + t * ce * (ce / 2 - yc) ** 2
        b1 = (be2 * t * (bp - be2 / 2) + ce * t * bp) / a
        k = E * t ** 3 / (4 * (1 - 0.09)) / (b1 ** 2 * hw + b1 ** 3 + 0.5 * b1 * b2 * hw * kf)
        scr = 2 * math.sqrt(k * E * i) / a
        lam = math.sqrt(fyb / scr)
        return 1.0 if lam <= 0.65 else (1.47 - 0.723 * lam if lam < 1.38 else 0.66 / lam), lam, scr, a, i

    sol = optimize.fixed_point(lambda c: step(float(c))[0], 1.0, xtol=1e-12, maxiter=500) if step(1.0)[0] < 1.0 else 1.0
    chi, lam, scr, a, i = step(float(sol))
    lw = 0.346 * 150 / 1.5 * math.sqrt(fyb / E)
    fbv = 0.58 * fyb if lw <= 0.83 else (0.48 * fyb / lw if lw < 1.4 else 0.67 * fyb / lw ** 2)
    rw = 0.075 * 1.5 ** 2 * math.sqrt(fyb * E) * (1 - 0.1 * math.sqrt(3.0 / 1.5)) * (0.5 + math.sqrt(0.02 * 10 / 1.5)) * (2.4 + (90 / 90) ** 2) / 1.0
    return {"cf_chi_d": chi, "cf_lambda_d": lam, "cf_sigma_cr_s": scr, "cf_a_s": a, "cf_i_s": i, "cf_fbv": fbv, "cf_vb_rd": 150 / math.sin(math.radians(90)) * 1.5 * fbv, "cf_rw_rd": rw}


def stainless():
    lam = 0.9

    def rho(a, b, x):
        r = optimize.brentq(lambda y: a / y - b / y ** 2 - 1, 2 * b / a, 5)
        return 1.0 if x <= r else min(1.0, a / x - b / x ** 2)

    sigma, fy, n, e = 200.0, 220.0, 7.0, 200000.0
    eps = sigma / e + 0.002 * (sigma / fy) ** n
    theta = [20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200]
    k02 = [1, .82, .68, .64, .60, .54, .49, .40, .27, .14, .06, .03, 0]
    ke = [1, .96, .92, .88, .84, .80, .76, .71, .63, .45, .20, .10, 0]
    return {
        "st_rho_int_cold": rho(0.772, 0.125, lam),
        "st_rho_int_weld": rho(0.772, 0.079, lam),
        "st_rho_out_cold": rho(1.0, 0.231, lam),
        "st_rho_out_weld": rho(1.0, 0.242, lam),
        "st_es": sigma / eps,
        "st_ro_strain": eps,
        "st_ro_tangent": 1.0 / float(np.gradient(np.array([ (x / e + 0.002 * (x / fy) ** n) for x in (sigma - 0.5, sigma, sigma + 0.5)]), 0.5)[1]),
        "st_k02_650": float(np.interp(650, theta, k02)),
        "st_ke_650": float(np.interp(650, theta, ke)),
        "st_chi_flex": chi_ap(1.0, 0.49, 0.4),
        "st_chi_lt_welded": chi_ap(0.8, 0.76, 0.2),
        "st_k_int": min(max(1 + 2 * (0.9 - 0.5) * 0.3, 1.2), 1.2 + 2 * 0.3),
    }


def plated():
    sig_e = math.pi ** 2 * E / (12 * (1 - 0.3 ** 2)) * (10 / 500) ** 2
    rho, chi_c, scp, scc = 0.8, 0.6, 300.0, 200.0
    xi = min(max(scp / scc - 1, 0), 1)
    rho_c = (rho - chi_c) * xi * (2 - xi) + chi_c
    ans = {"pl_sigma_e": sig_e, "pl_rho_c": rho_c}
    kappa = 0.4
    ans["pl_beta_hog"] = 1 / (1 + 6.0 * (kappa - 1 / (2500 * kappa)) + 1.6 * kappa ** 2)
    ans["pl_beta_hog_hi"] = 1 / (8.6 * 1.2)
    hw, tw, a, tf, bf, fyw, fyf, ss, c = 1000.0, 8.0, 2000.0, 20.0, 300.0, 355.0, 355.0, 100.0, 150.0
    m1 = fyf * bf / (fyw * tw)

    def patch(kf, ly_fn):
        fcr = 0.9 * kf * E * tw ** 3 / hw
        m2 = lambda ly: 0.02 * (hw / tf) ** 2 if math.sqrt(ly * tw * fyw / fcr) > 0.5 else 0.0
        ly = ly_fn(m2)
        lam = math.sqrt(ly * tw * fyw / fcr)
        return min(1.0, 0.5 / lam) * ly * tw * fyw / 1.0

    kfa = 6 + 2 * (hw / a) ** 2
    ans["pl_patch_a"] = patch(kfa, lambda m2: min(a, ss + 2 * tf * (1 + math.sqrt(m1 + m2(ss + 2 * tf * (1 + math.sqrt(m1)))))))
    kfb = 3.5 + 2 * (hw / a) ** 2
    le = min(kfb * E * tw ** 2 / (2 * fyw * hw), ss + c)
    ans["pl_patch_b"] = patch(kfb, lambda m2: min(le + tf * math.sqrt(m1 / 2 + (le / tf) ** 2 + m2(le + tf * math.sqrt(m1 / 2 + (le / tf) ** 2))), le + tf * math.sqrt(m1 + m2(le + tf * math.sqrt(m1 / 2 + (le / tf) ** 2)))))
    kfc = min(2 + 6 * (ss + c) / hw, 6.0)
    lec = min(kfc * E * tw ** 2 / (2 * fyw * hw), ss + c)
    ans["pl_patch_c"] = patch(kfc, lambda m2: min(le_c_a(lec, tf, m1, m2), le_c_b(lec, tf, m1, m2), ss + c))
    isl, t = 2.0e6, 8.0
    base = 5.34 + 4 * (hw / a) ** 2 if a / hw >= 1 else 4 + 5.34 * (hw / a) ** 2
    ans["pl_k_tau"] = base + max(9 * (hw / a) ** 2 * (isl / (t ** 3 * hw)) ** 0.75, 2.1 / t * (isl / hw) ** (1 / 3))
    ans["pl_k_tau_low"] = 4.0 + 5.34 * 2.0 ** 2 + max(9 * 2.0 ** 2 * (2.0e6 / (8.0 ** 3 * 1000.0)) ** 0.75, 2.1 / 8.0 * (2.0e6 / 1000.0) ** (1 / 3))
    ans["pl_k_tau_floor"] = 5.34 + 4.0 * 0.5 ** 2 + max(9 * 0.5 ** 2 * (1.0e3 / (8.0 ** 3 * 1000.0)) ** 0.75, 2.1 / 8.0 * (1.0e3 / 1000.0) ** (1 / 3))
    eta1, eta3, mf, mpl = 0.8, 0.8, 100.0, 400.0
    ans["pl_mv"] = 0.8 + (1 - mf / mpl) * (2 * 0.8 - 1) ** 2
    ans["pl_ist"] = 0.75 * 1000 * 8 ** 3
    ans["pl_alpha_ult"] = 1 / math.sqrt((100 / 355) ** 2 + (50 / 355) ** 2 - (100 / 355) * (50 / 355) + 3 * (30 / 355) ** 2)
    return ans


def le_c_a(le, tf, m1, m2):
    return le + tf * math.sqrt(m1 / 2 + (le / tf) ** 2 + m2(le + tf * math.sqrt(m1 / 2 + (le / tf) ** 2)))


def le_c_b(le, tf, m1, m2):
    return le + tf * math.sqrt(m1 + m2(le + tf * math.sqrt(m1 / 2 + (le / tf) ** 2)))


def shells():
    r, t, fy, l = 1000.0, 5.0, 235.0, 3000.0
    om = l / math.sqrt(r * t)
    cx = 1.0 if 1.7 <= om <= 0.5 * r / t else None
    scr = 0.605 * E * cx * t / r
    lam = math.sqrt(fy / scr)
    q = 25.0
    dw = math.sqrt(r / t) * t / q
    alpha = 0.62 / (1 + 1.91 * (dw / t) ** 1.44) ** 0.6
    lp = math.sqrt(alpha / 0.4)
    chi = 1.0 if lam <= 0.2 else (1 - 0.6 * (lam - 0.2) / (lp - 0.2) if lam < lp else alpha / lam ** 2)
    ans = {"sh_omega": om, "sh_sigma_cr": scr, "sh_lambda": lam, "sh_alpha": alpha, "sh_chi": chi, "sh_sigma_rd": chi * fy / 1.1}
    sth = 0.92 * E * 1.5 / om * t / r
    lth = math.sqrt(fy / sth)
    a_t = 0.65
    lpt = math.sqrt(a_t / 0.4)
    ct = 1.0 if lth <= 0.4 else (1 - 0.6 * (lth - 0.4) / (lpt - 0.4) if lth < lpt else a_t / lth ** 2)
    ans.update({"sh_theta_cr": sth, "sh_theta_chi": ct})
    kx, kt, kta = 1.25 + 0.75 * chi, 1.25 + 0.75 * ct, 1.75 + 0.25 * 0.7
    ki = (chi * ct) ** 2
    a, b, c = 0.5, 0.4, 0.2
    ans["sh_interaction"] = a ** kx - ki * a * b + b ** kt + c ** kta
    ans["sh_omega_short_cx"] = 1.36 - 1.83 / 1.0 + 2.07 / 1.0
    return ans


def toughness():
    temps = [10, 0, -10, -20, -30, -40, -50]
    col = [90, 75, 65, 55, 45, 40, 35]
    t_ed = 10 + (-3 * 100 * 20 / (2 * 100))
    cells = table_21()
    k2 = [cells["s355-k2.r75.t%d" % t] for t in temps]
    unset = sum(1 for v in cells.values() if v is None)
    r50 = [np.interp(0.6, [0.5, 0.75], [a, b]) for a, b in zip([80, 100, 120], [100, 120, 140])]
    return {
        "tf_t_ed": t_ed,
        "tf_tmax": float(np.interp(-5.0, temps[::-1], col[::-1])),
        "tf_tmax_s355j2_m25": float(np.interp(-25.0, temps[::-1], col[::-1])),
        "tf_tmax_s355k2_m15": float(np.interp(-15.0, temps[::-1], k2[::-1])),
        "tf_tmax_s355k2_r50_m10_fallback": float(cells["s355-k2.r75.t-10"]),
        "tf_cells_total": float(len(cells)),
        "tf_cells_unset": float(unset),
        "tf_blend_r60": float(np.interp(0.6, [0.5, 0.75], [100.0, 120.0])),
        "tf_blend_below_r25_conservative": float(np.interp(0.1, [0.25, 0.5], [80.0, 100.0])),
    }


def table_21():
    import pathlib
    here = pathlib.Path(__file__).resolve()
    root = next(p for p in here.parents if p.name == "📕️norm")
    entries = json.load(open(root / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json", encoding="utf-8"))["entries"]
    prefix = "ndp.en1993.1-10.t21."
    return {e["id"][len(prefix):]: e["recommended"] for e in entries if e["id"].startswith(prefix)}


def tension():
    return {
        "tc_ernst": E / (1 + (7.85e-5 * 100e3) ** 2 * E / (12 * 400.0 ** 3)),
        "tc_sag": 0.5 * 50000.0 ** 2 / (8 * 250e3),
        "tc_pin_m": 120e3 / 8 * (30 + 4 * 5 + 2 * 10),
        "tc_pin_comb": (40e6 / 80e6) ** 2 + (60e3 / 100e3) ** 2,
        "tc_rope": 0.6 * 20 ** 2 * 1770,
    }


def joints():
    tf, fy, m, n, ew, ft = 15.0, 355.0, 40.0, 50.0, 8.0, 2 * 141120.0
    l1, l2 = 280.0, 300.0
    mp1, mp2 = 0.25 * l1 * tf ** 2 * fy, 0.25 * l2 * tf ** 2 * fy
    lbs = 8.8 * m ** 3 * 245 * 2 / (l1 * tf ** 3)
    f1a = (8 * n - 2 * ew) * mp1 / (2 * m * n - ew * (m + n))
    f1b = 4 * mp1 / m
    f2 = (2 * mp2 + n * ft) / (m + n)
    ans = {"jc_lb_star": lbs, "jc_f1_m1": f1a, "jc_f1_m2": f1b, "jc_f2": f2, "jc_f3": ft, "jc_ft_rd_m1": min(f1a, f2, ft), "jc_ft_noprying": min(2 * mp1 / m, ft), "jc_prying_at_100": 1.0 if 100.0 <= lbs else 0.0, "jc_prying_at_300": 1.0 if 300.0 <= lbs else 0.0}
    z = [300.0, 220.0]
    h = [300.0, 220.0]
    k = [12.0, 8.0]
    ans["jc_z_eq"] = sum(a * b * b for a, b in zip(k, h)) / sum(a * b for a, b in zip(k, h))
    ans["jc_k_eq"] = sum(a * b for a, b in zip(k, h)) / ans["jc_z_eq"]
    beff, twc, avc = 200.0, 9.0, 3000.0
    x = (beff * twc / avc) ** 2
    w1, w2 = (1 + 1.3 * x) ** -0.5, (1 + 5.2 * x) ** -0.5
    ans["jc_omega"] = float(np.interp(0.75, [0.5, 1.0], [1.0, w1]))
    ans["jc_omega_15"] = float(np.interp(1.5, [1.0, 2.0], [w1, w2]))
    iso, fcap = [300e3, 250e3, 200e3], 650e3
    grp = {(0, 1): 420e3, (0, 2): 560e3, (1, 2): 380e3}
    f = []
    for xx, v in enumerate(iso):
        cand = [v, fcap - sum(f)]
        for r in range(xx):
            cand.append(grp[(r, xx)] - sum(f[r:xx]))
        f.append(max(0.0, min(cand)))
    ans["jc_rows"] = f
    ans["jc_mj"] = sum(a * b for a, b in zip([300.0, 220.0, 140.0], f))
    ans["jc_mu"] = (1.5 * 0.9) ** 2.7
    ans["jc_sj_ini"] = E * 300.0 ** 2 / (1 / 4000 + 1 / 9000 + 1 / 6000)
    msat = 0.6e6
    ftl, fcl, ftr, fcr = 150e3, 400e3, 150e3, 400e3
    for tag, ned, zl, zr in (("a", 100e3, 300.0, 300.0), ("b", -200e3, 300.0, 300.0), ("c", 100e3, 200.0, 400.0), ("d", -300e3, 200.0, 400.0)):
        res = optimize.linprog([-zl, zr], A_eq=[[1.0, 1.0]], b_eq=[ned], bounds=[(-fcl, ftl), (-fcr, ftr)])
        ans["jc_bp_" + tag] = -res.fun
    f_rdu = min(300 * 300 * 20 * math.sqrt(1.5), 3 * 20 * 300 * 300)
    ans["jc_fjd"] = 2 / 3 * f_rdu / (150 * 250)
    tfc, bfc, tp, fy, fcd = 17.0, 240.0, 30.0, 355.0, 20.0
    f_jd = 2 / 3 * 3 * fcd
    c = tp * math.sqrt(fy / (3 * f_jd))
    for tag, bp in (("free", 450.0), ("plate", 300.0)):
        b_eff, l_eff = tfc + 2 * c, min(bfc + 2 * c, bp)
        ans["jc_cs_leff_" + tag] = l_eff
        ans["jc_cs_fc_" + tag] = f_jd * b_eff * l_eff
    area = 350e3 / f_jd
    for tag, bp in (("free", 450.0), ("plate", 250.0)):
        ans["jc_cr_" + tag] = optimize.brentq(lambda x: (tfc + 2 * x) * min(bfc + 2 * x, bp) - area, 0.0, 1e4)
    return ans


def bridges():
    table = [(2, 2.5), (10, 2.0), (50, 1.2)]
    xs, ys = zip(*table)
    return {
        "br_l2": 1.0 / 0.5 * (1.2e6 / 0.5e6) ** 0.2 * 0.5,
        "br_l3": 0.8 ** 0.2,
        "br_l4": (1 + 0.4 * (0.8) ** 5 + 0.2 * (0.6) ** 5) ** 0.2,
        "br_l1": float(np.interp(30, xs, ys)),
        "br_ds": min(2.0, 1.1 * 1.0 * 0.9 * 1.05) * 1.0 * 40.0,
        "br_cat": 100.0,
        "br_ks": (25 / 40) ** 0.2,
    }


def towers():
    return {
        "tw_vcrit": 2000.0 * 1.2 / 0.2,
        "tw_sc": 2 * 0.05 * 80.0 / (1.25 * 2.0 ** 2),
        "tw_freq": 1.875 ** 2 / (2 * math.pi * 30000.0 ** 2) * math.sqrt(2.1e5 * 5e9 / 100.0 * 1e0),
        "tw_cycles": 2 * 50 * 1e6 * (5 / 8.0) ** 2 * math.exp(-(5 / 8.0) ** 2),
        "tw_ovalling": 3 * 8 / math.sqrt(10) / (2 * math.pi * 500.0 ** 2) * math.sqrt(E * 6 ** 2 / (12 * 0.91 * 7.85e-9)),
    }


def containment():
    gamma, k, mu, a, u, z = 9.0e-6, 0.5, 0.4, math.pi * 3000.0 ** 2, 2 * math.pi * 3000.0, 12000.0
    z0 = a / (mu * k * u)
    ph = lambda x: gamma * k * z0 * (1 - math.exp(-x / z0))
    nx, _ = integrate.quad(lambda x: mu * ph(x), 0, z)
    return {
        "ct_ph": ph(z),
        "ct_nx": nx,
        "ct_pv": gamma * z0 * (1 - math.exp(-z / z0)),
        "ct_tank_t": 1000.0 * 9.81 * 1e-9 * (15000.0 - 3000.0) * 10000.0 / 235.0,
        "ct_pipe_t": 8.0 * 508.0 / (2 * 355.0 / 1.1),
        "ct_pipe_ax": 0.3 * (8.0 * 500.0 / 10.0) - E * 1.2e-5 * 40.0,
        "ct_pcr": 2 * E / (1 - 0.09) * (10 / 500.0) ** 3,
    }


def piling():
    ys = [0, 5, 25, 50, 75, 100]
    return {
        "pi_loss_30": float(np.interp(30, ys, [0, 0.0, 0.30, 0.60, 0.90, 1.20])),
        "pi_loss_3": float(np.interp(3, ys, [0, 0.15, 0.75, 1.50, 2.25, 3.00])),
        "pi_fy_red": 355 * (1 - (2 * 0.8 - 1) ** 2),
        "pi_vpl": 3000.0 * 355 / (math.sqrt(3) * 1.0),
    }


def crane():
    ir, b, tf, tw = 1.2e6, 200.0, 20.0, 12.0
    irf = ir + b * tf ** 3 / 12
    leff = 3.25 * (irf / tw) ** (1 / 3)
    soz = 180e3 / (leff * tw)
    return {
        "cr_leff": leff,
        "cr_soz": soz,
        "cr_tau": 0.2 * soz,
        "cr_vm": math.sqrt(120 ** 2 + soz ** 2 - 120 * soz + 3 * (0.2 * soz) ** 2) / 355.0,
        "cr_hertz": 0.418 * math.sqrt(180e3 * 210000 / (50 * 300)),
        "cr_lambda": (0.5 * 0.25) ** (1 / 3),
        "cr_phi2": 1.1 + 0.3 * 0.5,
    }


def sections():
    """📐️ Angle (L 100x65x8, sharp corners, principal axes u-u and v-v) and circular hollow section (168.3x6.3) properties by the `sectionproperties` finite-element analysis (mm), and the Table 5.2 sheet 3 classes of S355 by the printed limits."""
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import angle_section, circular_hollow_section

    def solve(geometry, mesh):
        geometry.create_mesh(mesh_sizes=[mesh])
        s = Section(geometry)
        s.calculate_geometric_properties()
        s.calculate_warping_properties()
        s.calculate_plastic_properties()
        return s

    a = solve(angle_section(d=100.0, b=65.0, t=8.0, r_r=0, r_t=0, n_r=1), 0.5)
    c = solve(circular_hollow_section(d=168.3, t=6.3, n=128), 2.1)
    i11, i22 = a.get_ip()
    zp, sp = a.get_zp(), a.get_sp()
    ixx, _, _ = c.get_ic()
    eps = math.sqrt(235.0 / 355.0)
    return {
        "sec_ang_a": a.get_area(),
        "sec_ang_iu": i11,
        "sec_ang_iv": i22,
        "sec_ang_alpha": a.get_phi() % 180.0,
        "sec_ang_wel_u": min(abs(zp[0]), abs(zp[1])),
        "sec_ang_wel_v": min(abs(zp[2]), abs(zp[3])),
        "sec_ang_wpl_u": sp[0],
        "sec_ang_wpl_v": sp[1],
        "sec_ang_it": a.get_j(),
        "sec_chs_a": c.get_area(),
        "sec_chs_i": ixx,
        "sec_chs_wel": c.get_z()[0],
        "sec_chs_wpl": c.get_s()[0],
        "sec_chs_it": c.get_j(),
        "sec_ang_class": float(3 if 100.0 / 8.0 <= 15 * eps and (100.0 + 65.0) / 16.0 <= 11.5 * eps else 4),
        "sec_chs_class": float(next((i + 1 for i, lim in enumerate((50, 70, 90)) if 168.3 / 6.3 <= lim * eps * eps), 4)),
    }


def wired():
    """🔗️ Independent utilisations of the wired checks of the special-parts showcase subject (SI inputs of En1993Snapshot::special_parts_showcase, gamma_M0 = 1.0 of the DE annex)."""
    tank = (1000.0 * 9.81 * (6000.0 - 2000.0) * 3000.0 * 1e-9 / 235.0) / 8.0
    temps = [10.0, 0.0, -10.0, -20.0, -30.0, -40.0, -50.0]
    s355_j2 = [90.0, 75.0, 65.0, 55.0, 45.0, 40.0, 35.0]
    t_max = float(np.interp(-20.0, temps[::-1], s355_j2[::-1]))
    t_net = 12.0 - 2.0 * float(np.interp(50.0, [0.0, 5.0, 25.0, 50.0, 75.0, 100.0], [0.0, 0.15, 0.75, 1.50, 2.25, 3.00]))
    z0 = 28.3 / (0.4 * 0.5 * 18.8)
    ph = integrate.quad(lambda z: 9000.0 * 0.5 * math.exp(-z / z0), 0.0, 6.0)[0]
    fz = 1.35 * 1.25 * 150000.0
    irf = 1.0e7 + 200.0 * 20.0 ** 3 / 12.0
    leff = 3.25 * (irf / 10.0) ** (1 / 3)
    soz = fz / (leff * 10.0)
    tau = 0.2 * soz + (fz * 25.0) * 10.0 / 2.0e6
    return {
        "w_tank": tank,
        "w_brittle": 17.0 / t_max,
        "w_pile_il": 400.0 / (0.8 * 355.0 * t_net / math.sqrt(3.0)),
        "w_cf_shear": 6000.0 / (150.0 * 3.0 * 0.58 * 350.0),
        "w_thin_shell": (8.0 / 3000.0) * 20.0,
        "w_silo_ph": (ph * 3.0 / 0.008 / 1e6) / 235.0,
        "w_crane_vm": math.sqrt(60.0 ** 2 + soz ** 2 - 60.0 * soz + 3.0 * tau ** 2) / 355.0,
    }


def spec_rows():
    """🧭 The spec-row cross-checks of the sibling folder, loaded by path (the isolated interpreter has no script directory on its path)."""
    path = pathlib.Path(__file__).resolve().parent / "🧭️spec-rows" / "🐍️.py"
    spec = importlib.util.spec_from_file_location("en1993_spec_rows", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main():
    out = {}
    for f in (cold_formed, stainless, plated, shells, toughness, tension, joints, bridges, towers, containment, piling, crane, sections, wired):
        out.update(f())
    out.update(spec_rows().cross_checks(out))
    print(json.dumps(out))


if __name__ == "__main__":
    main()
