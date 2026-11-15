"""Author 32: delivery-van thermal, structure and decisions for all four scales."""
import sys; sys.path.insert(0, '.')
from author_32_kit import *

A = 'vehicles/delivery-van'
CH = 'chassis'
EXH = 'exhaust-steel'
FE = 'cast-iron'

def brakes(thick, rin, rout, caliper):
    out = []
    for sg, n in ((1, 'l'), (-1, 'r')):
        for bone, p in (('wheel-f', 'f'), ('wheel-r', 'r')):
            z = sg * 0.69
            out.append(th(rev(f'th-brake-{p}{n}', bone, FE, 'detail', 0.35, [(-thick / 2, rin), (-thick / 2, rout), (thick / 2, rout), (thick / 2, rin)], (0, 0, z)), 'brakes'))
            if caliper:
                out.append(th(box(f'th-caliper-{p}{n}', bone, FE, 'detail', 0.3, (0.09, 0.12, 0.06), (0.0, 0.16, z), corner=0.01), 'brakes'))
    return out

def thermal50(v):
    T = []
    B = lambda i: body_of(v, i)
    for i, n, sf, rg in (('body-shell', 'shell', ['-v'], 'body-panels'), ('bumper', 'bumper', None, 'body-panels'), ('roof-cap', 'roof', None, 'roof'), ('hood-panel', 'hood', None, 'bonnet'),
                         ('slider-panel', 'slider', None, 'slider-door'), ('slider-track-low', 'track-low', None, 'slider-door'), ('slider-track-high', 'track-high', None, 'slider-door'), ('rear-leaf-l', 'rear-l', None, 'rear-doors'), ('rear-leaf-r', 'rear-r', None, 'rear-doors')):
        T.append(ref(f'th-{n}-paint', i, rg, 'paint', sf))
        T.append(shell(B(i), 0.00008, f'th-{n}-wall', rg, 'wall', 0.004))
    for i in ('windscreen', 'door-window-l', 'door-window-r'):
        T.append(ref('th-' + i, i, 'cab-glazing'))
    T.append(th(cap('th-exhaust-line', CH, EXH, 'detail', 0.35, (-2.3, 0.42, -0.31), (-0.9, 0.42, -0.31), 0.03), 'exhaust-system', 'pipe'))
    T.append(th(cap('th-silencer-can', CH, EXH, 'detail', 0.4, (-2.6, 0.47, -0.31), (-2.12, 0.47, -0.31), 0.1, lateral=1.3), 'exhaust-system', 'pipe'))
    T.append(th(cap('th-tailpipe-nozzle', CH, EXH, 'detail', 0.5, (-2.62, 0.42, -0.31), (-2.665, 0.42, -0.31), 0.03), 'exhaust-system', 'pipe', ['tailpipe']))
    for i in ('fl', 'fr', 'rl', 'rr'):
        T.append(ref(f'th-tread-{i}', f'{i}-tyre', 'tyres', 'tread'))
    T += brakes(0.025, 0.05, 0.16, True)
    T.append(th(box('th-engine-block', CH, 'engine-metal', 'detail', 0.3, (0.4, 0.4, 0.45), (2.15, 0.62, 0), corner=0.03), 'engine-bay'))
    T.append(th(cap('th-gearbox', CH, 'engine-metal', 'detail', 0.25, (1.8, 0.6, 0), (1.95, 0.6, 0), 0.11), 'engine-bay'))
    T.append(th(cap('th-exhaust-manifold', CH, EXH, 'detail', 0.35, (2.3, 0.45, -0.2), (2.3, 0.45, 0.2), 0.035), 'engine-bay'))
    return T

