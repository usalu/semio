"""📇️ Metadata, rig, anchors, poses, clips, thermal and structural model of `used-writing-desk`; the solids and drawings are added by `author-used-writing-desk.py`.

@see ./author-used-writing-desk.py
"""

T = lambda en, de: {"en": en, "de": de}
VEN, TEAK, CHIP, HDF, CHROME, BRASS, FELT, LABEL = "teak-veneered-chipboard", "solid-teak", "chipboard-p2", "hardboard", "chrome-steel", "brass", "moving-blanket", "paper-label"
G = 9.80665

REFERENCES = [
    {"id": "en-527-1-2011", "citation": "EN 527-1:2011 Office furniture - Work tables and desks - Part 1: Dimensions (fixed work surface height 740 mm, clear leg room width and height under the top)."},
    {"id": "en-12521-2023", "citation": "EN 12521:2023 Furniture - Strength, durability and safety - Requirements for domestic tables (static vertical and horizontal loads, stability)."},
    {"id": "en-1730-2012", "citation": "EN 1730:2012 Furniture - Tables - Test methods for the determination of stability, strength and durability (load points on the top, vertical static load on the edge)."},
    {"id": "en-14749-2016", "citation": "EN 14749:2016 Domestic and kitchen storage units and worktops - Safety requirements and test methods (stability with extension elements open)."},
    {"id": "en-16122-2012", "citation": "EN 16122:2012 Domestic and non-domestic storage furniture - Test methods for the determination of strength, durability and stability (drawer and runner tests, opening forces)."},
    {"id": "en-312-2010", "citation": "EN 312:2010 Particleboards - Specifications (type P2, boards for interior fitments including furniture in dry conditions: bending strength 11 N/mm2 and modulus 1600 N/mm2 for 13-20 mm, density about 600-700 kg/m3)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (particleboard 600-900 kg/m3: 0.14-0.18 W/mK, 1700 J/kgK; hardwood 700 kg/m3: 0.18 W/mK, 1600 J/kgK; fibreboard, steel, brass)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.11 total hemispherical emissivity and solar absorptivity of varnished wood, paper, fabric, chromium-plated and oxidised brass surfaces)."},
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook - Wood as an Engineering Material, General Technical Report FPL-GTR-282, USDA (teak, Tectona grandis: specific gravity 0.55-0.65 at 12 % moisture, modulus of elasticity about 10-12 GPa, weathering and photo-discolouration of wood surfaces)."},
    {"id": "neufert-2012", "citation": "Neufert, E., Neufert, P. (2012). Architects' Data, 4th ed., Wiley-Blackwell (writing desks 1.20-1.60 x 0.60-0.80 m, height 0.72-0.75 m, drawer pedestals 0.40 m wide)."},
    {"id": "teak-desk-catalogue", "citation": "Catalogue-typical data of German and Danish teak-veneered pedestal writing desks of the 1960s and 1970s as second-hand dealers list them (top 120 x 60 cm, height 72-75 cm, one pedestal with three drawers and a central lock in the top drawer, panel end, modesty panel, 35-50 kg; authored-estimate, no single product)."},
    {"id": "moving-blanket-catalogue", "citation": "Catalogue-typical data of felt moving blankets (Moebeldecke) of removal companies: 180 x 200 cm, cotton-polyester felt with quilted cover, about 2 kg, folded in four for floor protection (authored-estimate, no single product)."},
]

