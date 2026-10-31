import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
from geo import *

LIT = 'authored-estimate'
refs = [
    ref('iso-10456-2007', 'ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel, polymers, rubber).'),
    ref('incropera-2018', 'Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties of zinc-plated steel and polymers, cylinder convection).'),
    ref('en-1991-1-1', 'EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1 (self-weight and imposed loads; mass of carried goods is an authored estimate here).'),
    ref('trolley-class', 'Typical dimensions and mass of 100 litre wire-basket shopping trolleys with 125 mm wheels from manufacturer datasheets of the product class (length about 1.0 m, height 1.0 m, width 0.57 m, mass 12-15 kg); authored-estimate summary, no single datasheet.'),
]
mats = [
    material('zinc-wire', 'Zinc-plated steel wire', 'Verzinkter Stahldraht', 0.25, 0.55, 7800, 50, 450, 'literature', 'iso-10456-2007', 210e9),
    material('zinc-tube', 'Zinc-plated steel tube', 'Verzinktes Stahlrohr', 0.25, 0.55, 7800, 50, 450, 'literature', 'iso-10456-2007', 210e9),
    material('pp-grip', 'Polypropylene grip', 'Polypropylen-Griff', 0.9, 0.7, 910, 0.22, 1900, 'literature', 'iso-10456-2007'),
    material('pu-tyre', 'Polyurethane tyre with nylon hub', 'Polyurethanreifen mit Nylonnabe', 0.92, 0.9, 1200, 0.25, 1800, 'authored-estimate', 'iso-10456-2007'),
]
rig = Rig('mechanical', 'chassis')
rig.bone('chassis', None, 0, 0, 0.7, FIXED, T('Chassis with castors and wheels', 'Fahrgestell mit Lenkrollen und Rädern'), (5.5, 0.02, 0.15, LIT, 'trolley-class'))
rig.bone('basket', 'chassis', 0, 0.38, 0.92, FIXED, T('Wire basket', 'Drahtkorb'), (6.0, 0.05, 0.62, LIT, 'trolley-class'))
rig.bone('handle', 'basket', -0.40, 0.88, 0.2, FIXED, T('Push handle', 'Schiebegriff'), (1.2, -0.45, 0.97, LIT, 'trolley-class'))
rig.bone('gate', 'basket', -0.42, 0.88, 0.495, revolute(0, 80), T('Rear nesting gate', 'Hintere Nesting-Klappe'), (1.3, -0.36, 0.64, LIT, 'trolley-class'))
anchors = [
    rig.anchor('wheel-rear', 'chassis', -0.30, 0.0, 'contact', T('Rear wheel patch', 'Aufstandsfläche hinteres Rad'), support='contact', footprintLengthM=0.03, footprintWidthM=0.03, frictionCoefficient=0.7),
    rig.anchor('wheel-front', 'chassis', 0.33, 0.0, 'contact', T('Front castor patch', 'Aufstandsfläche vordere Lenkrolle'), support='contact', footprintLengthM=0.03, footprintWidthM=0.03, frictionCoefficient=0.7),
    rig.anchor('basket-centre', 'basket', 0.02, 0.42, 'load', T('Centre of the basket floor', 'Mitte des Korbbodens')),
]
BASK = [(-0.42, 0.88), (0.50, 0.88), (0.34, 0.38), (-0.30, 0.38)]
WR = 0.0625


def wheels(s, filled=True, hub=False, pen=None):
    for nm, x in (('rear', -0.30), ('front', 0.33)):
        s.ell(f'wheel-{nm}', 'chassis', x, WR, WR, z=2.5, tone=0.15, region='wheels', material='pu-tyre')
        if hub:
            s.ell(f'hub-{nm}', 'chassis', x, WR, 0.022, z=2.6, tone=0.7, role='detail', region='wheels')


def mesh(s, ys, xs=None, role='detail', z=3.2, region='basket-wire'):
    for i, y in enumerate(ys):
        xl = -0.30 + (-0.12) * (y - 0.38) / 0.5
        xr = 0.34 + 0.16 * (y - 0.38) / 0.5
        s.line(f'wire-h-{i}', 'basket', [(xl, y), (xr, y)], role=role, z=z, region=region)
    for i, x0 in enumerate(xs or []):
        s.line(f'wire-v-{i}', 'basket', [(x0, 0.38), (x0 + (x0 + 0.0 if False else 0) + 0.0, 0.88)] if False else [(x0, 0.38), (x0 + 0.1 * (x0 / 0.4), 0.88)], role=role, z=z, region=region)


