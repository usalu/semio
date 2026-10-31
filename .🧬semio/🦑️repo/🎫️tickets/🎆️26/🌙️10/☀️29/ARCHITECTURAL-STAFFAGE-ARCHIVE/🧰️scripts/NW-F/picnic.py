import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nwf import *

HX, HY = 0.46, 0.40
XT, XB = 0.22, 0.69
bones = [bone('ground', None, 'Ground reference', 'Bodenbezug', 0, 0, 0, 1.52, FIXED),
         bone('frame', 'ground', 'A-frames, beams and brackets', 'A-Gestelle, Riegel und Konsolen', 0, 0, 0, 1.4, FIXED, (15.0, 0.0, 0.4, 'point', 'authored-estimate', 'wood-handbook-2021')),
         bone('top', 'frame', 'Table top planks', 'Tischplatte', -0.35, 0.71, 0, 0.7, FIXED, (25.0, 0.35, 0.02, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
         bone('bench-r', 'frame', 'Right fold-down bench', 'Rechte Klappbank', HX, HY, 0, 0.30, rev(-95, 0), (10.8, 0.15, 0.02, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
         bone('bench-l', 'frame', 'Left fold-down bench', 'Linke Klappbank', -HX, HY, 180, 0.30, rev(0, 95), (10.8, 0.15, -0.02, 'uniform', 'authored-estimate', 'wood-handbook-2021'))]
rig = Rig(bones)
RIG = {'kind': 'mechanical', 'root': 'ground', 'bones': bones}
mass = sum(b['mass']['kg'] for b in bones if 'mass' in b)
pine = mat('pine-timber', 'Pressure-treated pine, weathered', 'Kesseldruckimprägnierte Kiefer, verwittert', 0.9, 0.7, 500, 0.13, 1600, 1.1e10, 2.4e7, 'literature', 'wood-handbook-2021')
pine['source'] = 'authored-estimate'; pine['reference'] = 'iso-10456-2007'
steel = mat('steel-galvanised', 'Hot-dip galvanised steel', 'Feuerverzinkter Stahl', 0.3, 0.6, 7800, 50, 450, 2.1e11, 3.55e8, 'literature', 'en-1993-1-1')
MATS = [pine, steel]
pl = lambda i, en, de, t: lay(i, en, de, 'timber', 'pine-timber', t, pine, 'authored-estimate')
regions = [
    reg('table-top', 'Table top planks', 'Tischplanken', 'surface', 'pine-timber', 1.26, 0.7, 'horizontal-plate-up', 0.95, [pl('top-plank', 'Pine plank 40 mm', 'Kieferbohle 40 mm', 0.04)], source='authored-estimate'),
    reg('bench-seats', 'Bench seat planks', 'Sitzbohlen der Bänke', 'surface', 'pine-timber', 1.08, 0.3, 'horizontal-plate-up', 0.8, [pl('seat-plank', 'Pine plank 40 mm', 'Kieferbohle 40 mm', 0.04)]),
    reg('a-frame', 'A-frames and beams', 'A-Gestelle und Riegel', 'surface', 'pine-timber', 2.4, 0.09, 'vertical-plate', 0.5, [pl('frame-member', 'Pine member 45 x 90 mm', 'Kiefernholz 45 x 90 mm', 0.045)]),
    reg('hinges', 'Steel hinge brackets', 'Stahlscharnierkonsolen', 'surface', 'steel-galvanised', 0.06, 0.06, 'vertical-plate', 0.5, [lay('hinge-plate', 'Steel plate 5 mm', 'Stahlblech 5 mm', 'metal', 'steel-galvanised', 0.005, steel, 'literature')]),
]
RIDS = [r['id'] for r in regions]
ctx = Ctx(RIDS, {'table-top': (1.0, 0.9), 'bench-seats': (1.0, 0.55), 'a-frame': (1.0, 0.3), 'hinges': (1.0, 0.3)})
anchors = [anc(rig, 'foot-left', 'frame', -XB, 0, 'contact', 'Left foot of the A-frame (front and rear coincide)', 'Linker Fuß des A-Gestells (vorn und hinten fallen zusammen)', support='contact', normalDeg=90, footprintLengthM=0.09, footprintWidthM=1.4, frictionCoefficient=0.5),
           anc(rig, 'foot-right', 'frame', XB, 0, 'contact', 'Right foot of the A-frame', 'Rechter Fuß des A-Gestells', support='contact', normalDeg=90, footprintLengthM=0.09, footprintWidthM=1.4, frictionCoefficient=0.5),
           anc(rig, 'seat-r', 'bench-r', 0.20, 0.04, 'load', 'Sitting point on the right bench', 'Sitzpunkt auf der rechten Bank'),
           anc(rig, 'seat-l', 'bench-l', 0.20, -0.04, 'load', 'Sitting point on the left bench', 'Sitzpunkt auf der linken Bank'),
           anc(rig, 'table-edge', 'top', 0.65, 0.04, 'load', 'Test point at the table edge', 'Prüfpunkt am Tischrand')]
CONT = ['foot-left', 'foot-right']
STOW = {'bench-r': {'angleDeg': -90}, 'bench-l': {'angleDeg': 90}}
two = lambda s, a: load(f'sitters-{s}', f'Two sitters on the {"right" if s == "r" else "left"} bench (2 x 75 kg)', f'Zwei Personen auf der {"rechten" if s == "r" else "linken"} Bank (2 x 75 kg)', 'occupant', -1471.0, anchor=a, ref='en-1991-1-1')
P = [
    pose(ctx, 'deployed', 'Benches down', 'Bänke ausgeklappt', 'Both benches folded down and ready; full sun on the top, half shade on the benches.', 'Beide Bänke heruntergeklappt und nutzbar; volle Sonne auf der Platte, Halbschatten auf den Bänken.', contacts=CONT),
    pose(ctx, 'stowed', 'Benches stowed', 'Bänke eingeklappt', 'Both benches hang folded against the A-frames; the seat planks stand vertical.', 'Beide Bänke hängen eingeklappt an den A-Gestellen; die Sitzbohlen stehen senkrecht.', bones=STOW, contacts=CONT,
         over={'bench-seats': (1.0, 0.25), 'table-top': (1.0, 0.9)}),
    pose(ctx, 'occupied', 'Four persons at the table', 'Vier Personen am Tisch', 'Two adults on each bench; forearms on the table. The persons shade the benches.', 'Zwei Erwachsene auf jeder Bank; Unterarme auf dem Tisch. Die Personen beschatten die Bänke.', contacts=CONT,
         loads=[two('r', 'seat-r'), two('l', 'seat-l'), load('table-setting', 'Table setting (0.3 kN/m2 over 1.8 m)', 'Gedeck auf dem Tisch (0,3 kN/m2 über 1,8 m)', 'live', 0, dist={'bone': 'top', 'fromM': 0, 'toM': 0.7, 'wxNPerM': 0, 'wyNPerM': -540}, ref='en-1991-1-1')],
         over={'bench-seats': (1.0, 0.25), 'table-top': (1.0, 0.8)}, contact={'bench-seats': (0.26,), 'table-top': (0.04,)}),
    pose(ctx, 'bench-proof-load', 'Bench proof load', 'Bank-Prüflast', 'Static test load 1600 N at the centre of the right bench seat.', 'Statische Prüflast 1600 N in der Mitte der rechten Sitzfläche.', contacts=CONT,
         loads=[load('proof-seat', 'Static seat test load (1600 N)', 'Statische Sitzprüflast (1600 N)', 'imposed', -1600, anchor='seat-r', ref='en-581-2')]),
    pose(ctx, 'half-folded-key', 'Benches half folded (key)', 'Bänke halb eingeklappt (Schlüssel)', 'Benches at 45 degrees.', 'Bänke bei 45 Grad.', key=True, bones={'bench-r': {'angleDeg': -45}, 'bench-l': {'angleDeg': 45}}, contacts=CONT, over={'bench-seats': (1.0, 0.4)}),
]
clips = [
    clip('fold-benches', 'Fold the benches away', 'Bänke wegklappen', 'Both benches swing up on their hinges and hang down against the A-frames in 3 s.', 'Beide Bänke schwingen an ihren Scharnieren hoch und hängen in 3 s an den A-Gestellen.', 3.0,
         [{'t': 0, 'pose': 'deployed', 'ease': 'smooth'}, {'t': 1.5, 'pose': 'half-folded-key', 'ease': 'smooth'}, {'t': 3.0, 'pose': 'stowed', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 3.0} for a in CONT], [('swing', 0, 3.0, 'Swing', 'Schwenken')], [(3.0, 'motion', 'Benches hang stowed', 'Bänke hängen verstaut', None)], [('region:bench-seats:sunlitFraction', 'smooth', [(0, 0.55), (3.0, 0.25)])]),
    clip('deploy-benches', 'Fold the benches down', 'Bänke herunterklappen', 'Both benches are lowered into the seat position.', 'Beide Bänke werden in die Sitzstellung heruntergeklappt.', 3.0,
         [{'t': 0, 'pose': 'stowed', 'ease': 'smooth'}, {'t': 1.5, 'pose': 'half-folded-key', 'ease': 'smooth'}, {'t': 3.0, 'pose': 'deployed', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 3.0} for a in CONT], [('swing', 0, 3.0, 'Swing', 'Schwenken')], [(3.0, 'motion', 'Benches rest on the beams', 'Bänke liegen auf den Riegeln', None)], []),
    clip('sit-down-together', 'Sit down together', 'Gemeinsam hinsetzen', 'Two adults sit on each bench one after another (four persons, 5.9 kN in total).', 'Zwei Erwachsene setzen sich nacheinander auf jede Bank (vier Personen, insgesamt 5,9 kN).', 5.0,
         [{'t': 0, 'pose': 'deployed', 'ease': 'smooth'}, {'t': 5.0, 'pose': 'occupied', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 5.0} for a in CONT], [('sit', 0, 5.0, 'Sitting', 'Hinsetzen')],
         [(1.0, 'load-on', 'Right bench occupied', 'Rechte Bank besetzt', 'sitters-r'), (2.5, 'load-on', 'Left bench occupied', 'Linke Bank besetzt', 'sitters-l'), (4.0, 'load-on', 'Table set', 'Tisch gedeckt', 'table-setting'), (1.0, 'thermal', 'Benches take up 30 degC contact heat', 'Bänke nehmen 30-degC-Kontaktwärme auf', None)],
         [('load:sitters-r', 'smooth', [(0, 0), (0.6, 0), (1.0, 1), (5.0, 1)]), ('load:sitters-l', 'smooth', [(0, 0), (2.1, 0), (2.5, 1), (5.0, 1)]), ('load:table-setting', 'smooth', [(0, 0), (3.5, 0), (4.5, 1), (5.0, 1)])]),
]
thermal = {'environment': {'airTempC': 20, 'meanRadiantTempC': 20, 'relativeHumidity': 0.5, 'windMS': 1.0, 'windDirectionDeg': 180, 'solarWM2': 600, 'diffuseFraction': 0.15, 'pressurePa': 101325},
           'operating': [], 'regions': regions, 'emitters': [], 'assumptions': [T(*a) for a in [
               ('Weathered pressure-treated pine (500 kg/m3, 0.13 W/mK, 1600 J/kgK after ISO 10456; absorptivity 0.7 for weathered grey timber is an authored-estimate) with heat capacity from the one-hour penetration depth (40 mm plank: 24 kJ/m2K).', 'Verwitterte kesseldruckimprägnierte Kiefer (500 kg/m3, 0,13 W/mK, 1600 J/kgK nach ISO 10456; Absorption 0,7 für vergrautes Holz ist ein Schätzwert) mit Wärmekapazität aus der Eindringtiefe einer Stunde (40-mm-Bohle: 24 kJ/m2K).'),
               ('Full sun on the top (0.9), half shade on the benches; occupied poses cover 26 % of the bench planks and 4 % of the top with clothed bodies at 30 degC (ISO 7730 clothing surface).', 'Volle Sonne auf der Platte (0,9), Halbschatten auf den Bänken; besetzte Posen bedecken 26 % der Sitzbohlen und 4 % der Platte mit bekleideten Körpern bei 30 degC (Bekleidungsoberfläche nach ISO 7730).'),
               ('Hinges and brackets are hot-dip galvanised steel; the stowed bench stands vertical and keeps its region binding with lower sun.', 'Scharniere und Konsolen sind feuerverzinkter Stahl; die eingeklappte Bank steht senkrecht und behält ihre Regionsbindung mit weniger Sonne.'),
               ('Reference case: 600 W/m2 sun, 20 degC air, 1 m/s breeze.', 'Referenzfall: 600 W/m2 Sonne, 20 degC Luft, 1 m/s Wind.')]]}
struct = structure(CONT, [
    ('End elevation: the A-frame (legs, top beam, bench beam) is a rigid body on two foot contacts; the top and the two benches are bodies on it, the benches on revolute hinges (0 to 95 degrees).', 'Stirnansicht: das A-Gestell (Beine, Plattenriegel, Bankriegel) ist ein starrer Körper auf zwei Fußkontakten; Platte und beide Bänke sind Körper darauf, die Bänke an Drehgelenken (0 bis 95 Grad).'),
    ('Occupant loads are totals over the table length (3D); distributed loads are per metre of the elevation width (table setting 0.3 kN/m2 x 1.8 m = 540 N/m, authored-estimate).', 'Personenlasten sind Summen über die Tischlänge (3D); verteilte Lasten gelten je Meter der Ansichtsbreite (Gedeck 0,3 kN/m2 x 1,8 m = 540 N/m, Schätzwert).'),
    ('The 1600 N seat proof load follows the order of the static seat tests of EN 581-2 (outdoor furniture, seating); the value is an authored-estimate to be verified against the standard text.', 'Die Sitz-Prüflast 1600 N folgt der Größenordnung der statischen Sitzprüfungen nach EN 581-2 (Gartenmöbel, Sitzmöbel); der Wert ist ein Schätzwert und am Normtext zu prüfen.')])
def leg(S_, sid, sgn, pen=None, hatch=None, w=0.03):
    xt, xb = sgn * XT, sgn * XB
    kw = {}
    if pen: kw['penMm'] = pen
    if hatch: kw['hatch'] = hatch
    return S_.poly(sid, 'frame', [(xt - w, 0.71), (xt + w, 0.71), (xb + w, 0.0), (xb - w, 0.0)], region='a-frame', material='pine-timber', tone=0.55, z=3, **kw)
def bench(S_, sid, sgn, t=0.04, pen=None, stroke=False, x0=0.40, hatch=None):
    b = 'bench-r' if sgn > 0 else 'bench-l'
    kw = {'penMm': pen} if pen else {}
    xa, xb = (x0, 0.76) if sgn > 0 else (-0.76, -x0)
    if stroke:
        xs = (xa, xb)
        return S_.line(sid, b, [(xs[0], HY + 0.02), (xs[1], HY + 0.02)], role='body', z=4, region='bench-seats', **kw)
    if hatch: kw['hatch'] = hatch
    return S_.rect(sid, b, xa, HY, xb, HY + t, region='bench-seats', material='pine-timber', tone=0.7, z=5, **kw)
s5 = Shapes(rig)
s5.rect('slab', 'top', -0.35, 0.65, 0.35, 0.75, regions=['table-top'], material='pine-timber', tone=0.7, z=6)
s5.line('leg-l', 'frame', [(-XT, 0.66), (-0.69, 0.0)], role='body', z=3, region='a-frame')
s5.line('leg-r', 'frame', [(XT, 0.66), (0.69, 0.0)], role='body', z=3, region='a-frame')
bench(s5, 'bench-r', 1, stroke=True, x0=0.35); bench(s5, 'bench-l', -1, stroke=True, x0=0.35)
s5.rect('hinge-l', 'frame', -0.50, 0.34, -0.42, 0.44, region='hinges', material='steel-galvanised', tone=0.2, z=7)
s5.rect('hinge-r', 'frame', 0.42, 0.34, 0.50, 0.44, region='hinges', material='steel-galvanised', tone=0.2, z=7)
s2 = Shapes(rig)
s2.rect('slab', 'top', -0.35, 0.665, 0.35, 0.75, regions=['table-top', 'a-frame'], material='pine-timber', tone=0.7, z=6)
leg(s2, 'leg-l', -1); leg(s2, 'leg-r', 1)
s2.rect('beam', 'frame', -0.48, 0.35, 0.48, 0.40, region='a-frame', material='pine-timber', tone=0.5, z=3)
bench(s2, 'bench-r', 1, t=0.05); bench(s2, 'bench-l', -1, t=0.05)
s2.rect('hinge-l', 'frame', -0.50, 0.34, -0.42, 0.44, region='hinges', material='steel-galvanised', tone=0.2, z=7)
s2.rect('hinge-r', 'frame', 0.42, 0.34, 0.50, 0.44, region='hinges', material='steel-galvanised', tone=0.2, z=7)
def det(S_, pen=None, hatch=False):
    kw = {'penMm': pen} if pen else {}
    S_.rect('top', 'top', -0.35, 0.71, 0.35, 0.75, region='table-top', material='pine-timber', tone=0.7, z=6, hatch='grain-h' if hatch else None, **kw)
    S_.rect('top-beam', 'frame', -0.30, 0.665, 0.30, 0.71, region='a-frame', material='pine-timber', tone=0.5, z=3, hatch='grain-h' if hatch else None)
    leg(S_, 'leg-l', -1, hatch='grain-v' if hatch else None); leg(S_, 'leg-r', 1, hatch='grain-v' if hatch else None)
    S_.rect('beam', 'frame', -0.48, 0.355, 0.48, 0.40, region='a-frame', material='pine-timber', tone=0.5, z=3, hatch='grain-h' if hatch else None)
    bench(S_, 'bench-r', 1, **kw); bench(S_, 'bench-l', -1, **kw)
    S_.rect('hinge-l', 'frame', -0.50, 0.34, -0.42, 0.44, region='hinges', material='steel-galvanised', tone=0.2, z=7)
    S_.rect('hinge-r', 'frame', 0.42, 0.34, 0.50, 0.44, region='hinges', material='steel-galvanised', tone=0.2, z=7)
s1 = Shapes(rig); det(s1)
s0 = Shapes(rig); det(s0, hatch=True)
for sh in list(s0.items):
    if sh['id'] in ('top', 'leg-l', 'leg-r', 'bench-r', 'bench-l'):
        s0.items.remove(sh); s0.ids.discard(sh['id'])
s0.poly('top', 'top', [(-0.35, 0.75), (-0.35, 0.718), (-0.342, 0.71), (0.342, 0.71), (0.35, 0.718), (0.35, 0.75)], region='table-top', material='pine-timber', tone=0.7, z=6, hatch='grain-h')
for sid, sg in (('leg-l', -1), ('leg-r', 1)):
    xt, xb = sg * XT, sg * XB
    s0.poly(sid, 'frame', [(xt - 0.03, 0.71), (xt + 0.03, 0.71), (xb + 0.03, 0.012), (xb + 0.018, 0.0), (xb - 0.03, 0.0)] if sg > 0 else [(xt - 0.03, 0.71), (xt + 0.03, 0.71), (xb + 0.03, 0.0), (xb - 0.018, 0.0), (xb - 0.03, 0.012)], region='a-frame', material='pine-timber', tone=0.55, z=3, hatch='grain-v')
s0.path('bench-r', 'bench-r', rbox(0.40, HY, 0.76, HY + 0.04, rt=0.01), region='bench-seats', material='pine-timber', tone=0.7, z=5)
s0.path('bench-l', 'bench-l', rbox(-0.76, HY, -0.40, HY + 0.04, rt=0.01), region='bench-seats', material='pine-timber', tone=0.7, z=5)
for nm, cx in (('l', -0.46), ('r', 0.46)):
    s0.ell(f'screw-{nm}', 'frame', cx, 0.40, 0.009, region='hinges', material='steel-galvanised', tone=0.1, z=8, penMm=0.13)
HATCH = [{'id': 'grain-h', 'title': T('Grain horizontal', 'Maserung waagerecht'), 'pattern': 'lines', 'angleDeg': 0, 'spacingMm': 0.6, 'penMm': 0.13}, {'id': 'grain-v', 'title': T('Grain along the leg', 'Maserung entlang des Beins'), 'pattern': 'lines', 'angleDeg': 76, 'spacingMm': 0.6, 'penMm': 0.13}]
def dd(*a): return D(*a)
reps = {
 '500': rep(500, s5, dd(('Picnic table sign 3.0 x 1.5 mm in end view: top slab, two splayed leg strokes, two bench strokes, hinges.', 'Picknicktisch-Zeichen 3,0 x 1,5 mm in der Stirnansicht: Plattenblock, zwei gespreizte Beinstriche, zwei Bankstriche, Scharniere.'),
   ('The A-shaped legs, the top level 0.75 m, the benches at 0.42 m reaching beyond the feet.', 'Die A-förmigen Beine, die Plattenhöhe 0,75 m, die Bänke auf 0,42 m über die Füße hinaus.'),
   ('Top plank and top beam are one 0.10 m slab.', 'Tischplanke und Plattenriegel sind ein 0,10 m starker Block.'),
   ('Rear legs, bench beam (closer than the 0.165 m minimum gap to the bench strokes), braces, plank joints.', 'Hintere Beine, Bankriegel (näher als der Mindestabstand von 0,165 m an den Bankstrichen), Streben, Plankenstöße.'),
   ('Benches (0.04 m) and legs (0.06 m) are 0.18 mm strokes = 0.09 m real; the bench strokes follow the fold hinge.', 'Bänke (0,04 m) und Beine (0,06 m) sind 0,18-mm-Striche = 0,09 m real; die Bankstriche folgen dem Klappscharnier.'),
   ('None.', 'Keine.'), ('Slab 0.7; hinges 0.2.', 'Block 0,7; Scharniere 0,2.'), ('The slab lists table-top; legs, beam carry a-frame; bench strokes carry bench-seats; hinges carry hinges.', 'Der Block führt table-top; Beine, Riegel tragen a-frame; Bankstriche tragen bench-seats; Scharniere tragen hinges.'))),
 '200': rep(200, s2, dd(('Picnic table 7.6 x 3.8 mm: top slab with beam, two leg polygons, bench beam, two benches, two hinge plates.', 'Picknicktisch 7,6 x 3,8 mm: Plattenblock mit Riegel, zwei Beinpolygone, Bankriegel, zwei Bänke, zwei Scharnierplatten.'),
   ('A-frame legs, benches as planks (folding), hinge plates.', 'A-Beine, Bänke als Bohlen (klappbar), Scharnierplatten.'),
   ('Top plank and top beam are one slab.', 'Tischplanke und Plattenriegel sind ein Block.'), ('Rear legs, braces, bolts.', 'Hintere Beine, Streben, Schrauben.'),
   ('Benches 0.04 m are drawn 0.05 m (0.25 mm).', 'Bänke von 0,04 m sind 0,05 m (0,25 mm) stark gezeichnet.'), ('None.', 'Keine.'), ('Slab and benches 0.7, beam 0.5.', 'Block und Bänke 0,7, Riegel 0,5.'), ('The slab lists table-top and a-frame merged.', 'Der Block führt table-top und a-frame zusammen.'))),
 '100': rep(100, s1, dd(('Picnic table 15.2 x 7.5 mm with all members at real size: top planks, top beam, legs, bench beam, benches, hinges.', 'Picknicktisch 15,2 x 7,5 mm mit allen Bauteilen in Realgröße: Tischplanken, Plattenriegel, Beine, Bankriegel, Bänke, Scharniere.'),
   ('Real thicknesses 40 mm planks, 45 mm beams, 60 mm legs.', 'Echte Stärken: Bohlen 40 mm, Riegel 45 mm, Beine 60 mm.'), ('Rear legs coincide.', 'Hintere Beine fallen zusammen.'), ('Bolts, plank joints.', 'Schrauben, Plankenstöße.'),
   ('All members are filled shapes of at least 0.4 mm.', 'Alle Bauteile sind gefüllte Formen von mindestens 0,4 mm.'), ('None.', 'Keine.'), ('Planks 0.7, members 0.5, hinges 0.2.', 'Bohlen 0,7, Hölzer 0,5, Scharniere 0,2.'), ('One shape per member.', 'Eine Form je Bauteil.'))),
 '50': rep(50, s0, dd(('Picnic table 30 x 15 mm with grain on legs and beams, planks at real thickness and hinge plates.', 'Picknicktisch 30 x 15 mm mit Maserung auf Beinen und Riegeln, Bohlen in Realstärke und Scharnierplatten.'),
   ('As at 1:100 plus grain hatching along the legs and beams.', 'Wie bei 1:100 plus Maserungsschraffur entlang der Beine und Riegel.'), ('Rear legs coincide.', 'Hintere Beine fallen zusammen.'), ('Bolts (0.012 m), end-grain rings, chamfers.', 'Schrauben (0,012 m), Hirnholzringe, Fasen.'),
   ('Hinge pins are not drawn.', 'Scharnierbolzen sind nicht gezeichnet.'), ('Grain lines at 0.6 mm spacing, pen 0.13.', 'Maserungslinien im Abstand 0,6 mm, Stift 0,13.'), ('As 1:100.', 'Wie 1:100.'), ('One shape per member; hatched shapes keep their region.', 'Eine Form je Bauteil; schraffierte Formen behalten ihre Region.')), hatches=HATCH),
}
for k in reps:
    reps[k]['validRange'] = {'minDenominator': RANGES[k][0], 'maxDenominator': RANGES[k][1]}
SYM = {'id': 'picnic-plan', 'title': T('Picnic table plan symbol', 'Picknicktisch-Grundrisssymbol'), 'description': T('Floor-plan sign of the table with both benches: top 1.80 x 0.70 m between two benches of 0.30 m.', 'Grundrisszeichen des Tisches mit beiden Bänken: Platte 1,80 x 0,70 m zwischen zwei Bänken von 0,30 m.'),
       'semantics': 'true-extent', 'units': 'model-m', 'denominators': [200, 100, 50], 'pens': {'body': 0.25, 'detail': 0.13, 'texture': None, 'glazing': None, 'ground': 0.35}, 'hatches': [],
       'geometry': {'elevation': [], 'plan': [
           {'id': 'plan-top', 'bone': 'ground', 'role': 'body', 'z': 2, 'filled': True, 'tone': 0.9, 'kind': 'poly', 'points': [[-0.35, -0.9], [0.35, -0.9], [0.35, 0.9], [-0.35, 0.9]], 'closed': True},
           {'id': 'plan-bench-r', 'bone': 'ground', 'role': 'body', 'z': 1, 'filled': True, 'tone': 0.8, 'kind': 'poly', 'points': [[0.46, -0.9], [0.76, -0.9], [0.76, 0.9], [0.46, 0.9]], 'closed': True},
           {'id': 'plan-bench-l', 'bone': 'ground', 'role': 'body', 'z': 1, 'filled': True, 'tone': 0.8, 'kind': 'poly', 'points': [[-0.76, -0.9], [-0.46, -0.9], [-0.46, 0.9], [-0.76, 0.9]], 'closed': True}]},
       'dimensionalNote': T('Model metres at real size: x across the table -0.76 to 0.76 m, y along the table -0.90 to 0.90 m; benches in the deployed position.', 'Modellmeter in Originalgröße: x quer -0,76 bis 0,76 m, y längs -0,90 bis 0,90 m; Bänke in Sitzstellung.')}
A = assemble('picnic-table', T('Picnic table with fold-down benches', 'Picknicktisch mit Klappbänken'),
             T('A 1.80 m softwood picnic table with A-frame ends and two benches hinged to the frames that fold down for four persons or up against the frames; end elevation and plan symbol.', 'Ein 1,80 m langer Picknicktisch aus Weichholz mit A-Gestellen an den Enden und zwei Bänken, die an den Gestellen angelenkt sind und für vier Personen heruntergeklappt oder an die Gestelle hochgeklappt werden; Stirnansicht und Grundrisssymbol.'),
             ['picnic-table', 'bench', 'a-frame', 'outdoor', 'timber', 'folding'],
             T('End elevation 1.52 x 0.75 m (benches to 0.76 m each side of the axis), length 1.80 m. Masses from nominal volumes x 500 kg/m3 plus hinges. The hinge brackets and the fold-down benches are the moving parts; the A-frame of a classic fixed picnic table is the same construction with fixed benches.', 'Stirnansicht 1,52 x 0,75 m (Bänke bis 0,76 m beidseits der Achse), Länge 1,80 m. Massen aus Nennvolumen x 500 kg/m3 plus Scharniere. Die Scharnierkonsolen und die Klappbänke sind die beweglichen Teile; das A-Gestell eines klassischen festen Picknicktisches ist dieselbe Konstruktion mit festen Bänken.'),
             {'heightM': 0.75, 'widthM': 1.52, 'depthM': 1.8, 'massKg': round(mass, 3)}, MATS, RIG, anchors, reps, [SYM], P, clips, thermal, struct,
             {'defaultPose': 'deployed', 'defaultClip': 'fold-benches', 'defaultScale': 50, 'viewpoints': ['elevation']},
             ['iso-10456-2007', 'en-1991-1-1', 'en-581-1', 'en-581-2', 'en-581-3', 'en-1993-1-1', 'iso-7730-2005', 'wood-handbook-2021', 'incropera-2018', 'semio-nwf'])
if __name__ == '__main__':
    main(A, 'picnic-table', ['deployed', 'stowed', 'occupied'])
