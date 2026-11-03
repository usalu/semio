from lib import *

TY = 0.695
MY = 0.15


def build(s, sc):
    F, P = 'front', 'plan'
    if sc == '500':
        s.poly(F, 'l-plate', [(-0.3, 0.01), (0.3, 0.01)], like='l-plate', closed=False)
        s.rect(F, 'l-drive', -0.16, 0.0, 0.12, 0.15, like='l-drive')
        s.rect(F, 'r-drive', 0.04, 0.0, 0.12, 0.15, like='r-drive')
        s.poly(F, 'l-sleeve', [(0.0, 0.15), (0.0, 0.4)], like='l-column', closed=False, bone='foot-left', z=2.4)
        s.poly(F, 'l-stage', [(0.0, 0.05), (0.0, 0.55)], like='l-column', closed=False, bone='stage-left', z=2.45)
        s.poly(F, 'l-column', [(0.0, 0.15), (0.0, TY)], like='l-column', closed=False, followTip=True)
        s.poly(F, 'top-board', [(-0.4, TY + 0.0125), (0.4, TY + 0.0125)], like='top-board', closed=False)
        s.rect(F, 'top-rail', 0.25, TY - 0.06, 0.06, 0.06, like='top-rail')
        s.rect(F, 'monitor-block', 0.13, TY + 0.025, 0.04, 0.46, like='monitor-block')
        s.poly(F, 'laptop', [(-0.15, TY + 0.04), (0.2, TY + 0.04)], like='laptop', closed=False)
        s.rect(P, 'top-board', 0.0, -0.4, 1.6, 0.8, like='top-board', filled=True, tone=0.58)
        s.rect(P, 'monitor-block', 0.05, 0.11, 0.6, 0.08, like='monitor-block', z=5.1)
        s.rect(P, 'laptop', 0.7, -0.15, 0.25, 0.35, like='laptop', filled=True, tone=0.4)
        return ('Front view: the end of the desk with the foot plate, the single column, the two drive boxes side by side, the thin top, the rail under it, the monitor panel seen edge-on and the closed laptop. Plan: the top board as one plate with the monitor as a bar at the rear and the laptop in front of it; the legs are hidden below the top.',
                'Vorderansicht: die Stirnseite des Tisches mit Fußplatte, der einen Säule, den beiden Antriebskästen nebeneinander, der dünnen Platte, der Schiene darunter, dem Bildschirm von der Kante und dem geschlossenen Laptop. Grundriss: die Tischplatte als eine Fläche mit dem Bildschirm als Balken hinten und dem Laptop davor; die Beine verschwinden unter der Platte.')
    if sc == '200':
        s.poly(F, 'l-plate', [(-0.3, 0.008), (0.3, 0.008)], like='l-plate', closed=False)
        s.rect(F, 'l-sleeve', -0.035, 0.012, 0.07, 0.388, like='l-sleeve')
        s.rect(F, 'l-drive', -0.16, 0.012, 0.12, 0.138, like='l-drive')
        s.rect(F, 'r-drive', 0.04, 0.012, 0.12, 0.138, like='r-drive')
        s.poly(F, 'l-stage', [(-0.03, 0.05), (0.03, 0.05), (0.03, 0.55), (-0.03, 0.55)], like='l-stage', followTip=True)
        s.poly(F, 'l-column', [(-0.025, 0.25), (0.025, 0.25), (0.025, TY), (-0.025, TY)], like='l-column', followTip=True)
        s.rect(F, 'top-board', -0.4, TY, 0.8, 0.03, like='top-board')
        s.rect(F, 'top-rail', 0.25, TY - 0.06, 0.05, 0.05, like='top-rail')
        s.poly(F, 'monitor-neck', [(0.15, TY + 0.03), (0.15, TY + 0.14)], like='monitor-neck', closed=False)
        s.rect(F, 'monitor-panel', 0.13, TY + 0.12, 0.04, 0.365, like='monitor-panel')
        s.poly(F, 'laptop', [(-0.15, TY + 0.04), (0.2, TY + 0.04)], like='laptop', closed=False)
        s.rect(P, 'top-board', 0.0, -0.4, 1.6, 0.8, like='top-board')
        s.rect(P, 'monitor-panel', 0.05, 0.13, 0.6, 0.04, like='monitor-panel')
        s.rect(P, 'laptop', 0.7, -0.15, 0.25, 0.35, like='laptop', filled=True, tone=0.4)
        return ('Front view: the end of the desk with the foot plate, the sleeve, the two drive boxes, the stage and the column that slide up, the top with the rail under it, the monitor panel on its neck and the laptop. Plan: the top board with the monitor bar and the laptop; the legs are hidden below.',
                'Vorderansicht: die Stirnseite mit Fußplatte, Hülse, den beiden Antriebskästen, Stufe und Säule, die nach oben fahren, der Platte mit der Schiene darunter, dem Bildschirm auf seinem Hals und dem Laptop. Grundriss: die Tischplatte mit dem Bildschirmbalken und dem Laptop; die Beine liegen verdeckt darunter.')
    if sc == '100':
        s.poly(F, 'l-plate', [(-0.3, 0.008), (0.3, 0.008)], like='l-plate', closed=False)
        s.rect(F, 'l-sleeve', -0.032, 0.012, 0.064, 0.388, like='l-sleeve')
        s.rect(F, 'l-drive', -0.15, 0.012, 0.1, 0.138, like='l-drive')
        s.rect(F, 'r-drive', 0.05, 0.012, 0.1, 0.138, like='r-drive')
        s.poly(F, 'l-stage', [(-0.027, 0.05), (0.027, 0.05), (0.027, 0.55), (-0.027, 0.55)], like='l-stage', followTip=True)
        s.poly(F, 'l-column', [(-0.022, 0.25), (0.022, 0.25), (0.022, TY), (-0.022, TY)], like='l-column', followTip=True)
        s.rect(F, 'l-bracket', -0.045, TY - 0.035, 0.09, 0.035, like='l-bracket')
        s.rect(F, 'top-board', -0.4, TY, 0.8, 0.025, like='top-board')
        s.rect(F, 'top-rail', 0.25, TY - 0.075, 0.05, 0.05, like='top-rail')
        s.poly(F, 'top-cable-tray', [(0.2, TY - 0.075), (0.2, TY - 0.115), (0.3, TY - 0.115), (0.3, TY - 0.075)], like='top-cable-tray', closed=False)
        s.rect(F, 'monitor-base', 0.05, TY + 0.025, 0.2, 0.012, like='monitor-base', filled=True, tone=0.25)
        s.poly(F, 'monitor-neck', [(0.135, TY + 0.031), (0.165, TY + 0.031), (0.163, TY + 0.15), (0.137, TY + 0.15)], like='monitor-neck')
        s.rect(F, 'monitor-panel', 0.13, TY + 0.12, 0.04, 0.365, like='monitor-panel', r=0.012)
        s.rect(F, 'laptop', -0.15, TY + 0.025, 0.35, 0.02, like='laptop')
        s.rect(P, 'top-board', 0.0, -0.4, 1.6, 0.8, like='top-board')
        s.rect(P, 'monitor-base', 0.26, 0.05, 0.18, 0.2, like='monitor-panel', z=5, filled=True, tone=0.25, role='body', bone='top')
        s.rect(P, 'monitor-panel', 0.05, 0.13, 0.6, 0.04, like='monitor-panel', r=0.012)
        s.rect(P, 'laptop', 0.7, -0.15, 0.25, 0.35, like='laptop')
        s.poly(P, 'laptop-seam', [(0.705, 0.17), (0.945, 0.17)], like='top-cable-tray', closed=False, bone='top', z=5.1, region='laptop-lid', material='equipment-housing')
        return ('Front view: the end of the desk with plate, sleeve, drive boxes, the stage and column with its bracket, the top with rail and cable tray, the monitor with base, neck and panel and the closed laptop. Plan: the top board, the monitor with its base footprint and its thin panel, and the laptop with its hinge line.',
                'Vorderansicht: die Stirnseite mit Platte, Hülse, Antriebskästen, Stufe und Säule mit Konsole, der Platte mit Schiene und Kabelkanal, dem Bildschirm mit Fuß, Hals und Panel und dem geschlossenen Laptop. Grundriss: die Tischplatte, der Bildschirm mit der Standfläche seines Fußes und seinem dünnen Panel und der Laptop mit seiner Scharnierlinie.')
    s.path(F, 'l-plate', 'M -0.3 0 L 0.3 0 L 0.3 0.008 C 0.3 0.016 0.25 0.016 0.15 0.016 L -0.15 0.016 C -0.25 0.016 -0.3 0.016 -0.3 0.008 Z', like='l-plate')
    s.rect(F, 'l-sleeve', -0.032, 0.012, 0.064, 0.388, like='l-sleeve', r=0.004)
    s.rect(F, 'l-sleeve-collar', -0.036, 0.385, 0.072, 0.02, like='l-sleeve-collar', r=0.003)
    s.rect(F, 'l-drive', -0.15, 0.012, 0.1, 0.148, like='l-drive', r=0.012)
    s.rect(F, 'r-drive', 0.05, 0.012, 0.1, 0.148, like='r-drive', r=0.012)
    s.poly(F, 'l-drive-vent', [(-0.13, 0.12), (-0.07, 0.12)], like='l-drive-vent', closed=False)
    s.poly(F, 'r-drive-vent', [(0.07, 0.12), (0.13, 0.12)], like='r-drive-vent', closed=False)
    s.poly(F, 'l-stage', [(-0.027, 0.05), (0.027, 0.05), (0.027, 0.55), (-0.027, 0.55)], like='l-stage', followTip=True)
    s.poly(F, 'l-stage-groove', [(0.0, 0.46), (0.0, 0.55)], like='l-stage-groove', closed=False, followTip=True)
    s.poly(F, 'l-stage-cap', [(-0.029, 0.538), (0.029, 0.538), (0.029, 0.55), (-0.029, 0.55)], like='l-stage-cap', followTip=True)
    s.poly(F, 'l-column', [(-0.022, 0.25), (0.022, 0.25), (0.022, TY), (-0.022, TY)], like='l-column', followTip=True)
    s.poly(F, 'l-column-groove', [(0.0, 0.27), (0.0, 0.64)], like='l-column-groove', closed=False, followTip=True)
    s.rect(F, 'l-bracket', -0.045, TY - 0.035, 0.09, 0.035, like='l-bracket')
    s.rect(F, 'top-board', -0.4, TY, 0.8, 0.025, like='top-board', r=0.004)
    s.poly(F, 'top-edge', [(-0.396, TY + 0.0125), (0.396, TY + 0.0125)], like='top-edge', closed=False)
    s.rect(F, 'top-rail', 0.25, TY - 0.075, 0.05, 0.05, like='top-rail', r=0.003)
    s.poly(F, 'top-cable-tray', [(0.2, TY - 0.075), (0.2, TY - 0.115), (0.3, TY - 0.115), (0.3, TY - 0.075)], like='top-cable-tray', closed=False)
    s.rect(F, 'top-keypad', -0.4, TY - 0.02, 0.02, 0.09 * 0.222, like='top-keypad', r=0.003)
    s.rect(F, 'monitor-base', 0.05, TY + 0.025, 0.2, 0.012, like='monitor-base', r=0.005)
    s.poly(F, 'monitor-neck', [(0.135, TY + 0.037), (0.165, TY + 0.037), (0.162, TY + 0.15), (0.138, TY + 0.15)], like='monitor-neck')
    s.rect(F, 'monitor-panel', 0.13, TY + 0.12, 0.04, 0.365, like='monitor-panel', r=0.012)
    s.rect(F, 'monitor-screen', 0.172, TY + 0.135, 0.004, 0.335, like='monitor-screen')
    s.rect(F, 'laptop', -0.15, TY + 0.025, 0.35, 0.02, like='laptop', r=0.003)
    s.poly(F, 'laptop-seam', [(-0.145, TY + 0.034), (0.195, TY + 0.034)], like='laptop-seam', closed=False)
    s.rect(P, 'top-board', 0.0, -0.4, 1.6, 0.8, like='top-board', r=0.004)
    s.poly(P, 'top-edge', [(0.004, -0.396), (1.596, -0.396), (1.596, 0.396), (0.004, 0.396)], like='top-edge', closed=True, filled=False)
    s.rect(P, 'monitor-base', 0.26, 0.05, 0.18, 0.2, like='monitor-base', r=0.005)
    s.rect(P, 'monitor-panel', 0.05, 0.13, 0.6, 0.04, like='monitor-panel', r=0.012)
    s.poly(P, 'monitor-screen-line', [(0.07, 0.126), (0.63, 0.126)], like='monitor-bezel-logo', closed=False)
    s.rect(P, 'laptop', 0.7, -0.15, 0.25, 0.35, like='laptop', r=0.003)
    s.poly(P, 'laptop-seam', [(0.705, 0.17), (0.945, 0.17)], like='laptop-seam', closed=False)
    return ('Front view: the end of the desk with the foot plate, sleeve with collar, the two drive boxes with vents, the stage and the column with their grooves and bracket, the chamfered top with edge line, rail and cable tray, the keypad under the front edge, the monitor with base, neck, panel and screen and the closed laptop. Plan: the top board with its edge line, the keypad outline, the monitor base footprint and its thin panel with the bezel line, and the laptop with its hinge line.',
            'Vorderansicht: die Stirnseite mit Fußplatte, Hülse mit Manschette, den beiden Antriebskästen mit Lüftungsschlitzen, Stufe und Säule mit Nuten und Konsole, der Platte mit Kantenlinie, Schiene und Kabelkanal, dem Bedienfeld unter der Vorderkante, dem Bildschirm mit Fuß, Hals, Panel und Schirm und dem geschlossenen Laptop. Grundriss: die Tischplatte mit ihrer Kantenlinie, der Umriss des Bedienfelds, die Standfläche des Bildschirmfußes und sein dünnes Panel mit der Blendenlinie und der Laptop mit seiner Scharnierlinie.')


def pre(a):
    el = a['representations']['500']['geometry']['elevation']
    base = {x['id']: x for x in el}
    import copy
    for side in ('l', 'r'):
        foot, stage = f'foot-{"left" if side == "l" else "right"}', f'stage-{"left" if side == "l" else "right"}'
        sleeve = copy.deepcopy(base[f'{side}-column'])
        sleeve.update(id=f'{side}-sleeve', bone=foot, z=2.4, points=[[0.0, 0.15], [0.0, 0.4]])
        stg = copy.deepcopy(base[f'{side}-column'])
        stg.update(id=f'{side}-stage', bone=stage, z=2.45, points=[[0.0, 0.0], [0.5, 0.0]])
        i = el.index(base[f'{side}-column'])
        el[i:i] = [sleeve, stg]


def noted(s, sc):
    en, de = build(s, sc)
    if sc == '500':
        en += ' The telescopic leg is drawn as sleeve, stage and column lines so it stays attached to the foot at every height.'
        de += ' Das Teleskopbein ist als Hülsen-, Stufen- und Säulenlinie gezeichnet, damit es in jeder Höhe am Fuß anschließt.'
    return en, de


if __name__ == '__main__':
    run('height-adjustable-desk', noted, pre)
