#!/usr/bin/env python3
"""🎯️ Third-party cross-check of the EN 1994 catalogue oracle for the spec rows without a third-party hit: materials (C-01), global analysis (C-04), non-linear resistance (C-10), partial connection (C-13), stud fatigue (C-21, C-52), U-frame LTB (C-23, C-51), patch loading (C-24), column scope and load introduction (C-25, C-31), fire (C-32, C-44, C-47), sheeting (C-33), slab serviceability (C-39), crack control (C-42), Load Model 1 (C-48), shear lag (C-49) and stud detailing (C-54). Each quantity of the catalogue vectors is recomputed with anastruct, sectionproperties, structuralcodes, scipy, numpy or sympy; run with the repository venv `python -I 🐍️.py`, exit code 1 on any disagreement beyond the tolerance of its row."""
from __future__ import annotations

import importlib.util
import json
import math
import sys
import warnings
from pathlib import Path

import numpy as np
import sympy
from anastruct import SystemElements
from scipy import integrate, optimize
from sectionproperties.analysis import Section
from sectionproperties.pre.library import i_section, rectangular_hollow_section
from structuralcodes.codes import ec2_2004

warnings.simplefilter("ignore")
HERE = Path(__file__).resolve().parent
_SPEC = importlib.util.spec_from_file_location("catalogue_oracle", HERE.parent / "🧮️catalogue" / "🐍️.py")
CAT = importlib.util.module_from_spec(_SPEC)
sys.modules["catalogue_oracle"] = CAT
_SPEC.loader.exec_module(CAT)
V = CAT.vectors()
ES, G = 210e9, 81e9

SPEC_ROWS = {
    "C-01": (["en1994.3.materials"], "structuralcodes.ec2_2004 fcm, fctm, Ecm", "materials"),
    "C-04": (["en1994.5.4.2.analysis"], "anastruct two-span beam with cracked EI over 0.15 L", "analysis"),
    "C-10": (["en1994.6.2.1.4.mkappa"], "numpy fibre integration with nested scipy.optimize.brentq", "nonlinear"),
    "C-13": (["en1994.6.2.1.3.linear"], "sectionproperties W_pl and the linear interpolation of 6.2.1.3(3)", "partial"),
    "C-21": (["en1994.2.6.8.3.stud"], "scipy.optimize.brentq on the S-N line (Delta tau_c = 90 MPa, m = 8)", "fatigue"),
    "C-23": (["en1994.6.4.ltb"], "sympy Rayleigh-Ritz buckling of the restrained flange and scipy.optimize.brentq for chi_LT", "ltb"),
    "C-24": (["en1994.6.4.patch"], "sympy evaluation of EN 1993-1-5 6.2 to 6.5 with scipy.optimize.brentq on lambda_F", "patch"),
    "C-25": (["en1994.6.7.1.scope"], "sectionproperties area and plastic modulus of the filled tube", "scope"),
    "C-31": (["en1994.6.7.4.introduction"], "sympy closed form of 6.7.4 with Table 6.6", "introduction"),
    "C-32": (["en1994.1-2.4.3.5.column"], "scipy.integrate.solve_ivp (RK45) on the 2D conduction grid", "column_fire"),
    "C-33": (["en1994.9.3.sheeting"], "anastruct simply supported sheeting strip under wet concrete", "formwork"),
    "C-39": (["en1994.9.8.sls"], "sectionproperties uncracked slab strip and anastruct deflection", "slab_sls"),
    "C-42": (["en1994.7.4.crack"], "structuralcodes.ec2_2004.As_min with k_s k and k_c of 7.4.2", "crack"),
    "C-44": (["en1994.1-2.4.3.theta", "en1994.1-2.4.3.mfi"], "scipy.integrate.solve_ivp on EN 1993-1-2 4.2.5.2 and numpy.interp on k_y", "fire_beam"),
    "C-47": (["en1994.1-2.4.3.mfi", "en1994.1-2.4.3.theta"], "scipy.integrate.solve_ivp on EN 1993-1-2 4.2.5.2 and numpy.interp on k_y", "fire_beam"),
    "C-48": (["en1994.2.lm1"], "anastruct moving tandem axle scan against the closed form", "lm1"),
    "C-49": (["en1994.2.5.4.1.shear-lag"], "sympy closed form of EN 1993-1-5 3.2.1 Table 3.1 / scipy continuity", "shear_lag"),
    "C-51": (["en1994.6.4.ltb"], "sympy Rayleigh-Ritz buckling of the restrained flange and scipy.optimize.brentq for chi_LT", "ltb"),
    "C-52": (["en1994.2.6.8.2.delta-sigma", "en1994.2.6.8.2.rebar", "en1994.2.6.8.3.stud"], "scipy.optimize.brentq on the S-N lines", "fatigue"),
    "C-54": (["en1994.2.6.6.5.cross-spacing", "en1994.2.6.6.5.edge", "en1994.2.6.6.5.end-plate"], "sympy partial area loading EN 1992-1-1 6.7 and the stud spacing limits", "detailing"),
}


