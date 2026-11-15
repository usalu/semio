import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
if 'Fig' not in globals():
    exec(open(os.path.join(HERE, 'author-10-lib.py')).read())
if 'Human' not in globals():
    exec(open(os.path.join(HERE, 'author-10-human.py')).read())

DEFAULT_N = {
    50: dict(head=10, hair=7, neck=4, hood=7, torso=12, pelvis=4, arm=8, fore=8, hand=7, thigh=9, shank=9, foot=9, toe=5, details=True),
    100: dict(head=8, hair=5, neck=3, hood=5, torso=9, pelvis=3, arm=6, fore=6, hand=5, thigh=7, shank=7, foot=7, toe=4, details=False),
    200: dict(head=6, hair=4, neck=3, hood=4, torso=7, pelvis=3, arm=5, fore=5, hand=4, thigh=6, shank=6, foot=5, toe=3, details=False),
    500: dict(head=5, hair=0, neck=2, hood=0, torso=5, pelvis=2, arm=4, fore=4, hand=3, thigh=5, shank=5, foot=4, toe=2, details=False),
}


def standing_body(fig, cfg):
    if os.environ.get('A10_CFG'):
        cfg = {**cfg, **json.loads(os.environ['A10_CFG'])}
    sc = fig.scale
    n = dict(DEFAULT_N[sc])
    n.update(cfg.get('n', {}).get(sc, {}))
    H = Human(fig, None)
    M, TN = cfg['mat'], cfg['tone']
    ids = cfg.get('ids', {})
    AB = cfg.get('arm', 'arm-{s}')
    nose = n['details'] or sc == 100
    head_kw = dict(exp=2.2)
    if nose and cfg.get('nose_clamp') is not None:
        head_kw['vclamp'] = (cfg['nose_clamp'], 9.0)
    hur = None
    if cfg.get('head_ur_front'):
        fshape = H.pk('front', ids.get('head_f', ['head', 'sil-head']))
        hb = fig.fr('head')
        ys = [q[1] for sh in fshape for r in sh['rings'] for q in r]
        us = [(y - hb['o'][1]) * hb['u'][1] for y in ys]
        hur = (min(us), max(us))
    H.loft('head', 'head', M['skin'], TN['skin'], ids.get('head_e', ['head', 'sil-head']), ids.get('head_f', ['head', 'sil-head']), nst=n['head'], planfit=0, ur=hur, **head_kw)
    if n['hair']:
        H.loft('hair', 'head', M['hair'], TN['hair'], ids.get('hair_e', ['hair']), ids.get('hair_f', ['hair', 'head']), nst=n['hair'], exp=2.2, grow=0.002, lo=cfg.get('hair_lo', 0.35), planfit=0)
    if 'neck' not in fig.T.bones:
        pass
    elif H.pk('elevation', ['neck']) or H.pk('front', ['neck']):
        H.loft('neck', 'neck', M['skin'], TN['skin'], ['neck'], ['neck'], nst=n['neck'], exp=2.0)
    else:
        sh = H.pk('plan', ['shoulders'])
        ez, ex = H.T.extent(sh, 1), H.T.extent(sh, 0)
        fig.loft('neck', 'neck', M['shirt'], TN['shirt'], [{'at': [r5(u), r5(-(ex[0] + ex[1]) / 2 + 0.0), 0], 'rx': 0.175, 'ry': 0.098, 'exponent': 2.2, 'rotation': QU} for u in (-0.09, -0.01)])
    if n['hood']:
        H.loft('hood', 'torso', M['shirt'], cfg['tone'].get('hood', 0.4), ['hood'], ['hood'], ['hood'], nst=n['hood'], exp=2.4, wk=cfg.get('hood_wk', 0.85), planfit=cfg.get('hood_pf', 0))
    H.loft('torso', 'torso', M['shirt'], TN['shirt'], ids.get('torso_e', ['hoodie', 'sil-body', 'coat']), ids.get('torso_f', ['hoodie', 'sil-body', 'coat']), ['trunk'], nst=n['torso'], exp=cfg.get('torso_exp', 2.5), ur=cfg.get('torso_ur', (-0.03, 0.4522)), planfit=(cfg.get('torso_pf', 0).get(sc, 0) if isinstance(cfg.get('torso_pf', 0), dict) else cfg.get('torso_pf', 0)))
    H.loft('pelvis', 'pelvis', M['shirt'], TN['shirt'], ids.get('pelvis_e', ids.get('torso_e', ['hoodie', 'sil-body', 'coat'])), ids.get('pelvis_f', ids.get('torso_f', ['hoodie', 'sil-body', 'coat'])), ['waist'], nst=n['pelvis'], exp=cfg.get('torso_exp', 2.5), ur=cfg.get('pelvis_ur', (-0.075, fig.T.bones['pelvis']['lengthM'])), minr=(0.0385 if (sc == 500 and cfg.get('minr500')) else 0.0), planfit=(cfg.get('torso_pf', 0).get(sc, 0) if isinstance(cfg.get('torso_pf', 0), dict) else cfg.get('torso_pf', 0)))
    for s in 'lr':
        H.loft(AB.format(s=s), AB.format(s=s), M['shirt'], TN['shirt'], [f'sleeve-{s}', 'sil-arm', f'upper-arm-{s}'], [f'sleeve-{s}', f'sil-arm-{s}', f'upper-arm-{s}'], [f'upper-arm-{s}', f'sil-arm-{s}'], nst=n['arm'], exp=2.1, trim=True, minr=(0.0385 if (sc == 500 and cfg.get('minr500')) else 0.0))
        H.loft(f'forearm-{s}', f'forearm-{s}', M['shirt'], TN['shirt'], [f'forearm-{s}', 'sil-forearm'], [f'forearm-{s}', f'sil-forearm-{s}'], [f'forearm-{s}', f'sil-forearm-{s}'], nst=n['fore'], exp=2.0, minr=(0.0385 if (sc == 500 and cfg.get('minr500')) else 0.0))
        H.loft(f'hand-{s}', f'hand-{s}', M['skin'], TN['skin'], [f'hand-{s}', f'glove-{s}', 'sil-hand'], [f'hand-{s}', f'sil-hand-{s}'], [f'hand-{s}', f'sil-hand-{s}'], nst=n['hand'], exp=2.4, minr=0.0385 if sc == 500 else 0.0)
        H.loft(f'thigh-{s}', f'thigh-{s}', M['trousers'], TN['trousers'], [f'thigh-{s}', f'sil-thigh-{s}'], [f'thigh-{s}', f'sil-thigh-{s}'], [f'thigh-{s}', f'sil-thigh-{s}'], nst=n['thigh'], exp=2.1)
        H.loft(f'shank-{s}', f'shank-{s}', M['trousers'], TN['trousers'], [f'shank-{s}', f'sil-shank-{s}'], [f'shank-{s}', f'sil-shank-{s}'], [f'shank-{s}', f'sil-shank-{s}'], nst=n['shank'], exp=2.1)
        H.loft(f'shoe-{s}', f'foot-{s}', M['shoe'], TN['shoe'], [f'shoe-{s}', f'shoe-rear-{s}', f'boot-{s}', f'sil-shoe-{s}'], [f'shoe-{s}', f'sil-shoe-{s}'], [f'shoe-heel-{s}', f'shoe-{s}', f'sil-shoe-{s}'], nst=n['foot'], exp=2.8)
        if sc == 500:
            H.loft(f'toe-{s}', f'toe-{s}', M['shoe'], TN['shoe'], [f'sil-shoe-{s}', f'sil-toe-{s}'], [f'sil-shoe-{s}'], [f'sil-shoe-{s}'], nst=n['toe'], exp=2.8, ur=(-0.1, 0.0645), wk=cfg.get('toe_wk',1.0), planfit=0)
        else:
            H.loft(f'toe-{s}', f'toe-{s}', M['shoe'], TN['shoe'], [f'shoe-toe-{s}', f'boot-toe-{s}', f'toe-{s}', f'sil-toe-{s}'], [f'shoe-{s}', f'sil-shoe-{s}'], [f'shoe-toe-{s}', f'shoe-{s}', f'sil-shoe-{s}'], nst=n['toe'], exp=2.8)
    if sc <= 100:
        for s in 'lr':
            t = capsule_from(H, f'thumb-{s}', f'hand-{s}', M['skin'], TN['skin'], [f'thumb-{s}'], w=0.0, rscale=0.85)
            if t:
                shorten(t, 0.010)
    if n['details']:
        head_details(H, fig, cfg, n)
        garment_details(H, fig, cfg, n)
    return H, n


