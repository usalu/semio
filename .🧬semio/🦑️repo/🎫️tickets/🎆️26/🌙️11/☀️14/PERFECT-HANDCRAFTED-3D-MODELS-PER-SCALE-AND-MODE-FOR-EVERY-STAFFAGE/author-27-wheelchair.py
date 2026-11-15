import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-27-lib.py')).read())
GEN = os.path.join(TICKET, '🗑️generated', 'author10')

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
    n={50: dict(hair=0), 100: dict(hair=0), 200: dict(hair=0), 500: dict(hair=0, hood=0)},
)
Z = 0.205
FR = 0.305

# frame tubes of the 2D drawings: id, drawn shape, optional slice limits, lateral position
TUBES = [
    ('seat-rail', 'frame-seat-rail', dict(xr=(-0.07, 0.42)), Z),
    ('back-tube', 'frame-back-tube', dict(yr=(0.47, 0.93)), Z),
    ('hanger', 'frame-hanger', dict(yr=(0.14, 0.5)), Z),
    ('lower-rail', 'frame-lower-rail', dict(xr=(0.0, 0.40)), Z),
    ('lower-front', 'frame-lower-front', dict(yr=(0.125, 0.30)), Z),
    ('castor-fork', 'castor-fork', dict(yr=(0.085, 0.215)), 0.2155),
]
STROKES = {  # 1:200 / 1:500 open strokes of the elevation (frame bone)
    200: [('back-tube', 'frame-back-tube'), ('seat-rail', 'frame-seat-rail'), ('hanger', 'frame-hanger'), ('lower-rail', 'frame-lower-rail'), ('castor-fork', 'castor-fork')],
    500: [('frame-main', 'frame-main'), ('frame-lower', 'frame-lower')],
}


def lat(sgn, z):
    return sgn * z


def vloft(f, idp, bone, mat, tone, pts, rx, ry, exp=3.4, role='body'):
    secs = [{'at': [r5(p[0]), r5(p[1]), r5(p[2])], 'rx': r5(rx), 'ry': r5(ry), 'exponent': exp} for p in pts]
    return f.loft(idp, bone, mat, tone, secs, role=role)