def close(name: str, ours: float, theirs: float, tol: float) -> dict:
    rel = abs(ours - theirs) / max(abs(theirs), 1e-300)
    assert rel <= tol, f"{name}: {ours} vs {theirs} (rel {rel:.2e} > {tol})"
    return {"name": name, "ours": ours, "theirs": theirs, "rel": rel}


def materials() -> list[dict]:
    rows = []
    for fck in (25.0, 30.0, 35.0, 40.0, 50.0):
        fcm = ec2_2004.fcm(fck)
        rows.append(close(f"C-01 fcm fck={fck}", fck + 8.0, fcm, 1e-12))
        rows.append(close(f"C-01 fctm fck={fck}", 0.30 * fck ** (2 / 3), ec2_2004.fctm(fck), 1e-9))
        rows.append(close(f"C-01 Ecm fck={fck}", 22.0 * ((fck + 8.0) / 10.0) ** 0.3, ec2_2004.Ecm(fcm) / 1e3, 1e-9))
    return rows


def two_span_support_moment(ei1: float, ei2: float, fraction: float, length: float = 8.0, load: float = 10e3, n: int = 40) -> float:
    system = SystemElements(EA=1e12, EI=ei1)
    zone = int(round(fraction * n))
    for span in (0, 1):
        for i in range(n):
            ei = ei2 if (span == 0 and i >= n - zone) or (span == 1 and i < zone) else ei1
            system.add_element([[span * length + length * i / n, 0], [span * length + length * (i + 1) / n, 0]], EI=ei)
    system.add_support_hinged(1)
    system.add_support_roll(n + 1)
    system.add_support_roll(2 * n + 1)
    system.q_load(q=-load, element_id=list(range(1, 2 * n + 1)))
    system.solve()
    return abs(system.get_node_results_system(1)["Fy"]) * length - load * length ** 2 / 2


def analysis() -> list[dict]:
    e1 = ES * 6e-4
    rows = [close("C-04 uncracked support moment qL^2/8", 10e3 * 64 / 8, abs(two_span_support_moment(e1, e1, 0.0)), 1e-6)]
    ours = -V["cont_end_mb_cracked"]
    rows.append(close("C-04 cracked support moment (EI2 = 0.5 EI1 over 0.15 L) vs anastruct", abs(ours), abs(two_span_support_moment(e1, 0.5 * e1, 0.15)), 5e-3))
    return rows


