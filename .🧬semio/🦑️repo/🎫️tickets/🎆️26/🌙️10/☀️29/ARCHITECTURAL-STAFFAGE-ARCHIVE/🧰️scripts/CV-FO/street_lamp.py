"""street-lamp v2: 6 m tapered column with LED luminaire; operating levels off / dimmed / rated, day and night states."""
import sys
from lib import *
from common import *

path, A = load('objects', 'street-lamp')
rig = Rig(A['rig']['bones'])
PLI, POL, ARM, HOU, LEN = 'plinth-concrete', 'pole-galv', 'arm-alu', 'lamp-housing', 'lamp-lens'
KPL, KPO, KAR, KHO, KLE = (dict(region=PLI, material='concrete'), dict(region=POL, material='galv-steel'), dict(region=ARM, material='aluminium-anodised'), dict(region=HOU, material='aluminium-anodised'), dict(region=LEN, material='polycarbonate'))
WH = lw(rig, 'head')
HATCH = [{'id': 'concrete-stipple', 'title': T('Concrete stipple', 'Betonpunktierung'), 'pattern': 'dots', 'angleDeg': 0, 'spacingMm': 0.8, 'penMm': 0.13}]
HOUSING = [('M', 0, .03), ('C', .01, .045, .03, .05, .05, .055), ('C', .12, .082, .25, .078, .30, .04), ('C', .325, .025, .335, 0, .33, -.02), ('L', .31, -.05), ('C', .20, -.062, .10, -.062, .02, -.05), ('C', .005, -.04, -.005, -.01, 0, .03), ('Z',)]
LENS = [('M', .03, -.05), ('C', .10, -.088, .24, -.088, .30, -.05), ('L', .30, -.046), ('C', .24, -.076, .10, -.076, .03, -.046), ('Z',)]
POLE = [('M', -.085, .12), ('C', -.08, 2.12, -.06, 4.12, -.04, 6.0), ('L', .04, 6.0), ('C', .06, 4.12, .08, 2.12, .085, .12), ('Z',)]


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
    skip = ('pole-hatch-lock', 'head-latch', 'seam')
    sh.items = v1_shapes(A, skip=skip)
    for s in sh.items:
        i = s['id']
        if i == 'plinth':
            s['hatch'] = 'concrete-stipple'
        if i in ('pole', 'arm', 'head-housing', 'plinth', 'arm-brace', 'base-cover'):
            s['penMm'] = 0.35
        if i.startswith('head-fin') or i in ('pole-seam-0', 'pole-seam-1', 'pole-highlight', 'arm-seam', 'plinth-joint'):
            s['penMm'] = 0.13
        if i.startswith('base-nut'):
            s['penMm'] = 0.13
    sh.ids = {s['id'] for s in sh.items}
    return rep(50, sh, dict(
        silhouette=T('Side elevation 24 x 120 mm: concrete plinth, base plate with flared cover, a 5.88 m tapered column (3.4 mm at the foot, 1.6 mm at the top) with inspection hatch, collar and cap, a braced aluminium arm and a finned luminaire housing with a glazed lens.',
                     'Seitenansicht 24 x 120 mm: Betonsockel, Fußplatte mit ausgestellter Abdeckung, eine 5,88 m hohe verjüngte Säule (3,4 mm am Fuß, 1,6 mm oben) mit Revisionsklappe, Manschette und Kappe, ein verstrebter Aluminiumarm und ein gerippt geformtes Leuchtengehäuse mit Glasabdeckung.'),
        retained=T('Plinth with joint, base plate with nuts, base cover, column with collar, cap, hatch, seams and highlight, arm with brace, joint, clamp and seam, luminaire housing with spigot, driver bulge, six fin marks, rim, and lens with reflector line.',
                   'Sockel mit Fuge, Fußplatte mit Muttern, Fußabdeckung, Säule mit Manschette, Kappe, Klappe, Nähten und Lichtkante, Arm mit Strebe, Gelenk, Schelle und Naht, Leuchtengehäuse mit Zapfen, Treiberwölbung, sechs Rippenmarken, Rand und Abdeckung mit Reflektorlinie.'),
        merged=T('Nothing merged; the inspection hatch and the column seams stay as lines on the column.', 'Nichts verschmolzen; die Revisionsklappe und die Säulennähte bleiben als Linien auf der Säule.'),
        omitted=T('Hatch lock (0.28 mm dot) and housing latch (0.24 mm dot), below the minimum feature; the cable and the foundation bolts below the plinth.', 'Klappenschloss (0,28-mm-Punkt) und Gehäuseverschluss (0,24-mm-Punkt), unter dem Mindestmaß; das Kabel und die Fundamentanker unter dem Sockel.'),
        thinElements=T('Seams, highlight, plinth joint and fin marks are single 0.13 mm strokes; the 0.024 m brace is a 0.48 mm capsule; the 0.034 m lens is a 0.7 mm glazed crescent; nuts are 0.5 x 0.3 mm rectangles with a 0.13 mm outline.', 'Nähte, Lichtkante, Sockelfuge und Rippenmarken sind einzelne 0,13-mm-Striche; die 0,024 m dicke Strebe ist eine 0,48 mm breite Kapsel; die 0,034 m dicke Abdeckung ist eine 0,7 mm breite verglaste Sichel; Muttern sind 0,5 x 0,3 mm Rechtecke mit 0,13-mm-Kontur.'),
        hatching=T('Concrete stipple (dots every 0.8 mm = 0.04 m) in the plinth only; the galvanised column and the anodised parts stay plain tone because their texture does not print.', 'Betonpunktierung (Punkte alle 0,8 mm = 0,04 m) nur im Sockel; die verzinkte Säule und die eloxierten Teile bleiben ohne Schraffur, weil ihre Textur nicht druckt.'),
        styles=T('Plinth light (0.65), column mid (0.4), arm 0.5, housing 0.5, lens light (0.85); body 0.35-0.5, detail 0.13-0.25.', 'Sockel hell (0,65), Säule mittel (0,4), Arm 0,5, Gehäuse 0,5, Abdeckung hell (0,85); Körper 0,35-0,5, Detail 0,13-0,25.'),
        overlays=T('Plinth = plinth-concrete, column and base parts = pole-galv, arm and brace = arm-alu, housing and fins = lamp-housing (operating: LED heat release by level), lens = lamp-lens.', 'Sockel = plinth-concrete, Säule und Fußteile = pole-galv, Arm und Strebe = arm-alu, Gehäuse und Rippen = lamp-housing (Betrieb: LED-Wärmeabgabe je Stufe), Abdeckung = lamp-lens.')),
        hatches=HATCH)


