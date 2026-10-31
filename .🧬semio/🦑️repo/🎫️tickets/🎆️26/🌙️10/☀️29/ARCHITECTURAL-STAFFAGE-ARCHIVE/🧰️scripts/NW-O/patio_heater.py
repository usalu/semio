import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

refs = [
    ref('turns-2012', 'Turns, S. R. (2012). An Introduction to Combustion: Concepts and Applications, 3rd ed., McGraw-Hill (lower heating value of propane 46.35 MJ/kg, stoichiometric air 15.7 kg per kg propane, C3H8 + 5 O2 -> 3 CO2 + 4 H2O).'),
    ref('en-14543', 'EN 14543 Gas-fired patio heaters (product standard for free-standing outdoor radiant heaters; heat input classes of the product family are authored estimates).'),
    ref('en-1442', 'EN 1442 Transportable refillable welded steel gas cylinders for LPG (11 kg propane steel cylinder class; tare mass is an authored estimate).'),
    ref('en-1991-1-4', 'EN 1991-1-4:2005 Eurocode 1: Actions on structures, Part 1-4: Wind actions (dynamic pressure q = 0.5 rho v^2).'),
    ref('iso-10456-2007', 'ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel, stainless steel).'),
    ref('incropera-2018', 'Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties, convection, plumes).'),
]
mats = [
    material('powder-steel', 'Powder-coated steel, dark grey', 'Pulverbeschichteter Stahl, dunkelgrau', 0.9, 0.8, 7850, 50, 450, 'authored-estimate', 'incropera-2018', 210e9),
    material('stainless-brushed', 'Brushed stainless steel', 'Edelstahl gebürstet', 0.35, 0.45, 7900, 17, 500, 'literature', 'iso-10456-2007', 200e9),
    material('stainless-hot', 'Oxidised stainless emitter shroud', 'Oxidierter Edelstahl-Strahlermantel', 0.8, 0.8, 7900, 17, 500, 'authored-estimate', 'incropera-2018', 200e9),
    material('alu-reflector', 'Aluminised steel reflector', 'Aluminierter Stahlreflektor', 0.2, 0.35, 7850, 50, 450, 'authored-estimate', 'incropera-2018'),
    material('plastic-black', 'Black thermoset knob', 'Schwarzer Duroplastknopf', 0.9, 0.9, 1400, 0.3, 1400, 'authored-estimate', 'iso-10456-2007'),
]

rig = Rig('mechanical', 'base')
LIT = 'authored-estimate'
rig.bone('base', None, 0, 0, 0.46, FIXED, T('Base plate and bottle housing', 'Bodenplatte und Flaschengehäuse'), (8.7, 0, 0.3, LIT, 'en-14543'))
rig.bone('bottle', 'base', 0, 0.03, 0.62, FIXED, T('Propane bottle inside the housing (11 kg gas)', 'Propanflasche im Gehäuse (11 kg Gas)'), (24.5, 0, 0.34, LIT, 'en-1442'))
rig.bone('pole', 'base', 0, 0.62, 1.36, FIXED, T('Stainless pole', 'Edelstahlmast'), (4.5, 0, 1.3, LIT, None))
rig.bone('burner-head', 'pole', 0, 1.98, 0.14, FIXED, T('Burner head with emitter shroud', 'Brennerkopf mit Strahlermantel'), (3.5, 0, 2.04, LIT, None))
rig.bone('reflector', 'burner-head', 0, 2.06, 0.19, FIXED, T('Dome reflector', 'Reflektorhaube'), (3.5, 0, 2.15, LIT, None))
rig.bone('knob', 'base', 0.17, 0.46, 0.03, revolute(0, 120), T('Burner control knob', 'Brennerregelknopf'), (0.3, 0.17, 0.46, LIT, None))
anchors = [
    rig.anchor('base-contact', 'base', 0, 0, 'contact', T('Base plate on the ground', 'Bodenplatte auf dem Boden'), support='contact', footprintLengthM=0.46, footprintWidthM=0.46, frictionCoefficient=0.5),
    rig.anchor('flue-vent', 'burner-head', 0, 0.12, 'emitter', T('Flue gas outlet under the dome', 'Abgasaustritt unter der Haube')),
    rig.anchor('reflector-top', 'reflector', 0, 0.19, 'emitter', T('Top of the dome', 'Haubenscheitel')),
]


