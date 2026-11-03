from lib import *

W, L, S = 'wall', 'lintel', 'sill'
views = {}
dec = {}
HW = 0.2225
CONV_EN = 'Convention: the elevation is the face of the wall, so the front view is the vertical section A-A through the middle of the opening seen from the left (lateral x: exterior at the right, x = -0.2225 m, interior at +0.2225 m) with the reveal of the opening behind the cut, and the plan view is the horizontal section B-B at half height of the opening seen from above (lateral y: exterior at -y) with the sill below the cut.'
CONV_DE = 'Konvention: die Ansicht ist die Wandfläche, daher ist die Vorderansicht der Vertikalschnitt A-A durch die Öffnungsmitte von links gesehen (seitliches x: außen rechts bei x = -0,2225 m, innen bei +0,2225 m) mit der Laibung der Öffnung hinter dem Schnitt, und die Draufsicht der Horizontalschnitt B-B in halber Öffnungshöhe von oben gesehen (seitliches y: außen bei -y) mit der Fensterbank unter dem Schnitt.'
YS, YSH, YO, YL, YT = 0.83333, 0.91667, 2.25, 2.5, 2.75  # sill bottom, sill top = opening bottom, opening top = lintel bottom, lintel top, wall top
OX0, OX1 = 0.875, 2.125


def layers(prefix, bone, x0, y0, h, z=1, hatch_facing=None):
    spec = [('facing', -HW, 0.115, 0.55, 'brick', hatch_facing), ('fill', -HW + 0.115, 0.14, 0.9, 'wool', None), ('block', -HW + 0.255, 0.175, 0.7, 'block', None), ('plaster', HW - 0.015, 0.015, 0.85, 'plaster', None)]
    return [rect('%s-%s' % (prefix, n), bone, 'body', z, x, y0, w, h, t, region='masonry-field', material=m, hatch=ht) for n, x, w, t, m, ht in spec]


def players(prefix, bone, y0, x0, w, z=1, hatch_facing=None, lat=False):
    spec = [('facing', -HW, 0.115, 0.55, 'brick', hatch_facing), ('fill', -HW + 0.115, 0.14, 0.9, 'wool', None), ('block', -HW + 0.255, 0.175, 0.7, 'block', None), ('plaster', HW - 0.015, 0.015, 0.85, 'plaster', None)]
    return [rect('%s-%s' % (prefix, n), bone, 'body', z, x0, y, w, t_w, t, region='masonry-field', material=m, hatch=ht) for n, y, t_w, t, m, ht in spec]


# 500
views['500'] = {
    'front': [
        rect('field-lower', W, 'body', 1, -HW, 0.0, 0.445, YSH, 0.55, regions=['masonry-field', 'sill'], material='brick'),
        rect('field-upper', W, 'body', 1, -HW, YO, 0.445, YT - YO, 0.45, regions=['masonry-field', 'lintel'], material='brick'),
    ],
    'plan': [
        rect('pier-left', W, 'body', 1, 0.0, -HW, OX0, 0.445, 0.55, region='masonry-field', material='brick'),
        rect('pier-right', W, 'body', 1, OX1, -HW, 3.0 - OX1, 0.445, 0.55, region='masonry-field', material='brick'),
        rect('sill-below', S, 'body', 2, 0.8, -HW, 1.4, 0.34, 0.3, region='sill', material='concrete'),
    ],
}
dec['500'] = {
    'silhouette': (CONV_EN + ' At 1:500 section A-A is two filled blocks (0.9 x 2.1 mm below, 0.9 x 1.0 mm above the opening), section B-B two piers and the sill band between them (6 x 0.9 mm).', CONV_DE + ' Bei 1:500 ist der Schnitt A-A zwei gefüllte Blöcke (0,9 x 2,1 mm unter, 0,9 x 1,0 mm über der Öffnung), der Schnitt B-B zwei Pfeiler und das Fensterbankband dazwischen (6 x 0,9 mm).'),
    'omitted': ('Wall layers, lintel layers and the reveal behind the cut.', 'Wandschichten, Sturzschichten und die Laibung hinter dem Schnitt.'),
}