def r500():
    s = Shapes(rig)
    s.poly('basket', 'basket', BASK, z=3, tone=0.15, regions=['basket-wire', 'gate-wire'], material='zinc-wire')
    s.line('chassis-line', 'chassis', [(-0.32, 0.17), (0.36, 0.17)], z=2.6, region='frame-tube', pen=0.13)
    s.line('struts', 'chassis', [(-0.30, 0.38), (-0.32, 0.17), (-0.30, 0.0625)], z=2.6, region='frame-tube', pen=0.13)
    s.line('front-strut', 'chassis', [(0.34, 0.38), (0.36, 0.17), (0.33, 0.0625)], z=2.6, region='frame-tube', pen=0.13)
    s.line('handle', 'handle', [(-0.55, 0.99), (-0.38, 0.99), (-0.40, 0.88)], z=3.5, region='handle-grip', pen=0.18)
    s.ell('wheel-rear', 'chassis', -0.30, 0.075, 0.075, z=2.5, tone=0.15, region='wheels', material='pu-tyre')
    s.ell('wheel-front', 'chassis', 0.33, 0.075, 0.075, z=2.5, tone=0.15, region='wheels', material='pu-tyre')
    return rep(500, 350, 1000, pens(0.13, None, None, None, 0.18), s, [
        ('A trolley sign 2.1 mm long and 2 mm high: a light trapezoid basket on a chassis line with two dots as wheels and a hooked handle stroke at the rear.', 'Ein Wagenzeichen, 2,1 mm lang und 2 mm hoch: ein heller Trapezkorb auf einer Fahrgestelllinie mit zwei Punkten als Rädern und einem gekröpften Griffstrich hinten.'),
        ('Basket outline at real size and taper, chassis line, both strut strokes, handle height 1.0 m, two wheels.', 'Korbumriss in realer Größe und Verjüngung, Fahrgestelllinie, beide Strebenstriche, Griffhöhe 1,0 m, zwei Räder.'),
        ('Basket wires and the rear gate are one light solid (the gate is edge-on at the basket back).', 'Korbdrähte und hintere Klappe sind ein heller Körper (die Klappe steht hinten am Korb kantenparallel).'),
        ('All wire mesh, rim, castor forks, grip, hinge.', 'Alles Drahtgitter, Rand, Rollengabeln, Griff, Gelenk.'),
        ('Wires (4 mm) and tubes (20 mm) are 0.13 mm strokes; wheels are thickened to 0.15 m diameter (0.3 mm) from 0.125 m; the handle is a 0.18 mm stroke.', 'Drähte (4 mm) und Rohre (20 mm) sind 0,13-mm-Striche; Räder sind von 0,125 m auf 0,15 m Durchmesser (0,3 mm) verdickt; der Griff ist ein 0,18-mm-Strich.'),
        ('None.', 'Keine.'),
        ('Basket tone 0.15 (open mesh reads light), wheels 0.15; strokes 0.13 mm, ground 0.18 mm.', 'Korb Ton 0,15 (offenes Gitter wirkt hell), Räder 0,15; Striche 0,13 mm, Boden 0,18 mm.'),
        ('Basket solid lists basket-wire and gate-wire; the strokes carry frame-tube and handle-grip.', 'Der Korbkörper nennt basket-wire und gate-wire; die Striche tragen frame-tube und handle-grip.'),
    ])


