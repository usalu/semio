"""🌡️ Round 3 city-bus: the 1:50 thermal list (layer plates of the real wall, roof, glazing and door construction, operating parts) and the 1:50 structure list (the tube frame, axles, wheels, masses)."""
import copy, math
from r3_city_bus_kit import *
import r3_city_bus_body50 as B

GAP, PANE = 0.012, 0.004
INS = TW - SKIN


def of(id, body, region, layer=None, faces=None, tags=None):
    s = {"id": id, "of": body, "region": region}
    if layer:
        s["layer"] = layer
    if faces:
        s["sectionFaces"] = faces
    if tags:
        s["tags"] = tags
    return s


def th(solid, id, region, layer, faces=None, tone=None, **change):
    s = copy.deepcopy(solid)
    s.update(change)
    s["id"], s["region"], s["layer"] = id, region, layer
    if faces:
        s["sectionFaces"] = faces
    if tone is not None:
        s["tone"] = tone
    return s


def shrunk(solid, t, id, region, layer):
    s = copy.deepcopy(solid)
    s["id"], s["region"], s["layer"] = id, region, layer
    for q in s["sections"]:
        q["rx"], q["ry"] = rr(q["rx"] - t), rr(q["ry"] - t)
    s["tone"] = 0.5
    return s


def walls(by):
    t = []
    for side, tag in ((1, 'l'), (-1, 'r')):
        inward = '-w' if side > 0 else '+w'
        lo = by[f'side-lower-{tag}']
        a, b = (HW - SKIN, HW) if side > 0 else (-HW, -HW + SKIN)
        c, d = (HW - TW, HW - SKIN) if side > 0 else (-HW + SKIN, -HW + TW)
        t.append(th(lo, f'th-side-lower-{tag}-skin', 'body-panels', 'skin', [inward], **{"from": rr(a), "to": rr(b)}))
        t.append(th(lo, f'th-side-lower-{tag}-insulation', 'body-panels', 'insulation', None, 0.5, **{"from": rr(c), "to": rr(d)}))
        up = by[f'side-upper-{tag}']
        a, b = (-SKIN, 0) if side > 0 else (0, SKIN)
        c, d = (-TW, -SKIN) if side > 0 else (SKIN, TW)
        t.append(th(up, f'th-side-upper-{tag}-skin', 'body-panels', 'skin', [inward], **{"from": rr(a), "to": rr(b)}))
        t.append(th(up, f'th-side-upper-{tag}-insulation', 'body-panels', 'insulation', None, 0.5, **{"from": rr(c), "to": rr(d)}))
        t.append(of(f'th-a-pillar-{tag}-skin', f'a-pillar-{tag}', 'body-panels', 'skin'))
        t.append(shrunk(by[f'a-pillar-{tag}'], SKIN, f'th-a-pillar-{tag}-insulation', 'body-panels', 'insulation'))
    for id, d, n_, xc, sign, hidden in (('nose', NOSE_D, NOSE_N, X1, 1, '-u'), ('tail', TAIL_D, TAIL_N, X0, -1, '+u'), ('tail-upper', TAIL_D, TAIL_N, X0, -1, '+u')):
        t.append(th(by[id], f'th-{id}-skin', 'body-panels', 'skin', [hidden], profile=[[rr(p), rr(q)] for p, q in dband(d, HW, n_, SKIN, xc, sign)]))
        t.append(th(by[id], f'th-{id}-insulation', 'body-panels', 'insulation', None, 0.5, profile=[[rr(p), rr(q)] for p, q in dband(d - SKIN, HW - SKIN, n_, INS, xc, sign)]))
    t.append(of('th-bumper-front', 'bumper-front', 'body-panels', 'skin', ['-u']))
    t.append(of('th-bumper-rear', 'bumper-rear', 'body-panels', 'skin', ['+u']))
    for id in ('floor', 'floor-front', 'floor-rear'):
        t.append(th(by[id], f'th-{id}-skin', 'body-panels', 'skin', ['+v'], **{"from": SKIRT, "to": rr(SKIRT + SKIN)}))
        t.append(th(by[id], f'th-{id}-insulation', 'body-panels', 'insulation', None, 0.5, **{"from": rr(SKIRT + SKIN), "to": rr(SKIRT + TW)}))
    return t


