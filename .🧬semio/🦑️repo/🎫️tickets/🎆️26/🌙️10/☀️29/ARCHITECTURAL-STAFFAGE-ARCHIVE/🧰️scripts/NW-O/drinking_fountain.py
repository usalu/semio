import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

LIT = 'authored-estimate'
refs = [
    ref('en-1717', 'EN 1717:2000 Protection against pollution of potable water installations (free outlet above the basin rim, air gap) - rationale for the free-falling jet over an open basin.'),
    ref('iso-10456-2007', 'ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (cast iron, steel, brass).'),
    ref('incropera-2018', 'Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties, convective mass transfer from a wetted surface).'),
    ref('din-1988-200', 'DIN 1988-200 / DVGW W 551 Technical rules for drinking water installations: supply water temperature below 25 degC; the 12 degC mains temperature used here is an authored estimate for a buried supply line.'),
    ref('fountain-class', 'Typical cast-iron pedestal drinking fountains (bubbler flow about 2-4 L/min, mass 50-90 kg) from manufacturer datasheets of the product class; authored-estimate summary, no single datasheet.'),
]
mats = [
    material('cast-iron', 'Painted cast iron', 'Lackiertes Gusseisen', 0.92, 0.8, 7200, 50, 500, 'literature', 'iso-10456-2007', 100e9),
    material('stainless-basin', 'Brushed stainless steel basin', 'Edelstahlbecken gebürstet', 0.35, 0.45, 7900, 17, 500, 'literature', 'iso-10456-2007', 200e9),
    material('chrome-brass', 'Chrome-plated brass', 'Verchromtes Messing', 0.1, 0.35, 8500, 110, 380, 'authored-estimate', 'iso-10456-2007'),
    material('water', 'Drinking water', 'Trinkwasser', 0.96, 0.05, 1000, 0.6, 4180, 'literature', 'incropera-2018'),
]
rig = Rig('mechanical', 'pedestal')
rig.bone('pedestal', None, 0, 0, 0.74, FIXED, T('Cast pedestal with base flange', 'Gusssockel mit Fußflansch'), (62.3, 0, 0.30, LIT, 'fountain-class'))
rig.bone('basin', 'pedestal', 0, 0.74, 0.12, FIXED, T('Stainless basin', 'Edelstahlbecken'), (4.5, 0, 0.80, LIT, 'fountain-class'))
rig.bone('bubbler', 'basin', -0.15, 0.86, 0.14, FIXED, T('Goose-neck bubbler', 'Schwanenhals-Sprudler'), (1.0, -0.12, 0.95, LIT, 'fountain-class'))
rig.bone('button', 'pedestal', 0.102, 0.66, 0.02, prismatic(0, 0.01, 180), T('Push button', 'Drucktaster'), (0.2, 0.108, 0.66, LIT, 'fountain-class'))
rig.bone('jet', 'bubbler', -0.045, 0.955, 0.12, prismatic(-0.25, 0, 90), T('Water jet (sinks into the basin when off)', 'Wasserstrahl (versinkt im Becken, wenn aus)'))
anchors = [
    rig.anchor('base-fixed', 'pedestal', 0, 0, 'support', T('Base flange bolted to the footing', 'Fußflansch am Fundament verschraubt'), support='fixed', footprintLengthM=0.40, footprintWidthM=0.40),
    rig.anchor('nozzle', 'bubbler', -0.045, 0.955, 'emitter', T('Bubbler nozzle', 'Sprudlerdüse')),
    rig.anchor('lean-point', 'basin', 0.0, 0.865, 'load', T('Hands and chest at the basin rim', 'Hände und Brust am Beckenrand')),
]
COL = 'M -0.15 0.06 L -0.10 0.70 L -0.09 0.745 L 0.09 0.745 L 0.10 0.70 L 0.15 0.06 Z'
BOWL = 'M -0.20 0.865 C -0.20 0.79 -0.12 0.74 0 0.74 C 0.12 0.74 0.20 0.79 0.20 0.865 Z'
NECK = 'M -0.165 0.855 L -0.165 0.97 C -0.165 1.0 -0.14 1.0 -0.12 1.0 C -0.08 1.0 -0.05 0.99 -0.04 0.96 L -0.065 0.955 C -0.07 0.97 -0.09 0.975 -0.12 0.972 L -0.135 0.972 L -0.135 0.855 Z'
JET = 'M -0.045 0.955 C -0.02 1.00 0.05 1.00 0.085 0.87'


