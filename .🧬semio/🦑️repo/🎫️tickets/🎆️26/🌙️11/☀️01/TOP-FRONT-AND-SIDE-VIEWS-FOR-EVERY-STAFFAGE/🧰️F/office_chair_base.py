from lib import *

CASTORS = [('front', 0.3, 0.0, 'front'), ('side-a', 0.093, 0.285, 'side-a'), ('side-b', 0.093, -0.285, 'side-a'), ('back-a', -0.243, 0.176, 'back-a'), ('back-b', -0.243, -0.176, 'back-a')]
SY = 0.445


def first(s, *names):
    for n in names:
        if n in s.elev:
            return n
    raise KeyError(names)


def build(s, sc):
    F, P = 'front', 'plan'
    bk = first(s, 'backrest-shell', 'backrest')
    for view in (F, P):
        if sc == '500':
            s.solid(view, 'backrest', bk, -0.22, 0.22, kind='rect', geo={'x': 0.06, 'y': -0.05, 'w': 0.575, 'h': 0.1}, filled=True, role='body', z=4, tone=0.5, regions=['chair-plastic'], material='pa-plastic')
        else:
            s.solid(view, 'backrest', bk, -0.22, 0.22, filled=True, role='body', z=4)
    wheel = lambda kin: first(s, f'castor-{kin}-wheel', f'castor-{kin}')
    for name, x, y, kin in CASTORS:
        w = 0.03 if sc == '500' else 0.025
        s.rect(F, f'castor-{name}-wheel', y - w / 2, 0.0, w, 0.05, like=wheel(kin), bone=f'castor-{name}', r=w / 4)
        if sc == '50':
            s.rect(F, f'castor-{name}-hub', y - 0.004, 0.017, 0.008, 0.016, like=f'castor-{kin}-hub', bone=f'castor-{name}')
        s.ell(P, f'castor-{name}-wheel', x, y, 0.028 if sc == '500' else 0.025, 0.028 if sc == '500' else 0.025, like=wheel(kin), bone=f'castor-{name}')
        if sc == '50':
            s.ell(P, f'castor-{name}-hub', x, y, 0.008, 0.008, like=f'castor-{kin}-hub', bone=f'castor-{name}')
        arm_like = first(s, f'arm-{kin}', 'base-star', 'base-bar')
        s.cap(P, f'arm-{name}', 0.0, 0.0, x, y, 0.02, 0.016, like=arm_like, bone=f'arm-{name}', followTip=True, z=2.5, filled=True, role='body')
        if name != 'front':
            lat = abs(y)
            sg = 1 if y > 0 else -1
            if sc == '500':
                continue
            s.poly(F, f'arm-{name}', [(sg * 0.03, 0.059), (sg * (lat - 0.012), 0.047), (sg * (lat - 0.012), 0.022), (sg * 0.03, 0.027)], like=arm_like, bone=f'arm-{name}', z=2.5, filled=True, role='body')
    if sc == '500':
        s.rect(F, 'base-bar', -0.285, 0.02, 0.57, 0.03, like='base-bar')
        s.poly(F, 'column', [(0.0, 0.05), (0.0, 0.445)], like='column', closed=False, followTip=True)
        s.rect(F, 'seat-band', -0.24, SY - 0.055, 0.48, 0.1, like='seat-band')
        s.rect(P, 'seat-band', -0.23, -0.24, 0.462, 0.48, like='seat-band', r=0.06)
        return ('Front view: the base bar with the five castors, the column as a line, the seat band and the backrest as a panel that tilts back. Plan: the seat as a rounded plate, the backrest behind it as the extruded profile that leans back when it reclines, and the five star arms with their castors, which reach out past the seat.',
                'Vorderansicht: der Fußstern mit den fünf Rollen, die Säule als Linie, das Sitzband und die Rückenlehne als eine Fläche. Grundriss: der Sitz als gerundete Platte, die Rückenlehne dahinter als gezogenes Profil, das sich beim Zurücklehnen nach hinten neigt, und die fünf Sternarme mit ihren Rollen, die über den Sitz hinausreichen.')
    hub = first(s, 'hub', 'base-star')
    s.rect(F, 'hub', -0.055, -0.02 + 0.025, 0.11, 0.062, like=hub, bone='base', z=2.6, r=0.01)
    s.rect(F, 'arm-front', -0.03, 0.022, 0.06, 0.037, like=first(s, 'arm-front', 'base-star'), bone='arm-front', z=2.7)
    sleeve = first(s, 'column-sleeve')
    s.rect(F, 'column-sleeve', -0.03, 0.025, 0.06, 0.3, like=sleeve, followTip=True, r=0.004)
    rod = first(s, 'column', 'column-rod')
    s.rect(F, 'column', -0.0125, 0.225, 0.025, 0.23, like=rod, followTip=True, filled=True, role='body')
    cushion = first(s, 'seat-cushion')
    seatpath = 'M -0.2 0.429 L 0.2 0.429 C 0.225 0.429 0.24 0.44 0.24 0.46 C 0.24 0.478 0.225 0.485 0.2 0.485 L -0.2 0.485 C -0.225 0.485 -0.24 0.478 -0.24 0.46 C -0.24 0.44 -0.225 0.429 -0.2 0.429 Z'
    if sc == '200':
        s.rect(F, 'seat-cushion', -0.24, SY, 0.48, 0.035, like=cushion)
        s.poly(F, 'backrest-strut', [(0.0, SY + 0.012), (0.0, SY + 0.1)], like='backrest-strut', closed=False, followTip=True)
        for g in (1, -1):
            s.poly(F, f'armrest-{g}', [(g * 0.27, SY + 0.04), (g * 0.27, SY + 0.215)], like='armrest', closed=False)
        s.rect(P, 'seat-cushion', -0.23, -0.24, 0.462, 0.48, like=cushion, r=0.06)
        for g in (1, -1):
            s.poly(P, f'armrest-{g}', [(-0.19, g * 0.27), (0.115, g * 0.27)], like='armrest', closed=False)
        return ('Front view: the star arms with the hub, the five castors, sleeve and rod of the column, the seat cushion, the two armrest lines and the backrest shell on its strut; column, seat and backrest ride up and lean with their bones. Plan: the rounded seat, the armrests as two lines at its sides, the backrest as the extruded shell profile that leans back when it reclines and the five star arms with their castors.',
                'Vorderansicht: die Sternarme mit der Nabe, die fünf Rollen, Hülse und Stange der Säule, das Sitzpolster, die beiden Armlehnenlinien und die Rückenschale auf ihrer Strebe; Säule, Sitz und Lehne fahren mit ihren Knochen hoch und neigen sich. Grundriss: der gerundete Sitz, die Armlehnen als zwei Linien an seinen Seiten, die Rückenlehne als gezogenes Schalenprofil, das sich beim Zurücklehnen nach hinten neigt, und die fünf Sternarme mit ihren Rollen.')
    s.path(F, 'seat-cushion', seatpath, like=cushion)
    s.rect(F, 'seat-mechanism', -0.1, SY - 0.062, 0.2, 0.042, like='seat-mechanism', r=0.006)
    s.rect(F, 'backrest-strut', -0.011, SY + 0.0, 0.022, 0.25, like='backrest-strut', followTip=True)
    for i, g in enumerate((1, -1)):
        s.rect(F, f'armrest-post-{g}', g * 0.27 - 0.01, SY + 0.04, 0.02, 0.16, like='armrest-post', bone='seat')
        s.rect(F, f'armrest-pad-{g}', g * 0.27 - 0.02, SY + 0.197, 0.04, 0.036, like='armrest-pad', bone='seat', r=0.014)
    s.rect(P, 'seat-cushion', -0.23, -0.24, 0.462, 0.48, like=cushion, r=0.06)
    for g in (1, -1):
        s.cap(P, f'armrest-pad-{g}', -0.19, g * 0.27, 0.115, g * 0.27, 0.014, 0.018, like='armrest-pad', bone='seat')
    if sc == '100':
        return ('Front view: the star with hub, arms and five castors, the column sleeve and rod, the seat mechanism, the cushion, the two armrests on their posts and the backrest shell on its strut. Plan: the rounded seat, the two armrest pads, the backrest as the extruded shell profile that leans back when it reclines and the five star arms with castors.',
                'Vorderansicht: der Stern mit Nabe, Armen und fünf Rollen, Säulenhülse und -stange, die Sitzmechanik, das Polster, die beiden Armlehnen auf ihren Stützen und die Rückenschale auf ihrer Strebe. Grundriss: der gerundete Sitz, die zwei Armlehnenpolster, die Rückenlehne als gezogenes Schalenprofil, das sich beim Zurücklehnen nach hinten neigt, und die fünf Sternarme mit Rollen.')
    s.poly(F, 'seat-piping', [(-0.17, 0.475), (0.17, 0.475)], like='seat-piping', closed=False)
    s.rect(F, 'column-collar', -0.036, 0.31, 0.072, 0.02, like='column-collar', bone='column-sleeve', r=0.003)
    s.poly(P, 'seat-piping', [(-0.19, -0.2), (0.17, -0.2), (0.2, -0.17), (0.2, 0.17), (0.17, 0.2), (-0.19, 0.2)], like='seat-piping', closed=False, filled=False)
    s.path(P, 'seat-fabric-fold', 'M -0.10 -0.1 C -0.05 -0.09 0.02 -0.11 0.09 -0.1', like='seat-fabric-fold')
    return ('Front view: star, hub, the five castors with their hubs, column sleeve with collar and rod, the seat mechanism, the cushion with piping, the armrests on their posts and the backrest shell on its strut; every part rides up and leans with its bone. Plan: the rounded seat with piping and a fabric fold, the armrest pads, the backrest as the extruded shell profile that leans back when it reclines and the five star arms with castors and hubs.',
            'Vorderansicht: Stern, Nabe, die fünf Rollen mit ihren Naben, Säulenhülse mit Manschette und Stange, die Sitzmechanik, das Polster mit Keder, die Armlehnen auf ihren Stützen und die Rückenschale auf ihrer Strebe; jedes Teil fährt mit seinem Knochen hoch und neigt sich. Grundriss: der gerundete Sitz mit Keder und einer Stoffwelle, die Armlehnenpolster, die Rückenlehne als gezogenes Schalenprofil, das sich beim Zurücklehnen nach hinten neigt, und die fünf Sternarme mit Rollen und Naben.')


if False:
    run('office-swivel-chair', build)
