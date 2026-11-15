import sys, os
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
exec(open(os.path.join(HERE, 'author-27-lib.py')).read())

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
    if sc == 50:
        H.loft('cap', 'head', 'cap-fabric', 0.3, ['cap'], ['cap'], ['cap'], nst={50: 6, 100: 5, 200: 3}[sc], exp=2.2, planfit=0, grow=0.002)
    for bone, key in (('pointer', 'pointer'), ('flag-pole', 'pole')):
        c = capsule_from(H, bone, bone, 'pointer-aluminium', 0.55, [key], rscale=1.0)
        c['slender'] = True
        c['r1'] = c['r0']
    t = max(0.016, FEATURE[sc] * 1.02)
    box_from(H, 'flag', 'flag', 'flag-textile', 0.4, ['flag'], ['flag'], ['flag'], corner=min(0.004, t / 2 - 0.001), tmin=r5(FEATURE[sc] * 1.02))
    if sc <= 100:
        box_from(H, 'badge', 'torso', 'cotton-clothing', 0.6, ['badge'], ['badge'], None, role='detail', corner=0.003)
        box_from(H, 'pole-grip', 'hand-l', 'skin', 0.3, ['pole-grip'], ['pole-grip'], None, role='detail', corner=0.003)


def extras_thermal(fig):
    sc = fig.scale
    cap = [s['id'] for s in fig.body if s['id'] == 'cap']
    if sc <= 200:
        fig.thermal[:] = [s for s in fig.thermal if s.get('region') != 'hair']
        L = region_layers(fig.T, 'hair')
        thermal_layers(fig, 'hair', ['hair'] + cap, L) if any(s['id'] == 'hair' for s in fig.body) else None
    wr = fig.T.sel(fig.scale, 'elevation', ['wrist-skin-r'], roles=('body', 'detail'))
    fig.solid('thermal', 'th-wrist-skin-r', 'forearm-r', 'skin', 0.27, 'ellipsoid', radii=[max(0.016, FEATURE[sc] * 0.55), max(0.034, FEATURE[sc] * 0.55), max(0.05, FEATURE[sc] * 0.55)], transform={'translation': [0.27, 0.0004, 0.0113]}, region='arm-skin')
    for rid, sid in (('pointer-metal', 'pointer'), ('pole-metal', 'flag-pole'), ('flag-cloth', 'flag')):
        fig.ref(f'th-{sid}', sid, rid)


def make(scale):
    T = Truth(SLUG)
    fig = Fig(T, scale)
    H, n = standing_body(fig, CFG)
    fig.body[:] = [x for x in fig.body if x['id'] not in ('sole-l', 'sole-r', 'toe-sole-l', 'toe-sole-r', 'hem')]
    extras_body(fig, H)
    if scale in (50, 100):
        pl = fig.find('flag-pole')
        pl['r0'] = pl['r1'] = 0.012
    if scale == 500:
        pt = fig.find('pointer')
        pt.pop('slender', None)
        pt['r0'] = pt['r1'] = 0.0375
    standing_thermal(fig, CFG)
    extras_thermal(fig)
    standing_structure(fig, CFG)
    for sid, key in (('pointer', 'pointer'), ('flag-pole', 'pole')):
        b = fig.find(sid)
        kw = {'slender': True} if b.get('slender') else {}
        k = 0.7 if b.get('slender') else 0.95
        fig.solid('structure', f'member-{sid}', sid, 'pointer-aluminium', 0.55, 'capsule', p0=b['p0'], p1=b['p1'], r0=r5(b['r0'] * k), r1=r5(b['r1'] * k), member=sid, part='member', **kw)
    fl = fig.find('flag')
    fig.solid('structure', 'member-flag', 'flag', 'flag-textile', 0.4, 'box', size=[r5(x * 0.85) if i != 2 else r5(max(x * 0.6, FEATURE[sc] * 1.02)) for i, x in enumerate(fl['size'])], transform=fl['transform'], member='flag', part='member')
    return fig


