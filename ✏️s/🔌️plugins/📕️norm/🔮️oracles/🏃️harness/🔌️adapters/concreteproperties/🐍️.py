"""🧱️ concreteproperties adapter for EN 1992-1-1:2004 sections: ultimate bending and M-N points by fibre integration with the EN parabola-rectangle law, cracked-section properties and stresses; SI in, the library works in N and mm."""

import math

from concreteproperties import Concrete, ConcreteSection, SteelBar
from concreteproperties.pre import add_bar
from concreteproperties.stress_strain_profile import ConcreteLinear, EurocodeParabolicUltimate, RectangularStressBlock, SteelElasticPlastic, SteelHardening
from sectionproperties.pre.library import rectangular_section

LIBRARY = "concreteproperties"
MPA, MM = 1.0e6, 1.0e3


def eps_c2(fck):
    """📈️ Table 3.1 strain at maximum stress of the parabola-rectangle law (MPa input)."""
    return 2.0e-3 if fck <= 50.0 else (2.0 + 0.085 * (fck - 50.0) ** 0.53) * 1e-3


def eps_cu2(fck):
    """📉️ Table 3.1 ultimate strain of the parabola-rectangle law (MPa input)."""
    return 3.5e-3 if fck <= 50.0 else (2.6 + 35.0 * ((90.0 - fck) / 100.0) ** 4) * 1e-3


def exponent(fck):
    """🔢️ Table 3.1 exponent n of the parabola-rectangle law (MPa input)."""
    return 2.0 if fck <= 50.0 else 1.4 + 23.4 * ((90.0 - fck) / 100.0) ** 4


def materials(i):
    """🧪️ EN limit-state concrete and bar materials; EN limit strains come from Table 3.1 evaluated for the given fck; steel is bilinear with the hardening factor `k_hardening` reaching its limit at 0.9 `eps_uk`."""
    fck = i["fck"] / MPA
    fcd = i["alpha_cc"] * fck / i["gamma_c"]
    ecm = 22000.0 * ((fck + 8.0) / 10.0) ** 0.3
    concrete = Concrete(name="C", density=2.4e-6, stress_strain_profile=ConcreteLinear(elastic_modulus=ecm), colour="lightgrey", ultimate_stress_strain_profile=EurocodeParabolicUltimate(compressive_strength=fcd, compressive_strain=eps_c2(fck), ultimate_strain=eps_cu2(fck), n=exponent(fck)), flexural_tensile_strength=0.3 * fck ** (2.0 / 3.0))
    fyd, es, eps_ud, k = i["fyk"] / MPA / i["gamma_s"], i.get("Es", 200.0e9) / MPA, 0.9 * i.get("eps_uk", 0.075), i.get("k_hardening", 1.0)
    profile = SteelHardening(yield_strength=fyd, elastic_modulus=es, fracture_strain=eps_ud, ultimate_strength=k * fyd) if k > 1.0 else SteelElasticPlastic(yield_strength=fyd, elastic_modulus=es, fracture_strain=eps_ud)
    return concrete, SteelBar(name="S", density=7.85e-6, stress_strain_profile=profile, colour="grey"), ecm


def section(i):
    """🧱️ Rectangular section b x h with the bar group As at depth d below the top."""
    concrete, steel, ecm = materials(i)
    b, h = i["b"] * MM, i["h"] * MM
    geometry = add_bar(rectangular_section(b=b, d=h, material=concrete), area=i["As"] * MM * MM, material=steel, x=b / 2, y=h - i["d"] * MM, n=4)
    return ConcreteSection(geometry), ecm


def flexure_rect(i):
    """📐️ Pure-bending ultimate moment and neutral-axis depth below the compression edge."""
    result = section(i)[0].ultimate_bending_capacity(theta=0.0, n=0.0)
    return {"M_Rd": abs(result.m_xy) / MM, "x": result.d_n / MM}