def structure50(v):
    S = []
    M = lambda s, bone, part='member': S.append(st(s, bone, part))
    for sg, n in ((1, 'l'), (-1, 'r')):
        M(box(f'floor-rail-{n}', CH, FE, 'detail', 0.5, (4.6, 0.1, 0.08), (-0.15, 0.92, sg * 0.55)), CH)
        M(box(f'sill-{n}', CH, FE, 'detail', 0.5, (2.2, 0.12, 0.1), (0.05, 0.5, sg * 0.7)), CH)
        M(cap(f'a-pillar-{n}', CH, FE, 'detail', 0.5, (1.7, 1.0, sg * 0.78), (1.28, 1.7, sg * 0.72), 0.03), CH)
        M(cap(f'b-pillar-{n}', CH, FE, 'detail', 0.5, (0.4, 0.97, sg * 0.78), (0.4, 1.8, sg * 0.76), 0.03), CH)
        M(cap(f'roof-rail-{n}', CH, FE, 'detail', 0.5, (-2.5, 1.88, sg * 0.68), (1.2, 1.88, sg * 0.68), 0.04), CH)
        M(cap(f'rear-post-{n}', CH, FE, 'detail', 0.5, (-2.55, 0.97, sg * 0.78), (-2.55, 1.85, sg * 0.74), 0.035), CH)
    for x, nm in ((-2.45, 'rear-crossmember'), (-0.9, 'cross-rear-axle'), (0.2, 'floor-crossmember-mid'), (1.25, 'cross-front-axle'), (2.05, 'front-crossmember')):
        M(box(nm, CH, FE, 'detail', 0.5, (0.1, 0.1, 1.1), (x, 0.92, 0)), CH)
    for x, nm in ((-2.0, 'roof-bow-rear'), (-1.0, 'roof-bow-mid-rear'), (0.0, 'roof-bow-mid'), (1.0, 'roof-bow-cab')):
        M(cap(nm, CH, FE, 'detail', 0.5, (x, 1.88, -0.68), (x, 1.88, 0.68), 0.025), CH)
    M(box('bumper-beam-front', CH, FE, 'detail', 0.5, (0.1, 0.14, 1.4), (2.58, 0.66, 0)), CH)
    M(box('bumper-beam-rear', CH, FE, 'detail', 0.5, (0.1, 0.14, 1.4), (-2.62, 0.6, 0)), CH)
    S.append(st(ell('com-chassis', CH, FE, 'detail', 0.5, (0.4, 0.2, 0.4), (0.7, 0.95, 0)), CH, 'mass'))
    S.append(st(ell('engine-mass', CH, FE, 'detail', 0.5, (0.18, 0.2, 0.28), (2.3, 0.7, 0)), CH, 'mass'))
    M(box('slider-beam', 'slider', FE, 'detail', 0.5, (0.9, 0.08, 0.04), (0.5, 0.7, -0.885)), 'slider')
    for u, vv, n in ((0.1, 0.14, 'low-rear'), (0.95, 0.14, 'low-front'), (0.1, 1.44, 'high-rear'), (0.95, 1.44, 'high-front')):
        M(ell(f'slider-roller-{n}', 'slider', FE, 'detail', 0.5, (0.04, 0.04, 0.02), (u, vv, -0.885)), 'slider', 'joint')
    for z, n in (('r', 'rear-door-r'), ('l', 'rear-door-l')):
        M(box(f'{n}-stiffener', n, FE, 'detail', 0.5, (1.2, 0.08, 0.03), (0.675, 0.45, 0)), n)
        M(ell(f'{n}-hinge-low', n, FE, 'detail', 0.5, (0.03, 0.02, 0.015), (0.04, 0, 0)), n, 'joint')
        M(ell(f'{n}-hinge-high', n, FE, 'detail', 0.5, (0.03, 0.02, 0.015), (1.31, 0, 0)), n, 'joint')
    for ax in ('axle-f', 'axle-r'):
        M(cap(ax + '-beam-member', ax, FE, 'detail', 0.35, (0, 0, -0.84), (0, 0, 0.84), 0.05), ax)
        S.append(st(ell(ax + '-mass', ax, FE, 'detail', 0.35, (0.09, 0.045, 0.045), (0, 0, 0)), ax, 'mass'))
        for z, n in ((0.6, 'l'), (-0.6, 'r')):
            M(ell(f'{ax}-spring-{n}', ax, FE, 'detail', 0.5, (0.07, 0.12, 0.07), (0, 0.12, z)), ax, 'joint')
    for bone in ('wheel-f', 'wheel-r'):
        for sg, n in ((1, 'l'), (-1, 'r')):
            M(rev(f'{bone}-rim-{n}', bone, 'steel-rim', 'detail', 0.55, [(-0.085, 0.2), (0.085, 0.2)], (0, 0, sg * 0.84)), bone)
            M(rev(f'{bone}-hub-{n}', bone, 'steel-rim', 'detail', 0.7, [(-0.1, 0.055), (0.1, 0.055)], (0, 0, sg * 0.84)), bone, 'joint')
        S.append(st(ell(bone + '-mass', bone, 'rubber', 'detail', 0.1, (0.12, 0.12, 0.12), (0, 0, 0.84)), bone, 'mass'))
    return S

