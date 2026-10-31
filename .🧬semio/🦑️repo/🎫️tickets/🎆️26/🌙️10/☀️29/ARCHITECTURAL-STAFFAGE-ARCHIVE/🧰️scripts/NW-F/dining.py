import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nwf import *

TOP_Y0, TOP_Y1, AP_Y0, LEAF_Y0 = 0.72, 0.75, 0.64, 0.69
bones = [
    bone('ground', None, 'Floor reference', 'Bodenbezug', 0, 0, 0, 1.6, FIXED),
    bone('frame', 'ground', 'Slide rails', 'Auszugsschienen', 0, 0, 0, 1.0, FIXED, (5.0, 0.8, 0.69, 'point', 'authored-estimate', 'wood-handbook-2021')),
    bone('top-left', 'ground', 'Left table half (top, apron, legs)', 'Linke Tischhälfte (Platte, Zarge, Beine)', 0, 0, 0, 0.8, prism(0, -0.30, 0), (22.1, 0.4, 0.38, 'point', 'authored-estimate', 'wood-handbook-2021')),
    bone('top-right', 'ground', 'Right table half (top, apron, legs)', 'Rechte Tischhälfte (Platte, Zarge, Beine)', 0.8, 0, 0, 0.8, prism(0, 0, 0.30), (22.1, 0.4, 0.38, 'point', 'authored-estimate', 'wood-handbook-2021')),
    bone('leaf', 'ground', 'Extension leaf', 'Einlegeplatte', 0.5, LEAF_Y0, 0, 0.6, prism(90, 0, 0.03), (11.3, 0.3, 0.015, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
]
rig = Rig(bones)
RIG = {'kind': 'mechanical', 'root': 'ground', 'bones': bones}
mass = sum(b['mass']['kg'] for b in bones if 'mass' in b)

oak = mat('oak-timber', 'Solid oak, oiled', 'Eiche massiv, geölt', 0.9, 0.6, 700, 0.18, 1600, 1.1e10, 4.0e7, 'literature', 'wood-handbook-2021')
oak['source'] = 'authored-estimate'
steel = mat('steel-slides', 'Zinc-plated steel slides', 'Verzinkte Stahlschienen', 0.25, 0.5, 7800, 50, 450, 2.1e11, 3.55e8, 'literature', 'en-1993-1-1')
MATS = [oak, steel]
cap_oak = lambda t: lay('o', '', '', 'timber', 'oak-timber', t, oak)
def oakl(i, en, de, t):
    return lay(i, en, de, 'timber', 'oak-timber', t, oak, 'literature')
regions = [
    reg('table-top', 'Table top halves', 'Tischplattenhälften', 'surface', 'oak-timber', 1.44, 0.9, 'horizontal-plate-up', 0.85, [oakl('top-plank', 'Oak plank 30 mm', 'Eichenbohle 30 mm', 0.03)]),
    reg('leaf-top', 'Extension leaf', 'Einlegeplatte', 'surface', 'oak-timber', 0.54, 0.6, 'horizontal-plate-up', 0.85, [oakl('leaf-plank', 'Oak plank 30 mm', 'Eichenbohle 30 mm', 0.03)]),
    reg('apron', 'Aprons', 'Zargen', 'surface', 'oak-timber', 0.42, 0.08, 'vertical-plate', 0.45, [oakl('apron-board', 'Oak board 25 mm', 'Eichenbrett 25 mm', 0.025)]),
    reg('legs', 'Tapered legs', 'Konische Beine', 'surface', 'oak-timber', 0.75, 0.06, 'cylinder', 0.3, [oakl('leg-solid', 'Oak leg, half thickness 30 mm', 'Eichenbein, halbe Dicke 30 mm', 0.03)]),
    reg('slide-rails', 'Slide rails', 'Auszugsschienen', 'surface', 'steel-slides', 0.15, 0.06, 'horizontal-plate-down', 0.2,
        [lay('rail-steel', 'Steel rail 2 mm', 'Stahlschiene 2 mm', 'metal', 'steel-slides', 0.002, steel, 'literature')]),
]
RIDS = [r['id'] for r in regions]
ctx = Ctx(RIDS, {'table-top': (1.0, 0.3), 'leaf-top': (0.0, 0.0), 'apron': (1.0, 0.0), 'legs': (1.0, 0.0), 'slide-rails': (0.0, 0.0)})
anchors = [
    anc(rig, 'leg-left', 'top-left', 0.135, 0, 'contact', 'Left leg pair (front and rear coincide)', 'Linkes Beinpaar (vorn und hinten fallen zusammen)', support='contact', normalDeg=90, footprintLengthM=0.045, footprintWidthM=0.80, frictionCoefficient=0.5),
    anc(rig, 'leg-right', 'top-right', 1.465, 0, 'contact', 'Right leg pair (front and rear coincide)', 'Rechtes Beinpaar (vorn und hinten fallen zusammen)', support='contact', normalDeg=90, footprintLengthM=0.045, footprintWidthM=0.80, frictionCoefficient=0.5),
    anc(rig, 'edge-right', 'top-right', 1.55, TOP_Y1, 'load', 'Test point at the right table edge', 'Prüfpunkt am rechten Tischrand'),
]
CONT = ['leg-left', 'leg-right']
EXT = {'top-left': {'dx': -0.30}, 'top-right': {'dx': 0.30}, 'leaf': {'dy': 0.03}}
dist = lambda b, to, w: {'bone': b, 'fromM': 0, 'toM': to, 'wxNPerM': 0, 'wyNPerM': w}
dinner = lambda: [load('dinner-left', 'Table setting on the left half (0.5 kN/m2)', 'Gedeck auf der linken Hälfte (0,5 kN/m2)', 'live', 0, dist=dist('top-left', 0.8, -450), ref='en-1991-1-1'),
                  load('dinner-leaf', 'Table setting on the leaf (0.5 kN/m2)', 'Gedeck auf der Einlegeplatte (0,5 kN/m2)', 'live', 0, dist=dist('leaf', 0.6, -450), ref='en-1991-1-1'),
                  load('dinner-right', 'Table setting on the right half (0.5 kN/m2)', 'Gedeck auf der rechten Hälfte (0,5 kN/m2)', 'live', 0, dist=dist('top-right', 0.8, -450), ref='en-1991-1-1')]
P = [
    pose(ctx, 'closed', 'Closed', 'Geschlossen', 'Table closed at 1.60 m, the leaf stored under the top; a window sun patch lies on the left half.', 'Tisch geschlossen mit 1,60 m, die Einlegeplatte unter der Platte verstaut; ein Fensterlichtfleck liegt auf der linken Hälfte.', contacts=CONT),
    pose(ctx, 'extended', 'Extended with leaf', 'Ausgezogen mit Einlegeplatte', 'Halves pulled apart by 0.30 m each, the leaf raised flush: table length 2.20 m.', 'Hälften je 0,30 m auseinandergezogen, Einlegeplatte bündig angehoben: Tischlänge 2,20 m.', bones=EXT, contacts=CONT,
         over={'table-top': (1.0, 0.25), 'leaf-top': (1.0, 0.5), 'slide-rails': (0.8, 0.0)}),
    pose(ctx, 'set-for-dinner', 'Set for dinner', 'Zum Essen gedeckt', 'Extended table with six place settings; forearms of the guests rest on the top edge.', 'Ausgezogener Tisch mit sechs Gedecken; Unterarme der Gäste liegen am Plattenrand auf.', bones=EXT, contacts=CONT, loads=dinner(),
         over={'table-top': (1.0, 0.15), 'leaf-top': (1.0, 0.3), 'slide-rails': (0.8, 0.0)}, contact={'table-top': (0.05,), 'leaf-top': (0.03,)}),
    pose(ctx, 'edge-test-load', 'Edge test load', 'Randprüflast', 'Closed table with a vertical test load of 750 N at the table edge.', 'Geschlossener Tisch mit senkrechter Prüflast von 750 N am Tischrand.', contacts=CONT,
         loads=[load('edge-test', 'Vertical test load at the edge (750 N)', 'Senkrechte Prüflast am Rand (750 N)', 'imposed', -750, anchor='edge-right', ref='en-1730')]),
    pose(ctx, 'pulled-apart-key', 'Halves apart, leaf stored (key)', 'Hälften auseinander, Platte verstaut (Schlüssel)', 'Halves pulled apart, leaf still lying on the rails.', 'Hälften auseinandergezogen, Einlegeplatte liegt noch auf den Schienen.', key=True, bones={'top-left': {'dx': -0.30}, 'top-right': {'dx': 0.30}}, contacts=CONT,
         over={'table-top': (1.0, 0.25), 'leaf-top': (0.0, 0.0), 'slide-rails': (0.8, 0.0)}),
]
clips = [
    clip('extend-table', 'Extend the table', 'Tisch ausziehen', 'The halves are pulled apart on the slide rails, then the leaf is lifted into the gap.', 'Die Hälften werden auf den Schienen auseinandergezogen, dann wird die Einlegeplatte in die Lücke gehoben.', 4.0,
         [{'t': 0, 'pose': 'closed', 'ease': 'smooth'}, {'t': 2.4, 'pose': 'pulled-apart-key', 'ease': 'smooth'}, {'t': 3.4, 'pose': 'extended', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'extended', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 4.0} for a in CONT], [('pull', 0, 2.4, 'Pull apart', 'Auseinanderziehen'), ('lift', 2.4, 4.0, 'Lift the leaf', 'Einlegeplatte anheben')],
         [(2.4, 'motion', 'Leaf rises into the gap', 'Einlegeplatte hebt sich in die Lücke', None)], [('region:leaf-top:exposedFraction', 'smooth', [(0, 0), (2.4, 0), (3.4, 1)])]),
    clip('retract-table', 'Close the table', 'Tisch schließen', 'The leaf is lowered, then the halves are pushed together.', 'Die Einlegeplatte wird abgesenkt, dann werden die Hälften zusammengeschoben.', 4.0,
         [{'t': 0, 'pose': 'extended', 'ease': 'smooth'}, {'t': 1.0, 'pose': 'pulled-apart-key', 'ease': 'smooth'}, {'t': 3.4, 'pose': 'closed', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'closed', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 4.0} for a in CONT], [('lower', 0, 1.0, 'Lower the leaf', 'Einlegeplatte absenken'), ('push', 1.0, 3.4, 'Push together', 'Zusammenschieben')],
         [(1.0, 'motion', 'Leaf is stored', 'Einlegeplatte verstaut', None)], [('region:leaf-top:exposedFraction', 'smooth', [(0, 1), (1.0, 0), (4.0, 0)])]),
    clip('lay-the-table', 'Lay the table', 'Tisch decken', 'On the extended table the place settings arrive one after another (0.5 kN/m2 in total).', 'Auf dem ausgezogenen Tisch kommen die Gedecke nacheinander (insgesamt 0,5 kN/m2).', 6.0,
         [{'t': 0, 'pose': 'extended', 'ease': 'linear'}, {'t': 6.0, 'pose': 'set-for-dinner', 'ease': 'linear'}],
         [{'anchor': a, 'fromS': 0, 'toS': 6.0} for a in CONT], [('lay', 0, 6.0, 'Laying', 'Decken')],
         [(1.0, 'load-on', 'First place settings', 'Erste Gedecke', 'dinner-left'), (3.0, 'load-on', 'Leaf settings', 'Gedecke auf der Einlegeplatte', 'dinner-leaf'), (5.0, 'load-on', 'Last settings', 'Letzte Gedecke', 'dinner-right')],
         [('load:dinner-left', 'linear', [(0, 0), (1.0, 0), (2.0, 1)]), ('load:dinner-leaf', 'linear', [(0, 0), (3.0, 0), (4.0, 1)]), ('load:dinner-right', 'linear', [(0, 0), (5.0, 0), (6.0, 1)])]),
]
thermal = {'environment': {'airTempC': 21, 'meanRadiantTempC': 21, 'relativeHumidity': 0.5, 'windMS': 0.1, 'windDirectionDeg': 0, 'solarWM2': 350, 'diffuseFraction': 0.15, 'pressurePa': 101325},
           'operating': [], 'regions': regions, 'emitters': [], 'assumptions': [T(*a) for a in [
               ('Solid oak after ISO 10456 (700 kg/m3, 0.18 W/mK, 1600 J/kgK); areal heat capacity rho cp min(t, sqrt(alpha 3600 s)): a 30 mm plank takes part fully (17 kJ/m2K), the 12 mm penetration depth of one hour applies to thicker parts.', 'Massive Eiche nach ISO 10456 (700 kg/m3, 0,18 W/mK, 1600 J/kgK); flächenbezogene Wärmekapazität rho cp min(t, sqrt(alpha 3600 s)): eine 30-mm-Bohle nimmt voll teil (17 kJ/m2K), für dickere Teile gilt die Eindringtiefe einer Stunde.'),
               ('The leaf and the slide rails are exposed to air only in extended poses (exposedFraction 0 when closed: the leaf lies under the top, the rails behind the aprons).', 'Einlegeplatte und Auszugsschienen liegen nur in ausgezogenen Posen an der Luft (exposedFraction 0 im geschlossenen Zustand: die Einlegeplatte liegt unter der Platte, die Schienen hinter den Zargen).'),
               ('Guests rest forearms on the table edge: contact surface at 30 degC (ISO 7730 clothing surface), authored fractions 0.05 and 0.03 of the top area.', 'Gäste legen Unterarme auf den Tischrand: Kontaktfläche 30 degC (Bekleidungsoberfläche nach ISO 7730), Anteile 0,05 und 0,03 der Plattenfläche.'),
               ('Reference case: 350 W/m2 window sun patch behind glazing, 21 degC, still air; sunlit fractions are scenario data per pose.', 'Referenzfall: 350 W/m2 Fensterlichtfleck hinter Verglasung, 21 degC, ruhige Luft; besonnte Anteile sind Szenariodaten je Pose.')]]}
struct = structure(CONT, [
    ('Each table half is a rigid body with its two leg pairs on the floor; the halves slide on the rails (prismatic joints 0.30 m), the leaf lifts by 0.03 m onto the rails.', 'Jede Tischhälfte ist ein starrer Körper mit zwei Beinpaaren am Boden; die Hälften gleiten auf den Schienen (Schiebegelenke 0,30 m), die Einlegeplatte hebt sich um 0,03 m auf die Schienen.'),
    ('Dinner load 0.5 kN/m2 on the top (authored-estimate, deliberately above place settings of six persons; EN 1991-1-1 category A floor loads of 1.5-2.0 kN/m2 describe floors, not tables).', 'Essenslast 0,5 kN/m2 auf der Platte (Schätzwert, bewusst über den Gedecken von sechs Personen; die Bodenlasten 1,5-2,0 kN/m2 der Kategorie A in EN 1991-1-1 beschreiben Böden, nicht Tische).'),
    ('The 750 N vertical edge test load follows the order of the static load tests of EN 1730 and EN 12521; the magnitude is an authored-estimate to be verified against the standard text.', 'Die senkrechte Rand-Prüflast von 750 N folgt der Größenordnung der statischen Lastprüfungen nach EN 1730 und EN 12521; der Betrag ist ein Schätzwert und am Normtext zu prüfen.')])

# geometry
def top(S_, sid, b_, x0, x1, z=6):
    return S_.rect(sid, b_, x0, TOP_Y0, x1, TOP_Y1, region='table-top', material='oak-timber', tone=0.7, z=z)
def leg(S_, sid, b_, x0, x1, top_y=TOP_Y0, narrow=0.045, region='legs', z=3):
    xc = (x0 + x1) / 2
    return S_.poly(sid, b_, [(x0, top_y), (xc - narrow / 2, 0), (xc + narrow / 2, 0), (x1, top_y)] if False else [(xc - narrow / 2, 0), (xc + narrow / 2, 0), (x1 - 0.0, AP_Y0), (x1, top_y), (x0, top_y), (x0, AP_Y0)], region=region, material='oak-timber', tone=0.6, z=z)

# 1:500
s5 = Shapes(rig)
s5.rect('slab-left', 'top-left', 0.0, AP_Y0, 0.8, TOP_Y1, regions=['table-top', 'apron'], material='oak-timber', tone=0.7, z=6)
s5.rect('slab-right', 'top-right', 0.8, AP_Y0, 1.6, TOP_Y1, regions=['table-top', 'apron'], material='oak-timber', tone=0.7, z=6)
s5.rect('leaf-bar', 'leaf', 0.5, 0.63, 1.1, 0.735, regions=['leaf-top', 'slide-rails'], material='oak-timber', tone=0.6, z=5)
s5.line('leg-l', 'top-left', [(0.135, 0.70), (0.135, 0.0)], role='body', penMm=0.18, z=3, region='legs')
s5.line('leg-r', 'top-right', [(1.465, 0.70), (1.465, 0.0)], role='body', penMm=0.18, z=3, region='legs')
# 1:200
s2 = Shapes(rig)
s2.rect('slab-left', 'top-left', 0.0, AP_Y0, 0.8, TOP_Y1, regions=['table-top', 'apron'], material='oak-timber', tone=0.7, z=6)
s2.rect('slab-right', 'top-right', 0.8, AP_Y0, 1.6, TOP_Y1, regions=['table-top', 'apron'], material='oak-timber', tone=0.7, z=6)
s2.rect('leaf-bar', 'leaf', 0.5, 0.63, 1.1, 0.72, regions=['leaf-top', 'slide-rails'], material='oak-timber', tone=0.6, z=5)
leg(s2, 'leg-l', 'top-left', 0.10, 0.17); leg(s2, 'leg-r', 'top-right', 1.43, 1.50)
# 1:100
s1 = Shapes(rig)
top(s1, 'top-l', 'top-left', 0.0, 0.8); top(s1, 'top-r', 'top-right', 0.8, 1.6)
s1.rect('apron-l', 'top-left', 0.10, AP_Y0, 0.8, TOP_Y0, region='apron', material='oak-timber', tone=0.55, z=5)
s1.rect('apron-r', 'top-right', 0.8, AP_Y0, 1.5, TOP_Y0, region='apron', material='oak-timber', tone=0.55, z=5)
s1.rect('leaf', 'leaf', 0.5, LEAF_Y0, 1.1, TOP_Y0, region='leaf-top', material='oak-timber', tone=0.65, z=4)
s1.rect('rail', 'frame', 0.3, 0.66, 1.3, TOP_Y0, region='slide-rails', material='steel-slides', tone=0.35, z=2)
leg(s1, 'leg-l', 'top-left', 0.10, 0.17); leg(s1, 'leg-r', 'top-right', 1.43, 1.50)
# 1:50
s0 = Shapes(rig)
s0.poly('top-l', 'top-left', [(0.0, 0.728), (0.008, 0.72), (0.8, 0.72), (0.8, 0.75), (0.0, 0.75)], region='table-top', material='oak-timber', tone=0.7, z=6)
s0.poly('top-r', 'top-right', [(0.8, 0.72), (1.592, 0.72), (1.6, 0.728), (1.6, 0.75), (0.8, 0.75)], region='table-top', material='oak-timber', tone=0.7, z=6)
s0.rect('apron-l', 'top-left', 0.10, AP_Y0, 0.8, TOP_Y0, region='apron', material='oak-timber', tone=0.55, z=5, hatch='grain-h')
s0.rect('apron-r', 'top-right', 0.8, AP_Y0, 1.5, TOP_Y0, region='apron', material='oak-timber', tone=0.55, z=5, hatch='grain-h')
s0.poly('leaf', 'leaf', [(0.5, LEAF_Y0), (1.1, LEAF_Y0), (1.1, TOP_Y0 - 0.004), (1.094, TOP_Y0), (0.506, TOP_Y0), (0.5, TOP_Y0 - 0.004)], region='leaf-top', material='oak-timber', tone=0.65, z=4)
s0.rect('rail', 'frame', 0.30, 0.66, 1.30, TOP_Y0, region='slide-rails', material='steel-slides', tone=0.35, z=2)
for sid, b_, x0, x1 in (('leg-l', 'top-left', 0.10, 0.17), ('leg-r', 'top-right', 1.43, 1.50)):
    xc = (x0 + x1) / 2
    s0.poly(sid, b_, [(xc - 0.0225, 0.0), (xc + 0.0225, 0.0), (x1, 0.56), (x1, TOP_Y0), (x0, TOP_Y0), (x0, 0.56)], region='legs', material='oak-timber', tone=0.6, z=3, hatch='grain-v')
    s0.line(f'shoulder-{sid[-1]}', b_, [(x0, 0.56), (x1, 0.56)], role='detail', z=3.5, region='legs')
HATCH = [{'id': 'grain-h', 'title': T('Oak grain horizontal', 'Eichenmaserung waagerecht'), 'pattern': 'lines', 'angleDeg': 0, 'spacingMm': 0.6, 'penMm': 0.13},
         {'id': 'grain-v', 'title': T('Oak grain vertical', 'Eichenmaserung senkrecht'), 'pattern': 'lines', 'angleDeg': 90, 'spacingMm': 0.5, 'penMm': 0.13}]

def dec(*a):
    return D(*a)
reps = {
    '500': rep(500, s5, dec(('Table sign 3.2 x 1.5 mm: one slab per half (top and apron merged), a leaf bar between and two leg strokes.', 'Tischzeichen 3,2 x 1,5 mm: je Hälfte eine Platte (Platte und Zarge verschmolzen), ein Einlegebalken dazwischen und zwei Beinstriche.'),
        ('Top level 0.75 m, apron zone, leaf bar, legs.', 'Plattenhöhe 0,75 m, Zargenzone, Einlegebalken, Beine.'),
        ('Top halves with the aprons: one 0.11 m slab each; the leaf with its slide rails is one bar.', 'Plattenhälften mit Zargen: je eine 0,11 m dicke Platte; die Einlegeplatte mit ihren Schienen ist ein Balken.'),
        ('Tapering of the legs, chamfers, rear legs.', 'Verjüngung der Beine, Fasen, hintere Beine.'),
        ('Legs (0.07 m = 0.14 mm) are single strokes of 0.18 mm = 0.09 m real width; the 0.03 m leaf is drawn as part of a 0.09 m bar.', 'Beine (0,07 m = 0,14 mm) sind Einzelstriche von 0,18 mm = 0,09 m Realbreite; die 0,03 m dicke Einlegeplatte ist Teil eines 0,09 m hohen Balkens.'),
        ('None.', 'Keine.'), ('Slabs tone 0.7, leaf bar 0.6.', 'Platten Ton 0,7, Einlegebalken 0,6.'),
        ('Slabs list table-top and apron; the leaf bar lists leaf-top and slide-rails; legs carry their region.', 'Platten führen table-top und apron; der Einlegebalken führt leaf-top und slide-rails; Beine tragen ihre Region.'))),
    '200': rep(200, s2, dec(('Table 8 x 3.8 mm: two slabs, leaf bar and two tapered legs.', 'Tisch 8 x 3,8 mm: zwei Platten, Einlegebalken und zwei konische Beine.'),
        ('Slabs, leaf bar and the tapered legs.', 'Platten, Einlegebalken und die konischen Beine.'),
        ('Top and apron are one slab (0.03 m top = 0.15 mm alone); leaf and rails one bar.', 'Platte und Zarge sind eine Platte (0,03 m Platte allein = 0,15 mm); Einlegeplatte und Schienen ein Balken.'),
        ('Rear legs, joints, rails as own shapes.', 'Hintere Beine, Verbindungen, Schienen als eigene Formen.'),
        ('Leaf bar is 0.09 m thick instead of 0.03 m.', 'Der Einlegebalken ist 0,09 m statt 0,03 m dick.'),
        ('None.', 'Keine.'), ('Slabs 0.7, legs 0.6.', 'Platten 0,7, Beine 0,6.'),
        ('As at 1:500 for slabs and bar; legs bind legs.', 'Wie bei 1:500 für Platten und Balken; die Beine binden legs.'))),
    '100': rep(100, s1, dec(('Table 16 x 7.5 mm: top halves, aprons, stored leaf, slide rails behind and tapered legs.', 'Tisch 16 x 7,5 mm: Plattenhälften, Zargen, verstaute Einlegeplatte, Schienen dahinter und konische Beine.'),
        ('Top planks (0.3 mm), aprons, leaf and rails as own shapes so the extension reads when it opens.', 'Plattenbohlen (0,3 mm), Zargen, Einlegeplatte und Schienen als eigene Formen, damit sich das Ausziehen lesen lässt.'),
        ('Front and rear legs coincide.', 'Vordere und hintere Beine fallen zusammen.'),
        ('Grain, chamfers, slide bearings.', 'Maserung, Fasen, Gleitlager.'),
        ('All members are filled shapes of at least 0.3 mm.', 'Alle Bauteile sind gefüllte Formen von mindestens 0,3 mm.'),
        ('None.', 'Keine.'), ('Top 0.7, apron 0.55, leaf 0.65, rails 0.35.', 'Platte 0,7, Zarge 0,55, Einlegeplatte 0,65, Schienen 0,35.'),
        ('One shape per region.', 'Eine Form je Region.'))),
    '50': rep(50, s0, dec(('Table 32 x 15 mm with all members: planks, aprons with grain, leaf, rails, tapered legs with vertical grain.', 'Tisch 32 x 15 mm mit allen Bauteilen: Bohlen, Zargen mit Maserung, Einlegeplatte, Schienen, konische Beine mit senkrechter Maserung.'),
        ('Chamfered underside edges of the top, bevelled leaf edges, stepped legs (square shoulder to 0.56 m, then tapering), joint at mid-span, aprons set back from the ends, leaf and rails as in 1:100 plus grain hatching.', 'Stoß in Feldmitte, Zargen von den Enden eingerückt, Einlegeplatte und Schienen wie bei 1:100 mit Maserungsschraffur.'),
        ('Front and rear legs coincide.', 'Vordere und hintere Beine fallen zusammen.'),
        ('Corner joints, screws, edge chamfers (below 0.03 m).', 'Eckverbindungen, Schrauben, Kantenfasen (unter 0,03 m).'),
        ('Planks and slides as real thickness.', 'Bohlen und Schienen in Realdicke.'),
        ('Grain lines 0.6 / 0.5 mm spacing, 0.13 mm pen on aprons and legs.', 'Maserungslinien 0,6 / 0,5 mm Abstand, Stift 0,13 mm an Zargen und Beinen.'),
        ('Same tones as 1:100.', 'Töne wie bei 1:100.'), ('One shape per region; hatched shapes keep their region.', 'Eine Form je Region; schraffierte Formen behalten ihre Region.')), hatches=HATCH),
}
for k in reps:
    reps[k]['validRange'] = {'minDenominator': RANGES[k][0], 'maxDenominator': RANGES[k][1]}

SYM = {'id': 'table-plan', 'title': T('Dining table plan symbol', 'Esstisch-Grundrisssymbol'), 'description': T('Floor-plan sign of the closed table 1.60 x 0.90 m with the extension direction marked by the leaf outline (dashed in drawing practice, solid 0.13 mm here).', 'Grundrisszeichen des geschlossenen Tisches 1,60 x 0,90 m mit der Auszugsrichtung durch die Kontur der Einlegeplatte.'),
       'semantics': 'true-extent', 'units': 'model-m', 'denominators': [200, 100, 50], 'pens': {'body': 0.25, 'detail': 0.13, 'texture': None, 'glazing': None, 'ground': 0.35}, 'hatches': [],
       'geometry': {'elevation': [], 'plan': []}, 'dimensionalNote': T('Model metres at real size: x along the table 0 to 1.60 m, y across 0 to 0.90 m; the leaf outline marks the 0.60 m extension.', 'Modellmeter in Originalgröße: x entlang 0 bis 1,60 m, y quer 0 bis 0,90 m; die Kontur der Einlegeplatte markiert den 0,60 m langen Auszug.')}
SYM['geometry']['plan'] += [
    {'id': 'plan-top', 'bone': 'ground', 'role': 'body', 'z': 1, 'filled': True, 'tone': 0.9, 'kind': 'poly', 'points': [[0, 0], [1.6, 0], [1.6, 0.9], [0, 0.9]], 'closed': True},
    {'id': 'plan-leaf', 'bone': 'ground', 'role': 'detail', 'z': 2, 'filled': False, 'tone': 0.5, 'kind': 'poly', 'points': [[0.5, 0.0], [0.5, 0.9], [1.1, 0.9], [1.1, 0.0]], 'closed': True},
]
A = assemble('dining-table', T('Dining table with extension leaf', 'Esstisch mit Einlegeplatte'),
             T('A 1.60 m solid oak dining table whose halves slide apart on slide rails so that a 0.60 m leaf lifts in (2.20 m); closed, extended, set and test-load poses.', 'Ein 1,60 m langer Esstisch aus massiver Eiche, dessen Hälften auf Auszugsschienen auseinandergleiten, sodass eine 0,60 m lange Einlegeplatte einrastet (2,20 m); Posen geschlossen, ausgezogen, gedeckt und Prüflast.'),
             ['table', 'dining', 'timber', 'oak', 'extension', 'interior'],
             T('Long-side elevation 1.60 x 0.75 m closed, 2.20 m extended; depth 0.90 m. Masses from nominal volumes x 700 kg/m3. Leaf and halves are prismatic bones; leaf stored under the top (hidden behind the aprons) and rising 30 mm.', 'Längsansicht 1,60 x 0,75 m geschlossen, 2,20 m ausgezogen; Tiefe 0,90 m. Massen aus Nennvolumen x 700 kg/m3. Einlegeplatte und Hälften sind Schiebe-Knochen; die Einlegeplatte liegt verstaut unter der Platte (hinter den Zargen) und hebt sich um 30 mm.'),
             {'heightM': 0.75, 'widthM': 1.6, 'depthM': 0.9, 'massKg': round(mass, 3)}, MATS, RIG, anchors, reps, [SYM], P, clips, thermal, struct,
             {'defaultPose': 'closed', 'defaultClip': 'extend-table', 'defaultScale': 50, 'viewpoints': ['elevation']},
             ['iso-10456-2007', 'en-1991-1-1', 'en-1730', 'en-12521', 'en-1993-1-1', 'iso-7730-2005', 'wood-handbook-2021', 'incropera-2018', 'semio-nwf'])
if __name__ == '__main__':
    main(A, 'dining-table', ['closed', 'extended', 'pulled-apart-key'])
