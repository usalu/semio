from lib import *

RL, FL, SL = 0.27, 0.235, 0.18
W = 0.0225


def build(s, sc):
    cm = [m['id'] for m in s.asset['materials'] if 'canvas' in m['id']]
    cmat = cm[0] if cm else None
    E = s.elev
    for view in ('front', 'plan'):
        for i, g in enumerate((1, -1)):
            for name, lat in (('rear', RL), ('front', FL), ('seat', SL)):
                lo, hi = sorted((g * (lat - W), g * (lat + W)))
                s.solid(view, f'{name}-{i}', name, lo, hi, filled=True, role='body', tone=E[name].get('tone', 0.55), z=E[name]['z'] + (0.5 if name == 'rear' else 0))
        if sc == '500':
            cb = {'filled': True, 'role': 'body', 'tone': 0.7, 'regions': ['canvas-back', 'canvas-seat']}
            if cmat:
                cb['material'] = cmat
            s.solid(view, 'canvas-back', 'canvas', -0.2, 0.2, z=2, **cb)
            s.solid(view, 'canvas-seat', 'canvas', -0.2, 0.2, geo={'points': [(0.45, 0.0), (0.0, 0.0)], 'closed': False}, bone='canvas-b', z=2.1, **cb)
        else:
            for n, tone in (('a', 0.8), ('b', 0.65)):
                kw = {'filled': True, 'role': 'body', 'tone': tone}
                if cmat:
                    kw['material'] = cmat
                s.solid(view, f'canvas-{n}', f'canvas-{n}', -0.2, 0.2, z=2 + (0.1 if n == 'b' else 0), **kw)
        if sc in ('500', '200'):
            continue
        if sc == '100':
            for i, g in enumerate((1, -1)):
                lo, hi = sorted((g * 0.2575, g * 0.2925))
                s.solid(view, f'bolt-{i}', 'bolt', lo, hi, filled=True, role='body', tone=0.2, z=6)
            tr = {'geo': {'cx': 1.045, 'cy': 0.0, 'rx': 0.0125, 'ry': 0.0125}, 'kind': 'ellipse'}
            fr = {'geo': {'cx': 0.495, 'cy': 0.0, 'rx': 0.0125, 'ry': 0.0125}, 'kind': 'ellipse'}
            s.solid(view, 'top-rail', 'bolt', -RL, RL, bone='rear', filled=True, role='body', tone=0.4, z=2.5, **tr)
            s.solid(view, 'front-rail', 'bolt', -SL, SL, bone='seat', filled=True, role='body', tone=0.4, z=5.5, **fr)
            continue
        for i, g in enumerate((1, -1)):
            lo, hi = sorted((g * 0.2575, g * 0.2925))
            s.solid(view, f'bolt-{i}', 'bolt', lo, hi, filled=True, role='body', tone=0.2, z=6)
        s.solid(view, 'top-rail', 'top-rail', -RL, RL, filled=True, role='body', tone=0.4)
        s.solid(view, 'front-rail', 'front-rail', -SL, SL, filled=True, role='body', tone=0.4)
    en = {
        '500': ('Front and plan: every frame rail and both canvas panels are the elevation profiles of their bones extruded across the width, so the chair folds flat in all three views; the rear rails sit outermost, the front rails inside them and the seat rails innermost.',
                'Vorder- und Draufsicht: jeder Rahmenholm und beide Bespannungen sind die Seitenansichtsprofile ihrer Knochen, über die Breite gezogen, so klappt der Stuhl in allen drei Ansichten flach zusammen; die hinteren Holme liegen außen, die vorderen innen davon und die Sitzholme ganz innen.'),
        '200': ('Front and plan: the three frame rails per side and the back and seat canvas are the elevation profiles extruded across their widths; the rails are offset sideways so the three frames stay apart as the chair folds.',
                'Vorder- und Draufsicht: die drei Rahmenholme je Seite und die Rücken- und Sitzbespannung sind die über ihre Breite gezogenen Seitenansichtsprofile; die Holme sind seitlich versetzt, damit die drei Rahmen beim Zusammenklappen getrennt bleiben.'),
        '100': ('Front and plan: tube rails, the pivot bolts on the outer rails, the top and front cross rails and the two canvas panels are extruded elevation profiles, so each follows its folding bone.',
                'Vorder- und Draufsicht: Rohrholme, die Drehbolzen an den äußeren Holmen, die obere und vordere Querleiste und die zwei Bespannungen sind gezogene Seitenansichtsprofile, so folgt jedes seinem Faltknochen.'),
        '50': ('Front and plan: timber rails, pivot bolts, the top and front cross rails and the back and seat canvas as extruded elevation profiles at their own lateral positions, so every part folds with its bone; the outer bolts sit proud of the rear rails.',
               'Vorder- und Draufsicht: Holzholme, Drehbolzen, die obere und vordere Querleiste sowie Rücken- und Sitzbespannung als gezogene Seitenansichtsprofile an ihren eigenen Seitenpositionen, so faltet jedes Teil mit seinem Knochen; die äußeren Bolzen stehen vor den hinteren Holmen.'),
    }
    return en[sc]


if __name__ == '__main__':
    run('folding-deck-chair', build)