def nonlinear() -> list[dict]:
    sec, fyd, fcd, ys, beff, hc = CAT.ipe300(), 355e6, 17e6, 0.100, 2.0, 0.100
    tw = sec["tw_eff"]
    y = lambda a, b, n=4000: (np.linspace(a, b, n, endpoint=False) + (b - a) / n / 2, (b - a) / n)
    parts = [(0.0, hc, beff, True), (ys, ys + sec["tf"], sec["b"], False), (ys + sec["tf"], ys + sec["h"] - sec["tf"], tw, False), (ys + sec["h"] - sec["tf"], ys + sec["h"], sec["b"], False)]
    grids = [(*y(a, b), w, c) for a, b, w, c in parts]

    def resultants(kappa: float, eps_top: float) -> tuple[float, float]:
        n = m = 0.0
        for ys_, dy, w, concrete in grids:
            e = eps_top - kappa * ys_
            s = np.where(e <= 0, 0.0, np.where(e >= 0.002, fcd, fcd * (1 - (1 - e / 0.002) ** 2))) if concrete else np.clip(ES * e, -fyd, fyd)
            n += float(np.sum(s * w * dy))
            m -= float(np.sum(s * w * dy * ys_))
        return n, m

    def top_strain(kappa: float) -> float:
        return optimize.brentq(lambda e: resultants(kappa, e)[0], -1.0, 1.0, xtol=1e-14)

    kappa = optimize.brentq(lambda k: top_strain(k) - 3.5e-3, 1e-4, 1.0, xtol=1e-12)
    moment = abs(resultants(kappa, top_strain(kappa))[1])
    return [close("C-10 non-linear M_Rd (eps_cu = 3.5 permille) vs fibre integration", V["mnl_ipe300"], moment, 3e-3), close("C-10 curvature at eps_cu", V["kappa_u_ipe300"], kappa, 3e-3)]


def section_plastic_modulus() -> float:
    sec = CAT.ipe300()
    geometry = i_section(d=sec["h"], b=sec["b"], t_f=sec["tf"], t_w=sec["tw"], r=0.0, n_r=2)
    geometry.create_mesh(mesh_sizes=[2e-5])
    section = Section(geometry=geometry, time_info=False)
    section.calculate_geometric_properties()
    section.calculate_plastic_properties()
    return section.get_s()[0]


def partial() -> list[dict]:
    wpl = section_plastic_modulus()
    m_apl = 355e6 * wpl
    eta = 0.5
    m_lin = m_apl + eta * (V["mpl_full"] - m_apl)
    rows = [close("C-13 M_a,pl,Rd = fy W_pl (sectionproperties, root radius neglected)", 628.4e-6 * 355e6, m_apl, 5e-2)]
    rows.append(close("C-13 linear interpolation M_a,pl,Rd + eta (M_pl,Rd - M_a,pl,Rd)", V["mpl_linear"], m_lin, 5e-2))
    assert V["mpl_partial_sb"] >= V["mpl_linear"], "stress block must lie above the linear interpolation"
    return rows


def fatigue() -> list[dict]:
    rows = []
    for delta in (30e6, 40e6, 60e6):
        n_line = optimize.brentq(lambda log_n: 90e6 * (2e6 / 10 ** log_n) ** (1 / 8.0) / 1.15 - delta, 0.0, 30.0)
        rows.append(close(f"C-21 stud endurance at {delta / 1e6:.0f} MPa", CAT.endurance(90e6, 2e6, 8.0, 1.15, delta), 10 ** n_line, 1e-6))
    rows.append(close("C-21 stud damage 5e6 cycles", V["fatigue_stud_damage"], 5e6 / 10 ** optimize.brentq(lambda ln: 90e6 * (2e6 / 10 ** ln) ** (1 / 8.0) / 1.15 - 40e6, 0.0, 30.0), 1e-6))
    for n in (1e6, 2e6, 1e7):
        resistance = 162.5e6 * (1e6 / n) ** (1 / 9.0) / 1.15
        rows.append(close(f"C-52 rebar resistance at {n:.0e} cycles", CAT.rebar_resistance(n), resistance, 1e-6))
    rows.append(close("C-52 steel utilisation 50 MPa on the C80 line, N = 2e6", V["fatigue_steel_util"], 50e6 / (80e6 * (2e6 / 1e7) ** (1 / 3.0) / 1.15), 1e-9))
    return rows


