"""cafe-chair v2: side elevation, four scales, occupant sit clip, passive thermal model."""
import math, sys
from lib import *
from common import *

path, A = load('furniture', 'cafe-chair')
rig = Rig(A['rig']['bones'])
SEAT, BACK, STEEL = 'seat-beech', 'back-beech', 'steel-tube'
c95, s95 = math.cos(math.radians(95)), math.sin(math.radians(95))
ORG = (0.066, 0.46)


def bp(t, n=0.0):
    return (ORG[0] + t * c95 - n * s95, ORG[1] + t * s95 + n * c95)


REAR0, REAR1 = (0.05, 0.0), (0.0660, 0.4597)
FRONT0, FRONT1 = (0.42, 0.0), (0.4079, 0.4597)
KS, KB = dict(region=STEEL, material='steel-tube'), dict(region=BACK, material='beech-timber')
KSE = dict(region=SEAT, material='beech-timber')


def rep50():
    sh = Shapes(rig)
    sh.items = v1_shapes(A, skip=('seat-screw-0', 'seat-screw-1', 'back-rivet-a', 'back-rivet-b'))
    for s in sh.items:
        if s['id'] in ('leg-rear-tube', 'leg-front-tube'):
            s['x0'] = 0.0095
            s['x1'] = 0.42
        if s['id'] == 'leg-rear-knuckle':
            s['cx'] = 0.42
        if s['id'] in ('leg-foot-rear', 'leg-foot-front'):
            s['x'] = 0.0
        if s['id'] in ('leg-rear-tube', 'leg-front-tube', 'back-post', 'back-finial'):
            s['penMm'] = 0.3
        if s['id'] in ('seat-pad', 'back-panel'):
            s['penMm'] = 0.35
        if s['id'] in ('stretcher-tube', 'back-lower-rail', 'seat-apron'):
            s['penMm'] = 0.25
    sh.ids = {s['id'] for s in sh.items}
    return rep(50, sh, dict(
        silhouette=T('Side elevation 17.6 x 22.0 mm: padded beech seat on an apron, two splayed tubular legs (near and far legs coincide), a stretcher and a back post leaning 5 degrees with panel and lower rail.',
                     'Seitenansicht 17,6 x 22,0 mm: gepolsterter Buchensitz auf Zarge, zwei gespreizte Rohrbeine (nahe und ferne Beine fallen zusammen), Querstrebe und ein um 5 Grad geneigter Rückenpfosten mit Rückenbrett und unterer Leiste.'),
        retained=T('Seat pad with rounded edge, apron with line, back post with finial, back panel and rail, tube legs with ferrules and foot pads, rear knuckle, stretcher with joints.',
                   'Sitzpolster mit gerundeter Kante, Zarge mit Linie, Rückenpfosten mit Knauf, Rückenbrett und Leiste, Rohrbeine mit Hülsen und Fußkappen, hinterer Knoten, Querstrebe mit Anschlüssen.'),
        merged=T('Nothing merged; near and far legs are one projected tube each.', 'Nichts verschmolzen; nahes und fernes Bein sind je ein projiziertes Rohr.'),
        omitted=T('Seat screws and back rivets (5-6 mm = 0.1 mm paper, below the minimum feature); the second back post and stretcher end behind the first; no upholstery seams.', 'Sitzschrauben und Rückennieten (5-6 mm = 0,1 mm Papier, unter dem Mindestmaß); der zweite Rückenpfosten und das zweite Strebenende liegen hinter dem ersten; keine Polsternähte.'),
        thinElements=T('The 20-25 mm tubes are 0.4-0.5 mm solids (capsules) plus 0.5 mm outline; ferrule lines are 0.25 mm details; the stretcher (14 mm) is a 0.28 mm capsule.',
                       'Die 20-25-mm-Rohre sind 0,4-0,5 mm breite Kapseln plus 0,5-mm-Kontur; Hülsenlinien sind 0,25-mm-Details; die Querstrebe (14 mm) ist eine 0,28 mm breite Kapsel.'),
        hatching=T('Grain only as two short 0.13 mm texture strokes on seat and back panel; a hatch would need 0.5 mm spacing, which does not fit the 0.56 mm thick pad.', 'Maserung nur als zwei kurze 0,13-mm-Texturstriche auf Sitz und Rückenbrett; eine Schraffur bräuchte 0,5 mm Abstand, was nicht in das 0,56 mm dicke Polster passt.'),
        styles=T('Beech light (0.42-0.62), steel dark (0.35); body 0.5, detail 0.25, texture 0.13.', 'Buche hell (0,42-0,62), Stahl dunkel (0,35); Körper 0,5, Detail 0,25, Textur 0,13.'),
        overlays=T('Seat pad and apron are seat-beech, post, panel and rail are back-beech, tubes and ferrules steel-tube; contact warming of an occupant touches seat-beech and back-beech only.',
                   'Sitzpolster und Zarge sind seat-beech, Pfosten, Brett und Leiste back-beech, Rohre und Hülsen steel-tube; die Erwärmung durch eine Person betrifft nur seat-beech und back-beech.')))


