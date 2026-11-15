"""Author 32: city-bus thermal, structure and decisions for all four scales."""
import sys; sys.path.insert(0, '.')
from author_32_kit import *

A = 'vehicles/city-bus'
BD = 'body'
FE = 'cast-iron'

def wall_shells(T, v, ids, region, t, floor=0.004):
    """outer layer (insulation) = reference of the body solid, inner layer (skin) = inset shell by the insulation thickness"""
    for tid, i, sf in ids:
        T.append(ref(f'th-{tid}-insulation', i, region, 'insulation', sf))
        T.append(shell(body_of(v, i), t, f'th-{tid}-skin', region, 'skin', floor))

def thermal50(v):
    T = []
    ids = [('shell', 'body-shell', ['-v']), ('bumper-f', 'bumper-front', None), ('bumper-r', 'bumper-rear', None)]
    for sd in ('l', 'r'):
        for n in ('r', 'f'):
            ids.append((f'wheelhouse-{n}-{sd}', f'wheelhouse-{n}-{sd}', None))
    wall_shells(T, v, [x for x in ids if not x[0].startswith('wheelhouse')], 'body-panels', 0.04)
    for tid, i, sf in ids:
        if tid.startswith('wheelhouse'):
            T.append(ref(f'th-{tid}-insulation', i, 'body-panels', 'insulation'))
    wall_shells(T, v, [('roof', 'roof-panel', None)], 'roof', 0.04)
    wall_shells(T, v, [(f'leaf-{n}', f'leaf-{n}', None) for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd')], 'doors', 0.04)
    for s in v['body']:
        if s['role'] == 'glazing' and (s['id'].startswith(('side-window', 'upper-window')) or s['id'] == 'windscreen'):
            i = s['id']
            T.append(ref(f'th-{i}-pane-in', i, 'glazing', 'pane-in'))
            T.append(shell(s, 0.004, f'th-{i}-gap', 'glazing', 'gap', 0.004))
            T.append(shell(s, 0.016, f'th-{i}-pane-out', 'glazing', 'pane-out', 0.004))
    T.append(ref('th-battery-pack', 'battery-pack', 'battery-pack'))
    T.append(ref('th-hvac-unit', 'hvac-unit', 'hvac-unit'))
    T.append(th(ell('th-hvac-vent', 'hvac', 'hvac-housing', 'detail', 0.5, (0.012, 0.07, 0.35), (0.0, 0.08, 0)), 'hvac-unit', None, ['hvac-vent']))
    T.append(ref('th-e-axle-casting', 'e-axle-casting', 'e-axle', 'casting'))
    T.append(ref('th-axle-beam-r', 'axle-beam-r', 'e-axle', 'casting'))
    for i in ('fl', 'fr', 'rl', 'rr'):
        T.append(ref(f'th-tread-{i}', f'{i}-tyre', 'tyres', 'tread'))
    for sg, n in ((1, 'l'), (-1, 'r')):
        for bone, p in (('wheel-f', 'f'), ('wheel-r', 'r')):
            T.append(th(rev(f'th-drum-{p}{n}', bone, FE, 'detail', 0.4, [(-0.02, 0.1), (-0.02, 0.235), (0.02, 0.235), (0.02, 0.1)], (0, 0, sg * 0.88)), 'brakes'))
    return T

def structure50(v):
    S = []
    M = lambda s, bone, part='member': S.append(st(s, bone, part))
    for sg, n in ((1, 'l'), (-1, 'r')):
        M(box(f'floor-rail-{n}', BD, FE, 'detail', 0.5, (11.3, 0.1, 0.1), (6.0, 0.75, sg * 0.6)), BD)
        M(cap(f'roof-rail-{n}', BD, FE, 'detail', 0.5, (0.5, 2.88, sg * 1.0), (11.5, 2.88, sg * 1.0), 0.04), BD)
        M(cap(f'waist-rail-{n}', BD, FE, 'detail', 0.5, (0.4, 1.12, sg * 1.2), (11.6, 1.12, sg * 1.2), 0.03), BD)
    for x in (1.5, 2.75, 4.15, 5.4, 6.3, 7.5, 8.7, 9.9, 10.3, 11.5):
        for sg, n in ((1, 'l'), (-1, 'r')):
            M(cap(f'post-{x:.2f}-{n}', BD, FE, 'detail', 0.5, (x, 0.95, sg * 1.2), (x, 2.85, sg * 1.2), 0.035), BD)
        M(cap(f'bow-{x:.2f}', BD, FE, 'detail', 0.5, (x, 2.88, -1.05), (x, 2.88, 1.05), 0.035), BD)
    for x in (0.5, 1.6, 4.8, 5.8, 7.5, 10.4, 11.5):
        M(box(f'crossmember-{x:.1f}', BD, FE, 'detail', 0.5, (0.12, 0.1, 2.0), (x, 0.75, 0)), BD)
    M(box('bumper-beam-front', BD, FE, 'detail', 0.5, (0.08, 0.2, 2.3), (11.95, 0.45, 0)), BD)
    M(box('bumper-beam-rear', BD, FE, 'detail', 0.5, (0.08, 0.2, 2.3), (0.05, 0.45, 0)), BD)
    S.append(st(ell('com-body', BD, FE, 'detail', 0.5, (0.9, 0.3, 0.6), (5.8, 1.3, 0)), BD, 'mass'))
    M(box('battery-rail-l', 'battery', FE, 'detail', 0.5, (6.0, 0.05, 0.06), (3.15, 0.08, 0.9)), 'battery')
    M(box('battery-rail-r', 'battery', FE, 'detail', 0.5, (6.0, 0.05, 0.06), (3.15, 0.08, -0.9)), 'battery')
    S.append(st(ell('battery-mass', 'battery', FE, 'detail', 0.5, (1.0, 0.04, 0.5), (3.1, 0.08, 0)), 'battery', 'mass'))
    M(box('hvac-frame', 'hvac', FE, 'detail', 0.5, (2.2, 0.05, 0.8), (1.25, 0.08, 0)), 'hvac')
    S.append(st(ell('hvac-mass', 'hvac', FE, 'detail', 0.5, (0.5, 0.04, 0.4), (1.25, 0.08, 0)), 'hvac', 'mass'))
    for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
        M(box(f'leaf-{n}-stiffener', f'door-{n}', FE, 'detail', 0.5, (0.4, 1.9, 0.04), (0.275, 1.06, -1.24)), f'door-{n}')
        M(ell(f'leaf-{n}-roller', f'door-{n}', FE, 'detail', 0.5, (0.04, 0.04, 0.04), (0.275, 2.08, -1.24)), f'door-{n}', 'joint')
    for ax in ('axle-f', 'axle-r'):
        M(cap(ax + '-beam-member', ax, FE, 'detail', 0.35, (0, 0, -1.05), (0, 0, 1.05), 0.07), ax)
        S.append(st(ell(ax + '-mass', ax, FE, 'detail', 0.35, (0.12, 0.07, 0.07), (0, 0, 0)), ax, 'mass'))
        for z, n in ((0.8, 'l'), (-0.8, 'r')):
            M(ell(f'{ax}-air-spring-{n}', ax, FE, 'detail', 0.5, (0.13, 0.14, 0.08), (0, 0.6, z)), ax, 'joint')
    for bone in ('wheel-f', 'wheel-r'):
        for sg, n in ((1, 'l'), (-1, 'r')):
            M(rev(f'{bone}-rim-{n}', bone, 'steel-wheel', 'detail', 0.55, [(-0.14, 0.286), (0.14, 0.286)], (0, 0, sg * 1.05)), bone)
            M(rev(f'{bone}-hub-{n}', bone, 'steel-wheel', 'detail', 0.55, [(0, 0.11), (0.17, 0.11)], (0, 0, sg * 1.05)), bone, 'joint')
        S.append(st(ell(bone + '-mass', bone, 'rubber', 'detail', 0.1, (0.18, 0.18, 0.12), (0, 0, 1.05)), bone, 'mass'))
    return S

def thermal_coarse(v, sc):
    T = []
    ids = {s['id'] for s in v['body']}
    f = {100: 0.015, 200: 0.03, 500: 0.075}[sc]
    T.append(ref('th-shell', 'body-shell', 'body-panels', None, ['-v']))
    for i in sorted(ids):
        if i.startswith('wheelhouse-') or i in ('bumper-front', 'bumper-rear'):
            T.append(ref('th-' + i, i, 'body-panels'))
    T.append(ref('th-roof', 'roof-panel', 'roof'))
    for i in sorted(ids):
        if i.startswith('glass-') or i == 'windscreen':
            T.append(ref('th-' + i, i, 'glazing'))
        if i.startswith('leaf-'):
            T.append(ref('th-' + i, i, 'doors'))
    T.append(ref('th-battery-pack', 'battery-pack', 'battery-pack'))
    T.append(ref('th-hvac-unit', 'hvac-unit', 'hvac-unit'))
    T.append(th(ell('th-hvac-vent', 'hvac', 'hvac-housing', 'detail', 0.5, (max(0.012, f * 0.6), 0.07, 0.35), (0.0, 0.08, 0)), 'hvac-unit', None, ['hvac-vent']))
    T.append(ref('th-e-axle-casting', 'e-axle-casting', 'e-axle'))
    T.append(ref('th-axle-beam-r', 'axle-beam-r', 'e-axle'))
    for i in sorted(ids):
        if i.endswith('-tyre') or i.endswith('-wheel'):
            T.append(ref('th-tread-' + i[:2], i, 'tyres'))
    th_ = {100: 0.025, 200: 0.03, 500: 0.075}[sc]
    for sg, n in ((1, 'l'), (-1, 'r')):
        for bone, p in (('wheel-f', 'f'), ('wheel-r', 'r')):
            T.append(th(rev(f'th-drum-{p}{n}', bone, FE, 'detail', 0.4, [(-th_ / 2, 0.0 if sc >= 100 else 0.1), (-th_ / 2, 0.17 if sc == 100 else 0.235), (th_ / 2, 0.17 if sc == 100 else 0.235), (th_ / 2, 0.0 if sc >= 100 else 0.1)], (0, 0, sg * 0.88)), 'brakes'))
    return T

def structure_coarse(v, sc):
    S = []
    M = lambda s, bone, part='member': S.append(st(s, bone, part))
    k = {100: 1.2, 200: 1.6, 500: 2.4}[sc]
    for sg, n in ((1, 'l'), (-1, 'r')):
        M(box(f'floor-rail-{n}', BD, FE, 'detail', 0.5, (11.3, 0.1 * k, 0.1 * k), (6.0, 0.75, sg * 0.6)), BD)
        M(cap(f'roof-rail-{n}', BD, FE, 'detail', 0.5, (0.5, 2.88, sg * 1.0), (11.5, 2.88, sg * 1.0), 0.04 * k), BD)
    posts = {100: (2.75, 5.4, 7.5, 9.9, 11.5), 200: (2.75, 7.5, 11.5), 500: ()}[sc]
    for x in posts:
        for sg, n in ((1, 'l'), (-1, 'r')):
            M(cap(f'post-{x:.2f}-{n}', BD, FE, 'detail', 0.5, (x, 0.95, sg * 1.12), (x, 2.8, sg * 1.12), 0.035 * k), BD)
    for x in {100: (1.5, 6.3, 10.3), 200: (5.4,), 500: ()}[sc]:
        M(cap(f'bow-{x:.2f}', BD, FE, 'detail', 0.5, (x, 2.88, -1.05), (x, 2.88, 1.05), 0.035 * k), BD)
    for x in {100: (0.5, 4.8, 7.5, 10.4, 11.5), 200: (0.5, 5.8, 11.5), 500: (0.5, 11.5)}[sc]:
        M(box(f'crossmember-{x:.1f}', BD, FE, 'detail', 0.5, (0.12 * k, 0.1 * k, 2.0), (x, 0.75, 0)), BD)
    if sc <= 200:
        M(box('bumper-beam-front', BD, FE, 'detail', 0.5, (0.1, 0.22, 2.3), (11.86, 0.45, 0)), BD)
        M(box('bumper-beam-rear', BD, FE, 'detail', 0.5, (0.1, 0.22, 2.3), (0.14, 0.45, 0)), BD)
    S.append(st(ell('com-body', BD, FE, 'detail', 0.5, (0.9, 0.3, 0.6), (5.8, 1.3, 0)), BD, 'mass'))
    M(box('battery-frame', 'battery', FE, 'detail', 0.5, (6.0, 0.06 * k, 1.9), (3.15, 0.08, 0)), 'battery')
    M(box('hvac-frame', 'hvac', FE, 'detail', 0.5, (2.2, 0.06 * k, 0.8), (1.25, 0.08, 0)), 'hvac')
    if sc == 100:
        S.append(st(ell('battery-mass', 'battery', FE, 'detail', 0.5, (1.0, 0.04, 0.5), (3.1, 0.08, 0)), 'battery', 'mass'))
    for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
        M(box(f'leaf-{n}-stiffener', f'door-{n}', FE, 'detail', 0.5, (0.4, 1.9, 0.04 * k), (0.275, 1.06, -1.24)), f'door-{n}')
    for ax in ('axle-f', 'axle-r'):
        M(cap(ax + '-beam-member', ax, FE, 'detail', 0.35, (0, 0, -1.05), (0, 0, 1.05), 0.07 if sc < 500 else 0.04), ax)
        if sc <= 200:
            S.append(st(ell(ax + '-mass', ax, FE, 'detail', 0.35, (0.12, 0.07, 0.07), (0, 0, 0)), ax, 'mass'))
        if sc == 100:
            M(ell(ax + '-air-spring', ax, FE, 'detail', 0.5, (0.13, 0.08, 0.85), (0, 0.0, 0)), ax, 'joint')
    for bone in ('wheel-f', 'wheel-r'):
        for sg, n in ((1, 'l'), (-1, 'r')):
            hw = 0.14 if sc < 500 else 0.15
            M(rev(f'{bone}-rim-{n}', bone, 'steel-wheel', 'detail', 0.55, [(-hw, 0.286), (hw, 0.286)], (0, 0, sg * 1.05)), bone)
        if sc <= 200:
            S.append(st(ell(bone + '-mass', bone, 'rubber', 'detail', 0.1, (0.18, 0.18, 0.12), (0, 0, 1.05)), bone, 'mass'))
    return S

DEC = {
    50: [("Body: the shell is a box-like superelliptic loft along the bus with the wheel arches cut through the full width and vertical arch walls, thin inner wheelhouse plates behind each arch, a roof panel, roof battery and heat-pump lofts, a flat windscreen shell that stands behind its own backing wedge, 24 side and upper window panes as 60 mm glazing units on both sides (the right side leaves the door openings free), four plug-door leaves on their own bones, bumpers, mirrors, axle beams, air springs and revolved tyres, rims, hubs, studs and drums.",
          "Körper: die Hülle ist ein kastenförmiger superelliptischer Loft entlang des Busses mit über die ganze Breite durchgeschnittenen Radausschnitten und senkrechten Ausschnittwänden, dünnen inneren Radhausplatten hinter jedem Ausschnitt, einer Dachplatte, Dach-Batterie und Wärmepumpe als Lofts, einer ebenen Windschutzscheibenschale mit eigenem Stützkeil dahinter, 24 Seiten- und Oberfenstern als 60 mm dicken Glaseinheiten auf beiden Seiten (rechts bleiben die Türöffnungen frei), vier Pendeltürflügeln an eigenen Knochen, Stossfängern, Spiegeln, Achsbalken, Luftfedern und Rotationsreifen, Felgen, Naben, Bolzen und Trommeln."),
         ("Thermal: body-panels, roof, doors are nested wall shells (insulation as the outer reference, skin as the shell inset by 40 mm), glazing is three nested shells per window (inner pane, gap, outer pane), the underside of the shell is excluded with sectionFaces (floor, not a wall); battery, heat pump, tyres and the e-axle reference body solids; brake drums are real discs behind the tyres; the heat-pump outlet is a flat louvre solid carrying the emitter tag.",
          "Thermik: Karosserieblech, Dach und Türen sind verschachtelte Wandschalen (Dämmung als äussere Referenz, Haut als um 40 mm eingerückte Schale), die Verglasung besteht aus drei verschachtelten Schalen je Fenster (innere Scheibe, Zwischenraum, äussere Scheibe), die Unterseite der Hülle ist mit sectionFaces ausgenommen (Boden, keine Wand); Batterie, Wärmepumpe, Reifen und E-Achse verweisen auf Körper; Bremstrommeln sind eigene Scheiben hinter den Reifen; der Wärmepumpen-Auslass ist ein flacher Lamellenkörper mit dem Emitter-Tag."),
         ("Structure: two floor rails, seven cross members, ten posts per side with ten roof bows, roof and waist rails, bumper beams, battery rails and heat-pump frame on their bones, door stiffeners with rollers, axle beams with masses and air-spring joints, wheels with rim, hub joint and mass; body, battery and heat-pump masses.",
          "Tragwerk: zwei Bodenlängsträger, sieben Querträger, zehn Pfosten je Seite mit zehn Dachspriegeln, Dach- und Gürtelholme, Stossfängerträger, Batterieschienen und Wärmepumpenrahmen an ihren Knochen, Türversteifungen mit Rollen, Achsbalken mit Massen und Luftfedergelenken, Räder mit Felge, Nabengelenk und Masse; Aufbau-, Batterie- und Wärmepumpenmasse.")],
    100: [("Remodelled against the 1:100 drawing: shell, roof panel, battery, heat pump and windscreen are refitted to this scale's drawing (no inner wheelhouse plates: the 1:100 drawing leaves the arches open), glazing is six merged side bands, leaves, e-axle casting, axle beams, wheels as tyre, rim and hub, mirrors.",
           "Neu modelliert gegen die 1:100-Zeichnung: Hülle, Dachplatte, Batterie, Wärmepumpe und Windschutzscheibe sind an die Zeichnung dieses Massstabs angepasst (keine inneren Radhausplatten: die 1:100-Zeichnung lässt die Ausschnitte offen), die Verglasung besteht aus sechs zusammengefassten Seitenbändern, dazu Türflügel, E-Achsgehäuse, Achsbalken, Räder aus Reifen, Felge und Nabe, Spiegel."),
          ("Thermal: one surface solid per region (layers merged); the shell without underside, roof, glazing bands, leaves, battery, heat pump, tyres and e-axle are references; drum discs and the heat-pump louvre are real solids.",
           "Thermik: ein Oberflächenkörper je Region (Schichten zusammengefasst); Hülle ohne Unterseite, Dach, Glasbänder, Türflügel, Batterie, Wärmepumpe, Reifen und E-Achse sind Referenzen; Trommelscheiben und die Wärmepumpenlamelle sind eigene Körper."),
          ("Structure: floor rails, five cross members, five posts per side, three roof bows, roof rails, bumper beams, battery and heat-pump frames, door stiffeners, axle beams with one wide joint, wheel rims; merged bows and posts at heavier sections.",
           "Tragwerk: Bodenlängsträger, fünf Querträger, fünf Pfosten je Seite, drei Dachspriegel, Dachholme, Stossfängerträger, Batterie- und Wärmepumpenrahmen, Türversteifungen, Achsbalken mit einem breiten Gelenk, Radfelgen; zusammengefasste Spriegel und Pfosten in kräftigeren Querschnitten.")],
    200: [("Remodelled against the 1:200 drawing: kit refits of shell, roof panel, battery, heat pump and windscreen, one glass band per side, leaves, e-axle casting, axle beams, tyre and rim wheels, front bumper.",
           "Neu modelliert gegen die 1:200-Zeichnung: angepasste Hülle, Dachplatte, Batterie, Wärmepumpe und Windschutzscheibe, ein Glasband je Seite, Türflügel, E-Achsgehäuse, Achsbalken, Räder aus Reifen und Felge, vorderer Stossfänger."),
          ("Thermal: one solid per region; only the total of the exposed areas is compared at this scale.",
           "Thermik: ein Körper je Region; in diesem Massstab wird nur die Summe der freien Flächen verglichen."),
          ("Structure: floor rails, three cross members, three posts per side, one roof bow, roof rails, bumper beams, frames, door stiffeners, axle beams and rims with masses at 1.6 times the 1:50 sections.",
           "Tragwerk: Bodenlängsträger, drei Querträger, drei Pfosten je Seite, ein Dachspriegel, Dachholme, Stossfängerträger, Rahmen, Türversteifungen, Achsbalken und Felgen mit Massen im 1,6-fachen der 1:50-Querschnitte.")],
    500: [("Minimal model: refitted shell with trapezoid roof battery and heat-pump lofts (flat roof panel), one glass band per side at 75 mm, four door slabs, e-axle and axle beams, four solid wheel cylinders.",
           "Minimales Modell: angepasste Hülle mit trapezförmigen Dachlofts für Batterie und Wärmepumpe (ebene Dachplatte), ein Glasband je Seite mit 75 mm, vier Türplatten, E-Achse und Achsbalken, vier Vollzylinder als Räder."),
          ("Thermal: one solid per region at the smallest feature (75 mm discs for the drums, the heat-pump louvre); equipment on the roof buries the roof panel, so only the total of the exposed areas is compared.",
           "Thermik: ein Körper je Region in der kleinsten Detailgrösse (75-mm-Scheiben für die Trommeln, die Wärmepumpenlamelle); die Dachaufbauten verdecken die Dachplatte, daher wird nur die Summe der freien Flächen verglichen."),
          ("Structure: floor rails, two cross members, roof rails, frames, door stiffeners, axle beams and wheel rims at 2.4 times the 1:50 sections; one body mass.",
           "Tragwerk: Bodenlängsträger, zwei Querträger, Dachholme, Rahmen, Türversteifungen, Achsbalken und Felgen im 2,4-fachen der 1:50-Querschnitte; eine Aufbaumasse.")],
}

if __name__ == '__main__':
    scales = [int(x) for x in sys.argv[1:]] or [50]
    d = load(A)
    for sc in scales:
        v = d['volumes'][str(sc)]
        v['thermal'] = thermal50(v) if sc == 50 else thermal_coarse(v, sc)
        v['structure'] = structure50(v) if sc == 50 else structure_coarse(v, sc)
        v['decisions'] = [{"en": x, "de": y} for x, y in DEC[sc]]
    save(A, d)
    for sc in scales:
        fails(A, sc)
