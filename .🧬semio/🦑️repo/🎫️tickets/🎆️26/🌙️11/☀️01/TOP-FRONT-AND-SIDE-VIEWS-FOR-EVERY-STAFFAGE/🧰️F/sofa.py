from lib import *

XS = [0.19, 0.7333, 1.2667]
WD = [0.5433, 0.5334, 0.5433]
ARM = 'M -0.45 0.1 L -0.45 0.5 Q -0.45 0.62 -0.33 0.62 L 0.33 0.62 Q 0.45 0.62 0.45 0.5 L 0.45 0.1 Z'
SEAT = 'M -0.42 0.2 L -0.42 0.39 Q -0.42 0.43 -0.38 0.43 L 0.16 0.43 Q 0.2 0.43 0.2 0.39 L 0.2 0.2 Z'
BACK = [(0.19, 0.2), (0.44, 0.2), (0.45, 0.8), (0.42, 0.85), (0.33, 0.85), (0.3, 0.82)]


def feet_front(s, like, top=0.12):
    for i, g in enumerate((1, -1)):
        s.poly('front', f'foot-{i}', [(g * 0.385, 0.0), (g * 0.335, 0.0), (g * 0.325, top), (g * 0.395, top)], like=like)


def build(s, sc):
    F, P = 'front', 'plan'
    if sc == '500':
        s.poly(F, 'sofa-body', [(-0.45, 0.1), (-0.45, 0.55), (-0.4, 0.62), (0.265, 0.62), (0.30, 0.82), (0.33, 0.85), (0.42, 0.85), (0.45, 0.8), (0.45, 0.1)], like='sofa-body')
        s.poly(F, 'seat-line', [(-0.42, 0.43), (0.2, 0.43)], like='seat-line', closed=False, bone='seat-2')
        for i, g in enumerate((1, -1)):
            s.rect(F, f'foot-{i}', g * 0.35 - 0.05, 0.0, 0.1, 0.12, like='foot-l')
        s.rect(P, 'sofa-body', 0.0, -0.45, 2.0, 0.9, like='sofa-body')
        s.poly(P, 'seat-line', [(0.2, -0.42), (1.8, -0.42)], like='seat-line', closed=False)
        return ('Front view: a section through the middle seat, the side profile of the arm behind it as one silhouette with the seat and back, and the two feet; the seat line sinks with the middle cushion. Plan: the whole sofa as one plate with a line along the front edge of the seats.',
                'Vorderansicht: ein Schnitt durch den mittleren Sitz, dahinter das Seitenprofil des Arms als eine Silhouette mit Sitz und Rücken, und die zwei Füße; die Sitzlinie sinkt mit dem mittleren Polster ein. Grundriss: das ganze Sofa als eine Fläche mit einer Linie an der Vorderkante der Sitze.')
    if sc == '200':
        s.path(F, 'arm-l', ARM, like='arm-foot-l', bone='arm-left', z=2.5, regions=['arm-pads'])
        feet_front(s, 'arm-foot-l')
        s.poly(F, 'back-2', BACK, like='back', bone='back-2', z=3)
        s.path(F, 'seat-2', SEAT, like='seat-2')
        s.rect(F, 'skirt', -0.43, 0.1, 0.86, 0.14, like='skirt')
        s.rect(P, 'arm-l', 0.0, -0.45, 0.2, 0.9, like='arm-foot-l', r=0.04, regions=['arm-pads'])
        s.rect(P, 'arm-r', 1.8, -0.45, 0.2, 0.9, like='arm-foot-r', r=0.04, regions=['arm-pads'])
        s.rect(P, 'back', 0.19, 0.19, 1.62, 0.26, like='back', r=0.05)
        for n in (1, 2, 3):
            s.rect(P, f'seat-{n}', XS[n - 1], -0.42, WD[n - 1], 0.62, like=f'seat-{n}', r=0.04)
        for x in (0.7333, 1.2667):
            s.poly(P, f'back-seam-{x}', [(x, 0.2), (x, 0.44)], like=f'back-seam-{0.73 if x < 1 else 1.27}', closed=False)
        return ('Front view: a section through the middle seat with the back cushion behind it, the arm as a rounded silhouette behind the cut, the skirt and the two feet. Plan: the two arms, the three seat cushions and the one back panel with its two seams.',
                'Vorderansicht: ein Schnitt durch den mittleren Sitz mit dem Rückenkissen dahinter, der Arm als gerundete Silhouette hinter dem Schnitt, die Zarge und die zwei Füße. Grundriss: die zwei Arme, die drei Sitzkissen und das eine Rückenteil mit seinen zwei Nähten.')
    if sc == '100':
        s.path(F, 'arm-l', ARM, like='arm-l', z=2.5)
        s.poly(F, 'arm-welt-l', [(-0.45, 0.4), (0.45, 0.4)], like='arm-welt-l', closed=False, z=2.6)
        s.path(F, 'back-2', 'M 0.19 0.2 L 0.44 0.2 L 0.45 0.8 Q 0.45 0.85 0.4 0.85 L 0.34 0.85 Q 0.3 0.85 0.3 0.8 Z', like='back-2')
        s.path(F, 'seat-2', SEAT, like='seat-2')
        s.poly(F, 'seat-piping-2', [(-0.39, 0.375), (0.17, 0.375)], like='seat-piping-2', closed=False)
        s.rect(F, 'skirt', -0.43, 0.1, 0.86, 0.14, like='skirt')
        feet_front(s, 'foot-l')
        s.rect(P, 'arm-l', 0.0, -0.45, 0.2, 0.9, like='arm-l', r=0.04)
        s.rect(P, 'arm-r', 1.8, -0.45, 0.2, 0.9, like='arm-r', r=0.04)
        for n in (1, 2, 3):
            s.rect(P, f'back-{n}', XS[n - 1], 0.19, WD[n - 1], 0.26, like=f'back-{n}', r=0.05)
            s.rect(P, f'seat-{n}', XS[n - 1], -0.42, WD[n - 1], 0.62, like=f'seat-{n}', r=0.04)
            s.poly(P, f'seat-piping-{n}', [(XS[n - 1] + 0.06, -0.38), (XS[n - 1] + WD[n - 1] - 0.06, -0.38)], like=f'seat-piping-{n}', closed=False)
        return ('Front view: a section through the middle seat and back cushion with their piping, the arm with its welt as the backdrop, the skirt and the two feet. Plan: the two arms, the three back cushions and the three seat cushions with a piping line along each front edge.',
                'Vorderansicht: ein Schnitt durch das mittlere Sitz- und Rückenkissen mit ihrem Keder, dahinter der Arm mit seiner Naht, die Zarge und die zwei Füße. Grundriss: die zwei Arme, die drei Rückenkissen und die drei Sitzkissen mit einer Kederlinie an jeder Vorderkante.')
    s.path(F, 'arm-l', ARM, like='arm-l', z=2.5)
    s.poly(F, 'arm-welt-l', [(-0.45, 0.22), (0.45, 0.22)], like='arm-welt-l', closed=False, z=2.6)
    s.poly(F, 'arm-top-piping-l', [(-0.3, 0.56), (0.3, 0.56)], like='arm-top-piping-l', closed=False, z=2.6)
    s.path(F, 'back-2', 'M 0.19 0.2 L 0.44 0.2 L 0.45 0.8 Q 0.45 0.85 0.4 0.85 L 0.34 0.85 Q 0.3 0.85 0.3 0.8 Z', like='back-2')
    s.poly(F, 'back-piping-2', [(0.31, 0.8), (0.44, 0.8)], like='back-piping-2', closed=False)
    s.path(F, 'seat-2', SEAT, like='seat-2')
    s.poly(F, 'seat-piping-2', [(-0.39, 0.375), (0.17, 0.375)], like='seat-piping-2', closed=False)
    s.rect(F, 'skirt', -0.43, 0.1, 0.86, 0.14, like='skirt')
    feet_front(s, 'foot-l')
    s.poly(F, 'foot-ring', [(-0.39, 0.065), (-0.33, 0.065)], like='foot-ring-l', closed=False)
    s.poly(F, 'foot-ring-b', [(0.33, 0.065), (0.39, 0.065)], like='foot-ring-r', closed=False)
    s.rect(P, 'arm-l', 0.0, -0.45, 0.2, 0.9, like='arm-l', r=0.04)
    s.rect(P, 'arm-r', 1.8, -0.45, 0.2, 0.9, like='arm-r', r=0.04)
    s.poly(P, 'arm-top-piping-l', [(0.04, -0.4), (0.16, -0.4), (0.16, 0.4), (0.04, 0.4)], like='arm-top-piping-l', closed=True, filled=False)
    s.poly(P, 'arm-top-piping-r', [(1.84, -0.4), (1.96, -0.4), (1.96, 0.4), (1.84, 0.4)], like='arm-top-piping-r', closed=True, filled=False)
    for n in (1, 2, 3):
        x, w = XS[n - 1], WD[n - 1]
        s.rect(P, f'back-{n}', x, 0.19, w, 0.26, like=f'back-{n}', r=0.055)
        s.poly(P, f'back-piping-{n}', [(x + 0.04, 0.33), (x + w - 0.04, 0.33)], like=f'back-piping-{n}', closed=False)
        s.rect(P, f'seat-{n}', x, -0.42, w, 0.62, like=f'seat-{n}', r=0.04)
        s.poly(P, f'seat-piping-{n}', [(x + 0.04, -0.38), (x + w - 0.04, -0.38), (x + w - 0.04, 0.16), (x + 0.04, 0.16)], like=f'seat-piping-{n}', closed=False)
    return ('Front view: a section through the middle seat and back cushion with their piping, the arm with welt and top piping behind the cut, the skirt and the tapered feet with their rings. Plan: the two arms with top piping, the three back cushions with their piping and the three seat cushions with piping around three sides.',
            'Vorderansicht: ein Schnitt durch das mittlere Sitz- und Rückenkissen mit ihrem Keder, dahinter der Arm mit Naht und Oberkeder, die Zarge und die verjüngten Füße mit ihren Ringen. Grundriss: die zwei Arme mit Oberkeder, die drei Rückenkissen mit ihrem Keder und die drei Sitzkissen mit Keder an drei Seiten.')


if __name__ == '__main__':
    run('three-seat-sofa', build)
