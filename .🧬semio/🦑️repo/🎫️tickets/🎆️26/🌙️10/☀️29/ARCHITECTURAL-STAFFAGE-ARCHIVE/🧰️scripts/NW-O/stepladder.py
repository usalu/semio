import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
from geo import *

xf = lambda y: -0.36 + 0.24 * y / 1.5
H = (-0.13, 1.20)
xr = lambda y: H[0] + 0.36 * (H[1] - y) / H[1]
F0, T1 = (xf(0), 0.0), (xf(1.5), 1.5)
R0 = (0.23, 0.0)
P1 = (xf(0.62), 0.62)
P2 = (xr(0.50), 0.50)
KN = ((P1[0] + P2[0]) / 2, (P1[1] + P2[1]) / 2)
LK = math.hypot(P2[0] - P1[0], P2[1] - P1[1]) / 2


def ext(p, q, e):
    l = math.hypot(q[0] - p[0], q[1] - p[1])
    return (q[0] + (q[0] - p[0]) / l * e, q[1] + (q[1] - p[1]) / l * e)


def fold(theta):
    p2 = rot(P2, H, theta)
    k = circ_int(P1, LK, p2, LK, up=True)
    return {'front-frame': {'angleDeg': round(ang(P1, k) - ang(P1, KN), 3)}, 'rear-frame': {'angleDeg': round(ang(p2, k) - ang(P2, KN) - theta, 3)}, 'rear-frame': {'angleDeg': theta}}


refs = [
    ref('en-131-1', 'EN 131-1:2015+A1:2019 Ladders - Part 1: Terms, types, functional sizes (stepladder types, tread depth, step pitch).'),
    ref('en-131-2', 'EN 131-2:2010+A2:2017 Ladders - Part 2: Requirements, testing, marking (rated load 150 kg for professional ladders, strength and stability tests).'),
    ref('en-1999-1-1', 'EN 1999-1-1:2007 Eurocode 9: Design of aluminium structures, Part 1-1 (E = 70 GPa, density 2700 kg/m3).'),
    ref('iso-10456-2007', 'ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (aluminium, steel, rubber).'),
    ref('incropera-2018', 'Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties of metals).'),
]
mats = [
    material('alu-mill', 'Mill-finish aluminium extrusion', 'Aluminiumstrangpressprofil, walzblank', 0.12, 0.3, 2700, 160, 880, 'literature', 'iso-10456-2007', 70e9),
    material('steel-zinc', 'Zinc-plated steel', 'Verzinkter Stahl', 0.25, 0.55, 7800, 50, 450, 'literature', 'iso-10456-2007', 210e9),
    material('rubber-black', 'Black rubber', 'Schwarzer Gummi', 0.92, 0.9, 1100, 0.25, 1900, 'authored-estimate', 'iso-10456-2007'),
]

rig = Rig('mechanical', 'front-frame')
rig.bone('front-frame', None, 0, 0, 1.5, FIXED, T('Front frame with treads and platform', 'Vorderholm mit Stufen und Plattform'), (3.95, -0.2, 0.75, 'authored-estimate', 'en-131-1'))
rig.bone('rear-frame', 'front-frame', H[0], H[1], 1.25, FIXED, T('Rear support frame (locked open)', 'Hinterholm (Stützrahmen)'), (1.35, 0.05, 0.6, 'authored-estimate', 'en-131-1'))
anchors = [
    rig.anchor('foot-front', 'front-frame', F0[0], 0, 'contact', T('Front foot pads', 'Vordere Fußkappen'), support='contact', footprintLengthM=0.08, footprintWidthM=0.46, frictionCoefficient=0.7),
    rig.anchor('foot-rear', 'rear-frame', R0[0], 0, 'contact', T('Rear foot pads', 'Hintere Fußkappen'), support='contact', footprintLengthM=0.08, footprintWidthM=0.40, frictionCoefficient=0.7),
    rig.anchor('standing-point', 'front-frame', -0.05, 1.26, 'load', T('Standing point on the platform', 'Standpunkt auf der Plattform')),
]
# ----------------------------------------------------------- geometry
def treads(s, w, tone=0.5, z=3.4, prefix='tread', **kw):
    pass

