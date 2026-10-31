import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

K = 0.5523
D, R = 0.0483, 0.02415
XC, YC = 0.3508, 0.776
BR = 0.18
XO, XI = XC + R, XC - R
CX = XC - BR
YB = YC - BR


def hoop_band(o_w, i_w, top, thick, bend=BR):
    """closed U band: outer half width o_w, inner half width i_w, top y, bend centreline radius `bend`."""
    ro, ri = bend + thick / 2, bend - thick / 2
    cx = (o_w + i_w) / 2 - bend
    yb = top - thick / 2 - bend
    ko, ki = K * ro, K * ri
    return (f'M {-o_w:g} 0 L {-o_w:g} {yb:g} C {-o_w:g} {yb + ko:g} {-cx - ko:g} {top:g} {-cx:g} {top:g} L {cx:g} {top:g} C {cx + ko:g} {top:g} {o_w:g} {yb + ko:g} {o_w:g} {yb:g} L {o_w:g} 0 '
            f'L {i_w:g} 0 L {i_w:g} {yb:g} C {i_w:g} {yb + ki:g} {cx + ki:g} {top - thick:g} {cx:g} {top - thick:g} L {-cx:g} {top - thick:g} C {-cx - ki:g} {top - thick:g} {-i_w:g} {yb + ki:g} {-i_w:g} {yb:g} L {-i_w:g} 0 Z')


refs = [
    ref('en-10219-2', 'EN 10219-2:2006 Cold formed welded structural hollow sections of non-alloy and fine grain steels, Part 2: Tolerances, dimensions and sectional properties (circular hollow section 48.3 x 3.2 mm, 3.56 kg/m).'),
    ref('en-1993-1-1', 'EN 1993-1-1:2005 Eurocode 3: Design of steel structures, Part 1-1: General rules and rules for buildings (E = 210 GPa, density 7850 kg/m3).'),
    ref('en-iso-1461', 'EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles - Specifications and test methods (zinc coating of street furniture).'),
    ref('en-1991-1-1', 'EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: Densities, self-weight, imposed loads for buildings (guidance on barrier and imposed horizontal loads).'),
    ref('incropera-2018', 'Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (surface radiative properties, convection correlations).'),
    ref('iso-10456-2007', 'ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity, specific heat and density of steel).'),
]

mats = [material('galv-steel', 'Hot-dip galvanised steel', 'Feuerverzinkter Stahl', 0.28, 0.6, 7850, 50, 450, 'literature', 'iso-10456-2007', 210e9)]

rig = Rig('structural', 'stand')
rig.bone('stand', None, 0, 0, 0.0483, FIXED, T('Stand reference (ground point between the legs)', 'Standbezug (Bodenpunkt zwischen den Beinen)'))
rig.bone('hoop', 'stand', 0, 0, 0.8, FIXED, T('Inverted U hoop', 'Umgekehrter U-Bügel'), (7.47, 0, 0.42, 'literature', 'en-10219-2'))
rig.bone('crossbar', 'hoop', 0, 0.40, 0.7017, FIXED, T('Cross rail', 'Querholm'), (2.50, 0, 0.40, 'literature', 'en-10219-2'))

anchors = [
    rig.anchor('leg-left', 'hoop', -XC, 0, 'support', T('Left leg embedded in the footing', 'Linkes Bein im Fundament eingespannt'), support='fixed', footprintLengthM=D, footprintWidthM=D),
    rig.anchor('leg-right', 'hoop', XC, 0, 'support', T('Right leg embedded in the footing', 'Rechtes Bein im Fundament eingespannt'), support='fixed', footprintLengthM=D, footprintWidthM=D),
    rig.anchor('lean-point', 'hoop', XC, 0.55, 'load', T('Lean contact on the right leg', 'Anlehnpunkt am rechten Bein')),
    rig.anchor('lock-point', 'crossbar', 0, 0.40, 'load', T('Lock point on the cross rail', 'Schlosspunkt am Querholm')),
]

