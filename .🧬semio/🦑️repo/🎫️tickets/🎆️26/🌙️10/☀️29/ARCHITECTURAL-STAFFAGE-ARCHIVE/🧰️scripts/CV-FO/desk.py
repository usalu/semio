"""height-adjustable-desk v2: front elevation, telescoping columns, motors and monitor as operating regions."""
import sys
from lib import *
from common import *

path, A = load('furniture', 'height-adjustable-desk')
rig = Rig(A['rig']['bones'])
TOP, STEEL, DL, DR, MON, LAP = 'top-board', 'steel-desk', 'drive-left', 'drive-right', 'monitor-housing', 'laptop-lid'
KT, KS, KM, KL = (dict(region=TOP, material='particleboard-laminate'), dict(region=STEEL, material='steel-desk'), dict(region=MON, material='equipment-housing'), dict(region=LAP, material='equipment-housing'))
COLS = [('l', 'foot-left', 'stage-left', 'column-left', 0.25, DL), ('r', 'foot-right', 'stage-right', 'column-right', 1.35, DR)]
OY = 0.695
HATCH = [{'id': 'board-grain', 'title': T('Laminate grain', 'Dekormaserung'), 'pattern': 'lines', 'angleDeg': 0, 'spacingMm': 0.6, 'penMm': 0.13}]


def kd(r):
    return dict(region=r, material='steel-desk')


