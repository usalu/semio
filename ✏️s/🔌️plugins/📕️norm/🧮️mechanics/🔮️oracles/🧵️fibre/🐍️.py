"""🧵️ Third-party oracle for the fibre section kernel: `concreteproperties` integrates the same rectangular reinforced concrete section (parabola-rectangle concrete without tension, elastic perfectly plastic steel, bars displacing the concrete) by its own triangle mesh, ultimate-capacity search and moment-curvature analysis.

Prints `case.quantity value` lines in kN m and 1/m: the ultimate moment `m_u` and curvature `phi_u` (concrete strain limit over the neutral axis depth) from the ultimate capacity search, and the moment `m_k<i>` at each curvature of `CURVATURES` from the fixed-step moment-curvature curve.
"""
import warnings

from concreteproperties import Concrete, ConcreteSection, SteelBar
from concreteproperties.pre import add_bar
from concreteproperties.stress_strain_profile import ConcreteServiceProfile, EurocodeParabolicUltimate, SteelElasticPlastic
from sectionproperties.pre.library import rectangular_section

warnings.simplefilter("ignore")
MM = 1.0e3
SECTION = {"b": 0.3, "h": 0.5, "d": 0.45, "a_s1": 1.8e-3, "a_s2": 0.9e-3, "f_c": 30.0, "f_y": 500.0, "e_s": 200000.0, "eps_c2": 2.0e-3, "eps_cu": 3.5e-3, "n": 2.0, "eps_su": 0.045}
AXIAL_FORCES = {"n0": 0.0, "n300": 300.0e3}
CURVATURES = [0.004, 0.008, 0.016, 0.024]
STEP = 2.0e-6


def section(s: dict[str, float]) -> ConcreteSection:
    """🧱️ The section with the parabola-rectangle law as a 200-segment service profile and as the ultimate profile."""
    points = 200
    strains = [-1.0e-3, 0.0] + [s["eps_c2"] * i / points for i in range(1, points + 1)] + [s["eps_cu"]]
    stresses = [0.0, 0.0] + [s["f_c"] * (1.0 - (1.0 - i / points) ** s["n"]) for i in range(1, points + 1)] + [s["f_c"]]
    service = ConcreteServiceProfile(strains=strains, stresses=stresses, ultimate_strain=s["eps_cu"])
    service.elastic_modulus = s["n"] * s["f_c"] / s["eps_c2"]
    ultimate = EurocodeParabolicUltimate(compressive_strength=s["f_c"], compressive_strain=s["eps_c2"], ultimate_strain=s["eps_cu"], n=s["n"])
    concrete = Concrete(name="C", density=2.4e-6, stress_strain_profile=service, colour="lightgrey", ultimate_stress_strain_profile=ultimate, flexural_tensile_strength=0.0)
    steel = SteelBar(name="S", density=7.85e-6, stress_strain_profile=SteelElasticPlastic(yield_strength=s["f_y"], elastic_modulus=s["e_s"], fracture_strain=s["eps_su"]), colour="grey")
    b, h, d = s["b"] * MM, s["h"] * MM, s["d"] * MM
    geometry = add_bar(rectangular_section(b=b, d=h, material=concrete), area=s["a_s1"] * MM * MM, material=steel, x=b / 2, y=h - d, n=4)
    return ConcreteSection(add_bar(geometry, area=s["a_s2"] * MM * MM, material=steel, x=b / 2, y=d, n=4))


def main() -> None:
    cs, out = section(SECTION), []
    for name, n in AXIAL_FORCES.items():
        ultimate = cs.ultimate_bending_capacity(theta=0.0, n=n)
        out.append(f"{name}.m_u {float(abs(ultimate.m_xy)) / MM / MM!r}")
        out.append(f"{name}.phi_u {float(SECTION['eps_cu'] / (ultimate.d_n / MM))!r}")
        curve = cs.moment_curvature_analysis(theta=0.0, n=n, kappa_inc=STEP, kappa_mult=1.0, kappa_inc_max=STEP, progress_bar=False)
        kappas, moments = [float(k) * MM for k in curve.kappa], [float(abs(m)) / MM / MM for m in curve.m_xy]
        for i, target in enumerate(CURVATURES):
            j = max(k for k, value in enumerate(kappas) if value <= target)
            t = (target - kappas[j]) / (kappas[j + 1] - kappas[j])
            out.append(f"{name}.m_k{i} {moments[j] + t * (moments[j + 1] - moments[j])!r}")
    print("\n".join(out))


if __name__ == "__main__":
    main()