def jet(s, pen, z=2.6):
    s.path('jet', 'jet', JET, role='detail', z=z, filled=False, region='water-jet', pen=pen)


def r500():
    s = Shapes(rig)
    s.path('pedestal', 'pedestal', 'M -0.20 0 L 0.20 0 L 0.20 0.12 L 0.14 0.12 L 0.10 0.72 L -0.10 0.72 L -0.14 0.12 L -0.20 0.12 Z', z=3, tone=0.6, region='pedestal-cast', material='cast-iron')
    s.path('basin', 'basin', BOWL.replace('0.74 0 0.74', '0.72 0 0.72').replace('0.12 0.74', '0.12 0.72').replace('0.79', '0.78'), z=3.2, tone=0.5, region='basin-steel', material='stainless-basin')
    s.rect('bubbler', 'bubbler', -0.19, 0.86, -0.04, 1.0, z=3.3, tone=0.7, region='bubbler-brass', material='chrome-brass')
    s.rect('button', 'button', 0.09, 0.62, 0.115, 0.70, z=3.4, tone=0.8, region='bubbler-brass')
    jet(s, 0.13)
    return rep(500, 350, 1000, pens(0.13, None, None, None, 0.18), s, [
        ('A fountain sign 0.8 mm wide and 2 mm high: a flared pedestal with a basin disc and a bubbler block rising at the left.', 'Ein Brunnenzeichen, 0,8 mm breit und 2 mm hoch: ein ausgestellter Sockel mit Beckenscheibe und ein links aufsteigender Sprudlerblock.'),
        ('Height 1.0 m and basin width 0.40 m exactly; pedestal flare, basin, bubbler and the jet hairline.', 'Höhe 1,0 m und Beckenbreite 0,40 m exakt; Sockelausstellung, Becken, Sprudler und die Strahl-Haarlinie.'),
        ('Base flange and column are one flared path; the goose neck and its head are one block; the button is a small bar.', 'Fußflansch und Säule sind ein ausgestellter Pfad; Schwanenhals und Kopf sind ein Block; der Taster ist ein kleiner Balken.'),
        ('Column mouldings, rim bead, nozzle, drain.', 'Säulenprofile, Randwulst, Düse, Ablauf.'),
        ('The 0.03 m neck would be 0.06 mm: widened to a 0.15 m (0.3 mm) block; the 0.012 m jet is a 0.13 mm hairline (inflation 11 times); the basin is 0.02 m lower than real.', 'Der 0,03 m dicke Hals wäre 0,06 mm: auf einen Block von 0,15 m (0,3 mm) verbreitert; der 0,012 m dicke Strahl ist eine 0,13-mm-Haarlinie (Aufblähung 11-fach); das Becken liegt 0,02 m tiefer als real.'),
        ('None.', 'Keine.'),
        ('Pedestal 0.6, basin 0.5, bubbler 0.7; body pen 0.13 mm, ground 0.18 mm.', 'Sockel 0,6, Becken 0,5, Sprudler 0,7; Körperstift 0,13 mm, Boden 0,18 mm.'),
        ('Pedestal, basin and bubbler block carry their regions; the jet hairline carries water-jet and sinks behind the basin when the tap is off.', 'Sockel, Becken und Sprudlerblock tragen ihre Regionen; die Strahl-Haarlinie trägt water-jet und versinkt hinter dem Becken, wenn der Hahn aus ist.'),
    ])


