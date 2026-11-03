from lib import *

S, R = 'slab', 'railing'
views = {}
dec = {}
PX = [-0.44, 0.44]


def zigzag(x0, x1, n, y0=-0.5, y1=0.5):
    return [((x0 if i % 2 == 0 else x1), y0 + (y1 - y0) * i / n) for i in range(n + 1)]


views['500'] = {
    'front': [
        rect('balcony-solid', S, 'body', 1, -0.5, 0.0, 1.0, 1.3, 0.25, regions=['wall-face', 'slab-edge']),
        line('railing-line', R, 'body', 2, [(-0.5, 1.27), (0.5, 1.27)], 0.45, region='railing-steel', pen=0.13),
    ],
    'plan': [
        rect('balcony-solid', S, 'body', 1, -0.3, -0.5, 1.8, 1.0, 0.25, regions=['wall-face', 'slab-top']),
        line('railing-line', R, 'body', 2, [(1.45, -0.5), (1.45, 0.5)], 0.45, region='railing-steel', pen=0.13),
    ],
}
dec['500'] = {
    'silhouette': (
        'Front view (from the free edge towards the wall, strip of 1.0 m): one filled block of wall and slab edge (2 x 2.6 mm) with the handrail as a single pen line. Plan view (from above): one filled block of wall top and slab (3.6 x 2 mm) with the railing as a line across the strip.',
        'Vorderansicht (von der freien Kante zur Wand, Streifen von 1,0 m): ein gefüllter Block aus Wand und Plattenkante (2 x 2,6 mm) mit dem Handlauf als einzelner Federlinie. Draufsicht (von oben): ein gefüllter Block aus Wandkopf und Platte (3,6 x 2 mm) mit dem Geländer als Linie über den Streifen.'),
    'omitted': (
        'The thermal break lies behind the slab edge and below the wall top and is not visible in either view; posts, mid-rail, plates and bolts are merged into the handrail line.',
        'Das Dämmelement liegt hinter der Plattenkante und unter dem Wandkopf und ist in keiner Ansicht sichtbar; Pfosten, Zwischenstab, Platten und Schrauben gehen in der Handlauflinie auf.'),
}

views['200'] = {
    'front': [
        rect('wall', S, 'body', 1, -0.5, 0.0, 1.0, 1.3, 0.6, region='wall-face', material='render'),
        rect('slab', S, 'body', 3, -0.5, 0.0, 1.0, 0.2, 0.72, regions=['slab-edge'], material='concrete'),
        *[line('railing-post-%s' % ('l' if x < 0 else 'r'), R, 'body', 5, [(x, 0.2), (x, 1.24)], 0.45, region='railing-steel', material='steel', pen=0.25) for x in PX],
        rect('railing-handrail', R, 'body', 5.1, -0.5, 1.24, 1.0, 0.06, 0.35, region='railing-steel', material='steel'),
    ],
    'plan': [
        rect('wall', S, 'body', 1, -0.3, -0.5, 0.3, 1.0, 0.6, region='wall-face', material='concrete'),
        line('wall-core-edge', S, 'detail', 1.5, [(-0.12, -0.5), (-0.12, 0.5)], 0.3),
        rect('slab', S, 'body', 3, 0.0, -0.5, 1.5, 1.0, 0.72, regions=['slab-top'], material='concrete'),
        line('railing-post', R, 'body', 5, [(1.45, -0.5), (1.45, 0.5)], 0.45, region='railing-steel', material='steel', pen=0.25),
    ],
}
dec['200'] = {
    'silhouette': (
        'Front view: wall face, slab edge (5 x 1 mm) and the railing seen from the free edge as two post lines and a handrail strip. Plan view: wall top with its concrete and insulation edge, slab top (7.5 x 5 mm) and the railing as one line.',
        'Vorderansicht: Wandfläche, Plattenkante (5 x 1 mm) und das Geländer von der freien Kante gesehen als zwei Pfostenlinien und ein Handlaufstreifen. Draufsicht: Wandkopf mit seiner Kante Beton/Dämmung, Plattenoberseite (7,5 x 5 mm) und das Geländer als eine Linie.'),
    'omitted': (
        'The thermal break module is hidden behind the slab edge and under the wall top; mid-rail and plates are not drawn.',
        'Das Dämmelement liegt verdeckt hinter der Plattenkante und unter dem Wandkopf; Zwischenstab und Platten werden nicht gezeichnet.'),
}