# ------------------------------------------------------------- representations
def r500():
    s = Shapes(rig)
    s.path('hoop', 'hoop', hoop_band(0.375, 0.215, 0.80, 0.16, 0.15), z=3, tone=0.5, region='hoop-galv', material='galv-steel')
    s.rect('crossbar', 'crossbar', -0.23, 0.325, 0.23, 0.475, z=3.2, tone=0.5, region='crossbar-galv', material='galv-steel')
    return rep(500, 350, 1000, pens(0.13, None, None, None, 0.18), s, [
        ('One closed U band (1.5 x 1.6 mm on paper) with the cross rail as a bar; the 0.75 x 0.80 m outer size is kept exactly.', 'Ein geschlossenes U-Band (1,5 x 1,6 mm auf dem Papier) mit dem Querholm als Balken; das Außenmaß 0,75 x 0,80 m bleibt exakt.'),
        ('Outer width 0.75 m, height 0.80 m, the cross rail at 0.40 m.', 'Aussenbreite 0,75 m, Höhe 0,80 m, Querholm auf 0,40 m.'),
        ('Tube wall and weld collars merge into the solid band.', 'Rohrwand und Schweißkragen gehen im vollen Band auf.'),
        ('Weld lines, bend tangents, embedded leg stubs.', 'Schweißnähte, Biegetangenten, einbetonierte Beinstummel.'),
        ('The 0.0483 m tube would be 0.097 mm on paper, below the feature width: the band is thickened to 0.16 m (0.32 mm) inward from the real outer edge, so the outer size stays exact and the inner opening shrinks by 0.11 m per side; the bend radius is tightened to 0.15 m.', 'Das 0,0483-m-Rohr wäre auf dem Papier 0,097 mm breit, unter der Mindestbreite: das Band wird vom realen Außenrand nach innen auf 0,16 m (0,32 mm) verbreitert, das Außenmaß bleibt exakt, die Innenöffnung schrumpft je Seite um 0,11 m; der Biegeradius ist auf 0,15 m verengt.'),
        ('None.', 'Keine.'),
        ('Solid band tone 0.5, outline 0.13 mm; ground line 0.18 mm.', 'Volles Band Ton 0,5, Kontur 0,13 mm; Bodenlinie 0,18 mm.'),
        ('Hoop = hoop-galv, cross rail = crossbar-galv; overlay colours map onto the two solids.', 'Bügel = hoop-galv, Querholm = crossbar-galv; Überlagerungsfarben liegen auf den beiden Vollformen.'),
    ])


def hoop_q(o_w, i_w, top, thick, cx):
    ti = top - thick
    return (f'M {-o_w:g} 0 L {-o_w:g} {YB:g} Q {-o_w:g} {top:g} {-cx:g} {top:g} L {cx:g} {top:g} Q {o_w:g} {top:g} {o_w:g} {YB:g} L {o_w:g} 0 '
            f'L {i_w:g} 0 L {i_w:g} {YB:g} Q {i_w:g} {ti:g} {cx:g} {ti:g} L {-cx:g} {ti:g} Q {-i_w:g} {ti:g} {-i_w:g} {YB:g} L {-i_w:g} 0 Z')


def r200():
    s = Shapes(rig)
    s.path('hoop', 'hoop', hoop_q(0.375, XI, 0.80, D, 0.19), z=3, tone=0.45, region='hoop-galv', material='galv-steel')
    s.rect('crossbar', 'crossbar', -XC, 0.40 - R, XC, 0.40 + R, z=3.2, tone=0.45, region='crossbar-galv', material='galv-steel')
    return rep(200, 150, 350, pens(0.18, 0.13, None, None, 0.25), s, [
        ('Inverted U of true tube size (0.24 mm wide) with a straight cross rail between the legs.', 'Umgekehrtes U in wahrer Rohrstärke (0,24 mm breit) mit geradem Querholm zwischen den Beinen.'),
        ('Real tube diameter 0.0483 m, cross rail at 0.40 m.', 'Reale Rohrstärke 0,0483 m, Querholm auf 0,40 m.'),
        ('The two bends are simplified to one quadratic curve each; weld collars at the rail ends merge into the legs.', 'Die beiden Bögen sind zu je einer quadratischen Kurve vereinfacht; Schweißkragen an den Holmenden gehen in den Beinen auf.'),
        ('Weld lines and bend tangents.', 'Schweißnähte und Biegetangenten.'),
        ('Tube is 0.24 mm wide, just above the 0.2 mm feature width; outline 0.18 mm.', 'Das Rohr ist 0,24 mm breit, knapp über der Mindestbreite von 0,2 mm; Kontur 0,18 mm.'),
        ('None.', 'Keine.'),
        ('Tone 0.45; body pen 0.18 mm, ground 0.25 mm.', 'Ton 0,45; Körperstift 0,18 mm, Boden 0,25 mm.'),
        ('Same two regions as 1:500.', 'Dieselben zwei Regionen wie bei 1:500.'),
    ])


