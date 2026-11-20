"""🌱 Round 3 thermostatic radiator valve: every root value outside `volumes` (summary, references, dimensions, rig, anchors, the 2D drawings of 1:50 and 1:100 as true views of the handcrafted body, the reasons of 1:200 and 1:500, pose and clip contacts, thermal titles and areas, supports) and the volumes of `r3-thermostatic-radiator-valve-build.py`.

Usage: python3 r3-thermostatic-radiator-valve-roots.py            writes the candidate document to 🗑️generated/r3-thermostatic-radiator-valve/🔣️.json
       python3 r3-thermostatic-radiator-valve-roots.py --apply    re-reads the catalog file, replaces the changed members and `volumes` in one write, writes the volumes source and the manifest entry
"""
import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    """🧩 Imports a sibling script of the ticket folder as a module."""
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


B = load("r3-thermostatic-radiator-valve-build")
STAFFAGE = os.path.join(T, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "🧍️staffage")
CATALOG = os.path.join(STAFFAGE, "🗂️catalog", "🔌️appliances", "thermostatic-radiator-valve", "🔣️.json")
MANIFEST = os.path.join(STAFFAGE, "🗂️catalog", "🔣️.json")
MEMBERS = ["summary", "provenance", "dimensions", "rig", "anchors", "representations", "poses", "clips", "thermal", "structure", "volumes"]
HEIGHT, WIDTH, DEPTH = 0.1065, 0.0845, 0.1103
AREAS = {"body": 0.0137, "head": 0.0125, "knob": 0.0042}
HEAD_Z, KNOB_Z = 0.0, 0.0
text, m = B.text, B.m


def pts(points):
    """📍 Millimetre points to metre points."""
    return [[m(x), m(y)] for x, y in points]


