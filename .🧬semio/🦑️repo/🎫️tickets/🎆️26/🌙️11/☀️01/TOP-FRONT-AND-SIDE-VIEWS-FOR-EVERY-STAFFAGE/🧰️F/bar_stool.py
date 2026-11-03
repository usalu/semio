from lib import *

RADIUS = 0.14
PAD_Y = 0.72

REV = lambda bone: {'kind': 'revolved', 'axisBone': bone, 'axisX': 0, 'axisZ': 0, 'mirror': True}
EXT = lambda z: {'kind': 'extruded', 'z0': -z, 'z1': z}
KEYS = {
    '500': ['seat', 'back'],
    '200': ['seat', 'back-post', 'back-pad'],
    '100': ['plate', 'pad', 'back-post', 'back-pad'],
    '50': ['plate', 'pad', 'back-post', 'back-pad'],
}
SRC = {'back': 'seat', 'back-post': 'back', 'back-pad': 'back'}


def make(old, sid, bone, kind, geo, solid):
    sh = {k: v for k, v in old.items() if k in ('role', 'z', 'filled', 'tone', 'region', 'regions', 'material', 'hatch', 'penMm')}
    sh.update(id=sid, bone=bone, kind=kind)
    sh.update(geo)
    sh['solid'] = solid
    order = ['id', 'bone', 'role', 'z', 'filled', 'tone', 'kind']
    return {**{k: sh[k] for k in order if k in sh}, **{k: v for k, v in sh.items() if k not in order}}


def built(sc):
    half = lambda pts: {'points': [list(p) for p in pts], 'closed': True}
    if sc == '500':
        return {
            'seat': ('seat', 'poly', half([(0, -0.048), (0.18, -0.048), (0.18, 0.052), (0, 0.052)]), REV('seat')),
            'back': ('backrest', 'rect', {'x': -0.04, 'y': 0.03, 'w': 0.08, 'h': 0.12}, EXT(0.055)),
        }
    if sc == '200':
        return {
            'seat': ('seat', 'poly', half([(0, 0), (0.18, 0), (0.18, 0.052), (0, 0.052)]), REV('seat')),
            'back-post': ('backrest', 'rect', {'x': -0.02, 'y': -0.03, 'w': 0.04, 'h': 0.11}, EXT(0.02)),
            'back-pad': ('backrest', 'rect', {'x': -0.02, 'y': 0.08, 'w': 0.04, 'h': 0.07}, EXT(0.05)),
        }
    knee = 0.018 if sc == '100' else 0.012
    corner = 'Q 0.18 0.03 0.168 0.03' if sc == '100' else 'Q 0.18 0.03 0.162 0.03'
    return {
        'plate': ('seat', 'poly', half([(0, -0.018), (0.174, -0.018), (0.18, -0.012), (0.18, 0.022), (0, 0.022)]), REV('seat')),
        'pad': ('pad', 'path', {'d': f'M 0 -0.01 L 0.18 -0.01 L 0.18 {knee} {corner} L 0 0.03 Z'}, REV('seat')),
        'back-post': ('backrest', 'rect', {'x': -0.015, 'y': 0.0, 'w': 0.03, 'h': 0.09}, EXT(0.015)),
        'back-pad': ('backrest', 'path', {'d': 'M -0.02 0.09 L -0.02 0.135 Q -0.02 0.15 -0.005 0.15 L 0.005 0.15 Q 0.02 0.15 0.02 0.135 L 0.02 0.09 Z'}, EXT(0.05)),
    }


