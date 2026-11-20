"""🗣️ Round 3, machinery/sorption-wheel: the bilingual texts and the mass and area data shared by the volumes and the roots scripts."""

CASING_COM = [0.05, 0.25]
CASING_KG = 8
TOTAL_KG = 24
CASING_AREA_M2 = 0.93
MOTOR_AREA_M2 = 0.038
FOOTPRINT_LENGTH_M = 0.34
FOOTPRINT_WIDTH_M = 0.18


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


VOLUME_DECISIONS = {
    50: [
        T(
            "Body, wheel axis along the depth: the casing is a rolled ring of 1 mm galvanised sheet (0.62 m across, 0.22 m deep) with a 35 mm face flange turned in on both sides, in three parts so that the belt has its 16 mm window under the guard; it stands on two welded feet (8 mm plate, two 8 mm gussets each), the foot on the motor side is lengthened into the motor plate. A fixed 25 mm shaft is carried by one 40 by 8 mm channel beam per face, lapped behind the flanges. The rotor is the 0.57 m by 0.2 m matrix round a 90 mm hub tube with bearing end walls, eight 6 mm spoke plates (texture solids, drawn as strokes) and a 2 mm rim shell; EPDM rim seals close the gap to the flanges. The regeneration sector of 90 degrees is one welded V of two 30 by 8 mm divider bars per face on the fixed seal bone; the matrix face stays open, air has to pass. The drive is a 70 mm gearmotor (gearhead, motor, end cap, terminal box) on the plate, its pulley and the 10 by 5 mm flat belt round the rim shell run inside a belt guard of two 7.5 mm plates and an edge strip.",
            "Körper, Radachse entlang der Tiefe: Das Gehäuse ist ein gerollter Ring aus 1 mm verzinktem Blech (0,62 m Durchmesser, 0,22 m tief) mit beidseitig 35 mm nach innen gekantetem Stirnflansch, in drei Teilen, damit der Riemen sein 16 mm breites Fenster unter dem Schutz hat; er steht auf zwei geschweißten Füßen (8-mm-Platte, je zwei 8-mm-Knotenbleche), der Fuß auf der Motorseite ist zur Motorplatte verlängert. Eine feste 25-mm-Achse sitzt je Stirnseite in einem U-Träger 40 mal 8 mm, der hinter die Flansche greift. Der Rotor ist die Matrix von 0,57 m mal 0,2 m um ein 90-mm-Nabenrohr mit Lagerstirnwänden, acht 6-mm-Speichenbleche (Texturkörper, als Striche gezeichnet) und ein 2-mm-Mantelband; EPDM-Randdichtungen schließen den Spalt zu den Flanschen. Der Regenerationssektor von 90 Grad ist je Stirnseite ein geschweißtes V aus zwei Trennstäben 30 mal 8 mm am festen Knochen seal; die Matrixfläche bleibt offen, die Luft muss hindurch. Der Antrieb ist ein 70-mm-Getriebemotor (Getriebe, Motor, Lagerschild, Klemmenkasten) auf der Platte, seine Riemenscheibe und der Flachriemen 10 mal 5 mm um das Mantelband laufen in einem Riemenschutz aus zwei 7,5-mm-Blechen und einem Randstreifen.",
        ),
        T(
            "Thermal: the temperature field stands still while the matrix turns, so the matrix is modelled as stationary zones on the seal bone: the 272 degree process zone and the core of the sector belong to the disc (exchange area), the two 20 mm face skins of the 88 degree sector are the regen region (layer regen-skin; the faces against the core and the two radial cuts through the continuing matrix are section faces). The casing is its 1 mm skin: four quarter shells of the band and the two flange rings (the faces that look into the sealed gap round the rotor are marked, standing in for the cavity rule C31), beams, sector frames, feet and guard by reference; the belt window under the guard is not cut into the thermal band. The hub is its closed outer cylinder, the motor region the gearhead, motor, end cap and terminal box. The outlets are patches on the matrix faces (D2): process air over the whole process zone of the near face, regeneration air over the sector of the far face (counterflow).",
            "Thermik: Das Temperaturfeld steht still, während die Matrix dreht; die Matrix ist daher als ortsfeste Zonen am Knochen seal modelliert: die Prozesszone von 272 Grad und der Kern des Sektors gehören zur Scheibe (Austauschfläche), die zwei 20 mm dicken Stirnhäute des 88-Grad-Sektors sind die Region regen (Schicht regen-skin; die Flächen zum Kern und die zwei radialen Schnitte durch die weiterlaufende Matrix sind Schnittflächen). Das Gehäuse ist seine 1-mm-Haut: vier Viertelschalen des Mantels und die zwei Flanschringe (die Flächen zum abgedichteten Spalt um den Rotor sind markiert, als Ersatz für die Hohlraumregel C31), Träger, Sektorrahmen, Füße und Riemenschutz als Verweise; das Riemenfenster unter dem Schutz ist im thermischen Mantel nicht ausgeschnitten. Die Nabe ist ihr geschlossener Außenzylinder, die Region motor sind Getriebe, Motor, Lagerschild und Klemmenkasten. Die Auslässe sind Flächenstücke auf den Matrixstirnseiten (D2): Prozessluft über der ganzen Prozesszone der vorderen Stirnseite, Regenerationsluft über dem Sektor der hinteren (Gegenstrom).",
        ),
        T(
            "Structure: the load path of the 14 kg rotor is rim shell and spokes, hub, two bearings (joints, in the hub end walls), the fixed shaft, the two channel beams, the casing ring and the two feet; the sector frames are the members of the seal bone, welded to the beams at the root joints. The matrix, the seals, the motor and the guard carry nothing and are absent. One support solid of two pads lies under the feet; three balls mark the centres of mass of casing with motor, rotor and sector frames.",
            "Tragwerk: Der Lastpfad des 14 kg schweren Rotors ist Mantelband und Speichen, Nabe, zwei Lager (Gelenke, in den Nabenstirnwänden), die feste Achse, die zwei U-Träger, der Gehäusering und die zwei Füße; die Sektorrahmen sind die Stäbe des Knochens seal, an den Wurzelgelenken mit den Trägern verschweißt. Matrix, Dichtungen, Motor und Riemenschutz tragen nichts und fehlen. Ein Auflagerkörper aus zwei Platten liegt unter den Füßen; drei Kugeln markieren die Schwerpunkte von Gehäuse mit Motor, Rotor und Sektorrahmen.",
        ),
    ],
    100: [
        T(
            "Body, drawn anew for a 15 mm feature: the casing is one C-section drum (band and both flanges, no belt window), the beams and the sector frames are 15 mm bars set flush with the faces, so the matrix is 0.19 m deep; the rotor is one matrix ring with the rim shell merged, a closed hub cylinder and the eight spoke plates; each foot is one block of plate and gussets, the motor side with its 15 mm plinth; the gearmotor is one 70 mm block and the belt guard one solid 31 mm slab, with pulley and belt inside it left out. Seals, shaft and terminal box are below the feature.",
            "Körper, neu gezeichnet für 15 mm kleinste Einzelheit: Das Gehäuse ist eine C-Trommel (Mantel und beide Flansche, ohne Riemenfenster), Träger und Sektorrahmen sind 15-mm-Stäbe bündig mit den Stirnseiten, die Matrix ist daher 0,19 m tief; der Rotor ist ein Matrixring mit eingeschmolzenem Mantelband, ein geschlossener Nabenzylinder und die acht Speichenbleche; jeder Fuß ist ein Block aus Platte und Knotenblechen, auf der Motorseite mit 15-mm-Sockel; der Getriebemotor ist ein 70-mm-Block und der Riemenschutz eine massive 31-mm-Scheibe, Riemenscheibe und Riemen darin entfallen. Dichtungen, Achse und Klemmenkasten liegen unter der kleinsten Einzelheit.",
        ),
        T(
            "Thermal and structure: the casing wall is four quarter solids from the flange edge to the band with the inward face marked; the matrix zones reach to the flange edge, the hot face skins stay 20 mm. Structure keeps drum, beams, feet, hub, spokes and sector frames by reference and adds the rim shell, the two bearings and the root joints as its own solids.",
            "Thermik und Tragwerk: Die Gehäusewand sind vier Viertelkörper von der Flanschkante bis zum Mantel mit markierter Innenfläche; die Matrixzonen reichen bis zur Flanschkante, die heißen Stirnhäute bleiben 20 mm. Das Tragwerk übernimmt Trommel, Träger, Füße, Nabe, Speichen und Sektorrahmen als Verweise und ergänzt Mantelband, die zwei Lager und die Wurzelgelenke als eigene Körper.",
        ),
    ],
    200: [
        T(
            "Body for a 30 mm feature: the casing is a solid ring from the flange edge to the band (the 24 mm gap round the rotor is below the feature), the rotor one disc of 0.55 m by 0.16 m with hub and spokes merged, beams and sector frames 30 mm bars, the feet two blocks, motor and belt guard two blocks standing on the ground.",
            "Körper für 30 mm kleinste Einzelheit: Das Gehäuse ist ein massiver Ring von der Flanschkante bis zum Mantel (der 24-mm-Spalt um den Rotor liegt unter der Einzelheit), der Rotor eine Scheibe von 0,55 m mal 0,16 m mit eingeschmolzener Nabe und Speichen, Träger und Sektorrahmen 30-mm-Stäbe, die Füße zwei Blöcke, Motor und Riemenschutz zwei Blöcke auf dem Boden.",
        ),
        T(
            "Thermal and structure: casing quarters, matrix zones with 30 mm hot skins, the hub as a hidden cylinder inside the disc, the motor block. The structure is not the disc: hub, four 30 mm spoke plates and a rim ring carry the rotor, the casing ring, beams, feet and sector frames are references.",
            "Thermik und Tragwerk: Gehäuseviertel, Matrixzonen mit 30 mm dicken heißen Häuten, die Nabe als verdeckter Zylinder in der Scheibe, der Motorblock. Das Tragwerk ist nicht die Scheibe: Nabe, vier 30-mm-Speichenbleche und ein Mantelring tragen den Rotor, Gehäusering, Träger, Füße und Sektorrahmen sind Verweise.",
        ),
    ],
}

