from lib import *

B = 'radiator'
doc, _ = load('panel-radiator')
reps = doc['asset']['representations']

LEFT = {'valve-left': 0.06, 'pipe-left': 0.05, 'valve-body-left': 0.05, 'valve-head': 0.05, 'valve-index': 0.05}
RIGHT = {'valve-right': -0.01, 'pipe-right': -0.02, 'valve-body-right': -0.02, 'lockshield-cap': -0.02}


def fixed_front(sc):
    out = []
    for s in reps[sc]['geometry']['front']:
        s = json.loads(json.dumps(s))
        d = LEFT.get(s['id'], RIGHT.get(s['id'], 0))
        if d:
            if s['kind'] == 'rect':
                s['x'] = r4(s['x'] + d)
            else:
                s['points'] = [[r4(a + d), b] for a, b in s['points']]
        out.append(s)
    return out


views = {}
dec = {}
views['200'] = {
    'front': fixed_front('200') + [
        rect('pipe-left', B, 'detail', 3, -0.396, 0.0, 0.022, 0.06, 0.3, region='valve', material='brass'),
        rect('pipe-right', B, 'detail', 3, 0.404, 0.0, 0.022, 0.06, 0.3, region='valve', material='brass'),
    ],
    'plan': [
        rect('radiator-top', B, 'body', 3, 0.035, -0.5, 0.1, 1.0, 0.65, regions=['top-grille', 'convector'], material='painted-steel'),
        line('bracket-left', B, 'detail', 3, [(0.0, 0.4), (0.035, 0.4)], 0.4, region='bracket'),
        line('bracket-right', B, 'detail', 3, [(0.0, -0.4), (0.035, -0.4)], 0.4, region='bracket'),
    ],
}
dec['200'] = {
    'silhouette': (
        'Front view as before plus the two supply pipes down to the floor level of the pipes, with the thermostatic head and the lockshield drawn inside the 1.0 m length (they sit under the panel ends). Plan view: the panel seen from above as one strip (0.5 x 5 mm) with the two brackets as lines in the wall gap.',
        'Vorderansicht wie zuvor plus die beiden Anschlussrohre bis zur Rohrunterkante, Thermostatkopf und Rücklaufverschraubung jedoch innerhalb der Länge von 1,0 m gezeichnet (sie sitzen unter den Paneelenden). Draufsicht: das Paneel von oben als ein Streifen (0,5 x 5 mm) mit den beiden Konsolen als Linien im Wandabstand.'),
    'omitted': (
        'Plan: grille slots, end covers, valve and pipes (hidden below the panel).',
        'Draufsicht: Gitterschlitze, Endabdeckungen, Ventil und Rohre (unter dem Paneel verdeckt).'),
}
views['100'] = {
    'front': fixed_front('100'),
    'plan': [
        rect('top-grille', B, 'detail', 4, 0.035, -0.5, 0.1, 1.0, 0.35, region='top-grille', material='painted-steel'),
        rect('bracket-left', B, 'detail', 3, 0.0, 0.38, 0.035, 0.04, 0.4, region='bracket', pen=0.1),
        rect('bracket-right', B, 'detail', 3, 0.0, -0.42, 0.035, 0.04, 0.4, region='bracket', pen=0.1),
        line('convector-edge', B, 'detail', 5, [(0.085, -0.47), (0.085, 0.47)], 0.4, region='convector', pen=0.1),
    ],
}
dec['100'] = {
    'silhouette': (
        'Front view with the valve parts moved inside the 1.0 m length. Plan view: the grille plate (1.0 x 0.1 m) with a centre split line, and the two bracket plates (0.035 x 0.04 m) in the wall gap.',
        'Vorderansicht mit den Ventilteilen innerhalb der Länge von 1,0 m. Draufsicht: die Gitterplatte (1,0 x 0,1 m) mit einer Mittelteilungslinie und die beiden Konsolplatten (0,035 x 0,04 m) im Wandabstand.'),
    'omitted': (
        'Plan: grille slots, valve and pipes (below the panel).',
        'Draufsicht: Gitterschlitze, Ventil und Rohre (unter dem Paneel).'),
}
slots = [line('grille-slot-%d' % i, B, 'texture', 5, [(x, -0.47), (x, 0.47)], 0.2, region='top-grille') for i, x in enumerate([0.05, 0.075, 0.095, 0.12])]
views['50'] = {
    'front': fixed_front('50'),
    'plan': [
        rect('top-grille', B, 'detail', 4, 0.035, -0.5, 0.1, 1.0, 0.35, region='top-grille', material='painted-steel'),
        *slots,
        line('end-seam-left', B, 'detail', 5, [(0.035, 0.485), (0.135, 0.485)], 0.5),
        line('end-seam-right', B, 'detail', 5, [(0.035, -0.485), (0.135, -0.485)], 0.5),
        rect('bracket-left', B, 'detail', 3, 0.0, 0.38, 0.035, 0.04, 0.4, region='bracket', material='painted-steel'),
        rect('bracket-right', B, 'detail', 3, 0.0, -0.42, 0.035, 0.04, 0.4, region='bracket', material='painted-steel'),
    ],
}
dec['50'] = {
    'silhouette': (
        'Front view with thermostatic head and lockshield moved inside the 1.0 m length (head 0.50 to 0.42 m left of the centre). Plan view: the top grille (100 x 1000 mm) with four slot lines, two end seams and the two 12 mm bracket plates in the 0.035 m wall gap.',
        'Vorderansicht mit Thermostatkopf und Rücklaufverschraubung innerhalb der Länge von 1,0 m (Kopf 0,50 bis 0,42 m links der Mitte). Draufsicht: das Abdeckgitter (100 x 1000 mm) mit vier Schlitzlinien, zwei Stirnnähten und den beiden 12-mm-Konsolplatten im Wandabstand von 0,035 m.'),
    'omitted': (
        'Plan: the valve, pipes and the panel sides (all below the grille).',
        'Draufsicht: Ventil, Rohre und Paneelseiten (alle unter dem Gitter).'),
}
if __name__ == '__main__':
    apply('panel-radiator', views, dec)