def rep100():
    sh = Shapes(rig)
    sh.rect('plinth', 'foundation', -.15, 0, .15, .12, z=1, tone=0.65, **KPL)
    sh.path('base-cover', 'foundation', [('M', -.12, .12), ('C', -.11, .2, -.085, .24, -.082, .30), ('L', .082, .30), ('C', .085, .24, .11, .2, .12, .12), ('Z',)], z=2.5, tone=0.33, **KPO)
    sh.path('pole', 'pole', POLE, z=3, tone=0.4, **KPO)
    sh.rect('pole-collar', 'pole', -.055, 5.72, .055, 5.80, r=.01, z=3.5, tone=0.3, role='detail', **KPO)
    sh.path('arm', 'arm', [('M', 0, 5.766), ('C', .35, 5.77, .8, 5.78, .95, 5.80), ('C', .8, 5.824, .35, 5.834, 0, 5.836), ('Z',)], z=2, tone=0.5, **KAR)
    sh.line('arm-brace', 'arm', [(.02, 5.34), (.62, 5.788)], role='detail', z=1.9, tone=0.45, **KAR)
    sh.path('head-housing', 'head', mapc(WH, HOUSING), z=4, tone=0.5, **KHO)
    sh.path('lens', 'head', mapc(WH, LENS), role='glazing', z=5, tone=0.85, **KLE)
    return rep(100, sh, dict(
        silhouette=T('Side elevation 12 x 60 mm: plinth with flared base cover, one tapered column (1.7 mm to 0.8 mm), a slim arm with a diagonal brace stroke, a luminaire housing with a glazed lens crescent.',
                     'Seitenansicht 12 x 60 mm: Sockel mit ausgestellter Fußabdeckung, eine verjüngte Säule (1,7 mm auf 0,8 mm), ein schlanker Arm mit diagonalem Strebenstrich, ein Leuchtengehäuse mit verglaster Abdeckungssichel.'),
        retained=T('Plinth, base cover, column with its taper, collar below the arm, arm, brace, housing, lens.', 'Sockel, Fußabdeckung, Säule mit Verjüngung, Manschette unter dem Arm, Arm, Strebe, Gehäuse, Abdeckung.'),
        merged=T('Plinth joint, base plate and nuts merge into the plinth top (0.2 mm); cap and column are one path; spigot, driver bulge and rim merge into the housing outline.', 'Sockelfuge, Fußplatte und Muttern gehen in der Sockeloberkante auf (0,2 mm); Kappe und Säule sind ein Pfad; Zapfen, Treiberwölbung und Rand gehen in der Gehäusekontur auf.'),
        omitted=T('Hatch, seams, highlight, plate nuts, clamp, arm joint, fins, rim, reflector line, spigot.', 'Klappe, Nähte, Lichtkante, Plattenmuttern, Schelle, Armgelenk, Rippen, Rand, Reflektorlinie, Zapfen.'),
        thinElements=T('The 0.024 m brace is a single 0.18 mm stroke; the 0.034 m lens is a 0.35 mm glazed crescent; the 0.07 m arm a 0.7 mm solid; fins (0.028 m spacing = 0.28 mm) are dropped.', 'Die 0,024 m dicke Strebe ist ein einzelner 0,18-mm-Strich; die 0,034 m dicke Abdeckung eine 0,35 mm breite verglaste Sichel; der 0,07 m starke Arm eine 0,7 mm Fläche; Rippen (0,028 m Abstand = 0,28 mm) entfallen.'),
        hatching=T('None; concrete stipple would need dots every 0.5 mm, which does not fit the 1.2 mm high plinth.', 'Keine; eine Betonpunktierung bräuchte Punkte alle 0,5 mm, was nicht in den 1,2 mm hohen Sockel passt.'),
        styles=T('Plinth 0.65, column 0.4, arm and housing 0.5, lens 0.85; body pen 0.35, detail 0.18, glazing 0.13.', 'Sockel 0,65, Säule 0,4, Arm und Gehäuse 0,5, Abdeckung 0,85; Körperstift 0,35, Detail 0,18, Glas 0,13.'),
        overlays=T('Plinth = plinth-concrete, column, cover and collar = pole-galv, arm and brace = arm-alu, housing = lamp-housing (operating), lens = lamp-lens; each region has its own shape.', 'Sockel = plinth-concrete, Säule, Abdeckung und Manschette = pole-galv, Arm und Strebe = arm-alu, Gehäuse = lamp-housing (Betrieb), Abdeckung = lamp-lens; jede Region hat eine eigene Form.')))


