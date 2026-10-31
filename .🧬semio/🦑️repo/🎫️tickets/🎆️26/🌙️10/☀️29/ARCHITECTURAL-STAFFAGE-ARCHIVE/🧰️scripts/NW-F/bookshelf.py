import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nwf import *

YS = [0.36, 0.66, 0.96, 1.26, 1.56]
TH, XI0, XI1, HALF = 0.018, 0.018, 0.782, 0.382
PITCH = 0.032
chip = mat('melamine-chipboard', 'Melamine-faced chipboard P5, 18 mm', 'Melaminbeschichtete Spanplatte P5, 18 mm', 0.9, 0.35, 650, 0.14, 1700, 2.55e9, 1.5e7, 'literature', 'en-312')
hard = mat('hardboard-back', 'Hardboard back panel, 3 mm', 'Hartfaser-Rückwand, 3 mm', 0.9, 0.3, 900, 0.18, 1700, 3.0e9, 3.0e7, 'authored-estimate', 'iso-10456-2007')
MATS = [chip, hard]
E_ = chip['elasticModulusPa']; B = 0.28
I_ = B * TH ** 3 / 12
SEC = {'areaM2': round(B * TH, 6), 'secondMomentM4': float(f'{I_:.4e}'), 'extremeFibreM': TH / 2, 'material': 'melamine-chipboard', 'title': T('Shelf 280 x 18 mm', 'Fachboden 280 x 18 mm')}
BOOK_RHO, FILL_H = 612.0, 0.27
w_full = BOOK_RHO * 9.80665 * (XI1 - XI0) * 0 + BOOK_RHO * 9.80665 * B * FILL_H * 1.0
w_full = round(w_full * 0.28 / 0.28, 1)
W_FULL = round(BOOK_RHO * 9.80665 * B * FILL_H, 0)
def sag(w):
    L = XI1 - XI0
    return 5 * w * L ** 4 / (384 * E_ * I_)
DELTA = sag(W_FULL)
THETA = math.degrees(math.atan(DELTA / HALF))
bones = [bone('ground', None, 'Floor reference', 'Bodenbezug', 0, 0, 0, 0.8, FIXED),
         bone('frame', 'ground', 'Carcass, plinth and back panel', 'Korpus, Sockel und Rückwand', 0, 0, 0, 1.8, FIXED, (22.4, 0.4, 0.9, 'uniform', 'authored-estimate', 'en-312'))]
for i, y in enumerate(YS, 1):
    bones.append(bone(f'shelf-{i}', 'frame', f'Adjustable shelf {i} (pin height)', f'Verstellbarer Fachboden {i} (Stiftlage)', XI0, y, 0, XI1 - XI0, prism(90, -0.2, 0.2)))
    bones.append(bone(f'shelf-{i}-l', f'shelf-{i}', f'Shelf {i} left half', f'Fachboden {i} linke Hälfte', 0, 0, 0, HALF, rev(-3, 3), (1.25, HALF / 2, TH / 2, 'point', 'authored-estimate', 'en-312'), SEC))
    bones.append(bone(f'shelf-{i}-r', f'shelf-{i}', f'Shelf {i} right half', f'Fachboden {i} rechte Hälfte', XI1 - XI0, 0, 180, HALF, rev(-3, 3), (1.25, HALF / 2, -TH / 2, 'point', 'authored-estimate', 'en-312'), SEC))
rig = Rig(bones)
RIG = {'kind': 'mechanical', 'root': 'ground', 'bones': bones}
mass = sum(b['mass']['kg'] for b in bones if 'mass' in b)
def cl(i, en, de, t, m, M, kind='polymer'):
    return lay(i, en, de, 'timber', m, t, M, 'literature' if m == 'melamine-chipboard' else 'authored-estimate', 'en-312' if m == 'melamine-chipboard' else 'iso-10456-2007')
