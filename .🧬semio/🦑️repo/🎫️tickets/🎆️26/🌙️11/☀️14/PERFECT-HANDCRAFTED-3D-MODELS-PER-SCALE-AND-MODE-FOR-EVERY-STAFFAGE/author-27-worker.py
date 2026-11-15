import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-27-lib.py')).read())

_plan_fit = plan_fit


def plan_fit(f, bone, secs, pl, weight):
    return _plan_fit(f, bone, secs, pl, min(weight, PFS.get(f.scale, PF)))


import os as _os
PF = float(_os.environ.get('A27_PF', '0.0'))
PFS = {50: 0.25, 100: 0.25}
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
    else:
        H.loft('helmet', 'head', 'hdpe-shell', 0.75, ['head-helmet'], ['sil-head'], ['head-helmet'], nst=3, exp=2.2, planfit=0, minr=0.0385)

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
        f.solid('thermal', 'th-helmet', 'head', 'hdpe-shell', 0.75, 'ellipsoid', radii=[0.04, q['ry'] * 0.95, q['rx'] * 0.95], transform={'translation': [r5(q['at'][0]), r5(q['at'][1]), 0.0]}, region='helmet', layer=hl[-1]['id'])
    thermal_layers(f, 'torso-clothing', ['torso'], region_layers(fig.T, 'torso-clothing'))
    fig.ref('th-box', 'box', 'box-surface')


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    H, n = standing_body(fig, CFG)
    fig.body[:] = [x for x in fig.body if x['id'] not in ('sole-l', 'sole-r', 'toe-sole-l', 'toe-sole-r', 'hem')]
    extras_body(fig, H)
    cfg = dict(CFG)
    if scale == 500:
        cfg['plume_du'] = 0.205
    standing_thermal(fig, cfg)
    extras_thermal(fig)
    if scale == 500:
        for x in fig.thermal:
            if x['id'] in ('th-hair-cap', 'th-helmet') and x['kind'] == 'ellipsoid':
                rr = x.pop('radii')
                x['kind'] = 'box'
                x['size'] = [r5(2 * rr[0]), r5(2 * rr[1]), r5(2 * rr[2])]
    standing_structure(fig, CFG)
    for x in fig.structure:
        if x['id'].startswith(('member-foot', 'member-toe', 'member-upper-arm')) and x['kind'] == 'capsule' and scale <= 100:
            x['r0'] = r5(max(x['r0'] * 0.8, FEATURE[scale] * 0.55)); x['r1'] = x['r0']
    b = fig.find('box')
    for bone, wc in (('box-r', 0.1), ('box-l', -0.1)):
        sz = [r5(b['size'][0] * 0.8), r5(b['size'][1] * 0.8), r5(0.17)]
        at = [b['transform']['translation'][0], b['transform']['translation'][1], wc]
        fig.solid('structure', f'member-{bone}', bone, 'cardboard', 0.55, 'box', size=sz, transform={'translation': at}, member=bone, part='member')
        fig.solid('structure', f'mass-{bone}', bone, 'cardboard', 0.55, 'ellipsoid', radii=[0.08, 0.08, 0.06], transform={'translation': [at[0], at[1], wc]}, member=bone, part='mass')
    return fig