def rep50():
    sh = Shapes(rig)
    for p, f, st, co, c, dr in COLS:
        sh.path(f'{p}-plate', f, [('M', c - .25, 0), ('L', c + .25, 0), ('L', c + .25, .008), ('C', c + .25, .016, c + .2, .016, c + .12, .016), ('L', c - .12, .016), ('C', c - .2, .016, c - .25, .016, c - .25, .008), ('Z',)], z=1, tone=0.3, **KS)
        sh.rect(f'{p}-sleeve', f, c - .032, .012, c + .032, .4, r=.004, z=3.5, tone=0.28, penMm=0.35, **KS)
        sh.rect(f'{p}-sleeve-collar', f, c - .036, .385, c + .036, .405, r=.003, role='detail', z=3.6, tone=0.18, **KS)
        sh.rect(f'{p}-drive', f, c - .05, .012, c + .05, .16, r=.012, z=3.7, tone=0.22, penMm=0.35, region=dr, material='equipment-housing')
        sh.line(f'{p}-drive-vent', f, [(c - .03, .12), (c + .03, .12)], z=3.8, region=dr, material='equipment-housing')
        sh.line(f'{p}-drive-vent-b', f, [(c - .03, .135), (c + .03, .135)], z=3.8, region=dr, material='equipment-housing')
        sh.rect(f'{p}-stage', st, c - .027, .05, c + .027, .55, z=3.0, tone=0.42, penMm=0.35, **KS)
        sh.line(f'{p}-stage-groove', st, [(c, .46), (c, .55)], z=3.1, **KS)
        sh.rect(f'{p}-stage-cap', st, c - .029, .538, c + .029, .55, r=.002, role='detail', z=3.2, tone=0.2, **KS)
        sh.rect(f'{p}-column', co, c - .022, .25, c + .022, OY, z=2.5, tone=0.55, penMm=0.35, **KS)
        sh.line(f'{p}-column-groove', co, [(c, .27), (c, .64)], z=2.6, **KS)
        sh.rect(f'{p}-bracket', co, c - .045, OY - .035, c + .045, OY, r=.004, z=2.7, tone=0.22, **KS)
    sh.rect('top-board', 'top', 0, OY, 1.6, OY + .025, r=.004, z=4, tone=0.58, hatch='board-grain', penMm=0.35, **KT)
    sh.line('top-edge', 'top', [(.004, OY + .0125), (1.596, OY + .0125)], z=4.1, **KT)
    sh.rect('top-rail', 'top', .25, OY - .075, 1.35, OY - .025, r=.003, z=3.5, tone=0.35, penMm=0.35, **KS)
    sh.line('top-rail-line', 'top', [(.28, OY - .05), (1.32, OY - .05)], z=3.6, **KS)
    sh.line('top-cable-tray', 'top', [(.55, OY - .075), (.55, OY - .115), (1.05, OY - .115), (1.05, OY - .075)], z=3.4, **KS)
    sh.rect('top-keypad', 'top', 1.42, OY - .02, 1.51, OY, r=.003, role='detail', z=4.3, tone=0.15, penMm=0.13, **KT)
    sh.rect('monitor-base', 'top', .51, OY + .025, .69, OY + .037, r=.005, z=5, tone=0.25, penMm=0.3, **KM)
    sh.poly('monitor-neck', 'top', [(.585, OY + .037), (.615, OY + .037), (.612, OY + .15), (.588, OY + .15)], z=5, tone=0.3, penMm=0.3, **KM)
    sh.rect('monitor-panel', 'top', .3, OY + .12, .9, OY + .485, r=.012, z=5.1, tone=0.2, penMm=0.4, **KM)
    sh.rect('monitor-screen', 'top', .315, OY + .135, .885, OY + .47, r=.004, role='glazing', z=5.2, tone=0.1, **KM)
    sh.line('monitor-bezel-logo', 'top', [(.57, OY + .127), (.63, OY + .127)], z=5.3, tone=0.4, **KM)
    sh.rect('laptop', 'top', .95, OY + .025, 1.2, OY + .045, r=.003, z=5, tone=0.4, penMm=0.3, **KL)
    sh.line('laptop-seam', 'top', [(.955, OY + .034), (1.195, OY + .034)], z=5.1, **KL)
    return rep(50, sh, dict(
        silhouette=T('Front elevation 32 x 23.6 mm: two T-foot lifting columns, each a foot sleeve with a motor pod, a middle stage and an inner column with bracket, a cross rail, the laminated top, and on it a monitor on its stand and a closed laptop.',
                     'Vorderansicht 32 x 23,6 mm: zwei Hubsäulen auf T-Füßen, je mit Fußhülse und Motorgehäuse, Mittelstufe und Innensäule mit Konsole, Querträger, die laminierte Platte und darauf ein Monitor mit Fuß und ein geschlossener Laptop.'),
        retained=T('Foot plates, the three telescoping stages with their grooves and collar, motor pods with vent slots, bracket, cross rail with line and cable tray, top with edge, keypad, monitor with stand, neck, bezel mark and glazed screen, laptop with seam.',
                   'Fußplatten, die drei Teleskopstufen mit Nuten und Kragen, Motorgehäuse mit Lüftungsschlitzen, Konsole, Querträger mit Linie und Kabelwanne, Platte mit Kante, Bedienfeld, Monitor mit Fuß, Hals, Markenzeichen und verglastem Schirm, Laptop mit Fuge.'),
        merged=T('Nothing merged; the nested tubes (0.064 / 0.054 / 0.044 m) are drawn with 0.35 mm outlines so the 0.1 mm steps show as line joints.', 'Nichts verschmolzen; die geschachtelten Rohre (0,064 / 0,054 / 0,044 m) haben 0,35-mm-Konturen, sodass die 0,1-mm-Stufen als Fugenlinien erscheinen.'),
        omitted=T('Foot glides (0.1 mm high), bolts and the sleeve seam (0.16 mm dots, 0.13 mm line next to a 0.35 mm outline), cable bundle, controller wire.', 'Fußgleiter (0,1 mm hoch), Bolzen und Hülsennaht (0,16-mm-Punkte, 0,13-mm-Linie neben 0,35-mm-Kontur), Kabelbündel, Steuerleitung.'),
        thinElements=T('The foot plate (0.016 m = 0.32 mm) is a 0.32 mm solid; vent slots, grooves, edge and seam are single 0.25 mm strokes; the monitor neck (0.024 m) is a 0.48 mm solid; the closed laptop (0.02 m) a 0.4 mm solid with a seam stroke.', 'Die Fußplatte (0,016 m = 0,32 mm) ist eine 0,32 mm dicke Fläche; Lüftungsschlitze, Nuten, Kante und Fuge sind einzelne 0,25-mm-Striche; der Monitorhals (0,024 m) ist eine 0,48 mm breite Fläche; der geschlossene Laptop (0,02 m) eine 0,4 mm dicke Fläche mit Fugenstrich.'),
        hatching=T('Laminate grain as horizontal lines every 0.6 mm only inside the 0.5 mm thick top (hatch fills the top once); the 0.025 m top is only 0.5 mm tall, so one grain line is visible at most.', 'Dekormaserung als waagrechte Linien alle 0,6 mm nur in der 0,5 mm dicken Platte (höchstens eine Linie sichtbar).'),
        styles=T('Top light (0.58), steel mid (0.28-0.55), drives dark (0.22), monitor dark (0.2) with a light glazed screen, laptop 0.4; body pen 0.35-0.4, detail 0.25.', 'Platte hell (0,58), Stahl mittel (0,28-0,55), Antriebe dunkel (0,22), Monitor dunkel (0,2) mit hellem Glasschirm, Laptop 0,4; Körperstift 0,35-0,4, Detail 0,25.'),
        overlays=T('Sleeves, stages, columns, brackets and rail = steel-desk; the pods are the operating regions drive-left/right (heat release while lifting); the monitor is the operating region monitor-housing; top = top-board; laptop = laptop-lid.', 'Hülsen, Stufen, Säulen, Konsolen und Träger = steel-desk; die Gehäuse sind die Betriebsregionen drive-left/right (Wärmeabgabe beim Heben); der Monitor ist die Betriebsregion monitor-housing; Platte = top-board; Laptop = laptop-lid.')),
        hatches=HATCH)


