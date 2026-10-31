import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nwf import *

HH, HF = 0.78, 1.38
SL_TOP, MAT_TOP = 0.285, 0.485
bones = [bone('ground', None, 'Floor reference', 'Bodenbezug', 0, 0, 0, 2.12, FIXED),
         bone('frame', 'ground', 'Bed frame: posts, boards and side rails', 'Bettrahmen: Pfosten, Bretter und Seitenholme', 0, 0, 0, 2.12, FIXED, (27.6, 1.06, 0.3, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
         bone('slat-head', 'frame', 'Head section of the slatted base', 'Kopfteil des Lattenrosts', HH, SL_TOP, 0, 0.72, rev(-65, 0), (3.5, -0.36, -0.01, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
         bone('slat-mid', 'frame', 'Middle section of the slatted base', 'Mittelteil des Lattenrosts', HH, SL_TOP, 0, 0.60, FIXED, (3.2, 0.3, -0.01, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
         bone('slat-foot', 'frame', 'Foot section of the slatted base', 'Fußteil des Lattenrosts', HF, SL_TOP, 0, 0.68, FIXED, (2.7, 0.34, -0.01, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
         bone('mat-head', 'slat-head', 'Mattress head part', 'Matratze Kopfteil', 0, 0, 0, 0.72, prism(90, -0.06, 0), (8.1, -0.36, 0.1, 'uniform', 'authored-estimate', 'iso-3386-1')),
         bone('mat-mid', 'slat-mid', 'Mattress middle part', 'Matratze Mittelteil', 0, 0, 0, 0.60, prism(90, -0.06, 0), (7.0, 0.3, 0.1, 'uniform', 'authored-estimate', 'iso-3386-1')),
         bone('mat-foot', 'slat-foot', 'Mattress foot part', 'Matratze Fußteil', 0, 0, 0, 0.68, prism(90, -0.06, 0), (7.1, 0.34, 0.1, 'uniform', 'authored-estimate', 'iso-3386-1'))]
rig = Rig(bones)
RIG = {'kind': 'mechanical', 'root': 'ground', 'bones': bones}
mass = sum(b['mass']['kg'] for b in bones if 'mass' in b)
pine = mat('pine-timber', 'Solid pine, waxed', 'Kiefer massiv, gewachst', 0.9, 0.55, 500, 0.13, 1600, 1.1e10, 2.4e7, 'authored-estimate', 'iso-10456-2007')
birch = mat('birch-slat', 'Birch laminated slats 12 mm', 'Birkenschichtholz-Federleisten 12 mm', 0.9, 0.55, 650, 0.15, 1600, 9.0e9, 4.0e7, 'literature', 'iso-10456-2007')
cover = mat('quilted-cover', 'Quilted cotton cover with batting', 'Gesteppter Baumwollbezug mit Vlies', 0.9, 0.5, 150, 0.04, 1300, None, None, 'authored-estimate', 'iso-10456-2007')
foam = mat('cold-foam', 'Cold-cure PU foam 40 kg/m3', 'Kaltschaum 40 kg/m3', 0.9, 0.5, 40, 0.045, 1400, 1.0e4, 6.0e3, 'authored-estimate', 'iso-3386-1')
MATS = [pine, birch, cover, foam]
regions = [
    reg('mattress', 'Mattress cover and sides', 'Matratzenbezug und Seiten', 'surface', 'quilted-cover', 2.5, 0.9, 'horizontal-plate-up', 0.8,
        [lay('quilt', 'Quilted cover 10 mm', 'Steppbezug 10 mm', 'clothing', 'quilted-cover', 0.01, cover, 'authored-estimate'), lay('foam-core', 'Cold foam 0.18 m', 'Kaltschaum 0,18 m', 'insulation', 'cold-foam', 0.18, foam, 'authored-estimate')]),
    reg('slats', 'Slatted base', 'Lattenrost', 'surface', 'birch-slat', 1.5, 0.07, 'horizontal-plate-down', 0.3, [lay('slat-birch', 'Birch slat 12 mm', 'Birkenleiste 12 mm', 'timber', 'birch-slat', 0.012, birch, 'literature')],
        hidden=T('The slats lie under the mattress and behind the side rail; they show only where a raised section opens the base.', 'Die Latten liegen unter der Matratze und hinter dem Seitenholm; sie zeigen sich nur dort, wo ein angehobener Teil den Rost öffnet.')),
    reg('bed-frame', 'Frame, boards and side rails', 'Rahmen, Bretter und Seitenholme', 'surface', 'pine-timber', 4.0, 0.16, 'vertical-plate', 0.45, [lay('pine-member', 'Pine 40 mm, half thickness 20 mm', 'Kiefer 40 mm, halbe Dicke 20 mm', 'timber', 'pine-timber', 0.02, pine, 'authored-estimate')]),
]
RIDS = [r['id'] for r in regions]
ctx = Ctx(RIDS, {'mattress': (1.0, 0.3), 'slats': (0.0, 0.0), 'bed-frame': (1.0, 0.0)})
anchors = [anc(rig, 'foot-head', 'frame', 0.03, 0, 'contact', 'Head-end post feet', 'Füße der Kopfpfosten', support='contact', normalDeg=90, footprintLengthM=0.06, footprintWidthM=0.9, frictionCoefficient=0.5),
           anc(rig, 'foot-mid', 'frame', 1.06, 0, 'contact', 'Centre support leg', 'Mittelfuß', support='contact', normalDeg=90, footprintLengthM=0.06, footprintWidthM=0.06, frictionCoefficient=0.5),
           anc(rig, 'foot-foot', 'frame', 2.09, 0, 'contact', 'Foot-end post feet', 'Füße der Fußpfosten', support='contact', normalDeg=90, footprintLengthM=0.06, footprintWidthM=0.9, frictionCoefficient=0.5)]
CONT = ['foot-head', 'foot-mid', 'foot-foot']
def occ(split, name='occupant'):
    out = []
    for b_, share, ln in (('mat-head', split[0], 0.72), ('mat-mid', split[1], 0.60), ('mat-foot', split[2], 0.68)):
        out.append(load(f'{name}-{b_[4:]}', f'Occupant share on the mattress {b_[4:]} part ({share * 100:.0f} % of 75 kg)', f'Anteil der Person auf dem Matratzenteil {"Kopf" if b_[4:]=="head" else "Mitte" if b_[4:]=="mid" else "Fuß"} ({share * 100:.0f} % von 75 kg)', 'occupant', 0, dist={'bone': b_, 'fromM': -ln if b_ == 'mat-head' else 0, 'toM': 0 if b_ == 'mat-head' else ln, 'wxNPerM': 0, 'wyNPerM': -round(735.5 * share / ln, 1)}, ref='en-1991-1-1'))
    return out
LYING = {'mat-head': {'dy': -0.020}, 'mat-mid': {'dy': -0.028}, 'mat-foot': {'dy': -0.012}}
READ = {'slat-head': {'angleDeg': -50}, 'mat-head': {'dy': -0.025}, 'mat-mid': {'dy': -0.03}, 'mat-foot': {'dy': -0.010}}
P = [
    pose(ctx, 'flat', 'Flat, empty', 'Flach, leer', 'All sections flat; a sun patch from the window lies on the mattress.', 'Alle Teile flach; ein Fenster-Sonnenfleck liegt auf der Matratze.', contacts=CONT),
    pose(ctx, 'head-raised', 'Head section raised', 'Kopfteil hochgestellt', 'Head section raised by 50 degrees (empty bed).', 'Kopfteil um 50 Grad hochgestellt (leeres Bett).', bones={'slat-head': {'angleDeg': -50}}, contacts=CONT,
         over={'slats': (0.3, 0.0), 'mattress': (1.0, 0.2)}),
    pose(ctx, 'asleep', 'Person asleep', 'Person schläft', 'One adult lies flat (75 kg): the mattress sinks 28 mm at the hips; contact with the back and legs at 30 degC.', 'Eine erwachsene Person liegt flach (75 kg): die Matratze sinkt an der Hüfte um 28 mm; Kontakt mit Rücken und Beinen bei 30 degC.',
         bones=LYING, contacts=CONT, loads=occ((0.4, 0.35, 0.25)), over={'mattress': (1.0, 0.0)}, contact={'mattress': (0.35,)}),
    pose(ctx, 'reading-in-bed', 'Sitting up to read', 'Aufrecht zum Lesen', 'Head section at 50 degrees; the person sits up against the raised head section.', 'Kopfteil bei 50 Grad; die Person sitzt aufrecht am hochgestellten Kopfteil.',
         bones=READ, contacts=CONT, loads=occ((0.55, 0.30, 0.15)), over={'slats': (0.3, 0.0), 'mattress': (1.0, 0.0)}, contact={'mattress': (0.30,)}),
    pose(ctx, 'half-raised-key', 'Head half raised (key)', 'Kopfteil halb hoch (Schlüssel)', 'Head section at 25 degrees.', 'Kopfteil bei 25 Grad.', key=True, bones={'slat-head': {'angleDeg': -25}}, contacts=CONT, over={'slats': (0.1, 0.0)}),
]
clips = [
    clip('raise-head-section', 'Raise the head section', 'Kopfteil hochstellen', 'The head section rises to 50 degrees in 6 s, the mattress follows the slats.', 'Das Kopfteil hebt sich in 6 s auf 50 Grad, die Matratze folgt dem Lattenrost.', 6.0,
         [{'t': 0, 'pose': 'flat', 'ease': 'smooth'}, {'t': 3.0, 'pose': 'half-raised-key', 'ease': 'smooth'}, {'t': 6.0, 'pose': 'head-raised', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 6.0} for a in CONT], [('raise', 0, 6.0, 'Raising', 'Hochstellen')], [(6.0, 'motion', 'Sections reach their stops', 'Teile erreichen ihre Anschläge', None)], []),
    clip('get-into-bed', 'Get into bed', 'Ins Bett legen', 'An adult lies down on the flat bed; the mattress sinks and the contact surface warms.', 'Eine erwachsene Person legt sich auf das flache Bett; die Matratze sinkt ein und die Kontaktfläche erwärmt sich.', 4.0,
         [{'t': 0, 'pose': 'flat', 'ease': 'smooth'}, {'t': 2.0, 'pose': 'asleep', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'asleep', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 4.0} for a in CONT], [('lie', 0, 2.0, 'Lie down', 'Hinlegen')],
         [(1.5, 'load-on', 'Occupant lies on the mattress', 'Person liegt auf der Matratze', 'occupant-mid'), (1.5, 'thermal', 'Mattress top takes up 30 degC contact heat', 'Matratzenoberseite nimmt 30-degC-Kontaktwärme auf', None)],
         [(f'load:occupant-{p}', 'smooth', [(0, 0), (1.0, 0), (2.0, 1), (4.0, 1)]) for p in ('head', 'mid', 'foot')]),
    clip('sit-up-to-read', 'Sit up to read', 'Aufsetzen zum Lesen', 'The sleeper sits up: the head section is raised while the load shifts to the head part.', 'Die schlafende Person richtet sich auf: das Kopfteil wird hochgestellt, während die Last auf den Kopfteil wandert.', 6.0,
         [{'t': 0, 'pose': 'asleep', 'ease': 'smooth'}, {'t': 3.0, 'pose': 'half-raised-key', 'ease': 'smooth'}, {'t': 6.0, 'pose': 'reading-in-bed', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 6.0} for a in CONT], [('sit', 0, 6.0, 'Sitting up', 'Aufsetzen')], [(0.0, 'load-on', 'Occupant on the bed', 'Person auf dem Bett', 'occupant-mid')], []),
]
thermal = {'environment': {'airTempC': 21, 'meanRadiantTempC': 21, 'relativeHumidity': 0.5, 'windMS': 0.1, 'windDirectionDeg': 0, 'solarWM2': 350, 'diffuseFraction': 0.15, 'pressurePa': 101325},
           'operating': [], 'regions': regions, 'emitters': [], 'assumptions': [T(*a) for a in [
               ('The mattress cover is a 10 mm quilted layer (effective 150 kg/m3, 0.04 W/mK; authored-estimate) over 0.18 m cold foam (0.045 W/mK): the surface follows the contact body quickly (about 2.2 kJ/m2K) and insulates it from the foam; slats and frame are birch and pine after ISO 10456.', 'Der Matratzenbezug ist eine 10 mm Steppschicht (effektiv 150 kg/m3, 0,04 W/mK; Schätzwert) über 0,18 m Kaltschaum (0,045 W/mK): die Oberfläche folgt dem Kontaktkörper schnell (etwa 2,2 kJ/m2K) und isoliert ihn vom Schaum; Latten und Rahmen sind Birke und Kiefer nach ISO 10456.'),
               ('A lying person covers 35 % of the mattress surface (0.6 m2 of the 1.7 m2 top) with a clothed body at 30 degC (ISO 7730 clothing surface; a duvet is not modelled).', 'Eine liegende Person bedeckt 35 % der Matratzenoberfläche (0,6 m2 der 1,7 m2 Oberseite) mit einem bekleideten Körper bei 30 degC (Bekleidungsoberfläche nach ISO 7730; eine Bettdecke ist nicht modelliert).'),
               ('The slat region is declared hidden: it lies behind the side rail and under the mattress; raised poses bind more exposure to it.', 'Die Lattenregion ist als verdeckt erklärt: sie liegt hinter dem Seitenholm und unter der Matratze; angehobene Posen binden mehr Luftkontakt.'),
               ('Reference case: bedroom at 21 degC, still air, 350 W/m2 window sun patch behind glazing on the flat bed.', 'Referenzfall: Schlafzimmer bei 21 degC, ruhige Luft, 350 W/m2 Fenster-Sonnenfleck hinter Verglasung auf dem flachen Bett.')]]}
struct = structure(CONT, [
    ('The frame is a rigid body on three foot contacts (head posts, centre leg, foot posts); the slatted base is three sections, the head section on a revolute hinge at x = 0.78 m (0 to 65 degrees); the foot section is fixed because a raised foot section would pass within a few centimetres of the footboard; each mattress part rides on its section and sinks along the section normal.', 'Der Rahmen ist ein starrer Körper auf drei Fußkontakten (Kopfpfosten, Mittelfuß, Fußpfosten); der Lattenrost besteht aus drei Teilen, das Kopfteil an einem Drehgelenk bei x = 0,78 m (0 bis 65 Grad); das Fußteil ist fest, weil ein hochgestelltes Fußteil nur wenige Zentimeter am Fußbrett vorbeiliefe; jedes Matratzenteil sitzt auf seinem Teil und sinkt entlang der Teilnormalen ein.'),
    ('Mattress sink: hips 28 mm, shoulders 20 mm, heels 12 mm (authored-estimate for firm cold foam 0.18 m, ISO 3386-1 order of 6 kPa at 40 percent strain); the occupant load of 75 kg is split 40/35/25 percent lying and 55/30/15 percent sitting up.', 'Matratzeneinsinken: Hüfte 28 mm, Schultern 20 mm, Fersen 12 mm (Schätzwert für festen Kaltschaum 0,18 m, Größenordnung 6 kPa bei 40 Prozent Stauchung nach ISO 3386-1); die Personenlast von 75 kg verteilt sich liegend 40/35/25 Prozent und aufrecht 55/30/15 Prozent.'),
    ('Static load tests of bed bases follow EN 1725 (scope only; no test magnitude is modelled).', 'Statische Lastprüfungen von Bettunterbauten folgen EN 1725 (nur Rahmen; keine Prüflast modelliert).')])
# geometry
def frame_common(S_, pw, fw, pen_extra=None, hatch=None, knobs=False):
    kw = {'penMm': pen_extra} if pen_extra else {}
    S_.rect('head-post', 'frame', 0.0, 0.0, pw, 0.95, region='bed-frame', material='pine-timber', tone=0.55, z=6, hatch=hatch, **kw) if False else None
S5 = Shapes(rig)
S5.rect('head', 'frame', 0.0, 0.0, 0.10, 0.95, region='bed-frame', material='pine-timber', tone=0.5, z=6)
S5.rect('foot', 'frame', 2.02, 0.0, 2.12, 0.55, region='bed-frame', material='pine-timber', tone=0.5, z=6)
S5.rect('rail', 'frame', 0.06, 0.14, 2.06, 0.30, region='bed-frame', material='pine-timber', tone=0.55, z=5)
def mats(S_, rt, extra=True, rg=None):
    for nm, b_, x0, x1 in (('mat-head', 'mat-head', 0.055, 0.80), ('mat-mid', 'mat-mid', 0.78, 1.40), ('mat-foot', 'mat-foot', 1.38, 2.065)):
        S_.path(nm, b_, rbox(x0, SL_TOP - 0.01, x1, MAT_TOP, rt=rt), regions=rg or ['mattress'], material='quilted-cover', tone=0.85, z=3 if nm != 'mat-mid' else 3.1)
mats(S5, 0.03, rg=None)
for sh in S5.items:
    if sh['id'].startswith('mat-'):
        sh.pop('regions', None); sh['regions'] = ['mattress', 'slats']
S2 = Shapes(rig)
S2.rect('head', 'frame', 0.0, 0.0, 0.06, 0.95, region='bed-frame', material='pine-timber', tone=0.5, z=6)
S2.rect('foot', 'frame', 2.06, 0.0, 2.12, 0.55, region='bed-frame', material='pine-timber', tone=0.5, z=6)
S2.rect('rail', 'frame', 0.04, 0.14, 2.08, 0.30, region='bed-frame', material='pine-timber', tone=0.55, z=5)
S2.rect('leg-mid', 'frame', 1.03, 0.0, 1.09, 0.16, region='bed-frame', material='pine-timber', tone=0.5, z=5)
mats(S2, 0.03)
for sh in S2.items:
    if sh['id'].startswith('mat-'):
        sh['regions'] = ['mattress', 'slats']
def slats(S_, th, pen=None):
    kw = {'penMm': pen} if pen else {}
    S_.rect('slats-head', 'slat-head', 0.06, SL_TOP - th, HH, SL_TOP, region='slats', material='birch-slat', tone=0.6, z=2, **kw)
    S_.rect('slats-mid', 'slat-mid', HH, SL_TOP - th, HF, SL_TOP, region='slats', material='birch-slat', tone=0.6, z=2, **kw)
    S_.rect('slats-foot', 'slat-foot', HF, SL_TOP - th, 2.06, SL_TOP, region='slats', material='birch-slat', tone=0.6, z=2, **kw)
def frame(S_, ph, hatch=False, bolts=False):
    S_.path('head', 'frame', rbox(0.0, 0.0, 0.06, 0.95, rt=0.02), region='bed-frame', material='pine-timber', tone=0.5, z=6, hatch='grain-v' if hatch else None)
    S_.path('foot', 'frame', rbox(2.06, 0.0, 2.12, 0.55, rt=0.02), region='bed-frame', material='pine-timber', tone=0.5, z=6, hatch='grain-v' if hatch else None)
    S_.rect('rail', 'frame', 0.04, 0.14, 2.08, 0.30, region='bed-frame', material='pine-timber', tone=0.55, z=5, hatch='grain-h' if hatch else None)
    S_.rect('leg-mid', 'frame', 1.03, 0.0, 1.09, 0.16, region='bed-frame', material='pine-timber', tone=0.5, z=5, hatch='grain-v' if hatch else None)
S1 = Shapes(rig); frame(S1, 0.95); slats(S1, 0.022); mats(S1, 0.03)
for i, (nm, b_, x0, x1) in enumerate((('mat-head', 'mat-head', 0.055, 0.80), ('mat-mid', 'mat-mid', 0.78, 1.40), ('mat-foot', 'mat-foot', 1.38, 2.065))):
    S1.line(f'piping-{nm}', b_, [(max(x0, 0.055) if b_ == 'mat-head' else x0 + 0.065, MAT_TOP - 0.05), ((x1 - 0.30 if b_ == 'mat-head' else x1 - 0.08) if b_ != 'mat-foot' else x1, MAT_TOP - 0.05)], role='detail', z=3.5, region='mattress')
S0 = Shapes(rig); frame(S0, 0.95, hatch=True); slats(S0, 0.022); mats(S0, 0.04)
for sh in S0.items:
    if sh['id'] in ('head', 'foot'):
        sh['d'] = sh['d'].replace('0.02', '0.03') if False else sh['d']
for nm, b_, x0, x1 in (('mat-head', 'mat-head', 0.055, 0.80), ('mat-mid', 'mat-mid', 0.78, 1.40), ('mat-foot', 'mat-foot', 1.38, 2.065)):
    S0.line(f'piping-top-{nm}', b_, [(x0 if b_ == 'mat-head' else x0 + 0.065, MAT_TOP - 0.035), ((x1 - 0.30 if b_ == 'mat-head' else x1 - 0.08) if b_ != 'mat-foot' else x1, MAT_TOP - 0.035)], role='detail', z=3.5, region='mattress')
S0.ell('bolt-head', 'frame', 0.03, 0.22, 0.0095, region='bed-frame', material='pine-timber', tone=0.3, z=7, penMm=0.13) if False else None
HATCH = [{'id': 'grain-h', 'title': T('Grain horizontal', 'Maserung waagerecht'), 'pattern': 'lines', 'angleDeg': 0, 'spacingMm': 0.6, 'penMm': 0.13}, {'id': 'grain-v', 'title': T('Grain vertical', 'Maserung senkrecht'), 'pattern': 'lines', 'angleDeg': 90, 'spacingMm': 0.6, 'penMm': 0.13}]
def dd(*a): return D(*a)
reps = {
 '500': rep(500, S5, dd(('Bed sign 4.2 x 1.9 mm: head and foot blocks, a rail and three mattress parts, the head part movable.', 'Bettzeichen 4,2 x 1,9 mm: Kopf- und Fußblock, ein Holm und drei Matratzenteile, das Kopfteil beweglich.'),
   ('Headboard 0.95 m and footboard 0.55 m, side rail, mattress top at 0.485 m.', 'Kopfbrett 0,95 m und Fußbrett 0,55 m, Seitenholm, Matratzenoberkante 0,485 m.'),
   ('Slats lie under the mattress parts and are merged into them; boards and posts are one block each.', 'Latten liegen unter den Matratzenteilen und sind in sie eingeflossen; Bretter und Pfosten sind je ein Block.'),
   ('Centre leg, slat strips, piping.', 'Mittelfuß, Lattenstreifen, Paspeln.'),
   ('Board posts (0.06 m) are drawn 0.10 m wide (0.2 mm); the 0.04 m overshoot runs inside the mattress end.', 'Brettpfosten (0,06 m) sind 0,10 m breit gezeichnet (0,2 mm); die Überhöhung von 0,04 m liegt im Matratzenende.'),
   ('None.', 'Keine.'), ('Frame 0.5-0.55, mattress 0.85.', 'Rahmen 0,5-0,55, Matratze 0,85.'), ('Mattress parts list mattress and slats as merged; blocks and rail carry bed-frame.', 'Matratzenteile führen mattress und slats zusammen; Blöcke und Holm tragen bed-frame.'))),
 '200': rep(200, S2, dd(('Bed 10.6 x 4.8 mm: posts, rail, centre leg and three rounded mattress parts.', 'Bett 10,6 x 4,8 mm: Pfosten, Holm, Mittelfuß und drei gerundete Matratzenteile.'),
   ('Posts at real width 0.06 m, the centre leg, the rounded mattress parts (head part follows the raised section).', 'Pfosten in Realbreite 0,06 m, der Mittelfuß, die gerundeten Matratzenteile (das Kopfteil folgt dem hochgestellten Teil).'),
   ('Slats are merged into the mattress parts.', 'Latten sind in die Matratzenteile eingeflossen.'), ('Slat strips (0.02 m = 0.1 mm), piping.', 'Lattenstreifen (0,02 m = 0,1 mm), Paspeln.'),
   ('None.', 'Keine.'), ('None.', 'Keine.'), ('As 1:500.', 'Wie 1:500.'), ('As 1:500.', 'Wie 1:500.'))),
 '100': rep(100, S1, dd(('Bed 21 x 9.5 mm: rounded posts, rail, centre leg, three slat strips, three mattress parts with piping.', 'Bett 21 x 9,5 mm: gerundete Pfosten, Holm, Mittelfuß, drei Lattenstreifen, drei Matratzenteile mit Paspel.'),
   ('The slat sections as strips (they show when a section is raised), mattress piping.', 'Die Lattenteile als Streifen (sie zeigen sich, wenn ein Teil hochgestellt ist), Matratzenpaspel.'), ('Single slats are one strip per section.', 'Einzelne Latten sind ein Streifen je Teil.'),
   ('Slat gaps, bolts.', 'Lattenfugen, Schrauben.'), ('Strips 0.022 m thick (slat 0.012 m).', 'Streifen 0,022 m dick (Latte 0,012 m).'), ('None.', 'Keine.'), ('Mattress 0.85, frame 0.5.', 'Matratze 0,85, Rahmen 0,5.'), ('One shape per part.', 'Eine Form je Teil.'))),
 '50': rep(50, S0, dd(('Bed 42 x 19 mm with grain on rail and posts, slat strips and mattress with top piping.', 'Bett 42 x 19 mm mit Maserung an Holm und Pfosten, Lattenstreifen und Matratze mit Paspel oben.'),
   ('As 1:100 plus grain hatching.', 'Wie 1:100 plus Maserungsschraffur.'), ('Single slats are one strip per section.', 'Einzelne Latten sind ein Streifen je Teil.'), ('Slat gaps, bolts, mattress handles.', 'Lattenfugen, Schrauben, Matratzengriffe.'),
   ('Strips 0.022 m thick (slat 0.012 m).', 'Streifen 0,022 m dick (Latte 0,012 m).'), ('Grain lines at 0.6 mm, pen 0.13.', 'Maserungslinien im Abstand 0,6 mm, Stift 0,13.'), ('As 1:100.', 'Wie 1:100.'), ('One shape per part.', 'Eine Form je Teil.')), hatches=HATCH),
}
for k in reps:
    reps[k]['validRange'] = {'minDenominator': RANGES[k][0], 'maxDenominator': RANGES[k][1]}
SYM = {'id': 'bed-plan', 'title': T('Single bed plan symbol', 'Einzelbett-Grundrisssymbol'), 'description': T('Conventional plan sign of a single bed 0.90 x 2.00 m: mattress outline, head board at the left, pillow rectangle and the turned-down cover edge.', 'Übliches Grundrisszeichen eines Einzelbetts 0,90 x 2,00 m: Matratzenkontur, Kopfbrett links, Kissenrechteck und umgeschlagene Deckenkante.'),
       'semantics': 'true-extent', 'units': 'model-m', 'denominators': [200, 100, 50], 'pens': {'body': 0.25, 'detail': 0.13, 'texture': None, 'glazing': None, 'ground': 0.35}, 'hatches': [],
       'geometry': {'elevation': [], 'plan': [
           {'id': 'plan-frame', 'bone': 'ground', 'role': 'body', 'z': 1, 'filled': True, 'tone': 0.9, 'kind': 'poly', 'points': [[0, 0], [2.12, 0], [2.12, 0.9], [0, 0.9]], 'closed': True},
           {'id': 'plan-mattress', 'bone': 'ground', 'role': 'detail', 'z': 2, 'filled': False, 'tone': 0.5, 'kind': 'poly', 'points': [[0.08, 0.04], [2.03, 0.04], [2.03, 0.86], [0.08, 0.86]], 'closed': True},
           {'id': 'plan-pillow', 'bone': 'ground', 'role': 'detail', 'z': 3, 'filled': False, 'tone': 0.5, 'kind': 'poly', 'points': [[0.12, 0.12], [0.52, 0.12], [0.52, 0.78], [0.12, 0.78]], 'closed': True},
           {'id': 'plan-turn', 'bone': 'ground', 'role': 'detail', 'z': 3, 'filled': False, 'tone': 0.5, 'kind': 'poly', 'points': [[0.70, 0.04], [0.70, 0.86]], 'closed': False}]},
       'dimensionalNote': T('Model metres at real size: x along the bed 0 to 2.12 m (head at 0), y across 0 to 0.90 m. Pillow and cover edge are conventional marks with their real size.', 'Modellmeter in Originalgröße: x entlang des Betts 0 bis 2,12 m (Kopf bei 0), y quer 0 bis 0,90 m. Kissen und Deckenkante sind übliche Zeichen in Realgröße.')}
A = assemble('single-bed', T('Single bed with slatted base', 'Einzelbett mit Lattenrost'),
             T('A 0.90 x 2.00 m pine single bed with an adjustable slatted base in three sections and a 0.20 m foam mattress that sinks under the sleeper; side elevation and the conventional plan sign.', 'Ein Einzelbett aus Kiefer, 0,90 x 2,00 m, mit verstellbarem Lattenrost in drei Teilen und einer 0,20 m dicken Schaumstoffmatratze, die unter dem Schläfer einsinkt; Seitenansicht und das übliche Grundrisszeichen.'),
             ['bed', 'single-bed', 'sleeping', 'slatted-base', 'mattress', 'interior'],
             T('Side elevation 2.12 x 0.95 m (headboard 0.95 m, footboard 0.55 m, mattress top 0.485 m), width across 0.90 m. Masses from nominal volumes; the mattress is 22 kg of cold foam. Sleeper and sheets are not drawn.', 'Seitenansicht 2,12 x 0,95 m (Kopfbrett 0,95 m, Fußbrett 0,55 m, Matratzenoberkante 0,485 m), Breite quer 0,90 m. Massen aus Nennvolumen; die Matratze ist 22 kg Kaltschaum. Schläfer und Bettwäsche sind nicht gezeichnet.'),
             {'heightM': 0.95, 'widthM': 2.12, 'depthM': 0.9, 'massKg': round(mass, 3)}, MATS, RIG, anchors, reps, [SYM], P, clips, thermal, struct,
             {'defaultPose': 'flat', 'defaultClip': 'raise-head-section', 'defaultScale': 50, 'viewpoints': ['elevation']},
             ['iso-10456-2007', 'en-1991-1-1', 'en-1725', 'en-1957', 'iso-3386-1', 'iso-7730-2005', 'wood-handbook-2021', 'incropera-2018', 'semio-nwf'])
if __name__ == '__main__':
    main(A, 'single-bed', ['flat', 'head-raised', 'asleep', 'reading-in-bed'])