def base_path(w_plate=0.23, h_plate=0.03, w=0.17):
    return (f'M {-w_plate:g} 0 L {w_plate:g} 0 L {w_plate:g} {h_plate:g} L {w:g} {h_plate:g} L {w:g} 0.60 C {w:g} 0.64 0.05 0.66 0.03 0.66 L -0.03 0.66 C -0.05 0.66 {-w:g} 0.64 {-w:g} 0.60 L {-w:g} {h_plate:g} L {-w_plate:g} {h_plate:g} Z')


def dome(rim, side=0.4):
    return f'M {-side:g} {rim:g} C {-side:g} {rim + 0.06:g} -0.22 2.25 0 2.25 C 0.22 2.25 {side:g} {rim + 0.06:g} {side:g} {rim:g} Z'


def r500():
    s = Shapes(rig)
    s.path('base', 'base', 'M -0.23 0 L 0.23 0 L 0.23 0.10 L 0.17 0.10 L 0.17 0.58 C 0.17 0.64 0.06 0.66 0.05 0.66 L -0.05 0.66 C -0.06 0.66 -0.17 0.64 -0.17 0.58 L -0.17 0.10 L -0.23 0.10 Z', z=3, tone=0.6, region='base-housing', material='powder-steel')
    s.rect('pole', 'pole', -0.07, 0.62, 0.07, 1.98, z=3, tone=0.45, region='pole-stainless', material='stainless-brushed')
    s.rect('head', 'burner-head', -0.13, 1.96, 0.13, 2.10, z=2.9, tone=0.8, region='emitter-head', material='stainless-hot')
    s.path('reflector', 'reflector', dome(2.03), z=3.2, tone=0.5, region='reflector', material='alu-reflector')
    s.rect('knob', 'knob', 0.14, 0.40, 0.20, 0.52, z=3.5, tone=0.2, region='control', material='plastic-black')
    return rep(500, 350, 1000, pens(0.13, None, None, None, 0.18), s, [
        ('A mushroom sign 1.6 mm wide and 4.5 mm high: a stepped base block, a thick pole, a small head block and a solid dome.', 'Ein Pilzzeichen, 1,6 mm breit und 4,5 mm hoch: ein gestufter Sockelblock, ein dicker Mast, ein kleiner Kopfblock und eine volle Haube.'),
        ('Height 2.25 m and the 0.80 m dome width exactly; the base plate step, pole, emitter head, dome and the control knob.', 'Höhe 2,25 m und die Haubenbreite 0,80 m exakt; Bodenplattenstufe, Mast, Strahlerkopf, Haube und der Regelknopf.'),
        ('Base plate, housing and door merge into the stepped base block; the knob is a small bar on its side.', 'Bodenplatte, Gehäuse und Tür gehen im gestuften Sockelblock auf; der Knopf ist ein kleiner Balken an dessen Seite.'),
        ('Door seam, vents, wheels, pole joint, flame guard rods, mesh.', 'Türfuge, Lüftungsschlitze, Räder, Mastfuge, Flammenschutzstäbe, Gitter.'),
        ('The 0.06 m pole would be 0.12 mm wide: thickened to 0.14 m (0.28 mm); the 0.03 m base plate is merged into a 0.10 m (0.2 mm) step; the dome rim hangs 0.03 m lower than real so its edge reaches the feature width.', 'Der 0,06 m dicke Mast wäre 0,12 mm breit: auf 0,14 m (0,28 mm) verbreitert; die 0,03 m dicke Bodenplatte geht in einer 0,10-m-Stufe (0,2 mm) auf; der Haubenrand hängt 0,03 m tiefer als real, damit seine Kante die Mindestbreite erreicht.'),
        ('None.', 'Keine.'),
        ('Base 0.6, pole 0.45, hot head 0.8, dome 0.5, knob 0.2; body pen 0.13 mm, ground 0.18 mm.', 'Sockel 0,6, Mast 0,45, heißer Kopf 0,8, Haube 0,5, Knopf 0,2; Körperstift 0,13 mm, Boden 0,18 mm.'),
        ('Each block carries its region; the hot head and the dome are the operating regions seen on the thermal view.', 'Jeder Block trägt seine Region; heißer Kopf und Haube sind die Betriebsregionen der Thermalansicht.'),
    ])


