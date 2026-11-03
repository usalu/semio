import json, math, pathlib
from shapely import Polygon
from structuralcodes.geometry import SurfaceGeometry, add_reinforcement
from structuralcodes.materials.concrete import ConcreteEC2_2004
from structuralcodes.materials.reinforcement import ReinforcementEC2_2004
from structuralcodes.sections import GenericSection
V = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧫️vectors/catalogue.json')
def centred(pts, bars):
    a = cy = cz = 0.0
    for i in range(len(pts)):
        (y0, z0), (y1, z1) = pts[i], pts[(i + 1) % len(pts)]
        cr = y0 * z1 - y1 * z0
        a += cr; cy += (y0 + y1) * cr; cz += (z0 + z1) * cr
    a /= 2; cy /= 6 * a; cz /= 6 * a
    return [(y - cy, z - cz) for y, z in pts], [(y - cy, z - cz, d) for y, z, d in bars]
SHAPES = {
    'T': ([(-150, 0), (150, 0), (150, 450), (500, 450), (500, 600), (-500, 600), (-500, 450), (-150, 450)], [(-100, 50, 25), (0, 50, 25), (100, 50, 25)]),
    'L': ([(0, 0), (250, 0), (250, 380), (500, 380), (500, 500), (0, 500)], [(60, 50, 22), (125, 50, 22), (190, 50, 22)]),
}
cases = []
for name, (pts, bars) in SHAPES.items():
    pts, bars = centred(pts, bars)
    c = ConcreteEC2_2004(fck=30, gamma_c=1.5, alpha_cc=1.0)
    s = ReinforcementEC2_2004(fyk=500, Es=200000, ftk=540, epsuk=0.075, gamma_s=1.15)
    geo = SurfaceGeometry(Polygon(pts), c)
    for y, z, d in bars: geo = add_reinforcement(geo, (y, z), d, s)
    sec = GenericSection(geo)
    res = sec.section_calculator.calculate_moment_curvature(theta=0, n=0)
    kappa = [abs(float(k)) * 1000.0 for k in res.chi_y]
    base = {'annex': 'En', 'num': {'fck': 30.0, 'fyk': 500.0, 'eps_uk': 75.0, 'n_ed': 0.0, 'points': 4.0}, 'text': {'oracle': 'structuralcodes', 'shape': name}, 'list': {'poly_y': [p[0] for p in pts], 'poly_z': [p[1] for p in pts], 'bar_y': [b[0] for b in bars], 'bar_z': [b[1] for b in bars], 'bar_a': [math.pi * b[2] ** 2 / 4 for b in bars]}, 'tol': 0.01}
    cases.append({'id': 'S-04', **base})
    k = dict(base); k['list'] = {**base['list'], 'kappa': kappa[4:]}
    cases.append({'id': 'S-11', **k})
d = json.loads(V.read_text())
d['cases'] = [c for c in d['cases'] if c.get('text', {}).get('oracle') != 'structuralcodes'] + cases
V.write_text(json.dumps(d, indent=1, ensure_ascii=False))
print(len(cases), [len(c['list'].get('kappa', [])) for c in cases])
