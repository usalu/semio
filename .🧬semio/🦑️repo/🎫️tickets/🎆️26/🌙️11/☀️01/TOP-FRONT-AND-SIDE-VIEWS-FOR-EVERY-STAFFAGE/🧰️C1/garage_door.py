from lib import *

T, SS = 'track', 'spring-shaft'
P = ['panel-1', 'panel-2', 'panel-3', 'panel-4']
views = {}
dec = {}
CONV_EN = 'Convention: the elevation is the section along the travel, so the front view is the door seen from outside (lateral x: left of the door is +x) and the plan view is the view from above through the open garage; the lateral extent of 2.5 m is the track-to-track width.'
CONV_DE = 'Konvention: die Ansicht ist der Schnitt in Fahrtrichtung, daher ist die Vorderansicht das Tor von außen gesehen (seitliches x: links vom Tor ist +x) und die Draufsicht der Blick von oben durch die offene Garage; die seitliche Ausdehnung von 2,5 m ist die Breite von Schiene zu Schiene.'
LW = 1.21  # half width of the leaf, track legs from 1.21 to 1.25


SOLID = {'kind': 'extruded', 'z0': -LW, 'z1': LW}


def front_panels(prefix, tone, z=3, pen=None, material='sandwich-skin', region='panel-sandwich', t=0.02):
    return [rect('%s-%d' % (prefix, k + 1), P[k], 'body', z, 0.0, -t, 0.5, 2 * t, tone, region=region, material=material, pen=pen, solid=SOLID) for k in range(4)]


def bands(prefix, h, tone=0.2, z=3.5, pen=0.2):
    return [rect('%s-%d' % (prefix, k + 1), P[k], 'detail', z, -LW, 0.5 * k - h / 2 if k else 0.0, 2 * LW, h, tone, region='hardware-steel', material='steel', pen=pen) for k in range(4)]


def edges(prefix, r, tone=0.55, z=3, region='panel-sandwich', material='sandwich-skin'):
    return [rect('%s-%d' % (prefix, k + 1), P[k], 'body', z, 0.0, -r, 0.5, 2 * r, tone, region=region, material=material, solid=SOLID) for k in range(4)]


def hinges(prefix, w, z=3.5):
    return []


# 500
views['500'] = {
    'front': [
        *front_panels('panel', 0.5, pen=0.13),
        line('track-left', T, 'body', 1, [(LW + 0.02, 0.0), (LW + 0.02, 2.5)], 0.4, region='track-steel', material='steel', pen=0.13),
        line('track-right', T, 'body', 1, [(-LW - 0.02, 0.0), (-LW - 0.02, 2.5)], 0.4, region='track-steel', material='steel', pen=0.13),
        line('shaft', SS, 'body', 2, [(-1.2, 2.75), (1.2, 2.75)], 0.4, region='spring-assembly', material='steel', pen=0.13),
        rect('drum-left', SS, 'body', 2, 1.1, 2.66, 0.1, 0.18, 0.4, region='spring-assembly', material='steel'),
        rect('drum-right', SS, 'body', 2, -1.2, 2.66, 0.1, 0.18, 0.4, region='spring-assembly', material='steel'),
    ],
    'plan': [
        *edges('panel-edge', 0.0325),
        line('track-left', T, 'body', 1, [(-0.02, LW + 0.02), (2.6, LW + 0.02)], 0.4, region='track-steel', material='steel', pen=0.13),
        line('track-right', T, 'body', 1, [(-0.02, -LW - 0.02), (2.6, -LW - 0.02)], 0.4, region='track-steel', material='steel', pen=0.13),
        line('shaft', SS, 'body', 2, [(0.12, -1.2), (0.12, 1.2)], 0.4, region='spring-assembly', material='steel', pen=0.13),
    ],
}
dec['500'] = {
    'silhouette': (CONV_EN + ' At 1:500 the front view is four panel bars stacked between two track lines with the spring shaft as a line; the plan view is two track lines, the shaft line and the four panels as extruded bars that fold with the door.', CONV_DE + ' Bei 1:500 ist die Vorderansicht vier gestapelte Paneelbalken zwischen zwei Schienenlinien mit der Federwelle als Linie; die Draufsicht ist zwei Schienenlinien, die Wellenlinie und die vier Paneele als extrudierte Balken, die mit dem Tor falten.'),
    'thinElements': ('The 0.04 m panels and track legs are 0.13 mm lines or bars (0.065 m).', 'Die 0,04 m dicken Paneele und Schienenschenkel sind 0,13-mm-Linien oder -Balken (0,065 m).'),
}

