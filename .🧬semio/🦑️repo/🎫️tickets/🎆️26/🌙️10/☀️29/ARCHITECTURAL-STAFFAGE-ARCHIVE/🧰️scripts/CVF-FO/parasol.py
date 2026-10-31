"""market-parasol v2: eight ribs, scalloped canopy, sun-lit top and shaded underside."""
import sys
from lib import *
from common import *

path, A = load('objects', 'market-parasol')
rig = Rig(A['rig']['bones'])
TOP, UND, FRM, BAS = 'canopy-top', 'canopy-underside', 'frame-alu', 'base-concrete'
KT, KF, KB = (dict(region=TOP, material='polyester-canvas'), dict(region=FRM, material='aluminium-anodised'), dict(region=BAS, material='concrete'))
RIBS = [('rib-right', 1.237426), ('rib-b', .750699), ('rib-c', .403393), ('rib-d', 1.063203), ('rib-left', 1.237426), ('rib-f', .750699), ('rib-g', .403393), ('rib-h', 1.063203)]
WR = {n: lw(rig, n) for n, _ in RIBS}
PANEL = {'rib-right': [('M', 0, 0), ('C', .3465, .115, .8909, .1035, 1.2374, 0), ('Z',)], 'rib-left': [('M', 0, 0), ('C', .3465, -.115, .8909, -.1035, 1.2374, 0), ('Z',)]}


PANEL_W = {'rib-right': [('M', 0, 0), ('C', .3465, .115, .8909, .1035, 1.2374, 0), ('L', 1.3339, .0264), ('Z',)], 'rib-left': [('M', 0, 0), ('C', .3465, -.115, .8909, -.1035, 1.2374, 0), ('L', 1.3339, -.0264), ('Z',)]}


def mapc(W, cmds):
    out = []
    for c in cmds:
        if c[0] == 'Z':
            out.append(c)
            continue
        v = []
        for i in range(1, len(c), 2):
            v += list(W(c[i], c[i + 1]))
        out.append((c[0], *v))
    return out


def rep50():
    sh = Shapes(rig)
    sh.items = v1_shapes(A, skip=('base-bolt-0', 'base-bolt-1', 'stretch-right-eye', 'stretch-left-eye'))
    for s in sh.items:
        if s.get('region') == 'canopy-fabric':
            s['region'] = TOP
            s['material'] = 'polyester-canvas'
        i = s['id']
        if i.startswith('panel-') and not i.endswith(('seam', 'fold')):
            s['penMm'] = 0.35
        if i.endswith('-rod'):
            s['penMm'] = 0.25
        if i.endswith('-tip'):
            s['penMm'] = 0.13
        if '-scallop-' in i:
            s['penMm'] = 0.25
        if i in ('mast', 'base-block', 'crown-hub'):
            s['penMm'] = 0.35
    sh.ids = {s['id'] for s in sh.items}
    return rep(50, sh, dict(
        silhouette=T('Side elevation 52 x 49 mm: a concrete block with socket, a tilting mast with collar and crank, a runner with two stretchers, a crown hub and eight ribs fanning into a gently domed canopy whose hem is scalloped between the ribs.',
                     'Seitenansicht 52 x 49 mm: ein Betonblock mit Hülse, ein kippbarer Mast mit Kragen und Kurbel, ein Läufer mit zwei Streben, eine Krone und acht Speichen, die sich zu einer flach gewölbten Bespannung auffächern, deren Saum zwischen den Speichen gewellt ist.'),
        retained=T('All eight ribs as rods with tips (the two main ribs full length, the others in their foreshortened projection), the scallop pair at every rib tip, both fabric panels with seam and fold, mast knuckle, collar, crank with knob, runner collar and detent, crown hub with finial, base block with seam, plate, socket and grit.', 'Alle acht Speichen als Stäbe mit Spitzen (die zwei Hauptspeichen in voller Länge, die übrigen in ihrer verkürzten Projektion), das Bogenpaar an jeder Speichenspitze, beide Stoffbahnen mit Naht und Falte, Mastgelenk, Kragen, Kurbel mit Knopf, Läuferkragen und Raste, Krone mit Knauf, Betonblock mit Fuge, Platte, Hülse und Körnung.'),
        merged=T('Nothing merged.', 'Nichts verschmolzen.'),
        omitted=T('Base bolts and stretcher eyes (0.32 mm dots, below the minimum feature); the underside of the canopy (hidden from the side).', 'Sockelbolzen und Streberösen (0,32-mm-Punkte, unter dem Mindestmaß); die Unterseite der Bespannung (von der Seite verdeckt).'),
        thinElements=T('Ribs (0.014-0.022 m) are 0.28-0.44 mm capsules with a 0.25 mm outline; scallop curves are single 0.25 mm strokes; stretchers (0.012 m) are 0.24 mm capsules; the rib tip is a 0.5 mm dot with a 0.13 mm outline.', 'Speichen (0,014-0,022 m) sind 0,28-0,44 mm breite Kapseln mit 0,25-mm-Kontur; die Bögen sind einzelne 0,25-mm-Striche; Streben (0,012 m) sind 0,24 mm breite Kapseln; die Speichenspitze ist ein 0,5-mm-Punkt mit 0,13-mm-Kontur.'),
        hatching=T('None; fabric folds and base grit are single 0.13 mm strokes.', 'Keine; Stofffalten und Sockelkörnung sind einzelne 0,13-mm-Striche.'),
        styles=T('Fabric very light (0.82), frame mid (0.35-0.45), base block 0.62; body pen 0.35-0.5, scallops 0.25, folds 0.13.', 'Stoff sehr hell (0,82), Gestell mittel (0,35-0,45), Betonblock 0,62; Körperstift 0,35-0,5, Bögen 0,25, Falten 0,13.'),
        overlays=T('Panels, seams and scallops = canopy-top (sun-lit upper fabric); mast, ribs, runner, stretchers, crown and socket = frame-alu; block and grit = base-concrete; the shaded underside (canopy-underside) is hidden in elevation and read numerically.', 'Bahnen, Nähte und Bögen = canopy-top (besonnter Oberstoff); Mast, Speichen, Läufer, Streben, Krone und Hülse = frame-alu; Block und Körnung = base-concrete; die beschattete Unterseite (canopy-underside) ist in der Ansicht verdeckt und wird numerisch gelesen.')))