def r100():
    s = Shapes(rig)
    s.path('hoop', 'hoop', hoop_band(0.375, XI, 0.80, D), z=3, tone=0.45, region='hoop-galv', material='galv-steel')
    s.rect('crossbar', 'crossbar', -XI, 0.40 - R, XI, 0.40 + R, z=3.2, tone=0.45, region='crossbar-galv', material='galv-steel')
    s.line('weld-l', 'crossbar', [(-XI, 0.40 - R), (-XI, 0.40 + R)], role='detail', z=4, region='crossbar-galv')
    s.line('weld-r', 'crossbar', [(XI, 0.40 - R), (XI, 0.40 + R)], role='detail', z=4, region='crossbar-galv')
    return rep(100, 75, 150, pens(0.25, 0.13, None, None, 0.35), s, [
        ('Hoop band 0.48 mm wide at true tube size, cross rail with its two weld lines.', 'Bügelband in wahrer Rohrstärke 0,48 mm breit, Querholm mit seinen zwei Schweißnähten.'),
        ('True tube size and bend radius, cross rail, both weld joints.', 'Wahre Rohrstärke und Biegeradius, Querholm, beide Schweißstöße.'),
        ('Bend tangents merge into the continuous band.', 'Biegetangenten gehen im durchgehenden Band auf.'),
        ('Embedded leg stubs, zinc texture.', 'Einbetonierte Beinstummel, Zinktextur.'),
        ('Welds are 0.13 mm detail strokes across the tube (0.48 mm).', 'Schweißnähte sind 0,13-mm-Detailstriche quer zum Rohr (0,48 mm).'),
        ('None; the tube is too narrow for hatching.', 'Keine; das Rohr ist für Schraffur zu schmal.'),
        ('Tone 0.45; body pen 0.25 mm, detail 0.13 mm, ground 0.35 mm.', 'Ton 0,45; Körperstift 0,25 mm, Detail 0,13 mm, Boden 0,35 mm.'),
        ('Hoop and cross rail are separate solids, each with its region.', 'Bügel und Querholm sind getrennte Vollformen mit je einer Region.'),
    ])


def r50():
    s = Shapes(rig)
    s.path('hoop', 'hoop', hoop_band(0.375, XI, 0.80, D), z=3, tone=0.45, region='hoop-galv', material='galv-steel')
    s.rect('crossbar', 'crossbar', -XI, 0.40 - R, XI, 0.40 + R, z=3.2, tone=0.45, region='crossbar-galv', material='galv-steel')
    for i, x in enumerate((-XI + 0.014, XI - 0.014)):
        s.line(f'weld-collar-{i}', 'crossbar', [(x, 0.40 - R), (x, 0.40 + R)], role='detail', z=4, region='crossbar-galv')
    for i, (x, y) in enumerate(((-XC, 0.596), (XC, 0.596))):
        s.line(f'bend-tangent-low-{i}', 'hoop', [(x - R, y), (x + R, y)], role='texture', z=4, region='hoop-galv')
    for i, x in enumerate((-CX, CX)):
        s.line(f'bend-tangent-top-{i}', 'hoop', [(x, 0.80), (x, 0.80 - D)], role='texture', z=4, region='hoop-galv')
    for i, x in enumerate((-XC, XC)):
        s.line(f'ground-collar-{i}', 'hoop', [(x - R, 0.004), (x + R, 0.004)], role='detail', z=4, region='hoop-galv')
    return rep(50, 20, 75, pens(0.35, 0.18, 0.13, None, 0.5), s, [
        ('Hoop band 0.97 mm wide at true size with bend tangent lines, the cross rail with weld collars at both joints and ground-level collar lines where the legs enter the footing.', 'Bügelband in wahrer Größe 0,97 mm breit mit Biegetangentenlinien, Querholm mit Schweißkragen an beiden Stößen und Bodenkragenlinien, wo die Beine ins Fundament eintreten.'),
        ('Real tube 48.3 mm, bend radius 0.18 m, cross rail 0.40 m, weld collars, bend tangents, ground collars.', 'Reales Rohr 48,3 mm, Biegeradius 0,18 m, Querholm 0,40 m, Schweißkragen, Biegetangenten, Bodenkragen.'),
        ('Weld collar and tube wall form one region each.', 'Schweißkragen und Rohrwand bilden je eine Region.'),
        ('Zinc spangle texture, bolted footing detail.', 'Zinkblumen-Textur, Fundamentschrauben.'),
        ('Weld collars and tangent lines are 0.13-0.18 mm strokes across the 0.97 mm tube.', 'Schweißkragen und Tangentenlinien sind 0,13-0,18-mm-Striche quer zum 0,97-mm-Rohr.'),
        ('None; the tube is narrower than any hatch spacing.', 'Keine; das Rohr ist schmaler als jeder Schraffurabstand.'),
        ('Tone 0.45; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm, ground 0.5 mm.', 'Ton 0,45; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm, Boden 0,5 mm.'),
        ('Collar lines and bend tangents carry the region of the tube they cross.', 'Kragen- und Tangentenlinien tragen die Region des Rohrs, das sie queren.'),
    ])


