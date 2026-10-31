import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

LIT = 'authored-estimate'
refs = [
    ref('iwa-14-1', 'IWA 14-1:2013 Vehicle security barriers - Part 1: Performance requirement, vehicle impact test method and performance rating (context: this asset is a traffic-control bollard and carries no impact rating).'),
    ref('en-1991-1-7', 'EN 1991-1-7:2006 Eurocode 1: Actions on structures, Part 1-7: Accidental actions (impact on structures; the 1.5 kN nudge load here is far below any impact design value).'),
    ref('iso-10456-2007', 'ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel, cast iron, polymers).'),
    ref('incropera-2018', 'Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties, conduction to soil, cylinder convection).'),
    ref('bollard-class', 'Typical automatic rising bollards of 0.20 m diameter, 0.65 m raised height, electro-hydraulic drive with a 0.75 kW pump motor and a lift time of about 5 s from manufacturer datasheets of the product class; authored-estimate summary, no single datasheet.'),
]
mats = [
    material('steel-grey', 'Grey painted steel tube', 'Grau lackiertes Stahlrohr', 0.9, 0.6, 7850, 50, 450, 'literature', 'iso-10456-2007', 210e9),
    material('retro-film', 'Retro-reflective white film', 'Retroreflektierende weiße Folie', 0.9, 0.35, 1400, 0.2, 1200, 'authored-estimate', 'iso-10456-2007'),
    material('pc-amber', 'Amber polycarbonate lens', 'Bernsteinfarbene Polycarbonatlinse', 0.9, 0.7, 1200, 0.2, 1200, 'literature', 'iso-10456-2007'),
    material('cast-iron-frame', 'Painted cast-iron cover frame', 'Lackierter Gusseisen-Abdeckrahmen', 0.9, 0.7, 7200, 50, 500, 'literature', 'iso-10456-2007', 100e9),
    material('steel-sleeve', 'Galvanised steel sleeve', 'Verzinkte Stahlhülse', 0.28, 0.5, 7850, 50, 450, 'authored-estimate', 'incropera-2018', 210e9),
    material('steel-hyd', 'Painted steel power unit housing', 'Lackiertes Stahlgehäuse der Hydraulikeinheit', 0.9, 0.6, 7850, 50, 450, 'authored-estimate', 'incropera-2018'),
]
rig = Rig('mechanical', 'sleeve')
rig.bone('sleeve', None, 0, 0, 1.25, FIXED, T('Buried sleeve and cover frame (cut)', 'Eingegrabene Hülse und Abdeckrahmen (geschnitten)'), (42.0, 0, -0.6, LIT, 'bollard-class'))
rig.bone('hydraulic', 'sleeve', 0.2, -0.8, 0.4, FIXED, T('Hydraulic cylinder and power unit', 'Hydraulikzylinder und Antriebseinheit'), (32.0, 0.2, -0.8, LIT, 'bollard-class'))
rig.bone('shaft', 'sleeve', 0, 0, 1.15, prismatic(0, 0.65, 90), T('Rising shaft (rest = lowered)', 'Hubrohr (Ruhelage = abgesenkt)'), (38.0, 0, -0.55, LIT, 'bollard-class'))
anchors = [
    rig.anchor('sleeve-fixed', 'sleeve', 0, 0, 'support', T('Sleeve fixed in the concrete foundation at ground level', 'Hülse im Betonfundament in Geländehöhe eingespannt'), support='fixed', footprintLengthM=0.30, footprintWidthM=0.30),
    rig.anchor('push-point', 'shaft', 0, -0.10, 'load', T('Contact point 0.55 m above ground (raised)', 'Kontaktpunkt 0,55 m über Boden (ausgefahren)')),
    rig.anchor('beacon-glow', 'shaft', 0, -0.07, 'emitter', T('Beacon ring', 'Leuchtring')),
]
CAP = 'M -0.10 -0.09 L -0.10 -0.05 C -0.10 -0.01 -0.05 0 0 0 C 0.05 0 0.10 -0.01 0.10 -0.05 L 0.10 -0.09 Z'


def sleeve_stroke(s, x, pen, z=2.2):
    s.line('sleeve', 'sleeve', [(-x, 0.0), (-x, -1.25), (x, -1.25), (x, 0.0)], z=z, region='sleeve-steel', pen=pen)


