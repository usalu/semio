"""🍳 Hand-authored rework of `appliances/kitchen-extractor-hood` (pass 1): states every hand-chosen number of the coarse scales and the patches of 1:50, the rig and the 2D drawings.

Usage: python3 rework-kitchen-extractor-hood-model.py candidate|publish
`candidate` writes the source `volumes/appliances/kitchen-extractor-hood.json` and the candidate document `🗑️generated/rework-kitchen-extractor-hood/🔣️.json`; `publish` re-reads the catalog file and writes rig, 2D and volumes into it.
Reference: Bosch DWP96BC50B data sheet (see `provenance.references` of the asset).
"""
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/appliances/kitchen-extractor-hood.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/kitchen-extractor-hood/🔣️.json"
CANDIDATE = os.path.join(T, "🗑️generated/rework-kitchen-extractor-hood/🔣️.json")

SS, GALV, MESH, LAMP = "stainless-steel", "galvanised-steel", "filter-mesh", "lamp-diffuser"
DUCT_U, DUCT_R = 0.085, 0.075
WHEEL = (0.135, 0.08)
SLOPE_FRONT = {"translation": [0.5, 0.045, 0.0], "rotation": [0.0, 0.0, 0.9567575, 0.2908866]}
SLOPE_LEFT = {"translation": [0.0, 0.045, 0.45], "rotation": [-0.4837869, 0.0, 0.0, 0.8751858]}
SLOPE_RIGHT = {"translation": [0.0, 0.045, -0.45], "rotation": [0.4837869, 0.0, 0.0, 0.8751858]}
RIM = [[-0.45, 0.0], [0.45, 0.0], [0.45, 0.5], [-0.45, 0.5]]
FILTER_FIELD = [[-0.428, 0.04], [0.428, 0.04], [0.428, 0.36], [-0.428, 0.36]]
LAMP_HOLES = [[[-0.26, 0.4], [-0.14, 0.4], [-0.14, 0.45], [-0.26, 0.45]], [[0.14, 0.4], [0.26, 0.4], [0.26, 0.45], [0.14, 0.45]]]
VOLUTE = [[0.205, 0.05]] + [[round(WHEEL[0] + (0.0825 + 0.0475 * k / 12) * math.cos(math.radians(22.5 * k)), 4), round(WHEEL[1] + (0.0825 + 0.0475 * k / 12) * math.sin(math.radians(22.5 * k)), 4)] for k in range(13)] + [[0.231, -0.05], [0.231, 0.05]]


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
    return False


def dump(value, depth):
    """🖨️ The catalog file format (same writer as `volumes-insert.py`)."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def circle(cu, cv, r, n):
    """⭕️ A regular n-gon of circumradius r about (cu, cv)."""
    return [[round(cu + r * math.cos(2 * math.pi * k / n), 5), round(cv + r * math.sin(2 * math.pi * k / n), 5)] for k in range(n)]


def solid(id, bone, material, role, tone, kind, **fields):
    """🧱 One solid with the common members first."""
    return {"id": id, "bone": bone, "material": material, "role": role, "tone": tone, "kind": kind, **fields}


def convex(id, bone, material, tone, vertices, quads):
    """💎 A convex polyhedron from hand-listed corners and faces (three or four corners), each face wound outward here."""
    centre = [sum(v[i] for v in vertices) / len(vertices) for i in range(3)]
    triangles = []
    for quad in quads:
        a, b, c = (vertices[i] for i in quad[:3])
        n = [(b[1] - a[1]) * (c[2] - a[2]) - (b[2] - a[2]) * (c[1] - a[1]), (b[2] - a[2]) * (c[0] - a[0]) - (b[0] - a[0]) * (c[2] - a[2]), (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])]
        out = sum(n[i] * (a[i] - centre[i]) for i in range(3)) > 0
        q = list(quad) if out else list(reversed(quad))
        triangles += [[q[0], q[1], q[2]]] + ([[q[0], q[2], q[3]]] if len(q) == 4 else [])
    return solid(id, bone, material, "body", tone, "polyhedron", vertices=vertices, triangles=triangles)


def ref(id, of, **fields):
    """🔗 A reference to a body solid."""
    return {"id": id, "of": of, **fields}


def th(s, region, layer=None, **fields):
    """🌡️ Thermal members on an own solid."""
    return {**s, "region": region, **({"layer": layer} if layer else {}), **fields}


def st(s, member, part, **fields):
    """🏗️ Structural members on an own solid."""
    return {**s, "member": member, "part": part, **fields}


def text(en, de):
    """🗣️ One bilingual decision."""
    return {"en": en, "de": de}


def leaves(thickness):
    """🚪 The two backdraft leaves as half-octagon plates of the scale's feature thickness, structure only (they are hidden in the duct)."""
    half = [[0.0, -0.072], [0.051, -0.051], [0.072, 0.0], [0.051, 0.051], [0.0, 0.072]]
    return [
        st(solid("member-damper-left-leaf", "damper-left", SS, "body", 0.6, "extrude", profile=half, plane="zx", **{"from": -thickness / 2, "to": thickness / 2}), "damper-left", "member"),
        st(solid("member-damper-right-leaf", "damper-right", SS, "body", 0.6, "extrude", profile=[[-w, u] for w, u in half], plane="zx", **{"from": -thickness / 2, "to": thickness / 2}), "damper-right", "member"),
    ]