# 200
views['200'] = {
    'front': [
        *front_panels('panel', 0.55, pen=0.18),
        rect('track-left', T, 'detail', 1, LW, 0.0, 0.04, 2.5, 0.4, region='track-steel', material='steel'),
        rect('track-right', T, 'detail', 1, -LW - 0.04, 0.0, 0.04, 2.5, 0.4, region='track-steel', material='steel'),
        line('hanger-left', T, 'detail', 1, [(LW + 0.02, 2.5), (LW + 0.02, 2.85)], 0.4, region='track-steel', material='steel', pen=0.1),
        line('hanger-right', T, 'detail', 1, [(-LW - 0.02, 2.5), (-LW - 0.02, 2.85)], 0.4, region='track-steel', material='steel', pen=0.1),
        rect('drum-left', SS, 'body', 2, 1.1, 2.66, 0.1, 0.18, 0.4, region='spring-assembly', material='steel'),
        rect('drum-right', SS, 'body', 2, -1.2, 2.66, 0.1, 0.18, 0.4, region='spring-assembly', material='steel'),
        line('shaft', SS, 'detail', 2, [(-1.1, 2.75), (1.1, 2.75)], 0.4, region='spring-assembly', material='steel', pen=0.1),
    ],
    'plan': [
        *edges('panel-edge', 0.02),
        *hinges('panel-hinge', 0.04, 3.2),
        rect('track-left', T, 'detail', 1, -0.02, LW, 2.62, 0.04, 0.4, region='track-steel', material='steel'),
        rect('track-right', T, 'detail', 1, -0.02, -LW - 0.04, 2.62, 0.04, 0.4, region='track-steel', material='steel'),
        rect('drum-left', SS, 'body', 2, 0.03, 1.1, 0.18, 0.1, 0.4, region='spring-assembly', material='steel'),
        rect('drum-right', SS, 'body', 2, 0.03, -1.2, 0.18, 0.1, 0.4, region='spring-assembly', material='steel'),
        line('shaft', SS, 'detail', 2, [(0.12, -1.1), (0.12, 1.1)], 0.4, region='spring-assembly', material='steel', pen=0.1),
    ],
}
dec['200'] = {
    'silhouette': (CONV_EN + ' At 1:200 the front view shows four panel bars, the two track bars with hanger lines, the two spring drums and the shaft; the plan view shows the leaf as four extruded panel bars, the two tracks, the drums and the shaft.', CONV_DE + ' Bei 1:200 zeigt die Vorderansicht vier Paneelbalken, die beiden Schienenbalken mit Abhängerlinien, die beiden Federtrommeln und die Welle; die Draufsicht zeigt das Torblatt als vier extrudierte Paneelbalken, die beiden Schienen, die Trommeln und die Welle.'),
    'omitted': ('Hardware, rollers and the spring coil.', 'Beschläge, Rollen und die Feder.'),
}

