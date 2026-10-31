"""playground-swing v2: A-frame end elevation, chain with per-scale decisions, sun-heated rubber seat."""
import sys
from lib import *
from common import *

path, A = load('objects', 'playground-swing')
rig = Rig(A['rig']['bones'])
FRM, CHN, SEA = 'frame-galv', 'chain-steel', 'seat-rubber'
KF, KC, KS = (dict(region=FRM, material='steel-galvanised'), dict(region=CHN, material='steel-galvanised'), dict(region=SEA, material='rubber'))
WS = lw(rig, 'seat')
PLANK = [('M', -.21, -.06), ('C', -.21, -.045, -.20, -.038, -.18, -.04), ('L', .18, -.04), ('C', .20, -.038, .21, -.045, .21, -.06), ('C', .21, -.092, .19, -.098, .17, -.098), ('L', -.17, -.098), ('C', -.19, -.098, -.21, -.092, -.21, -.06), ('Z',)]


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


def plates(sh, pen=None):
    for p, b, x in (('left', 'ground', 0.0), ('right', 'ground', 2.6)):
        sh.rect(f'foot-{p}-plate', b, x - .09, 0, x + .09, .012, z=2.3, tone=0.25, **KF)


def seat_world(sh, z=4, simple=False):
    sh.path('seat-plank', 'seat', mapc(WS, PLANK), z=z, tone=0.25, **KS)


def rep50():
    sh = Shapes(rig)
    sh.items = v1_shapes(A, skip=('foot-left-bolt-0', 'foot-left-bolt-1', 'foot-right-bolt-0', 'foot-right-bolt-1', 'chain-left-rings', 'chain-left-bars', 'seat-bolt-l', 'seat-bolt-r', 'chain-left-texture'))
    for s in sh.items:
        i = s['id']
        if i in ('leg-left-tube', 'leg-right-tube', 'seat-plank', 'beam-end'):
            s['penMm'] = 0.35
        if i in ('brace-tube', 'foot-left-plate', 'foot-right-plate'):
            s['penMm'] = 0.3
        if i.startswith('chain-') and i.endswith('link'):
            s['penMm'] = 0.13
    sh.ids = {s['id'] for s in sh.items}
    for i in range(18):
        x = .075 + .1 * i
        if i % 2 == 0:
            sh._add(f'chain-left-link-{i}', 'chain-left', 'detail', 2.6, False, 0.2, {'kind': 'ellipse', 'cx': x, 'cy': 0, 'rx': .017, 'ry': .0075}, **KC)
        else:
            sh._add(f'chain-left-link-{i}', 'chain-left', 'detail', 2.6, True, 0.25, {'kind': 'rect', 'x': x - .019, 'y': -.0035, 'w': .038, 'h': .007}, **KC)
    return rep(50, sh, dict(
        silhouette=T('End elevation 55.6 x 47 mm: an A-frame of two splayed legs with foot plates, welds and clamps, a brace tube, the beam seen end-on with cap and hanger, a chain of alternating links and a rubber seat plank on a shackle.',
                     'Stirnansicht 55,6 x 47 mm: ein A-Rahmen aus zwei gespreizten Beinen mit Fußplatten, Schweißnähten und Schellen, ein Verbindungsrohr, der Holm im Schnitt mit Kappe und Aufhängung, eine Kette aus wechselnden Gliedern und ein Gummisitzbrett am Schäkel.'),
        retained=T('Legs with welds and clamps, foot plates, brace with both clamps, beam end with cap and hanger, 18 chain links (rings and edge-on bars alternating) with the chain line, shackle, seat plank with edge and tread strokes.', 'Beine mit Schweißnähten und Schellen, Fußplatten, Rohr mit beiden Schellen, Holmende mit Kappe und Aufhängung, 18 Kettenglieder (Ringe und hochkant gesehene Stege im Wechsel) mit der Kettenlinie, Schäkel, Sitzbrett mit Kanten- und Profilstrichen.'),
        merged=T('The second chain is hidden behind the first one in this view and is drawn once as a coincident capsule.', 'Die zweite Kette liegt in dieser Ansicht hinter der ersten und wird als deckungsgleiche Kapsel einmal gezeichnet.'),
        omitted=T('Foot and seat bolts (0.3 mm dots), the chain centre texture line.', 'Fuß- und Sitzbolzen (0,3-mm-Punkte), die Kettenmittellinie.'),
        thinElements=T('The 0.011 m chain is a 0.22 mm capsule with 0.13 mm outline plus link outlines of 0.68 x 0.3 mm; the brace (0.04 m) a 0.8 mm capsule; welds and tread are single 0.25-0.13 mm strokes.', 'Die 0,011 m starke Kette ist eine 0,22 mm breite Kapsel mit 0,13-mm-Kontur plus Gliedkonturen von 0,68 x 0,3 mm; das Rohr (0,04 m) eine 0,8 mm breite Kapsel; Schweißnähte und Profil sind einzelne 0,25-0,13-mm-Striche.'),
        hatching=T('None; the seat tread is three short 0.13 mm strokes, galvanised steel stays plain tone.', 'Keine; das Sitzprofil sind drei kurze 0,13-mm-Striche, verzinkter Stahl bleibt einfarbig.'),
        styles=T('Frame 0.4, brace 0.45, chain 0.3 with dark links, rubber seat dark (0.25); body pen 0.3-0.35, detail 0.13-0.25.', 'Rahmen 0,4, Rohr 0,45, Kette 0,3 mit dunklen Gliedern, Gummisitz dunkel (0,25); Körperstift 0,3-0,35, Detail 0,13-0,25.'),
        overlays=T('Legs, plates, brace, beam = frame-galv; chain, links, hanger, shackle = chain-steel; seat plank = seat-rubber (sun-heated, with the child in contact).', 'Beine, Platten, Rohr, Holm = frame-galv; Kette, Glieder, Aufhängung, Schäkel = chain-steel; Sitzbrett = seat-rubber (sonnenerwärmt, mit dem Kind im Kontakt).')))


