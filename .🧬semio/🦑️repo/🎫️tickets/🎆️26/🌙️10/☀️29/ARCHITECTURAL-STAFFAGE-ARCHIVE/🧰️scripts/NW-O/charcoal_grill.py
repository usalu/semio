import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

LIT = 'authored-estimate'
RIM = 0.74
RX = 0.2925
refs = [
    ref('turns-2012', 'Turns, S. R. (2012). An Introduction to Combustion: Concepts and Applications, 3rd ed., McGraw-Hill (charcoal as nearly pure carbon, about 29-30 MJ/kg lower heating value, C + O2 -> CO2, stoichiometric air about 11.5 kg per kg carbon).'),
    ref('en-1860-1', 'EN 1860-1:2013 Appliances, solid fuels and firelighters for barbecueing - Part 1: Barbecues burning solid fuels - Requirements and test methods (stability, surface temperatures of handles).'),
    ref('iso-10456-2007', 'ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel, wood).'),
    ref('incropera-2018', 'Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties, natural convection, buoyant plumes).'),
]
mats = [
    material('enamel-steel', 'Porcelain-enamelled steel, black', 'Emaillierter Stahl, schwarz', 0.9, 0.85, 7850, 50, 450, 'authored-estimate', 'incropera-2018', 210e9),
    material('steel-black', 'Black painted steel tube', 'Schwarz lackiertes Stahlrohr', 0.9, 0.85, 7850, 50, 450, 'authored-estimate', 'incropera-2018', 210e9),
    material('alu-cast', 'Cast aluminium', 'Aluminiumguss', 0.2, 0.4, 2700, 160, 880, 'authored-estimate', 'iso-10456-2007'),
    material('wood-handle', 'Heat-resistant wood handle', 'Hitzebeständiger Holzgriff', 0.9, 0.6, 700, 0.15, 1600, 'authored-estimate', 'iso-10456-2007'),
    material('chrome-steel', 'Chrome-plated steel rod grate', 'Verchromter Stahlstab-Rost', 0.15, 0.4, 7850, 50, 450, 'authored-estimate', 'incropera-2018'),
    material('charcoal', 'Glowing charcoal bed', 'Glühende Holzkohleglut', 0.95, 0.95, 250, 0.2, 1000, 'authored-estimate', 'turns-2012'),
]
rig = Rig('mechanical', 'bowl')
rig.bone('bowl', None, 0, 0, 0.74, FIXED, T('Bowl with tripod legs and ash catcher', 'Kessel mit Dreibein und Aschefänger'), (8.0, 0, 0.42, LIT, 'en-1860-1'))
rig.bone('grate', 'bowl', 0, 0.70, 0.39, FIXED, T('Cooking grate', 'Grillrost'), (1.3, 0, 0.70, LIT, None))
rig.bone('charcoal', 'bowl', 0, 0.58, 0.39, FIXED, T('Charcoal bed (initial charge 1.5 kg)', 'Holzkohlebett (Erstfüllung 1,5 kg)'), (1.5, 0, 0.58, LIT, 'turns-2012'))
rig.bone('lid', 'bowl', -RX, RIM, 0.585, revolute(0, 115), T('Lid (hung on the rim hook, modelled as a hinge)', 'Deckel (am Randhaken eingehängt, als Gelenk modelliert)'), (3.0, 0, RIM + 0.07, LIT, None))
rig.bone('lid-vent', 'lid', 0, 0.94, 0.07, revolute(0, 90), T('Lid vent dial', 'Deckel-Lüftungsscheibe'), (0.1, 0, 0.95, LIT, None))
rig.bone('ash-damper', 'bowl', 0, 0.495, 0.1, revolute(0, 90), T('Bottom air damper', 'Unterluft-Schieber'), (0.1, 0, 0.495, LIT, None))
anchors = [
    rig.anchor('leg-left', 'bowl', -0.27, 0, 'contact', T('Left leg foot', 'Fuß linkes Bein'), support='contact', footprintLengthM=0.04, footprintWidthM=0.04, frictionCoefficient=0.5),
    rig.anchor('leg-right', 'bowl', 0.27, 0, 'contact', T('Right leg foot (with wheel)', 'Fuß rechtes Bein (mit Rad)'), support='contact', footprintLengthM=0.04, footprintWidthM=0.04, frictionCoefficient=0.5),
    rig.anchor('grate-point', 'grate', 0, 0.70, 'load', T('Food on the grate', 'Grillgut auf dem Rost')),
    rig.anchor('vent-top', 'lid-vent', 0, 0.965, 'emitter', T('Lid vent opening', 'Deckelöffnung')),
    rig.anchor('bowl-mouth', 'bowl', 0, RIM, 'emitter', T('Open bowl mouth', 'Offene Kesselmündung')),
]
LID = f'M {-RX:g} 0.735 L {-RX:g} {RIM:g} C {-RX:g} 0.88 -0.17 0.94 0 0.94 C 0.17 0.94 {RX:g} 0.88 {RX:g} {RIM:g} L {RX:g} 0.735 Z'
BOWL = f'M {-RX:g} {RIM:g} C {-RX:g} 0.60 -0.18 0.50 0 0.50 C 0.18 0.50 {RX:g} 0.60 {RX:g} {RIM:g} Z'