views['100'] = {
    'front': [
        rect('wall', S, 'body', 1, -0.5, 0.0, 1.0, 1.3, 0.55, region='wall-face', material='render'),
        rect('slab', S, 'body', 3, -0.5, 0.0, 1.0, 0.2, 0.72, regions=['slab-edge'], material='concrete'),
        *[rect('railing-post-%s' % ('l' if x < 0 else 'r'), R, 'body', 5, x - 0.025, 0.2, 0.05, 1.04, 0.45, region='railing-steel', material='steel') for x in PX],
        rect('railing-handrail', R, 'body', 5.1, -0.5, 1.24, 1.0, 0.06, 0.35, region='railing-steel', material='steel'),
        rect('railing-mid-rail', R, 'body', 5.1, -0.44, 0.73, 0.88, 0.04, 0.35, region='railing-steel', material='steel'),
    ],
    'plan': [
        rect('wall-core', S, 'body', 1, -0.3, -0.5, 0.18, 1.0, 0.6, region='wall-face', material='concrete', hatch='concrete'),
        rect('wall-insulation-render', S, 'body', 1, -0.12, -0.5, 0.12, 1.0, 0.9, region='wall-face', material='mineral-wool'),
        line('wall-insulation-zigzag', S, 'texture', 1.2, zigzag(-0.11, -0.01, 20), 0.3, region='wall-face'),
        rect('slab', S, 'body', 3, 0.0, -0.5, 1.5, 1.0, 0.72, regions=['slab-top'], material='concrete'),
        rect('railing-handrail', R, 'body', 5.1, 1.42, -0.5, 0.06, 1.0, 0.35, region='railing-steel', material='steel'),
    ],
}
dec['100'] = {
    'silhouette': (
        'Front view: render face, slab edge, two post bars, handrail and mid-rail as in the section. Plan view: the wall top cut in its layers (stippled concrete, insulation with zigzag line), the slab top and the handrail tube as a strip.',
        'Vorderansicht: Putzfläche, Plattenkante, zwei Pfostenbalken, Handlauf und Zwischenstab wie im Schnitt. Draufsicht: der Wandkopf in seinen Schichten geschnitten (punktierter Beton, Dämmung mit Zickzacklinie), die Plattenoberseite und das Handlaufrohr als Streifen.'),
    'omitted': (
        'The base plates (12 mm, 0.12 mm) and bolts; the thermal break is hidden behind the slab edge and below the wall top. No hatch on the uncut slab edge and slab top.',
        'Die Fußplatten (12 mm, 0,12 mm) und Schrauben; das Dämmelement liegt verdeckt hinter der Plattenkante und unter dem Wandkopf. Keine Schraffur auf der ungeschnittenen Plattenkante und -oberseite.'),
}