def chair_body(fig, H):
    f, T, sc = fig, fig.T, fig.scale
    FE = FEATURE[sc]
    rmin = r5(FE * 0.55)
    el = lambda ids, roles=('body',): T.sel(sc, 'elevation', ids, roles=roles)
    fr_ = lambda ids, roles=('body',): T.sel(sc, 'front', ids, roles=roles)
    pl_ = lambda ids, roles=('body',): T.sel(sc, 'plan', ids, roles=roles)
    al = 'aluminium'
    ex = fr_(['rear-tyre-l', 'sil-wheel-l'])
    zr = T.extent(ex, 0)
    a0, a1 = sorted((abs(zr[0]), abs(zr[1])))
    fine = sc <= 100
    for s, sg in (('l', 1), ('r', -1)):
        def R(idp, zlo, zhi, r_in, r_out, mat='rubber', tone=0.05, **kw):
            lo, hi = sorted((sg * zlo, sg * zhi))
            return ring(f, f'{idp}-{s}', 'rear-wheel', mat, tone, lo, hi, r_in, r_out, **kw)
        if sc <= 100:
            R('rear-tyre', a0, a1, 0.281, 0.3045)
            R('rear-hub', 0.285, 0.345, 0.0, 0.035, mat=al, tone=0.5)
            R('hand-rim', 0.294 if sc == 50 else 0.2925, 0.306 if sc == 50 else 0.3075, 0.2575 if sc == 50 else 0.2535, 0.2718 if sc == 50 else 0.2685, mat=al, tone=0.5, role='detail')
            if sc == 50:
                R('rear-rim', 0.312, 0.347, 0.2716, 0.2806, mat=al, tone=0.5, role='detail')
            f.capsule(f'spoke-{s}', 'rear-wheel', al, 0.5, [0.03, 0.0, sg * 0.33], [0.2755, 0.0, sg * 0.33], 0.0045, 0.0045, role='detail', slender=True, repeat={'count': 12, 'rotationAxis': [0, 0, 1], 'rotationRad': r5(2 * math.pi / 12), 'origin': [0, 0, 0]})
        elif sc == 200:
            R('rear-tyre', a0, a1, 0.2765, 0.3105)
            R('rear-hub', 0.285, 0.345, 0.0, 0.04, mat=al, tone=0.5)
            f.capsule(f'spoke-{s}', 'rear-wheel', al, 0.5, [0.04, 0.0, sg * 0.33], [0.265, 0.0, sg * 0.33], 0.0045, 0.0045, role='detail', slender=True, repeat={'count': 6, 'rotationAxis': [0, 0, 1], 'rotationRad': r5(2 * math.pi / 6), 'origin': [0, 0, 0]})
        else:
            lo, hi = sorted((sg * 0.25, sg * 0.35))
            ring(f, f'rear-tyre-{s}', 'rear-wheel', 'rubber', 0.05, lo, hi, 0.2655, 0.3305)
        c0, c1 = (0.198, 0.233) if sc < 500 else (0.05, 0.125)
        lo, hi = sorted((sg * c0, sg * c1))
        ring(f, f'castor-tyre-{s}', 'castor', 'rubber', 0.05, lo, hi, 0.0, 0.0678)
        if sc == 50:
            lo, hi = sorted((sg * 0.195, sg * 0.236))
            ring(f, f'castor-hub-{s}', 'castor', al, 0.5, lo, hi, 0.0, 0.018, role='detail')
    for sg, s in ((1, 'l'), (-1, 'r')):
        if sc <= 100:
            for tid, shp, kw, zz in TUBES:
                sh = el([shp])
                if not sh:
                    continue
                if tid == 'castor-fork' and sc > 50:
                    pass
                pts, r = chain_from(T, sh, tol=0.005 if sc == 50 else 0.012, n=40, **kw)
                r = max(r, rmin)
                capsule_chain(f, f'{tid}-{s}', 'frame', al, 0.5, pts, sg * zz, r)
            sh = el(['frame-push-handle'])
            pts, r = chain_from(T, sh, tol=0.01, n=20, along='x')
            loc = [f.to_local('frame', [p[0], p[1], 0]) for p in (pts[0], pts[-1])]
            secs = [{'at': [r5(loc[0][0]), r5(loc[0][1]), sg * Z], 'rx': 0.02, 'ry': r5(max(r, rmin)), 'exponent': 2.0, 'rotation': QU}, {'at': [r5(loc[1][0]), r5(loc[1][1]), sg * Z], 'rx': 0.02, 'ry': r5(max(r, rmin)), 'exponent': 2.0, 'rotation': QU}]
            f.loft(f'push-handle-{s}', 'frame', 'rubber', 0.1, secs)
            bl = el(['brake-lever'], roles=('body', 'detail'))
            pts, r = chain_from(T, bl, tol=0.008, n=24, along='y')
            capsule_chain(f, f'brake-lever-{s}', 'frame', al, 0.5, pts, sg * 0.185, max(r, rmin * 0.9), role='detail')
            hs = el(['castor-housing'])
            hx, hy = T.extent(hs, 0), T.extent(hs, 1)
            f.box(f'castor-housing-{s}', 'frame', al, 0.5, [hx[1] - hx[0], hy[1] - hy[0], 0.035], [(hx[0] + hx[1]) / 2, (hy[0] + hy[1]) / 2 - FR, sg * 0.2155], corner=min(0.01, rmin))
            if sc == 50:
                f.box(f'heel-loop-{s}', 'frame', al, 0.5, [0.015, 0.045, 0.02], [0.4675, 0.1225 - FR, sg * 0.09], role='detail')
        elif sc == 200:
            for tid, shp in STROKES[sc]:
                sh = [x for x in T.shapes(sc, 'elevation') if x['id'] == shp]
                pts = [tuple(p) for p in sh[0]['rings'][0]]
                capsule_chain(f, f'{tid}-{s}', 'frame', al, 0.5, pts, sg * (0.2155 if tid == 'castor-fork' else Z), max(rmin, 0.0155))
        else:
            tube = 0.0105
            zz = sg * 0.068
            for tid, a, b, rr, sl in (('back-tube', (-0.115, 0.93), (-0.07, 0.47), tube, True), ('seat-rail', (-0.07, 0.47), (0.42, 0.495), tube, True), ('hanger', (0.42, 0.495), (0.49, 0.1), tube, True), ('lower-rail', (0.0, 0.305), (0.4, 0.3), tube, True), ('push-handle', (-0.17, 0.915), (-0.115, 0.93), 0.0375, False)):
                capsule_chain(f, f'{tid}-{s}', 'frame', al, 0.5, [a, b], zz, rr, **({'slender': True} if sl else {}))
    # cross tubes between the side frames, hidden in the lower-rail strip of the elevation
    xb = 0.2 if sc <= 200 else 0.2
    zc = Z
    if sc <= 200:
        f.capsule('lower-cross', 'frame', al, 0.5, [xb, 0.3 - FR, -zc], [xb, 0.3 - FR, zc], r5(max(rmin, 0.0125 if sc <= 100 else 0.0155)), r5(max(rmin, 0.0125 if sc <= 100 else 0.0155)), role='body')
    # footplate: a plate of the plan outline
    fp_t = max(0.025, FE)
    f.box('footplate', 'frame', al, 0.5, [0.32, r5(fp_t), 0.34], [0.62, 0.0875 - FR, 0.0], corner=r5(min(0.008, rmin)))
    # seat cushion and back sling as pillow slabs
    if sc < 500:
        cu = slab_loft(f, 'seat-cushion', 'frame', 'foam', 0.2, ['seat-cushion'], ['seat-cushion'], ['seat-cushion'], nst={50: 5, 100: 4, 200: 3}[sc], exp=3.6)
    else:
        f.box('seat-cushion', 'frame', 'foam', 0.2, [0.49, 0.075, 0.42], [0.175, 0.5045 - FR, 0.0], corner=0.02)
    if sc <= 200:
        bs = el(['back-sling'], roles=('body',)) or [x for x in T.shapes(sc, 'elevation') if x['id'] == 'back-sling']
        t = max(0.025, FE)
        pts = [(-0.0875, 0.53 + t / 2 - FR + 0.001), (-0.1, 0.70 - FR), (-0.106, 0.86 - t / 2 - FR)]
        vloft(f, 'back-sling', 'frame', 'canvas-sneaker', 0.3, [(-0.0875, 0.53 - FR + t / 2, 0.0), (-0.0975, 0.695 - FR, 0.0), (-0.106, 0.86 - FR - t / 2, 0.0)], t / 2, 0.2, exp=4.0)