def common_legs(s, width, line=False, pen=None, z=2.5):
    for i, (fx, ax) in enumerate(((-0.27, -0.17), (0.27, 0.17))):
        if line:
            s.line(f'leg-{i}', 'bowl', [(fx, 0.0), (ax, 0.58)], z=z, region='legs-steel', pen=pen)
        else:
            s.cap(f'leg-{i}', 'bowl', fx, 0.012, ax, 0.58, width, width, z=z, tone=0.3, region='legs-steel', material='steel-black')


def r500():
    s = Shapes(rig)
    s.path('lid', 'lid', LID.replace('0.735', '0.725').replace('0.94 0 0.94 C 0.17 0.94','0.96 0 0.96 C 0.17 0.96').replace('-0.17 0.94','-0.17 0.96'), z=3.2, tone=0.3, regions=['lid-shell', 'vents-alu'], material='enamel-steel')
    s.path('bowl', 'bowl', 'M -0.2925 0.735 C -0.2925 0.60 -0.18 0.46 0 0.46 C 0.18 0.46 0.2925 0.60 0.2925 0.735 Z', z=3, tone=0.3, regions=['bowl-shell', 'ash-catcher'], material='enamel-steel')
    common_legs(s, 0, line=True, pen=0.13)
    s.line('handle', 'lid', [(0.27, 0.85), (0.38, 0.85)], z=3.5, region='handle-wood', pen=0.13)
    return rep(500, 350, 1000, pens(0.13, None, None, None, 0.18), s, [
        ('A kettle sign 1.3 mm wide and 1.9 mm high: a dark lid dome over a bowl cup on two leg strokes, with a handle stroke to the right.', 'Ein Kesselzeichen, 1,3 mm breit und 1,9 mm hoch: eine dunkle Deckelkuppel über einem Kesselnapf auf zwei Beinstrichen, mit einem Griffstrich nach rechts.'),
        ('Lid and bowl at their real diameter 0.585 m, the tripod legs at their real spread, the lid handle reaching to 0.38 m, the dial on the lid top.', 'Deckel und Kessel in realem Durchmesser 0,585 m, die Dreibeinbeine in realer Spreizung, der Deckelgriff bis 0,38 m, die Scheibe auf dem Deckel.'),
        ('Lid and vent dial form one solid; bowl and ash catcher form one solid; the rim bead merges into the lid-bowl joint.', 'Deckel und Lüftungsscheibe bilden einen Körper; Kessel und Aschefänger bilden einen Körper; der Randwulst geht in der Deckel-Kessel-Fuge auf.'),
        ('Grate, charcoal bed, damper, wheel, side handle, leg ring (the grate and bed are hidden anyway).', 'Rost, Kohlebett, Schieber, Rad, Seitengriff, Beinring (Rost und Bett sind ohnehin verdeckt).'),
        ('The 0.02 m legs (0.04 mm) and the 0.03 m handle are single 0.13 mm strokes, inflation 3 to 4 times; lid and bowl overlap by 0.01 m so they print as one joined sign.', 'Die 0,02 m dicken Beine (0,04 mm) und der 0,03 m dicke Griff sind einzelne 0,13-mm-Striche, Aufblähung 3- bis 4-fach; Deckel und Kessel überlappen um 0,01 m und drucken als ein verbundenes Zeichen.'),
        ('None.', 'Keine.'),
        ('Lid and bowl tone 0.3 (black enamel), strokes 0.13 mm, ground 0.18 mm.', 'Deckel und Kessel Ton 0,3 (schwarzes Email), Striche 0,13 mm, Boden 0,18 mm.'),
        ('The merged lid solid lists lid-shell and vents-alu, the merged bowl solid lists bowl-shell and ash-catcher; the hot lid and bowl shells are the operating regions of the thermal view.', 'Der verschmolzene Deckelkörper nennt lid-shell und vents-alu, der Kesselkörper bowl-shell und ash-catcher; die heißen Deckel- und Kesselschalen sind die Betriebsregionen der Thermalansicht.'),
    ])