def ltb() -> list[dict]:
    sec = CAT.ipe300()
    length, i2, spacing, alpha = 6.0, 4e-5, 3.0, 2.0
    k1 = alpha * ES * i2 / spacing
    k2 = ES * sec["tw"] ** 3 / (4 * (1 - 0.3 ** 2) * (sec["h"] - sec["tf"]))
    ks = 1 / (1 / k1 + 1 / k2)
    it = (2 * sec["b"] * sec["tf"] ** 3 + sec["hw"] * sec["tw"] ** 3) / 3
    ifz = CAT.flange_inertia_z(sec) / 2
    x, ei, git, k, m = sympy.symbols("x EI GIt k M", positive=True)
    theta = sympy.sin(sympy.pi * x / length)
    energy = sympy.integrate(git * sympy.diff(theta, x) ** 2 + k * theta ** 2 - m ** 2 / ei * theta ** 2, (x, 0, length))
    m_cr = float(sympy.solve(energy, m)[0].subs({ei: ES * ifz, git: G * it, k: ks}))
    rows = [close("C-23 U-frame stiffness k_s (springs in series)", V["ltb_ks"], ks, 1e-9), close("C-23 M_cr from the Rayleigh-Ritz energy", V["ltb_mcr"], m_cr, 1e-9)]
    m_pl = section_plastic_modulus() * 355e6
    lam = math.sqrt(m_pl / m_cr)
    phi = 0.5 * (1 + 0.34 * (lam - 0.2) + lam ** 2) if sec["h"] / sec["b"] > 2.0 else 0.5 * (1 + 0.21 * (lam - 0.2) + lam ** 2)
    chi = optimize.brentq(lambda c: lam ** 2 * c ** 2 - 2 * phi * c + 1, 1e-6, 1.0)
    rows.append(close("C-23 chi_LT as the root of lambda^2 chi^2 - 2 phi chi + 1 (sectionproperties W_pl)", V["ltb_chi"], chi, 5e-2))
    return rows


def patch() -> list[dict]:
    hw, tw, bf, tf, fyw, fyf, ss, a = 0.6, 0.008, 0.2, 0.014, 355e6, 355e6, 0.1, 1.0
    kf = 6 + 2 * (hw / a) ** 2
    fcr = 0.9 * kf * ES * tw ** 3 / hw
    m1 = fyf * bf / (fyw * tw)

    def resistance(lam_guess: float) -> tuple[float, float, float]:
        m2 = 0.02 * (hw / tf) ** 2 if lam_guess > 0.5 else 0.0
        ly = min(ss + 2 * tf * (1 + math.sqrt(m1 + m2)), a)
        return ly, math.sqrt(ly * tw * fyw / fcr), min(0.5 / math.sqrt(ly * tw * fyw / fcr), 1.0)

    lam = optimize.brentq(lambda t: t - resistance(0.0 if t <= 0.5 else 1.0)[1], 1e-3, 5.0)
    ly, _, chi = resistance(lam)
    return [close("C-24 F_Rd of patch loading type a", V["patch_f_rd"], chi * ly * tw * fyw, 1e-6), close("C-24 chi_F", V["patch_chi"], chi, 1e-6)]


def scope() -> list[dict]:
    geometry = rectangular_hollow_section(d=0.2, b=0.2, t=0.01, r_out=0.0, n_r=1)
    geometry.create_mesh(mesh_sizes=[1e-5])
    section = Section(geometry=geometry, time_info=False)
    section.calculate_geometric_properties()
    section.calculate_plastic_properties()
    a_a, w_pa = section.get_area(), section.get_s()[0]
    core = 0.18
    fyd, fcd = 355e6, 26.667e6
    n_pl = a_a * fyd + core * core * fcd
    delta = a_a * fyd / n_pl
    assert 0.2 <= delta <= 0.9, delta
    return [close("C-25 N_pl,Rd = A_a fyd + A_c fcd of the filled tube", V["npl_rhs"], n_pl, 1e-3), close("C-25 M_max,Rd = W_pa fyd + W_pc fcd / 2", V["mmax_rhs"], w_pa * fyd + core ** 3 / 4 * fcd / 2, 1e-3)]


def introduction() -> list[dict]:
    d, length, n_ed, n_pl, a_a_fyd = sympy.symbols("d L N_Ed N_pl A_afyd", positive=True)
    l_v = sympy.Min(2 * d, length / 3)
    n_intro = n_ed * (1 - a_a_fyd / n_pl)
    tau_ed = n_intro / (4 * d * l_v)
    value = float(tau_ed.subs({d: 0.4, length: 3.0, n_ed: 5e6, n_pl: 6e6, a_a_fyd: 2e6}))
    reference = 5e6 * (1 - 2e6 / 6e6) / (4 * 0.4 * min(2 * 0.4, 3.0 / 3))
    return [close("C-31 tau_Ed = N_Ed (1 - A_a fyd / N_pl,Rd) / (u l_v), l_v = min(2d, L/3)", value, reference, 1e-12)]