def thermal_coarse(v, sc):
    T = []
    ids = {s['id'] for s in v['body']}
    f = {100: 0.015, 200: 0.03, 500: 0.075}[sc]
    T.append(ref('th-shell', 'body-shell', 'body-panels', None, ['-v']))
    T.append(ref('th-roof', 'roof-cap', 'roof'))
    T.append(ref('th-hood', 'hood-panel', 'bonnet'))
    T.append(ref('th-slider', 'slider-panel', 'slider-door'))
    for i in ('slider-track-low', 'slider-track-high'):
        if i in ids: T.append(ref('th-' + i, i, 'slider-door'))
    T.append(ref('th-rear-l', 'rear-leaf-l', 'rear-doors'))
    T.append(ref('th-rear-r', 'rear-leaf-r', 'rear-doors'))
    if sc <= 200:
        for i in ('windscreen', 'door-window-l', 'door-window-r'):
            T.append(ref('th-' + i, i, 'cab-glazing'))
    else:
        t = f / 2
        RQg = [-0.31623, -0.63246, -0.31623, 0.63246]
        T.append(th(loft('th-windscreen', CH, 'glass', 'glazing', 0.88, [sec((1.74, 1.2, 0), 0.8, t, 8, RQg), sec((1.28, 1.8, 0), 0.78, t, 8, RQg)]), 'cab-glazing'))
        for sg, n in ((1, 'l'), (-1, 'r')):
            T.append(th(box(f'th-door-window-{n}', CH, 'glass', 'glazing', 0.88, (1.1, 0.5, t * 2), (0.95, 1.5, sg * 0.885)), 'cab-glazing'))
    T.append(th(cap('th-exhaust-line', CH, EXH, 'detail', 0.35, (-2.3, 0.42, -0.31), (-0.9, 0.42, -0.31), max(0.03, f * 0.6)), 'exhaust-system'))
    T.append(th(cap('th-silencer-can', CH, EXH, 'detail', 0.4, (-2.6, 0.47, -0.31), (-2.12, 0.47, -0.31), max(0.1, f * 0.8), lateral=1.3), 'exhaust-system'))
    T.append(th(cap('th-tailpipe-nozzle', CH, EXH, 'detail', 0.5, (-2.62, 0.42, -0.31), (-2.665, 0.42, -0.31), max(0.03, f * 0.6)), 'exhaust-system', None, ['tailpipe']))
    for i in ('fl-tyre', 'fr-tyre', 'rl-tyre', 'rr-tyre', 'fl-wheel', 'fr-wheel', 'rl-wheel', 'rr-wheel'):
        if i in ids: T.append(ref('th-tread-' + i[:2], i, 'tyres'))
    T += brakes({100: 0.03, 200: 0.03, 500: 0.075}[sc], 0.055 if sc < 500 else 0.0, 0.16, sc == 100)
    if sc == 100:
        T.append(th(box('th-engine-block', CH, 'engine-metal', 'detail', 0.3, (0.42, 0.42, 0.46), (2.15, 0.62, 0), corner=0.04), 'engine-bay'))
        T.append(th(cap('th-gearbox', CH, 'engine-metal', 'detail', 0.25, (1.76, 0.6, 0), (1.95, 0.6, 0), 0.115), 'engine-bay'))
    else:
        T.append(th(box('th-engine-unit', CH, 'engine-metal', 'detail', 0.3, (0.7 if sc == 200 else 0.7, 0.45, 0.5), (2.05, 0.62, 0), corner=0.05), 'engine-bay'))
    return T

