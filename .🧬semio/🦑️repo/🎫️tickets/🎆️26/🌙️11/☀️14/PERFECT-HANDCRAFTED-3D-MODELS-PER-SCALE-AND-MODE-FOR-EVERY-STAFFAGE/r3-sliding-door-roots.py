"""🚪 Round 3 sliding door: the root fixes outside `volumes` (rig, poses, masses, track section, materials, references, thermal layers and areas, the 2D drawings of every scale, the symbol, texts), applied to the catalog asset.

Usage: python3 r3-sliding-door-roots.py            writes the candidate document (with the volumes source) to 🗑️generated/r3-sliding-door/🔣️.json
       python3 r3-sliding-door-roots.py --apply    writes the changed members into the catalog file (re-read right before, only these members change)
"""
import copy
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = load("kit", "r3-sliding-door-kit.py")
V = load("vol", "r3-sliding-door-volumes.py")
L = load("layout", "r3-sliding-door-layout.py")
shape, rect, poly, box_poly, disc, stadium, text, r5 = K.shape, K.rect, K.poly, K.box_poly, K.disc, K.stadium, K.text, K.r5

MEMBERS = ["summary", "provenance", "dimensions", "materials", "rig", "representations", "symbols", "poses", "thermal", "structure"]
TR, LF = "track", "leaf"
HV = K.HANGER_V
RHO = 7850

TRACK_A, TRACK_CY, TRACK_I = K.polygon(K.track_profile())
TRACK_C = r5(K.HALF + abs(TRACK_CY))
JAMB_A = K.polygon(K.jamb_profile(0, 1))[0] - K.polygon(K.jamb_profile(0, 1, K.T_TRACK))[0]
CAP_A = K.polygon(K.rounded_rect(-K.HALF, -K.HALF, K.HALF, K.HALF, K.R_OUT))[0]
TRACK_KG = round(TRACK_A * (K.TRACK_X1 - K.TRACK_X0 - 2 * K.CAP_T) * RHO + 2 * CAP_A * K.CAP_T * RHO + 0.2, 2)
JAMBS_KG = round(2 * (JAMB_A * (K.JAMB_TOP - K.FLOOR) * RHO + K.SEAL_T * 0.05 * (K.JAMB_TOP - K.FLOOR) * 1150), 2)
GUIDE_KG = 0.25
LEAF_KG = round(V.LEAF_KG, 2)
LEAF_COM = round(V.LEAF_COM_U, 3)
TOTAL_KG = round(TRACK_KG + JAMBS_KG + GUIDE_KG + LEAF_KG, 2)

STEEL = dict(region=None, material="steel")


def jamb_plan(cx, side, r=0.0):
    """🏛️ The cut jamb in plan (x ahead, y lateral): the rebated post with its receiving channel."""
    return poly([[u, w] for w, u in K.jamb_profile(cx, side, 0.0, r)])


