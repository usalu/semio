"""🍼 Writes the asset document `🔣️.json` of the used beech baby cot (60 x 120 cm, convertible, height-adjustable base, castors) from the part model of `author-geometry.py`.

Run: `python3 author-cot.py` in this folder. The one authoritative script of the asset: every number of the document is stated here or in the geometry module.
@see ./author-geometry.py
@see ../../reports/used-baby-cot.md
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("geo", os.path.join(HERE, "author-geometry.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


REFS = [
    ("en-716-1-2017", "EN 716-1:2017+A1:2019 Furniture - Children's cots and folding cots for domestic use - Part 1: Safety requirements (gaps between bars 45 to 65 mm; internal depth from the top of the mattress base to the top of the sides at least 600 mm in the lowest and 300 mm in the higher base positions; removable bars only with a tool)."),
    ("en-716-2-2017", "EN 716-2:2017+A1:2019 Furniture - Children's cots and folding cots for domestic use - Part 2: Test methods (mattress base strength, side and bar strength, durability of removable bars)."),
    ("en-12528-1999", "EN 12528:1999 Castors and wheels - Castors for furniture - Requirements (castor sizes and loads of furniture castors, wheel diameters 35 to 75 mm)."),
    ("en-338-2016", "EN 338:2016 Structural timber - Strength classes (hardwood class D35: bending strength 35 N/mm2, mean modulus of elasticity 12 kN/mm2, mean density 650 to 800 kg/m3)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (timber 500 and 700 kg/m3: conductivity 0.13 and 0.18 W/mK, specific heat 1600 J/kgK; polyamide, rubber, steel; flexible PU foam)."),
    ("wood-handbook-2021", "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. FPL-GTR-282, USDA Forest Service (European beech and Scots pine air-dry density and stiffness, emissivity of finished wood about 0.9)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (total hemispherical emissivity of lacquered wood, rubber, galvanised steel, textiles; solar absorptance of light and darkened surfaces)."),
    ("iso-3386-1-1986", "ISO 3386-1:1986 Polymeric materials, cellular flexible - Determination of stress-strain characteristics in compression (cold foams of 35 to 45 kg/m3 for child mattresses)."),
    ("en-1991-1-1-2002", "EN 1991-1-1:2002 Eurocode 1: Actions on structures - General actions: densities, self-weight and imposed loads (masses of the occupants as quasi-permanent loads)."),
    ("who-growth-2006", "WHO Child Growth Standards (2006): median weight of a newborn about 3.3 kg and of a child of nine months about 8.9 kg, median length at nine months 0.72 m."),
    ("cot-60x120-catalogue", "Catalogue-typical beech cots 60 x 120 cm of the German mid-range market (convertible 'Kinderbett/Gitterbett Buche massiv, 3 Schlupfsprossen, Lattenrost 3-fach höhenverstellbar, mit Rollen'; types, no brand): outside 126 to 130 x 66 to 70 cm, height 84 to 90 cm, posts 40 to 45 mm square, round bars 20 to 24 mm, castors 50 mm with two brakes, mass 17 to 23 kg without mattress; child mattress 60 x 120 x 10 cm of 2.5 to 4 kg (read 2026-11-22)."),
    ("reuse-depot-practice", "Practice of German municipal reuse depots and second-hand department stores (Gebrauchtwarenkaufhaus) for cots: offered assembled and cleaned with the base in the top position, removed slip bars and the hexagon key kept with the cot, castor brakes set, mattress offered separately or wrapped (authored from depot descriptions, 2026-11-22)."),
]

MATERIALS = [
    {"id": "beech-lacquered-aged", "title": T("Solid beech, clear lacquer, yellowed and scratched", "Buche massiv, klar lackiert, vergilbt und verkratzt"), "emissivity": 0.91, "solarAbsorptivity": 0.55, "densityKgM3": 720, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "elasticModulusPa": 12e9, "strengthPa": 35e6, "source": "literature", "reference": "en-338-2016"},
    {"id": "beech-slat", "title": T("Beech slats and base rails, waxed, greyed", "Buchenleisten und Rostholme, gewachst, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.58, "densityKgM3": 690, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 35e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "pine-slat-replacement", "title": T("Replacement slat, unfinished pine", "Ersatzleiste, Kiefer unbehandelt"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 500, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 24e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "mattress-cover", "title": T("Quilted cotton cover of the mattress, washed and stained", "Gesteppter Baumwollbezug der Matratze, gewaschen und fleckig"), "emissivity": 0.9, "solarAbsorptivity": 0.42, "densityKgM3": 150, "conductivityWmK": 0.05, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "cold-foam", "title": T("Cold-cure PU foam 40 kg/m3", "Kaltschaum 40 kg/m3"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 40, "conductivityWmK": 0.04, "specificHeatJKgK": 1400, "elasticModulusPa": 1.2e4, "strengthPa": 5000, "source": "authored-estimate", "reference": "iso-3386-1-1986"},
    {"id": "castor-polyamide", "title": T("Castor fork and socket, grey polyamide", "Rollengabel und Steckhülse, Polyamid grau"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 1140, "conductivityWmK": 0.25, "specificHeatJKgK": 1600, "elasticModulusPa": 2.5e9, "strengthPa": 70e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "castor-polyamide-black", "title": T("Replacement castor fork, black polyamide", "Ersatzrolle, Gabel Polyamid schwarz"), "emissivity": 0.93, "solarAbsorptivity": 0.9, "densityKgM3": 1140, "conductivityWmK": 0.25, "specificHeatJKgK": 1600, "elasticModulusPa": 2.5e9, "strengthPa": 70e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "castor-rubber", "title": T("Castor tread, thermoplastic rubber, worn", "Rollenlauffläche, thermoplastischer Kautschuk, abgefahren"), "emissivity": 0.92, "solarAbsorptivity": 0.88, "densityKgM3": 1100, "conductivityWmK": 0.2, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "steel-zinc", "title": T("Base hanger brackets, zinc-plated steel, dulled", "Rosthalter, Stahl verzinkt, matt"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "incropera-2018"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}
DENSITY["mattress-cover"] = 48.6

BONE_TITLES = {
    "ground": T("Floor reference", "Bodenbezug"),
    "frame": T("Cot frame: four posts, long and end rails", "Bettgestell: vier Pfosten, Längs- und Stirnholme"),
    "bars-right": T("Bar row of the right long side (with the gap of the slip bars)", "Sprossenreihe der rechten Längsseite (mit der Lücke der Schlupfsprossen)"),
    "bars-left": T("Bar row of the left long side", "Sprossenreihe der linken Längsseite"),
    "bars-head": T("Bar row of the head end", "Sprossenreihe des Kopfendes"),
    "bars-foot": T("Bar row of the foot end", "Sprossenreihe des Fußendes"),
    "slip-carrier": T("Removed slip bars: placement in the cot", "Herausgenommene Schlupfsprossen: Ablage im Bett"),
    "slip-bars": T("Three removable slip bars", "Drei herausnehmbare Schlupfsprossen"),
    "base": T("Slatted base, three height positions", "Lattenrost, drei Höhenstufen"),
    "mattress": T("Child mattress 60 x 120 x 10 cm", "Kindermatratze 60 x 120 x 10 cm"),
}
for key, label in (("hl", ("head left", "Kopfende links")), ("hr", ("head right", "Kopfende rechts")), ("fl", ("foot left", "Fußende links")), ("fr", ("foot right", "Fußende rechts"))):
    BONE_TITLES[f"castor-{key}"] = T(f"Swivel castor, {label[0]}", f"Lenkrolle, {label[1]}")
    BONE_TITLES[f"wheel-{key}"] = T(f"Castor wheel, {label[0]}", f"Laufrad, {label[1]}")

JOINTS = {
    "ground": {"type": "fixed"}, "frame": {"type": "fixed"},
    "bars-right": {"type": "fixed"}, "bars-left": {"type": "fixed"}, "bars-head": {"type": "fixed"}, "bars-foot": {"type": "fixed"},
    "slip-carrier": {"type": "prismatic", "axisDeg": math.degrees(math.atan2(G.SLIP_SHIFT[1], G.SLIP_SHIFT[0])), "minM": 0, "maxM": 0.05},
    "slip-bars": {"type": "revolute", "minDeg": -5, "maxDeg": 5},
    "base": {"type": "prismatic", "axisDeg": 90, "minM": G.BASE_LOW, "maxM": 0},
    "mattress": {"type": "prismatic", "axisDeg": 90, "minM": -0.02, "maxM": 0},
}
for key in G.CASTORS:
    JOINTS[f"castor-{key}"] = {"type": "revolute", "minDeg": -2, "maxDeg": 2, "twistMinDeg": -180, "twistMaxDeg": 180}
    JOINTS[f"wheel-{key}"] = {"type": "revolute", "minDeg": -180, "maxDeg": 180}


def bone_masses():
    """⚖️ Masses per bone from the 1:50 parts (volume x density), centre in the bone frame."""
    acc = {}
    for p in G.parts(50):
        m = p.mass(DENSITY["mattress-cover"] if p.id == "mattress" else DENSITY[p.material])
        c = G.to_local(p.bone, p.centre())
        a = acc.setdefault(p.bone, [0.0, 0.0, 0.0])
        a[0] += m
        a[1] += m * c[0]
        a[2] += m * c[1]
    return {b: (round(v[0], 3), round(v[1] / v[0], 4), round(v[2] / v[0], 4)) for b, v in acc.items()}


MASSES = bone_masses()
TOTAL_KG = round(sum(v[0] for v in MASSES.values()), 2)


def rig():
    """🦴 The mechanical rig."""
    bones = []
    for bid, parent, x, y, z, ang, length in G.BONES:
        b = {"id": bid, "parent": parent, "title": BONE_TITLES[bid], "x": round(x, 5), "y": round(y, 5), "angleDeg": ang, "lengthM": round(length, 4), "joint": JOINTS[bid]}
        if z:
            b["z"] = round(z, 5)
        if bid in MASSES:
            kg, cx, cy = MASSES[bid]
            b["mass"] = {"kg": kg, "comX": cx, "comY": cy, "distribution": "point", "source": "authored-estimate", "reference": "cot-60x120-catalogue" if bid not in ("mattress",) else "iso-3386-1-1986"}
        bones.append(b)
    return {"kind": "mechanical", "root": "ground", "bones": bones}


CASTOR_NAMES = {"hl": ("head left", "Kopfende links"), "hr": ("head right", "Kopfende rechts"), "fl": ("foot left", "Fußende links"), "fr": ("foot right", "Fußende rechts")}


def anchors():
    """📍 Floor contacts under the four wheels and the two carrying grips on the end top rails."""
    out = []
    for key, (xc, zc, inward) in G.CASTORS.items():
        braked = key in ("hl", "hr")
        en, de = CASTOR_NAMES[key]
        out.append({"id": f"floor-{key}", "bone": f"castor-{key}", "x": G.POST_Y0, "y": inward * G.CASTOR_TRAIL, "kind": "contact", "title": T(f"Wheel on the floor, {en}" + (" (braked)" if braked else ""), f"Rad auf dem Boden, {de}" + (" (gebremst)" if braked else "")), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.012, "footprintWidthM": 0.02, "frictionCoefficient": 0.6 if braked else 0.03})
    out.append({"id": "grip-head", "bone": "frame", "x": G.XH, "y": G.H - 0.02, "kind": "support", "title": T("Hands of the first carrier on the head-end top rail", "Hände der ersten tragenden Person am oberen Kopfholm"), "support": "pin"})
    out.append({"id": "grip-foot", "bone": "frame", "x": G.XF, "y": G.H - 0.02, "kind": "support", "title": T("Hands of the second carrier on the foot-end top rail", "Hände der zweiten tragenden Person am oberen Fußholm"), "support": "pin"})
    return out


def part_index(scale):
    """🗂️ Parts of one scale by id."""
    return {p.id: p for p in G.parts(scale)}


def extras(scale, view):
    """🦷 Traces of use and secondary lines drawn as texture and detail strokes (1:50) or omitted (coarser)."""
    out = []
    if scale != 50:
        return out
    if view == "elevation":
        for i in range(1, 29, 2):
            x = 0.79 - 0.29 * i / 29
            out.append({"id": f"teeth-{i:02d}", "bone": "frame", "role": "texture", "z": 3.5, "filled": False, "tone": 0.25, "kind": "poly", "closed": False, "points": [[round(x - 0.003, 4), 0.854], [round(x, 4), 0.849], [round(x + 0.003, 4), 0.854]], "region": "beech-frame"})
        for k, y in enumerate((0.48, 0.556)):
            out.append({"id": f"piping-{k}", "bone": "mattress", "role": "detail", "z": 0.6, "filled": False, "tone": 0.4, "kind": "poly", "closed": False, "points": [[0.02, round(y - G.MAT_Y[0], 4)], [1.18, round(y - G.MAT_Y[0], 4)]], "region": "mattress"})
    if view == "front":
        cx, cy = 0.12, 0.83
        out.append({"id": "sticker-faded", "bone": "frame", "role": "texture", "z": 13.0, "filled": False, "tone": 0.35, "kind": "ellipse", "cx": cx, "cy": cy, "rx": 0.02, "ry": 0.02, "region": "beech-frame"})
        out.append({"id": "scratches-foot", "bone": "frame", "role": "texture", "z": 13.0, "filled": False, "tone": 0.3, "kind": "path", "d": "M -0.22 0.845 L -0.17 0.835 M -0.15 0.848 L -0.09 0.84 M -0.05 0.83 L -0.01 0.842", "region": "beech-frame"})
    if view == "plan":
        out.append({"id": "stain-mattress", "bone": "mattress", "role": "texture", "z": 6.0, "filled": False, "tone": 0.6, "kind": "poly", "closed": True, "points": [[0.52, -0.08], [0.6, -0.11], [0.69, -0.09], [0.72, -0.02], [0.66, 0.04], [0.57, 0.03], [0.5, -0.02]], "region": "mattress"})
        out.append({"id": "teeth-plan", "bone": "frame", "role": "texture", "z": 8.9, "filled": False, "tone": 0.25, "kind": "path", "d": " ".join(f"M {round(0.79 - 0.29 * i / 29, 4)} -0.336 L {round(0.79 - 0.29 * i / 29, 4)} -0.328" for i in range(1, 29, 2)), "region": "beech-frame"})
    return out


def slat_strip(scale):
    """🪜 The slat layer in the side elevation as one strip (the 13 slats overlap edge-on behind the base rail)."""
    if scale != 50:
        return []
    y0 = G.SLAT_Y[0] - G.BASE_RAIL_Y[0]
    return [{"id": "slat-layer", "bone": "base", "role": "body", "z": 2.976, "filled": True, "tone": 0.62, "kind": "rect", "x": round(G.SLAT_X[0] - G.SLAT_W / 2 - 0.045, 4), "y": round(y0, 4), "w": round(G.SLAT_X[-1] - G.SLAT_X[0] + G.SLAT_W, 4), "h": 0.012, "region": "slatted-base", "material": "beech-slat"}]


def geometry(scale):
    """✏️ The three views of one scale."""
    ps = G.parts(scale)
    out = {}
    for view in ("elevation", "front", "plan"):
        shapes = []
        for p in ps:
            if view == "elevation" and scale == 50 and p.id.startswith("slat-"):
                continue
            shapes += p.shapes(view)
        if view == "elevation":
            shapes += slat_strip(scale)
        shapes += extras(scale, view)
        out[view] = shapes
    return out


DEC = {
    50: {
        "silhouette": T("True orthographic views of the 1:50 model, 25.8 x 17.6 mm in the side elevation: four 40 mm posts with eased edges on 50 mm castors, long and end rails, 20 mm round bars at a clear gap of 57 to 59 mm, the gap of the three removed slip bars in the right long side with the bars laid across the bottom rails behind it, the slatted base in the top position under the sagging mattress.", "Wahre orthogonale Ansichten des Modells 1:50, 25,8 x 17,6 mm in der Seitenansicht: vier 40-mm-Pfosten mit gebrochenen Kanten auf 50-mm-Rollen, Längs- und Stirnholme, runde 20-mm-Sprossen mit 57 bis 59 mm lichtem Abstand, die Lücke der drei herausgenommenen Schlupfsprossen in der rechten Längsseite mit den quer über die unteren Holme gelegten Sprossen dahinter, der Lattenrost in der oberen Stufe unter der durchgelegenen Matratze."),
        "retained": T("Every bar, the teeth marks in the right top rail (3 mm deep, drawn as notches and ticks), the 12 mm sag of the mattress top, the piped seams, the brake pedals of the head castors, the black replacement castor, the faded sticker and scratches on the foot end, the stain on the mattress.", "Jede Sprosse, die Bissspuren im rechten oberen Holm (3 mm tief, als Kerben und Striche gezeichnet), die 12 mm Mulde der Matratzenoberseite, die Kedernähte, die Bremspedale der Kopfrollen, die schwarze Ersatzrolle, der verblasste Aufkleber und die Kratzer am Fußende, der Fleck auf der Matratze."),
        "merged": T("The 13 slats are one strip in the side elevation, where they stand edge-on behind the base rail; in front and plan every slat is drawn.", "Die 13 Leisten sind in der Seitenansicht ein Streifen, weil sie dort hochkant hinter dem Rostholm stehen; in Vorderansicht und Draufsicht ist jede Leiste gezeichnet."),
        "omitted": T("Screws, the hexagon key, the tape round the slip bars and the castor axles are below 7.5 mm and omitted.", "Schrauben, Innensechskantschlüssel, das Klebeband um die Schlupfsprossen und die Rollenachsen liegen unter 7,5 mm und entfallen."),
        "thinElements": T("Bars, rails and slats are filled shapes at their true width; pens 0.35 / 0.18 / 0.13 mm keep every gap above 0.15 mm up to 1:75.", "Sprossen, Holme und Leisten sind gefüllte Formen in wahrer Breite; die Stifte 0,35 / 0,18 / 0,13 mm halten jede Lücke bis 1:75 über 0,15 mm."),
        "hatching": T("Grain hatching along posts and rails (0.6 mm, 0.13 mm pen); bars and slats are plain tones.", "Maserungsschraffur längs der Pfosten und Holme (0,6 mm, Stift 0,13 mm); Sprossen und Leisten sind einfache Töne."),
        "styles": T("Light beech tones for frame and bars, darker waxed slats, a paler replacement slat, white mattress, grey and black castors.", "Helle Buchentöne für Gestell und Sprossen, dunklere gewachste Leisten, eine hellere Ersatzleiste, weiße Matratze, graue und schwarze Rollen."),
        "overlays": T("Thermal regions of frame, base, mattress, castors and brackets; structural members along every bar row, the base and the frame.", "Thermische Regionen von Gestell, Rost, Matratze, Rollen und Haltern; Tragglieder längs jeder Sprossenreihe, des Rosts und des Gestells."),
    },
    100: {
        "silhouette": T("Authored for 15 mm: posts and rails as plain bars without eased edges, every 20 mm bar kept (above the feature), the base as one tray of rails and slats 60 mm deep, the mattress as a plain block, each castor as fork block and wheel.", "Gezeichnet für 15 mm: Pfosten und Holme als einfache Stäbe ohne gebrochene Kanten, jede 20-mm-Sprosse bleibt (über dem Merkmal), der Rost als eine 60 mm tiefe Wanne aus Holmen und Leisten, die Matratze als einfacher Block, jede Rolle als Gabelblock und Rad."),
        "retained": T("All 41 bars and the gap of the slip bars, the slip bars laid across the bottom rails, the four base brackets, the replacement castor by its tone.", "Alle 41 Sprossen und die Lücke der Schlupfsprossen, die quer über die unteren Holme gelegten Schlupfsprossen, die vier Rosthalter, die Ersatzrolle an ihrem Ton."),
        "merged": T("Slats and base rails into one tray; socket and fork of each castor into one block.", "Leisten und Rostholme zu einer Wanne; Steckhülse und Gabel jeder Rolle zu einem Block."),
        "omitted": T("Teeth marks (3 mm), mattress sag (12 mm), piping, brake pedals (8 mm), sticker, scratches and stain are below 15 mm and omitted.", "Bissspuren (3 mm), Matratzenmulde (12 mm), Keder, Bremspedale (8 mm), Aufkleber, Kratzer und Fleck liegen unter 15 mm und entfallen."),
        "thinElements": T("Bars stay filled at 20 mm; the body pen of 0.18 mm keeps the 57 mm gaps printable to 1:150.", "Sprossen bleiben mit 20 mm gefüllt; der Körperstift von 0,18 mm hält die 57-mm-Lücken bis 1:150 druckbar."),
        "hatching": T("No hatching.", "Keine Schraffur."),
        "styles": T("The tones of 1:50.", "Die Töne von 1:50."),
        "overlays": T("Same regions and members as 1:50.", "Dieselben Regionen und Tragglieder wie 1:50."),
    },
    200: {
        "silhouette": T("Authored for 30 mm: posts, rails, base tray, mattress and castors as plain masses; the bar rows typified as fewer 30 mm bars over the true extent of each row (3 + 3 in the right side around the slip gap, 7 in the left side, 3 in each end, 2 of 28 mm for the slip bars).", "Gezeichnet für 30 mm: Pfosten, Holme, Rostwanne, Matratze und Rollen als einfache Massen; die Sprossenreihen typisiert als weniger 30-mm-Sprossen über die wahre Ausdehnung jeder Reihe (3 + 3 in der rechten Seite um die Schlupflücke, 7 links, je 3 an den Enden, 2 zu 28 mm für die Schlupfsprossen)."),
        "retained": T("The open rhythm of the bars, the slip gap, the base and mattress height, castors and brackets.", "Der offene Rhythmus der Sprossen, die Schlupflücke, Rost- und Matratzenhöhe, Rollen und Halter."),
        "merged": T("Bar rows typified (envelope solids in 3D); each castor one fork block and a 30 mm wide wheel.", "Sprossenreihen typisiert (Hüllkörper in 3D); jede Rolle ein Gabelblock und ein 30 mm breites Rad."),
        "omitted": T("Everything below 30 mm that 1:100 still shows: true bar count, slats, rounding.", "Alles unter 30 mm, was 1:100 noch zeigt: wahre Sprossenzahl, Leisten, Rundungen."),
        "thinElements": T("Typified bars of 30 mm at gaps of 116 to 203 mm print to 1:350 with the 0.18 mm pen.", "Typisierte 30-mm-Sprossen mit 116 bis 203 mm Abstand drucken mit dem 0,18-mm-Stift bis 1:350."),
        "hatching": T("No hatching.", "Keine Schraffur."),
        "styles": T("The tones of 1:50.", "Die Töne von 1:50."),
        "overlays": T("Regions as 1:50; the thermal and structural models keep the true 20 mm bars as slender solids.", "Regionen wie 1:50; Thermik- und Tragmodell behalten die wahren 20-mm-Sprossen als schlanke Körper."),
    },
}


def representation(scale):
    """🗺️ The authored representation of one scale."""
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, 100: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350)}[scale]
    hatches = [{"id": "grain-h", "title": T("Grain along the rail", "Maserung längs des Holms"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13}, {"id": "grain-v", "title": T("Grain along the post", "Maserung längs des Pfostens"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.6, "penMm": 0.13}] if scale == 50 else []
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches, "geometry": geometry(scale), "decisions": DEC[scale]}


def unavailable500():
    """🚫 1:500: the whole cot is 2.6 x 1.8 mm on paper."""
    return {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
            "reason": T("At 1:500 the whole cot is 2.6 mm by 1.8 mm on paper, under the 3 mm a recognisable drawing needs, and its 20 mm bars at 57 mm gaps are 0.04 mm and 0.11 mm, below the 0.2 mm feature and 0.15 mm gap of the office-laser profile; draw it at 1:200 or larger.", "Bei 1:500 ist das ganze Kinderbett 2,6 mm mal 1,8 mm groß auf Papier, unter den 3 mm, die eine erkennbare Zeichnung braucht, und seine 20-mm-Sprossen mit 57 mm Abstand messen 0,04 mm und 0,11 mm, unter dem Merkmal von 0,2 mm und der Lücke von 0,15 mm des Büro-Laserprofils; bei 1:200 oder größer zeichnen."),
            "limitingFeature": T("Whole cot of 1.29 m and its 20 mm bars", "Ganzes Bett von 1,29 m und seine 20-mm-Sprossen"), "limitingSizeM": 1.29, "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}


LAYERS = {"beech-frame": "beech-member", "slatted-base": "slat", "castors": "polymer-shell", "brackets": "zinc-steel", "mattress": "cover"}


def thermal_ref(p, sid=None):
    """🌡️ A thermal reference to a body part."""
    return {"id": sid or f"th-{p.id}", "of": p.id, "region": p.region, "layer": LAYERS[p.region]}


def mattress_core(scale):
    """🧽 The foam core of the mattress, 10 mm inside the cover."""
    x0, x1 = G.MAT_X
    y0, y1 = G.MAT_Y
    return {"id": "th-mattress-core", "kind": "box", "bone": "mattress", "material": "cold-foam", "role": "body", "tone": 0.75, "size": [round(x1 - x0 - 0.02, 4), round(y1 - y0 - 0.02 - (0.012 if scale == 50 else 0), 4), round(2 * G.MAT_Z - 0.02, 4)], "transform": {"translation": [0.6, round((y1 - y0 - (0.012 if scale == 50 else 0)) / 2, 4), 0]}, "region": "mattress", "layer": "foam-core"}


MEMBER_OF = {"frame": "frame", "bars-right": "bars-right", "bars-left": "bars-left", "bars-head": "bars-head", "bars-foot": "bars-foot", "slip-bars": "slip-bars", "base": "base", "mattress": "mattress"}


def octagonal(b, sid):
    """🛑 A true bar of the typified scale as an octagonal prism of its 20 mm section along its bone (the circle generalised for 30 mm)."""
    s = b.solid(sid)
    o, length, r = s["origin"], s["profile"][2][0], b.k["r"]
    ring = [[round(o[1] + r * math.cos(math.pi * (2 * i + 1) / 8), 6), round(o[2] + r * math.sin(math.pi * (2 * i + 1) / 8), 6)] for i in range(8)]
    for key in ("profile", "axis", "origin"):
        s.pop(key)
    return {**s, "kind": "extrude", "profile": ring, "plane": "yz", "from": round(o[0], 6), "to": round(o[0] + length, 6)}


def volume(scale):
    """🧊 The handcrafted volume of one scale: body, thermal and structure lists."""
    ps = G.parts(scale)
    body = [p.solid() for p in ps if p.k.get("solid", True)]
    thermal, structure = [], []
    bars = G.true_bars(scale) if scale == 200 else []
    for p in ps:
        if p.flags.get("envelope") and scale != 500:
            continue
        if p.id == "mattress" and scale != 500:
            thermal.append(mattress_core(scale))
        if p.id == "mattress" and scale == 500:
            thermal.append({**p.solid("th-mattress"), "region": "mattress", "layer": "cover"})
            continue
        thermal.append(thermal_ref(p))
    for b in bars:
        thermal.append({**octagonal(b, f"th-{b.id}"), "region": "beech-frame", "layer": "beech-member"})
    for p in ps:
        if p.flags.get("envelope") and scale != 500:
            continue
        if p.bone in MEMBER_OF and p.bone != "mattress":
            if p.region == "brackets":
                structure.append({"id": f"joint-{p.id}", "of": p.id, "member": "base", "part": "joint"})
            else:
                structure.append({"id": f"member-{p.id}", "of": p.id, "member": MEMBER_OF[p.bone], "part": "member"})
        elif p.bone.startswith("castor-") or p.bone.startswith("wheel-"):
            structure.append({"id": f"member-{p.id}", "of": p.id, "member": p.bone, "part": "member"})
    for b in bars:
        structure.append({**octagonal(b, f"st-{b.id}"), "member": b.bone, "part": "member"})
    x0, x1 = G.MAT_X
    structure.append({"id": "member-mattress", "kind": "box", "bone": "mattress", "material": "cold-foam", "role": "body", "tone": 0.4, "size": [{50: 1.16, 100: 1.16, 200: 1.14, 500: 1.1}[scale], {50: 0.03, 100: 0.03, 200: 0.04, 500: 0.06}[scale], {50: 0.56, 100: 0.56, 200: 0.54, 500: 0.5}[scale]], "transform": {"translation": [0.6, {50: 0.015, 100: 0.015, 200: 0.02, 500: 0.03}[scale], 0]}, "member": "mattress", "part": "member"})
    if scale in (50, 100):
        structure.append({"id": "mass-mattress", "kind": "ellipsoid", "bone": "mattress", "material": "cold-foam", "role": "detail", "tone": 0.3, "radii": [0.04, 0.03, 0.04] if scale == 50 else [0.05, 0.03, 0.05], "transform": {"translation": [0.6, 0.05, 0]}, "member": "mattress", "part": "mass"})
    if scale == 50:
        structure += dowels()
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VDEC[scale]}


def dowels():
    """🔩 Beech dowel joints of 10 mm at both ends of every rail, inside the posts."""
    out = []
    for zn, zc in (("l", G.ZS), ("r", -G.ZS)):
        for yn, (y0, y1) in (("top", G.LONG_TOP), ("bottom", G.BOTTOM)):
            yc = round((y0 + y1) / 2, 4)
            out.append({"id": f"joint-dowel-{yn}-{zn}", "kind": "revolve", "bone": "frame", "material": "beech-lacquered-aged", "role": "detail", "tone": 0.2, "profile": [[0, 0], [0, 0.005], [0.04, 0.005], [0.04, 0]], "axis": [1, 0, 0], "origin": [0.02, yc, zc], "repeat": {"count": 2, "translation": [1.21, 0, 0]}, "member": "frame", "part": "joint"})
    for xn, xc in (("head", G.XH), ("foot", G.XF)):
        for yn, (y0, y1) in (("top", G.END_TOP), ("bottom", G.BOTTOM)):
            yc = round((y0 + y1) / 2, 4)
            out.append({"id": f"joint-dowel-{xn}-{yn}", "kind": "revolve", "bone": "frame", "material": "beech-lacquered-aged", "role": "detail", "tone": 0.2, "profile": [[0, 0], [0, 0.005], [0.04, 0.005], [0.04, 0]], "axis": [0, 0, 1], "origin": [xc, yc, -G.ZS - 0.02], "repeat": {"count": 2, "translation": [0, 0, 2 * G.ZS]}, "member": "frame", "part": "joint"})
    return out


VDEC = {
    50: [T("Body, authored for 7.5 mm: four 40 x 40 mm beech posts with 6 mm eased edges from 70 mm to 0.88 m; long rails 28 x 45 mm (top) and 28 x 35 mm (bottom), the right top rail with its 0.29 m run of teeth marks up to 3 mm deep in the profile; end rails 28 x 55 / 28 x 35 mm; 41 round 20 mm bars with 15 mm tenons in the rails; three slip bars removed and laid across the bottom rails half a pitch beside the left bars; the slatted base with two 22 x 40 mm rails, two end rails, 13 slats 60 x 10 mm (one an unfinished pine replacement) on four zinc-plated hanger brackets; the 60 x 120 x 10 cm mattress with rounded ends and a 12 mm sag; four castors with socket, fork, 50 mm wheel, brake pedals on the head pair and a black replacement at the foot right.", "Körper, gebaut für 7,5 mm: vier Buchenpfosten 40 x 40 mm mit 6 mm gebrochenen Kanten von 70 mm bis 0,88 m; Längsholme 28 x 45 mm (oben) und 28 x 35 mm (unten), der rechte obere Holm mit seinem 0,29 m langen Bereich bis 3 mm tiefer Bissspuren im Profil; Stirnholme 28 x 55 / 28 x 35 mm; 41 runde 20-mm-Sprossen mit 15-mm-Zapfen in den Holmen; drei Schlupfsprossen herausgenommen und quer über die unteren Holme gelegt, eine halbe Teilung neben den linken Sprossen; der Lattenrost mit zwei Holmen 22 x 40 mm, zwei Stirnleisten, 13 Leisten 60 x 10 mm (eine aus unbehandelter Kiefer ersetzt) auf vier verzinkten Rosthaltern; die Matratze 60 x 120 x 10 cm mit gerundeten Enden und 12 mm Mulde; vier Rollen mit Hülse, Gabel, 50-mm-Rad, Bremspedalen am Kopfpaar und einer schwarzen Ersatzrolle hinten rechts."),
         T("Thermal: every timber part of frame and bars is the lacquered beech surface, the base the waxed slat surface, the mattress two shells (10 mm cover around the foam core), castors and brackets their own surfaces. Structure: posts, rails, bars, slip bars, base rails and slats, the forks and wheels of the castors are members; 16 rail dowels and the four brackets are joints; the mattress is a 30 mm bearing strip of foam with its mass at the centre; the floor contacts under the wheels are the supports of the pose, the grips on the end rails those of the carried pose.", "Thermik: jedes Holzteil von Gestell und Sprossen ist die lackierte Buchenoberfläche, der Rost die gewachste Leistenoberfläche, die Matratze zwei Schalen (10 mm Bezug um den Schaumkern), Rollen und Halter eigene Oberflächen. Tragwerk: Pfosten, Holme, Sprossen, Schlupfsprossen, Rostholme und Leisten, Gabeln und Räder der Rollen sind Tragglieder; 16 Holmdübel und die vier Halter sind Gelenke; die Matratze ist ein 30 mm Auflagerstreifen aus Schaum mit ihrer Masse in der Mitte; die Bodenkontakte unter den Rädern sind die Auflager der Pose, die Griffe an den Stirnholmen die der getragenen Pose.")],
    100: [T("Body, authored anew for 15 mm: posts and rails as plain boxes (eased edges and teeth marks below the feature), 41 bars as capsules at the true 20 mm, the slip bars laid across the bottom rails, the base as one 60 mm tray of rails and slats, the brackets, the mattress as a plain block (12 mm sag dropped), each castor as fork block and 50 mm wheel (socket and brake pedal merged or dropped).", "Körper, neu gebaut für 15 mm: Pfosten und Holme als einfache Quader (gebrochene Kanten und Bissspuren unter dem Merkmal), 41 Sprossen als Kapseln in wahren 20 mm, die quer über die unteren Holme gelegten Schlupfsprossen, der Rost als eine 60-mm-Wanne aus Holmen und Leisten, die Halter, die Matratze als einfacher Block (12 mm Mulde entfällt), jede Rolle als Gabelblock und 50-mm-Rad (Hülse und Bremspedal verschmolzen oder entfallen)."),
          T("Thermal and structure follow the body solids of this scale; the rail dowels (10 mm) are below the feature and left out, the brackets stay the joints of the base.", "Thermik und Tragwerk folgen den Körpern dieses Maßstabs; die Holmdübel (10 mm) liegen unter dem Merkmal und entfallen, die Halter bleiben die Gelenke des Rosts.")],
    200: [T("Body, authored anew for 30 mm: posts, rails, tray, mattress and brackets as plain masses; the bar rows typified as 30 mm envelope bars over the true extent of each row (3 + 3, 7, 3, 3 and 2 slip bars), each castor a fork block with a 30 mm wide wheel.", "Körper, neu gebaut für 30 mm: Pfosten, Holme, Wanne, Matratze und Halter als einfache Massen; die Sprossenreihen typisiert als 30-mm-Hüllsprossen über die wahre Ausdehnung jeder Reihe (3 + 3, 7, 3, 3 und 2 Schlupfsprossen), jede Rolle ein Gabelblock mit 30 mm breitem Rad."),
          T("Thermal and structure keep the 44 true 20 mm bars as slender solids (their surface and their members are real), the rest follows the body of this scale.", "Thermik und Tragwerk behalten die 44 wahren 20-mm-Sprossen als schlanke Körper (ihre Oberfläche und ihre Tragglieder sind real), der Rest folgt dem Körper dieses Maßstabs.")],
    500: [T("Digital model only (no print drawing at 1:500): posts and rails as plain bars, each bar row and the slip bars as one envelope slab of the row's true extent, base tray and mattress as two blocks, each castor a 40 mm block. The model is minimal.", "Nur digitales Modell (keine Druckzeichnung bei 1:500): Pfosten und Holme als einfache Stäbe, jede Sprossenreihe und die Schlupfsprossen als eine Hüllplatte über die wahre Ausdehnung der Reihe, Rostwanne und Matratze als zwei Blöcke, jede Rolle ein 40-mm-Block. Das Modell ist minimal."),
          T("Thermal: the envelope slabs stand for the bar rows (they give the beech region its coverage; their hull is not counted as surface), the rest refers to the body. Structure: each envelope slab is the member of its bar row, posts, rails, tray, castor blocks and the four brackets as before.", "Thermik: die Hüllplatten stehen für die Sprossenreihen (sie geben der Buchenregion ihre Abdeckung; ihre Hülle zählt nicht als Oberfläche), der Rest verweist auf den Körper. Tragwerk: jede Hüllplatte ist das Tragglied ihrer Sprossenreihe, Pfosten, Holme, Wanne, Rollenblöcke und die vier Halter wie zuvor.")],
}


def regions_binding(pose):
    """🌬️ Region bindings of one pose."""
    sun = 0.0 if pose == "carried" else 0.2
    mat = {"exposedFraction": 0.75, "sunlitFraction": sun}
    if pose == "in-use":
        mat = {"exposedFraction": 0.63, "sunlitFraction": 0.0, "contact": {"fraction": 0.12, "tempC": 31}}
    if pose == "newborn":
        mat = {"exposedFraction": 0.67, "sunlitFraction": 0.0, "contact": {"fraction": 0.08, "tempC": 32}}
    castors = {"exposedFraction": 1.0, "sunlitFraction": 0.0} if pose == "carried" else {"exposedFraction": 0.95, "sunlitFraction": 0.0, "contact": {"fraction": 0.02, "tempC": 18}}
    return {"beech-frame": {"exposedFraction": 1.0, "sunlitFraction": sun}, "slatted-base": {"exposedFraction": 0.6, "sunlitFraction": 0.0}, "mattress": mat, "castors": castors, "brackets": {"exposedFraction": 0.7, "sunlitFraction": 0.0}}


FLOORS = [f"floor-{k}" for k in G.CASTORS]
STORED_BONES = {"slip-carrier": {"dx": G.SLIP_SHIFT[0], "dy": G.SLIP_SHIFT[1]}, "slip-bars": {"abductionDeg": 90}}


def pose(pid, title, desc, bones, contacts, loads=(), key=False, root=None, binding=None):
    """📥 One pose."""
    p = {"id": pid, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "authored", "contacts": contacts, "thermal": {"regions": regions_binding(binding or pid), "emitters": {}}, "loads": list(loads)})
    p["root"] = root or {"x": 0, "y": 0, "angleDeg": 0}
    return p


def occupant(lid, kg, a, b, en, de):
    """👶 The child lying on the mattress as a distributed load along the mattress bone."""
    return {"id": lid, "kind": "occupant", "title": T(en, de), "distributed": {"bone": "mattress", "fromM": a, "toM": b, "wxNPerM": 0, "wyNPerM": round(-kg * 9.80665 / (b - a), 2)}, "fxN": 0, "fyN": 0, "source": "literature", "reference": "who-growth-2006"}


def poses():
    """🧍 Named poses and clip keys."""
    lift = 0.30
    return [
        pose("stored", T("Stored at the reuse depot", "Gelagert im Gebrauchtwarenlager"), T("As the depot offers it: assembled, cleaned, base in the top position, the three removed slip bars laid across the bottom rails, head castors braked, on its four wheels.", "Wie das Lager es anbietet: aufgebaut, gereinigt, Rost in der oberen Stufe, die drei herausgenommenen Schlupfsprossen quer über die unteren Holme gelegt, Kopfrollen gebremst, auf seinen vier Rädern."), dict(STORED_BONES), FLOORS),
        pose("carried", T("Carried by two people", "Von zwei Personen getragen"), T("Two people lift the cot by the end top rails 0.30 m off the floor, level, to carry it over a threshold or down the stairs; the slip bars and the mattress ride along.", "Zwei Personen heben das Bett an den oberen Stirnholmen 0,30 m vom Boden, waagerecht, um es über eine Schwelle oder die Treppe hinunter zu tragen; Schlupfsprossen und Matratze fahren mit."), dict(STORED_BONES), ["grip-head", "grip-foot"], root={"x": 0, "y": lift, "angleDeg": 0}),
        pose("in-use", T("In use with a sitting-age child", "In Gebrauch mit einem Kind im Sitzalter"), T("Back in a nursery: slip bars refitted, base in the lowest position (internal depth 0.62 m), a nine-month-old child of 8.9 kg asleep on the mattress, which sinks 6 mm.", "Wieder im Kinderzimmer: Schlupfsprossen eingesetzt, Rost in der untersten Stufe (Innenhöhe 0,62 m), ein neun Monate altes Kind von 8,9 kg schläft auf der Matratze, die 6 mm einsinkt."), {"base": {"dy": G.BASE_LOW}, "mattress": {"dy": -0.006}}, FLOORS, [occupant("child-asleep", 8.9, 0.25, 0.97, "Child of nine months asleep (8.9 kg over 0.72 m)", "Kind von neun Monaten schlafend (8,9 kg auf 0,72 m)")]),
        pose("newborn", T("In use with a newborn", "In Gebrauch mit einem Neugeborenen"), T("Base in the top position for a baby that cannot sit up yet (internal depth 0.39 m), slip bars in place, a newborn of 3.3 kg asleep in a sleeping bag.", "Rost in der oberen Stufe für ein Baby, das noch nicht sitzen kann (Innenhöhe 0,39 m), Schlupfsprossen eingesetzt, ein Neugeborenes von 3,3 kg schläft im Schlafsack."), {"mattress": {"dy": -0.003}}, FLOORS, [occupant("newborn-asleep", 3.3, 0.35, 0.85, "Newborn asleep (3.3 kg over 0.50 m)", "Neugeborenes schlafend (3,3 kg auf 0,50 m)")]),
        pose("base-top-key", T("Base in the top position (key)", "Rost in der oberen Stufe (Schlüssel)"), T("Empty cot, bars in place, base at the top.", "Leeres Bett, Sprossen eingesetzt, Rost oben."), {}, FLOORS, key=True, binding="stored"),
        pose("base-mid-key", T("Base in the middle position (key)", "Rost in der mittleren Stufe (Schlüssel)"), T("Empty cot, base lowered by 0.115 m to the middle holes.", "Leeres Bett, Rost um 0,115 m in die mittleren Bohrungen gesenkt."), {"base": {"dy": G.BASE_MID}}, FLOORS, key=True, binding="stored"),
        pose("base-low-key", T("Base in the lowest position (key)", "Rost in der untersten Stufe (Schlüssel)"), T("Empty cot, base in the lowest holes.", "Leeres Bett, Rost in den untersten Bohrungen."), {"base": {"dy": G.BASE_LOW}}, FLOORS, key=True, binding="stored"),
    ]


def clips():
    """🎞️ The carry clip and the lowering of the base."""
    carry = {"id": "carry", "title": T("Carry away", "Forttragen"), "description": T("Two people take the end top rails, lift the stored cot 0.30 m off the floor and hold it level to carry it.", "Zwei Personen fassen die oberen Stirnholme, heben das gelagerte Bett 0,30 m vom Boden und halten es zum Tragen waagerecht."),
             "durationS": 3.0, "fps": 30, "loop": False, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "stored", "ease": "smooth"}, {"t": 2.4, "pose": "carried", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth"}],
             "contacts": [{"anchor": a, "fromS": 0, "toS": 1.0} for a in FLOORS] + [{"anchor": "grip-head", "fromS": 0.6, "toS": 3.0}, {"anchor": "grip-foot", "fromS": 0.6, "toS": 3.0}],
             "phases": [{"id": "grip", "fromS": 0, "toS": 1.0, "title": T("Take hold", "Anfassen")}, {"id": "lift", "fromS": 1.0, "toS": 2.4, "title": T("Lift", "Anheben")}, {"id": "carry", "fromS": 2.4, "toS": 3.0, "title": T("Carry", "Tragen")}],
             "events": [{"t": 0.6, "kind": "contact-on", "title": T("Hands on the end rails", "Hände an den Stirnholmen"), "target": "grip-head"}, {"t": 1.0, "kind": "contact-off", "title": T("Wheels leave the floor", "Räder verlassen den Boden"), "target": "floor-hl"}, {"t": 2.4, "kind": "motion", "title": T("Carrying height reached", "Tragehöhe erreicht")}],
             "tracks": []}
    lower = {"id": "lower-base", "title": T("Lower the base", "Rost absenken"), "description": T("When the child begins to sit up, the base goes from the top position through the middle to the lowest holes, 0.23 m in all; the mattress rides on it.", "Wenn das Kind zu sitzen beginnt, wandert der Rost aus der oberen Stufe über die mittlere in die untersten Bohrungen, insgesamt 0,23 m; die Matratze fährt mit."),
             "durationS": 9.0, "fps": 30, "loop": False, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "base-top-key", "ease": "smooth"}, {"t": 1.5, "pose": "base-top-key", "ease": "smooth"}, {"t": 4.5, "pose": "base-mid-key", "ease": "smooth"}, {"t": 6.0, "pose": "base-mid-key", "ease": "smooth"}, {"t": 9.0, "pose": "base-low-key", "ease": "smooth"}],
             "contacts": [{"anchor": a, "fromS": 0, "toS": 9.0} for a in FLOORS],
             "phases": [{"id": "to-middle", "fromS": 1.5, "toS": 4.5, "title": T("Top to middle", "Oben nach Mitte")}, {"id": "to-low", "fromS": 6.0, "toS": 9.0, "title": T("Middle to lowest", "Mitte nach unten")}],
             "events": [{"t": 4.5, "kind": "motion", "title": T("Base screwed into the middle holes", "Rost in den mittleren Bohrungen verschraubt"), "target": "base"}, {"t": 9.0, "kind": "motion", "title": T("Base in the lowest holes", "Rost in den untersten Bohrungen"), "target": "base"}],
             "tracks": []}
    return [carry, lower]


def region(rid, title, kind, material, area, char, geom, sky, layers, extra=None):
    """🔥 One passive thermal region."""
    hc = round(sum(l["heatCapacityJM2K"] for l in layers), 1)
    r = {"id": rid, "title": title, "kind": kind, "model": "passive", "material": material, "areaM2": area, "characteristicLengthM": char, "geometry": geom, "skyViewFactor": sky, "layers": layers, "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        r.update(extra)
    return r


def layer(lid, title, kind, material, t, k, rho, cp, ref="iso-10456-2007"):
    """🧶 One layer with resistance and areal heat capacity from its thickness."""
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / k, 5), "heatCapacityJM2K": round(rho * cp * t, 1), "source": "literature", "reference": ref}


AREAS = {"beech-frame": 3.15, "slatted-base": 1.45, "mattress": 1.92, "castors": 0.053, "brackets": 0.034}


def thermal():
    """🌡️ Passive thermal model of the cot in a nursery."""
    return {
        "environment": {"airTempC": 18, "meanRadiantTempC": 18, "relativeHumidity": 0.5, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 200, "diffuseFraction": 0.3, "pressurePa": 101325},
        "passengers": [], "operating": [], "emitters": [],
        "regions": [
            region("beech-frame", T("Lacquered beech of posts, rails and bars", "Lackierte Buche von Pfosten, Holmen und Sprossen"), "surface", "beech-lacquered-aged", AREAS["beech-frame"], 0.022, "cylinder", 0.5, [layer("beech-member", T("Beech, half bar 11 mm", "Buche, halbe Sprosse 11 mm"), "timber", "beech-lacquered-aged", 0.011, 0.16, 720, 1600)], {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
            region("slatted-base", T("Slatted base, waxed beech", "Lattenrost, gewachste Buche"), "surface", "beech-slat", AREAS["slatted-base"], 0.06, "horizontal-plate-up", 0.2, [layer("slat", T("Beech slat, half thickness 5 mm", "Buchenleiste, halbe Dicke 5 mm"), "timber", "beech-slat", 0.005, 0.16, 690, 1600)]),
            region("mattress", T("Mattress cover and sides", "Matratzenbezug und Seiten"), "surface", "mattress-cover", AREAS["mattress"], 0.6, "horizontal-plate-up", 0.5, [layer("foam-core", T("Cold foam core 80 mm", "Kaltschaumkern 80 mm"), "insulation", "cold-foam", 0.08, 0.04, 40, 1400, "iso-3386-1-1986"), layer("cover", T("Quilted cotton cover 10 mm", "Gesteppter Baumwollbezug 10 mm"), "clothing", "mattress-cover", 0.01, 0.05, 150, 1300, "incropera-2018")]),
            region("castors", T("Castor forks and worn wheels", "Rollengabeln und abgefahrene Räder"), "surface", "castor-polyamide", AREAS["castors"], 0.05, "cylinder", 0.1, [layer("polymer-shell", T("Polyamide wall 3 mm", "Polyamidwand 3 mm"), "polymer", "castor-polyamide", 0.003, 0.25, 1140, 1600)]),
            region("brackets", T("Zinc-plated hanger brackets of the base", "Verzinkte Rosthalter"), "surface", "steel-zinc", AREAS["brackets"], 0.03, "vertical-plate", 0.2, [layer("zinc-steel", T("Steel angle 1.5 mm", "Stahlwinkel 1,5 mm"), "metal", "steel-zinc", 0.0015, 50, 7850, 450, "incropera-2018")]),
        ],
        "assumptions": [
            T("Reference case: a nursery at 18 degC (the room temperature recommended for sleeping infants), still air, 200 W/m2 of window sun on the cot; every part is passive and draws no power.", "Referenzfall: ein Kinderzimmer bei 18 degC (die für schlafende Säuglinge empfohlene Raumtemperatur), ruhige Luft, 200 W/m2 Fenstersonne auf dem Bett; jedes Teil ist passiv und nimmt keine Leistung auf."),
            T("Aged surfaces: the clear lacquer of the beech has yellowed and is scratched, so its solar absorptance is taken as 0.55 against about 0.45 for new light beech and its emissivity 0.91; the waxed slats have greyed (0.58); the black replacement castor absorbs 0.9 against 0.6 of the grey originals (authored-estimate after Incropera).", "Gealterte Oberflächen: der Klarlack der Buche ist vergilbt und verkratzt, deshalb wird sein solarer Absorptionsgrad mit 0,55 statt etwa 0,45 für neue helle Buche angesetzt und sein Emissionsgrad mit 0,91; die gewachsten Leisten sind vergraut (0,58); die schwarze Ersatzrolle absorbiert 0,9 statt 0,6 der grauen Originale (Schätzwert nach Incropera)."),
            T("The child is not part of the asset: in use it is a contact on 12 % (nine months) or 8 % (newborn) of the mattress surface at 31 to 32 degC, the surface of a sleeping bag over a sleeping infant.", "Das Kind gehört nicht zum Asset: in Gebrauch ist es ein Kontakt auf 12 % (neun Monate) oder 8 % (Neugeborenes) der Matratzenoberfläche bei 31 bis 32 degC, der Oberfläche eines Schlafsacks über einem schlafenden Säugling."),
        ],
    }


def structure():
    """🏗️ Structural model."""
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [
        T("The cot is one rigid frame of four 40 mm beech posts with dowelled long and end rails and 44 round bars of 20 mm; it stands on four castors whose wheel contacts are the supports when stored or in use (head pair braked, friction 0.6; foot pair rolling, 0.03). Carried, the two grips on the end top rails are pins and the castors are in the air.", "Das Bett ist ein starrer Rahmen aus vier 40-mm-Buchenpfosten mit gedübelten Längs- und Stirnholmen und 44 runden 20-mm-Sprossen; es steht auf vier Rollen, deren Radkontakte gelagert und in Gebrauch die Auflager sind (Kopfpaar gebremst, Reibung 0,6; Fußpaar rollend, 0,03). Getragen sind die beiden Griffe an den oberen Stirnholmen Gelenke und die Rollen in der Luft."),
        T("The slatted base hangs on four zinc-plated brackets screwed into the posts at three heights (slat tops 0.47, 0.355 and 0.24 m); the mattress lies on it and sinks 3 to 6 mm under the child (cold foam, ISO 3386-1 order of 4 kPa at 40 % compression). EN 716-2 strength tests of base, sides and bars are the scope, no test load is modelled.", "Der Lattenrost hängt an vier verzinkten Haltern, die in drei Höhen in die Pfosten geschraubt sind (Leistenoberkante 0,47, 0,355 und 0,24 m); die Matratze liegt darauf und sinkt unter dem Kind 3 bis 6 mm ein (Kaltschaum, Größenordnung 4 kPa bei 40 % Stauchung nach ISO 3386-1). Die Festigkeitsprüfungen von Rost, Seiten und Sprossen nach EN 716-2 sind der Rahmen, keine Prüflast ist modelliert."),
        T("Masses from section x length x density of every part (beech 720, slats 690, pine 500, foam mattress 3.5 kg, polyamide 1140, steel 7850 kg/m3); the child is a distributed occupant load of 8.9 or 3.3 kg (WHO medians) on the mattress.", "Massen aus Querschnitt x Länge x Dichte jedes Teils (Buche 720, Leisten 690, Kiefer 500, Schaummatratze 3,5 kg, Polyamid 1140, Stahl 7850 kg/m3); das Kind ist eine verteilte Personenlast von 8,9 oder 3,3 kg (WHO-Mediane) auf der Matratze."),
    ], "clinical": False}


def document():
    """📄 The whole asset document."""
    reps = {"500": unavailable500(), "200": representation(200), "100": representation(100), "50": representation(50)}
    vols = {"500": volume(500), "200": volume(200), "100": volume(100), "50": volume(50)}
    asset = {
        "id": "used-baby-cot", "category": "furniture",
        "title": T("Used Baby Cot", "Gebrauchtes Kinderbett"),
        "summary": T(f"A second-hand convertible beech cot for a 60 x 120 cm mattress, 1.29 x 0.69 x 0.88 m on four 50 mm castors, {TOTAL_KG:.1f} kg with its mattress: 20 mm round bars at 57 mm gaps, three removable slip bars, a slatted base in three heights, teeth marks gnawed into the top rail, a sagging mattress, one replaced slat and one mismatched castor; stored at the reuse depot with the slip bars laid inside, carried by two people and back in use with a sleeping child.", f"Ein gebrauchtes umbaubares Kinderbett aus Buche für eine Matratze 60 x 120 cm, 1,29 x 0,69 x 0,88 m auf vier 50-mm-Rollen, {TOTAL_KG:.1f} kg mit Matratze: runde 20-mm-Sprossen mit 57 mm Abstand, drei herausnehmbare Schlupfsprossen, ein Lattenrost in drei Höhen, in den oberen Holm genagte Bissspuren, eine durchgelegene Matratze, eine ersetzte Leiste und eine fremde Rolle; im Gebrauchtwarenlager mit den eingelegten Schlupfsprossen gelagert, von zwei Personen getragen und wieder in Gebrauch mit einem schlafenden Kind."),
        "tags": ["used", "household", "second-hand", "children", "bedroom", "baby-cot", "cot", "bed", "beech", "solid-wood", "convertible", "slatted-base", "mattress", "castors", "interior"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS],
                       "notes": T("Type of a German mid-range beech cot of the 2000s to 2010s (no brand). Dimensions, sections and the height positions of the base are catalogue-typical and checked against EN 716-1 (bar gaps 57 and 59 mm, internal depth 0.62 m lowest and 0.39 m top); masses from section x density (authored-estimate). Traces of use: teeth marks on the right top rail, yellowed and scratched lacquer, a faded sticker, a sagging and stained mattress, one pine replacement slat, one black replacement castor without brake. Nothing is measured.", "Typ eines deutschen Mittelklasse-Kinderbetts aus Buche der 2000er bis 2010er Jahre (keine Marke). Maße, Querschnitte und die Höhenstufen des Rosts sind katalogtypisch und an EN 716-1 geprüft (Sprossenabstände 57 und 59 mm, Innenhöhe 0,62 m unten und 0,39 m oben); Massen aus Querschnitt x Dichte (Schätzwert). Gebrauchsspuren: Bissspuren am rechten oberen Holm, vergilbter und verkratzter Lack, ein verblasster Aufkleber, eine durchgelegene und fleckige Matratze, eine Ersatzleiste aus Kiefer, eine schwarze Ersatzrolle ohne Bremse. Nichts ist gemessen.")},
        "dimensions": {"heightM": G.H, "widthM": G.L, "depthM": G.W, "massKg": TOTAL_KG},
        "materials": MATERIALS, "rig": rig(), "anchors": anchors(), "representations": reps, "symbols": [],
        "poses": poses(), "clips": clips(), "thermal": thermal(), "structure": structure(), "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print("[DEBUG] written, mass", TOTAL_KG, {b: m[0] for b, m in MASSES.items()})