def rep100():
    sh = Shapes(rig)
    sh.rect('seat-band', 'seat', .03, .398, .458, .46, r=.012, z=4, tone=0.6, **KSE)
    sh.line('seat-edge', 'seat', [(.04, .432), (.448, .432)], z=4.1, **KSE)
    sh.line('leg-rear', 'leg-rear', [REAR0, (0.0660, 0.40)], role='body', z=3, tone=0.35, **KS)
    sh.line('leg-front', 'leg-front', [FRONT0, (0.4086, 0.40)], role='body', z=3, tone=0.35, **KS)
    sh.line('stretcher', 'stretcher', [(.0542, .12), (.4158, .12)], role='detail', z=2, **KS)
    sh.line('back-post', 'back', [bp(0), bp(.418)], role='body', z=3.5, tone=0.42, **KB)
    a, b = bp(.215, -.006), bp(.385, -.006)
    sh.cap('back-panel', 'back', a[0], a[1], b[0], b[1], .014, .014, z=4, tone=0.52, **KB)
    a, b = bp(.11, -.007), bp(.17, -.007)
    sh.cap('back-rail', 'back', a[0], a[1], b[0], b[1], .01, .01, z=3.9, tone=0.5, **KB)
    return rep(100, sh, dict(
        silhouette=T('Side elevation 4.4 x 8.8 mm: a seat band, two leg strokes, a stretcher, a leaning back post with panel and lower rail.', 'Seitenansicht 4,4 x 8,8 mm: Sitzband, zwei Beinstriche, Querstrebe, geneigter Rückenpfosten mit Brett und unterer Leiste.'),
        retained=T('Seat (pad and apron as one band with the pad edge as a line), both tube legs, stretcher, back post, back panel, lower rail.', 'Sitz (Polster und Zarge als ein Band mit der Polsterkante als Linie), beide Rohrbeine, Querstrebe, Rückenpfosten, Rückenbrett, untere Leiste.'),
        merged=T('Pad and apron become one band (0.62 mm); near and far legs stay one projected line.', 'Polster und Zarge werden ein Band (0,62 mm); nahes und fernes Bein bleiben eine projizierte Linie.'),
        omitted=T('Ferrules, foot pads, finial, screws, rivets, knuckle, grain.', 'Hülsen, Fußkappen, Knauf, Schrauben, Nieten, Knoten, Maserung.'),
        thinElements=T('Tube legs (20-25 mm) are single 0.35 mm strokes (35 mm real line weight, within the 0.35 mm pen tolerance); the stretcher (14 mm) is one 0.18 mm stroke; the post is a single stroke; panel and rail are filled capsules.',
                       'Rohrbeine (20-25 mm) sind einzelne 0,35-mm-Striche (35 mm reale Linienstärke, innerhalb der Stifttoleranz); die Querstrebe (14 mm) ist ein 0,18-mm-Strich; der Pfosten ist ein Einzelstrich; Brett und Leiste sind gefüllte Kapseln.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Beech tone 0.5-0.6; steel strokes black; body pen 0.35, detail 0.18.', 'Buche Ton 0,5-0,6; Stahlstriche schwarz; Körperstift 0,35, Detail 0,18.'),
        overlays=T('Strokes carry the colour of their region (steel-tube) as line colour; seat band is seat-beech, post, panel and rail back-beech.', 'Striche tragen die Farbe ihrer Region (steel-tube) als Linienfarbe; das Sitzband ist seat-beech, Pfosten, Brett und Leiste back-beech.')))


