import importlib.util
import json
import sys

ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any'
spec = importlib.util.spec_from_file_location('oracle', f'{ART}/🔮️oracles/⚖️compliance/🐍️.py')
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)


def layer(d, lam, mu):
    return {'thicknessM': d, 'lambda': lam, 'mu': mu}


CASES = [
    ('etics-no-condensation', [layer(0.015, 0.7, 10), layer(0.24, 0.81, 10), layer(0.14, 0.035, 40), layer(0.008, 0.7, 20)], (-5.0, 0.8), (12.0, 0.7)),
    ('vapour-tight-outside-fails-drying', [layer(0.0125, 0.25, 1), layer(0.2, 0.035, 1.3), layer(0.02, 0.13, 1000)], (-5.0, 0.8), (12.0, 0.7)),
    ('vapour-tight-outside-dries-in-hot-summer', [layer(0.0125, 0.25, 1), layer(0.2, 0.035, 1.3), layer(0.02, 0.13, 1000)], (-5.0, 0.8), (25.0, 0.5)),
]
rows = []
for name, layers, winter, summer in CASES:
    condensed, evaporated = oracle.glaser_masses(layers, 0.13, 0.04, 20.0, 0.5, winter, summer, 90 * 24, oracle.saturation_pa)
    rows.append({'id': name, 'rSi': 0.13, 'rSe': 0.04, 'tIntC': 20.0, 'rhInt': 0.5, 'winter': {'tC': winter[0], 'rh': winter[1]}, 'summer': {'tC': summer[0], 'rh': summer[1]}, 'hours': 2160, 'layers': layers, 'condensedKgM2': condensed, 'evaporatedKgM2': evaporated, 'dries': evaporated >= condensed})
out = f'{ART}/🧪️tests/🧫️fixtures/🧮️glaser-drying/🔣️.json'
import os
os.makedirs(os.path.dirname(out), exist_ok=True)
json.dump({'oracle': 'numpy nodal solve, DIN EN ISO 13788 saturation pressure', 'toleranceRelative': 0.02, 'cases': rows}, open(out, 'w', encoding='utf8'), indent=2, ensure_ascii=False)
print(out)