def rep100():
    sh = Shapes(rig)
    sh.rect('base-block', 'base', -.2, 0, .2, .2, r=.01, z=1, tone=0.62, **KB)
    sh.path('base-socket', 'base', [('M', -.045, .215), ('L', -.035, .30), ('L', .035, .30), ('L', .045, .215), ('Z',)], z=1.3, tone=0.3, **KF)
    sh.line('mast', 'mast', [(0, .30), (0, 2.45)], role='body', z=3, tone=0.45, **KF)
    sh.rect('crown-hub', 'crown', -.045, 2.29, .045, 2.43, r=.02, z=3.5, tone=0.35, **KF) if False else None
    ch = lw(rig, 'crown')
    p0, p1 = ch(-.06, -.045), ch(.08, .045)
    sh.rect('crown-hub', 'crown', min(p0[0], p1[0]), min(p0[1], p1[1]), max(p0[0], p1[0]), max(p0[1], p1[1]), z=3.5, tone=0.35, **KF)
    for n, L in RIBS:
        if n in PANEL:
            sh.path(f'panel-{n[4:]}', n, mapc(WR[n], PANEL[n]), z=2, tone=0.82, **KT)
    for n, L in RIBS:
        sh.line(f'{n}-rod', n, [WR[n](0, 0), WR[n](L, 0)], role='detail', z=2.5, tone=0.35, **KF)
    for n in ('rib-right', 'rib-left'):
        sg = 1 if n == 'rib-right' else -1
        sh.path(f'{n}-scallop-a', n, mapc(WR[n], [('M', 1.2374, 0), ('Q', 1.2712, sg * .0662, 1.3339, sg * .0264)]), role='detail', z=2.7, filled=False, tone=0.3, **KT)
        sh.path(f'{n}-scallop-b', n, mapc(WR[n], [('M', 1.2374, 0), ('Q', 1.1747, sg * .0399, 1.1410, -sg * .0264)]), role='detail', z=2.7, filled=False, tone=0.3, **KT)
    rc = lw(rig, 'runner')
    a0, a1 = rc(-.035, -.04), rc(.035, .04)
    sh.rect('runner-collar', 'runner', min(a0[0], a1[0]), min(a0[1], a1[1]), max(a0[0], a1[0]), max(a0[1], a1[1]), z=3.5, tone=0.35, **KF)
    for n in ('stretch-right', 'stretch-left'):
        w = lw(rig, n)
        sh.line(n, n, [w(0, 0), w(.55, 0)], role='detail', z=2.4, tone=0.4, **KF)
    return rep(100, sh, dict(
        silhouette=T('Side elevation 26 x 24.6 mm: a block with socket, a single mast stroke, a crown hub and a fan of eight rib strokes under two domed fabric panels with scalloped hem on the two main ribs.',
                     'Seitenansicht 26 x 24,6 mm: ein Block mit Hülse, ein einzelner Mastenstrich, eine Krone und ein Fächer aus acht Speichenstrichen unter zwei gewölbten Stoffbahnen mit gewelltem Saum an den zwei Hauptspeichen.'),
        retained=T('Both fabric panels, all eight ribs (they define the fan and are the animated parts), the two main scallop pairs, runner collar with both stretchers, crown hub, mast, socket, block.', 'Beide Stoffbahnen, alle acht Speichen (sie bilden den Fächer und sind die bewegten Teile), die zwei Hauptbogenpaare, Läuferkragen mit beiden Streben, Krone, Mast, Hülse, Block.'),
        merged=T('Mast knuckle, collar, crank, seam and fold merge into the mast stroke and the panels; finial merges into the hub; base plate merges into the block top.', 'Mastgelenk, Kragen, Kurbel, Naht und Falte gehen in Mastenstrich und Bahnen auf; der Knauf geht in der Krone auf; die Sockelplatte geht in der Blockoberkante auf.'),
        omitted=T('Scallops of the six secondary ribs, rib tips, stretcher eyes, seams, fold, grit, crank, knuckle.', 'Bögen der sechs Nebenspeichen, Speichenspitzen, Streberösen, Nähte, Falte, Körnung, Kurbel, Gelenk.'),
        thinElements=T('Ribs (0.014-0.022 m) and stretchers (0.012 m) are single 0.18 mm strokes (18 mm real weight); the 0.042-0.052 m mast a single 0.35 mm stroke; the tilt joint below it is hidden.', 'Speichen (0,014-0,022 m) und Streben (0,012 m) sind einzelne 0,18-mm-Striche (18 mm reale Stärke); der 0,042-0,052 m starke Mast ein einzelner 0,35-mm-Strich; das Kippgelenk darunter ist verdeckt.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Fabric 0.82, frame black strokes, block 0.62; body pen 0.35, rods and scallops 0.18.', 'Stoff 0,82, Gestell schwarze Striche, Block 0,62; Körperstift 0,35, Speichen und Bögen 0,18.'),
        overlays=T('Panels and scallops = canopy-top; ribs, mast, stretchers, collar, hub, socket = frame-alu (line colour); block = base-concrete; the underside canopy-underside is not drawn.', 'Bahnen und Bögen = canopy-top; Speichen, Mast, Streben, Kragen, Krone, Hülse = frame-alu (Linienfarbe); Block = base-concrete; die Unterseite canopy-underside wird nicht gezeichnet.')))