MATERIALS = [
    {"id": VEN, "title": T("Teak-veneered chipboard, aged lacquer", "Teakfurnierte Spanplatte, gealterter Lack"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 680, "conductivityWmK": 0.13, "specificHeatJKgK": 1700, "elasticModulusPa": 1.6e9, "strengthPa": 1.1e7, "source": "literature", "reference": "en-312-2010"},
    {"id": TEAK, "title": T("Solid teak grip strip", "Griffleiste aus massivem Teak"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 650, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 1.06e10, "strengthPa": 9.5e7, "source": "literature", "reference": "wood-handbook-2021"},
    {"id": CHIP, "title": T("Raw chipboard P2 (drawer boxes, runners, back, bottom)", "Rohspanplatte P2 (Schubkastenkästen, Laufleisten, Rückwand, Boden)"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 650, "conductivityWmK": 0.13, "specificHeatJKgK": 1700, "elasticModulusPa": 1.6e9, "strengthPa": 1.1e7, "source": "literature", "reference": "en-312-2010"},
    {"id": HDF, "title": T("Hardboard drawer bottom 4 mm", "Hartfaser-Schubkastenboden 4 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 850, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "source": "literature", "reference": "iso-10456-2007"},
    {"id": CHROME, "title": T("Chromed steel bow handle (replacement)", "Verchromter Stahlbügelgriff (Ersatz)"), "emissivity": 0.1, "solarAbsorptivity": 0.4, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 2.1e11, "source": "literature", "reference": "incropera-2018"},
    {"id": BRASS, "title": T("Tarnished brass escutcheon", "Angelaufenes Messing-Schlüsselschild"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "source": "literature", "reference": "incropera-2018"},
    {"id": FELT, "title": T("Felt moving blanket, folded in four", "Filz-Möbeldecke, vierfach gefaltet"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": 126, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "moving-blanket-catalogue"},
    {"id": LABEL, "title": T("Paper price sticker", "Papier-Preisaufkleber"), "emissivity": 0.92, "solarAbsorptivity": 0.3, "densityKgM3": 800, "source": "literature", "reference": "incropera-2018"},
]


def bone(i, parent, en, de, x, y, angle, length, joint, z=None, abduction=None):
    """🦴️ One rig bone."""
    b = {"id": i, "parent": parent, "title": T(en, de), "x": x, "y": y}
    if z is not None:
        b["z"] = z
    b["angleDeg"] = angle
    if abduction is not None:
        b["abductionDeg"] = abduction
    b["lengthM"] = length
    b["joint"] = joint
    return b


FIXED = {"type": "fixed"}
RIG = {
    "kind": "mechanical",
    "root": "ground",
    "bones": [
        bone("ground", None, "Floor reference", "Bodenbezug", 0, 0, 0, 0, FIXED),
        bone("blanket", "ground", "Folded moving blanket under the desk (stays behind when the desk is carried)", "Gefaltete Möbeldecke unter dem Tisch (bleibt beim Tragen liegen)", -0.62, 0, 0, 1.24, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.32}),
        bone("leg-panel", "ground", "Panel end on the user's left, lifted with the whole desk", "Wange links vom Nutzer, mit dem ganzen Tisch gehoben", -0.5575, 0.02, 90, 0.715, {"type": "prismatic", "axisDeg": 90, "minM": -0.02, "maxM": 0.1}),
        bone("top", "leg-panel", "Desk top with the modesty panel, from the panel end to the pedestal", "Tischplatte mit Knieblende, von der Wange zum Unterschrank", 0.715, 0, -90, 1.1575, FIXED),
        bone("carcass", "top", "Drawer pedestal carcass hanging from the top to the floor", "Schubkasten-Unterschrank vom Deckblatt zum Boden", 0.9275, 0, -90, 0.715, FIXED),
        bone("drawer-rail", "carcass", "Runner direction of the pedestal (towards the user)", "Laufrichtung des Unterschranks (zum Nutzer)", 0, 0, 90, 0.03, FIXED, abduction=-90),
        bone("drawer-bottom", "drawer-rail", "Bottom file drawer", "Unterer Hängeregister-Schubkasten", 0.261, -0.64, 0, 0.359, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.36}, z=-0.1795, abduction=90),
        bone("drawer-middle", "drawer-rail", "Middle drawer with the replaced bow handle", "Mittlerer Schubkasten mit ersetztem Bügelgriff", 0.261, -0.315, 0, 0.359, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.36}, z=-0.1795, abduction=90),
        bone("drawer-top", "drawer-rail", "Top drawer with the keyhole of the central lock", "Oberer Schubkasten mit dem Schlüsselloch des Zentralverschlusses", 0.261, -0.13, 0, 0.359, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.36}, z=-0.1795, abduction=90),
    ],
}


def anchor(i, b, x, y, z, kind, en, de, **k):
    """📍️ One anchor."""
    a = {"id": i, "bone": b, "x": x, "y": y, "z": z, "kind": kind, "title": T(en, de)}
    a.update(k)
    return a


ANCHORS = [
    anchor("floor-leg-user", "blanket", 0.0625, 0, -0.24, "contact", "Blanket on the floor under the user-side foot of the panel end", "Decke am Boden unter dem nutzerseitigen Fuß der Wange", support="contact", normalDeg=90, footprintLengthM=0.025, footprintWidthM=0.08, frictionCoefficient=0.5),
    anchor("floor-leg-wall", "blanket", 0.0625, 0, 0.24, "contact", "Blanket on the floor under the wall-side foot of the panel end", "Decke am Boden unter dem wandseitigen Fuß der Wange", support="contact", normalDeg=90, footprintLengthM=0.025, footprintWidthM=0.08, frictionCoefficient=0.5),
    anchor("floor-pedestal-user", "blanket", 0.99, 0, -0.24, "contact", "Blanket on the floor under the user side of the pedestal", "Decke am Boden unter der Nutzerseite des Unterschranks", support="contact", normalDeg=90, footprintLengthM=0.4, footprintWidthM=0.08, frictionCoefficient=0.5),
    anchor("floor-pedestal-wall", "blanket", 0.99, 0, 0.24, "contact", "Blanket on the floor under the wall side of the pedestal", "Decke am Boden unter der Wandseite des Unterschranks", support="contact", normalDeg=90, footprintLengthM=0.4, footprintWidthM=0.08, frictionCoefficient=0.5),
    anchor("foot-leg-user", "leg-panel", 0, 0, -0.24, "contact", "User-side foot of the panel end", "Nutzerseitiger Fuß der Wange", support="contact", normalDeg=90, footprintLengthM=0.025, footprintWidthM=0.08, frictionCoefficient=0.4),
    anchor("foot-leg-wall", "leg-panel", 0, 0, 0.24, "contact", "Wall-side foot of the panel end", "Wandseitiger Fuß der Wange", support="contact", normalDeg=90, footprintLengthM=0.025, footprintWidthM=0.08, frictionCoefficient=0.4),
    anchor("foot-pedestal-user", "carcass", 0.715, 0, -0.24, "contact", "User-side end of the pedestal sides", "Nutzerseitiges Ende der Unterschrankseiten", support="contact", normalDeg=90, footprintLengthM=0.4, footprintWidthM=0.08, frictionCoefficient=0.4),
    anchor("foot-pedestal-wall", "carcass", 0.715, 0, 0.24, "contact", "Wall-side end of the pedestal sides", "Wandseitiges Ende der Unterschrankseiten", support="contact", normalDeg=90, footprintLengthM=0.4, footprintWidthM=0.08, frictionCoefficient=0.4),
    anchor("grip-leg-user", "top", -0.0275, 0, -0.29, "contact", "First mover's hand under the user-side corner of the top (panel end)", "Hand des ersten Trägers unter der nutzerseitigen Plattenecke (Wangenende)", support="contact", normalDeg=90, footprintLengthM=0.1, footprintWidthM=0.02, frictionCoefficient=0.6),
    anchor("grip-leg-wall", "top", -0.0275, 0, 0.29, "contact", "First mover's hand under the wall-side corner of the top (panel end)", "Hand des ersten Trägers unter der wandseitigen Plattenecke (Wangenende)", support="contact", normalDeg=90, footprintLengthM=0.1, footprintWidthM=0.02, frictionCoefficient=0.6),
    anchor("grip-pedestal-user", "top", 1.1425, 0, -0.29, "contact", "Second mover's hand under the user-side corner of the top (pedestal end)", "Hand des zweiten Trägers unter der nutzerseitigen Plattenecke (Unterschrankende)", support="contact", normalDeg=90, footprintLengthM=0.1, footprintWidthM=0.02, frictionCoefficient=0.6),
    anchor("grip-pedestal-wall", "top", 1.1425, 0, 0.29, "contact", "Second mover's hand under the wall-side corner of the top (pedestal end)", "Hand des zweiten Trägers unter der wandseitigen Plattenecke (Unterschrankende)", support="contact", normalDeg=90, footprintLengthM=0.1, footprintWidthM=0.02, frictionCoefficient=0.6),
    anchor("laptop-spot", "top", 0.4575, 0.025, -0.05, "load", "Laptop on the top", "Laptop auf der Platte"),
    anchor("forearm-rest", "top", 0.4575, 0.025, -0.27, "load", "Forearms of the user on the front edge", "Unterarme des Nutzers an der Vorderkante"),
]

FLOOR = ["floor-leg-user", "floor-leg-wall", "floor-pedestal-user", "floor-pedestal-wall"]
FEET = ["foot-leg-user", "foot-leg-wall", "foot-pedestal-user", "foot-pedestal-wall"]
GRIPS = ["grip-leg-user", "grip-leg-wall", "grip-pedestal-user", "grip-pedestal-wall"]


def regions(top, car, fro, bla):
    """🌬️ The region binding of a pose: (exposed, sunlit, contact fraction) per region."""
    out = {}
    for rid, (e, s, c) in (("desk-top", top), ("carcass", car), ("drawer-fronts", fro), ("blanket", bla)):
        r = {"exposedFraction": e, "sunlitFraction": s}
        if c:
            r["contact"] = {"fraction": c[0], "tempC": c[1]}
        out[rid] = r
    return {"regions": out, "emitters": {}}


STORED_TH = regions((1, 0, None), (0.95, 0, None), (1, 0, None), (0.5, 0, (0.5, 18)))
CARRIED_TH = regions((1, 0, None), (1, 0, None), (1, 0, None), (0.5, 0, (0.5, 18)))
USE_TH = regions((0.95, 0.3, (0.04, 31)), (0.95, 0.05, None), (1, 0.1, None), (0.5, 0, (0.5, 20)))
LAPTOP = {"id": "laptop", "kind": "equipment", "title": T("Laptop of 2.0 kg on the top", "Laptop mit 2,0 kg auf der Platte"), "anchor": "laptop-spot", "fxN": 0, "fyN": round(-2.0 * G, 2), "source": "authored-estimate", "reference": "en-1730-2012"}
FOREARMS = {"id": "forearms", "kind": "occupant", "title": T("Forearms of the seated user resting on the front edge (100 N)", "Unterarme des sitzenden Nutzers auf der Vorderkante (100 N)"), "anchor": "forearm-rest", "fxN": 0, "fyN": -100, "source": "authored-estimate", "reference": "en-1730-2012"}

POSES = [
    {"id": "stored", "title": T("Stored on a moving blanket", "Auf einer Möbeldecke gelagert"), "description": T("In the reuse depot: the emptied desk stands with its panel end and pedestal on a moving blanket folded in four, the drawers closed and held by the central lock of the top drawer.", "Im Gebrauchtwarenlager: Der geleerte Tisch steht mit Wange und Unterschrank auf einer vierfach gefalteten Möbeldecke, die Schubkästen geschlossen und vom Zentralverschluss des oberen Schubkastens gehalten."), "bones": {}, "ground": "auto", "contacts": FLOOR, "thermal": STORED_TH, "loads": []},
    {"id": "carried", "title": T("Carried level by two movers", "Von zwei Trägern waagerecht getragen"), "description": T("Two movers lift the locked desk 0.10 m off the blanket, each with both hands under the corners of the top at one end; the blanket stays on the floor.", "Zwei Träger heben den verschlossenen Tisch 0,10 m von der Decke, jeder mit beiden Händen unter den Plattenecken an einem Ende; die Decke bleibt am Boden liegen."), "bones": {"leg-panel": {"dy": 0.1}}, "ground": "authored", "contacts": GRIPS, "thermal": CARRIED_TH, "loads": []},
    {"id": "in-use", "title": T("In use at a laptop", "Am Laptop in Gebrauch"), "description": T("Back in a home: the desk stands on the floor, the moving blanket it was delivered on lies pulled out beside the pedestal, a laptop on the top, the forearms of the user on the front edge and the top drawer pulled 0.12 m for a pen.", "Wieder in einer Wohnung: Der Tisch steht auf dem Boden, die Möbeldecke der Anlieferung liegt herausgezogen neben dem Unterschrank, ein Laptop auf der Platte, die Unterarme des Nutzers auf der Vorderkante und der obere Schubkasten für einen Stift 0,12 m gezogen."), "bones": {"leg-panel": {"dy": -0.02}, "blanket": {"dx": 1.32}, "drawer-top": {"dx": 0.12}}, "ground": "auto", "contacts": FEET, "thermal": USE_TH, "loads": [LAPTOP, FOREARMS]},
    {"id": "drawers-open", "title": T("All drawers pulled out", "Alle Schubkästen gezogen"), "description": T("In use with all three drawers pulled to their stops (0.36, 0.33 and 0.30 m): the stability case with the extension elements open of EN 14749.", "In Gebrauch mit allen drei Schubkästen bis zum Anschlag gezogen (0,36, 0,33 und 0,30 m): der Standsicherheitsfall mit geöffneten Auszügen nach EN 14749."), "bones": {"leg-panel": {"dy": -0.02}, "blanket": {"dx": 1.32}, "drawer-top": {"dx": 0.36}, "drawer-middle": {"dx": 0.33}, "drawer-bottom": {"dx": 0.3}}, "ground": "auto", "contacts": FEET, "thermal": USE_TH, "loads": [LAPTOP]},
    {"id": "lift-key", "key": True, "title": T("Lifting off the blanket", "Abheben von der Decke"), "description": T("In-between key of the carry clip: the desk 0.05 m above the blanket.", "Zwischenschlüssel des Trageclips: der Tisch 0,05 m über der Decke."), "bones": {"leg-panel": {"dy": 0.05}}, "ground": "authored", "contacts": GRIPS, "thermal": CARRIED_TH, "loads": []},
]

span = lambda ids, a, b: [{"anchor": i, "fromS": a, "toS": b} for i in ids]
CLIPS = [
    {"id": "carry", "title": T("Lift and carry", "Anheben und tragen"), "description": T("The movers take the corners of the top, lift the desk off the blanket and hold it level at 0.10 m; the blanket stays behind on the floor.", "Die Träger fassen die Plattenecken, heben den Tisch von der Decke und halten ihn waagerecht in 0,10 m Höhe; die Decke bleibt am Boden zurück."), "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.5, "pose": "stored", "ease": "smooth"}, {"t": 1.2, "pose": "lift-key", "ease": "smooth"}, {"t": 2.0, "pose": "carried", "ease": "smooth"}, {"t": 4, "pose": "carried", "ease": "smooth"}],
     "contacts": span(FLOOR, 0, 0.8) + span(GRIPS, 0.5, 4),
     "phases": [{"id": "grip", "fromS": 0, "toS": 0.5, "title": T("Take hold", "Anfassen")}, {"id": "lift", "fromS": 0.5, "toS": 2.0, "title": T("Lift off", "Abheben")}, {"id": "carry", "fromS": 2.0, "toS": 4, "title": T("Carry level", "Waagerecht tragen")}],
     "events": [{"t": 0.5, "kind": "contact-on", "title": T("Hands under the corners of the top", "Hände unter den Plattenecken")}, {"t": 0.8, "kind": "contact-off", "title": T("The desk leaves the blanket", "Der Tisch verlässt die Decke")}, {"t": 2.0, "kind": "motion", "title": T("Held level 0.10 m above the floor", "Waagerecht 0,10 m über dem Boden gehalten")}],
     "tracks": []},
    {"id": "drawers", "title": T("Opening the drawers", "Schubkästen öffnen"), "description": T("At the laptop the user pulls each drawer in turn to its stop and pushes it back, top, middle, bottom; the top drawer returns to its pen position.", "Am Laptop zieht der Nutzer nacheinander jeden Schubkasten bis zum Anschlag und schiebt ihn zurück, oben, Mitte, unten; der obere kehrt in seine Stiftstellung zurück."), "durationS": 8, "fps": 30, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}, {"t": 1.0, "pose": "in-use", "ease": "smooth", "bones": {"drawer-top": {"dx": 0.36}}}, {"t": 2.0, "pose": "in-use", "ease": "smooth", "bones": {"drawer-top": {"dx": 0}}}, {"t": 3.0, "pose": "in-use", "ease": "smooth", "bones": {"drawer-top": {"dx": 0}, "drawer-middle": {"dx": 0.33}}}, {"t": 4.0, "pose": "in-use", "ease": "smooth", "bones": {"drawer-top": {"dx": 0}}}, {"t": 5.2, "pose": "in-use", "ease": "smooth", "bones": {"drawer-top": {"dx": 0}, "drawer-bottom": {"dx": 0.3}}}, {"t": 6.4, "pose": "in-use", "ease": "smooth", "bones": {"drawer-top": {"dx": 0}}}, {"t": 8, "pose": "in-use", "ease": "smooth"}],
     "contacts": span(FEET, 0, 8),
     "phases": [{"id": "top-drawer", "fromS": 0, "toS": 2.0, "title": T("Top drawer", "Oberer Schubkasten")}, {"id": "middle-drawer", "fromS": 2.0, "toS": 4.0, "title": T("Middle drawer", "Mittlerer Schubkasten")}, {"id": "bottom-drawer", "fromS": 4.0, "toS": 6.4, "title": T("Bottom drawer", "Unterer Schubkasten")}, {"id": "pen-position", "fromS": 6.4, "toS": 8, "title": T("Back to the pen position", "Zurück in die Stiftstellung")}],
     "events": [{"t": 1.0, "kind": "motion", "title": T("Top drawer at its stop", "Oberer Schubkasten am Anschlag"), "target": "drawer-top"}, {"t": 3.0, "kind": "motion", "title": T("Middle drawer at its stop", "Mittlerer Schubkasten am Anschlag"), "target": "drawer-middle"}, {"t": 5.2, "kind": "motion", "title": T("Bottom drawer at its stop", "Unterer Schubkasten am Anschlag"), "target": "drawer-bottom"}],
     "tracks": []},
]


def layer(i, en, de, kind, mat, t, lam, rho, cp, src, ref):
    """🧶️ One thermal layer: resistance t/lambda and the areal heat capacity of the depth one hour reaches."""
    depth = min(t, (lam / (rho * cp) * 3600) ** 0.5)
    return {"id": i, "title": T(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": round(t / lam, 4), "heatCapacityJM2K": round(rho * cp * depth), "source": src, "reference": ref}


def region(i, en, de, mat, area, length, geometry, sky, lay, **k):
    """🔥️ One passive thermal region."""
    r = {"id": i, "title": T(en, de), "kind": "surface", "model": "passive", "material": mat, "areaM2": area, "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"]}
    r.update(k)
    r.setdefault("source", "authored-estimate")
    r.setdefault("reference", "iso-10456-2007")
    return r


THERMAL = {
    "environment": {"airTempC": 21, "meanRadiantTempC": 20, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 250, "diffuseFraction": 0.2, "pressurePa": 101325},
    "passengers": [],
    "operating": [],
    "regions": [
        region("desk-top", "Desk top, teak veneer with worn lacquer", "Tischplatte, Teakfurnier mit abgenutztem Lack", VEN, 1.53, 0.6, "horizontal-plate-up", 0.85, layer("top-board", "Veneered chipboard 25 mm", "Furnierte Spanplatte 25 mm", "timber", VEN, 0.025, 0.13, 680, 1700, "literature", "iso-10456-2007"), ratedPowerW=0, ratedPowerBasis="passive"),
        region("carcass", "Panel end, pedestal and modesty panel", "Wange, Unterschrank und Knieblende", VEN, 2.96, 0.715, "vertical-plate", 0.45, layer("carcass-board", "Veneered chipboard 19-25 mm", "Furnierte Spanplatte 19-25 mm", "timber", VEN, 0.019, 0.13, 680, 1700, "literature", "iso-10456-2007")),
        region("drawer-fronts", "Drawer fronts with grip strips and the replaced handle", "Schubkastenfronten mit Griffleisten und dem ersetzten Griff", VEN, 0.29, 0.36, "vertical-plate", 0.45, layer("front-board", "Veneered chipboard front 19 mm", "Furnierte Spanplattenfront 19 mm", "timber", VEN, 0.019, 0.13, 680, 1700, "literature", "iso-10456-2007")),
        region("blanket", "Felt moving blanket", "Filz-Möbeldecke", FELT, 1.66, 0.64, "horizontal-plate-up", 0.5, layer("felt", "Felt folded in four, 20 mm", "Filz vierfach gefaltet, 20 mm", "clothing", FELT, 0.02, 0.045, 126, 1300, "authored-estimate", "moving-blanket-catalogue"), source="authored-estimate", reference="moving-blanket-catalogue"),
    ],
    "emitters": [],
    "assumptions": [
        T("A passive object indoors: the desk draws no power (the desk-top region states 0 W, basis passive); the laptop is a load, not part of the asset, and its heat is not modelled.", "Ein passives Objekt im Innenraum: Der Tisch nimmt keine Leistung auf (der Bereich Tischplatte nennt 0 W, Basis passiv); der Laptop ist eine Last, kein Teil des Assets, seine Wärme ist nicht modelliert."),
        T("Veneered chipboard after ISO 10456 and EN 312 (680 kg/m3, 0.13 W/mK, 1700 J/kgK); the areal heat capacity is rho c min(t, sqrt(a 3600 s)), the depth one hour reaches (20 mm).", "Furnierte Spanplatte nach ISO 10456 und EN 312 (680 kg/m3, 0,13 W/mK, 1700 J/kgK); die flächenbezogene Wärmekapazität ist rho c min(t, sqrt(a 3600 s)), die Tiefe, die eine Stunde erreicht (20 mm)."),
        T("Aged surface: fifty years of light have bleached the teak and the lacquer is worn and scratched, so the solar absorptance is taken as 0.55 instead of about 0.65 of fresh dark teak (authored-estimate after the photo-discolouration of wood surfaces, wood-handbook-2021); the emissivity of the worn lacquer stays 0.90 (incropera-2018).", "Gealterte Oberfläche: Fünfzig Jahre Licht haben das Teak gebleicht, der Lack ist abgenutzt und zerkratzt, daher wird der solare Absorptionsgrad mit 0,55 statt etwa 0,65 für frisches dunkles Teak angesetzt (Schätzwert nach der Lichtvergilbung von Holzoberflächen, wood-handbook-2021); der Emissionsgrad des abgenutzten Lacks bleibt 0,90 (incropera-2018)."),
        T("The drawer boxes, runners, back and bottom lie inside the closed pedestal and are not thermal surfaces; the 3 mm shadow gaps of the fronts are closed in the thermal model.", "Schubkastenkästen, Laufleisten, Rückwand und Boden liegen im geschlossenen Unterschrank und sind keine Wärmeflächen; die 3-mm-Schattenfugen der Fronten sind im Wärmemodell geschlossen."),
    ],
}

STRUCTURE = {
    "gravityMS2": G,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        T("A rigid panel desk: the 25 mm top spans 1.16 m from the panel end to the 0.40 m pedestal and is screwed to both with connector blocks; the modesty panel stiffens the frame against racking; the panel end and the two pedestal sides stand on the floor.", "Ein starrer Wangentisch: Die 25-mm-Platte spannt 1,16 m von der Wange zum 0,40 m breiten Unterschrank und ist mit Verbinderklötzen an beide geschraubt; die Knieblende steift das Gestell gegen Verschieben aus; Wange und beide Unterschrankseiten stehen auf dem Boden."),
        T("Stored: the desk stands on the folded blanket, which rests on the floor under the panel end and the pedestal (four contacts). Carried: four hand contacts under the corners of the top; the 2 kg of the blanket left on the floor are lumped into the rigid body and so hang on the hands (4 % of the 49 kg). In use: the four feet of the panel end and the pedestal on the floor; the blanket beside it is lumped the same way.", "Gelagert: Der Tisch steht auf der gefalteten Decke, die unter Wange und Unterschrank am Boden aufliegt (vier Kontakte). Getragen: vier Handkontakte unter den Plattenecken; die 2 kg der am Boden liegenden Decke sind dem starren Körper zugeschlagen und hängen so an den Händen (4 % der 49 kg). In Gebrauch: die vier Füße von Wange und Unterschrank auf dem Boden; die danebenliegende Decke ist ebenso zugeschlagen."),
        T("Masses from section x density of the 1:50 solids: veneered chipboard 680, raw chipboard P2 650, hardboard 850 kg/m3 (EN 312, ISO 10456), solid teak 650 kg/m3 (wood-handbook-2021), chromed steel 7850 kg/m3, blanket 2.0 kg (catalogue-typical).", "Massen aus Querschnitt x Dichte der 1:50-Körper: furnierte Spanplatte 680, Rohspanplatte P2 650, Hartfaser 850 kg/m3 (EN 312, ISO 10456), Teak massiv 650 kg/m3 (wood-handbook-2021), verchromter Stahl 7850 kg/m3, Decke 2,0 kg (katalogtypisch)."),
        T("In use a 2 kg laptop and 100 N of resting forearms load the top (authored-estimates of the order of the EN 1730 load points); with all three drawers pulled out (EN 14749 stability case) the centre of mass moves 0.07 m towards the user and stays well inside the 0.48 m deep foot contacts.", "In Gebrauch belasten ein Laptop von 2 kg und 100 N aufgelegte Unterarme die Platte (Schätzwerte in der Größenordnung der Lastpunkte nach EN 1730); mit allen drei gezogenen Schubkästen (Standsicherheitsfall nach EN 14749) wandert der Schwerpunkt 0,07 m zum Nutzer und bleibt weit innerhalb der 0,48 m tiefen Fußkontakte."),
    ],
    "clinical": False,
}


def meta():
    """📇️ The asset document without representations and volumes."""
    return {
        "$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument",
        "version": "semio.staffage/v2",
        "asset": {
            "id": "used-writing-desk",
            "category": "furniture",
            "title": T("Used Writing Desk", "Gebrauchter Schreibtisch"),
            "summary": T(
                "A second-hand teak-veneered pedestal writing desk of the 1970s, 1.20 x 0.60 m and 0.74 m high: a 25 mm top on a panel end and a 0.40 m pedestal with three drawers (keyhole of the central lock in the top drawer), a modesty panel at the back. Its use left a scratched and sun-bleached top with two ring marks, a knocked-off corner of the top, scuffed fronts and a chromed bow handle that replaced a lost teak grip strip. Stored on a folded moving blanket, carried level by two movers, back in use at a laptop with the drawers opening.",
                "Ein gebrauchter teakfurnierter Schreibtisch mit Unterschrank aus den 1970er-Jahren, 1,20 x 0,60 m und 0,74 m hoch: eine 25-mm-Platte auf einer Wange und einem 0,40 m breiten Unterschrank mit drei Schubkästen (Schlüsselloch des Zentralverschlusses im oberen), hinten eine Knieblende. Der Gebrauch hinterließ eine zerkratzte, sonnengebleichte Platte mit zwei Glasrändern, eine abgeschlagene Plattenecke, bestoßene Fronten und einen verchromten Bügelgriff anstelle einer verlorenen Teak-Griffleiste. Auf einer gefalteten Möbeldecke gelagert, von zwei Trägern waagerecht getragen, wieder am Laptop in Gebrauch mit sich öffnenden Schubkästen."),
            "tags": ["used", "household", "second-hand", "office", "desk", "writing-desk", "pedestal", "drawers", "teak", "veneer", "chipboard", "1970s", "moving-blanket"],
            "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T(
                "Real standard product of its period as a type, no brand: German or Danish teak-veneered pedestal desk of the 1970s after catalogue-typical data (teak-desk-catalogue) and EN 527-1 (740 mm work surface height). Top 1200 x 600 x 25 mm, panel end 560 x 715 x 25 mm set in 30 mm from the end, pedestal 400 x 560 x 715 mm with 19 mm sides, three drawers 140, 183 and 320 mm high behind 19 mm fronts with 3 mm shadow gaps, 8 mm back, 16 mm bottom over a 60 mm plinth recessed 40 mm, modesty panel 715 x 435 x 16 mm. Mass 49 kg from section x density. The user sits at -z; x runs along the top towards the pedestal on the user's right.",
                "Reales Standardprodukt seiner Zeit als Typ, ohne Marke: deutscher oder dänischer teakfurnierter Schreibtisch mit Unterschrank der 1970er-Jahre nach katalogtypischen Daten (teak-desk-catalogue) und EN 527-1 (740 mm Arbeitshöhe). Platte 1200 x 600 x 25 mm, Wange 560 x 715 x 25 mm, 30 mm vom Ende eingerückt, Unterschrank 400 x 560 x 715 mm mit 19-mm-Seiten, drei Schubkästen 140, 183 und 320 mm hoch hinter 19-mm-Fronten mit 3-mm-Schattenfugen, 8-mm-Rückwand, 16-mm-Boden über einem 60-mm-Sockel, 40 mm zurückgesetzt, Knieblende 715 x 435 x 16 mm. Masse 49 kg aus Querschnitt x Dichte. Der Nutzer sitzt bei -z; x läuft entlang der Platte zum Unterschrank zu seiner Rechten.")},
            "dimensions": {"heightM": 0.76, "widthM": 1.24, "depthM": 0.64, "massKg": 0},
            "materials": MATERIALS,
            "rig": RIG,
            "anchors": ANCHORS,
            "representations": {},
            "symbols": [],
            "poses": POSES,
            "clips": CLIPS,
            "thermal": THERMAL,
            "structure": STRUCTURE,
            "volumes": {},
            "presentation": {"defaultPose": "stored", "defaultClip": "drawers", "defaultScale": 50},
            "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
            "completion": "authored",
        },
    }