TREAD_Y = [0.25, 0.50, 0.75, 1.00]


def r500():
    s = Shapes(rig)
    s.poly('front-rail', 'front-frame', band((xf(0.05), 0.05), T1, 0.14), z=3, tone=0.5, region='front-rails', material='alu-mill')
    for i, y in enumerate(TREAD_Y):
        s.line(f'tread-{i}', 'front-frame', [(xf(y) - 0.09, y), (xf(y) + 0.03, y)], z=3.2, region='treads', pen=0.13)
    s.line('platform', 'front-frame', [(-0.22, 1.25), (0.04, 1.25)], z=3.3, region='platform', pen=0.25)
    s.poly('rear-rail', 'rear-frame', band((H[0], H[1]), (0.185, 0.05), 0.16), z=2.8, tone=0.5, regions=['rear-frame', 'spreaders'], material='alu-mill')
    s.line('foot-rear', 'rear-frame', [(0.19, 0.02), (0.27, 0.02)], z=2.9, region='feet', pen=0.18)
    s.line('foot-front', 'front-frame', [(-0.40, 0.02), (-0.32, 0.02)], z=2.9, region='feet', pen=0.18)
    return rep(500, 350, 1000, pens(0.13, None, None, None, 0.18), s, [
        ('An A-shaped sign 1.4 mm wide and 3 mm high: a solid front rail leaning towards the rear rail, four tread ticks and a platform tick.', 'Ein A-förmiges Zeichen, 1,4 mm breit und 3 mm hoch: ein voller Vorderholm, der zum Hinterholm hin geneigt ist, vier Stufenstriche und ein Plattformstrich.'),
        ('Both rails with their true inclination, the four treads and the platform at their true heights, both foot pads, the 0.68 m span.', 'Beide Holme mit wahrer Neigung, die vier Stufen und die Plattform in wahrer Höhe, beide Fußkappen, die Spannweite 0,68 m.'),
        ('The two front rails merge into one band, the two rear rails likewise; the top cap merges into the rail end.', 'Die zwei Vorderholme gehen in einem Band auf, ebenso die zwei Hinterholme; die Abschlusskappe geht im Holmende auf.'),
        ('Separate spreader stroke, hinge, rear cross brace, tread profile and handrail (the spreaders are merged into the rear band).', 'Eigener Spreizenstrich, Gelenk, hintere Quertraverse, Stufenprofil und Handlauf (die Spreizen gehen im Hinterband auf).'),
        ('The 0.06 m rail would be 0.12 mm wide on paper: thickened symmetrically to 0.14 m (0.28 mm); the 0.03 m treads are single 0.13 mm strokes and the platform one 0.25 mm stroke, pen inflation 4 to 8 times.', 'Der 0,06 m breite Holm wäre auf dem Papier 0,12 mm breit: symmetrisch auf 0,14 m (0,28 mm) verbreitert; die 0,03 m starken Stufen sind einzelne 0,13-mm-Striche und die Plattform ein 0,25-mm-Strich, Stiftaufblähung 4- bis 8-fach.'),
        ('None.', 'Keine.'),
        ('Rails tone 0.5 with 0.13 mm outline; ticks and ground 0.13 / 0.18 mm.', 'Holme Ton 0,5 mit 0,13-mm-Kontur; Striche und Boden 0,13 / 0,18 mm.'),
        ('Front rail = front-rails, treads = treads, platform = platform, rear rail = rear-frame and spreaders (merged), foot ticks = feet.', 'Vorderholm = front-rails, Stufen = treads, Plattform = platform, Hinterholm = rear-frame und spreaders (verschmolzen), Fußstriche = feet.'),
    ])


