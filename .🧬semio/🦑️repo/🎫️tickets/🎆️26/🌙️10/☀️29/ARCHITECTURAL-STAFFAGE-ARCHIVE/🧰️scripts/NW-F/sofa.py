import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nwf import *

E_ = [0.19, 0.7333, 1.2667, 1.81]
W_ = 2.0
SEAT_Y, SEAT_TOP, BACK_BOT, BACK_TOP = 0.22, 0.43, 0.20, 0.85

bones = [
    bone('ground', None, 'Floor reference', 'Bodenbezug', 0, 0, 0, 2.0, FIXED),
    bone('frame', 'ground', 'Timber frame, skirt and feet', 'Holzrahmen, Zarge und Füße', 0, 0, 0, 2.0, FIXED, (26.0, 1.0, 0.30, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
    bone('arm-left', 'frame', 'Left padded arm', 'Linke gepolsterte Armlehne', 0, 0.10, 0, 0.2, FIXED, (3.6, 0.10, 0.26, 'point', 'authored-estimate', 'semio-nwf')),
    bone('arm-right', 'frame', 'Right padded arm', 'Rechte gepolsterte Armlehne', 1.8, 0.10, 0, 0.2, FIXED, (3.6, 0.10, 0.26, 'point', 'authored-estimate', 'semio-nwf')),
]
for i in range(3):
    cx = (E_[i] + E_[i + 1]) / 2
    bones.append(bone(f'seat-{i + 1}', 'frame', f'Seat cushion {i + 1}', f'Sitzkissen {i + 1}', E_[i], SEAT_Y, 0, 0.5333, prism(90, -0.09, 0), (3.6, 0.2667, 0.10, 'point', 'authored-estimate', 'iso-3386-1')))
for i in range(3):
    bones.append(bone(f'back-{i + 1}', 'frame', f'Back cushion {i + 1}', f'Rückenkissen {i + 1}', E_[i], BACK_BOT, 0, 0.5333, prism(90, -0.04, 0), (2.6, 0.2667, 0.27, 'point', 'authored-estimate', 'iso-3386-1')))
rig = Rig(bones)
RIG = {'kind': 'structural', 'root': 'ground', 'bones': bones}
mass = sum(b['mass']['kg'] for b in bones if 'mass' in b)

M = {
    'beech': mat('beech-timber', 'Beech timber', 'Buchenholz', 0.9, 0.55, 720, 0.18, 1600, 1.2e10, 1.0e8, 'literature', 'wood-handbook-2021'),
    'ply': mat('birch-plywood', 'Birch plywood', 'Birkensperrholz', 0.9, 0.55, 680, 0.17, 1600, 9.0e9, 4.0e7, 'literature', 'iso-10456-2007'),
    'fabric': mat('upholstery-fabric', 'Polyester-cotton upholstery fabric', 'Polyester-Baumwoll-Polsterstoff', 0.9, 0.55, 400, 0.05, 1300, None, None, 'authored-estimate', 'iso-10456-2007'),
    'foam': mat('hr-foam', 'High-resilience PU foam 40 kg/m3', 'HR-Polyurethanschaum 40 kg/m3', 0.9, 0.55, 40, 0.045, 1400, 1.0e4, 6.0e3, 'authored-estimate', 'iso-3386-1'),
    'batting': mat('polyester-batting', 'Polyester fibre batting', 'Polyestervlies', 0.9, 0.55, 20, 0.04, 1300, None, None, 'authored-estimate', 'iso-10456-2007'),
}
MATS = list(M.values())
fab = lay('fabric', 'Cover fabric 1 mm', 'Bezugsstoff 1 mm', 'clothing', 'upholstery-fabric', 0.001, M['fabric'], 'authored-estimate')
regions = [
    reg('seat-cushions', 'Seat cushion tops and fronts', 'Sitzkissen oben und vorn', 'surface', 'upholstery-fabric', 1.28, 0.55, 'horizontal-plate-up', 0.8,
        [fab, lay('foam-seat', 'HR foam core 0.20 m', 'HR-Schaumkern 0,20 m', 'insulation', 'hr-foam', 0.20, M['foam'], 'authored-estimate')]),
    reg('back-cushions', 'Back cushion faces', 'Rückenkissen vorn', 'surface', 'upholstery-fabric', 1.1, 0.5, 'vertical-plate', 0.5,
        [fab, lay('foam-back', 'Foam core 0.15 m', 'Schaumkern 0,15 m', 'insulation', 'hr-foam', 0.15, M['foam'], 'authored-estimate')]),
    reg('arm-pads', 'Padded arms', 'Gepolsterte Armlehnen', 'surface', 'upholstery-fabric', 2.4, 0.5, 'vertical-plate', 0.5,
        [fab, lay('batting-arm', 'Batting and foam 0.04 m', 'Vlies und Schaum 0,04 m', 'insulation', 'polyester-batting', 0.04, M['batting'], 'authored-estimate')]),
    reg('base-rail', 'Upholstered skirt rail', 'Gepolsterte Zarge', 'surface', 'upholstery-fabric', 0.45, 0.16, 'vertical-plate', 0.35,
        [fab, lay('ply-skirt', 'Plywood skirt 18 mm', 'Sperrholzzarge 18 mm', 'timber', 'birch-plywood', 0.018, M['ply'], 'literature')]),
    reg('feet', 'Turned beech feet', 'Gedrechselte Buchenfüße', 'surface', 'beech-timber', 0.06, 0.05, 'cylinder', 0.3,
        [lay('beech-foot', 'Beech foot, half thickness 0.025 m', 'Buchenfuß, halbe Dicke 0,025 m', 'timber', 'beech-timber', 0.025, M['beech'], 'literature', 'wood-handbook-2021')]),
]
RIDS = [r['id'] for r in regions]
base = {'seat-cushions': (1.0, 0.3), 'back-cushions': (1.0, 0.1), 'arm-pads': (1.0, 0.2), 'base-rail': (1.0, 0.0), 'feet': (1.0, 0.0)}
ctx = Ctx(RIDS, base)

anchors = [
    anc(rig, 'foot-left', 'frame', 0.10, 0, 'contact', 'Left feet (front and rear pair)', 'Linke Füße (vorderes und hinteres Paar)', support='contact', normalDeg=90, footprintLengthM=0.05, footprintWidthM=0.62, frictionCoefficient=0.5),
    anc(rig, 'foot-right', 'frame', 1.90, 0, 'contact', 'Right feet (front and rear pair)', 'Rechte Füße (vorderes und hinteres Paar)', support='contact', normalDeg=90, footprintLengthM=0.05, footprintWidthM=0.62, frictionCoefficient=0.5),
]
for i in range(3):
    anchors.append(anc(rig, f'seat-load-{i + 1}', f'seat-{i + 1}', (E_[i] + E_[i + 1]) / 2, SEAT_TOP, 'load', f'Sitting point on seat cushion {i + 1}', f'Sitzpunkt auf Sitzkissen {i + 1}'))
CONT = ['foot-left', 'foot-right']
OCC = 'EN 1991-1-1 category A: 75 kg adult occupant, 9.81 m/s2'
occ = lambda i, anchor, n=735.5: load(f'occupant-{i}', f'Seated adult {i} (75 kg)', f'Sitzende erwachsene Person {i} (75 kg)', 'occupant', -n, anchor=anchor, ref='en-1991-1-1')
sink = lambda **k: {b: {'dy': v} for b, v in k.items()}
P = []
P.append(pose(ctx, 'unoccupied', 'Unoccupied', 'Unbesetzt', 'Empty sofa beside a south window; a sun patch (350 W/m2 behind glazing) reaches the left seat cushion and arm.', 'Leeres Sofa neben einem Südfenster; ein Sonnenfleck (350 W/m2 hinter Verglasung) trifft das linke Sitzkissen und die Armlehne.', contacts=CONT))
P.append(pose(ctx, 'one-seated', 'One person seated', 'Eine Person sitzt', 'One adult sits on the centre cushion: it sinks 60 mm, the neighbours 12 mm.', 'Eine erwachsene Person sitzt auf dem mittleren Kissen: es sinkt um 60 mm, die Nachbarn um 12 mm.',
              bones={'seat-1': {'dy': -0.012}, 'seat-2': {'dy': -0.060}, 'seat-3': {'dy': -0.012}, 'back-2': {'dy': -0.018}}, contacts=CONT, loads=[occ(1, 'seat-load-2')],
              over={'seat-cushions': (1.0, 0.22)}, contact={'seat-cushions': (0.11,), 'back-cushions': (0.07,)}))
P.append(pose(ctx, 'two-seated', 'Two persons seated', 'Zwei Personen sitzen', 'Two adults sit on the outer cushions (60 mm each); the centre cushion stays almost level.', 'Zwei Erwachsene sitzen auf den äußeren Kissen (je 60 mm); das mittlere Kissen bleibt fast eben.',
              bones={'seat-1': {'dy': -0.060}, 'seat-2': {'dy': -0.012}, 'seat-3': {'dy': -0.060}, 'back-1': {'dy': -0.018}, 'back-3': {'dy': -0.018}}, contacts=CONT, loads=[occ(1, 'seat-load-1'), occ(2, 'seat-load-3')],
              over={'seat-cushions': (1.0, 0.1)}, contact={'seat-cushions': (0.22,), 'back-cushions': (0.14,)}))
P.append(pose(ctx, 'lounging', 'Person lounging', 'Person liegt', 'One adult lies along the three seat cushions: low contact pressure, 20 mm sink everywhere.', 'Eine erwachsene Person liegt über die drei Sitzkissen: geringe Pressung, überall 20 mm Einsinken.',
              bones={'seat-1': {'dy': -0.020}, 'seat-2': {'dy': -0.020}, 'seat-3': {'dy': -0.020}, 'back-1': {'dy': -0.008}, 'back-2': {'dy': -0.008}, 'back-3': {'dy': -0.008}}, contacts=CONT,
              loads=[load('occupant-1', 'Adult lying on the seats (75 kg)', 'Erwachsene Person liegt auf den Sitzen (75 kg)', 'occupant', 0, dist={'bone': 'frame', 'fromM': 0.2, 'toM': 1.8, 'wxNPerM': 0, 'wyNPerM': -459.7}, ref='en-1991-1-1')],
              over={'seat-cushions': (1.0, 0.0)}, contact={'seat-cushions': (0.4,), 'back-cushions': (0.12,), 'arm-pads': (0.03,)}))
P.append(pose(ctx, 'seat-proof-load', 'Seat proof load', 'Sitz-Prüflast', 'Static seat test load 1600 N on the centre cushion: foam bottoms out at 90 mm.', 'Statische Sitz-Prüflast 1600 N auf dem mittleren Kissen: der Schaum geht bei 90 mm auf Block.',
              bones={'seat-1': {'dy': -0.02}, 'seat-2': {'dy': -0.09}, 'seat-3': {'dy': -0.02}, 'back-2': {'dy': -0.03}}, contacts=CONT,
              loads=[load('proof-seat', 'Static seat test load (1600 N)', 'Statische Sitzprüflast (1600 N)', 'imposed', -1600, anchor='seat-load-2', ref='en-12520')],
              over={'seat-cushions': (1.0, 0.0)}))
P.append(pose(ctx, 'impact-key', 'Sit-down impact (key)', 'Aufsetzen, Spitze (Schlüssel)', 'Dynamic overshoot 72 mm while sitting down.', 'Dynamisches Überschwingen 72 mm beim Hinsetzen.', key=True,
              bones={'seat-1': {'dy': -0.015}, 'seat-2': {'dy': -0.072}, 'seat-3': {'dy': -0.015}, 'back-2': {'dy': -0.022}}, contacts=CONT, loads=[occ(1, 'seat-load-2', 955.0)],
              over={'seat-cushions': (1.0, 0.22)}, contact={'seat-cushions': (0.11,), 'back-cushions': (0.07,)}))
P.append(pose(ctx, 'recovering-key', 'Foam recovering (key)', 'Schaum erholt sich (Schlüssel)', 'Seat foam half recovered after standing up.', 'Sitzschaum nach dem Aufstehen halb erholt.', key=True,
              bones={'seat-1': {'dy': -0.004}, 'seat-2': {'dy': -0.030}, 'seat-3': {'dy': -0.004}, 'back-2': {'dy': -0.008}}, contacts=CONT,
              over={'seat-cushions': (1.0, 0.3)}, contact={'seat-cushions': (0.11,)}))

clips = [
    clip('sit-down-and-settle', 'Sit down and settle', 'Hinsetzen und Einsinken', 'An adult drops onto the centre cushion (72 mm dynamic peak) and settles at 60 mm while the foam relaxes.', 'Ein Erwachsener lässt sich auf das mittlere Kissen fallen (72 mm Spitze) und pendelt sich bei 60 mm ein, während der Schaum relaxiert.', 2.5,
         [{'t': 0, 'pose': 'unoccupied', 'ease': 'in'}, {'t': 0.6, 'pose': 'impact-key', 'ease': 'out'}, {'t': 1.2, 'pose': 'one-seated', 'ease': 'smooth'}, {'t': 2.5, 'pose': 'one-seated', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 2.5} for a in CONT], [('drop', 0, 0.6, 'Drop onto the seat', 'Fallen auf den Sitz'), ('settle', 0.6, 2.5, 'Foam settles', 'Schaum pendelt sich ein')],
         [(0.6, 'load-on', 'Occupant sits down', 'Person setzt sich', 'occupant-1'), (0.6, 'thermal', 'Seat surface starts taking up 30 degC contact heat', 'Sitzfläche nimmt 30-degC-Kontaktwärme auf', None)],
         [('load:occupant-1', 'smooth', [(0, 0), (0.3, 0.2), (0.6, 1.3), (1.2, 1.0), (2.5, 1.0)])]),
    clip('stand-up-and-recover', 'Stand up and foam recovery', 'Aufstehen und Schaumerholung', 'The occupant stands up; the foam recovers slowly from 60 mm sink (viscoelastic recovery shown compressed to 4 s).', 'Die Person steht auf; der Schaum erholt sich langsam aus 60 mm Einsinken (viskoelastische Erholung auf 4 s gerafft).', 4.0,
         [{'t': 0, 'pose': 'one-seated', 'ease': 'out'}, {'t': 0.7, 'pose': 'recovering-key', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'unoccupied', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 4.0} for a in CONT], [('rise', 0, 0.7, 'Occupant rises', 'Person steht auf'), ('recover', 0.7, 4.0, 'Foam recovery', 'Schaumerholung')],
         [(0.7, 'load-off', 'Occupant has left the seat', 'Person hat den Sitz verlassen', 'occupant-1')],
         [('load:occupant-1', 'smooth', [(0, 1), (0.5, 0.2), (0.7, 0)])]),
    clip('lie-down-on-sofa', 'Lie down on the sofa', 'Auf dem Sofa ausstrecken', 'One occupant sits, then stretches out: the load spreads from the centre cushion over all three.', 'Eine Person sitzt und streckt sich dann aus: die Last verteilt sich vom mittleren Kissen auf alle drei.', 5.0,
         [{'t': 0, 'pose': 'unoccupied', 'ease': 'smooth'}, {'t': 1.0, 'pose': 'one-seated', 'ease': 'smooth'}, {'t': 2.5, 'pose': 'one-seated', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'lounging', 'ease': 'smooth'}, {'t': 5.0, 'pose': 'lounging', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 5.0} for a in CONT], [('sit', 0, 2.5, 'Sit', 'Sitzen'), ('lie', 2.5, 5.0, 'Stretch out', 'Ausstrecken')],
         [(1.0, 'load-on', 'Occupant sits', 'Person setzt sich', 'occupant-1')], [('region:seat-cushions:sunlitFraction', 'smooth', [(0, 0.3), (1.0, 0.22), (4.0, 0.0)])]),
]
for c in clips:
    for t in c['tracks']:
        pass

thermal = {
    'environment': {'airTempC': 21, 'meanRadiantTempC': 21, 'relativeHumidity': 0.5, 'windMS': 0.1, 'windDirectionDeg': 0, 'solarWM2': 350, 'diffuseFraction': 0.15, 'pressurePa': 101325},
    'operating': [], 'regions': regions, 'emitters': [],
    'assumptions': [
        ('Passive upholstery: absorbed sun, long-wave exchange, convection and conduction; foam layers carry the thermal resistance (0.20 m HR foam = 4.4 m2K/W), so the covers follow air and sun quickly (heat capacity 3.3 kJ/m2K from the thermal penetration depth sqrt(alpha t) of one hour).', 'Passive Polster: absorbierte Sonne, Langwellenaustausch, Konvektion und Leitung; die Schaumschichten tragen den Wärmewiderstand (0,20 m HR-Schaum = 4,4 m2K/W), die Bezüge folgen Luft und Sonne daher schnell (Wärmekapazität 3,3 kJ/m2K aus der Eindringtiefe sqrt(alpha t) einer Stunde).'),
        ('Foam conductivity 0.045 W/mK, density 40 kg/m3 and cover fabric (effective density 400 kg/m3, 0.05 W/mK) are authored-estimates within manufacturer ranges; timber and plywood after ISO 10456.', 'Schaum-Wärmeleitfähigkeit 0,045 W/mK, Dichte 40 kg/m3 und Bezugsstoff (effektive Dichte 400 kg/m3, 0,05 W/mK) sind Schätzwerte innerhalb üblicher Herstellerbereiche; Holz und Sperrholz nach ISO 10456.'),
        ('Occupied poses cover part of the cushions with a clothed body (contact surface 30 degC, ISO 7730) and shade it; contact fractions are authored estimates (0.14 m2 seat and 0.08 m2 back per adult).', 'Besetzte Posen bedecken einen Teil der Kissen mit einem bekleideten Körper (Kontaktfläche 30 degC, ISO 7730) und beschatten ihn; Kontaktanteile sind Schätzwerte (0,14 m2 Sitz, 0,08 m2 Rücken je Erwachsenem).'),
        ('Reference case: south-window sun patch of 350 W/m2 (about half of 700 W/m2 direct sun after double glazing), 21 degC air, still room. Sunlit fractions are scenario data of each pose.', 'Referenzfall: Südfenster-Sonnenfleck mit 350 W/m2 (etwa die Hälfte von 700 W/m2 direkter Sonne nach Zweifachverglasung), 21 degC Luft, ruhiger Raum. Besonnte Anteile sind Szenariodaten der jeweiligen Pose.'),
    ]}
thermal['assumptions'] = [T(a, b) for a, b in thermal['assumptions']]
thermal['emitters'] = []

struct = structure(CONT, [
    ('The sofa is a rigid frame on two foot contacts (front and rear feet coincide in elevation); each cushion is a rigid slab on a prismatic joint that sinks into the frame, the foam deformation is hidden behind the skirt rail.', 'Das Sofa ist ein starrer Rahmen auf zwei Fußkontakten (vordere und hintere Füße fallen in der Ansicht zusammen); jedes Kissen ist eine starre Platte auf einem Schiebegelenk, die in den Rahmen einsinkt, die Schaumverformung verdeckt die Zarge.'),
    ('Sink depth s = 0.4 (sigma/sigma40) t with sigma = 0.8 x 735 N / 0.13 m2 = 4.5 kPa, CLD40 = 6 kPa (ISO 3386-1) and t = 0.20 m gives 60 mm; the 1600 N proof load bottoms out at 90 mm.', 'Einsinktiefe s = 0,4 (sigma/sigma40) t mit sigma = 0,8 x 735 N / 0,13 m2 = 4,5 kPa, CLD40 = 6 kPa (ISO 3386-1) und t = 0,20 m ergibt 60 mm; die Prüflast 1600 N lässt den Schaum bei 90 mm auf Block gehen.'),
    ('The seat proof load magnitude (1600 N) follows the static seat test of EN 1728 as used by EN 12520; it is an authored-estimate to be checked against the standard text.', 'Die Größe der Sitz-Prüflast (1600 N) folgt dem statischen Sitztest nach EN 1728, wie ihn EN 12520 verwendet; sie ist ein Schätzwert und am Normtext zu prüfen.'),
])

#region geometry
def build(den):
    S_ = Shapes(rig)
    return S_


def arm(S_, sid, x0, x1, bone_, region='arm-pads', pen=None, tone=0.5, rt=0.07, z=6):
    return S_.path(sid, bone_, rbox(x0, 0.10, x1, 0.62, rt=rt), region=region, material='upholstery-fabric', tone=tone, z=8)


def cushions(S_, prefix, bonepref, y0, y1, rt, region, z, tone, mat='upholstery-fabric'):
    for i in range(3):
        S_.path(f'{prefix}-{i + 1}', f'{bonepref}-{i + 1}', rbox(E_[i], y0, E_[i + 1], y1, rt=rt), region=region, material=mat, tone=tone, z=z)


def foot(S_, sid, cx, top, w_top, w_bot, region='feet', z=2):
    S_.poly(sid, 'frame', [(cx - w_bot / 2, 0), (cx + w_bot / 2, 0), (cx + w_top / 2, top), (cx - w_top / 2, top)], region=region, material='beech-timber', tone=0.4, z=z)


# 1:500
s5 = Shapes(rig)
s5.path('sofa-body', 'frame', [('M', 0.0, 0.10), ('L', 0.0, 0.55), ('Q', 0.0, 0.62, 0.06, 0.62), ('L', 0.14, 0.62), ('Q', 0.20, 0.62, 0.20, 0.68), ('L', 0.20, 0.79), ('Q', 0.20, 0.85, 0.27, 0.85), ('L', 1.73, 0.85), ('Q', 1.80, 0.85, 1.80, 0.79),
                               ('L', 1.80, 0.68), ('Q', 1.80, 0.62, 1.86, 0.62), ('L', 1.94, 0.62), ('Q', 2.0, 0.62, 2.0, 0.55), ('L', 2.0, 0.10), ('Z',)],
        regions=['seat-cushions', 'back-cushions', 'arm-pads', 'base-rail'], tone=0.55, z=4)
s5.line('seat-line', 'frame', [(0.2, 0.43), (1.8, 0.43)], role='body', z=5, regions=None, penMm=0.13, region='seat-cushions')
s5.rect('foot-l', 'frame', 0.06, 0.0, 0.20, 0.12, region='feet', material='beech-timber', tone=0.3, z=2)
s5.rect('foot-r', 'frame', 1.80, 0.0, 1.94, 0.12, region='feet', material='beech-timber', tone=0.3, z=2)

# 1:200
s2 = Shapes(rig)
for nm, b_, o in (('l', 'arm-left', 0.0), ('r', 'arm-right', 1.8)):
    s2.path(f'arm-foot-{nm}', b_, [('M', o + 0.09, 0), ('L', o + 0.15, 0), ('L', o + 0.155, 0.10), ('L', o + 0.20, 0.10), ('L', o + 0.20, 0.55), ('Q', o + 0.20, 0.62, o + 0.13, 0.62), ('L', o + 0.07, 0.62), ('Q', o, 0.62, o, 0.55), ('L', o, 0.10), ('L', o + 0.085, 0.10), ('Z',)],
            regions=['arm-pads', 'feet'], material='upholstery-fabric', tone=0.5, z=8)
s2.path('back', 'frame', rbox(0.19, BACK_BOT, 1.81, BACK_TOP, rt=0.06), region='back-cushions', material='upholstery-fabric', tone=0.6, z=3)
for x in (E_[1], E_[2]):
    s2.line(f'back-seam-{x:.2f}', 'frame', [(x, 0.22), (x, 0.84)], role='detail', z=3.5, region='back-cushions')
cushions(s2, 'seat', 'seat', 0.20, SEAT_TOP, 0.03, 'seat-cushions', 5, 0.7)
s2.rect('skirt', 'frame', 0.15, 0.10, 1.85, 0.24, region='base-rail', material='upholstery-fabric', tone=0.45, z=7)

# 1:100
s1 = Shapes(rig)
arm(s1, 'arm-l', 0.0, 0.20, 'arm-left', rt=0.08)
arm(s1, 'arm-r', 1.8, 2.0, 'arm-right', rt=0.08)
s1.line('arm-welt-l', 'arm-left', [(0.0, 0.50), (0.20, 0.50)], role='detail', z=6.5, region='arm-pads')
s1.line('arm-welt-r', 'arm-right', [(1.8, 0.50), (2.0, 0.50)], role='detail', z=6.5, region='arm-pads')
cushions(s1, 'back', 'back', BACK_BOT, BACK_TOP, 0.05, 'back-cushions', 3, 0.62)
cushions(s1, 'seat', 'seat', 0.20, SEAT_TOP, 0.035, 'seat-cushions', 5, 0.72)
for i in range(3):
    s1.line(f'seat-piping-{i + 1}', f'seat-{i + 1}', [(E_[i] + 0.06, SEAT_TOP - 0.05), (E_[i + 1] - 0.06, SEAT_TOP - 0.05)], role='detail', z=5.5, region='seat-cushions')
s1.rect('skirt', 'frame', 0.15, 0.10, 1.85, 0.24, region='base-rail', material='upholstery-fabric', tone=0.45, z=7)
foot(s1, 'foot-l', 0.12, 0.12, 0.07, 0.05)
foot(s1, 'foot-r', 1.88, 0.12, 0.07, 0.05)

# 1:50
s0 = Shapes(rig)
arm(s0, 'arm-l', 0.0, 0.20, 'arm-left', rt=0.09)
arm(s0, 'arm-r', 1.8, 2.0, 'arm-right', rt=0.09)
for nm, b_, xa, xb in (('l', 'arm-left', 0.0, 0.20), ('r', 'arm-right', 1.8, 2.0)):
    s0.line(f'arm-top-piping-{nm}', b_, [(xa + 0.04, 0.52), (xa + 0.04, 0.56), (xb - 0.04, 0.56), (xb - 0.04, 0.52)], role='detail', z=6.5, region='arm-pads')
    s0.line(f'arm-welt-{nm}', b_, [(xa, 0.22), (xb, 0.22)], role='detail', z=6.5, region='arm-pads')
cushions(s0, 'back', 'back', BACK_BOT, BACK_TOP, 0.055, 'back-cushions', 3, 0.62)
cushions(s0, 'seat', 'seat', 0.20, SEAT_TOP, 0.04, 'seat-cushions', 5, 0.72)
for i in range(3):
    s0.line(f'back-piping-{i + 1}', f'back-{i + 1}', [(E_[i] + 0.04, BACK_TOP - 0.10), (E_[i] + 0.04, BACK_TOP - 0.06), (E_[i + 1] - 0.04, BACK_TOP - 0.06), (E_[i + 1] - 0.04, BACK_TOP - 0.10)], role='detail', z=3.5, region='back-cushions')
    s0.line(f'seat-piping-{i + 1}', f'seat-{i + 1}', [(E_[i] + 0.04, SEAT_TOP - 0.06), (E_[i] + 0.04, SEAT_TOP - 0.05), (E_[i + 1] - 0.04, SEAT_TOP - 0.05), (E_[i + 1] - 0.04, SEAT_TOP - 0.06)], role='detail', z=5.5, region='seat-cushions')
s0.rect('skirt', 'frame', 0.15, 0.10, 1.85, 0.24, region='base-rail', material='upholstery-fabric', tone=0.45, z=7)
foot(s0, 'foot-l', 0.12, 0.12, 0.07, 0.05)
foot(s0, 'foot-r', 1.88, 0.12, 0.07, 0.05)
for nm, cx in (('l', 0.12), ('r', 1.88)):
    s0.line(f'foot-ring-{nm}', 'frame', [(cx - 0.03, 0.065), (cx + 0.03, 0.065)], role='detail', z=2.5, region='feet')
#endregion

reps = {
    '500': rep(500, s5, D(('One composed sofa sign 4.0 x 1.7 mm: arms, back and seat as one outline with a seat line, on two foot blocks.', 'Ein zusammengesetztes Sofazeichen 4,0 x 1,7 mm: Armlehnen, Rücken und Sitz als eine Kontur mit Sitzlinie auf zwei Fußblöcken.'),
        ('Arm tops at 0.62 m, back top at 0.85 m, seat line at 0.43 m and the two feet.', 'Armlehnenoberkante 0,62 m, Rückenoberkante 0,85 m, Sitzlinie 0,43 m und die beiden Füße.'),
        ('Arms, back cushions, seat cushions and skirt are one filled outline.', 'Armlehnen, Rückenkissen, Sitzkissen und Zarge sind eine gefüllte Kontur.'),
        ('Cushion seams, piping, rear feet.', 'Kissenfugen, Paspeln, hintere Füße.'),
        ('Turned feet (0.06 m, 0.12 mm) are thickened to 0.14 m blocks: 0.28 mm wide; the overshoot of 0.08 m stays inside the real outline tolerance.', 'Gedrechselte Füße (0,06 m, 0,12 mm) sind zu 0,14 m breiten Blöcken verdickt; die Überhöhung bleibt in der Strichbreite von 0,18 mm.'),
        ('None.', 'Keine.'), ('Body tone 0.55, feet 0.3, seat line 0.13 mm.', 'Körperton 0,55, Füße 0,3, Sitzlinie 0,13 mm.'),
        ('The single body shape lists all four upholstered regions as merged; feet have their own shape.', 'Die eine Körperform führt alle vier gepolsterten Regionen als zusammengefasst; die Füße haben eigene Formen.'))),
    '200': rep(200, s2, D(('Sofa 10 x 4.3 mm in main masses: two arms, one back band, three seat cushions, skirt, two feet.', 'Sofa 10 x 4,3 mm in Hauptmassen: zwei Armlehnen, ein Rückenband, drei Sitzkissen, Zarge, zwei Füße.'),
        ('Rounded arm tops, back band with two seam lines, three seat cushions, skirt rail.', 'Gerundete Armlehnen, Rückenband mit zwei Nahtlinien, drei Sitzkissen, Zargenleiste.'),
        ('The three back cushions are one band; rear feet coincide with the front feet.', 'Die drei Rückenkissen sind ein Band; hintere Füße fallen mit den vorderen zusammen.'),
        ('Piping, cushion corners smaller than 0.03 m.', 'Paspeln, Kissenecken unter 0,03 m.'),
        ('Seams are 0.13 mm detail lines.', 'Nähte sind 0,13-mm-Detaillinien.'), ('None.', 'Keine.'), ('Fabric tone 0.45-0.7, feet 0.4.', 'Stofftöne 0,45-0,7, Füße 0,4.'),
        ('Back band = back-cushions, three seat shapes = seat-cushions, arms = arm-pads, skirt = base-rail.', 'Rückenband = back-cushions, drei Sitzformen = seat-cushions, Arme = arm-pads, Zarge = base-rail.'))),
    '100': rep(100, s1, D(('Sofa 20 x 8.5 mm: arms with welt, three back and three seat cushions with rounded corners and seat piping, skirt, tapered feet.', 'Sofa 20 x 8,5 mm: Armlehnen mit Ziernaht, drei Rücken- und drei Sitzkissen mit gerundeten Ecken und Sitzpaspel, Zarge, konische Füße.'),
        ('Cushion division (compresses independently), piping along each seat front, arm welt.', 'Kissenteilung (sinkt einzeln ein), Paspel an jeder Sitzfront, Armnaht.'),
        ('Each cushion is one shape; piping at the cushion sides is merged into the outline.', 'Jedes Kissen ist eine Form; Paspeln an den Kissenseiten gehen in der Kontur auf.'),
        ('Back piping, foot rings, arm piping.', 'Rückenpaspeln, Fußringe, Armpaspeln.'),
        ('Piping lines are inset 0.05 m (0.5 mm) so that they stay separate from the outline at pen 0.35.', 'Paspeln sind 0,05 m (0,5 mm) eingerückt, damit sie bei Stift 0,35 von der Kontur getrennt bleiben.'),
        ('None.', 'Keine.'), ('Cushion tones 0.62 and 0.72, skirt 0.45.', 'Kissentöne 0,62 und 0,72, Zarge 0,45.'),
        ('Each cushion carries its region; welts and piping carry the region of the surface they lie on.', 'Jedes Kissen trägt seine Region; Nähte und Paspeln tragen die Region ihrer Fläche.'))),
    '50': rep(50, s0, D(('Sofa 40 x 17 mm with all cushions, piping, arm welts and top piping and turned feet with ring.', 'Sofa 40 x 17 mm mit allen Kissen, Paspeln, Armnähten und Armpaspel, gedrechselten Füßen mit Ring.'),
        ('Front boxing piping around every cushion, arm top piping, foot ring.', 'Frontpaspeln um jedes Kissen, Armpaspel oben, Fußring.'),
        ('Rear feet coincide with the front feet; the cover seam on the back is not visible from the front.', 'Hintere Füße fallen mit den vorderen zusammen; die Rückennaht ist von vorn nicht sichtbar.'),
        ('Fabric weave, buttons, zip, cushion filling folds.', 'Gewebestruktur, Knöpfe, Reißverschluss, Füllfalten.'),
        ('Piping is inset 0.04 m (0.8 mm), welts are 0.25 mm detail lines.', 'Paspeln sind 0,04 m (0,8 mm) eingerückt, Nähte sind 0,25-mm-Detaillinien.'),
        ('No hatching: upholstery is shown by tone.', 'Keine Schraffur: Polster werden durch Ton gezeigt.'), ('Tone 0.62 back, 0.72 seat, 0.45 skirt, 0.4 feet.', 'Ton 0,62 Rücken, 0,72 Sitz, 0,45 Zarge, 0,4 Füße.'),
        ('One shape per cushion and per arm; piping lines carry their cushion region, so overlays follow every cushion separately.', 'Eine Form je Kissen und Armlehne; Paspeln tragen die Region ihres Kissens, Überlagerungen folgen jedem Kissen einzeln.'))),
}
reps['500'] = rep(500, s5, reps['500']['decisions'], valid=RANGES['500'])
for k in ('200', '100', '50'):
    reps[k]['validRange'] = {'minDenominator': RANGES[k][0], 'maxDenominator': RANGES[k][1]}

SYM = {
    'id': 'sofa-plan', 'title': T('Sofa plan symbol', 'Sofa-Grundrisssymbol'),
    'description': T('Conventional floor-plan sign of a three-seat sofa with arms, back and three seat cushions, true to the real footprint 2.00 x 0.90 m.', 'Übliches Grundrisszeichen eines Dreisitzers mit Armlehnen, Rücken und drei Sitzkissen, maßgetreu im Grundriss 2,00 x 0,90 m.'),
    'semantics': 'true-extent', 'units': 'model-m', 'denominators': [200, 100, 50],
    'pens': {'body': 0.25, 'detail': 0.13, 'texture': None, 'glazing': None, 'ground': 0.35}, 'hatches': [],
    'geometry': {'elevation': [], 'plan': []}, 'dimensionalNote': T('Model metres at real size: x along the sofa 0 to 2.00 m, y from the front edge (0) to the back (0.90 m). Not for 1:500.', 'Modellmeter in Originalgröße: x entlang des Sofas 0 bis 2,00 m, y von der Vorderkante (0) bis zum Rücken (0,90 m). Nicht für 1:500.')}
sp = Shapes(rig)
sp.items = []
def pl(sid, pts, **kw):
    s = {'id': sid, 'bone': 'ground', 'role': kw.get('role', 'body'), 'z': kw.get('z', 1), 'filled': kw.get('filled', True), 'tone': kw.get('tone', 0.9), 'kind': 'poly', 'points': pts, 'closed': kw.get('closed', True)}
    SYM['geometry']['plan'].append(s)
pl('plan-outline', [[0, 0], [2.0, 0], [2.0, 0.90], [0, 0.90]], tone=0.95, z=1)
pl('plan-back', [[0.2, 0.70], [1.8, 0.70], [1.8, 0.90], [0.2, 0.90]], tone=0.8, z=2)
pl('plan-arm-l', [[0, 0], [0.2, 0], [0.2, 0.90], [0, 0.90]], tone=0.8, z=2)
pl('plan-arm-r', [[1.8, 0], [2.0, 0], [2.0, 0.90], [1.8, 0.90]], tone=0.8, z=2)
for i in range(3):
    pl(f'plan-seat-{i + 1}', [[E_[i], 0.02], [E_[i + 1], 0.02], [E_[i + 1], 0.70], [E_[i], 0.70]], tone=0.95, z=3)
SYMS = [SYM]

A = assemble('three-seat-sofa', T('Three-seat sofa', 'Dreisitzer-Sofa'),
             T('A 2.00 m upholstered three-seat sofa with padded arms, three separately sinking seat cushions and three back cushions on turned beech feet; front elevation and plan symbol.', 'Ein 2,00 m breites gepolstertes Dreisitzer-Sofa mit gepolsterten Armlehnen, drei einzeln einsinkenden Sitzkissen und drei Rückenkissen auf gedrechselten Buchenfüßen; Vorderansicht und Grundrisssymbol.'),
             ['sofa', 'seating', 'upholstery', 'interior', 'foam'],
             T('Front elevation, 2.00 x 0.85 m, depth 0.90 m. Masses from nominal volumes x density, foam 40 kg/m3 (authored-estimate). Cushions are rigid slabs on prismatic joints; sink depths follow foam compression stress (ISO 3386-1) and are authored estimates. Occupants are loads, not drawn.', 'Vorderansicht, 2,00 x 0,85 m, Tiefe 0,90 m. Massen aus Nennvolumen x Dichte, Schaum 40 kg/m3 (Schätzwert). Kissen sind starre Platten auf Schiebegelenken; Einsinktiefen folgen der Schaumdruckspannung (ISO 3386-1) und sind Schätzwerte. Personen sind Lasten, nicht gezeichnet.'),
             {'heightM': 0.85, 'widthM': 2.0, 'depthM': 0.9, 'massKg': round(mass, 3)}, MATS, RIG, anchors, reps, SYMS, P, clips, thermal, struct,
             {'defaultPose': 'unoccupied', 'defaultClip': 'sit-down-and-settle', 'defaultScale': 50, 'viewpoints': ['elevation']},
             ['iso-10456-2007', 'en-1991-1-1', 'en-12520', 'en-1728', 'iso-3386-1', 'iso-7730-2005', 'wood-handbook-2021', 'incropera-2018', 'semio-nwf'])

if __name__ == '__main__':
    errs = validate(A)
    if '--write' in sys.argv and not errs:
        write(A, asset_path('three-seat-sofa'))
    if '--suit' in sys.argv:
        for k in ('500', '200', '100', '50'):
            suitability(A, k, poses=['unoccupied', 'one-seated', 'seat-proof-load'])
    for k in sys.argv:
        if k.startswith('--prev='):
            print(preview(A, k[7:], zoom=8))
