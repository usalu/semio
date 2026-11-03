from lib import *

F, I, SP = 'frame', 'igu', 'spandrel'
CUT = 0.08
views = {}
dec = {}
ALLR = ['glass-centre', 'glass-edge', 'mullion', 'transom']
CONV_EN = 'Convention: the elevation is the face of the bay, so the front view is the vertical section A-A through the bay centre seen from the left (lateral x: exterior at the right, x = -0.075 m, interior at the left side +0.075 m) and the plan view is the horizontal section B-B through the middle of the vision glass seen from above (lateral y: exterior at -y); the mullion side face lies behind the vertical cut, the top of the middle transom below the horizontal cut.'
CONV_DE = 'Konvention: die Ansicht ist die Fläche des Feldes, daher ist die Vorderansicht der Vertikalschnitt A-A durch die Feldmitte von links gesehen (seitliches x: außen rechts bei x = -0,075 m, innen links bei +0,075 m) und die Draufsicht der Horizontalschnitt B-B in halber Höhe der Verglasung von oben gesehen (seitliches y: außen bei -y); die Seitenfläche des Pfostens liegt hinter dem Vertikalschnitt, die Oberseite des mittleren Riegels unter dem Horizontalschnitt.'


def three(id, x, y, w, h, z=3):
    return [
        rect(id + '-outer', F, 'body', z, x, y, w, h, 0.4, region='transom', material='aluminium'),
        rect(id + '-break', F, 'detail', z, x, y, w, h, 0.85, region='transom', material='aluminium'),
        rect(id + '-inner', F, 'body', z, x, y, w, h, 0.4, region='transom', material='aluminium'),
    ]


# 500
views['500'] = {
    'front': [rect('bay-section', F, 'body', 1, -0.075, 0.0, 0.15, 3.3, 0.3, regions=['spandrel'] + ALLR, material='aluminium')],
    'plan': [rect('bay-section', F, 'body', 1, 0.0, -0.075, 1.4, 0.15, 0.3, regions=ALLR, material='aluminium')],
}
dec['500'] = {
    'silhouette': (CONV_EN + ' At 1:500 each is one filled strip: 0.3 x 6.6 mm in section A-A, 2.8 x 0.3 mm in section B-B.', CONV_DE + ' Bei 1:500 ist jede ein gefüllter Streifen: 0,3 x 6,6 mm im Schnitt A-A, 2,8 x 0,3 mm im Schnitt B-B.'),
    'omitted': ('Glass layers, spandrel build-up, thermal breaks and profile parts are inside the strips.', 'Glasschichten, Brüstungsaufbau, thermische Trennungen und Profilteile gehen in den Streifen auf.'),
}

# 200
views['200'] = {
    'front': [
        rect('mullion-side', F, 'body', 1, -0.075, 0.0, 0.15, 3.3, 0.75, region='mullion', material='aluminium'),
        rect('spandrel', SP, 'body', 2, -0.07, 0.05, 0.127, 0.8, 0.65, region='spandrel', material='spandrel-glass'),
        rect('glass', I, 'glazing', 2, -0.045, 0.9, 0.04, 2.35, 0.15, regions=['glass-centre', 'glass-edge'], material='glass'),
        rect('transom-base', F, 'detail', 3, -0.075, 0.0, 0.15, 0.05, CUT, region='transom', material='aluminium'),
        rect('transom-mid', F, 'detail', 3, -0.075, 0.85, 0.15, 0.05, CUT, region='transom', material='aluminium'),
        rect('transom-head', F, 'detail', 3, -0.075, 3.25, 0.15, 0.05, CUT, region='transom', material='aluminium'),
    ],
    'plan': [
        rect('mullion-left', F, 'detail', 3, 0.0, -0.075, 0.05, 0.15, CUT, region='mullion', material='aluminium'),
        rect('mullion-right', F, 'detail', 3, 1.35, -0.075, 0.05, 0.15, CUT, region='mullion', material='aluminium'),
        rect('glass', I, 'glazing', 2, 0.05, -0.045, 1.3, 0.04, 0.15, regions=['glass-centre', 'glass-edge'], material='glass'),
    ],
}
dec['200'] = {
    'silhouette': (CONV_EN + ' At 1:200 section A-A shows the mullion side as the ground mass with spandrel, glass strip and three transom blocks over it; section B-B shows the two mullion blocks and the glass strip.', CONV_DE + ' Bei 1:200 zeigt der Schnitt A-A die Pfostenseite als Grundmasse mit Brüstung, Glasstreifen und drei Riegelblöcken darüber; der Schnitt B-B zeigt die beiden Pfostenblöcke und den Glasstreifen.'),
    'thinElements': ('The 28 mm glass unit is drawn 40 mm thick (0.2 mm) in both sections so that it prints.', 'Die 28 mm dicke Verglasung ist in beiden Schnitten 40 mm dick (0,2 mm) gezeichnet, damit sie druckt.'),
    'omitted': ('Thermal breaks, caps and the top of the transom below the horizontal cut.', 'Thermische Trennungen, Deckschalen und die Riegeloberseite unter dem Horizontalschnitt.'),
}

