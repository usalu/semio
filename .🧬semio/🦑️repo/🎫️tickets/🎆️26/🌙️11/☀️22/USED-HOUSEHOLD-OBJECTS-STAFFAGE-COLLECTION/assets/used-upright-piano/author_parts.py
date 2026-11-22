"""🎼 Parts of the used upright piano: dimensions, materials, references, rig, anchors, the solids of every scale and the 2D drawings drawn to the same numbers.

Frames: the case bone has its origin at the back of the piano, centre plane, on the contact plane of its own castors (x ahead, y up, z left);
at rest it stands 0.113 m high on the piano dolly. See also author-piano.py and author_geom.py.
"""
import math

from author_geom import T, r6, arc, box, extrude, revolve, with_repeat, sh, rect, poly, circle, tex, det, rect_pts, swept

G = 9.80665
H = 1.25
W2 = 0.75
D = 0.60
DOLLY_TOP = 0.113
CASTOR_X = (0.07, 0.50)
CASTOR_Z = 0.66
LID_Y0 = 1.225
LID_X1 = 0.375
STEP_Y = 0.79
KEYBED_X0, KEYBED_Y0, KEYBED_Y1 = 0.13, 0.64, 0.68
SLIP_X0, SLIP_Y1 = 0.584, 0.712
CHEEK_ZI = 0.61
KEY_Z = 0.6095
KEYS_X0, KEYS_X1, KEYS_Y1 = 0.15, 0.584, 0.722
BLACK_X0, BLACK_X1, BLACK_Y1, BLACK_HW = 0.20, 0.535, 0.734, 0.00525
NAME_X0, NAME_X1, NAME_Y0 = 0.36, 0.42, 0.745
FALL_HINGE = (0.42, 0.763)
FALL_LEN, FALL_T, FALL_LIP, FALL_HZ = 0.177, 0.018, 0.0495, 0.6085
PEDAL_PIVOT = (0.25, 0.09)
PEDAL_Z = {"pedal-soft": -0.12, "pedal-mute": 0.0, "pedal-sustain": 0.12}
PEDAL_HW = 0.0125
DOLLY_X0, DOLLY_X1, DOLLY_Z = 0.04, 0.52, 0.70
DOLLY_CASTOR_X, DOLLY_CASTOR_Z = (0.10, 0.46), 0.60
BOARD_Y0, BOARD_Y1 = 0.085, 0.106
STRAP_X, STRAP_Y = 0.205, 0.0835
STRAP_HW, STRAP_T, STRAP_ZI, STRAP_ZO = 0.025, 0.0015, 0.7505, 0.752
STRAP_TOP = DOLLY_TOP + H - STRAP_Y
BUCKLE_V0, BUCKLE_V1, BUCKLE_ZO = 0.60, 0.72, 0.777
BENCH_X, BENCH_D, BENCH_HW = 0.597, 0.33, 0.28
BENCH_SEAT_Y, BENCH_TOP = 0.415, 0.475
BENCH_USE_DX = 0.28
DOLLY_PARK_DX = -0.54
STRAP_DROP_DY = STRAP_HW - STRAP_Y
CARRY_DX, CARRY_TILT = -0.70, 3.0
FALL_OPEN, LID_OPEN, PEDAL_DOWN = 90.0, 80.0, -2.5
DIM_H = DOLLY_TOP + H + STRAP_T
DIM_W = BENCH_X + BENCH_D
DIM_D = BUCKLE_ZO + STRAP_ZO
BONE_Z = dict(PEDAL_Z)
P_KEY = 2 * KEY_Z / 52
BLACK_OFFSETS = (0.9, 2.1, 3.9, 5.0, 6.1)