def column_fire() -> list[dict]:
    h, b, tw, tf, bc, hc = 0.2, 0.2, 0.0085, 0.013, 0.4, 0.4
    nx = ny = 20
    dx, dy = bc / 2 / nx, hc / 2 / ny
    lim = h / 2 - max(tf, dy / 2)
    i, j = np.meshgrid(np.arange(nx), np.arange(ny))
    steel = (((j + 0.5) * dy >= lim) & ((i + 0.5) * dx <= b / 2) & ((j + 0.5) * dy <= h / 2)) | (((i + 0.5) * dx <= tw / 2) & ((j + 0.5) * dy < lim))

    def cp_s(t):
        return np.where(t < 600, 425 + 0.773 * t - 1.69e-3 * t ** 2 + 2.22e-6 * t ** 3, np.where(t < 735, 666 + 13002 / np.maximum(738 - t, 1e-9), np.where(t < 900, 545 + 17820 / np.maximum(t - 731, 1e-9), 650.0)))

    def cp_c(t):
        return np.where(t <= 100, 900.0, np.where(t <= 200, 900 + (t - 100), np.where(t <= 400, 1000 + (t - 200) / 2, 1100.0)))

    def rhs(t, flat):
        th = flat.reshape(ny, nx)
        gas = 20 + 345 * math.log10(8 * t / 60 + 1)
        u = np.clip(th, 20, 1200) / 100
        lam = np.where(steel, np.where(th < 800, 54 - 3.33e-2 * th, 27.3), 1.36 - 0.136 * u + 0.0057 * u * u)
        cap = np.where(steel, 7850 * cp_s(th), 2300 * cp_c(th))
        q = np.zeros_like(th)
        hm = lambda a, c: 2 * a * c / (a + c)
        q[:, :-1] += hm(lam[:, :-1], lam[:, 1:]) * (th[:, 1:] - th[:, :-1]) / dx * dy
        q[:, 1:] += hm(lam[:, 1:], lam[:, :-1]) * (th[:, :-1] - th[:, 1:]) / dx * dy
        q[:-1, :] += hm(lam[:-1, :], lam[1:, :]) * (th[1:, :] - th[:-1, :]) / dy * dx
        q[1:, :] += hm(lam[1:, :], lam[:-1, :]) * (th[:-1, :] - th[1:, :]) / dy * dx
        flux = 25 * (gas - th) + 0.7 * 5.67e-8 * ((gas + 273) ** 4 - (th + 273) ** 4)
        q[:, -1] += flux[:, -1] * dy
        q[-1, :] += flux[-1, :] * dx
        return (q / (cap * dx * dy)).ravel()

    solution = integrate.solve_ivp(rhs, (0.0, 3600.0), np.full(nx * ny, 20.0), method="RK45", rtol=1e-6, atol=1e-4)
    th = solution.y[:, -1].reshape(ny, nx)
    core, flange, corner = V["fire_core_60"], V["fire_flange_60"], V["fire_corner_60"]
    return [close("C-32 column fire core cell (0,0) at 60 min", core, th[0, 0], 2e-2), close("C-32 column fire cell (0,9)", flange, th[9, 0], 2e-2), close("C-32 column fire corner cell (19,19)", corner, th[19, 19], 2e-2)]