regions = [
    reg('carcass', 'Side panels, top and bottom panel', 'Seitenwände, Deckel und Boden', 'surface', 'melamine-chipboard', 1.9, 0.3, 'vertical-plate', 0.5, [cl('carcass-board', 'Chipboard 18 mm', 'Spanplatte 18 mm', 0.018, 'melamine-chipboard', chip)]),
    reg('shelves', 'Adjustable shelves', 'Verstellbare Fachböden', 'surface', 'melamine-chipboard', 1.07, 0.28, 'horizontal-plate-up', 0.35, [cl('shelf-board', 'Chipboard 18 mm', 'Spanplatte 18 mm', 0.018, 'melamine-chipboard', chip)]),
    reg('back-panel', 'Back panel', 'Rückwand', 'surface', 'hardboard-back', 1.4, 0.3, 'vertical-plate', 0.4, [cl('back-board', 'Hardboard 3 mm', 'Hartfaserplatte 3 mm', 0.003, 'hardboard-back', hard)]),
    reg('plinth', 'Plinth', 'Sockel', 'surface', 'melamine-chipboard', 0.12, 0.06, 'vertical-plate', 0.1, [cl('plinth-board', 'Chipboard 18 mm', 'Spanplatte 18 mm', 0.018, 'melamine-chipboard', chip)]),
]
RIDS = [r['id'] for r in regions]
ctx = Ctx(RIDS, {'carcass': (1.0, 0.15), 'shelves': (1.0, 0.0), 'back-panel': (1.0, 0.0), 'plinth': (1.0, 0.0)})
anchors = [anc(rig, 'plinth-left', 'frame', 0.06, 0, 'contact', 'Left plinth foot', 'Linker Sockelfuß', support='contact', normalDeg=90, footprintLengthM=0.1, footprintWidthM=0.28, frictionCoefficient=0.5),
           anc(rig, 'plinth-right', 'frame', 0.74, 0, 'contact', 'Right plinth foot', 'Rechter Sockelfuß', support='contact', normalDeg=90, footprintLengthM=0.1, footprintWidthM=0.28, frictionCoefficient=0.5)]
CONT = ['plinth-left', 'plinth-right']
def sagbones(ids, th=THETA, raised=None):
    d = {}
    for i in ids:
        d[f'shelf-{i}-l'] = {'angleDeg': -round(th, 4)}
        d[f'shelf-{i}-r'] = {'angleDeg': round(th, 4)}
    for i, v in (raised or {}).items():
        d[f'shelf-{i}'] = {'dy': v}
    return d
def books(ids):
    out = []
    for i in ids:
        for s in 'lr':
            out.append(load(f'books-{i}-{s}', f'Books on shelf {i}, {"left" if s == "l" else "right"} half ({W_FULL:.0f} N/m)', f'Bücher auf Fachboden {i}, {"linke" if s == "l" else "rechte"} Hälfte ({W_FULL:.0f} N/m)', 'carried', 0, dist={'bone': f'shelf-{i}-{s}', 'fromM': 0, 'toM': HALF, 'wxNPerM': 0, 'wyNPerM': -W_FULL}, ref='en-1991-1-1'))
    out.append(load('books-base', f'Books on the bottom panel ({W_FULL:.0f} N/m)', f'Bücher auf dem Boden ({W_FULL:.0f} N/m)', 'carried', 0, dist={'bone': 'frame', 'fromM': XI0, 'toM': XI1, 'wxNPerM': 0, 'wyNPerM': -W_FULL}, ref='en-1991-1-1'))
    return out