# 100
views['100'] = {
    'front': [
        rect('mullion-side', F, 'body', 1, -0.075, 0.0, 0.15, 3.3, 0.75, region='mullion', material='aluminium'),
        rect('spandrel', SP, 'body', 2, -0.07, 0.05, 0.127, 0.8, 0.65, region='spandrel', material='spandrel-glass'),
        rect('glass', I, 'glazing', 2, -0.039, 0.9, 0.028, 2.35, 0.15, regions=['glass-centre', 'glass-edge'], material='glass'),
        rect('transom-base', F, 'body', 3, -0.075, 0.0, 0.15, 0.05, CUT, region='transom', material='aluminium'),
        rect('transom-mid', F, 'body', 3, -0.075, 0.85, 0.15, 0.05, CUT, region='transom', material='aluminium'),
        rect('transom-head', F, 'body', 3, -0.075, 3.25, 0.15, 0.05, CUT, region='transom', material='aluminium'),
        line('glass-reflex-a', I, 'glazing', 3, [(-0.034, 1.5), (-0.034, 2.0)], 0.2, region='glass-centre'),
    ],
    'plan': [
        rect('transom-top', F, 'detail', 1, 0.05, -0.075, 1.3, 0.15, 0.75, region='transom', material='aluminium'),
        rect('mullion-left', F, 'body', 3, 0.0, -0.075, 0.05, 0.15, CUT, region='mullion', material='aluminium'),
        rect('mullion-right', F, 'body', 3, 1.35, -0.075, 0.05, 0.15, CUT, region='mullion', material='aluminium'),
        rect('glass', I, 'glazing', 2, 0.05, -0.039, 1.3, 0.028, 0.15, regions=['glass-centre', 'glass-edge'], material='glass'),
    ],
}
dec['100'] = {
    'silhouette': (CONV_EN + ' At 1:100 the cut glass unit has its true 28 mm thickness; the middle transom top is drawn light below the horizontal cut.', CONV_DE + ' Bei 1:100 hat die geschnittene Verglasung ihre echte Dicke von 28 mm; die Oberseite des mittleren Riegels ist unter dem Horizontalschnitt hell gezeichnet.'),
    'omitted': ('The three layers of the glass unit and the thermal breaks of the profiles.', 'Die drei Schichten der Verglasung und die thermischen Trennungen der Profile.'),
}

