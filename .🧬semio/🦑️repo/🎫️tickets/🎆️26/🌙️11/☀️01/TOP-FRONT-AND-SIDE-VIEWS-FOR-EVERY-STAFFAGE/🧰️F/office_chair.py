import copy
from lib import *
import office_chair_base as base

EXT = lambda a, b: {'kind': 'extruded', 'z0': a, 'z1': b}
SEAT_ONLY = {'500': ['seat-band'], '200': ['seat-cushion'], '100': ['seat-cushion', 'seat-mechanism'], '50': ['seat-cushion', 'seat-mechanism']}
Z = {'seat-band': (-0.24, 0.24), 'seat-cushion': (-0.24, 0.24), 'seat-mechanism': (-0.1, 0.1), 'backrest-shell': (-0.22, 0.22), 'backrest': (-0.22, 0.22), 'backrest-strut': (-0.011, 0.011)}
SOLID_IDS = {
    '500': ['seat-band', 'backrest'],
    '200': ['seat-cushion', 'backrest-shell'],
    '100': ['seat-cushion', 'seat-mechanism', 'backrest-shell', 'armrest-pad-1', 'armrest-pad--1'],
    '50': ['seat-cushion', 'seat-mechanism', 'backrest-shell', 'backrest-strut', 'armrest-post-1', 'armrest-post--1', 'armrest-pad-1', 'armrest-pad--1'],
}


def pre(a):
    bones = {b['id']: b for b in a['rig']['bones']}
    seat = bones['seat']
    seat.update(angleDeg=0, joint={'type': 'revolute', 'minDeg': 0, 'maxDeg': 0})
    seat['mass'].update(comX=0, comY=0)
    bones['backrest'].update(x=0, y=0.23, angleDeg=0)
    for sc, rep in a['representations'].items():
        if rep['status'] != 'authored':
            continue
        el = rep['geometry']['elevation']
        out = []
        for sh in el:
            if sc == '500' and sh['id'] == 'backrest':
                sh = {'id': 'backrest', 'bone': 'backrest', 'role': 'body', 'z': 4, 'filled': True, 'tone': 0.5, 'kind': 'rect', 'x': 0.0, 'y': -0.05, 'w': 0.635, 'h': 0.1, 'region': 'chair-plastic', 'material': 'pa-plastic'}
            if sh['id'] in ('armrest-pad', 'armrest-post') and sc in ('100', '50') and (sh['id'] == 'armrest-pad' or sc == '50'):
                for g in (1, -1):
                    c = copy.deepcopy(sh)
                    c['id'] = f"{sh['id']}-{g}"
                    lat = 0.27
                    half = 0.02 if sh['id'] == 'armrest-pad' else 0.01
                    c['solid'] = EXT(g * lat - half, g * lat + half)
                    out.append(c)
                continue
            if sh['id'] in SOLID_IDS[sc]:
                sh['solid'] = EXT(*Z[sh['id']])
            out.append(sh)
        for sh in out:
            if sh['bone'] == 'seat':
                turn_shape(sh, 0)
        rep['geometry']['elevation'] = out
    low = next(p for p in a['poses'] if p['id'] == 'seat-low')
    q = copy.deepcopy(low)
    q.update(id='swivel-quarter', title={'en': 'Seat swivelled a quarter turn', 'de': 'Sitz eine Vierteldrehung geschwenkt'}, description={'en': 'The seat, armrests and backrest are turned 90 degrees about the column; the star base and the castors stay put.', 'de': 'Sitz, Armlehnen und Rückenlehne sind um 90 Grad um die Säule gedreht; Stern und Rollen bleiben stehen.'})
    q['bones'] = {'column': {'dy': 0}, 'seat': {'twistDeg': 90}}
    r = copy.deepcopy(q)
    r.update(id='swivel-quarter-opposite-key', key=True, title={'en': 'Seat swivelled a quarter turn the other way (key)', 'de': 'Sitz eine Vierteldrehung in die Gegenrichtung (Schlüssel)'}, description={'en': 'The same quarter turn to the opposite side.', 'de': 'Dieselbe Vierteldrehung zur Gegenseite.'})
    r['bones'] = {'column': {'dy': 0}, 'seat': {'twistDeg': -90}}
    k = a['poses'].index(low)
    a['poses'][k + 1:k + 1] = [q, r]
    for c in a['clips']:
        if c['id'] == 'swivel-rotation':
            c['description'] = {'en': 'The seat, armrests and backrest swivel a quarter turn to each side and back about the column while the star base and the five castors stay on the floor.', 'de': 'Sitz, Armlehnen und Rückenlehne schwenken eine Vierteldrehung zu jeder Seite und zurück um die Säule, während Stern und fünf Rollen auf dem Boden bleiben.'}
            c['keys'] = [{'t': 0, 'pose': 'seat-low', 'ease': 'smooth'}, {'t': 1.0, 'pose': 'swivel-quarter', 'ease': 'smooth'}, {'t': 2.0, 'pose': 'seat-low', 'ease': 'smooth'}, {'t': 3.0, 'pose': 'swivel-quarter-opposite-key', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'seat-low', 'ease': 'smooth'}]
            c['events'] = [{'t': 1.0, 'kind': 'motion', 'title': {'en': 'Seat swivels a quarter turn', 'de': 'Sitz schwenkt eine Vierteldrehung'}}, {'t': 3.0, 'kind': 'motion', 'title': {'en': 'Seat swivels to the other side', 'de': 'Sitz schwenkt zur Gegenseite'}}]
    a['structure']['assumptions'][1] = {'en': 'The swivel about the vertical axis is an axial twist of the vertical seat bone; seat cushion, mechanism, armrests and backrest shell are extruded solids, so the side view, front view and plan follow one three-dimensional definition. The two far arms and castors coincide with the near ones.', 'de': 'Die Drehung um die Hochachse ist eine axiale Verdrehung des senkrechten Sitzknochens; Sitzpolster, Mechanik, Armlehnen und Rückenschale sind gezogene Körper, sodass Seitenansicht, Vorderansicht und Grundriss einer dreidimensionalen Definition folgen. Die zwei fernen Arme und Rollen fallen mit den nahen zusammen.'}


EXTRA = (' Seat, mechanism, armrests and the backrest shell are extruded solids shared by all three views, so the swivel turns them about the column axis in side view, front view and plan.',
         ' Sitz, Mechanik, Armlehnen und Rückenschale sind gezogene Körper für alle drei Ansichten, damit die Drehung sie in Seitenansicht, Vorderansicht und Grundriss um die Säulenachse dreht.')


def build(s, sc):
    en, de = base.build(s, sc)
    drop = set(SOLID_IDS[sc]) | {'backrest', 'backrest-shell'}
    s.front[:] = [x for x in s.front if x['id'] not in drop]
    s.plan[:] = [x for x in s.plan if x['id'] not in drop]
    for x in s.plan + s.front:
        if x['id'] == 'seat-piping':
            x['id'] = 'seat-piping-ring'
    for sid in SOLID_IDS[sc]:
        s.twin('front', sid)
        s.twin('plan', sid)
    return en + EXTRA[0], de + EXTRA[1]


if __name__ == '__main__':
    run('office-swivel-chair', build, pre)