def structure_coarse(v, sc):
    S = []
    M = lambda s, bone, part='member': S.append(st(s, bone, part))
    f = {100: 0.015, 200: 0.03, 500: 0.075}[sc]
    k = {100: 1.2, 200: 1.5, 500: 2.0}[sc]
    for sg, n in ((1, 'l'), (-1, 'r')):
        M(box(f'floor-rail-{n}', CH, FE, 'detail', 0.5, (4.6, 0.1 * k, 0.08 * k), (-0.15, 0.92, sg * 0.55)), CH)
        if sc <= 200:
            M(box(f'sill-{n}', CH, FE, 'detail', 0.5, (1.9 if sc == 200 else 2.1, 0.12 * k, 0.1 * k), (0.05, 0.5, sg * 0.7)), CH)
        M(cap(f'a-pillar-{n}', CH, FE, 'detail', 0.5, (1.7, 1.0, sg * 0.78), (1.28, 1.7, sg * 0.72), 0.03 * k if sc < 500 else 0.04), CH)
        if sc == 100:
            M(cap(f'b-pillar-{n}', CH, FE, 'detail', 0.5, (0.4, 0.97, sg * 0.78), (0.4, 1.8, sg * 0.76), 0.035), CH)
            M(cap(f'rear-post-{n}', CH, FE, 'detail', 0.5, (-2.55, 0.97, sg * 0.78), (-2.55, 1.85, sg * 0.74), 0.04), CH)
        M(cap(f'roof-rail-{n}', CH, FE, 'detail', 0.5, (-2.5, 1.88, sg * 0.68), (1.2, 1.88, sg * 0.68), 0.04 * k if sc < 500 else 0.04), CH)
    cross = {100: ((-2.45, 'rear-crossmember'), (-0.9, 'cross-rear-axle'), (1.25, 'cross-front-axle'), (2.05, 'front-crossmember')), 200: ((-2.45, 'rear-crossmember'), (0.2, 'floor-crossmember-mid'), (2.05, 'front-crossmember')), 500: ((-2.45, 'rear-crossmember'), (2.05, 'front-crossmember'))}[sc]
    for x, nm in cross:
        M(box(nm, CH, FE, 'detail', 0.5, (0.1 * k, 0.1 * k, 1.1), (x, 0.92, 0)), CH)
    if sc == 100:
        for x, nm in ((-1.5, 'roof-bow-rear'), (0.0, 'roof-bow-mid')):
            M(cap(nm, CH, FE, 'detail', 0.5, (x, 1.88, -0.68), (x, 1.88, 0.68), 0.03), CH)
    if sc <= 200:
        M(box('bumper-beam-front', CH, FE, 'detail', 0.5, (0.12, 0.16, 1.4), (2.58, 0.66, 0)), CH)
        M(box('bumper-beam-rear', CH, FE, 'detail', 0.5, (0.12, 0.16, 1.4), (-2.62, 0.6, 0)), CH)
    S.append(st(ell('com-chassis', CH, FE, 'detail', 0.5, (0.4, 0.2, 0.4), (0.7, 0.95, 0)), CH, 'mass'))
    if sc == 100:
        S.append(st(ell('engine-mass', CH, FE, 'detail', 0.5, (0.18, 0.2, 0.28), (2.3, 0.7, 0)), CH, 'mass'))
    sb = {100: 0.09, 200: 0.12, 500: 0.2}[sc]
    M(box('slider-beam', 'slider', FE, 'detail', 0.5, (0.9, sb if sc < 500 else 0.2, 0.04 if sc == 100 else 0.04 if sc == 200 else 0.075), (0.5, 0.7, -0.885)), 'slider')
    if sc == 100:
        for u, vv, n in ((0.1, 0.14, 'low'), (0.95, 1.44, 'high')):
            M(ell(f'slider-roller-{n}', 'slider', FE, 'detail', 0.5, (0.04, 0.012, 0.02), (u, vv, -0.885)), 'slider', 'joint')
    for n in ('rear-door-r', 'rear-door-l'):
        M(box(f'{n}-stiffener', n, FE, 'detail', 0.5, (1.2, 0.09 if sc == 100 else 0.12 if sc == 200 else 0.2, 0.03 if sc == 100 else 0.035 if sc == 200 else 0.075), (0.675, 0.45, 0)), n)
        if sc == 100:
            M(ell(f'{n}-hinge', n, FE, 'detail', 0.5, (0.6, 0.015, 0.015), (0.675, 0.02, 0)), n, 'joint')
    for ax in ('axle-f', 'axle-r'):
        M(cap(ax + '-beam-member', ax, FE, 'detail', 0.35, (0, 0, -0.84), (0, 0, 0.84), 0.05 if sc < 500 else 0.04), ax)
        if sc <= 200:
            S.append(st(ell(ax + '-mass', ax, FE, 'detail', 0.35, (0.09, 0.045, 0.045), (0, 0, 0)), ax, 'mass'))
        if sc == 100:
            M(ell(ax + '-spring', ax, FE, 'detail', 0.5, (0.07, 0.12, 0.6), (0, 0.12, 0)), ax, 'joint')
    for bone in ('wheel-f', 'wheel-r'):
        for sg, n in ((1, 'l'), (-1, 'r')):
            hw = 0.0875 if sc == 500 else 0.085
            M(rev(f'{bone}-rim-{n}', bone, 'steel-rim', 'detail', 0.55, [(-hw, 0.2), (hw, 0.2)], (0, 0, sg * 0.84)), bone)
        if sc <= 200:
            S.append(st(ell(bone + '-mass', bone, 'rubber', 'detail', 0.1, (0.12, 0.12, 0.09), (0, 0, 0.84)), bone, 'mass'))
    return S