def rep100():
    sh = Shapes(rig)
    for p, f, st, co, c, dr in COLS:
        sh.line(f'{p}-plate', f, [(c - .25, .008), (c + .25, .008)], role='body', z=1, tone=0.3, **KS)
        sh.rect(f'{p}-sleeve', f, c - .032, .012, c + .032, .4, z=3.5, tone=0.28, penMm=0.25, **KS)
        sh.rect(f'{p}-drive', f, c - .05, .012, c + .05, .15, z=3.7, tone=0.22, region=dr, material='equipment-housing')
        sh.rect(f'{p}-stage', st, c - .027, .05, c + .027, .55, z=3.0, tone=0.42, penMm=0.25, **KS)
        sh.rect(f'{p}-column', co, c - .022, .25, c + .022, OY, z=2.5, tone=0.55, penMm=0.25, **KS)
        sh.rect(f'{p}-bracket', co, c - .045, OY - .035, c + .045, OY, z=2.7, tone=0.22, penMm=0.25, **KS)
    sh.rect('top-board', 'top', 0, OY, 1.6, OY + .025, z=4, tone=0.58, **KT)
    sh.rect('top-rail', 'top', .25, OY - .075, 1.35, OY - .025, z=3.5, tone=0.35, penMm=0.25, **KS)
    sh.line('top-cable-tray', 'top', [(.55, OY - .075), (.55, OY - .115), (1.05, OY - .115), (1.05, OY - .075)], z=3.4, **KS)
    sh.line('monitor-base', 'top', [(.52, OY + .031), (.68, OY + .031)], role='body', z=5, tone=0.25, penMm=0.25, **KM)
    sh.poly('monitor-neck', 'top', [(.585, OY + .031), (.615, OY + .031), (.613, OY + .15), (.587, OY + .15)], z=5, tone=0.3, **KM)
    sh.rect('monitor-panel', 'top', .3, OY + .12, .9, OY + .485, r=.012, z=5.1, tone=0.2, **KM)
    sh.rect('laptop', 'top', .95, OY + .025, 1.2, OY + .045, z=5, tone=0.4, penMm=0.25, **KL)
    return rep(100, sh, dict(
        silhouette=T('Front elevation 16 x 11.8 mm: two columns on foot lines with motor pods, a cross rail and cable tray under the top, the top, a monitor on a neck and a closed laptop.', 'Vorderansicht 16 x 11,8 mm: zwei Säulen auf Fußlinien mit Motorgehäusen, Querträger und Kabelwanne unter der Platte, die Platte, Monitor auf Hals und geschlossener Laptop.'),
        retained=T('Both columns as three nested tubes with 0.25 mm outlines (the stage steps still show as joints), motor pods, bracket, rail, cable tray, top, monitor panel, neck and stand line, laptop.', 'Beide Säulen als drei geschachtelte Rohre mit 0,25-mm-Konturen (die Stufen zeichnen sich als Fugen ab), Motorgehäuse, Konsole, Träger, Kabelwanne, Platte, Monitorpaneel, Hals und Fußlinie, Laptop.'),
        merged=T('Foot plates become single strokes; grooves, collars and the sleeve seam merge into the tube outlines; the monitor panel, bezel and screen become one dark panel.', 'Fußplatten werden einzelne Striche; Nuten, Kragen und Hülsennaht gehen in den Rohrkonturen auf; Monitorpaneel, Rahmen und Schirm werden ein dunkles Paneel.'),
        omitted=T('Keypad, vent slots, edge line, grain, bolts, glides, rail line, screen glazing, laptop seam.', 'Bedienfeld, Lüftungsschlitze, Kantenlinie, Maserung, Bolzen, Gleiter, Trägerlinie, Schirmverglasung, Laptopfuge.'),
        thinElements=T('The 0.016 m foot plate is a 0.35 mm stroke (35 mm real line weight); the 0.025 m top is a 0.25 mm solid with a 0.35 mm outline; the monitor stand (0.012 m) is one stroke; the laptop (0.02 m) is a 0.2 mm solid with 0.25 mm outline.', 'Die 0,016 m starke Fußplatte ist ein 0,35-mm-Strich (35 mm reale Linienstärke); die 0,025 m starke Platte ist eine 0,25 mm dicke Fläche mit 0,35-mm-Kontur; der Monitorfuß (0,012 m) ist ein Strich; der Laptop (0,02 m) eine 0,2 mm dicke Fläche mit 0,25-mm-Kontur.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Top 0.58, steel 0.28-0.55, drives and monitor dark; body pen 0.35 (parts 0.25), detail 0.18.', 'Platte 0,58, Stahl 0,28-0,55, Antriebe und Monitor dunkel; Körperstift 0,35 (Teile 0,25), Detail 0,18.'),
        overlays=T('Pods = drive-left/right (operating), panel and stand = monitor-housing (operating), top = top-board, rail, columns and plate lines = steel-desk, laptop = laptop-lid.', 'Gehäuse = drive-left/right (Betrieb), Paneel und Fuß = monitor-housing (Betrieb), Platte = top-board, Träger, Säulen und Fußlinien = steel-desk, Laptop = laptop-lid.')))