def nm_interaction(i):
    """🧭️ Ultimate moment about the centroid for each axial force in `N_Ed` (compression positive, the library's sign)."""
    cs = section(i)[0]
    return {f"M_Rd_{k}": abs(cs.ultimate_bending_capacity(theta=0.0, n=n).m_xy) / MM for k, n in enumerate(i["N_Ed"])}


def cracked_section(i):
    """🕸️ Cracked neutral axis, cracked second moment of area (E-weighted, divided by E_cm) and the stresses under the bending moment `M_Ed` at the cracked state."""
    cs, ecm = section(i)
    cracked = cs.calculate_cracked_properties(theta=0.0)
    stress = cs.calculate_cracked_stress(cracked_results=cracked, m=i["M_Ed"] * MM)
    sigma_c = max(float(v.max()) for v in stress.concrete_stresses)
    sigma_s = max(abs(float(v)) for v in stress.lumped_reinforcement_stresses)
    return {"x_cr": cracked.d_nc / MM, "I_cr": cracked.e_ixx_c_cr / ecm / MM**4, "M_cr": cracked.m_cr / MM, "sigma_c": sigma_c * MPA, "sigma_s": sigma_s * MPA}


def section_t(i):
    """🏛️ T-section with flange b_f x h_f on top and web b_w down to h (union of two rectangles), bar group A_s at depth d below the top edge."""
    concrete, steel, _ = materials(i)
    bf, hf, bw, h = (i[k] * MM for k in ("b_f", "h_f", "b_w", "h"))
    web = rectangular_section(b=bw, d=h - hf, material=concrete)
    flange = rectangular_section(b=bf, d=hf, material=concrete).shift_section(x_offset=(bw - bf) / 2.0, y_offset=h - hf)
    return ConcreteSection(add_bar(web + flange, area=i["As"] * MM * MM, material=steel, x=bw / 2, y=h - i["d"] * MM, n=4))


def flexure_t(i):
    """📐️ Pure-bending ultimate moment of the T-section by fibre integration."""
    return {"M_Rd": abs(section_t(i).ultimate_bending_capacity(theta=0.0, n=0.0).m_xy) / MM}


def masonry_flexure(i):
    """🧱️ EN 1996-1-1 6.6.1 reinforced masonry beam or wall strip: rectangular stress block f_d (alpha = 1, depth factor 0.8, strain 3.5 permille) on b x h, bar group A_s at depth d with design strength f_yd; ultimate moment by fibre integration and neutral-axis depth."""
    fd, fyd, b, h = i["f_d"] / MPA, i["f_yd"] / MPA, i["b"] * MM, i["h"] * MM
    masonry = Concrete(name="M", density=2.0e-6, stress_strain_profile=ConcreteLinear(elastic_modulus=1.0e4), colour="lightgrey", ultimate_stress_strain_profile=RectangularStressBlock(compressive_strength=fd, alpha=1.0, gamma=0.8, ultimate_strain=3.5e-3), flexural_tensile_strength=0.0)
    steel = SteelBar(name="S", density=7.85e-6, stress_strain_profile=SteelElasticPlastic(yield_strength=fyd, elastic_modulus=i.get("Es", 200.0e9) / MPA, fracture_strain=0.05), colour="grey")
    geometry = add_bar(rectangular_section(b=b, d=h, material=masonry), area=i["As"] * MM * MM, material=steel, x=b / 2, y=h - i["d"] * MM, n=4)
    result = ConcreteSection(geometry).ultimate_bending_capacity(theta=0.0, n=0.0)
    return {"M_Rd": abs(result.m_xy) / MM, "x": result.d_n / MM}


CHECKS = {
    "en1992.6.1.flexure-rect": flexure_rect,
    "en1992.6.1.nm-interaction": nm_interaction,
    "en1992.7.2.cracked-section": cracked_section,
    "en1992.6.1.flexure-t": flexure_t,
    "en1996.6.6.1.flexure-reinforced": masonry_flexure,
}