# symbols (plan view: the hoop seen from above is a line with two leg dots)
def sym_true():
    s = Shapes(rig)
    s.rect('hoop-plan', 'stand', -0.375, -R, 0.375, R, z=3, tone=0.6, region='hoop-galv', material='galv-steel')
    s.ell('leg-plan-l', 'stand', -XC, 0, R * 0.55, z=4, tone=0.2, region='hoop-galv')
    s.ell('leg-plan-r', 'stand', XC, 0, R * 0.55, z=4, tone=0.2, region='hoop-galv')
    return {'id': 'plan-true', 'title': T('Plan, true extent', 'Grundriss, wahre Ausdehnung'),
            'description': T('The hoop seen from above: a 0.75 x 0.0483 m bar with the two leg centres marked; the bikes park along the long side of this bar.', 'Der Bügel von oben: ein Balken 0,75 x 0,0483 m mit den zwei markierten Beinmitten; die Räder parken entlang der Längsseite des Balkens.'),
            'semantics': 'true-extent', 'units': 'model-m', 'denominators': [200, 100, 50], 'pens': pens(0.18, 0.13, None, None, 0.25), 'hatches': [], 'geometry': {'elevation': [], 'plan': s.items},
            'dimensionalNote': T('Model metres at real size: the bar length 0.75 m and width 0.0483 m are true; pen widths are paper millimetres.', 'Modellmeter in wahrer Größe: Balkenlänge 0,75 m und Breite 0,0483 m sind wahr; Stiftbreiten sind Papiermillimeter.')}


def sym_nominal():
    s = Shapes(rig)
    s.line('bar', 'stand', [(-2.2, 0), (2.2, 0)], role='body', z=3)
    s.ell('foot-l', 'stand', -2.2, 0, 0.45, z=4, tone=1.0)
    s.ell('foot-r', 'stand', 2.2, 0, 0.45, z=4, tone=1.0)
    return {'id': 'plan-mark', 'title': T('Plan mark', 'Planzeichen'),
            'description': T('A 4.4 mm bar with a dot at each end marks one stand on a small-scale site plan; the row of marks shows the parking capacity.', 'Ein 4,4-mm-Balken mit einem Punkt an jedem Ende markiert einen Bügel auf einem Lageplan; die Zeichenreihe zeigt die Stellplatzzahl.'),
            'semantics': 'nominal', 'units': 'paper-mm', 'denominators': [500, 200], 'pens': pens(0.25, None, None, None, 0.35), 'hatches': [], 'geometry': {'elevation': [], 'plan': s.items},
            'dimensionalNote': T('Nominal paper sign: its 4.4 mm length carries no dimension (the real hoop is 0.75 m = 1.5 mm at 1:500); insertion point at the bar centre.', 'Nominelles Papierzeichen: seine Länge von 4,4 mm trägt kein Maß (der reale Bügel ist 0,75 m = 1,5 mm bei 1:500); Einfügepunkt in der Balkenmitte.')}


# ------------------------------------------------------------- thermal
tube = lambda lid: layer(lid, T('Galvanised steel tube wall 3.2 mm', 'Verzinkte Stahlrohrwand 3,2 mm'), 'metal', 'galv-steel', 0.0032, 50, 7850, 450, 'literature', 'iso-10456-2007')
regions = [
    region('hoop-galv', T('Galvanised hoop tube', 'Verzinktes Bügelrohr'), 'surface', 'passive', 'galv-steel', 0.32, 0.0483, 'cylinder', 0.5, [tube('wall')], 'authored-estimate', 'incropera-2018'),
    region('crossbar-galv', T('Galvanised cross rail tube', 'Verzinktes Querholmrohr'), 'surface', 'passive', 'galv-steel', 0.106, 0.0483, 'cylinder', 0.4, [tube('wall')], 'authored-estimate', 'incropera-2018'),
]
RID = [r['id'] for r in regions]
SUN, SHADE = {'hoop-galv': 0.5, 'crossbar-galv': 0.5}, {}