def shorten(c, d):
    p0, p1 = c['p0'], c['p1']
    L = math.dist(p0, p1)
    if L > 2 * d:
        k = d / L
        c['p1'] = [r5(p1[i] - (p1[i] - p0[i]) * k) for i in range(3)]


def head_details(H, fig, cfg, n):
    M, TN = cfg['mat'], cfg['tone']
    sc = fig.scale
    e = H.pk('elevation', ['head']) 
    fig.ellipsoid('nose', 'head', M['skin'], TN['skin'], [0.019, 0.013, 0.010], [0.036, cfg.get('nose_v', -0.086), 0.0], role='detail')
    H.blob('ear-l', 'head', M['skin'], TN['skin'], ['ear'], ['ear-l'], grow=1.0, minr=0.006)
    H.blob('ear-r', 'head', M['skin'], TN['skin'], ['ear'], ['ear-r'], grow=1.0, minr=0.006)
    for s in 'lr':
        b = H.blob(f'eye-{s}', 'head', M['skin'], 0.9, ['eye'], [f'eye-{s}'], depth=0.005, minr=0.005)
        if b:
            b['transform']['translation'][1] = cfg.get('eye_v', -0.076)


def garment_details(H, fig, cfg, n):
    M, TN = cfg['mat'], cfg['tone']
    for s in 'lr':
        H.loft(f'cuff-{s}', f'forearm-{s}', M['shirt'], TN['rib'], [f'cuff-rib-{s}'], [f'cuff-rib-{s}'], nst=3, exp=2.0, grow=0.003, roles=('body', 'detail'))
        H.loft(f'sole-{s}', f'foot-{s}', M['shoe'], TN['sole'], [f'sole-{s}'], [f'shoe-{s}'], [f'shoe-heel-{s}'], nst=5, exp=2.8, grow=0.002, roles=('body', 'detail'))
        H.loft(f'toe-sole-{s}', f'toe-{s}', M['shoe'], TN['sole'], [f'toe-sole-{s}'], [f'shoe-{s}'], [f'shoe-toe-{s}'], nst=3, exp=2.8, grow=0.002, roles=('body', 'detail'))
    H.loft('hem', 'pelvis', M['shirt'], TN['rib'], ['hem-rib'], ['hem-rib'], ['waist'], nst=3, exp=2.5, grow=0.004, roles=('body', 'detail'))


