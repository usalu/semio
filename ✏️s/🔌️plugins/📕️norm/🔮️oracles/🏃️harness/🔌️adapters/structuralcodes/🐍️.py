"""🏗️ fib structuralcodes adapter for EN 1992-1-1:2004: material laws, shear, crack width and section bending/M-N strength; SI in (Pa, m, N), the library works in MPa and mm."""

import math

import shapely
import structuralcodes.codes.ec2_2004 as ec2
from structuralcodes.geometry import SurfaceGeometry, add_reinforcement
from structuralcodes.materials.concrete import create_concrete
from structuralcodes.materials.reinforcement import create_reinforcement
from structuralcodes.sections import BeamSection

LIBRARY = "structuralcodes"
MPA, MM = 1.0e6, 1.0e3


def concrete_laws(i):
    """📈️ Table 3.1 strengths and strains, 3.1.7 parabola-rectangle parameters and Eq. 3.15 design strength."""
    fck = i["fck"] / MPA
    fcm = ec2.fcm(fck)
    return {"fcm": fcm * MPA, "fctm": ec2.fctm(fck) * MPA, "fctk_5": ec2.fctk_5(ec2.fctm(fck)) * MPA, "fctk_95": ec2.fctk_95(ec2.fctm(fck)) * MPA, "Ecm": ec2.Ecm(fcm) * MPA, "eps_c1": ec2.eps_c1(fcm), "eps_cu1": ec2.eps_cu1(fck), "eps_c2": ec2.eps_c2(fck), "eps_cu2": ec2.eps_cu2(fck), "n": ec2.n_parabolic_rectangular(fck), "fcd": ec2.fcd(fck, i["alpha_cc"], i["gamma_c"]) * MPA}


def design_strength(i):
    """🧮️ Eq. 3.15 design compressive strength with alpha_cc and gamma_c from the National Annex."""
    return {"f_cd": ec2.fcd(i["f_ck"] / MPA, i["alpha_cc"], i["gamma_c"]) * MPA}


def shear_without_links(i):
    """✂️ Eq. 6.2 V_Rd,c with C_Rd,c and k1 from the National Annex; sigma_cp is turned into N_Ed over A_c = b_w d."""
    fck, fcd = i["f_ck"] / MPA, i["alpha_cc"] * i["f_ck"] / i["gamma_c"] / MPA
    ac = i["b_w"] * i["d"] * MM * MM
    return {"V_Rd_c": ec2.VRdc(fck, i["d"] * MM, i["A_sl"] * MM * MM, i["b_w"] * MM, i["sigma_cp"] / MPA * ac, ac, fcd, k1=i["k1"], gamma_c=i["gamma_c"], CRdc=i["C_Rdc"])}


def shear_with_links(i):
    """🪢 Eq. 6.8-6.9 V_Rd,s and V_Rd,max for a strut angle given by cot(theta); alpha_cw from N_Ed and A_c."""
    theta = math.degrees(math.atan(1.0 / i["cot_theta"]))
    fck, fcd = i["f_ck"] / MPA, i["alpha_cc"] * i["f_ck"] / i["gamma_c"] / MPA
    ac = i["Ac"] * MM * MM
    return {"V_Rd_max": ec2.VRdmax(i["b"] * MM, i["z"] * MM, fck, theta, i.get("N_Ed", 0.0), ac, fcd), "V_Ed_max_unreinforced": ec2.VEdmax_unreinf(i["b"] * MM, i["d"] * MM, fck, fcd), "V_Rd_s": ec2.VRds(i["Asw"] * MM * MM, i["s"] * MM, i["z"] * MM, theta, i["fyk"] / MPA, gamma_s=i["gamma_s"])}


def crack_width(i):
    """🕸️ Eq. 7.8 crack width w_k = s_r,max (eps_sm - eps_cm)."""
    return {"w_k": ec2.wk(i["s_r_max"] * MM, i["epsilon_sm_minus_epsilon_cm"]) / MM}


def crack_spacing(i):
    """📏️ Eq. 7.11 maximum crack spacing for close bar spacing with k1..k4 from the National Annex."""
    return {"sr_max": ec2.sr_max_close(i["c"] * MM, i["diam"] * MM, i["rho_p_eff"], i["k_1"], i["k_2"], i["k_3"], i["k_4"]) / MM}


def crack_strain(i):
    """🧵️ Eq. 7.9 strain difference eps_sm - eps_cm with tension stiffening factor k_t."""
    ae = ec2.alpha_e(i["e_s"] / MPA, i["e_cm"] / MPA)
    return {"alpha_e": ae, "epsilon_sm_minus_epsilon_cm": ec2.eps_sm_eps_cm(i["sigma_s"] / MPA, ae, i["rho_p_eff"], i["k_t"], i["f_ct_eff"] / MPA, i["e_s"] / MPA)}