def by_id(items):
    return {item["id"]: item for item in items}


ALL_FACES = ["+u", "-u", "+v", "-v", "+w", "-w"]
WEDGE_FACES = [[0, 1, 2, 3], [0, 1, 4, 5], [2, 3, 5, 4], [1, 2, 4], [0, 3, 5]]


def pyramid():
    """🔺 The pyramid as four convex masses that meet in the hip planes, so both hips and the top plate edge are real edges: a wedge under each sloping sheet and the core under the top plate."""
    return [
        convex("pyramid-front", "canopy", SS, 0.84, [[0.5, 0.045, -0.45], [0.5, 0.045, 0.45], [0.2, 0.246, 0.13], [0.2, 0.246, -0.13], [0.2, 0.045, 0.13], [0.2, 0.045, -0.13]], WEDGE_FACES),
        convex("pyramid-left", "canopy", SS, 0.8, [[0.5, 0.045, 0.45], [0.0, 0.045, 0.45], [0.0, 0.246, 0.13], [0.2, 0.246, 0.13], [0.0, 0.045, 0.13], [0.2, 0.045, 0.13]], WEDGE_FACES),
        convex("pyramid-right", "canopy", SS, 0.8, [[0.5, 0.045, -0.45], [0.0, 0.045, -0.45], [0.0, 0.246, -0.13], [0.2, 0.246, -0.13], [0.0, 0.045, -0.13], [0.2, 0.045, -0.13]], WEDGE_FACES),
        solid("pyramid-core", "canopy", SS, "body", 0.86, "box", size=[0.2, 0.201, 0.26], transform={"translation": [0.1, 0.1455, 0.0]}),
    ]


PYRAMID_SHELL = [ref("th-pyramid-" + part, "pyramid-" + part, region="canopy", layer="shell", sectionFaces=["-v"]) for part in ("front", "left", "right", "core")]