def pre(a):
    bones = {b['id']: b for b in a['rig']['bones']}
    seat = bones['seat']
    seat.update(angleDeg=90, lengthM=0.15, joint={'type': 'revolute', 'minDeg': 0, 'maxDeg': 0})
    seat['title'] = {'en': 'Swivel seat plate (turns about the vertical seat axis)', 'de': 'Drehbare Sitzplatte (dreht um die senkrechte Sitzachse)'}
    seat['mass'].update(comX=0.01, comY=0.0)
    bones['pad'].update(parent='frame', x=0, y=PAD_Y)
    back = bones['backrest']
    back.update(x=0.022, y=RADIUS, angleDeg=0, lengthM=0.15, joint={'type': 'fixed'})
    back['title'] = {'en': 'Low backrest on the swivel seat (radius 0.14 m)', 'de': 'Niedrige Lehne am Drehsitz (Radius 0,14 m)'}
    back['mass'].update(comX=0.1, comY=0.0)
    for sc in ('500', '200', '100', '50'):
        geometry = a['representations'][sc]['geometry']
        old = {s['id']: s for s in geometry['elevation']}
        out = [s for s in geometry['elevation'] if s['bone'] not in ('seat', 'pad', 'backrest')]
        for sid, (bone, kind, geo, solid) in built(sc).items():
            sh = make(old[sid] if sid in old else old[SRC[sid]], sid, bone, kind, geo, solid)
            if sc == '500' and sid == 'back':
                sh.pop('regions', None)
                sh.update(region='backrest', tone=0.35, z=7)
            if sc == '500' and sid == 'seat':
                sh['regions'] = ['seat-pad', 'seat-plate']
            if bone == 'seat':
                turn_shape(sh, 0)
            if bone == 'backrest':
                turn_shape(sh, 0)
            out.append(sh)
        geometry['elevation'] = out
    for p in a['poses']:
        if 'backrest' in p['bones']:
            sign = -1 if p['bones']['backrest']['dx'] < 0 else 1
            del p['bones']['backrest']
            p['bones']['seat'] = {'twistDeg': 90 * sign}
    texts = {
        'turned-90': ('Unoccupied seat turned a quarter turn: the backrest swings round the seat circle from the rear to the side.', 'Unbesetzter Sitz um eine Vierteldrehung gedreht: die Lehne wandert auf dem Sitzkreis vom Heck zur Seite.'),
        'turned-270-key': ('Backrest on the opposite side of the seat circle.', 'Lehne auf der Gegenseite des Sitzkreises.'),
    }
    for p in a['poses']:
        if p['id'] in texts:
            p['description'] = {'en': texts[p['id']][0], 'de': texts[p['id']][1]}
    for c in a['clips']:
        if c['id'] == 'swivel-turn':
            c['title'] = {'en': 'Swivel to both sides', 'de': 'Drehung nach beiden Seiten'}
            c['description'] = {'en': 'The unoccupied seat swings a quarter turn to each side and back in 6 s; the backrest runs round the seat circle.', 'de': 'Der unbesetzte Sitz schwenkt in 6 s eine Vierteldrehung zu jeder Seite und zurück; die Lehne läuft auf dem Sitzkreis.'}
    ass = a['structure']['assumptions']
    ass[0] = {'en': 'The stool is a rigid steel frame on three foot contacts in elevation (the front and rear legs coincide); the seat is a body, the pad sits on a prismatic joint and the backrest turns on a revolute swivel.', 'de': 'Der Hocker ist ein starres Stahlgestell auf drei Fußkontakten in der Ansicht (vordere und hintere Beine fallen zusammen); der Sitz ist ein Körper, das Polster sitzt auf einem Schiebegelenk und die Lehne dreht auf einem Drehgelenk.'}
    ass[1] = {'en': 'The swivel is a rotation about the vertical axis (axial twist of the vertical seat bone). Plate and pad are revolved solids and look the same at every azimuth; the backrest is an extruded solid at radius 0.14 m, 0.10 m wide, so its depth and width show in all three views at every angle.', 'de': 'Die Drehung ist eine Rotation um die Hochachse (axiale Verdrehung des senkrechten Sitzknochens). Platte und Polster sind Rotationskörper und sehen bei jedem Winkel gleich aus; die Lehne ist ein gezogener Körper auf dem Radius 0,14 m, 0,10 m breit, sodass ihre Tiefe und Breite in allen drei Ansichten bei jedem Winkel sichtbar sind.'}