def rep200():
    sh = Shapes(rig)
    for p, f, st, co, c, dr in COLS:
        sh.line(f'{p}-plate', f, [(c - .25, .008), (c + .25, .008)], role='body', z=1, tone=0.3, **KS)
        sh.rect(f'{p}-sleeve', f, c - .035, .012, c + .035, .4, z=3.5, tone=0.28, penMm=0.13, **KS)
        sh.rect(f'{p}-drive', f, c - .06, .012, c + .06, .15, z=3.7, tone=0.22, penMm=0.13, region=dr, material='equipment-housing')
        sh.rect(f'{p}-stage', st, c - .03, .05, c + .03, .55, z=3.0, tone=0.42, penMm=0.13, **KS)
        sh.rect(f'{p}-column', co, c - .025, .25, c + .025, OY, z=2.5, tone=0.55, penMm=0.13, **KS)
    sh.rect('top-board', 'top', 0, OY, 1.6, OY + .03, z=4, tone=0.58, **KT)
    sh.rect('top-rail', 'top', .25, OY - .06, 1.35, OY - .01, z=3.5, tone=0.35, penMm=0.13, **KS)
    sh.line('monitor-neck', 'top', [(.6, OY + .03), (.6, OY + .14)], role='body', z=5, tone=0.3, **KM)
    sh.rect('monitor-panel', 'top', .3, OY + .12, .9, OY + .485, z=5.1, tone=0.2, **KM)
    sh.line('laptop', 'top', [(.95, OY + .04), (1.2, OY + .04)], role='body', z=5, tone=0.4, **KL)
    return rep(200, sh, dict(
        silhouette=T('Front elevation 8 x 5.9 mm: two columns with motor pods on foot lines, the top over a cross rail, a dark monitor panel on a stroke neck, a laptop stroke.', 'Vorderansicht 8 x 5,9 mm: zwei Säulen mit Motorgehäusen auf Fußlinien, die Platte über einem Querträger, ein dunkles Monitorpaneel auf Halsstrich, ein Laptopstrich.'),
        retained=T('Overall size, both columns as three nested stages, motor pods, the top with the rail beneath, monitor panel, neck, laptop.', 'Gesamtgröße, beide Säulen als drei geschachtelte Stufen, Motorgehäuse, die Platte mit dem Träger darunter, Monitorpaneel, Hals, Laptop.'),
        merged=T('The three nested stages are drawn with a light 0.13 mm outline (0.3 / 0.27 / 0.25 mm wide, the 0.05 mm steps cannot show); bracket, collar, groove and cable tray disappear into the column and rail.', 'Die drei geschachtelten Stufen haben eine leichte 0,13-mm-Kontur (0,3 / 0,27 / 0,25 mm breit, die 0,05-mm-Stufen sind nicht darstellbar); Konsole, Kragen, Nut und Kabelwanne gehen in Säule und Träger auf.'),
        omitted=T('Foot glides and bolts, grooves, collars, brackets, cable tray, edge, grain, keypad, monitor stand foot and bezel, laptop seam.', 'Fußgleiter und Bolzen, Nuten, Kragen, Konsolen, Kabelwanne, Kante, Maserung, Bedienfeld, Monitorfuß und Rahmen, Laptopfuge.'),
        thinElements=T('The foot plate (0.016 m) is a 0.25 mm stroke; the monitor neck (0.024 m) a single 0.25 mm stroke; the laptop (0.02 m) a single 0.25 mm stroke; the top is thickened from 0.025 to 0.03 m to reach 0.15 mm (within the 0.25 mm pen tolerance).', 'Die Fußplatte (0,016 m) ist ein 0,25-mm-Strich; der Monitorhals (0,024 m) ein einzelner 0,25-mm-Strich; der Laptop (0,02 m) ein einzelner 0,25-mm-Strich; die Platte wird von 0,025 auf 0,03 m verstärkt (0,15 mm, innerhalb der Stifttoleranz 0,25 mm).'),
        hatching=T('None.', 'Keine.'),
        styles=T('Top 0.58, steel 0.28-0.55, pods and monitor dark; body pen 0.25, part outlines 0.13.', 'Platte 0,58, Stahl 0,28-0,55, Gehäuse und Monitor dunkel; Körperstift 0,25, Teilkonturen 0,13.'),
        overlays=T('Pods = drive-left/right (operating), monitor panel = monitor-housing (operating), top = top-board, columns/rail/plate lines = steel-desk, laptop stroke = laptop-lid.', 'Gehäuse = drive-left/right (Betrieb), Monitorpaneel = monitor-housing (Betrieb), Platte = top-board, Säulen/Träger/Fußlinien = steel-desk, Laptopstrich = laptop-lid.')))