def scale_50(old):
    """🔬 1:50: the as-built sheet model stays; the spigot, the two-part duct and true fan masses are added."""
    body = [s for s in old["body"] if s["id"] not in ("duct", "duct-lower", "duct-spigot")]
    at = next(i for i, s in enumerate(body) if s["id"] == "blower-scroll")
    body[at:at] = [
        solid("duct-spigot", "canopy", GALV, "body", 0.4, "revolve", profile=[[0.246, 0.0725], [0.319, 0.0725], [0.319, 0.0745], [0.246, 0.0745], [0.246, 0.0725]], axis=[0, 1, 0], origin=[DUCT_U, 0.0, 0.0]),
        solid("duct-lower", "chimney", GALV, "body", 0.55, "revolve", profile=[[0.0, 0.0745], [0.4, 0.0745], [0.4, 0.075], [0.0, 0.075], [0.0, 0.0745]], axis=[0, 1, 0], origin=[DUCT_U, 0.0, 0.0]),
        solid("duct", "chimney-top", GALV, "body", 0.55, "revolve", profile=[[-0.2, 0.0752], [0.269, 0.0752], [0.269, 0.0758], [-0.2, 0.0758], [-0.2, 0.0752]], axis=[0, 1, 0], origin=[DUCT_U, 0.0, 0.0]),
    ]
    structure = []
    for s in old["structure"]:
        if s["id"] == "mass-fan-volute":
            s = ref("mass-fan-volute", "blower-scroll", member="canopy", part="mass")
        elif s["id"] == "mass-fan-motor":
            s = ref("mass-fan-motor", "blower-motor", member="canopy", part="mass")
        elif s["id"] == "mass-fan-outlet":
            s = ref("mass-fan-outlet", "blower-transition", member="canopy", part="mass")
        structure.append(s)
    decisions = [
        text("Body: a 0.9 m pyramid chimney hood as built from 0.6 mm sheet. The canopy is the folded rim band of 45 mm with its 12 mm return flange, three sloping sheets (front and two sides) that meet in two hips, the top plate of 0.26 m by 0.2 m with the 150 mm outlet and the back sheet on the wall; every sloping sheet is a trapezoid in its own plane, so faces and hips are exact.", "Körper: eine 0,9 m breite Pyramiden-Kaminhaube, wie sie aus 0,6-mm-Blech gebaut ist. Der Haubenkörper besteht aus dem gekanteten Randband von 45 mm mit 12 mm Umbug, drei geneigten Blechen (Front und zwei Seiten), die sich in zwei Graten treffen, der Deckplatte von 0,26 m mal 0,2 m mit dem 150-mm-Auslass und dem Rückblech an der Wand; jedes geneigte Blech ist ein Trapez in seiner eigenen Ebene, sodass Flächen und Grate exakt sind."),
        text("Underside, 6 mm above the rim edge: the base panel with three grease filter cassettes of 0.28 m by 0.32 m (frame, mesh pack, latch) and two lamp modules of 0.12 m by 0.05 m in the front strip. The switch panel with four buttons sits on the front rim.", "Unterseite, 6 mm über der Randkante: das Bodenblech mit drei Fettfilterkassetten von 0,28 m mal 0,32 m (Rahmen, Gitterpaket, Riegel) und zwei Lampenmodulen von 0,12 m mal 0,05 m im vorderen Streifen. Die Schalterblende mit vier Tasten sitzt am vorderen Rand."),
        text("Inside: the scroll housing of the centrifugal blower with its inlet ring towards the wall, the forward-curved wheel of 134 mm with 24 vanes on its own bone, the external-rotor motor on the room side, the transition to the round outlet, the 73 mm duct spigot above the top plate and the two backdraft leaves on their spindle inside it, each on its own bone.", "Innen: das Spiralgehäuse des Radialgebläses mit Einlaufring zur Wand, das vorwärtsgekrümmte Laufrad von 134 mm mit 24 Schaufeln auf eigenem Knochen, der Außenläufermotor zur Raumseite, das Übergangsstück zum runden Auslass, der 73 mm hohe Rohrstutzen über der Deckplatte und darin die zwei Rückstauklappen auf ihrer Achse, jede auf eigenem Knochen."),
        text("The chimney is two U channels of sheet, open to the wall, with their return lips; the upper channel is 6 mm narrower, slides 146 mm into the lower one, carries the louvre slots and hangs on the wall bracket. The duct is two pieces like the chimney: the lower pipe stands on the spigot, the upper pipe sleeves over it and follows the upper channel when the telescope joint slides between 0.799 m and 0.975 m total height.", "Der Kamin besteht aus zwei zur Wand offenen U-Schächten aus Blech mit ihren Umbügen; der obere Schacht ist 6 mm schmaler, steckt 146 mm im unteren, trägt die Lüftungsschlitze und hängt am Wandwinkel. Das Rohr ist wie der Kamin zweiteilig: das untere Rohr steht auf dem Stutzen, das obere ist darübergeschoben und folgt dem oberen Schacht, wenn das Teleskopgelenk zwischen 0,799 m und 0,975 m Gesamthöhe gleitet."),
        text("Thermal: the canopy shell is every outer sheet with its inner face marked as section, the cassettes and lamp modules are the hidden regions under it, the chimney is both channels with the leaves, the outlet disc at the duct mouth is the extract emitter and the motor is the heat source inside the canopy.", "Thermisch: Die Haubenschale ist jedes Außenblech, dessen Innenseite als Schnittfläche markiert ist, Kassetten und Lampenmodule sind die verdeckten Regionen darunter, der Kamin sind beide Schächte mit den Klappen, die Scheibe an der Rohrmündung ist der Abluft-Emitter und der Motor die Wärmequelle im Haubenkörper."),
        text("Structure: the load path of a hung sheet box. The back sheet hangs on two keyhole screws and bears on two spacer pads; rim band, top plate and the two folded hips stiffen it; the chimney channels stand on the top plate (seat joint), slide in each other (telescope joint) and are held by the wall bracket. The masses are the scroll housing, the motor and the transition themselves, the wheel on its bone and the two chimney channels.", "Tragwerk: der Lastpfad eines aufgehängten Blechkastens. Das Rückblech hängt an zwei Schlüssellochschrauben und stützt sich auf zwei Distanzpuffer; Randband, Deckplatte und die zwei gekanteten Grate steifen es aus; die Kaminschächte stehen auf der Deckplatte (Sitzfuge), stecken ineinander (Teleskopfuge) und werden vom Wandwinkel gehalten. Die Massen sind Spiralgehäuse, Motor und Übergangsstück selbst, das Laufrad auf seinem Knochen und die zwei Kaminschächte."),
    ]
    return {**old, "body": body, "structure": structure, "decisions": decisions}


