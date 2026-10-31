import json, sys, glob
sys.path.insert(0, '.')
from lib import *
S2 = S + '/🗂️catalog'
files = glob.glob(S2 + '/🛋️furniture/*/🔣️.json') + glob.glob(S2 + '/🪑️objects/*/🔣️.json')
bad = 0
for f in sorted(files):
    a = json.load(open(f))['asset']
    rig = Rig(a['rig']['bones'])
    bones = set(rig.bones)
    regs = {r['id'] for r in a['thermal']['regions']}
    emis = {e['id'] for e in a['thermal']['emitters']}
    ops = {o['id'] for o in a['thermal']['operating']}
    d = a['dimensions']
    probs = []
    for k in ('500', '200', '100', '50'):
        r = a['representations'][k]
        if r['status'] != 'authored':
            continue
        sh = r['geometry']['elevation']
        ids = [s['id'] for s in sh]
        if len(ids) != len(set(ids)): probs.append(f'{k} dup ids')
        used = set()
        for s in sh:
            if s['bone'] not in bones: probs.append(f'{k} bone {s["bone"]}')
            for rr in ([s['region']] if 'region' in s else []) + s.get('regions', []):
                if rr not in regs: probs.append(f'{k} region {rr}')
                used.add(rr)
            if 'region' not in s and 'regions' not in s: probs.append(f'{k} unbound {s["id"]}')
            if s.get('hatch') and s['hatch'] not in {h['id'] for h in r['hatches']}: probs.append(f'{k} hatch')
            if r['pens'].get(s['role'], 1) is None and 'penMm' not in s: probs.append(f'{k} role {s["role"]} no pen {s["id"]}')
        x0, y0, x1, y1 = bbox_world(rig, sh)
        w, h = x1 - x0, y1 - max(0, y0) if y0 < 0.02 else y1 - y0
        if abs(h - d['heightM']) > max(0.02 * d['heightM'], 0.015): probs.append(f'{k} height {h:.3f} vs {d["heightM"]}')
        if abs(w - d['widthM']) > max(0.02 * d['widthM'], 0.015): probs.append(f'{k} width {w:.3f} vs {d["widthM"]}')
        print(a['id'], k, 'regions drawn', sorted(used))
    for p in a['poses']:
        t = p['thermal']
        if set(t['regions']) != regs: probs.append(f'pose {p["id"]} regions')
        if set(t['emitters']) != emis: probs.append(f'pose {p["id"]} emitters')
        if set(t.get('operating', {})) != ops: probs.append(f'pose {p["id"]} operating')
    print('==', a['id'], 'OK' if not probs else probs)
    bad += bool(probs)
print('bad', bad)