def r200():
    s = Shapes(rig)
    s.path('base', 'base', base_path(0.23, 0.05, 0.17), z=3, tone=0.6, region='base-housing', material='powder-steel')
    s.rect('pole', 'pole', -0.035, 0.62, 0.035, 1.98, z=3, tone=0.45, region='pole-stainless', material='stainless-brushed')
    s.rect('head', 'burner-head', -0.13, 1.98, 0.13, 2.10, z=2.9, tone=0.8, region='emitter-head', material='stainless-hot')
    s.path('reflector', 'reflector', dome(2.05), z=3.2, tone=0.5, region='reflector', material='alu-reflector')
    s.ell('knob', 'knob', 0.17, 0.46, 0.03, z=3.5, tone=0.2, region='control', material='plastic-black')
    s.line('door-seam', 'base', [(-0.11, 0.08), (-0.11, 0.52), (0.11, 0.52), (0.11, 0.08)], role='detail', z=3.4, region='base-housing')
    return rep(200, 150, 350, pens(0.18, 0.13, None, None, 0.25), s, [
        ('A heater 4 mm wide and 11 mm high: base with housing shoulder, a 0.3 mm pole, the emitter head under a solid dome, a round knob and the door seam.', 'Ein Heizpilz 4 mm breit und 11 mm hoch: Sockel mit Gehäuseschulter, ein 0,3 mm dicker Mast, der Strahlerkopf unter einer vollen Haube, ein runder Knopf und die Türfuge.'),
        ('True proportions: plate, housing, pole, head, dome, knob; the door as one seam line.', 'Wahre Proportionen: Platte, Gehäuse, Mast, Kopf, Haube, Knopf; die Tür als eine Fugenlinie.'),
        ('Plate and housing are one base path; the pole joint merges into the pole.', 'Platte und Gehäuse sind ein Sockelpfad; die Mastfuge geht im Mast auf.'),
        ('Vents, wheels, handle, flame guard, mesh.', 'Lüftungsschlitze, Räder, Griff, Flammenschutz, Gitter.'),
        ('Pole is 0.07 m = 0.35 mm wide (real 0.06 m): thickened 0.01 m; the dome rim is 0.01 m lower than real.', 'Der Mast ist 0,07 m = 0,35 mm breit (real 0,06 m): um 0,01 m verbreitert; der Haubenrand liegt 0,01 m tiefer als real.'),
        ('None.', 'Keine.'),
        ('Base 0.6, pole 0.45, head 0.8, dome 0.5; body pen 0.18 mm, detail 0.13 mm.', 'Sockel 0,6, Mast 0,45, Kopf 0,8, Haube 0,5; Körperstift 0,18 mm, Detail 0,13 mm.'),
        ('Door seam carries base-housing; knob control.', 'Die Türfuge trägt base-housing; der Knopf control.'),
    ])


