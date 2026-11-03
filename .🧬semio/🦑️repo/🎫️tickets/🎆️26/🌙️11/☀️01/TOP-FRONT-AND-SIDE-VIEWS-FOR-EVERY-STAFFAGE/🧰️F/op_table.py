from lib import *

HY = 1.66
CY = 0.16
SX = 1.17
HX = 0.67


def head_path(y0, hi=0.12):
    return f'M -0.25 {r4(HY + y0)} L 0.25 {r4(HY + y0)} L 0.25 {r4(HY + 0.04)} C 0.25 {r4(HY + 0.1)} 0.12 {r4(HY + hi)} 0 {r4(HY + hi)} C -0.12 {r4(HY + hi)} -0.25 {r4(HY + 0.1)} -0.25 {r4(HY + 0.04)} Z'


def pre(a):
    for b in a['rig']['bones']:
        if b['id'] == 'stand-base':
            b['x'] = 1.17
    a['dimensions']['widthM'] = 2.45
    swaps = {'200': [('10 mm by 9.8 mm', '12.3 mm by 9.8 mm'), ('10 mm mal 9,8 mm', '12,3 mm mal 9,8 mm')], '100': [('20 mm by 19.6 mm', '24.5 mm by 19.6 mm'), ('20 mm mal 19,6 mm', '24,5 mm mal 19,6 mm')], '50': [('A 2.0 m long, 1.96 m high sign, 40 mm by 39 mm', 'A 2.45 m long, 1.96 m high sign, 49 mm by 39 mm'), ('Ein 2,0 m langes und 1,96 m hohes Zeichen, auf Papier 40 mm mal 39 mm', 'Ein 2,45 m langes und 1,96 m hohes Zeichen, auf Papier 49 mm mal 39 mm')]}
    for sc, pairs in swaps.items():
        d = a['representations'][sc]['decisions']['silhouette']
        for old, new in pairs:
            for lang in ('en', 'de'):
                d[lang] = d[lang].replace(old, new)
    a['structure']['assumptions'].append({'en': 'The light stand stands beyond the head end of the table (column at 1.17 m, the table top ends at 1.0 m), so neither the level nor the tilted top can touch the column; the arm reaches back over the table.', 'de': 'Der Leuchtenständer steht hinter dem Kopfende des Tisches (Säule bei 1,17 m, die Tischplatte endet bei 1,0 m), sodass weder die ebene noch die gekippte Platte die Säule berühren kann; der Arm reicht zurück über den Tisch.'})


def ends(s, sc, top):
    y0, h = (0.0, 0.12) if sc == '200' else (0.04, 0.08)
    for k, x in enumerate((-1.0, 0.97)):
        s.solid('front', f'top-end-{k}', top, -0.25, 0.25, kind='rect', geo={'x': x, 'y': y0, 'w': 0.03, 'h': h}, z=3.8, tone=0.5)
    if sc in ('50', '100'):
        for k, x in enumerate((-0.98, 0.95)):
            s.solid('front', f'carrier-end-{k}', 'top-carrier', -0.2, 0.2, kind='rect', geo={'x': x, 'y': 0.0, 'w': 0.03, 'h': 0.04}, z=3.9, tone=0.7)


