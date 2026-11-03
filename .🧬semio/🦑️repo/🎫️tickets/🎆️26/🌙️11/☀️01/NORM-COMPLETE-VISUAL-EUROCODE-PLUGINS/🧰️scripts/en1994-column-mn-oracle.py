"""Third-party N-M cross-check of the EN 1994 composite column interaction curve: python -I en1994-column-mn-oracle.py

The concrete-filled equivalent tube (no bars) is modelled in `concreteproperties` (parabola-rectangle concrete, elastic perfectly plastic steel, plane sections, compression positive for `n`). Its ultimate bending capacity at several axial forces must agree with the independent strain-compatibility curve of the document oracle (`strain_curve`, 5 % near pure compression where the mesh and the stress-strain tables differ), and both stay below the exact rigid-plastic curve (the upper bound of every strain plane), while the polygon A-C-D-B of EN 1994-1-1 6.7.3.2 (chords between the plastic points) lies inside that curve. Every committed column fixture of the oracle documents must keep that bound at its design point.
"""
import importlib.util, json, math, pathlib

SUBSET = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any")
spec = importlib.util.spec_from_file_location("columns", SUBSET / "🔮️oracles/🚦️check-outcome/🏛️columns/🐍️.py")
columns = importlib.util.module_from_spec(spec)
spec.loader.exec_module(columns)


def concreteproperties_tube(b_mm, a_s_mm2, fy, fck, gamma_c=1.5):
    from concreteproperties import Concrete, ConcreteSection, Steel
    from concreteproperties.stress_strain_profile import ConcreteLinearNoTension, EurocodeParabolicUltimate, SteelElasticPlastic
    from sectionproperties.pre.library import rectangular_hollow_section, rectangular_section

    t = (b_mm - math.sqrt(b_mm * b_mm - a_s_mm2)) / 2
    fcd = fck / gamma_c
    concrete = Concrete(name="C", density=2.4e-6, stress_strain_profile=ConcreteLinearNoTension(elastic_modulus=33e3, ultimate_strain=0.0035, compressive_strength=fcd), ultimate_stress_strain_profile=EurocodeParabolicUltimate(compressive_strength=fcd, compressive_strain=0.002, ultimate_strain=0.0035, n=2.0), flexural_tensile_strength=0.0, colour="lightgrey")
    steel = Steel(name="S", density=7.85e-6, stress_strain_profile=SteelElasticPlastic(yield_strength=fy, elastic_modulus=210e3, fracture_strain=0.5), colour="grey")
    tube = rectangular_hollow_section(d=b_mm, b=b_mm, t=t, r_out=0.0, n_r=1, material=steel)
    core = rectangular_section(d=b_mm - 2 * t, b=b_mm - 2 * t, material=concrete).shift_section(t, t)
    return ConcreteSection(tube + core)


def main():
    col = {"outerSizeM": 0.3, "steelAM2": 0.0084, "reinforcementAsM2": 0.0}
    section = concreteproperties_tube(300.0, 8400.0, 355.0, 30.0)
    ours, _ = columns.tube(col, 355e6, 30e6 / 1.5, 0.0)
    curve, polygon, plastic = columns.strain_curve(ours), ours.polygon(), columns.plastic_curve(ours)
    rows = []
    for n in (0.5e6, 1.0e6, 1.5e6, 2.5e6, 3.5e6, 4.2e6):
        theirs = section.ultimate_bending_capacity(theta=0, n=n).m_xy * 1e-6
        mine = columns.moment_at(curve, n) / 1e3
        bound = columns.moment_at(plastic, n) / 1e3
        chords = columns.mu_d(polygon, n) * polygon[3][1] / 1e3
        assert abs(mine - theirs) <= 5e-2 * theirs + 1.0, (n, mine, theirs)
        assert mine <= bound * 1.01 and theirs <= bound * 1.01 and chords <= bound * 1.01, (n, mine, theirs, bound, chords)
        rows.append({"nKN": n / 1e3, "concreteproperties": round(theirs, 3), "strainCompatibility": round(mine, 3), "plastic": round(bound, 3), "polygon": round(chords, 3)})
    fixtures = {}
    for path in sorted((SUBSET / "🧪️tests/🧫️fixtures/📄️oracle-documents").glob("column-*/🔣️.json")):
        n_ed, m_sc, m_pl, m_poly = columns.strain_compatibility_bound(json.loads(path.read_text(encoding="utf8"))["columns"][0])
        assert m_sc <= m_pl * 1.01 and m_poly <= m_pl * 1.01, (path.parent.name, m_sc, m_pl, m_poly)
        fixtures[path.parent.name] = {"nEdKN": n_ed / 1e3, "strainCompatibilityKNm": round(m_sc / 1e3, 3), "plasticKNm": round(m_pl / 1e3, 3), "polygonKNm": round(m_poly / 1e3, 3)}
    print(json.dumps({"tube": rows, "fixtures": fixtures}, indent=1))


if __name__ == "__main__":
    main()