ALL = [1, 2, 3, 4, 5]
RAISE = {3: 4 * PITCH}
cov = {'shelves': {'exposedFraction': 0.1, 'sunlitFraction': 0.0, 'contact': {'fraction': 0.85, 'tempC': 21}}}
P = [
    pose(ctx, 'empty', 'Empty', 'Leer', 'Unloaded shelves: all five boards straight.', 'Unbelastete Regalböden: alle fünf Böden gerade.', contacts=CONT),
    pose(ctx, 'loaded', 'Fully loaded with books', 'Voll mit Büchern beladen', f'Every compartment filled with books ({W_FULL:.0f} N/m = 35 kg per shelf): each board sags {DELTA * 1000:.1f} mm at mid-span.', f'Jedes Fach mit Büchern gefüllt ({W_FULL:.0f} N/m = 35 kg je Boden): jeder Boden hängt in Feldmitte {DELTA * 1000:.1f} mm durch.',
         bones=sagbones(ALL), contacts=CONT, loads=books(ALL), over={'shelves': cov['shelves']}),
    pose(ctx, 'shelf-adjusted', 'Third shelf moved up four holes', 'Dritter Boden vier Löcher höher', 'Shelf 3 re-pinned 4 x 32 mm = 128 mm higher (System 32); loaded and sagging.', 'Boden 3 um 4 x 32 mm = 128 mm höher gesteckt (System 32); beladen und durchgebogen.',
         bones=sagbones(ALL, raised={3: 4 * PITCH}), contacts=CONT, loads=books(ALL), over={'shelves': cov['shelves']}),
    pose(ctx, 'top-two-loaded-key', 'Lower two shelves loaded (key)', 'Untere zwei Böden beladen (Schlüssel)', 'Key pose of the loading clip.', 'Schlüsselpose des Beladens.', key=True, bones=sagbones([1, 2]), contacts=CONT, loads=books([1, 2]), over={'shelves': {'exposedFraction': 0.55, 'sunlitFraction': 0.0, 'contact': {'fraction': 0.4, 'tempC': 21}}}),
]
def key_with(ids):
    return {'t': 0, 'pose': 'empty', 'ease': 'smooth', 'bones': sagbones(ids)}
keys = [{'t': 0, 'pose': 'empty', 'ease': 'smooth'}]
for n, t in zip([1, 2, 3, 4, 5], [1.0, 2.0, 3.0, 4.0, 5.0]):
    keys.append({'t': t, 'pose': 'empty', 'ease': 'smooth', 'bones': sagbones(ALL[:n])})