# 100
views['100'] = {
    'front': [
        *front_panels('panel', 0.55, pen=0.25),
        *bands('hinge-band', 0.07),
        rect('track-left', T, 'detail', 1, LW, 0.0, 0.04, 2.5, 0.4, region='track-steel', material='steel'),
        rect('track-right', T, 'detail', 1, -LW - 0.04, 0.0, 0.04, 2.5, 0.4, region='track-steel', material='steel'),
        rect('hanger-left', T, 'detail', 1, LW + 0.005, 2.5, 0.03, 0.35, 0.4, region='track-steel', material='steel'),
        rect('hanger-right', T, 'detail', 1, -LW - 0.035, 2.5, 0.03, 0.35, 0.4, region='track-steel', material='steel'),
        rect('drum-left', SS, 'body', 2, 1.1, 2.66, 0.1, 0.18, 0.5, region='spring-assembly', material='steel'),
        rect('drum-right', SS, 'body', 2, -1.2, 2.66, 0.1, 0.18, 0.5, region='spring-assembly', material='steel'),
        rect('spring-coil', SS, 'detail', 3, -0.5, 2.7125, 1.0, 0.075, 0.4, region='spring-assembly', material='steel'),
        line('shaft', SS, 'detail', 2, [(-1.1, 2.75), (-0.5, 2.75), (0.5, 2.75), (1.1, 2.75)], 0.4, region='spring-assembly', material='steel', pen=0.13),
    ],
    'plan': [
        *edges('panel-edge', 0.02),
        *hinges('panel-hinge', 0.04, 3.2),
        rect('track-left', T, 'detail', 1, -0.02, LW, 2.62, 0.04, 0.4, region='track-steel', material='steel'),
        rect('track-right', T, 'detail', 1, -0.02, -LW - 0.04, 2.62, 0.04, 0.4, region='track-steel', material='steel'),
        rect('hanger-left-a', T, 'detail', 1.5, 1.28, LW, 0.04, 0.04, 0.3, region='track-steel', material='steel'),
        rect('hanger-left-b', T, 'detail', 1.5, 2.48, LW, 0.04, 0.04, 0.3, region='track-steel', material='steel'),
        rect('hanger-right-a', T, 'detail', 1.5, 1.28, -LW - 0.04, 0.04, 0.04, 0.3, region='track-steel', material='steel'),
        rect('hanger-right-b', T, 'detail', 1.5, 2.48, -LW - 0.04, 0.04, 0.04, 0.3, region='track-steel', material='steel'),
        rect('drum-left', SS, 'body', 2, 0.03, 1.1, 0.18, 0.1, 0.5, region='spring-assembly', material='steel'),
        rect('drum-right', SS, 'body', 2, 0.03, -1.2, 0.18, 0.1, 0.5, region='spring-assembly', material='steel'),
        rect('spring-coil', SS, 'detail', 3, 0.0825, -0.5, 0.075, 1.0, 0.4, region='spring-assembly', material='steel'),
        line('shaft', SS, 'detail', 2, [(0.12, -1.1), (0.12, 1.1)], 0.4, region='spring-assembly', material='steel', pen=0.13),
    ],
}
dec['100'] = {
    'silhouette': (CONV_EN + ' At 1:100 the front view adds the hinge bands between the panels, hanger bars, the spring coil between the drums; the plan view adds the four hangers.', CONV_DE + ' Bei 1:100 ergänzt die Vorderansicht die Scharnierbänder zwischen den Paneelen, Abhängerbalken und die Feder zwischen den Trommeln; die Draufsicht ergänzt die vier Abhänger.'),
    'omitted': ('Rollers, cable and handle.', 'Rollen, Seil und Griff.'),
}