plates = [rect('railing-base-plate-%s' % n, R, 'body', 5, x - 0.05, 0.2, 0.1, 0.012, 0.3, region='railing-steel', material='steel', pen=0.18) for n, x in (('l', -0.44), ('r', 0.44))]
bolts = [rect('railing-bolt-%s%d' % (n, k), R, 'detail', 5.3, x + dx - 0.01, 0.212, 0.02, 0.012, 0.1, region='railing-steel', material='steel') for n, x in (('l', -0.44), ('r', 0.44)) for k, dx in enumerate((-0.035, 0.035))]
dots = [ell('wall-rebar-%s%d' % (row, k), S, 'detail', 1.1, xr, y, 0.006, 0.006, 0.2, region='wall-face', pen=0.1) for row, xr in (('i', -0.27), ('o', -0.15)) for k, y in enumerate([-0.4, -0.2, 0.0, 0.2, 0.4])]
pl_plates = [rect('railing-base-plate-%s' % n, R, 'body', 5, 1.4, c - 0.05, 0.1, 0.1, 0.3, region='railing-steel', material='steel', pen=0.18) for n, c in (('l', 0.44), ('r', -0.44))]
pl_bolts = [ell('railing-bolt-%s%d%d' % (n, i, j), R, 'detail', 5.3, 1.45 + dx, c + dy, 0.01, 0.01, 0.1, region='railing-steel', material='steel') for n, c in (('l', 0.44), ('r', -0.44)) for i, dx in enumerate((-0.035, 0.035)) for j, dy in enumerate((-0.035, 0.035))]
views['50'] = {
    'front': [
        rect('wall', S, 'body', 1, -0.5, 0.0, 1.0, 1.3, 0.55, region='wall-face', material='render'),
        rect('slab', S, 'body', 3, -0.5, 0.0, 1.0, 0.2, 0.72, regions=['slab-edge'], material='concrete'),
        line('slab-arris', S, 'texture', 3.5, [(-0.5, 0.19), (0.5, 0.19)], 0.3, region='slab-edge'),
        *plates,
        *bolts,
        *[rect('railing-post-%s' % ('l' if x < 0 else 'r'), R, 'body', 5, x - 0.025, 0.212, 0.05, 1.028, 0.45, region='railing-steel', material='steel') for x in PX],
        rect('railing-handrail', R, 'body', 5.1, -0.5, 1.24, 1.0, 0.06, 0.35, region='railing-steel', material='steel'),
        rect('railing-mid-rail', R, 'body', 5.1, -0.44, 0.73, 0.88, 0.04, 0.35, region='railing-steel', material='steel'),
    ],
    'plan': [
        rect('wall-core', S, 'body', 1, -0.3, -0.5, 0.18, 1.0, 0.6, region='wall-face', material='concrete', hatch='concrete'),
        *dots,
        rect('wall-insulation', S, 'body', 1, -0.12, -0.5, 0.1, 1.0, 0.9, region='wall-face', material='mineral-wool'),
        line('wall-insulation-zigzag', S, 'texture', 1.2, zigzag(-0.11, -0.03, 24), 0.3, region='wall-face'),
        rect('wall-render', S, 'body', 1, -0.02, -0.5, 0.02, 1.0, 0.55, region='wall-face', material='render'),
        rect('slab', S, 'body', 3, 0.0, -0.5, 1.5, 1.0, 0.72, regions=['slab-top'], material='concrete'),
        *pl_plates,
        *pl_bolts,
        rect('railing-handrail', R, 'body', 5.1, 1.42, -0.5, 0.06, 1.0, 0.35, region='railing-steel', material='steel'),
    ],
}
dec['50'] = {
    'silhouette': (
        'Front view: render face, slab edge with its arris line, two posts on base plates with four bolt heads, handrail and mid-rail seen from the free edge. Plan view: the wall top cut through concrete core with ten vertical bars, insulation with zigzag line and render, the slab top, the railing tube across the strip and the two base plates with eight bolts.',
        'Vorderansicht: Putzfläche, Plattenkante mit Kantenlinie, zwei Pfosten auf Fußplatten mit vier Schraubenköpfen, Handlauf und Zwischenstab von der freien Kante gesehen. Draufsicht: der Wandkopf geschnitten durch Betonkern mit zehn senkrechten Stäben, Dämmung mit Zickzacklinie und Putz, die Plattenoberseite, das Geländerrohr über den Streifen und die beiden Fußplatten mit acht Schrauben.'),
    'omitted': (
        'The thermal break, the slab reinforcement and the drip groove lie behind the slab edge, below the wall top or under the soffit and are not visible in either view; the post section under the handrail.',
        'Das Dämmelement, die Plattenbewehrung und die Tropfkante liegen hinter der Plattenkante, unter dem Wandkopf oder unter der Unterseite und sind in keiner Ansicht sichtbar; der Pfostenquerschnitt unter dem Handlauf.'),
}

if __name__ == '__main__':
    apply('cantilever-balcony', views, dec)
