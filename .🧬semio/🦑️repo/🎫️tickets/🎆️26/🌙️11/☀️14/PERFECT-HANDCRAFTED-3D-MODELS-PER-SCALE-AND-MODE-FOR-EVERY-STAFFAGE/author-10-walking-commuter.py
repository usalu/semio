import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-10-standing.py')).read())

SLUG = 'walking-commuter'
CFG = dict(
    mat=dict(skin='skin', hair='hair', shirt='wool-coat', trousers='wool-trouser', shoe='leather-shoe'),
    tone=dict(skin=0.27, hair=0.7, shirt=0.3, rib=0.25, trousers=0.35, shoe=0.1, sole=0.05, hood=0.3),
    nose_clamp=-0.088, torso_pf=0.3, hood_pf=0, toe_wk=1.0, hair_lo=0.12, hood_wk=0.9, torso_ur=(-0.40, 0.469), head_ur_front=True, minr500=True,
    ids=dict(head_e=['head', 'sil-head', 'sil-torso'], head_f=['head', 'sil-head']),
    regions=dict(face='face-skin', hair='hair', hands='hand-skin', top='coat', legs='trousers', feet='shoes'),
    anchors=dict(nostrils=('head', 0.03, -0.104), mouth=('head', -0.005, -0.0801), crown=('head', 0.175, 0.0)),
    emit={'nose-breath': ('nostrils', 'face-skin'), 'mouth-breath': ('mouth', 'face-skin'), 'body-plume': ('crown', 'coat')},
    n={500: dict(hair=0)},
)


def extras_body(fig, H):
    sc = fig.scale
    H.loft('scarf', 'neck', 'wool-scarf', 0.55, ['scarf'], ['scarf'], ['scarf-plan'], nst={50: 5, 100: 4, 200: 3, 500: 0}[sc] or 1, exp=2.2, planfit=0) if sc < 500 else None
    box_from(H, 'bag', 'bag', 'leather-bag', 0.2, ['bag-body', 'sil-bag'], ['bag-body', 'sil-bag'], ['bag'], corner=FEATURE[sc] * 1.2)
    if sc <= 100:
        box_from(H, 'bag-flap', 'bag', 'leather-bag', 0.15, ['bag-flap'], ['bag-flap'], None, role='detail', corner=0.004, grow=0.004)


def extras_thermal(fig):
    if any(s['id'] == 'scarf' for s in fig.body):
        fig.ref('th-scarf', 'scarf', 'scarf', layer='scarf')
    else:
        fig.solid('thermal', 'th-scarf', 'neck', 'wool-scarf', 0.55, 'ellipsoid', radii=[0.0385, 0.075, 0.1], transform={'translation': [0.0, 0.0, 0.0]}, region='scarf', layer='scarf')
    fig.ref('th-bag', 'bag', 'bag-leather')


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    H, n = standing_body(fig, CFG)
    extras_body(fig, H)
    standing_thermal(fig, CFG)
    extras_thermal(fig)
    standing_structure(fig, CFG)
    b = fig.find('bag')
    fig.solid('structure', 'member-bag', 'bag', 'leather-bag', 0.2, 'box', size=[r5(x * 0.8) for x in b['size']], transform=b['transform'], member='bag', part='member')
    return fig


if __name__ == '__main__':
    vols = {}
    for sc in SCALES:
        fg = make(sc)
        vols[str(sc)] = {'status': 'authored', 'denominator': sc, 'body': fg.body, 'thermal': fg.thermal, 'structure': fg.structure, 'decisions': standing_decisions(fg, 'commuter')}
    print(dump(SLUG, 'humans', vols))