def r200():
    s = Shapes(rig)
    s.poly('basket', 'basket', BASK, z=3, tone=0.12, region='basket-wire', material='zinc-wire')
    mesh(s, (0.54, 0.70), role='detail')
    s.line('gate', 'gate', [(-0.42, 0.88), (-0.30, 0.40)], z=3.4, region='gate-wire', pen=0.25)
    s.line('chassis-line', 'chassis', [(-0.32, 0.17), (0.36, 0.17)], z=2.6, region='frame-tube', pen=0.18)
    s.line('rear-fork', 'chassis', [(-0.30, 0.38), (-0.32, 0.17), (-0.30, 0.0625)], z=2.6, region='frame-tube', pen=0.18)
    s.line('front-fork', 'chassis', [(0.34, 0.38), (0.36, 0.17), (0.33, 0.0625)], z=2.6, region='frame-tube', pen=0.18)
    s.line('handle-post', 'handle', [(-0.38, 0.98), (-0.40, 0.88)], z=3.5, region='frame-tube', pen=0.18)
    s.rect('grip', 'handle', -0.55, 0.96, -0.36, 1.0, z=3.6, tone=0.7, region='handle-grip', material='pp-grip')
    wheels(s)
    return rep(200, 150, 350, pens(0.18, 0.13, None, None, 0.25), s, [
        ('A trolley 5.3 mm long and 5 mm high: light tapered basket with two wire lines, a rear gate stroke, chassis line with fork strokes, a grip bar on a post and two wheel discs.', 'Ein Wagen 5,3 mm lang und 5 mm hoch: heller verjüngter Korb mit zwei Drahtlinien, ein Klappenstrich hinten, Fahrgestelllinie mit Gabelstrichen, ein Griffbalken auf einer Stütze und zwei Radscheiben.'),
        ('Basket taper, two of the seven wire levels, gate, chassis, both forks, handle post and grip, both wheels at 0.125 m.', 'Korbverjüngung, zwei der sieben Drahtebenen, Klappe, Fahrgestell, beide Gabeln, Griffstütze und Griff, beide Räder mit 0,125 m.'),
        ('The paired side wire frames coincide in elevation; verticals are merged into the fill.', 'Die paarigen seitlichen Drahtrahmen fallen in der Ansicht zusammen; Senkrechte gehen in der Füllung auf.'),
        ('Vertical wires, rim double, castor plates, hubs.', 'Senkrechte Drähte, Randdopplung, Rollenplatten, Naben.'),
        ('The gate (wire panel edge-on) is a 0.25 mm stroke, the chassis tube a 0.18 mm stroke; the grip is a 0.04 m (0.2 mm) bar.', 'Die Klappe (Drahtfläche kantenparallel) ist ein 0,25-mm-Strich, das Fahrgestellrohr ein 0,18-mm-Strich; der Griff ist ein Balken von 0,04 m (0,2 mm).'),
        ('None.', 'Keine.'),
        ('Basket 0.12, wheels 0.15, grip 0.7; body pen 0.18 mm, detail 0.13 mm.', 'Korb 0,12, Räder 0,15, Griff 0,7; Körperstift 0,18 mm, Detail 0,13 mm.'),
        ('Wire lines carry basket-wire; the gate stroke gate-wire; forks and post frame-tube; the grip handle-grip; wheels wheels.', 'Drahtlinien tragen basket-wire; der Klappenstrich gate-wire; Gabeln und Stütze frame-tube; der Griff handle-grip; Räder wheels.'),
    ])