def build(s, sc):
    F, P = 'front', 'plan'
    E = s.elev
    top = 'top' if 'top' in E else 'top-pad'
    s.solid(F, top, top, -0.25, 0.25, z=E[top]['z'])
    s.solid(P, top, top, -0.25, 0.25, z=E[top]['z'])
    if sc == '50':
        s.solid(F, 'top-carrier', 'top-carrier', -0.2, 0.2, z=3.5)
        s.solid(P, 'top-carrier', 'top-carrier', -0.2, 0.2, z=3.5)
    ends(s, sc, top)
    if sc == '200':
        s.poly(F, 'st-base', [(-0.26, 0.0), (0.26, 0.0), (0.22, 0.105), (-0.22, 0.105)], like='st-base', z=6)
        s.rect(F, 'st-column', -0.04, 0.105, 0.08, 1.795, like='st-column', z=9)
        s.rect(F, 'st-arm', -0.03, 1.89, 0.06, 0.06, like='st-arm', z=9.1)
        s.path(F, 'head', head_path(0.0), like='head')
        s.poly(F, 'tb-plate', [(-0.2, 0.0), (0.2, 0.0), (0.2, 0.12), (0.15, 0.16), (-0.15, 0.16), (-0.2, 0.12)], like='tb-plate')
        s.poly(F, 'col', [(-0.16, CY), (0.16, CY), (0.11, CY + 0.62), (-0.11, CY + 0.62)], like='col')
        s.rect(P, 'st-base', 0.89, -0.26, 0.56, 0.52, like='st-base', z=1)
        s.rect(P, 'st-arm', HX, -0.03, 0.54 - 0.04, 0.06, like='st-arm', z=9)
        s.ell(P, 'head', HX, 0.0, 0.25, 0.25, like='head', z=8)
        return ('Front view: the pivot section of the table, a tapered base and column under the pad, which is drawn as its extruded side profile so it rises and dips when the table tilts, and the stand with its column in front of the round light head. Plan: the pad as the extruded profile, the stand base showing at its sides, the arm and the round light head above it.',
                'Vorderansicht: der Schwenkschnitt des Tisches, ein verjüngter Fuß und Säule unter der Polsterfläche, die als gezogenes Seitenprofil gezeichnet ist und sich beim Kippen des Tisches hebt und senkt, und der Ständer mit seiner Säule vor dem runden Leuchtenkopf. Grundriss: die Polsterfläche als gezogenes Profil, der Ständerfuß an ihren Seiten sichtbar, der Arm und darüber der runde Leuchtenkopf.')
    if sc == '100':
        s.poly(F, 'st-base', [(-0.26, 0.06), (0.26, 0.06), (0.22, 0.105), (-0.22, 0.105)], like='st-base', z=6)
        for i, g in enumerate((1, -1)):
            s.rect(F, f'st-castor-{i}', g * 0.2 - 0.015, 0.0, 0.03, 0.06, like='st-castor-a', z=6.1, r=0.01)
            s.rect(F, f'tb-castor-{i}', g * 0.17 - 0.02, 0.0, 0.04, 0.08, like='tb-castor-a', r=0.012)
        s.rect(F, 'st-column', -0.035, 0.105, 0.07, 1.775, like='st-column', z=9)
        s.rect(F, 'st-arm', -0.0225, 1.9, 0.045, 0.045, like='st-arm', z=9.1)
        s.path(F, 'head-housing', head_path(0.015), like='head-housing')
        s.rect(F, 'head-lens', -0.24, HY - 0.005, 0.48, 0.02, like='head-lens')
        s.rect(F, 'head-handle', -0.02, HY - 0.06, 0.04, 0.06, like='head-handle')
        s.poly(F, 'tb-plate', [(-0.2, 0.07), (0.2, 0.07), (0.2, 0.12), (0.15, 0.16), (-0.15, 0.16), (-0.2, 0.12)], like='tb-plate')
        s.poly(F, 'col-lower', [(-0.15, CY), (0.15, CY), (0.13, CY + 0.24), (-0.13, CY + 0.24)], like='col-lower')
        s.rect(F, 'col-upper', -0.105, CY + 0.24, 0.21, 0.34, like='col-upper')
        s.rect(F, 'col-head', -0.2, CY + 0.58, 0.4, 0.04, like='col-head')
        s.solid(F, 'top-carrier', 'top-carrier', -0.2, 0.2, z=3.5)
        s.solid(P, 'top-carrier', 'top-carrier', -0.2, 0.2, z=3.5)
        s.rect(P, 'st-base', 0.89, -0.26, 0.56, 0.52, like='st-base', z=1)
        s.rect(P, 'st-arm', HX, -0.0225, 0.52, 0.045, like='st-arm', z=9)
        s.ell(P, 'head-housing', HX, 0.0, 0.25, 0.25, like='head-housing', z=8)
        s.ell(P, 'st-rod', HX, 0.0, 0.03, 0.03, like='st-rod', z=9.5)
        for i, x in enumerate((-0.62, -0.05, 0.2)):
            s.solid(P, f'pad-joint-{i}', 'top-pad', -0.24, 0.24, kind='poly', geo={'points': [(x, 0.04), (x, 0.12)], 'closed': False}, z=3.7, role='body', filled=False, tone=0.5)
        return ('Front view: the pivot section with the base plate and castors, the column in its stacked sections, the head and the carrier and pad as extruded side profiles that rise and dip when the table tilts, and the stand with its column and castors in front of the light head with lens and handle. Plan: carrier and pad as extruded profiles with the pad joints, the stand base at the sides, the arm and the round light head with its knob.',
                'Vorderansicht: der Schwenkschnitt mit Fußplatte und Rollen, die Säule in ihren gestapelten Abschnitten, der Kopf und Träger und Polster als gezogene Seitenprofile, die sich beim Kippen des Tisches heben und senken, und der Ständer mit Säule und Rollen vor dem Leuchtenkopf mit Linse und Griff. Grundriss: Träger und Polster als gezogene Profile mit den Polsterfugen, der Ständerfuß an den Seiten, der Arm und der runde Leuchtenkopf mit seinem Knauf.')
    s.poly(F, 'st-base', [(-0.26, 0.06), (0.26, 0.06), (0.22, 0.105), (-0.22, 0.105)], like='st-base', z=6)
    for i, g in enumerate((1, -1)):
        s.rect(F, f'st-castor-{i}', g * 0.2 - 0.015, 0.0, 0.03, 0.06, like='st-castor-a', z=6.1, r=0.01)
        s.rect(F, f'tb-castor-{i}', g * 0.17 - 0.02, 0.0, 0.04, 0.08, like='tb-castor-a', r=0.012)
    s.rect(F, 'st-column', -0.03, 0.105, 0.06, 1.775, like='st-column', z=9)
    s.rect(F, 'st-collar', -0.045, 1.88, 0.09, 0.08, like='st-collar', z=9.1)
    s.path(F, 'head-housing', head_path(0.015), like='head-housing')
    s.rect(F, 'head-lens', -0.24, HY, 0.48, 0.015, like='head-lens')
    s.rect(F, 'head-handle', -0.02, HY - 0.06, 0.04, 0.06, like='head-handle')
    s.poly(F, 'head-groove', [(-0.24, HY + 0.055), (0.24, HY + 0.055)], like='head-groove', closed=False)
    s.poly(F, 'tb-plate', [(-0.2, 0.07), (0.2, 0.07), (0.2, 0.12), (0.15, 0.16), (-0.15, 0.16), (-0.2, 0.12)], like='tb-plate')
    s.poly(F, 'col-lower', [(-0.15, CY), (0.15, CY), (0.13, CY + 0.24), (-0.13, CY + 0.24)], like='col-lower')
    s.rect(F, 'col-upper', -0.105, CY + 0.24, 0.21, 0.34, like='col-upper')
    s.rect(F, 'col-head', -0.2, CY + 0.58, 0.4, 0.04, like='col-head')
    for k in range(5):
        y = CY + 0.3 + 0.06 * k
        s.poly(F, f'pleat-{k + 1}', [(-0.105, y), (0.105, y)], like=f'pleat-{k + 1}', closed=False)
    s.rect(P, 'st-base', 0.89, -0.26, 0.56, 0.52, like='st-base', z=1, r=0.03)
    s.rect(P, 'st-arm', HX, -0.02, 0.5, 0.04, like='st-arm', z=9)
    s.ell(P, 'head-housing', HX, 0.0, 0.25, 0.25, like='head-housing', z=8)
    s.ell(P, 'head-groove', HX, 0.0, 0.2, 0.2, like='head-groove', filled=False, z=8.5)
    s.ell(P, 'st-collar', SX, 0.0, 0.045, 0.045, like='st-collar', z=9.1)
    s.ell(P, 'st-knob', HX, 0.0, 0.03, 0.03, like='st-knob', z=9.5)
    s.ell(P, 'st-rod', HX, 0.0, 0.012, 0.012, like='st-rod', z=9.6)
    for i, x in enumerate((-0.62, -0.05, 0.2)):
        s.solid(P, f'pad-joint-{i + 1}', f'pad-joint-{i + 1}', -0.24, 0.24)
    return ('Front view: the pivot section with the base plate, castors, the stacked column with its bellows pleats and head, the carrier and the rounded pad as extruded side profiles that rise and dip when the table tilts, and the stand with its collar, column and castors in front of the light head with lens, handle and groove. Plan: carrier and pad as extruded profiles with the three pad joints, the stand base at the sides, the arm with the stand collar and the round light head with its groove, knob and rod.',
            'Vorderansicht: der Schwenkschnitt mit Fußplatte, Rollen, der gestapelten Säule mit ihren Faltenbalgfalten und dem Kopf, Träger und gerundetes Polster als gezogene Seitenprofile, die sich beim Kippen des Tisches heben und senken, und der Ständer mit Manschette, Säule und Rollen vor dem Leuchtenkopf mit Linse, Griff und Rille. Grundriss: Träger und Polster als gezogene Profile mit den drei Polsterfugen, der Ständerfuß an den Seiten, der Arm mit der Ständermanschette und der runde Leuchtenkopf mit Rille, Knauf und Stange.')


if __name__ == '__main__':
    def noted(s, sc):
        en, de = build(s, sc)
        return (en + ' The two end faces of the pad are drawn on top of the tilt envelope so the tilt reads as a slab, not a block; the joints follow the tilt in plan.',
                de + ' Die beiden Stirnflächen des Polsters liegen über der Kipphülle, damit das Kippen als Platte und nicht als Block lesbar ist; die Fugen folgen im Grundriss der Neigung.')
    run('operating-table', noted, pre)
