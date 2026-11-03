from lib import *

B = 'panel'
views = {}
dec = {}

# 200
views['200'] = {
    'front': [
        rect('shoe-front', B, 'body', 2, -1.0, 0.0, 2.0, 0.125, 0.35, region='clamp-steel', material='steel', pen=0.1),
        rect('glass-front', B, 'glazing', 3, -1.0, 0.125, 2.0, 0.945, 0.92, region='panel-glass', material='glass', pen=0.1),
        rect('handrail-front', B, 'body', 4, -1.0, 1.0735 - 0.0265, 2.0, 0.053, 0.35, region='handrail-steel', material='steel', pen=0.1),
    ],
    'plan': [
        rect('shoe-plan', B, 'body', 2, -0.15, -1.0, 0.3, 2.0, 0.35, region='clamp-steel', material='steel', pen=0.1),
        rect('handrail-plan', B, 'body', 4, -0.0265, -1.0, 0.053, 2.0, 0.35, region='handrail-steel', material='steel', pen=0.1),
    ],
}
dec['200'] = {
    'silhouette': (
        'Front view: the 2.0 m long panel as a glass sheet between a steel shoe strip and the handrail strip. Plan view: the 0.30 m wide shoe from above with the handrail strip on its centre line.',
        'Vorderansicht: das 2,0 m lange Feld als Glasscheibe zwischen Schuhstreifen aus Stahl und Handlaufstreifen. Draufsicht: der 0,30 m breite Schuh von oben mit dem Handlaufstreifen auf der Mittellinie.'),
    'retained': (
        'Front: shoe strip (10 x 0.625 mm), glass sheet (10 x 4.7 mm) and handrail strip (10 x 0.265 mm). Plan: shoe (1.5 x 10 mm) and handrail (0.265 x 10 mm).',
        'Vorderansicht: Schuhstreifen (10 x 0,625 mm), Glasscheibe (10 x 4,7 mm) und Handlaufstreifen (10 x 0,265 mm). Draufsicht: Schuh (1,5 x 10 mm) und Handlauf (0,265 x 10 mm).'),
    'omitted': (
        'In plan the glass edge and the gaskets lie below the handrail (42 mm wide) and are not drawn; the bolts are not visible from either side.',
        'In der Draufsicht liegen Glaskante und Dichtungen unter dem Handlauf (42 mm breit) und werden nicht gezeichnet; die Schrauben sind von keiner Seite sichtbar.'),
}

# 100
views['100'] = {
    'front': [
        rect('shoe-base-front', B, 'body', 2, -1.0, 0.0, 2.0, 0.1, 0.35, region='clamp-steel', material='steel'),
        rect('glass-sheet-front', B, 'glazing', 3, -0.98, 0.1, 1.96, 0.98, 0.92, region='panel-glass', material='glass', pen=0.2),
        line('glass-reflex-a', B, 'glazing', 3.5, [(-0.62, 0.3), (-0.34, 0.86)], 0.2, region='panel-glass', pen=0.1),
        line('glass-reflex-b', B, 'glazing', 3.5, [(-0.5, 0.3), (-0.22, 0.86)], 0.2, region='panel-glass', pen=0.1),
        rect('handrail-tube-front', B, 'body', 4, -1.0, 1.079 - 0.0212, 2.0, 0.0424, 0.35, region='handrail-steel', material='steel'),
    ],
    'plan': [
        rect('shoe-base-plan', B, 'body', 2, -0.15, -1.0, 0.3, 2.0, 0.35, region='clamp-steel', material='steel'),
        rect('handrail-tube-plan', B, 'body', 4, -0.0212, -1.0, 0.0424, 2.0, 0.35, region='handrail-steel', material='steel'),
    ],
}
dec['100'] = {
    'silhouette': (
        'Front view: dark shoe base, a light glass sheet with two reflex strokes and the round handrail as a strip. Plan view: the shoe as a 3 mm wide bar with the handrail strip on its centre line.',
        'Vorderansicht: dunkler Schuhfuß, eine helle Glasscheibe mit zwei Reflexstrichen und der runde Handlauf als Streifen. Draufsicht: der Schuh als 3 mm breiter Balken mit dem Handlaufstreifen auf der Mittellinie.'),
    'retained': (
        'Front: shoe 20 x 1 mm, glass 19.6 x 9.8 mm, handrail 20 x 0.42 mm. Plan: shoe 3 x 20 mm, handrail 0.42 x 20 mm.',
        'Vorderansicht: Schuh 20 x 1 mm, Glas 19,6 x 9,8 mm, Handlauf 20 x 0,42 mm. Draufsicht: Schuh 3 x 20 mm, Handlauf 0,42 x 20 mm.'),
    'omitted': (
        'Plan: the glass edge (21.5 mm) and the gaskets are hidden below the 42 mm handrail. Front: bolts and the glass edge inside the shoe slot.',
        'Draufsicht: Glaskante (21,5 mm) und Dichtungen liegen unter dem 42-mm-Handlauf. Vorderansicht: Schrauben und Glaskante im Schuhschlitz.'),
}

