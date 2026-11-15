import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-10-standing.py')).read())

SLUG = 'tour-guide'
CFG = dict(
    mat=dict(skin='skin', hair='hair', shirt='softshell', trousers='cotton-clothing', shoe='leather-shoe'),
    tone=dict(skin=0.27, hair=0.7, shirt=0.35, rib=0.3, trousers=0.45, shoe=0.12, sole=0.85, hood=0.4),
    nose_clamp=None, torso_pf=0.3, hood_pf=0, toe_wk=1.0, hair_lo=0.12, arm='upper-arm-{s}', minr500=True,
    torso_ur=(-0.12, 0.46), pelvis_ur=(-0.115, 0.11), head_ur_front=True,
    ids=dict(head_e=['head', 'sil-head', 'figure'], head_f=['head', 'sil-head', 'neck'], torso_e=['torso', 'figure'], torso_f=['torso', 'sil-body'], pelvis_e=['pelvis', 'figure'], pelvis_f=['pelvis', 'torso', 'sil-body']),
    regions=dict(face='head-skin', hair='hair', hands='hand-skin', top='torso-clothing', legs='leg-clothing', arms='arm-clothing', feet='shoes'),
    anchors=dict(nostrils=('head', 0.1423, -0.0993), mouth=('head', 0.1134, -0.0945), crown=('head', 0.2766, 0.0)),
    emit={'nose-breath': ('nostrils', 'head-skin'), 'mouth-breath': ('mouth', 'head-skin'), 'plume': ('crown', 'torso-clothing')},
    plume_r=0.09, plume_du=0.03,
)


def extras_body(fig, H):
    sc = fig.scale
    f = fig
    if sc <= 200:
        H.loft('cap', 'head', 'cap-fabric', 0.3, ['cap'], ['cap'], ['cap'], nst={50: 6, 100: 5, 200: 3}[sc], exp=2.2, planfit=0, grow=0.002)
    for bone, key in (('pointer', 'pointer'), ('flag-pole', 'pole')):
        c = capsule_from(H, bone, bone, 'pointer-aluminium', 0.55, [key], rscale=1.0)
        c['slender'] = True
        c['r1'] = c['r0']
    t = max(0.016, FEATURE[sc] * 1.02)
    box_from(H, 'flag', 'flag', 'flag-textile', 0.4, ['flag'], ['flag'], ['flag'], corner=min(0.004, t / 2 - 0.001))
    if sc <= 100:
        box_from(H, 'badge', 'torso', 'cotton-clothing', 0.6, ['badge'], ['badge'], None, role='detail', corner=0.003)
        box_from(H, 'pole-grip', 'hand-l', 'skin', 0.3, ['pole-grip'], ['pole-grip'], None, role='detail', corner=0.003)


def extras_thermal(fig):
    sc = fig.scale
    cap = [s['id'] for s in fig.body if s['id'] == 'cap']
    fig.thermal[:] = [s for s in fig.thermal if s.get('region') != 'hair']
    L = region_layers(fig.T, 'hair')
    thermal_layers(fig, 'hair', ['hair'] + cap if sc <= 200 else ['hair'], L) if any(s['id'] == 'hair' for s in fig.body) else None
    wr = fig.T.sel(fig.scale, 'elevation', ['wrist-skin-r'], roles=('body', 'detail'))
    fig.solid('thermal', 'th-wrist-skin-r', 'forearm-r', 'skin', 0.27, 'ellipsoid', radii=[0.016, max(0.0075, FEATURE[sc] * 0.55), 0.03], transform={'translation': [0.255, 0.0, 0.0]}, region='arm-skin')
    for rid, sid in (('pointer-metal', 'pointer'), ('pole-metal', 'flag-pole'), ('flag-cloth', 'flag')):
        fig.ref(f'th-{sid}', sid, rid)


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    H, n = standing_body(fig, CFG)
    extras_body(fig, H)
    standing_thermal(fig, CFG)
    extras_thermal(fig)
    standing_structure(fig, CFG)
    for sid, key in (('pointer', 'pointer'), ('flag-pole', 'pole')):
        b = fig.find(sid)
        fig.solid('structure', f'member-{sid}', sid, 'pointer-aluminium', 0.55, 'capsule', p0=b['p0'], p1=b['p1'], r0=r5(b['r0'] * 0.7), r1=r5(b['r1'] * 0.7), member=sid, part='member', slender=True)
    fl = fig.find('flag')
    fig.solid('structure', 'member-flag', 'flag', 'flag-textile', 0.4, 'box', size=[r5(x * 0.85) if i != 2 else r5(x * 0.6) for i, x in enumerate(fl['size'])], transform=fl['transform'], member='flag', part='member')
    return fig


if __name__ == '__main__':
    vols = {}
    for sc in SCALES:
        fg = make(sc)
        vols[str(sc)] = {'status': 'authored', 'denominator': sc, 'body': fg.body, 'thermal': fg.thermal, 'structure': fg.structure, 'decisions': standing_decisions(fg, 'guide')}
    print(dump(SLUG, 'humans', vols))