def leaf_elevation(scale):
    """🚪 The leaf in its bone frame (x = u down from the track axis, y = v along the track) at one scale."""
    out = []
    if scale == 50:
        out += [
            shape("leaf-frame", LF, "body", 2, True, 0.5, box_poly(K.U_TOP, -K.V_OUT, K.U_RAIL_BOT, K.V_OUT), region="leaf-frame", material="steel"),
            shape("leaf-glass", LF, "glazing", 3, True, 0.92, box_poly(K.U_TOP_RAIL, -K.V_IN, K.U_GLASS_BOT, K.V_IN), region="leaf-glass", material="glass"),
            shape("glazing-bead", LF, "detail", 3.05, False, 0.35, box_poly(K.U_TOP_RAIL + K.BEAD, -K.V_IN + K.BEAD, K.U_GLASS_BOT - K.BEAD, K.V_IN - K.BEAD), region="leaf-frame"),
            shape("glass-reflection-a", LF, "texture", 3.1, False, 0.3, poly([[1.6, -0.3], [1.0, 0.3]], False), region="leaf-glass"),
            shape("glass-reflection-b", LF, "texture", 3.1, False, 0.3, poly([[1.7, -0.2], [1.2, 0.3]], False), region="leaf-glass"),
            shape("glass-reflection-c", LF, "texture", 3.1, False, 0.3, poly([[0.9, -0.35], [0.6, -0.05]], False), region="leaf-glass"),
        ]
        for name, sign in (("a", -1), ("b", 1)):
            v = sign * HV
            hanger = [[0.05, v - 0.006], [K.NUT_U[0], v - 0.006], [K.NUT_U[0], v - 0.011], [K.CLEVIS_U[0], v - 0.011], [K.CLEVIS_U[0], v - 0.025], [K.CLEVIS_U[1], v - 0.025], [K.CLEVIS_U[1], v + 0.025], [K.CLEVIS_U[0], v + 0.025], [K.CLEVIS_U[0], v + 0.011], [K.NUT_U[0], v + 0.011], [K.NUT_U[0], v + 0.006], [0.05, v + 0.006]]
            out.append(shape(f"hanger-{name}", LF, "body", 2.5, True, 0.3, poly(hanger), region="leaf-frame", material="steel"))
            out.append(shape(f"hanger-bolt-{name}", LF, "detail", 2.6, True, 0.1, disc(K.PIN_U, v, 0.008), region="leaf-frame"))
        out += [
            shape("handle-base", LF, "body", 3.2, True, 0.3, disc(K.LEG_U[0], K.HANDLE_V, K.ROSETTE[2]), material="steel"),
            shape("handle-base-b", LF, "body", 3.2, True, 0.3, disc(K.LEG_U[1], K.HANDLE_V, K.ROSETTE[2]), material="steel"),
            shape("handle-grip", LF, "body", 3.3, True, 0.25, stadium(K.HANDLE_U[0] + 0.008, K.HANDLE_V, K.HANDLE_U[1] - 0.008, K.HANDLE_V, 0.008), material="steel"),
            shape("bottom-seal", LF, "body", 2.2, True, 0.2, box_poly(K.U_RAIL_BOT, -K.V_OUT, K.U_BRUSH, K.V_OUT), region="leaf-frame", material="steel", pen=0.1),
        ]
    elif scale == 100:
        out += [
            shape("leaf-frame", LF, "body", 2, True, 0.5, box_poly(K.U_TOP, -K.V_OUT, K.U_BRUSH, K.V_OUT), region="leaf-frame", material="steel"),
            shape("leaf-glass", LF, "glazing", 3, True, 0.92, box_poly(K.U_TOP_RAIL, -K.V_IN, K.U_GLASS_BOT, K.V_IN), region="leaf-glass", material="glass"),
            shape("glass-reflection-a", LF, "texture", 3.1, False, 0.3, poly([[1.6, -0.3], [1.0, 0.3]], False), region="leaf-glass"),
            shape("glass-reflection-b", LF, "texture", 3.1, False, 0.3, poly([[1.7, -0.2], [1.2, 0.3]], False), region="leaf-glass"),
        ]
        for name, sign in (("a", -1), ("b", 1)):
            out.append(shape(f"hanger-{name}", LF, "body", 2.5, True, 0.3, box_poly(0.05, sign * HV - 0.025, K.U_TOP, sign * HV + 0.025), region="leaf-frame", material="steel", pen=0.1))
        out.append(shape("handle", LF, "body", 3.2, True, 0.3, box_poly(K.HANDLE_U[0], K.HANDLE_V - 0.01, K.HANDLE_U[1], K.HANDLE_V + 0.01), material="steel", pen=0.1))
    elif scale == 200:
        out += [
            shape("leaf-frame-200", LF, "body", 2, True, 0.5, box_poly(0.05, -K.V_OUT, K.U_BRUSH, K.V_OUT), region="leaf-frame", material="steel"),
            shape("leaf-glass-200", LF, "glazing", 3, True, 0.92, box_poly(0.125, -0.425, K.U_GLASS_BOT, 0.425), region="leaf-glass", material="glass"),
        ]
    else:
        out.append(shape("leaf", LF, "body", 2, True, 0.8, box_poly(K.U_TOP, -K.V_OUT, K.U_BRUSH, K.V_OUT), regions=["leaf-glass", "leaf-frame"], material="glass"))
    return out