tracks = [(f'load:books-{i}-{s}', 'smooth', [(0, 0), (i - 1 + 0.001 if i > 1 else 0, 0), (i, 1), (5.0, 1)]) for i in ALL for s in 'lr']
tracks = [(t, e, [(a, v) for a, v in ks if True]) for t, e, ks in tracks]
tracks = [(f'load:books-{i}-{s}', 'smooth', [(0, 0), (float(i - 1), 0), (float(i), 1), (5.0, 1)]) for i in ALL for s in 'lr']
tracks[0] = ('load:books-1-l', 'smooth', [(0, 0), (1.0, 1), (5.0, 1)])
tracks[1] = ('load:books-1-r', 'smooth', [(0, 0), (1.0, 1), (5.0, 1)])
clips = [
    clip('load-the-shelves', 'Load the shelves', 'Regal beladen', f'The shelves are filled from the bottom up; every board sags by up to {DELTA * 1000:.1f} mm as its books arrive.', f'Die Böden werden von unten nach oben gefüllt; jeder Boden hängt mit den Büchern bis zu {DELTA * 1000:.1f} mm durch.', 5.0, keys,
         [{'anchor': a, 'fromS': 0, 'toS': 5.0} for a in CONT], [('fill', 0, 5.0, 'Filling', 'Befüllen')],
         [(float(i), 'load-on', f'Shelf {i} filled', f'Boden {i} gefüllt', f'books-{i}-l') for i in ALL], tracks),
    clip('re-pin-shelf', 'Re-pin the third shelf', 'Dritten Boden umstecken', 'The empty shelf 3 is lifted off its pins and re-pinned four holes (128 mm) higher.', 'Der leere Boden 3 wird von den Stiften genommen und vier Löcher (128 mm) höher gesteckt.', 4.0,
         [{'t': 0, 'pose': 'empty', 'ease': 'smooth'}, {'t': 1.0, 'pose': 'empty', 'ease': 'smooth', 'bones': {'shelf-3': {'dy': 0.04}}}, {'t': 2.5, 'pose': 'empty', 'ease': 'smooth', 'bones': {'shelf-3': {'dy': 0.128}}}, {'t': 4.0, 'pose': 'empty', 'ease': 'smooth', 'bones': {'shelf-3': {'dy': 0.128}}}],
         [{'anchor': a, 'fromS': 0, 'toS': 4.0} for a in CONT], [('lift', 0, 2.5, 'Lift and move', 'Anheben und umsetzen'), ('pin', 2.5, 4.0, 'Pinned', 'Eingesteckt')],
         [(2.5, 'motion', 'Shelf rests on the new pins', 'Boden liegt auf den neuen Stiften', None)], []),
    clip('unload-shelves', 'Unload the shelves', 'Regal leeren', 'All books are taken out at once; the boards spring back from the sag.', 'Alle Bücher werden auf einmal entnommen; die Böden federn aus der Durchbiegung zurück.', 3.0,
         [{'t': 0, 'pose': 'loaded', 'ease': 'smooth'}, {'t': 1.0, 'pose': 'empty', 'ease': 'smooth'}, {'t': 3.0, 'pose': 'empty', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 3.0} for a in CONT], [('spring', 0, 1.0, 'Springing back', 'Zurückfedern')],
         [(0.5, 'load-off', 'Books taken out', 'Bücher entnommen', 'books-1-l')], [(f'load:books-{i}-{s}', 'smooth', [(0, 1), (0.5, 0), (3.0, 0)]) for i in ALL for s in 'lr']),
]
thermal = {'environment': {'airTempC': 21, 'meanRadiantTempC': 21, 'relativeHumidity': 0.5, 'windMS': 0.1, 'windDirectionDeg': 0, 'solarWM2': 350, 'diffuseFraction': 0.15, 'pressurePa': 101325},
           'operating': [], 'regions': regions, 'emitters': [], 'assumptions': [T(*a) for a in [
               ('Passive melamine-faced chipboard (650 kg/m3, 0.14 W/mK, 1700 J/kgK after ISO 10456 / EN 312 data) and 3 mm hardboard; heat capacity from the penetration depth of one hour (18 mm board: 20 kJ/m2K).', 'Passive melaminbeschichtete Spanplatte (650 kg/m3, 0,14 W/mK, 1700 J/kgK nach ISO 10456 / EN 312) und 3 mm Hartfaser; Wärmekapazität aus der Eindringtiefe einer Stunde (18-mm-Platte: 20 kJ/m2K).'),
               ('Loaded poses cover 85 % of the shelf tops with books at room temperature (conductive contact at 21 degC, exposed fraction 0.1); the books themselves are not a region.', 'Beladene Posen bedecken 85 % der Fachbodenoberseiten mit Büchern bei Raumtemperatur (Kontakt bei 21 degC, freie Fläche 0,1); die Bücher selbst sind keine Region.'),
               ('Reference case: 350 W/m2 window sun patch behind glazing falling on the left carcass side (15 percent), 21 degC, still air.', 'Referenzfall: 350 W/m2 Fensterlichtfleck hinter Verglasung auf der linken Korpusseite (15 Prozent), 21 degC, ruhige Luft.')]]}