def r100():
    s = Shapes(rig)
    s.path('base', 'base', base_path(0.23, 0.04, 0.17), z=3, tone=0.6, region='base-housing', material='powder-steel')
    s.rect('pole', 'pole', -0.03, 0.62, 0.03, 1.98, z=3, tone=0.45, region='pole-stainless', material='stainless-brushed')
    s.line('pole-joint', 'pole', [(-0.03, 1.30), (0.03, 1.30)], role='detail', z=3.4, region='pole-stainless')
    s.rect('head', 'burner-head', -0.13, 1.98, 0.13, 2.10, r=0.02, z=2.9, tone=0.8, region='emitter-head', material='stainless-hot')
    for i, x in enumerate((-0.08, 0.0, 0.08)):
        s.line(f'guard-rod-{i}', 'burner-head', [(x, 1.985), (x, 2.055)], role='detail', z=3.3, region='emitter-head')
    s.path('reflector', 'reflector', dome(2.06), z=3.2, tone=0.5, region='reflector', material='alu-reflector')
    s.line('dome-rim', 'reflector', [(-0.40, 2.08), (0.40, 2.08)], role='detail', z=3.4, region='reflector')
    s.ell('knob', 'knob', 0.17, 0.46, 0.028, z=3.5, tone=0.2, region='control', material='plastic-black')
    s.line('door-seam', 'base', [(-0.11, 0.08), (-0.11, 0.52), (0.11, 0.52), (0.11, 0.08)], role='detail', z=3.4, region='base-housing')
    s.line('vent-slots', 'base', [(-0.08, 0.57), (0.08, 0.57)], role='detail', z=3.4, region='base-housing')
    return rep(100, 75, 150, pens(0.25, 0.13, None, None, 0.35), s, [
        ('A heater 8 mm wide and 22.5 mm high: stepped base with door and vent line, pole with a joint, emitter head with three guard rods, dome with rim line, round knob.', 'Ein Heizpilz 8 mm breit und 22,5 mm hoch: gestufter Sockel mit Tür und Lüftungslinie, Mast mit Fuge, Strahlerkopf mit drei Schutzstäben, Haube mit Randlinie, runder Knopf.'),
        ('Plate, housing, door, pole, joint, head, guard rods, dome and rim, knob.', 'Platte, Gehäuse, Tür, Mast, Fuge, Kopf, Schutzstäbe, Haube und Rand, Knopf.'),
        ('Only six guard rods of the head are drawn as three; the housing shoulder is one curve.', 'Von den sechs Schutzstäben des Kopfes sind nur drei gezeichnet; die Gehäuseschulter ist eine Kurve.'),
        ('Wheels, handle, mesh, label plate, bottle valve.', 'Räder, Griff, Gitter, Typenschild, Flaschenventil.'),
        ('The three guard rods are 0.13 mm strokes 0.08 m (0.8 mm) apart; the pole is true at 0.6 mm.', 'Die drei Schutzstäbe sind 0,13-mm-Striche im Abstand 0,08 m (0,8 mm); der Mast ist mit 0,6 mm wahr.'),
        ('None; the mesh would need spacing below 0.4 mm.', 'Keine; das Gitter bräuchte Abstände unter 0,4 mm.'),
        ('Base 0.6, pole 0.45, head 0.8, dome 0.5; body pen 0.25 mm, detail 0.13 mm.', 'Sockel 0,6, Mast 0,45, Kopf 0,8, Haube 0,5; Körperstift 0,25 mm, Detail 0,13 mm.'),
        ('Lines carry the region of the part they mark; the emitter head and dome are the operating regions.', 'Linien tragen die Region des Teils, das sie kennzeichnen; Strahlerkopf und Haube sind die Betriebsregionen.'),
    ])


