import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nwf import *

bones = [bone('ground', None, 'Floor reference', 'Bodenbezug', 0, 0, 0, 0.42, FIXED),
         bone('frame', 'ground', 'Steel frame: four legs and footrest ring', 'Stahlgestell: vier Beine und Fußring', 0, 0, 0, 0.7, FIXED, (4.0, 0.0, 0.35, 'point', 'authored-estimate', 'en-1993-1-1')),
         bone('seat', 'frame', 'Swivel seat plate', 'Drehbare Sitzplatte', 0, 0.698, 0, 0.36, FIXED, (1.4, 0.0, 0.01, 'point', 'authored-estimate', 'semio-nwf')),
         bone('pad', 'seat', 'Foam seat pad', 'Schaumpolster', 0, 0.022, 0, 0.36, prism(90, -0.04, 0), (0.5, 0.0, 0.015, 'point', 'authored-estimate', 'iso-3386-1')),
         bone('backrest', 'seat', 'Low backrest (swivel azimuth projected)', 'Niedrige Lehne (Drehwinkel projiziert)', 0, 0.022, 0, 0.10, prism(0, -0.15, 0.15), (0.9, 0.0, 0.10, 'point', 'authored-estimate', 'semio-nwf'))]
rig = Rig(bones)
RIG = {'kind': 'mechanical', 'root': 'ground', 'bones': bones}
mass = sum(b['mass']['kg'] for b in bones if 'mass' in b)
chrome = mat('chrome-steel', 'Chromium-plated steel tube 25 x 1.5 mm', 'Verchromtes Stahlrohr 25 x 1,5 mm', 0.1, 0.4, 7850, 50, 450, 2.1e11, 2.35e8, 'literature', 'en-1993-1-1')
leather = mat('pu-leatherette', 'PU leatherette cover', 'PU-Kunstlederbezug', 0.9, 0.6, 1000, 0.2, 1500, None, None, 'authored-estimate', 'iso-10456-2007')
foam = mat('hr-foam', 'HR foam 40 kg/m3', 'HR-Schaum 40 kg/m3', 0.9, 0.6, 40, 0.045, 1400, 1.0e4, 6.0e3, 'authored-estimate', 'iso-3386-1')
ply = mat('birch-plywood', 'Birch plywood 12 mm', 'Birkensperrholz 12 mm', 0.9, 0.55, 680, 0.17, 1600, 9.0e9, 4.0e7, 'literature', 'iso-10456-2007')
MATS = [chrome, leather, foam, ply]
lf = lay('leatherette', 'Leatherette 1.5 mm', 'Kunstleder 1,5 mm', 'polymer', 'pu-leatherette', 0.0015, leather, 'authored-estimate')
regions = [
    reg('seat-pad', 'Seat pad top and rim', 'Sitzpolster oben und Rand', 'surface', 'pu-leatherette', 0.147, 0.36, 'horizontal-plate-up', 0.8, [lf, lay('pad-foam', 'HR foam 40 mm', 'HR-Schaum 40 mm', 'insulation', 'hr-foam', 0.04, foam, 'authored-estimate')]),
    reg('seat-plate', 'Seat plate underside', 'Sitzplatten-Unterseite', 'surface', 'birch-plywood', 0.102, 0.36, 'horizontal-plate-down', 0.3, [lay('plate-ply', 'Plywood 12 mm', 'Sperrholz 12 mm', 'timber', 'birch-plywood', 0.012, ply)],
        hidden=T('The underside of the seat plate faces the floor and is hidden by the seat rim in every elevation.', 'Die Unterseite der Sitzplatte zeigt zum Boden und ist in jeder Ansicht vom Sitzrand verdeckt.')),
    reg('legs', 'Chromed legs', 'Verchromte Beine', 'surface', 'chrome-steel', 0.25, 0.025, 'cylinder', 0.4, [lay('leg-wall', 'Tube wall 1.5 mm', 'Rohrwand 1,5 mm', 'metal', 'chrome-steel', 0.0015, chrome)]),
    reg('footrest-ring', 'Footrest ring', 'Fußring', 'surface', 'chrome-steel', 0.084, 0.025, 'cylinder', 0.3, [lay('ring-wall', 'Tube wall 1.5 mm', 'Rohrwand 1,5 mm', 'metal', 'chrome-steel', 0.0015, chrome)]),
    reg('backrest', 'Low backrest', 'Niedrige Lehne', 'surface', 'pu-leatherette', 0.06, 0.1, 'vertical-plate', 0.5, [lf, lay('back-foam', 'Foam 20 mm', 'Schaum 20 mm', 'insulation', 'hr-foam', 0.02, foam, 'authored-estimate')]),
]
RIDS = [r['id'] for r in regions]
ctx = Ctx(RIDS, {'seat-pad': (1.0, 0.0), 'seat-plate': (1.0, 0.0), 'legs': (1.0, 0.0), 'footrest-ring': (1.0, 0.0), 'backrest': (1.0, 0.0)})
anchors = [anc(rig, 'foot-left', 'frame', -0.21, 0, 'contact', 'Left leg foot', 'Linker Beinfuß', support='contact', normalDeg=90, footprintLengthM=0.03, footprintWidthM=0.03, frictionCoefficient=0.5),
           anc(rig, 'foot-mid', 'frame', 0.0, 0, 'contact', 'Front and rear leg feet', 'Vorderer und hinterer Beinfuß', support='contact', normalDeg=90, footprintLengthM=0.03, footprintWidthM=0.30, frictionCoefficient=0.5),
           anc(rig, 'foot-right', 'frame', 0.21, 0, 'contact', 'Right leg foot', 'Rechter Beinfuß', support='contact', normalDeg=90, footprintLengthM=0.03, footprintWidthM=0.03, frictionCoefficient=0.5),
           anc(rig, 'seat-top', 'pad', 0.0, 0.75, 'load', 'Sitting point', 'Sitzpunkt'), anc(rig, 'ring-front', 'frame', 0.0, 0.27, 'load', 'Footrest point', 'Fußraststelle')]
