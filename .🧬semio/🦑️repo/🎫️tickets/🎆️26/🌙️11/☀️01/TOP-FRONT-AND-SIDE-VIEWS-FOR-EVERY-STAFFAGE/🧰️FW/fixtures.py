import json, sys, copy
FX = 'C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/'

def load(name):
    return json.load(open(FX + name + '/🔣️.json', encoding='utf8'))

def save(name, d):
    open(FX + name + '/🔣️.json', 'w', encoding='utf8', newline='\n').write(json.dumps(d, indent=2, ensure_ascii=False) + '\n')

def mk(meta, id, **geom):
    base = {k: meta[id][k] for k in ('id', 'bone', 'role', 'z', 'filled', 'tone', 'region', 'material', 'hatch') if k in meta[id]}
    base.update(geom)
    return base

def human(rep, scale):
    meta = {s['id']: s for s in rep['geometry']['elevation']}
    c = {500: 0.02, 200: 0.01, 100: 0.0, 50: 0.0}[scale]
    q = lambda v: round(v, 3)
    front, plan = [], []
    def add(view, id, **g):
        if id in meta:
            view.append(mk(meta, id, **g))
    add(front, 'head', kind='ellipse', cx=0, cy=0.65, rx=q(0.085 + c / 2), ry=q(0.115))
    add(front, 'neck', kind='capsule', x0=0, y0=0.53, x1=0, y1=0.58, r0=q(0.05 + c / 2), r1=q(0.05 + c / 2))
    add(front, 'torso', kind='rect', x=-0.19, y=0.1, w=0.38, h=0.45, r=0.03, followTip=True)
    add(front, 'pelvis', kind='rect', x=-0.17, y=-0.14, w=0.34, h=0.24, r=0.04)
    for side, sg, name in ((1, 1, 'l'), (-1, -1, 'r')):
        arm = q(0.09 + c)
        add(front, 'arm-' + name, kind='rect', x=q(0.18 if side > 0 else -0.18 - arm), y=-0.08, w=arm, h=0.6, r=0.02, followTip=True)
        thigh = q(0.16 + c)
        add(front, 'thigh-' + name, kind='rect', x=q(0.02 if side > 0 else -0.02 - thigh), y=-0.45, w=thigh, h=0.45, r=0.03, followTip=True)
        shank = q(0.12 + c)
        add(front, 'shank-' + name, kind='rect', x=q(0.03 if side > 0 else -0.03 - shank), y=-0.88, w=shank, h=0.43, r=0.02, followTip=True)
        foot = q(0.11 + c)
        add(front, 'foot-' + name, kind='rect', x=q(0.02 if side > 0 else -0.02 - foot), y=-0.96, w=foot, h=0.08, r=0.02)
        add(plan, 'arm-' + name, kind='capsule', x0=0, y0=0.225 * side, x1=0, y1=0.225 * side, r0=q(0.045 + c), r1=q(0.035 + c), followTip=True)
        add(plan, 'thigh-' + name, kind='capsule', x0=0, y0=0.09 * side, x1=0, y1=0.09 * side, r0=q(0.085 + c), r1=q(0.065 + c), followTip=True)
        add(plan, 'shank-' + name, kind='capsule', x0=0, y0=0.09 * side, x1=0, y1=0.09 * side, r0=q(0.06 + c), r1=q(0.04 + c), followTip=True)
        add(plan, 'foot-' + name, kind='capsule', x0=-0.015, y0=0.09 * side, x1=0.165, y1=0.09 * side, r0=q(0.045 + c / 2), r1=q(0.045 + c / 2))
    add(plan, 'head', kind='ellipse', cx=0, cy=0, rx=q(0.095 + c / 2), ry=q(0.085 + c / 2))
    add(plan, 'neck', kind='ellipse', cx=0, cy=0, rx=q(0.05 + c / 2), ry=q(0.05 + c / 2))
    add(plan, 'torso', kind='ellipse', cx=0, cy=0, rx=q(0.15 + c), ry=0.19)
    add(plan, 'pelvis', kind='ellipse', cx=0, cy=0, rx=q(0.14 + c), ry=0.17)
    for v in (front, plan):
        v.sort(key=lambda s: -s['z'] if False else 0)
    return front, plan

def depth_order(shapes, view):
    pass

for name in ['👤️sample-figure', '🚶️reference-walker', '🦶️sparse-smooth-walk']:
    d = load(name)
    a = d.get('asset', d)
    a['presentation'].pop('viewpoints', None)
    for key, rep in a['representations'].items():
        if rep['status'] != 'authored':
            continue
        front, plan = human(rep, int(key))
        g = rep['geometry']
        rep['geometry'] = {'elevation': g['elevation'], 'front': front, 'plan': plan}
        for dec in ('silhouette',):
            rep['decisions'][dec]['en'] += ' Front and plan are drawn from the same bones: limbs stretch with their bones.'
            rep['decisions'][dec]['de'] += ' Vorderansicht und Grundriss folgen denselben Knochen: Gliedmaßen strecken sich mit ihren Knochen.'
    save(name, d)

d = load('🔄️continuous-rotation')
a = d['asset']
a['presentation'].pop('viewpoints', None)
for key, rep in a['representations'].items():
    if rep['status'] != 'authored':
        continue
    scale = int(key)
    half = {500: 0.05, 200: 0.049, 100: 0.048, 50: 0.047}[scale]
    meta = {s['id']: s for s in rep['geometry']['elevation']}
    front = [mk(meta, 'stand', kind='rect', x=-half, y=0, w=round(2 * half, 3), h=0.05)]
    plan = [mk(meta, 'stand', kind='rect', x=-0.1, y=-half, w=0.2, h=round(2 * half, 3))]
    for view in (front, plan):
        pass
    rim = mk(meta, 'rim', kind='ellipse', cx=0, cy=0, rx=meta['rim']['rx'], ry=meta['rim']['ry'])
    front.append({**rim, 'solid': {'kind': 'extruded', 'z0': -half, 'z1': half}})
    plan.append({**rim, 'solid': {'kind': 'extruded', 'z0': -half, 'z1': half}})
    if 'spoke' in meta:
        sp = mk(meta, 'spoke', kind='poly', points=meta['spoke']['points'], closed=False)
        front.append({**sp, 'solid': {'kind': 'extruded', 'z0': -0.01, 'z1': 0.01}})
        plan.append({**sp, 'solid': {'kind': 'extruded', 'z0': -0.01, 'z1': 0.01}})
    g = rep['geometry']
    rep['geometry'] = {'elevation': g['elevation'], 'front': front, 'plan': plan}
save('🔄️continuous-rotation', d)
print('ok')