def rep200():
    sh = Shapes(rig)
    sh.rect('plinth', 'foundation', -.15, 0, .15, .12, z=1, tone=0.65, **KPL)
    sh.path('pole', 'pole', [('M', -.09, .12), ('C', -.085, 2.12, -.065, 4.12, -.045, 6.0), ('L', .045, 6.0), ('C', .065, 4.12, .085, 2.12, .09, .12), ('Z',)], z=3, tone=0.4, **KPO)
    sh.line('arm', 'arm', [(0, 5.80), (.82, 5.80)], role='body', z=2, tone=0.5, **KAR)
    sh.line('arm-brace', 'arm', [(.02, 5.34), (.62, 5.788)], role='detail', z=1.9, tone=0.45, **KAR)
    sh.path('head-housing', 'head', mapc(WH, [('M', 0, .03), ('C', .12, .082, .25, .078, .30, .04), ('L', .33, -.02), ('L', .31, -.05), ('L', .02, -.05), ('Z',)]), z=4, tone=0.5, **KHO)
    sh.line('lens', 'head', [WH(.04, -.058), WH(.29, -.058)], role='glazing', z=5, tone=0.85, **KLE)
    return rep(200, sh, dict(
        silhouette=T('Side elevation 6 x 30 mm: a plinth block, a single tapered column (0.9 mm to 0.45 mm), an arm stroke with a brace hairline, a wedge-shaped housing with a lens stroke under it.',
                     'Seitenansicht 6 x 30 mm: ein Sockelblock, eine einzelne verjüngte Säule (0,9 mm auf 0,45 mm), ein Armstrich mit Strebenhaarlinie, ein keilförmiges Gehäuse mit einem Abdeckungsstrich darunter.'),
        retained=T('Plinth, taper of the column, the arm and its brace, housing wedge and lens line; overall height 6.0 m and width 1.2 m.', 'Sockel, Verjüngung der Säule, der Arm und seine Strebe, Gehäusekeil und Abdeckungslinie; Gesamthöhe 6,0 m und Breite 1,2 m.'),
        merged=T('Base plate, cover, collar and cap merge into the column outline; arm clamp and joint merge into the arm stroke; spigot, rim and driver bulge merge into the housing wedge.', 'Fußplatte, Abdeckung, Manschette und Kappe gehen in der Säulenkontur auf; Armschelle und Gelenk gehen im Armstrich auf; Zapfen, Rand und Treiberwölbung gehen im Gehäusekeil auf.'),
        omitted=T('Hatch, seams, highlight, nuts, fins, latch, reflector, the plinth joint.', 'Klappe, Nähte, Lichtkante, Muttern, Rippen, Verschluss, Reflektor, die Sockelfuge.'),
        thinElements=T('The 0.07 m arm is a single 0.25 mm stroke (50 mm real weight); the 0.024 m brace a 0.13 mm stroke; the 0.034 m lens a 0.13 mm glazing stroke; the column top (0.08 m = 0.4 mm) is widened to 0.09 m (0.45 mm).', 'Der 0,07 m starke Arm ist ein einzelner 0,25-mm-Strich (50 mm reale Stärke); die 0,024 m dicke Strebe ein 0,13-mm-Strich; die 0,034 m dicke Abdeckung ein 0,13-mm-Glasstrich; das Säulenende (0,08 m = 0,4 mm) wird auf 0,09 m (0,45 mm) verbreitert.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Plinth 0.65, column 0.4, housing 0.5; body pen 0.25, brace 0.13, lens 0.13.', 'Sockel 0,65, Säule 0,4, Gehäuse 0,5; Körperstift 0,25, Strebe 0,13, Abdeckung 0,13.'),
        overlays=T('Plinth = plinth-concrete, column = pole-galv, arm and brace = arm-alu (line colour), housing = lamp-housing, lens stroke = lamp-lens; the operating colour of the housing is the main signal.', 'Sockel = plinth-concrete, Säule = pole-galv, Arm und Strebe = arm-alu (Linienfarbe), Gehäuse = lamp-housing, Abdeckungsstrich = lamp-lens; die Betriebsfarbe des Gehäuses ist das Hauptsignal.')))