def rep500():
    sh = Shapes(rig)
    for p, f, st, co, c, dr in COLS:
        sh.line(f'{p}-plate', f, [(c - .25, .01), (c + .25, .01)], role='body', z=1, tone=0.3, **KS)
        sh.rect(f'{p}-drive', f, c - .06, 0, c + .06, .15, z=3.7, tone=0.22, region=dr, material='equipment-housing')
        sh.line(f'{p}-column', co, [(c, .15), (c, OY)], role='body', z=2.5, tone=0.4, **KS)
    sh.line('top-board', 'top', [(0, OY + .0125), (1.6, OY + .0125)], role='body', z=4, tone=0.58, **KT)
    sh.rect('top-rail', 'top', .25, OY - .06, 1.35, OY, z=3.5, tone=0.35, **KS)
    sh.rect('monitor-block', 'top', .3, OY + .025, .9, OY + .485, z=5.1, tone=0.2, **KM)
    sh.line('laptop', 'top', [(.95, OY + .04), (1.2, OY + .04)], role='body', z=5, tone=0.4, **KL)
    return rep(500, sh, dict(
        silhouette=T('Composed desk sign 3.2 x 2.4 mm: a top line on two column strokes with a solid monitor block above it; the leg space under the top (1.4 mm) stays open.', 'Zusammengesetztes Tischzeichen 3,2 x 2,4 mm: eine Plattenlinie auf zwei Säulenstrichen mit einem Monitorblock darüber; der Beinraum unter der Platte (1,4 mm) bleibt offen.'),
        retained=T('Overall width and height, top height (0.72 m), monitor block, motor pods at the column feet, the laptop on the top.', 'Gesamtbreite und -höhe, Plattenhöhe (0,72 m), Monitorblock, Motorgehäuse an den Säulenfüßen, der Laptop auf der Platte.'),
        merged=T('Sleeve, stage and column become one stroke each; monitor panel, neck and 0.1 m air gap to the top merge into one block (the gap is 0.2 mm, under the 0.28 mm minimum); rail and top join.', 'Hülse, Stufe und Säule werden je ein Strich; Monitorpaneel, Hals und 0,1 m Luft zur Platte werden ein Block (die Fuge ist 0,2 mm, unter dem Minimum 0,28 mm); Träger und Platte verbinden sich.'),
        omitted=T('Brackets, cable tray, keypad, stand foot, all details; foot plates are lines, not plates.', 'Konsolen, Kabelwanne, Bedienfeld, Standfuß, alle Details; Fußplatten sind Linien, keine Platten.'),
        thinElements=T('Columns (0.044-0.064 m) and the 0.025 m top are single 0.18 mm strokes (90 mm real line weight); foot plates (0.016 m) are strokes at ground level; the laptop (0.02 m) is a 0.5 mm long stroke on the top line.', 'Säulen (0,044-0,064 m) und die 0,025 m starke Platte sind einzelne 0,18-mm-Striche (90 mm reale Linienstärke); Fußplatten (0,016 m) sind Striche am Boden; der Laptop (0,02 m) ist ein 0,5 mm langer Strich auf der Plattenlinie.'),
        hatching=T('None.', 'Keine.'),
        styles=T('Monitor block dark (0.2), pods dark, strokes black at 0.18 mm; ground 0.25 mm.', 'Monitorblock dunkel (0,2), Gehäuse dunkel, Striche schwarz in 0,18 mm; Boden 0,25 mm.'),
        overlays=T('Monitor block = monitor-housing (operating), pods = drive-left/right (operating), top line = top-board, columns/foot/rail = steel-desk (line colour), laptop stroke = laptop-lid; no merged regions.', 'Monitorblock = monitor-housing (Betrieb), Gehäuse = drive-left/right (Betrieb), Plattenlinie = top-board, Säulen/Fuß/Träger = steel-desk (Linienfarbe), Laptopstrich = laptop-lid; keine verschmolzenen Regionen.')))