def rep100():
    sh = Shapes(rig)
    sh.items = [s for s in v1_shapes(A, skip=('foot-left-bolt-0', 'foot-left-bolt-1', 'foot-right-bolt-0', 'foot-right-bolt-1', 'chain-left-rings', 'chain-left-bars', 'chain-left-texture', 'seat-bolt-l', 'seat-bolt-r',
                                              'leg-left-weld', 'leg-right-weld', 'leg-left-clamp', 'leg-right-clamp', 'brace-clamp-l', 'brace-clamp-r', 'beam-cap', 'beam-hanger', 'seat-shackle', 'seat-edge', 'seat-tread', 'chain-right-link', 'seat-plank', 'brace-tube'))]
    sh.ids = {s['id'] for s in sh.items}
    for s in sh.items:
        if s['id'].startswith('chain'):
            s['role'] = 'detail'
        if s['id'].startswith('foot'):
            s['penMm'] = 0.25
    sh.line('brace-tube', 'brace', [(.34, .6), (2.26, .6)], role='body', z=2.5, tone=0.45, **KF)
    sh.path('seat-plank', 'seat', mapc(WS, PLANK), z=4, tone=0.25, **KS)
    return rep(100, sh, dict(
        silhouette=T('End elevation 27.8 x 23.5 mm: an A-frame of two leg solids on foot plates, a brace stroke, the beam as a dot, a single chain stroke and a dark seat plank.', 'Stirnansicht 27,8 x 23,5 mm: ein A-Rahmen aus zwei Beinflächen auf Fußplatten, ein Rohrstrich, der Holm als Punkt, ein einzelner Kettenstrich und ein dunkles Sitzbrett.'),
        retained=T('Both legs with their chamfered foot, the foot plates, the brace, the beam end, the chain, the seat.', 'Beide Beine mit ihrem abgeschrägten Fuß, die Fußplatten, das Rohr, das Holmende, die Kette, der Sitz.'),
        merged=T('Welds, clamps, bolts, cap, hanger and shackle disappear into the leg and beam solids; the two chains are one stroke.', 'Schweißnähte, Schellen, Bolzen, Kappe, Aufhängung und Schäkel gehen in den Bein- und Holmflächen auf; die zwei Ketten sind ein Strich.'),
        omitted=T('All details of the 1:50 drawing, the chain links (0.034 m = 0.34 mm, spacing 1 mm: they would print as noise next to the 0.18 mm chain line), seat edge and tread.', 'Alle Details der Zeichnung 1:50, die Kettenglieder (0,034 m = 0,34 mm, Abstand 1 mm: sie würden neben der 0,18-mm-Kettenlinie als Rauschen drucken), Sitzkante und Profil.'),
        thinElements=T('The 0.011 m chain is one 0.18 mm stroke (18 mm real weight, the minimum to see it); the 0.04 m brace a 0.35 mm stroke; the 0.012 m foot plates 0.25 mm solids; legs are 0.6 mm solids.', 'Die 0,011 m starke Kette ist ein einzelner 0,18-mm-Strich (18 mm reale Stärke, das Minimum, um sie zu sehen); das 0,04 m starke Rohr ein 0,35-mm-Strich; die 0,012 m dicken Fußplatten 0,25 mm Flächen; Beine sind 0,6 mm Flächen.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Frame 0.4, brace and chain black strokes, seat dark; body pen 0.35, chain 0.18.', 'Rahmen 0,4, Rohr und Kette schwarze Striche, Sitz dunkel; Körperstift 0,35, Kette 0,18.'),
        overlays=T('Legs, plates, brace, beam dot = frame-galv; chain stroke = chain-steel (line colour); seat = seat-rubber.', 'Beine, Platten, Rohr, Holmpunkt = frame-galv; Kettenstrich = chain-steel (Linienfarbe); Sitz = seat-rubber.')))