def portal_elevation(scale):
    if scale == 500:
        return None
    out = [
        shape("jamb-west", TR, "body", 1, True, 0.45, rect(-0.05, K.FLOOR, 0.05, K.JAMB_TOP), region="jamb-steel", material="steel"),
        shape("jamb-east", TR, "body", 1, True, 0.45, rect(2.95, K.FLOOR, 3.05, K.JAMB_TOP), region="jamb-steel", material="steel"),
        shape("track", TR, "body", 1, True, 0.45, rect(-0.05, -0.05, 3.05, 0.05), region="track-steel", material="steel"),
    ]
    if scale == 50:
        out.append(shape("floor-guide", TR, "detail", 1.6, True, 0.2, rect(K.GUIDE_X[0], K.FLOOR, K.GUIDE_X[1], K.FLOOR + 0.045), region="track-steel", material="steel"))
    return out


def front(scale):
    """🔭️ The end view along the track from the east: the east jamb and the track head; the leaf behind the jamb as thin hidden outlines."""
    if scale == 500:
        return None
    out = [
        shape("jamb-east", TR, "body", 2, True, 0.45, rect(-0.05, 0, 0.05, 2.05), region="jamb-steel", material="steel"),
        shape("track", TR, "body", 1, True, 0.45, rect(-0.05, 2.05, 0.05, 2.15), region="track-steel", material="steel"),
    ]
    y_top = K.TRACK_Y - (K.U_TOP if scale != 200 else 0.05)
    y_bot = K.TRACK_Y - (K.U_RAIL_BOT if scale == 50 else K.U_BRUSH)
    out.append(shape("leaf-behind", LF, "detail", 3, False, 0.35, rect(-0.025, y_bot, 0.025, y_top), region="leaf-frame"))
    if scale == 50:
        out += [
            shape("hanger-behind", LF, "detail", 3, False, 0.35, rect(-0.025, K.TRACK_Y - K.CLEVIS_U[1], 0.025, 2.05), region="leaf-frame"),
            shape("leaf-glass-behind", LF, "detail", 3, False, 0.35, poly([[0, K.TRACK_Y - K.U_GLASS_BOT], [0, K.TRACK_Y - K.U_TOP_RAIL]], False), region="leaf-glass"),
            shape("jamb-seal", TR, "detail", 2.5, False, 0.35, poly([[-K.CHANNEL_W, 0], [-K.CHANNEL_W, 2.05]], False), region="jamb-steel"),
            shape("jamb-seal-b", TR, "detail", 2.5, False, 0.35, poly([[K.CHANNEL_W, 0], [K.CHANNEL_W, 2.05]], False), region="jamb-steel"),
            shape("bottom-seal-behind", LF, "detail", 3, False, 0.35, rect(-K.BRUSH_W[1], K.TRACK_Y - K.U_BRUSH, K.BRUSH_W[1], K.TRACK_Y - K.U_RAIL_BOT), region="leaf-frame"),
            shape("handle-behind", LF, "detail", 3, False, 0.35, rect(-0.046, K.TRACK_Y - K.HANDLE_U[1], 0.046, K.TRACK_Y - K.HANDLE_U[0]), region="leaf-frame"),
        ]
    return out