DEC = {
    50: [("Body: the shell is a loft along the van whose superelliptic sections carry the wheel arches (open cut-outs through the whole width), the cab step and the sloped windscreen line; a 28 mm hood panel stands 1.5 mm proud of the shell, the roof cap, windscreen shell and door window panes are glazing and panel solids on top, the sliding door panel, its two guide tracks and the two rear door leaves sit on their own bones, wheels are revolved tyres, rims, hubs, wheel nuts and brake discs, axles are capsule beams; the exhaust line and silencer are not body (they run inside the shell) and only the tail pipe is drawn.",
          "Körper: die Hülle ist ein Loft entlang des Transporters, dessen superelliptische Querschnitte die Radausschnitte (offene Ausschnitte über die ganze Breite), die Kabinenstufe und die geneigte Windschutzscheibenlinie tragen; eine 28 mm dicke Haubenplatte steht 1,5 mm über der Hülle, Dachkappe, Windschutzscheibenschale und Türfenster sind Glas- und Blechkörper darauf, das Schiebetürblatt, seine zwei Führungsschienen und die zwei Hecktürflügel sitzen an ihren eigenen Knochen, Räder sind Rotationsreifen, Felgen, Naben, Radmuttern und Bremsscheiben, Achsen sind Kapselbalken; Abgasleitung und Schalldämpfer sind kein Körper (sie liegen innerhalb der Hülle), gezeichnet wird nur das Endrohr."),
         ("Thermal: body-panels, roof, bonnet, slider door and rear doors are nested layer shells (paint as the outer reference to the body solid, steel sheet as the inset shell); the underside of the shell is excluded with sectionFaces because it is the floor pan, not a painted panel; glazing and tyres reference body solids; the hidden engine bay (block, gearbox, manifold) and the hidden exhaust (line, silencer, outlet with the emitter tag) are real solids inside the body; brake discs and calipers sit inboard of the tyres.",
          "Thermik: Karosserieblech, Dach, Haube, Schiebetür und Hecktüren sind verschachtelte Schichtschalen (Lack als äussere Referenz auf den Körper, Stahlblech als eingerückte Schale); die Unterseite der Hülle ist mit sectionFaces ausgenommen, weil sie das Bodenblech und kein lackiertes Blech ist; Verglasung und Reifen verweisen auf Körper; der verdeckte Motorraum (Block, Getriebe, Krümmer) und die verdeckte Abgasanlage (Leitung, Schalldämpfer, Auslass mit Emitter-Markierung) sind eigene Körper im Inneren; Bremsscheiben und Sättel sitzen innen von den Reifen."),
         ("Structure: two floor rails above the wheel arches, two sills between the arches, five cross members, A, B and rear posts, roof rails and four roof bows, bumper beams, a slider beam with four roller joints, door stiffeners with hinge joints, axle beams with spring joints and wheels with rim, hub joint and mass, plus body and engine masses; all inside the silhouette.",
          "Tragwerk: zwei Bodenlängsträger oberhalb der Radausschnitte, zwei Schweller zwischen den Ausschnitten, fünf Querträger, A-, B- und Heckpfosten, Dachholme und vier Dachspriegel, Stossfängerträger, ein Schiebetürträger mit vier Rollengelenken, Türversteifungen mit Scharniergelenken, Achsbalken mit Federgelenken und Räder mit Felge, Nabengelenk und Masse, dazu Aufbau- und Motormasse; alles innerhalb der Silhouette.")],
    100: [("Remodelled against the 1:100 drawing: the shell, roof cap, windscreen and door windows are refitted to this scale's drawing (fewer stations, 15 mm tolerance); hood panel of three stations; wheels are tyre, rim and hub without nuts and discs; mirrors, grille, tail pipe, slider tracks, sliding door and rear leaves stay as single solids; bumper, lamps, handle and hinges are merged into the neighbouring masses.",
           "Neu modelliert gegen die 1:100-Zeichnung: Hülle, Dachkappe, Windschutzscheibe und Türfenster sind an die Zeichnung dieses Massstabs angepasst (weniger Stationen, 15 mm Toleranz); Haubenplatte aus drei Stationen; Räder aus Reifen, Felge und Nabe ohne Muttern und Scheiben; Spiegel, Grill, Endrohr, Schiebetürschienen, Schiebetür und Hecktürflügel bleiben einzelne Körper; Stossfänger, Leuchten, Griff und Scharniere sind in die Nachbarmassen aufgegangen."),
          ("Thermal: one surface solid per region (layers merged); shell without underside, roof cap, hood panel, slider panel with tracks, rear leaves, windscreen and door windows and the tyres are references; hidden engine bay (block and gearbox) and hidden exhaust are real solids inside the shell; discs are 30 mm.",
           "Thermik: ein Oberflächenkörper je Region (Schichten zusammengefasst); Hülle ohne Unterseite, Dachkappe, Haubenplatte, Schiebetürblatt mit Schienen, Hecktürflügel, Windschutz- und Türfenster sowie die Reifen sind Referenzen; verdeckter Motorraum (Block und Getriebe) und verdeckte Abgasanlage sind eigene Körper in der Hülle; Scheiben sind 30 mm dick."),
          ("Structure: floor rails, sills, four cross members, A, B and rear posts, roof rails and two roof bows, bumper beams, slider beam with two rollers, door stiffeners with one hinge joint each, axle beams with one wide spring joint, wheel rims with merged hubs; heavier sections.",
           "Tragwerk: Bodenlängsträger, Schweller, vier Querträger, A-, B- und Heckpfosten, Dachholme und zwei Dachspriegel, Stossfängerträger, Schiebetürträger mit zwei Rollen, Türversteifungen mit je einem Scharniergelenk, Achsbalken mit einem breiten Federgelenk, Felgen mit eingegangenen Naben; kräftigere Querschnitte.")],
    200: [("Remodelled against the 1:200 drawing: the kit refit of shell, roof cap, hood and windscreen with 30 mm stations, one pair of mirrors, wheels as tyre and rim, slider panel, upper guide track and rear leaves; the grille, tail pipe and lower track are merged away.",
           "Neu modelliert gegen die 1:200-Zeichnung: angepasste Hülle, Dachkappe, Haube und Windschutzscheibe mit 30-mm-Stationen, ein Spiegelpaar, Räder aus Reifen und Felge, Schiebetürblatt, obere Führungsschiene und Hecktürflügel; Grill, Endrohr und untere Schiene sind aufgegangen."),
          ("Thermal: one solid per region; the engine bay is one engine unit and the exhaust one line with silencer inside the shell, brake discs 30 mm; only the total of the exposed areas is compared.",
           "Thermik: ein Körper je Region; der Motorraum ist eine Motoreinheit und die Abgasanlage eine Leitung mit Schalldämpfer in der Hülle, Bremsscheiben 30 mm; verglichen wird nur die Summe der freien Flächen."),
          ("Structure: floor rails, shortened sills, three cross members, A pillars, roof rails, bumper beams, slider beam, door stiffeners, axle beams and rims with masses at 1.5 times the 1:50 sections; B pillars, bows and joints are merged into the frames.",
           "Tragwerk: Bodenlängsträger, verkürzte Schweller, drei Querträger, A-Säulen, Dachholme, Stossfängerträger, Schiebetürträger, Türversteifungen, Achsbalken und Felgen mit Massen im 1,5-fachen der 1:50-Querschnitte; B-Säulen, Spriegel und Gelenke sind in die Rahmen aufgegangen.")],
    500: [("Minimal model: shell, roof cap, hood and windscreen refit to the 1:500 drawing, sliding door and rear leaves as 80 mm slabs, a mirror stub on each side, wheels as four solid cylinders, axle beams; glass is part of the shell.",
           "Minimales Modell: angepasste Hülle, Dachkappe, Haube und Windschutzscheibe nach der 1:500-Zeichnung, Schiebetür und Hecktürflügel als 80-mm-Platten, je ein Spiegelstummel, Räder als vier Vollzylinder, Achsbalken; das Glas gehört zur Hülle."),
          ("Thermal: glazing is modelled as 75 mm shells proud of the shell (windscreen and two side windows), exhaust line, silencer and engine unit are compact hidden solids inside the shell, discs are 75 mm pucks; only the total exposed area counts.",
           "Thermik: die Verglasung ist als 75 mm dicke Schalen über der Hülle modelliert (Windschutzscheibe und zwei Seitenfenster), Abgasleitung, Schalldämpfer und Motoreinheit sind kompakte verdeckte Körper in der Hülle, Scheiben sind 75 mm dicke Pucks; nur die Summe der freien Fläche zählt."),
          ("Structure: floor rails, two cross members, A struts, roof rails, slider beam, door stiffeners, axle beams and wheel rims at 2 times the 1:50 sections; masses merged into the body mass.",
           "Tragwerk: Bodenlängsträger, zwei Querträger, A-Streben, Dachholme, Schiebetürträger, Türversteifungen, Achsbalken und Felgen im Doppelten der 1:50-Querschnitte; die Massen sind in der Aufbaumasse zusammengefasst.")],
}

if __name__ == '__main__':
    scales = [int(x) for x in sys.argv[1:]] or [50, 100, 200, 500]
    d = load(A)
    for sc in scales:
        v = d['volumes'][str(sc)]
        v['thermal'] = thermal50(v) if sc == 50 else thermal_coarse(v, sc)
        v['structure'] = structure50(v) if sc == 50 else structure_coarse(v, sc)
        v['decisions'] = [{"en": x, "de": y} for x, y in DEC[sc]]
    save(A, d)
    for sc in scales:
        fails(A, sc)