def rep200():
    sh = Shapes(rig)
    sh.rect('seat-band', 'seat', .03, .40, .458, .46, z=4, tone=0.6, **KSE)
    sh.line('leg-rear', 'leg-rear', [REAR0, (0.0660, 0.40)], role='body', z=3, **KS)
    sh.line('leg-front', 'leg-front', [FRONT0, (0.4086, 0.40)], role='body', z=3, **KS)
    sh.line('stretcher', 'stretcher', [(.0542, .12), (.4158, .12)], role='detail', z=2, **KS)
    sh.line('back-post', 'back', [bp(0), bp(.418)], role='body', z=3.5, **KB)
    a, b = bp(.2, -.006), bp(.385, -.006)
    sh.cap('back-panel', 'back', a[0], a[1], b[0], b[1], .016, .016, z=4, tone=0.52, **KB)
    return rep(200, sh, dict(
        silhouette=T('Side elevation 2.2 x 4.4 mm: seat band, two leg strokes, a stretcher hairline, a leaning post ending in a panel.', 'Seitenansicht 2,2 x 4,4 mm: Sitzband, zwei Beinstriche, eine Querstrebe als Haarlinie, geneigter Pfosten mit Brett am Ende.'),
        retained=T('Seat band, both legs, stretcher, back post and the back panel as a filled bar at the top.', 'Sitzband, beide Beine, Querstrebe, Rückenpfosten und das Rückenbrett als gefüllter Balken oben.'),
        merged=T('Pad, apron and edge become one 0.3 mm band; the lower rail is absorbed into the post; the panel is one capsule.', 'Polster, Zarge und Kante werden ein 0,3 mm breites Band; die untere Leiste geht im Pfosten auf; das Brett ist eine Kapsel.'),
        omitted=T('Lower rail, finial, ferrules, feet, screws, rivets, knuckle, edge line, grain.', 'Untere Leiste, Knauf, Hülsen, Füße, Schrauben, Nieten, Knoten, Kantenlinie, Maserung.'),
        thinElements=T('Legs and post are single 0.25 mm strokes (50 mm real line weight versus 20-25 mm tubes: the unavoidable pen inflation); the 14 mm stretcher is a 0.13 mm detail stroke (kept because it makes the legs read as a frame).',
                       'Beine und Pfosten sind einzelne 0,25-mm-Striche (50 mm reale Linienstärke gegenüber 20-25-mm-Rohren: die unvermeidliche Stiftaufweitung); die 14-mm-Querstrebe ist ein 0,13-mm-Detailstrich (gehalten, weil sie die Beine als Gestell lesbar macht).'),
        hatching=T('None.', 'Keine.'),
        styles=T('Seat and panel mid tone (0.52-0.6), strokes black; body pen 0.25, detail 0.13.', 'Sitz und Brett mittlerer Ton (0,52-0,6), Striche schwarz; Körperstift 0,25, Detail 0,13.'),
        overlays=T('Seat band is seat-beech, panel and post back-beech, leg and stretcher strokes steel-tube (line colour).', 'Sitzband ist seat-beech, Brett und Pfosten back-beech, Bein- und Strebenstriche steel-tube (Linienfarbe).')))


def rep500():
    sh = Shapes(rig)
    sh.rect('seat-band', 'seat', .03, .36, .458, .46, z=4, tone=0.6, **KSE)
    sh.line('leg-rear', 'leg-rear', [REAR0, (0.0660, 0.42)], role='body', z=3, **KS)
    sh.line('leg-front', 'leg-front', [FRONT0, (0.4086, 0.42)], role='body', z=3, **KS)
    sh.line('back-post', 'back', [bp(0), bp(.418)], role='body', z=3.5, **KB)
    return rep(500, sh, dict(
        silhouette=T('Composed chair sign 0.9 x 1.8 mm: a seat bar, two leg strokes and one leaning back stroke; it reads as a chair only next to its table.', 'Zusammengesetztes Stuhlzeichen 0,9 x 1,8 mm: ein Sitzbalken, zwei Beinstriche und ein geneigter Rückenstrich; es liest sich als Stuhl nur neben seinem Tisch.'),
        retained=T('Overall height and depth, seat height, the splayed legs, the leaning back.', 'Gesamthöhe und -tiefe, Sitzhöhe, die gespreizten Beine, die geneigte Lehne.'),
        merged=T('Pad, apron and the back panel with its rail merge into the seat bar and the back stroke.', 'Polster, Zarge sowie Rückenbrett mit Leiste gehen in Sitzbalken und Rückenstrich auf.'),
        omitted=T('Stretcher (14 mm, 0.03 mm paper), panel, finial, all details.', 'Querstrebe (14 mm, 0,03 mm Papier), Brett, Knauf, alle Details.'),
        thinElements=T('The seat bar is thickened from 0.06 to 0.1 m (0.2 mm minimum feature, within the 0.09 m pen tolerance); legs and back post (20-25 mm) are single 0.18 mm strokes, i.e. 90 mm of real line weight; they cannot be made thinner and are not thickened further; the stretcher is dropped.',
                       'Der Sitzbalken wird von 0,06 auf 0,1 m verstärkt (0,2 mm Mindestmaß, innerhalb der Stifttoleranz 0,09 m); Beine und Rückenpfosten (20-25 mm) sind einzelne 0,18-mm-Striche, also 90 mm reale Linienstärke; dünner geht es nicht, weiter verstärkt wird nicht; die Querstrebe entfällt.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Seat bar mid tone, strokes black; one 0.18 mm weight.', 'Sitzbalken mittlerer Ton, Striche schwarz; eine Strichstärke 0,18 mm.'),
        overlays=T('The seat bar is seat-beech, the back stroke back-beech and the legs steel-tube, shown as line colour; no merged regions.', 'Der Sitzbalken ist seat-beech, der Rückenstrich back-beech und die Beine steel-tube, als Linienfarbe; keine verschmolzenen Regionen.')))