def plan(scale):
    """🗺️ The horizontal section at 1.0 m (x ahead = world x, y lateral)."""
    x0, x1 = 1.5 - K.V_OUT, 1.5 + K.V_OUT
    if scale == 50:
        hx = 1.5 + K.HANDLE_V
        stile = lambda xa, xb, sign: poly([[xa, -0.025], [xb, -0.025], [xb, -0.02], [xb + sign * K.BEAD, -0.02], [xb + sign * K.BEAD, -0.005], [xb, -0.005], [xb, 0.005], [xb + sign * K.BEAD, 0.005], [xb + sign * K.BEAD, 0.02], [xb, 0.02], [xb, 0.025], [xa, 0.025]])
        return [
            shape("jamb-west", TR, "body", 1, True, 0.45, jamb_plan(0.0, 1, K.R_OUT), region="jamb-steel", material="steel"),
            shape("jamb-east", TR, "body", 1, True, 0.45, jamb_plan(3.0, -1, K.R_OUT), region="jamb-steel", material="steel"),
            shape("track-above", TR, "texture", 0.5, False, 0.35, poly([[-0.05, -0.05], [3.05, -0.05], [3.05, 0.05], [-0.05, 0.05]]), region="track-steel"),
            shape("leaf-stile-west", LF, "body", 2, True, 0.5, stile(x0, x0 + 0.05, 1), region="leaf-frame", material="steel"),
            shape("leaf-stile-east", LF, "body", 2, True, 0.5, stile(x1, x1 - 0.05, -1), region="leaf-frame", material="steel"),
            shape("handle-base-cut", LF, "body", 2, True, 0.5, rect(hx - K.ROSETTE[2], -K.ROSETTE[1], hx + K.ROSETTE[2], K.ROSETTE[1]), region="leaf-frame", material="steel"),
            shape("handle-grip-cut", LF, "body", 2, True, 0.5, rect(hx - 0.008, -(K.GRIP_W + 0.008), hx + 0.008, K.GRIP_W + 0.008), region="leaf-frame", material="steel"),
            shape("leaf-glass", LF, "glazing", 3, True, 0.92, rect(x0 + 0.05, -0.005, x1 - 0.05, 0.005), region="leaf-glass", material="glass"),
            shape("leaf-rail-below", LF, "body", 1.8, True, 0.6, rect(x0 + 0.05, -0.025, x1 - 0.05, 0.025), region="leaf-frame", material="steel"),
            shape("jamb-west-seal", TR, "detail", 1.5, False, 0.35, poly([[K.HALF - K.CHANNEL_D + K.SEAL_T, -K.CHANNEL_W], [K.HALF - K.CHANNEL_D + K.SEAL_T, K.CHANNEL_W]], False), region="jamb-steel"),
            shape("jamb-east-seal", TR, "detail", 1.5, False, 0.35, poly([[3.0 - K.HALF + K.CHANNEL_D - K.SEAL_T, -K.CHANNEL_W], [3.0 - K.HALF + K.CHANNEL_D - K.SEAL_T, K.CHANNEL_W]], False), region="jamb-steel"),
            shape("hanger-above-a", LF, "texture", 2.5, False, 0.35, rect(1.5 - HV - 0.025, -0.025, 1.5 - HV + 0.025, 0.025), region="leaf-frame"),
            shape("hanger-above-b", LF, "texture", 2.5, False, 0.35, rect(1.5 + HV - 0.025, -0.025, 1.5 + HV + 0.025, 0.025), region="leaf-frame"),
        ]
    if scale == 100:
        hx = 1.5 + K.HANDLE_V
        return [
            shape("jamb-west", TR, "body", 1, True, 0.45, jamb_plan(0.0, 1), region="jamb-steel", material="steel"),
            shape("jamb-east", TR, "body", 1, True, 0.45, jamb_plan(3.0, -1), region="jamb-steel", material="steel"),
            shape("track-above", TR, "texture", 0.5, False, 0.35, poly([[-0.05, -0.05], [3.05, -0.05], [3.05, 0.05], [-0.05, 0.05]]), region="track-steel"),
            shape("leaf-frame", LF, "body", 2, True, 0.5, rect(x0, -0.025, x1, 0.025), region="leaf-frame", material="steel"),
            shape("handle-cut", LF, "body", 2.1, True, 0.3, rect(hx - 0.01, -0.046, hx + 0.01, 0.046), region="leaf-frame", material="steel"),
            shape("leaf-glass", LF, "glazing", 3, True, 0.92, rect(x0 + 0.05, -0.005, x1 - 0.05, 0.005), region="leaf-glass", material="glass"),
        ]
    return None