def rep200():
    sh = Shapes(rig)
    sh.rect('base-block', 'base', -.2, 0, .2, .2, z=1, tone=0.62, **KB)
    sh.line('mast', 'mast', [(0, .2), (0, 2.46)], role='body', z=3, tone=0.45, **KF)
    ch = lw(rig, 'crown')
    p0, p1 = ch(-.06, -.045), ch(.08, .045)
    sh.rect('crown-hub', 'crown', min(p0[0], p1[0]), min(p0[1], p1[1]), max(p0[0], p1[0]), max(p0[1], p1[1]), z=3.5, tone=0.35, **KF)
    for n in PANEL:
        sh.path(f'panel-{n[4:]}', n, mapc(WR[n], PANEL_W[n]), z=2, tone=0.82, **KT)
    for n, L in RIBS:
        if n in ('rib-right', 'rib-left', 'rib-d', 'rib-h'):
            sh.line(f'{n}-rod', n, [WR[n](0, 0), WR[n](L, 0)], role='detail', z=2.5, tone=0.35, **KF)
    for n in ('stretch-right', 'stretch-left'):
        w = lw(rig, n)
        sh.line(n, n, [w(0, 0), w(.55, 0)], role='detail', z=2.4, tone=0.4, **KF)
    return rep(200, sh, dict(
        silhouette=T('Side elevation 13 x 12.3 mm: a block, a mast stroke, a small hub, four rib hairlines under two shallow fabric lenses, two stretcher hairlines.', 'Seitenansicht 13 x 12,3 mm: ein Block, ein Mastenstrich, eine kleine Krone, vier Speichenhaarlinien unter zwei flachen Stofflinsen, zwei Strebenhaarlinien.'),
        retained=T('The two fabric panels (the dome reads as a flat umbrella), the four longest ribs, the mast, hub, stretchers, block.', 'Die zwei Stoffbahnen (die Wölbung liest sich als flacher Schirm), die vier längsten Speichen, Mast, Krone, Streben, Block.'),
        merged=T('The four short ribs (0.4 and 0.75 m, 0.2-0.4 mm apart from the long ones) merge into the long rib fan; socket, plate and block become one block; runner collar and stretcher joint merge.', 'Die vier kurzen Speichen (0,4 und 0,75 m, 0,2-0,4 mm von den langen entfernt) gehen im Fächer der langen auf; Hülse, Platte und Block werden ein Block; Läuferkragen und Strebenanschluss gehen ineinander auf.'),
        omitted=T('Four short ribs, all scallops (0.07 m = 0.33 mm), seams, folds, tips, crank, knuckle, grit, finial.', 'Vier kurze Speichen, alle Bögen (0,07 m = 0,33 mm), Nähte, Falten, Spitzen, Kurbel, Gelenk, Körnung, Knauf.'),
        thinElements=T('Ribs and stretchers (0.012-0.022 m) are 0.13 mm strokes (26 mm real); the mast (0.05 m) a single 0.25 mm stroke.', 'Speichen und Streben (0,012-0,022 m) sind 0,13-mm-Striche (26 mm real); der Mast (0,05 m) ein einzelner 0,25-mm-Strich.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Fabric 0.82, block 0.62, strokes black; body pen 0.25, rods 0.13.', 'Stoff 0,82, Block 0,62, Striche schwarz; Körperstift 0,25, Speichen 0,13.'),
        overlays=T('Panels = canopy-top; ribs, mast, hub, stretchers = frame-alu (line colour); block = base-concrete; underside not drawn.', 'Bahnen = canopy-top; Speichen, Mast, Krone, Streben = frame-alu (Linienfarbe); Block = base-concrete; Unterseite nicht gezeichnet.')))