def shape(sid, bone, role, z, filled, tone, geometry, region=None, regions=None, material=None, solid=None):
    """✏️ A 2D shape with its members in the catalog's order."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geometry)
    if region is not None:
        s["region"] = region
    if regions is not None:
        s["regions"] = regions
    if material is not None:
        s["material"] = material
    if solid is not None:
        s["solid"] = solid
    return s


def poly(points):
    """📐 A closed polygon from millimetre points."""
    return {"kind": "poly", "points": pts(points), "closed": True}


def disc(radius):
    """⚪ A circle about the bone origin, radius in millimetres."""
    return {"kind": "ellipse", "cx": 0, "cy": 0, "rx": m(radius), "ry": m(radius)}


def rect(x0, y0, x1, y1):
    """▭ A rectangle from two millimetre corners."""
    return {"kind": "rect", "x": m(x0), "y": m(y0), "w": m(x1 - x0), "h": m(y1 - y0)}


def swept(z0, z1, bone_z):
    """↔️ An extrusion along the head axis between two world z (millimetres) of a bone at bone_z."""
    return {"kind": "extruded", "sweep": "w", "z0": m(z0 - bone_z), "z1": m(z1 - bone_z)}


def mirrored(points):
    """🪞 A half outline (millimetres, from bottom to top on the +x side) closed by its mirror image."""
    return points + [(-x, y) for x, y in reversed(points)]


ELEVATION_BODY_50 = [(-7.5, -80), (7.5, -80), (7.5, -46), (12.75, -46), (12.75, -19), (11, -19), (11, -13.2), (18, -13.2), (18, -15), (32, -15), (32, -10), (58, -10), (58, 10), (32, 10), (32, 15), (18, 15), (18, 13.2), (6, 13.6), (0, 15), (-10.6, 10.6), (-15, 0), (-11, -10.2), (-11, -19), (-12.75, -19), (-12.75, -46), (-7.5, -46)]
FRONT_BODY_50 = [(-7.5, -80), (7.5, -80), (7.5, -46), (14.7, -46), (14.7, -19), (11, -19), (11, -11), (17.3, 0), (8.7, 15), (-22.7, 15), (-23.5, 14.2), (-23.5, -14.2), (-22.7, -15), (-11, -15), (-11, -19), (-14.7, -19), (-14.7, -46), (-7.5, -46)]
PLAN_BODY_50 = [(-15, -23.5), (-15, 13), (-12, 15.5), (12, 15.5), (15, 13.2), (18, 17.3), (32, 17.3), (32, 10), (58, 10), (58, -10), (32, -10), (32, -17.3), (18, -17.3), (15, -13.2), (15, -23.5)]
BASE_50 = [(-25.5, -17), (-12.5, -17), (-12.5, 17), (-25.5, 17), (-25.5, 22), (-35, 22), (-35, -22), (-25.5, -22)]
ELEVATION_BODY_100 = [(-11, -80), (11, -80), (11, -46), (13, -46), (13, -19), (11, -19), (11, -13), (18, -13), (18, -16), (32, -16), (32, -11), (58, -11), (58, 11), (32, 11), (32, 16), (18, 16), (18, 13), (0, 15), (-11, 11), (-11, -19), (-13, -19), (-13, -46), (-11, -46)]
FRONT_BODY_100 = [(-11, -80), (11, -80), (11, -46), (13.5, -46), (13.5, -19), (11, -19), (11, -11.5), (16, 0), (15, 8), (11, 14), (8, 16), (-12.5, 16), (-12.5, -15), (-11, -15), (-11, -19), (-13.5, -19), (-13.5, -46), (-11, -46)]
PLAN_BODY_100 = [(-14, -12.5), (-14, 11), (-9, 15), (9, 15), (14, 13.5), (18, 16), (32, 16), (34, 11), (58, 11), (58, -11), (34, -11), (32, -16), (18, -16), (14, -13.5), (14, -12.5)]
HEAD_100 = [(-12.5, -18), (-12.5, 18), (-35, 21), (-35, 26.5), (-71, 25.5), (-88, 21.5), (-93, 15), (-93, -15), (-88, -21.5), (-71, -25.5), (-35, -26.5), (-35, -21)]


def lateral(points, bone_z):
    """🧭 Front outline points (world z, height) of a bone at bone_z in the front frame of that bone."""
    return [(z - bone_z, y) for z, y in points]


def planwise(points, bone_z):
    """🧭 Outline points given as (world z, radial) turned into the plan frame (ahead, lateral) of a bone at bone_z."""
    return [(y, z - bone_z) for z, y in points]


def drawing50():
    """🖊️ 1:50: the three true views of the 1:50 body; everything a pen of 0.18 mm can keep apart."""
    knob = dict(region="head", material="abs-plastic")
    return {
        "elevation": [
            shape("body", "body", "body", 3, True, 0.6, poly(ELEVATION_BODY_50), region="body", material="brass"),
            shape("collar", "head", "body", 3.1, True, 0.8, disc(22), region="head", material="abs-plastic"),
            shape("knob", "knob", "body", 3.2, True, 0.95, disc(26.5), regions=["knob", "head"], material="abs-plastic"),
            shape("pointer", "knob", "detail", 3.3, True, 0.1, rect(-5.5, 3.5, 5.5, 8), region="knob", material="abs-plastic"),
        ],
        "plan": [
            shape("body", "body", "body", 3, True, 0.6, poly(PLAN_BODY_50), region="body", material="brass"),
            shape("collar", "head", "body", 3.1, True, 0.8, poly(planwise(BASE_50, HEAD_Z)), region="head", material="abs-plastic"),
            shape("index", "head", "detail", 3.15, True, 0.12, rect(-5.5, -35 - HEAD_Z, 5.5, -26.5 - HEAD_Z), region="head", material="abs-plastic"),
            shape("knob", "knob", "body", 3.2, True, 0.95, disc(26.5), solid=swept(-72, -35, KNOB_Z), **knob),
            shape("cap", "knob", "body", 3.25, True, 0.95, disc(22.5), region="knob", material="abs-plastic", solid=swept(-93, -72, KNOB_Z)),
        ],
        "front": [
            shape("body", "body", "body", 3, True, 0.6, poly(FRONT_BODY_50), region="body", material="brass"),
            shape("collar", "head", "body", 3.1, True, 0.8, poly(lateral(BASE_50, HEAD_Z)), region="head", material="abs-plastic"),
            shape("knob", "knob", "body", 3.2, True, 0.95, disc(26.5), solid=swept(-72, -35, KNOB_Z), **knob),
            shape("cap", "knob", "body", 3.25, True, 0.95, disc(22.5), region="knob", material="abs-plastic", solid=swept(-93, -72, KNOB_Z)),
        ],
    }


def drawing100():
    """🖊️ 1:100: body and head as two outlines per view."""
    both = dict(regions=["head", "knob"], material="abs-plastic")
    return {
        "elevation": [
            shape("body", "body", "body", 3, True, 0.6, poly(ELEVATION_BODY_100), region="body", material="brass"),
            shape("head", "head", "body", 3.1, True, 0.8, disc(26.5), **both),
        ],
        "plan": [
            shape("body", "body", "body", 3, True, 0.6, poly(PLAN_BODY_100), region="body", material="brass"),
            shape("head", "head", "body", 3.1, True, 0.8, poly(planwise(HEAD_100, HEAD_Z)), **both),
        ],
        "front": [
            shape("body", "body", "body", 3, True, 0.6, poly(FRONT_BODY_100), region="body", material="brass"),
            shape("head", "head", "body", 3.1, True, 0.8, poly(lateral(HEAD_100, HEAD_Z)), **both),
        ],
    }


DECISIONS = {
    "50": {
        "silhouette": text(
            "A corner angle thermostatic valve seen along the axis of its head, 2 mm tall on paper: the round handwheel of 53 mm in front of the valve body, the pipe with its fitting below and the union with the tailpiece towards the radiator on the right. Front view (seen from the radiator): the body with the hexagon of the union nut on the right, the head lying 80 mm to the left of it as base, handwheel and sensor cap. Top view: the same three head parts below the body, whose outlet arm reaches 58 mm to the right.",
            "Ein Thermostat-Winkeleckventil in Richtung der Kopfachse gesehen, auf Papier 2 mm hoch: das runde Handrad von 53 mm vor dem Ventilkörper, darunter das Rohr mit seiner Verschraubung und rechts die Verschraubung mit dem Schraubnippel zum Heizkörper. Vorderansicht (vom Heizkörper gesehen): der Körper mit dem Sechskant der Überwurfmutter rechts, der Kopf 80 mm links daneben als Sockel, Handrad und Fühlerkappe. Draufsicht: dieselben drei Kopfteile unter dem Körper, dessen Ausgangsarm 58 mm nach rechts reicht."),
        "retained": text(
            "Handwheel of 53 mm with its front mark of 11 mm, the base with the index mark in the top view, the sensor cap, the valve body with pipe, fitting, union nut and tailpiece.",
            "Handrad von 53 mm mit seiner 11 mm breiten Frontmarke, der Sockel mit der Indexmarke in der Draufsicht, die Fühlerkappe, der Ventilkörper mit Rohr, Verschraubung, Überwurfmutter und Schraubnippel."),
        "merged": text(
            "All brass parts are one outline carrying the body region; compression nut and inlet socket are one collar; the coupling nut and the base ring are one stepped outline carrying the head region; in the side elevation the handwheel disc carries the head and the knob region because the grip rim around the cap is 2.5 mm wide.",
            "Alle Messingteile sind eine Kontur, die die Körperregion trägt; Klemmmutter und Eingangsmuffe sind ein Bund; Überwurfmutter und Sockelring sind eine gestufte Kontur, die die Kopfregion trägt; in der Seitenansicht trägt die Handradscheibe die Kopf- und die Knopfregion, weil der Griffrand um die Kappe 2,5 mm breit ist."),
        "omitted": text(
            "The scale numerals, the ten finger flutes, the two slot rings of the sensor cap, the thread run-outs and the chamfers. The side elevation shows nothing of the base ring, which hides behind the handwheel; front and top view omit the front mark on the end face.",
            "Die Skalenziffern, die zehn Griffmulden, die beiden Schlitzringe der Fühlerkappe, die Gewindeausläufe und die Fasen. Die Seitenansicht zeigt nichts vom Sockelring, der hinter dem Handrad liegt; Vorder- und Draufsicht lassen die Frontmarke auf der Stirnfläche weg."),
        "thinElements": text(
            "Front mark and index mark are 11 mm wide (0.22 mm on paper) and filled, the index arrow of 6 mm is widened to that; the pipe is 15 mm wide (0.3 mm).",
            "Frontmarke und Indexmarke sind 11 mm breit (0,22 mm auf Papier) und gefüllt, der 6 mm breite Indexpfeil ist darauf verbreitert; das Rohr ist 15 mm breit (0,3 mm)."),
        "hatching": text("None.", "Keine."),
        "styles": text(
            "Body 0.6, base 0.8, handwheel and cap 0.95, front mark and index 0.1; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.",
            "Körper 0,6, Sockel 0,8, Handrad und Kappe 0,95, Frontmarke und Index 0,1; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": text(
            "Three regions: the brass body carries the supply water temperature all the time, the head (coupling nut, base ring and handwheel barrel) and the knob (sensor cap) warm only while hot water flows through the open valve; handwheel, cap and front mark turn with the knob bone.",
            "Drei Regionen: der Messingkörper trägt dauernd die Vorlauftemperatur, der Kopf (Überwurfmutter, Sockelring und Handradtrommel) und der Knopf (Fühlerkappe) erwärmen sich nur, solange heißes Wasser durch das offene Ventil fließt; Handrad, Kappe und Frontmarke drehen sich mit dem Knopfknochen."),
    },
    "100": {
        "silhouette": text(
            "A valve 1 mm tall on paper: the round head in front of a body with the pipe below and the outlet arm to the right. Front view: the body and the tapered head to its left. Top view: the same head below the body with its outlet arm.",
            "Ein auf Papier 1 mm hohes Ventil: der runde Kopf vor einem Körper mit dem Rohr darunter und dem Ausgangsarm nach rechts. Vorderansicht: der Körper und der verjüngte Kopf links daneben. Draufsicht: derselbe Kopf unter dem Körper mit seinem Ausgangsarm."),
        "retained": text("Head, valve body, pipe and outlet arm.", "Kopf, Ventilkörper, Rohr und Ausgangsarm."),
        "merged": text(
            "Coupling nut, base ring, handwheel and sensor cap are one head outline carrying the head and the knob region; fitting, socket and neck are one column, union nut and tailpiece one arm.",
            "Überwurfmutter, Sockelring, Handrad und Fühlerkappe sind eine Kopfkontur, die die Kopf- und die Knopfregion trägt; Verschraubung, Muffe und Hals sind eine Säule, Überwurfmutter und Schraubnippel ein Arm."),
        "omitted": text(
            "Front mark and index mark: the turning of the handwheel is not drawn at this scale; hexagons, steps and chamfers.",
            "Frontmarke und Indexmarke: das Drehen des Handrads wird bei diesem Maßstab nicht gezeichnet; Sechskante, Stufen und Fasen."),
        "thinElements": text(
            "The pipe of 15 mm and the tailpiece of 21 mm are widened to 22 mm (0.22 mm on paper).",
            "Das Rohr von 15 mm und der Schraubnippel von 21 mm sind auf 22 mm (0,22 mm auf Papier) verbreitert."),
        "hatching": text("None.", "Keine."),
        "styles": text("Body 0.6, head 0.8; one body pen of 0.18 mm.", "Körper 0,6, Kopf 0,8; ein Körperstift von 0,18 mm."),
        "overlays": text("The head outline carries the head and the knob region; the body keeps its own.", "Die Kopfkontur trägt Kopf- und Knopfregion; der Körper behält seine eigene."),
    },
}


def anchor(aid, x, y, title, support, normal, length, width, **extra):
    """⚓ A support anchor on the body bone in the catalog's member order."""
    a = {"id": aid, "bone": "body", "x": x, "y": y, "kind": "support", "title": title, "support": support, "normalDeg": normal, "footprintLengthM": length, "footprintWidthM": width}
    a.update(extra)
    return a


