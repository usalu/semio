from lib import *

K = 0.0349
BX, BY = 0.066, 0.46


def fl(y):
    return 0.25 - 0.06 * y / 0.42


def rl(y):
    return 0.23 - 0.05 * y / 0.42


def fx(y):
    return 0.42 - K * y


def rx(y):
    return 0.05 + K * y


def build(s, sc):
    r = build0(s, sc)
    s.follow('leg-front', 'leg-rear', 'back')
    return r


def build0(s, sc):
    F, P = 'front', 'plan'
    sg = (1, -1)
    if sc == '500':
        s.rect(F, 'seat-band', -0.21, 0.36, 0.42, 0.1, like='seat-band')
        for i, g in enumerate(sg):
            s.poly(F, f'leg-front-{i}', [(g * 0.25, 0.0), (g * 0.19, 0.42)], like='leg-front', closed=False)
            s.poly(F, f'back-post-{i}', [(g * 0.19, 0.46), (g * 0.19, 0.878)], like='back-post', closed=False)
        s.solid(P, 'seat-band', 'seat-band', -0.21, 0.21)
        for i, g in enumerate(sg):
            s.poly(P, f'leg-front-{i}', [(0.404, g * 0.19), (0.42, g * 0.25)], like='leg-front', closed=False)
            s.poly(P, f'leg-rear-{i}', [(0.065, g * 0.18), (0.05, g * 0.23)], like='leg-rear', closed=False)
        s.poly(P, 'back-post', [(0.03, -0.19), (0.03, 0.19)], like='back-post', closed=False)
        return ('Front view: the seat band, the two splayed front legs and the two back posts as lines. Plan: the seat as one rounded plate, the back as a line across the rear edge and the four feet as short strokes that stick out past the seat.',
                'Vorderansicht: das Sitzband, die zwei gespreizten Vorderbeine und die zwei Lehnenpfosten als Linien. Grundriss: der Sitz als eine gerundete Platte, die Lehne als Linie am hinteren Rand und die vier Füße als kurze Striche, die über den Sitz hinausragen.')
    if sc == '200':
        s.rect(F, 'seat-band', -0.21, 0.4, 0.42, 0.06, like='seat-band')
        for i, g in enumerate(sg):
            s.poly(F, f'leg-front-{i}', [(g * 0.25, 0.0), (g * 0.19, 0.4)], like='leg-front', closed=False)
            s.poly(F, f'leg-rear-{i}', [(g * 0.23, 0.0), (g * 0.18, 0.4)], like='leg-rear', closed=False, z=2.5)
            s.poly(F, f'back-post-{i}', [(g * 0.19, 0.46), (g * 0.19, 0.878)], like='back-post', closed=False)
        s.rect(F, 'back-panel', -0.185, 0.66, 0.37, 0.185, like='back-panel', r=0.016)
        s.solid(P, 'seat-band', 'seat-band', -0.21, 0.21)
        s.solid(P, 'back-panel', 'back-panel', -0.185, 0.185)
        for i, g in enumerate(sg):
            s.poly(P, f'leg-front-{i}', [(0.404, g * 0.19), (0.42, g * 0.25)], like='leg-front', closed=False)
            s.poly(P, f'leg-rear-{i}', [(0.065, g * 0.18), (0.05, g * 0.23)], like='leg-rear', closed=False)
            s.poly(P, f'stretcher-{i}', [(0.054, g * 0.225), (0.416, g * 0.225)], like='stretcher', closed=False)
        return ('Front view: seat band, back panel between the two posts, the splayed front legs in front and the rear legs behind them. Plan: the rounded seat plate, the back panel as a thin bar along the rear edge and, outside the seat, the two side stretchers and the leg feet.',
                'Vorderansicht: Sitzband, Lehnenfüllung zwischen den beiden Pfosten, vorn die gespreizten Vorderbeine und dahinter die Hinterbeine. Grundriss: die gerundete Sitzplatte, die Lehnenfüllung als dünner Balken am hinteren Rand und außerhalb des Sitzes die beiden Seitenstreben und die Beinfüße.')
    if sc == '100':
        s.rect(F, 'seat-band', -0.21, 0.398, 0.42, 0.062, like='seat-band', r=0.012)
        s.poly(F, 'seat-edge', [(-0.2, 0.432), (0.2, 0.432)], like='seat-edge', closed=False)
        for i, g in enumerate(sg):
            s.poly(F, f'leg-front-{i}', [(g * 0.25, 0.0), (g * 0.19, 0.4)], like='leg-front', closed=False)
            s.poly(F, f'leg-rear-{i}', [(g * 0.23, 0.0), (g * 0.18, 0.4)], like='leg-rear', closed=False, z=2.5)
            s.poly(F, f'back-post-{i}', [(g * 0.19, 0.46), (g * 0.19, 0.878)], like='back-post', closed=False)
        s.rect(F, 'back-panel', -0.185, 0.66, 0.37, 0.185, like='back-panel', r=0.014)
        s.rect(F, 'back-rail', -0.185, 0.56, 0.37, 0.07, like='back-rail', r=0.01)
        s.poly(F, 'stretcher', [(-0.225, 0.12), (0.225, 0.12)], like='stretcher', closed=False)
        s.solid(P, 'seat-band', 'seat-band', -0.21, 0.21)
        s.poly(P, 'seat-edge', [(0.05, -0.19), (0.44, -0.19), (0.44, 0.19), (0.05, 0.19)], like='seat-edge', closed=False)
        s.solid(P, 'back-panel', 'back-panel', -0.185, 0.185)
        s.poly(P, 'back-post-0', [(0.066, 0.19), (0.03, 0.19)], like='back-post', closed=False)
        s.poly(P, 'back-post-1', [(0.066, -0.19), (0.03, -0.19)], like='back-post', closed=False)
        for i, g in enumerate(sg):
            s.poly(P, f'leg-front-{i}', [(0.404, g * 0.19), (0.42, g * 0.25)], like='leg-front', closed=False)
            s.poly(P, f'leg-rear-{i}', [(0.065, g * 0.18), (0.05, g * 0.23)], like='leg-rear', closed=False)
            s.poly(P, f'stretcher-{i}', [(0.054, g * 0.225), (0.416, g * 0.225)], like='stretcher', closed=False)
        return ('Front view: seat band with its edge line, the back panel and lower rail between the posts, front legs and the rear legs behind them and the side stretchers as one line between the legs. Plan: the seat plate with the inset edge, the back panel and posts at the rear, four leg feet and the two side stretchers outside the seat.',
                'Vorderansicht: Sitzband mit Kantenlinie, Lehnenfüllung und untere Querstrebe zwischen den Pfosten, Vorderbeine mit den Hinterbeinen dahinter und die Seitenstreben als eine Linie zwischen den Beinen. Grundriss: die Sitzplatte mit der eingerückten Kante, Lehnenfüllung und Pfosten hinten, vier Beinfüße und die zwei Seitenstreben außerhalb des Sitzes.')
    s.path(F, 'seat-pad', 'M -0.2 0.46 L 0.2 0.46 C 0.213 0.46 0.215 0.454 0.215 0.446 C 0.215 0.438 0.213 0.432 0.2 0.432 L -0.2 0.432 C -0.213 0.432 -0.215 0.438 -0.215 0.446 C -0.215 0.454 -0.213 0.46 -0.2 0.46 Z', like='seat-pad')
    s.rect(F, 'seat-apron', -0.18, 0.398, 0.36, 0.036, like='seat-apron', r=0.004)
    s.poly(F, 'seat-edge', [(-0.21, 0.4455), (0.21, 0.4455)], like='seat-edge', closed=False)
    for i, g in enumerate(sg):
        yb, yt = 0.0, 0.42
        s.poly(F, f'leg-rear-tube-{i}', [(g * (rl(yb) - 0.0105), yb), (g * (rl(yb) + 0.0105), yb), (g * (rl(yt) + 0.0095), yt), (g * (rl(yt) - 0.0095), yt)], like='leg-rear-tube', z=2.8)
        s.poly(F, f'leg-front-tube-{i}', [(g * (fl(yb) - 0.0125), yb), (g * (fl(yb) + 0.0125), yb), (g * (fl(yt) + 0.0095), yt), (g * (fl(yt) - 0.0095), yt)], like='leg-front-tube', followTip=True)
        s.rect(F, f'leg-foot-front-{i}', g * fl(0) - 0.0115, 0.0, 0.023, 0.016, like='leg-foot-front', r=0.003)
        s.rect(F, f'leg-foot-rear-{i}', g * rl(0) - 0.0115, 0.0, 0.023, 0.016, like='leg-foot-rear', r=0.003, z=2.9)
        s.ell(F, f'stretcher-joint-{i}', g * 0.225, 0.12, 0.0085, 0.0095, like='stretcher-joint-a')
        s.rect(F, f'back-post-{i}', g * 0.19 - 0.011, 0.46, 0.022, 0.405, like='back-post', followTip=True, r=0.008)
        s.ell(F, f'back-finial-{i}', g * 0.19, 0.87, 0.012, 0.011, like='back-finial')
    s.rect(F, 'back-panel', -0.185, 0.674, 0.37, 0.168, like='back-panel', r=0.009, followTip=True)
    s.rect(F, 'back-lower-rail', -0.185, 0.57, 0.37, 0.06, like='back-lower-rail', r=0.006, followTip=True)
    s.poly(F, 'back-panel-line', [(-0.17, 0.762), (0.17, 0.762)], like='back-panel-line', closed=False)
    s.path(F, 'seat-grain', 'M -0.14 0.452 C -0.07 0.455 0.0 0.449 0.08 0.453 M 0.02 0.44 C 0.08 0.443 0.12 0.437 0.17 0.441', like='seat-grain')
    s.solid(P, 'seat-pad', 'seat-pad', -0.215, 0.215)
    s.poly(P, 'seat-edge', [(0.06, -0.195), (0.44, -0.195), (0.44, 0.195), (0.06, 0.195)], like='seat-edge', closed=False)
    s.path(P, 'seat-grain', 'M 0.12 -0.15 C 0.17 -0.14 0.24 -0.16 0.3 -0.15 M 0.15 0.05 C 0.22 0.06 0.3 0.04 0.38 0.05', like='seat-grain')
    s.solid(P, 'back-panel', 'back-panel', -0.185, 0.185)
    s.poly(P, 'back-panel-line', [(0.041, -0.17), (0.041, 0.17)], like='back-panel-line', closed=False)
    for i, g in enumerate(sg):
        s.cap(P, f'back-post-{i}', 0.066, g * 0.19, 0.031, g * 0.19, 0.013, 0.009, like='back-post', followTip=True)
        s.ell(P, f'back-finial-{i}', 0.03, g * 0.19, 0.012, 0.012, like='back-finial')
        s.cap(P, f'leg-front-tube-{i}', 0.404, g * 0.19, 0.42, g * 0.25, 0.0095, 0.0125, like='leg-front-tube', followTip=True)
        s.cap(P, f'leg-rear-tube-{i}', 0.0647, g * 0.18, 0.05, g * 0.23, 0.0095, 0.0125, like='leg-rear-tube', followTip=True)
        s.ell(P, f'leg-foot-front-{i}', 0.42, g * 0.25, 0.0115, 0.0115, like='leg-foot-front')
        s.ell(P, f'leg-foot-rear-{i}', 0.05, g * 0.23, 0.0115, 0.0115, like='leg-foot-rear')
        s.cap(P, f'stretcher-tube-{i}', 0.054, g * 0.225, 0.416, g * 0.225, 0.007, like='stretcher-tube')
    return ('Front view: the seat pad over its apron, the front legs splayed outward to their feet with the rear legs just behind them, the stretcher joints, the two back posts with finials and the back panel and lower rail between them. Plan: the seat pad with edge line and grain, the back panel and posts across the rear edge and, outside the seat, the stretcher tubes and the four round feet.',
            'Vorderansicht: das Sitzpolster über der Zarge, die nach außen gespreizten Vorderbeine bis zu den Füßen mit den Hinterbeinen knapp dahinter, die Strebenverbindungen, die beiden Lehnenpfosten mit Endknöpfen und dazwischen Lehnenfüllung und untere Querstrebe. Grundriss: das Sitzpolster mit Kantenlinie und Maserung, Lehnenfüllung und Pfosten am hinteren Rand und außerhalb des Sitzes die Strebenrohre und die vier runden Füße.')


if __name__ == '__main__':
    run('cafe-chair', build)