def rep500():
    sh = Shapes(rig)
    sh.rect('base-block', 'base', -.2, 0, .2, .2, z=1, tone=0.62, **KB)
    sh.line('mast', 'mast', [(0, .2), (0, 2.46)], role='body', z=3, tone=0.45, **KF)
    for n in PANEL:
        sh.path(f'panel-{n[4:]}', n, mapc(WR[n], PANEL_W[n]), z=2, tone=0.82, **KT)
    return rep(500, sh, dict(
        silhouette=T('Composed parasol sign 5.2 x 4.8 mm: a base block, a mast stroke and a flat two-lobed canopy of two thin lenses meeting at the crown.', 'Zusammengesetztes Schirmzeichen 5,2 x 4,8 mm: ein Sockelblock, ein Mastenstrich und ein flaches zweilappiges Dach aus zwei dünnen Linsen, die sich an der Krone treffen.'),
        retained=T('Overall width (2.4 m canopy), height, the canopy as the dominant shape, the block as the only mass at the foot.', 'Gesamtbreite (2,4 m Bespannung), Höhe, die Bespannung als dominante Form, der Block als einzige Masse am Fuß.'),
        merged=T('Eight ribs, runner, stretchers, hub, scallops and socket vanish into the two panel outlines and the mast stroke.', 'Acht Speichen, Läufer, Streben, Krone, Bögen und Hülse gehen in den zwei Bahnenkonturen und dem Mastenstrich auf.'),
        omitted=T('Ribs, stretchers, hub, runner, scallops, seams, socket, crank - all below 0.1 m.', 'Speichen, Streben, Krone, Läufer, Bögen, Nähte, Hülse, Kurbel - alle unter 0,1 m.'),
        thinElements=T('The 0.05 m mast is a single 0.18 mm stroke (90 mm real); the canopy lenses are 0.1 m (0.2 mm) arches thickened by their own outline; ribs are not drawn.', 'Der 0,05 m starke Mast ist ein einzelner 0,18-mm-Strich (90 mm real); die Bespannungslinsen sind 0,1 m (0,2 mm) hohe Bögen, durch ihre eigene Kontur verstärkt; Speichen werden nicht gezeichnet.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Fabric very light (0.82), block 0.62, mast black; one 0.18 mm outline.', 'Stoff sehr hell (0,82), Block 0,62, Mast schwarz; eine 0,18-mm-Kontur.'),
        overlays=T('Lenses = canopy-top, mast stroke = frame-alu (line colour), block = base-concrete; canopy-underside is not drawn; the two lenses stay separate shapes so the canopy opens and furls with its ribs.', 'Linsen = canopy-top, Mastenstrich = frame-alu (Linienfarbe), Block = base-concrete; canopy-underside wird nicht gezeichnet; die zwei Linsen bleiben getrennte Formen, damit sich die Bespannung mit den Speichen öffnet und schließt.')))


