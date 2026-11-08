import json
import os

import numpy as np

ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any'


def layer(d, lam, segments=None):
    return {'thicknessM': d, 'lambda': lam, 'segments': segments or []}


def homogeneous(layers, r_si, r_se):
    return r_si + r_se + float(np.sum([l['thicknessM'] / l['lambda'] for l in layers]))


def combined(layers, r_si, r_se):
    """EN ISO 6946 6.7: R_T = (R_upper + R_lower) / 2 for one inhomogeneous layer; upper bound by the parallel paths, lower bound by the isothermal planes."""
    rows = [l for l in layers if l['segments']]
    if not rows:
        return homogeneous(layers, r_si, r_se)
    primary = rows[0]
    fractions = np.array([s['fraction'] for s in primary['segments']])
    paths = []
    for j, section in enumerate(primary['segments']):
        total = r_si + r_se
        for l in layers:
            total += (l['thicknessM'] / l['segments'][j]['lambda']) if l['segments'] else l['thicknessM'] / l['lambda']
        paths.append(total)
    upper = 1.0 / float(np.sum(fractions / np.array(paths)))
    lower = r_si + r_se
    for l in layers:
        if l['segments']:
            conductance = float(np.sum([s['fraction'] / (l['thicknessM'] / s['lambda']) for s in l['segments']]))
            lower += 1.0 / conductance
        else:
            lower += l['thicknessM'] / l['lambda']
    return 0.5 * (upper + lower)


def segment(fraction, lam):
    return {'fraction': fraction, 'lambda': lam}


CASES = [
    ('etics-wall', [layer(0.015, 0.7), layer(0.24, 0.81), layer(0.14, 0.035), layer(0.008, 0.7)], 0.13, 0.04),
    ('roof-upward-heat-flow', [layer(0.0125, 0.25), layer(0.2, 0.035), layer(0.16, 2.1)], 0.10, 0.04),
    ('floor-on-ground', [layer(0.05, 1.4), layer(0.10, 0.035), layer(0.20, 2.1)], 0.17, 0.0),
    ('timber-studs-in-insulation', [layer(0.0125, 0.25), layer(0.16, 0.0, [segment(0.88, 0.035), segment(0.12, 0.13)]), layer(0.02, 0.13)], 0.13, 0.04),
]
rows = []
for name, layers, r_si, r_se in CASES:
    r = combined(layers, r_si, r_se)
    rows.append({'id': name, 'rSi': r_si, 'rSe': r_se, 'layers': layers, 'rTotal': r, 'u': 1.0 / r})
out = f'{ART}/🧪️tests/🧫️fixtures/🧮️u-values/🔣️.json'
os.makedirs(os.path.dirname(out), exist_ok=True)
json.dump({'oracle': 'numpy, EN ISO 6946 series and combined method', 'toleranceRelative': 1e-9, 'cases': rows}, open(out, 'w', encoding='utf8'), indent=2, ensure_ascii=False)
print(out)
for r in rows:
    print(r['id'], round(r['rTotal'], 4), round(r['u'], 4))
