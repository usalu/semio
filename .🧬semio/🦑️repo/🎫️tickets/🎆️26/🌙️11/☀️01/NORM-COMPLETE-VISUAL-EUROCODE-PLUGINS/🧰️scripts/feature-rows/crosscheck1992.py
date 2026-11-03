"""Third-party cross-check of the EN 1992 oracle values behind the new feature rows: M_Rd of the beam section (structuralcodes GenericSection), mid-span deflection of the derived load (anastruct), shear without links (structuralcodes). Run with .venv/bin/python -I."""
import importlib.util, json, pathlib, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any")
spec = importlib.util.spec_from_file_location("o", ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py")
o = importlib.util.module_from_spec(spec); sys.modules["o"] = o; spec.loader.exec_module(o)
from shapely import Polygon
from structuralcodes.geometry import SurfaceGeometry, add_reinforcement_line
from structuralcodes.materials.concrete import ConcreteEC2_2004
from structuralcodes.materials.reinforcement import ReinforcementEC2_2004
from structuralcodes.sections import GenericSection
from anastruct import SystemElements

doc = json.load(open(ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome/beam-sls-pass/🔣️.json"))
m = doc["members"][0]
p = o.NDP[doc["annex"].lower()]
fck = doc["concreteGrades"][0]["fCk"] / 1e6
combos = o.combinations(m)
uls = o.governing(combos["uls"])
b, h, d = m["width"] * 1e3, m["height"] * 1e3, m["effectiveDepth"] * 1e3
bar = m["longitudinal"][0]
geo = SurfaceGeometry(Polygon([(-b / 2, -h / 2), (b / 2, -h / 2), (b / 2, h / 2), (-b / 2, h / 2)]), ConcreteEC2_2004(fck=fck, alpha_cc=p["alpha_cc"], gamma_c=p["gamma_c"]))
cover = h - d
geo = add_reinforcement_line(geo, (-b / 2 + 50, -h / 2 + cover), (b / 2 - 50, -h / 2 + cover), bar["diameter"] * 1e3, ReinforcementEC2_2004(fyk=500.0, Es=200000.0, density=7850, ftk=540.0, epsuk=0.05, gamma_s=1.15), n=bar["count"])
theirs = abs(GenericSection(geo).section_calculator.calculate_bending_strength(n=-uls[1]).m_y) / 1e6
ours_util = dict((i, u) for i, u in o.evaluate(doc)["checks"] and [(c["id"], c["utilization"]) for c in o.evaluate(doc)["checks"]])
flex_util = ours_util["en1992.6.1.flexure.beam-B1"]
their_util = abs(uls[0]) / 1e3 / theirs
print("flexure utilisation oracle", flex_util, "structuralcodes", their_util, "rel", abs(flex_util - their_util) / their_util)
assert abs(flex_util - their_util) <= 1e-2 * their_util
q = 1.0
ss = SystemElements(EA=1e9, EI=1.0)
ss.add_element([[0, 0], [m["span"] / 2, 0]]); ss.add_element([[m["span"] / 2, 0], [m["span"], 0]])
ss.add_support_hinged(1); ss.add_support_roll(3); ss.q_load(-1.0, element_id=[1, 2])
ss.solve()
mid = abs(ss.get_node_results_system(2)["uy"])
mine = o.elastic_line("SimplySupported", m["span"], 1.0, 0.0, 0.5)
print("midspan deflection x EI, unit load: oracle", mine, "anastruct", mid)
assert abs(mine - mid) <= 1e-4 * mid
fctm = ConcreteEC2_2004(fck=fck, alpha_cc=p["alpha_cc"], gamma_c=p["gamma_c"]).fctm
a_min = max(0.26 * fctm / 500.0 * b * d, 0.0013 * b * d)
a_s = bar["count"] * 3.141592653589793 * (bar["diameter"] * 1e3) ** 2 / 4
print("minas oracle", ours_util["en1992.9.minas.beam-B1"], "structuralcodes fctm", a_min / a_s)
assert abs(ours_util["en1992.9.minas.beam-B1"] - a_min / a_s) <= 1e-2 * a_min / a_s