lean = load('bike-lean', T('In-plane lean of a parked bicycle', 'In der Ebene wirkendes Anlehnen eines Fahrrads'), 'imposed', 150, 0, 'authored-estimate', 'en-1991-1-1', anchor='lean-point')
pull = load('lock-pull', T('Lever pull on the lock point', 'Hebelzug am Schlosspunkt'), 'imposed', 500, 200, 'authored-estimate', 'en-1991-1-1', anchor='lock-point')
both = ['leg-left', 'leg-right']

poses = [
    pose('empty-shade', T('Empty, in shade', 'Leer, im Schatten'), T('The bare stand in shade; only its own weight acts.', 'Der leere Bügel im Schatten; nur das Eigengewicht wirkt.'), {}, both, bind(RID, 0.0)),
    pose('empty-sun', T('Empty, in the sun', 'Leer, in der Sonne'), T('The bare stand with the sun on one half of each tube.', 'Der leere Bügel, die Sonne auf der Hälfte jedes Rohrs.'), {}, both, bind(RID, sunmap=SUN)),
    pose('bike-leaning', T('Bicycle leaning', 'Fahrrad angelehnt'), T('A parked bicycle leans on the right leg: an in-plane horizontal push of 150 N at 0.55 m. The bicycle is a load only, not a second asset.', 'Ein geparktes Fahrrad lehnt am rechten Bein: ein in der Ebene wirkender Horizontalschub von 150 N auf 0,55 m. Das Fahrrad ist nur eine Last, kein zweites Asset.'), {}, both, bind(RID, sunmap=SUN), [lean]),
    pose('lock-pulled', T('Lock pulled', 'Schloss gezogen'), T('A D-lock on the cross rail is pulled with 500 N sideways and 200 N upwards (theft or a bicycle falling over).', 'Ein Bügelschloss am Querholm wird mit 500 N seitlich und 200 N nach oben gezogen (Diebstahl oder umfallendes Fahrrad).'), {}, both, bind(RID, 0.0), [pull]),
]

clips = [
    clip('bike-parked', T('Bicycle parked and removed', 'Fahrrad abgestellt und weggenommen'), T('A bicycle is leaned on the stand, rests for a while and is taken away again; the lean load follows.', 'Ein Fahrrad wird angelehnt, steht eine Weile und wird wieder weggenommen; die Anlehnlast folgt.'),
         12, 10, False, [(0, 'bike-leaning', 'smooth'), (12, 'bike-leaning', 'smooth')], [('leg-left', 0, 12), ('leg-right', 0, 12)],
         [('approach', 0, 3, T('Bicycle arrives', 'Fahrrad kommt')), ('parked', 3, 9, T('Parked', 'Geparkt')), ('departure', 9, 12, T('Bicycle leaves', 'Fahrrad geht'))],
         [(2, 'load-on', T('Bicycle leans on the stand', 'Fahrrad lehnt am Bügel'), 'bike-lean'), (10, 'load-off', T('Bicycle is taken away', 'Fahrrad wird weggenommen'), 'bike-lean')],
         [('load:bike-lean', 'smooth', [(0, 0), (2, 0), (3, 1), (9, 1), (10, 0), (12, 0)])]),
    clip('sun-and-shade', T('Sun and shade over four hours', 'Sonne und Schatten über vier Stunden'), T('Four hours in which a building shadow leaves and returns: the tubes warm in the sun and cool in shade according to their heat capacity.', 'Vier Stunden, in denen ein Gebäudeschatten verschwindet und zurückkehrt: die Rohre erwärmen sich in der Sonne und kühlen im Schatten gemäß ihrer Wärmekapazität.'),
         14400, 0.01, False, [(0, 'empty-shade', 'step'), (3600, 'empty-sun', 'step'), (10800, 'empty-shade', 'step')], [('leg-left', 0, 14400), ('leg-right', 0, 14400)],
         [('shade-1', 0, 3600, T('In shade', 'Im Schatten')), ('sun', 3600, 10800, T('In the sun', 'In der Sonne')), ('shade-2', 10800, 14400, T('In shade again', 'Wieder im Schatten'))],
         [(3600, 'thermal', T('Sun reaches the stand', 'Sonne erreicht den Bügel'), None), (10800, 'thermal', T('Shadow returns', 'Schatten kehrt zurück'), None)],
         [('region:hoop-galv:sunlitFraction', 'step', [(0, 0), (3600, 0.5), (10800, 0)]), ('region:crossbar-galv:sunlitFraction', 'step', [(0, 0), (3600, 0.5), (10800, 0)])]),
]

