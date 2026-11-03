import json, math, pathlib
V = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧫️vectors/catalogue.json')
A25, A20 = math.pi * 25 ** 2 / 4, math.pi * 20 ** 2 / 4
T = [(-150, 0), (150, 0), (150, 450), (500, 450), (500, 600), (-500, 600), (-500, 450), (-150, 450)]
RECT = [(-150, -250), (150, -250), (150, 250), (-150, 250)]
OUTER, INNER = [(-200, -300), (200, -300), (200, 300), (-200, 300)], [(-150, -250), (150, -250), (150, 250), (-150, 250)]
def lists(poly, bars, hole=None):
    out = {'poly_y': [p[0] for p in poly], 'poly_z': [p[1] for p in poly], 'bar_y': [b[0] for b in bars], 'bar_z': [b[1] for b in bars], 'bar_a': [b[2] for b in bars]}
    if hole: out.update({'hole_y': [p[0] for p in hole], 'hole_z': [p[1] for p in hole]})
    return out
tbars = [(-100, 50, A25), (0, 50, A25), (100, 50, A25)]
rbars = [(y, -200, A20) for y in (-105, -35, 35, 105)]
cases = [
    {'id': 'SG-01', 'annex': 'De', 'num': {'fck': 30.0, 'c': 30.0, 'phi_l': 25.0}, 'text': {'oracle': 'sectionproperties+shapely', 'shape': 'T'}, 'list': lists(T, tbars), 'tol': 1e-3},
    {'id': 'SG-01', 'annex': 'De', 'num': {'fck': 30.0, 'c': 15.0, 'phi_l': 10.0}, 'text': {'oracle': 'sectionproperties+shapely', 'shape': 'box'}, 'list': lists(OUTER, [(0, -270, A20)], INNER), 'tol': 1e-3},
    {'id': 'SG-02', 'annex': 'De', 'num': {'fck': 30.0, 'm_ed': 150.0e6, 'n_ed': 0.0}, 'text': {'oracle': 'concreteproperties', 'shape': 'T'}, 'list': lists(T, tbars), 'tol': 2e-3},
    {'id': 'SG-02', 'annex': 'De', 'num': {'fck': 30.0, 'm_ed': 100.0e6, 'n_ed': 0.0}, 'text': {'oracle': 'concreteproperties', 'shape': 'rectangle'}, 'list': lists(RECT, rbars), 'tol': 2e-3},
    {'id': 'R-04', 'annex': 'De', 'num': {'w': 100.0, 'theta_m': 300.0}, 'text': {'oracle': 'numpy'}, 'list': {'zone_theta': [760.0, 640.0, 520.0, 420.0, 340.0]}, 'tol': 1e-6},
]
d = json.loads(V.read_text())
d['cases'] = [c for c in d['cases'] if not (c['id'] in ('SG-01', 'SG-02', 'R-04'))] + cases
V.write_text(json.dumps(d, indent=1, ensure_ascii=False))
print(len(d['cases']))