FEAT = FEATURE


def standing_thermal(fig, cfg):
    f, sc, T = fig, fig.scale, fig.T
    R = cfg['regions']
    body = {s['id'] for s in f.body}
    have = lambda ids: [i for i in ids if i in body]
    L = lambda rid: region_layers(T, rid)
    thermal_layers(f, R['face'], have(['head', 'neck']) if sc < 500 else have(['head']), L(R['face']) if R['face'] else [])
    if R.get('hair'):
        if 'hair' in body:
            thermal_layers(f, R['hair'], ['hair'], L(R['hair']))
        else:
            hs = f.find('head')
            q = hs['sections'][-1]
            top = q['at']
            f.solid('thermal', 'th-hair-cap', 'head', cfg['mat']['hair'], cfg['tone']['hair'], 'ellipsoid', radii=[r5(0.5 * (hs['sections'][-1]['at'][0] - hs['sections'][0]['at'][0]) * 0.75), r5(max(x['ry'] for x in hs['sections']) * 1.08), r5(max(x['rx'] for x in hs['sections']) * 1.08)], transform={'translation': [r5(top[0] - 0.045), r5(top[1]), 0.0]}, region=R['hair'], layer=L(R['hair'])[-1]['id'])
    hands = have(['hand-l', 'hand-r', 'thumb-l', 'thumb-r'])
    thermal_layers(f, R['hands'], hands, L(R['hands']))
    AB = cfg.get('arm', 'arm-{s}')
    arms = [AB.format(s='l'), AB.format(s='r'), 'forearm-l', 'forearm-r', 'cuff-l', 'cuff-r']
    if R.get('arms'):
        thermal_layers(f, R['arms'], have(arms), L(R['arms']))
        thermal_layers(f, R['top'], have(['torso', 'hood', 'hem'] + ([] if cfg.get('pelvis_to_legs') else ['pelvis'])), L(R['top']))
    else:
        thermal_layers(f, R['top'], have(['torso', 'hood', 'hem'] + ([] if cfg.get('pelvis_to_legs') else ['pelvis']) + arms), L(R['top']))
    legs = have(['thigh-l', 'thigh-r', 'shank-l', 'shank-r'] + (['pelvis'] if cfg.get('pelvis_to_legs') else []))
    thermal_layers(f, R['legs'], legs, L(R['legs']))
    feet = have(['shoe-l', 'shoe-r', 'toe-l', 'toe-r'])
    thermal_layers(f, R['feet'], feet, L(R['feet']))
    emitters(f, cfg)