def thermal():
    be, st = MAT['beech'], MAT['steel']
    capb = effective_cap(be['rho'], be['cp'], be['k'], 0.028)
    capp = effective_cap(be['rho'], be['cp'], be['k'], 0.018)
    caps = effective_cap(st['rho'], st['cp'], st['k'], 0.002)
    lb = lambda t, c: layer('beech', T('Beech', 'Buche'), 'timber', 'beech-timber', t, be['k'], c, reference='iso-10456-2007')
    regions = [
        region(SEAT, T('Beech seat pad and apron', 'Buchensitz und Zarge'), 'surface', 'passive', 'beech-timber', 0.17, 0.42, 'horizontal-plate-up', 0.9, [lb(0.028, capb)], capb, reference='iso-10456-2007'),
        region(BACK, T('Beech back post, panel and rail', 'Buchen-Rückenpfosten, -brett und -leiste'), 'surface', 'passive', 'beech-timber', 0.1, 0.17, 'vertical-plate', 0.5, [lb(0.018, capp)], capp, reference='iso-10456-2007'),
        region(STEEL, T('Steel tubes', 'Stahlrohre'), 'surface', 'passive', 'steel-tube', 0.13, 0.022, 'cylinder', 0.6,
               [layer('tube-wall', T('Tube wall 22 x 2 mm', 'Rohrwand 22 x 2 mm'), 'metal', 'steel-tube', 0.002, st['k'], caps, reference='iso-10456-2007')], caps, reference='iso-10456-2007')]
    A['thermal'] = {'environment': {'airTempC': 20, 'meanRadiantTempC': 20, 'relativeHumidity': 0.5, 'windMS': 1.0, 'windDirectionDeg': 180, 'solarWM2': 600, 'diffuseFraction': 0.15, 'pressurePa': 101325},
                    'operating': [], 'regions': regions, 'emitters': [], 'assumptions': [
        T('Passive beech and steel surfaces in sun: absorbed radiation, long-wave exchange, convection and conduction with heat capacity; no emitters, no authored temperatures.', 'Passive Buchen- und Stahlflächen in der Sonne: absorbierte Strahlung, Langwellenaustausch, Konvektion und Leitung mit Wärmekapazität; keine Emitter, keine festgelegten Temperaturen.'),
        T('Thin tubes (2 mm wall) respond within minutes (7 kJ/m2K), the 28 mm beech pad within an hour (27 kJ/m2K): heat capacity is rho*cp*min(thickness, sqrt(alpha*3600 s)), values after ISO 10456.', 'Dünne Rohre (2 mm Wand) reagieren in Minuten (7 kJ/m2K), das 28 mm Buchenpolster in einer Stunde (27 kJ/m2K): Wärmekapazität ist rho*cp*min(Dicke, sqrt(alpha*3600 s)), Werte nach ISO 10456.'),
        T('A seated adult covers 0.12 m2 of the seat and 0.04 m2 of the back with clothing at about 30 degC (ISO 7730 clothing surface range) and shades them; these fractions are authored estimates.', 'Eine sitzende erwachsene Person bedeckt 0,12 m2 des Sitzes und 0,04 m2 der Lehne mit Bekleidung bei etwa 30 degC (Bekleidungsoberflächen nach ISO 7730) und beschattet sie; diese Anteile sind Schätzwerte.'),
        T('Near and far legs are one projected tube in the drawing but both count in the 0.13 m2 steel area.', 'Nahes und fernes Bein sind in der Zeichnung ein projiziertes Rohr, zählen aber beide zur Stahlfläche 0,13 m2.')]}