def rep200():
    sh = Shapes(rig)
    sh.line('leg-left', 'leg-left', [(0, 0), (1.2995, 2.3)], role='body', z=2, tone=0.4, **KF)
    sh.line('leg-right', 'leg-right', [(2.6, 0), (1.3005, 2.3)], role='body', z=2, tone=0.4, **KF)
    plates(sh)
    sh.line('brace-tube', 'brace', [(.34, .6), (2.26, .6)], role='detail', z=2.5, **KF)
    sh.ell('beam-end', 'beam', 1.3, 2.3, .05, z=3, tone=0.38, **KF)
    sh.line('chain-left-link', 'chain-left', [(1.3, 2.25), (1.3, .4)], role='detail', z=2.5, **KC)
    sh.path('seat-plank', 'seat', mapc(WS, [('M', -.21, -.04), ('L', .21, -.04), ('L', .21, -.098), ('L', -.21, -.098), ('Z',)]), z=4, tone=0.25, **KS)
    return rep(200, sh, dict(
        silhouette=T('End elevation 13.9 x 11.8 mm: an A of two leg strokes with a brace hairline, a beam dot at the apex, a chain hairline and a dark seat bar.', 'Stirnansicht 13,9 x 11,8 mm: ein A aus zwei Beinstrichen mit einer Rohrhaarlinie, einem Holmpunkt an der Spitze, einer Kettenhaarlinie und einem dunklen Sitzbalken.'),
        retained=T('Frame geometry (leg splay, brace height), apex beam, foot plates as short strokes, chain, seat.', 'Rahmengeometrie (Beinspreizung, Rohrhöhe), Holm an der Spitze, Fußplatten als kurze Striche, Kette, Sitz.'),
        merged=T('Leg solids (0.06 m = 0.3 mm), welds and clamps become one stroke per leg; cap, hanger and beam become one dot; shackle and plank merge into the seat bar.', 'Beinflächen (0,06 m = 0,3 mm), Schweißnähte und Schellen werden ein Strich je Bein; Kappe, Aufhängung und Holm werden ein Punkt; Schäkel und Brett gehen im Sitzbalken auf.'),
        omitted=T('Links, bolts, weld lines, clamps, seat edge and tread, the hidden second chain.', 'Glieder, Bolzen, Schweißlinien, Schellen, Sitzkante und Profil, die verdeckte zweite Kette.'),
        thinElements=T('Legs (0.06 m) are single 0.25 mm strokes (50 mm real); the 0.04 m brace and 0.011 m chain are 0.13 mm hairlines (26 mm real weight for the chain: the minimum drawn).', 'Beine (0,06 m) sind einzelne 0,25-mm-Striche (50 mm real); das 0,04 m starke Rohr und die 0,011 m starke Kette sind 0,13-mm-Haarlinien (26 mm reale Stärke für die Kette: das gezeichnete Minimum).'),
        hatching=T('None.', 'Keine.'),
        styles=T('Strokes black, beam dot 0.38, seat dark (0.25); body pen 0.25, detail 0.13.', 'Striche schwarz, Holmpunkt 0,38, Sitz dunkel (0,25); Körperstift 0,25, Detail 0,13.'),
        overlays=T('Legs, plates, brace, beam = frame-galv, chain = chain-steel (line colour), seat = seat-rubber.', 'Beine, Platten, Rohr, Holm = frame-galv, Kette = chain-steel (Linienfarbe), Sitz = seat-rubber.')))