def r200():
    s = Shapes(rig)
    s.path('pedestal', 'pedestal', 'M -0.20 0 L 0.20 0 L 0.20 0.07 L 0.15 0.07 L 0.10 0.70 L 0.09 0.745 L -0.09 0.745 L -0.10 0.70 L -0.15 0.07 L -0.20 0.07 Z', z=3, tone=0.6, region='pedestal-cast', material='cast-iron')
    s.path('basin', 'basin', BOWL, z=3.2, tone=0.5, region='basin-steel', material='stainless-basin')
    s.path('bubbler', 'bubbler', NECK.replace('-0.165', '-0.17').replace('-0.135', '-0.125'), z=3.3, tone=0.7, region='bubbler-brass', material='chrome-brass')
    s.rect('button', 'button', 0.09, 0.64, 0.115, 0.68, z=3.4, tone=0.8, region='bubbler-brass')
    s.line('collar', 'pedestal', [(-0.115, 0.55), (0.115, 0.55)], role='detail', z=3.5, region='pedestal-cast')
    jet(s, 0.13)
    return rep(200, 150, 350, pens(0.18, 0.13, None, None, 0.25), s, [
        ('A fountain 2 mm wide and 5 mm high: a baluster pedestal with a flange, a basin and a hooked bubbler neck with a jet arc.', 'Ein Brunnen 2 mm breit und 5 mm hoch: ein Baluster-Sockel mit Flansch, ein Becken und ein gekrümmter Sprudlerhals mit Strahlbogen.'),
        ('Flange, baluster profile, collar line, basin, hooked neck, button, jet.', 'Flansch, Balusterprofil, Kragenlinie, Becken, gekrümmter Hals, Taster, Strahl.'),
        ('The pedestal mouldings merge into the baluster outline.', 'Die Sockelprofile gehen in der Balusterkontur auf.'),
        ('Nozzle detail, rim bead, bolt holes, drain.', 'Düsendetail, Randwulst, Schraublöcher, Ablauf.'),
        ('The neck is 0.05 m (0.25 mm) wide, real 0.03 m; the jet is a 0.13 mm hairline for a 0.012 m stream.', 'Der Hals ist 0,05 m (0,25 mm) breit, real 0,03 m; der Strahl ist eine 0,13-mm-Haarlinie für einen 0,012 m dicken Strahl.'),
        ('None.', 'Keine.'),
        ('Pedestal 0.6, basin 0.5, bubbler 0.7; body pen 0.18 mm, detail 0.13 mm.', 'Sockel 0,6, Becken 0,5, Sprudler 0,7; Körperstift 0,18 mm, Detail 0,13 mm.'),
        ('The collar line carries pedestal-cast; the jet water-jet.', 'Die Kragenlinie trägt pedestal-cast; der Strahl water-jet.'),
    ])


def r100():
    s = Shapes(rig)
    s.path('pedestal', 'pedestal', 'M -0.20 0 L 0.20 0 L 0.20 0.05 L 0.17 0.06 L 0.15 0.08 L 0.10 0.70 L 0.09 0.745 L -0.09 0.745 L -0.10 0.70 L -0.15 0.08 L -0.17 0.06 L -0.20 0.05 Z', z=3, tone=0.6, region='pedestal-cast', material='cast-iron')
    s.path('basin', 'basin', BOWL, z=3.2, tone=0.5, region='basin-steel', material='stainless-basin')
    s.line('rim', 'basin', [(-0.19, 0.84), (0.19, 0.84)], role='detail', z=3.5, region='basin-steel')
    s.path('bubbler', 'bubbler', NECK.replace('-0.165', '-0.18').replace('-0.135', '-0.13'), z=3.3, tone=0.7, region='bubbler-brass', material='chrome-brass')
    s.ell('button', 'button', 0.108, 0.66, 0.02, 0.025, z=3.4, tone=0.8, region='bubbler-brass')
    for i, y in enumerate((0.30, 0.55)):
        s.line(f'collar-{i}', 'pedestal', [(-0.12 + 0.04 * (y - 0.06) / 0.64 * 0.0 + 0.0, y), (0.12, y)], role='detail', z=3.5, region='pedestal-cast')
    jet(s, 0.18)
    return rep(100, 75, 150, pens(0.25, 0.13, None, None, 0.35), s, [
        ('A fountain 4 mm wide and 10 mm high: a flanged baluster pedestal with two collar lines, a basin with rim line, a goose-neck bubbler with a round button.', 'Ein Brunnen 4 mm breit und 10 mm hoch: ein Baluster-Sockel mit Flansch und zwei Kragenlinien, ein Becken mit Randlinie, ein Schwanenhals-Sprudler mit rundem Taster.'),
        ('Stepped flange, baluster, collars, basin and rim, neck with head, button, jet arc.', 'Gestufter Flansch, Baluster, Kragen, Becken und Rand, Hals mit Kopf, Taster, Strahlbogen.'),
        ('The cast mouldings of the column are drawn as two collars.', 'Die gegossenen Profile der Säule sind als zwei Kragen gezeichnet.'),
        ('Nozzle detail, drain, flutes, bolt holes.', 'Düsendetail, Ablauf, Kannelüren, Schraublöcher.'),
        ('The neck is the real 0.03 m (0.3 mm) plus 0.015 m; the jet is a 0.18 mm line for a 0.012 m stream.', 'Der Hals ist die reale 0,03 m (0,3 mm) plus 0,015 m; der Strahl ist eine 0,18-mm-Linie für einen 0,012 m dicken Strahl.'),
        ('None.', 'Keine.'),
        ('Pedestal 0.6, basin 0.5, bubbler 0.7; body pen 0.25 mm, detail 0.13 mm.', 'Sockel 0,6, Becken 0,5, Sprudler 0,7; Körperstift 0,25 mm, Detail 0,13 mm.'),
        ('Collars carry pedestal-cast, the rim line basin-steel, the button bubbler-brass, the jet water-jet.', 'Kragen tragen pedestal-cast, die Randlinie basin-steel, der Taster bubbler-brass, der Strahl water-jet.'),
    ])