def r500():
    s = Shapes(rig)
    s.rect('shaft', 'shaft', -0.10, -1.15, 0.10, -0.09, z=3.5, tone=0.5, regions=['shaft-steel', 'reflector-band'], material='steel-grey')
    s.path('cap', 'shaft', CAP.replace('-0.09', '-0.13'), z=3.6, tone=0.5, regions=['shaft-steel', 'beacon-led'], material='steel-grey')
    sleeve_stroke(s, 0.10, 0.13)
    s.rect('frame-l', 'sleeve', -0.21, -0.11, -0.10, 0.02, z=3.0, tone=0.6, region='cover-frame', material='cast-iron-frame')
    s.rect('frame-r', 'sleeve', 0.10, -0.11, 0.21, 0.02, z=3.0, tone=0.6, region='cover-frame', material='cast-iron-frame')
    s.rect('power-unit', 'hydraulic', 0.10, -0.95, 0.36, -0.55, z=3.0, tone=0.5, region='hydraulic-unit', material='steel-hyd')
    return rep(500, 350, 1000, pens(0.13, None, None, None, 0.18), s, [
        ('A section sign 1.1 mm wide and 3.8 mm high: a solid shaft (0.4 mm wide) rising 1.3 mm above the ground line, a sleeve outline 2.5 mm below it, two frame blocks at ground level and the power unit block at the right.', 'Ein Schnittzeichen, 1,1 mm breit und 3,8 mm hoch: ein voller Schaft (0,4 mm breit), der 1,3 mm über die Bodenlinie ragt, eine Hülsenkontur 2,5 mm darunter, zwei Rahmenblöcke in Geländehöhe und der Antriebsblock rechts.'),
        ('The 0.20 m shaft at true width, the true raised height 0.65 m and buried depth 1.25 m, the power unit at 0.26 x 0.40 m.', 'Der 0,20 m dicke Schaft in wahrer Breite, die wahre Ausfahrhöhe 0,65 m und Einbautiefe 1,25 m, die Antriebseinheit mit 0,26 x 0,40 m.'),
        ('Reflective band, beacon ring and shaft tube are one solid; the cylinder in the sleeve and the hose are omitted from this scale; the sleeve wall coincides with the shaft edges.', 'Reflexband, Leuchtring und Schaftrohr sind ein Körper; Zylinder in der Hülse und Schlauch fehlen in diesem Maßstab; die Hülsenwand fällt mit den Schaftkanten zusammen.'),
        ('Cylinder, hose, bolts, flange, lens details.', 'Zylinder, Schlauch, Schrauben, Flansch, Linsendetails.'),
        ('The 0.012 m sleeve wall is a single 0.13 mm line on the shaft edge; the 0.055 m cover frame is thickened to 0.12 m (0.24 mm) inward to the shaft edge; the 0.03 m shaft-sleeve clearance is omitted.', 'Die 0,012 m dicke Hülsenwand ist eine einzelne 0,13-mm-Linie auf der Schaftkante; der 0,055 m breite Abdeckrahmen ist auf 0,12 m (0,24 mm) bis zur Schaftkante verbreitert; das Spiel Schaft-Hülse von 0,03 m entfällt.'),
        ('None.', 'Keine.'),
        ('Shaft 0.5, frame 0.6, power unit 0.5; body pen 0.13 mm, ground 0.18 mm.', 'Schaft 0,5, Rahmen 0,6, Antriebseinheit 0,5; Körperstift 0,13 mm, Boden 0,18 mm.'),
        ('The shaft solid lists shaft-steel and reflector-band, the cap shaft-steel and beacon-led; the sleeve line carries sleeve-steel.', 'Der Schaftkörper nennt shaft-steel und reflector-band, die Kappe shaft-steel und beacon-led; die Hülsenlinie trägt sleeve-steel.'),
    ])