def emitters(f, cfg):
    sc = f.scale
    r = max(FEAT[sc] * 0.55, 0.0045)
    T = f.T
    A = cfg['anchors']
    em = {e['id']: e for e in T.thermal['emitters']}
    for eid, (anchor, region) in cfg['emit'].items():
        if eid not in em:
            continue
        bone, u, v = A[anchor]
        d = max(em[eid]['diameterM'] / 2, r)
        if eid == 'body-plume':
            f.solid('thermal', f'th-{eid}', bone, cfg['mat']['skin'], 0.5, 'ellipsoid', radii=[r5(r), r5(min(d, cfg.get('plume_r', 0.08))), r5(min(d, cfg.get('plume_r', 0.08)))], transform={'translation': [r5(u - cfg.get('plume_du', 0.0)), r5(v), 0.0]}, region=region, tags=['emitter', eid])
        else:
            f.solid('thermal', f'th-{eid}', bone, cfg['mat']['skin'], 0.5, 'ellipsoid', radii=[r5(r), r5(r), r5(r)], transform={'translation': [r5(u), r5(v), 0.0]}, region=region, tags=['emitter', eid])


MEMBER_R = {50: dict(thigh=0.0155, shank=0.014, arm=0.0125, fore=0.01, hand=0.009, foot=0.012, spine=0.022, neck=0.017, pelvis=0.03),
            100: dict(thigh=0.0165, shank=0.015, arm=0.0135, fore=0.011, hand=0.0105, foot=0.013, spine=0.024, neck=0.018, pelvis=0.032),
            200: dict(thigh=0.0165, shank=0.0155, arm=0.0155, fore=0.0155, hand=0.0155, foot=0.0155, spine=0.026, neck=0.0185, pelvis=0.034),
            500: dict(thigh=0.0385, shank=0.0385, arm=0.0385, fore=0.0385, hand=0.0385, foot=0.0385, spine=0.0385, neck=0.0385, pelvis=0.0385)}