def r200():
    s = Shapes(rig)
    s.poly('front-rail', 'front-frame', band((xf(0.0), 0.0), T1, 0.06), z=3, tone=0.5, region='front-rails', material='alu-mill')
    for i, y in enumerate(TREAD_Y):
        s.line(f'tread-{i}', 'front-frame', [(xf(y) - 0.09, y), (xf(y) + 0.03, y)], z=3.2, region='treads', pen=0.18)
    s.poly('platform', 'front-frame', [(-0.22, 1.205), (0.04, 1.205), (0.04, 1.265), (-0.22, 1.265)], z=3.3, tone=0.4, region='platform', material='alu-mill')
    s.poly('rear-rail', 'rear-frame', band((-0.135, 1.235), (0.225, 0.03), 0.10), z=2.8, tone=0.5, regions=['rear-frame', 'spreaders'], material='alu-mill')
    s.poly('foot-front', 'front-frame', [(-0.40, 0.0), (-0.32, 0.0), (-0.32, 0.06), (-0.40, 0.06)], z=2.9, tone=0.9, region='feet', material='rubber-black')
    s.poly('foot-rear', 'rear-frame', [(0.19, 0.0), (0.27, 0.0), (0.27, 0.06), (0.19, 0.06)], z=2.9, tone=0.9, region='feet', material='rubber-black')
    return rep(200, 150, 350, pens(0.18, 0.13, None, None, 0.25), s, [
        ('A-frame 3.4 mm wide and 7.5 mm high: front rail band, a thicker rear rail band (0.10 m, real 0.05 m), four tread strokes, platform slab, two foot pads.', 'A-Gestell 3,4 mm breit und 7,5 mm hoch: Vorderholmband, ein dickeres Hinterholmband (0,10 m, real 0,05 m), vier Stufenstriche, Plattformplatte, zwei Fußkappen.'),
        ('Front rail at its true profile depth 0.06 m, rear rail 0.10 m, treads, platform, spreader and both feet.', 'Vorderholm in wahrer Profiltiefe 0,06 m, Hinterholm 0,10 m, Stufen, Plattform, Spreize und beide Füße.'),
        ('The paired rails per side are one band; tread nosing and anti-slip ribs merge into the stroke.', 'Die paarigen Holme je Seite sind ein Band; Stufenkante und Rippen gehen im Strich auf.'),
        ('Hinge, rivets, cross braces, handrail, tread ribs; the spreader links are merged into the rear rail band (region spreaders listed on it) and not drawn separately.', 'Gelenk, Nieten, Traversen, Handlauf, Stufenrippen; die Spreizenglieder gehen im Hinterholmband auf (Region spreaders dort aufgeführt) und werden nicht getrennt gezeichnet.'),
        ('Treads (0.03 m = 0.15 mm) are 0.18 mm strokes; the spreader is a 0.13 mm hairline in two links; the platform slab is thickened to 0.06 m and the foot pads to 0.06 m (0.3 mm).', 'Stufen (0,03 m = 0,15 mm) sind 0,18-mm-Striche; die Spreize ist eine 0,13-mm-Haarlinie in zwei Gliedern; die Plattformplatte ist auf 0,06 m und die Fußkappen sind auf 0,06 m (0,3 mm) verdickt.'),
        ('None.', 'Keine.'),
        ('Rails 0.5, platform 0.4, rubber feet 0.9; body pen 0.18 mm, detail 0.13 mm.', 'Holme 0,5, Plattform 0,4, Gummifüße 0,9; Körperstift 0,18 mm, Detail 0,13 mm.'),
        ('Spreader strokes carry the region spreaders; feet carry feet; all regions drawn.', 'Spreizenstriche tragen die Region spreaders, Füße die Region feet; alle Regionen sind gezeichnet.'),
    ])