def section(i):
    """🧱️ Rectangular section of height h with one lumped bar group of area As at depth d below the top, bilinear steel (hardening factor `k_hardening` on f_yk reached at 0.9 `eps_uk`) and parabola-rectangle concrete."""
    fck = i["fck"] / MPA
    concrete = create_concrete(fck=fck, design_code="ec2_2004", alpha_cc=i["alpha_cc"], gamma_c=i["gamma_c"])
    steel = create_reinforcement(fyk=i["fyk"] / MPA, Es=i.get("Es", 200.0e9) / MPA, ftk=i.get("k_hardening", 1.0) * i["fyk"] / MPA, epsuk=i.get("eps_uk", 0.075), gamma_s=i["gamma_s"], design_code="ec2_2004")
    b, h = i["b"] * MM, i["h"] * MM
    outline = shapely.Polygon([(-b / 2, -h / 2), (b / 2, -h / 2), (b / 2, h / 2), (-b / 2, h / 2)])
    geometry = add_reinforcement(SurfaceGeometry(outline, concrete), (0.0, h / 2 - i["d"] * MM), math.sqrt(4.0 * i["As"] * MM * MM / math.pi), steel)
    return BeamSection(geometry).section_calculator


def flexure_rect(i):
    """📐️ Pure-bending ultimate moment M_Rd of the singly reinforced rectangle by fibre integration; neutral axis depth from the strain profile."""
    result = section(i).calculate_bending_strength(theta=0.0, n=0.0)
    return {"M_Rd": abs(result.m_y) / MM}


def nm_interaction(i):
    """🧭️ Ultimate moment about the section centroid for each axial force in `N_Ed` (compression positive); the library takes tension positive."""
    calc = section(i)
    return {f"M_Rd_{k}": abs(calc.calculate_bending_strength(theta=0.0, n=-n).m_y) / MM for k, n in enumerate(i["N_Ed"])}


def creep_shrinkage(i):
    """⏳️ Annex B creep coefficient phi(t, t0) (B.1-B.9) and shrinkage strains (3.8-3.12) from the fib library; cement class by `alpha_cement` index 0, 1, 2 for S, N, R; Eq. B.9 adjusts t0 inside beta(t0) of Eq. B.5 only, beta_c(t, t0) keeps the actual loading age."""
    fck, rh, t0, t, ts = i["fck"] / MPA, i["RH"], i["t0"], i["t"], i["t_s"]
    cement = "SNR"[int(i["cement_class"])]
    fcm = ec2.fcm(fck)
    h0 = ec2.h_0(i["Ac"] * MM * MM, i["u"] * MM)
    a1, a2, a3 = ec2.alpha_1(fcm), ec2.alpha_2(fcm), ec2.alpha_3(fcm)
    phi_rh = ec2.phi_RH(h0, fcm, rh, a1, a2)
    phi0 = ec2.phi_0(phi_rh, ec2.beta_fcm(fcm), ec2.beta_t0(ec2.t0_adj(t0, ec2.alpha_cement(cement))))
    phi = float(ec2.phi(phi0, ec2.beta_c(t0, t, ec2.beta_H(h0, fcm, rh, a3))))
    eps_cd_0 = ec2.eps_cd_0(ec2.alpha_ds1(cement), ec2.alpha_ds2(cement), fcm, ec2.beta_RH(rh))
    eps_cd = float(ec2.eps_cd(ec2.beta_ds(t, ts, h0), ec2.k_h(h0), eps_cd_0))
    eps_ca = float(ec2.eps_ca(ec2.beta_as(t), ec2.eps_ca_inf(fck)))
    return {"h_0": h0 / MM, "phi_RH": float(phi_rh), "phi_0": float(phi0), "phi": phi, "eps_cd_0": eps_cd_0, "eps_cd": eps_cd, "eps_ca": eps_ca, "eps_cs": float(ec2.eps_cs(eps_cd, eps_ca))}


def min_reinforcement(i):
    """🧷️ Eq. 7.1 A_s,min = k_c k f_ct,eff A_ct / sigma_s for a rectangle in bending (A_ct = b h / 2 in tension, k_c from 7.3.2(2) with N_Ed, which the library takes in kN) and k(h) from Figure 7.1."""
    h, b = i["h"] * MM, i["b"] * MM
    kc = ec2.kc_rect_area(h, b, i["f_ct_eff"] / MPA, i["N_Ed"] / 1.0e3)
    k = ec2.k(h)
    return {"k_c": kc, "k": k, "A_s_min": ec2.As_min(b * h / 2.0, i["sigma_s"] / MPA, i["f_ct_eff"] / MPA, k, kc) / (MM * MM)}


def tension_area(i):
    """📦️ 7.3.2(3) effective tension height h_c,ef = min(2.5 (h - d), (h - x) / 3, h / 2) and Eq. 7.10 rho_p,eff = A_s / A_c,eff."""
    h, d, x, b = i["h"] * MM, i["d"] * MM, i["x"] * MM, i["b"] * MM
    hc = ec2.hc_eff(h, d, x)
    return {"h_c_ef": hc / MM, "rho_p_eff": ec2.rho_p_eff(i["As"] * MM * MM, 1.0, 0.0, b * hc)}


def crack_spacing_far(i):
    """📏️ Eq. 7.14 s_r,max = 1.3 (h - x) for bar spacing above 5 (c + phi / 2)."""
    return {"sr_max": ec2.sr_max_far(i["h"] * MM, i["x"] * MM) / MM}