def rep500():
    sh = Shapes(rig)
    sh.rect('plinth', 'foundation', -.15, 0, .15, .12, z=1, tone=0.65, **KPL)
    sh.poly('pole', 'pole', [(-.07, .12), (.07, .12), (.05, 6.0), (-.05, 6.0)], z=3, tone=0.4, **KPO)
    sh.line('arm', 'arm', [(0, 5.80), (.72, 5.80)], role='body', z=2, tone=0.5, **KAR)
    sh.rect('head', 'head', .72, 5.74, 1.05, 5.86, z=4, tone=0.5, regions=[HOU, LEN], material='aluminium-anodised')
    return rep(500, sh, dict(
        silhouette=T('Composed lamp sign 2.4 x 12 mm: a plinth, a slender tapered column (0.28 to 0.2 mm), a short arm stroke and one solid luminaire block at its end.', 'Zusammengesetztes Leuchtenzeichen 2,4 x 12 mm: ein Sockel, eine schlanke verjüngte Säule (0,28 auf 0,2 mm), ein kurzer Armstrich und ein Leuchtenblock an seinem Ende.'),
        retained=T('Height, plinth width, taper of the column, the cantilever of 1.05 m, the luminaire as a block.', 'Höhe, Sockelbreite, Verjüngung der Säule, die Auskragung von 1,05 m, die Leuchte als Block.'),
        merged=T('Housing, lens, spigot and fins become one block (0.66 x 0.24 mm) listing lamp-housing and lamp-lens; base plate, cover, collar and cap merge into the column; clamp, brace and arm form one stroke.', 'Gehäuse, Abdeckung, Zapfen und Rippen werden ein Block (0,66 x 0,24 mm) mit lamp-housing und lamp-lens; Fußplatte, Abdeckung, Manschette und Kappe gehen in der Säule auf; Schelle, Strebe und Arm bilden einen Strich.'),
        omitted=T('Brace (0.024 m), hatch, seams, nuts, fins - all below 0.1 m.', 'Strebe (0,024 m), Klappe, Nähte, Muttern, Rippen - alle unter 0,1 m.'),
        thinElements=T('The column (0.17 m at the foot, 0.08 m at the top) is a closed tapered polygon 0.14 to 0.10 m wide (0.28-0.2 mm): the foot is narrowed by 0.03 m and the top widened by 0.02 m, within the 0.09 m tolerance of the pen; the 0.07 m arm is a single 0.18 mm stroke.', 'Die Säule (0,17 m am Fuß, 0,08 m oben) ist ein geschlossenes verjüngtes Polygon von 0,14 bis 0,10 m Breite (0,28-0,2 mm): der Fuß wird um 0,03 m verengt, das Ende um 0,02 m verbreitert, innerhalb der Stifttoleranz von 0,09 m; der 0,07 m starke Arm ist ein einzelner 0,18-mm-Strich.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Plinth light, column mid, head block mid; one 0.18 mm outline; ground 0.25 mm.', 'Sockel hell, Säule mittel, Kopfblock mittel; eine 0,18-mm-Kontur; Boden 0,25 mm.'),
        overlays=T('The head block is a merged form painted with the area-weighted value of lamp-housing and lamp-lens (labelled merged); the operating heat of the housing dominates it.', 'Der Kopfblock ist eine verschmolzene Form, gefärbt mit dem flächengewichteten Wert von lamp-housing und lamp-lens (als verschmolzen gekennzeichnet); die Betriebswärme des Gehäuses dominiert.')))


