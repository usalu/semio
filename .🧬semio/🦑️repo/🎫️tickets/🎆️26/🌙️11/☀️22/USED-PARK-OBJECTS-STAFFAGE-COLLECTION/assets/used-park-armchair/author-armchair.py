"""🪑 Authoritative author script of `used-park-armchair`: states every hand-chosen number of the used movable park armchair and writes `🔣️.json` next to it.

Run: `python3 author-armchair.py` (prints dimensions, masses and areas as `[DEBUG]` lines).
@see ../../brief.md
@see author_geometry.py — the construction
@see author_parts.py — the parts per scale
"""
import json
import math
import os
from author_geometry import (BACK_DIR, BONES, CARRIER_BACK_TOP_H, CARRIER_BACK_U, CARRIER_H, CARRIER_SEAT_U, CARRIER_W, CORNER_FRONT, CORNER_REAR, FLANGE_R, FLANGE_T, FOOT_FRONT, FOOT_H, FOOT_REAR,
                             HEM_R, OPEN_AREA, RAIL_R, RHO_STEEL, SEAT_HALF, SHEET_T, STRETCH_R, STRETCH_Y, T_R, T_SEC, dir_local, norm, path_length, side_path, sub, to_local,
                             to_world, tube_mass, unit, zleg)
from author_parts import FEATURE, SIDES, outline, parts_of, r6, solid

HERE = os.path.dirname(os.path.abspath(__file__))
SCALES = (50, 100, 200, 500)


def T(en, de):
    """🌍 A bilingual text, English first."""
    return {"en": en, "de": de}


REFS = [
    ("din-en-581-1-2017", "DIN EN 581-1:2017-09 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 1: General safety requirements."),
    ("din-en-581-2-2015", "DIN EN 581-2:2015-12 Outdoor furniture - Part 2: Mechanical safety requirements and test methods for seating (seat static load 1300 N, back static load 560 N, arm downward load 900 N for contract use)."),
    ("din-en-16139-2013", "DIN EN 16139:2013-07 Furniture - Strength, durability and safety - Requirements for non-domestic seating (level 2: public spaces with heavy use)."),
    ("din-en-1728-2012", "DIN EN 1728:2012-12 Furniture - Seating - Test methods for the determination of strength and durability (seat and back load points)."),
    ("din-en-10305-3-2016", "DIN EN 10305-3:2016-08 Steel tubes for precision applications - Welded cold sized tubes (Ø25 x 2 mm: 1.13 kg/m, A = 144.5 mm2, I = 9.63 cm4 x 0.1)."),
    ("din-en-10058-2018", "DIN EN 10058:2018-08 Hot rolled flat steel bars for general purposes - Dimensions and tolerances (30 x 8 mm: 1.88 kg/m)."),
    ("din-24041-2002", "DIN 24041:2002-12 Perforated plates - Dimensions (round holes in staggered rows, Rv 8-12: hole 8 mm, pitch 12 mm, open area 40 %)."),
    ("din-en-iso-12944-2-2018", "DIN EN ISO 12944-2:2018-04 Paints and varnishes - Corrosion protection of steel structures by protective paint systems - Part 2: Classification of environments (C3 urban atmosphere; feet in wet gravel behave as C4/Im3 splash zone)."),
    ("din-en-1993-1-1-2010", "DIN EN 1993-1-1:2010-12 Eurocode 3: Design of steel structures - General rules (E = 210 GPa, density 7850 kg/m3, S235 fy = 235 MPa)."),
    ("din-en-1991-1-1-2010", "DIN EN 1991-1-1:2010-12 Eurocode 1: Actions on structures - Densities, self-weight, imposed loads."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel: conductivity 50 W/mK, specific heat 450 J/kgK)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley, Table A.11 (oxidised and rusted steel emissivity 0.79-0.85, polished or abraded steel 0.2-0.4, paints 0.9)."),
    ("levinson-2005", "Levinson, R., Berdahl, P., Akbari, H. (2005). Solar spectral optical properties of pigments, Parts I and II. Solar Energy Materials and Solar Cells 89, 319-389 (dark green chromium-oxide and phthalocyanine coatings: solar absorptance 0.85-0.9 new; chalked and faded films reflect more)."),
    ("iso-7730-2005", "ISO 7730:2005 Ergonomics of the thermal environment - Analytical determination and interpretation of thermal comfort (outer clothing surface of seated persons about 30-33 degC)."),
    ("municipal-park-chair-catalogue", "Municipal street-furniture catalogue data for movable steel park armchairs of the city-park type (low lounge armchair, stackable; Ø25 mm tube side frames, perforated sheet seat and back on flat-bar carriers, seat height 0.40 m, armrest 0.61 m, overall 0.80 x 0.62 x 0.60 m, 11-14 kg, RAL 6005 moss green); catalogue-typical values, no single product."),
]