def rep500():
    sh = Shapes(rig)
    sh.line('leg-left', 'leg-left', [(0, 0), (1.2995, 2.3)], role='body', z=2, tone=0.4, **KF)
    sh.line('leg-right', 'leg-right', [(2.6, 0), (1.3005, 2.3)], role='body', z=2, tone=0.4, **KF)
    plates(sh)
    sh.line('brace-tube', 'brace', [(.34, .6), (2.26, .6)], role='body', z=2.5, **KF)
    sh.ell('beam-end', 'beam', 1.3, 2.3, .06, z=3, tone=0.38, **KF)
    sh.line('chain-left-link', 'chain-left', [(1.3, 2.25), (1.3, .4)], role='body', z=2.5, **KC)
    sh.path('seat-plank', 'seat', mapc(WS, [('M', -.21, -.04), ('L', .21, -.04), ('L', .21, -.098), ('L', -.21, -.098), ('Z',)]), z=4, tone=0.25, **KS)
    return rep(500, sh, dict(
        silhouette=T('Composed swing sign 5.6 x 4.7 mm: an A of two strokes with a crossbar, a beam dot at the apex, one chain stroke and a seat bar.', 'Zusammengesetztes Schaukelzeichen 5,6 x 4,7 mm: ein A aus zwei Strichen mit Querstrich, ein Holmpunkt an der Spitze, ein Kettenstrich und ein Sitzbalken.'),
        retained=T('Frame width and height, apex, crossbar, chain length, seat height (0.30 m above ground).', 'Rahmenbreite und -höhe, Spitze, Querstrich, Kettenlänge, Sitzhöhe (0,30 m über dem Boden).'),
        merged=T('Legs with welds and clamps are one stroke each; foot plates are 0.36 mm stubs; cap, hanger and beam are one dot; both chains and the shackle are one stroke; plank and edge are one bar.', 'Beine mit Schweißnähten und Schellen sind je ein Strich; Fußplatten sind 0,36-mm-Stummel; Kappe, Aufhängung und Holm sind ein Punkt; beide Ketten und der Schäkel sind ein Strich; Brett und Kante sind ein Balken.'),
        omitted=T('Links, bolts, clamps, welds, tread, the second chain - all below 0.05 m.', 'Glieder, Bolzen, Schellen, Schweißnähte, Profil, die zweite Kette - alle unter 0,05 m.'),
        thinElements=T('The 0.011 m chain is drawn as a 0.18 mm stroke (90 mm real, an 8-fold inflation that cannot be avoided without losing the swing); legs (0.06 m) and brace (0.04 m) are 0.18 mm strokes; the beam is thickened from r 0.05 to 0.06 m.', 'Die 0,011 m starke Kette wird als 0,18-mm-Strich gezeichnet (90 mm real, eine 8-fache Aufweitung, die sich ohne Verlust der Schaukel nicht vermeiden lässt); Beine (0,06 m) und Rohr (0,04 m) sind 0,18-mm-Striche; der Holm wird von r 0,05 auf 0,06 m verstärkt.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Black strokes, beam dot 0.38, seat bar dark; one 0.18 mm weight; ground 0.25 mm.', 'Schwarze Striche, Holmpunkt 0,38, Sitzbalken dunkel; eine Strichstärke 0,18 mm; Boden 0,25 mm.'),
        overlays=T('Legs, plates, brace, beam = frame-galv, chain = chain-steel (line colour), seat bar = seat-rubber; the swinging chain and seat stay separate bone-bound shapes.', 'Beine, Platten, Rohr, Holm = frame-galv, Kette = chain-steel (Linienfarbe), Sitzbalken = seat-rubber; die schwingende Kette und der Sitz bleiben getrennte, an Knochen gebundene Formen.')))