def r200():
    s = Shapes(rig)
    s.rect('shaft', 'shaft', -0.10, -1.15, 0.10, -0.25, z=3.5, tone=0.5, region='shaft-steel', material='steel-grey')
    s.rect('reflector', 'shaft', -0.10, -0.25, 0.10, -0.13, z=3.55, tone=0.08, region='reflector-band', material='retro-film')
    s.path('cap', 'shaft', CAP.replace('-0.09', '-0.13'), z=3.6, tone=0.5, regions=['shaft-steel', 'beacon-led'], material='steel-grey')
    sleeve_stroke(s, 0.10, 0.13)
    s.rect('frame-l', 'sleeve', -0.21, -0.08, -0.10, 0.015, z=3.0, tone=0.6, region='cover-frame', material='cast-iron-frame')
    s.rect('frame-r', 'sleeve', 0.10, -0.08, 0.21, 0.015, z=3.0, tone=0.6, region='cover-frame', material='cast-iron-frame')
    s.rect('power-unit', 'hydraulic', 0.10, -0.95, 0.36, -0.55, z=3.0, tone=0.5, region='hydraulic-unit', material='steel-hyd')
    s.rect('ram', 'hydraulic', -0.06, -1.25, 0.06, -0.70, z=2.4, tone=0.7, region='hydraulic-unit', material='steel-hyd')
    return rep(200, 150, 350, pens(0.18, 0.13, None, None, 0.25), s, [
        ('A section 2.8 mm wide and 9.5 mm high: shaft 1 mm wide with a white band and a dark cap, the sleeve outline, two frame blocks, the power unit block and the cylinder in the sleeve.', 'Ein Schnitt 2,8 mm breit und 9,5 mm hoch: Schaft 1 mm breit mit weißem Band und dunkler Kappe, die Hülsenkontur, zwei Rahmenblöcke, der Antriebsblock und der Zylinder in der Hülse.'),
        ('Shaft, reflective band, cap with beacon ring, sleeve, frame, power unit, ram at true size and position.', 'Schaft, Reflexband, Kappe mit Leuchtring, Hülse, Rahmen, Antriebseinheit, Zylinder in wahrer Größe und Lage.'),
        ('Beacon ring merges into the cap; the sleeve wall coincides with the shaft edges.', 'Der Leuchtring geht in der Kappe auf; die Hülsenwand fällt mit den Schaftkanten zusammen.'),
        ('Hose, bolts, flange, lens, brush seal.', 'Schlauch, Schrauben, Flansch, Linse, Bürstendichtung.'),
        ('Sleeve wall is a 0.13 mm line (real 0.012 m = 0.06 mm); the frame blocks are 0.11 m (0.55 mm) wide, real 0.055 m, up to the shaft edge.', 'Die Hülsenwand ist eine 0,13-mm-Linie (real 0,012 m = 0,06 mm); die Rahmenblöcke sind 0,11 m (0,55 mm) breit, real 0,055 m, bis zur Schaftkante.'),
        ('None.', 'Keine.'),
        ('Shaft 0.5, band 0.08 (white), frame 0.6, ram 0.7; body pen 0.18 mm, detail 0.13 mm.', 'Schaft 0,5, Band 0,08 (weiß), Rahmen 0,6, Zylinder 0,7; Körperstift 0,18 mm, Detail 0,13 mm.'),
        ('Cap lists shaft-steel and beacon-led; the ram carries hydraulic-unit like the power block.', 'Die Kappe nennt shaft-steel und beacon-led; der Zylinder trägt hydraulic-unit wie der Antriebsblock.'),
    ])


def r100():
    s = Shapes(rig)
    s.rect('shaft', 'shaft', -0.10, -1.15, 0.10, -0.24, z=3.5, tone=0.5, region='shaft-steel', material='steel-grey')
    s.rect('reflector', 'shaft', -0.10, -0.24, 0.10, -0.13, z=3.55, tone=0.08, region='reflector-band', material='retro-film')
    s.rect('beacon', 'shaft', -0.10, -0.13, 0.10, -0.07, z=3.6, tone=0.7, region='beacon-led', material='pc-amber')
    s.path('cap', 'shaft', 'M -0.10 -0.07 C -0.10 -0.02 -0.05 0 0 0 C 0.05 0 0.10 -0.02 0.10 -0.07 Z', z=3.65, tone=0.5, region='shaft-steel', material='steel-grey')
    sleeve_stroke(s, 0.15, 0.13)
    s.rect('frame-l', 'sleeve', -0.20, -0.07, -0.145, 0.015, z=3.0, tone=0.6, region='cover-frame', material='cast-iron-frame')
    s.rect('frame-r', 'sleeve', 0.145, -0.07, 0.20, 0.015, z=3.0, tone=0.6, region='cover-frame', material='cast-iron-frame')
    s.rect('power-unit', 'hydraulic', 0.15, -0.95, 0.36, -0.55, z=3.0, tone=0.5, region='hydraulic-unit', material='steel-hyd')
    s.line('hose', 'hydraulic', [(0.15, -0.70), (0.06, -0.70)], role='detail', z=3.1, region='hydraulic-unit')
    s.rect('ram', 'hydraulic', -0.06, -1.25, 0.06, -0.70, z=2.4, tone=0.7, region='hydraulic-unit', material='steel-hyd')
    s.line('motor-seam', 'hydraulic', [(0.15, -0.80), (0.36, -0.80)], role='detail', z=3.2, region='hydraulic-unit')
    return rep(100, 75, 150, pens(0.25, 0.13, None, None, 0.35), s, [
        ('A section 5.6 mm wide and 19 mm high: shaft 2 mm wide with white band, amber beacon band and domed cap, the sleeve outline 3 mm wide, two frame blocks, the power unit with a motor seam and a hose to the ram.', 'Ein Schnitt 5,6 mm breit und 19 mm hoch: Schaft 2 mm breit mit weißem Band, bernsteinfarbenem Leuchtband und gewölbter Kappe, die Hülsenkontur 3 mm breit, zwei Rahmenblöcke, die Antriebseinheit mit Motornaht und Schlauch zum Zylinder.'),
        ('All parts at true size except the hairline sleeve wall.', 'Alle Teile in wahrer Größe außer der haarfeinen Hülsenwand.'),
        ('Lens, ring and seal of the beacon merge into one band.', 'Linse, Ring und Dichtung des Leuchtrings gehen in einem Band auf.'),
        ('Bolts, flange, brush seal, power cable.', 'Schrauben, Flansch, Bürstendichtung, Stromkabel.'),
        ('The sleeve wall is a 0.13 mm line (real 0.06 mm); the hose a 0.13 mm line.', 'Die Hülsenwand ist eine 0,13-mm-Linie (real 0,06 mm); der Schlauch eine 0,13-mm-Linie.'),
        ('None.', 'Keine.'),
        ('Shaft 0.5, band 0.08, beacon 0.7, frame 0.6; body pen 0.25 mm, detail 0.13 mm.', 'Schaft 0,5, Band 0,08, Leuchtring 0,7, Rahmen 0,6; Körperstift 0,25 mm, Detail 0,13 mm.'),
        ('Each part carries its own region; the power unit, ram and hose carry hydraulic-unit.', 'Jedes Teil trägt seine eigene Region; Antriebseinheit, Zylinder und Schlauch tragen hydraulic-unit.'),
    ])