# 50
ribs = [poly('panel-rib-%d' % (k + 1), P[k], 'texture', 3.2, [(-LW + 0.1, 0.5 * k + 0.25), (LW - 0.1, 0.5 * k + 0.25)], 0.3, closed=False, filled=False, region='panel-sandwich', follow=True) for k in range(4)]
hanger_f = [rect('hanger-%s' % n, T, 'detail', 1, x, 2.5, 0.03, 0.35, 0.4, region='track-steel', material='steel') for n, x in (('left', LW + 0.005), ('right', -LW - 0.035))]
roll = [rect('roller-%s-%d' % (n, k + 1), P[k], 'detail', 4, x, 0.5 * k - 0.011 if k else 0.0, 0.022, 0.022, 0.2, region='hardware-steel', material='steel', pen=0.18) for k in range(4) for n, x in (('l', LW - 0.011), ('r', -LW - 0.011))]
views['50'] = {
    'front': [
        *front_panels('panel', 0.55, pen=0.35),
        *ribs,
        *bands('hinge-band', 0.08, 0.3),
        rect('bottom-seal', P[0], 'detail', 3.6, -LW, 0.0, 2 * LW, 0.07, 0.15, region='hardware-steel', material='steel', pen=0.2),
        rect('track-left', T, 'detail', 1, LW, 0.0, 0.04, 2.5, 0.4, region='track-steel', material='steel'),
        rect('track-right', T, 'detail', 1, -LW - 0.04, 0.0, 0.04, 2.5, 0.4, region='track-steel', material='steel'),
        *hanger_f,
        rect('drum-left', SS, 'body', 2, 1.1, 2.66, 0.1, 0.18, 0.5, region='spring-assembly', material='steel'),
        rect('drum-right', SS, 'body', 2, -1.2, 2.66, 0.1, 0.18, 0.5, region='spring-assembly', material='steel'),
        rect('spring-coil', SS, 'detail', 3, -0.5, 2.7125, 1.0, 0.075, 0.4, region='spring-assembly', material='steel'),
        rect('shaft-left', SS, 'detail', 4, -1.1, 2.7375, 0.6, 0.025, 0.2, region='spring-assembly', material='steel'),
        rect('shaft-right', SS, 'detail', 4, 0.5, 2.7375, 0.6, 0.025, 0.2, region='spring-assembly', material='steel'),
    ],
    'plan': [
        *edges('panel-edge', 0.02),
        *hinges('panel-hinge', 0.05),
        rect('track-left', T, 'detail', 1, -0.02, LW, 2.62, 0.04, 0.4, region='track-steel', material='steel'),
        rect('track-right', T, 'detail', 1, -0.02, -LW - 0.04, 2.62, 0.04, 0.4, region='track-steel', material='steel'),
        rect('hanger-left-a', T, 'detail', 1.5, 1.28, LW, 0.04, 0.04, 0.3, region='track-steel', material='steel'),
        rect('hanger-left-b', T, 'detail', 1.5, 2.48, LW, 0.04, 0.04, 0.3, region='track-steel', material='steel'),
        rect('hanger-right-a', T, 'detail', 1.5, 1.28, -LW - 0.04, 0.04, 0.04, 0.3, region='track-steel', material='steel'),
        rect('hanger-right-b', T, 'detail', 1.5, 2.48, -LW - 0.04, 0.04, 0.04, 0.3, region='track-steel', material='steel'),
        rect('track-end-bar', T, 'detail', 1.6, 2.58, -LW, 0.02, 2 * LW, 0.4, region='track-steel', material='steel'),
        rect('drum-left', SS, 'body', 2, 0.03, 1.1, 0.18, 0.1, 0.5, region='spring-assembly', material='steel'),
        rect('drum-right', SS, 'body', 2, 0.03, -1.2, 0.18, 0.1, 0.5, region='spring-assembly', material='steel'),
        rect('spring-coil', SS, 'detail', 3, 0.0825, -0.5, 0.075, 1.0, 0.4, region='spring-assembly', material='steel'),
        rect('shaft-left', SS, 'detail', 4, 0.1075, -1.1, 0.025, 0.6, 0.2, region='spring-assembly', material='steel'),
        rect('shaft-right', SS, 'detail', 4, 0.1075, 0.5, 0.025, 0.6, 0.2, region='spring-assembly', material='steel'),
    ],
}
dec['50'] = {
    'silhouette': (CONV_EN + ' At 1:50 the front view shows four ribbed panels with hinge bands and bottom seal, both tracks with hangers, drums, coil and shaft; the plan view shows the four panels as extruded bars (they fold and travel with the door), tracks, four hangers, the track end bar, drums, coil and shaft.', CONV_DE + ' Bei 1:50 zeigt die Vorderansicht vier gesickte Paneele mit Scharnierbändern und Bodendichtung, beide Schienen mit Abhängern, Trommeln, Feder und Welle; die Draufsicht zeigt die vier Paneele als extrudierte Balken (sie falten und fahren mit dem Tor), Schienen, vier Abhänger, den Schienenabschluss, Trommeln, Feder und Welle.'),
    'omitted': ('Cable, handle, lock; the rollers are inside the track.', 'Seil, Griff, Schloss; die Rollen liegen in der Schiene.'),
}
if __name__ == '__main__':
    apply('sectional-garage-door', views, dec)