MATERIALS = [
    {"id": "walnut-veneer-aged", "title": T("Walnut-veneered chipboard P2 with satin lacquer, dulled and slightly faded after about 45 years", "Nussbaumfurnierte Spanplatte P2 mit seidenmattem Lack, nach etwa 45 Jahren stumpf und leicht verblasst"), "emissivity": 0.9, "solarAbsorptivity": 0.68, "densityKgM3": 680, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 1.8e9, "strengthPa": 1.1e7, "source": "literature", "reference": "en-312-2010"},
    {"id": "walnut-veneer-bleached", "title": T("Walnut-veneered chipboard of the lid, bleached by decades of sun at a window", "Nussbaumfurnierte Spanplatte des Deckels, durch Jahrzehnte Sonne am Fenster ausgebleicht"), "emissivity": 0.9, "solarAbsorptivity": 0.52, "densityKgM3": 680, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 1.8e9, "strengthPa": 1.1e7, "source": "authored-estimate", "reference": "en-312-2010"},
    {"id": "beech-solid", "title": T("Solid beech (keybed, cheek blocks, pedal rail), veneered where seen", "Massive Buche (Klaviaturboden, Backen, Pedalleiste), wo sichtbar furniert"), "emissivity": 0.9, "solarAbsorptivity": 0.68, "densityKgM3": 720, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 1.3e10, "strengthPa": 4.0e7, "source": "literature", "reference": "en-338-2016"},
    {"id": "spruce-raw-aged", "title": T("Raw spruce of the back posts and soundboard, darkened and dusty", "Rohe Fichte der Rasten und des Resonanzbodens, nachgedunkelt und staubig"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 450, "conductivityWmK": 0.12, "specificHeatJKgK": 1600, "elasticModulusPa": 1.1e10, "strengthPa": 2.4e7, "source": "authored-estimate", "reference": "wagenfuehr-holzatlas"},
    {"id": "cast-iron-frame", "title": T("Grey cast iron EN-GJL-200 of the frame, bronze lacquer", "Grauguss EN-GJL-200 der Gussplatte, bronzelackiert"), "emissivity": 0.8, "solarAbsorptivity": 0.5, "densityKgM3": 7200, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 1.0e11, "strengthPa": 2.0e8, "source": "literature", "reference": "en-1561-2011"},
    {"id": "string-plane", "title": T("Music wire and wound bass strings, effective density of the 16 mm string plane", "Saitendraht und umsponnene Basssaiten, wirksame Dichte der 16 mm starken Saitenebene"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 680, "conductivityWmK": 5.0, "specificHeatJKgK": 470, "source": "authored-estimate", "reference": "fletcher-rossing-1998"},
    {"id": "beech-multiplex", "title": T("Beech multiplex pin block", "Stimmstock aus Buchenmultiplex"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 750, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 1.0e10, "strengthPa": 5.0e7, "source": "literature", "reference": "wagenfuehr-holzatlas"},
    {"id": "action-effective", "title": T("Hammer action of maple, beech and felt, effective density over its envelope", "Hammermechanik aus Ahorn, Buche und Filz, wirksame Dichte über ihre Hüllform"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 160, "conductivityWmK": 0.1, "specificHeatJKgK": 1600, "source": "authored-estimate", "reference": "upright-piano-catalogue"},
    {"id": "keytop-acrylic-yellowed", "title": T("Key levers of lime wood with acrylic key tops yellowed by age, lead weights included", "Tastenhebel aus Linde mit altersbedingt vergilbten Acrylbelägen, Bleigewichte eingerechnet"), "emissivity": 0.9, "solarAbsorptivity": 0.38, "densityKgM3": 420, "conductivityWmK": 0.15, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "upright-piano-catalogue"},
    {"id": "ebonised-sharps", "title": T("Ebonised wooden sharps, worn glossy", "Schwarz gebeizte Holz-Obertasten, glänzend abgegriffen"), "emissivity": 0.92, "solarAbsorptivity": 0.95, "densityKgM3": 750, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "source": "authored-estimate", "reference": "upright-piano-catalogue"},
    {"id": "brass-tarnished", "title": T("Cast brass of pedals and castors, tarnished, rubbed bright where touched", "Messingguss von Pedalen und Rollen, angelaufen, an den Trittflächen blank gerieben"), "emissivity": 0.35, "solarAbsorptivity": 0.55, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "elasticModulusPa": 1.0e11, "strengthPa": 2.0e8, "source": "literature", "reference": "incropera-2018"},
    {"id": "birch-plywood", "title": T("Birch plywood 21 mm of the dolly board", "Birkensperrholz 21 mm der Rollbrettplatte"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 680, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 9.0e9, "strengthPa": 4.0e7, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "needle-felt", "title": T("Grey needle felt on the dolly, worn through at the edges", "Grauer Nadelfilz auf dem Rollbrett, an den Kanten durchgescheuert"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": 250, "conductivityWmK": 0.06, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "piano-dolly-catalogue"},
    {"id": "polyurethane-castor", "title": T("Polyurethane castor tread", "Polyurethan-Laufbelag der Lenkrollen"), "emissivity": 0.9, "solarAbsorptivity": 0.85, "densityKgM3": 1200, "conductivityWmK": 0.25, "specificHeatJKgK": 1800, "source": "literature", "reference": "incropera-2018"},
    {"id": "galvanised-steel", "title": T("Galvanised steel of castor plates and the ratchet", "Verzinkter Stahl der Rollenplatten und der Ratsche"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 2.1e11, "strengthPa": 2.35e8, "source": "literature", "reference": "incropera-2018"},
    {"id": "ratchet-steel", "title": T("Pressed galvanised steel of the ratchet, effective density over its envelope", "Gepresster verzinkter Stahl der Ratsche, wirksame Dichte über ihre Hüllform"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 2600, "conductivityWmK": 50, "specificHeatJKgK": 470, "source": "authored-estimate", "reference": "en-12195-2-2000"},
    {"id": "polyester-webbing", "title": T("Polyester webbing 50 mm of the lashing strap, orange, faded", "Polyestergurtband 50 mm des Zurrgurts, orange, verblichen"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 900, "conductivityWmK": 0.14, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "en-12195-2-2000"},
    {"id": "beech-stained", "title": T("Solid beech of the bench, stained walnut, scuffed", "Massive Buche der Klavierbank, nussbaumfarben gebeizt, verschrammt"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 720, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 1.3e10, "strengthPa": 4.0e7, "source": "literature", "reference": "en-338-2016"},
    {"id": "vinyl-foam", "title": T("Brown vinyl over polyurethane foam, cracked and sagging", "Braunes Kunstleder über Polyurethanschaum, rissig und durchgesessen"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": 80, "conductivityWmK": 0.04, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "piano-bench-catalogue"},
]

REFERENCES = [
    {"id": "upright-piano-catalogue", "citation": "Catalogue-typical German upright pianos of 1965-1990 (West and East German production, the 125 cm class): 1.48-1.52 m wide, 0.58-0.62 m deep, 1.18-1.30 m high, 210-250 kg, 88 keys A0-C8 on a 1.22 m keyboard, white key tops 0.72-0.74 m above the floor, continental case without legs, two or three pedals (soft, practice moderator, sustain), walnut or mahogany veneer on chipboard; taken as the type, no brand."},
    {"id": "fletcher-rossing-1998", "citation": "Fletcher, N. H., Rossing, T. D. (1998) The Physics of Musical Instruments, 2nd ed., Springer, New York, chapter 12 (piano strings, total string tension of about 18-20 t on the iron frame, soundboard of spruce about 8-10 mm)."},
    {"id": "en-1561-2011", "citation": "EN 1561:2011 Founding - Grey cast irons (EN-GJL-200: tensile strength 200 MPa, modulus of elasticity about 100 GPa, density 7.2 g/cm3)."},
    {"id": "en-312-2010", "citation": "EN 312:2010 Particleboards - Specifications (type P2 for interior fitments including furniture in dry conditions; density about 650-700 kg/m3, bending strength 11 MPa and modulus 1.6-1.8 GPa for 20-25 mm)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (softwood C24: 24 MPa, 11 GPa; hardwood D40: 40 MPa, 13 GPa)."},
    {"id": "wagenfuehr-holzatlas", "citation": "Wagenführ, R., Wagenführ, A. (2022) Holzatlas, 7th ed., Hanser, München: spruce (Picea abies) 0.43-0.47 g/cm3, beech (Fagus sylvatica) 0.69-0.75 g/cm3, lime 0.49-0.53 g/cm3 at 12-15 % moisture content."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (softwood 450 kg/m3: 0.12 W/mK; hardwood 700 kg/m3: 0.18 W/mK; particleboard 600-700 kg/m3: 0.14 W/mK, 1700 J/kgK; plywood 0.13 W/mK)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018) Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.11: emissivity of wood about 0.9, oxidised brass 0.22-0.6, galvanised steel weathered about 0.28; Table A.1 brass 8530 kg/m3, 110 W/mK, 380 J/kgK)."},
    {"id": "en-1728-2012", "citation": "EN 1728:2012 Furniture - Seating - Test methods for the determination of strength and durability (seat static load, seat front edge load)."},
    {"id": "en-12520-2015", "citation": "EN 12520:2015 Furniture - Strength, durability and safety - Requirements for domestic seating (seat static load 1300 N as the strength level)."},
    {"id": "en-14749-2016", "citation": "EN 14749:2016 Furniture - Domestic and kitchen storage units and worktops - Safety requirements and test methods (stability of tall free-standing furniture against an overturning force at the top, used here as the frame for the tipping case of the piano)."},
    {"id": "en-12195-2-2000", "citation": "EN 12195-2:2000 Load restraint assemblies on road vehicles - Safety - Part 2: Web lashing made from man-made fibres (50 mm polyester lashing strap with ratchet, lashing capacity LC 2500 daN)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - General actions, densities, self-weight, imposed loads for buildings (gravity 9.80665 m/s2, densities Annex A)."},
    {"id": "piano-dolly-catalogue", "citation": "Catalogue-typical piano dolly for upright pianos (Klavierroller, Klavierrollbrett) as used by removal firms: carpet-covered birch plywood board 1.2-1.5 x 0.4-0.5 m on four swivel castors of 75 mm with polyurethane treads, load 300-400 kg, 10-15 kg."},
    {"id": "piano-bench-catalogue", "citation": "Catalogue-typical piano bench (Klavierbank) of the 1970s: 0.55-0.60 x 0.32-0.35 m, seat 0.47-0.50 m high, beech frame stained to match the piano, vinyl-covered foam seat, 5-8 kg."},
    {"id": "semio-used-household", "citation": "Semio staffage content, used household objects collection (2026-11-22): traces of use, storage state and carrying of a reuse-depot object are authored estimates of the typical case; nothing is measured."},
]

MASS_REF = {"case": "upright-piano-catalogue", "lid": "en-312-2010", "fallboard": "en-312-2010", "pedal-soft": "incropera-2018", "pedal-mute": "incropera-2018", "pedal-sustain": "incropera-2018", "dolly": "piano-dolly-catalogue", "strap": "en-12195-2-2000", "bench": "piano-bench-catalogue"}
MASS_MATERIAL = {"case": "cast-iron-frame", "lid": "walnut-veneer-bleached", "fallboard": "walnut-veneer-aged", "pedal-soft": "brass-tarnished", "pedal-mute": "brass-tarnished", "pedal-sustain": "brass-tarnished", "dolly": "birch-plywood", "strap": "polyester-webbing", "bench": "beech-stained"}

BONES = [
    {"id": "floor", "parent": None, "title": T("Floor of the depot hall", "Boden der Lagerhalle"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "dolly-roll", "parent": "floor", "title": T("Rolling travel of the dolly with the strapped piano", "Rollweg des Rollbretts mit dem verzurrten Klavier"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.0, "maxM": 0.2}},
    {"id": "dolly-tilt", "parent": "dolly-roll", "title": T("Tilt about the line of the back dolly castors (over a door sill)", "Kippung um die Linie der hinteren Rollbrettrollen (über eine Türschwelle)"), "x": DOLLY_CASTOR_X[0], "y": 0, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 8}},
    {"id": "dolly", "parent": "dolly-tilt", "title": T("Piano dolly: carpeted board on four swivel castors (parked behind the piano in use)", "Klavierroller: Platte mit Filzbelag auf vier Lenkrollen (in Gebrauch hinter dem Klavier abgestellt)"), "x": -DOLLY_CASTOR_X[0], "y": 0, "angleDeg": 0, "lengthM": DOLLY_X1, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.8, "maxM": 0}},
    {"id": "strap-drop", "parent": "dolly", "title": T("Height of the lashing strap (lowered to the floor when released)", "Höhe des Zurrgurts (beim Lösen auf den Boden gelegt)"), "x": STRAP_X, "y": STRAP_Y, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.07, "maxM": 0}},
    {"id": "strap", "parent": "strap-drop", "title": T("Ratchet lashing strap round piano and dolly (laid down backwards when released)", "Zurrgurt mit Ratsche um Klavier und Rollbrett (beim Lösen nach hinten abgelegt)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.05, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 90}},
    {"id": "piano-level", "parent": "dolly-tilt", "title": T("Standing level of the piano (on the dolly or on the floor)", "Standhöhe des Klaviers (auf dem Rollbrett oder auf dem Boden)"), "x": -DOLLY_CASTOR_X[0], "y": DOLLY_TOP, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -DOLLY_TOP, "maxM": 0.2}},
    {"id": "case", "parent": "piano-level", "title": T("Case with back, iron frame, strings, action and keyboard", "Gehäuse mit Rasten, Gussplatte, Saiten, Mechanik und Klaviatur"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": D, "joint": {"type": "fixed"}},
    {"id": "lid", "parent": "case", "title": T("Top lid on its back hinge", "Deckel am hinteren Scharnier"), "x": 0, "y": LID_Y0, "angleDeg": 0, "lengthM": LID_X1, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 85}},
    {"id": "fallboard", "parent": "case", "title": T("Fallboard (key cover) on its hinge in front of the name board", "Tastenklappe an ihrem Scharnier vor der Namensleiste"), "x": FALL_HINGE[0], "y": FALL_HINGE[1], "angleDeg": 0, "lengthM": FALL_LEN, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 92}},
]
for pid, title in (("pedal-soft", T("Left pedal (soft)", "Linkes Pedal (Piano)")), ("pedal-mute", T("Middle pedal (practice moderator)", "Mittleres Pedal (Moderator)")), ("pedal-sustain", T("Right pedal (sustain)", "Rechtes Pedal (Forte)"))):
    BONES.append({"id": pid, "parent": "case", "title": title, "x": PEDAL_PIVOT[0], "y": PEDAL_PIVOT[1], "z": PEDAL_Z[pid], "angleDeg": 0, "lengthM": 0.32, "joint": {"type": "revolute", "minDeg": -4, "maxDeg": 0}})
BONES += [
    {"id": "bench-slide", "parent": "floor", "title": T("Position of the bench in front of the keyboard", "Stellung der Bank vor der Klaviatur"), "x": BENCH_X, "y": 0, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.6}},
    {"id": "bench", "parent": "bench-slide", "title": T("Piano bench (stays in the depot when the piano is rolled away)", "Klavierbank (bleibt im Lager, wenn das Klavier weggerollt wird)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": BENCH_D, "joint": {"type": "fixed"}},
]


def side_name(sgn):
    """↔️ The bilingual side name of a lateral sign (right is -z)."""
    return T("right", "rechts") if sgn < 0 else T("left", "links")


ANCHORS = []
for name, x in (("back", DOLLY_CASTOR_X[0]), ("front", DOLLY_CASTOR_X[1])):
    for sgn in (-1, 1):
        s = side_name(sgn)
        ANCHORS.append({"id": f"dolly-castor-{name}-{'r' if sgn < 0 else 'l'}", "bone": "dolly", "x": x, "y": 0.0, "z": r6(sgn * DOLLY_CASTOR_Z), "kind": "contact", "title": T(f"Dolly castor {name}, {s['en']}", f"Rollbrettrolle {'hinten' if name == 'back' else 'vorn'}, {s['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.03, "frictionCoefficient": 0.05})
for name, x in (("back", CASTOR_X[0]), ("front", CASTOR_X[1])):
    for sgn in (-1, 1):
        s = side_name(sgn)
        ANCHORS.append({"id": f"piano-castor-{name}-{'r' if sgn < 0 else 'l'}", "bone": "case", "x": x, "y": 0.0, "z": r6(sgn * CASTOR_Z), "kind": "contact", "title": T(f"Piano castor {name}, {s['en']}", f"Klavierrolle {'hinten' if name == 'back' else 'vorn'}, {s['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.01, "footprintWidthM": 0.018, "frictionCoefficient": 0.1})
for sgn in (-1, 1):
    s = side_name(sgn)
    ANCHORS.append({"id": f"grip-back-{'r' if sgn < 0 else 'l'}", "bone": "case", "x": 0.0, "y": 0.75, "z": r6(sgn * 0.62), "kind": "support", "title": T(f"Mover {s['en']}: hands in the carrying grip of the outer back post", f"Träger {s['de']}: Hände im Tragegriff der äußeren Raste"), "support": "pin"})
for name, x in (("back", 0.02), ("front", 0.31)):
    for sgn in (-1, 1):
        s = side_name(sgn)
        ANCHORS.append({"id": f"bench-foot-{name}-{'r' if sgn < 0 else 'l'}", "bone": "bench", "x": x, "y": 0.0, "z": r6(sgn * 0.26), "kind": "contact", "title": T(f"Bench leg {name}, {s['en']}", f"Bankbein {'hinten' if name == 'back' else 'vorn'}, {s['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.03, "footprintWidthM": 0.03, "frictionCoefficient": 0.4})
ANCHORS += [
    {"id": "bench-seat", "bone": "bench", "x": 0.165, "y": BENCH_TOP, "kind": "load", "title": T("Seat of the bench (pianist)", "Sitzfläche der Bank (Pianist)")},
    {"id": "sustain-foot", "bone": "pedal-sustain", "x": 0.31, "y": 0.01, "kind": "load", "title": T("Right foot on the sustain pedal", "Rechter Fuß auf dem Fortepedal")},
]
DOLLY_CASTORS = [a["id"] for a in ANCHORS if a["id"].startswith("dolly-castor")]
DOLLY_BACK = [a for a in DOLLY_CASTORS if "back" in a]
PIANO_CASTORS = [a["id"] for a in ANCHORS if a["id"].startswith("piano-castor")]
GRIPS = ["grip-back-r", "grip-back-l"]
BENCH_FEET = [a["id"] for a in ANCHORS if a["id"].startswith("bench-foot")]


class Dim:
    """📏 The case numbers one scale models to (thickness of sides, lid and front panels follow the smallest feature)."""

    def __init__(self, scale):
        self.s = scale
        self.st = {"50": 0.025, "100": 0.025, "200": 0.03, "500": 0.14}[scale]
        self.zi = W2 - self.st
        self.lid0 = {"50": LID_Y0, "100": LID_Y0, "200": 1.22, "500": 1.175}[scale]
        self.up0 = {"50": 0.34, "100": 0.34, "200": 0.33, "500": 0.33}[scale]
        self.knee0 = {"50": 0.40, "100": 0.40, "200": 0.39, "500": 0.39}[scale]
        self.bot1 = {"50": 0.06, "100": 0.06, "200": 0.065, "500": 0.06}[scale]
        self.foot = 0.0 if scale == "500" else 0.035


def side_profile(scale, worn=False):
    """🎹 The side panel in the (u, v) plane of the case: full depth below the keyboard, rounded front step at the cheek blocks, the narrow upper side up to the lid."""
    d = Dim(scale)
    top = d.lid0
    if scale == "500":
        return [(0.0, 0.0), (D, 0.0), (D, STEP_Y), (0.36, STEP_Y), (0.36, top), (0.0, top)]
    if scale == "200":
        return [(0.0, d.foot), (D, d.foot), (D, 0.77), (0.58, STEP_Y), (0.38, STEP_Y), (0.36, 0.81), (0.36, top), (0.0, top)]
    n = 5 if scale == "50" else 3
    front = [(D, d.foot)]
    if worn and scale == "50":
        front += [(D, 0.405), (0.589, 0.411), (0.587, 0.422), (D, 0.428)]
    corner = arc(0.58, 0.77, 0.02, 0, 90, n)
    fillet = arc(0.38, 0.81, 0.02, -90, -180, n)
    return [(0.0, d.foot)] + front + corner + fillet + [(0.36, top), (0.0, top)]


def cheek_profile(scale):
    """🪵 The cheek block beside the keys in the (u, v) plane, its front-top corner rounded and worn."""
    if scale == "500":
        return None
    n = {"50": 5, "100": 3, "200": 2}[scale]
    return [(0.36, KEYBED_Y1), (D, KEYBED_Y1)] + arc(0.58, 0.77, 0.02, 0, 90, n) + [(0.36, STEP_Y)]


def knee_profile(scale, notched=True):
    """🦵 The knee board in the (v, w) plane with the slots the pedals pass through."""
    d = Dim(scale)
    out = [(d.bot1, -d.zi), (KEYBED_Y0, -d.zi), (KEYBED_Y0, d.zi), (d.bot1, d.zi)]
    if not notched:
        return out
    if scale in ("50", "100"):
        for zp in (0.12, 0.0, -0.12):
            out += [(d.bot1, zp + 0.02), (0.115, zp + 0.02), (0.115, zp - 0.02), (d.bot1, zp - 0.02)]
    else:
        out += [(d.bot1, 0.15), (0.12, 0.15), (0.12, -0.15), (d.bot1, -0.15)]
    return out


PEDAL_PROFILE = [(-0.012, -0.010), (0.25, -0.010), (0.285, -0.012), (0.312, -0.012), (0.322, -0.006), (0.322, 0.004), (0.312, 0.010), (-0.012, 0.010)]
PEDAL_PROFILE_100 = [(-0.012, -0.010), (0.30, -0.010), (0.322, -0.004), (0.322, 0.004), (0.31, 0.010), (-0.012, 0.010)]
PEDAL_GROUP = {"200": [(0.14, -0.025), (0.322, -0.025), (0.322, 0.005), (0.14, 0.005)], "500": [(0.17, -0.05), (0.322, -0.05), (0.322, 0.025), (0.17, 0.025)]}
PEDAL_GROUP_HW = {"200": 0.135, "500": 0.135}


def fall_profile(scale):
    """🎹 The fallboard in the (u, v) plane of its hinge: leaf with the lip that covers the key fronts (a block at 1:500)."""
    if scale == "500":
        return [(0.0, -0.075), (FALL_LEN, -0.075), (FALL_LEN, 0.0), (0.0, 0.0)]
    t = 0.03 if scale == "200" else FALL_T
    lip0 = FALL_LEN - t
    if scale == "50":
        return [(0.0, -t), (lip0, -t), (lip0, -FALL_LIP), (FALL_LEN, -FALL_LIP), (FALL_LEN, -0.004), (FALL_LEN - 0.004, 0.0), (0.0, 0.0)]
    return [(0.0, -t), (lip0, -t), (lip0, -FALL_LIP), (FALL_LEN, -FALL_LIP), (FALL_LEN, 0.0), (0.0, 0.0)]


def lid_profile(scale):
    """🔝 The lid in the (u, v) plane of its back hinge, the front edge rounded by wear at 1:50."""
    d = Dim(scale)
    v0 = d.lid0 - LID_Y0
    t = 0.025
    if scale == "50":
        return [(0.0, v0), (LID_X1, v0)] + arc(LID_X1 - 0.008, 0.017, 0.008, 0, 90, 4) + [(0.0, t)]
    return [(0.0, v0), (LID_X1, v0), (LID_X1, t), (0.0, t)]


def body(scale):
    """🧊 The body solids of one scale."""
    return dolly_body(scale) + strap_body(scale) + case_body(scale) + bench_body(scale)


def dolly_body(scale):
    """🛞 The piano dolly: plywood board, needle felt, four swivel castors (one block at 1:500)."""
    b = "dolly"
    if scale == "500":
        return [box("dolly-board", b, "birch-plywood", 0.55, DOLLY_X0, DOLLY_X1, 0.0, DOLLY_TOP, -DOLLY_Z, DOLLY_Z)]
    out = []
    if scale == "200":
        out.append(box("dolly-board", b, "birch-plywood", 0.55, DOLLY_X0, DOLLY_X1, 0.083, DOLLY_TOP, -DOLLY_Z, DOLLY_Z))
    else:
        out.append(box("dolly-board", b, "birch-plywood", 0.55, DOLLY_X0, DOLLY_X1, BOARD_Y0, BOARD_Y1, -DOLLY_Z, DOLLY_Z))
        out.append(box("dolly-felt", b, "needle-felt", 0.3, DOLLY_X0, DOLLY_X1, BOARD_Y1, DOLLY_TOP, -DOLLY_Z, DOLLY_Z, sheet=True))
    for i, x in enumerate(DOLLY_CASTOR_X):
        for sgn in (-1, 1):
            zc = sgn * DOLLY_CASTOR_Z
            tag = f"{'back' if i == 0 else 'front'}-{'r' if sgn < 0 else 'l'}"
            if scale == "50":
                out.append(revolve(f"dolly-wheel-{tag}", b, "polyurethane-castor", 0.15, [(0.0, 0.0375), (0.03, 0.0375)], [0, 0, 1], [x, 0.0375, zc - 0.015]))
                out.append(box(f"dolly-plate-{tag}", b, "galvanised-steel", 0.7, x - 0.045, x + 0.045, 0.075, BOARD_Y0, zc - 0.045, zc + 0.045))
            elif scale == "100":
                out.append(box(f"dolly-castor-{tag}", b, "polyurethane-castor", 0.15, x - 0.0375, x + 0.0375, 0.0, BOARD_Y0, zc - 0.015, zc + 0.015))
            else:
                out.append(box(f"dolly-castor-{tag}", b, "polyurethane-castor", 0.15, x - 0.0375, x + 0.0375, 0.0, 0.083, zc - 0.015, zc + 0.015))
    return out


def strap_body(scale):
    """🪢 The ratchet lashing strap round piano and dolly: bottom bar under the dolly board, two sides, top bar on the lid, the ratchet on the right."""
    b = "strap"
    top = STRAP_TOP
    if scale == "500":
        kw = {"slender": True}
    else:
        kw = {"sheet": True}
    out = [
        box("strap-bottom", b, "polyester-webbing", 0.6, -STRAP_HW, STRAP_HW, 0.0, STRAP_T, -STRAP_ZO, STRAP_ZO, **kw),
        box("strap-side-r", b, "polyester-webbing", 0.6, -STRAP_HW, STRAP_HW, STRAP_T, top - STRAP_T, -STRAP_ZO, -STRAP_ZI, **kw),
        box("strap-side-l", b, "polyester-webbing", 0.6, -STRAP_HW, STRAP_HW, STRAP_T, top - STRAP_T, STRAP_ZI, STRAP_ZO, **kw),
        box("strap-top", b, "polyester-webbing", 0.6, -STRAP_HW, STRAP_HW, top - STRAP_T, top, -STRAP_ZO, STRAP_ZO, **kw),
    ]
    if scale != "500":
        out.append(box("strap-ratchet", b, "ratchet-steel", 0.2, -STRAP_HW, STRAP_HW, BUCKLE_V0, BUCKLE_V1, -BUCKLE_ZO, -STRAP_ZO))
    return out


def black_keys(scale, prefix="black-keys", bone="case", region=None):
    """🎼 The 36 sharps: B flat 0 and five per octave repeated over the seven octaves C1-B7."""
    out = []
    kw = {"region": region} if region else {}
    z = -KEY_Z + 1.1 * P_KEY
    out.append(box(f"{prefix}-bb0", bone, "ebonised-sharps", 0.05, BLACK_X0, BLACK_X1, KEYS_Y1, BLACK_Y1, z - BLACK_HW, z + BLACK_HW, **kw))
    for i, off in enumerate(BLACK_OFFSETS):
        z = -KEY_Z + (2 + off) * P_KEY
        out.append(with_repeat(box(f"{prefix}-{i + 1}", bone, "ebonised-sharps", 0.05, BLACK_X0, BLACK_X1, KEYS_Y1, BLACK_Y1, z - BLACK_HW, z + BLACK_HW, **kw), 7, (0, 0, 7 * P_KEY)))
    return out


def case_body(scale):
    """🎹 The case, its back, its works and the keyboard, the lid, the fallboard and the pedals of one scale."""
    d = Dim(scale)
    c = "case"
    V = "walnut-veneer-aged"
    out = []
    if scale == "500":
        mid = [(0.09, 0.0), (0.42, 0.0), (0.42, KEYBED_Y0), (D, KEYBED_Y0), (D, 0.688), (0.42, 0.688), (0.42, STEP_Y), (0.36, STEP_Y), (0.36, d.lid0), (0.09, d.lid0)]
        out += [
            extrude("case-mass", c, V, 0.38, mid, "xy", -CHEEK_ZI, CHEEK_ZI),
            extrude("side-cheek-r", c, V, 0.32, side_profile(scale), "xy", -W2, -CHEEK_ZI),
            extrude("side-cheek-l", c, V, 0.32, side_profile(scale), "xy", CHEEK_ZI, W2),
            box("back-block", c, "spruce-raw-aged", 0.6, 0.0, 0.09, 0.0, d.lid0, -CHEEK_ZI, CHEEK_ZI),
            extrude("lid", "lid", "walnut-veneer-bleached", 0.45, lid_profile(scale), "xy", -W2, W2),
            extrude("fallboard", "fallboard", V, 0.36, fall_profile(scale), "xy", -CHEEK_ZI, CHEEK_ZI),
            extrude("pedal-group", "pedal-mute", "brass-tarnished", 0.8, PEDAL_GROUP["500"], "xy", -PEDAL_GROUP_HW["500"], PEDAL_GROUP_HW["500"]),
        ]
        return out
    out += [
        extrude("side-r", c, V, 0.32, side_profile(scale, worn=True), "xy", -W2, -d.zi),
        extrude("side-l", c, V, 0.32, side_profile(scale), "xy", d.zi, W2),
        extrude("cheek-r", c, "beech-solid", 0.34, cheek_profile(scale), "xy", -d.zi, -CHEEK_ZI),
        extrude("cheek-l", c, "beech-solid", 0.34, cheek_profile(scale), "xy", CHEEK_ZI, d.zi),
        box("keybed", c, "beech-solid", 0.4, KEYBED_X0, D, KEYBED_Y0, KEYBED_Y1, -d.zi, d.zi),
        box("bottom-board", c, V, 0.4, 0.0, D, d.foot, d.bot1, -d.zi, d.zi),
        extrude("knee-board", c, V, 0.36, knee_profile(scale), "yz", d.knee0, 0.42),
        box("nameboard", c, V, 0.36, NAME_X0, NAME_X1, NAME_Y0, STEP_Y, -CHEEK_ZI, CHEEK_ZI),
        box("upper-panel", c, V, 0.36, d.up0, 0.36, STEP_Y, d.lid0, -d.zi, d.zi),
        extrude("lid", "lid", "walnut-veneer-bleached", 0.45, lid_profile(scale), "xy", -W2, W2),
        extrude("fallboard", "fallboard", V, 0.36, fall_profile(scale), "xy", -FALL_HZ, FALL_HZ),
    ]
    slip_x0 = 0.57 if scale == "200" else SLIP_X0
    out.append(box("keyslip", c, V, 0.34, slip_x0, D, KEYBED_Y1, SLIP_Y1, -CHEEK_ZI, CHEEK_ZI))
    if scale == "200":
        out += [
            extrude("back-frame", c, "spruce-raw-aged", 0.6, [(d.bot1, -d.zi), (d.lid0, -d.zi), (d.lid0, d.zi), (d.bot1, d.zi)], "yz", 0.0, 0.09, holes=[[(0.14, -0.655), (1.15, -0.655), (1.15, -0.275), (0.14, -0.275)], [(0.14, -0.205), (1.15, -0.205), (1.15, 0.205), (0.14, 0.205)], [(0.14, 0.275), (1.15, 0.275), (1.15, 0.655), (0.14, 0.655)]]),
            box("soundboard", c, "spruce-raw-aged", 0.7, 0.09, 0.099, 0.10, 1.18, -0.70, 0.70, sheet=True),
            box("works", c, "cast-iron-frame", 0.5, 0.099, 0.13, 0.12, 1.17, -0.68, 0.68),
            box("action", c, "action-effective", 0.5, 0.13, d.up0, 0.74, 1.06, -0.62, 0.62),
            box("keys", c, "keytop-acrylic-yellowed", 0.92, KEYS_X0, 0.57, KEYBED_Y1, 0.732, -KEY_Z, KEY_Z),
            extrude("pedal-group", "pedal-mute", "brass-tarnished", 0.8, PEDAL_GROUP["200"], "xy", -PEDAL_GROUP_HW["200"], PEDAL_GROUP_HW["200"]),
        ]
        for x in CASTOR_X:
            for sgn in (-1, 1):
                zc = sgn * CASTOR_Z
                out.append(box(f"castor-{'back' if x < 0.3 else 'front'}-{'r' if sgn < 0 else 'l'}", c, "brass-tarnished", 0.75, x - 0.015, x + 0.015, 0.0, d.foot, zc - 0.015, zc + 0.015))
        return out
    out += [
        box("beam-bottom", c, "spruce-raw-aged", 0.6, 0.0, 0.09, d.bot1, 0.14, -d.zi, d.zi),
        box("beam-top", c, "spruce-raw-aged", 0.6, 0.0, 0.09, 1.15, LID_Y0, -d.zi, d.zi),
    ]
    for i, zc in enumerate((-0.69, -0.24, 0.24, 0.69)):
        out.append(box(f"post-{i + 1}", c, "spruce-raw-aged", 0.6, 0.0, 0.09, 0.14, 1.15, zc - 0.035, zc + 0.035))
    out += [
        box("soundboard", c, "spruce-raw-aged", 0.7, 0.09, 0.099, 0.10, 1.18, -0.70, 0.70, sheet=True),
        box("strings", c, "string-plane", 0.3, 0.099, 0.115, 0.16, 1.00, -0.66, 0.66),
        box("pin-block", c, "beech-multiplex", 0.4, 0.099, 0.115, 1.00, 1.15, -0.66, 0.66),
        extrude("iron-frame", c, "cast-iron-frame", 0.5, [(0.12, -0.68), (1.17, -0.68), (1.17, 0.68), (0.12, 0.68)], "yz", 0.115, 0.13, holes=[[(0.19, -0.62), (0.63, -0.62), (0.63, -0.06), (0.19, -0.06)], [(0.19, 0.04), (0.63, 0.04), (0.63, 0.60), (0.19, 0.60)], [(0.69, -0.62), (0.955, -0.62), (0.955, -0.06), (0.69, -0.06)], [(0.69, 0.04), (0.955, 0.04), (0.955, 0.60), (0.69, 0.60)]]),
        box("action", c, "action-effective", 0.5, 0.13, 0.33, 0.74, 1.06, -0.62, 0.62),
        box("keys", c, "keytop-acrylic-yellowed", 0.92, KEYS_X0, KEYS_X1, KEYBED_Y1, KEYS_Y1 if scale == "50" else BLACK_Y1, -KEY_Z, KEY_Z),
        box("pedal-rail", c, "beech-solid", 0.4, 0.235, 0.265, d.bot1, 0.08, -0.16, 0.16),
    ]
    if scale == "50":
        out += black_keys(scale)
    prof = PEDAL_PROFILE if scale == "50" else PEDAL_PROFILE_100
    for pid in PEDAL_Z:
        out.append(extrude(pid, pid, "brass-tarnished", 0.8, prof, "xy", -PEDAL_HW, PEDAL_HW))
    for x in CASTOR_X:
        for sgn in (-1, 1):
            zc = sgn * CASTOR_Z
            tag = f"{'back' if x < 0.3 else 'front'}-{'r' if sgn < 0 else 'l'}"
            if scale == "50":
                out.append(revolve(f"castor-wheel-{tag}", c, "brass-tarnished", 0.75, [(0.0, 0.015), (0.018, 0.015)], [0, 0, 1], [x, 0.015, zc - 0.009]))
                out.append(box(f"castor-housing-{tag}", c, "brass-tarnished", 0.75, x - 0.016, x + 0.016, 0.022, d.foot, zc - 0.012, zc + 0.012))
            else:
                out.append(box(f"castor-{tag}", c, "brass-tarnished", 0.75, x - 0.015, x + 0.015, 0.0, d.foot, zc - 0.009, zc + 0.009))
    return out


def bench_body(scale):
    """🪑 The piano bench: four tapered legs, rails, seat board and the sagging vinyl cushion (merged masses at 1:200 and 1:500)."""
    b = "bench"
    M = "beech-stained"
    out = []
    if scale == "500":
        u = [(0.0, -BENCH_HW), (0.40, -BENCH_HW), (0.40, BENCH_HW), (0.0, BENCH_HW), (0.0, 0.205), (0.325, 0.205), (0.325, -0.205), (0.0, -0.205)]
        return [
            extrude("bench-frame", b, M, 0.35, u, "yz", 0.003, BENCH_D),
            box("bench-cushion", b, "vinyl-foam", 0.25, 0.0, BENCH_D, 0.40, BENCH_TOP, -BENCH_HW, BENCH_HW),
        ]
    for i, u0 in enumerate((0.0, BENCH_D - 0.04)):
        for sgn in (-1, 1):
            w0, w1 = sorted((sgn * 0.24, sgn * BENCH_HW))
            tag = f"{'back' if i == 0 else 'front'}-{'r' if sgn < 0 else 'l'}"
            sq = [(w0, u0), (w1, u0), (w1, u0 + 0.04), (w0, u0 + 0.04)]
            if scale == "200":
                out.append(extrude(f"bench-leg-{tag}", b, M, 0.35, [(w, u + (0.003 if i == 0 else 0.0)) for w, u in sq], "zx", 0.0, 0.33))
            else:
                out.append(extrude(f"bench-leg-{tag}", b, M, 0.35, sq, "zx", 0.0, 0.40, taper=(0.75, 1.0)))
    if scale == "200":
        out.append(box("bench-board", b, M, 0.35, 0.0, BENCH_D, 0.33, BENCH_SEAT_Y, -BENCH_HW, BENCH_HW))
        out.append(box("bench-cushion", b, "vinyl-foam", 0.25, 0.005, BENCH_D - 0.005, BENCH_SEAT_Y, BENCH_TOP, -0.275, 0.275))
        return out
    out += [
        box("bench-rail-back", b, M, 0.35, 0.0, 0.02, 0.33, 0.40, -0.24, 0.24),
        box("bench-rail-front", b, M, 0.35, BENCH_D - 0.02, BENCH_D, 0.33, 0.40, -0.24, 0.24),
        box("bench-rail-r", b, M, 0.35, 0.04, BENCH_D - 0.04, 0.33, 0.40, -BENCH_HW, -0.26),
        box("bench-rail-l", b, M, 0.35, 0.04, BENCH_D - 0.04, 0.33, 0.40, 0.26, BENCH_HW),
        box("bench-board", b, M, 0.35, 0.0, BENCH_D, 0.40, BENCH_SEAT_Y, -BENCH_HW, BENCH_HW),
    ]
    out.append(extrude("bench-cushion", b, "vinyl-foam", 0.25, cushion_profile(scale), "yz", 0.005, BENCH_D - 0.005))
    return out


def cushion_profile(scale):
    """🛋️ The cushion in the (v, w) plane: rounded edges, the seat sagging 12 mm in the middle where pianists sat (flat from 1:100)."""
    if scale == "50":
        return [(BENCH_SEAT_Y, -0.275), (0.462, -0.275), (0.470, -0.27), (0.475, -0.255), (0.475, -0.17), (0.469, -0.10), (0.464, -0.04), (0.463, 0.0), (0.464, 0.04), (0.469, 0.10), (0.475, 0.17), (0.475, 0.255), (0.470, 0.27), (0.462, 0.275), (BENCH_SEAT_Y, 0.275)]
    return [(BENCH_SEAT_Y, -0.275), (BENCH_TOP, -0.275), (BENCH_TOP, 0.275), (BENCH_SEAT_Y, 0.275)]


REGION_PREFIX = [
    ("lid", "lid"), ("fallboard", "fallboard"), ("pedal", "brass-fittings"), ("castor", "brass-fittings"),
    ("dolly", "dolly"), ("strap", "strap"), ("bench-cushion", "bench-seat"), ("bench", "bench"),
    ("back", "back"), ("beam", "back"), ("post", "back"), ("soundboard", "back"),
    ("side", "case"), ("cheek", "case"), ("keybed", "case"), ("bottom-board", "case"), ("nameboard", "case"), ("upper-panel", "case"), ("keyslip", "case"), ("case-mass", "case"),
]
INSIDE = ("iron-frame", "strings", "pin-block", "action", "keys", "black-keys", "works", "pedal-rail", "knee-board")


def region_of(sid):
    """🌡️ The thermal region a body solid references (None: inside the case, the hidden keyboard, or closed by an own thermal solid)."""
    if sid.startswith(INSIDE):
        return None
    for p, reg in REGION_PREFIX:
        if sid.startswith(p):
            return reg
    return None


def structural(sid):
    """🏗️ Whether a body solid is a member of the structure list (the sharps are no load path)."""
    return not sid.startswith("black-keys")


def thermal_extra(scale):
    """🔥 Own thermal solids: the knee board without pedal slots (closed by felt in reality), the hidden keyboard as real solids inside the case."""
    d = Dim(scale)
    out = []
    if scale != "500":
        out.append(extrude("th-knee-board", "case", "walnut-veneer-aged", 0.36, knee_profile(scale, notched=False), "yz", d.knee0, 0.42, region="case"))
    if scale == "500":
        out.append(box("th-keys", "case", "keytop-acrylic-yellowed", 0.92, KEYS_X0, 0.42, 0.613, 0.688, -KEY_Z, KEY_Z, region="keyboard"))
    elif scale == "200":
        out.append(box("th-keys", "case", "keytop-acrylic-yellowed", 0.92, KEYS_X0, 0.57, KEYBED_Y1, 0.732, -KEY_Z, KEY_Z, region="keyboard"))
    else:
        out.append(box("th-keys", "case", "keytop-acrylic-yellowed", 0.92, KEYS_X0, KEYS_X1, KEYBED_Y1, KEYS_Y1 if scale == "50" else BLACK_Y1, -KEY_Z, KEY_Z, region="keyboard"))
        if scale == "50":
            out += black_keys(scale, prefix="th-black-keys", region="keyboard")
    return out


AREAS = {"case": 4.2, "lid": 1.2, "fallboard": 0.5, "keyboard": 1.6, "back": 2.2, "brass-fittings": 0.08, "dolly": 1.45, "strap": 0.28, "bench": 0.55, "bench-seat": 0.25}


def layer(lid, title, kind, mat, d):
    """🧶 One layer: thickness, resistance d/k, areal heat capacity rho c d."""
    m = next(x for x in MATERIALS if x["id"] == mat)
    return {"id": lid, "title": title, "kind": kind, "material": mat, "thicknessM": d, "resistanceM2KW": r6(d / m["conductivityWmK"]), "heatCapacityJM2K": round(m["densityKgM3"] * m["specificHeatJKgK"] * d), "source": "literature", "reference": m["reference"]}


def region(rid, title, mat, ref_, length, geometry, sky, lay, areas, **kw):
    """🌡️ One passive region."""
    r = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": mat, "areaM2": round(areas[rid], 4), "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": ref_}
    r.update(kw)
    return r


def regions(areas):
    """🌡️ The ten passive regions of piano, dolly, strap and bench."""
    return [
        region("case", T("Case: side panels, cheek blocks, fronts, keyslip and bottom", "Gehäuse: Seitenwände, Backen, Füllungen, Vorsatzleiste und Boden"), "walnut-veneer-aged", "en-312-2010", 1.25, "vertical-plate", 0.4, layer("case-board", T("Veneered chipboard 25 mm", "Furnierte Spanplatte 25 mm"), "timber", "walnut-veneer-aged", 0.025), areas, ratedPowerW=0, ratedPowerBasis="passive"),
        region("lid", T("Top lid, veneer bleached by sun", "Deckel, Furnier von der Sonne ausgebleicht"), "walnut-veneer-bleached", "en-312-2010", 0.375, "horizontal-plate-up", 0.5, layer("lid-board", T("Veneered chipboard 25 mm", "Furnierte Spanplatte 25 mm"), "timber", "walnut-veneer-bleached", 0.025), areas),
        region("fallboard", T("Fallboard with its scratched lacquer", "Tastenklappe mit zerkratztem Lack"), "walnut-veneer-aged", "en-312-2010", 0.177, "horizontal-plate-up", 0.4, layer("fall-board", T("Veneered board 18 mm", "Furniertes Brett 18 mm"), "timber", "walnut-veneer-aged", 0.018), areas),
        region("keyboard", T("Keyboard: 52 yellowed naturals and 36 sharps", "Klaviatur: 52 vergilbte Untertasten und 36 Obertasten"), "keytop-acrylic-yellowed", "upright-piano-catalogue", 0.15, "horizontal-plate-up", 0.3, layer("key-levers", T("Key levers of lime wood 22 mm", "Tastenhebel aus Linde 22 mm"), "timber", "keytop-acrylic-yellowed", 0.022), areas, hidden=T("The keys lie between the cheek blocks behind the side panels and, in the default pose, under the closed fallboard, so the side elevation never shows them; they show in front and plan when the fallboard is open.", "Die Tasten liegen zwischen den Backen hinter den Seitenwänden und in der Grundpose unter der geschlossenen Tastenklappe, die Seitenansicht zeigt sie daher nie; Vorderansicht und Draufsicht zeigen sie bei offener Klappe.")),
        region("back", T("Back posts and soundboard, raw spruce", "Rasten und Resonanzboden, rohe Fichte"), "spruce-raw-aged", "wagenfuehr-holzatlas", 1.2, "vertical-plate", 0.4, layer("soundboard", T("Spruce soundboard 9 mm", "Resonanzboden aus Fichte 9 mm"), "timber", "spruce-raw-aged", 0.009), areas),
        region("brass-fittings", T("Brass pedals and castors", "Messingpedale und -rollen"), "brass-tarnished", "incropera-2018", 0.05, "horizontal-plate-up", 0.2, layer("brass", T("Cast brass 20 mm", "Messingguss 20 mm"), "metal", "brass-tarnished", 0.02), areas),
        region("dolly", T("Piano dolly: needle felt, plywood edges, castors", "Klavierroller: Nadelfilz, Sperrholzkanten, Rollen"), "needle-felt", "piano-dolly-catalogue", 0.48, "horizontal-plate-up", 0.3, layer("felt", T("Needle felt 7 mm", "Nadelfilz 7 mm"), "polymer", "needle-felt", 0.007), areas),
        region("strap", T("Lashing strap with ratchet", "Zurrgurt mit Ratsche"), "polyester-webbing", "en-12195-2-2000", 0.05, "vertical-plate", 0.4, layer("webbing", T("Polyester webbing 1.5 mm", "Polyestergurtband 1,5 mm"), "polymer", "polyester-webbing", 0.0015), areas),
        region("bench", T("Bench frame of stained beech", "Bankgestell aus gebeizter Buche"), "beech-stained", "en-338-2016", 0.33, "vertical-plate", 0.4, layer("bench-wood", T("Beech 20 mm", "Buche 20 mm"), "timber", "beech-stained", 0.02), areas),
        region("bench-seat", T("Vinyl seat cushion, cracked", "Sitzpolster aus Kunstleder, rissig"), "vinyl-foam", "piano-bench-catalogue", 0.33, "horizontal-plate-up", 0.5, layer("foam", T("Vinyl over foam 60 mm", "Kunstleder über Schaum 60 mm"), "polymer", "vinyl-foam", 0.06), areas),
    ]


PENS = {
    "50": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5},
    "100": {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.35},
    "200": {"body": 0.25, "detail": 0.18, "texture": None, "glazing": None, "ground": 0.25},
    "500": {"body": 0.25, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}
Y0 = DOLLY_TOP


def elevation(scale):
    """🖼️ Side elevation from the right: dolly, castors, right side panel, lid, the strap over it, the bench; works, keys, pedals and fallboard drawn behind the side so they appear when they move."""
    d = Dim(scale)
    out = dolly_elevation(scale)
    out.append(rect("back-edge", "case", 0.0, d.bot1 if scale != "500" else 0.0, 0.09, d.lid0, z=1.0, tone=0.6, region="back", material="spruce-raw-aged"))
    out.append(poly("fallboard", "fallboard", fall_profile(scale), z=1.3, tone=0.36, region="fallboard", material="walnut-veneer-aged"))
    if scale in ("50", "100"):
        prof = PEDAL_PROFILE if scale == "50" else PEDAL_PROFILE_100
        for pid in PEDAL_Z:
            out.append(poly(pid, pid, prof, z=1.2 + 0.01 * PEDAL_Z[pid], tone=0.8, region="brass-fittings", material="brass-tarnished"))
        for x in CASTOR_X:
            tag = "back" if x < 0.3 else "front"
            if scale == "50":
                out.append(rect(f"castor-housing-{tag}-r", "case", x - 0.016, 0.022, x + 0.016, d.foot, z=5.4, tone=0.75, region="brass-fittings", material="brass-tarnished"))
                out.append(circle(f"castor-wheel-{tag}-r", "case", x, 0.015, 0.015, role="body", z=5.5, tone=0.75, region="brass-fittings", material="brass-tarnished"))
            else:
                out.append(rect(f"castor-{tag}-r", "case", x - 0.015, 0.0, x + 0.015, d.foot, z=5.5, tone=0.75, region="brass-fittings", material="brass-tarnished"))
    else:
        out.append(poly("pedal-group", "pedal-mute", PEDAL_GROUP[scale], z=1.2, tone=0.8, region="brass-fittings", material="brass-tarnished"))
        if scale == "200":
            for x in CASTOR_X:
                out.append(rect(f"castor-{'back' if x < 0.3 else 'front'}-r", "case", x - 0.015, 0.0, x + 0.015, d.foot, z=5.5, tone=0.75, region="brass-fittings", material="brass-tarnished"))
    if scale == "500":
        out.append(poly("side-cheek", "case", side_profile(scale), z=6, tone=0.32, region="case", material="walnut-veneer-aged"))
    else:
        out.append(poly("side", "case", side_profile(scale, worn=True), z=6, tone=0.32, region="case", material="walnut-veneer-aged"))
    out.append(poly("lid", "lid", lid_profile(scale), z=6.5, tone=0.45, region="lid", material="walnut-veneer-bleached"))
    if scale == "50":
        for i, (x0, x1) in enumerate(((0.06, 0.08), (0.18, 0.21), (0.31, 0.33), (0.45, 0.47))):
            top = 1.2 if x1 < 0.36 else 0.76
            out.append(tex(f"side-grain-{i + 1}", "case", [(x0, 0.06), ((x0 + x1) / 2 + 0.008, (0.06 + top) / 2), (x1, top)], z=6.2))
        out.append(tex("side-scuff-foot", "case", [(0.53, 0.045), (0.56, 0.05), (0.595, 0.047)], z=6.2))
        out.append(tex("side-scuff-cheek", "case", [(0.555, 0.785), (0.585, 0.778), (0.597, 0.765)], z=6.2))
        out.append(rect("side-sticker", "case", 0.48, 0.30, 0.53, 0.33, role="detail", z=6.3, tone=0.9))
        out.append(tex("lid-edge-wear", "lid", [(0.30, 0.024), (0.36, 0.023), (0.372, 0.016)], z=6.6))
        out.append(tex("side-water-stain", "case", [(0.22, 0.96), (0.26, 0.975), (0.30, 0.968), (0.31, 0.95)], z=6.2))
    if scale == "100":
        out.append(tex("side-grain-1", "case", [(0.18, 0.06), (0.205, 0.63), (0.21, 1.2)], z=6.2))
    out += strap_elevation(scale)
    out += bench_elevation(scale)
    return out


def dolly_elevation(scale):
    """🛞 The dolly from the right: board, felt, castors."""
    b = "dolly"
    out = []
    if scale == "500":
        return [rect("dolly-board", b, DOLLY_X0, 0.0, DOLLY_X1, DOLLY_TOP, z=2, tone=0.55, region="dolly", material="birch-plywood")]
    if scale == "200":
        out.append(rect("dolly-board", b, DOLLY_X0, 0.083, DOLLY_X1, DOLLY_TOP, z=2, tone=0.55, region="dolly", material="birch-plywood"))
    else:
        out.append(rect("dolly-board", b, DOLLY_X0, BOARD_Y0, DOLLY_X1, BOARD_Y1, z=2, tone=0.55, region="dolly", material="birch-plywood"))
        out.append(rect("dolly-felt", b, DOLLY_X0, BOARD_Y1, DOLLY_X1, DOLLY_TOP, z=2.1, tone=0.3, region="dolly", material="needle-felt"))
    for i, x in enumerate(DOLLY_CASTOR_X):
        tag = f"{'back' if i == 0 else 'front'}-r"
        if scale == "50":
            out.append(rect(f"dolly-plate-{tag}", b, x - 0.045, 0.075, x + 0.045, BOARD_Y0, z=2.3, tone=0.7, region="dolly", material="galvanised-steel"))
            out.append(circle(f"dolly-wheel-{tag}", b, x, 0.0375, 0.0375, role="body", z=2.4, tone=0.15, region="dolly", material="polyurethane-castor"))
        else:
            out.append(rect(f"dolly-castor-{tag}", b, x - 0.0375, 0.0, x + 0.0375, BOARD_Y0 if scale == "100" else 0.083, z=2.4, tone=0.15, region="dolly", material="polyurethane-castor"))
    if scale == "50":
        out.append(tex("dolly-felt-worn", b, [(0.06, 0.11), (0.12, 0.111), (0.20, 0.11)], z=2.2))
    return out


def strap_elevation(scale):
    """🪢 The strap from the right: the right side of the loop over the side panel, the ratchet (an open stroke at true width at 1:500)."""
    b = "strap"
    if scale == "500":
        return [poly("strap-side-r", b, [(0.0, 0.0), (0.0, STRAP_TOP)], role="body", z=8, tone=0.6, closed=False, filled=False, region="strap", material="polyester-webbing", pen=0.1)]
    out = [rect("strap-side-r", b, -STRAP_HW, 0.0, STRAP_HW, STRAP_TOP, z=8, tone=0.6, region="strap", material="polyester-webbing")]
    out.append(rect("strap-ratchet", b, -STRAP_HW, BUCKLE_V0, STRAP_HW, BUCKLE_V1, z=8.5, tone=0.2, region="strap", material="ratchet-steel"))
    if scale == "50":
        out.append(det("strap-ratchet-lever", b, [(-0.02, 0.62), (0.02, 0.70)], z=8.6))
    return out


def bench_elevation(scale):
    """🪑 The bench from the right: legs, side rail, seat board, cushion."""
    b = "bench"
    if scale == "500":
        return [rect("bench-frame", b, 0.003, 0.0, BENCH_D, 0.40, z=3, tone=0.35, region="bench", material="beech-stained"), rect("bench-cushion", b, 0.0, 0.40, BENCH_D, BENCH_TOP, z=3.3, tone=0.25, region="bench-seat", material="vinyl-foam")]
    out = []
    top = 0.33 if scale == "200" else 0.40
    for i, u0 in enumerate((0.0, BENCH_D - 0.04)):
        tag = f"{'back' if i == 0 else 'front'}-r"
        if scale == "200":
            out.append(rect(f"bench-leg-{tag}", b, u0 + (0.003 if i == 0 else 0.0), 0.0, u0 + 0.04, top, z=3, tone=0.35, region="bench", material="beech-stained"))
        else:
            out.append(poly(f"bench-leg-{tag}", b, [(u0 + 0.005, 0.0), (u0 + 0.035, 0.0), (u0 + 0.04, top), (u0, top)], z=3, tone=0.35, region="bench", material="beech-stained"))
    if scale == "200":
        out.append(rect("bench-board", b, 0.0, 0.33, BENCH_D, BENCH_SEAT_Y, z=3.1, tone=0.35, region="bench", material="beech-stained"))
    else:
        out.append(rect("bench-rail-r", b, 0.04, 0.33, BENCH_D - 0.04, 0.40, z=3.1, tone=0.35, region="bench", material="beech-stained"))
        out.append(rect("bench-board", b, 0.0, 0.40, BENCH_D, BENCH_SEAT_Y, z=3.2, tone=0.35, region="bench", material="beech-stained"))
    out.append(rect("bench-cushion", b, 0.005, BENCH_SEAT_Y, BENCH_D - 0.005, BENCH_TOP, z=3.3, tone=0.25, region="bench-seat", material="vinyl-foam", **({"r": 0.01} if scale == "50" else {})))
    if scale == "50":
        out.append(tex("bench-cushion-crack", b, [(0.08, 0.47), (0.13, 0.466), (0.17, 0.471)], z=3.4))
        out.append(tex("bench-leg-scuff", b, [(BENCH_D - 0.035, 0.03), (BENCH_D - 0.005, 0.07)], z=3.4))
    return out


def front(scale):
    """🪟 Front elevation from ahead (paper x = lateral, the asset's left on the right): dolly, castors, case fronts, pedals, keys under the fallboard, the lid edge, the strap round it, the bench in front."""
    d = Dim(scale)
    out = dolly_front(scale)
    if scale == "500":
        out += [
            rect("side-cheek-r", "case", -W2, Y0, -CHEEK_ZI, Y0 + d.lid0, z=6.0, tone=0.32, region="case", material="walnut-veneer-aged"),
            rect("side-cheek-l", "case", CHEEK_ZI, Y0, W2, Y0 + d.lid0, z=6.0, tone=0.32, region="case", material="walnut-veneer-aged"),
            rect("case-mass", "case", -CHEEK_ZI, Y0, CHEEK_ZI, Y0 + d.lid0, z=4.0, tone=0.38, region="case", material="walnut-veneer-aged"),
            rect("keyslip", "case", -CHEEK_ZI, Y0 + KEYBED_Y0, CHEEK_ZI, Y0 + 0.688, z=6.0, tone=0.34, region="case", material="walnut-veneer-aged"),
            swept("fallboard", "fallboard", fall_profile(scale), -CHEEK_ZI, CHEEK_ZI, z=5.97, tone=0.36, region="fallboard", material="walnut-veneer-aged"),
            swept("pedal-group", "pedal-mute", PEDAL_GROUP[scale], -PEDAL_GROUP_HW[scale], PEDAL_GROUP_HW[scale], z=5.7, tone=0.8, region="brass-fittings", material="brass-tarnished"),
            swept("lid", "lid", lid_profile(scale), -W2, W2, z=3.75, tone=0.45, region="lid", material="walnut-veneer-bleached"),
        ]
        return out + strap_front(scale) + bench_front(scale)
    for x in CASTOR_X:
        if x < 0.3:
            continue
        for sgn in (-1, 1):
            zc = sgn * CASTOR_Z
            tag = f"front-{'r' if sgn < 0 else 'l'}"
            if scale == "50":
                out.append(rect(f"castor-housing-{tag}", "case", zc - 0.012, Y0 + 0.022, zc + 0.012, Y0 + d.foot, z=5.0, tone=0.75, region="brass-fittings", material="brass-tarnished"))
                out.append(rect(f"castor-wheel-{tag}", "case", zc - 0.009, Y0, zc + 0.009, Y0 + 0.03, z=5.05, tone=0.75, region="brass-fittings", material="brass-tarnished"))
            else:
                out.append(rect(f"castor-{tag}", "case", zc - (0.015 if scale == "200" else 0.009), Y0, zc + (0.015 if scale == "200" else 0.009), Y0 + d.foot, z=5.0, tone=0.75, region="brass-fittings", material="brass-tarnished"))
    out += [
        rect("side-r", "case", -W2, Y0 + d.foot, -d.zi, Y0 + d.lid0, z=6.0, tone=0.32, region="case", material="walnut-veneer-aged"),
        rect("side-l", "case", d.zi, Y0 + d.foot, W2, Y0 + d.lid0, z=6.0, tone=0.32, region="case", material="walnut-veneer-aged"),
        rect("bottom-board", "case", -d.zi, Y0 + d.foot, d.zi, Y0 + d.bot1, z=6.0, tone=0.4, region="case", material="walnut-veneer-aged"),
        poly("knee-board", "case", [(w, Y0 + v) for v, w in knee_profile(scale)], z=4.2, tone=0.36, region="case", material="walnut-veneer-aged"),
        rect("keybed", "case", -d.zi, Y0 + KEYBED_Y0, d.zi, Y0 + KEYBED_Y1, z=6.0, tone=0.4, region="case", material="walnut-veneer-aged"),
        rect("cheek-r", "case", -d.zi, Y0 + KEYBED_Y1, -CHEEK_ZI, Y0 + STEP_Y, z=6.0, tone=0.34, region="case", material="walnut-veneer-aged"),
        rect("cheek-l", "case", CHEEK_ZI, Y0 + KEYBED_Y1, d.zi, Y0 + STEP_Y, z=6.0, tone=0.34, region="case", material="walnut-veneer-aged"),
        rect("keyslip", "case", -(d.zi if scale == "200" else CHEEK_ZI), Y0 + KEYBED_Y1, d.zi if scale == "200" else CHEEK_ZI, Y0 + SLIP_Y1, z=6.0, tone=0.34, region="case", material="walnut-veneer-aged"),
        rect("keys", "case", -KEY_Z, Y0 + KEYBED_Y1, KEY_Z, Y0 + {"50": KEYS_Y1, "100": BLACK_Y1, "200": 0.732}[scale], z=5.85, tone=0.92, region="keyboard", material="keytop-acrylic-yellowed"),
        rect("nameboard", "case", -CHEEK_ZI, Y0 + (SLIP_Y1 if scale == "200" else NAME_Y0), CHEEK_ZI, Y0 + STEP_Y, z=4.2, tone=0.36, region="case", material="walnut-veneer-aged"),
        rect("upper-panel", "case", -d.zi, Y0 + STEP_Y, d.zi, Y0 + d.lid0, z=3.6, tone=0.36, region="case", material="walnut-veneer-aged"),
        swept("fallboard", "fallboard", fall_profile(scale), -FALL_HZ, FALL_HZ, z=5.97, tone=0.36, region="fallboard", material="walnut-veneer-aged"),
        swept("lid", "lid", lid_profile(scale), -W2, W2, z=3.75, tone=0.45, region="lid", material="walnut-veneer-bleached"),
    ]
    if scale in ("50", "100"):
        prof = PEDAL_PROFILE if scale == "50" else PEDAL_PROFILE_100
        for pid, zp in PEDAL_Z.items():
            out.append(swept(pid, pid, prof, zp - PEDAL_HW, zp + PEDAL_HW, z=5.7, tone=0.8, region="brass-fittings", material="brass-tarnished"))
    else:
        out.append(swept("pedal-group", "pedal-mute", PEDAL_GROUP[scale], -PEDAL_GROUP_HW[scale], PEDAL_GROUP_HW[scale], z=5.7, tone=0.8, region="brass-fittings", material="brass-tarnished"))
    if scale == "50":
        for i in range(1, 52):
            z = -KEY_Z + i * P_KEY
            out.append(det(f"key-gap-{i}", "case", [(z, Y0 + SLIP_Y1), (z, Y0 + KEYS_Y1)], z=5.86, tone=0.5))
        for i, zc in enumerate((-0.45, 0.0, 0.45)):
            out.append(det(f"upper-panel-moulding-{i + 1}", "case", rect_pts(zc - 0.19, Y0 + 0.84, zc + 0.19, Y0 + 1.17), closed=True, z=3.7, tone=0.4))
        for i, zc in enumerate((-0.5, -0.3, -0.1, 0.1, 0.3)):
            out.append(rect(f"note-sticker-{i + 1}", "case", zc - 0.008, Y0 + 0.768, zc + 0.008, Y0 + 0.782, role="detail", z=4.3, tone=0.85))
        out.append(rect("price-label", "case", 0.30, Y0 + 1.06, 0.40, Y0 + 1.12, role="detail", z=3.8, tone=0.95))
        for zc in (-0.62, 0.62):
            out.append(circle(f"candle-holder-hole-{'r' if zc < 0 else 'l'}", "case", zc * 0.95, Y0 + 1.02, 0.004, z=3.8, tone=0.1))
        out.append(circle("fallboard-lock", "case", 0.0, Y0 + 0.738, 0.006, z=6.1, tone=0.8))
        out.append(tex("fallboard-scratch-1", "case", [(-0.32, Y0 + 0.73), (-0.24, Y0 + 0.735)], z=6.1))
        out.append(tex("fallboard-scratch-2", "case", [(0.15, Y0 + 0.728), (0.26, Y0 + 0.733)], z=6.1))
        out.append(tex("keyslip-wear", "case", [(-0.45, Y0 + 0.709), (-0.1, Y0 + 0.71), (0.35, Y0 + 0.709)], z=6.1))
        out.append(tex("knee-board-scuff", "case", [(-0.25, Y0 + 0.16), (-0.1, Y0 + 0.17), (0.08, Y0 + 0.165), (0.2, Y0 + 0.17)], z=4.3))
    if scale == "100":
        out.append(rect("price-label", "case", 0.30, Y0 + 1.06, 0.40, Y0 + 1.12, role="detail", z=3.8, tone=0.95))
    return out + strap_front(scale) + bench_front(scale)


def dolly_front(scale):
    """🛞 The dolly from ahead: board edge with felt, the two front castors."""
    b = "dolly"
    if scale == "500":
        return [rect("dolly-board", b, -DOLLY_Z, 0.0, DOLLY_Z, DOLLY_TOP, z=5.2, tone=0.55, region="dolly", material="birch-plywood")]
    out = []
    if scale == "200":
        out.append(rect("dolly-board", b, -DOLLY_Z, 0.083, DOLLY_Z, DOLLY_TOP, z=5.2, tone=0.55, region="dolly", material="birch-plywood"))
    else:
        out.append(rect("dolly-board", b, -DOLLY_Z, BOARD_Y0, DOLLY_Z, BOARD_Y1, z=5.2, tone=0.55, region="dolly", material="birch-plywood"))
        out.append(rect("dolly-felt", b, -DOLLY_Z, BOARD_Y1, DOLLY_Z, DOLLY_TOP, z=5.21, tone=0.3, region="dolly", material="needle-felt"))
    for sgn in (-1, 1):
        zc = sgn * DOLLY_CASTOR_Z
        tag = f"front-{'r' if sgn < 0 else 'l'}"
        if scale == "50":
            out.append(rect(f"dolly-plate-{tag}", b, zc - 0.045, 0.075, zc + 0.045, BOARD_Y0, z=4.9, tone=0.7, region="dolly", material="galvanised-steel"))
            out.append(rect(f"dolly-wheel-{tag}", b, zc - 0.015, 0.0, zc + 0.015, 0.075, z=4.95, tone=0.15, region="dolly", material="polyurethane-castor"))
        else:
            out.append(rect(f"dolly-castor-{tag}", b, zc - 0.015, 0.0, zc + 0.015, BOARD_Y0 if scale == "100" else 0.083, z=4.95, tone=0.15, region="dolly", material="polyurethane-castor"))
    return out


STRAP_PARTS = [
    ("strap-bottom", [(-STRAP_HW, 0.0), (STRAP_HW, 0.0), (STRAP_HW, STRAP_T), (-STRAP_HW, STRAP_T)], -STRAP_ZO, STRAP_ZO),
    ("strap-side-r", [(-STRAP_HW, STRAP_T), (STRAP_HW, STRAP_T), (STRAP_HW, STRAP_TOP - STRAP_T), (-STRAP_HW, STRAP_TOP - STRAP_T)], -STRAP_ZO, -STRAP_ZI),
    ("strap-side-l", [(-STRAP_HW, STRAP_T), (STRAP_HW, STRAP_T), (STRAP_HW, STRAP_TOP - STRAP_T), (-STRAP_HW, STRAP_TOP - STRAP_T)], STRAP_ZI, STRAP_ZO),
    ("strap-top", [(-STRAP_HW, STRAP_TOP - STRAP_T), (STRAP_HW, STRAP_TOP - STRAP_T), (STRAP_HW, STRAP_TOP), (-STRAP_HW, STRAP_TOP)], -STRAP_ZO, STRAP_ZO),
]


def strap_front(scale, z=2.3):
    """🪢 The strap seen from ahead or above: every band swept laterally from its profile, so it lies down correctly when released."""
    out = []
    for sid, prof, z0, z1 in STRAP_PARTS:
        out.append(swept(sid, "strap", prof, z0, z1, z=z, tone=0.6, region="strap", material="polyester-webbing"))
    if scale != "500":
        out.append(swept("strap-ratchet", "strap", [(-STRAP_HW, BUCKLE_V0), (STRAP_HW, BUCKLE_V0), (STRAP_HW, BUCKLE_V1), (-STRAP_HW, BUCKLE_V1)], -BUCKLE_ZO, -STRAP_ZO, z=z + 0.1, tone=0.2, region="strap", material="ratchet-steel"))
    return out


def bench_front(scale):
    """🪑 The bench from ahead: front legs, front rail, seat board, the sagging cushion."""
    b = "bench"
    if scale == "500":
        u = [(-BENCH_HW, 0.0), (-0.205, 0.0), (-0.205, 0.325), (0.205, 0.325), (0.205, 0.0), (BENCH_HW, 0.0), (BENCH_HW, 0.40), (-BENCH_HW, 0.40)]
        return [poly("bench-frame", b, u, z=9, tone=0.35, region="bench", material="beech-stained"), rect("bench-cushion", b, -BENCH_HW, 0.40, BENCH_HW, BENCH_TOP, z=9.1, tone=0.25, region="bench-seat", material="vinyl-foam")]
    out = []
    top = 0.33 if scale == "200" else 0.40
    for sgn in (-1, 1):
        w0, w1 = sorted((sgn * 0.24, sgn * BENCH_HW))
        tag = f"front-{'r' if sgn < 0 else 'l'}"
        if scale == "200":
            out.append(rect(f"bench-leg-{tag}", b, w0, 0.0, w1, top, z=9, tone=0.35, region="bench", material="beech-stained"))
        else:
            out.append(poly(f"bench-leg-{tag}", b, [(w0 + 0.005, 0.0), (w1 - 0.005, 0.0), (w1, top), (w0, top)], z=9, tone=0.35, region="bench", material="beech-stained"))
    if scale == "200":
        out.append(rect("bench-board", b, -BENCH_HW, 0.33, BENCH_HW, BENCH_SEAT_Y, z=9.1, tone=0.35, region="bench", material="beech-stained"))
        out.append(rect("bench-cushion", b, -0.275, BENCH_SEAT_Y, 0.275, BENCH_TOP, z=9.2, tone=0.25, region="bench-seat", material="vinyl-foam"))
        return out
    out.append(rect("bench-rail-front", b, -0.24, 0.33, 0.24, 0.40, z=9.1, tone=0.35, region="bench", material="beech-stained"))
    out.append(rect("bench-board", b, -BENCH_HW, 0.40, BENCH_HW, BENCH_SEAT_Y, z=9.2, tone=0.35, region="bench", material="beech-stained"))
    out.append(poly("bench-cushion", b, [(w, v) for v, w in cushion_profile(scale)], z=9.3, tone=0.25, region="bench-seat", material="vinyl-foam"))
    if scale == "50":
        out.append(tex("bench-cushion-crack-1", b, [(-0.12, 0.468), (-0.05, 0.472), (0.02, 0.466)], z=9.4))
        out.append(tex("bench-cushion-crack-2", b, [(0.09, 0.47), (0.16, 0.473)], z=9.4))
    return out


def plan(scale):
    """🗺️ Plan from above (paper x ahead, paper y lateral): dolly and pedals under the case, keys under the fallboard, the cheek tops, the lid with its ring marks and bleached field, the strap over it, the bench in front."""
    d = Dim(scale)
    out = []
    if scale == "500":
        out.append(rect("dolly-board", "dolly", DOLLY_X0, -DOLLY_Z, DOLLY_X1, DOLLY_Z, z=1.1, tone=0.55, region="dolly", material="birch-plywood"))
        out.append(swept("pedal-group", "pedal-mute", PEDAL_GROUP[scale], -PEDAL_GROUP_HW[scale], PEDAL_GROUP_HW[scale], z=2.1, tone=0.8, region="brass-fittings", material="brass-tarnished"))
        out.append(rect("case-mass", "case", 0.09, -CHEEK_ZI, D, CHEEK_ZI, z=6.5, tone=0.38, region="case", material="walnut-veneer-aged"))
        out.append(rect("back-block", "case", 0.0, -CHEEK_ZI, 0.09, CHEEK_ZI, z=6.5, tone=0.6, region="back", material="spruce-raw-aged"))
        out.append(rect("side-cheek-r", "case", 0.0, -W2, D, -CHEEK_ZI, z=9.0, tone=0.32, region="case", material="walnut-veneer-aged"))
        out.append(rect("side-cheek-l", "case", 0.0, CHEEK_ZI, D, W2, z=9.0, tone=0.32, region="case", material="walnut-veneer-aged"))
        out.append(swept("fallboard", "fallboard", fall_profile(scale), -CHEEK_ZI, CHEEK_ZI, z=8.8, tone=0.36, region="fallboard", material="walnut-veneer-aged"))
        out.append(swept("lid", "lid", lid_profile(scale), -W2, W2, z=13.6, tone=0.45, region="lid", material="walnut-veneer-bleached"))
        return out + strap_front(scale, z=13.7) + bench_plan(scale)
    out.append(rect("dolly-board", "dolly", DOLLY_X0, -DOLLY_Z, DOLLY_X1, DOLLY_Z, z=1.1, tone=0.3, region="dolly", material="needle-felt"))
    if scale in ("50", "100"):
        prof = PEDAL_PROFILE if scale == "50" else PEDAL_PROFILE_100
        for pid, zp in PEDAL_Z.items():
            out.append(swept(pid, pid, prof, zp - PEDAL_HW, zp + PEDAL_HW, z=2.1, tone=0.8, region="brass-fittings", material="brass-tarnished"))
    else:
        out.append(swept("pedal-group", "pedal-mute", PEDAL_GROUP[scale], -PEDAL_GROUP_HW[scale], PEDAL_GROUP_HW[scale], z=2.1, tone=0.8, region="brass-fittings", material="brass-tarnished"))
    out.append(rect("keybed", "case", KEYBED_X0, -d.zi, D, d.zi, z=6.8, tone=0.4, region="case", material="walnut-veneer-aged"))
    kx1 = 0.57 if scale == "200" else KEYS_X1
    out.append(rect("keys", "case", KEYS_X0, -KEY_Z, kx1, KEY_Z, z=8.4, tone=0.92, region="keyboard", material="keytop-acrylic-yellowed"))
    if scale == "50":
        for i in range(1, 52):
            z = -KEY_Z + i * P_KEY
            out.append(det(f"key-gap-{i}", "case", [(0.42, z), (KEYS_X1, z)], z=8.45, tone=0.5))
        out.append(rect("black-key-bb0", "case", BLACK_X0, -KEY_Z + 1.1 * P_KEY - BLACK_HW, BLACK_X1, -KEY_Z + 1.1 * P_KEY + BLACK_HW, z=8.5, tone=0.05, region="keyboard", material="ebonised-sharps"))
        for k in range(7):
            for j, off in enumerate(BLACK_OFFSETS):
                zc = -KEY_Z + (2 + 7 * k + off) * P_KEY
                out.append(rect(f"black-key-{k + 1}-{j + 1}", "case", BLACK_X0, zc - BLACK_HW, BLACK_X1, zc + BLACK_HW, z=8.5, tone=0.05, region="keyboard", material="ebonised-sharps"))
        out.append(rect("key-top-missing", "case", 0.45, -KEY_Z + 25 * P_KEY + 0.001, KEYS_X1, -KEY_Z + 26 * P_KEY - 0.001, role="detail", z=8.46, tone=0.6))
    elif scale == "100":
        for k in range(7):
            for j, off in enumerate(BLACK_OFFSETS):
                zc = -KEY_Z + (2 + 7 * k + off) * P_KEY
                out.append(det(f"black-key-{k + 1}-{j + 1}", "case", [(0.42, zc), (BLACK_X1, zc)], z=8.5, tone=0.05))
    out += [
        rect("side-r", "case", 0.0, -W2, D, -d.zi, z=9.0, tone=0.32, region="case", material="walnut-veneer-aged"),
        rect("side-l", "case", 0.0, d.zi, D, W2, z=9.0, tone=0.32, region="case", material="walnut-veneer-aged"),
        rect("cheek-r", "case", 0.36, -d.zi, D, -CHEEK_ZI, z=9.0, tone=0.34, region="case", material="walnut-veneer-aged"),
        rect("cheek-l", "case", 0.36, CHEEK_ZI, D, d.zi, z=9.0, tone=0.34, region="case", material="walnut-veneer-aged"),
        rect("nameboard", "case", NAME_X0, -CHEEK_ZI, NAME_X1, CHEEK_ZI, z=9.0, tone=0.36, region="case", material="walnut-veneer-aged"),
        rect("upper-panel", "case", d.up0, -d.zi, 0.36, d.zi, z=12.2, tone=0.36, region="case", material="walnut-veneer-aged"),
        swept("fallboard", "fallboard", fall_profile(scale), -FALL_HZ, FALL_HZ, z=8.8, tone=0.36, region="fallboard", material="walnut-veneer-aged"),
        swept("lid", "lid", lid_profile(scale), -W2, W2, z=13.6, tone=0.45, region="lid", material="walnut-veneer-bleached"),
    ]
    if scale == "50":
        out.append(sh_ring("ring-mark-1", 0.22, 0.31, 0.042))
        out.append(sh_ring("ring-mark-2", 0.26, 0.20, 0.038))
        out.append(sh_ring("ring-mark-3", 0.15, -0.42, 0.05))
        out.append(tex("lid-bleach-edge", "lid", [(0.02, -0.75), (0.12, -0.45), (0.18, -0.1), (0.21, 0.25), (0.26, 0.55), (0.30, 0.75)], z=13.65))
        out.append(tex("lid-scratch-1", "lid", [(0.10, 0.05), (0.16, 0.12)], z=13.65))
        out.append(tex("lid-scratch-2", "lid", [(0.29, -0.30), (0.33, -0.22)], z=13.65))
        out.append(tex("lid-burn-mark", "lid", [(0.32, 0.61), (0.34, 0.625), (0.325, 0.635)], z=13.66, tone=0.1))
        out.append(det("lid-hinge", "lid", [(0.012, -0.7), (0.012, 0.7)], z=13.65))
        out.append(tex("cheek-wear-r", "case", [(0.56, -0.70), (0.59, -0.66), (0.595, -0.62)], z=9.1))
        out.append(tex("fallboard-wear", "case", [(0.58, -0.4), (0.59, 0.0), (0.585, 0.35)], z=8.85))
    if scale == "100":
        out.append(sh_ring("ring-mark-1", 0.22, 0.31, 0.042))
        out.append(sh_ring("ring-mark-3", 0.15, -0.42, 0.05))
    return out + strap_front(scale, z=13.7) + bench_plan(scale)


def sh_ring(sid, x, z, r):
    """⭕ A ring mark of a glass or vase on the lid."""
    return sh(sid, "lid", "ellipse", "texture", 13.65, 0.3, filled=False, cx=r6(x), cy=r6(z), rx=r6(r), ry=r6(r))


def bench_plan(scale):
    """🪑 The bench from above: seat board and cushion."""
    b = "bench"
    x0 = BENCH_X
    if scale == "500":
        return [rect("bench-cushion", b, x0, -BENCH_HW, x0 + BENCH_D, BENCH_HW, z=4.75, tone=0.25, region="bench-seat", material="vinyl-foam")]
    out = [rect("bench-board", b, x0, -BENCH_HW, x0 + BENCH_D, BENCH_HW, z=4.15, tone=0.35, region="bench", material="beech-stained")]
    out.append(rect("bench-cushion", b, x0 + 0.005, -0.275, x0 + BENCH_D - 0.005, 0.275, z=4.75, tone=0.25, region="bench-seat", material="vinyl-foam", **({"r": 0.015} if scale == "50" else {})))
    if scale == "50":
        out.append(tex("bench-sag", b, [(x0 + 0.05, -0.12), (x0 + 0.165, -0.16), (x0 + 0.28, -0.12), (x0 + 0.29, 0.12), (x0 + 0.165, 0.16), (x0 + 0.04, 0.12)], z=4.8, closed=True))
        out.append(tex("bench-crack-1", b, [(x0 + 0.10, -0.20), (x0 + 0.14, -0.15), (x0 + 0.13, -0.09)], z=4.8))
        out.append(tex("bench-crack-2", b, [(x0 + 0.21, 0.05), (x0 + 0.25, 0.11)], z=4.8))
    return out


def structure_extra(scale):
    """🦿 Own member bars of the side pedals at the scales where the pedals are one group block: each pedal lever from its pivot to its tread at its true section, declared slender."""
    if scale in ("50", "100"):
        return []
    return [box(f"st-bar-{pid}", pid, "brass-tarnished", 0.8, 0.0, 0.32, -0.01, 0.01, -PEDAL_HW, PEDAL_HW, member=pid, part="member", slender=True) for pid in ("pedal-soft", "pedal-sustain")]