def roof(by):
    t = []
    o, i, m = roof_outline(), roof_outline(SKIN), roof_outline(TW)
    k = 4
    parts = (('r', o[:k + 1], i[:k + 1], ['+w', '-v']), ('c', o[k:len(o) - k], i[k:len(o) - k], ['-v']), ('l', o[len(o) - k - 1:], i[len(o) - k - 1:], ['-w', '-v']))
    for tag, po, pi, faces in parts:
        t.append(th(by['roof'], f'th-roof-skin-{tag}', 'roof', 'skin', faces, profile=[[rr(a), rr(b)] for a, b in po + list(reversed(pi))]))
    t.append(th(by['roof'], 'th-roof-insulation', 'roof', 'insulation', None, 0.5, profile=[[rr(a), rr(b)] for a, b in i + list(reversed(m))]))
    for id, back in (('header', '-u'), ('tail-cap', '+u')):
        t.append(of(f'th-{id}-skin', id, 'body-panels', 'skin', [back, '-v']))
        t.append(shrunk(by[id], SKIN, f'th-{id}-insulation', 'body-panels', 'insulation'))
    return t


def glazing(body):
    t = []
    layers = (('pane-out', 0.0, PANE), ('gap', PANE, GAP), ('pane-in', PANE + GAP, PANE))
    for s in body:
        i = s['id']
        if i.startswith('pane-') or (i.startswith('leaf-') and 'glass' in i):
            side = 1 if i.startswith('pane-l') else -1
            rot = s['transform'].get('rotation')
            a = 0.0 if rot is None else -side * PHI
            c = s['transform']['translation']
            for layer, start, thick in layers:
                off = -side * (start + thick / 2 - 0.01)
                at = [c[0], c[1] - off * math.sin(a), c[2] + off * math.cos(a)]
                x = th(s, f'th-{i}-{layer}', 'glazing', layer, ['-w' if side > 0 else '+w'] if layer == 'pane-out' else None, size=[s['size'][0], s['size'][1], thick])
                x['transform'] = {"translation": [rr(v) for v in at]}
                if rot:
                    x['transform']['rotation'] = rot
                t.append(x)
        if i == 'rear-window':
            c = s['transform']['translation']
            for layer, start, thick in layers:
                x = th(s, f'th-{i}-{layer}', 'glazing', layer, ['+u'] if layer == 'pane-out' else None, size=[thick, s['size'][1], s['size'][2]])
                x['transform'] = {"translation": [rr(c[0] - 0.01 + start + thick / 2), c[1], c[2]]}
                t.append(x)
        if i == 'windscreen':
            for layer, start, thick in layers:
                prof = dband(GLASS_D - start, GLASS_W - start, NOSE_N, thick, 0.0, 1, 0.02, math.pi - 0.02)
                t.append(th(s, f'th-{i}-{layer}', 'glazing', layer, ['-u'] if layer == 'pane-out' else None, profile=[[rr(p), rr(q)] for p, q in prof]))
    return t


def doors(by):
    t = []
    for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
        for part, zr in (('', None), ('-upper', None)):
            s = by[f'leaf-{n}{part}']
            a, b = s['from'], s['to']
            x = th(s, f'th-leaf-{n}{part}-skin', 'doors', 'skin', ['+w'], **{"from": a, "to": rr(a + SKIN)})
            x.pop('holes', None)
            t.append(x)
            y = th(s, f'th-leaf-{n}{part}-insulation', 'doors', 'insulation', None, 0.5, **{"from": rr(a + SKIN), "to": b})
            t.append(y)
    return t