def thermal():
    al, cc = MAT['aluminium'], MAT['concrete']
    c_al = effective_cap(al['rho'], al['cp'], al['k'], 0.002)
    c_cc = effective_cap(cc['rho'], cc['cp'], cc['k'], 0.2)
    c_fab = round(0.28 * 1250)
    fab = lambda i: layer(i, T('Polyester canopy cloth, 280 g/m2', 'Polyester-Schirmstoff, 280 g/m2'), 'polymer', 'polyester-canvas', 0.0005, 0.2, c_fab, source='authored-estimate')
    A['thermal'] = {'environment': {'airTempC': 25, 'meanRadiantTempC': 35, 'relativeHumidity': 0.4, 'windMS': 2.2, 'windDirectionDeg': 180, 'solarWM2': 700, 'diffuseFraction': 0.15, 'pressurePa': 101325}, 'operating': [], 'emitters': [],
                    'regions': [
        region(TOP, T('Canopy fabric, sun-lit top', 'Schirmstoff, besonnte Oberseite'), 'surface', 'passive', 'polyester-canvas', 5.3, 1.3, 'horizontal-plate-up', 1.0, [fab('cloth-top')], c_fab),
        region(UND, T('Canopy fabric, shaded underside', 'Schirmstoff, beschattete Unterseite'), 'surface', 'passive', 'polyester-canvas', 5.3, 1.3, 'horizontal-plate-down', 0.15, [fab('cloth-under')], c_fab),
        region(FRM, T('Aluminium mast and ribs', 'Aluminium-Mast und Speichen'), 'surface', 'passive', 'aluminium-anodised', 0.6, 0.05, 'cylinder', 0.6, [layer('tube', T('Aluminium tube wall 2 mm', 'Aluminiumrohrwand 2 mm'), 'metal', 'aluminium-anodised', 0.002, al['k'], c_al, reference='iso-10456-2007')], c_al, reference='iso-10456-2007'),
        region(BAS, T('Concrete base', 'Betonfuß'), 'surface', 'passive', 'concrete', 0.4, 0.3, 'vertical-plate', 0.5, [layer('concrete', T('Concrete block (thermally active 51 mm)', 'Betonblock (thermisch aktiv 51 mm)'), 'masonry', 'concrete', 0.2, cc['k'], c_cc, reference='iso-10456-2007')], c_cc, reference='iso-10456-2007')],
                    'assumptions': [
        T('Summer sun reference case; no emitters, no authored temperatures. The canopy separates a sun-exposed top (canopy-top, sky view 1) from a shaded underside (canopy-underside, facing the ground and table) and shades the frame and the base; these sunlit fractions are bound by pose.', 'Sommerliche Sonne als Referenzfall; keine Emitter, keine festgelegten Temperaturen. Die Bespannung trennt eine besonnte Oberseite (canopy-top, Himmelssicht 1) von einer beschatteten Unterseite (canopy-underside, dem Boden und Tisch zugewandt) und beschattet Gestell und Fuß; diese besonnten Anteile werden je Pose gebunden.'),
        T('The cloth is thermally thin: 280 g/m2 at about 1250 J/kgK gives 350 J/m2K, a response time of seconds to a minute; the 2 mm aluminium tube 5 kJ/m2K; the concrete block 117 kJ/m2K (51 mm active), responding over hours.', 'Der Stoff ist thermisch dünn: 280 g/m2 bei etwa 1250 J/kgK ergibt 350 J/m2K, eine Antwortzeit von Sekunden bis zu einer Minute; das 2-mm-Aluminiumrohr 5 kJ/m2K; der Betonblock 117 kJ/m2K (51 mm aktiv) mit Antwort über Stunden.'),
        T('Cloth mass, conductivity and specific heat are authored estimates; metals and concrete after ISO 10456.', 'Stoffmasse, Leitfähigkeit und spezifische Wärme sind Schätzwerte; Metalle und Beton nach ISO 10456.')]}