def decisions(fg, sc):
    nb, nt, ns = len(fg.body), len(fg.thermal), len(fg.structure)
    flag_t = {50: '16 mm', 100: '16 mm', 200: '30 mm', 500: '75 mm'}[sc]
    body = {
        50: (f"1:50 detail model, {nb} body solids. The guide stands with the right arm raised in front and the left hand holding the flag pole at hip height. Every body segment is a loft of superelliptic sections whose depth is cut from the elevation outline and whose width and lateral centre come from the front view (plan view for the near-horizontal foot); eyes, ears, nose, thumbs are separate solids; the cap is its own loft over the hair loft (the 2D draws both). The telescopic pointer is a slender capsule (thickness 6 mm to 5 mm along its drawn outline) that leaves the right hand and rises at 45 degrees, the flag pole a slender capsule of 24 mm held in the left hand (grip) from 0.9 m to 2.26 m, and the flag a rounded thin box, {flag_t} thick, standing in the plane of the pole and flying backwards (the wave of the cloth outline is not modelled, the silhouette stays within the drawn wave). Soles, hem and cuffs are not separate solids: their drawn outlines are lines and bands inside the shoe and jacket lofts.",
             f"Detailmodell 1:50 mit {nb} Körper-Solids. Die Führerin steht mit erhobenem rechten Arm vor dem Körper und hält mit der linken Hand die Fahnenstange auf Hüfthöhe. Jedes Körpersegment ist ein Loft aus superelliptischen Schnitten, deren Tiefe der Umriss der Seitenansicht und deren Breite und seitliche Lage der Vorderansicht (beim fast waagrechten Fuss dem Grundriss) entnommen sind; Augen, Ohren, Nase und Daumen sind eigene Solids; die Kappe ist ein eigener Loft über dem Haar-Loft (die 2D zeichnet beide). Der Teleskop-Zeigestock ist eine schlanke Kapsel (6 bis 5 mm dick entlang seines gezeichneten Umrisses), die aus der rechten Hand unter 45 Grad aufsteigt, die Fahnenstange eine schlanke Kapsel von 24 mm, in der linken Hand gehalten, von 0,9 m bis 2,26 m, und die Fahne eine abgerundete dünne Box, {flag_t} dick, in der Ebene der Stange, nach hinten wehend (die Welle des Tuchumrisses ist nicht modelliert, die Silhouette bleibt innerhalb der gezeichneten Welle). Sohlen, Saum und Bündchen sind keine eigenen Solids: ihre gezeichneten Umrisse sind Linien und Bänder innerhalb der Schuh- und Jackenlofts."),
        100: (f"1:100 model, {nb} body solids, modelled anew against the 1:100 drawings: fewer stations per loft, the drawing merges cap and hair into one outline so the hair loft carries the cap (no separate cap), eyes, ears and nose are merged into the head, thumbs stay. Pointer and flag pole are slender capsules (the pole at its true 24 mm, the drawing gives 18 mm in elevation), the flag a thin box of {flag_t}.",
              f"Modell 1:100 mit {nb} Körper-Solids, neu gegen die Zeichnungen 1:100 modelliert: weniger Stationen je Loft, die Zeichnung verschmilzt Kappe und Haar zu einem Umriss, daher trägt der Haar-Loft die Kappe (keine eigene Kappe), Augen, Ohren und Nase sind im Kopf aufgegangen, Daumen bleiben. Zeigestock und Fahnenstange sind schlanke Kapseln (die Stange in ihrer echten Stärke von 24 mm, die Zeichnung gibt in der Seitenansicht 18 mm), die Fahne eine dünne Box von {flag_t}."),
        200: (f"1:200 model, {nb} body solids: limbs generalised to 4-6 stations, hair and cap are one mass, hands and toes remain separate solids. Pointer and pole are slender capsules at the thickness of the repaired 2D (16 mm and 24 mm), the flag is a {flag_t} box (the smallest allowed; the drawing's cloth is 16 mm thick).",
              f"Modell 1:200 mit {nb} Körper-Solids: Glieder auf 4-6 Stationen verallgemeinert, Haar und Kappe sind eine Masse, Hände und Zehen bleiben eigene Solids. Zeigestock und Stange sind schlanke Kapseln in der Stärke der reparierten 2D (16 mm und 24 mm), die Fahne ist eine Box von {flag_t} (das kleinste Erlaubte; das Tuch der Zeichnung ist 16 mm dick)."),
        500: (f"1:500 model, {nb} body solids: a generalised figure of broad lofts, no hair or hood, hands and toes at least 75 mm. The pole stays slender (24 mm, the 2D draws 64 mm), the pointer is a 75 mm capsule (the minimum; it is drawn 60 mm), the flag a {flag_t} box.",
              f"Modell 1:500 mit {nb} Körper-Solids: eine verallgemeinerte Figur aus breiten Lofts, ohne Haar und Kapuze, Hände und Zehen mindestens 75 mm. Die Stange bleibt schlank (24 mm, die 2D zeichnet 64 mm), der Zeigestock ist eine 75-mm-Kapsel (das Minimum; gezeichnet sind 60 mm), die Fahne eine Box von {flag_t}."),
    }[sc]
    th = (f"Thermal, {nt} solids by reference to the body solids: head-skin on head and neck, hair on hair and cap ({'both nested layers at 1:50' if sc == 50 else 'outer layer only'}), jacket layers on torso, pelvis and hood as nested shells at 1:50, sleeves on the arm lofts, trousers on legs, shoes on the shoe and toe lofts, hand-skin on the hands and thumbs, and the right wrist as a real band that protrudes from the sleeve (it is no body surface); pointer, pole and flag by reference. Where the body regions deviate from areaM2 beyond 30 % (C18) it is reported with the check, the declared areas are untouched.",
          f"Thermik, {nt} Solids über Verweise auf die Körper-Solids: head-skin an Kopf und Hals, hair an Haar und Kappe ({'beide verschachtelten Lagen bei 1:50' if sc == 50 else 'nur die äussere Lage'}), die Jackenlagen an Rumpf, Becken und Kapuze als verschachtelte Schalen bei 1:50, die Ärmel an den Armlofts, die Hose an den Beinen, die Schuhe an Schuh- und Zehenlofts, hand-skin an Händen und Daumen und das rechte Handgelenk als echtes Band, das aus dem Ärmel ragt (keine Körperfläche); Zeigestock, Stange und Fahne über Verweise. Weicht eine Körperregion mehr als 30 % von areaM2 ab (C18), wird das mit dem Check berichtet, die deklarierten Flächen bleiben unberührt.")
    st = (f"Structure, {ns} solids: a capsule per load-bearing bone along its loft centre line, joint ellipsoids at the major joints, mass ellipsoids at torso, pelvis and thighs; the pointer and the flag pole are slender member capsules inside their bodies, the flag a member core (the flag is the load at the top of the pole). The model declares no supports.",
          f"Tragwerk, {ns} Solids: eine Kapsel je tragendem Knochen entlang der Loft-Mittellinie, Gelenk-Ellipsoide an den grossen Gelenken, Massen-Ellipsoide an Rumpf, Becken und Oberschenkeln; Zeigestock und Fahnenstange sind schlanke Gliedkapseln in ihren Körpern, die Fahne ein Gliedkern (die Fahne ist die Last am Stangenkopf). Das Modell deklariert keine Auflager.")
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