def standing_structure(fig, cfg):
    f, sc = fig, fig.scale
    R = MEMBER_R[sc]
    AB = cfg.get('arm', 'arm-{s}')
    B = {s['id']: s for s in f.body}
    mat = cfg['mat']['skin']

    def ends(bid, a=0.12, b=0.88):
        return axis_pts(B[bid], a, b)

    for s in 'lr':
        for bone, key in (('thigh', 'thigh'), ('shank', 'shank'), (AB.format(s='x')[:-2], 'arm'), ('forearm', 'fore'), ('hand', 'hand')):
            bid = f'{bone}-{s}'
            (p, _, _), (q, _, _) = ends(bid)
            f.solid('structure', f'member-{bid}', bid, mat, 0.5, 'capsule', p0=[r5(x) for x in p], p1=[r5(x) for x in q], r0=R[key], r1=R[key], member=bid, part='member')
        (p, _, _), (q, _, _) = ends(f'shoe-{s}', 0.1, 0.9)
        f.solid('structure', f'member-foot-{s}', f'foot-{s}', mat, 0.5, 'capsule', p0=[r5(x) for x in p], p1=[r5(x) for x in q], r0=R['foot'], r1=R['foot'], member=f'foot-{s}', part='member')
    for s in 'lr':
        if f'toe-{s}' in f.T.d['scales'][str(sc)]['memberBones'] and f'toe-{s}' in B:
            (p, _, _), (q, _, _) = ends(f'toe-{s}', 0.2, 0.9)
            f.solid('structure', f'member-toe-{s}', f'toe-{s}', mat, 0.5, 'capsule', p0=[r5(x) for x in p], p1=[r5(x) for x in q], r0=R['hand'], r1=R['hand'], member=f'toe-{s}', part='member')
    (p, _, _), (q, _, _) = ends('torso', 0.1, 0.9)
    f.solid('structure', 'member-torso', 'torso', mat, 0.5, 'capsule', p0=[r5(p[0]), r5(p[1] + 0.03), 0.0], p1=[r5(q[0]), r5(q[1] + 0.03), 0.0], r0=R['spine'], r1=R['spine'], member='torso', part='member')
    f.solid('structure', 'member-pelvis', 'pelvis', mat, 0.5, 'capsule', p0=[-0.02, 0.0, -0.085], p1=[-0.02, 0.0, 0.085], r0=R['pelvis'], r1=R['pelvis'], member='pelvis', part='member')
    if 'neck' not in f.T.bones:
        pass
    elif 'neck' in B and sc < 500:
        (p, _, _), (q, _, _) = ends('neck', 0.1, 0.9)
        f.solid('structure', 'member-neck', 'neck', mat, 0.5, 'capsule', p0=[r5(x) for x in p], p1=[r5(x) for x in q], r0=R['neck'], r1=R['neck'], member='neck', part='member')
    else:
        f.solid('structure', 'member-neck', 'neck', mat, 0.5, 'capsule', p0=[-0.07, 0.0, 0.0], p1=[-0.02, 0.0, 0.0], r0=R['neck'], r1=R['neck'], member='neck', part='member')
    hs = B['head']
    q0 = hs['sections'][len(hs['sections']) // 2]
    f.solid('structure', 'member-head', 'head', mat, 0.5, 'ellipsoid', radii=[r5(max(0.04, q0['ry'] * 0.7)), r5(max(0.04, q0['ry'] * 0.7)), r5(max(0.04, q0['rx'] * 0.7))], transform={'translation': [r5(q0['at'][0]), r5(q0['at'][1]), 0.0]}, member='head', part='member')
    big = sc <= 100
    jlist = ['thigh-l', 'thigh-r', 'shank-l', 'shank-r', AB.format(s='l'), AB.format(s='r'), 'forearm-l', 'forearm-r', 'foot-l', 'foot-r', 'neck', 'head', 'torso']
    jlist = [j for j in jlist if j in f.T.bones]
    if sc == 100:
        jlist = ['thigh-l', 'thigh-r', 'shank-l', 'shank-r', AB.format(s='l'), AB.format(s='r'), 'forearm-l', 'forearm-r', 'torso']
    for bone in (jlist if big else (['thigh-l', 'thigh-r', 'shank-l', 'shank-r', AB.format(s='l'), AB.format(s='r'), 'forearm-l', 'forearm-r'] if sc == 200 else ['thigh-l', 'thigh-r', AB.format(s='l'), AB.format(s='r')])):
        if bone not in ('head',):
            kd = 'arm' if bone.endswith(('-l', '-r')) and 'arm' in bone and 'forearm' not in bone else bone.split('-')[0]
            rj = {'thigh': 1.55, 'shank': 1.5, 'arm': 1.5, 'forearm': 1.5, 'foot': 1.5, 'neck': 1.3, 'torso': 1.3}[kd]
            key = {'forearm': 'fore'}.get(kd, kd)
            base = R.get(key, R['spine'])
            r = max(base * rj, FEAT[sc] * 0.5 + 0.001)
            if sc == 500:
                r = 0.0385
            bid = bone
            sbody = B.get(bid) or B.get('shoe-' + bone[-1] if bone.startswith('foot') else bid)
            c = sbody['sections'][0]['at'] if sbody and sbody.get('sections') else [0, 0, 0]
            at = [0.0, r5(c[1]), r5(c[2])]
            if bone == 'torso':
                at = [0.0, r5(c[1] + 0.03), 0.0]
            if bone == 'neck' and 'neck' in B:
                at = [0.0, 0.0, 0.0]
            f.solid('structure', f'joint-{bone}', bone, mat, 0.5, 'ellipsoid', radii=[r5(r), r5(r), r5(r)], transform={'translation': at}, member=bone, part='joint')
    for bid in ('torso', 'pelvis', 'thigh-l', 'thigh-r'):
        bn = T_BONE(f, bid)
        b = B[bid]
        k = len(b['sections']) // 2
        qm = b['sections'][k]
        comx = f.T.bones[bid]['comX'] if f.T.bones[bid].get('comX') is not None else qm['at'][0]
        radii = [qm['ry'] * 0.55, qm['ry'] * 0.55, qm['rx'] * 0.55]
        radii = [r5(max(FEAT[sc] * 0.55, x)) for x in radii]
        if bid in ('thigh-l', 'thigh-r'):
            radii = [r5(max(FEAT[sc] * 0.55, qm['rx'] * 0.6)), r5(max(FEAT[sc] * 0.55, qm['ry'] * 0.6)), r5(max(FEAT[sc] * 0.55, qm['rx'] * 0.6))]
            radii = [r5(max(FEAT[sc] * 0.55, qm['ry'] * 0.6)), r5(max(FEAT[sc] * 0.55, qm['ry'] * 0.6)), r5(max(FEAT[sc] * 0.55, qm['rx'] * 0.6))]
        f.solid('structure', f'mass-{bid}', bid, mat, 0.5, 'ellipsoid', radii=radii, transform={'translation': [r5(comx), r5(qm['at'][1]), r5(qm['at'][2])]}, member=bid, part='mass')


def T_BONE(f, bid):
    return f.T.bones[bid]


def standing_decisions(fig, kind, extra=None):
    sc = fig.scale
    nb, nt, ns = len(fig.body), len(fig.thermal), len(fig.structure)
    en = {
        50: f"1:50 detail model, {nb} body solids: every segment is a loft of superelliptic sections whose depth is cut from the elevation outline and whose width and lateral centre from the front view (plan view for the nearly horizontal foot); section stations are chosen where the drawn outline bends, so thigh, calf, chest and hood keep their real third dimension instead of a view extrusion. Eyes, ears, nose, thumbs, rib cuffs, hem, soles are separate solids; features under 7.5 mm (drawstrings, seams, laces) are not modelled.",
        100: f"1:100 model, {nb} body solids, modelled anew against the 1:100 drawings: fewer stations per loft, the cuffs, hem, soles, eyes and ears are merged into their parent solids, nose and thumbs stay because they are still wider than 15 mm.",
        200: f"1:200 model, {nb} body solids: limbs are generalised to four to six stations, face, thumbs and rib details are gone, hood and hair stay as masses; hands and toes remain separate solids because their bones are still drawn.",
        500: f"1:500 model, {nb} body solids: a generalised figure of broad lofts, no hair or hood, the head is one loft, the neck bone is a shoulder yoke hidden inside the torso because the 1:500 drawing leaves no visible neck, hands and toes are at least 75 mm thick overlapping masses.",
    }[sc]
    de = {
        50: f"Detailmodell 1:50 mit {nb} Körper-Solids: jedes Segment ist ein Loft aus superelliptischen Schnitten, deren Tiefe der Seitenansicht und deren Breite und seitliche Lage der Vorderansicht (beim fast waagrechten Fuss dem Grundriss) entnommen sind; die Schnittstellen liegen dort, wo die gezeichnete Kontur knickt, sodass Oberschenkel, Wade, Brust und Kapuze ihre echte dritte Dimension behalten. Augen, Ohren, Nase, Daumen, Bündchen, Saum und Sohlen sind eigene Solids; Merkmale unter 7,5 mm (Kordeln, Nähte, Schnürsenkel) sind nicht modelliert.",
        100: f"Modell 1:100 mit {nb} Körper-Solids, neu gegen die Zeichnungen 1:100 modelliert: weniger Stationen je Loft, Bündchen, Saum, Sohlen, Augen und Ohren sind in ihre Elternteile verschmolzen, Nase und Daumen bleiben, weil sie noch breiter als 15 mm sind.",
        200: f"Modell 1:200 mit {nb} Körper-Solids: Gliedmassen auf vier bis sechs Stationen verallgemeinert, Gesicht, Daumen und Bündchen entfallen, Kapuze und Haar bleiben als Massen; Hände und Zehen bleiben eigene Solids, weil ihre Knochen noch gezeichnet sind.",
        500: f"Modell 1:500 mit {nb} Körper-Solids: eine verallgemeinerte Figur aus breiten Lofts ohne Haar und Kapuze, der Kopf ist ein Loft, der Halsknochen ist eine im Rumpf verborgene Schulterpartie, weil die Zeichnung 1:500 keinen sichtbaren Hals lässt, Hände und Zehen sind überlappende Massen von mindestens 75 mm Dicke.",
    }[sc]
    th_en = f"Thermal, {nt} solids: each region is built from the body solids it covers (shared geometry through references); " + ("multi-layer regions (shirt, trousers, shoes) are nested shells, the inner layers are the outer solid shrunk by the thickness of the layers outside; " if sc == 50 else "layers are merged into one outer surface at this scale because their thickness is far below the smallest feature; ") + "the two breath outlets and the plume are small solids at the head anchors."
    th_de = f"Thermik, {nt} Solids: jede Region entsteht aus den Körper-Solids, die sie bedeckt (geteilte Geometrie über Verweise); " + ("mehrlagige Regionen (Oberteil, Hose, Schuhe) sind verschachtelte Schalen, die inneren Lagen sind das äussere Solid, geschrumpft um die Dicke der darüberliegenden Lagen; " if sc == 50 else "die Lagen sind auf diesem Massstab zu einer äusseren Fläche verschmolzen, weil ihre Dicke weit unter dem kleinsten Merkmal liegt; ") + "die beiden Atemauslässe und die Fahne sind kleine Solids an den Kopfankern."
    st_en = f"Structure, {ns} solids: one capsule per load-bearing bone along the centre line of its loft with the section of the real bone shaft (raised to the smallest feature at coarse scales), a pelvic girdle bar, a spine behind the torso centre, joint ellipsoids at hips, knees, ankles, shoulders, elbows and neck (merged to the major joints at 1:200 and 1:500), and compact mass ellipsoids at the centres of mass of torso, pelvis and thighs; the model has no supports."
    st_de = f"Tragwerk, {ns} Solids: je tragendem Knochen eine Kapsel entlang der Mittellinie seines Lofts mit dem Querschnitt des echten Knochenschafts (auf grossen Massstäben auf das kleinste Merkmal angehoben), ein Beckenring-Stab, eine Wirbelsäule hinter der Rumpfmitte, Gelenk-Ellipsoide an Hüfte, Knie, Knöchel, Schulter, Ellbogen und Hals (auf 1:200 und 1:500 auf die grossen Gelenke verschmolzen) sowie kompakte Massen-Ellipsoide in den Schwerpunkten von Rumpf, Becken und Oberschenkeln; das Modell hat keine Auflager."
    d = [text(en, de), text(th_en, th_de), text(st_en, st_de)]
    if extra:
        d += extra
    return d