def r100():
    s = Shapes(rig)
    s.poly('front-rail', 'front-frame', band((xf(0.0), 0.03), T1, 0.06), z=3, tone=0.5, region='front-rails', material='alu-mill')
    for i, y in enumerate(TREAD_Y):
        s.poly(f'tread-{i}', 'front-frame', [(xf(y) - 0.09, y - 0.012), (xf(y) + 0.03, y - 0.012), (xf(y) + 0.03, y + 0.018), (xf(y) - 0.09, y + 0.018)], z=3.2, tone=0.55, region='treads', material='alu-mill')
    s.poly('platform', 'front-frame', [(-0.23, 1.225), (0.04, 1.225), (0.04, 1.265), (-0.23, 1.265)], z=3.3, tone=0.4, region='platform', material='alu-mill')
    s.poly('rear-rail', 'rear-frame', band((-0.135, 1.235), (0.225, 0.03), 0.05), z=2.8, tone=0.5, region='rear-frame', material='alu-mill')
    s.line('rear-brace', 'rear-frame', [(xr(0.22) - 0.02, 0.22), (xr(0.22) + 0.02, 0.22)], role='detail', z=2.85, region='rear-frame')
    s.poly('spreader-front', 'front-frame', band(P1, ext(P1, KN, 0.03), 0.04), z=2.7, tone=0.6, region='spreaders', material='steel-zinc')
    s.poly('spreader-rear', 'rear-frame', band(P2, ext(P2, KN, 0.03), 0.04), z=2.7, tone=0.6, region='spreaders', material='steel-zinc')
    s.ell('hinge', 'rear-frame', H[0], H[1], 0.025, z=3.6, tone=0.7, role='detail', region='platform', filled=False)
    s.path('foot-front', 'front-frame', 'M -0.41 0 L -0.315 0 L -0.32 0.05 L -0.405 0.05 Z', z=2.9, tone=0.9, region='feet', material='rubber-black')
    s.path('foot-rear', 'rear-frame', 'M 0.18 0 L 0.27 0 L 0.265 0.05 L 0.185 0.05 Z', z=2.9, tone=0.9, region='feet', material='rubber-black')
    return rep(100, 75, 150, pens(0.25, 0.13, None, None, 0.35), s, [
        ('A-frame 6.8 mm wide and 15 mm high with treads protruding on the climbing side, a slab platform, rear rail with one brace and a two-link spreader.', 'A-Gestell 6,8 mm breit und 15 mm hoch mit auf der Aufstiegsseite vorstehenden Stufen, Plattformplatte, Hinterholm mit einer Traverse und zweigliedriger Spreize.'),
        ('Rail bands at true depth, four treads of 0.09 m depth, platform, brace, spreader links, hinge circle, two tapered foot pads.', 'Holmbänder in wahrer Tiefe, vier Stufen von 0,10 m Tiefe, Plattform, Traverse, Spreizenglieder, Gelenkkreis, zwei verjüngte Fußkappen.'),
        ('Paired rails merge to one band per side; the top cap ends the front band.', 'Paarige Holme gehen je Seite in einem Band auf; die Kappe schließt das Vorderband ab.'),
        ('Tread ribs, rivets, handrail, second rear brace.', 'Stufenrippen, Nieten, Handlauf, zweite Hintertraverse.'),
        ('The spreader links are 0.04 m (0.4 mm) solid bands; the hinge ring (0.05 m = 0.5 mm) is a 0.13 mm line.', 'Die Spreizenglieder sind 0,04-m-Vollbänder (0,4 mm); der Gelenkring (0,05 m = 0,5 mm) ist eine 0,13-mm-Linie.'),
        ('None.', 'Keine.'),
        ('Rails 0.5, treads 0.55, platform 0.4, spreader 0.6, feet 0.9; body pen 0.25 mm, detail 0.13 mm.', 'Holme 0,5, Stufen 0,55, Plattform 0,4, Spreize 0,6, Füße 0,9; Körperstift 0,25 mm, Detail 0,13 mm.'),
        ('The hinge ring carries the platform region it belongs to; spreader bands carry spreaders.', 'Der Gelenkring trägt die Region platform, zu der er gehört; Spreizenbänder tragen spreaders.'),
    ])