# 200
views['200'] = {
    'front': [
        rect('reveal', W, 'body', 0.5, -HW, YSH, 0.445, YO - YSH, 0.8, region='masonry-field', material='brick'),
        rect('field-lower', W, 'body', 1, -HW, 0.0, 0.445, YS, 0.55, region='masonry-field', material='brick'),
        rect('field-upper', W, 'body', 1, -HW, YL, 0.445, YT - YL, 0.55, region='masonry-field', material='brick'),
        rect('lintel', L, 'body', 2, -HW, YO, 0.34, 0.25, 0.35, region='lintel', material='concrete'),
        rect('sill', S, 'body', 2, -HW, YS, 0.34, 0.08334, 0.35, region='sill', material='concrete'),
    ],
    'plan': [
        rect('pier-left', W, 'body', 1, 0.0, -HW, OX0, 0.445, 0.55, region='masonry-field', material='brick'),
        rect('pier-right', W, 'body', 1, OX1, -HW, 3.0 - OX1, 0.445, 0.55, region='masonry-field', material='brick'),
        rect('sill-below', S, 'body', 2, 0.8, -HW, 1.4, 0.34, 0.35, region='sill', material='concrete'),
    ],
}
dec['200'] = {
    'silhouette': (CONV_EN + ' At 1:200 section A-A shows the wall below and above the opening, the lintel and the sill as darker blocks flush with the outside, and the reveal as a light block; section B-B shows two piers and the sill block between them.', CONV_DE + ' Bei 1:200 zeigt der Schnitt A-A die Wand unter und über der Öffnung, Sturz und Fensterbank als dunklere Blöcke bündig außen und die Laibung als hellen Block; der Schnitt B-B zeigt zwei Pfeiler und den Fensterbankblock dazwischen.'),
    'omitted': ('Layers of the wall (the leaves are 0.2 to 0.9 mm).', 'Schichten der Wand (die Schalen sind 0,2 bis 0,9 mm breit).'),
}


# 100
views['100'] = {
    'hatches': [{'id': 'cut-brick', 'title': {'en': 'Cut facing brick', 'de': 'Geschnittener Verblender'}, 'pattern': 'lines', 'angleDeg': 45, 'spacingMm': 0.5, 'penMm': 0.1}],
    'front': [
        rect('reveal', W, 'body', 0.5, -HW, YSH, 0.445, YO - YSH, 0.85, region='masonry-field', material='brick'),
        *layers('lower', W, 0, 0.0, YS, hatch_facing='cut-brick'),
        *layers('upper', W, 0, YL, YT - YL, hatch_facing='cut-brick'),
        rect('lintel', L, 'body', 2, -HW, YO, 0.31, 0.25, 0.35, region='lintel', material='concrete', hatch='concrete-stipple'),
        rect('sill', S, 'body', 2, -HW, YS, 0.34, 0.08334, 0.35, region='sill', material='concrete', hatch='concrete-stipple'),
    ],
    'plan': [
        *players('pier-left', W, 0, 0.0, OX0, hatch_facing='cut-brick'),
        *players('pier-right', W, 0, OX1, 3.0 - OX1, hatch_facing='cut-brick'),
        rect('sill-below', S, 'body', 2, 0.8, -HW, 1.4, 0.34, 0.35, region='sill', material='concrete', hatch='concrete-stipple'),
    ],
}
# players y arguments are lateral offsets taken from the spec; x0/w are the pier extent
dec['100'] = {
    'silhouette': (CONV_EN + ' At 1:100 the leaves are separate: facing brick, mineral wool, inner block and plaster in both sections; lintel (0.31 m, flush outside) and sill (0.34 m) are stippled.', CONV_DE + ' Bei 1:100 sind die Schalen getrennt: Verblender, Mineralwolle, Innenblock und Putz in beiden Schnitten; Sturz (0,31 m, außen bündig) und Fensterbank (0,34 m) sind punktiert.'),
    'omitted': ('The lintel and sill layers (0.04 to 0.175 m) are one stippled block; head joints and coursing.', 'Die Sturz- und Fensterbankschichten (0,04 bis 0,175 m) sind ein punktierter Block; Stoßfugen und Schichtung.'),
}
dec['100']['hatching'] = ('!Cut facing brick in both sections as 45 degree lines at 0.5 mm (added hatch cut-brick, drafting convention for cut masonry); cut concrete (lintel, sill) as stipple; the leaves of wool, inner block and plaster by tone only. The coursing lines of the elevation are not repeated in the sections.', '!Geschnittener Verblender in beiden Schnitten als 45-Grad-Linien mit 0,5 mm (zusätzliche Schraffur cut-brick, Zeichenkonvention für geschnittenes Mauerwerk); geschnittener Beton (Sturz, Fensterbank) punktiert; die Schalen aus Wolle, Innenblock und Putz nur durch Ton. Die Schichtlinien der Ansicht werden in den Schnitten nicht wiederholt.')

# 50
def sill_layers():
    spec = [('shell', -HW, 0.07, 0.3, 'concrete', 'concrete-stipple'), ('ins', -HW + 0.07, 0.08, 0.9, 'wool', None), ('block', -HW + 0.15, 0.175, 0.7, 'block', None), ('plaster', -HW + 0.325, 0.015, 0.85, 'plaster', None)]
    return [rect('sill-%s' % n, S, 'body', 2, x, YS, w, 0.08334, t, region='sill', material=m, hatch=ht) for n, x, w, t, m, ht in spec]


def lintel_layers():
    spec = [('shell', -HW, 0.04, 0.3, 'concrete', 'concrete-stipple'), ('ins', -HW + 0.04, 0.08, 0.9, 'wool', None), ('inner', -HW + 0.12, 0.175, 0.3, 'concrete', 'concrete-stipple'), ('plaster', -HW + 0.295, 0.015, 0.85, 'plaster', None)]
    return [rect('lintel-%s' % n, L, 'body', 2, x, YO, w, 0.25, t, region='lintel', material=m, hatch=ht) for n, x, w, t, m, ht in spec]