def decisions(fg, sc):
    nb, nt, ns = len(fg.body), len(fg.thermal), len(fg.structure)
    body = {
        50: (f"1:50 detail model, {nb} body solids. The worker stands upright and holds a cardboard box in front of the belly with both arms bent. Every segment is a loft of superelliptic sections: depth from the elevation outline, width and lateral centre from the front view, with a quarter weight of the plan outline for the depth of the limbs; eyes, ears, nose and thumbs are separate solids, the hard hat is its own loft over the hair loft, the hi-vis vest is the torso and pelvis loft itself (the vest and the shirt under it are layers of that solid), the safety boots are shoe and toe lofts. The box is a rounded box 0.40 m wide, bound to the box bones, exactly the drawn outline with the lateral width of the front view; the strap, tape and handle details of the 2D are not separate solids (they are surface detail below 7.5 mm relief). Soles, hem and cuffs are not modelled: their drawn outlines are lines inside the boot and vest lofts.",
             f"Detailmodell 1:50 mit {nb} Körper-Solids. Der Arbeiter steht aufrecht und hält mit beiden angewinkelten Armen einen Karton vor dem Bauch. Jedes Segment ist ein Loft aus superelliptischen Schnitten: Tiefe aus dem Umriss der Seitenansicht, Breite und seitliche Lage aus der Vorderansicht, mit einem Viertel Gewicht des Grundrissumrisses für die Tiefe der Glieder; Augen, Ohren, Nase und Daumen sind eigene Solids, der Schutzhelm ist ein eigener Loft über dem Haar-Loft, die Warnweste ist der Rumpf- und Becken-Loft selbst (Weste und Hemd darunter sind Lagen dieses Solids), die Sicherheitsstiefel sind Schuh- und Zehenlofts. Der Karton ist eine abgerundete Box von 0,40 m Breite an den Kartonknochen, genau der gezeichnete Umriss mit der seitlichen Breite der Vorderansicht; Band, Klebeband und Griff der 2D sind keine eigenen Solids (Oberflächendetail unter 7,5 mm Relief). Sohlen, Saum und Bündchen sind nicht modelliert: ihre gezeichneten Umrisse sind Linien innerhalb der Stiefel- und Westenlofts."),
        100: (f"1:100 model, {nb} body solids, modelled anew against the 1:100 drawings: fewer stations per loft, eyes, ears and nose merged into the head, thumbs stay, hard hat and hair are separate lofts, the box keeps its rounded corners (15 mm).",
              f"Modell 1:100 mit {nb} Körper-Solids, neu gegen die Zeichnungen 1:100 modelliert: weniger Stationen je Loft, Augen, Ohren und Nase im Kopf aufgegangen, Daumen bleiben, Schutzhelm und Haar sind getrennte Lofts, der Karton behält seine gerundeten Ecken (15 mm)."),
        200: (f"1:200 model, {nb} body solids: limbs of 4-6 stations, helmet and hair as masses, hands and toes stay separate solids because their bones are drawn, the box is a rounded 30 mm box.",
              f"Modell 1:200 mit {nb} Körper-Solids: Glieder aus 4-6 Stationen, Helm und Haar als Massen, Hände und Zehen bleiben eigene Solids, weil ihre Knochen gezeichnet sind, der Karton ist eine gerundete 30-mm-Box."),
        500: (f"1:500 model, {nb} body solids: broad lofts (hands, toes and limbs at least 75 mm thick), the drawing's helmet outline is its own 75 mm loft over the head, no hair, the right-hand silhouettes of the elevation give the depth of both sides (the 1:500 elevation draws the near limbs), the box a rounded 75 mm box. The plan outline weights are zero at this scale: it would thin the legs to the plan's narrower footprint.",
              f"Modell 1:500 mit {nb} Körper-Solids: breite Lofts (Hände, Zehen und Glieder mindestens 75 mm dick), der Helmumriss der Zeichnung ist ein eigener 75-mm-Loft über dem Kopf, kein Haar, die Silhouetten der Seitenansicht liefern die Tiefe beider Seiten (die Ansicht 1:500 zeichnet die nahen Glieder), der Karton eine gerundete 75-mm-Box. Das Gewicht des Grundrissumrisses ist auf diesem Massstab null: es würde die Beine auf die schmalere Grundrissfläche ausdünnen."),
    }[sc]
    th = (f"Thermal, {nt} solids by reference to the body solids: head-skin on head, hair on the hair loft, helmet as nested shells at 1:50 (hair layer, air layer, shell) and the shell alone at coarser scales, the shirt layer and the vest layer both on the torso loft (the vest is the outer layer, the shirt is the inner), arms, trousers, gloves, boots and the box by reference; emitters as tagged jets at the head anchors. The body regions deviate from areaM2 beyond 30 % in several places (helmet, shirt, gloves, boots at 1:50 and 1:100): this is a finding about the model (the shirt layer is buried by the vest on the same solid, the hard hat loft is a full dome) and about the declared areas, reported with the check; the declared areas are untouched (C18).",
          f"Thermik, {nt} Solids über Verweise auf die Körper-Solids: head-skin am Kopf, hair am Haar-Loft, helmet als verschachtelte Schalen bei 1:50 (Haarlage, Luftlage, Schale) und auf gröberen Massstäben nur die Schale, die Hemdlage und die Westenlage beide auf dem Rumpf-Loft (die Weste ist die äussere Lage, das Hemd die innere), Arme, Hose, Handschuhe, Stiefel und der Karton über Verweise; Emitter als markierte Strahlen an den Kopfankern. Die Körperregionen weichen an mehreren Stellen mehr als 30 % von areaM2 ab (Helm, Hemd, Handschuhe, Stiefel bei 1:50 und 1:100): das ist ein Befund über das Modell (die Hemdlage ist durch die Weste auf demselben Solid verdeckt, der Helm-Loft ist eine volle Kuppel) und über die deklarierten Flächen, mit dem Check berichtet; die deklarierten Flächen bleiben unberührt (C18).")
    st = (f"Structure, {ns} solids: a capsule per load-bearing bone along the loft centre line, joint ellipsoids at the major joints, mass ellipsoids at torso, pelvis and thighs; the load is carried by two box members in the box bones (the 80 % core of the box with the real 0.17 m lateral extent) and a mass ellipsoid per half box at the box centre. The model declares no supports.",
          f"Tragwerk, {ns} Solids: eine Kapsel je tragendem Knochen entlang der Loft-Mittellinie, Gelenk-Ellipsoide an den grossen Gelenken, Massen-Ellipsoide an Rumpf, Becken und Oberschenkeln; die Last tragen zwei Box-Glieder in den Kartonknochen (der 80-%-Kern des Kartons mit der echten seitlichen Ausdehnung von 0,17 m) und je Kartonhälfte ein Massen-Ellipsoid in der Kartonmitte. Das Modell deklariert keine Auflager.")
    return [{'en': body[0], 'de': body[1]}, {'en': th[0], 'de': th[1]}, {'en': st[0], 'de': st[1]}]


if __name__ == '__main__':
    only = int(sys.argv[1]) if len(sys.argv) > 1 else None
    path = os.path.join(TICKET, 'volumes', 'humans', SLUG + '.json')
    cur = json.load(open(path))['volumes']
    for sc in SCALES:
        if only and sc != only:
            continue
        fg = make(sc)
        v = cur[str(sc)]
        v['body'], v['thermal'], v['structure'] = fg.body, fg.thermal, fg.structure
        v['decisions'] = decisions(fg, sc)
        print(sc, len(fg.body), len(fg.thermal), len(fg.structure))
    json.dump({'volumes': cur}, open(os.path.join(TICKET, '🗑️generated', 'author-27', SLUG + '.work.json'), 'w'), ensure_ascii=False, separators=(',', ':'))
    print('wrote work file')
