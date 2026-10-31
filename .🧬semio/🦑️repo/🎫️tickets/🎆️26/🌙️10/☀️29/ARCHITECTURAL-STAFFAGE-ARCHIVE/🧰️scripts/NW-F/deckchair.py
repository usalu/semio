import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nwf import *

LR, LP, LS, A_LEN, B_LEN = 1.045, 0.726, 0.495, 0.60, 0.45
ALPHA0 = 62.0
XF0 = 2 * LP * math.cos(math.radians(ALPHA0))
def pts_at(alpha, seat_world_deg):
    a = math.radians(alpha)
    xf = 2 * LP * math.cos(a)
    T = (LR * math.cos(a), LR * math.sin(a))
    P = (LP * math.cos(a), LP * math.sin(a))
    sa = math.radians(seat_world_deg)
    S = (P[0] + LS * math.cos(sa), P[1] + LS * math.sin(sa))
    return xf, P, T, S
def junction(T, S, a=A_LEN, b=B_LEN):
    d = math.hypot(S[0] - T[0], S[1] - T[1])
    u = ((S[0] - T[0]) / d, (S[1] - T[1]) / d)
    n = (-u[1], u[0])
    x = (a * a - b * b + d * d) / (2 * d)
    h = math.sqrt(max(a * a - x * x, 0))
    return (T[0] + x * u[0] - h * n[0], T[1] + x * u[1] - h * n[1])