struct = structure(CONT, [
    (f'Each adjustable shelf is a simply supported beam on shelf pins (two half-span bones hinged at the supports and at mid-span); loaded sag delta = 5 w L^4 / (384 E I) with w = {W_FULL:.0f} N/m (books 6.0 kN/m3 x 0.28 m x 0.27 m, EN 1991-1-1 Annex A), L = 0.764 m, E = 2.55 GPa (EN 312 P5), I = {I_:.3e} m4 gives {DELTA * 1000:.1f} mm = L/{(XI1 - XI0) / DELTA:.0f}; bending stress {(W_FULL * (XI1 - XI0) ** 2 / 8) * TH / 2 / I_ / 1e6:.1f} MPa of 15 MPa.', f'Jeder verstellbare Fachboden ist ein Einfeldträger auf Bodenträgern (zwei Halbfeld-Knochen, an den Auflagern und in Feldmitte gelenkig); Durchbiegung delta = 5 w L^4 / (384 E I) mit w = {W_FULL:.0f} N/m (Bücher 6,0 kN/m3 x 0,28 m x 0,27 m, EN 1991-1-1 Anhang A), L = 0,764 m, E = 2,55 GPa (EN 312 P5), I = {I_:.3e} m4 ergibt {DELTA * 1000:.1f} mm = L/{(XI1 - XI0) / DELTA:.0f}; Biegespannung {(W_FULL * (XI1 - XI0) ** 2 / 8) * TH / 2 / I_ / 1e6:.1f} MPa von 15 MPa.'),
    ('The book density 612 kg/m3 (6.0 kN/m3) is rounded from EN 1991-1-1 Annex A and an authored-estimate; the creep of chipboard over years (factor about 2) is not modelled.', 'Die Buchdichte 612 kg/m3 (6,0 kN/m3) ist aus EN 1991-1-1 Anhang A gerundet und ein Schätzwert; das Kriechen der Spanplatte über Jahre (Faktor etwa 2) ist nicht modelliert.'),
    ('The carcass is a rigid body on the plinth feet; EN 14749 shelf tests (uniformly distributed shelf loads) are the scope of the load case.', 'Der Korpus ist ein starrer Körper auf den Sockelfüßen; die Fachbodenprüfungen nach EN 14749 (gleichmäßig verteilte Last) sind der Rahmen des Lastfalls.')])
# geometry
def sides(S_, t, y0=0.0, bone_='frame', sid=''):
    S_.rect('side-l', bone_, 0, y0, t, 1.8, region='carcass', material='melamine-chipboard', tone=0.75, z=6)
    S_.rect('side-r', bone_, 0.8 - t, y0, 0.8, 1.8, region='carcass', material='melamine-chipboard', tone=0.75, z=6)
def half(S_, sid, i, s, t, pen=None, y_off=0.0, ext=0.005):
    b = f'shelf-{i}-{s}'
    y = YS[i - 1]
    if s == 'l':
        x0, x1 = XI0 - 0.004, XI0 + HALF + ext
    else:
        x0, x1 = XI1 - HALF - ext, XI1 + 0.004
    kw = {'penMm': pen} if pen else {}
    return S_.rect(sid, b, x0, y, x1, y + t, region='shelves', material='melamine-chipboard', tone=0.7, z=5, **kw)
# 1:500
s5 = Shapes(rig)
s5.rect('body', 'frame', 0, 0.06, 0.8, 1.8, regions=['carcass', 'back-panel'], material='melamine-chipboard', tone=0.8, z=3)
s5.rect('plinth', 'frame', 0.04, 0.0, 0.76, 0.08, region='plinth', material='melamine-chipboard', tone=0.4, z=2)
for i in ALL:
    for s in 'lr':
        y = YS[i - 1] + TH / 2
        xs = (XI0 - 0.004, XI0 + HALF + 0.005) if s == 'l' else (XI1 - HALF - 0.005, XI1 + 0.004)
        s5.line(f'shelf-{i}-{s}', f'shelf-{i}-{s}', [(xs[0], y), (xs[1], y)], role='body', penMm=0.13, z=4, region='shelves')