def r50():
    s = Shapes(rig)
    s.path('base', 'base', base_path(0.23, 0.03, 0.17), z=3, tone=0.6, region='base-housing', material='powder-steel')
    s.ell('wheel', 'base', -0.14, 0.05, 0.04, z=2.8, tone=0.9, region='base-housing', material='plastic-black')
    s.line('base-lip', 'base', [(-0.17, 0.045), (0.17, 0.045)], role='detail', z=3.4, region='base-housing')
    s.line('door-seam', 'base', [(-0.12, 0.09), (-0.12, 0.52), (0.12, 0.52), (0.12, 0.09)], role='detail', z=3.4, region='base-housing')
    for i, y in enumerate((0.55, 0.58, 0.61)):
        s.line(f'vent-slot-{i}', 'base', [(-0.07, y), (0.07, y)], role='texture', z=3.4, region='base-housing')
    s.rect('pole', 'pole', -0.03, 0.62, 0.03, 1.98, z=3, tone=0.45, region='pole-stainless', material='stainless-brushed')
    s.line('pole-joint', 'pole', [(-0.03, 1.30), (0.03, 1.30)], role='detail', z=3.4, region='pole-stainless')
    s.line('pole-highlight', 'pole', [(-0.012, 0.70), (-0.012, 1.90)], role='texture', z=3.5, region='pole-stainless')
    s.rect('head', 'burner-head', -0.13, 1.98, 0.13, 2.10, r=0.02, z=2.9, tone=0.8, region='emitter-head', material='stainless-hot', hatch='mesh')
    for i, x in enumerate((-0.10, -0.05, 0.0, 0.05, 0.10)):
        s.line(f'guard-rod-{i}', 'burner-head', [(x, 1.99), (x, 2.055)], role='detail', z=3.3, region='emitter-head')
    s.path('reflector', 'reflector', dome(2.06), z=3.2, tone=0.5, region='reflector', material='alu-reflector')
    s.path('dome-bead', 'reflector', 'M -0.395 2.085 C -0.30 2.105 0.30 2.105 0.395 2.085', role='detail', z=3.4, filled=False, region='reflector')
    s.ell('top-cap', 'reflector', 0, 2.235, 0.03, 0.012, z=3.5, tone=0.2, role='detail', region='reflector')
    s.ell('knob', 'knob', 0.17, 0.46, 0.03, z=3.5, tone=0.2, region='control', material='plastic-black')
    s.line('knob-pointer', 'knob', [(0.17, 0.46), (0.17 + 0.026, 0.46)], role='detail', z=3.6, region='control')
    return rep(50, 20, 75, pens(0.35, 0.18, 0.13, None, 0.5), s, [
        ('A heater 16 mm wide and 45 mm high in construction detail: base plate with lip and wheel, housing with door seam and three vent slots, pole with joint and highlight, burner head with five guard rods over a mesh, dome with rim bead and cap, knob with pointer.', 'Ein Heizpilz 16 mm breit und 45 mm hoch im Konstruktionsdetail: Bodenplatte mit Rand und Rad, Gehäuse mit Türfuge und drei Lüftungsschlitzen, Mast mit Fuge und Lichtkante, Brennerkopf mit fünf Schutzstäben über einem Gitter, Haube mit Randwulst und Kappe, Knopf mit Zeiger.'),
        ('All visible construction at real size; the knob pointer turns with the knob bone.', 'Alle sichtbare Konstruktion in realer Größe; der Knopfzeiger dreht sich mit dem Knopfknochen.'),
        ('The far wheel is hidden behind the near one; the dome interior is not visible from the side.', 'Das ferne Rad ist hinter dem nahen verdeckt; das Haubeninnere ist von der Seite nicht sichtbar.'),
        ('Label plate, bottle valve, tipping handle.', 'Typenschild, Flaschenventil, Kippgriff.'),
        ('Guard rods are 0.18 mm strokes at 1.3 mm spacing; the dome bead is a 0.18 mm line; the top cap ellipse is 1.2 x 0.5 mm.', 'Schutzstäbe sind 0,18-mm-Striche im Abstand 1,3 mm; der Haubenwulst ist eine 0,18-mm-Linie; die Kappenellipse misst 1,2 x 0,5 mm.'),
        ('Fine crosshatch (0.5 mm) in the emitter head as a mesh indication.', 'Feine Kreuzschraffur (0,5 mm) im Strahlerkopf als Gitterandeutung.'),
        ('Base 0.6, pole 0.45, hot head 0.8, dome 0.5, knob 0.2; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.', 'Sockel 0,6, Mast 0,45, heißer Kopf 0,8, Haube 0,5, Knopf 0,2; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm.'),
        ('Wheel and lines carry base-housing, mesh and rods emitter-head: the thermal view paints the hot shroud under the dome.', 'Rad und Linien tragen base-housing, Gitter und Stäbe emitter-head: die Thermalansicht färbt den heißen Mantel unter der Haube.'),
    ], [hatch('mesh', 'Burner mesh', 'Brennergitter', 'cross', 45, 0.5, 0.13)])