SEAT0 = -29.8
xf0, P0, T0, S0 = pts_at(ALPHA0, SEAT0)
J0 = junction(T0, S0)
ANG_A0 = math.degrees(math.atan2(J0[1] - T0[1], J0[0] - T0[0]))
ANG_B0 = math.degrees(math.atan2(J0[1] - S0[1], J0[0] - S0[0]))
bones = [bone('ground', None, 'Ground reference', 'Bodenbezug', 0, 0, 0, 0.8, FIXED),
         bone('rear', 'ground', 'Rear frame with top rail', 'Hinterer Rahmen mit Kopfleiste', 0, 0, ALPHA0, LR, rev(-10, 30), (2.5, LR / 2, 0.0, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
         bone('front-slide', 'ground', 'Sliding front foot', 'Gleitender Vorderfuß', round(XF0, 4), 0, 0, 0.1, prism(0, -0.60, 0.20)),
         bone('front', 'front-slide', 'Front frame with armrests', 'Vorderrahmen mit Armlehnen', 0, 0, 180 - ALPHA0, LP, rev(-30, 10), (1.9, LP / 2, 0.0, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
         bone('seat', 'rear', 'Seat frame with front rail', 'Sitzrahmen mit Vorderleiste', LP, 0, round(SEAT0 - ALPHA0, 4), LS, rev(-100, 15), (1.3, LS / 2, 0.0, 'uniform', 'authored-estimate', 'wood-handbook-2021')),
         bone('canvas-a', 'rear', 'Back canvas (top rail to the sag)', 'Rückenbahn (Kopfleiste bis zum Durchhang)', LR, 0, round(ANG_A0 - ALPHA0, 4), A_LEN, rev(-180, 180), (0.14, A_LEN / 2, 0.0, 'uniform', 'authored-estimate', 'semio-nwf')),
         bone('canvas-b', 'seat', 'Seat canvas (sag to the front rail)', 'Sitzbahn (Durchhang bis zur Vorderleiste)', LS, 0, round(ANG_B0 - (SEAT0), 4), B_LEN, rev(-180, 180), (0.11, B_LEN / 2, 0.0, 'uniform', 'authored-estimate', 'semio-nwf'))]
bones[1]['mass']['kg'] = 2.5
rig0 = Rig(bones)
RIG = {'kind': 'mechanical', 'root': 'ground', 'bones': bones}
mass = sum(b['mass']['kg'] for b in bones if 'mass' in b)

def norm(d):
    while d > 180: d -= 360
    while d < -180: d += 360
    return d
def solve(alpha, seat_off):
    """pose bone offsets for a scissor angle alpha (deg) and seat-frame offset; canvas bones by IK."""
    d_rear = alpha - ALPHA0
    dx = 2 * LP * math.cos(math.radians(alpha)) - XF0
    off = {'rear': {'angleDeg': round(d_rear, 4)}, 'front-slide': {'dx': round(dx, 5)}, 'front': {'angleDeg': round(-d_rear, 4)}, 'seat': {'angleDeg': round(seat_off, 4)}}
    r = Rig(bones, off)
    ax, ay = r.world('canvas-a')[0][2], r.world('canvas-a')[1][2]
    bx, by = r.world('canvas-b')[0][2], r.world('canvas-b')[1][2]
    J = junction((ax, ay), (bx, by))
    ta = math.degrees(math.atan2(J[1] - ay, J[0] - ax)); tb = math.degrees(math.atan2(J[1] - by, J[0] - bx))
    wa = r.rot('rear') + bones[5]['angleDeg']; wb = r.rot('seat') + bones[6]['angleDeg']
    off['canvas-a'] = {'angleDeg': round(norm(ta - wa), 4)}; off['canvas-b'] = {'angleDeg': round(norm(tb - wb), 4)}
    return off, J
POSES_K = {}
def mk(alpha, seat_world):
    return solve(alpha, seat_world - SEAT0 - (alpha - ALPHA0) + 0) if False else solve(alpha, (seat_world - alpha) - (SEAT0 - ALPHA0))
O_UNF, J_UNF = solve(ALPHA0, 0.0)
O_FOLD, J_FOLD = mk(84.0, -96.0)
O_HALF, J_HALF = mk(73.0, -60.0)
O_RECL, J_RECL = mk(56.0, -29.8 + 0.0)
O_RECL, J_RECL = mk(56.0, -26.0)
for nm, o, j in (('unfolded', O_UNF, J_UNF), ('folded', O_FOLD, J_FOLD), ('half', O_HALF, J_HALF), ('reclined', O_RECL, J_RECL)):
    print(' ', nm, 'J', [round(v, 3) for v in j], o)

steel = mat('steel-hardware', 'Zinc-plated steel pivots', 'Verzinkte Stahlgelenke', 0.3, 0.5, 7800, 50, 450, 2.1e11, 3.55e8, 'literature', 'en-1993-1-1')
beech = mat('beech-timber', 'Beech timber, varnished', 'Buchenholz, lackiert', 0.9, 0.6, 720, 0.18, 1600, 1.2e10, 1.0e8, 'literature', 'wood-handbook-2021')
canvas = mat('cotton-canvas', 'Cotton canvas, striped', 'Baumwollsegeltuch, gestreift', 0.9, 0.55, 640, 0.06, 1300, 3.0e8, 3.0e7, 'authored-estimate', 'iso-10456-2007')
MATS = [beech, canvas, steel]
cl = lambda i, en, de: lay(i, en, de, 'clothing', 'cotton-canvas', 0.0007, canvas, 'authored-estimate')
regions = [
    reg('frame-timber', 'Beech frame members', 'Buchenrahmen', 'surface', 'beech-timber', 0.88, 0.04, 'vertical-plate', 0.5, [lay('beech-member', 'Beech 25 x 45 mm, half thickness', 'Buche 25 x 45 mm, halbe Dicke', 'timber', 'beech-timber', 0.0125, beech, 'literature', 'wood-handbook-2021')]),
    reg('canvas-back', 'Back canvas', 'Rückenbahn', 'surface', 'cotton-canvas', 0.30, 0.5, 'vertical-plate', 0.6, [cl('canvas-back', 'Canvas 0.7 mm', 'Segeltuch 0,7 mm')]),
    reg('canvas-seat', 'Seat canvas', 'Sitzbahn', 'surface', 'cotton-canvas', 0.23, 0.45, 'horizontal-plate-up', 0.8, [cl('canvas-seat', 'Canvas 0.7 mm', 'Segeltuch 0,7 mm')]),
]
RIDS = [r['id'] for r in regions]
ctx = Ctx(RIDS, {'frame-timber': (1.0, 0.5), 'canvas-back': (1.0, 0.8), 'canvas-seat': (1.0, 0.7)})
anchors = [anc(rig0, 'foot-rear', 'rear', 0, 0, 'contact', 'Rear feet', 'Hintere Füße', support='contact', normalDeg=90, footprintLengthM=0.05, footprintWidthM=0.58, frictionCoefficient=0.5),
           anc(rig0, 'foot-front', 'front', 0, 0, 'contact', 'Front feet', 'Vordere Füße', support='contact', normalDeg=90, footprintLengthM=0.05, footprintWidthM=0.58, frictionCoefficient=0.5),
           {'id': 'sling', 'bone': 'canvas-a', 'x': A_LEN, 'y': 0, 'kind': 'load', 'title': T('Sitting point in the sling', 'Sitzpunkt im Segeltuch')}]
CONT = ['foot-rear', 'foot-front']
occ = lambda n=735.5: [load('occupant-sling', 'Seated adult in the sling (75 kg)', 'Sitzende Person im Segeltuch (75 kg)', 'occupant', -n, anchor='sling', ref='en-1991-1-1')]
P = [
    pose(ctx, 'unfolded', 'Unfolded', 'Aufgeklappt', 'Frames at 62 degrees, the canvas hangs as a sling between the top rail and the front rail; in full sun.', 'Rahmen bei 62 Grad, das Segeltuch hängt als Sack zwischen Kopfleiste und Vorderleiste; in voller Sonne.', contacts=CONT),
    pose(ctx, 'folded', 'Folded flat', 'Zusammengeklappt', 'The scissor frames close to 84 degrees and stand almost parallel; the canvas hangs between them.', 'Die Scherenrahmen schließen sich auf 84 Grad und stehen fast parallel; das Segeltuch hängt dazwischen.', bones=O_FOLD, contacts=CONT,
         over={'canvas-back': (0.5, 0.3), 'canvas-seat': (0.4, 0.2), 'frame-timber': (0.8, 0.4)}),
    pose(ctx, 'reclined-seated', 'Reclined and seated', 'Zurückgelehnt, besetzt', 'Second notch (56 degrees): the front feet slide forward, an adult lies back in the sling.', 'Zweite Raste (56 Grad): die Vorderfüße gleiten nach vorn, eine erwachsene Person liegt im Segeltuch.', bones=O_RECL, contacts=CONT, loads=occ(),
         over={'canvas-back': (0.6, 0.35), 'canvas-seat': (0.6, 0.3)}, contact={'canvas-back': (0.4,), 'canvas-seat': (0.45,)}),
    pose(ctx, 'sling-proof-load', 'Seat proof load', 'Sitz-Prüflast', 'Static seat test load 1600 N in the sling of the unfolded chair.', 'Statische Sitz-Prüflast 1600 N im Segeltuch des aufgeklappten Stuhls.', contacts=CONT,
         loads=[load('proof-sling', 'Static seat test load (1600 N)', 'Statische Sitzprüflast (1600 N)', 'imposed', -1600, anchor='sling', ref='en-581-2')]),
    pose(ctx, 'half-folded-key', 'Half folded (key)', 'Halb zusammengeklappt (Schlüssel)', 'Frames at 73 degrees.', 'Rahmen bei 73 Grad.', key=True, bones=O_HALF, contacts=CONT, over={'canvas-back': (0.8, 0.5), 'canvas-seat': (0.7, 0.45)}),
]
clips = [
    clip('fold-chair', 'Fold the chair', 'Stuhl zusammenklappen', 'The front feet slide back, the scissor frames close and the sling collapses between them (4 s).', 'Die Vorderfüße gleiten zurück, die Scherenrahmen schließen sich und das Segeltuch fällt dazwischen zusammen (4 s).', 4.0,
         [{'t': 0, 'pose': 'unfolded', 'ease': 'smooth'}, {'t': 2.0, 'pose': 'half-folded-key', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'folded', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 4.0} for a in CONT], [('close', 0, 4.0, 'Closing', 'Schließen')], [(4.0, 'motion', 'Chair is flat', 'Stuhl liegt flach', None)],
         [('region:canvas-back:sunlitFraction', 'smooth', [(0, 0.8), (4.0, 0.3)]), ('region:canvas-seat:sunlitFraction', 'smooth', [(0, 0.7), (4.0, 0.2)])]),
    clip('unfold-chair', 'Unfold the chair', 'Stuhl aufklappen', 'The frames open until the seat frame locks on the ratchet at 62 degrees.', 'Die Rahmen öffnen sich, bis der Sitzrahmen bei 62 Grad in der Raste einrastet.', 4.0,
         [{'t': 0, 'pose': 'folded', 'ease': 'smooth'}, {'t': 2.0, 'pose': 'half-folded-key', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'unfolded', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 4.0} for a in CONT], [('open', 0, 4.0, 'Opening', 'Öffnen')], [(4.0, 'motion', 'Frame locks on the ratchet', 'Rahmen rastet ein', None)], []),
    clip('sit-and-recline', 'Sit and recline', 'Hinsetzen und zurücklehnen', 'An adult sits in the open chair and the frame slides into the second notch while the sling takes the load.', 'Eine erwachsene Person setzt sich in den geöffneten Stuhl, der Rahmen gleitet in die zweite Raste, während das Segeltuch die Last aufnimmt.', 4.0,
         [{'t': 0, 'pose': 'unfolded', 'ease': 'smooth'}, {'t': 1.5, 'pose': 'unfolded', 'ease': 'smooth'}, {'t': 3.5, 'pose': 'reclined-seated', 'ease': 'smooth'}, {'t': 4.0, 'pose': 'reclined-seated', 'ease': 'smooth'}],
         [{'anchor': a, 'fromS': 0, 'toS': 4.0} for a in CONT], [('sit', 0, 1.5, 'Sit down', 'Hinsetzen'), ('recline', 1.5, 4.0, 'Recline', 'Zurücklehnen')],
         [(1.5, 'load-on', 'Occupant sits in the sling', 'Person sitzt im Segeltuch', 'occupant-sling'), (1.5, 'thermal', 'Canvas starts taking 30 degC contact heat', 'Segeltuch nimmt 30-degC-Kontaktwärme auf', None)],
         [('load:occupant-sling', 'smooth', [(0, 0), (1.0, 0), (1.5, 1), (4.0, 1)])]),
]
thermal = {'environment': {'airTempC': 20, 'meanRadiantTempC': 20, 'relativeHumidity': 0.5, 'windMS': 1.0, 'windDirectionDeg': 180, 'solarWM2': 600, 'diffuseFraction': 0.15, 'pressurePa': 101325},
           'operating': [], 'regions': regions, 'emitters': [], 'assumptions': [T(*a) for a in [
               ('Cotton canvas (0.7 mm, 640 kg/m3 effective, 0.06 W/mK, 1300 J/kgK; authored-estimate) has a heat capacity of only 0.6 kJ/m2K: it follows sun and air within seconds and takes up contact warmth almost at once.', 'Baumwollsegeltuch (0,7 mm, 640 kg/m3 effektiv, 0,06 W/mK, 1300 J/kgK; Schätzwert) hat nur 0,6 kJ/m2K Wärmekapazität: es folgt Sonne und Luft binnen Sekunden und nimmt Kontaktwärme fast sofort auf.'),
               ('Beech members 25 x 45 mm after ISO 10456 / Wood Handbook; half thickness 12.5 mm is the heat-storing layer.', 'Buchenhölzer 25 x 45 mm nach ISO 10456 / Wood Handbook; die halbe Dicke 12,5 mm ist die wärmespeichernde Schicht.'),
               ('The seated pose covers 40 % of the back canvas and 45 % of the seat canvas with a clothed body at 30 degC (ISO 7730 clothing surface) and shades them partly.', 'Die besetzte Pose bedeckt 40 % der Rückenbahn und 45 % der Sitzbahn mit einem bekleideten Körper bei 30 degC (Bekleidungsoberfläche nach ISO 7730) und beschattet sie teilweise.'),
               ('Reference case: 600 W/m2 sun, 20 degC air, 1 m/s breeze; the folded chair is partly shaded by its own frames.', 'Referenzfall: 600 W/m2 Sonne, 20 degC Luft, 1 m/s Wind; der zusammengeklappte Stuhl wird von den eigenen Rahmen teilweise beschattet.')]]}
struct = structure(CONT, [
    ('The chair is a scissor mechanism: the front frame tip coincides with the rear frame at 0.726 m for every angle, so the front foot slides by x = 1.452 m cos(alpha) (prismatic joint) while the rear foot turns on the ground; the seat frame pivots at the scissor joint.', 'Der Stuhl ist ein Scherenmechanismus: die Vorderrahmenspitze fällt bei jedem Winkel mit dem Hinterrahmen bei 0,726 m zusammen, der Vorderfuß gleitet daher um x = 1,452 m cos(alpha) (Schiebegelenk), während sich der Hinterfuß am Boden dreht; der Sitzrahmen dreht am Scherengelenk.'),
    ('The canvas is two bones (0.60 m and 0.45 m, together 1.05 m, inextensible): their joint is solved for every pose as the lower intersection of the two circles around the top rail and the front rail (IK), hence the hanging sling.', 'Das Segeltuch besteht aus zwei Knochen (0,60 m und 0,45 m, zusammen 1,05 m, undehnbar): ihr Gelenk wird für jede Pose als untere Schnittstelle der beiden Kreise um Kopfleiste und Vorderleiste berechnet (IK), daher der hängende Sack.'),
    ('The seat proof load 1600 N follows the order of the static seat test of EN 581-2 (outdoor seating); an authored-estimate to be verified against the standard text.', 'Die Sitz-Prüflast 1600 N folgt der Größenordnung des statischen Sitztests nach EN 581-2 (Gartensitzmöbel); ein Schätzwert, am Normtext zu prüfen.')])
W = lambda x, y: (x, y)
# geometry: all in default pose, world metres
def tube(S_, sid, bone_, p, q, r, z, region='frame-timber', pen=None, mat_='beech-timber'):
    kw = {'penMm': pen} if pen else {}
    d = math.hypot(q[0] - p[0], q[1] - p[1]); ux, uy = (q[0] - p[0]) / d, (q[1] - p[1]) / d
    p = (p[0] + ux * r, p[1] + uy * r); q = (q[0] - ux * r, q[1] - uy * r)
    return S_.cap(sid, bone_, p[0], p[1], q[0], q[1], r, r, region=region, material=mat_, tone=0.55, z=z, **kw)
def line(S_, sid, bone_, p, q, z, region, pen=None, role='body'):
    kw = {'penMm': pen} if pen else {}
    return S_.line(sid, bone_, [p, q], role=role, z=z, region=region, **kw)
PIV = P0; FOOT_F = (XF0, 0.0)
def common(S_, mode, pen=None):
    """mode 'line' (500/200) or 'cap' (100/50)."""
    if mode == 'line':
        line(S_, 'rear', 'rear', (0, 0), T0, 3, 'frame-timber', pen)
        line(S_, 'front', 'front', FOOT_F, P0, 3, 'frame-timber', pen)
        line(S_, 'seat', 'seat', P0, S0, 3, 'frame-timber', pen)
    else:
        tube(S_, 'rear', 'rear', (0.0, 0.0), T0, 0.0225, 3, pen=pen)
        tube(S_, 'front', 'front', (XF0, 0.0), P0, 0.0225, 4, pen=pen)
        tube(S_, 'seat', 'seat', P0, S0, 0.0225, 5, pen=pen)
s5 = Shapes(rig0); common(s5, 'line', 0.18)
line(s5, 'canvas', 'canvas-a', T0, J0, 2, None, 0.13)
s5.items[-1].pop('region', None); s5.items[-1]['regions'] = ['canvas-back', 'canvas-seat']
s2 = Shapes(rig0); common(s2, 'line', 0.25)
line(s2, 'canvas-a', 'canvas-a', T0, J0, 2, 'canvas-back', 0.13, 'detail')
line(s2, 'canvas-b', 'canvas-b', J0, S0, 2, 'canvas-seat', 0.13, 'detail')
s1 = Shapes(rig0); common(s1, 'cap', 0.25)
line(s1, 'canvas-a', 'canvas-a', T0, J0, 2, 'canvas-back', 0.18, 'detail')
line(s1, 'canvas-b', 'canvas-b', J0, S0, 2, 'canvas-seat', 0.18, 'detail')
s1.ell('bolt', 'rear', P0[0], P0[1], 0.0125, region='frame-timber', material='steel-hardware', tone=0.2, z=6, penMm=0.13)
s0 = Shapes(rig0)
def strip(S_, sid, bone_, p, q, w, z, pen=0.25):
    d = math.hypot(q[0] - p[0], q[1] - p[1]); nx, ny = -(q[1] - p[1]) / d * w, (q[0] - p[0]) / d * w
    return S_.poly(sid, bone_, [(p[0] + nx, p[1] + ny), (q[0] + nx, q[1] + ny), (q[0] - nx, q[1] - ny), (p[0] - nx, p[1] - ny)], region='frame-timber', material='beech-timber', tone=0.55, z=z, penMm=pen)
strip(s0, 'rear', 'rear', (0.012, 0.022), T0, 0.0225, 3); strip(s0, 'front', 'front', (XF0 - 0.012, 0.022), P0, 0.0225, 4); strip(s0, 'seat', 'seat', P0, S0, 0.0225, 5)
line(s0, 'canvas-a', 'canvas-a', T0, J0, 2, 'canvas-back', 0.25, 'detail')
line(s0, 'canvas-b', 'canvas-b', J0, S0, 2, 'canvas-seat', 0.25, 'detail')
s0.ell('bolt', 'rear', P0[0], P0[1], 0.0125, region='frame-timber', material='steel-hardware', tone=0.2, z=6, penMm=0.13)
s0.ell('top-rail', 'rear', T0[0], T0[1], 0.0125, region='frame-timber', material='beech-timber', tone=0.4, z=2.5, penMm=0.13)
s0.ell('front-rail', 'seat', S0[0], S0[1], 0.0125, region='frame-timber', material='beech-timber', tone=0.4, z=5.5, penMm=0.13)
def dd(*a): return D(*a)
reps = {
 '500': rep(500, s5, dd(('Chair sign 1.6 x 1.9 mm: the scissor frames and the seat frame as three strokes with the sling as one stroke.', 'Stuhlzeichen 1,6 x 1,9 mm: die Scherenrahmen und der Sitzrahmen als drei Striche mit dem Segeltuch als einem Strich.'),
   ('The crossing A of rear and front frame, the seat frame and the hanging back canvas.', 'Das Kreuz aus Hinter- und Vorderrahmen, der Sitzrahmen und die hängende Rückenbahn.'),
   ('The sling stroke lists back and seat canvas as merged.', 'Der Segeltuchstrich führt Rücken- und Sitzbahn zusammen.'),
   ('The seat canvas from the sag to the front rail (it would run closer than the 0.15 m minimum gap to the rear frame), crossbars, pivots.', 'Die Sitzbahn vom Durchhang zur Vorderleiste (sie läge näher als der Mindestabstand 0,15 m am Hinterrahmen), Querleisten, Gelenke.'),
   ('Rails (0.045 m) are 0.25 mm strokes = 0.125 m real; the canvas is a 0.13 mm stroke.', 'Leisten (0,045 m) sind 0,25-mm-Striche = 0,125 m real; das Segeltuch ist ein 0,13-mm-Strich.'),
   ('None.', 'Keine.'), ('Strokes black.', 'Striche schwarz.'), ('Frame strokes carry frame-timber, the sling stroke both canvas regions.', 'Rahmenstriche tragen frame-timber, der Segeltuchstrich beide Segeltuchregionen.'))),
 '200': rep(200, s2, dd(('Chair 3.6 x 4.7 mm: three frame strokes and two canvas strokes meeting at the sag.', 'Stuhl 3,6 x 4,7 mm: drei Rahmenstriche und zwei Segeltuchstriche, die sich im Durchhang treffen.'),
   ('Scissor, seat frame, both canvas halves with the sag.', 'Schere, Sitzrahmen, beide Segeltuchhälften mit dem Durchhang.'), ('None.', 'Keine.'), ('Crossbars, pivots, ratchet.', 'Querleisten, Gelenke, Raste.'),
   ('Rails are 0.25 mm strokes; canvas 0.13 mm detail strokes.', 'Leisten sind 0,25-mm-Striche; Segeltuch 0,13-mm-Detailstriche.'), ('None.', 'Keine.'), ('Strokes black.', 'Striche schwarz.'), ('One stroke per member and canvas half.', 'Ein Strich je Leiste und Segeltuchhälfte.'))),
 '100': rep(100, s1, dd(('Chair 7.8 x 9.3 mm: three tubes as filled capsules, two canvas strokes and the pivot bolt.', 'Stuhl 7,8 x 9,3 mm: drei Leisten als gefüllte Kapseln, zwei Segeltuchstriche und der Gelenkbolzen.'),
   ('Real rail depth 45 mm, the pivot, the canvas sling.', 'Echte Leistentiefe 45 mm, das Gelenk, das Segeltuchsack.'), ('Left and right rails coincide.', 'Linke und rechte Leisten fallen zusammen.'), ('Crossbars, ratchet.', 'Querleisten, Raste.'),
   ('Canvas is a 0.18 mm stroke (0.0018 m thick on paper only).', 'Das Segeltuch ist ein 0,18-mm-Strich.'), ('None.', 'Keine.'), ('Rails 0.55, bolt 0.2.', 'Leisten 0,55, Bolzen 0,2.'), ('One shape per member.', 'Eine Form je Bauteil.'))),
 '50': rep(50, s0, dd(('Chair 15.6 x 18.6 mm: rails as capsules, top rail and front rail dowels in section, pivot bolt and canvas strokes.', 'Stuhl 15,6 x 18,6 mm: Leisten als Kapseln, Kopf- und Vorderleiste als Rundholzschnitt, Gelenkbolzen und Segeltuchstriche.'),
   ('Dowels that hold the canvas, bolt, real rail depth.', 'Rundhölzer, die das Segeltuch halten, Bolzen, echte Leistentiefe.'), ('Left and right rails coincide.', 'Linke und rechte Leisten fallen zusammen.'), ('Ratchet teeth, canvas stripes, hem.', 'Rastzähne, Segeltuchstreifen, Saum.'),
   ('Canvas is a 0.25 mm stroke.', 'Das Segeltuch ist ein 0,25-mm-Strich.'), ('None.', 'Keine.'), ('Rails 0.55.', 'Leisten 0,55.'), ('One shape per member.', 'Eine Form je Bauteil.'))),
}
for k in reps:
    reps[k]['validRange'] = {'minDenominator': RANGES[k][0], 'maxDenominator': RANGES[k][1]}
SYM = {'id': 'chair-plan', 'title': T('Deck chair plan symbol', 'Liegestuhl-Grundrisssymbol'), 'description': T('Plan sign of the unfolded chair: footprint 0.78 x 0.58 m with the canvas strip.', 'Grundrisszeichen des aufgeklappten Stuhls: Grundfläche 0,78 x 0,58 m mit dem Segeltuchstreifen.'),
       'semantics': 'true-extent', 'units': 'model-m', 'denominators': [200, 100, 50], 'pens': {'body': 0.25, 'detail': 0.13, 'texture': None, 'glazing': None, 'ground': 0.35}, 'hatches': [],
       'geometry': {'elevation': [], 'plan': [
           {'id': 'plan-frame', 'bone': 'ground', 'role': 'body', 'z': 1, 'filled': False, 'tone': 0.5, 'kind': 'poly', 'points': [[0, 0], [0.78, 0], [0.78, 0.58], [0, 0.58]], 'closed': True},
           {'id': 'plan-canvas', 'bone': 'ground', 'role': 'detail', 'z': 2, 'filled': False, 'tone': 0.5, 'kind': 'poly', 'points': [[0.05, 0.04], [0.73, 0.04], [0.73, 0.54], [0.05, 0.54]], 'closed': True}]},
       'dimensionalNote': T('Model metres at real size: x along the chair 0 to 0.78 m, y across 0 to 0.58 m, unfolded.', 'Modellmeter in Originalgröße: x entlang 0 bis 0,78 m, y quer 0 bis 0,58 m, aufgeklappt.')}
bb = bbox_world(rig0, [s for s in s0.items if s['role'] != 'ground'])
A = assemble('folding-deck-chair', T('Folding deck chair with canvas sling', 'Klapp-Liegestuhl mit Segeltuch'),
             T('A beech deck chair of three hinged frames and a hanging canvas sling that folds flat: the scissor frames slide and turn, the sling hangs as two inextensible canvas bones solved for every pose.', 'Ein Buchen-Liegestuhl aus drei gelenkigen Rahmen und einem hängenden Segeltuchsack, der flach zusammenklappt: die Scherenrahmen gleiten und drehen sich, das Segeltuch hängt als zwei undehnbare Knochen, die für jede Pose berechnet werden.'),
             ['deck-chair', 'folding', 'sling', 'canvas', 'outdoor', 'seating'],
             T('Side elevation, 0.78 x 0.93 m unfolded, width across 0.58 m. Joint geometry: rear frame 1.045 m at 62 degrees, front frame and seat pivot at 0.726 m, seat frame 0.495 m, canvas 0.60 + 0.45 m. Canvas poses are computed by a scratch script (circle intersection) and stored as bone offsets.', 'Seitenansicht, 0,78 x 0,93 m aufgeklappt, Breite quer 0,58 m. Gelenkgeometrie: Hinterrahmen 1,045 m bei 62 Grad, Vorderrahmen und Sitzgelenk bei 0,726 m, Sitzrahmen 0,495 m, Segeltuch 0,60 + 0,45 m. Segeltuchposen werden von einem Hilfsskript (Kreisschnitt) berechnet und als Knochen-Offsets gespeichert.'),
             {'heightM': 0.93, 'widthM': 0.78, 'depthM': 0.58, 'massKg': round(mass, 3)}, MATS, RIG, anchors, reps, [SYM], P, clips, thermal, struct,
             {'defaultPose': 'unfolded', 'defaultClip': 'fold-chair', 'defaultScale': 50, 'viewpoints': ['elevation']},
             ['iso-10456-2007', 'en-1991-1-1', 'en-581-1', 'en-581-2', 'en-1993-1-1', 'iso-7730-2005', 'wood-handbook-2021', 'incropera-2018', 'semio-nwf'])
if __name__ == '__main__':
    main(A, 'folding-deck-chair', ['unfolded', 'folded', 'reclined-seated'])