QUADRANT_FACES = ['-u', '-v', '+u', '+v']
QUADRANT_TURN = -math.pi / 4


def operating(by):
    t = [of('th-battery-pack', 'battery-pack', 'battery-pack', 'housing'), of('th-battery-module', 'battery-module', 'battery-pack', 'housing'),
         of('th-hvac-unit', 'hvac-unit', 'hvac-unit'), {**copy.deepcopy(by['hvac-fan']), 'id': 'th-hvac-vent', 'region': 'hvac-unit', 'tags': ['emitter', 'hvac-vent']},
         of('th-axle-beam-r', 'axle-beam-r', 'e-axle', 'casting')]
    for tag, side in (('l', 1), ('r', -1)):
        t.append(of(f'th-e-axle-motor-{tag}', f'e-axle-motor-{tag}', 'e-axle', 'casting'))
        t.append(of(f'th-e-axle-drop-{tag}', f'e-axle-drop-{tag}', 'e-axle', 'casting'))
        t.append(of(f'th-e-axle-spindle-{tag}', f'e-axle-spindle-{tag}', 'e-axle', 'casting'))
        t.append(of(f'th-spring-carrier-r-{tag}', f'spring-carrier-r-{tag}', 'e-axle', 'casting'))
        for w in (f'f{tag}', f'r{tag}', f'r{tag}-inner'):
            for k, face in enumerate(QUADRANT_FACES):
                s = copy.deepcopy(by[f'{w}-tyre'])
                s.update({"id": f'th-{w}-tyre-{k + 1}', "region": 'tyres', "layer": 'tread', "profile": [[rr(a), rr(r)] for a, r in tyre_loop()], "sweepRad": rr(math.pi / 2), "transform": {"rotation": qz(k * math.pi / 2 + QUADRANT_TURN)}, "sectionFaces": [face]})
                t.append(s)
        for w in (f'f{tag}', f'r{tag}'):
            t.append(of(f'th-{w}-brake-disc', f'{w}-brake-disc', 'brakes'))
    return t


def thermal50(body):
    by = {s['id']: s for s in body}
    return walls(by) + roof(by) + glazing(body) + doors(by) + operating(by)


STEEL = 'steel-wheel'


def m(id, bone, p0, p1, h=0.05, w=0.04, member=None):
    s = bar(id, bone, STEEL, 'body', 0.4, p0, p1, h, w)
    s.update({"member": member or bone, "part": "member"})
    return s


def sref(id, body, member, part):
    return {"id": id, "of": body, "member": member, "part": part}


def node(id, bone, at, r, part, tone=0.5):
    s = ell(id, bone, STEEL, 'detail', tone, [r, r, r] if not isinstance(r, (list, tuple)) else r, at)
    s.update({"member": bone, "part": part})
    return s


def post_points(side, x):
    return [x, 0.35, side * 1.25], [x, BELT, side * 1.25], [x, CANT, side * (hw(CANT) - 0.025)]