def thermal():
    st, al, cc, pc = MAT['steel'], MAT['aluminium'], MAT['concrete'], MAT['polycarbonate']
    c_st, c_al, c_cc, c_pc = effective_cap(st['rho'], st['cp'], st['k'], 0.004), effective_cap(al['rho'], al['cp'], al['k'], 0.003), effective_cap(cc['rho'], cc['cp'], cc['k'], 0.1), effective_cap(pc['rho'], pc['cp'], pc['k'], 0.003)
    c_hou = round(2.5 * 880 / 0.25)
    regions = [
        region(POL, T('Galvanised column and base plate', 'Verzinkte Säule und Fußplatte'), 'surface', 'passive', 'galv-steel', 2.4, 0.13, 'cylinder', 0.5,
               [layer('tube', T('Galvanised steel tube wall 4 mm', 'Verzinkte Stahlrohrwand 4 mm'), 'metal', 'galv-steel', 0.004, st['k'], c_st, reference='iso-10456-2007')], c_st, reference='iso-10456-2007'),
        region(PLI, T('Concrete plinth', 'Betonsockel'), 'surface', 'passive', 'concrete', 0.5, 0.3, 'vertical-plate', 0.5,
               [layer('concrete', T('Concrete 100 mm exposed (thermally active 51 mm)', 'Beton 100 mm sichtbar (thermisch aktiv 51 mm)'), 'masonry', 'concrete', 0.1, cc['k'], c_cc, reference='iso-10456-2007')], c_cc, reference='iso-10456-2007'),
        region(ARM, T('Aluminium luminaire arm', 'Aluminium-Leuchtenarm'), 'surface', 'passive', 'aluminium-anodised', 0.18, 0.06, 'cylinder', 0.7,
               [layer('profile', T('Extruded aluminium wall 3 mm', 'Aluminium-Strangpressprofil 3 mm'), 'metal', 'aluminium-anodised', 0.003, al['k'], c_al, reference='iso-10456-2007')], c_al, reference='iso-10456-2007'),
        region(HOU, T('LED luminaire housing', 'LED-Leuchtengehäuse'), 'lamp', 'operating', 'aluminium-anodised', 0.25, 0.2, 'horizontal-plate-up', 0.7,
               [layer('housing', T('Die-cast aluminium housing with fins, 2.5 kg', 'Aluminium-Druckgussgehäuse mit Rippen, 2,5 kg'), 'metal', 'aluminium-anodised', 0.003, al['k'], c_hou, source='authored-estimate')], c_hou, operating={'key': 'lamp', 'heatW': 40}),
        region(LEN, T('Polycarbonate lens', 'Polycarbonat-Abdeckung'), 'glazing', 'passive', 'polycarbonate', 0.08, 0.2, 'horizontal-plate-down', 0.3,
               [layer('lens', T('Polycarbonate lens 3 mm', 'Polycarbonat-Abdeckung 3 mm'), 'polymer', 'polycarbonate', 0.003, pc['k'], c_pc, reference='iso-10456-2007')], c_pc, reference='iso-10456-2007')]
    A['thermal'] = {'environment': {'airTempC': 15, 'meanRadiantTempC': 15, 'relativeHumidity': 0.6, 'windMS': 1.0, 'windDirectionDeg': 0, 'solarWM2': 0, 'diffuseFraction': 0.0, 'pressurePa': 101325},
                    'operating': [{'id': 'lamp', 'title': T('LED luminaire', 'LED-Leuchte'), 'levels': [{'v': 0, 'title': T('Off', 'Aus')}, {'v': 0.5, 'title': T('Dimmed (night reduction, 50 percent)', 'Gedimmt (Nachtabsenkung, 50 Prozent)')}, {'v': 1, 'title': T('Rated (60 W, 40 W of heat)', 'Nennbetrieb (60 W, 40 W Wärme)')}]}],
                    'regions': regions,
                    'emitters': [{'id': 'lamp-plume', 'kind': 'plume', 'title': T('Warm luminaire plume', 'Warme Leuchtenfahne'), 'anchor': 'lamp-plume', 'directionDeg': 90, 'diameterM': 0.3, 'region': HOU, 'source': 'authored-estimate'}],
                    'assumptions': [
        T('Reference case: a clear still evening at 15 degC, no sun. The sun-lit day state is the pose day-off; all other poses are night states without sun. No temperature is authored; the housing temperature results from the heat balance.', 'Referenzfall: ein klarer ruhiger Abend bei 15 degC ohne Sonne. Der besonnte Tageszustand ist die Pose day-off; alle anderen Posen sind Nachtzustände ohne Sonne. Es wird keine Temperatur festgelegt; die Gehäusetemperatur ergibt sich aus der Wärmebilanz.'),
        T('The luminaire is estimated at 60 W electrical input (typical residential street luminaire). A white LED converts only a minor share into visible radiation, so about two thirds (40 W) leave as heat through the housing (Schubert 2006); the level scales the heat release (dimmed 0.5, rated 1). The 60 W and the share are authored estimates.', 'Die Leuchte wird mit 60 W elektrischer Aufnahme angesetzt (typische Wohnstraßenleuchte). Eine weiße LED wandelt nur einen kleineren Teil in sichtbare Strahlung um, sodass etwa zwei Drittel (40 W) als Wärme über das Gehäuse abgegeben werden (Schubert 2006); die Stufe skaliert die Wärmeabgabe (gedimmt 0,5, Nennbetrieb 1). Die 60 W und der Anteil sind Schätzwerte.'),
        T('The housing of 2.5 kg die-cast aluminium over 0.25 m2 has a heat capacity of 8.8 kJ/m2K: it needs minutes to warm up after ignition and to cool after switch-off (time constant of the order of 10 minutes), so the ignition clip runs 15 minutes.', 'Das Gehäuse aus 2,5 kg Aluminiumdruckguss über 0,25 m2 hat eine Wärmekapazität von 8,8 kJ/m2K: es braucht nach dem Einschalten und nach dem Ausschalten Minuten zum Erwärmen und Abkühlen (Zeitkonstante in der Größenordnung von 10 Minuten), daher dauert der Zündclip 15 Minuten.'),
        T('The plume emitter is the free convection above the warm housing; its strength follows the solved sensible heat of lamp-housing (binding 1 when lit or dimmed, 0 when off).', 'Der Fahnen-Emitter ist die freie Konvektion über dem warmen Gehäuse; seine Stärke folgt der berechneten fühlbaren Wärme von lamp-housing (Bindung 1 bei Betrieb oder gedimmt, 0 bei Aus).'),
        T('Heat capacities: rho*cp*min(thickness, sqrt(alpha*3600 s)) with ISO 10456 values (the 100 mm plinth: 51 mm thermally active).', 'Wärmekapazitäten: rho*cp*min(Dicke, sqrt(alpha*3600 s)) mit ISO-10456-Werten (der 100-mm-Sockel: 51 mm thermisch aktiv).')]}