def thermal():
    st, pb = MAT['steel'], MAT['particleboard']
    c_st, c_pb = effective_cap(st['rho'], st['cp'], st['k'], 0.002), effective_cap(pb['rho'], pb['cp'], pb['k'], 0.025)
    pa = MAT['pa']
    c_pa = effective_cap(pa['rho'], pa['cp'], pa['k'], 0.003)
    c_drv = round(0.8 * 450 / 0.05)
    regions = [
        region(TOP, T('Laminated desk top', 'Laminierte Tischplatte'), 'surface', 'passive', 'particleboard-laminate', 1.28, 0.8, 'horizontal-plate-up', 0.5,
               [layer('laminate', T('Melamine laminate 1 mm', 'Melaminharzbeschichtung 1 mm'), 'polymer', 'particleboard-laminate', 0.001, 0.3, 1200, source='authored-estimate'), layer('board', T('Particleboard 25 mm', 'Spanplatte 25 mm'), 'timber', 'particleboard-laminate', 0.025, pb['k'], c_pb, reference='iso-10456-2007')], c_pb, reference='iso-10456-2007'),
        region(STEEL, T('Steel columns, feet and rail', 'Stahlsäulen, Füße und Träger'), 'surface', 'passive', 'steel-desk', 0.8, 0.06, 'vertical-plate', 0.4,
               [layer('tube', T('Steel tube wall 2 mm', 'Stahlrohrwand 2 mm'), 'metal', 'steel-desk', 0.002, st['k'], c_st, reference='iso-10456-2007')], c_st, reference='iso-10456-2007'),
        region(DL, T('Left lifting drive housing', 'Linkes Hubantriebsgehäuse'), 'engine', 'operating', 'equipment-housing', 0.05, 0.14, 'cylinder', 0.3,
               [layer('housing', T('Housing wall 3 mm with motor mass', 'Gehäusewand 3 mm mit Motormasse'), 'polymer', 'equipment-housing', 0.003, pa['k'], c_drv, source='authored-estimate')], c_drv, operating={'key': 'lift-drive', 'heatW': 35}),
        region(DR, T('Right lifting drive housing', 'Rechtes Hubantriebsgehäuse'), 'engine', 'operating', 'equipment-housing', 0.05, 0.14, 'cylinder', 0.3,
               [layer('housing', T('Housing wall 3 mm with motor mass', 'Gehäusewand 3 mm mit Motormasse'), 'polymer', 'equipment-housing', 0.003, pa['k'], c_drv, source='authored-estimate')], c_drv, operating={'key': 'lift-drive', 'heatW': 35}),
        region(MON, T('Monitor housing and screen', 'Monitorgehäuse und Schirm'), 'surface', 'operating', 'equipment-housing', 0.5, 0.45, 'vertical-plate', 0.5,
               [layer('housing', T('Plastic housing 3 mm', 'Kunststoffgehäuse 3 mm'), 'polymer', 'equipment-housing', 0.003, pa['k'], c_pa, source='authored-estimate')], c_pa, operating={'key': 'monitor', 'heatW': 25}),
        region(LAP, T('Closed laptop lid and case', 'Geschlossener Laptopdeckel und Gehäuse'), 'surface', 'passive', 'equipment-housing', 0.2, 0.3, 'horizontal-plate-up', 0.5,
               [layer('case', T('Aluminium case 1.5 mm', 'Aluminiumgehäuse 1,5 mm'), 'metal', 'equipment-housing', 0.0015, 160.0, effective_cap(2800, 880, 160.0, 0.0015), reference='iso-10456-2007')], effective_cap(2800, 880, 160.0, 0.0015), reference='iso-10456-2007')]
    A['thermal'] = {'environment': {'airTempC': 22, 'meanRadiantTempC': 22, 'relativeHumidity': 0.5, 'windMS': 0.1, 'windDirectionDeg': 180, 'solarWM2': 0, 'diffuseFraction': 1.0, 'pressurePa': 101325},
                    'operating': [
                        {'id': 'lift-drive', 'title': T('Lifting drives', 'Hubantriebe'), 'levels': [{'v': 0, 'title': T('Off', 'Aus')}, {'v': 1, 'title': T('Lifting at nominal load', 'Heben bei Nennlast')}]},
                        {'id': 'monitor', 'title': T('Monitor', 'Monitor'), 'levels': [{'v': 0, 'title': T('Off', 'Aus')}, {'v': 0.04, 'title': T('Standby (about 1 W)', 'Bereitschaft (etwa 1 W)')}, {'v': 1, 'title': T('On (25 W)', 'Ein (25 W)')}]}],
                    'regions': regions, 'emitters': [], 'assumptions': [
        T('Indoor reference case: no sun, 22 degC air and radiant temperature; no authored temperatures. Operating regions release heat into their own surface: the drives only while lifting (level 1), the monitor while on.', 'Innenraum-Referenzfall: keine Sonne, 22 degC Luft- und Strahlungstemperatur; keine festgelegten Temperaturen. Betriebsregionen geben Wärme an ihre eigene Oberfläche ab: die Antriebe nur beim Heben (Stufe 1), der Monitor im Betrieb.'),
        T('Each lifting drive is estimated at 60 W electrical input at nominal load (24 V linear actuator of a sit-stand desk) with a worm-gear efficiency of about 40 percent, leaving 35 W as heat in the housing (authored estimate, no manufacturer sheet).', 'Jeder Hubantrieb wird mit 60 W elektrischer Aufnahme bei Nennlast (24-V-Linearantrieb eines Sitz-Steh-Tischs) und einem Schneckengetriebe-Wirkungsgrad von etwa 40 Prozent geschätzt; es bleiben 35 W Wärme im Gehäuse (Schätzwert, kein Herstellerdatenblatt).'),
        T('The monitor converts its 25 W electrical input almost completely into heat at the housing (emitted light is a few watts, neglected); standby is 1 W (authored estimates for a 27-inch LED monitor).', 'Der Monitor wandelt seine elektrische Aufnahme von 25 W fast vollständig in Wärme am Gehäuse um (das abgestrahlte Licht beträgt wenige Watt und wird vernachlässigt); Bereitschaft 1 W (Schätzwerte für einen 27-Zoll-LED-Monitor).'),
        T('Heat capacity of the drive pods is a 0.8 kg steel-and-copper motor mass over 0.05 m2 (7.2 kJ/m2K); other capacities are rho*cp*min(thickness, sqrt(alpha*3600 s)) with ISO 10456 values.', 'Die Wärmekapazität der Antriebsgehäuse ist eine Motormasse von 0,8 kg Stahl und Kupfer auf 0,05 m2 (7,2 kJ/m2K); die übrigen Kapazitäten sind rho*cp*min(Dicke, sqrt(alpha*3600 s)) mit ISO-10456-Werten.')]}