def bone(old, title, length, kg, com_x, com_y, y=0, z=None):
    """🦴 A bone with corrected joint, title, length and mass, members in the catalog's order."""
    out = {"id": old["id"], "parent": old["parent"], "x": 0, "y": y}
    if z is not None:
        out["z"] = z
    out.update({"angleDeg": 0, "title": title, "lengthM": length, "joint": old["joint"]})
    mass = dict(old["mass"])
    mass.update({"kg": kg, "comX": com_x, "comY": com_y})
    out["mass"] = mass
    return out


def apply(asset):
    """🛠️ Writes every corrected root value and the volumes into the asset."""
    asset["summary"] = text(
        "A corner angle thermostatic radiator valve DN 15 seen along the axis of its head: the brass body with the pipe coming up from below and the union with the tailpiece towards the radiator, and a thermostat head of 53 mm whose handwheel turns from the star setting to setting 5, with the flow and the head warmth following it.",
        "Ein Thermostat-Winkeleckventil DN 15 in Richtung der Kopfachse gesehen: der Messingkörper mit dem von unten kommenden Rohr und der Verschraubung mit Schraubnippel zum Heizkörper sowie ein Thermostatkopf von 53 mm, dessen Handrad von der Frostschutzstufe auf Stufe 5 dreht, wobei Durchfluss und Kopfwärme ihm folgen.")
    provenance = asset["provenance"]
    kept = [r for r in provenance["references"] if r["id"] not in ("imi-heimeier-v-exact-ii", "imi-heimeier-thermostatic-head-k", "en-1254-2-1998")]
    provenance["references"] = kept + [
        {"id": "imi-heimeier-v-exact-ii", "citation": "IMI Hydronic Engineering: IMI Heimeier V-exact II thermostatic valve bodies with presetting, technical data sheet (corner angle pattern, Winkeleck, DN 15, articles 3713-02.000 and 3714-02.000: inlet Rp 1/2, tailpiece R 1/2, l3 = 29 mm from the centre to the inlet face, l4 = 58 mm from the centre to the end of the tailpiece; angle pattern DN 15: H1 = 23.5 mm from the centre to the contact face of the head; spanner widths 27 mm at the body and 30 mm at the union nut; head connection M30 x 1.5).", "url": "https://documents.unidomo.de/de/v-exact-ii_in_de_low.pdf"},
        {"id": "imi-heimeier-thermostatic-head-k", "citation": "IMI Hydronic Engineering: IMI Heimeier thermostatic head K (6000-00.500), product data and dimension drawing (liquid-filled sensor, knurled coupling nut M30 x 1.5, diameter 53 mm, length about 80 mm, settings star and 1 to 5 with tactile marks).", "url": "https://www.imi-hydronic.com/en-gb/product/thermostatic-head-k"},
        {"id": "en-1254-2-1998", "citation": "EN 1254-2:1998 Copper and copper alloys - Plumbing fittings - Part 2: Fittings with compression ends for use with copper tubes (15 mm compression end with nut and olive on the 15 x 1 mm tube of EN 1057)."},
    ]
    provenance["notes"] = text(
        "A nickel-plated brass corner angle valve (Winkeleck) DN 15 with a liquid-filled white ABS thermostat head on the M30 x 1.5 connection, seen along the head axis as from the room towards the wall beside the end of a radiator: the 15 mm pipe comes up from below through a compression fitting into the Rp 1/2 inlet 29 mm under the centre, the tailpiece ends 58 mm to the right of the centre in the radiator, and the head stands 23.5 mm in front of the centre and is 80 mm long and 53 mm across (body sizes from the V-exact II data sheet, head sizes from the head K data). The origin is the point where the three axes of the body meet. The fixed base ring carries the index mark; the handwheel with the sensor cap turns from the star setting (0 degrees, frost protection) through settings 1 to 5 at 54 degrees each (270 degrees); positive angles turn counter-clockwise, which opens the valve. The body stays at the supply water temperature the whole time because the water stands in it even when the seat is closed (3 W of loss at 21 degrees Celsius room air), while the head and the sensor cap warm only with the flow through the seat (0.8 W and 0.4 W at full flow, 60 percent at setting 3). Masses, heat flows and loads are authored estimates for this product class.",
        "Ein Winkeleckventil DN 15 aus vernickeltem Messing mit flüssigkeitsgefülltem weißem ABS-Thermostatkopf am Anschluss M30 x 1,5, in Richtung der Kopfachse gesehen wie aus dem Raum zur Wand neben dem Ende eines Heizkörpers: das 15-mm-Rohr kommt von unten durch eine Klemmverschraubung in den Eingang Rp 1/2 29 mm unter der Mitte, der Schraubnippel endet 58 mm rechts der Mitte im Heizkörper, und der Kopf steht 23,5 mm vor der Mitte und ist 80 mm lang und 53 mm dick (Körpermaße aus dem Datenblatt V-exact II, Kopfmaße aus den Daten des Kopfes K). Der Ursprung ist der Punkt, in dem sich die drei Achsen des Körpers treffen. Der feste Sockelring trägt die Indexmarke; das Handrad mit der Fühlerkappe dreht von der Frostschutzstufe (0 Grad) über die Stufen 1 bis 5 zu je 54 Grad (270 Grad); positive Winkel drehen gegen den Uhrzeigersinn und öffnen das Ventil. Der Körper bleibt dauernd auf Vorlauftemperatur, weil das Wasser auch bei geschlossenem Sitz in ihm steht (3 W Verlust bei 21 Grad Celsius Raumluft), während Kopf und Fühlerkappe sich nur mit dem Durchfluss durch den Sitz erwärmen (0,8 W und 0,4 W bei vollem Durchfluss, 60 Prozent auf Stufe 3). Massen, Wärmeströme und Lasten sind Schätzwerte für diese Produktklasse.")
    asset["dimensions"].update({"heightM": HEIGHT, "widthM": WIDTH, "depthM": DEPTH})
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    asset["rig"]["bones"] = [
        bone(bones["body"], text("Corner angle valve body with pipe and tailpiece", "Winkeleckventilkörper mit Rohr und Schraubnippel"), 0.058, 0.26, 0.009, -0.015),
        bone(bones["head"], text("Coupling nut and base ring with index mark", "Überwurfmutter und Sockelring mit Indexmarke"), 0.022, 0.03, 0, 0),
        bone(bones["knob"], text("Handwheel with sensor cap", "Handrad mit Fühlerkappe"), 0.0265, 0.09, 0, 0),
    ]
    asset["anchors"] = [
        anchor("pipe-joint", 0, -0.039, text("Compression fitting on the pipe", "Klemmverschraubung am Rohr"), "pin", 90, 0.015, 0.015),
        anchor("radiator-union", 0.051, 0, text("Tailpiece screwed into the radiator", "Schraubnippel im Heizkörper"), "roller", 90, 0.014, 0.021, frictionCoefficient=0.4),
    ]
    reps = asset["representations"]
    reps["50"]["geometry"] = drawing50()
    reps["100"]["geometry"] = drawing100()
    for scale in ("50", "100"):
        reps[scale]["decisions"] = DECISIONS[scale]
    reps["200"]["reason"] = text(
        "At 1:200 the 0.107 m valve is 0.53 mm tall and its pipe 0.08 mm wide, below every print profile; use the 1:100 drawing at this scale.",
        "Bei 1:200 ist das 0,107 m hohe Ventil 0,53 mm hoch und sein Rohr 0,08 mm breit, unter jedem Druckprofil; bei diesem Maßstab dient die Zeichnung 1:100.")
    reps["500"]["reason"] = text(
        "At 1:500 the 0.107 m valve is 0.21 mm tall and its pipe 0.03 mm wide, below every print profile; use the 1:100 drawing at this scale.",
        "Bei 1:500 ist das 0,107 m hohe Ventil 0,21 mm hoch und sein Rohr 0,03 mm breit, unter jedem Druckprofil; bei diesem Maßstab dient die Zeichnung 1:100.")
    for scale in ("200", "500"):
        reps[scale]["limitingFeature"] = text("Pipe of 15 mm diameter", "Rohr von 15 mm Durchmesser")
        reps[scale]["limitingSizeM"] = 0.015
    for pose in asset["poses"]:
        pose["contacts"] = ["pipe-joint", "radiator-union"]
    asset["poses"][0]["description"] = text(
        "The handwheel stands at setting 5 (270 degrees): hot water flows through the valve and warms the head and the sensor cap a little.",
        "Das Handrad steht auf Stufe 5 (270 Grad): heißes Wasser fließt durch das Ventil und erwärmt Kopf und Fühlerkappe ein wenig.")
    asset["poses"][1]["description"] = text(
        "The handwheel stands at setting 3 (162 degrees): about 60 percent of the water flows.",
        "Das Handrad steht auf Stufe 3 (162 Grad): etwa 60 Prozent des Wassers fließen.")
    asset["poses"][2]["description"] = text(
        "The handwheel stands at the star setting (0 degrees): no water flows, the head cools to the room temperature while the body stays at the supply temperature.",
        "Das Handrad steht auf der Frostschutzstufe (0 Grad): kein Wasser fließt, der Kopf kühlt auf Raumtemperatur ab, während der Körper auf Vorlauftemperatur bleibt.")
    for clip in asset["clips"]:
        for contact, anchor_id in zip(clip["contacts"], ("pipe-joint", "radiator-union")):
            contact["anchor"] = anchor_id
        clip["events"][0]["title"] = text("Handwheel starts to turn", "Handrad beginnt zu drehen")
    asset["clips"][0]["description"] = text(
        "The handwheel rests at setting 5 for one second, turns down to the star setting between 1 and 7 seconds and rests there; the flow through the valve falls with it and the head cools.",
        "Das Handrad ruht eine Sekunde auf Stufe 5, dreht zwischen 1 und 7 Sekunden auf die Frostschutzstufe und ruht dort; der Durchfluss fällt mit, und der Kopf kühlt ab.")
    asset["clips"][1]["description"] = text(
        "The valve is closed for one second, the handwheel turns up to setting 5 between 1 and 7 seconds and the flow and the head warmth rise with it.",
        "Das Ventil ist eine Sekunde geschlossen, das Handrad dreht zwischen 1 und 7 Sekunden auf Stufe 5, und Durchfluss und Kopfwärme steigen mit.")
    th = asset["thermal"]
    titles = {
        "body": text("Valve body, unions and pipe", "Ventilkörper, Verschraubungen und Rohr"),
        "head": text("Thermostat head: coupling nut, base ring and handwheel barrel", "Thermostatkopf: Überwurfmutter, Sockelring und Handradtrommel"),
        "knob": text("Sensor cap of the handwheel", "Fühlerkappe des Handrads"),
    }
    lengths = {"body": 0.12, "head": 0.053, "knob": 0.045}
    for region in th["regions"]:
        region["title"] = titles[region["id"]]
        region["areaM2"] = AREAS[region["id"]]
        region["characteristicLengthM"] = lengths[region["id"]]
    th["regions"][2]["layers"][0]["title"] = text("ABS cap 3 mm", "ABS-Kappe 3 mm")
    th["assumptions"][2] = text(
        "The head region (coupling nut, base ring and handwheel barrel) and the knob region (sensor cap) receive their heat only by conduction from the valve seat while water flows, so they follow the flow level and not the supply level; the knurled coupling nut is part of the head and is counted with it; the liquid-filled sensor inside the cap is not modelled.",
        "Die Kopfregion (Überwurfmutter, Sockelring und Handradtrommel) und die Knopfregion (Fühlerkappe) erhalten ihre Wärme nur durch Leitung vom Ventilsitz, solange Wasser fließt, folgen also dem Durchflussniveau und nicht dem Vorlauf; die gerändelte Überwurfmutter ist Teil des Kopfes und wird mit ihm gezählt; der flüssigkeitsgefüllte Fühler in der Kappe wird nicht nachgebildet.")
    st = asset["structure"]
    st["supports"] = ["pipe-joint", "radiator-union"]
    st["assumptions"][0] = text(
        "The 0.38 kg valve is carried by its two connections: the compression fitting on the pipe (pin) and the tailpiece screwed into the radiator (roller with a vertical reaction). The 80 mm head overhangs towards the room across the plane of the model; its small torsion on the pipe is not part of the plane analysis.",
        "Das 0,38 kg schwere Ventil wird von seinen beiden Anschlüssen getragen: der Klemmverschraubung am Rohr (Festlager) und dem in den Heizkörper geschraubten Schraubnippel (Rollenlager mit senkrechter Reaktion). Der 80 mm lange Kopf kragt quer zur Modellebene in den Raum aus; seine kleine Torsion am Rohr gehört nicht zur ebenen Berechnung.")
    asset["volumes"] = B.volumes()
    return asset