def r50():
    s = Shapes(rig)
    s.rect('shaft', 'shaft', -0.10, -1.15, 0.10, -0.24, z=3.5, tone=0.5, region='shaft-steel', material='steel-grey')
    s.line('shaft-highlight', 'shaft', [(-0.065, -1.10), (-0.065, -0.28)], role='texture', z=3.7, region='shaft-steel')
    s.rect('reflector', 'shaft', -0.10, -0.24, 0.10, -0.13, z=3.55, tone=0.08, region='reflector-band', material='retro-film', hatch='chevron')
    s.rect('beacon', 'shaft', -0.10, -0.13, 0.10, -0.07, z=3.6, tone=0.7, region='beacon-led', material='pc-amber')
    for i, x in enumerate((-0.06, 0.0, 0.06)):
        s.ell(f'led-{i}', 'shaft', x, -0.10, 0.01, z=3.7, tone=0.1, role='detail', region='beacon-led')
    s.path('cap', 'shaft', 'M -0.10 -0.07 C -0.10 -0.02 -0.05 0 0 0 C 0.05 0 0.10 -0.02 0.10 -0.07 Z', z=3.65, tone=0.5, region='shaft-steel', material='steel-grey')
    s.line('cap-seam', 'shaft', [(-0.09, -0.05), (0.09, -0.05)], role='texture', z=3.8, region='shaft-steel')
    sleeve_stroke(s, 0.15, 0.25)
    s.rect('frame-l', 'sleeve', -0.20, -0.07, -0.145, 0.015, z=3.0, tone=0.6, region='cover-frame', material='cast-iron-frame')
    s.rect('frame-r', 'sleeve', 0.145, -0.07, 0.20, 0.015, z=3.0, tone=0.6, region='cover-frame', material='cast-iron-frame')
    for i, x in enumerate((-0.175, 0.175)):
        s.ell(f'frame-bolt-{i}', 'sleeve', x, -0.02, 0.01, z=3.1, tone=0.2, role='detail', region='cover-frame')
    s.line('brush-seal', 'sleeve', [(-0.10, -0.015), (-0.145, -0.015)], role='texture', z=3.1, region='cover-frame')
    s.rect('power-unit', 'hydraulic', 0.15, -0.95, 0.36, -0.55, r=0.01, z=3.0, tone=0.5, region='hydraulic-unit', material='steel-hyd')
    s.line('hose', 'hydraulic', [(0.15, -0.70), (0.06, -0.70)], role='detail', z=3.1, region='hydraulic-unit')
    s.line('motor-seam', 'hydraulic', [(0.15, -0.80), (0.36, -0.80)], role='detail', z=3.2, region='hydraulic-unit')
    s.ell('motor', 'hydraulic', 0.255, -0.64, 0.035, z=3.2, tone=0.8, role='detail', region='hydraulic-unit')
    s.rect('ram', 'hydraulic', -0.06, -1.25, 0.06, -0.70, z=2.4, tone=0.7, region='hydraulic-unit', material='steel-hyd')
    s.line('ram-rod', 'hydraulic', [(0.0, -0.70), (0.0, -0.45)], role='detail', z=2.5, region='hydraulic-unit')
    return rep(50, 20, 75, pens(0.35, 0.18, 0.13, None, 0.5), s, [
        ('A section 11 mm wide and 38 mm high with technical detail: shaft with highlight, chevron band, three-LED beacon and seamed cap, sleeve outline, bolted frame blocks with brush seal, power unit with motor and seam, hose, ram with rod.', 'Ein Schnitt 11 mm breit und 38 mm hoch mit technischem Detail: Schaft mit Lichtkante, Chevronband, Leuchtring mit drei LEDs und genähte Kappe, Hülsenkontur, verschraubte Rahmenblöcke mit Bürstendichtung, Antriebseinheit mit Motor und Naht, Schlauch, Zylinder mit Stange.'),
        ('All parts at true size and position; the shaft slides on its prismatic bone.', 'Alle Teile in wahrer Größe und Lage; der Schaft gleitet an seinem Schubknochen.'),
        ('The sleeve wall is one outline; the third LED row and the cable gland are not shown.', 'Die Hülsenwand ist eine Kontur; die weiteren LED-Reihen und die Kabelverschraubung sind nicht gezeigt.'),
        ('Cable, valve block, anchor bolts below the frame.', 'Kabel, Ventilblock, Ankerschrauben unter dem Rahmen.'),
        ('LEDs are 0.02 m (0.4 mm) circles, the sleeve wall a 0.25 mm line (real 0.012 m = 0.24 mm).', 'LEDs sind Kreise von 0,02 m (0,4 mm), die Hülsenwand eine 0,25-mm-Linie (real 0,012 m = 0,24 mm).'),
        ('Chevron hatch in the reflective band (45 degrees, 0.5 mm).', 'Chevronschraffur im Reflexband (45 Grad, 0,5 mm).'),
        ('Shaft 0.5, band 0.08, beacon 0.7, frame 0.6; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.', 'Schaft 0,5, Band 0,08, Leuchtring 0,7, Rahmen 0,6; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm.'),
        ('LEDs and ring carry beacon-led; the hose, motor and rod carry hydraulic-unit: the thermal view shows pump heat in the buried block.', 'LEDs und Ring tragen beacon-led; Schlauch, Motor und Stange tragen hydraulic-unit: die Thermalansicht zeigt die Pumpenwärme im vergrabenen Block.'),
    ], [hatch('chevron', 'Chevron band', 'Chevronband', 'lines', 45, 0.5, 0.13)])