def apply(asset):
    """🔧 Every root change outside `volumes`, in place."""
    asset["summary"] = text(
        "A 1.0 m wide, 1.99 m high single-glazed (10 mm toughened glass) steel-framed leaf hung from two trolleys in a 3.1 m slotted steel box track that rests on two rebated steel jambs, with a floor guide; conditioned element with interior at 20 degC and a winter reference case. The adjacent bays of the opening are not modelled.",
        "Ein 1,0 m breiter, 1,99 m hoher einfachverglaster (10 mm ESG) Stahlrahmenflügel an zwei Laufwagen in einer 3,1 m langen geschlitzten Stahl-Kastenlaufschiene, die auf zwei Stahl-Falzpfosten liegt, mit Bodenführung; konditioniertes Bauteil mit Innenraum 20 °C und Winter-Referenzfall. Die übrigen Felder der Öffnung sind nicht modelliert.")
    refs = asset["provenance"]["references"]
    known = {r["id"] for r in refs}
    for r in (
        {"id": "en-1527", "citation": "EN 1527:2019 Building hardware, hardware for sliding doors and folding doors, requirements and test methods (top-hung sliding door gear: trolleys in a box track, hanger bolts, floor guide, end stops)."},
        {"id": "en-10162", "citation": "EN 10162:2003 Cold rolled steel sections, technical delivery conditions, dimensional and cross-sectional tolerances (cold-formed box track and rebated posts, inside bend radius about one wall thickness)."},
        {"id": "din-18101", "citation": "DIN 18101:2014 Doors for residential buildings, door leaf sizes, hinge and lock positions (leaf height about 1985 mm for the 2010 mm nominal opening, handle height 1050 mm above the finished floor)."},
    ):
        if r["id"] not in known:
            refs.append(r)
    asset["provenance"]["notes"] = text(
        "The leaf hangs from two trolleys 350 mm either side of its centre that run on the treads of the box track; their resultant acts at the carriage, the prismatic joint at the centre of the top edge, and transmits the leaf weight into the track, which rests on the jamb heads. The floor guide only restrains the leaf out of plane; the wind on the leaf is out of plane and is not in this elevation model.",
        "Der Flügel hängt an zwei Laufwagen 350 mm beidseits seiner Mitte, die auf den Laufflächen der Kastenschiene rollen; ihre Resultierende wirkt am Wagen, dem Prismengelenk mittig an der Oberkante, und leitet das Flügelgewicht in die Schiene, die auf den Pfostenköpfen liegt. Die Bodenführung hält den Flügel nur quer zur Ebene; der Winddruck auf den Flügel wirkt senkrecht zur Ebene und ist in dieser Ansichts-Modellierung nicht enthalten.")
    asset["dimensions"]["massKg"] = TOTAL_KG
    if not any(m["id"] == "seal" for m in asset["materials"]):
        asset["materials"].append({"id": "seal", "title": text("EPDM buffer and nylon brush seals", "EPDM-Puffer und Nylon-Bürstendichtungen"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "source": "literature", "reference": "en-iso-10456", "conductivityWmK": 0.25, "specificHeatJKgK": 1000})
    track, leaf = asset["rig"]["bones"]
    track["title"] = text("Track (rigid root on the jambs)", "Schiene (starre Wurzel auf den Pfosten)")
    track["mass"].update({"kg": TRACK_KG, "comX": 1.5, "comY": 0, "reference": "en-10162"})
    track["section"] = {"areaM2": round(TRACK_A, 7), "secondMomentM4": round(TRACK_I, 10), "extremeFibreM": TRACK_C, "material": "steel", "title": text("Slotted box track 100 x 100 x 4", "Geschlitzte Kastenschiene 100 x 100 x 4")}
    leaf["lengthM"] = K.U_BRUSH
    leaf["joint"]["maxM"] = K.TRAVEL
    leaf["mass"].update({"kg": LEAF_KG, "comX": LEAF_COM})
    for pose in asset["poses"]:
        if pose["id"] == "open":
            pose["bones"] = {"leaf": {"dx": K.TRAVEL}}
            pose["description"] = text("Leaf parked in the receiving channel of the east jamb against its buffer, 0.962 m from closed.", "Flügel im Aufnahmefalz des Ostpfostens am Puffer geparkt, 0,962 m aus der Schließlage.")
    regions = {r["id"]: r for r in asset["thermal"]["regions"]}
    for rid in ("track-steel", "jamb-steel"):
        for layer in regions[rid]["layers"]:
            if layer["id"] == "cavity":
                layer["thicknessM"] = 0.092
    regions["track-steel"]["title"] = text("Head track, slotted steel box track 100 x 100 x 4", "Laufschiene, geschlitzte Stahl-Kastenschiene 100 x 100 x 4")
    frame = regions["leaf-frame"]
    frame["title"] = text("Leaf frame, steel tubes 50 x 50 x 2 and 50 x 100 x 2 (thermal bridge around the glass)", "Flügelrahmen, Stahlrohre 50 x 50 x 2 und 50 x 100 x 2 (Wärmebrücke um das Glas)")
    for layer in frame["layers"]:
        if layer["kind"] == "metal":
            layer.update({"title": text("Steel wall 2 mm", "Stahlwand 2 mm"), "thicknessM": 0.002, "resistanceM2KW": 0.00004, "heatCapacityJM2K": 7050})
        else:
            layer["thicknessM"] = 0.046
    frame["heatCapacityJM2K"] = 14100
    for rid, area in AREAS.items():
        regions[rid]["areaM2"] = area
    reps = asset["representations"]
    for scale in (50, 100, 200, 500):
        rep = reps[str(scale)]
        geo = rep["geometry"]
        if scale == 500:
            geo["elevation"] = [geo["elevation"][0]] + leaf_elevation(500)
            for s in geo["plan"]:
                if s["id"] == "leaf":
                    s.update(rect(1.0, -0.0375, 2.0, 0.0375))
        else:
            geo["elevation"] = portal_elevation(scale) + leaf_elevation(scale)
            geo["front"] = front(scale)
            if scale in (50, 100):
                geo["plan"] = plan(scale)
        rep["decisions"].update(TEXTS[scale])
    sym = asset["symbols"][0]["geometry"]
    for s in sym["elevation"]:
        if s["id"] == "leaf-frame-200":
            s["points"] = [[1, 0.01], [2, 0.01], [2, 2.05], [1, 2.05]]
        if s["id"] == "leaf-glass-200":
            s["points"] = [[1.075, 0.13], [1.925, 0.13], [1.925, 1.975], [1.075, 1.975]]
    for s in sym["front"]:
        if s["id"] == "leaf-behind":
            s.update({"y": 0.01, "h": 2.04})
    asset["structure"]["assumptions"] = [
        text("Track is a rigid simply supported slotted box beam between the pin on the west jamb head and the roller on the east jamb head; the leaf weight is transmitted through its two trolleys as a moving vertical load whose resultant acts at the carriage.", "Die Schiene ist ein starrer, frei aufliegender geschlitzter Kastenträger zwischen dem Festlager auf dem West- und dem Rollenlager auf dem Ostpfostenkopf; das Flügelgewicht wird über die zwei Laufwagen als wandernde Vertikallast übertragen, deren Resultierende am Wagen angreift."),
        text("Wind on the leaf is out of plane and is not represented in this elevation model; the floor guide takes only that out-of-plane restraint, the latch is not modelled.", "Winddruck auf den Flügel wirkt senkrecht zur Ebene und ist in diesem Ansichtsmodell nicht enthalten; die Bodenführung nimmt nur diese Querhalterung auf, der Riegel ist nicht modelliert."),
    ]
    return asset


AREAS = {"track-steel": 0.92}

TEXTS = {
    50: {
        "silhouette": text("A framed single-glazed sliding leaf on two pendant hangers in a slotted steel box track that rests on two rebated jambs; the leaf runs 10 mm above the floor over a floor guide. Front and plan: the front is the end view along the track; the receiving channel of the jamb, the leaf frame, hanger, glass line, handle and bottom brushes behind the jamb are thin hidden outlines. The plan is the horizontal section at 1.0 m: the cut rebated jambs with their buffer lines, the two leaf stiles (50 x 50 mm) with the glazing beads and the 10 mm glass between them, the cut pull handles, the bottom rail below the cut, and the two hangers and the track above the cut as thin outlines.",
                           "Ein gerahmter einfachverglaster Schiebeflügel an zwei Pendelaufhängungen in einer geschlitzten Stahl-Kastenschiene, die auf zwei Falzpfosten liegt; der Flügel läuft 10 mm über dem Boden über eine Bodenführung. Vorder- und Draufsicht: Die Vorderansicht ist die Ansicht entlang der Schiene; Aufnahmefalz des Pfostens, Flügelrahmen, Aufhängung, Glaslinie, Griff und Bodenbürsten hinter dem Pfosten sind dünne verdeckte Umrisslinien. Die Draufsicht ist der Horizontalschnitt in 1,0 m Höhe: die geschnittenen Falzpfosten mit ihren Pufferlinien, die beiden Flügelpfosten (50 x 50 mm) mit den Glashalteleisten und dem 10-mm-Glas dazwischen, die geschnittenen Stoßgriffe, der Fußriegel unter dem Schnitt sowie die beiden Aufhängungen und die Schiene über der Schnittebene als dünne Umrisslinien."),
        "retained": text("Jambs and track (2 mm), the floor guide, leaf frame (20 x 39 mm with the 2 mm bottom rail), glass with the glazing bead line, three reflection lines, two hangers (nuts and clevis) with their pins, the pull handle (two rosettes and the grip), the bottom brush.",
                         "Pfosten und Schiene (2 mm), die Bodenführung, Flügelrahmen (20 x 39 mm mit dem 2-mm-Fußriegel), Glas mit der Linie der Glashalteleisten, drei Reflexlinien, zwei Aufhängungen (Muttern und Gabelkopf) mit ihren Bolzen, der Stoßgriff (zwei Rosetten und die Griffstange), die Bodenbürste."),
        "merged": text("The steel walls of the hollow profiles are not resolved; the frame is one filled shape, the stiles and rails are welded flush.",
                       "Die Stahlwände der Hohlprofile sind nicht aufgelöst; der Rahmen ist eine gefüllte Form, Pfosten und Riegel sind bündig verschweißt."),
        "omitted": text("Trolleys and wheels (hidden inside the track), the slot brushes under the track, the welds.",
                        "Laufwagen und Rollen (in der Schiene verdeckt), die Schlitzbürsten unter der Schiene, die Schweißnähte."),
        "thinElements": text("Pins are 0.32 mm dots (16 mm), the hanger rod neck is 0.24 mm wide (12 mm); the 20 mm bottom brush is 0.4 mm tall with a 0.1 mm outline.",
                             "Bolzen sind Punkte von 0,32 mm (16 mm), der Hals des Hängebolzens ist 0,24 mm breit (12 mm); die 20-mm-Bodenbürste ist 0,4 mm hoch mit 0,1-mm-Umriss."),
    },
    100: {
        "silhouette": text("The door elevation as a steel portal with a framed glass leaf, a pair of hanger blocks and a pull handle; the leaf runs 10 mm above the floor. Front and plan: the front adds the leaf frame behind the jamb as a thin hidden outline. The plan is the horizontal section at 1.0 m: the rebated jambs filled, the glass 10 mm as a thin pane between the two stiles drawn as one frame bar, the cut handle across it; the track above the cut is a thin outline.",
                           "Die Türansicht als Stahlportal mit gerahmtem Glasflügel, einem Paar Aufhängeblöcken und einem Griff; der Flügel läuft 10 mm über dem Boden. Vorder- und Draufsicht: Die Vorderansicht ergänzt den Flügelrahmen hinter dem Pfosten als dünne verdeckte Umrisslinie. Die Draufsicht ist der Horizontalschnitt in 1,0 m Höhe: die Falzpfosten gefüllt, das 10-mm-Glas als dünne Scheibe zwischen den beiden als ein Rahmenbalken gezeichneten Pfosten, der geschnittene Griff darüber; die Schiene über der Schnittebene ist eine dünne Umrisslinie."),
        "retained": text("Jambs and track (1 mm), the 0.2 mm jamb rebate in plan, leaf frame (10 x 19.9 mm) with 0.5 mm frame width and the 1 mm bottom rail, the glass pane, two hanger blocks (0.5 x 0.5 mm), handle (0.2 x 3 mm).",
                         "Pfosten und Schiene (1 mm), der 0,2-mm-Pfostenfalz im Grundriss, Flügelrahmen (10 x 19,9 mm) mit 0,5 mm Rahmenbreite und dem 1-mm-Fußriegel, die Glasscheibe, zwei Aufhängeblöcke (0,5 x 0,5 mm), Griff (0,2 x 3 mm)."),
        "merged": text("The leaf frame profile and the bottom brush are one frame outline around the glass; rod, nuts and clevis are one hanger block.",
                       "Flügelprofil und Bodenbürste sind eine Rahmenkontur um das Glas; Bolzen, Muttern und Gabelkopf sind ein Aufhängeblock."),
        "omitted": text("Trolleys inside the track, glazing beads, floor guide, pins.",
                        "Laufwagen in der Schiene, Glashalteleisten, Bodenführung, Bolzen."),
    },
    200: {
        "silhouette": text("Jambs and head track as three steel bars, the leaf as a framed light pane that reaches up to the track (hangers merged) and down to 10 mm above the floor. Front and plan: the front is the end view along the track (jamb and track head, the leaf is hidden behind the jamb). The plan is the horizontal section at 1.0 m: two filled jamb squares and the leaf as one bar, the track above the cut as a thin outline; the leaf slides along the track in every pose.",
                           "Pfosten und Laufschiene als drei Stahlbalken, der Flügel als gerahmte helle Scheibe, die bis zur Schiene (Aufhängung zusammengefasst) und bis 10 mm über den Boden reicht. Vorder- und Draufsicht: Die Vorderansicht ist die Ansicht entlang der Schiene (Pfosten und Schienenkopf, der Flügel ist hinter dem Pfosten verdeckt). Die Draufsicht ist der Horizontalschnitt in 1,0 m Höhe: zwei gefüllte Pfostenquadrate und der Flügel als ein Balken, die Schiene über der Schnittebene als dünne Umrisslinie; der Flügel fährt in jeder Pose entlang der Schiene."),
        "omitted": text("Handle, trolleys (hidden in the track), brush seals, floor guide.",
                        "Griff, Laufwagen (in der Schiene verdeckt), Bürstendichtungen, Bodenführung."),
    },
    500: {
        "omitted": text("Frame profile, handle, hangers, trolleys, floor guide, the 10 mm floor clearance.",
                        "Rahmenprofil, Griff, Aufhängung, Laufwagen, Bodenführung, die 10-mm-Bodenluft."),
        "thinElements": text("The 0.1 m jambs and track (0.2 mm) are drawn 0.15 m (0.3 mm, inside the opening) so the filled U keeps a printable width; the 50 mm hanger gap is closed by letting the leaf touch the drawn head.",
                             "Die 0,1-m-Pfosten und die Schiene (0,2 mm) sind 0,15 m breit gezeichnet (0,3 mm, zur Öffnung hin), damit das gefüllte U druckbar bleibt; der 50-mm-Spalt der Aufhängung ist geschlossen, indem der Flügel den gezeichneten Kopf berührt."),
    },
}


if __name__ == "__main__":
    for arg in sys.argv[1:]:
        if arg.startswith("--area="):
            rid, value = arg[7:].split(":")
            AREAS[rid] = float(value)
    source = open(L.CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = copy.deepcopy(document)
    apply(document["asset"])
    print("track A %.7f I %.4e c %.5f kg %.2f | jambs kg %.2f | leaf kg %.2f com %.3f | total %.2f" % (TRACK_A, TRACK_I, TRACK_C, TRACK_KG, JAMBS_KG, LEAF_KG, LEAF_COM, TOTAL_KG))
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = L.member(source, key)
            source = source[:start] + L.dumped(document["asset"][key]) + source[end:]
        check = json.loads(source)
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
            if key in MEMBERS and check["asset"][key] != document["asset"][key]:
                raise SystemExit("member %s not written as intended" % key)
        with open(L.CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog members written:", ", ".join(MEMBERS))
    else:
        document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "components", "sliding-door.json"), encoding="utf-8"))["volumes"]
        folder = os.path.join(T, "🗑️generated", "r3-sliding-door")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print("candidate written")
