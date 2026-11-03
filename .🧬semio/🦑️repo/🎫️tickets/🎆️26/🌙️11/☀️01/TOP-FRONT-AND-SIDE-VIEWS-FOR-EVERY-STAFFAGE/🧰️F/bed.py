from lib import *


def mattress(s, view, sc):
    for n in ('head', 'mid', 'foot'):
        s.solid(view, f'mat-{n}', f'mat-{n}', -0.41, 0.41)
    if view == 'front' and sc in ('100', '50'):
        for n in ('head', 'mid', 'foot'):
            s.solid(view, f'slats-{n}', f'slats-{n}', -0.43, 0.43)


def build(s, sc):
    F, P = 'front', 'plan'
    mattress(s, F, sc)
    mattress(s, P, sc)
    if sc == '500':
        s.rect(F, 'head', -0.45, 0.0, 0.9, 0.95, like='head', z=1)
        s.rect(F, 'foot', -0.45, 0.0, 0.9, 0.55, like='foot')
        s.rect(P, 'head', 0.0, -0.45, 0.1, 0.9, like='head')
        s.rect(P, 'foot', 2.02, -0.45, 0.1, 0.9, like='foot')
        return ('Front and plan: headboard and footboard as panels and the three mattress sections as extruded elevation profiles, so the head section tilts up above the footboard in the front view and shortens in the plan.',
                'Vorderansicht und Draufsicht: Kopf- und Fußteil als Flächen und die drei Matratzenabschnitte als gezogene Seitenansichtsprofile, so kippt der Kopfabschnitt in der Vorderansicht über das Fußteil und verkürzt sich im Grundriss.')
    if sc == '200':
        s.rect(F, 'head', -0.45, 0.0, 0.9, 0.95, like='head', z=1, r=0.02)
        for i, g in enumerate((1, -1)):
            s.rect(F, f'foot-post-{i}', g * 0.42 - 0.03, 0.0, 0.06, 0.55, like='foot')
        s.rect(F, 'foot-rail', -0.45, 0.47, 0.9, 0.08, like='foot')
        s.rect(F, 'rail-end', -0.45, 0.14, 0.9, 0.16, like='rail', z=4.5)
        s.rect(P, 'head', 0.0, -0.45, 0.06, 0.9, like='head')
        s.rect(P, 'foot', 2.06, -0.45, 0.06, 0.9, like='foot')
        for i, g in enumerate((1, -1)):
            s.rect(P, f'rail-{i}', 0.04, g * 0.43 - 0.02, 2.04, 0.04, like='rail', z=4.5)
        return ('Front and plan: headboard, the open footboard of two posts and a top rail, the side rails and the three mattress sections as extruded elevation profiles that tilt and sink with their bones.',
                'Vorderansicht und Draufsicht: Kopfteil, das offene Fußteil aus zwei Pfosten und einem oberen Riegel, die Seitenholme und die drei Matratzenabschnitte als gezogene Seitenansichtsprofile, die mit ihren Knochen kippen und einsinken.')
    s.path(F, 'head', 'M -0.45 0 L -0.45 0.93 Q -0.45 0.95 -0.43 0.95 L 0.43 0.95 Q 0.45 0.95 0.45 0.93 L 0.45 0 Z', like='head', z=1)
    for i, g in enumerate((1, -1)):
        if sc == '100':
            s.rect(F, f'foot-post-{i}', g * 0.42 - 0.03, 0.0, 0.06, 0.55, like='foot', r=0.02)
        else:
            s.path(F, f'foot-post-{i}', f'M {g * 0.45} 0 L {g * 0.45} 0.53 Q {g * 0.45} 0.55 {g * 0.43} 0.55 L {g * 0.39} 0.55 L {g * 0.39} 0 Z', like='foot')
    s.rect(F, 'foot-rail', -0.45, 0.47, 0.9, 0.08, like='foot', r=0.02)
    s.rect(F, 'rail-end', -0.45, 0.14, 0.9, 0.16, like='rail', z=4.5)
    s.rect(F, 'leg-mid', -0.03, 0.0, 0.06, 0.16, like='leg-mid', z=4.4)
    s.rect(P, 'head', 0.0, -0.45, 0.06, 0.9, like='head', r=0.02)
    s.rect(P, 'foot', 2.06, -0.45, 0.06, 0.9, like='foot', r=0.02)
    for i, g in enumerate((1, -1)):
        s.rect(P, f'rail-{i}', 0.04, g * 0.43 - 0.02, 2.04, 0.04, like='rail', z=4.5)
    if sc == '100':
        return ('Front and plan: the rounded headboard, the open footboard, the side rails and middle leg, the slat sections (front only) and the three mattress sections as extruded elevation profiles; the head section and its slats tilt with their bone.',
                'Vorderansicht und Draufsicht: das gerundete Kopfteil, das offene Fußteil, die Seitenholme und der Mittelfuß, die Lattenabschnitte (nur Vorderansicht) und die drei Matratzenabschnitte als gezogene Seitenansichtsprofile; der Kopfabschnitt und seine Latten kippen mit ihrem Knochen.')
    return ('Front and plan: the rounded headboard, the open footboard of two rounded posts and a top rail, the side rails and middle leg, the slat sections (front only) and the three quilted mattress sections as extruded elevation profiles at full detail; the head section tilts, and the mattress sections sink, with their bones.',
            'Vorderansicht und Draufsicht: das gerundete Kopfteil, das offene Fußteil aus zwei gerundeten Pfosten und einem oberen Riegel, die Seitenholme und der Mittelfuß, die Lattenabschnitte (nur Vorderansicht) und die drei Matratzenabschnitte als gezogene Seitenansichtsprofile in voller Detaillierung; der Kopfabschnitt kippt und die Matratzenabschnitte sinken mit ihren Knochen.')


def pre(a):
    el = a['representations']['500']['geometry']['elevation']
    el.append({'id': 'leg-mid', 'bone': 'frame', 'role': 'body', 'z': 5, 'filled': True, 'tone': 0.5, 'kind': 'rect', 'x': 1.0, 'y': 0.0, 'w': 0.1, 'h': 0.16, 'region': 'bed-frame', 'material': 'pine-timber'})


def noted(s, sc):
    en, de = build(s, sc)
    if sc == '500':
        en += ' The centre support leg under the middle rail is drawn in the side view.'
        de += ' Das mittlere Stützbein unter dem mittleren Längsholm ist in der Seitenansicht gezeichnet.'
    return en, de


if __name__ == '__main__':
    run('single-bed', noted, pre)
