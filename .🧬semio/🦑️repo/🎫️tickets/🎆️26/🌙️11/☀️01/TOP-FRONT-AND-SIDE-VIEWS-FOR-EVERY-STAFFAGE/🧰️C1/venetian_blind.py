from lib import *

FA, BX, BR = 'facade', 'box', 'bottom-rail'
views = {}
dec = {}
HW = 0.6
N = 17
YS = [2.325 - 0.08 * k for k in range(N)]
CONV_EN = 'Convention: the elevation is the vertical section, so the front view is the blind seen from outside (lateral x: left of the blind is +x, 1.2 m wide) with the wall bands and the glazing behind it, and the plan view is the view from above: the headrail box and the wall head, with the lamellae and the bottom rail drawn as extruded solids below the box, where they stay hidden in every pose (they hang within the 0.15 m depth of the box).'
CONV_DE = 'Konvention: die Ansicht ist der Vertikalschnitt, daher ist die Vorderansicht die Jalousie von außen gesehen (seitliches x: links von der Jalousie ist +x, 1,2 m breit) mit den Wandbändern und der Verglasung dahinter, und die Draufsicht der Blick von oben: Kastenblende und Wandkopf, die Lamellen und die Unterschiene sind als extrudierte Körper unter dem Kasten gezeichnet und bleiben in jeder Pose verdeckt (sie hängen innerhalb der 0,15 m Tiefe des Kastens).'


SOL = {'kind': 'extruded', 'z0': -0.585, 'z1': 0.585}


def slats(t, pen, z=3, tone=0.4):
    return [rect('slat-%d' % k, 'slat-%d' % k, 'detail', z, -0.045, -t / 2, 0.09, t, tone, region='lamellae', material='aluminium', pen=pen, solid=SOL) for k in range(N)]


def rail(pen, z=4):
    return rect('bottom-rail', BR, 'detail', z, -0.045, -0.02, 0.09, 0.04, 0.4, region='box-and-rails', material='aluminium', pen=pen, solid=SOL)


def walls(tone=0.5, pen=None):
    return [
        rect('wall-sill', FA, 'body', 1, -HW, 0.0, 2 * HW, 1.0, tone, region='facade-wall', material='masonry', pen=pen),
        rect('wall-head', FA, 'body', 1, -HW, 2.25, 2 * HW, 0.5, tone, region='facade-wall', material='masonry', pen=pen),
    ]


# 200
views['200'] = {
    'front': [
        *walls(0.5, 0.13),
        rect('glass', FA, 'glazing', 2, -0.55, 1.0, 1.1, 1.25, 0.9, region='glazing-behind', material='glass'),
        rect('box', BX, 'body', 5, -HW, 2.32, 2 * HW, 0.21, 0.45, region='box-and-rails', material='aluminium', pen=0.171),
        rect('guide-left', BX, 'detail', 2, 0.57, 1.03, 0.03, 1.29, 0.3, region='box-and-rails', material='aluminium', pen=0.1),
        rect('guide-right', BX, 'detail', 2, -0.6, 1.03, 0.03, 1.29, 0.3, region='box-and-rails', material='aluminium', pen=0.1),
        *slats(0.02, 0.1),
        rail(0.18),
    ],
    'plan': [
        *slats(0.02, 0.1, 3), rail(0.18, 3.5),
        rect('wall-head', FA, 'body', 1, -0.27, -HW, 0.27, 2 * HW, 0.5, region='facade-wall', material='masonry', pen=0.13),
        rect('box', BX, 'body', 5, 0.0, -HW, 0.15, 2 * HW, 0.45, region='box-and-rails', material='aluminium', pen=0.171),
    ],
}
dec['200'] = {
    'silhouette': (CONV_EN + ' At 1:200 the front view has the two wall bands, the glazing, the box, two guide bars, 17 extruded lamella bars (the tilt reads in the front view) and the bottom rail; the plan view is the wall head (1.35 x 6 mm) and the box (0.75 x 6 mm).', CONV_DE + ' Bei 1:200 hat die Vorderansicht die zwei Wandbänder, die Verglasung, den Kasten, zwei Führungsbalken, 17 extrudierte Lamellenbalken (die Kippung liest sich in der Vorderansicht) und die Unterschiene; die Draufsicht ist der Wandkopf (1,35 x 6 mm) und der Kasten (0,75 x 6 mm).'),
    'omitted': ('Window frame, cords and the lamellae in plan (below the box).', 'Fensterrahmen, Schnüre und die Lamellen in der Draufsicht (unter dem Kasten).'),
}

