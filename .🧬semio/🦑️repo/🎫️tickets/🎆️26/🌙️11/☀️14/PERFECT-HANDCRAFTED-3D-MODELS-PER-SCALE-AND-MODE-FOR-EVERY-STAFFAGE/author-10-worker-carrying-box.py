import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-10-standing.py')).read())

SLUG = 'worker-carrying-box'
CFG = dict(
    mat=dict(skin='skin', hair='hair', shirt='hi-vis-polyester', trousers='cotton-clothing', shoe='leather-shoe'),
    tone=dict(skin=0.27, hair=0.7, shirt=0.6, rib=0.5, trousers=0.35, shoe=0.1, sole=0.85, hood=0.4),
    nose_clamp=None, torso_pf=0.3, hood_pf=0, toe_wk=1.0, hair_lo=0.12, arm='upper-arm-{s}', minr500=True,
    torso_ur=(-0.12, 0.47), pelvis_ur=(-0.115, 0.1), head_ur_front=True,
    ids=dict(head_e=['head', 'head-helmet'], head_f=['head', 'sil-head', 'neck'], torso_e=['torso', 'vest', 'torso-mass'], torso_f=['torso', 'vest', 'sil-body'], pelvis_e=['pelvis', 'hips'], pelvis_f=['pelvis', 'sil-body']),
    regions=dict(face='head-skin', hair='hair', hands='gloves', top='vest', legs='leg-clothing', feet='shoes', arms='arm-clothing'),
    anchors=dict(nostrils=('head', 0.1526, -0.1216), mouth=('head', 0.1153, -0.0961), crown=('head', 0.3206, -0.0166)),
    emit={'breath-nose': ('nostrils', 'head-skin'), 'breath-mouth': ('mouth', 'head-skin'), 'plume': ('crown', 'vest')},
    plume_r=0.09, plume_du=0.03, n={500: dict(hair=0, hood=0)},
)


def extras_body(fig, H):
    sc = fig.scale
    if sc <= 200:
        H.loft('helmet', 'head', 'hdpe-shell', 0.75, ['helmet'], ['helmet'], ['helmet'], nst={50: 6, 100: 5, 200: 4}[sc], exp=2.2, planfit=0, grow=0.002)
    box_from(H, 'box', 'box-r', 'cardboard', 0.55, ['box'], ['box'], ['box'], corner=min(0.01, FEATURE[sc]))


def extras_thermal(fig):
    sc = fig.scale
    f = fig
    have = {s['id'] for s in f.body}
    hl = region_layers(fig.T, 'helmet')
    if 'helmet' in have:
        thermal_layers(f, 'helmet', ['helmet'], hl)
    else:
        hs = f.find('head')
        q = hs['sections'][-2]
        f.solid('thermal', 'th-helmet', 'head', 'hdpe-shell', 0.75, 'ellipsoid', radii=[0.075, q['ry'] * 0.95, q['rx'] * 0.95], transform={'translation': [r5(q['at'][0] + 0.02), r5(q['at'][1]), 0.0]}, region='helmet', layer=hl[-1]['id'])
    thermal_layers(f, 'torso-clothing', ['torso'], region_layers(fig.T, 'torso-clothing'))
    fig.ref('th-box', 'box', 'box-surface')


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    H, n = standing_body(fig, CFG)
    extras_body(fig, H)
    standing_thermal(fig, CFG)
    extras_thermal(fig)
    standing_structure(fig, CFG)
    b = fig.find('box')
    for bone, wc in (('box-r', 0.1), ('box-l', -0.1)):
        sz = [r5(b['size'][0] * 0.8), r5(b['size'][1] * 0.8), r5(0.17)]
        at = [b['transform']['translation'][0], b['transform']['translation'][1], wc]
        fig.solid('structure', f'member-{bone}', bone, 'cardboard', 0.55, 'box', size=sz, transform={'translation': at}, member=bone, part='member')
        fig.solid('structure', f'mass-{bone}', bone, 'cardboard', 0.55, 'ellipsoid', radii=[0.08, 0.08, 0.06], transform={'translation': [at[0], at[1], wc]}, member=bone, part='mass')
    return fig


if __name__ == '__main__':
    vols = {}
    for sc in SCALES:
        fg = make(sc)
        vols[str(sc)] = {'status': 'authored', 'denominator': sc, 'body': fg.body, 'thermal': fg.thermal, 'structure': fg.structure, 'decisions': standing_decisions(fg, 'worker')}
    print(dump(SLUG, 'humans', vols))