def bind(level, sun, plume):
    return binding({POL: (1.0, sun), PLI: (1.0, sun), ARM: (1.0, sun), HOU: (1.0, sun), LEN: (1.0, 0.0)}, emitters={'lamp-plume': plume}, operating={'lamp': level})


def finish():
    A['category'] = 'objects'
    A['provenance']['author'] = 'Semio staffage content (CV-FO)'
    A['provenance']['references'] = [REF_EN1993, REF_EN1999, REF_EN1992, REF_EN1991_1_4, REF_ASHBY, REF_ISO10456, REF_INCROPERA, REF_SCHUBERT]
    A['materials'] = [
        {'id': 'galv-steel', 'title': T('Hot-dip galvanised steel', 'Feuerverzinkter Stahl'), 'emissivity': 0.28, 'solarAbsorptivity': 0.5, 'densityKgM3': 7850, 'conductivityWmK': 50, 'specificHeatJKgK': 450, 'elasticModulusPa': 210000000000, 'source': 'authored-estimate', 'reference': 'incropera-2018'},
        {'id': 'concrete', 'title': T('Concrete', 'Beton'), 'emissivity': 0.92, 'solarAbsorptivity': 0.6, 'densityKgM3': 2400, 'conductivityWmK': 1.65, 'specificHeatJKgK': 1000, 'source': 'literature', 'reference': 'iso-10456-2007'},
        {'id': 'aluminium-anodised', 'title': T('Anodised aluminium', 'Eloxiertes Aluminium'), 'emissivity': 0.8, 'solarAbsorptivity': 0.5, 'densityKgM3': 2700, 'conductivityWmK': 160, 'specificHeatJKgK': 880, 'elasticModulusPa': 70000000000, 'source': 'authored-estimate', 'reference': 'iso-10456-2007'},
        {'id': 'polycarbonate', 'title': T('Polycarbonate', 'Polycarbonat'), 'emissivity': 0.9, 'solarAbsorptivity': 0.3, 'densityKgM3': 1200, 'conductivityWmK': 0.2, 'specificHeatJKgK': 1200, 'source': 'literature', 'reference': 'iso-10456-2007'}]
    A['representations'] = {'500': rep500(), '200': rep200(), '100': rep100(), '50': rep50()}
    A['symbols'] = []
    A.pop('geometry', None)
    thermal()
    A['poses'] = [p for p in A['poses'] if p['id'] != 'day-off']
    by = {p['id']: p for p in A['poses']}
    by['dusk-off']['title'] = T('Dusk, off', 'Dämmerung, aus')
    by['dusk-off']['thermal'] = bind(0, 0.0, 0)
    by['lit-night']['thermal'] = bind(1, 0.0, 1)
    by['lit-in-wind']['thermal'] = bind(1, 0.0, 1)
    dim = {**by['lit-night'], 'id': 'dimmed-night', 'title': T('Dimmed night', 'Gedimmte Nacht'),
           'description': T('Night reduction: the luminaire runs at half power, the housing releases half the heat.', 'Nachtabsenkung: die Leuchte läuft mit halber Leistung, das Gehäuse gibt die halbe Wärme ab.'), 'thermal': bind(0.5, 0.0, 1)}
    day = {**by['dusk-off'], 'id': 'day-off', 'title': T('Day, off, in sun', 'Tag, aus, in der Sonne'),
           'description': T('Daytime with the luminaire off: the sun heats column, arm, plinth and the top of the housing; the lens on the underside stays shaded.', 'Tagsüber bei ausgeschalteter Leuchte: die Sonne erwärmt Säule, Arm, Sockel und die Gehäuseoberseite; die Abdeckung an der Unterseite bleibt beschattet.'), 'thermal': bind(0, 1.0, 0)}
    A['poses'] = [by['dusk-off'], by['lit-night'], dim, by['lit-in-wind'], day]
    for p in A['poses']:
        p['bones'] = by['lit-in-wind']['bones'] if p['id'] == 'lit-in-wind' else {}
    ign = next(c for c in A['clips'] if c['id'] == 'ignition-at-dusk')
    ign.update({'durationS': 900.0, 'fps': 2, 'description': T('At dusk the luminaire switches on at full level and its housing warms up over 13 minutes while a plume rises above it.', 'In der Dämmerung schaltet die Leuchte mit voller Stufe ein und ihr Gehäuse erwärmt sich über 13 Minuten, während über ihr eine Fahne aufsteigt.')})
    ign['keys'] = [{'t': 0, 'pose': 'dusk-off', 'ease': 'step'}, {'t': 120, 'pose': 'lit-night', 'ease': 'step'}, {'t': 900, 'pose': 'lit-night', 'ease': 'step'}]
    ign['contacts'] = [{'anchor': 'foundation-support', 'fromS': 0, 'toS': 900.0}]
    ign['phases'] = [{'id': 'dark', 'fromS': 0, 'toS': 120, 'title': T('Dark', 'Dunkel')}, {'id': 'warm-up', 'fromS': 120, 'toS': 900, 'title': T('Lit, housing warming up', 'Beleuchtet, Gehäuse erwärmt sich')}]
    ign['events'] = [{'t': 120, 'kind': 'ignition', 'title': T('Luminaire switches on', 'Leuchte schaltet ein'), 'target': 'lamp'}, {'t': 150, 'kind': 'thermal', 'title': T('Housing starts to warm up', 'Gehäuse beginnt sich zu erwärmen')}]
    ign['tracks'] = [{'target': 'operating:lamp', 'ease': 'step', 'keys': [{'t': 0, 'v': 0}, {'t': 120, 'v': 1}]}, {'target': 'emitter:lamp-plume', 'ease': 'step', 'keys': [{'t': 0, 'v': 0}, {'t': 120, 'v': 1}]}]
    A['structure']['assumptions'] = A['structure']['assumptions'][:2]
    A['presentation']['defaultScale'] = 100
    A['completion'] = 'authored'
    return sorted_keys(A)


if __name__ == '__main__':
    out = finish()
    for k in ('500', '200', '100', '50'):
        print(k, [round(v, 4) for v in bbox_world(rig, out['representations'][k]['geometry']['elevation'])])
    if '--write' in sys.argv:
        save(path, out)
    for k in ('500', '200', '100', '50'):
        r = out['representations'][k]
        render(r, rig, int(k), f'lamp-{k}-print', dpi=300)
        render(r, rig, int(k), f'lamp-{k}-head', dpi=600, zoom=6 if k in ('500',) else 4 if k == '200' else 2, clip=(0.0, 5.0, 1.2, 6.1))