# 100
views['100'] = {
    'front': [
        *walls(0.5, 0.18),
        rect('glass', FA, 'glazing', 2, -0.55, 1.0, 1.1, 1.25, 0.9, region='glazing-behind', material='glass', pen=0.1),
        line('glass-reflex', FA, 'glazing', 2.5, [(-0.4, 1.3), (-0.2, 1.8)], 0.2, region='glazing-behind'),
        rect('box', BX, 'body', 5, -HW, 2.32, 2 * HW, 0.21, 0.45, region='box-and-rails', material='aluminium', pen=0.331),
        rect('guide-left', BX, 'detail', 2, 0.57, 1.03, 0.03, 1.29, 0.3, region='box-and-rails', material='aluminium', pen=0.13),
        rect('guide-right', BX, 'detail', 2, -0.6, 1.03, 0.03, 1.29, 0.3, region='box-and-rails', material='aluminium', pen=0.13),
        *slats(0.013, 0.13),
        rail(0.13),
    ],
    'plan': [
        *slats(0.013, 0.13, 3), rail(0.13, 3.5),
        rect('wall-head', FA, 'body', 1, -0.27, -HW, 0.27, 2 * HW, 0.5, region='facade-wall', material='masonry', hatch='cut-masonry', pen=0.18),
        rect('box', BX, 'body', 5, 0.0, -HW, 0.15, 2 * HW, 0.45, region='box-and-rails', material='aluminium', pen=0.331),
        line('box-lid-seam', BX, 'texture', 5.5, [(0.075, -0.57), (0.075, 0.57)], 0.3, region='box-and-rails'),
    ],
}
dec['100'] = {
    'silhouette': (CONV_EN + ' At 1:100 the front view adds a reflex stroke on the glazing; the plan view shows the cut wall head hatched and the box with its lid seam.', CONV_DE + ' Bei 1:100 ergänzt die Vorderansicht einen Reflexstrich auf der Verglasung; die Draufsicht zeigt den geschnittenen Wandkopf schraffiert und den Kasten mit seiner Deckelfuge.'),
    'omitted': ('Window frame, cords; plan: lamellae, rails and the bottom rail (below the box).', 'Fensterrahmen, Schnüre; Draufsicht: Lamellen, Schienen und Unterschiene (unter dem Kasten).'),
}

# 50
views['50'] = {
    'front': [
        *walls(0.5, 0.25),
        rect('render-sill', FA, 'detail', 1.5, -HW, 0.0, 2 * HW, 0.01, 0.4, region='facade-wall', material='masonry'),
        rect('glass', FA, 'glazing', 2, -0.55, 1.0, 1.1, 1.25, 0.9, region='glazing-behind', material='glass', pen=0.13),
        line('glass-reflex-a', FA, 'glazing', 2.5, [(-0.4, 1.3), (-0.2, 1.8)], 0.2, region='glazing-behind'),
        line('glass-reflex-b', FA, 'glazing', 2.5, [(-0.3, 1.3), (-0.1, 1.8)], 0.2, region='glazing-behind'),
        rect('box', BX, 'body', 5, -HW, 2.32, 2 * HW, 0.21, 0.45, region='box-and-rails', material='aluminium'),
        line('box-slot', BX, 'texture', 6, [(-0.5, 2.335), (0.5, 2.335)], 0.3, region='box-and-rails'),
        rect('guide-left', BX, 'detail', 2, 0.57, 1.03, 0.03, 1.29, 0.3, region='box-and-rails', material='aluminium', pen=0.13),
        rect('guide-right', BX, 'detail', 2, -0.6, 1.03, 0.03, 1.29, 0.3, region='box-and-rails', material='aluminium', pen=0.13),
        *slats(0.012, 0.216),
        rail(0.296),
    ],
    'plan': [
        *slats(0.012, 0.216, 3), rail(0.296, 3.5),
        rect('block-head', FA, 'body', 1, -0.27, -HW, 0.185, 2 * HW, 0.5, region='facade-wall', material='masonry', hatch='cut-masonry'),
        rect('insulation-head', FA, 'detail', 1, -0.085, -HW, 0.085, 2 * HW, 0.7, region='facade-wall', material='eps', hatch='insulation'),
        line('render-head', FA, 'detail', 2, [(-0.0075, -HW), (-0.0075, HW)], 0.4, region='facade-wall', material='masonry', pen=0.3),
        rect('box', BX, 'body', 5, 0.0, -HW, 0.15, 2 * HW, 0.45, region='box-and-rails', material='aluminium'),
        line('box-lid-seam', BX, 'texture', 5.5, [(0.03, -0.57), (0.12, -0.57)], 0.3, region='box-and-rails'),
        line('box-slot-plan', BX, 'texture', 5.5, [(0.075, -0.55), (0.075, 0.55)], 0.3, region='box-and-rails'),
        line('box-end-left', BX, 'texture', 5.5, [(0.0, 0.57), (0.15, 0.57)], 0.3, region='box-and-rails'),
        line('box-end-right', BX, 'texture', 5.5, [(0.0, -0.57), (0.15, -0.57)], 0.3, region='box-and-rails'),
    ],
}
dec['50'] = {
    'silhouette': (CONV_EN + ' At 1:50 the front view shows 17 extruded lamella bars of 0.012 m thickness (their apparent height grows with the tilt up to the 0.09 m chord), the glass with two reflex strokes, the box with its slot line and the guides; the plan view shows the wall head in its layers (hatched block, stippled insulation, render line) and the box with lid seams.', CONV_DE + ' Bei 1:50 zeigt die Vorderansicht 17 extrudierte Lamellenbalken von 0,012 m Dicke (ihre scheinbare Höhe wächst mit der Kippung bis zur Sehne von 0,09 m), das Glas mit zwei Reflexstrichen, den Kasten mit Schlitzlinie und die Führungen; die Draufsicht zeigt den Wandkopf in seinen Schichten (schraffierter Block, punktierte Dämmung, Putzlinie) und den Kasten mit Deckelfugen.'),
    'omitted': ('Window frame, cords; plan: lamellae, rails and the bottom rail (they hang below the box).', 'Fensterrahmen, Schnüre; Draufsicht: Lamellen, Schienen und Unterschiene (sie hängen unter dem Kasten).'),
}
if __name__ == '__main__':
    apply('external-venetian-blind', views, dec)