def bind(drive, mon):
    return binding({TOP: (1.0, 0.0), STEEL: (1.0, 0.0), DL: (1.0, 0.0), DR: (1.0, 0.0), MON: (1.0, 0.0), LAP: (1.0, 0.0)}, operating={'lift-drive': drive, 'monitor': mon})


def finish():
    A['category'] = 'furniture'
    A['provenance']['author'] = 'Semio staffage content (CV-FO)'
    A['provenance']['references'] = [REF_ASHBY, REF_EN1993, REF_EN1991, REF_ISO10456, REF_INCROPERA]
    A['materials'] = [
        {'id': 'steel-desk', 'title': T('Painted steel', 'Lackierter Stahl'), 'emissivity': 0.9, 'solarAbsorptivity': 0.6, 'densityKgM3': 7850, 'conductivityWmK': 50, 'specificHeatJKgK': 450, 'elasticModulusPa': 210000000000, 'source': 'literature', 'reference': 'iso-10456-2007'},
        {'id': 'particleboard-laminate', 'title': T('Laminated particleboard', 'Laminierte Spanplatte'), 'emissivity': 0.9, 'solarAbsorptivity': 0.5, 'densityKgM3': 650, 'conductivityWmK': 0.13, 'specificHeatJKgK': 1700, 'source': 'literature', 'reference': 'iso-10456-2007'},
        {'id': 'equipment-housing', 'title': T('Plastic and aluminium housing', 'Kunststoff- und Aluminiumgehäuse'), 'emissivity': 0.9, 'solarAbsorptivity': 0.6, 'densityKgM3': 1100, 'source': 'authored-estimate'}]
    A['representations'] = {'500': rep500(), '200': rep200(), '100': rep100(), '50': rep50()}
    A['symbols'] = []
    A.pop('geometry', None)
    thermal()
    for p in A['poses']:
        p['thermal'] = bind(0, 1)
    lift = next(c for c in A['clips'] if c['id'] == 'raise-and-lower')
    lift.update({'durationS': 30.0, 'description': T('The top rises from sitting height (0.72 m) to standing height (1.15 m) at about 29 mm/s, then lowers again; both drives run at nominal load for the whole stroke and warm their housings.', 'Die Platte fährt mit etwa 29 mm/s von der Sitzhöhe (0,72 m) auf Stehhöhe (1,15 m) und wieder zurück; beide Antriebe laufen den ganzen Hub bei Nennlast und erwärmen ihre Gehäuse.')})
    lift['keys'] = [{'t': 0, 'pose': 'seated-height', 'ease': 'smooth'}, {'t': 15.0, 'pose': 'standing-height', 'ease': 'smooth'}, {'t': 30.0, 'pose': 'seated-height', 'ease': 'smooth'}]
    lift['contacts'] = [{'anchor': 'contact-foot-left', 'fromS': 0, 'toS': 30.0}, {'anchor': 'contact-foot-right', 'fromS': 0, 'toS': 30.0}]
    lift['phases'] = [{'id': 'raise', 'fromS': 0, 'toS': 15.0, 'title': T('Raising', 'Heben')}, {'id': 'lower', 'fromS': 15.0, 'toS': 30.0, 'title': T('Lowering', 'Senken')}]
    lift['events'] = [{'t': 0.0, 'kind': 'operating', 'title': T('Drives start', 'Antriebe starten'), 'target': 'lift-drive'}, {'t': 15.0, 'kind': 'motion', 'title': T('Standing height reached', 'Stehhöhe erreicht')}, {'t': 30.0, 'kind': 'shutdown', 'title': T('Drives stop', 'Antriebe stoppen'), 'target': 'lift-drive'}]
    lift['tracks'] = [{'target': 'operating:lift-drive', 'ease': 'step', 'keys': [{'t': 0, 'v': 1}, {'t': 29.9, 'v': 0}]}]
    A['structure']['assumptions'] = A['structure']['assumptions'][:1]
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
        render(out['representations'][k], rig, int(k), f'desk-{k}-print', dpi=600)
        render(out['representations'][k], rig, int(k), f'desk-{k}-zoom', dpi=600, zoom=6 if k == '500' else 4 if k == '200' else 2)
