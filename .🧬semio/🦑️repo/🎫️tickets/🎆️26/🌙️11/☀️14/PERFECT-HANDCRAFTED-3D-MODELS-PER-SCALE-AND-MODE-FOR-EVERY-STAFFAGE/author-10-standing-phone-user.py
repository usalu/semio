import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-10-standing.py')).read())

SLUG = 'standing-phone-user'
CFG = dict(
    mat=dict(skin='skin', hair='hair', shirt='fleece-sweatshirt', trousers='denim', shoe='canvas-sneaker'),
    tone=dict(skin=0.27, hair=0.75, shirt=0.45, rib=0.35, trousers=0.62, shoe=0.12, sole=0.88, hood=0.4),
    nose_clamp=-0.081, plume_r=0.06, plume_du=0.03, torso_pf=0.3, hood_pf=0, toe_wk=1.0, hair_lo=0.12, hood_wk=0.9,
    regions=dict(face='face-skin', hair='hair', hands='hand-skin', top='hoodie', legs='jeans', feet='sneakers'),
    anchors=dict(nostrils=('head', 0.0291, -0.1009), mouth=('head', -0.0049, -0.0777), crown=('head', 0.1697, 0.0)),
    emit={'nose-breath': ('nostrils', 'face-skin'), 'mouth-breath': ('mouth', 'face-skin'), 'body-plume': ('crown', 'hoodie')},
)


def extras_body(fig, H):
    sc = fig.scale
    pe = fig.T.sel(sc, 'elevation', ['ponytail', 'sil-pony'])
    if pe:
        capsule_from(H, 'ponytail', 'head', 'hair', 0.75, ['ponytail', 'sil-pony'], w=0.0, rscale=0.95)
    if sc <= 200:
        for s in 'lr':
            H.blob(f'headphone-cup-{s}', 'neck', 'headphone-polymer', 0.2, ['headphone-cup', 'headphones'], [f'headphone-cup-{s}'], None, role='body', minr=0.0)
    if sc <= 200:
        ph = [x for x in fig.T.shapes(50, 'elevation') if x['id'] == 'phone'][0]
        pts = [fig.to_local('phone', [q[0], q[1], fig.fr('phone')['o'][2]]) for r in ph['rings'] for q in r]
        us = [p[0] for p in pts]; vs = [p[1] for p in pts]
        sz = [max(us) - min(us), max(vs) - min(vs), 0.072]
        sz = [max(x, FEATURE[sc] * 1.05) for x in sz]
        fig.box('phone', 'phone', 'phone-glass-metal', 0.1, sz, [(max(us) + min(us)) / 2, (max(vs) + min(vs)) / 2, 0.0], corner=0.003)


def phone_hidden(fig):
    pass


def phone_solid_geometry(fig):
    b = [s for s in fig.body if s['id'] == 'phone']
    return b[0] if b else None


def extras_thermal(fig):
    sc = fig.scale
    cups = [s['id'] for s in fig.body if s['id'].startswith('headphone-cup')]
    ph = phone_solid_geometry(fig)
    if cups:
        for c in cups:
            fig.ref(f'th-{c}', c, 'headphones')
    else:
        r = 0.04 if sc >= 500 else 0.03
        for s, sgn in (('l', 1), ('r', -1)):
            fig.solid('thermal', f'th-headphone-cup-{s}', 'neck', 'headphone-polymer', 0.2, 'ellipsoid', radii=[0.0375, 0.0375, 0.0375], transform={'translation': [0.04, 0.0, sgn * 0.09]}, region='headphones')
    if ph:
        fig.ref('th-phone', 'phone', 'phone')
    else:
        fig.box('th-phone', 'phone', 'phone-glass-metal', 0.1, [0.146, 0.076, 0.076], [0.05, 0.0, 0.0], lst='thermal', region='phone')


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    H, n = standing_body(fig, CFG)
    extras_body(fig, H)
    standing_thermal(fig, CFG)
    extras_thermal(fig)
    standing_structure(fig, CFG)
    ph = next((x for x in fig.body if x['id'] == 'phone'), None)
    if ph:
        sz = [r5(max(x * 0.8, FEATURE[scale] * 1.02)) for x in ph['size']]
        fig.solid('structure', 'member-phone', 'phone', 'phone-glass-metal', 0.1, 'box', size=sz, transform=ph['transform'], member='phone', part='member')
    else:
        fig.solid('structure', 'member-phone', 'phone', 'phone-glass-metal', 0.1, 'box', size=[0.14, 0.076, 0.076], transform={'translation': [0.05, 0.0, 0.0]}, member='phone', part='member')
    return fig


if __name__ == '__main__':
    vols = {}
    for sc in SCALES:
        fg = make(sc)
        vols[str(sc)] = {'status': 'authored', 'denominator': sc, 'body': fg.body, 'thermal': fg.thermal, 'structure': fg.structure, 'decisions': standing_decisions(fg, 'phone')}
    print(dump(SLUG, 'humans', vols))