# 1:200
s2 = Shapes(rig)
s2.rect('body', 'frame', 0, 0.06, 0.8, 1.8, regions=['carcass', 'back-panel'], material='melamine-chipboard', tone=0.85, z=3)
s2.rect('plinth', 'frame', 0.03, 0.0, 0.77, 0.08, region='plinth', material='melamine-chipboard', tone=0.4, z=2)
for i in ALL:
    for s in 'lr':
        y = YS[i - 1] + TH / 2
        xs = (XI0 - 0.004, XI0 + HALF + 0.005) if s == 'l' else (XI1 - HALF - 0.005, XI1 + 0.004)
        s2.line(f'shelf-{i}-{s}', f'shelf-{i}-{s}', [(xs[0], y), (xs[1], y)], role='detail', z=4, region='shelves')
# 1:100 and 1:50
def full(S_, t, pen, pin=False):
    S_.rect('back', 'frame', t, 0.07, 0.8 - t, 1.8 - t, region='back-panel', material='hardboard-back', tone=0.9, z=1)
    S_.rect('side-l', 'frame', 0, 0.06, t, 1.8, region='carcass', material='melamine-chipboard', tone=0.7, z=6, penMm=pen)
    S_.rect('side-r', 'frame', 0.8 - t, 0.06, 0.8, 1.8, region='carcass', material='melamine-chipboard', tone=0.7, z=6, penMm=pen)
    S_.rect('top', 'frame', 0, 1.8 - t, 0.8, 1.8, region='carcass', material='melamine-chipboard', tone=0.7, z=6, penMm=pen)
    S_.rect('bottom', 'frame', 0, 0.06, 0.8, 0.06 + t, region='carcass', material='melamine-chipboard', tone=0.7, z=6, penMm=pen)
    S_.rect('plinth', 'frame', t - 0.004, 0.0, 0.8 - t + 0.004, 0.075, region='plinth', material='melamine-chipboard', tone=0.4, z=2)
    for i in ALL:
        for s in 'lr':
            half(S_, f'shelf-{i}-{s}', i, s, t, pen=pen)
        if pin:
            y = YS[i - 1]
            S_.rect(f'pin-{i}-l', f'shelf-{i}', XI0, y - 0.012, XI0 + 0.02, y + 0.001, region='shelves', material='steel-slides' if False else 'melamine-chipboard', tone=0.3, z=5.5, penMm=0.13)
            S_.rect(f'pin-{i}-r', f'shelf-{i}', XI1 - 0.02, y - 0.012, XI1, y + 0.001, region='shelves', material='melamine-chipboard', tone=0.3, z=5.5, penMm=0.13)
s1 = Shapes(rig); full(s1, 0.022, 0.25)
s0 = Shapes(rig); full(s0, 0.018, 0.25, pin=True)
def dd(*a):
    return D(*a)