# 50
mull = lambda n, x: [
    rect('mullion-%s-outer' % n, F, 'body', 3, x, -0.075, 0.05, 0.035, CUT, region='mullion', material='aluminium'),
    rect('mullion-%s-break' % n, F, 'detail', 3, x, -0.04, 0.05, 0.03, 0.85, region='mullion', material='aluminium'),
    rect('mullion-%s-inner' % n, F, 'body', 3, x, -0.01, 0.05, 0.085, CUT, region='mullion', material='aluminium'),
]
tr = lambda n, y: [
    rect('transom-%s-outer' % n, F, 'body', 3, -0.075, y, 0.035, 0.05, CUT, region='transom', material='aluminium'),
    rect('transom-%s-break' % n, F, 'detail', 3, -0.04, y, 0.03, 0.05, 0.85, region='transom', material='aluminium'),
    rect('transom-%s-inner' % n, F, 'body', 3, -0.01, y, 0.085, 0.05, CUT, region='transom', material='aluminium'),
]
views['50'] = {
    'front': [
        rect('mullion-side', F, 'body', 1, -0.075, 0.0, 0.15, 3.3, 0.75, region='mullion', material='aluminium'),
        line('spandrel-enamel', SP, 'glazing', 2, [(-0.067, 0.05), (-0.067, 0.85)], 0.65, region='spandrel', material='spandrel-glass', pen=0.13),
        rect('spandrel-wool', SP, 'body', 2, -0.044, 0.05, 0.1, 0.8, 0.9, region='spandrel', material='wool'),
        line('spandrel-pan', SP, 'detail', 2.5, [(0.0568, 0.05), (0.0568, 0.85)], 0.3, region='spandrel', material='steel', pen=0.1),
        line('pane-out', I, 'glazing', 2, [(-0.036, 0.9), (-0.036, 3.25)], 0.15, region='glass-centre', material='glass', pen=0.13),
        line('pane-in', I, 'glazing', 2, [(-0.014, 0.9), (-0.014, 3.25)], 0.15, region='glass-centre', material='glass', pen=0.13),
        rect('spacer-base', I, 'detail', 2, -0.033, 0.9, 0.016, 0.012, 0.4, region='glass-edge', material='glass'),
        rect('spacer-head', I, 'detail', 2, -0.033, 3.238, 0.016, 0.012, 0.4, region='glass-edge', material='glass'),
        *tr('base', 0.0), *tr('mid', 0.85), *tr('head', 3.25),
    ],
    'plan': [
        rect('transom-top', F, 'detail', 1, 0.05, -0.075, 1.3, 0.15, 0.75, region='transom', material='aluminium'),
        line('transom-top-break', F, 'texture', 1.5, [(0.05, -0.04), (1.35, -0.04)], 0.4, region='transom'),
        line('transom-top-break-b', F, 'texture', 1.5, [(0.05, -0.01), (1.35, -0.01)], 0.4, region='transom'),
        *mull('left', 0.0), *mull('right', 1.35),
        line('pane-out', I, 'glazing', 2, [(0.05, -0.036), (1.35, -0.036)], 0.15, region='glass-centre', material='glass', pen=0.13),
        line('pane-in', I, 'glazing', 2, [(0.05, -0.014), (1.35, -0.014)], 0.15, region='glass-centre', material='glass', pen=0.13),
        rect('spacer-left', I, 'detail', 2, 0.05, -0.033, 0.012, 0.016, 0.4, region='glass-edge', material='glass'),
        rect('spacer-right', I, 'detail', 2, 1.338, -0.033, 0.012, 0.016, 0.4, region='glass-edge', material='glass'),
    ],
}
dec['50'] = {
    'silhouette': (CONV_EN + ' At 1:50 section A-A shows the spandrel build-up (enamel line, 100 mm wool, steel pan line), the two glass panes as lines with the spacers, and each transom as outer part, thermal break and inner part. Section B-B shows both mullions in three parts, the two panes with spacers and the top of the middle transom with the break lines.', CONV_DE + ' Bei 1:50 zeigt der Schnitt A-A den Brüstungsaufbau (Emaillelinie, 100 mm Wolle, Stahlblechlinie), die beiden Glasscheiben als Linien mit den Abstandhaltern und jeden Riegel als Außenteil, thermische Trennung und Innenteil. Der Schnitt B-B zeigt beide Pfosten in drei Teilen, die beiden Scheiben mit Abstandhaltern und die Riegeloberseite mit den Trennungslinien.'),
    'thinElements': ('The 6 mm panes, the 6 mm enamel and the 1.5 mm steel pan are pen lines (0.13 and 0.1 mm), the 16 mm cavity stays open.', 'Die 6-mm-Scheiben, das 6-mm-Email und das 1,5-mm-Stahlblech sind Federlinien (0,13 und 0,1 mm), der 16-mm-Scheibenzwischenraum bleibt offen.'),
    'omitted': ('Gaskets, pressure plate screws and the 20 mm air gap of the spandrel (it is an open channel between enamel line and wool).', 'Dichtungen, Pressleistenschrauben und die 20 mm Luftschicht der Brüstung (sie ist ein offener Kanal zwischen Emaillelinie und Wolle).'),
}

HATCH_EN = 'Cut aluminium (mullion and transom parts in the section planes) is filled solid dark, the drafting convention for thin metal sections; the uncut mullion side and the transom top below the cut stay light. Glass, spandrel and wool are shown by their lines and tone, not hatched.'
HATCH_DE = 'Geschnittenes Aluminium (Pfosten- und Riegelteile in den Schnittebenen) ist dunkel voll ausgefüllt, die Zeichenkonvention für dünne Metallschnitte; die ungeschnittene Pfostenseite und die Riegeloberseite unter dem Schnitt bleiben hell. Glas, Brüstung und Wolle sind durch Linien und Ton dargestellt, nicht schraffiert.'
for _sc in ('200', '100', '50'):
    dec[_sc]['hatching'] = ('!' + HATCH_EN, '!' + HATCH_DE)
dec['500']['hatching'] = ('!None: at 1:500 the cut bay is one filled strip.', '!Keine: bei 1:500 ist das geschnittene Feld ein gefüllter Streifen.')

if __name__ == '__main__':
    apply('curtain-wall-bay', views, dec)
