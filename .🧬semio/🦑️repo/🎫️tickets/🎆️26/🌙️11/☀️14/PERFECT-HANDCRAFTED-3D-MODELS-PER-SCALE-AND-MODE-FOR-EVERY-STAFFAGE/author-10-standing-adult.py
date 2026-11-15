import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-10-standing.py')).read())

SLUG = 'standing-adult'
CFG = dict(
    mat=dict(skin='skin', hair='hair', shirt='fleece-sweatshirt', trousers='denim', shoe='canvas-sneaker'),
    tone=dict(skin=0.27, hair=0.75, shirt=0.45, rib=0.35, trousers=0.62, shoe=0.12, sole=0.88, hood=0.4),
    nose_clamp=-0.081,
    torso_exp=3.0,
)


CFG.update(
    regions=dict(face='face-skin', hair='hair', hands='hand-skin', top='hoodie', legs='jeans', feet='sneakers'),
    anchors=dict(nostrils=('head', 0.0291, -0.1009), mouth=('head', -0.0049, -0.0777), crown=('head', 0.1697, 0.0)),
    emit={'nose-breath': ('nostrils', 'face-skin'), 'mouth-breath': ('mouth', 'face-skin'), 'body-plume': ('crown', 'hoodie')},
    torso_pf=0.3, hood_pf=0, torso_exp=2.2, toe_wk=1.0, hair_lo=0.12, hood_wk=0.9,
)


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    standing_body(fig, CFG)
    standing_thermal(fig, CFG)
    standing_structure(fig, CFG)
    return fig


if __name__ == '__main__':
    vols = {}
    for sc in SCALES:
        fg = make(sc)
        vols[str(sc)] = {'status': 'authored', 'denominator': sc, 'body': fg.body, 'thermal': fg.thermal, 'structure': fg.structure, 'decisions': standing_decisions(fg, 'adult')}
    print(dump(SLUG, 'humans', vols))