def r100():
    s = Shapes(rig)
    s.poly('basket', 'basket', BASK, z=3, tone=0.1, region='basket-wire', material='zinc-wire')
    mesh(s, (0.48, 0.58, 0.68, 0.78), xs=(-0.12, 0.06, 0.24), role='detail')
    s.line('rim', 'basket', [(-0.42, 0.865), (0.50, 0.865)], role='detail', z=3.3, region='basket-wire')
    s.poly('gate', 'gate', band((-0.42, 0.88), (-0.30, 0.40), 0.03), z=3.4, tone=0.5, region='gate-wire', material='zinc-wire')
    s.line('chassis-line', 'chassis', [(-0.32, 0.17), (0.36, 0.17)], z=2.6, region='frame-tube', pen=0.25)
    s.line('rear-fork', 'chassis', [(-0.30, 0.38), (-0.32, 0.17), (-0.30, 0.0625)], z=2.6, region='frame-tube', pen=0.25)
    s.line('front-fork', 'chassis', [(0.34, 0.38), (0.36, 0.17), (0.33, 0.0625)], z=2.6, region='frame-tube', pen=0.25)
    s.line('handle-post', 'handle', [(-0.38, 0.97), (-0.40, 0.88)], z=3.5, region='frame-tube', pen=0.25)
    s.rect('grip', 'handle', -0.55, 0.96, -0.36, 1.0, r=0.012, z=3.6, tone=0.7, region='handle-grip', material='pp-grip')
    wheels(s, hub=True)
    return rep(100, 75, 150, pens(0.25, 0.13, None, None, 0.35), s, [
        ('A trolley 10.5 mm long and 10 mm high: wire basket with a grid of four horizontals and three verticals, rim wire, rear gate band, chassis tube with forks, grip with post, two wheels with hubs.', 'Ein Wagen 10,5 mm lang und 10 mm hoch: Drahtkorb mit einem Gitter aus vier Waagrechten und drei Senkrechten, Randdraht, hinteres Klappenband, Fahrgestellrohr mit Gabeln, Griff mit Stütze, zwei Räder mit Naben.'),
        ('Basket, grid, rim, gate, chassis, forks, handle, wheels with hubs.', 'Korb, Gitter, Rand, Klappe, Fahrgestell, Gabeln, Griff, Räder mit Naben.'),
        ('The twelve real horizontal wires per side are drawn as four lines (spacing 1 mm); the far side frame coincides.', 'Die zwölf realen Waagrechtdrähte je Seite sind als vier Linien gezeichnet (Abstand 1 mm); der ferne Seitenrahmen fällt zusammen.'),
        ('Castor plates, wire bends, hinge eyes.', 'Rollenplatten, Drahtbögen, Gelenkösen.'),
        ('Wires are 0.13 mm lines (real 4 mm = 0.04 mm); the gate is a 0.03 m (0.3 mm) band; tubes are 0.25 mm strokes.', 'Drähte sind 0,13-mm-Linien (real 4 mm = 0,04 mm); die Klappe ist ein 0,03-m-Band (0,3 mm); Rohre sind 0,25-mm-Striche.'),
        ('None; the mesh is drawn as lines.', 'Keine; das Gitter ist als Linien gezeichnet.'),
        ('Basket 0.1, wheels 0.15, gate 0.5, grip 0.7; body pen 0.25 mm, detail 0.13 mm.', 'Korb 0,1, Räder 0,15, Klappe 0,5, Griff 0,7; Körperstift 0,25 mm, Detail 0,13 mm.'),
        ('Grid and rim carry basket-wire; the thermal view paints the open mesh as a light basket.', 'Gitter und Rand tragen basket-wire; die Thermalansicht färbt das offene Gitter als hellen Korb.'),
    ])