def thermal():
    st, ru = MAT['steel'], MAT['rubber']
    c_f, c_c, c_r = effective_cap(st['rho'], st['cp'], st['k'], 0.003), effective_cap(st['rho'], st['cp'], st['k'], 0.004), effective_cap(ru['rho'], ru['cp'], ru['k'], 0.03)
    A['thermal'] = {'environment': {'airTempC': 25, 'meanRadiantTempC': 30, 'relativeHumidity': 0.4, 'windMS': 2.0, 'windDirectionDeg': 180, 'solarWM2': 700, 'diffuseFraction': 0.15, 'pressurePa': 101325}, 'operating': [], 'emitters': [],
                    'regions': [
        region(FRM, T('Galvanised A-frame and beam', 'Verzinkter A-Rahmen und Holm'), 'surface', 'passive', 'steel-galvanised', 1.3, 0.05, 'cylinder', 0.6, [layer('tube', T('Galvanised tube wall 3 mm', 'Verzinkte Rohrwand 3 mm'), 'metal', 'steel-galvanised', 0.003, st['k'], c_f, reference='iso-10456-2007')], c_f, reference='iso-10456-2007'),
        region(CHN, T('Steel chains', 'Stahlketten'), 'surface', 'passive', 'steel-galvanised', 0.1, 0.01, 'cylinder', 0.6, [layer('link', T('Galvanised link bar 8 mm (4 mm active)', 'Verzinkter Kettenstab 8 mm (4 mm aktiv)'), 'metal', 'steel-galvanised', 0.004, st['k'], c_c, reference='iso-10456-2007')], c_c, reference='iso-10456-2007'),
        region(SEA, T('Rubber seat', 'Gummisitz'), 'surface', 'passive', 'rubber', 0.12, 0.28, 'horizontal-plate-up', 0.8, [layer('rubber', T('Moulded rubber 40 mm (28 mm active)', 'Formgummi 40 mm (28 mm aktiv)'), 'polymer', 'rubber', 0.03, ru['k'], c_r, reference='iso-10456-2007')], c_r, reference='iso-10456-2007')],
                    'assumptions': [
        T('Outdoor summer reference case: 700 W/m2 sun, 25 degC air, 2 m/s wind; passive surfaces, no emitters, no authored temperatures. The dark rubber seat (absorptivity 0.9) is the surface that heats most in the sun; its low conductivity and 32 kJ/m2K capacity make it a slow store.', 'Sommerlicher Außen-Referenzfall: 700 W/m2 Sonne, 25 degC Luft, 2 m/s Wind; passive Flächen, keine Emitter, keine festgelegten Temperaturen. Der dunkle Gummisitz (Absorptionsgrad 0,9) ist die Fläche, die sich in der Sonne am stärksten erwärmt; seine geringe Leitfähigkeit und 32 kJ/m2K Kapazität machen ihn zu einem trägen Speicher.'),
        T('The seated child (25 kg) covers about half of the seat with clothing at about 30 degC (ISO 7730 range) and shades it; the contact and shade fractions are authored estimates.', 'Das sitzende Kind (25 kg) bedeckt etwa die Hälfte des Sitzes mit Bekleidung bei etwa 30 degC (ISO-7730-Bereich) und beschattet ihn; Kontakt- und Schattenanteile sind Schätzwerte.'),
        T('Heat capacities rho*cp*min(thickness, sqrt(alpha*3600 s)) with ISO 10456 values; the second chain is hidden in the drawing but counts in the 0.1 m2 chain area.', 'Wärmekapazitäten rho*cp*min(Dicke, sqrt(alpha*3600 s)) mit ISO-10456-Werten; die zweite Kette ist in der Zeichnung verdeckt, zählt aber zur Kettenfläche von 0,1 m2.')]}


def bind():
    seat = {'exposedFraction': 0.5, 'sunlitFraction': 0.5, 'contact': {'fraction': 0.5, 'tempC': 30}}
    return binding({FRM: (1.0, 0.7), CHN: (1.0, 0.8), SEA: seat})


def finish():
    A['category'] = 'objects'
    A['dimensions'] = {'heightM': 2.35, 'widthM': 2.78, 'depthM': 1.2, 'massKg': 54.3}
    A['provenance']['author'] = 'Semio staffage content (CV-FO)'
    A['provenance']['references'] = [REF_EN1993, REF_ASHBY, REF_HALLIDAY, REF_ISO10456, REF_INCROPERA, REF_ISO7730]
    A['materials'] = [
        {'id': 'steel-galvanised', 'title': T('Galvanised steel', 'Verzinkter Stahl'), 'emissivity': 0.28, 'solarAbsorptivity': 0.5, 'densityKgM3': 7850, 'conductivityWmK': 50, 'specificHeatJKgK': 450, 'elasticModulusPa': 210000000000, 'source': 'authored-estimate', 'reference': 'incropera-2018'},
        {'id': 'rubber', 'title': T('Moulded rubber', 'Formgummi'), 'emissivity': 0.9, 'solarAbsorptivity': 0.9, 'densityKgM3': 1100, 'conductivityWmK': 0.25, 'specificHeatJKgK': 1000, 'source': 'literature', 'reference': 'iso-10456-2007'}]
    A['representations'] = {'500': rep500(), '200': rep200(), '100': rep100(), '50': rep50()}
    A['symbols'] = []
    A.pop('geometry', None)
    thermal()
    for p in A['poses']:
        p['thermal'] = bind()
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
        render(out['representations'][k], rig, int(k), f'swing-{k}-print', dpi=400 if k == '50' else 600, zoom=4 if k == '500' else 2 if k == '200' else 1)