CONT = ['foot-left', 'foot-mid', 'foot-right']
occ = lambda: [load('occupant-seat', 'Seated adult on the seat (85 % of 75 kg)', 'Sitzende Person auf dem Sitz (85 % von 75 kg)', 'occupant', -625.2, anchor='seat-top', ref='en-1991-1-1'),
               load('occupant-feet', 'Feet on the footrest (15 % of 75 kg)', 'Füße auf dem Fußring (15 % von 75 kg)', 'occupant', -110.3, anchor='ring-front', ref='en-1991-1-1')]
SINK = {'pad': {'dy': -0.018}}
P = [
    pose(ctx, 'unoccupied', 'Unoccupied', 'Unbesetzt', 'Empty stool, backrest at the rear; the chromed tubes shine beside a window.', 'Leerer Hocker, Lehne hinten; die verchromten Rohre glänzen neben einem Fenster.', contacts=CONT),
    pose(ctx, 'occupied', 'Occupied', 'Besetzt', 'An adult sits, feet on the ring; the pad sinks 18 mm.', 'Eine erwachsene Person sitzt, Füße auf dem Ring; das Polster sinkt um 18 mm.', bones=SINK, contacts=CONT, loads=occ(), over={'seat-pad': (1.0, 0.0), 'backrest': (1.0, 0.0)}, contact={'seat-pad': (0.85,), 'footrest-ring': (0.12,)}),
    pose(ctx, 'turned-90', 'Seat turned by 90 degrees', 'Sitz um 90 Grad gedreht', 'Unoccupied seat turned a quarter turn: the backrest appears at the right side (projection 0.15 m).', 'Unbesetzter Sitz um eine Vierteldrehung gedreht: die Lehne erscheint seitlich rechts (Projektion 0,15 m).', bones={'backrest': {'dx': 0.15}}, contacts=CONT),
    pose(ctx, 'seat-proof-load', 'Seat proof load', 'Sitz-Prüflast', 'Static seat test load 1600 N: foam compressed by 30 mm.', 'Statische Sitz-Prüflast 1600 N: Schaum um 30 mm zusammengedrückt.', bones={'pad': {'dy': -0.03}}, contacts=CONT,
         loads=[load('proof-seat', 'Static seat test load (1600 N)', 'Statische Sitzprüflast (1600 N)', 'imposed', -1600, anchor='seat-top', ref='en-12520')]),
    pose(ctx, 'turned-270-key', 'Seat turned by 270 degrees (key)', 'Sitz um 270 Grad gedreht (Schlüssel)', 'Backrest projected at the left side.', 'Lehne links projiziert.', key=True, bones={'backrest': {'dx': -0.15}}, contacts=CONT),
    pose(ctx, 'occupied-turned-key', 'Occupied, turned 90 degrees (key)', 'Besetzt, 90 Grad gedreht (Schlüssel)', 'Seated and turned a quarter turn.', 'Sitzend, um eine Vierteldrehung gedreht.', key=True, bones={'pad': {'dy': -0.018}, 'backrest': {'dx': 0.15}}, contacts=CONT, loads=occ(), contact={'seat-pad': (0.85,), 'footrest-ring': (0.12,)}),
]
clips = [
    clip('swivel-turn', 'Full swivel turn', 'Volle Drehung', 'The unoccupied seat turns once round in 6 s; the backrest runs left and right as x = 0.15 m sin(azimuth).', 'Der unbesetzte Sitz dreht sich in 6 s einmal herum; die Lehne läuft als x = 0,15 m sin(Azimut) nach links und rechts.', 6.0,
         [{'t': 0, 'pose': 'unoccupied', 'ease': 'smooth'}, {'t': 1.5, 'pose': 'turned-90', 'ease': 'smooth'}, {'t': 3.0, 'pose': 'unoccupied', 'ease': 'smooth'}, {'t': 4.5, 'pose': 'turned-270-key', 'ease': 'smooth'}, {'t': 6.0, 'pose': 'unoccupied', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 6.0} for a in CONT], [('turn', 0, 6.0, 'Turn', 'Drehen')], [(1.5, 'motion', 'Quarter turn', 'Vierteldrehung', None)], [], loop=True),
    clip('sit-and-turn', 'Sit down and turn', 'Hinsetzen und Drehen', 'An adult sits (pad sinks 18 mm), turns a quarter turn and turns back.', 'Eine erwachsene Person setzt sich (Polster sinkt 18 mm), dreht sich eine Vierteldrehung und zurück.', 5.0,
         [{'t': 0, 'pose': 'unoccupied', 'ease': 'smooth'}, {'t': 1.0, 'pose': 'occupied', 'ease': 'smooth'}, {'t': 2.5, 'pose': 'occupied-turned-key', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'occupied', 'ease': 'smooth'}, {'t': 5.0, 'pose': 'occupied', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 5.0} for a in CONT], [('sit', 0, 1.0, 'Sit', 'Hinsetzen'), ('turn', 1.0, 4.0, 'Turn', 'Drehen')],
         [(1.0, 'load-on', 'Occupant sits', 'Person setzt sich', 'occupant-seat'), (1.0, 'thermal', 'Seat pad starts taking 30 degC contact heat', 'Sitzpolster nimmt 30-degC-Kontaktwärme auf', None)],
         [('load:occupant-seat', 'smooth', [(0, 0), (0.6, 0), (1.0, 1), (5.0, 1)]), ('load:occupant-feet', 'smooth', [(0, 0), (0.8, 0), (1.2, 1), (5.0, 1)])]),
    clip('stand-up', 'Stand up', 'Aufstehen', 'The occupant leaves the stool; the pad recovers.', 'Die Person verlässt den Hocker; das Polster erholt sich.', 3.0,
         [{'t': 0, 'pose': 'occupied', 'ease': 'smooth'}, {'t': 1.0, 'pose': 'unoccupied', 'ease': 'smooth'}, {'t': 3.0, 'pose': 'unoccupied', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 3.0} for a in CONT], [('rise', 0, 1.0, 'Rise', 'Aufstehen')], [(0.6, 'load-off', 'Occupant leaves', 'Person verlässt den Hocker', 'occupant-seat')],
         [('load:occupant-seat', 'smooth', [(0, 1), (0.6, 0), (3.0, 0)]), ('load:occupant-feet', 'smooth', [(0, 1), (0.4, 0), (3.0, 0)])]),
]
thermal = {'environment': {'airTempC': 21, 'meanRadiantTempC': 21, 'relativeHumidity': 0.5, 'windMS': 0.1, 'windDirectionDeg': 0, 'solarWM2': 350, 'diffuseFraction': 0.15, 'pressurePa': 101325},
           'operating': [], 'regions': regions, 'emitters': [], 'assumptions': [T(*a) for a in [
               ('Chromium-plated steel tube: emissivity 0.1 and solar absorptivity 0.4 (Incropera Table A.11 order of magnitude), 1.5 mm wall (5.3 kJ/m2K): the tubes stay near air temperature, the footrest ring feels cold to the touch.', 'Verchromtes Stahlrohr: Emissionsgrad 0,1 und solare Absorption 0,4 (Größenordnung nach Incropera Tabelle A.11), Wand 1,5 mm (5,3 kJ/m2K): die Rohre bleiben nahe der Lufttemperatur, der Fußring fühlt sich kalt an.'),
               ('Pad: 1.5 mm PU leatherette (authored-estimate) over 40 mm HR foam (0.045 W/mK); the seat plate underside faces the floor and is hidden in elevation.', 'Polster: 1,5 mm PU-Kunstleder (Schätzwert) auf 40 mm HR-Schaum (0,045 W/mK); die Unterseite der Sitzplatte zeigt zum Boden und ist in der Ansicht verdeckt.'),
               ('An occupied pose covers 85 % of the pad with a clothed body at 30 degC (ISO 7730 clothing surface) and 12 % of the ring with shoes (28 degC, authored-estimate).', 'Eine besetzte Pose bedeckt 85 % des Polsters mit einem bekleideten Körper bei 30 degC (Bekleidungsoberfläche nach ISO 7730) und 12 % des Rings mit Schuhen (28 degC, Schätzwert).'),
               ('Reference case: 350 W/m2 window sun patch (not on the unoccupied stool by default), 21 degC, still air.', 'Referenzfall: 350 W/m2 Fensterlichtfleck (standardmäßig nicht auf dem unbesetzten Hocker), 21 degC, ruhige Luft.')]]}
