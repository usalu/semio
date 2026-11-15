import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-10-standing.py')).read())

SLUG = 'sleeping-person'
NS = {50: dict(head=9, hair=6, neck=3, duv=7, arm=6, fore=6, hand=6, foot=6, toe=3), 100: dict(head=7, hair=4, neck=3, duv=6, arm=5, fore=5, hand=5, foot=5, toe=3), 200: dict(head=5, hair=3, neck=2, duv=5, arm=4, fore=4, hand=4, foot=4, toe=2), 500: dict(head=4, hair=0, neck=2, duv=4, arm=3, fore=3, hand=3, foot=3, toe=2)}


def body(fig):
    sc = fig.scale
    n = NS[sc]
    H = Human(fig, None)
    pk = lambda *a, **k: None
    H.loft('head', 'head', 'skin', 0.27, ['head'], ['head'], ['head'], nst=n['head'], exp=2.2, planfit=0)
    if n['hair']:
        H.loft('hair', 'head', 'hair', 0.7, ['hair'], ['hair'], ['hair'], nst=n['hair'], exp=2.2, grow=0.002, lo=0.2, planfit=0)
    H.loft('neck', 'neck', 'skin', 0.27, ['neck'], ['neck'], ['neck'], nst=n['neck'], exp=2.0, planfit=0)
    for bid, shp in (('torso', 'duvet-chest'), ('pelvis', 'duvet-hips'), ('thigh-r', 'duvet-thighs'), ('shank-r', ['duvet-shins', 'shin-bare'])):
        pats = shp if isinstance(shp, list) else [shp]
        H.loft(shp[0] if isinstance(shp, list) else shp, bid, 'duvet-cover', 0.8, pats, pats, pats, nst=n['duv'], exp=3.5, planfit=0)
    for s in 'lr':
        H.loft(f'arm-{s}', f'arm-{s}', 'cotton-underwear', 0.7, [f'sleeve-{s}'], [f'sleeve-{s}'], [f'sleeve-{s}'], nst=n['arm'], exp=2.1, planfit=0)
        H.loft(f'forearm-{s}', f'forearm-{s}', 'cotton-underwear', 0.7, [f'forearm-{s}'], [f'forearm-{s}'], [f'forearm-{s}'], nst=n['fore'], exp=2.0, planfit=0)
        H.loft(f'hand-{s}', f'hand-{s}', 'skin', 0.27, [f'hand-{s}'], [f'hand-{s}'], [f'hand-{s}'], nst=n['hand'], exp=2.3, planfit=0)
        H.loft(f'foot-{s}', f'foot-{s}', 'skin', 0.27, [f'foot-rear-{s}', f'foot-{s}'], [f'foot-{s}', f'sil-foot-{s}'], [f'foot-{s}', f'sil-foot-{s}'], nst=n['foot'], exp=2.3, planfit=0)
        H.loft(f'toe-{s}', f'toe-{s}', 'skin', 0.27, [f'foot-toe-{s}'], [f'foot-{s}', f'sil-foot-{s}'], [f'foot-{s}', f'sil-foot-{s}'], nst=n['toe'], exp=2.3, planfit=0, minr=(0.0385 if sc == 500 else (0.016 if sc == 200 else 0.0)))
    box_from(H, 'pillow', 'bed-surface', 'duvet-cover', 0.9, ['pillow'], ['pillow'], ['pillow'], corner=min(0.03, FEATURE[sc] * 2))
    box_from(H, 'duvet-skirt-top', 'bed-surface', 'duvet-cover', 0.75, ['duvet-skirt-top'], ['duvet-skirt-legs'], ['duvet-skirt-top'], corner=0.01)
    box_from(H, 'duvet-skirt-legs', 'bed-surface', 'duvet-cover', 0.75, ['duvet-skirt-legs', 'sil-duvet-skirt'], ['duvet-skirt-legs'], ['duvet-skirt-legs'], corner=0.01)
    return H