def shear_link_limit(i):
    """🪢 Eq. 6.12 maximum effective cross-section of the shear reinforcement A_sw,max / s ... returned as A_sw,max in m2."""
    fck = i["f_ck"] / MPA
    fcd = i["alpha_cc"] * fck / i["gamma_c"]
    return {"A_sw_max": ec2.Asw_max(fcd, fck, i["b_w"] * MM, i["s"] * MM, i["f_ywd"] / MPA, i.get("N_Ed", 0.0), i["Ac"] * MM * MM) / (MM * MM)}


def section_t(i):
    """🏛️ T-section with flange b_f x h_f on top and web b_w down to h, bar group A_s at depth d below the top edge."""
    fck = i["fck"] / MPA
    concrete = create_concrete(fck=fck, design_code="ec2_2004", alpha_cc=i["alpha_cc"], gamma_c=i["gamma_c"])
    steel = create_reinforcement(fyk=i["fyk"] / MPA, Es=i.get("Es", 200.0e9) / MPA, ftk=i["fyk"] / MPA, epsuk=i.get("eps_uk", 0.075), gamma_s=i["gamma_s"], design_code="ec2_2004")
    bf, hf, bw, h = (i[k] * MM for k in ("b_f", "h_f", "b_w", "h"))
    outline = shapely.Polygon([(-bw / 2, -h / 2), (bw / 2, -h / 2), (bw / 2, h / 2 - hf), (bf / 2, h / 2 - hf), (bf / 2, h / 2), (-bf / 2, h / 2), (-bf / 2, h / 2 - hf), (-bw / 2, h / 2 - hf)])
    geometry = add_reinforcement(SurfaceGeometry(outline, concrete), (0.0, h / 2 - i["d"] * MM), math.sqrt(4.0 * i["As"] * MM * MM / math.pi), steel)
    return BeamSection(geometry).section_calculator


def flexure_t(i):
    """📐️ Pure-bending ultimate moment of the T-section by fibre integration."""
    return {"M_Rd": abs(section_t(i).calculate_bending_strength(theta=0.0, n=0.0).m_y) / MM}


def composite_ultimate(i):
    """📉️ EN 1994-1-1 6.2.1.4 non-linear bending resistance of a steel I-section with concrete flange: structuralcodes fibre integration with the parabola-rectangle law and bilinear steel up to eps_cu or eps_su; SI in, library units MPa and mm."""
    from shapely import Polygon
    from structuralcodes.geometry import CompoundGeometry
    from structuralcodes.materials.basic import GenericMaterial
    from structuralcodes.materials.constitutive_laws import ElasticPlastic, ParabolaRectangle
    from structuralcodes.sections import GenericSection

    concrete = GenericMaterial(density=2400, constitutive_law=ParabolaRectangle(fc=-i["f_cd"] / MPA, eps_u=-i.get("eps_cu", 3.5e-3)))
    steel = GenericMaterial(density=7850, constitutive_law=ElasticPlastic(E=i["E_a"] / MPA, fy=i["f_yd"] / MPA, eps_su=i.get("eps_su", 0.02)))
    h, b, tf, tw, hc, beff = (i[k] * MM for k in ("h", "b", "tf", "tw_eff", "h_c", "b_eff"))
    profile = Polygon([(-b / 2, 0), (b / 2, 0), (b / 2, tf), (tw / 2, tf), (tw / 2, h - tf), (b / 2, h - tf), (b / 2, h), (-b / 2, h), (-b / 2, h - tf), (-tw / 2, h - tf), (-tw / 2, tf), (-b / 2, tf)])
    slab = Polygon([(-beff / 2, h), (beff / 2, h), (beff / 2, h + hc), (-beff / 2, h + hc)])
    geometry = CompoundGeometry([SurfaceGeometry(profile, steel), SurfaceGeometry(slab, concrete)])
    result = GenericSection(geometry).section_calculator.calculate_bending_strength(theta=0.0, n=0.0)
    return {"M_Rd": abs(result.m_y) / MM, "kappa_u": abs(result.chi_y) * MM}


CHECKS = {
    "en1992.3.1.concrete-laws": concrete_laws,
    "en1992.3.15.fcd": design_strength,
    "en1992.6.2.shear-vrdc": shear_without_links,
    "en1992.6.2.shear-links": shear_with_links,
    "en1992.7.8.crack-width": crack_width,
    "en1992.7.11.sr-max": crack_spacing,
    "en1992.7.9.strain-difference": crack_strain,
    "en1992.6.1.flexure-rect": flexure_rect,
    "en1992.6.1.nm-interaction": nm_interaction,
    "en1992.3.1.creep-shrinkage": creep_shrinkage,
    "en1992.7.3.min-reinforcement": min_reinforcement,
    "en1992.7.3.effective-tension-area": tension_area,
    "en1992.7.14.sr-max-far": crack_spacing_far,
    "en1992.6.2.shear-link-limit": shear_link_limit,
    "en1992.6.1.flexure-t": flexure_t,
    "en1994.6.2.1.4.composite-ultimate": composite_ultimate,
}