def r200():
    s = Shapes(rig)
    s.path('lid', 'lid', LID, z=3.2, tone=0.3, region='lid-shell', material='enamel-steel')
    s.rect('vent-dial', 'lid-vent', -0.04, 0.935, 0.04, 0.97, z=3.4, tone=0.7, region='vents-alu', material='alu-cast')
    s.path('bowl', 'bowl', BOWL, z=3, tone=0.3, region='bowl-shell', material='enamel-steel')
    s.rect('ash-catcher', 'bowl', -0.12, 0.43, 0.12, 0.50, z=2.9, tone=0.6, region='ash-catcher', material='alu-cast')
    common_legs(s, 0, line=True, pen=0.13)
    s.rect('handle', 'lid', 0.29, 0.825, 0.38, 0.875, z=3.5, tone=0.5, region='handle-wood', material='wood-handle')
    s.line('rim-line', 'bowl', [(-RX, RIM), (RX, RIM)], role='detail', z=3.6, region='bowl-shell')
    return rep(200, 150, 350, pens(0.18, 0.13, None, None, 0.25), s, [
        ('A kettle grill 3.4 mm wide and 4.9 mm high: black lid dome with a dial, a bowl with ash catcher, two leg strokes, a wooden lid handle and the rim line.', 'Ein Kugelgrill 3,4 mm breit und 4,9 mm hoch: schwarze Deckelkuppel mit Scheibe, ein Kessel mit Aschefänger, zwei Beinstriche, ein Holzgriff am Deckel und die Randlinie.'),
        ('Dome and bowl profile, lid dial, ash catcher under the bowl, leg spread, handle, rim joint line.', 'Kuppel- und Kesselprofil, Deckelscheibe, Aschefänger unter dem Kessel, Beinspreizung, Griff, Randfugenlinie.'),
        ('Handle bracket and heat shield merge into the handle block; the bowl bead merges into the rim line.', 'Griffhalter und Hitzeschild gehen im Griffblock auf; der Kesselwulst geht in der Randlinie auf.'),
        ('Wheel, leg ring, side handle, grate, charcoal bed.', 'Rad, Beinring, Seitengriff, Rost, Kohlebett.'),
        ('Legs are 0.13 mm strokes (real 0.02 m = 0.1 mm); the lid dial is 0.35 mm tall.', 'Beine sind 0,13-mm-Striche (real 0,02 m = 0,1 mm); die Deckelscheibe ist 0,35 mm hoch.'),
        ('None.', 'Keine.'),
        ('Enamel 0.3, aluminium 0.6-0.7, wood 0.5; body pen 0.18 mm, detail 0.13 mm.', 'Email 0,3, Aluminium 0,6-0,7, Holz 0,5; Körperstift 0,18 mm, Detail 0,13 mm.'),
        ('Lid shell, bowl shell, ash catcher, dial, handle and legs each carry their own region.', 'Deckelschale, Kesselschale, Aschefänger, Scheibe, Griff und Beine tragen je ihre eigene Region.'),
    ])