def r50():
    s = Shapes(rig)
    s.path('pedestal', 'pedestal', 'M -0.20 0 L 0.20 0 L 0.20 0.04 L 0.17 0.055 L 0.16 0.075 L 0.15 0.08 L 0.10 0.70 L 0.11 0.72 L 0.09 0.745 L -0.09 0.745 L -0.11 0.72 L -0.10 0.70 L -0.15 0.08 L -0.16 0.075 L -0.17 0.055 L -0.20 0.04 Z', z=3, tone=0.6, region='pedestal-cast', material='cast-iron', hatch='flutes')
    for i, y in enumerate((0.20, 0.42, 0.62)):
        w = 0.14 - 0.04 * (y - 0.08) / 0.62
        s.line(f'collar-{i}', 'pedestal', [(-w, y), (w, y)], role='detail', z=3.5, region='pedestal-cast')
    for i, x in enumerate((-0.14, 0.14)):
        s.ell(f'bolt-{i}', 'pedestal', x * 1.28, 0.02, 0.008, z=3.6, tone=0.2, role='detail', region='pedestal-cast')
    s.path('basin', 'basin', BOWL, z=3.2, tone=0.5, region='basin-steel', material='stainless-basin')
    s.line('rim', 'basin', [(-0.19, 0.84), (0.19, 0.84)], role='detail', z=3.5, region='basin-steel')
    s.path('rim-bead', 'basin', 'M -0.205 0.865 C -0.20 0.872 0.20 0.872 0.205 0.865', role='detail', z=3.6, filled=False, region='basin-steel')
    s.path('bubbler', 'bubbler', NECK.replace('-0.165', '-0.18').replace('-0.135', '-0.13'), z=3.3, tone=0.7, region='bubbler-brass', material='chrome-brass')
    s.ell('nozzle-ring', 'bubbler', -0.05, 0.958, 0.014, z=3.6, tone=0.3, role='detail', region='bubbler-brass')
    s.ell('button', 'button', 0.108, 0.66, 0.02, 0.025, z=3.4, tone=0.8, region='bubbler-brass')
    jet(s, 0.25)
    for i, (x, y) in enumerate(((0.07, 0.915), (0.095, 0.885))):
        s.ell(f'drop-{i}', 'jet', x, y, 0.007, z=2.7, tone=0.3, role='detail', region='water-jet')
    return rep(50, 20, 75, pens(0.35, 0.18, 0.13, None, 0.5), s, [
        ('A fountain 8 mm wide and 20 mm high with cast detail: stepped flange with bolt heads, fluted baluster with three collars, basin with rim bead, goose-neck bubbler with a nozzle ring, push button and the jet arc with two drops.', 'Ein Brunnen 8 mm breit und 20 mm hoch mit Gussdetail: gestufter Flansch mit Schraubenköpfen, kannelierter Baluster mit drei Kragen, Becken mit Randwulst, Schwanenhals-Sprudler mit Düsenring, Drucktaster und der Strahlbogen mit zwei Tropfen.'),
        ('All visible parts at real size; the button moves 10 mm in with the button bone, the jet sinks with the jet bone.', 'Alle sichtbaren Teile in realer Größe; der Taster fährt 10 mm mit dem Tasterknochen ein, der Strahl versinkt mit dem Strahlknochen.'),
        ('The far bolt head is hidden by the near flange.', 'Der ferne Schraubenkopf ist vom nahen Flansch verdeckt.'),
        ('Maker plate, drain grid, cast ornament.', 'Typenschild, Ablaufgitter, Gussornament.'),
        ('The jet is a 0.25 mm line for the 0.012 m stream (0.24 mm real); drops are 0.014 m (0.28 mm) circles; the nozzle ring is 0.56 mm.', 'Der Strahl ist eine 0,25-mm-Linie für den 0,012 m (0,24 mm) dicken Strahl; Tropfen sind Kreise von 0,014 m (0,28 mm); der Düsenring misst 0,56 mm.'),
        ('Flutes as vertical lines at 0.5 mm spacing inside the column.', 'Kannelüren als senkrechte Linien im Abstand 0,5 mm in der Säule.'),
        ('Pedestal 0.6, basin 0.5, bubbler 0.7; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.', 'Sockel 0,6, Becken 0,5, Sprudler 0,7; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm.'),
        ('Jet and drops carry water-jet: the thermal view paints the 12 degC water against the warm basin.', 'Strahl und Tropfen tragen water-jet: die Thermalansicht färbt das 12-grädige Wasser gegen das warme Becken.'),
    ], [hatch('flutes', 'Column flutes', 'Säulenkannelüren', 'lines', 90, 0.5, 0.13)])