reps = {
 '500': rep(500, s5, dd(('Shelf sign 1.6 x 3.6 mm: one carcass block, a plinth and five shelf strokes.', 'Regalzeichen 1,6 x 3,6 mm: ein Korpusblock, ein Sockel und fünf Fachbodenstriche.'),
   ('Overall height 1.80 m, the five-shelf rhythm, plinth.', 'Gesamthöhe 1,80 m, der Rhythmus der fünf Böden, Sockel.'),
   ('Sides, top, bottom and back panel are one filled block.', 'Seiten, Deckel, Boden und Rückwand sind ein gefüllter Block.'),
   ('Shelf pins, panel thickness, edge banding.', 'Bodenträger, Plattenstärke, Kanten.'),
   ('The 0.018 m shelves (0.036 mm) are 0.13 mm strokes (0.065 m real) so that the compartments stay readable.', 'Die 0,018 m starken Böden (0,036 mm) sind 0,13-mm-Striche (0,065 m real), damit die Fächer lesbar bleiben.'),
   ('None.', 'Keine.'), ('Block tone 0.8, plinth 0.4.', 'Blockton 0,8, Sockel 0,4.'), ('Block lists carcass and back-panel as merged; shelves carry shelves; sag (0.9 deg, 6 mm) is far below 0.1 mm and invisible.', 'Der Block führt carcass und back-panel zusammen; Striche tragen shelves; die Durchbiegung (0,9 Grad, 6 mm) liegt weit unter 0,1 mm und ist unsichtbar.'))),
 '200': rep(200, s2, dd(('Shelf 4 x 9 mm: carcass block with five shelf strokes per half and a plinth.', 'Regal 4 x 9 mm: Korpusblock mit fünf Fachbodenstrichen je Hälfte und Sockel.'),
   ('Five boards, each as two half-span strokes that follow the sag and the re-pinning.', 'Fünf Böden, jeder als zwei Halbfeldstriche, die Durchbiegung und Umstecken folgen.'),
   ('Carcass panels and back panel are one block.', 'Korpusplatten und Rückwand sind ein Block.'),
   ('Panel thickness, pins.', 'Plattenstärke, Bodenträger.'), ('Boards (0.09 mm) are 0.13 mm detail lines.', 'Böden (0,09 mm) sind 0,13-mm-Detaillinien.'),
   ('None.', 'Keine.'), ('Block 0.85, plinth 0.4.', 'Block 0,85, Sockel 0,4.'), ('Block merges carcass and back-panel; strokes carry shelves.', 'Der Block führt carcass und back-panel zusammen; Striche tragen shelves.'))),
 '100': rep(100, s1, dd(('Shelf 8 x 18 mm: separate side, top and bottom panels, back panel, ten shelf halves and plinth.', 'Regal 8 x 18 mm: getrennte Seiten, Deckel und Boden, Rückwand, zehn Fachbodenhälften und Sockel.'),
   ('Every board and panel as its own filled shape; the halves overlap at mid-span so that the sag does not open a gap.', 'Jedes Brett und jede Platte als eigene gefüllte Form; die Hälften überlappen in Feldmitte, damit die Durchbiegung keine Lücke öffnet.'),
   ('Edge banding is merged into the panels.', 'Kanten gehen in den Platten auf.'),
   ('Shelf pins and the 3 mm back panel thickness.', 'Bodenträger und die 3-mm-Rückwandstärke.'),
   ('Panels 0.018 m are drawn 0.022 m (0.22 mm) so that they pass the 0.2 mm feature limit; the thickening of 4 mm stays inside the outline tolerance.', 'Platten von 0,018 m sind 0,022 m (0,22 mm) stark gezeichnet, damit sie die 0,2-mm-Grenze einhalten; die Verdickung von 4 mm bleibt innerhalb der Konturtoleranz.'),
   ('None.', 'Keine.'), ('Panels 0.7, back 0.9, plinth 0.4; panel outlines 0.25 mm.', 'Platten 0,7, Rückwand 0,9, Sockel 0,4; Plattenkonturen 0,25 mm.'), ('One shape per panel; shelf halves carry shelves.', 'Eine Form je Platte; Fachbodenhälften tragen shelves.'))),
 '50': rep(50, s0, dd(('Shelf 16 x 36 mm with real 18 mm panels, five boards in two halves each, shelf pins and recessed plinth.', 'Regal 16 x 36 mm mit echten 18-mm-Platten, fünf Böden in je zwei Hälften, Bodenträgern und zurückgesetztem Sockel.'),
   ('Real panel thickness, back panel, the pin under each board end that moves with the board when it is re-pinned.', 'Echte Plattenstärke, Rückwand, der Stift unter jedem Brettende, der beim Umstecken mit dem Brett wandert.'),
   ('Edge banding is merged into the panels.', 'Kanten gehen in den Platten auf.'),
   ('Pin holes (5 mm), the 32 mm hole grid, back-panel joints.', 'Stiftlöcher (5 mm), das 32-mm-Lochraster, Rückwandstöße.'),
   ('Pins 0.02 x 0.013 m are filled at 0.4 x 0.26 mm.', 'Stifte 0,02 x 0,013 m sind mit 0,4 x 0,26 mm gefüllt.'),
   ('None; laminate is shown by tone.', 'Keine; Laminat wird durch Ton gezeigt.'), ('Panels 0.7 with 0.25 mm outlines.', 'Platten 0,7 mit Konturen 0,25 mm.'), ('One shape per panel, board half and pin.', 'Eine Form je Platte, Brettzhälfte und Stift.'))),
}
for k in reps:
    reps[k]['validRange'] = {'minDenominator': RANGES[k][0], 'maxDenominator': RANGES[k][1]}