def r100():
    s = Shapes(rig)
    s.path('lid', 'lid', LID, z=3.2, tone=0.3, region='lid-shell', material='enamel-steel')
    s.rect('vent-dial', 'lid-vent', -0.04, 0.935, 0.04, 0.97, z=3.4, tone=0.7, region='vents-alu', material='alu-cast')
    s.path('bowl', 'bowl', BOWL, z=3, tone=0.3, region='bowl-shell', material='enamel-steel')
    s.path('ash-catcher', 'bowl', 'M -0.12 0.50 L -0.12 0.44 C -0.12 0.425 -0.09 0.42 -0.08 0.42 L 0.08 0.42 C 0.09 0.42 0.12 0.425 0.12 0.44 L 0.12 0.50 Z', z=2.9, tone=0.6, region='ash-catcher', material='alu-cast')
    common_legs(s, 0.012)
    s.line('leg-ring', 'bowl', [(-0.235, 0.22), (0.235, 0.22)], role='detail', z=2.4, region='legs-steel')
    s.ell('wheel', 'bowl', 0.27, 0.05, 0.05, z=2.3, tone=0.9, region='legs-steel', material='steel-black')
    s.path('handle', 'lid', 'M 0.255 0.835 L 0.30 0.84 L 0.30 0.82 L 0.385 0.82 L 0.385 0.88 L 0.30 0.88 L 0.30 0.865 L 0.255 0.87 Z', z=3.5, tone=0.5, region='handle-wood', material='wood-handle')
    s.path('side-handle', 'bowl', 'M 0.2925 0.715 L 0.345 0.715 L 0.345 0.745 L 0.2925 0.745 Z', z=3.4, tone=0.5, region='handle-wood', material='wood-handle')
    s.line('rim-line', 'bowl', [(-RX, RIM), (RX, RIM)], role='detail', z=3.6, region='bowl-shell')
    s.line('lid-bead', 'lid', [(-0.27, 0.80), (-0.13, 0.905)], role='detail', z=3.6, region='lid-shell')
    return rep(100, 75, 150, pens(0.25, 0.13, None, None, 0.35), s, [
        ('A kettle grill 6.7 mm wide and 9.7 mm high: lid dome with dial and highlight line, bowl with rim line and ash catcher, two legs with a ring and a wheel, a stepped lid handle and a side handle.', 'Ein Kugelgrill 6,7 mm breit und 9,7 mm hoch: Deckelkuppel mit Scheibe und Lichtlinie, Kessel mit Randlinie und Aschefänger, zwei Beine mit Ring und Rad, ein abgesetzter Deckelgriff und ein Seitengriff.'),
        ('Lid, dial, bowl, rim, ash catcher, leg tubes at real 0.024 m, leg ring, wheel, both handles.', 'Deckel, Scheibe, Kessel, Rand, Aschefänger, Beinrohre in realen 0,024 m, Beinring, Rad, beide Griffe.'),
        ('Third leg hidden behind the pair; heat shield and bracket merge into the handle.', 'Das dritte Bein ist hinter dem Paar verdeckt; Hitzeschild und Halter gehen im Griff auf.'),
        ('Grate, charcoal bed, damper slot, lid vent holes.', 'Rost, Kohlebett, Schieberschlitz, Lüftungslöcher des Deckels.'),
        ('Leg tubes are 0.24 mm solid capsules; the ring wire is a 0.13 mm line; the handle block 0.6 mm thick.', 'Beinrohre sind 0,24-mm-Vollkapseln; der Ringdraht ist eine 0,13-mm-Linie; der Griffblock ist 0,6 mm stark.'),
        ('None; enamel is solid black.', 'Keine; Email ist voll schwarz.'),
        ('Enamel 0.3, legs 0.3, aluminium 0.6-0.7, wood 0.5; body pen 0.25 mm, detail 0.13 mm.', 'Email 0,3, Beine 0,3, Aluminium 0,6-0,7, Holz 0,5; Körperstift 0,25 mm, Detail 0,13 mm.'),
        ('Wheel and ring carry legs-steel; both handles handle-wood; the highlight line lid-shell.', 'Rad und Ring tragen legs-steel; beide Griffe handle-wood; die Lichtlinie lid-shell.'),
    ])