for p in P:
    if p['id'] in ('occupied', 'occupied-turned-key'):
        p['thermal']['regions']['footrest-ring']['contact']['tempC'] = 28
struct = structure(CONT, [
    ('The stool is a rigid steel frame on three foot contacts in elevation (the front and rear legs coincide); the seat, pad and backrest are bodies on prismatic joints.', 'Der Hocker ist ein starres Stahlgestell auf drei Fußkontakten in der Ansicht (vordere und hintere Beine fallen zusammen); Sitz, Polster und Lehne sind Körper auf Schiebegelenken.'),
    ('The swivel is a rotation about the vertical axis that does not change a symmetric seat in elevation; it is drawn through the backrest, which sits at radius 0.15 m: its projected offset is x = 0.15 m sin(azimuth), the foreshortening of the 0.10 m wide pad is neglected.', 'Die Drehung ist eine Rotation um die Hochachse, die einen symmetrischen Sitz in der Ansicht nicht verändert; sie wird über die Lehne gezeichnet, die auf dem Radius 0,15 m sitzt: ihre projizierte Verschiebung ist x = 0,15 m sin(Azimut), die Verkürzung der 0,10 m breiten Lehne ist vernachlässigt.'),
    ('85 % of the 75 kg occupant on the seat, 15 % on the footrest (authored-estimate); the 1600 N seat proof load follows the order of EN 1728 / EN 12520 and is to be verified against the standard text.', '85 % der 75 kg Person auf dem Sitz, 15 % auf dem Fußring (Schätzwert); die Sitz-Prüflast 1600 N folgt der Größenordnung von EN 1728 / EN 12520 und ist am Normtext zu prüfen.')])