H_ = lambda lid, t, k, m, th, kk, rho, cp: layer(lid, T(*t), k, m, th, kk, rho, cp, 'authored-estimate', 'iso-10456-2007')
regions = [
    region('pedestal-cast', T('Cast pedestal', 'Gusssockel'), 'surface', 'passive', 'cast-iron', 0.65, 0.25, 'cylinder', 0.5, [H_('casting', ('Cast iron 12 mm', 'Gusseisen 12 mm'), 'metal', 'cast-iron', 0.012, 50, 7200, 500)], 'authored-estimate', 'incropera-2018'),
    region('basin-steel', T('Stainless basin', 'Edelstahlbecken'), 'surface', 'passive', 'stainless-basin', 0.35, 0.4, 'horizontal-plate-up', 0.8, [H_('sheet', ('Stainless steel 1.5 mm', 'Edelstahl 1,5 mm'), 'metal', 'stainless-basin', 0.0015, 17, 7900, 500)], 'authored-estimate', 'incropera-2018'),
    region('bubbler-brass', T('Bubbler neck and button', 'Sprudlerhals und Taster'), 'surface', 'passive', 'chrome-brass', 0.04, 0.03, 'cylinder', 0.6, [H_('tube', ('Chrome-plated brass tube 2 mm', 'Verchromtes Messingrohr 2 mm'), 'metal', 'chrome-brass', 0.002, 110, 8500, 380)], 'authored-estimate', 'incropera-2018'),
    region('water-jet', T('Water jet', 'Wasserstrahl'), 'water', 'passive', 'water', 0.0057, 0.012, 'cylinder', 0.6, [H_('stream', ('Water stream 6 mm (jet radius)', 'Wasserstrahl 6 mm (Strahlradius)'), 'polymer', 'water', 0.006, 0.6, 1000, 4180)], 'authored-estimate', 'incropera-2018'),
]
RID = [r['id'] for r in regions]
OPK = [{'id': 'tap', 'title': T('Bubbler tap', 'Sprudlerhahn'), 'levels': [{'v': 0, 'title': T('Off', 'Aus')}, {'v': 1, 'title': T('Running (3 L/min)', 'Läuft (3 L/min)')}]}]
emitters = [{'id': 'jet-spray', 'kind': 'vent', 'title': T('Moisture of the water jet', 'Feuchte des Wasserstrahls'), 'anchor': 'nozzle', 'directionDeg': 60, 'diameterM': 0.012, 'operatingKey': 'tap', 'volumeFlowM3S': 0.0003, 'temperatureRiseK': -4, 'vapourKgPerKg': 0.0087, 'source': 'authored-estimate', 'reference': 'incropera-2018'}]
SUN = {'pedestal-cast': 0.5, 'basin-steel': 1.0, 'bubbler-brass': 0.8, 'water-jet': 0.0}
lean = load('basin-lean', T('Person bending over the basin', 'Person über das Becken gebeugt'), 'imposed', 30, -200, 'authored-estimate', 'fountain-class', anchor='lean-point')
JETUP = {'jet': {'dy': 0}, 'button': {'dx': -0.01}}
JETDN = {'jet': {'dy': -0.25}}
WATER = {'water-jet': {'fraction': 0.8, 'tempC': 12}}
poses = [
    pose('idle-sun', T('Idle in the sun', 'Ruhend in der Sonne'), T('Tap off, the jet sunk into the basin, sun on the basin and the neck.', 'Hahn aus, der Strahl im Becken versunken, Sonne auf Becken und Hals.'), JETDN, ['base-fixed'], bind(RID, sunmap=SUN, emitters={'jet-spray': 0}, operating={'tap': 0})),
    pose('drinking', T('Running', 'Läuft'), T('Button pressed 10 mm, the jet arcs from the nozzle into the basin and a person leans on the rim with 200 N.', 'Taster 10 mm gedrückt, der Strahl bogt von der Düse ins Becken und eine Person stützt sich mit 200 N auf den Rand.'), JETUP, ['base-fixed'], bind(RID, sunmap=SUN, emitters={'jet-spray': 1}, operating={'tap': 1}, contact=WATER, wet={'water-jet': 1.0, 'basin-steel': 0.5}), [lean]),
    pose('idle-shade', T('Idle in shade', 'Ruhend im Schatten'), T('Tap off in the shade of a tree; the cast pedestal stays cool.', 'Hahn aus im Baumschatten; der Gusssockel bleibt kühl.'), JETDN, ['base-fixed'], bind(RID, 0.0, emitters={'jet-spray': 0}, operating={'tap': 0})),
]
clips = [
    clip('press-and-drink', T('Press and drink', 'Drücken und trinken'), T('The button is pressed, the jet rises over two seconds, runs for eight seconds and sinks when the button is released.', 'Der Taster wird gedrückt, der Strahl steigt in zwei Sekunden, läuft acht Sekunden und versinkt beim Loslassen.'),
         14, 12, False, [(0, 'idle-sun', 'smooth'), (2, 'drinking', 'smooth'), (11, 'drinking', 'smooth'), (13, 'idle-sun', 'smooth'), (14, 'idle-sun', 'smooth')], [('base-fixed', 0, 14)],
         [('rest', 0, 1, T('At rest', 'In Ruhe')), ('flow', 1, 12, T('Water flows', 'Wasser fließt')), ('stop', 12, 14, T('Stopped', 'Gestoppt'))],
         [(1, 'ignition', T('Button pressed, tap opens', 'Taster gedrückt, Hahn öffnet'), 'tap'), (12, 'shutdown', T('Button released, tap closes', 'Taster losgelassen, Hahn schließt'), 'tap'), (3, 'load-on', T('Person leans over the basin', 'Person beugt sich über das Becken'), 'basin-lean'), (10, 'load-off', T('Person straightens up', 'Person richtet sich auf'), 'basin-lean')],
         [('operating:tap', 'smooth', [(0, 0), (1, 0), (2, 1), (11, 1), (12, 0), (14, 0)]), ('emitter:jet-spray', 'smooth', [(0, 0), (1, 0), (2, 1), (11, 1), (12, 0), (14, 0)]), ('load:basin-lean', 'smooth', [(0, 0), (3, 0), (4, 1), (9, 1), (10, 0), (14, 0)])]),
    clip('shade-to-sun', T('Shade to sun over two hours', 'Schatten zur Sonne in zwei Stunden'), T('A tree shadow leaves the fountain and the pedestal and basin warm up in the sun over two hours.', 'Ein Baumschatten verlässt den Brunnen, Sockel und Becken wärmen sich in der Sonne über zwei Stunden auf.'),
         7200, 0.01, False, [(0, 'idle-shade', 'step'), (1200, 'idle-sun', 'step'), (7200, 'idle-sun', 'step')], [('base-fixed', 0, 7200)],
         [('shade', 0, 1200, T('In shade', 'Im Schatten')), ('sun', 1200, 7200, T('In the sun', 'In der Sonne'))],
         [(1200, 'thermal', T('The shadow leaves', 'Der Schatten verlässt den Brunnen'), None)], []),
]
asset = {
    'id': 'drinking-fountain', 'category': 'objects',
    'title': T('Cast Drinking Fountain', 'Guss-Trinkbrunnen'),
    'summary': T('A 1.0 m cast-iron pedestal drinking fountain with a stainless basin and a goose-neck bubbler: a push button opens the tap, the water jet arcs into the basin as a cool, wet source of moisture, and the pedestal warms in the sun.', 'Ein 1,0 m hoher Trinkbrunnen mit Gusssockel, Edelstahlbecken und Schwanenhals-Sprudler: ein Drucktaster öffnet den Hahn, der Wasserstrahl bogt als kühle, feuchte Feuchtequelle ins Becken und der Sockel erwärmt sich in der Sonne.'),
    'tags': ['drinking-fountain', 'water', 'cast-iron', 'pedestal', 'moisture-source', 'public-space'],
    'provenance': {'author': 'Semio staffage content (NW-O)', 'method': 'handcrafted', 'created': '2026-10-29', 'references': refs,
                   'notes': T('Side elevation, mass 68 kg and a flow of 3 L/min are class values (authored estimates). The jet is a bone that retracts 0.25 m behind the basin when the tap is off, so it is not drawn as running water in the idle poses. The jet emitter releases air saturated at the 12 degC supply temperature (humidity ratio 0.0087 kg/kg, 14 K below the 26 degC ambient of the reference case, here applied as a 4 K cooling of the entrained air) at 0.0003 m3/s: an authored estimate of the entrained boundary-layer air of a 0.012 m jet (evaporating surface about 0.006 m2).', 'Seitenansicht, Masse 68 kg und ein Durchfluss von 3 L/min sind Klassenwerte (Schätzwerte). Der Strahl ist ein Knochen, der sich bei geschlossenem Hahn 0,25 m hinter dem Becken zurückzieht, damit er in den Ruheposen nicht als fließendes Wasser gezeichnet wird. Der Strahl-Emitter gibt bei 0,0003 m3/s Luft ab, die bei der Zulauftemperatur von 12 degC gesättigt ist (Feuchtegrad 0,0087 kg/kg): eine Schätzung der mitgerissenen Grenzschichtluft eines 0,012 m dicken Strahls (verdunstende Fläche etwa 0,006 m2).')},
    'dimensions': {'heightM': 1.00, 'widthM': 0.40, 'depthM': 0.40, 'massKg': round(rig.total_mass(), 2)},
    'materials': mats, 'rig': rig.rig(), 'anchors': anchors,
    'representations': {'500': r500(), '200': r200(), '100': r100(), '50': r50()}, 'symbols': [],
    'poses': poses, 'clips': clips,
    'thermal': {'environment': environment(26, 32, 0.45, 0.5, 800, 0.15, sky=12, ground=36), 'operating': OPK, 'passengers': [], 'regions': regions, 'emitters': emitters,
                'assumptions': [T('The water jet is a passive water region held toward the 12 degC mains temperature by a conductive contact (fraction 0.8) while the tap runs; its wettedness is 1 and the basin 0.5.', 'Der Wasserstrahl ist eine passive Wasserregion, die bei laufendem Hahn durch einen Leitungskontakt (Anteil 0,8) zur Netztemperatur von 12 degC gezogen wird; seine Benetzung ist 1, die des Beckens 0,5.'),
                                T('The cooling of the basin by the running water is carried by the wetted region only; the drain heat flow is neglected.', 'Die Kühlung des Beckens durch das laufende Wasser wird nur über die benetzte Region getragen; der Wärmestrom im Ablauf wird vernachlässigt.')]},
    'structure': structure(['base-fixed'], [('A rigid cast pedestal fixed at its flange; a person leaning on the basin adds 200 N down and 30 N sideways (authored estimate).', 'Ein starrer Gusssockel, am Flansch eingespannt; eine sich aufstützende Person fügt 200 N nach unten und 30 N seitlich hinzu (Schätzung).'),
                                         ('The jet and the button are posed bones; the water mass of the jet is neglected.', 'Strahl und Taster sind gestellte Knochen; die Wassermasse des Strahls wird vernachlässigt.')]),
    'presentation': {'defaultPose': 'drinking', 'defaultClip': 'press-and-drink', 'defaultScale': 100, 'viewpoints': ['elevation']},
    'exports': ENV_EXPORTS, 'completion': 'authored',
}
save(asset)
print('mass', rig.total_mass())
