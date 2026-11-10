import importlib.util
import json
import os

ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any'
spec = importlib.util.spec_from_file_location('oracle', f'{ART}/🔮️oracles/🌐️cross-checks/🐍️.py')
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)


def layer(d, lam, mu):
    return {'thicknessM': d, 'lambda': lam, 'mu': mu}


TIGHT = [layer(0.0125, 0.25, 1), layer(0.2, 0.035, 1.3), layer(0.02, 0.13, 1000)]
CASES = [
    ('etics-no-condensation', [layer(0.015, 0.7, 10), layer(0.24, 0.81, 10), layer(0.14, 0.035, 40), layer(0.008, 0.7, 20)], (12.0, 0.7)),
    ('vapour-tight-outside-fails-drying', TIGHT, (12.0, 0.7)),
    ('vapour-tight-outside-dries-in-hot-summer', TIGHT, (25.0, 0.5)),
]
WINTER = {'tIntC': 20.0, 'rhInt': 0.5, 'tExtC': -5.0, 'rhExt': 0.8, 'hours': 2160}
rows = []
for name, layers, (t, rh) in CASES:
    summer = {'tIntC': t, 'rhInt': rh, 'tExtC': t, 'rhExt': rh, 'hours': 2160}
    ref = oracle.glaser_standard([dict(L, segments=[]) for L in layers], 0.13, 0.04, winter=(20.0, 0.5, -5.0, 0.8), summer=(t, rh, t, rh))
    rows.append({'id': name, 'rSi': 0.13, 'rSe': 0.04, 'winter': WINTER, 'summer': summer, 'layers': layers, 'condensedKgM2': ref['condensedKgM2'], 'evaporatedKgM2': ref['evaporatedKgM2'], 'dries': ref['dries']})
out = f'{ART}/🧪️tests/🧫️fixtures/🧮️glaser-drying/🔣️.json'
os.makedirs(os.path.dirname(out), exist_ok=True)
json.dump({'oracle': 'ISO 13788 condensation-plane iteration, DIN 4108-3 Annex A periods, flux g = (p_i - p_c) / Z, Z = s_d / delta0, seconds', 'toleranceRelative': 1e-6, 'cases': rows}, open(out, 'w', encoding='utf8'), indent=2, ensure_ascii=False)
print(out)