# geometry
FX = 0.1975
def legs(S_, pen):
    S_.cap('leg-l', 'frame', -0.14, 0.712, -FX, 0.0125, 0.0125, region='legs', material='chrome-steel', tone=0.7, z=2, penMm=pen)
    S_.cap('leg-r', 'frame', 0.14, 0.712, FX, 0.0125, 0.0125, region='legs', material='chrome-steel', tone=0.7, z=2, penMm=pen)
    S_.cap('leg-m', 'frame', 0.0, 0.712, 0.0, 0.0125, 0.0125, region='legs', material='chrome-steel', tone=0.7, z=2, penMm=pen)
    S_.cap('ring', 'frame', -0.19, 0.27, 0.19, 0.27, 0.0125, region='footrest-ring', material='chrome-steel', tone=0.7, z=3, penMm=pen)
s5 = Shapes(rig)
s5.path('seat', 'seat', [('M', -0.18, 0.65), ('L', 0.18, 0.65), ('L', 0.18, 0.75), ('L', 0.055, 0.75), ('L', 0.055, 0.87), ('L', -0.055, 0.87), ('L', -0.055, 0.75), ('L', -0.18, 0.75), ('Z',)], regions=['seat-pad', 'seat-plate', 'backrest'], material='pu-leatherette', tone=0.35, z=6)
s5.line('leg-l', 'frame', [(-0.14, 0.66), (-0.21, 0.0)], role='body', z=2, region='legs')
s5.line('leg-r', 'frame', [(0.14, 0.66), (0.21, 0.0)], role='body', z=2, region='legs')
s5.line('ring', 'frame', [(-0.195, 0.27), (0.195, 0.27)], role='body', penMm=0.13, z=3, region='footrest-ring')
s2 = Shapes(rig)
s2.rect('seat', 'seat', -0.18, 0.698, 0.18, 0.75, regions=['seat-pad', 'seat-plate'], material='pu-leatherette', tone=0.35, z=6)
s2.poly('back', 'backrest', [(-0.025, 0.69), (0.025, 0.69), (0.025, 0.80), (0.05, 0.80), (0.05, 0.87), (-0.05, 0.87), (-0.05, 0.80), (-0.025, 0.80)], region='backrest', material='pu-leatherette', tone=0.35, z=5)
s2.line('leg-l', 'frame', [(-0.14, 0.70), (-0.21, 0.0)], role='body', penMm=0.18, z=2, region='legs')
s2.line('leg-r', 'frame', [(0.14, 0.70), (0.21, 0.0)], role='body', penMm=0.18, z=2, region='legs')
s2.line('ring', 'frame', [(-0.195, 0.27), (0.195, 0.27)], role='body', penMm=0.18, z=3, region='footrest-ring')
def top(S_, pen):
    S_.rect('plate', 'seat', -0.18, 0.68, 0.18, 0.72, region='seat-plate', material='birch-plywood', tone=0.6, z=7, penMm=pen)
    S_.path('pad', 'pad', rbox(-0.18, 0.71, 0.18, 0.75, rt=0.012), region='seat-pad', material='pu-leatherette', tone=0.3, z=6, penMm=pen)
    S_.rect('back-post', 'backrest', -0.015, 0.72, 0.015, 0.81, region='backrest', material='chrome-steel', tone=0.7, z=4)
    S_.path('back-pad', 'backrest', rbox(-0.05, 0.81, 0.05, 0.87, rt=0.02), region='backrest', material='pu-leatherette', tone=0.3, z=5)