def r50():
    s = Shapes(rig)
    s.poly('front-rail', 'front-frame', band((xf(0.0), 0.03), T1, 0.06), z=3, tone=0.5, region='front-rails', material='alu-mill')
    s.line('rail-groove', 'front-frame', [(xf(0.1) - 0.012, 0.1), (xf(1.45) - 0.012, 1.45)], role='texture', z=3.5, region='front-rails')
    s.path('top-cap', 'front-frame', f'M {T1[0] - 0.03:g} 1.49 C {T1[0] - 0.03:g} 1.52 {T1[0] + 0.03:g} 1.52 {T1[0] + 0.03:g} 1.49 L {T1[0] + 0.03:g} 1.44 L {T1[0] - 0.03:g} 1.44 Z', z=3.3, tone=0.2, region='front-rails', material='alu-mill')
    for i, y in enumerate(TREAD_Y):
        s.path(f'tread-{i}', 'front-frame', f'M {xf(y) - 0.09:g} {y - 0.012:g} L {xf(y) + 0.03:g} {y - 0.012:g} L {xf(y) + 0.03:g} {y + 0.018:g} L {xf(y) - 0.08:g} {y + 0.018:g} C {xf(y) - 0.09:g} {y + 0.018:g} {xf(y) - 0.094:g} {y + 0.005:g} {xf(y) - 0.09:g} {y - 0.012:g} Z', z=3.2, tone=0.55, region='treads', material='alu-mill')
        for k in range(3):
            xx = xf(y) - 0.07 + k * 0.035
            s.line(f'tread-{i}-rib-{k}', 'front-frame', [(xx, y + 0.002), (xx, y + 0.016)], role='texture', z=3.6, region='treads')
    s.path('platform', 'front-frame', 'M -0.23 1.225 L 0.04 1.225 L 0.04 1.265 L -0.215 1.265 C -0.228 1.265 -0.234 1.245 -0.23 1.225 Z', z=3.3, tone=0.4, region='platform', material='alu-mill')
    s.line('platform-ribs', 'front-frame', [(-0.19, 1.258), (0.01, 1.258)], role='texture', z=3.6, region='platform')
    s.poly('rear-rail', 'rear-frame', band((-0.135, 1.235), (0.225, 0.04), 0.05), z=2.8, tone=0.5, region='rear-frame', material='alu-mill')
    s.line('rear-groove', 'rear-frame', [(xr(1.1) + 0.01, 1.1), (xr(0.1) + 0.01, 0.1)], role='texture', z=3.5, region='rear-frame')
    for i, y in enumerate((0.22, 0.85)):
        s.poly(f'rear-brace-{i}', 'rear-frame', [(xr(y) - 0.022, y - 0.012), (xr(y) + 0.022, y - 0.012), (xr(y) + 0.022, y + 0.012), (xr(y) - 0.022, y + 0.012)], z=3.1, tone=0.6, region='rear-frame', material='alu-mill')
    s.poly('spreader-front', 'front-frame', band(P1, ext(P1, KN, 0.03), 0.024), z=2.7, tone=0.6, region='spreaders', material='steel-zinc')
    s.poly('spreader-rear', 'rear-frame', band(P2, ext(P2, KN, 0.03), 0.024), z=2.7, tone=0.6, region='spreaders', material='steel-zinc')
    s.ell('hinge', 'rear-frame', H[0], H[1], 0.027, z=3.6, tone=0.7, role='detail', region='platform')
    s.path('foot-front', 'front-frame', 'M -0.405 0 L -0.315 0 L -0.32 0.045 L -0.40 0.045 Z', z=2.9, tone=0.9, region='feet', material='rubber-black')
    s.path('foot-rear', 'rear-frame', 'M 0.18 0 L 0.27 0 L 0.265 0.045 L 0.185 0.045 Z', z=2.9, tone=0.9, region='feet', material='rubber-black')
    s.line('foot-tread', 'front-frame', [(-0.395, 0.008), (-0.325, 0.008)], role='texture', z=3.6, region='feet')
    return rep(50, 20, 75, pens(0.5, 0.25, 0.13, None, 0.7), s, [
        ('A-frame 13.6 mm wide and 30 mm high, drawn in full construction: rails with groove line and top cap, four ribbed treads with nosing, ribbed platform, rear rail with two braces, two-link spreader, hinge disc, rubber feet.', 'A-Gestell 13,6 mm breit und 30 mm hoch, in voller Konstruktion gezeichnet: Holme mit Rillenlinie und Kappe, vier gerippte Stufen mit Nase, gerippte Plattform, Hinterholm mit zwei Traversen, zweigliedrige Spreize, Gelenkscheibe, Gummifüße.'),
        ('All construction elements at real size and position.', 'Alle Konstruktionselemente in realer Größe und Lage.'),
        ('Paired rails merge to one band per side.', 'Paarige Holme gehen je Seite in einem Band auf.'),
        ('Tread-bolt ends and rivets of the rungs, the side handrail, labels.', 'Bolzenenden der Stufen und Nieten, der seitliche Handlauf, Beschriftungen.'),
        ('Spreader links are 0.024 m (0.48 mm) bands, the hinge disc 0.054 m (1.1 mm); ribs are 0.13 mm lines every 1.75 mm.', 'Spreizenglieder sind 0,024-m-Bänder (0,48 mm), die Gelenkscheibe 0,054 m (1,1 mm); Rippen sind 0,13-mm-Linien im Abstand 1,75 mm.'),
        ('Tread ribs and platform ribs as parallel 0.13 mm lines (not as hatch fill); no other hatching.', 'Stufen- und Plattformrippen als parallele 0,13-mm-Linien (keine Schraffur); sonst keine Schraffur.'),
        ('Aluminium 0.4-0.55, spreader 0.6, rubber 0.9; body pen 0.5 mm, detail 0.25 mm, texture 0.13 mm.', 'Aluminium 0,4-0,55, Spreize 0,6, Gummi 0,9; Körperstift 0,5 mm, Detail 0,25 mm, Textur 0,13 mm.'),
        ('Ribs and grooves carry the region of the part they mark; the hinge disc carries platform.', 'Rippen und Rillen tragen die Region des Teils, das sie kennzeichnen; die Gelenkscheibe trägt platform.'),
    ])


