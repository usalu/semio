import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-10-standing.py')).read())

SLUG = 'wheelchair-user'
CFG = dict(
    mat=dict(skin='skin', hair='hair', shirt='fleece', trousers='denim', shoe='canvas-sneaker'),
    tone=dict(skin=0.27, hair=0.7, shirt=0.4, rib=0.3, trousers=0.6, shoe=0.12, sole=0.85, hood=0.35),
    nose_clamp=None, torso_pf=0.3, hood_pf=0, toe_wk=1.0, hair_lo=0.12, minr500=True, pelvis_to_legs=True,
    torso_ur=(-0.14, 0.47), pelvis_ur=(-0.074, 0.103), head_ur_front=True,
    ids=dict(head_e=['head', 'sil-torso'], head_f=['head', 'sil-head'], torso_e=['hoodie', 'sil-torso'], torso_f=['hoodie', 'sil-body'], pelvis_e=['pelvis-jeans', 'sil-torso'], pelvis_f=['pelvis', 'sil-body']),
    regions=dict(face='face-skin', hair=None, hands='gloves', top='hoodie', legs='jeans', feet='shoes', arms=None),
    anchors=dict(nostrils=('head', 0.0294, -0.1018), mouth=('head', -0.0049, -0.0785), crown=('head', 0.17, 0.0)),
    emit={'nose-breath': ('nostrils', 'face-skin'), 'mouth-breath': ('mouth', 'face-skin'), 'body-plume': ('crown', 'hoodie')},
    n={500: dict(hair=0, hood=0)},
)


def tyre(fig, sid, bone, side, r_out, r_in, a0, a1, mat='rubber', tone=0.05):
    sg = 1 if side == 'l' else -1
    lo, hi = sorted((sg * a0, sg * a1))
    prof = [[lo, r_in], [hi, r_in], [hi, r_out], [lo, r_out], [lo, r_in]] if r_in > 0 else [[lo, 0.0], [hi, 0.0], [hi, r_out], [lo, r_out]]
    return fig.revolve(sid, bone, mat, tone, prof, [0, 0, 1], [0, 0, 0])