def bind(occ=False):
    seat = {'exposedFraction': 0.3 if occ else 1.0, 'sunlitFraction': 0.3 if occ else 1.0}
    back = {'exposedFraction': 0.6 if occ else 1.0, 'sunlitFraction': 0.45 if occ else 1.0}
    if occ:
        seat['contact'] = {'fraction': 0.7, 'tempC': 30}
        back['contact'] = {'fraction': 0.4, 'tempC': 30}
    return binding({SEAT: seat, BACK: back, STEEL: (1.0, 1.0 if not occ else 0.85)})


def finish():
    A['category'] = 'furniture'
    A['provenance']['author'] = 'Semio staffage content (CV-FO)'
    A['provenance']['references'] = [REF_WOOD, REF_EN1993, REF_ISO10456, REF_INCROPERA, REF_ISO7730, REF_EN1991]
    A['dimensions'] = {'heightM': 0.88, 'widthM': 0.44, 'depthM': 0.5, 'massKg': 4.5}
    A['materials'] = [
        {'id': 'beech-timber', 'title': T('Beech timber', 'Buchenholz'), 'emissivity': 0.9, 'solarAbsorptivity': 0.5, 'densityKgM3': 720, 'conductivityWmK': 0.18, 'specificHeatJKgK': 1600, 'source': 'authored-estimate', 'reference': 'iso-10456-2007'},
        {'id': 'steel-tube', 'title': T('Painted steel tube', 'Lackiertes Stahlrohr'), 'emissivity': 0.9, 'solarAbsorptivity': 0.7, 'densityKgM3': 7850, 'conductivityWmK': 50, 'specificHeatJKgK': 450, 'elasticModulusPa': 210000000000, 'source': 'authored-estimate', 'reference': 'iso-10456-2007'}]
    A['representations'] = {'500': rep500(), '200': rep200(), '100': rep100(), '50': rep50()}
    A['symbols'] = []
    A.pop('geometry', None)
    thermal()
    poses = {'upright-empty': bind(), 'pulled-out': bind(), 'tilted-on-two-legs': bind(), 'seated-pulled-out': bind(True), 'seated-occupied': bind(True)}
    for p in A['poses']:
        p['thermal'] = poses[p['id']]
    sit = next(c for c in A['clips'] if c['id'] == 'pull-out-and-sit')
    sit['description'] = T('The chair is pulled out, an adult sits down (75 kg seat load and 60 N back load ramp in while sitting), then draws up to the table; the occupant warms and shades seat and back.',
                           'Der Stuhl wird herausgezogen, eine erwachsene Person setzt sich (75 kg Sitzlast und 60 N Rückenlast steigen beim Hinsetzen an) und rückt an den Tisch heran; die Person erwärmt und beschattet Sitz und Rücken.')
    sit['tracks'] = [
        {'target': 'load:occupant', 'ease': 'smooth', 'keys': [{'t': 0, 'v': 0}, {'t': 0.6, 'v': 0}, {'t': 1.3, 'v': 1}, {'t': 2.8, 'v': 1}]},
        {'target': 'load:occupant-back', 'ease': 'smooth', 'keys': [{'t': 0, 'v': 0}, {'t': 1.3, 'v': 0}, {'t': 2.4, 'v': 1}, {'t': 2.8, 'v': 1}]},
        {'target': f'region:{SEAT}:sunlitFraction', 'ease': 'smooth', 'keys': [{'t': 0, 'v': 1}, {'t': 0.6, 'v': 1}, {'t': 1.3, 'v': 0.3}, {'t': 2.8, 'v': 0.3}]},
        {'target': f'region:{BACK}:sunlitFraction', 'ease': 'smooth', 'keys': [{'t': 0, 'v': 1}, {'t': 1.3, 'v': 1}, {'t': 2.4, 'v': 0.45}, {'t': 2.8, 'v': 0.45}]}]
    sit['events'] = [
        {'t': 1.3, 'kind': 'load-on', 'title': T('Occupant loads the seat', 'Person belastet den Sitz'), 'target': 'occupant'},
        {'t': 1.3, 'kind': 'thermal', 'title': T('Body starts to warm and shade the seat', 'Körper beginnt, den Sitz zu erwärmen und zu beschatten')},
        {'t': 2.4, 'kind': 'load-on', 'title': T('Occupant leans on the back', 'Person lehnt sich an die Lehne'), 'target': 'occupant-back'},
        {'t': 2.4, 'kind': 'motion', 'title': T('Chair reaches the table', 'Stuhl erreicht den Tisch')}]
    A['structure']['assumptions'] = A['structure']['assumptions'][:2]
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
        render(out['representations'][k], rig, int(k), f'chair-{k}-print', dpi=600)
        render(out['representations'][k], rig, int(k), f'chair-{k}-zoom', dpi=600, zoom=8 if k in ('500',) else 5 if k == '200' else 3)