asset = {
    'id': 'bicycle-stand', 'category': 'objects',
    'title': T('Bicycle Stand (Sheffield Hoop)', 'Fahrradbügel (Sheffield-Bügel)'),
    'summary': T('A galvanised 48.3 mm tube hoop with a cross rail, set in a concrete footing: 0.75 m wide, 0.80 m high, drawn face-on from the side where the bikes park; a leaning bicycle is a load, not a second asset. The plan shows the hoop as a bar with two leg dots.', 'Ein feuerverzinkter Bügel aus 48,3-mm-Rohr mit Querholm im Betonfundament: 0,75 m breit, 0,80 m hoch, von der Parkseite her frontal gezeichnet; ein angelehntes Fahrrad ist eine Last, kein zweites Asset. Der Grundriss zeigt den Bügel als Balken mit zwei Beinpunkten.'),
    'tags': ['bicycle-stand', 'sheffield-stand', 'steel', 'street-furniture', 'galvanised', 'mobility'],
    'provenance': {'author': 'Semio staffage content (NW-O)', 'method': 'handcrafted', 'created': '2026-10-29', 'references': refs,
                   'notes': T('Face-on elevation of the hoop plane; bicycles stand parallel to it, so the real lean force acts out of this plane and the planar solver only sees the in-plane share (authored estimate, 150 N lean, 500/200 N lock pull). Tube 48.3 x 3.2 mm (3.56 kg/m, EN 10219-2); legs concreted in 0.3 m, the buried part is neither drawn nor counted. Mass 10.0 kg above ground.', 'Frontale Ansicht der Bügelebene; die Fahrräder stehen parallel dazu, die reale Anlehnkraft wirkt also aus der Ebene heraus und der ebene Löser sieht nur den Anteil in der Ebene (Schätzung, 150 N Anlehnen, 500/200 N Schlosszug). Rohr 48,3 x 3,2 mm (3,56 kg/m, EN 10219-2); Beine 0,3 m einbetoniert, der vergrabene Teil wird weder gezeichnet noch gezählt. Masse 10,0 kg über Boden.')},
    'dimensions': {'heightM': 0.80, 'widthM': 0.75, 'depthM': 0.0483, 'massKg': round(rig.total_mass(), 2)},
    'materials': mats, 'rig': rig.rig(), 'anchors': anchors,
    'representations': {'500': r500(), '200': r200(), '100': r100(), '50': r50()},
    'symbols': [sym_true(), sym_nominal()],
    'poses': poses, 'clips': clips,
    'thermal': {'environment': environment(22, 28, 0.5, 1.5, 700, 0.15, sky=10, ground=30), 'operating': [], 'passengers': [], 'regions': regions, 'emitters': [],
                'assumptions': [T('Both tubes are inert passive surfaces: solar gain on the lit half, radiation to sky and ground, forced convection from the wind; no internal heat.', 'Beide Rohre sind inerte passive Flächen: Solargewinn auf der besonnten Hälfte, Strahlung zu Himmel und Boden, erzwungene Konvektion durch den Wind; keine innere Wärme.'),
                                T('The embedded legs conduct to the footing; that exchange is neglected (authored estimate, small compared with the exposed tube area).', 'Die einbetonierten Beine leiten in das Fundament; dieser Austausch wird vernachlässigt (Schätzung, klein gegenüber der freien Rohrfläche).')]},
    'structure': structure(both, [('The hoop is a rigid body clamped by two fixed supports in the footing; loads are in the plane of the hoop.', 'Der Bügel ist ein starrer Körper, durch zwei feste Auflager im Fundament eingespannt; die Lasten liegen in der Bügelebene.'),
                                  ('Out-of-plane lean of a bicycle is not represented; the in-plane shares are authored estimates.', 'Das Anlehnen eines Fahrrads aus der Ebene wird nicht abgebildet; die Anteile in der Ebene sind Schätzungen.')]),
    'presentation': {'defaultPose': 'empty-sun', 'defaultClip': 'bike-parked', 'defaultScale': 100, 'viewpoints': ['elevation']},
    'exports': ENV_EXPORTS, 'completion': 'authored',
}
save(asset)
print('mass', rig.total_mass())