def structure50():
    s = []
    for tag, side in (('l', 1), ('r', -1)):
        s.append(m(f'longeron-{tag}', 'body', [0.4, 0.35, side * 0.28], [11.4, 0.35, side * 0.28], 0.1, 0.06))
        for k, (a, b) in enumerate(((0.4, 2.84), (4.06, 8.74), (9.96, 11.4))):
            s.append(m(f'sill-{tag}-{k + 1}', 'body', [a, 0.35, side * 1.25], [b, 0.35, side * 1.25], 0.08, 0.04))
        runs = ((0.4, 11.4),) if side > 0 else ((0.4, 6.23), (7.57, 10.155))
        for k, (a, b) in enumerate(runs):
            s.append(m(f'waist-rail-{tag}-{k + 1}', 'body', [a, BELT, side * 1.25], [b, BELT, side * 1.25], 0.05, 0.04))
        zc = side * (hw(CANT) - 0.025)
        s.append(m(f'cant-rail-{tag}', 'body', [0.4, CANT, zc], [11.4, CANT, zc], 0.06, 0.04))
        for k, x in enumerate(POSTS if side > 0 else POSTS_R):
            p0, p1, p2 = post_points(side, x)
            in_well = any(abs(x - xa) < ARCH_HALF + 0.05 for xa in (XR, XF))
            if in_well:
                p0 = [x, 1.07, side * 1.25]
            s.append(m(f'post-{tag}-{k + 1}-lower', 'body', p0, p1))
            s.append(m(f'post-{tag}-{k + 1}', 'body', p1, p2))
        if side < 0:
            for k, (a, b) in enumerate(((6.23, 7.57), (10.155, 11.4))):
                y = 2.41
                z = -(hw(y) - 0.025)
                s.append(m(f'door-header-{k + 1}', 'body', [a, y, z], [b, y, z], 0.06, 0.04))
    for k, x in enumerate(POSTS):
        half = 1.25
        for xa, zin in ((XR, Z_WELL_R), (XF, Z_WELL_F)):
            if abs(x - xa) < ARCH_HALF + 0.05:
                half = zin - 0.04
        s.append(m(f'cross-member-{k + 1}', 'body', [x, 0.35, -half], [x, 0.35, half], 0.08, 0.05))
        zc = hw(CANT) - 0.025
        pts = [[x, CANT, -zc], [x, 2.89, -1.0], [x, 2.935, 0.0], [x, 2.89, 1.0], [x, CANT, zc]]
        for j, (a, b) in enumerate(zip(pts, pts[1:])):
            s.append(m(f'roof-bow-{k + 1}-{j + 1}', 'body', a, b, 0.04, 0.04))
    s.append(m('bumper-beam-front', 'body', [11.86, 0.48, -1.0], [11.86, 0.48, 1.0], 0.12, 0.06))
    s.append(m('bumper-beam-rear', 'body', [0.12, 0.48, -1.1], [0.12, 0.48, 1.1], 0.12, 0.06))
    s.append(m('screen-sill-beam', 'body', [11.80, 1.1, -0.9], [11.80, 1.1, 0.9], 0.06, 0.05))
    s.append(m('screen-header-beam', 'body', [11.62, 2.76, -0.95], [11.62, 2.76, 0.95], 0.06, 0.05))
    s.append(m('rear-header-beam', 'body', [0.2, 2.6, -1.1], [0.2, 2.6, 1.1], 0.06, 0.05))
    for tag, side in (('l', 1), ('r', -1)):
        s.append(m(f'a-pillar-tube-{tag}', 'body', [11.51, 1.1, side * (HW - 0.03)], [11.34, 2.7, side * (hw(2.7) - 0.03)], 0.06, 0.05))
        s.append(m(f'rear-corner-post-{tag}', 'body', [0.2, 0.35, side * 1.1], [0.2, 2.6, side * 1.1], 0.06, 0.05))
    s.append(node('mass-body', 'body', [5.8, 1.3, 0], [0.45, 0.3, 0.45], 'mass', 0.6))
    for tag, side in (('l', 1), ('r', -1)):
        s.append(m(f'battery-rail-{tag}', 'battery', [0.2, 0.0, side * 0.8], [6.1, 0.0, side * 0.8], 0.06, 0.06))
        s.append(m(f'hvac-rail-{tag}', 'hvac', [0.15, 0.0, side * 0.7], [2.35, 0.0, side * 0.7], 0.05, 0.05))
        for e, x in (('a', 0.6), ('b', 5.7)):
            s.append(node(f'battery-mount-{tag}-{e}', 'battery', [x, -0.02, side * 0.8], 0.045, 'joint'))
        for e, x in (('a', 0.4), ('b', 2.1)):
            s.append(node(f'hvac-mount-{tag}-{e}', 'hvac', [x, -0.02, side * 0.7], 0.04, 'joint'))
    s.append(node('mass-battery', 'battery', [3.1, 0.15, 0], [0.5, 0.1, 0.4], 'mass', 0.6))
    s.append(node('mass-hvac', 'hvac', [1.25, 0.12, 0], [0.25, 0.08, 0.25], 'mass', 0.6))
    for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
        bone = f'door-{n}'
        z = -(HW + 0.017)
        for e, x in (('a', 0.035), ('b', LEAF - 0.035)):
            s.append(m(f'leaf-{n}-stile-{e}', bone, [x, 0.0, z], [x, 0.76, z], 0.04, 0.02))
            top = [x, 2.03, z + (2.03 - 0.76) * math.tan(PHI)]
            s.append(m(f'leaf-{n}-stile-{e}-upper', bone, [x, 0.76, z], top, 0.04, 0.02))
        for e, y in (('bottom', 0.07), ('mid', 0.76)):
            s.append(m(f'leaf-{n}-rail-{e}', bone, [0.02, y, z], [LEAF - 0.02, y, z], 0.05, 0.02))
        zt = z + (1.98 - 0.76) * math.tan(PHI)
        s.append(m(f'leaf-{n}-rail-top', bone, [0.02, 1.98, zt], [LEAF - 0.02, 1.98, zt], 0.05, 0.02))
        s.append(node(f'leaf-{n}-carriage', bone, [LEAF / 2, 2.03, zt + 0.01], [0.06, 0.025, 0.012], 'joint'))
    s.append(sref('axle-f-beam', 'axle-beam-f', 'axle-f', 'member'))
    s.append(sref('axle-r-beam', 'axle-beam-r', 'axle-r', 'member'))
    s.append(node('mass-axle-f', 'axle-f', [0, -0.31, 0], [0.1, 0.05, 0.1], 'mass', 0.6))
    s.append(node('mass-axle-r', 'axle-r', [0, -0.29, 0], [0.12, 0.06, 0.12], 'mass', 0.6))
    for tag, side in (('l', 1), ('r', -1)):
        s.append(sref(f'axle-f-spring-{tag}', f'air-spring-f-{tag}', 'axle-f', 'joint'))
        s.append(node(f'axle-f-kingpin-{tag}', 'axle-f', [0, 0, side * 0.84], 0.07, 'joint'))
        for e in ('a', 'b'):
            s.append(sref(f'axle-r-spring-{tag}-{e}', f'air-spring-r-{tag}-{e}', 'axle-r', 'joint'))
        s.append(sref(f'axle-r-carrier-{tag}', f'spring-carrier-r-{tag}', 'axle-r', 'member'))
        s.append(sref(f'wheel-f-rim-{tag}', f'f{tag}-rim', 'wheel-f', 'member'))
        s.append(sref(f'wheel-f-hub-{tag}', f'f{tag}-hub', 'wheel-f', 'joint'))
        s.append(sref(f'wheel-r-rim-{tag}', f'r{tag}-rim', 'wheel-r', 'member'))
        s.append(sref(f'wheel-r-rim-{tag}-inner', f'r{tag}-inner-rim', 'wheel-r', 'member'))
        s.append(sref(f'wheel-r-hub-{tag}', f'r{tag}-hub', 'wheel-r', 'joint'))
        s.append(node(f'mass-wheel-f-{tag}', 'wheel-f', [0, 0, side * Z_FRONT], [0.09, 0.09, 0.05], 'mass', 0.6))
        s.append(node(f'mass-wheel-r-{tag}', 'wheel-r', [0, 0, side * 0.915], [0.09, 0.09, 0.07], 'mass', 0.6))
    return s