_pk = Human.pk
_plan_fit = plan_fit
PFS = {500: 0.0}


def plan_fit(f, bone, secs, pl, weight):
    return _plan_fit(f, bone, secs, pl, min(weight, PFS.get(f.scale, 9.0)))




def _pk500(self, view, pats, roles=('body',)):
    if self.sc == 500 and view == 'elevation':
        np_ = []
        for p in ([pats] if isinstance(pats, str) else pats):
            q = p[:-2] + '-r' if p.endswith('-l') else p
            np_ += [q, 'sil-' + q]
        pats = np_
    return _pk(self, view, pats, roles)


Human.pk = _pk500


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    H, n = standing_body(fig, CFG)
    sc = scale
    fig.body[:] = [x for x in fig.body if x['id'] != 'hem']
    if sc <= 200:
        bn = H.loft('beanie', 'head', 'knit-wool', 0.35, ['beanie'], ['beanie', 'hair'], ['head'], nst={50: 6, 100: 5, 200: 3}[sc], exp=2.2, planfit=0, grow=0.002)
        if sc >= 100:
            q0, q1 = bn['sections'][-2], bn['sections'][-1]
            dome = []
            for t in ((0.55, 0.85) if sc == 100 else (0.7,)):
                f_ = math.sqrt(1 - t * t)
                dome.append({'at': [r5(q0['at'][0] + (q1['at'][0] - q0['at'][0]) * t), r5(q0['at'][1] + (q1['at'][1] - q0['at'][1]) * t), 0.0], 'rx': r5(q0['rx'] * f_), 'ry': r5(q0['ry'] * f_), 'exponent': q0['exponent'], 'rotation': q0['rotation']})
            q1['rx'], q1['ry'] = r5(q0['rx'] * 0.3), r5(q0['ry'] * 0.3)
            bn['sections'][-1:-1] = dome
    if sc == 500:
        for x in fig.body:
            if x['id'].startswith('hand-'):
                x['sections'][-1]['at'][0] = 0.085
                x['sections'][-2]['at'][0] = 0.016
                for q in x['sections']:
                    q['at'][1] = 0.0
                    q['at'][2] = 0.0
        fig.body[:] = [x for x in fig.body if x['id'] != 'neck']
        nk = fig.loft('neck', 'neck', CFG['mat']['shirt'], CFG['tone']['shirt'], [{'at': [r5(u), 0.0, 0.0], 'rx': 0.06, 'ry': 0.1, 'exponent': 2.4, 'rotation': QU} for u in (-0.03, 0.11)])
        fig.body.remove(nk)
        fig.body.insert(1, nk)
    n0 = len(fig.body)
    chair_body(fig, H)
    chair_ids = [x['id'] for x in fig.body[n0:]]
    standing_thermal(fig, CFG)
    have = {x['id'] for x in fig.body}
    L = lambda r: region_layers(T, r)
    if 'beanie' in have:
        thermal_layers(fig, 'beanie', ['beanie'], L('beanie'))
    else:
        hs = fig.find('head')
        q = hs['sections'][-2]
        fig.solid('thermal', 'th-beanie', 'head', 'knit-wool', 0.35, 'ellipsoid', radii=[0.075, r5(q['ry'] * 1.05), r5(q['rx'] * 1.05)], transform={'translation': [r5(q['at'][0]), r5(q['at'][1]), 0.0]}, region='beanie', layer='beanie')
    FE = FEATURE[sc]
    for s_ in 'lr':
        rr_ = {50: [0.022, 0.016, 0.055], 100: [0.022, 0.016, 0.055], 200: [0.0165, 0.0165, 0.05], 500: [0.0375, 0.0375, 0.05]}[sc]
        fig.solid('thermal', f'th-fingertips-{s_}', f'hand-{s_}', 'skin', 0.27, 'ellipsoid', radii=rr_, transform={'translation': [0.18 if sc <= 200 else 0.085, 0.0, 0.0]}, region='hand-skin')
    for i in chair_ids:
        b = fig.find(i)
        if i.startswith(('rear-tyre', 'castor-tyre')):
            fig.ref(f'th-{i}', i, 'tyres')
        elif i in ('seat-cushion', 'back-sling'):
            fig.ref(f'th-{i}', i, 'cushion')
        else:
            fig.ref(f'th-{i}', i, 'frame-metal')
    standing_structure(fig, CFG)
    for x in fig.structure:
        if x['id'].startswith(('member-foot', 'member-neck', 'member-hand', 'member-arm')) and x['kind'] == 'capsule' and sc <= 100:
            x['r0'] = r5(max(x['r0'] * 0.85, FE * 0.55)); x['r1'] = x['r0']
    chair_structure(fig, chair_ids)
    return fig, H