def r50():
    s = Shapes(rig)
    s.path('lid', 'lid', LID, z=3.2, tone=0.3, region='lid-shell', material='enamel-steel')
    s.line('lid-bead', 'lid', [(-0.28, 0.79), (-0.22, 0.87), (-0.12, 0.915)], role='texture', z=3.6, region='lid-shell')
    s.line('lid-rim', 'lid', [(-0.2925, 0.752), (0.2925, 0.752)], role='detail', z=3.6, region='lid-shell')
    s.rect('vent-dial', 'lid-vent', -0.04, 0.935, 0.04, 0.97, r=0.006, z=3.4, tone=0.7, region='vents-alu', material='alu-cast')
    for i, x in enumerate((-0.022, 0.0, 0.022)):
        s.ell(f'vent-hole-{i}', 'lid-vent', x, 0.9525, 0.006, z=3.5, tone=0.1, role='detail', region='vents-alu')
    s.path('bowl', 'bowl', BOWL, z=3, tone=0.3, region='bowl-shell', material='enamel-steel')
    s.line('rim-line', 'bowl', [(-RX, RIM), (RX, RIM)], role='detail', z=3.6, region='bowl-shell')
    s.line('bowl-highlight', 'bowl', [(-0.25, 0.68), (-0.17, 0.585), (-0.08, 0.535)], role='texture', z=3.6, region='bowl-shell')
    s.path('ash-catcher', 'bowl', 'M -0.12 0.50 L -0.12 0.44 C -0.12 0.425 -0.09 0.42 -0.08 0.42 L 0.08 0.42 C 0.09 0.42 0.12 0.425 0.12 0.44 L 0.12 0.50 Z', z=2.9, tone=0.6, region='ash-catcher', material='alu-cast')
    s.path('damper', 'ash-damper', 'M -0.05 0.495 L 0.05 0.495 L 0.05 0.47 L -0.05 0.47 Z', z=3.0, tone=0.8, role='detail', region='vents-alu', filled=False)
    common_legs(s, 0.012)
    s.line('leg-ring', 'bowl', [(-0.235, 0.22), (0.235, 0.22)], role='detail', z=2.4, region='legs-steel')
    s.ell('wheel', 'bowl', 0.27, 0.05, 0.05, z=2.3, tone=0.9, region='legs-steel', material='steel-black')
    s.ell('wheel-hub', 'bowl', 0.27, 0.05, 0.016, z=2.4, tone=0.2, role='detail', region='legs-steel')
    s.path('handle', 'lid', 'M 0.255 0.835 L 0.30 0.84 L 0.30 0.82 L 0.385 0.82 L 0.385 0.88 L 0.30 0.88 L 0.30 0.865 L 0.255 0.87 Z', z=3.5, tone=0.5, region='handle-wood', material='wood-handle', hatch='grain')
    s.path('side-handle', 'bowl', 'M 0.2925 0.715 L 0.345 0.715 L 0.345 0.745 L 0.2925 0.745 Z', z=3.4, tone=0.5, region='handle-wood', material='wood-handle', hatch='grain')
    return rep(50, 20, 75, pens(0.35, 0.18, 0.13, 0.13, 0.5), s, [
        ('A kettle grill 13.4 mm wide and 19.4 mm high with its construction: lid with rim, bead, dial with three air holes, bowl with rim, highlight and ash catcher with damper, tubular legs with ring and wheel hub, wooden lid and side handles with grain.', 'Ein Kugelgrill 13,4 mm breit und 19,4 mm hoch mit seiner Konstruktion: Deckel mit Rand, Wulst, Scheibe mit drei Luftlöchern, Kessel mit Rand, Lichtlinie und Aschefänger mit Schieber, Rohrbeine mit Ring und Radnabe, hölzerne Deckel- und Seitengriffe mit Maserung.'),
        ('Every visible part at real size; the dial turns with the lid-vent bone, the damper with the ash-damper bone.', 'Jedes sichtbare Teil in realer Größe; die Scheibe dreht mit dem Lid-Vent-Knochen, der Schieber mit dem Ash-Damper-Knochen.'),
        ('The third leg and the far wheel are hidden behind the near ones; grate and bed are inside the bowl.', 'Das dritte Bein und das ferne Rad sind hinter den nahen verdeckt; Rost und Bett liegen im Kessel.'),
        ('Handle screws, firmware stamp, bowl drain slot.', 'Griffschrauben, Prägung, Kesselablassschlitz.'),
        ('Air holes are 0.012 m (0.24 mm) circles; legs are 0.024 m (0.48 mm) capsules; ring wire a 0.18 mm line.', 'Luftlöcher sind Kreise von 0,012 m (0,24 mm); Beine sind 0,024-m-Kapseln (0,48 mm); der Ringdraht eine 0,18-mm-Linie.'),
        ('Wood grain as parallel lines at 0.5 mm spacing on both handles.', 'Holzmaserung als parallele Linien im Abstand 0,5 mm auf beiden Griffen.'),
        ('Enamel 0.3, wood 0.5, aluminium 0.7; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.', 'Email 0,3, Holz 0,5, Aluminium 0,7; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm.'),
        ('Lines carry the region of the part they mark; the thermal view paints the hot lid and bowl shells.', 'Linien tragen die Region des Teils, das sie kennzeichnen; die Thermalansicht färbt die heißen Deckel- und Kesselschalen.'),
    ], [hatch('grain', 'Wood grain', 'Holzmaserung', 'lines', 0, 0.5, 0.13)])