def bind(top, und, frame, base, ex_und=1.0):
    return binding({TOP: (1.0, top), UND: (ex_und, und), FRM: (1.0, frame), BAS: (1.0, base)})


def finish():
    A['category'] = 'objects'
    A['dimensions'] = {'heightM': 2.46, 'widthM': 2.6, 'depthM': 2.6, 'massKg': 66.7}
    A['provenance']['author'] = 'Semio staffage content (CV-FO)'
    A['provenance']['references'] = [REF_EN1991_1_4, REF_EN1992, REF_EN1999, REF_ASHBY, REF_ISO10456, REF_INCROPERA]
    A['materials'] = [
        {'id': 'concrete', 'title': T('Concrete', 'Beton'), 'emissivity': 0.92, 'solarAbsorptivity': 0.6, 'densityKgM3': 2400, 'conductivityWmK': 1.65, 'specificHeatJKgK': 1000, 'source': 'literature', 'reference': 'iso-10456-2007'},
        {'id': 'aluminium-anodised', 'title': T('Anodised aluminium', 'Eloxiertes Aluminium'), 'emissivity': 0.8, 'solarAbsorptivity': 0.5, 'densityKgM3': 2700, 'conductivityWmK': 160, 'specificHeatJKgK': 880, 'elasticModulusPa': 70000000000, 'source': 'authored-estimate', 'reference': 'iso-10456-2007'},
        {'id': 'polyester-canvas', 'title': T('Polyester canopy cloth', 'Polyester-Schirmstoff'), 'emissivity': 0.9, 'solarAbsorptivity': 0.5, 'conductivityWmK': 0.2, 'specificHeatJKgK': 1250, 'source': 'authored-estimate'}]
    A['representations'] = {'500': rep500(), '200': rep200(), '100': rep100(), '50': rep50()}
    A['symbols'] = []
    A.pop('geometry', None)
    thermal()
    b = {'open-still': bind(1.0, 0.0, 0.2, 0.5), 'tilted-to-sun': bind(1.0, 0.0, 0.25, 0.3), 'furled': bind(0.5, 0.0, 0.8, 1.0, 0.1)}
    for p in A['poses']:
        p['thermal'] = b[p['id']]
    for c in A['clips']:
        if c['id'] == 'sun-tilt':
            c['tracks'] += [{'target': f'region:{BAS}:sunlitFraction', 'ease': 'smooth', 'keys': [{'t': 0, 'v': 0.5}, {'t': 4, 'v': 0.3}, {'t': 8, 'v': 0.5}]}]
    A['structure']['assumptions'] = [a for i, a in enumerate(A['structure']['assumptions']) if i != 3]
    A['presentation']['defaultScale'] = 50
    A['completion'] = 'authored'
    return sorted_keys(A)


if __name__ == '__main__':
    out = finish()
    for k in ('500', '200', '100', '50'):
        print(k, [round(v, 4) for v in bbox_world(rig, out['representations'][k]['geometry']['elevation'])])
    if '--write' in sys.argv:
        save(path, out)
    for k in ('500', '200', '100', '50'):
        render(out['representations'][k], rig, int(k), f'parasol-{k}-print', dpi=600 if k != '50' else 400, zoom=4 if k == '500' else 2 if k == '200' else 1)