def chair_structure(fig, chair_ids):
    f, sc = fig, fig.scale
    FE = FEATURE[sc]
    B = {x['id']: x for x in f.body}
    for i in chair_ids:
        b = B[i]
        if b['kind'] == 'capsule' and i.startswith(('seat-rail', 'back-tube', 'hanger', 'lower-rail', 'lower-front', 'castor-fork', 'lower-cross', 'frame-main', 'frame-lower')):
            r = r5(max(b['r0'] * 0.8, FE * 0.55)) if not b.get('slender') else r5(b['r0'] * 0.9)
            kw = {'slender': True} if b.get('slender') else {}
            f.solid('structure', f'member-{i}', 'frame', 'aluminium', 0.5, 'capsule', p0=b['p0'], p1=b['p1'], r0=r, r1=r, member='frame', part='member', **kw)
        elif b['kind'] == 'box' and i == 'footplate':
            f.solid('structure', 'member-footplate', 'frame', 'aluminium', 0.5, 'box', size=[r5(b['size'][0] * 0.8), r5(max(FE * 1.1, b['size'][1] * 0.7)), r5(b['size'][2] * 0.8)], transform=b['transform'], member='frame', part='member')
    for s_, sg in (('l', 1), ('r', -1)):
        r = r5(max(0.02, FE * 0.55))
        zlo, zhi = (0.305, 0.335) if sc <= 200 else (0.28, 0.33)
        f.solid('structure', f'member-axle-{s_}', 'rear-wheel', 'aluminium', 0.5, 'capsule', p0=[0.0, 0.0, sg * zlo], p1=[0.0, 0.0, sg * zhi], r0=r, r1=r, member='rear-wheel', part='member')
        if sc <= 200:
            R = {50: (0.2716, 0.2806), 100: (0.2716, 0.2806), 200: (0.2765, 0.3105)}[sc]
            if sc == 50:
                lo, hi = sorted((sg * 0.312, sg * 0.347))
                f.solid('structure', f'member-rim-{s_}', 'rear-wheel', 'aluminium', 0.5, 'revolve', profile=[[lo, R[0]], [hi, R[0]], [hi, R[1]], [lo, R[1]], [lo, R[0]]], axis=[0, 0, 1], origin=[0, 0, 0], member='rear-wheel', part='member')
            f.capsule(f'member-spoke-{s_}', 'rear-wheel', 'aluminium', 0.5, [0.04, 0.0, sg * 0.33], [0.265, 0.0, sg * 0.33], 0.0045, 0.0045, role='body', lst='structure', slender=True, member='rear-wheel', part='member', repeat={'count': 12 if sc <= 100 else 6, 'rotationAxis': [0, 0, 1], 'rotationRad': r5(2 * math.pi / (12 if sc <= 100 else 6)), 'origin': [0, 0, 0]})
        else:
            lo, hi = sorted((sg * 0.26, sg * 0.34))
            f.solid('structure', f'member-rim-{s_}', 'rear-wheel', 'aluminium', 0.5, 'revolve', profile=[[lo, 0.2775], [hi, 0.2775], [hi, 0.3185], [lo, 0.3185], [lo, 0.2775]], axis=[0, 0, 1], origin=[0, 0, 0], member='rear-wheel', part='member')
        f.solid('structure', f'joint-axle-{s_}', 'rear-wheel', 'aluminium', 0.5, 'ellipsoid', radii=[r5(r * 1.2)] * 3, transform={'translation': [0.0, 0.0, sg * 0.315]}, member='rear-wheel', part='joint')