def sym_true():
    s = Shapes(rig)
    s.ell('shaft-plan', 'sleeve', 0, 0, 0.10, z=3, tone=0.3, region='shaft-steel', material='steel-grey')
    s.ell('sleeve-plan', 'sleeve', 0, 0, 0.15, z=2.5, tone=0.9, role='detail', filled=False, region='sleeve-steel')
    s.ell('frame-plan', 'sleeve', 0, 0, 0.20, z=2.4, tone=0.9, role='detail', filled=False, region='cover-frame')
    return {'id': 'plan-true', 'title': T('Plan, true extent', 'Grundriss, wahre Ausdehnung'),
            'description': T('The bollard seen from above: the 0.20 m shaft disc inside the 0.30 m sleeve and the 0.40 m frame ring.', 'Der Poller von oben: die 0,20-m-Schaftscheibe in der 0,30-m-Hülse und dem 0,40-m-Rahmenring.'),
            'semantics': 'true-extent', 'units': 'model-m', 'denominators': [200, 100, 50], 'pens': pens(0.25, 0.13, None, None, 0.35), 'hatches': [], 'geometry': {'elevation': [], 'plan': s.items},
            'dimensionalNote': T('Model metres at real size: circles of 0.20, 0.30 and 0.40 m diameter; pen widths are paper millimetres.', 'Modellmeter in wahrer Größe: Kreise mit 0,20, 0,30 und 0,40 m Durchmesser; Stiftbreiten sind Papiermillimeter.')}


def sym_dot():
    s = Shapes(rig)
    s.ell('dot', 'sleeve', 0, 0, 0.45, z=3, tone=1.0, region='shaft-steel')
    return {'id': 'plan-dot', 'title': T('Bollard dot', 'Pollerpunkt'),
            'description': T('A filled 0.9 mm dot marks one bollard on a site plan; at the bollard spacing of 1.2 to 1.5 m a row of dots reads as a line of control.', 'Ein gefüllter Punkt von 0,9 mm markiert einen Poller im Lageplan; beim Pollerabstand von 1,2 bis 1,5 m liest sich eine Punktreihe als Sperrlinie.'),
            'semantics': 'true-position', 'units': 'paper-mm', 'denominators': [500, 200], 'pens': pens(0.13, None, None, None, 0.18), 'hatches': [], 'geometry': {'elevation': [], 'plan': s.items},
            'dimensionalNote': T('The dot marks the real position of the shaft axis (insertion point at the origin); its 0.9 mm diameter is a paper size and carries no dimension (the real shaft is 0.2 m = 0.4 mm at 1:500).', 'Der Punkt markiert die reale Lage der Schaftachse (Einfügepunkt im Ursprung); sein Durchmesser von 0,9 mm ist ein Papiermaß und trägt kein Maß (der reale Schaft ist 0,2 m = 0,4 mm bei 1:500).')}