def zig(x0, x1, y0, y1, n):
    return [((x0 if i % 2 == 0 else x1), y0 + (y1 - y0) * i / n) for i in range(n + 1)]


views['50'] = {
    'hatches': [{'id': 'cut-brick', 'title': {'en': 'Cut facing brick', 'de': 'Geschnittener Verblender'}, 'pattern': 'lines', 'angleDeg': 45, 'spacingMm': 0.8, 'penMm': 0.1}],
    'front': [
        rect('reveal', W, 'body', 0.5, -HW, YSH, 0.445, YO - YSH, 0.85, region='masonry-field', material='brick'),
        *layers('lower', W, 0, 0.0, YS, hatch_facing='cut-brick'),
        *layers('upper', W, 0, YL, YT - YL, hatch_facing='cut-brick'),
        line('lower-fill-zigzag', W, 'texture', 1.2, zig(-HW + 0.125, -HW + 0.245, 0.02, YS - 0.02, 20), 0.3, region='masonry-field'),
        line('upper-fill-zigzag', W, 'texture', 1.2, zig(-HW + 0.125, -HW + 0.245, YL + 0.02, YT - 0.02, 5), 0.3, region='masonry-field'),
        *sill_layers(),
        *lintel_layers(),
        rect('lintel-back', W, 'body', 1.5, -HW + 0.31, YO, 0.135 - 0.015, 0.25, 0.7, region='masonry-field', material='block'),
        rect('lintel-back-plaster', W, 'body', 1.5, HW - 0.015, YO, 0.015, 0.25, 0.85, region='masonry-field', material='plaster'),
        rect('sill-back', W, 'body', 1.5, -HW + 0.34, YS, 0.105 - 0.015, 0.08334, 0.7, region='masonry-field', material='block'),
        rect('sill-back-plaster', W, 'body', 1.5, HW - 0.015, YS, 0.015, 0.08334, 0.85, region='masonry-field', material='plaster'),
    ],
    'plan': [
        *players('pier-left', W, 0, 0.0, OX0, hatch_facing='cut-brick'),
        *players('pier-right', W, 0, OX1, 3.0 - OX1, hatch_facing='cut-brick'),
        line('pier-left-fill-zigzag', W, 'texture', 1.2, [(y, x) for x, y in zig(-HW + 0.125, -HW + 0.245, 0.03, OX0 - 0.03, 14)], 0.3, region='masonry-field'),
        line('pier-right-fill-zigzag', W, 'texture', 1.2, [(y, x) for x, y in zig(-HW + 0.125, -HW + 0.245, OX1 + 0.03, 2.97, 14)], 0.3, region='masonry-field'),
        *[rect('sill-%s-below' % n, S, 'body', 2, 0.8, y, 1.4, w, t, region='sill', material=m, hatch=ht) for n, y, w, t, m, ht in [('shell', -HW, 0.07, 0.3, 'concrete', 'concrete-stipple'), ('ins', -HW + 0.07, 0.08, 0.9, 'wool', None), ('block', -HW + 0.15, 0.175, 0.7, 'block', None), ('plaster', -HW + 0.325, 0.015, 0.85, 'plaster', None)]],
    ],
}
dec['50'] = {
    'silhouette': (CONV_EN + ' At 1:50 every leaf of the wall is drawn in both sections: hatched facing brick, wool with zigzag line, inner block and plaster; the sill (shell, insulation, block, plaster) and the lintel (shell, insulation, inner concrete, plaster) keep their own layers, backed by the wall leaves on the inside.', CONV_DE + ' Bei 1:50 ist jede Schale der Wand in beiden Schnitten gezeichnet: schraffierter Verblender, Wolle mit Zickzacklinie, Innenblock und Putz; Fensterbank (Schale, Dämmung, Block, Putz) und Sturz (Schale, Dämmung, Innenbeton, Putz) behalten ihre eigenen Schichten und sind innen von den Wandschalen hinterlegt.'),
    'hatching': ('Cut facing brick as 45 degree lines at 0.8 mm (added hatch cut-brick), cut concrete as stipple, wool as zigzag line; the uncut sill top in plan is not hatched.', 'Geschnittener Verblender als 45-Grad-Linien mit 0,8 mm (zusätzliche Schraffur cut-brick), geschnittener Beton punktiert, Wolle als Zickzacklinie; die ungeschnittene Fensterbankoberseite in der Draufsicht ist nicht schraffiert.'),
    'omitted': ('Wall ties, damp-proof course, window frame and the head joints of the courses.', 'Mauerwerksanker, Feuchtesperre, Fensterrahmen und die Stoßfugen der Schichten.'),
}

if __name__ == '__main__':
    apply('masonry-wall-with-lintel', views, dec)
