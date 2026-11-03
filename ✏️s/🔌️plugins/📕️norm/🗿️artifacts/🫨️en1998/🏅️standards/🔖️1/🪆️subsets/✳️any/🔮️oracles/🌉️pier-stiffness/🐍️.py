"""🌉️ Third-party oracle for the computed pier stiffness EI_eff = M_y/phi_y (EN 1998-2 2.3.6.1, Annex C): `concreteproperties` integrates the same rectangular pier section (parabola-rectangle concrete without tension, elastic perfectly plastic steel, two outer layers of bars) under the axial force N and the first yield of the tension steel is found on its fixed-step moment-curvature curve from the service stress of the lumped bars.

Prints `case.quantity value` lines in N m and 1/m: `phi_y`, `m_y` and `ei_eff`. The cases keep the steel yield governing (the kernel's yield is the first of tension steel at f_y/E_s and the extreme concrete at eps_c2).
"""
import importlib.util
import pathlib
import warnings

from concreteproperties import Concrete, ConcreteSection, SteelBar
from concreteproperties.pre import add_bar
from concreteproperties.stress_strain_profile import ConcreteServiceProfile, EurocodeParabolicUltimate, SteelElasticPlastic
from sectionproperties.pre.library import rectangular_section

warnings.simplefilter("ignore")
MM = 1.0e3
STEEL = {"f_y": 500.0, "e_s": 200000.0, "eps_c2": 2.0e-3, "eps_cu": 3.5e-3, "n": 2.0, "eps_su": 0.045}
CASES = {
    "pier_n1000": {"b": 0.8, "h": 1.2, "cover": 0.08, "rho": 0.008, "f_c": 30.0, "n_ed": 1.0e6},
    "pier_n0": {"b": 0.8, "h": 1.2, "cover": 0.08, "rho": 0.008, "f_c": 30.0, "n_ed": 0.0},
}
STEP = 1.0e-6


def section(c: dict[str, float]) -> ConcreteSection:
    """🧱️ The pier section with the parabola-rectangle law as a 200-segment service profile and as the ultimate profile."""
    s, points = STEEL, 200
    strains = [-1.0e-3, 0.0] + [s["eps_c2"] * i / points for i in range(1, points + 1)] + [s["eps_cu"]]
    stresses = [0.0, 0.0] + [c["f_c"] * (1.0 - (1.0 - i / points) ** s["n"]) for i in range(1, points + 1)] + [c["f_c"]]
    service = ConcreteServiceProfile(strains=strains, stresses=stresses, ultimate_strain=s["eps_cu"])
    service.elastic_modulus = s["n"] * c["f_c"] / s["eps_c2"]
    ultimate = EurocodeParabolicUltimate(compressive_strength=c["f_c"], compressive_strain=s["eps_c2"], ultimate_strain=s["eps_cu"], n=s["n"])
    concrete = Concrete(name="C", density=2.4e-6, stress_strain_profile=service, colour="lightgrey", ultimate_stress_strain_profile=ultimate, flexural_tensile_strength=0.0)
    steel = SteelBar(name="S", density=7.85e-6, stress_strain_profile=SteelElasticPlastic(yield_strength=s["f_y"], elastic_modulus=s["e_s"], fracture_strain=s["eps_su"]), colour="grey")
    b, h, cover = c["b"] * MM, c["h"] * MM, c["cover"] * MM
    area = 0.5 * c["rho"] * b * h
    geometry = add_bar(rectangular_section(b=b, d=h, material=concrete), area=area, material=steel, x=b / 2, y=cover, n=4)
    return ConcreteSection(add_bar(geometry, area=area, material=steel, x=b / 2, y=h - cover, n=4))


def tension_strain(cs: ConcreteSection, curve, index: int) -> float:
    """🔩 Largest bar tension strain (positive) at the curve point `index`."""
    result = cs.calculate_service_stress(curve, m=float(curve.m_xy[index]), kappa=float(curve.kappa[index]))
    return max(-float(strain) for strains in result.lumped_reinforcement_strains for strain in (strains if hasattr(strains, "__iter__") else [strains]))


def yield_point(cs: ConcreteSection, n: float) -> tuple[float, float]:
    """🎯 First yield (phi_y in 1/m, M_y in N m) of the tension steel: bisection over the monotone curve, linear in between."""
    eps_y = STEEL["f_y"] / STEEL["e_s"]
    curve = cs.moment_curvature_analysis(theta=0.0, n=n, kappa_inc=STEP, kappa_mult=1.0, kappa_inc_max=STEP, progress_bar=False)
    kappas, moments = [float(k) * MM for k in curve.kappa], [abs(float(m)) / MM for m in curve.m_xy]
    lo, hi = 0, len(kappas) - 1
    while hi - lo > 1:
        mid = (lo + hi) // 2
        lo, hi = (mid, hi) if tension_strain(cs, curve, mid) < eps_y else (lo, mid)
    e_lo, e_hi = tension_strain(cs, curve, lo), tension_strain(cs, curve, hi)
    t = (eps_y - e_lo) / (e_hi - e_lo)
    return kappas[lo] + t * (kappas[hi] - kappas[lo]), moments[lo] + t * (moments[hi] - moments[lo])


def main() -> None:
    out = []
    for name, case in CASES.items():
        phi, m = yield_point(section(case), case["n_ed"])
        out += [f"{name}.phi_y {phi!r}", f"{name}.m_y {m!r}", f"{name}.ei_eff {m / phi!r}"]
    print("\n".join(out))


if __name__ == "__main__":
    main()