def thermal(fig):
    f, sc = fig, fig.scale
    have = {s['id'] for s in f.body}
    L = lambda r: region_layers(f.T, r)
    thermal_layers(f, 'face-skin', [i for i in ('head', 'neck') if i in have], [])
    if 'hair' in have:
        thermal_layers(f, 'hair', ['hair'], L('hair'))
    else:
        hs = f.find('head')
        q = hs['sections'][-1]
        f.solid('thermal', 'th-hair-cap', 'head', 'hair', 0.7, 'ellipsoid', radii=[0.075, 0.1, 0.085], transform={'translation': [r5(q['at'][0] - 0.05), 0.0, 0.0]}, region='hair', layer=L('hair')[-1]['id'])
    thermal_layers(f, 'hand-skin', ['hand-l', 'hand-r'], [])
    thermal_layers(f, 'top', [i for i in ('duvet-chest',) if i in have], L('top'))
    thermal_layers(f, 'top', [i for i in ('arm-l', 'arm-r', 'forearm-l', 'forearm-r') if i in have], L('top')[-1:])
    thermal_layers(f, 'bottom', [i for i in ('duvet-hips', 'duvet-thighs', 'duvet-shins') if i in have], L('bottom'))
    thermal_layers(f, 'feet-skin', [i for i in ('foot-l', 'foot-r', 'toe-l', 'toe-r') if i in have], [])
    f.ref('th-pillow', 'pillow', 'pillow')
    r = max(FEATURE[sc] * 0.55, 0.0045)
    for eid, anchor, region in (('nose-breath', ('head', 0.0287, -0.0995), 'face-skin'), ('mouth-breath', ('head', -0.0049, -0.0765), 'face-skin')):
        f.solid('thermal', f'th-{eid}', anchor[0], 'skin', 0.5, 'ellipsoid', radii=[r5(r)] * 3, transform={'translation': [anchor[1], anchor[2], 0.0]}, region=region, tags=['emitter', eid])
    f.solid('thermal', 'th-body-plume', 'torso', 'duvet-cover', 0.5, 'ellipsoid', radii=[r5(r), 0.1, 0.1], transform={'translation': [0.2226, -0.1, 0.0]}, region='top', tags=['emitter', 'body-plume'])


def structure(fig):
    f, sc = fig, fig.scale
    members = f.T.d['scales'][str(sc)]['memberBones']
    B = {}
    for s in f.body:
        B.setdefault(s['bone'], s)
    for bone in members:
        s = B.get(bone)
        if s is None:
            alt = {'thigh-l': 'thigh-r', 'shank-l': 'shank-r'}.get(bone)
            s = B.get(alt) if alt else None
            sid = {'pillow': 'pillow', 'duvet': 'duvet-skirt-legs'}.get(bone)
            if sid:
                s = next((x for x in f.body if x['id'] == sid), None)
        r = max(FEATURE[sc] * 0.52, 0.012)
        if s is None:
            L = f.T.bones[bone]['lengthM']
            f.solid('structure', f'member-{bone}', bone, 'skin', 0.5, 'capsule', p0=[0.02, 0.0, 0.0], p1=[L - 0.02, 0.0, 0.0], r0=r5(r), r1=r5(r), member=bone, part='member')
            continue
        if s['kind'] == 'loft':
            (p, rx0, ry0), (q, rx1, ry1) = axis_pts(s, 0.15, 0.85)
            rr = max(r, min(rx0, ry0, rx1, ry1) * 0.45)
            if sc >= 200:
                rr = max(FEATURE[sc] * 0.52, rr)
            rr = min(rr, min(rx0, ry0, rx1, ry1) * 0.9)
            if sc == 500:
                rr = 0.0385
            tr = None
            if bone != s['bone']:
                tr = (s['bone'], p, q)
            wshift = -0.06 if bone == 'thigh-l' or bone == 'shank-l' else (0.06 if bone in ('thigh-r', 'shank-r') else 0.0)
            f.solid('structure', f'member-{bone}', s['bone'], 'skin', 0.5, 'capsule', p0=[r5(p[0]), r5(p[1]), r5(p[2] + (wshift if s['bone'] != bone else 0))], p1=[r5(q[0]), r5(q[1]), r5(q[2] + (wshift if s['bone'] != bone else 0))], r0=r5(rr), r1=r5(rr), member=bone, part='member')
        elif s['kind'] == 'box':
            sz = [r5(x * 0.7) for x in s['size']]
            f.solid('structure', f'member-{bone}', s['bone'], 'skin', 0.5, 'box', size=sz, transform=s['transform'], member=bone, part='member')
    for bone in ('torso', 'pelvis', 'thigh-r'):
        s = B[bone]
        qm = s['sections'][len(s['sections']) // 2]
        f.solid('structure', f'mass-{bone}', bone, 'skin', 0.5, 'ellipsoid', radii=[r5(max(FEATURE[sc] * 0.55, qm['ry'] * 0.6))] * 2 + [r5(max(FEATURE[sc] * 0.55, qm['rx'] * 0.5))], transform={'translation': [r5(qm['at'][0]), r5(qm['at'][1]), r5(qm['at'][2])]}, member=bone, part='mass')


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    body(fig)
    thermal(fig)
    structure(fig)
    return fig


if __name__ == '__main__':
    vols = {}
    for sc in SCALES:
        fg = make(sc)
        vols[str(sc)] = {'status': 'authored', 'denominator': sc, 'body': fg.body, 'thermal': fg.thermal, 'structure': fg.structure, 'decisions': standing_decisions(fg, 'sleeper', extra=([text('Minimal: the structure keeps one member capsule per load-bearing bone, a single mass solid per heavy segment and no joint solids, so no further solid can be dropped without losing a bone.', 'Minimal: Das Tragwerk behält je tragendem Knochen eine Kapsel, je schwerem Segment ein Massen-Solid und keine Gelenk-Solids, sodass kein weiteres Solid entfallen kann, ohne einen Knochen zu verlieren.')] if sc > 50 else None))}
    print(dump(SLUG, 'humans', vols))