sheet = lambda lid, t, k, mat, th, kk, rho, cp: layer(lid, T(t[0], t[1]), k, mat, th, kk, rho, cp, 'authored-estimate', 'iso-10456-2007')
regions = [
    region('base-housing', T('Bottle housing and base plate', 'Flaschengehäuse und Bodenplatte'), 'surface', 'passive', 'powder-steel', 0.75, 0.6, 'vertical-plate', 0.5, [sheet('shell', ('Powder-coated steel sheet 1.2 mm', 'Pulverbeschichtetes Stahlblech 1,2 mm'), 'metal', 'powder-steel', 0.0012, 50, 7850, 450)], 'authored-estimate', 'incropera-2018'),
    region('pole-stainless', T('Stainless pole', 'Edelstahlmast'), 'surface', 'passive', 'stainless-brushed', 0.26, 0.06, 'cylinder', 0.5, [sheet('tube', ('Stainless tube wall 1.5 mm', 'Edelstahlrohrwand 1,5 mm'), 'metal', 'stainless-brushed', 0.0015, 17, 7900, 500)], 'authored-estimate', 'incropera-2018'),
    region('emitter-head', T('Emitter shroud (hot head)', 'Strahlermantel (heißer Kopf)'), 'heater', 'operating', 'stainless-hot', 0.2, 0.26, 'cylinder', 0.6, [sheet('shroud', ('Oxidised stainless shroud 1.5 mm', 'Oxidierter Edelstahlmantel 1,5 mm'), 'metal', 'stainless-hot', 0.0015, 17, 7900, 500)], 'authored-estimate', 'turns-2012', operating=('burner', 8200)),
    region('reflector', T('Dome reflector', 'Reflektorhaube'), 'heater', 'operating', 'alu-reflector', 0.55, 0.8, 'horizontal-plate-up', 0.9, [sheet('dish', ('Aluminised steel dish 1 mm', 'Aluminierte Stahlschale 1 mm'), 'metal', 'alu-reflector', 0.001, 50, 7850, 450)], 'authored-estimate', 'turns-2012', operating=('burner', 2200)),
    region('control', T('Control knob and valve', 'Regelknopf und Ventil'), 'surface', 'passive', 'plastic-black', 0.004, 0.05, 'sphere', 0.5, [sheet('knob', ('Thermoset 5 mm', 'Duroplast 5 mm'), 'polymer', 'plastic-black', 0.005, 0.3, 1400, 1400)], 'authored-estimate', 'iso-10456-2007'),
]
RID = [r['id'] for r in regions]
SUNOFF = {'base-housing': 0.5, 'pole-stainless': 0.5, 'emitter-head': 0.3, 'reflector': 1.0, 'control': 0.5}
OPK = [{'id': 'burner', 'title': T('Propane burner', 'Propanbrenner'), 'levels': [
    {'v': 0, 'title': T('Off', 'Aus')}, {'v': 0.46, 'title': T('Low (6.0 kW heat input)', 'Klein (6,0 kW Wärmebelastung)')}, {'v': 1, 'title': T('High (13.0 kW heat input)', 'Groß (13,0 kW Wärmebelastung)')}]}]
emitters = [
    {'id': 'flue-gas', 'kind': 'flue', 'title': T('Flue gas plume under the dome', 'Abgasfahne unter der Haube'), 'anchor': 'flue-vent', 'directionDeg': 90, 'diameterM': 0.2, 'operatingKey': 'burner', 'volumeFlowM3S': 0.0098, 'temperatureRiseK': 280, 'vapourKgPerKg': 0.078, 'source': 'authored-estimate', 'reference': 'turns-2012'},
    {'id': 'dome-plume', 'kind': 'plume', 'title': T('Free convection above the hot dome', 'Freie Konvektion über der heißen Haube'), 'anchor': 'reflector-top', 'directionDeg': 90, 'diameterM': 0.6, 'region': 'reflector', 'source': 'authored-estimate', 'reference': 'incropera-2018'},
]
wind = [load('wind-dome', T('Breeze on the dome', 'Wind auf die Haube'), 'wind', 12, 0, 'authored-estimate', 'en-1991-1-4', bone='reflector', x=0, y=0.1),
        load('wind-pole', T('Breeze on the pole', 'Wind auf den Mast'), 'wind', 8, 0, 'authored-estimate', 'en-1991-1-4', bone='pole', x=0, y=0.7),
        load('wind-base', T('Breeze on the housing', 'Wind auf das Gehäuse'), 'wind', 10, 0, 'authored-estimate', 'en-1991-1-4', bone='base', x=0, y=0.3)]
C = ['base-contact']
def hb(level, sun, emit):
    return bind(RID, sunmap=sun, emitters={'flue-gas': emit, 'dome-plume': emit}, operating={'burner': level})