def member(source, key):
    """🔎 The character range of the value of a top-level member of the asset in the catalog text."""
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = source[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def dumped(value):
    """🖨️ The plain two-space layout of the catalog for a member of the asset."""
    lines = re.sub(r"(\d)e-0(\d)(?=,?$)", r"\1e-\2", json.dumps(value, indent=2, ensure_ascii=False), flags=re.M).split("\n")
    return "\n".join(line if i == 0 else "    " + line for i, line in enumerate(lines))


def inline(value):
    """🧵 Whether volumes-insert.py writes a list on one line (scalars, or short point lists)."""
    scalar = lambda v: not isinstance(v, (dict, list))
    return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)


def spliced(value, depth):
    """🪡 The layout volumes-insert.py gives the `volumes` member."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict) and value:
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + spliced(item, depth + 1) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list) and value and not inline(value):
        return "[\n" + ",\n".join(inner + spliced(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def manifest_entry(source):
    """📇 The manifest text with the entry of this asset corrected (height, width, distinction)."""
    start = source.index('"id": "thermostatic-radiator-valve"')
    end = source.index('"completion"', start)
    entry = source[start:end]
    entry = re.sub(r'"heightM": [0-9.]+', '"heightM": %s' % HEIGHT, entry)
    entry = re.sub(r'"widthM": [0-9.]+', '"widthM": %s' % WIDTH, entry)
    en = "A brass corner angle valve with a 53 mm thermostat head seen along its axis whose handwheel turns from the star setting to setting 5: the body stays at the hot supply temperature while the head and the sensor cap warm only with the flow."
    de = "Ein Winkeleckventil aus Messing mit 53 mm dickem Thermostatkopf in Achsrichtung gesehen, dessen Handrad von der Frostschutzstufe auf Stufe 5 dreht: der Körper bleibt auf heißer Vorlauftemperatur, während sich Kopf und Fühlerkappe nur mit dem Durchfluss erwärmen."
    entry = re.sub(r'("distinction": \{\s*"en": )"[^"]*"(,\s*"de": )"[^"]*"', lambda g: g.group(1) + json.dumps(en, ensure_ascii=False) + g.group(2) + json.dumps(de, ensure_ascii=False), entry)
    return source[:start] + entry + source[end:]


if __name__ == "__main__":
    source = open(CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = json.loads(source)
    apply(document["asset"])
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = member(source, key)
            text_new = spliced(document["asset"][key], 2) if key == "volumes" else dumped(document["asset"][key])
            source = source[:start] + text_new + source[end:]
        check = json.loads(source)
        if check != document:
            raise SystemExit("the written text does not parse to the candidate, not written")
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        with open(B.SOURCE, "w", encoding="utf-8") as f:
            json.dump({"volumes": document["asset"]["volumes"]}, f, ensure_ascii=False, indent=1)
            f.write("\n")
        manifest = open(MANIFEST, encoding="utf-8").read()
        corrected = manifest_entry(manifest)
        if corrected != manifest:
            with open(MANIFEST, "w", encoding="utf-8") as f:
                f.write(corrected)
        print("catalog members written:", ", ".join(MEMBERS))
    else:
        folder = os.path.join(T, "🗑️generated", "r3-thermostatic-radiator-valve")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print("candidate written", document["asset"]["dimensions"])