SYM = {'id': 'shelf-plan', 'title': T('Shelf unit plan symbol', 'Regal-Grundrisssymbol'), 'description': T('Floor-plan sign of the shelf unit: footprint 0.80 x 0.30 m with the back panel marked by a line.', 'Grundrisszeichen des Regals: Grundfläche 0,80 x 0,30 m, die Rückwand ist durch eine Linie markiert.'),
       'semantics': 'true-extent', 'units': 'model-m', 'denominators': [200, 100, 50], 'pens': {'body': 0.25, 'detail': 0.13, 'texture': None, 'glazing': None, 'ground': 0.35}, 'hatches': [],
       'geometry': {'elevation': [], 'plan': [
           {'id': 'plan-body', 'bone': 'ground', 'role': 'body', 'z': 1, 'filled': True, 'tone': 0.9, 'kind': 'poly', 'points': [[0, 0], [0.8, 0], [0.8, 0.3], [0, 0.3]], 'closed': True},
           {'id': 'plan-back', 'bone': 'ground', 'role': 'detail', 'z': 2, 'filled': False, 'tone': 0.5, 'kind': 'poly', 'points': [[0, 0.285], [0.8, 0.285]], 'closed': False}]},
       'dimensionalNote': T('Model metres at real size: x along the shelf 0 to 0.80 m, y from the front (0) to the back (0.30 m).', 'Modellmeter in Originalgröße: x entlang des Regals 0 bis 0,80 m, y von der Front (0) bis zur Rückseite (0,30 m).')}
A = assemble('bookshelf', T('Bookshelf with adjustable shelves', 'Bücherregal mit verstellbaren Böden'),
             T('A 0.80 x 1.80 m chipboard bookshelf with five adjustable shelves on 32 mm pin holes; the loaded pose bends every board by its computed beam deflection, the clips load the shelves and re-pin one.', 'Ein Bücherregal aus Spanplatte, 0,80 x 1,80 m, mit fünf verstellbaren Böden im 32-mm-Lochraster; die beladene Pose biegt jeden Boden um die berechnete Balkendurchbiegung, die Clips beladen die Böden und stecken einen um.'),
             ['bookshelf', 'storage', 'shelf', 'chipboard', 'adjustable', 'interior'],
             T(f'Front elevation 0.80 x 1.80 m, depth 0.30 m. Shelf deflection {DELTA * 1000:.1f} mm computed from beam theory (see structure). Books are loads and not drawn so that the empty and loaded states differ only physically.', f'Vorderansicht 0,80 x 1,80 m, Tiefe 0,30 m. Fachbodendurchbiegung {DELTA * 1000:.1f} mm aus der Balkentheorie (siehe Tragwerk). Bücher sind Lasten und nicht gezeichnet, damit sich leerer und beladener Zustand nur physikalisch unterscheiden.'),
             {'heightM': 1.8, 'widthM': 0.8, 'depthM': 0.3, 'massKg': round(mass, 3)}, MATS, RIG, anchors, reps, [SYM], P, clips, thermal, struct,
             {'defaultPose': 'empty', 'defaultClip': 'load-the-shelves', 'defaultScale': 50, 'viewpoints': ['elevation']},
             ['iso-10456-2007', 'en-1991-1-1', 'en-312', 'en-14749', 'system-32', 'incropera-2018', 'semio-nwf'])
if __name__ == '__main__':
    print(f'delta {DELTA*1000:.2f} mm theta {THETA:.3f} deg W {W_FULL}')
    main(A, 'bookshelf', ['empty', 'loaded', 'shelf-adjusted'])