H_ = lambda lid, t, k, m, th, kk, rho, cp: layer(lid, T(*t), k, m, th, kk, rho, cp, 'authored-estimate', 'iso-10456-2007')
regions = [
    region('shaft-steel', T('Painted steel shaft', 'Lackiertes Stahlschaftrohr'), 'surface', 'passive', 'steel-grey', 0.41, 0.2, 'cylinder', 0.5, [H_('tube', ('Steel tube wall 5 mm', 'Stahlrohrwand 5 mm'), 'metal', 'steel-grey', 0.005, 50, 7850, 450)], 'authored-estimate', 'incropera-2018'),
    region('reflector-band', T('Retro-reflective band', 'Retroreflektierendes Band'), 'surface', 'passive', 'retro-film', 0.063, 0.2, 'cylinder', 0.5, [H_('film', ('Reflective film 0.5 mm', 'Reflexfolie 0,5 mm'), 'polymer', 'retro-film', 0.0005, 0.2, 1400, 1200)], 'authored-estimate', 'iso-10456-2007'),
    region('beacon-led', T('Amber LED beacon ring', 'Bernsteinfarbener LED-Leuchtring'), 'lamp', 'operating', 'pc-amber', 0.025, 0.2, 'cylinder', 0.5, [H_('lens', ('Polycarbonate lens 3 mm', 'Polycarbonatlinse 3 mm'), 'polymer', 'pc-amber', 0.003, 0.2, 1200, 1200)], 'authored-estimate', 'iso-10456-2007', operating=('beacon', 8)),
    region('cover-frame', T('Cast-iron cover frame', 'Gusseisen-Abdeckrahmen'), 'surface', 'passive', 'cast-iron-frame', 0.12, 0.4, 'horizontal-plate-up', 0.9, [H_('casting', ('Cast iron 25 mm', 'Gusseisen 25 mm'), 'metal', 'cast-iron-frame', 0.025, 50, 7200, 500)], 'authored-estimate', 'incropera-2018'),
    region('sleeve-steel', T('Buried sleeve (in soil)', 'Eingegrabene Hülse (im Erdreich)'), 'surface', 'passive', 'steel-sleeve', 1.2, 0.3, 'cylinder', 0.0, [H_('wall', ('Galvanised steel wall 8 mm', 'Verzinkte Stahlwand 8 mm'), 'metal', 'steel-sleeve', 0.008, 50, 7850, 450)], 'authored-estimate', 'incropera-2018'),
    region('hydraulic-unit', T('Hydraulic power unit and cylinder', 'Hydraulik-Antriebseinheit und Zylinder'), 'engine', 'operating', 'steel-hyd', 0.35, 0.3, 'vertical-plate', 0.0, [H_('housing', ('Steel housing 3 mm', 'Stahlgehäuse 3 mm'), 'metal', 'steel-hyd', 0.003, 50, 7850, 450)], 'authored-estimate', 'bollard-class', operating=('hydraulics', 560)),
]
RID = [r['id'] for r in regions]
OPK = [
    {'id': 'hydraulics', 'title': T('Hydraulic pump', 'Hydraulikpumpe'), 'levels': [{'v': 0, 'title': T('Stopped', 'Steht')}, {'v': 1, 'title': T('Running (0.75 kW motor, 560 W of heat)', 'Läuft (0,75-kW-Motor, 560 W Wärme)')}]},
    {'id': 'beacon', 'title': T('Warning beacon', 'Warnleuchte'), 'levels': [{'v': 0, 'title': T('Off', 'Aus')}, {'v': 1, 'title': T('Flashing amber (10 W, 8 W of heat)', 'Gelbes Blinklicht (10 W, 8 W Wärme)')}]},
]
BURIED = {'sleeve-steel': 0.0, 'hydraulic-unit': 0.0}
SOIL = {'sleeve-steel': {'fraction': 1.0, 'tempC': 11}, 'hydraulic-unit': {'fraction': 0.7, 'tempC': 11}}
SUN = {'shaft-steel': 0.5, 'reflector-band': 0.5, 'beacon-led': 0.5, 'cover-frame': 1.0}
push = load('nudge', T('Lateral nudge at 0.55 m', 'Seitlicher Anstoß auf 0,55 m'), 'imposed', 1500, 0, 'authored-estimate', 'en-1991-1-7', anchor='push-point')
F = ['sleeve-fixed']
def hb(hyd, beacon, sun, contact=SOIL):
    return bind(RID, sunmap=sun, exposed=BURIED, contact=contact, operating={'hydraulics': hyd, 'beacon': beacon}, emitters={})