def r50():
    s = Shapes(rig)
    s.poly('basket', 'basket', BASK, z=3, tone=0.08, region='basket-wire', material='zinc-wire')
    mesh(s, (0.44, 0.50, 0.56, 0.62, 0.68, 0.74, 0.80, 0.86), xs=(-0.20, -0.08, 0.04, 0.16, 0.28), role='texture')
    s.line('rim', 'basket', [(-0.42, 0.865), (0.50, 0.865)], role='detail', z=3.3, region='basket-wire')
    s.line('floor-wire', 'basket', [(-0.30, 0.395), (0.34, 0.395)], role='detail', z=3.3, region='basket-wire')
    s.poly('gate', 'gate', band((-0.42, 0.88), (-0.30, 0.40), 0.03), z=3.4, tone=0.5, region='gate-wire', material='zinc-wire')
    s.ell('gate-eye', 'gate', -0.42, 0.88, 0.014, z=3.6, tone=0.9, role='detail', region='gate-wire')
    s.line('chassis-line', 'chassis', [(-0.32, 0.17), (0.36, 0.17)], z=2.6, region='frame-tube', pen=0.35)
    s.line('rear-fork', 'chassis', [(-0.30, 0.38), (-0.32, 0.17), (-0.30, 0.0625)], z=2.6, region='frame-tube', pen=0.35)
    s.line('front-fork', 'chassis', [(0.34, 0.38), (0.36, 0.17), (0.33, 0.0625)], z=2.6, region='frame-tube', pen=0.35)
    s.poly('castor-plate', 'chassis', [(0.335, 0.18), (0.385, 0.18), (0.385, 0.165), (0.335, 0.165)], z=2.7, tone=0.6, region='frame-tube', material='zinc-tube')
    s.line('handle-post', 'handle', [(-0.38, 0.97), (-0.40, 0.88)], z=3.5, region='frame-tube', pen=0.35)
    s.rect('grip', 'handle', -0.55, 0.96, -0.36, 1.0, r=0.012, z=3.6, tone=0.7, region='handle-grip', material='pp-grip', hatch='grip-ribs')
    wheels(s, hub=True)
    for nm, x in (('rear', -0.30), ('front', 0.33)):
        s.ell(f'tread-{nm}', 'chassis', x, WR, 0.047, z=2.55, tone=0.3, role='texture', region='wheels', filled=False)
    return rep(50, 20, 75, pens(0.5, 0.25, 0.13, None, 0.7), s, [
        ('A trolley 21 mm long and 20 mm high in construction detail: wire basket with a mesh of eight horizontals and five verticals, rim and floor wire, gate band with hinge eye, chassis tube, forks, castor plate, ribbed grip, two wheels with hubs and tread ring.', 'Ein Wagen 21 mm lang und 20 mm hoch im Konstruktionsdetail: Drahtkorb mit einem Gitter aus acht Waagrechten und fünf Senkrechten, Rand- und Bodendraht, Klappenband mit Gelenköse, Fahrgestellrohr, Gabeln, Rollenplatte, gerippter Griff, zwei Räder mit Naben und Laufring.'),
        ('All visible tubes, wires and the gate at real positions; mesh spacing 0.06 m horizontal, 0.12 m vertical.', 'Alle sichtbaren Rohre, Drähte und die Klappe in realen Lagen; Gitterabstand 0,06 m waagrecht, 0,12 m senkrecht.'),
        ('The far-side frame coincides with the near side; the second front castor is hidden behind the first.', 'Der Rahmen der fernen Seite fällt mit der nahen zusammen; die zweite vordere Rolle ist hinter der ersten verdeckt.'),
        ('Wire cross welds, brake lever, coin lock.', 'Drahtkreuzschweißungen, Bremshebel, Pfandschloss.'),
        ('Mesh lines are 0.13 mm (real wire 4 mm); the tread ring is a 0.13 mm ellipse; the hinge eye a 0.028 m (0.56 mm) circle.', 'Gitterlinien sind 0,13 mm (realer Draht 4 mm); der Laufring ist eine 0,13-mm-Ellipse; die Gelenköse ein Kreis von 0,028 m (0,56 mm).'),
        ('Grip ribs as 0.5 mm spaced lines at 90 degrees.', 'Griffrippen als Linien im Abstand 0,5 mm unter 90 Grad.'),
        ('Basket 0.08, wheels 0.15, gate 0.5, grip 0.7; body pen 0.5 mm, detail 0.25 mm, texture 0.13 mm.', 'Korb 0,08, Räder 0,15, Klappe 0,5, Griff 0,7; Körperstift 0,5 mm, Detail 0,25 mm, Textur 0,13 mm.'),
        ('Mesh, rim and floor wires carry basket-wire; the tread rings carry wheels.', 'Gitter-, Rand- und Bodendrähte tragen basket-wire; die Laufringe tragen wheels.'),
    ], [hatch('grip-ribs', 'Grip ribs', 'Griffrippen', 'lines', 90, 0.5, 0.13)])