def scale_100(old):
    """📏 1:100, feature 15 mm: sheets as 15 mm plates towards the hidden inside, one filter field, the blower as housing, wheel drum and motor."""
    keep = by_id(old["structure"])
    channel = [[-0.125, 0.0], [-0.125, 0.19], [0.125, 0.19], [0.125, 0.0], [0.11, 0.0], [0.11, 0.175], [-0.11, 0.175], [-0.11, 0.0]]
    body = [
        solid("rim", "canopy", SS, "body", 0.78, "extrude", profile=RIM, plane="zx", **{"from": 0.0, "to": 0.045}, holes=[FILTER_FIELD] + LAMP_HOLES),
        solid("filter-field", "canopy", MESH, "body", 0.5, "box", size=[0.32, 0.015, 0.856], transform={"translation": [0.2, 0.0135, 0.0]}),
        solid("lamp-left", "canopy", LAMP, "glazing", 0.97, "box", size=[0.05, 0.015, 0.12], transform={"translation": [0.425, 0.0115, 0.2]}),
        solid("lamp-right", "canopy", LAMP, "glazing", 0.97, "box", size=[0.05, 0.015, 0.12], transform={"translation": [0.425, 0.0115, -0.2]}),
        solid("slope-front", "canopy", SS, "body", 0.84, "extrude", profile=[[-0.45, 0.0], [0.45, 0.0], [0.13, 0.36111], [-0.13, 0.36111]], plane="zx", **{"from": 0.0, "to": 0.015}, transform=SLOPE_FRONT),
        solid("slope-left", "canopy", SS, "body", 0.8, "extrude", profile=[[0.0, 0.0], [0.5, 0.0], [0.2, 0.37789], [0.0, 0.37789]], plane="xy", **{"from": -0.015, "to": 0.0}, transform=SLOPE_LEFT),
        solid("slope-right", "canopy", SS, "body", 0.8, "extrude", profile=[[0.0, 0.0], [0.5, 0.0], [0.2, 0.37789], [0.0, 0.37789]], plane="xy", **{"from": 0.0, "to": 0.015}, transform=SLOPE_RIGHT),
        solid("top-plate", "canopy", SS, "body", 0.86, "extrude", profile=[[-0.13, 0.0], [0.13, 0.0], [0.13, 0.2], [-0.13, 0.2]], plane="zx", **{"from": 0.231, "to": 0.246}, holes=[circle(0.0, DUCT_U, 0.07, 16)]),
        solid("back-plate", "canopy", SS, "body", 0.7, "extrude", profile=[[0.045, -0.45], [0.045, 0.45], [0.246, 0.13], [0.246, -0.13]], plane="yz", **{"from": 0.0, "to": 0.015}),
        solid("blower-scroll", "canopy", GALV, "body", 0.49, "extrude", profile=VOLUTE, plane="yz", **{"from": 0.065, "to": 0.145}, holes=[circle(WHEEL[0], WHEEL[1], 0.0705, 16)]),
        solid("blower-motor", "canopy", GALV, "body", 0.43, "revolve", profile=[[0.04, 0.0], [0.04, 0.04], [0.094, 0.04], [0.094, 0.0]], axis=[1, 0, 0], origin=[0.105, WHEEL[0], WHEEL[1]]),
        solid("wheel-drum", "blower-wheel", GALV, "body", 0.56, "revolve", profile=[[-0.034, 0.0], [-0.034, 0.067], [0.034, 0.067], [0.034, 0.0]], axis=[1, 0, 0], origin=[0.0, 0.0, 0.0]),
        solid("chimney-lower", "chimney", SS, "body", 0.86, "extrude", profile=channel, plane="zx", **{"from": 0.0, "to": 0.4}),
        solid("chimney-upper", "chimney-top", SS, "body", 0.9, "extrude", profile=channel, plane="zx", **{"from": 0.0, "to": 0.254}),
        solid("duct", "chimney-top", GALV, "body", 0.55, "revolve", profile=[[-0.4, 0.0], [-0.4, DUCT_R], [0.269, DUCT_R], [0.269, 0.0]], axis=[0, 1, 0], origin=[DUCT_U, 0.0, 0.0]),
    ]
    thermal = [
        ref("th-rim", "rim", region="canopy", layer="shell"),
        ref("th-slope-front", "slope-front", region="canopy", layer="shell", sectionFaces=["-v"]),
        ref("th-slope-left", "slope-left", region="canopy", layer="shell", sectionFaces=["-v"]),
        ref("th-slope-right", "slope-right", region="canopy", layer="shell", sectionFaces=["-v"]),
        ref("th-top-plate", "top-plate", region="canopy", layer="shell", sectionFaces=["-v"]),
        ref("th-back-plate", "back-plate", region="canopy", layer="shell", sectionFaces=["+u"]),
        th(solid("th-filter-field", "canopy", MESH, "body", 0.6, "box", size=[0.32, 0.015, 0.856], transform={"translation": [0.2, 0.0135, 0.0]}), "filter", "mesh"),
        th(solid("th-lamp-left", "canopy", LAMP, "detail", 0.97, "box", size=[0.05, 0.002, 0.12], transform={"translation": [0.425, 0.005, 0.2]}), "lamps", "diffuser"),
        th(solid("th-lamp-right", "canopy", LAMP, "detail", 0.97, "box", size=[0.05, 0.002, 0.12], transform={"translation": [0.425, 0.005, -0.2]}), "lamps", "diffuser"),
        ref("th-chimney-lower", "chimney-lower", region="chimney", layer="tube"),
        ref("th-chimney-upper", "chimney-upper", region="chimney", layer="tube"),
        th(solid("th-extract-outlet", "chimney-top", GALV, "detail", 0.3, "revolve", profile=[[0.254, 0.0], [0.254, DUCT_R], [0.269, DUCT_R], [0.269, 0.0]], axis=[0, 1, 0], origin=[DUCT_U, 0.0, 0.0]), "chimney", tags=["emitter", "extract"]),
        ref("th-blower-motor", "blower-motor", region="canopy", sectionFaces=ALL_FACES),
    ]
    structure = [
        keep["member-back-plate"], keep["member-hip-left"], keep["member-hip-right"], keep["member-rim"],
        ref("member-top-plate", "top-plate", member="canopy", part="member"),
        ref("member-chimney-lower", "chimney-lower", member="chimney", part="member"),
        ref("member-chimney-upper", "chimney-upper", member="chimney-top", part="member"),
        keep["joint-chimney-seat"], keep["joint-wall-bracket"], keep["support-wall-hook"], keep["support-wall-rest"],
        ref("mass-fan-housing", "blower-scroll", member="canopy", part="mass"),
        ref("mass-fan-motor", "blower-motor", member="canopy", part="mass"),
        keep["mass-chimney"], keep["mass-chimney-top"],
        ref("wheel-mass", "wheel-drum", member="blower-wheel", part="mass"),
    ] + leaves(0.015)
    decisions = [
        text("Body, modelled for a smallest feature of 15 mm: the rim band is one block of 45 mm with the openings of the underside cut into it, the three cassettes are one filter field of 0.856 m by 0.32 m and the two lamp modules are blocks in the front strip. The three sloping sheets, the top plate and the back sheet are plates of 15 mm towards the hidden inside; their outer faces and the two hips are exact.", "Körper, modelliert für ein kleinstes Merkmal von 15 mm: Das Randband ist ein Block von 45 mm, in den die Öffnungen der Unterseite geschnitten sind, die drei Kassetten sind ein Filterfeld von 0,856 m mal 0,32 m und die zwei Lampenmodule Blöcke im vorderen Streifen. Die drei geneigten Bleche, die Deckplatte und das Rückblech sind 15 mm dicke Platten zur verdeckten Innenseite; ihre Außenflächen und die beiden Grate sind exakt."),
        text("The blower is three masses: the scroll housing as a volute whose radius grows from 82.5 mm at the tongue to 130 mm at the outlet wall in steps of 22.5 degrees, with the bore of the wheel and its outlet neck up to the top plate, the wheel as a plain drum of 134 mm on its own bone and the motor as a cylinder of 80 mm. Vanes, inlet ring, transition, spigot and the backdraft leaves are below 15 mm or hidden in the duct and are left out.", "Das Gebläse besteht aus drei Massen: dem Spiralgehäuse, dessen Radius in Schritten von 22,5 Grad von 82,5 mm an der Zunge auf 130 mm an der Auslasswand wächst, mit der Bohrung des Laufrads und seinem Auslasshals bis zur Deckplatte, dem Laufrad als glatter Trommel von 134 mm auf eigenem Knochen und dem Motor als Zylinder von 80 mm. Schaufeln, Einlaufring, Übergangsstück, Stutzen und die Rückstauklappen liegen unter 15 mm oder verdeckt im Rohr und entfallen."),
        text("The chimney is two U channels with walls of 15 mm, open to the wall and of one outer size (the 3 mm telescope step is below the feature); the duct is one solid cylinder of 150 mm from the top plate to 15 mm above the chimney. Switch panel, buttons, latches, cassette frames, louvres, return lips and the wall bracket are left out.", "Der Kamin besteht aus zwei zur Wand offenen U-Schächten mit 15 mm Wandstärke und einem Außenmaß (der Teleskopabsatz von 3 mm liegt unter dem Merkmal); das Rohr ist ein Vollzylinder von 150 mm von der Deckplatte bis 15 mm über den Kamin. Schalterblende, Tasten, Riegel, Kassettenrahmen, Lüftungsschlitze, Umbüge und der Wandwinkel entfallen."),
        text("Thermal: the canopy shell is the rim block and the five plates with their inner faces as sections, the filter field and the two diffuser sheets of 2 mm are the hidden regions, the chimney is both channels, the emitter is a disc of 15 mm at the duct mouth and the motor is the heat source inside.", "Thermisch: Die Haubenschale sind der Randblock und die fünf Platten mit ihren Innenseiten als Schnittflächen, Filterfeld und die zwei 2 mm dicken Diffusorscheiben sind die verdeckten Regionen, der Kamin sind beide Schächte, der Emitter ist eine 15 mm dicke Scheibe an der Rohrmündung und der Motor die Wärmequelle im Inneren."),
        text("Structure: back plate on two screws and two pads, rim frame, top plate and the two hips as angles of 15 mm, the chimney channels with their seat on the top plate and the wall bracket; the masses are the blower housing and the motor themselves, the wheel drum and the two chimney channels. The two backdraft leaves are posed bones and stay as half-octagon plates of 15 mm inside the duct.", "Tragwerk: Rückplatte an zwei Schrauben und zwei Puffern, Randrahmen, Deckplatte und die zwei Grate als Winkel von 15 mm, die Kaminschächte mit ihrem Sitz auf der Deckplatte und dem Wandwinkel; die Massen sind Gebläsegehäuse und Motor selbst, die Laufradtrommel und die zwei Kaminschächte. Die zwei Rückstauklappen sind gestellte Knochen und bleiben als 15 mm dicke Halbachteck-Platten im Rohr."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_200(old):
    """📐 1:200, feature 30 mm: rim slab, one pyramid solid, one filter field, two chimney blocks."""
    keep = by_id(old["structure"])
    body = [
        solid("rim", "canopy", SS, "body", 0.78, "extrude", profile=RIM, plane="zx", **{"from": 0.0, "to": 0.045}, holes=[FILTER_FIELD]),
        solid("filter-field", "canopy", MESH, "body", 0.5, "box", size=[0.32, 0.03, 0.856], transform={"translation": [0.2, 0.021, 0.0]}),
        *pyramid(),
        solid("chimney-lower", "chimney", SS, "body", 0.86, "box", size=[0.19, 0.4, 0.25], transform={"translation": [0.095, 0.2, 0.0]}),
        solid("chimney-upper", "chimney-top", SS, "body", 0.9, "box", size=[0.19, 0.269, 0.25], transform={"translation": [0.095, 0.1345, 0.0]}),
    ]
    thermal = [
        ref("th-rim", "rim", region="canopy", layer="shell"),
        *PYRAMID_SHELL,
        th(solid("th-filter-field", "canopy", MESH, "body", 0.6, "box", size=[0.32, 0.03, 0.856], transform={"translation": [0.2, 0.021, 0.0]}), "filter", "mesh"),
        th(solid("th-lamps", "canopy", LAMP, "detail", 0.97, "box", size=[0.05, 0.002, 0.12], transform={"translation": [0.425, 0.005, -0.2]}, repeat={"count": 2, "translation": [0, 0, 0.4]}), "lamps", "diffuser"),
        ref("th-chimney-lower", "chimney-lower", region="chimney", layer="tube"),
        ref("th-chimney-upper", "chimney-upper", region="chimney", layer="tube"),
        th(solid("th-extract-outlet", "chimney-top", GALV, "detail", 0.3, "revolve", profile=[[0.239, 0.0], [0.239, DUCT_R], [0.269, DUCT_R], [0.269, 0.0]], axis=[0, 1, 0], origin=[DUCT_U, 0.0, 0.0]), "chimney", tags=["emitter", "extract"]),
        th(solid("th-blower-motor", "canopy", GALV, "body", 0.43, "revolve", profile=[[0.04, 0.0], [0.04, 0.04], [0.094, 0.04], [0.094, 0.0]], axis=[1, 0, 0], origin=[0.105, WHEEL[0], WHEEL[1]]), "canopy", sectionFaces=ALL_FACES),
    ]
    structure = [
        st(solid("member-back-plate", "canopy", SS, "body", 0.6, "extrude", profile=[[0.045, -0.45], [0.045, 0.45], [0.246, 0.13], [0.246, -0.13]], plane="yz", **{"from": 0.0, "to": 0.03}), "canopy", "member"),
        st(solid("member-top-plate", "canopy", SS, "body", 0.6, "box", size=[0.2, 0.03, 0.26], transform={"translation": [0.1, 0.231, 0.0]}), "canopy", "member"),
        keep["member-rim"], keep["member-hip-left"], keep["member-hip-right"],
        ref("member-chimney-lower", "chimney-lower", member="chimney", part="member"),
        ref("member-chimney-upper", "chimney-upper", member="chimney-top", part="member"),
        keep["support-wall-hook"], keep["support-wall-rest"],
        st(solid("mass-fan-unit", "canopy", GALV, "body", 0.45, "ellipsoid", radii=[0.065, 0.085, 0.095], transform={"translation": [0.125, 0.125, 0.07]}), "canopy", "mass"),
        st(solid("wheel-mass", "blower-wheel", GALV, "body", 0.45, "ellipsoid", radii=[0.034, 0.067, 0.067]), "blower-wheel", "mass"),
    ] + leaves(0.03)
    decisions = [
        text("Body, modelled for a smallest feature of 30 mm, eight masses: the rim is a slab of 45 mm with the filter field of 0.856 m by 0.32 m set into its underside, the pyramid above it is four convex masses that meet in the hip planes (a wedge under each sloping sheet and the core under the top plate) so that faces, hips and the top plate edge are exact, and the chimney is two blocks of 0.19 m by 0.25 m, the upper one including the duct stub up to 0.915 m.", "Körper, modelliert für ein kleinstes Merkmal von 30 mm, acht Massen: Der Rand ist eine Platte von 45 mm mit dem in die Unterseite eingelassenen Filterfeld von 0,856 m mal 0,32 m, die Pyramide darüber besteht aus vier konvexen Massen, die sich in den Gratebenen treffen (ein Keil unter jedem geneigten Blech und der Kern unter der Deckplatte), sodass Flächen, Grate und Deckplattenkante exakt sind, und der Kamin besteht aus zwei Blöcken von 0,19 m mal 0,25 m, der obere einschließlich des Rohrstutzens bis 0,915 m."),
        text("Lamp modules, switch panel, telescope step, louvres, the open back of the chimney, the duct and the whole blower are below 30 mm or hidden inside the closed masses and are left out of the body.", "Lampenmodule, Schalterblende, Teleskopabsatz, Lüftungsschlitze, die offene Kaminrückseite, das Rohr und das ganze Gebläse liegen unter 30 mm oder verdeckt in den geschlossenen Massen und entfallen im Körper."),
        text("Thermal: the canopy shell is the rim slab and the pyramid without its covered base, the filter field and one diffuser sheet of 2 mm per lamp module are the hidden regions inside the rim, the chimney is both blocks, the emitter is a disc of 30 mm under the duct mouth and the motor is a cylinder inside the pyramid.", "Thermisch: Die Haubenschale sind Randplatte und Pyramide ohne ihre verdeckte Grundfläche, das Filterfeld und je eine 2 mm dicke Diffusorscheibe pro Lampenmodul sind die verdeckten Regionen im Rand, der Kamin sind beide Blöcke, der Emitter ist eine 30 mm dicke Scheibe unter der Rohrmündung und der Motor ein Zylinder in der Pyramide."),
        text("Structure: back plate and top plate of 30 mm, the rim frame and the two hips as angles, both chimney blocks, the two screw and pad supports as bars on the wall, and two masses where they are: the fan unit as an ellipsoid at the blower, 70 mm left of the centre line, and the wheel on its bone.", "Tragwerk: Rück- und Deckplatte von 30 mm, der Randrahmen und die zwei Grate als Winkel, beide Kaminblöcke, die Schrauben- und Pufferauflager als Leisten an der Wand und zwei Massen an ihrem Ort: die Lüftereinheit als Ellipsoid am Gebläse, 70 mm links der Mittellinie, und das Laufrad auf seinem Knochen."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_500(old):
    """🏙️ 1:500, feature 75 mm: one canopy solid and two chimney blocks."""
    keep = by_id(old["structure"])
    body = [
        solid("rim", "canopy", SS, "body", 0.78, "box", size=[0.5, 0.045, 0.9], transform={"translation": [0.25, 0.0225, 0.0]}),
        *pyramid(),
        solid("chimney-lower", "chimney", SS, "body", 0.86, "box", size=[0.19, 0.4, 0.25], transform={"translation": [0.095, 0.2, 0.0]}),
        solid("chimney-upper", "chimney-top", SS, "body", 0.9, "box", size=[0.19, 0.269, 0.25], transform={"translation": [0.095, 0.1345, 0.0]}),
    ]
    thermal = [
        ref("th-rim", "rim", region="canopy", layer="shell"),
        *PYRAMID_SHELL,
        th(solid("th-filter-field", "canopy", MESH, "body", 0.6, "box", size=[0.32, 0.03, 0.856], transform={"translation": [0.2, 0.021, 0.0]}), "filter", "mesh"),
        th(solid("th-lamps", "canopy", LAMP, "detail", 0.97, "box", size=[0.05, 0.002, 0.24], transform={"translation": [0.425, 0.005, 0.0]}), "lamps", "diffuser"),
        ref("th-chimney-lower", "chimney-lower", region="chimney", layer="tube"),
        ref("th-chimney-upper", "chimney-upper", region="chimney", layer="tube"),
        th(solid("th-extract-outlet", "chimney-top", GALV, "detail", 0.3, "revolve", profile=[[0.194, 0.0], [0.194, DUCT_R], [0.269, DUCT_R], [0.269, 0.0]], axis=[0, 1, 0], origin=[DUCT_U, 0.0, 0.0]), "chimney", tags=["emitter", "extract"]),
        th(solid("th-blower-motor", "canopy", GALV, "body", 0.43, "box", size=[0.08, 0.08, 0.08], transform={"translation": [0.16, WHEEL[0], WHEEL[1]]}), "canopy", sectionFaces=ALL_FACES),
    ]
    structure = [
        keep["member-hanger"], keep["member-rim"], keep["member-hip-left"], keep["member-hip-right"], keep["member-chimney"], keep["member-chimney-top"],
        keep["support-wall-hook"], keep["support-wall-rest"],
        st(solid("mass-fan-unit", "canopy", GALV, "body", 0.45, "box", size=[0.15, 0.15, 0.18], transform={"translation": [0.125, 0.125, 0.07]}), "canopy", "mass"),
        st(solid("wheel-mass", "blower-wheel", GALV, "body", 0.45, "ellipsoid", radii=[0.04, 0.067, 0.067]), "blower-wheel", "mass"),
    ] + leaves(0.03)
    decisions = [
        text("Digital scale without a drawing: at 1:500 the hood is 1 mm by 1.8 mm. Body, seven masses: the canopy is the closed rim slab of 45 mm and the pyramid of three wedges and a core with exact faces and hips up to the top plate of 0.26 m by 0.2 m, and the chimney is two blocks of 0.19 m by 0.25 m up to 0.915 m.", "Digitaler Maßstab ohne Zeichnung: bei 1:500 ist die Haube 1 mm mal 1,8 mm groß. Körper, sieben Massen: Die Haube ist die geschlossene Randplatte von 45 mm und die Pyramide aus drei Keilen und einem Kern mit exakten Flächen und Graten bis zur Deckplatte von 0,26 m mal 0,2 m, und der Kamin besteht aus zwei Blöcken von 0,19 m mal 0,25 m bis 0,915 m."),
        text("Everything below 75 mm is left out: filter field, lamps, switch panel, telescope step, duct, blower and backdraft leaves. The rim of 45 mm stays as its own slab because it is the outline of the hood.", "Alles unter 75 mm entfällt: Filterfeld, Lampen, Schalterblende, Teleskopabsatz, Rohr, Gebläse und Rückstauklappen. Der Rand von 45 mm bleibt als eigene Platte, weil er der Umriss der Haube ist."),
        text("Thermal: the rim slab and the pyramid masses without their covered bases are the shell, the filter field and one lamp strip are hidden regions inside the rim, the chimney is both blocks, the emitter is a disc of 75 mm under the duct mouth and the motor is a block inside the canopy.", "Thermisch: Randplatte und Pyramidenmassen ohne ihre verdeckten Grundflächen sind die Schale, das Filterfeld und ein Lampenstreifen sind verdeckte Regionen im Rand, der Kamin sind beide Blöcke, der Emitter ist eine 75 mm dicke Scheibe unter der Rohrmündung und der Motor ein Block im Haubenkörper."),
        text("Structure: a hanger angle (back sheet and top plate), the rim frame, the two hips, both chimney channels, two wall supports and the fan unit and wheel masses at the blower, 70 mm left of the centre line.", "Tragwerk: ein Hängewinkel (Rückblech und Deckplatte), der Randrahmen, die zwei Grate, beide Kaminschächte, zwei Wandauflager sowie die Massen von Lüftereinheit und Laufrad am Gebläse, 70 mm links der Mittellinie."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def outline(id, bone, x, y, w, h, z=9.1):
    """✏️ An unfilled detail rectangle of the section."""
    return {"id": id, "bone": bone, "role": "detail", "z": z, "filled": False, "tone": 0.3, "kind": "rect", "x": x, "y": y, "w": w, "h": h}


def patch_root(asset):
    """🦴 Rig and 2D: the telescope joint slides, the section draws the blower it cuts."""
    for bone in asset["rig"]["bones"]:
        if bone["id"] == "chimney-top":
            bone["joint"] = {"type": "prismatic", "axisDeg": 90, "minM": -0.101, "maxM": 0.076}
    blower = lambda: [outline("blower-housing", "canopy", 0.065, 0.021, 0.08, 0.21), outline("blower-motor", "canopy", 0.145, 0.095, 0.054, 0.08), outline("blower-wheel", "blower-wheel", -0.034, -0.067, 0.068, 0.134)]
    own = {"blower-housing", "blower-motor", "blower-wheel", "duct-spigot"}
    slivers = {"back-cover-section", "rotor-disc-section"}
    r50, r100, r200 = (asset["representations"][k] for k in ("50", "100", "200"))
    r50["geometry"]["elevation"] = [s for s in r50["geometry"]["elevation"] if s["id"] not in own] + blower() + [outline("duct-spigot", "canopy", 0.0105, 0.246, 0.149, 0.073)]
    r100["geometry"]["elevation"] = [s for s in r100["geometry"]["elevation"] if s["id"] not in own | slivers] + blower()
    r200["geometry"]["elevation"] = [s for s in r200["geometry"]["elevation"] if s["id"] not in own | slivers]
    r200.pop("sections", None)


def volumes():
    old = json.load(open(SOURCE))["volumes"]
    return {"50": scale_50(old["50"]), "100": scale_100(old["100"]), "200": scale_200(old["200"]), "500": scale_500(old["500"])}


def main():
    mode = sys.argv[1]
    tool = sys.modules[__name__]
    built = volumes()
    raw = open(CATALOG).read()
    document = json.loads(raw)
    stable = tool.dump(document, 0).strip() == raw.strip()
    patch_root(document["asset"])
    if mode == "candidate":
        open(SOURCE, "w").write(tool.dump({"volumes": built}, 0) + "\n")
        document["asset"]["volumes"] = built
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        open(CANDIDATE, "w").write(tool.dump(document, 0) + "\n")
        print("candidate", CANDIDATE, "catalog dump stable:", stable)
    elif mode == "publish":
        for key in ("rig", "representations"):
            start = raw.index('\n    "' + key + '": ') + len('\n    "' + key + '": ')
            depth, index, quoted = 0, start, False
            while True:
                char = raw[index]
                if quoted:
                    index += 1 if char == "\\" else 0
                    quoted = char != '"'
                elif char == '"':
                    quoted = True
                elif char in "{[":
                    depth += 1
                elif char in "}]":
                    depth -= 1
                    if depth == 0:
                        break
                index += 1
            raw = raw[:start] + tool.dump(document["asset"][key], 2) + raw[index + 1:]
        json.loads(raw)
        open(CATALOG, "w").write(raw)
        print("root written", CATALOG)


main()