# 50
bolts = [rect('bolt-front-%d' % i, B, 'detail', 3.5, x - 0.007, 0.043, 0.014, 0.014, 0.15, rad=0.007, region='clamp-steel') for i, x in enumerate([-0.8, -0.4, 0.0, 0.4, 0.8])]
views['50'] = {
    'front': [
        rect('shoe-front', B, 'body', 2, -1.0, 0.0, 2.0, 0.1, 0.35, region='clamp-steel', material='steel'),
        *bolts,
        rect('glass-front', B, 'glazing', 3, -0.98, 0.1, 1.96, 0.98, 0.92, region='panel-glass', material='glass'),
        line('glass-reflex-a', B, 'glazing', 3.5, [(-0.62, 0.3), (-0.34, 0.86)], 0.2, region='panel-glass'),
        line('glass-reflex-b', B, 'glazing', 3.5, [(-0.5, 0.3), (-0.22, 0.86)], 0.2, region='panel-glass'),
        rect('handrail-front', B, 'body', 4, -1.0, 1.079 - 0.0212, 2.0, 0.0424, 0.35, region='handrail-steel', material='steel'),
        ell('handrail-cap-left', B, 'detail', 4.5, -1.0, 1.079, 0.004, 0.0212, 0.2, region='handrail-steel', material='steel'),
        ell('handrail-cap-right', B, 'detail', 4.5, 1.0, 1.079, 0.004, 0.0212, 0.2, region='handrail-steel', material='steel'),
    ],
    'plan': [
        rect('shoe-plan', B, 'body', 2, -0.15, -1.0, 0.3, 2.0, 0.35, region='clamp-steel', material='steel'),
        rect('gasket-a-plan', B, 'body', 2.5, 0.01075, -1.0, 0.0165, 2.0, 0.2, region='clamp-steel', material='steel', pen=0.1),
        rect('gasket-b-plan', B, 'body', 2.5, -0.0275, -1.0, 0.0165, 2.0, 0.2, region='clamp-steel', material='steel', pen=0.1),
        rect('glass-plan', B, 'glazing', 3, -0.01075, -0.98, 0.0215, 1.96, 0.92, region='panel-glass', material='glass', pen=0.1),
        rect('handrail-plan', B, 'body', 4, -0.0212, -1.0, 0.0424, 2.0, 0.35, region='handrail-steel', material='steel'),
        line('handrail-axis-plan', B, 'texture', 5, [(0.0, -0.99), (0.0, 0.99)], 0.9, region='handrail-steel', pen=0.1),
    ],
}
dec['50'] = {
    'silhouette': (
        'Front view (2.0 m panel): the shoe with five clamp bolt heads, the laminated glass sheet with two reflex strokes entering the shoe into the slot, and the tube handrail with its end caps. Plan view: the shoe from above, its slot with both gaskets and the glass edge, closed by the handrail tube on the centre line.',
        'Vorderansicht (2,0 m Feld): der Schuh mit fünf Klemmschraubenköpfen, die Verbundglasscheibe mit zwei Reflexstrichen, die in den Schuhschlitz eintritt, und der Rohrhandlauf mit seinen Endkappen. Draufsicht: der Schuh von oben, sein Schlitz mit beiden Dichtungen und der Glaskante, überdeckt vom Handlaufrohr auf der Mittellinie.'),
    'retained': (
        'Front: shoe 40 x 2 mm, bolts 0.28 mm, glass 39.2 x 19.6 mm, handrail 40 x 0.85 mm. Plan: shoe 6 x 40 mm, gaskets 0.33 mm, glass 0.43 mm.',
        'Vorderansicht: Schuh 40 x 2 mm, Schrauben 0,28 mm, Glas 39,2 x 19,6 mm, Handlauf 40 x 0,85 mm. Draufsicht: Schuh 6 x 40 mm, Dichtungen 0,33 mm, Glas 0,43 mm.'),
    'omitted': (
        'Front: the glass edge inside the slot, the handrail wall. Plan: the shoe bolts (they sit in the side of the shoe) and the PVB layer.',
        'Vorderansicht: die Glaskante im Schlitz, die Handlaufwand. Draufsicht: die Schuhschrauben (sie sitzen in der Seite des Schuhs) und die PVB-Schicht.'),
    'thinElements': (
        'The 21.5 mm glass edge in plan is a 0.43 mm filled strip, the gaskets 0.33 mm; the handrail tube (0.85 mm) covers their inner parts, as seen from above.',
        'Die 21,5-mm-Glaskante in der Draufsicht ist ein gefüllter Streifen von 0,43 mm, die Dichtungen 0,33 mm; das Handlaufrohr (0,85 mm) überdeckt ihre inneren Teile, wie von oben gesehen.'),
}

if __name__ == '__main__':
    apply('glass-balustrade', views, dec)