# ----------------------------------------------------------- thermal
al = lambda t: layer('wall', T('Extruded aluminium wall', 'Aluminiumprofilwand'), 'metal', 'alu-mill', t, 160, 2700, 880, 'literature', 'iso-10456-2007')
regions = [
    region('front-rails', T('Front rails', 'Vorderholme'), 'surface', 'passive', 'alu-mill', 0.52, 0.06, 'vertical-plate', 0.5, [al(0.0015)], 'authored-estimate', 'incropera-2018'),
    region('treads', T('Treads', 'Stufen'), 'surface', 'passive', 'alu-mill', 0.50, 0.10, 'horizontal-plate-up', 0.7, [al(0.002)], 'authored-estimate', 'incropera-2018'),
    region('platform', T('Platform', 'Plattform'), 'surface', 'passive', 'alu-mill', 0.30, 0.34, 'horizontal-plate-up', 0.9, [al(0.002)], 'authored-estimate', 'incropera-2018'),
    region('rear-frame', T('Rear frame', 'Hinterholme'), 'surface', 'passive', 'alu-mill', 0.43, 0.05, 'vertical-plate', 0.5, [al(0.0012)], 'authored-estimate', 'incropera-2018'),
    region('spreaders', T('Spreader links', 'Spreizenglieder'), 'surface', 'passive', 'steel-zinc', 0.06, 0.02, 'cylinder', 0.4, [layer('strap', T('Zinc-plated steel strap 2 mm', 'Verzinktes Stahlband 2 mm'), 'metal', 'steel-zinc', 0.002, 50, 7800, 450, 'literature', 'iso-10456-2007')], 'authored-estimate', 'incropera-2018'),
    region('feet', T('Rubber foot pads', 'Gummifußkappen'), 'surface', 'passive', 'rubber-black', 0.02, 0.08, 'vertical-plate', 0.3, [layer('rubber', T('Rubber pad 20 mm (active 12 mm)', 'Gummikappe 20 mm (wirksam 12 mm)'), 'polymer', 'rubber-black', 0.02, 0.25, 1100, 1900, 'authored-estimate', 'iso-10456-2007')], 'authored-estimate', 'incropera-2018'),
]
RID = [r['id'] for r in regions]
SUNO = {'front-rails': 0.5, 'treads': 0.4, 'platform': 1.0, 'rear-frame': 0.3, 'spreaders': 0.2, 'feet': 0.0}
occ = load('occupant', T('Person standing on the platform (100 kg)', 'Person auf der Plattform (100 kg)'), 'occupant', 0, -981, 'authored-estimate', 'en-131-2', anchor='standing-point')
thr = load('lateral-thrust', T('Lateral thrust of work above the head', 'Seitenschub durch Arbeit über Kopf'), 'live', 80, 0, 'authored-estimate', 'en-131-2', anchor='standing-point')
FEET = ['foot-front', 'foot-rear']
test = load('test-load', T('EN 131 test load of 150 kg', 'EN-131-Prüflast von 150 kg'), 'occupant', 0, -1471, 'literature', 'en-131-2', anchor='standing-point')
tthr = load('test-thrust', T('Lateral test thrust of 150 N', 'Seitliche Prüfkraft von 150 N'), 'live', 150, 0, 'authored-estimate', 'en-131-2', anchor='standing-point')
poses = [
    pose('open-empty', T('Open, empty', 'Aufgeklappt, leer'), T('Opened and locked, standing on four feet, no one on it, the sun on the platform.', 'Aufgeklappt und gesperrt, auf vier Füßen stehend, niemand darauf, die Sonne auf der Plattform.'), {}, FEET, bind(RID, sunmap=SUNO)),
    pose('open-in-use', T('Open, in use', 'Aufgeklappt, in Gebrauch'), T('A 100 kg person stands on the platform (981 N) and pushes 80 N sideways while working overhead; the rated load class is 150 kg.', 'Eine 100-kg-Person steht auf der Plattform (981 N) und drückt bei Überkopfarbeit mit 80 N seitlich; die Nennlastklasse beträgt 150 kg.'), {}, FEET, bind(RID, sunmap=SUNO), [occ, thr]),
    pose('rated-load-test', T('Rated load test', 'Nennlastprüfung'), T('The 150 kg rated load (1471 N) on the platform with a 150 N lateral thrust, the test case of EN 131-2.', 'Die Nennlast von 150 kg (1471 N) auf der Plattform mit 150 N Seitenschub, der Prüffall der EN 131-2.'), {}, FEET, bind(RID, sunmap=SUNO), [test, tthr]),
]
clips = [
    clip('climb-and-work', T('Climb and work', 'Besteigen und Arbeiten'), T('A person steps onto the platform, works overhead and steps down: the standing load and the lateral thrust rise and fall.', 'Eine Person steigt auf die Plattform, arbeitet über Kopf und steigt wieder ab: Standlast und Seitenschub steigen und fallen.'),
         12, 10, False, [(0, 'open-in-use', 'smooth'), (12, 'open-in-use', 'smooth')], [('foot-front', 0, 12), ('foot-rear', 0, 12)],
         [('climb', 0, 4, T('Climbing', 'Aufsteigen')), ('work', 4, 9, T('Working on the platform', 'Arbeit auf der Plattform')), ('descent', 9, 12, T('Descending', 'Absteigen'))],
         [(2, 'load-on', T('Person steps onto the platform', 'Person betritt die Plattform'), 'occupant'), (10, 'load-off', T('Person steps off', 'Person steigt ab'), 'occupant')],
         [('load:occupant', 'smooth', [(0, 0), (2, 0), (4, 1), (9, 1), (10.5, 0), (12, 0)]), ('load:lateral-thrust', 'smooth', [(0, 0), (4, 0), (5, 1), (8, 1), (9, 0), (12, 0)])]),
    clip('rated-load-ramp', T('Rated load ramp', 'Nennlast-Rampe'), T('The test load rises from zero to 150 kg in ten seconds and the lateral thrust follows: a quasi-static EN 131-2 style load ramp.', 'Die Prüflast steigt in zehn Sekunden von null auf 150 kg und der Seitenschub folgt: eine quasistatische Lastrampe im Stil der EN 131-2.'),
         20, 10, False, [(0, 'rated-load-test', 'smooth'), (20, 'rated-load-test', 'smooth')], [('foot-front', 0, 20), ('foot-rear', 0, 20)],
         [('ramp', 0, 10, T('Load rises', 'Last steigt')), ('hold', 10, 16, T('Load held', 'Last gehalten')), ('release', 16, 20, T('Load released', 'Last entlastet'))],
         [(0, 'load-on', T('Test load starts to rise', 'Prüflast beginnt zu steigen'), 'test-load'), (16, 'load-off', T('Test load released', 'Prüflast entlastet'), 'test-load')],
         [('load:test-load', 'smooth', [(0, 0), (10, 1), (16, 1), (20, 0)]), ('load:test-thrust', 'smooth', [(0, 0), (6, 0), (10, 1), (16, 1), (19, 0), (20, 0)])]),
]
asset = {
    'id': 'stepladder', 'category': 'objects',
    'title': T('Aluminium Stepladder', 'Aluminium-Stehleiter'),
    'summary': T('A five-step aluminium stepladder with platform, 1.50 m high and 0.68 m across the feet, open and locked: empty, under a person standing on it and under the EN 131 rated test load, with the standing load and lateral thrust as loads.', 'Eine fünfstufige Aluminium-Stehleiter mit Plattform, 1,50 m hoch und 0,68 m über den Füßen, aufgeklappt und gesperrt: leer, unter einer stehenden Person und unter der EN-131-Nennprüflast, mit Standlast und Seitenschub als Lasten.'),
    'tags': ['stepladder', 'ladder', 'aluminium', 'tool', 'standing-load'],
    'provenance': {'author': 'Semio staffage content (NW-O)', 'method': 'handcrafted', 'created': '2026-10-29', 'references': refs,
                   'notes': T('Side elevation of the open A-frame: front frame with four treads (pitch 0.25 m) and a platform at 1.25 m, rear frame hinged under the platform and locked by the spreaders. Dimensions and mass (5.3 kg) are typical of the 5-step class, authored estimates; the person is a load (100 kg), not a second asset. The folded state is NOT authored: in a folded pose the nearly parallel rails, treads and spreader links crowd into gaps below the print profile at every scale, and a folded rear foot would sink 0.05 m into the floor (rear rail 4 % longer than the front rail below the hinge); the hinge is therefore a fixed joint here.', 'Seitenansicht des aufgeklappten A-Gestells: Vorderholm mit vier Stufen (Teilung 0,25 m) und Plattform auf 1,25 m, Hinterholm unter der Plattform angelenkt und durch die Spreizen gesperrt. Maße und Masse (5,3 kg) sind für die 5-Stufen-Klasse typische Schätzwerte; die Person ist eine Last (100 kg), kein zweites Asset. Der zusammengeklappte Zustand ist NICHT verfasst: in einer zusammengeklappten Pose drängen sich die nahezu parallelen Holme, Stufen und Spreizenglieder in Abstände unter dem Druckprofil bei jedem Maßstab, und ein zusammengeklappter Hinterfuß sänke 0,05 m in den Boden ein (Hinterholm 4 % länger als der Vorderholm unter dem Gelenk); das Gelenk ist daher hier ein festes Gelenk.')},
    'dimensions': {'heightM': 1.50, 'widthM': 0.68, 'depthM': 0.46, 'massKg': round(rig.total_mass(), 2)},
    'materials': mats, 'rig': rig.rig(), 'anchors': anchors,
    'representations': {'500': r500(), '200': r200(), '100': r100(), '50': r50()}, 'symbols': [],
    'poses': poses, 'clips': clips,
    'thermal': {'environment': environment(18, 22, 0.5, 0.8, 600, 0.2, sky=8, ground=22), 'operating': [], 'passengers': [], 'regions': regions, 'emitters': [],
                'assumptions': [T('All parts are inert passive aluminium, zinc-plated steel or rubber: solar gain, radiation, convection; no internal heat. Bare aluminium has low emissivity and absorptivity.', 'Alle Teile sind inerte passive Flächen aus Aluminium, verzinktem Stahl oder Gummi: Solargewinn, Strahlung, Konvektion; keine innere Wärme. Blankes Aluminium hat niedrigen Emissions- und Absorptionsgrad.'),
                                T('The person standing on the ladder is a load only; the thermal exchange with the feet is neglected.', 'Die stehende Person ist nur eine Last; der Wärmeaustausch mit den Füßen wird vernachlässigt.')]},
    'structure': structure(FEET, [('A rigid body on elastic contacts: the four foot pads (two in the plane).', 'Starrer Körper auf elastischen Kontakten: die vier Fußkappen (zwei in der Ebene).'),
                                  ('Rated load 150 kg (EN 131-2); the standing load of 100 kg and the 80 N lateral thrust are authored estimates.', 'Nennlast 150 kg (EN 131-2); die Standlast von 100 kg und der Seitenschub von 80 N sind Schätzwerte.')]),
    'presentation': {'defaultPose': 'open-empty', 'defaultClip': 'climb-and-work', 'defaultScale': 100, 'viewpoints': ['elevation']},
    'exports': ENV_EXPORTS, 'completion': 'authored',
}
save(asset)
print('mass', rig.total_mass())