STYLES = {
    50: T("Darker tones carry heavy steel parts and the motor, the lightest tone the hot sector; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Dunklere Töne tragen schwere Stahlteile und den Motor, der hellste Ton den heißen Sektor; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
    100: T("Darker tones carry heavy steel parts and the motor, the lightest tone the hot sector; body pen 0.18 mm, detail 0.13 mm, ground 0.25 mm.", "Dunklere Töne tragen schwere Stahlteile und den Motor, der hellste Ton den heißen Sektor; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,25 mm."),
    200: T("Darker tones carry heavy steel parts and the motor, the lightest tone the hot sector; body pen 0.13 mm, no detail pen, ground 0.25 mm.", "Dunklere Töne tragen schwere Stahlteile und den Motor, der hellste Ton den heißen Sektor; Körperstift 0,13 mm, kein Detailstift, Boden 0,25 mm."),
}

DECISIONS_2D = {
    50: {
        "silhouette": T(
            "A 0.62 m desiccant wheel seen face-on, 12 mm across on paper, drawn as the true view of the 1:50 body: the casing ring with its face flange round the open matrix face, the vertical shaft beam and the V of the two sector divider bars over the hub, two welded feet, the belt guard between ring and motor plate and the gearmotor in front of it. Front view (from ahead): the casing band edge-on, the guard edge, the gearmotor with end cap and terminal box, the motor-side foot. Top view: only the band, everything else lies under it. The wheel turns about the lateral axis, so the rotation shows in the side elevation only.",
            "Ein 0,62 m großes Sorptionsrad in der Frontansicht, auf Papier 12 mm im Durchmesser, als wahre Ansicht des Körpers 1:50 gezeichnet: der Gehäusering mit seinem Stirnflansch um die offene Matrixfläche, der senkrechte Achsträger und das V der zwei Sektortrennstäbe über der Nabe, zwei geschweißte Füße, der Riemenschutz zwischen Ring und Motorplatte und davor der Getriebemotor. Vorderansicht: der Gehäusemantel hochkant, die Kante des Riemenschutzes, der Getriebemotor mit Lagerschild und Klemmenkasten, der motorseitige Fuß. Draufsicht: nur der Mantel, alles andere liegt darunter. Das Rad dreht sich um die Querachse, die Drehung zeigt sich daher nur in der Seitenansicht.",
        ),
        "retained": T("Casing ring, matrix face with a honeycomb symbol, eight spokes, hub, shaft beam, sector frame, hot sector, both feet, belt guard, gearmotor with end cap and terminal box.", "Gehäusering, Matrixfläche mit Wabensymbol, acht Speichen, Nabe, Achsträger, Sektorrahmen, heißer Sektor, beide Füße, Riemenschutz, Getriebemotor mit Lagerschild und Klemmenkasten."),
        "merged": T("Foot plate and gussets of a foot are one outline in the side elevation; gearhead and motor are one outline.", "Fußplatte und Knotenbleche eines Fußes sind in der Seitenansicht ein Umriss; Getriebe und Motor sind ein Umriss."),
        "omitted": T("Belt and pulley (inside the guard), the rim seals and the shaft (hidden), bolts.", "Riemen und Riemenscheibe (im Riemenschutz), Randdichtungen und Achse (verdeckt), Schrauben."),
        "thinElements": T("The honeycomb is a symbol of 0.02 m cells; real flutes are 1.3 to 4 mm high and would close up on paper. The spokes are 6 mm plates drawn as 0.13 mm lines.", "Die Wabe ist ein Symbol aus Zellen von 0,02 m; reale Wellen sind 1,3 bis 4 mm hoch und liefen auf Papier zu. Die Speichen sind 6-mm-Bleche, als 0,13-mm-Linien gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": STYLES[50],
        "overlays": T("Five regions: casing (ring, beam, sector frame, feet, guard), disc, the hot regeneration sector between the divider bars, hub and motor; honeycomb and spokes turn with the rotor bone, the sector stays in place.", "Fünf Regionen: Gehäuse (Ring, Träger, Sektorrahmen, Füße, Riemenschutz), Scheibe, der heiße Regenerationssektor zwischen den Trennstäben, Nabe und Motor; Wabe und Speichen drehen sich mit dem Rotorknochen, der Sektor bleibt stehen."),
    },
    100: {
        "silhouette": T(
            "A 6 mm wheel: casing ring, matrix face with eight spokes and hub, shaft beam, sector frame and hot sector, two block feet, belt guard and a motor block on its plinth. Front view: band, guard edge, motor block and the motor-side foot. Top view: the band alone.",
            "Ein 6 mm großes Rad: Gehäusering, Matrixfläche mit acht Speichen und Nabe, Achsträger, Sektorrahmen und heißer Sektor, zwei Blockfüße, Riemenschutz und ein Motorblock auf seinem Sockel. Vorderansicht: Mantel, Kante des Riemenschutzes, Motorblock und der motorseitige Fuß. Draufsicht: nur der Mantel.",
        ),
        "retained": T("Casing ring, matrix face, eight spokes (they show the rotation), hub, beam, sector frame, hot sector, feet, guard and motor.", "Gehäusering, Matrixfläche, acht Speichen (sie zeigen die Drehung), Nabe, Träger, Sektorrahmen, heißer Sektor, Füße, Riemenschutz und Motor."),
        "merged": T("Gearhead, motor and end cap are one block; the motor plinth is part of the foot.", "Getriebe, Motor und Lagerschild sind ein Block; der Motorsockel gehört zum Fuß."),
        "omitted": T("The honeycomb symbol, the terminal box, belt, pulley and seals.", "Das Wabensymbol, der Klemmenkasten, Riemen, Riemenscheibe und Dichtungen."),
        "thinElements": T("Spokes are 0.13 mm lines; the beam is 0.4 mm and the divider bars 0.3 mm wide on paper.", "Die Speichen sind 0,13-mm-Linien; der Träger ist auf Papier 0,4 mm, die Trennstäbe sind 0,3 mm breit."),
        "hatching": T("None.", "Keine."),
        "styles": STYLES[100],
        "overlays": T("The same five regions as at 1:50.", "Dieselben fünf Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": T(
            "A 3 mm wheel: casing ring, matrix face, shaft beam, sector frame with the hot sector, two feet, and the guard and motor blocks on the ground. Front view: band, guard edge, motor block and foot. Top view: the band alone.",
            "Ein 3 mm großes Rad: Gehäusering, Matrixfläche, Achsträger, Sektorrahmen mit dem heißen Sektor, zwei Füße sowie Riemenschutz und Motor als Blöcke auf dem Boden. Vorderansicht: Mantel, Kante des Riemenschutzes, Motorblock und Fuß. Draufsicht: nur der Mantel.",
        ),
        "retained": T("Casing ring, matrix face, beam, sector frame, hot sector, feet, guard and motor.", "Gehäusering, Matrixfläche, Träger, Sektorrahmen, heißer Sektor, Füße, Riemenschutz und Motor."),
        "merged": T("The hub lies behind the beam and is merged into it (regions casing and hub); the motor plate is merged into motor and guard.", "Die Nabe liegt hinter dem Träger und ist mit ihm zusammengeführt (Regionen Gehäuse und Nabe); die Motorplatte ist in Motor und Riemenschutz aufgegangen."),
        "omitted": T("Honeycomb and spokes; at this scale the rotation cannot be seen.", "Wabe und Speichen; bei diesem Maßstab ist die Drehung nicht zu sehen."),
        "thinElements": T("The face flange is a 35 mm band, the beam 40 mm and the divider bars 30 mm wide: 0.15 to 0.2 mm on paper.", "Der Stirnflansch ist ein 35-mm-Band, der Träger 40 mm und die Trennstäbe sind 30 mm breit: 0,15 bis 0,2 mm auf Papier."),
        "hatching": T("None.", "Keine."),
        "styles": STYLES[200],
        "overlays": T("The beam lists casing and hub as merged regions; disc, hot sector and motor are their own shapes.", "Der Träger führt Gehäuse und Nabe als zusammengeführte Regionen; Scheibe, heißer Sektor und Motor sind eigene Formen."),
    },
}

UNAVAILABLE_500 = {
    "reason": T(
        "At 1:500 the asset is only 1.2 mm wide on paper and its smallest parts, the 0.02 m cells of the honeycomb symbol and the 35 mm face flange of the casing, are far below every print profile.",
        "Bei 1:500 ist das Gerät auf Papier nur 1,2 mm breit, und seine kleinsten Teile, die 0,02 m großen Zellen des Wabensymbols und der 35 mm breite Stirnflansch des Gehäuses, liegen weit unter jedem Druckprofil.",
    ),
    "limitingFeature": T("The 0.02 m cells of the honeycomb symbol and the 35 mm face flange", "Die 0,02 m großen Zellen des Wabensymbols und der 35 mm breite Stirnflansch"),
}

SUMMARY = T(
    "A 0.62 m desiccant wheel seen face-on: a honeycomb silica-gel matrix on a spoked rotor turns slowly inside a casing ring on two feet, driven by a small belt gearmotor; coil-heated regeneration air at 70 degC dries it through the hot sector, and the process air leaves dry and warmed by the adsorption heat.",
    "Ein 0,62 m großes Sorptionsrad in der Frontansicht: eine Silicagel-Wabenmatrix auf einem Speichenrotor dreht sich langsam in einem Gehäusering auf zwei Füßen, angetrieben von einem kleinen Riemen-Getriebemotor; durch den heißen Sektor trocknet vom Heizregister auf 70 degC erwärmte Regenerationsluft sie aus, und die Prozessluft verlässt das Rad trocken und durch die Adsorptionswärme erwärmt.",
)

NOTES_REPLACE = {
    "en": [
        (
            "in a 0.62 m galvanised casing ring on a stand, driven by a belt motor at 12 revolutions per hour (5 minutes per turn); a 90 degree sector is closed off for regeneration. The mass is 22 kg (casing and stand 6, matrix 14, sector plates and seals 2).",
            "in a 0.62 m galvanised casing ring on two welded feet, carried on a fixed shaft between two beams and driven through a flat belt round its rim shell by a 30 W gearmotor under a belt guard at 12 revolutions per hour (5 minutes per turn); a 90 degree sector between two divider bars per face is closed off for regeneration. The mass is 24 kg (casing ring, beams, shaft, feet and guard 6.4, gearmotor 1.6, rotor with matrix 14, sector bars and seals 2). The gearmotor turns nearly all of its 30 W input into heat at its housing (28 W), 1.5 W appear at the hub bearings. Standard rotor depths are 50, 100, 200 and 400 mm; real flutes are 1.3 to 4 mm high, the 0.02 m honeycomb of the drawing is a symbol.",
        ),
        ("The origin is the centre of the casing on the base line of the stand, x at the hub.", "The origin is the centre of the casing on the base line of the feet, x at the hub."),
    ],
    "de": [
        (
            "in einem 0,62 m großen verzinkten Gehäusering auf einem Fuß, über einen Riemenmotor mit 12 Umdrehungen je Stunde (5 Minuten je Umdrehung) angetrieben; ein 90-Grad-Sektor ist für die Regeneration abgetrennt. Die Masse beträgt 22 kg (Gehäuse und Fuß 6, Matrix 14, Sektorbleche und Dichtungen 2).",
            "in einem 0,62 m großen verzinkten Gehäusering auf zwei geschweißten Füßen, auf einer festen Achse zwischen zwei Trägern gelagert und über einen Flachriemen um das Mantelband von einem 30-W-Getriebemotor unter einem Riemenschutz mit 12 Umdrehungen je Stunde (5 Minuten je Umdrehung) angetrieben; ein 90-Grad-Sektor zwischen je zwei Trennstäben je Stirnseite ist für die Regeneration abgetrennt. Die Masse beträgt 24 kg (Gehäusering, Träger, Achse, Füße und Riemenschutz 6,4, Getriebemotor 1,6, Rotor mit Matrix 14, Sektorstäbe und Dichtungen 2). Der Getriebemotor setzt fast seine ganze Aufnahme von 30 W an seinem Gehäuse in Wärme um (28 W), 1,5 W fallen an den Nabenlagern an. Übliche Rotortiefen sind 50, 100, 200 und 400 mm; reale Wellen sind 1,3 bis 4 mm hoch, die Wabe von 0,02 m der Zeichnung ist ein Symbol.",
        ),
        ("Der Ursprung liegt auf der Grundlinie des Fußes, x an der Nabe.", "Der Ursprung liegt auf der Grundlinie der Füße, x an der Nabe."),
    ],
}

REFERENCES = [
    {"id": "dri-rotors-components", "citation": "DRI Desiccant Rotors International. Components: desiccant rotors and cassettes (standard rotor depths 50, 100, 200 and 400 mm, cassette with drive motor and belt), drirotors.com, read 2026-11-20."},
    {"id": "us-5683532-1997", "citation": "US Patent 5,683,532 (1997). Method of manufacturing an active silica gel honeycomb adsorbing body (corrugation wavelength 2.5 to 6.5 mm, wave height 1.3 to 4.0 mm)."},
]

MATERIALS = [
    {"id": "painted-steel", "title": T("Painted motor housing", "Lackiertes Motorgehäuse"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 7800, "conductivityWmK": 50, "specificHeatJKgK": 480, "source": "literature", "reference": "incropera-2018"},
    {"id": "epdm", "title": T("EPDM rubber", "EPDM-Gummi"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 2000, "source": "literature", "reference": "incropera-2018"},
]

MOTOR_REGION = {
    "id": "motor",
    "title": T("Belt gearmotor", "Riemen-Getriebemotor"),
    "kind": "engine",
    "model": "operating",
    "material": "painted-steel",
    "areaM2": MOTOR_AREA_M2,
    "characteristicLengthM": 0.07,
    "geometry": "cylinder",
    "skyViewFactor": 0.5,
    "layers": [{"id": "motor-housing", "title": T("Motor and gearhead housing", "Motor- und Getriebegehäuse"), "kind": "metal", "material": "painted-steel", "thicknessM": 0.003, "resistanceM2KW": 6e-05, "heatCapacityJM2K": 19000, "source": "authored-estimate", "reference": "incropera-2018"}],
    "heatCapacityJM2K": 19000,
    "operating": {"key": "rotation", "heatW": 28},
    "ratedPowerW": 30,
    "ratedPowerBasis": "electrical-input",
    "source": "authored-estimate",
    "reference": "ashrae-2020-equipment",
}

HUB_TITLE = T("Hub with bearings", "Nabe mit Lagern")
HUB_LAYER_TITLE = T("Hub tube", "Nabenrohr")
CASING_TITLE = T("Casing ring, beams, feet and belt guard", "Gehäusering, Träger, Füße und Riemenschutz")

THERMAL_ASSUMPTION = T(
    "The 30 W gearmotor runs with the wheel drive: about 28 W leave its housing as heat (a small gearmotor at light load turns nearly all of its input into heat), 1.5 W are friction at the hub bearings.",
    "Der 30-W-Getriebemotor läuft mit dem Radantrieb: rund 28 W verlassen sein Gehäuse als Wärme (ein kleiner Getriebemotor bei geringer Last setzt fast seine ganze Aufnahme in Wärme um), 1,5 W sind Reibung an den Nabenlagern.",
)

STRUCTURE_ASSUMPTION = T(
    "The 24 kg wheel stands on its two welded feet, 0.34 m over all and 0.18 m wide: a unilateral contact carries the weight.",
    "Das 24 kg schwere Rad steht auf seinen zwei geschweißten Füßen, 0,34 m über alles und 0,18 m breit: ein einseitiger Kontakt trägt das Gewicht.",
)

CASING_BONE_TITLE = T("Casing ring with feet and drive", "Gehäusering mit Füßen und Antrieb")
SEAL_BONE_TITLE = T("Regeneration sector frame with seals", "Rahmen des Regenerationssektors mit Dichtungen")
BASE_ANCHOR_TITLE = T("Feet on the surface", "Füße auf der Unterlage")