s1 = Shapes(rig); top(s1, 0.25); legs(s1, 0.25)
s0 = Shapes(rig); top(s0, 0.25); legs(s0, 0.25)
for sh in list(s0.items):
    if sh['id'] in ('plate', 'pad', 'back-pad'):
        s0.items.remove(sh); s0.ids.discard(sh['id'])
s0.poly('plate', 'seat', [(-0.174, 0.68), (0.174, 0.68), (0.18, 0.686), (0.18, 0.72), (-0.18, 0.72), (-0.18, 0.686)], region='seat-plate', material='birch-plywood', tone=0.6, z=7, penMm=0.25)
s0.path('pad', 'pad', rbox(-0.18, 0.71, 0.18, 0.75, rt=0.018), region='seat-pad', material='pu-leatherette', tone=0.3, z=6, penMm=0.25)
s0.path('back-pad', 'backrest', rbox(-0.05, 0.81, 0.05, 0.87, rt=0.028), region='backrest', material='pu-leatherette', tone=0.3, z=5)
for nm, cx in (('l', -FX), ('r', FX), ('m', 0.0)):
    s0.rect(f'ferrule-{nm}', 'frame', cx - 0.015, 0.0, cx + 0.015, 0.022, region='legs', material='chrome-steel', tone=0.2, z=2.5, penMm=0.25)