def decisions(fg, sc):
    nb, nt, ns = len(fg.body), len(fg.thermal), len(fg.structure)
    body = {
        50: (f"1:50 detail model, {nb} body solids. The seated person is built as in the standing family (every segment a superelliptic loft cut from this scale's elevation depth and front/plan width; eyes, ears, nose, thumbs, cuffs, hem, soles, beanie as own solids), the thighs lie level on the cushion and the shanks drop to the footplate. The chair is modelled as the object it is: every frame tube (seat rail, back tube, push-handle grip, hanger, lower rail, lower front tube, castor fork) is a chain of round capsules along the centre line that was extracted from the drawn tube outline, one per side at the lateral position of the front and plan views, never a box of the outline; the swivel housings are small blocks, the footplate is a plate with two heel loops, the brake levers are capsule chains, the seat cushion is a pillow loft whose depth follows the wedge outline, the back sling a thin vertical loft. Each rear wheel is a revolved tyre ring (inner radius 281 mm, outer 304.5 mm, 45 mm wide), a hub cylinder 60 mm long, a thin rim ring, a hand rim 12 mm wide inboard of the tyre and 12 slender spokes laced to the middle of the tyre; the castors are solid discs with a proud hub. Spokes and the second castor spokes drawn inside the castor disc are not modelled (inside the disc).",
             f"Detailmodell 1:50 mit {nb} Körper-Solids. Die sitzende Person entsteht wie in der Stehenden-Familie (jedes Segment ein superelliptischer Loft aus Tiefe der Seitenansicht und Breite von Vorderansicht und Grundriss; Augen, Ohren, Nase, Daumen, Bündchen, Saum, Sohlen und Mütze als eigene Solids), die Oberschenkel liegen waagrecht auf dem Kissen, die Unterschenkel fallen zur Fussplatte. Der Stuhl ist als das Objekt modelliert, das er ist: jedes Rahmenrohr (Sitzschiene, Rückenrohr, Schiebegriff, Beinstütze, untere Schiene, unteres Frontrohr, Lenkrollengabel) ist eine Kette runder Kapseln entlang der Mittellinie, die aus dem gezeichneten Rohrumriss gewonnen wurde, je eine pro Seite in der seitlichen Lage von Vorderansicht und Grundriss, nie eine Box des Umrisses; die Lenkrollengehäuse sind kleine Blöcke, die Fussplatte eine Platte mit zwei Fersenschlaufen, die Bremshebel Kapselketten, das Sitzkissen ein Kissen-Loft, dessen Tiefe dem Keilumriss folgt, die Rückenbespannung ein dünner senkrechter Loft. Jedes Hinterrad ist ein gedrehter Reifenring (Innenradius 281 mm, Aussenradius 304,5 mm, 45 mm breit), ein 60 mm langer Nabenzylinder, ein dünner Felgenring, ein 12 mm breiter Greifreifen innerhalb des Reifens und 12 schlanke Speichen zur Reifenmitte; die Lenkrollen sind volle Scheiben mit vorstehender Nabe. Speichen und die in die Lenkrollenscheibe gezeichneten Speichen sind nicht modelliert (liegen in der Scheibe)."),
        100: (f"1:100 model, {nb} body solids, modelled anew against the 1:100 drawings: fewer stations per loft, the cuffs, hem, soles, eyes and ears merged into their parents, nose and thumbs kept. The chair keeps round capsule tubes, but each tube chain is simplified to the bends that are wider than a smallest feature (15 mm), the castor forks, swivel housings and brake levers stay, the heel loops go; the rear wheel is the tyre ring, the hub, the 12 slender spokes and the hand rim (drawn at 1:100 as a single hoop and modelled as a 15 mm ring, the minimum), the thin rim ring is merged into the tyre.",
              f"Modell 1:100 mit {nb} Körper-Solids, neu gegen die Zeichnungen 1:100 modelliert: weniger Stationen je Loft, Bündchen, Saum, Sohlen, Augen und Ohren in ihre Elternteile verschmolzen, Nase und Daumen bleiben. Der Stuhl behält runde Kapselrohre, jede Rohrkette ist aber auf die Knicke vereinfacht, die breiter als ein kleinstes Merkmal (15 mm) sind; Lenkrollengabeln, Gehäuse und Bremshebel bleiben, die Fersenschlaufen entfallen; das Hinterrad besteht aus Reifenring, Nabe, 12 schlanken Speichen und Greifreifen (auf 1:100 als ein Reif gezeichnet und als 15-mm-Ring, das Minimum, modelliert), der dünne Felgenring ist im Reifen aufgegangen."),
        200: (f"1:200 model, {nb} body solids: the person is generalised to 4-6 stations per limb, hood and beanie stay as masses. The 2D draws the chair as open strokes, so every tube is a capsule chain along its stroke (31 mm diameter, the smallest allowed), the sling is a 30 mm vertical sheet, the footplate a plate 30 mm thick. The rear tyre is a 33 mm ring whose centre radius is the drawn hoop (the drawing gives the tyre as a stroke), with a hub and six slender spokes; the hand rim, rim ring, brake levers, heel loops and swivel housings are merged into the tyre, the hub and the tubes.",
              f"Modell 1:200 mit {nb} Körper-Solids: die Person ist auf 4-6 Stationen je Glied verallgemeinert, Kapuze und Mütze bleiben als Massen. Die 2D zeichnet den Stuhl als offene Striche, daher ist jedes Rohr eine Kapselkette entlang seines Strichs (31 mm Durchmesser, das kleinste Erlaubte), die Bespannung ein 30 mm dickes senkrechtes Blech, die Fussplatte eine 30 mm dicke Platte. Der Hinterreifen ist ein 33 mm breiter Ring mit dem gezeichneten Reif als Mittelradius (die Zeichnung gibt den Reifen als Strich), mit Nabe und sechs schlanken Speichen; Greifreifen, Felgenring, Bremshebel, Fersenschlaufen und Lenkrollengehäuse sind in Reifen, Nabe und Rohren aufgegangen."),
        500: (f"1:500 model, {nb} body solids: a generalised figure of broad lofts (hands, toes and limbs at least 75 mm thick); the right-side silhouettes of the elevation (the 1:500 elevation draws only the near limbs) give both sides' depth, the hands are short stubs on the forearm lofts, the neck is a short loft that closes the drawn gap between torso and head. The chair has no drawn spokes, hub, rim or tubes in front or plan, so the wheel is one ring 66 mm wide radially and 100 mm across (the drawn hoop is 65 mm thick), the four frame tubes per side are slender capsules of 21 mm (true tube size) that run along the drawn strokes and, laterally, inside the leg footprint where the front and plan views hide them, the grip is a 75 mm capsule, the footplate a 75 mm plate, the castor a 75 mm disc placed behind the shoe; no cross tube, no brake levers, no heel loops, no sling.",
              f"Modell 1:500 mit {nb} Körper-Solids: eine verallgemeinerte Figur aus breiten Lofts (Hände, Zehen und Glieder mindestens 75 mm dick); die Silhouetten der rechten Seite aus der Seitenansicht (die Ansicht 1:500 zeichnet nur die nahen Glieder) liefern die Tiefe beider Seiten, die Hände sind kurze Stummel auf den Unterarm-Lofts, der Hals ein kurzer Loft, der die gezeichnete Lücke zwischen Rumpf und Kopf schliesst. Der Stuhl hat auf 1:500 weder Speichen, Nabe, Felge noch Rohre in Vorderansicht und Grundriss, daher ist das Rad ein Ring (66 mm radial, 100 mm breit; der gezeichnete Reif ist 65 mm dick), die vier Rahmenrohre je Seite sind schlanke Kapseln von 21 mm (echte Rohrgrösse) entlang der gezeichneten Striche und seitlich innerhalb des Beinfussabdrucks, wo Vorder- und Draufsicht sie verdecken, der Griff eine 75-mm-Kapsel, die Fussplatte eine 75-mm-Platte, die Lenkrolle eine 75-mm-Scheibe hinter dem Schuh; kein Querrohr, keine Bremshebel, keine Fersenschlaufen, keine Bespannung."),
    }[sc]
    th = (f"Thermal, {nt} solids: every region is bound to the solids it covers by reference. Face, hoodie, jeans and shoes use the body lofts of the person ({'nested layer shells for the multi-layer regions at this scale, each shrunk by the thickness of the layers outside' if sc == 50 else 'the outer layer only, the thin inner layers are merged'}); the beanie is a shell over the head loft; the bare fingertips are real small solids that protrude from the glove ends (they are no body surface); the tyre region references the tyre rings and castor discs, the frame-metal region every tube, hub, rim, spoke and plate, the cushion region the seat pillow and the sling. Exposed surface is what the model gives; areaM2 of the body regions (C18) was not touched, the deviations are findings reported with the check.",
          f"Thermik, {nt} Solids: jede Region ist über Verweise an die Solids gebunden, die sie bedeckt. Gesicht, Hoodie, Jeans und Schuhe nutzen die Körper-Lofts der Person ({'verschachtelte Lagenschalen für die mehrlagigen Regionen auf diesem Massstab, je um die Dicke der aussen liegenden Lagen geschrumpft' if sc == 50 else 'nur die äussere Lage, die dünnen inneren Lagen sind verschmolzen'}); die Mütze ist eine Schale über dem Kopf-Loft; die freien Fingerspitzen sind echte kleine Solids, die aus den Handschuhenden ragen (keine Körperfläche); die Reifenregion verweist auf Reifenringe und Lenkrollenscheiben, die Region frame-metal auf jedes Rohr, jede Nabe, Felge, Speiche und Platte, die Kissenregion auf Sitzkissen und Bespannung. Die frei liegende Fläche ist, was das Modell liefert; areaM2 der Körperregionen (C18) wurde nicht angetastet, die Abweichungen sind Befunde, die mit dem Check berichtet werden.")
    st = (f"Structure, {ns} solids: the person carries a capsule per load-bearing bone along the loft centre line, joint ellipsoids at hip, knee, ankle, shoulder, elbow and neck (merged to the major joints at coarse scales) and compact mass ellipsoids at torso, pelvis and thighs; the chair frame member is one core per tube chain inside its tube ({'0.8 of the tube radius' if sc <= 200 else 'slender 21 mm cores'}), the footplate core and a lower cross tube (1:50 to 1:200); the rear-wheel member is the axle capsule inside the hub, the rim ring and the spokes, with a joint at each wheel centre. The ground contacts are not structure supports in this asset (structure.supports is empty), the load path runs seat rail, hanger, footplate and through the axle to the tyre rings.",
          f"Tragwerk, {ns} Solids: die Person trägt je tragendem Knochen eine Kapsel entlang der Loft-Mittellinie, Gelenk-Ellipsoide an Hüfte, Knie, Knöchel, Schulter, Ellbogen und Hals (auf grossen Massstäben auf die grossen Gelenke verschmolzen) und kompakte Massen-Ellipsoide an Rumpf, Becken und Oberschenkeln; das Rahmen-Glied ist je Rohrkette ein Kern im Rohr ({'0,8 des Rohrradius' if sc <= 200 else 'schlanke 21-mm-Kerne'}), ein Fussplattenkern und ein unteres Querrohr (1:50 bis 1:200); das Hinterrad-Glied ist die Achskapsel in der Nabe, der Felgenring und die Speichen mit je einem Gelenk im Radmittelpunkt. Die Bodenkontakte sind in diesem Asset keine Tragwerks-Auflager (structure.supports ist leer), der Lastpfad läuft über Sitzschiene, Beinstütze, Fussplatte und durch die Achse zu den Reifenringen.")
    return [{'en': body[0], 'de': body[1]}, {'en': th[0], 'de': th[1]}, {'en': st[0], 'de': st[1]}]


if __name__ == '__main__':
    only = int(sys.argv[1]) if len(sys.argv) > 1 else None
    path = os.path.join(TICKET, 'volumes', 'humans', SLUG + '.json')
    cur = json.load(open(path))['volumes']
    for sc in SCALES:
        if only and sc != only:
            continue
        fg, H = make(sc)
        v = cur[str(sc)]
        v['body'], v['thermal'], v['structure'] = fg.body, fg.thermal, fg.structure
        v['decisions'] = decisions(fg, sc)
        print(sc, len(fg.body), len(fg.thermal), len(fg.structure))
    json.dump({'volumes': cur}, open(os.path.join(TICKET, '🗑️generated', 'author-27', SLUG + '.work.json'), 'w'), ensure_ascii=False, separators=(',', ':'))
    print('wrote work file')