ES = lambda lid, t, k, m, th, kk, rho, cp: layer(lid, T(*t), k, m, th, kk, rho, cp, 'authored-estimate', 'iso-10456-2007')
HID = T('Inside the closed bowl: not visible from the side in any pose; its state is read numerically and in the thermal overlay of the bowl shell above it.', 'Im geschlossenen Kessel: in keiner Pose von der Seite sichtbar; sein Zustand wird numerisch und über die Thermalansicht der darüberliegenden Kesselschale gelesen.')
regions = [
    region('bowl-shell', T('Bowl shell (outside)', 'Kesselschale (außen)'), 'surface', 'operating', 'enamel-steel', 0.45, 0.585, 'sphere', 0.5, [ES('shell', ('Enamelled steel sheet 0.8 mm', 'Emailliertes Stahlblech 0,8 mm'), 'metal', 'enamel-steel', 0.0008, 50, 7850, 450)], 'authored-estimate', 'incropera-2018', operating=('charcoal', 1800)),
    region('lid-shell', T('Lid shell (outside)', 'Deckelschale (außen)'), 'surface', 'operating', 'enamel-steel', 0.40, 0.585, 'sphere', 0.9, [ES('shell', ('Enamelled steel sheet 0.8 mm', 'Emailliertes Stahlblech 0,8 mm'), 'metal', 'enamel-steel', 0.0008, 50, 7850, 450)], 'authored-estimate', 'incropera-2018', operating=('lid-heat', 1200)),
    region('legs-steel', T('Tripod legs, ring and wheel', 'Dreibein, Ring und Rad'), 'surface', 'passive', 'steel-black', 0.30, 0.02, 'cylinder', 0.4, [ES('tube', ('Painted steel tube 1.5 mm', 'Lackiertes Stahlrohr 1,5 mm'), 'metal', 'steel-black', 0.0015, 50, 7850, 450)], 'authored-estimate', 'incropera-2018'),
    region('ash-catcher', T('Ash catcher pan', 'Aschefängerschale'), 'surface', 'passive', 'alu-cast', 0.12, 0.25, 'horizontal-plate-down', 0.3, [ES('pan', ('Cast aluminium pan 2 mm', 'Aluminiumgussschale 2 mm'), 'metal', 'alu-cast', 0.002, 160, 2700, 880)], 'authored-estimate', 'incropera-2018'),
    region('vents-alu', T('Vent dial and air damper', 'Lüftungsscheibe und Luftschieber'), 'surface', 'passive', 'alu-cast', 0.01, 0.08, 'horizontal-plate-up', 0.8, [ES('plate', ('Cast aluminium 3 mm', 'Aluminiumguss 3 mm'), 'metal', 'alu-cast', 0.003, 160, 2700, 880)], 'authored-estimate', 'iso-10456-2007'),
    region('handle-wood', T('Wooden handles', 'Holzgriffe'), 'surface', 'passive', 'wood-handle', 0.03, 0.08, 'cylinder', 0.6, [ES('wood', ('Wood 25 mm (active 10 mm)', 'Holz 25 mm (wirksam 10 mm)'), 'timber', 'wood-handle', 0.025, 0.15, 700, 1600)], 'authored-estimate', 'iso-10456-2007'),
    region('cooking-grate', T('Cooking grate', 'Grillrost'), 'surface', 'passive', 'chrome-steel', 0.30, 0.01, 'cylinder', 0.3, [ES('rods', ('Chrome-plated steel rods 6 mm', 'Verchromte Stahlstäbe 6 mm'), 'metal', 'chrome-steel', 0.006, 50, 7850, 450)], 'authored-estimate', 'incropera-2018', hidden=HID),
    region('charcoal-bed', T('Glowing charcoal bed', 'Glühende Holzkohleglut'), 'heater', 'operating', 'charcoal', 0.12, 0.39, 'horizontal-plate-up', 0.3, [ES('embers', ('Ember layer 30 mm (active 20 mm)', 'Glutschicht 30 mm (wirksam 20 mm)'), 'masonry', 'charcoal', 0.03, 0.2, 250, 1000)], 'authored-estimate', 'turns-2012', operating=('charcoal', 3600), hidden=HID),
]
RID = [r['id'] for r in regions]
OPK = [
    {'id': 'charcoal', 'title': T('Charcoal fire', 'Holzkohlefeuer'), 'levels': [{'v': 0, 'title': T('Cold', 'Kalt')}, {'v': 0.3, 'title': T('Embers (2.4 kW fuel)', 'Glut (2,4 kW Brennstoff)')}, {'v': 1, 'title': T('Hot grilling fire (8 kW fuel)', 'Heißes Grillfeuer (8 kW Brennstoff)')}]},
    {'id': 'lid-heat', 'title': T('Heat reaching the lid shell', 'Wärme an der Deckelschale'), 'levels': [{'v': 0, 'title': T('None (lid off)', 'Keine (Deckel ab)')}, {'v': 0.2, 'title': T('Lid raised', 'Deckel angehoben')}, {'v': 1, 'title': T('Lid closed', 'Deckel geschlossen')}]},
]
emitters = [
    {'id': 'lid-vent-flue', 'kind': 'flue', 'title': T('Smoke and flue gas through the lid vent', 'Rauch und Abgas durch die Deckelöffnung'), 'anchor': 'vent-top', 'directionDeg': 90, 'diameterM': 0.05, 'operatingKey': 'charcoal', 'volumeFlowM3S': 0.0086, 'temperatureRiseK': 220, 'vapourKgPerKg': 0.003, 'source': 'authored-estimate', 'reference': 'turns-2012'},
    {'id': 'open-bowl-plume', 'kind': 'vent', 'title': T('Hot air and smoke over the open bowl', 'Heiße Luft und Rauch über dem offenen Kessel'), 'anchor': 'bowl-mouth', 'directionDeg': 90, 'diameterM': 0.40, 'operatingKey': 'charcoal', 'volumeFlowM3S': 0.027, 'temperatureRiseK': 150, 'vapourKgPerKg': 0.004, 'source': 'authored-estimate', 'reference': 'turns-2012'},
]
SUNC = {'bowl-shell': 0.5, 'lid-shell': 1.0, 'legs-steel': 0.3, 'ash-catcher': 0.0, 'vents-alu': 1.0, 'handle-wood': 0.6, 'cooking-grate': 0.0, 'charcoal-bed': 0.0}
food = load('food', T('Food on the grate (5 kg)', 'Grillgut auf dem Rost (5 kg)'), 'carried', 0, -49, 'authored-estimate', 'en-1860-1', anchor='grate-point')
B = ['leg-left', 'leg-right']
def hb(ch, lh, fl, op, sun=None, loads=None):
    return bind(RID, sunmap=sun or {}, emitters={'lid-vent-flue': fl, 'open-bowl-plume': op}, operating={'charcoal': ch, 'lid-heat': lh})