W = lambda lid, t, k, m, th, kk, rho, cp: layer(lid, T(*t), k, m, th, kk, rho, cp, 'authored-estimate', 'iso-10456-2007')
regions = [
    region('basket-wire', T('Basket wire mesh', 'Korbdrahtgitter'), 'surface', 'passive', 'zinc-wire', 0.57, 0.004, 'cylinder', 0.5, [W('wire', ('Zinc-plated steel wire 4 mm (equivalent skin 1 mm)', 'Verzinkter Stahldraht 4 mm (Ersatzhaut 1 mm)'), 'metal', 'zinc-wire', 0.001, 50, 7800, 450)], 'authored-estimate', 'incropera-2018'),
    region('gate-wire', T('Rear nesting gate', 'Hintere Nesting-Klappe'), 'surface', 'passive', 'zinc-wire', 0.12, 0.004, 'vertical-plate', 0.5, [W('wire', ('Zinc-plated steel wire 4 mm (equivalent skin 1 mm)', 'Verzinkter Stahldraht 4 mm (Ersatzhaut 1 mm)'), 'metal', 'zinc-wire', 0.001, 50, 7800, 450)], 'authored-estimate', 'incropera-2018'),
    region('frame-tube', T('Chassis and handle tubes', 'Fahrgestell- und Griffrohre'), 'surface', 'passive', 'zinc-tube', 0.28, 0.02, 'cylinder', 0.4, [W('tube', ('Zinc-plated steel tube wall 1.5 mm', 'Verzinkte Stahlrohrwand 1,5 mm'), 'metal', 'zinc-tube', 0.0015, 50, 7800, 450)], 'authored-estimate', 'incropera-2018'),
    region('handle-grip', T('Polypropylene grip', 'Polypropylen-Griff'), 'surface', 'passive', 'pp-grip', 0.04, 0.03, 'cylinder', 0.6, [W('pp', ('Polypropylene sleeve 3 mm', 'Polypropylen-Hülse 3 mm'), 'polymer', 'pp-grip', 0.003, 0.22, 910, 1900)], 'authored-estimate', 'iso-10456-2007'),
    region('wheels', T('Wheels and castors', 'Räder und Lenkrollen'), 'tyre', 'passive', 'pu-tyre', 0.08, 0.125, 'cylinder', 0.3, [W('tyre', ('Polyurethane tyre 15 mm (active 8 mm)', 'Polyurethanreifen 15 mm (wirksam 8 mm)'), 'polymer', 'pu-tyre', 0.015, 0.25, 1200, 1800)], 'authored-estimate', 'iso-10456-2007'),
]
RID = [r['id'] for r in regions]
SUNP = {'basket-wire': 0.4, 'gate-wire': 0.4, 'frame-tube': 0.3, 'handle-grip': 1.0, 'wheels': 0.0}
groc = load('groceries', T('Groceries in the basket (30 kg)', 'Einkäufe im Korb (30 kg)'), 'carried', 0, -294, 'authored-estimate', 'trolley-class', anchor='basket-centre')
push = load('push', T('Hand push on the handle', 'Handschub am Griff'), 'live', 40, 0, 'authored-estimate', 'trolley-class', bone='handle', x=-0.15, y=0.095)
W2 = ['wheel-rear', 'wheel-front']
poses = [
    pose('empty', T('Empty', 'Leer'), T('Empty trolley in the car park sun, gate hanging closed.', 'Leerer Einkaufswagen in der Parkplatzsonne, Klappe geschlossen.'), {}, W2, bind(RID, sunmap=SUNP)),
    pose('loaded', T('Loaded', 'Beladen'), T('Thirty kilograms of groceries on the basket floor and a 40 N push on the handle.', 'Dreißig Kilogramm Einkäufe auf dem Korbboden und ein Schub von 40 N am Griff.'), {}, W2, bind(RID, sunmap=SUNP), [groc, push]),
    pose('gate-raised', T('Gate raised for nesting', 'Klappe zum Ineinanderschieben angehoben'), T('The rear gate swung 65 degrees up and inward as when the nose of another trolley pushes in; the other trolley is not drawn.', 'Die hintere Klappe ist um 65 Grad nach oben und innen geschwenkt, wie wenn die Spitze eines anderen Wagens hineingeschoben wird; der andere Wagen ist nicht gezeichnet.'), {'gate': {'angleDeg': 65}}, W2, bind(RID, sunmap=SUNP, exposed={'gate-wire': 0.6})),
]
clips = [
    clip('load-groceries', T('Load groceries', 'Einkäufe einladen'), T('Groceries are put into the basket one after another and the loading load rises to 30 kg while the shopper starts to push.', 'Einkäufe werden nacheinander in den Korb gelegt, die Last steigt auf 30 kg, während der Einkaufende zu schieben beginnt.'),
         16, 10, False, [(0, 'loaded', 'smooth'), (16, 'loaded', 'smooth')], [('wheel-rear', 0, 16), ('wheel-front', 0, 16)],
         [('loading', 0, 10, T('Loading', 'Einladen')), ('pushing', 10, 16, T('Starting to push', 'Anschieben'))],
         [(1, 'load-on', T('First goods in the basket', 'Erste Ware im Korb'), 'groceries'), (10, 'load-on', T('Hands on the handle', 'Hände am Griff'), 'push')],
         [('load:groceries', 'smooth', [(0, 0), (1, 0.1), (5, 0.45), (9, 1), (16, 1)]), ('load:push', 'smooth', [(0, 0), (10, 0), (12, 1), (16, 1)])]),
    clip('nesting-swing', T('Gate swings for nesting', 'Klappe schwingt beim Ineinanderschieben'), T('A 12 s loop: the rear gate swings up as another trolley is pushed in and drops back when it is pulled out.', 'Eine 12-s-Schleife: die hintere Klappe schwingt auf, wenn ein anderer Wagen hineingeschoben wird, und fällt zurück, wenn er herausgezogen wird.'),
         12, 12, True, [(0, 'empty', 'smooth'), (4, 'gate-raised', 'smooth'), (8, 'gate-raised', 'smooth'), (12, 'empty', 'smooth')], [('wheel-rear', 0, 12), ('wheel-front', 0, 12)],
         [('nest-in', 0, 4, T('Another trolley pushes in', 'Ein anderer Wagen wird hineingeschoben')), ('nested', 4, 8, T('Nested', 'Ineinandergeschoben')), ('nest-out', 8, 12, T('Pulled out', 'Herausgezogen'))],
         [(4, 'motion', T('Gate fully raised', 'Klappe ganz oben'), None), (8, 'motion', T('Gate starts to fall', 'Klappe beginnt zu fallen'), None)], []),
]
asset = {
    'id': 'shopping-trolley', 'category': 'objects',
    'title': T('Shopping Trolley', 'Einkaufswagen'),
    'summary': T('A 100 litre zinc-plated wire-basket trolley on four castors, 1.05 m long and 1.0 m high: empty, loaded with 30 kg of groceries, and with the rear nesting gate swung up; a second trolley is not drawn.', 'Ein 100-Liter-Einkaufswagen aus verzinktem Draht auf vier Rollen, 1,05 m lang und 1,0 m hoch: leer, mit 30 kg Einkäufen beladen und mit hochgeschwenkter hinterer Nesting-Klappe; ein zweiter Wagen wird nicht gezeichnet.'),
    'tags': ['shopping-trolley', 'wire-basket', 'castors', 'retail', 'nesting', 'carried-load'],
    'provenance': {'author': 'Semio staffage content (NW-O)', 'method': 'handcrafted', 'created': '2026-10-29', 'references': refs,
                   'notes': T('Side elevation, push direction to the right, handle at the left. Dimensions and the 14 kg mass are typical class values (authored estimates). The nested state is the rear gate swung up, as when another trolley nose pushes in; the second trolley is not drawn (it would be a hidden second asset). Castors are drawn static: a rolling clip is not authored because the planted root solver is not used for caster swivel.', 'Seitenansicht, Schieberichtung nach rechts, Griff links. Maße und die Masse von 14 kg sind typische Klassenwerte (Schätzwerte). Der Nesting-Zustand ist die hochgeschwenkte hintere Klappe, wie wenn die Spitze eines anderen Wagens hineingeschoben wird; der zweite Wagen wird nicht gezeichnet (er wäre ein verstecktes zweites Asset). Die Rollen sind statisch gezeichnet: ein Rollclip ist nicht verfasst, weil der Wurzellöser für das Schwenken der Lenkrollen nicht genutzt wird.')},
    'dimensions': {'heightM': 1.00, 'widthM': 1.05, 'depthM': 0.57, 'massKg': round(rig.total_mass(), 2)},
    'materials': mats, 'rig': rig.rig(), 'anchors': anchors,
    'representations': {'500': r500(), '200': r200(), '100': r100(), '50': r50()}, 'symbols': [],
    'poses': poses, 'clips': clips,
    'thermal': {'environment': environment(22, 30, 0.5, 1.5, 800, 0.15, sky=10, ground=38), 'operating': [], 'passengers': [], 'regions': regions, 'emitters': [],
                'assumptions': [T('All parts are inert passive surfaces; wire is modelled as a thin equivalent skin of 1 mm steel (heat capacity 3.5 kJ/m2K) with cylinder convection on the 4 mm wire.', 'Alle Teile sind inerte passive Flächen; der Draht wird als dünne Ersatzhaut aus 1 mm Stahl (Wärmekapazität 3,5 kJ/m2K) mit Zylinderkonvektion am 4-mm-Draht modelliert.'),
                                T('The groceries are a load only; their temperature is not modelled.', 'Die Einkäufe sind nur eine Last; ihre Temperatur wird nicht modelliert.')]},
    'structure': structure(W2, [('A rigid body on two wheel contacts in the plane (the paired wheels coincide in elevation); the gate is a revolute joint posed, not simulated.', 'Ein starrer Körper auf zwei Radkontakten in der Ebene (die paarigen Räder fallen in der Ansicht zusammen); die Klappe ist ein Drehgelenk, das gestellt, nicht simuliert wird.'),
                                ('Goods mass of 30 kg and the 40 N push are authored estimates.', 'Warenmasse von 30 kg und Schub von 40 N sind Schätzwerte.')]),
    'presentation': {'defaultPose': 'empty', 'defaultClip': 'load-groceries', 'defaultScale': 100, 'viewpoints': ['elevation']},
    'exports': ENV_EXPORTS, 'completion': 'authored',
}
save(asset)
print('mass', rig.total_mass())