DECISIONS = [
    {"en": "Body: a real shell instead of a block. Each side is a flat lower panel with the two wheel arches (and, on the kerb side, the door notches) and an upper window frame that leans in by 35 mm from the belt at 1.10 m to the cant rail at 2.62 m, cut with one opening per bay; the roof is one arched skin with 0.3 m cant radius and a 3 cm crown; nose and tail are curved walls on the plan outline (bowed front, tight rear corners) with their own floors, a rounded header whose lower edge stands 25 mm proud of the windscreen as a lip, and a tail cap. The windscreen is one curved pane raked by 7 degrees between the A-pillars, every side window is a 20 mm unit set 6 mm into its opening with a hopper transom, the rear window, lamp clusters, number plates, wipers, destination display, rub rails, arch trims, charging flap and both mirrors on their arms are parts of their own. The four sliding leaves hang 3 mm outside the wall, kinked at the belt like the wall, with two panes and edge seals each, so that they slide clear of the wall in every pose. Inside are the floor with its wheel-well cut-outs, the four wheelhouses, seats, grab rails, stanchions, the driver's seat, partition, dashboard and steering wheel. Running gear: 275/70 R 22.5 tyres with grooves on 22.5 x 8.25 disc wheels (front convex, rear twins dished against each other, ten hand holes and ten nuts), brake discs, the dropped front beam with stub axles and air springs, the portal drive axle with its two hub motor housings, the stub spindles that carry the twin wheels, drop arms, spring carriers and four air springs. Roof: battery fairing with six module lids and the heat pump housing with its fan and louvres.", "de": "Körper: eine echte Schale statt eines Blocks. Jede Seite besteht aus einem ebenen unteren Blech mit den beiden Radausschnitten (und auf der Bordsteinseite den Türausschnitten) und einem oberen Fensterrahmen, der vom Gürtel auf 1,10 m bis zum Dachlängsträger auf 2,62 m um 35 mm einzieht und je Feld eine Öffnung hat; das Dach ist eine gewölbte Haut mit 0,3 m Kantenradius und 3 cm Stich; Bug und Heck sind gebogene Wände auf dem Grundrissumriss (gewölbte Front, enge Heckecken) mit eigenen Böden, gerundetem Frontdom, dessen Unterkante als Lippe 25 mm vor der Windschutzscheibe steht, und Heckkappe. Die Windschutzscheibe ist eine gebogene, um 7 Grad geneigte Scheibe zwischen den A-Säulen, jedes Seitenfenster eine 20 mm starke Einheit, 6 mm in ihre Öffnung gesetzt, mit Kämpfer für das Klappfenster; Heckscheibe, Leuchten, Kennzeichen, Wischer, Zielanzeige, Scheuerleisten, Radlaufblenden, Ladeklappe und beide Spiegel an ihren Armen sind eigene Teile. Die vier Schiebeflügel hängen 3 mm vor der Wand, am Gürtel geknickt wie die Wand, mit je zwei Scheiben und Dichtkanten, sodass sie in jeder Pose frei vor der Wand laufen. Innen liegen der Boden mit den Radkastenausschnitten, die vier Radkästen, Sitze, Haltestangen, Stützen, Fahrersitz, Trennwand, Armaturenbrett und Lenkrad. Fahrwerk: Reifen 275/70 R 22,5 mit Rillen auf Scheibenrädern 22,5 x 8,25 (vorn gewölbt, hinten Zwillinge gegeneinander geschüsselt, zehn Handlöcher und zehn Muttern), Bremsscheiben, die gekröpfte Vorderachse mit Achsschenkeln und Luftfedern, die Portalantriebsachse mit zwei Radnabenmotorgehäusen, den Achsstummeln, die die Zwillingsräder tragen, Abtriebsarmen, Federträgern und vier Luftfedern. Dach: Batterieverkleidung mit sechs Moduldeckeln und Wärmepumpengehäuse mit Lüfter und Lamellen."},
    {"en": "Thermal: the conditioned envelope is modelled as its layers. Side panels, window frames, nose, tail and floor are a 1.5 mm skin plate with the 40 mm lining plate behind it; the roof skin is cut into two cant strips and the crown so that only its outer face counts, with its lining underneath; header and tail cap belong to the roof. Every pane, the windscreen and the rear window are a 4 mm outer pane, a 12 mm gap and a 4 mm inner pane. The door region is the whole outward face of the four leaves (skin and lining, the glazed part included, as its area is declared), the door glass also belongs to the glazing. Faces towards the cabin are marked as built-in faces. Wheelhouse liners and interior are not part of the painted panel region. Battery fairing with lids, heat pump housing with the fan disc as the outlet patch of the exhaust air, the drive axle beam, motor housings and drop arms, six tyres as tread and sidewalls down to the rim flange and the four brake discs are the operating regions.", "de": "Thermik: die konditionierte Hülle ist in ihren Schichten modelliert. Seitenbleche, Fensterrahmen, Bug, Heck und Boden sind eine 1,5 mm starke Hautplatte mit der 40 mm starken Verkleidung dahinter; die Dachhaut ist in zwei Kantenstreifen und die Kuppe geteilt, damit nur ihre Außenseite zählt, darunter ihre Verkleidung; Frontdom und Heckkappe gehören zum Dach. Jede Scheibe, die Windschutz- und die Heckscheibe bestehen aus 4 mm Außenscheibe, 12 mm Zwischenraum und 4 mm Innenscheibe. Die Türregion ist die ganze Außenfläche der vier Flügel (Haut und Verkleidung, der verglaste Teil eingeschlossen, so wie ihre Fläche deklariert ist), das Türglas gehört zusätzlich zur Verglasung. Flächen zum Innenraum sind als eingebaute Flächen markiert. Radkastenschalen und Innenausbau gehören nicht zur Region der lackierten Bleche. Batterieverkleidung mit Deckeln, Wärmepumpengehäuse mit der Lüfterscheibe als Austrittsfläche der Abluft, Antriebsachsträger, Motorgehäuse und Abtriebsarme, sechs Reifen als Lauffläche und Flanken bis zum Felgenhorn und die vier Bremsscheiben sind die Betriebsregionen."},
    {"en": "Structure: the welded tube frame of the body, not its skin. Two floor longerons, side sills interrupted at the wheel wells, a cross member and a roof bow in five straight pieces at every post, posts that are vertical to the belt and lean above it, waist rails, cant rails, door headers, bumper beams, the windscreen sill and header beams, A-pillar tubes and rear corner posts; battery and heat pump stand on two rails each with four mounts; every door leaf is its frame of two stiles and three rails with the carriage it hangs on; the front beam and the drive axle beam are the axle members with air springs and king pins as joints; the disc wheels are the wheel members with their hubs as joints; one mass at each centre of mass.", "de": "Tragwerk: das geschweißte Rohrgerippe des Aufbaus, nicht seine Haut. Zwei Bodenlängsträger, an den Radkästen unterbrochene Schweller, an jedem Pfosten ein Querträger und ein Dachspriegel aus fünf geraden Stücken, Pfosten, die bis zum Gürtel senkrecht stehen und darüber einziehen, Gürtelholme, Dachlängsträger, Türstürze, Stoßfängerträger, Brüstungs- und Sturzträger der Frontscheibe, A-Säulen-Rohre und hintere Eckpfosten; Batterie und Wärmepumpe stehen auf je zwei Schienen mit vier Lagern; jeder Türflügel ist sein Rahmen aus zwei Holmen und drei Riegeln mit dem Laufwagen, an dem er hängt; die Vorderachse und der Antriebsachsträger sind die Achsstäbe mit Luftfedern und Achsschenkelbolzen als Gelenken; die Scheibenräder sind die Radstäbe mit ihren Naben als Gelenken; je eine Masse in jedem Schwerpunkt."},
]