poses = [
    pose('cold-closed', T('Cold, lid closed', 'Kalt, Deckel geschlossen'), T('Unlit grill with closed lid and open vents in the afternoon sun.', 'Unbefeuerter Grill mit geschlossenem Deckel und offenen Lüftungen in der Nachmittagssonne.'), {}, B, hb(0, 0, 0, 0, SUNC)),
    pose('lit-lid-open', T('Lit, lid raised', 'Angezündet, Deckel angehoben'), T('Burning charcoal with the lid swung up 105 degrees: hot air and smoke rise freely from the open bowl.', 'Brennende Holzkohle mit um 105 Grad hochgeschwenktem Deckel: heiße Luft und Rauch steigen frei aus dem offenen Kessel.'), {'lid': {'angleDeg': 105}}, B, hb(1, 0.2, 0, 1)),
    pose('grilling-closed', T('Grilling, lid closed', 'Grillen, Deckel geschlossen'), T('Hot fire under a closed lid with open vents and five kilograms of food on the grate: the lid shell heats and smoke leaves through the lid vent.', 'Heißes Feuer unter geschlossenem Deckel mit offenen Lüftungen und fünf Kilogramm Grillgut auf dem Rost: die Deckelschale heizt sich auf und Rauch verlässt den Grill durch die Deckelöffnung.'), {}, B, hb(1, 1, 1, 0), [food]),
    pose('embers-vents-low', T('Embers, vents nearly closed', 'Glut, Lüftungen fast geschlossen'), T('Slow-burning embers with the lid dial and the bottom damper turned to 70 degrees: little air, low heat, a weak smoke plume.', 'Langsam brennende Glut mit auf 70 Grad gedrehter Deckelscheibe und Unterluftschieber: wenig Luft, geringe Wärme, eine schwache Rauchfahne.'), {'lid-vent': {'angleDeg': 70}, 'ash-damper': {'angleDeg': 70}}, B, hb(0.3, 0.35, 0.5, 0)),
]
clips = [
    clip('light-and-close', T('Light the fire and close the lid', 'Feuer anzünden und Deckel schließen'), T('The charcoal is lit with the lid raised, builds up over five minutes, then the lid closes and the shell heats during half an hour.', 'Die Holzkohle wird bei angehobenem Deckel angezündet, baut sich über fünf Minuten auf, dann schließt der Deckel und die Schale heizt sich in einer halben Stunde auf.'),
         1800, 1, False, [(0, 'lit-lid-open', 'smooth'), (300, 'lit-lid-open', 'smooth'), (306, 'grilling-closed', 'smooth'), (1800, 'grilling-closed', 'smooth')], [('leg-left', 0, 1800), ('leg-right', 0, 1800)],
         [('light', 0, 300, T('Lighting, lid raised', 'Anzünden, Deckel angehoben')), ('close', 300, 306, T('Lid closes', 'Deckel schließt')), ('grill', 306, 1800, T('Grilling, lid closed', 'Grillen, Deckel geschlossen'))],
         [(0, 'ignition', T('Charcoal ignites', 'Holzkohle zündet'), 'charcoal'), (303, 'motion', T('Lid swings shut', 'Deckel schwenkt zu'), None)],
         [('operating:charcoal', 'smooth', [(0, 0.3), (300, 1), (1800, 1)]), ('operating:lid-heat', 'smooth', [(0, 0.2), (300, 0.2), (306, 1), (1800, 1)])]),
    clip('lid-lift-cycle', T('Lift the lid and close it again', 'Deckel heben und wieder schließen'), T('A 20 s loop: the lid is lifted to turn the food and put back; smoke switches between the lid vent and the open bowl.', 'Eine 20-s-Schleife: der Deckel wird zum Wenden des Grillguts angehoben und wieder aufgesetzt; der Rauch wechselt zwischen Deckelöffnung und offenem Kessel.'),
         20, 12, True, [(0, 'grilling-closed', 'smooth'), (6, 'lit-lid-open', 'smooth'), (14, 'lit-lid-open', 'smooth'), (20, 'grilling-closed', 'smooth')], [('leg-left', 0, 20), ('leg-right', 0, 20)],
         [('lift', 0, 6, T('Lifting', 'Anheben')), ('turn', 6, 14, T('Turning the food', 'Wenden')), ('lower', 14, 20, T('Lowering', 'Aufsetzen'))],
         [(6, 'motion', T('Lid fully open', 'Deckel ganz offen'), None), (14, 'motion', T('Lid comes back', 'Deckel kommt zurück'), None)], []),
]
asset = {
    'id': 'charcoal-grill', 'category': 'objects',
    'title': T('Charcoal Kettle Grill', 'Holzkohle-Kugelgrill'),
    'summary': T('A 57 cm charcoal kettle grill on three legs: lid closed or raised, a glowing charcoal bed inside, hot enamel shells and smoke leaving through the lid vent or over the open bowl.', 'Ein Holzkohle-Kugelgrill von 57 cm auf drei Beinen: Deckel geschlossen oder angehoben, glühende Kohle im Inneren, heiße Emailleschalen und Rauch, der durch die Deckelöffnung oder über den offenen Kessel abzieht.'),
    'tags': ['charcoal-grill', 'kettle-grill', 'barbecue', 'fire', 'smoke', 'outdoor', 'lid'],
    'provenance': {'author': 'Semio staffage content (NW-O)', 'method': 'handcrafted', 'created': '2026-10-29', 'references': refs,
                   'notes': T('Side elevation of a 57 cm kettle grill (mass 14 kg, typical class data, authored estimates). The lid is lifted off its rim hook in reality; the rig hinges it at the left rim to pose open and closed. The fire is 8 kW fuel (about 1 kg charcoal per hour at 29 MJ/kg): 3.6 kW released into the glowing bed, 1.8 kW into the bowl shell, 1.2 kW into the lid shell (closed), 1.4 kW sensible heat in the flue gas (0.0086 m3/s at 220 K), the rest to the food. Bed and grate lie inside the bowl and are hidden in the side elevation.', 'Seitenansicht eines 57-cm-Kugelgrills (Masse 14 kg, typische Klassenwerte, Schätzwerte). Der Deckel wird in Wirklichkeit vom Randhaken gehoben; das Rig lenkt ihn am linken Rand an, um offen und geschlossen darzustellen. Das Feuer hat 8 kW Brennstoff (etwa 1 kg Holzkohle je Stunde bei 29 MJ/kg): 3,6 kW in die Glut, 1,8 kW in die Kesselschale, 1,2 kW in die Deckelschale (geschlossen), 1,4 kW fühlbare Wärme im Abgas (0,0086 m3/s bei 220 K), der Rest ins Grillgut. Bett und Rost liegen im Kessel und sind in der Seitenansicht verdeckt.')},
    'dimensions': {'heightM': 0.97, 'widthM': 0.67, 'depthM': 0.64, 'massKg': round(rig.total_mass(), 2)},
    'materials': mats, 'rig': rig.rig(), 'anchors': anchors,
    'representations': {'500': r500(), '200': r200(), '100': r100(), '50': r50()}, 'symbols': [],
    'poses': poses, 'clips': clips,
    'thermal': {'environment': environment(20, 22, 0.55, 1.0, 700, 0.15, sky=8, ground=24), 'operating': OPK, 'passengers': [], 'regions': regions, 'emitters': emitters,
                'assumptions': [T('Bed, bowl shell and lid shell are operating regions heated by the stated watts at level 1; the lid-heat key is 1 with the lid closed, 0.2 raised and 0 off. Flue and open-bowl flows are sized from the sensible heat of the 8 kW fire.', 'Bett, Kessel- und Deckelschale sind Betriebsregionen, geheizt mit den genannten Watt bei Stufe 1; der Schlüssel lid-heat ist 1 bei geschlossenem, 0,2 bei angehobenem und 0 bei abgenommenem Deckel. Abgas- und Offenkesselströme sind aus der fühlbaren Wärme des 8-kW-Feuers bemessen.'),
                                T('Smoke particles are not modelled; the vapour share covers moisture and hydrogen of the charcoal.', 'Rauchpartikel werden nicht modelliert; der Dampfanteil deckt Feuchte und Wasserstoff der Holzkohle.')]},
    'structure': structure(B, [('A rigid tripod body on two leg supports in the plane (the third leg coincides with the pair in elevation); the raised lid shifts the centre of mass by less than the leg span.', 'Ein starrer Dreibeinkörper auf zwei Beinauflagern in der Ebene (das dritte Bein fällt in der Ansicht mit dem Paar zusammen); der angehobene Deckel verschiebt den Schwerpunkt weniger als die Beinspreizung.'),
                              ('Food load 5 kg on the grate is an authored estimate.', 'Die Grillgutlast von 5 kg ist eine Schätzung.')]),
    'presentation': {'defaultPose': 'grilling-closed', 'defaultClip': 'light-and-close', 'defaultScale': 100, 'viewpoints': ['elevation']},
    'exports': ENV_EXPORTS, 'completion': 'authored',
}
save(asset)
print('mass', rig.total_mass())