def chair_body(fig, H):
    sc = fig.scale
    f = fig
    big = sc >= 500
    for s in 'lr':
        sh = f.T.sel(sc, 'front', [f'rear-tyre-{s}', f'sil-wheel-{s}'], roles=('body',))
        zr = f.T.extent(sh, 0)
        a0, a1 = sorted((abs(zr[0]), abs(zr[1])))
        ro = 0.305
        sh_e = [x for x in f.T.shapes(sc, 'elevation') if x['id'] in ('rear-tyre', 'wheel-ring') and x['rings']]
        ri = 0.0
        if sh_e and len(sh_e[0]['rings']) > 1:
            inner = max(sh_e[0]['rings'], key=lambda r: len(r) and 0) if False else sorted(sh_e[0]['rings'], key=lambda r: max(q[0] for q in r) - min(q[0] for q in r))[0]
            ri = (max(q[0] for q in inner) - min(q[0] for q in inner)) / 2
        tyre(f, f'rear-tyre-{s}', 'rear-wheel', s, ro, ri if ri else 0.0, a0, a1)
        sh = f.T.sel(sc, 'front', [f'castor-tyre-{s}'], roles=('body',))
        if sh:
            zr = f.T.extent(sh, 0)
            c0, c1 = sorted((abs(zr[0]), abs(zr[1])))
        else:
            c0, c1 = 0.2, 0.235
        tyre(f, f'castor-tyre-{s}', 'castor', s, 0.068, 0.0, c0, c1)
        if sc <= 100:
            tyre(f, f'rear-hub-{s}', 'rear-wheel', s, 0.035, 0.0, a0 - 0.022, a1 - 0.001, mat='aluminium', tone=0.5)
        if sc == 50:
            tyre(f, f'hand-rim-{s}', 'rear-wheel', s, 0.286, 0.274, a0 - 0.014, a0 - 0.002, mat='aluminium', tone=0.5)
            sg = 1 if s == 'l' else -1
            f.capsule(f'spoke-{s}', 'rear-wheel', 'aluminium', 0.5, [0.03, 0.0, sg * (a0 - 0.0)], [0.278, 0.0, sg * (a0 + 0.0)], 0.0045, 0.0045, role='detail', slender=True, repeat={'count': 12, 'rotationAxis': [0, 0, 1], 'rotationRad': r5(2 * math.pi / 12), 'origin': [0, 0, 0]})
    pf = [(f'seat-rail-{s}', ['frame-seat-rail'], None, [f'frame-seat-rail-{s}']) for s in 'lr']
    pf += [(f'back-tube-{s}', ['frame-back-tube'], [f'frame-back-tube-{s}'], None) for s in 'lr']
    pf += [(f'push-handle-{s}', ['frame-push-handle'], [f'frame-push-handle-{s}'], None) for s in 'lr']
    pf += [(f'hanger-{s}', ['frame-hanger'], [f'frame-hanger-{s}'], None) for s in 'lr']
    pf += [(f'castor-fork-{s}', ['castor-fork', 'castor-housing'], None, [f'castor-tyre-{s}']) for s in 'lr']
    pf += [('lower-rail', ['frame-lower-rail'], ['frame-lower-rail'], None), ('footplate', ['footplate', 'sil-footplate'], ['footplate', 'sil-footplate'], ['footplate', 'sil-footplate']), ('back-sling', ['back-sling'], ['back-sling'], None)]
    for sid, e, fr, pl in pf:
        if sc >= 500 and not sid.startswith('footplate'):
            continue
        if sc >= 200 and sid.startswith(('seat-rail', 'back-tube', 'push-handle', 'castor-fork', 'lower-rail')):
            continue
        if sc == 200 and sid.startswith('back-sling') is False and sid.startswith('hanger') is False and sid != 'footplate':
            continue
        b = box_from(H, sid, 'frame', 'aluminium', 0.5, e, fr, pl, corner=min(0.01, FEATURE[sc]))
        if b is None:
            continue
    if sc >= 200:
        r = max(FEATURE[sc] * 0.53, 0.0125)
        for sh in f.T.shapes(sc, 'elevation'):
            if sh['bone'] != 'frame' or sh['closed'] or sh['role'] != 'body' or not sh['rings'] or sh['id'] == 'ground':
                continue
            pts = sh['rings'][0]
            for i in range(len(pts) - 1):
                a = f.to_local('frame', [pts[i][0], pts[i][1], 0.0])
                b = f.to_local('frame', [pts[i + 1][0], pts[i + 1][1], 0.0])
                if math.dist(a, b) < 0.02:
                    continue
                for sg, tag in ((1, 'l'), (-1, 'r')):
                    f.capsule(f"{sh['id']}-{i}-{tag}", 'frame', 'aluminium', 0.5, [a[0], a[1], sg * 0.205], [b[0], b[1], sg * 0.205], r, r)
    cu = box_from(H, 'seat-cushion', 'frame', 'foam', 0.2, ['seat-cushion', 'sil-seat'], ['seat-cushion', 'sil-seat'], ['seat-cushion', 'sil-seat'], corner=min(0.015, FEATURE[sc] * 1.5))


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    H, n = standing_body(fig, CFG)
    sc = scale
    if sc <= 200:
        H.loft('beanie', 'head', 'knit-wool', 0.35, ['beanie'], ['beanie', 'hair'], ['head'], nst={50: 6, 100: 5, 200: 3}[sc], exp=2.2, planfit=0, grow=0.002)
    chair_body(fig, H)
    standing_thermal(fig, CFG)
    have = {s['id'] for s in fig.body}
    L = lambda r: region_layers(T, r)
    if 'beanie' in have:
        thermal_layers(fig, 'beanie', ['beanie'], L('beanie'))
    else:
        hs = fig.find('head'); q = hs['sections'][-2]
        fig.solid('thermal', 'th-beanie', 'head', 'knit-wool', 0.35, 'ellipsoid', radii=[0.075, r5(q['ry'] * 1.05), r5(q['rx'] * 1.05)], transform={'translation': [r5(q['at'][0]), r5(q['at'][1]), 0.0]}, region='beanie', layer='beanie')
    for s in 'lr':
        fig.solid('thermal', f'th-wrist-{s}', f'forearm-{s}', 'skin', 0.27, 'ellipsoid', radii=[0.0375 if sc == 500 else max(0.012, FEATURE[sc] * 0.52), max(0.0075, FEATURE[sc] * 0.52), 0.035], transform={'translation': [0.215, 0.0, 0.0]}, region='hand-skin')
    fr = [i for i in have if i.startswith(('seat-rail', 'back-tube', 'push-handle', 'hanger', 'castor-fork', 'lower-rail', 'footplate', 'back-sling', 'frame-'))]
    for i in fr:
        fig.ref(f'th-{i}', i, 'frame-metal')
    for i in have:
        if i.startswith(('rear-tyre', 'castor-tyre')):
            fig.ref(f'th-{i}', i, 'tyres')
    fig.ref('th-seat-cushion', 'seat-cushion', 'cushion')
    standing_structure(fig, CFG)
    fb = [s for s in fig.body if s['id'] in fr]
    for s in fb:
        if s['kind'] == 'capsule':
            fig.solid('structure', f"member-{s['id']}", 'frame', 'aluminium', 0.5, 'capsule', p0=s['p0'], p1=s['p1'], r0=s['r0'], r1=s['r1'], member='frame', part='member')
        if s['kind'] == 'box':
            sz = [r5(max(x * 0.6, FEATURE[sc] * 1.05)) for x in s['size']]
            fig.solid('structure', f"member-{s['id']}", 'frame', 'aluminium', 0.5, 'box', size=sz, transform=s['transform'], member='frame', part='member')
    for s in 'lr':
        w = fig.find(f'rear-tyre-{s}')
        r = max(0.02, FEATURE[sc] * 0.52)
        sg = 1 if s == 'l' else -1
        fig.solid('structure', f'member-wheel-{s}', 'rear-wheel', 'aluminium', 0.5, 'capsule', p0=[0.0, 0.0, sg * 0.31], p1=[0.0, 0.0, sg * 0.33], r0=r5(max(r, 0.035)), r1=r5(max(r, 0.035)), member='rear-wheel', part='member')
    return fig


if __name__ == '__main__':
    vols = {}
    for sc in SCALES:
        fg = make(sc)
        vols[str(sc)] = {'status': 'authored', 'denominator': sc, 'body': fg.body, 'thermal': fg.thermal, 'structure': fg.structure, 'decisions': standing_decisions(fg, 'wheelchair')}
    print(dump(SLUG, 'humans', vols))