def dd(*a): return D(*a)
reps = {
 '500': rep(500, s5, dd(('Stool sign 0.84 x 1.74 mm: seat block with a backrest block above, two splayed leg strokes and the footrest line.', 'Hockerzeichen 0,84 x 1,74 mm: Sitzblock mit Lehnenblock darüber, zwei gespreizte Beinstriche und die Fußringlinie.'),
   ('Seat height 0.75 m, backrest to 0.87 m, splayed legs, footrest ring.', 'Sitzhöhe 0,75 m, Lehne bis 0,87 m, gespreizte Beine, Fußring.'),
   ('Seat plate and pad are one 0.10 m block; the backrest post and pad are one block.', 'Sitzplatte und Polster sind ein 0,10 m hoher Block; Lehnenpfosten und -polster sind ein Block.'),
   ('The front and rear leg (it would come closer than the 0.17 m minimum gap to the side legs), seams, ferrules; the backrest is part of the seat block (its 0.3 mm swivel travel is not resolved at this scale).', 'Vorderes und hinteres Bein (es käme den Seitenbeinen näher als der Mindestabstand von 0,17 m), Nähte, Fußkappen; die Lehne ist Teil des Sitzblocks (ihr Drehweg von 0,3 mm ist in diesem Maßstab nicht auflösbar).'),
   ('Tubes (0.025 m, 0.05 mm) are 0.18 mm strokes = 0.09 m real; the ring is a 0.13 mm line.', 'Rohre (0,025 m, 0,05 mm) sind 0,18-mm-Striche = 0,09 m real; der Ring ist eine 0,13-mm-Linie.'),
   ('None.', 'Keine.'), ('Seat and backrest tone 0.35; chrome strokes black.', 'Sitz und Lehne Ton 0,35; Chromstriche schwarz.'),
   ('The seat block lists seat-pad and seat-plate merged; strokes carry legs and footrest-ring.', 'Der Sitzblock führt seat-pad und seat-plate zusammen; Striche tragen legs und footrest-ring.'))),
 '200': rep(200, s2, dd(('Stool 2.1 x 4.4 mm: seat slab, T-shaped backrest, two leg strokes and the ring.', 'Hocker 2,1 x 4,4 mm: Sitzplatte, T-förmige Lehne, drei Beinstriche und der Ring.'),
   ('Seat slab (plate and pad merged), moving backrest with post, the two side leg lines, ring.', 'Sitzplatte (Platte und Polster zusammengefasst), Lehne mit Pfosten, die drei Beinlinien, Ring.'),
   ('Plate and pad are one slab; backrest post is widened to 0.05 m and joined to the seat.', 'Platte und Polster sind eine Platte; der Lehnenpfosten ist auf 0,05 m verbreitert und mit dem Sitz verbunden.'),
   ('Front and rear leg (closer than the 0.07 m minimum gap to the backrest stem), ferrules, pad compression (0.1 mm).', 'Fußkappen, Polsterstauchung (0,1 mm).'),
   ('Tubes are 0.18 mm strokes (0.036 m real instead of 0.025 m).', 'Rohre sind 0,18-mm-Striche (0,036 m real statt 0,025 m).'),
   ('None.', 'Keine.'), ('Slab and back 0.35.', 'Platte und Lehne 0,35.'), ('Slab merges seat-pad and seat-plate.', 'Die Platte führt seat-pad und seat-plate zusammen.'))),
 '100': rep(100, s1, dd(('Stool 4.2 x 8.7 mm: plate, pad, backrest post and pad, tubes as capsules and the ring.', 'Hocker 4,2 x 8,7 mm: Platte, Polster, Lehnenpfosten und -polster, Rohre als Kapseln und der Ring.'),
   ('Plate and pad separately (the pad sinks under load), post and back pad, four tubes (three visible), ring.', 'Platte und Polster getrennt (das Polster sinkt unter Last), Pfosten und Lehnenpolster, vier Rohre (drei sichtbar), Ring.'),
   ('Front and rear legs coincide.', 'Vordere und hintere Beine fallen zusammen.'), ('Ferrules, seams.', 'Fußkappen, Nähte.'),
   ('Tubes 0.025 m = 0.25 mm filled capsules.', 'Rohre 0,025 m = 0,25 mm gefüllte Kapseln.'), ('None.', 'Keine.'), ('Pad 0.3, chrome 0.7.', 'Polster 0,3, Chrom 0,7.'), ('One shape per region part.', 'Eine Form je Regionsteil.'))),
 '50': rep(50, s0, dd(('Stool 8.4 x 17.4 mm: all parts as at 1:100 plus rubber ferrules.', 'Hocker 8,4 x 17,4 mm: alle Teile wie bei 1:100 plus Gummifußkappen.'),
   ('Plate 22 mm, pad 30 mm visible foam, chromed tubes, ring, backrest post and pad, ferrules.', 'Platte 22 mm, 30 mm sichtbarer Schaum, verchromte Rohre, Ring, Lehnenpfosten und -polster, Fußkappen.'),
   ('Front and rear legs coincide.', 'Vordere und hintere Beine fallen zusammen.'), ('Welds, cover seam, screws.', 'Schweißnähte, Bezugsnaht, Schrauben.'),
   ('Tubes drawn at real diameter 0.025 m (0.5 mm).', 'Rohre im echten Durchmesser 0,025 m (0,5 mm).'), ('None.', 'Keine.'), ('Pad 0.3, ferrules 0.2, chrome 0.7.', 'Polster 0,3, Fußkappen 0,2, Chrom 0,7.'), ('One shape per region part; ferrules carry legs.', 'Eine Form je Regionsteil; Fußkappen tragen legs.'))),
}
for k in reps:
    reps[k]['validRange'] = {'minDenominator': RANGES[k][0], 'maxDenominator': RANGES[k][1]}