poses = [
    pose('raised', T('Raised', 'Ausgefahren'), T('Shaft up 0.65 m, pump stopped, beacon off; the sun on the shaft and the cover frame.', 'Schaft 0,65 m oben, Pumpe steht, Leuchtring aus; Sonne auf Schaft und Abdeckrahmen.'), {'shaft': {'dy': 0.65}}, F, hb(0, 0, SUN)),
    pose('lowered', T('Lowered', 'Abgesenkt'), T('Shaft sunk flush with the ground inside the sleeve: only the cap and the frame show above the ground line.', 'Schaft bündig im Boden in der Hülse versenkt: nur Kappe und Rahmen zeigen sich über der Bodenlinie.'), {'shaft': {'dy': 0}}, F, hb(0, 0, {'cover-frame': 1.0, 'shaft-steel': 1.0})),
    pose('moving-up', T('Moving up', 'Fährt aus'), T('Shaft at 0.33 m, pump running at 560 W of heat and the beacon flashing.', 'Schaft auf 0,33 m, Pumpe läuft mit 560 W Wärme und der Leuchtring blinkt.'), {'shaft': {'dy': 0.33}}, F, hb(1, 1, SUN)),
    pose('raised-nudged', T('Raised, nudged', 'Ausgefahren, angestoßen'), T('Raised shaft with a 1.5 kN lateral nudge at 0.55 m (a gentle vehicle touch, no impact rating).', 'Ausgefahrener Schaft mit einem seitlichen Anstoß von 1,5 kN auf 0,55 m (eine sanfte Fahrzeugberührung, keine Anprallklasse).'), {'shaft': {'dy': 0.65}}, F, hb(0, 0, SUN), [push]),
]
clips = [
    clip('raise-and-lower', T('Raise and lower', 'Ausfahren und Absenken'), T('From flush the shaft rises in five seconds with the pump running and the beacon flashing, stays up for thirteen seconds and sinks again in five.', 'Aus der bündigen Lage fährt der Schaft in fünf Sekunden mit laufender Pumpe und blinkendem Leuchtring aus, bleibt dreizehn Sekunden oben und senkt sich wieder in fünf.'),
         30, 12, False, [(0, 'lowered', 'linear'), (2, 'lowered', 'linear'), (7, 'raised', 'linear'), (20, 'raised', 'linear'), (25, 'lowered', 'linear'), (30, 'lowered', 'linear')], [('sleeve-fixed', 0, 30)],
         [('flush', 0, 2, T('Flush', 'Bündig')), ('rising', 2, 7, T('Rising', 'Fährt aus')), ('up', 7, 20, T('Up', 'Oben')), ('lowering', 20, 25, T('Lowering', 'Senkt sich')), ('flush-again', 25, 30, T('Flush again', 'Wieder bündig'))],
         [(2, 'ignition', T('Pump starts', 'Pumpe startet'), 'hydraulics'), (7, 'shutdown', T('Pump stops at the top', 'Pumpe stoppt oben'), 'hydraulics'), (20, 'ignition', T('Pump starts to lower', 'Pumpe startet zum Absenken'), 'hydraulics'), (25, 'shutdown', T('Pump stops at the bottom', 'Pumpe stoppt unten'), 'hydraulics')],
         [('operating:hydraulics', 'step', [(0, 0), (2, 1), (7, 0), (20, 1), (25, 0)]), ('operating:beacon', 'step', [(0, 0), (2, 1), (9, 0), (20, 1), (27, 0)])]),
    clip('nudge-test', T('Nudge test', 'Anstoßprüfung'), T('The raised bollard is pushed sideways with up to 1.5 kN and released.', 'Der ausgefahrene Poller wird seitlich mit bis zu 1,5 kN angestoßen und entlastet.'),
         10, 12, False, [(0, 'raised-nudged', 'smooth'), (10, 'raised-nudged', 'smooth')], [('sleeve-fixed', 0, 10)],
         [('push', 0, 4, T('Pushing', 'Anstoß')), ('hold', 4, 7, T('Held', 'Gehalten')), ('release', 7, 10, T('Released', 'Entlastet'))],
         [(1, 'load-on', T('Contact starts', 'Berührung beginnt'), 'nudge'), (7, 'load-off', T('Contact ends', 'Berührung endet'), 'nudge')],
         [('load:nudge', 'smooth', [(0, 0), (1, 0), (4, 1), (7, 1), (9, 0), (10, 0)])]),
]
asset = {
    'id': 'retractable-bollard', 'category': 'objects',
    'title': T('Retractable Bollard (Section)', 'Versenkbarer Poller (Schnitt)'),
    'summary': T('An electro-hydraulic rising bollard of 0.20 m diameter, drawn as a section: 0.65 m raised above the ground, 1.25 m of sleeve below it with the cylinder and a power unit that heats up while the shaft moves; raised, lowered, moving and nudged.', 'Ein elektrohydraulischer Hubpoller mit 0,20 m Durchmesser, als Schnitt gezeichnet: 0,65 m ausgefahren über dem Boden, darunter 1,25 m Hülse mit Zylinder und einer Antriebseinheit, die sich bei Schaftbewegung erwärmt; ausgefahren, abgesenkt, fahrend und angestoßen.'),
    'tags': ['retractable-bollard', 'rising-bollard', 'hydraulic', 'traffic-control', 'section', 'access-control'],
    'provenance': {'author': 'Semio staffage content (NW-O)', 'method': 'handcrafted', 'created': '2026-10-29', 'references': refs,
                   'notes': T('Section elevation: the sleeve is cut so the lowered shaft stays visible, hence the 1.90 m total height (0.65 m above, 1.25 m below ground) and the 0.56 m width including the power unit. Mass 112 kg (shaft 38, sleeve and frame 42, cylinder and power unit 32) and the 0.75 kW pump are class data (authored estimates). Pump heat 560 W = 75 % of the 0.75 kW input in the unit while it runs; lift time 5 s. The bollard carries no impact rating (IWA 14-1 is context only).', 'Schnittansicht: die Hülse ist aufgeschnitten, damit der abgesenkte Schaft sichtbar bleibt, daher die Gesamthöhe 1,90 m (0,65 m über, 1,25 m unter dem Boden) und die Breite 0,56 m einschließlich der Antriebseinheit. Masse 112 kg (Schaft 38, Hülse und Rahmen 42, Zylinder und Antriebseinheit 32) und die 0,75-kW-Pumpe sind Klassenwerte (Schätzwerte). Pumpenwärme 560 W = 75 % der 0,75 kW Eingang in der Einheit, solange sie läuft; Hubzeit 5 s. Der Poller hat keine Anprallklasse (IWA 14-1 nur als Kontext).')},
    'dimensions': {'heightM': 1.90, 'widthM': 0.56, 'depthM': 0.40, 'massKg': round(rig.total_mass(), 2)},
    'materials': mats, 'rig': rig.rig(), 'anchors': anchors,
    'representations': {'500': r500(), '200': r200(), '100': r100(), '50': r50()}, 'symbols': [sym_true(), sym_dot()],
    'poses': poses, 'clips': clips,
    'thermal': {'environment': environment(12, 14, 0.7, 2.0, 300, 0.4, sky=2, ground=11), 'operating': OPK, 'passengers': [], 'regions': regions, 'emitters': [],
                'assumptions': [T('The buried sleeve and the hydraulic unit are conductively tied to soil at 11 degC (contact fractions 1.0 and 0.7) and not exposed to air; the pump heat of 560 W while running warms the unit above the soil.', 'Die eingegrabene Hülse und die Hydraulikeinheit sind leitend mit dem Erdreich von 11 degC verbunden (Kontaktanteile 1,0 und 0,7) und der Luft nicht ausgesetzt; die Pumpenwärme von 560 W bei Lauf erwärmt die Einheit über das Erdreich.'),
                                T('The beacon heats only by 8 W; the lowered cap is sun-heated flush with the paving.', 'Der Leuchtring erwärmt sich nur mit 8 W; die abgesenkte Kappe wird bündig im Belag besonnt.')]},
    'structure': structure(F, [('The sleeve is fixed in the foundation at ground level; the shaft is posed on its prismatic joint as one rigid body with the sleeve.', 'Die Hülse ist im Fundament in Geländehöhe eingespannt; der Schaft wird an seinem Schubgelenk als ein starrer Körper mit der Hülse gestellt.'),
                               ('The 1.5 kN nudge load is an authored estimate and not a crash design value.', 'Die Anstoßlast von 1,5 kN ist eine Schätzung und kein Anpralllast-Bemessungswert.')]),
    'presentation': {'defaultPose': 'raised', 'defaultClip': 'raise-and-lower', 'defaultScale': 100, 'viewpoints': ['elevation']},
    'exports': ENV_EXPORTS, 'completion': 'authored',
}
save(asset)
print('mass', rig.total_mass())