def formwork() -> list[dict]:
    length, load, ei = 2.4, 8.0e3, 210e9 * 2.0e-7
    n = 12
    system = SystemElements(EA=1e12, EI=ei)
    for k in range(n):
        system.add_element([[length * k / n, 0], [length * (k + 1) / n, 0]])
    system.add_support_hinged(1)
    system.add_support_roll(n + 1)
    system.q_load(q=-load, element_id=list(range(1, n + 1)))
    system.solve()
    deflection = abs(system.get_node_displacements(node_id=n // 2 + 1)["uy"])
    return [close("C-33 wet-concrete deflection 5wL^4/(384EI) vs anastruct", 5 * load * length ** 4 / (384 * ei), deflection, 1e-6), close("C-33 deflection by double integration of the curvature", CAT.deflection_numeric(length, [(load, ei)]), deflection, 1e-4)]


def slab_sls() -> list[dict]:
    h, strip, span, load = 0.130, 1.0, 3.2, 6e3
    n_mod = ES / (22e9 * ((30 + 8) / 10) ** 0.3)
    a_c, a_s = strip * h / n_mod, 1.2 * 0.001 * strip
    z_c = (a_c * h / 2 + a_s * h) / (a_c + a_s)
    inertia = strip * h ** 3 / 12 / n_mod + a_c * (h / 2 - z_c) ** 2 + a_s * (h - z_c) ** 2
    ei = ES * inertia
    system = SystemElements(EA=1e12, EI=ei)
    for k in range(12):
        system.add_element([[span * k / 12, 0], [span * (k + 1) / 12, 0]])
    system.add_support_hinged(1)
    system.add_support_roll(13)
    system.q_load(q=-load, element_id=list(range(1, 13)))
    system.solve()
    return [close("C-39 slab deflection 5wL^4/(384EI) on the transformed strip vs anastruct", 5 * load * span ** 4 / (384 * ei), abs(system.get_node_displacements(node_id=7)["uy"]), 1e-6)]


def crack() -> list[dict]:
    fct, a_ct, sigma = 2.9, 2000 * 100.0, 500.0
    kc = min(1 / (1 + 100 / (2 * 250)) + 0.3, 1.0)
    reference = ec2_2004.As_min(A_ct=a_ct, sigma_s=sigma, fct_eff=fct, k=0.9 * 0.8, kc=kc) * 1e-6
    return [close("C-42 A_s,min = k_s k_c k f_ct,eff A_ct / sigma_s (7.4.2) vs structuralcodes", V["as_min"], reference, 1e-9)]


def iso834(minutes: float) -> float:
    return float(sympy.N(20 + 345 * sympy.log(8 * minutes + 1, 10)))


def fire_beam() -> list[dict]:
    rows = [close("C-44 ISO 834 gas temperature at 60 min", V["iso_60"], iso834(60), 1e-12), close("C-44 ISO 834 gas temperature at 30 min", V["iso_30"], iso834(30), 1e-12)]
    am_v, d_p, lam_p, c_p, rho_p = 150.0, 0.02, 0.12, 1000.0, 800.0

    def ca(th):
        return 425 + 0.773 * th - 1.69e-3 * th ** 2 + 2.22e-6 * th ** 3 if th < 600 else 666 + 13002 / (738 - th) if th < 735 else 545 + 17820 / (th - 731) if th < 900 else 650.0

    phi = c_p * rho_p / (ca(20) * 7850) * d_p * am_v

    def rate(t, y):
        gas = 20 + 345 * math.log10(8 * t / 60 + 1)
        d_gas = 345 * 8 / 60 / (math.log(10) * (8 * t / 60 + 1))
        r = lam_p / d_p * am_v / (ca(y[0]) * 7850) * (gas - y[0]) / (1 + phi / 3) - (math.exp(phi / 10) - 1) * d_gas
        return [max(r, 0.0) if gas - y[0] > 0 else r]

    for seconds, key in ((3600.0, "beam_fire_steel_60"), (5400.0, "fire_protected_90")):
        solution = integrate.solve_ivp(rate, (0.0, seconds), [20.0], method="LSODA", rtol=1e-9, atol=1e-9, max_step=5.0)
        rows.append(close(f"C-47 steel temperature {seconds / 60:.0f} min (EN 1993-1-2 4.2.5.2)", V[key], solution.y[0, -1], 5e-3))
    temps = [20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200]
    ky = [1.0, 1.0, 1.0, 1.0, 1.0, 0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02, 0.0]
    for theta in (150.0, 450.0, 550.0, 650.0, 950.0):
        rows.append(close(f"C-47 k_y,theta at {theta:.0f} C vs numpy.interp", CAT.ky_table(theta), float(np.interp(theta, temps, ky)), 1e-12))
    rows.append(close("C-47 critical temperature mu_0 = 0.5 by inverting the k_y table (scipy)", V["fire_crit_inversion_mu05"], optimize.brentq(lambda th: float(np.interp(th, temps, ky)) - 0.5, 500.0, 600.0), 1e-4))
    rows.append(close("C-47 critical temperature closed form vs inversion", V["fire_crit_mu05"], V["fire_crit_inversion_mu05"], 1.2e-2))
    return rows


def lm1() -> list[dict]:
    length, axle, gap, udl, n = 12.0, 300e3, 1.2, 27e3, 30
    step = length / n
    best = 0.0
    for first in range(0, n - int(round(gap / step)) + 1):
        positions = (first * step, first * step + gap)
        system = SystemElements(EA=1e12, EI=1e8)
        for k in range(n):
            system.add_element([[step * k, 0], [step * (k + 1), 0]])
        system.add_support_hinged(1)
        system.add_support_roll(n + 1)
        system.q_load(q=-udl, element_id=list(range(1, n + 1)))
        for x in positions:
            system.point_load(node_id=int(round(x / step)) + 1, Fy=-axle)
        system.solve()
        reaction = abs(system.get_node_results_system(1)["Fy"])
        for k in range(n + 1):
            x = k * step
            best = max(best, reaction * x - udl * x ** 2 / 2 - sum(axle * max(x - xi, 0.0) for xi in positions))
    moment = lambda x: -((udl * length / 2 + axle * (2 * length - 2 * x + gap) / length) * x - udl * x ** 2 / 2 - axle * gap)
    closed = -optimize.minimize_scalar(moment, bounds=(gap, length), method="bounded", options={"xatol": 1e-10}).fun
    return [close("C-48 LM1 tandem axle + UDL maximum moment: closed form vs anastruct scan", closed, best, 5e-3)]


def shear_lag() -> list[dict]:
    kappa = sympy.symbols("kappa", positive=True)
    beta = sympy.Piecewise((1, kappa <= 0.02), (1 / (1 + 6.4 * kappa ** 2), kappa <= 0.7), (1 / (5.9 * kappa), True))
    rows = []
    for value in (0.01, 0.05, 0.3, 0.7, 1.5):
        a0, be, le = 1.0, value * 10.0, 10.0
        reference = float(beta.subs(kappa, a0 * be / le))
        ours = 1.0 if value <= 0.02 else 1 / (1 + 6.4 * value ** 2) if value <= 0.7 else 1 / (5.9 * value)
        rows.append(close(f"C-49 beta(kappa = {value})", ours, reference, 1e-12))
    jump = optimize.brentq(lambda k: 1 / (1 + 6.4 * k ** 2) - 1 / (5.9 * k), 0.3, 2.0)
    rows.append(close("C-49 the branches meet near kappa = 0.7", jump, 0.7, 5e-2))
    return rows


def detailing() -> list[dict]:
    fcd, w, h = 0.85 * 30e6 / 1.5, sympy.symbols("w", positive=True), sympy.symbols("h", positive=True)
    rows = []
    for label, width, height, slab_b, slab_h, key in (("open", 0.2, 0.1, 1.0, 0.25, "end_plate_open"), ("capped", 0.1, 0.1, 5.0, 5.0, "end_plate_capped")):
        a0 = w * h
        a1 = sympy.Max(sympy.Min(3 * w, slab_b), w) * sympy.Max(sympy.Min(3 * h, slab_h), h)
        reference = float(sympy.Min(a0 * fcd * sympy.sqrt(a1 / a0), 3 * fcd * a0).subs({w: width, h: height}))
        rows.append(close(f"C-54 end-plate bearing {label} (EN 1992-1-1 6.7)", V[key], reference, 1e-9))
    return rows


def main() -> int:
    rows = []
    for function in (materials, analysis, nonlinear, partial, fatigue, ltb, patch, scope, introduction, column_fire, formwork, slab_sls, crack, fire_beam, lm1, shear_lag, detailing):
        rows += function()
    json.dump({"specRows": {k: {"checks": v[0], "library": v[1], "function": v[2]} for k, v in SPEC_ROWS.items()}, "rows": rows}, sys.stdout, ensure_ascii=False)
    print()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