def build(s, sc):
    F, P = 'front', 'plan'
    for sid in KEYS[sc]:
        s.twin(F, sid)
        s.twin(P, sid)
        if sid.startswith('back'):
            s.plan[-1]['z'] = 8
    if sc in ('500', '200'):
        y = 0.66 if sc == '500' else 0.7
        s.poly(F, 'leg-l', [(0.14, y), (0.21, 0.0)], like='leg-l', closed=False)
        s.poly(F, 'leg-r', [(-0.14, y), (-0.21, 0.0)], like='leg-r', closed=False)
        s.poly(F, 'ring', [(-0.195, 0.27), (0.195, 0.27)], like='ring', closed=False)
        s.poly(P, 'leg-x', [(-0.205, 0.0), (0.205, 0.0)], like='leg-l', closed=False)
        s.poly(P, 'leg-y', [(0.0, -0.205), (0.0, 0.205)], like='leg-r', closed=False)
    else:
        s.cap(F, 'leg-l', 0.14, 0.712, 0.1975, 0.0125, 0.0125, like='leg-l')
        s.cap(F, 'leg-r', -0.14, 0.712, -0.1975, 0.0125, 0.0125, like='leg-r')
        s.cap(F, 'leg-m', 0.0, 0.712, 0.0, 0.0125, 0.0125, like='leg-m')
        s.cap(F, 'ring', -0.19, 0.27, 0.19, 0.27, 0.0125, like='ring')
        for n, (a, b, c, d) in {'leg-xp': (0.14, 0, 0.1975, 0), 'leg-xn': (-0.14, 0, -0.1975, 0), 'leg-yp': (0, 0.14, 0, 0.1975), 'leg-yn': (0, -0.14, 0, -0.1975)}.items():
            s.cap(P, n, a, b, c, d, 0.0125, like='leg-l')
        if sc == '50':
            s.rect(F, 'ferrule-l', 0.1825, 0.0, 0.03, 0.022, like='ferrule-l')
            s.rect(F, 'ferrule-r', -0.2125, 0.0, 0.03, 0.022, like='ferrule-r')
            s.rect(F, 'ferrule-m', -0.015, 0.0, 0.03, 0.022, like='ferrule-m')
            s.ell(P, 'ring', 0, 0, 0.195, 0.195, like='ring', filled=False, z=1)
            for n, (a, b) in {'ferrule-xp': (0.1975, 0), 'ferrule-xn': (-0.1975, 0), 'ferrule-yp': (0, 0.1975), 'ferrule-yn': (0, -0.1975)}.items():
                s.ell(P, n, a, b, 0.015, 0.015, like='ferrule-l')
    return NOTES[sc]


NOTES = {
    '500': ('Front view: the revolved seat block and the extruded backrest block, centred above it; the two outer legs and the ring as lines. Plan: the seat disc with the backrest block at the rear edge, only the leg feet stick out as short stubs along both axes. The backrest swivels round the seat circle.',
            'Vorderansicht: der Rotationskörper des Sitzes und der gezogene Lehnenblock mittig darüber; die beiden äußeren Beine und der Ring als Linien. Grundriss: die Sitzscheibe mit dem Lehnenblock am hinteren Rand, nur die Füße der Beine ragen als kurze Stummel entlang beider Achsen heraus. Die Lehne schwenkt auf dem Sitzkreis.'),
    '200': ('Front view: the revolved seat bar, the backrest post and pad as extruded solids above it and the two outer legs with the ring. Plan: the seat disc, the backrest as a short bar on the rear edge that swivels round the seat circle, and four leg stubs that stick out beyond the disc.',
            'Vorderansicht: der Sitzbalken als Rotationskörper, Lehnenpfosten und -polster als gezogene Körper darüber und die zwei äußeren Beine mit dem Ring. Grundriss: die Sitzscheibe, die Lehne als kurzer Balken am hinteren Rand, der auf dem Sitzkreis schwenkt, und vier Beinstummel, die über die Scheibe hinausragen.'),
    '100': ('Front view: revolved seat plate and pad, the extruded backrest post and pad above them, three legs and the ring as tubes. Plan: plate and pad as two concentric discs, the backrest pad as a bar at the rear edge that swivels round the seat circle and the four leg tubes whose feet reach out past the seat.',
            'Vorderansicht: Sitzplatte und Polster als Rotationskörper, darüber Lehnenpfosten und Lehnenpolster als gezogene Körper, drei Beine und der Ring als Rohre. Grundriss: Platte und Polster als zwei konzentrische Scheiben, das Lehnenpolster als Balken am hinteren Rand, der auf dem Sitzkreis schwenkt, und die vier Beinrohre, deren Füße über den Sitz hinausragen.'),
    '50': ('Front view: revolved plate with pad, extruded backrest post and pad above them, three tube legs with their ferrules and the footrest ring. Plan: footrest ring, plate and pad as concentric discs, the backrest pad and post at the rear edge that swivel round the seat circle, and four leg tubes that end in ferrule dots outside the seat; the seat covers the tops of the legs.',
           'Vorderansicht: Platte mit Polster als Rotationskörper, darüber Lehnenpfosten und Lehnenpolster als gezogene Körper, drei Rohrbeine mit Fußkappen und der Fußring. Grundriss: Fußring, Platte und Polster als konzentrische Scheiben, Lehnenpolster und Pfosten am hinteren Rand, die auf dem Sitzkreis schwenken, und vier Beinrohre, die außerhalb des Sitzes in Fußkappenpunkten enden; der Sitz verdeckt die Beinköpfe.'),
}

if __name__ == '__main__':
    run('bar-stool', build, pre)