def materials():
    """🧱 Weathered steels of the chair; perforated sheet smeared over its 40 % open area."""
    return [
        {"id": "painted-steel", "title": T("Steel, weathered moss-green coating", "Stahl, verwitterte moosgrüne Beschichtung"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "levinson-2005"},
        {"id": "perforated-sheet", "title": T("Perforated steel sheet 2 mm Rv 8-12, coated (smeared over 40 % open area)", "Lochblech 2 mm Rv 8-12, beschichtet (über 40 % offene Fläche verschmiert)"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": round(RHO_STEEL * (1 - OPEN_AREA)), "conductivityWmK": 30, "specificHeatJKgK": 460, "elasticModulusPa": 120e9, "source": "authored-estimate", "reference": "din-24041-2002"},
        {"id": "rusted-steel", "title": T("Rusted bare steel (feet, weld seams)", "Verrosteter blanker Stahl (Füße, Schweißnähte)"), "emissivity": 0.82, "solarAbsorptivity": 0.78, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "source": "literature", "reference": "incropera-2018"},
        {"id": "bare-steel", "title": T("Hand-worn bare steel (armrest grips)", "Von Händen blank gegriffener Stahl (Armlehnen)"), "emissivity": 0.3, "solarAbsorptivity": 0.55, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "sticker-paper", "title": T("Sticker residue (paper and adhesive)", "Aufkleberreste (Papier und Kleber)"), "emissivity": 0.9, "solarAbsorptivity": 0.4, "densityKgM3": 900, "conductivityWmK": 0.15, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "iso-10456-2007"},
    ]


def mass_breakdown():
    """⚖️ Masses per bone from the true sections and densities (tube Ø25 x 2, rails Ø20 x 2 and Ø16 x 2, flat bar 30 x 8, sheet 2 mm with 40 % open area), with their world centres."""
    rows = {k: [] for k in ("leg-rear", "leg-front", "armrest", "seat", "back")}
    for tag, side in SIDES:
        path, (b0, b1, f0, f1) = side_path(side, 24)
        rear = [(FOOT_REAR[0], FOOT_REAR[1], side * zleg(0)), path[b0]]
        front = [path[f1], (FOOT_FRONT[0], FOOT_FRONT[1], side * zleg(0))]
        arm = path[b0:f1 + 1]
        foot_m = tube_mass(FOOT_H) + math.pi * FLANGE_R ** 2 * FLANGE_T * RHO_STEEL
        rows["leg-rear"].append((tube_mass(path_length(rear)), mid(rear)))
        rows["leg-rear"].append((math.pi * FLANGE_R ** 2 * FLANGE_T * RHO_STEEL, rear[0]))
        rows["leg-front"].append((tube_mass(path_length(front)), mid(front)))
        rows["leg-front"].append((math.pi * FLANGE_R ** 2 * FLANGE_T * RHO_STEEL, front[1]))
        rows["armrest"].append((tube_mass(path_length(arm)), centroid(arm)))
        zc = side * (0.2685 - CARRIER_W / 2)
        flat = CARRIER_H * CARRIER_W * RHO_STEEL
        u0, u1 = CARRIER_SEAT_U
        rows["seat"].append((flat * (u1 - u0), to_world("seat", ((u0 + u1) / 2, -SHEET_T - CARRIER_H / 2, zc))))
        u0, u1 = CARRIER_BACK_U
        rows["back"].append((flat * (u1 - u0) * (CARRIER_H + CARRIER_BACK_TOP_H) / (2 * CARRIER_H), to_world("back", ((u0 + u1) / 2 - 0.02, SHEET_T + CARRIER_H * 0.45, zc))))
    fy = 0.373
    fx = FOOT_FRONT[0] + (CORNER_FRONT[0] - FOOT_FRONT[0]) * fy / CORNER_FRONT[1]
    rows["leg-front"].append((tube_mass(2 * zleg(fy), 0.020, 0.002), (fx, fy, 0)))
    sx = FOOT_REAR[0] + (CORNER_REAR[0] - FOOT_REAR[0]) * STRETCH_Y / CORNER_REAR[1]
    rows["leg-rear"].append((tube_mass(2 * zleg(STRETCH_Y), 0.016, 0.002), (sx, STRETCH_Y, 0)))
    sheet = SHEET_T * RHO_STEEL * (1 - OPEN_AREA)
    hem = math.pi * 2 * HEM_R * SHEET_T * RHO_STEEL
    seat_len, back_len = BONES["seat"]["lengthM"], BONES["back"]["lengthM"]
    rows["seat"].append((sheet * seat_len * 2 * SEAT_HALF, to_world("seat", (seat_len / 2, -SHEET_T / 2, 0))))
    rows["seat"].append((hem * 2 * SEAT_HALF, to_world("seat", (seat_len, -HEM_R, 0))))
    rows["back"].append((sheet * back_len * 2 * SEAT_HALF, to_world("back", (back_len / 2, SHEET_T / 2, 0))))
    rows["back"].append((hem * 2 * SEAT_HALF, to_world("back", (back_len, 0.007, 0))))
    out = {}
    for k, items in rows.items():
        m = sum(i[0] for i in items)
        c = tuple(sum(i[0] * i[1][j] for i in items) / m for j in range(3))
        out[k] = (m, c)
    return out


def mid(pts):
    """🎯 Middle of a straight run."""
    return tuple((a + b) / 2 for a, b in zip(pts[0], pts[-1]))


def centroid(pts):
    """🎯 Length-weighted centroid of a polyline."""
    tot, acc = 0.0, [0.0, 0.0, 0.0]
    for i in range(len(pts) - 1):
        l = norm(sub(pts[i + 1], pts[i]))
        tot += l
        for j in range(3):
            acc[j] += l * (pts[i][j] + pts[i + 1][j]) / 2
    return tuple(a / tot for a in acc)


MASS = mass_breakdown()
MASS_KG = sum(m for m, _ in MASS.values())
COM = tuple(sum(m * c[j] for m, c in MASS.values()) / MASS_KG for j in range(3))
WEIGHT_N = MASS_KG * 9.80665
LIFT_X = COM[0] - CORNER_REAR[0]


def areas():
    """📐 Outer areas of the four thermal regions of the 1:50 model (tube mantles, flat bars, rails, two faces of each sheet with edges, hems, feet)."""
    frame = 0.0
    for tag, side in SIDES:
        path, (b0, b1, f0, f1) = side_path(side, 24)
        rear_top = FOOT_H
        frame += 2 * math.pi * T_R * (path_length([(FOOT_REAR[0], FOOT_REAR[1], 0), path[b0]]) - rear_top)
        frame += 2 * math.pi * T_R * (path_length([path[f1], (FOOT_FRONT[0], FOOT_FRONT[1], 0)]) - FOOT_H)
        frame += 2 * math.pi * T_R * path_length(path[b0:f1 + 1])
        u0, u1 = CARRIER_SEAT_U
        frame += 2 * (CARRIER_H + CARRIER_W) * (u1 - u0) + 2 * CARRIER_H * CARRIER_W
        u0, u1 = CARRIER_BACK_U
        hm = (CARRIER_H + CARRIER_BACK_TOP_H) / 2
        frame += 2 * hm * (u1 - u0) + 2 * CARRIER_W * (u1 - u0) + CARRIER_W * (CARRIER_H + CARRIER_BACK_TOP_H)
    frame += 2 * math.pi * RAIL_R * (2 * zleg(0.373) - 2 * T_R) + 2 * math.pi * STRETCH_R * (2 * zleg(STRETCH_Y) - 2 * T_R)
    seat = 2 * BONES["seat"]["lengthM"] * 2 * SEAT_HALF + 2 * math.pi * HEM_R * 2 * SEAT_HALF
    back = 2 * BONES["back"]["lengthM"] * 2 * SEAT_HALF + 2 * math.pi * HEM_R * 2 * SEAT_HALF
    foot = 4 * (math.pi * FLANGE_R ** 2 + 2 * math.pi * FLANGE_R * FLANGE_T + math.pi * (FLANGE_R ** 2 - T_R ** 2) + 2 * math.pi * T_R * (FOOT_H - FLANGE_T) + math.pi * T_R ** 2)
    return {"frame-steel": frame, "seat-sheet": seat, "back-sheet": back, "feet-rust": foot}


AREA = areas()

TUBE_PAIR = {"areaM2": r6(2 * math.pi / 4 * (T_SEC[0] ** 2 - (T_SEC[0] - 2 * T_SEC[1]) ** 2)), "secondMomentM4": r6(2 * math.pi / 64 * (T_SEC[0] ** 4 - (T_SEC[0] - 2 * T_SEC[1]) ** 4) * 1e6) / 1e6, "extremeFibreM": T_R, "material": "painted-steel", "title": T("Two tubes Ø25 x 2 mm (both side frames)", "Zwei Rohre Ø25 x 2 mm (beide Seitenrahmen)")}
FLAT_PAIR = {"areaM2": r6(2 * CARRIER_H * CARRIER_W), "secondMomentM4": r6(2 * CARRIER_W * CARRIER_H ** 3 / 12 * 1e6) / 1e6, "extremeFibreM": CARRIER_H / 2, "material": "painted-steel", "title": T("Two flat-bar carriers 30 x 8 mm on edge", "Zwei Flachstahlträger 30 x 8 mm hochkant")}
BONE_TEXT = {
    "ground": (T("Ground frame", "Bezugsrahmen"), None),
    "leg-rear": (T("Rear legs with feet and rear stretcher", "Hinterbeine mit Füßen und hinterer Strebe"), TUBE_PAIR),
    "leg-front": (T("Front legs with feet and front seat rail", "Vorderbeine mit Füßen und vorderer Sitztraverse"), TUBE_PAIR),
    "armrest": (T("Armrests with both bends", "Armlehnen mit beiden Biegungen"), TUBE_PAIR),
    "seat": (T("Seat: perforated sheet on flat-bar carriers", "Sitz: Lochblech auf Flachstahlträgern"), FLAT_PAIR),
    "back": (T("Back: perforated sheet on flat-bar carriers", "Rücken: Lochblech auf Flachstahlträgern"), FLAT_PAIR),
}


def rig():
    """🦴 Six bones: the ground root and five fixed members of the welded chair, each with its mass from the true sections and its member section."""
    bones = []
    for k in ("ground", "leg-rear", "leg-front", "armrest", "seat", "back"):
        b = BONES[k]
        title, section = BONE_TEXT[k]
        row = {"id": k, "parent": None if k == "ground" else "ground", "title": title, "x": r6(b["x"]), "y": r6(b["y"]), "angleDeg": r6(b["angleDeg"]), "lengthM": r6(b["lengthM"]), "joint": {"type": "fixed"}}
        if k in MASS:
            m, c = MASS[k]
            u, v = to_local(k, (c[0], c[1]))
            row["mass"] = {"kg": round(m, 3), "comX": r6(u), "comY": r6(v), "distribution": "point", "source": "literature", "reference": "din-en-10305-3-2016"}
            row["section"] = section
        bones.append(row)
    return {"kind": "structural", "root": "ground", "bones": bones}


SIDE_EN = {1: ("left", "links"), -1: ("right", "rechts")}


def anchors():
    """🪝 Four foot contacts on the gravel, two lifting grips at the armrests over the centre of mass, and the grip at the back top where the chair is dragged."""
    out = []
    for bone_id, name, de in (("leg-rear", "rear", "hinten"), ("leg-front", "front", "vorn")):
        for tag, side in SIDES:
            en_s, de_s = SIDE_EN[side]
            out.append({"id": f"contact-{name}-{en_s}", "bone": bone_id, "x": 0, "y": 0, "z": r6(side * zleg(0)), "kind": "contact", "title": T(f"Rusted foot {name} {en_s} on the gravel", f"Verrosteter Fuß {de} {de_s} auf dem Kies"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.03, "footprintWidthM": 0.03, "frictionCoefficient": 0.5})
    out.append({"id": "lift-left", "bone": "armrest", "x": r6(LIFT_X), "y": T_R, "z": r6(zleg(0.6)), "kind": "support", "title": T("Left hand of the depot worker on the armrest", "Linke Hand des Bauhofmitarbeiters an der Armlehne"), "support": "pin", "footprintLengthM": 0.09, "footprintWidthM": 0.03})
    out.append({"id": "lift-right", "bone": "armrest", "x": r6(LIFT_X), "y": T_R, "z": r6(-zleg(0.6)), "kind": "support", "title": T("Right hand of the depot worker on the armrest", "Rechte Hand des Bauhofmitarbeiters an der Armlehne"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.03})
    out.append({"id": "drag-grip", "bone": "back", "x": r6(BONES["back"]["lengthM"] + HEM_R), "y": 0.007, "z": 0, "kind": "load", "title": T("Hands on the rolled back top when the chair is dragged", "Hände an der gerollten Rückenoberkante beim Ziehen")})
    return out


def flake_strokes():
    """🍂 Flaking paint and rust streaks drawn as texture strokes at 1:50 in the side elevation and plan."""
    out = []
    for i, (x0, w) in enumerate(((0.12, 0.05), (0.30, 0.035))):
        d = f"M {x0:.3f} 0.004 L {x0 + w * 0.3:.3f} 0.009 L {x0 + w * 0.6:.3f} 0.003 L {x0 + w:.3f} 0.007"
        out.append(("elevation", {"id": f"paint-flake-{i}", "bone": "armrest", "role": "texture", "filled": False, "tone": 0.75, "kind": "path", "d": d, "region": "frame-steel"}))
    u0 = CARRIER_SEAT_U[0]
    out.append(("elevation", {"id": "rust-streak-seat", "bone": "seat", "role": "texture", "filled": False, "tone": 0.15, "kind": "path", "d": f"M {u0 + 0.012:.4f} -0.006 L {u0 + 0.016:.4f} -0.018 L {u0 + 0.013:.4f} -0.029", "region": "frame-steel"}))
    for tag, side in SIDES:
        z = side * zleg(0.6)
        out.append(("plan", {"id": f"paint-flake-plan-{tag}", "bone": "armrest", "role": "texture", "filled": False, "tone": 0.75, "kind": "path", "d": f"M 0.200 {z - 0.004:.4f} L 0.225 {z + 0.004:.4f} L 0.250 {z - 0.003:.4f}", "region": "frame-steel"}))
    out.append(("front", {"id": "bird-dropping", "bone": "back", "role": "texture", "filled": False, "tone": 0.85, "kind": "path", "d": "M 0.140 0.700 L 0.150 0.688 L 0.146 0.676", "region": "back-sheet"}))
    return out


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
        200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def shapes(scale, view):
    """✏️ The drawn shapes of one view of one scale, from the same parts as the solids."""
    out = []
    z = 1.0
    for part in parts_of(scale):
        if view == "elevation" and part.get("side", 0) < 0:
            continue
        if part.get("role") == "detail" and scale != 50:
            continue
        kind, data = outline(part, view)
        s = {"id": part["id"], "bone": part["bone"], "role": part.get("role", "body"), "z": round(z, 2), "filled": True, "tone": part["tone"]}
        if kind == "poly":
            s.update({"kind": "poly", "points": [[r6(a), r6(b)] for a, b in data], "closed": True})
        else:
            (cx, cy), (rx, ry) = data
            s.update({"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry)})
        if part.get("regions2d"):
            s["regions"] = part["regions2d"]
        elif part.get("region"):
            s["region"] = part["region"]
        s["material"] = part["material"]
        if part.get("hatch") == view and scale in (50, 100):
            s["hatch"] = "perforation"
        out.append(s)
        z += 0.1
    if scale == 50:
        for v, s in flake_strokes():
            if v == view:
                out.append({**s, "z": round(z, 2)})
                z += 0.1
    return out

REP_DECISIONS = {
    50: {
        "silhouette": T("Side elevation 12.4 x 16.0 mm: the bent Ø25 side frame as one closed tube outline (raked rear leg, 60 mm bends, level armrest, raked front leg) on its rusted flanged feet, the perforated seat sheet edge-on on its 30 mm flat-bar carrier with the rolled front hem over the Ø20 front rail, the reclined back sheet with its rolled top hem and carrier crossing the rear bend, the Ø16 rear stretcher end-on. Front view: splayed legs rising to the round armrest ends, the seat strip with hem, the back sheet behind it with its perforation, the carriers just inside the frames. Plan: the two frames with the bends running out to the feet, the seat sheet with its perforation and hem, the back sheet foreshortened behind it.", "Seitenansicht 12,4 x 16,0 mm: der gebogene Seitenrahmen Ø25 als ein geschlossener Rohrumriss (ausgestelltes Hinterbein, 60-mm-Bögen, waagrechte Armlehne, ausgestelltes Vorderbein) auf den verrosteten Flanschfüßen, das gelochte Sitzblech von der Kante auf seinem 30-mm-Flachstahlträger mit gerolltem Vordersaum über der vorderen Traverse Ø20, das geneigte Rückenblech mit gerolltem Obersaum und dem Träger, der den hinteren Bogen kreuzt, die hintere Strebe Ø16 von vorn. Vorderansicht: gespreizte Beine bis zu den runden Armlehnenenden, der Sitzstreifen mit Saum, dahinter das gelochte Rückenblech, die Träger knapp innerhalb der Rahmen. Grundriss: die zwei Rahmen mit den zu den Füßen auslaufenden Bögen, das gelochte Sitzblech mit Saum, dahinter verkürzt das Rückenblech."),
        "retained": T("True tube, rail and flat-bar sections, the 15 mm leg splay that lets the chairs nest, seat height 0.40 m at the hem, armrest 0.61 m, back top 0.80 m, the flanged feet, and the traces of use: rust on the lowest 65 mm of every leg, bare hand-worn steel on both armrest fronts, rust bloom at the rear carrier welds, flaking paint, sticker residue and a dropping on the back.", "Echte Rohr-, Traversen- und Flachstahlquerschnitte, die 15-mm-Spreizung der Beine, mit der die Stühle ineinander stapeln, Sitzhöhe 0,40 m am Saum, Armlehne 0,61 m, Rückenoberkante 0,80 m, die Flanschfüße und die Gebrauchsspuren: Rost an den untersten 65 mm jedes Beins, blank gegriffener Stahl vorn an beiden Armlehnen, Rostblüte an den hinteren Trägerschweißungen, abblätternde Farbe, Aufkleberreste und ein Vogelklecks auf dem Rücken."),
        "merged": T("Near and far side frames coincide in the side elevation and are drawn once.", "Nahe und ferne Seitenrahmen fallen in der Seitenansicht zusammen und sind einmal gezeichnet."),
        "omitted": T("The 8 mm perforation holes are not drawn one by one (0.16 mm on paper at 12 mm pitch); a dot hatch stands for them. Weld beads and the screw heads of the sheet fixings are below the scale.", "Die 8-mm-Löcher werden nicht einzeln gezeichnet (0,16 mm auf Papier bei 12 mm Teilung); ein Punktraster steht für sie. Schweißraupen und Schraubenköpfe der Blechbefestigung liegen unter dem Maßstab."),
        "thinElements": T("The 2 mm sheets keep their thickness and read through the body pen of 0.35 mm; the 8 mm carriers are drawn at true width in front and plan.", "Die 2-mm-Bleche behalten ihre Dicke und lesen sich über den Körperstift 0,35 mm; die 8-mm-Träger sind in Vorderansicht und Grundriss in echter Breite gezeichnet."),
        "hatching": T("Dot hatch 0.5 mm for the perforated seat (plan) and back (front view).", "Punktraster 0,5 mm für den gelochten Sitz (Grundriss) und Rücken (Vorderansicht)."),
        "styles": T("Moss-green steel mid-dark, sheets a lighter mid tone, hems between, rusted feet and weld blooms near black, worn grips and the sticker light.", "Moosgrüner Stahl mitteldunkel, Bleche hellerer Mittelton, Säume dazwischen, verrostete Füße und Schweißblüten fast schwarz, blanke Griffe und Aufkleber hell."),
        "overlays": T("Tubes, rails and carriers frame-steel; seat sheet and hem seat-sheet; back sheet and hem back-sheet; feet feet-rust.", "Rohre, Traversen und Träger frame-steel; Sitzblech und Saum seat-sheet; Rückenblech und Saum back-sheet; Füße feet-rust."),
    },
    100: {
        "silhouette": T("Side elevation 6.2 x 8.0 mm: the bent side frame with coarser bends on plain rusted foot sleeves, the seat sheet on its carrier with the front hem over the front rail, the reclined back sheet with hem and carrier, the rear stretcher end-on. Front view and plan as at 1:50 without the wear patches.", "Seitenansicht 6,2 x 8,0 mm: der gebogene Seitenrahmen mit gröberen Bögen auf glatten verrosteten Fußhülsen, das Sitzblech auf seinem Träger mit Vordersaum über der vorderen Traverse, das geneigte Rückenblech mit Saum und Träger, die hintere Strebe von vorn. Vorderansicht und Grundriss wie 1:50 ohne Abnutzungsflecken."),
        "retained": T("Overall 0.80 x 0.62 x 0.61 m, the splay, seat 0.40 m, armrest 0.61 m, tube and rail sections, the rusted feet as their own region.", "Gesamtmaß 0,80 x 0,62 x 0,61 m, die Spreizung, Sitz 0,40 m, Armlehne 0,61 m, Rohr- und Traversenquerschnitte, die verrosteten Füße als eigene Region."),
        "merged": T("The foot flanges go into the foot sleeves; each bend is five chords.", "Die Fußflansche gehen in den Fußhülsen auf; jeder Bogen besteht aus fünf Sehnen."),
        "omitted": T("Wear patches, weld blooms, flakes and the sticker (below 15 mm or texture only at 1:50).", "Abnutzungsflecken, Schweißblüten, Abplatzungen und Aufkleber (unter 15 mm oder nur Textur bei 1:50)."),
        "thinElements": T("Carriers widened to 15 mm (the 8 mm flat bar), sheets 2 mm read through the 0.25 mm pen.", "Träger auf 15 mm verbreitert (der 8-mm-Flachstahl), Bleche 2 mm über den 0,25-mm-Stift lesbar."),
        "hatching": T("Dot hatch 0.5 mm for the perforated seat and back.", "Punktraster 0,5 mm für gelochten Sitz und Rücken."),
        "styles": T("Steel mid-dark, sheets lighter, feet near black.", "Stahl mitteldunkel, Bleche heller, Füße fast schwarz."),
        "overlays": T("As at 1:50.", "Wie bei 1:50."),
    },
    200: {
        "silhouette": T("Side elevation 3.1 x 4.0 mm: the side frame as one 25 mm tube outline from the ground over both bends, a 30 mm seat slab and a 30 mm back slab, front rail and rear stretcher end-on. Front view: legs, armrest ends, seat and back slabs. Plan: frames, seat and back slabs.", "Seitenansicht 3,1 x 4,0 mm: der Seitenrahmen als ein 25-mm-Rohrumriss vom Boden über beide Bögen, eine 30-mm-Sitzplatte und eine 30-mm-Rückenplatte, vordere Traverse und hintere Strebe von vorn. Vorderansicht: Beine, Armlehnenenden, Sitz- und Rückenplatte. Grundriss: Rahmen, Sitz- und Rückenplatte."),
        "retained": T("Overall extents, seat and armrest heights, the reclined back, the bends, the splay of the legs.", "Gesamtmaße, Sitz- und Armlehnenhöhe, die geneigte Lehne, die Bögen, die Spreizung der Beine."),
        "merged": T("Sheet, hem and carriers become one 30 mm slab each for seat and back; the rusted feet are part of the leg outline (regions frame-steel and feet-rust).", "Blech, Saum und Träger werden je eine 30-mm-Platte für Sitz und Rücken; die verrosteten Füße gehören zum Beinumriss (Regionen frame-steel und feet-rust)."),
        "omitted": T("Perforation, hems as separate parts, all traces of use.", "Lochung, Säume als eigene Teile, alle Gebrauchsspuren."),
        "thinElements": T("Tubes 25 mm and rails 16-20 mm are true-width strips (0.13 mm and 0.08-0.1 mm plus the 0.18 mm pen).", "Rohre 25 mm und Traversen 16-20 mm sind Streifen in echter Breite (0,13 mm und 0,08-0,1 mm plus 0,18-mm-Stift)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Steel mid-dark, slabs lighter.", "Stahl mitteldunkel, Platten heller."),
        "overlays": T("Legs frame-steel with feet-rust merged; slabs seat-sheet and back-sheet.", "Beine frame-steel mit feet-rust zusammengefasst; Platten seat-sheet und back-sheet."),
    },
    500: {
        "silhouette": T("Chair sign 1.2 x 1.6 mm: the side frame strip (legs and armrest), a 75 mm seat slab and a 75 mm back slab. Front view: the two leg strips with armrest ends, seat and back slabs. Plan: the two frame strips, seat and back slabs.", "Stuhlzeichen 1,2 x 1,6 mm: der Seitenrahmenstreifen (Beine und Armlehne), eine 75-mm-Sitzplatte und eine 75-mm-Rückenplatte. Vorderansicht: die zwei Beinstreifen mit Armlehnenenden, Sitz- und Rückenplatte. Grundriss: die zwei Rahmenstreifen, Sitz- und Rückenplatte."),
        "retained": T("Overall extents, seat and armrest heights, the reclined back.", "Gesamtmaße, Sitz- und Armlehnenhöhe, die geneigte Lehne."),
        "merged": T("Seat and back into 75 mm slabs; feet into the legs.", "Sitz und Rücken zu 75-mm-Platten; Füße in die Beine."),
        "omitted": T("Front rail and rear stretcher (16-20 mm, 0.03-0.04 mm on paper), hems, perforation, traces of use.", "Vordere Traverse und hintere Strebe (16-20 mm, 0,03-0,04 mm auf Papier), Säume, Lochung, Gebrauchsspuren."),
        "thinElements": T("Tubes are true 25 mm strips read through the 0.18 mm pen; slabs thickened to the 75 mm feature.", "Rohre sind echte 25-mm-Streifen, über den 0,18-mm-Stift lesbar; Platten auf das 75-mm-Mindestmaß verstärkt."),
        "hatching": T("None.", "Keine."),
        "styles": T("One pen of 0.18 mm; steel mid-dark, slabs lighter.", "Ein Stift 0,18 mm; Stahl mitteldunkel, Platten heller."),
        "overlays": T("Legs frame-steel with feet-rust merged; slabs seat-sheet and back-sheet.", "Beine frame-steel mit feet-rust zusammengefasst; Platten seat-sheet und back-sheet."),
    },
}


def representation(scale):
    """🗺️ The authored representation of one preset scale."""
    rep = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": RANGES[scale][0], "maxDenominator": RANGES[scale][1]}, "pens": PENS[scale],
           "hatches": [{"id": "perforation", "title": T("Perforated sheet Rv 8-12", "Lochblech Rv 8-12"), "pattern": "dots", "angleDeg": 30, "spacingMm": 0.5, "penMm": 0.1}] if scale in (50, 100) else [],
           "geometry": {v: shapes(scale, v) for v in ("elevation", "front", "plan")}, "decisions": REP_DECISIONS[scale]}
    return rep


def ell(id_, bone_id, world, r, member, part, material="painted-steel", tone=0.25):
    """🔵 A compact structural marker (joint or mass) at a world point on a bone."""
    loc = to_local(bone_id, world)
    return {"id": id_, "kind": "ellipsoid", "bone": bone_id, "material": material, "role": "detail", "tone": tone, "radii": [r, r, r], "transform": {"translation": [r6(c) for c in loc]}, "member": member, "part": part}


def joints(scale):
    """🔩 Joint solids at the bends and welds; fewer at the coarse scales."""
    r = {50: 0.012, 100: 0.016, 200: 0.016, 500: 0.0375}[scale]
    out = []
    for tag, side in SIDES:
        path, (b0, b1, f0, f1) = side_path(side, 8)
        zc = side * (0.2685 - CARRIER_W / 2)
        out.append(ell(f"joint-bend-rear-{tag}", "armrest", path[(b0 + b1) // 2], r, "armrest", "joint"))
        out.append(ell(f"joint-bend-front-{tag}", "armrest", path[(f0 + f1) // 2], r, "armrest", "joint"))
        out.append(ell(f"joint-seat-rear-{tag}", "seat", (0.058, 0.35, side * 0.2685), r, "seat", "joint"))
        out.append(ell(f"joint-seat-front-{tag}", "seat", to_world("seat", (CARRIER_SEAT_U[1] - 0.004, -0.017, zc)), r, "seat", "joint"))
        if scale != 500:
            out.append(ell(f"joint-back-arm-{tag}", "back", to_world("back", (0.15, 0.017, zc)), r, "back", "joint"))
            out.append(ell(f"joint-back-seat-{tag}", "back", to_world("back", (-0.06, 0.017, zc)), r, "back", "joint"))
    return out


def masses(scale):
    """⚫ Mass markers at the centre of mass of every member bone."""
    r = {50: 0.012, 100: 0.012, 200: 0.016, 500: 0.0375}[scale]
    out = []
    for k, (m, c) in MASS.items():
        if k in ("leg-rear", "leg-front", "armrest"):
            out += [ell(f"mass-{k}-{tag}", k, (c[0], c[1], side * zleg(c[1])), r, k, "mass", tone=0.4) for tag, side in SIDES]
        else:
            out.append(ell(f"mass-{k}", k, (c[0], c[1], 0.0), r, k, "mass", tone=0.4))
    return out


def thermal_extra(scale):
    """🌡️ Thermal-only foot solids where the body merges the rusted feet into the slender legs: Ø30 foot blocks at 1:200, Ø75 rust discs at 1:500."""
    out = []
    if scale not in (200, 500):
        return out
    for bone_id, foot, corner in (("leg-rear", FOOT_REAR, CORNER_REAR), ("leg-front", FOOT_FRONT, CORNER_FRONT)):
        for tag, side in SIDES:
            p0 = to_local(bone_id, (foot[0], foot[1], side * zleg(0)))
            if scale == 200:
                top = (foot[0] + (corner[0] - foot[0]) * 0.07 / corner[1], 0.07, side * zleg(0.07))
                p1 = to_local(bone_id, top)
                axis = [r6(c) for c in unit(sub(p1, p0))]
                out.append({"id": f"th-foot-{bone_id}-{tag}", "kind": "revolve", "bone": bone_id, "material": "rusted-steel", "role": "body", "tone": 0.12, "profile": [[0, 0], [0, 0.015], [0.07, 0.015], [0.07, 0]], "axis": axis, "origin": [r6(c) for c in p0], "region": "feet-rust"})
            else:
                up = [r6(c) for c in dir_local(bone_id, (0.0, 1.0, 0.0))]
                out.append({"id": f"th-foot-{bone_id}-{tag}", "kind": "revolve", "bone": bone_id, "material": "rusted-steel", "role": "body", "tone": 0.12, "profile": [[0, 0], [0, 0.0375], [0.003, 0.0375], [0.003, 0]], "axis": up, "origin": [r6(c) for c in p0], "sheet": True, "region": "feet-rust"})
    return out


VOL_DECISIONS = {
    50: [T("1:50 is the full construction of the real chair: per side one Ø25 x 2 mm steel tube bent with a 60 mm centre-line radius from the raked rear leg into the level armrest and down into the raked front leg, the legs splayed 15 mm so the chairs nest when stacked; each leg ends in a rusted 65 mm foot with a welded Ø30 x 4 mm flange (the polyamide glides are long gone); a Ø20 front seat rail and a Ø16 rear stretcher are welded between the legs; seat and back are 2 mm perforated sheets (declared sheets, perforation as texture in 2D) with Ø16 rolled hems, each on two 30 x 8 mm flat-bar carriers welded to the frames; the back carriers cross the rear bends and are welded there.", "1:50 ist die volle Konstruktion des echten Stuhls: je Seite ein Stahlrohr Ø25 x 2 mm, mit 60 mm Mittellinienradius vom ausgestellten Hinterbein in die waagrechte Armlehne und hinunter ins ausgestellte Vorderbein gebogen, die Beine 15 mm gespreizt, damit die Stühle beim Stapeln ineinander passen; jedes Bein endet in einem verrosteten 65-mm-Fuß mit angeschweißtem Flansch Ø30 x 4 mm (die Polyamidgleiter sind längst verloren); eine vordere Sitztraverse Ø20 und eine hintere Strebe Ø16 sind zwischen die Beine geschweißt; Sitz und Rücken sind 2-mm-Lochbleche (deklarierte Bleche, Lochung als Textur in 2D) mit gerollten Säumen Ø16 auf je zwei Flachstahlträgern 30 x 8 mm, die an die Rahmen geschweißt sind; die Rückenträger kreuzen die hinteren Bögen und sind dort verschweißt."),
         T("Traces of use as texture solids: bare hand-worn steel sleeves on both armrest fronts, rust blooms at the rear seat-carrier welds and a sticker residue on the back; rust on the feet is geometry (its own solids and region). Thermal references the body by region; structure references the tubes, carriers and sheets as members and adds joints at the bends and welds and mass markers at the centres of the members (one per tube for the side-frame bones); the chair has no permanent supports: feet on the gravel and the worker's hands are pose contacts.", "Gebrauchsspuren als Textur-Körper: blank gegriffene Stahlhülsen vorn an beiden Armlehnen, Rostblüten an den hinteren Sitzträgerschweißungen und ein Aufkleberrest am Rücken; der Rost an den Füßen ist Geometrie (eigene Körper und Region). Die Thermik verweist regionsweise auf den Körper; das Tragwerk verweist auf Rohre, Träger und Bleche als Glieder und ergänzt Gelenke an Bögen und Schweißungen und Massenmarken in den Gliedschwerpunkten (je Rohr für die Seitenrahmenknochen); der Stuhl hat keine ständigen Auflager: Füße auf dem Kies und die Hände des Mitarbeiters sind Posenkontakte.")],
    100: [T("1:100 keeps every structural part at its true section (tubes Ø25, rails Ø20 and Ø16, hems Ø16 are at least the 15 mm feature) but generalises: the bends have five chords, the feet are plain rusted Ø25 sleeves without flange, the 8 mm carriers are widened inwards to 15 mm, the sheets stay declared 2 mm sheets; all texture solids of 1:50 are dropped.", "1:100 behält jedes tragende Teil in echtem Querschnitt (Rohre Ø25, Traversen Ø20 und Ø16, Säume Ø16 erreichen das 15-mm-Mindestmaß), generalisiert aber: die Bögen haben fünf Sehnen, die Füße sind glatte verrostete Hülsen Ø25 ohne Flansch, die 8-mm-Träger sind nach innen auf 15 mm verbreitert, die Bleche bleiben deklarierte 2-mm-Bleche; alle Textur-Körper von 1:50 entfallen."),
          T("Thermal and structure as at 1:50 on this body, with 32 mm joints and 24 mm mass markers.", "Thermik und Tragwerk wie bei 1:50 auf diesem Körper, mit 32-mm-Gelenken und 24-mm-Massenmarken.")],
    200: [T("1:200 draws the frame as true Ø25 tubes from the ground over three-chord bends (the 2D draws them at their true 25 mm width, so the feature rule follows the drawing), keeps the front rail and rear stretcher as slender rods and merges sheet, hem and carriers into one 30 mm slab for seat and back.", "1:200 zeichnet den Rahmen als echte Rohre Ø25 vom Boden über dreisehnige Bögen (die 2D-Zeichnung zeigt sie in echter Breite 25 mm, das Mindestmaß folgt der Zeichnung), behält vordere Traverse und hintere Strebe als schlanke Stäbe und fasst Blech, Saum und Träger zu je einer 30-mm-Platte für Sitz und Rücken zusammen."),
          T("The rusted feet are thermal-only Ø30 x 70 mm foot blocks around the leg ends (the body merges them into the legs); structure references tubes and slabs, 32 mm joints and masses.", "Die verrosteten Füße sind rein thermische Fußklötze Ø30 x 70 mm um die Beinenden (der Körper fasst sie in die Beine); das Tragwerk verweist auf Rohre und Platten, 32-mm-Gelenke und Massen.")],
    500: [T("1:500 is the chair sign: true Ø25 tubes over two-chord bends, drawn at their true width, 75 mm seat and back slabs; front rail and rear stretcher are omitted (0.03-0.04 mm on paper).", "1:500 ist das Stuhlzeichen: echte Rohre Ø25 über zweisehnige Bögen, in echter Breite gezeichnet, 75-mm-Sitz- und Rückenplatte; vordere Traverse und hintere Strebe entfallen (0,03-0,04 mm auf Papier)."),
          T("The rusted feet are thermal-only Ø75 x 3 mm rust discs under the legs (declared sheets, the region is below the size this scale shows); structure: the tubes and slabs, 75 mm joints at the bends and seat welds, 75 mm masses.", "Die verrosteten Füße sind rein thermische Rostscheiben Ø75 x 3 mm unter den Beinen (deklarierte Bleche, die Region liegt unter dem, was dieser Maßstab zeigt); Tragwerk: Rohre und Platten, 75-mm-Gelenke an Bögen und Sitzschweißungen, 75-mm-Massen.")],
}


def volume(scale):
    """🧊 The authored volume of one scale: body from the parts, thermal by region, structure as members, joints, supports and masses."""
    parts = parts_of(scale)
    body = [solid(p) for p in parts]
    thermal = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in parts if p.get("region") and not p.get("texture")] + thermal_extra(scale)
    members = [{"id": f"m-{p['id']}", "of": p["id"], "member": p["bone"], "part": "member"} for p in parts if not p.get("texture")]
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": members + joints(scale) + masses(scale), "decisions": VOL_DECISIONS[scale]}


REGIONS = ("frame-steel", "seat-sheet", "back-sheet", "feet-rust")


def binding(frame, seat, back, feet):
    """🌬️ A thermal binding of the four regions from (exposed, sunlit, contact fraction, contact °C) tuples."""
    out = {}
    for rid, (e, s, cf, ct) in zip(REGIONS, (frame, seat, back, feet)):
        row = {"exposedFraction": e, "sunlitFraction": s}
        if cf:
            row["contact"] = {"fraction": cf, "tempC": ct}
        out[rid] = row
    return {"regions": out, "emitters": {}}


FEET = ["contact-rear-left", "contact-rear-right", "contact-front-left", "contact-front-right"]
REAR = FEET[:2]
TILT = 20.0
DRAG_X = -0.9


def drag_load():
    """🧮 Hand force at the back top that holds the chair tilted by TILT about its rear feet while it slides backwards on gravel (friction 0.5): moment and horizontal balance about the rear feet."""
    th = math.radians(TILT)
    grip = to_world("back", (BONES["back"]["lengthM"] + HEM_R, 0.007))

    def rot(p):
        dx, dy = p[0] - FOOT_REAR[0], p[1] - FOOT_REAR[1]
        return (dx * math.cos(th) - dy * math.sin(th), dx * math.sin(th) + dy * math.cos(th))

    xc, _ = rot(COM)
    rx, ry = rot(grip)
    mu = 0.4
    fy = WEIGHT_N * (xc - ry * mu) / (rx - ry * mu)
    fx = -mu * (WEIGHT_N - fy)
    return fx, fy


DRAG_FX, DRAG_FY = drag_load()
SEAT_N = 0.8 * 75 * 9.80665
BACK_V = (-math.sin(math.radians(BONES["back"]["angleDeg"])), math.cos(math.radians(BONES["back"]["angleDeg"])))


def occupant_loads():
    """🧍 A 75 kg adult reclining: 80 % of the body weight on the seat, 150 N lean normal to the back, 40 N on each armrest (EN 1728 load points, authored-estimate shares)."""
    return [
        {"id": "occupant-seat", "kind": "occupant", "title": T("Seated adult on the seat (80 % of 75 kg)", "Sitzende erwachsene Person auf dem Sitz (80 % von 75 kg)"), "bone": "seat", "x": 0.2, "y": 0, "fxN": 0, "fyN": round(-SEAT_N, 1), "source": "authored-estimate", "reference": "din-en-1728-2012"},
        {"id": "occupant-back", "kind": "occupant", "title": T("Leaning back against the back sheet (150 N)", "Anlehnen an das Rückenblech (150 N)"), "bone": "back", "x": 0.25, "y": 0, "fxN": round(150 * BACK_V[0], 1), "fyN": round(150 * BACK_V[1], 1), "source": "authored-estimate", "reference": "din-en-1728-2012"},
        {"id": "forearm-left", "kind": "occupant", "title": T("Left forearm on the armrest (40 N)", "Linker Unterarm auf der Armlehne (40 N)"), "bone": "armrest", "x": 0.3, "y": T_R, "z": r6(zleg(0.6)), "fxN": 0, "fyN": -40, "source": "authored-estimate", "reference": "din-en-1728-2012"},
        {"id": "forearm-right", "kind": "occupant", "title": T("Right forearm on the armrest (40 N)", "Rechter Unterarm auf der Armlehne (40 N)"), "bone": "armrest", "x": 0.3, "y": T_R, "z": r6(-zleg(0.6)), "fxN": 0, "fyN": -40, "source": "authored-estimate", "reference": "din-en-1728-2012"},
    ]


def drag_loads():
    """🫳 The pulling hands at the back top while dragging."""
    return [{"id": "drag-hands", "kind": "imposed", "title": T(f"Hands dragging the chair by the back top ({abs(DRAG_FX):.0f} N back, {DRAG_FY:.0f} N up)", f"Hände ziehen den Stuhl an der Rückenoberkante ({abs(DRAG_FX):.0f} N nach hinten, {DRAG_FY:.0f} N nach oben)"), "anchor": "drag-grip", "fxN": round(DRAG_FX, 1), "fyN": round(DRAG_FY, 1), "source": "authored-estimate", "reference": "din-en-1991-1-1-2010"}]


SUN = ((1, 0.5, 0, 0), (1, 0.3, 0, 0), (1, 0.3, 0, 0), (1, 0.3, 0.1, 40))
SEATED = ((1, 0.45, 0.03, 33), (0.6, 0.05, 0.4, 32), (0.65, 0.15, 0.35, 32), (1, 0.3, 0.1, 40))
DRAGGED = ((1, 0.5, 0, 0), (1, 0.35, 0, 0), (1, 0.25, 0, 0), (1, 0.3, 0.05, 40))
LIFTED = ((1, 0.5, 0.01, 33), (1, 0.3, 0, 0), (1, 0.3, 0, 0), (1, 0.35, 0, 0))


def pose(id_, title, description, contacts, therm, loads, root=None, ground="auto", key=False):
    """📥 One pose."""
    p = {"id": id_, "title": title, "description": description}
    if key:
        p["key"] = True
    p["bones"] = {}
    if root:
        p["root"] = root
    p.update({"ground": ground, "contacts": contacts, "thermal": binding(*therm), "loads": loads})
    return p


def poses():
    """🧍 In service, lifted by the depot onto the winter stack, reclined in the sun with a visitor, dragged on its rear feet, and the key poses of the clips."""
    lift_y = 0.25
    return [
        pose("in-service", T("In service on the gravel", "Im Einsatz auf dem Kies"), T("Standing free by its own weight on its four rusted feet on the water-bound gravel of the park lawn edge, empty, in the afternoon sun.", "Steht frei durch das Eigengewicht auf den vier verrosteten Füßen auf der wassergebundenen Kiesdecke am Rasenrand, leer, in der Nachmittagssonne."), FEET, SUN, []),
        pose("maintenance", T("Lifted onto the winter stack", "Auf den Winterstapel gehoben"), T(f"In autumn the depot collects the movable chairs: a worker lifts the chair by both armrests directly over its centre of mass, {lift_y:.2f} m up onto the nesting stack; the hands are the supports, the feet are free.", f"Im Herbst sammelt der Bauhof die beweglichen Stühle ein: ein Mitarbeiter hebt den Stuhl an beiden Armlehnen genau über dem Schwerpunkt {lift_y:.2f} m hoch auf den Stapel; die Hände sind die Auflager, die Füße sind frei."), ["lift-left", "lift-right"], LIFTED, [], root={"x": 0, "y": lift_y, "angleDeg": 0}, ground="authored"),
        pose("in-use", T("Reclined in the sun", "In der Sonne zurückgelehnt"), T("A visitor of 75 kg sits reclined with the forearms on the armrests; seat, back and armrests carry the EN 1728 load points, the four feet carry the chair.", "Eine Besucherin von 75 kg sitzt zurückgelehnt mit den Unterarmen auf den Armlehnen; Sitz, Rücken und Armlehnen tragen die Lastpunkte nach EN 1728, die vier Füße tragen den Stuhl."), FEET, SEATED, occupant_loads()),
        pose("dragged", T("Dragged on its rear feet", "Auf den Hinterfüßen gezogen"), T(f"Tilted back {TILT:.0f} degrees and dragged by the back top across the gravel into the sun; only the rear feet touch the ground and scrape their rust tracks.", f"Um {TILT:.0f} Grad nach hinten gekippt und an der Rückenoberkante über den Kies in die Sonne gezogen; nur die Hinterfüße berühren den Boden und ziehen ihre Rostspuren."), REAR, DRAGGED, drag_loads(), root={"x": 0, "y": 0, "angleDeg": TILT}),
        pose("drag-arrive", T("Dragged, arriving in the sun", "Gezogen, Ankunft in der Sonne"), T("Still tilted on the rear feet, 0.9 m further back.", "Noch auf den Hinterfüßen gekippt, 0,9 m weiter hinten."), REAR, DRAGGED, drag_loads(), root={"x": DRAG_X, "y": 0, "angleDeg": TILT}, key=True),
        pose("drag-set-down", T("Set down in the sun", "In der Sonne abgesetzt"), T("Back on all four feet at its new place.", "Wieder auf allen vier Füßen am neuen Platz."), FEET, SUN, [], root={"x": DRAG_X, "y": 0, "angleDeg": 0}, key=True),
        pose("in-use-moved", T("Reclined at the new place", "Am neuen Platz zurückgelehnt"), T("The visitor reclines in the chair at its new place in the sun.", "Die Besucherin lehnt sich im Stuhl an seinem neuen Platz in der Sonne zurück."), FEET, SEATED, occupant_loads(), root={"x": DRAG_X, "y": 0, "angleDeg": 0}, key=True),
    ]


def clips():
    """🎞️ The depot lifting the chair onto the winter stack, and a visitor dragging it into the sun and sitting down."""
    lift = {"id": "maintenance", "title": T("Lifting onto the winter stack", "Auf den Winterstapel heben"), "description": T("The depot worker grips both armrests over the centre of mass, lifts the chair off the gravel and carries it 0.25 m up onto the stack.", "Der Bauhofmitarbeiter greift beide Armlehnen über dem Schwerpunkt, hebt den Stuhl vom Kies und trägt ihn 0,25 m hoch auf den Stapel."),
            "durationS": 4.0, "fps": 25, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 0.8, "pose": "in-service", "ease": "smooth"}, {"t": 2.6, "pose": "maintenance", "ease": "smooth"}, {"t": 4.0, "pose": "maintenance", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 1.0} for a in FEET] + [{"anchor": a, "fromS": 0.6, "toS": 4.0} for a in ("lift-left", "lift-right")],
            "phases": [{"id": "grip", "fromS": 0, "toS": 0.8, "title": T("Gripping the armrests", "Armlehnen greifen")}, {"id": "lift", "fromS": 0.8, "toS": 2.6, "title": T("Lifting off the gravel", "Vom Kies abheben")}, {"id": "on-stack", "fromS": 2.6, "toS": 4.0, "title": T("Set onto the stack", "Auf den Stapel gesetzt")}],
            "events": [{"t": 0.6, "kind": "contact-on", "title": T("Hands grip the left armrest", "Hände greifen die linke Armlehne"), "target": "lift-left"}, {"t": 0.6, "kind": "contact-on", "title": T("Hands grip the right armrest", "Hände greifen die rechte Armlehne"), "target": "lift-right"}]
            + [{"t": 1.0, "kind": "contact-off", "title": T(f"Foot {a.split('-', 1)[1]} leaves the gravel", f"Fuß {a.split('-', 1)[1]} verlässt den Kies"), "target": a} for a in FEET],
            "tracks": []}
    drag = {"id": "drag-and-recline", "title": T("Dragged into the sun and reclined", "In die Sonne gezogen und zurückgelehnt"), "description": T("A visitor tilts the chair back on its rear feet, drags it 0.9 m across the gravel into the sun, sets it down and sits down reclining; the body load comes on, the seat and back are shaded and warmed by contact.", "Eine Besucherin kippt den Stuhl auf die Hinterfüße, zieht ihn 0,9 m über den Kies in die Sonne, setzt ihn ab und setzt sich zurückgelehnt hinein; die Körperlast kommt auf, Sitz und Rücken werden beschattet und durch Kontakt erwärmt."),
            "durationS": 10.0, "fps": 25, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 0.8, "pose": "in-service", "ease": "smooth"}, {"t": 1.8, "pose": "dragged", "ease": "linear"}, {"t": 4.6, "pose": "drag-arrive", "ease": "smooth"}, {"t": 5.6, "pose": "drag-set-down", "ease": "smooth"}, {"t": 6.4, "pose": "drag-set-down", "ease": "smooth"}, {"t": 8.0, "pose": "in-use-moved", "ease": "smooth"}, {"t": 10.0, "pose": "in-use-moved", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 10.0} for a in REAR] + [{"anchor": a, "fromS": s, "toS": e} for a in FEET[2:] for s, e in ((0, 1.1), (5.3, 10.0))],
            "phases": [{"id": "tilt", "fromS": 0, "toS": 1.8, "title": T("Tilting back", "Zurückkippen")}, {"id": "drag", "fromS": 1.8, "toS": 4.6, "title": T("Dragging across the gravel", "Über den Kies ziehen")}, {"id": "set-down", "fromS": 4.6, "toS": 6.4, "title": T("Setting down in the sun", "In der Sonne absetzen")}, {"id": "sit-down", "fromS": 6.4, "toS": 8.0, "title": T("Sitting down", "Hinsetzen")}, {"id": "reclined", "fromS": 8.0, "toS": 10.0, "title": T("Reclined in the sun", "In der Sonne zurückgelehnt")}],
            "events": [{"t": 1.1, "kind": "contact-off", "title": T("Front left foot lifts", "Vorderer linker Fuß hebt ab"), "target": "contact-front-left"}, {"t": 1.1, "kind": "contact-off", "title": T("Front right foot lifts", "Vorderer rechter Fuß hebt ab"), "target": "contact-front-right"},
                       {"t": 1.8, "kind": "load-on", "title": T("Hands drag the chair", "Hände ziehen den Stuhl"), "target": "drag-hands"}, {"t": 5.3, "kind": "contact-on", "title": T("Front left foot touches down", "Vorderer linker Fuß setzt auf"), "target": "contact-front-left"}, {"t": 5.3, "kind": "contact-on", "title": T("Front right foot touches down", "Vorderer rechter Fuß setzt auf"), "target": "contact-front-right"},
                       {"t": 7.2, "kind": "load-on", "title": T("The visitor sits down", "Die Besucherin setzt sich"), "target": "occupant-seat"}, {"t": 7.2, "kind": "thermal", "title": T("Body shades and warms seat and back", "Körper beschattet und wärmt Sitz und Rücken")}],
            "tracks": []}
    return [lift, drag]


def layer(id_, title, kind, material, t, k, rho, cp, ref):
    """🧶 One metal layer with resistance t/k and the heat capacity rho cp t."""
    return {"id": id_, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / k), "source": "literature", "heatCapacityJM2K": round(rho * cp * t), "reference": ref}


def thermal():
    """🌡️ Four passive regions in the summer afternoon sun of a park: the weathered green frame, the two perforated sheets and the rusted feet."""
    wall = layer("tube-wall", T("Tube and bar wall 2 mm, coated", "Rohr- und Stabwand 2 mm, beschichtet"), "metal", "painted-steel", 0.002, 50, RHO_STEEL, 460, "iso-10456-2007")
    sheet = layer("sheet", T("Perforated sheet 2 mm", "Lochblech 2 mm"), "metal", "perforated-sheet", 0.002, 30, RHO_STEEL * (1 - OPEN_AREA), 460, "din-24041-2002")
    rust = layer("rusted-wall", T("Rusted tube wall and flange", "Verrostete Rohrwand und Flansch"), "metal", "rusted-steel", 0.002, 50, RHO_STEEL, 460, "incropera-2018")
    regions = [
        {"id": "frame-steel", "title": T("Bent tube frames, rails and flat-bar carriers (weathered moss green)", "Gebogene Rohrrahmen, Traversen und Flachstahlträger (verwittertes Moosgrün)"), "kind": "surface", "model": "passive", "material": "painted-steel", "areaM2": round(AREA["frame-steel"], 3), "characteristicLengthM": 0.025, "geometry": "cylinder", "skyViewFactor": 0.6, "layers": [wall], "heatCapacityJM2K": wall["heatCapacityJM2K"], "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "levinson-2005"},
        {"id": "seat-sheet", "title": T("Perforated seat sheet with rolled front hem", "Gelochtes Sitzblech mit gerolltem Vordersaum"), "kind": "surface", "model": "passive", "material": "perforated-sheet", "areaM2": round(AREA["seat-sheet"], 3), "characteristicLengthM": 0.45, "geometry": "horizontal-plate-up", "skyViewFactor": 0.85, "layers": [sheet], "heatCapacityJM2K": sheet["heatCapacityJM2K"], "source": "authored-estimate", "reference": "din-24041-2002"},
        {"id": "back-sheet", "title": T("Perforated back sheet with rolled top hem", "Gelochtes Rückenblech mit gerolltem Obersaum"), "kind": "surface", "model": "passive", "material": "perforated-sheet", "areaM2": round(AREA["back-sheet"], 3), "characteristicLengthM": 0.38, "geometry": "vertical-plate", "skyViewFactor": 0.55, "layers": [sheet], "heatCapacityJM2K": sheet["heatCapacityJM2K"], "source": "authored-estimate", "reference": "din-24041-2002"},
        {"id": "feet-rust", "title": T("Rusted feet with flanges (glides lost)", "Verrostete Füße mit Flanschen (Gleiter verloren)"), "kind": "surface", "model": "passive", "material": "rusted-steel", "areaM2": round(AREA["feet-rust"], 4), "characteristicLengthM": 0.025, "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [rust], "heatCapacityJM2K": rust["heatCapacityJM2K"], "source": "authored-estimate", "reference": "incropera-2018"},
    ]
    return {"environment": {"airTempC": 28, "meanRadiantTempC": 31, "skyTempC": 14, "groundTempC": 42, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 850, "diffuseFraction": 0.15, "pressurePa": 101325},
            "referenceEnvironment": "summer-hot-sun", "passengers": [], "operating": [], "regions": regions, "emitters": [],
            "assumptions": [
                T("Passive steel in the summer sun of a park afternoon (850 W/m2, 28 degC air, sunlit water-bound gravel at 42 degC): absorbed radiation, long-wave exchange with sky and ground, convection and conduction with heat capacity; no emitters and no power (the frame states passive 0 W).", "Passiver Stahl in der Sommersonne eines Parknachmittags (850 W/m2, Luft 28 degC, besonnte wassergebundene Kiesdecke 42 degC): absorbierte Strahlung, Langwellenaustausch mit Himmel und Boden, Konvektion und Leitung mit Wärmekapazität; keine Emitter und keine Leistung (der Rahmen gibt passiv 0 W an)."),
                T("Weathered surfaces differ from new ones: the moss-green coating has chalked and faded from a solar absorptance of about 0.88 to 0.80 (Levinson et al. 2005, authored estimate for the weathering); the rusted feet absorb 0.78 at an emissivity of 0.82 (Incropera, oxidised steel); the hand-worn bare grips emit only 0.3.", "Verwitterte Flächen unterscheiden sich von neuen: die moosgrüne Beschichtung ist von einem solaren Absorptionsgrad um 0,88 auf 0,80 gekreidet und verblasst (Levinson et al. 2005, Schätzwert für die Verwitterung); die verrosteten Füße absorbieren 0,78 bei einem Emissionsgrad von 0,82 (Incropera, oxidierter Stahl); die blank gegriffenen Armlehnen emittieren nur 0,3."),
                T("The 2 mm sheets are thin (4.3 kJ/m2K) and follow the sun within minutes: an empty dark seat in full sun is the hot part of the chair. Their areaM2 counts both faces of the plate; the 40 % open perforation is smeared into density and conductivity, and sunlitFraction 0.3 is the top face (half the area) times the closed 60 %.", "Die 2-mm-Bleche sind dünn (4,3 kJ/m2K) und folgen der Sonne binnen Minuten: ein leerer dunkler Sitz in voller Sonne ist der heiße Teil des Stuhls. Ihr areaM2 zählt beide Blechseiten; die 40 % offene Lochung ist in Dichte und Leitfähigkeit verschmiert, und sunlitFraction 0,3 ist die Oberseite (die halbe Fläche) mal die geschlossenen 60 %."),
                T("A seated adult covers 80 % of the seat top and most of the back sheet front with clothing at about 32 degC (ISO 7730) and shades them; the forearms rest on the armrests; the feet touch the gravel with a tenth of their area.", "Eine sitzende erwachsene Person bedeckt 80 % der Sitzoberseite und den größten Teil der Rückenvorderseite mit Kleidung von etwa 32 degC (ISO 7730) und beschattet sie; die Unterarme liegen auf den Armlehnen; die Füße berühren den Kies mit einem Zehntel ihrer Fläche."),
            ]}


def structure():
    """🏗️ Quasi-static load paths: four feet on the gravel in service and in use, two rear feet when dragged, the worker's hands at the armrests when lifted."""
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "clinical": False,
            "assumptions": [
                T("The welded chair is one rigid body of five members on the ground frame: each bent side frame is two legs and an armrest of Ø25 x 2 mm tube (S235, both frames in one section), seat and back are carried by two 30 x 8 mm flat bars each; joints at the bends and welds are rigid.", "Der geschweißte Stuhl ist ein starrer Körper aus fünf Gliedern am Bezugsrahmen: jeder gebogene Seitenrahmen besteht aus zwei Beinen und einer Armlehne aus Rohr Ø25 x 2 mm (S235, beide Rahmen in einem Querschnitt), Sitz und Rücken tragen je zwei Flachstähle 30 x 8 mm; Gelenke an Bögen und Schweißungen sind biegesteif."),
                T(f"Self weight {MASS_KG:.1f} kg from the true sections (tube 1.13 kg/m, flat bar 1.88 kg/m, perforated sheet 9.4 kg/m2 at 40 % open area, rails Ø20 and Ø16); free-standing by its weight, one unilateral contact with friction 0.5 per foot on water-bound gravel.", f"Eigengewicht {MASS_KG:.1f} kg aus den echten Querschnitten (Rohr 1,13 kg/m, Flachstahl 1,88 kg/m, Lochblech 9,4 kg/m2 bei 40 % offener Fläche, Traversen Ø20 und Ø16); frei stehend durch das Eigengewicht, je Fuß ein einseitiger Kontakt mit Reibung 0,5 auf wassergebundener Decke."),
                T("In use a 75 kg adult reclines: 588 N on the seat, 150 N normal to the back, 40 N per armrest at the EN 1728 load points; the EN 581-2 / EN 16139 proof loads for public seating (1300 N seat, 560 N back) are far above these service loads and are not a pose.", "In Benutzung lehnt sich eine Person von 75 kg zurück: 588 N auf dem Sitz, 150 N senkrecht zum Rücken, 40 N je Armlehne an den Lastpunkten nach EN 1728; die Prüflasten nach EN 581-2 / EN 16139 für öffentliche Sitzmöbel (1300 N Sitz, 560 N Rücken) liegen weit über diesen Gebrauchslasten und sind keine Pose."),
                T(f"Dragging: tilted {TILT:.0f} degrees about the rear feet and pulled by the back top, the hands hold {DRAG_FY:.0f} N up and pull {abs(DRAG_FX):.0f} N back so that the moment about the rear feet and the sliding friction of the scraping feet balance (sliding friction 0.4 below the static 0.5 of the contacts). Lifting: one worker holds both armrests over the centre of mass (pin left, roller right), the feet are free.", f"Ziehen: um {TILT:.0f} Grad um die Hinterfüße gekippt und an der Rückenoberkante gezogen, halten die Hände {DRAG_FY:.0f} N nach oben und ziehen {abs(DRAG_FX):.0f} N nach hinten, sodass Moment um die Hinterfüße und die Gleitreibung der schleifenden Füße im Gleichgewicht sind (Gleitreibung 0,4 unter der Haftreibung 0,5 der Kontakte). Heben: ein Mitarbeiter hält beide Armlehnen über dem Schwerpunkt (links Gelenk, rechts Rolle), die Füße sind frei."),
            ]}


def extents():
    """📏 Overall extents of the 1:50 drawing in the rest pose (front width, plan length, height)."""
    xs, ys, zs = [], [], []
    for part in parts_of(50):
        if part.get("texture"):
            continue
        for view in ("front", "plan"):
            kind, data = outline(part, view)
            pts = data if kind == "poly" else [(data[0][0] + sx * data[1][0], data[0][1] + sy * data[1][1]) for sx in (-1, 1) for sy in (-1, 1)]
            for a, b in pts:
                if view == "front":
                    zs.append(a)
                    ys.append(b)
                else:
                    xs.append(a)
                    zs.append(b)
    return max(ys) - min(ys), max(xs) - min(xs), max(zs) - min(zs)


def document():
    """📄 The whole asset document."""
    h, w, d = extents()
    asset = {
        "id": "used-park-armchair", "category": "furniture", "title": T("Used Movable Park Armchair", "Gebrauchter Parkstuhl"),
        "summary": T("A movable steel armchair of the city-park type after years of service on the gravel of a municipal park: two Ø25 mm tube side frames bent into rear leg, armrest and front leg, a perforated 2 mm steel seat and back on flat-bar carriers, seat 0.40 m, armrests 0.61 m, 12.8 kg. The moss-green paint has chalked and flakes, the armrest fronts are worn bare by hands, the polyamide glides are lost and the feet have rusted. The depot lifts it by the armrests onto the winter stack (the splayed legs let the chairs nest); visitors drag it across the gravel into the sun and recline.", "Ein beweglicher Stahlsessel vom Stadtpark-Typ nach Jahren im Einsatz auf dem Kies eines städtischen Parks: zwei Seitenrahmen aus Rohr Ø25 mm, zu Hinterbein, Armlehne und Vorderbein gebogen, Sitz und Rücken aus 2-mm-Lochblech auf Flachstahlträgern, Sitz 0,40 m, Armlehnen 0,61 m, 12,8 kg. Die moosgrüne Farbe ist gekreidet und blättert, die Armlehnen sind vorn von Händen blank gegriffen, die Polyamidgleiter sind verloren und die Füße verrostet. Der Bauhof hebt ihn an den Armlehnen auf den Winterstapel (die gespreizten Beine lassen die Stühle ineinander stapeln); Besucher ziehen ihn über den Kies in die Sonne und lehnen sich zurück."),
        "tags": ["used", "park", "public-space", "seating", "armchair", "chair", "movable", "stackable", "steel", "perforated-sheet", "street-furniture", "rust"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS],
                       "notes": T("Construction after the municipal movable park armchair of the city-park type (catalogue-typical, no single product): bent Ø25 x 2 mm precision tube side frames splayed 15 mm for stacking, Ø20 front seat rail and Ø16 rear stretcher, perforated 2 mm sheets Rv 8-12 with rolled Ø16 hems on 30 x 8 mm flat-bar carriers, Ø30 foot flanges. Masses from the true sections and densities; the perforated sheet is smeared over its open area. Traces of use are authored estimates. Nothing is measured.", "Konstruktion nach dem beweglichen städtischen Parksessel vom Stadtpark-Typ (katalogtypisch, kein Einzelprodukt): gebogene Seitenrahmen aus Präzisionsrohr Ø25 x 2 mm, für das Stapeln 15 mm gespreizt, vordere Sitztraverse Ø20 und hintere Strebe Ø16, 2-mm-Lochbleche Rv 8-12 mit gerollten Säumen Ø16 auf Flachstahlträgern 30 x 8 mm, Fußflansche Ø30. Massen aus echten Querschnitten und Dichten; das Lochblech ist über seine offene Fläche verschmiert. Gebrauchsspuren sind Schätzwerte. Nichts ist gemessen.")},
        "dimensions": {"heightM": round(h, 3), "widthM": round(w, 3), "depthM": round(d, 3), "massKg": round(MASS_KG, 2)},
        "materials": materials(), "rig": rig(), "anchors": anchors(),
        "representations": {str(s): representation(s) for s in SCALES},
        "symbols": [], "poses": poses(), "clips": clips(), "thermal": thermal(), "structure": structure(),
        "volumes": {str(s): volume(s) for s in SCALES},
        "presentation": {"defaultPose": "in-service", "defaultClip": "drag-and-recline", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    a = doc["asset"]
    print("[DEBUG] dimensions", a["dimensions"], "COM", [round(c, 4) for c in COM], "drag", round(DRAG_FX, 1), round(DRAG_FY, 1))
    print("[DEBUG] areas", {k: round(v, 4) for k, v in AREA.items()})
    for s in SCALES:
        v = a["volumes"][str(s)]
        print("[DEBUG] 1:%d body %d thermal %d structure %d; shapes %s" % (s, len(v["body"]), len(v["thermal"]), len(v["structure"]), {k: len(x) for k, x in a["representations"][str(s)]["geometry"].items()}))
