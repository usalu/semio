"""sectionproperties reference values for the blessed cases (run with the repo venv, -I)."""
import json
import sys
import warnings

sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from cases import CASES
from shapely.geometry import Polygon
from sectionproperties.pre.geometry import Geometry
from sectionproperties.analysis.section import Section
from sectionproperties.pre import Material

warnings.simplefilter("ignore")


def reference(outer, holes, nu=0.0, mesh=None):
    poly = Polygon(outer, holes)
    area = poly.area
    mesh = mesh or area / 4000
    if nu:
        mat = Material("m", 2.0 * (1 + nu) * 1e9 / 2.0 * 1.0, nu, 1.0, 1e6, 1.0)
        mat = Material(name="m", elastic_modulus=1e9 * 2 * (1 + nu) / 2 * 1.0, poissons_ratio=nu, yield_strength=1e6, density=1.0, color="w")
        g = Geometry(poly, material=mat)
    else:
        g = Geometry(poly)
    g.create_mesh(mesh_sizes=[mesh])
    s = Section(g)
    s.calculate_geometric_properties()
    s.calculate_warping_properties()
    if nu:
        e = 1e9 * (1 + nu)
        a_s = s.get_eas(e_ref=e)
        return dict(area=s.get_ea(e_ref=e), c=s.get_c(), j=s.get_ej(e_ref=e), sc=s.get_sc(), gamma=s.get_egamma(e_ref=e), a_s=a_s, ic=s.get_eic(e_ref=e), elements=len(s.elements))
    return dict(area=s.get_area(), c=s.get_c(), j=s.get_j(), sc=s.get_sc(), gamma=s.get_gamma(), a_s=s.get_as(), ic=s.get_ic(), elements=len(s.elements))


if __name__ == "__main__":
    out = {}
    for name, (outer, holes) in CASES.items():
        out[name] = reference(outer, holes)
        print(name, out[name], flush=True)