poses = [
    pose('off-afternoon', T('Off, in the afternoon sun', 'Aus, in der Nachmittagssonne'), T('Cold heater, knob at off; the sun shines on the dome and the pole.', 'Kalter Heizpilz, Knopf auf Aus; die Sonne scheint auf Haube und Mast.'), {'knob': {'angleDeg': 0}}, C, hb(0, SUNOFF, 0)),
    pose('low', T('Burning on low', 'Brennt auf kleiner Stufe'), T('Knob at 60 degrees: 6.0 kW heat input, the shroud glows dull, a weak flue plume rises.', 'Knopf auf 60 Grad: 6,0 kW Wärmebelastung, der Mantel glüht schwach, eine schwache Abgasfahne steigt auf.'), {'knob': {'angleDeg': 60}}, C, hb(0.46, {}, 1)),
    pose('high', T('Burning on high', 'Brennt auf großer Stufe'), T('Knob at 120 degrees: 13.0 kW heat input, the emitter shroud radiates downwards and the flue gas warms the dome.', 'Knopf auf 120 Grad: 13,0 kW Wärmebelastung, der Strahlermantel strahlt nach unten und das Abgas erwärmt die Haube.'), {'knob': {'angleDeg': 120}}, C, hb(1, {}, 1)),
    pose('high-in-breeze', T('Burning on high in a breeze', 'Brennt auf großer Stufe im Wind'), T('Full output with an 8 m/s breeze (about 40 Pa) pushing on dome, pole and housing.', 'Volle Leistung bei 8 m/s Wind (etwa 40 Pa) auf Haube, Mast und Gehäuse.'), {'knob': {'angleDeg': 120}}, C, hb(1, {}, 1), wind),
]
clips = [
    clip('ignite-and-warm-up', T('Ignite and warm up', 'Zünden und aufheizen'), T('Cold start: the knob is turned to high and pressed, the burner ignites and the shroud and dome heat up over fifteen minutes.', 'Kaltstart: der Knopf wird auf Groß gedreht und gedrückt, der Brenner zündet und Mantel und Haube heizen sich über fünfzehn Minuten auf.'),
         900, 1, False, [(0, 'off-afternoon', 'step'), (5, 'high', 'step'), (900, 'high', 'step')], [('base-contact', 0, 900)],
         [('cold', 0, 5, T('Cold', 'Kalt')), ('warm-up', 5, 900, T('Burning, warming up', 'Brennt, heizt auf'))],
         [(5, 'ignition', T('Burner ignites', 'Brenner zündet'), 'burner'), (60, 'thermal', T('Shroud reaches radiant temperature', 'Mantel erreicht Strahlungstemperatur'), None)],
         [('operating:burner', 'step', [(0, 0), (5, 1)]), ('emitter:flue-gas', 'step', [(0, 0), (5, 1)]), ('emitter:dome-plume', 'step', [(0, 0), (5, 1)])]),
    clip('turn-down-and-shut-off', T('Turn down and shut off', 'Herunterregeln und abstellen'), T('After an hour on high the knob goes to low, later to off; the flame goes out and the hot parts cool for twenty minutes.', 'Nach einer Stunde auf Groß wird der Knopf auf Klein, später auf Aus gedreht; die Flamme erlischt und die heißen Teile kühlen zwanzig Minuten ab.'),
         1800, 1, False, [(0, 'high', 'step'), (60, 'low', 'step'), (600, 'off-afternoon', 'step'), (1800, 'off-afternoon', 'step')], [('base-contact', 0, 1800)],
         [('high', 0, 60, T('On high', 'Auf Groß')), ('low', 60, 600, T('On low', 'Auf Klein')), ('cool-down', 600, 1800, T('Off, cooling down', 'Aus, kühlt ab'))],
         [(60, 'operating', T('Turned down to low', 'Auf Klein heruntergeregelt'), 'burner'), (600, 'shutdown', T('Gas valve closed', 'Gasventil geschlossen'), 'burner')],
         [('operating:burner', 'step', [(0, 1), (60, 0.46), (600, 0)]), ('emitter:flue-gas', 'step', [(0, 1), (600, 0)]), ('emitter:dome-plume', 'step', [(0, 1), (900, 0)])]),
]
asset = {
    'id': 'patio-heater', 'category': 'objects',
    'title': T('Propane Patio Heater', 'Gas-Heizpilz (Propan)'),
    'summary': T('A free-standing 13 kW propane radiant heater, 2.25 m high with a 0.80 m dome: operating levels off, low (6 kW) and high (13 kW) heat the emitter shroud and the dome and send a hot flue plume up through the dome.', 'Ein freistehender Propan-Heizstrahler von 13 kW, 2,25 m hoch mit 0,80 m Haube: die Betriebsstufen Aus, Klein (6 kW) und Groß (13 kW) heizen Strahlermantel und Haube und schicken eine heiße Abgasfahne durch die Haube nach oben.'),
    'tags': ['patio-heater', 'propane', 'radiant-heater', 'outdoor', 'burner', 'flue'],
    'provenance': {'author': 'Semio staffage content (NW-O)', 'method': 'handcrafted', 'created': '2026-10-29', 'references': refs,
                   'notes': T('Side elevation of the mushroom-type heater with the 11 kg propane bottle in the base housing (mass 45 kg with a full bottle). Heat input 13 kW at high and 6 kW at low are authored estimates for the 12-14 kW patio-heater class (EN 14543 product family). The release is split into 8.2 kW at the emitter shroud and 2.2 kW at the dome, the flue gas sensible heat (0.0098 m3/s at 280 K rise = 1.9 kW) makes up the rest; the flue vapour share 0.078 kg/kg follows from C3H8 + 5 O2 -> 3 CO2 + 4 H2O (1.64 kg water per kg propane) with 21 kg flue gas per kg propane at excess air 1.3.', 'Seitenansicht des Heizpilzes vom Pilztyp mit der 11-kg-Propanflasche im Sockelgehäuse (Masse 45 kg mit voller Flasche). Wärmebelastung 13 kW auf Groß und 6 kW auf Klein sind Schätzwerte für die Heizpilzklasse von 12-14 kW (Produktfamilie EN 14543). Die Freisetzung teilt sich in 8,2 kW am Strahlermantel und 2,2 kW an der Haube, die fühlbare Abgaswärme (0,0098 m3/s bei 280 K Anstieg = 1,9 kW) bildet den Rest; der Dampfanteil 0,078 kg/kg folgt aus C3H8 + 5 O2 -> 3 CO2 + 4 H2O (1,64 kg Wasser je kg Propan) mit 21 kg Abgas je kg Propan bei Luftüberschuss 1,3.')},
    'dimensions': {'heightM': 2.25, 'widthM': 0.80, 'depthM': 0.80, 'massKg': round(rig.total_mass(), 2)},
    'materials': mats, 'rig': rig.rig(), 'anchors': anchors,
    'representations': {'500': r500(), '200': r200(), '100': r100(), '50': r50()}, 'symbols': [],
    'poses': poses, 'clips': clips,
    'thermal': {'environment': environment(8, 8, 0.7, 0.5, 0, 0.0, sky=-5, ground=8), 'operating': OPK, 'passengers': [], 'regions': regions, 'emitters': emitters,
                'assumptions': [T('The shroud and dome are operating regions: the stated watts at level 1 are released into them and leave by radiation and convection; the flue gas carries a further 1.9 kW as a hot plume. Heat input 13 kW = 0.28 g/s propane at 46.35 MJ/kg.', 'Mantel und Haube sind Betriebsregionen: die genannten Watt bei Stufe 1 werden in sie freigesetzt und verlassen sie durch Strahlung und Konvektion; das Abgas trägt weitere 1,9 kW als heiße Fahne. Wärmebelastung 13 kW = 0,28 g/s Propan bei 46,35 MJ/kg.'),
                                T('The propane bottle inside the housing is not a thermal region; evaporative cooling of the bottle is neglected.', 'Die Propanflasche im Gehäuse ist keine thermische Region; die Verdunstungskühlung der Flasche wird vernachlässigt.')]},
    'structure': structure(C, [('A free-standing rigid body on its base plate; the plate transmits a centre-of-pressure moment up to N times half the plate width, which carries the breeze.', 'Ein freistehender starrer Körper auf seiner Bodenplatte; die Platte überträgt ein Druckmittelpunktmoment bis N mal halbe Plattenbreite, das den Wind aufnimmt.'),
                              ('Wind pressure 0.5 rho v^2 at 8 m/s with authored drag coefficients (EN 1991-1-4 basis).', 'Winddruck 0,5 rho v^2 bei 8 m/s mit geschätzten Widerstandsbeiwerten (Grundlage EN 1991-1-4).')]),
    'presentation': {'defaultPose': 'high', 'defaultClip': 'ignite-and-warm-up', 'defaultScale': 100, 'viewpoints': ['elevation']},
    'exports': ENV_EXPORTS, 'completion': 'authored',
}
save(asset)
print('mass', rig.total_mass())