SYM = {'id': 'stool-plan', 'title': T('Bar stool plan symbol', 'Barhocker-Grundrisssymbol'), 'description': T('Plan sign of the stool: seat disc of 0.36 m diameter within the 0.42 m footprint ring of the legs.', 'Grundrisszeichen des Hockers: Sitzscheibe mit 0,36 m Durchmesser im Standkreis der Beine von 0,42 m.'),
       'semantics': 'true-extent', 'units': 'model-m', 'denominators': [200, 100, 50], 'pens': {'body': 0.25, 'detail': 0.13, 'texture': None, 'glazing': None, 'ground': 0.35}, 'hatches': [],
       'geometry': {'elevation': [], 'plan': [
           {'id': 'plan-seat', 'bone': 'ground', 'role': 'body', 'z': 2, 'filled': True, 'tone': 0.85, 'kind': 'ellipse', 'cx': 0.0, 'cy': 0.0, 'rx': 0.18, 'ry': 0.18},
           {'id': 'plan-ring', 'bone': 'ground', 'role': 'detail', 'z': 1, 'filled': False, 'tone': 0.5, 'kind': 'ellipse', 'cx': 0.0, 'cy': 0.0, 'rx': 0.21, 'ry': 0.21}]},
       'dimensionalNote': T('Model metres at real size, centred on the stool axis: seat radius 0.18 m, leg footprint radius 0.21 m.', 'Modellmeter in Originalgröße, auf die Hockerachse zentriert: Sitzradius 0,18 m, Standkreisradius der Beine 0,21 m.')}
A = assemble('bar-stool', T('Bar stool with footrest ring', 'Barhocker mit Fußring'),
             T('A 0.75 m swivel bar stool with a chromed four-leg frame, footrest ring, foam seat pad that sinks under load and a low backrest that shows the swivel as it turns.', 'Ein 0,75 m hoher Drehbarhocker mit verchromtem Vierbeingestell, Fußring, unter Last einsinkendem Schaumpolster und niedriger Lehne, die die Drehung beim Drehen zeigt.'),
             ['stool', 'bar', 'seating', 'chrome', 'swivel', 'interior'],
             T('Front elevation 0.42 x 0.87 m (seat 0.75 m). The swivel is shown through the backrest projection x = 0.15 m sin(azimuth); the swivel rotation itself is invisible for a round seat. Masses from tube length x 0.89 kg/m and nominal volumes.', 'Vorderansicht 0,42 x 0,87 m (Sitzhöhe 0,75 m). Die Drehung wird durch die Lehnenprojektion x = 0,15 m sin(Azimut) gezeigt; die Drehung selbst ist bei einem runden Sitz unsichtbar. Massen aus Rohrlänge x 0,89 kg/m und Nennvolumen.'),
             {'heightM': 0.87, 'widthM': 0.42, 'depthM': 0.42, 'massKg': round(mass, 3)}, MATS, RIG, anchors, reps, [SYM], P, clips, thermal, struct,
             {'defaultPose': 'unoccupied', 'defaultClip': 'swivel-turn', 'defaultScale': 50, 'viewpoints': ['elevation']},
             ['iso-10456-2007', 'en-1991-1-1', 'en-12520', 'en-1728', 'en-1993-1-1', 'iso-3386-1', 'iso-7730-2005', 'incropera-2018', 'semio-nwf'])
if __name__ == '__main__':
    main(A, 'bar-stool', ['unoccupied', 'occupied', 'turned-90', 'seat-proof-load'])
