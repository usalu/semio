#!/usr/bin/env python3
"""🏷️ W2-R-mid: writes the `x-semio-ui` input annotations (design §6, manifest `$defs/InputUi`) onto every mutation leaf
payload schema of the wfc, block, fem and layout plugins and of the os-owned leaves (os config lane, os artifacts, gis map
window lane), including every nested record field, every `$defs` record field, every root-union variant and every
nested-union branch. Formatting is preserved per file: pretty (indent 2), minified, one-line and hand-formatted files keep
their layout; hand-formatted files are edited in place through a JSON span editor. Leaves under `🧫️fixtures` are test
fixtures and stay untouched. Payload schemas stay structural: no validation keyword is added or changed.

Run: python3 🧪️w2-r-mid-annotate-inputs.py [--check | --preview <dir>]   (--check writes nothing; --preview writes the
annotated copies under <dir> instead of in place)"""
import json
import math
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
ROOTS = [
    "✏️s/🔌️plugins/🀄️wfc",
    "✏️s/🔌️plugins/🧱️block",
    "✏️s/🔌️plugins/🏗️fem",
    "✏️s/🔌️plugins/📏️layout",
    "🧰️framework/🛍️products/💻️os",
    "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config",
]
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")
NUMBER_KEYS = {"step", "precision", "softMin", "softMax", "snaps", "snapSource", "displayUnit", "displayFactor", "scale"}
PI = math.pi
DEGREES = 57.29577951308232
RADIAN_STEP = 0.017453292519943295
ANGLE_SNAPS = [-PI, -PI / 2, 0, PI / 2, PI]


def L(en, de):
    return {"en": en, "de": de}


def S(en, de, **extra):
    return {"label": L(en, de), **extra}


def D(en, de):
    return L(en, de)


#region 🔖️Presets
def angle(en, de, **extra):
    return S(en, de, widget="dial", unit="rad", displayUnit="deg", displayFactor=DEGREES, step=RADIAN_STEP, softMin=-PI, softMax=PI, snaps=ANGLE_SNAPS, **extra)


def zoom(en, de, **extra):
    return S(en, de, widget="slider", step=0.01, precision=2, softMin=0.1, softMax=10, scale="log", snaps=[0.25, 0.5, 1, 2, 4], **extra)


def coordinate(en, de, unit=None, step=1, precision=2, **extra):
    return S(en, de, widget="stepper", **({"unit": unit} if unit else {}), step=step, precision=precision, **extra)


def count(en, de, **extra):
    return S(en, de, widget="stepper", step=1, precision=0, **extra)


def channel(en, de):
    return S(en, de, widget="slider", step=1, precision=0, group="color")


def text(en, de, **extra):
    return S(en, de, widget="text", **extra)


def multiline(en, de, **extra):
    return S(en, de, widget="multiline", **extra)


def toggle(en, de, **extra):
    return S(en, de, widget="toggle", **extra)


def record(en, de, **extra):
    return S(en, de, group="record", **extra)


def target(entity, en=None, de=None, **extra):
    return {"target": entity, **({"label": L(en, de)} if en else {}), **extra}


def hidden(en, de, **extra):
    return S(en, de, widget="hidden", **extra)
#endregion 🔖️Presets


#region 🔖️Vocabulary
RGBA = {"r": channel("Red", "Rot"), "g": channel("Green", "Grün"), "b": channel("Blue", "Blau"), "a": channel("Alpha", "Alpha")}
DOF = {
    "Tx": L("Translation X", "Verschiebung x"), "Ty": L("Translation Y", "Verschiebung y"), "Tz": L("Translation Z", "Verschiebung z"),
    "Rx": L("Rotation about X", "Verdrehung um x"), "Ry": L("Rotation about Y", "Verdrehung um y"), "Rz": L("Rotation about Z", "Verdrehung um z"),
}
AXES = {"x": L("X axis", "x-Achse"), "y": L("Y axis", "y-Achse"), "z": L("Z axis", "z-Achse")}
DISCRIMINATOR = {"widget": "hidden", "role": "discriminator", "label": L("Mutation", "Mutation")}

FEM_ENTITIES = {
    "node": L("Node", "Knoten"), "element": L("Element", "Element"), "region": L("Plate region", "Scheibenbereich"), "solid": L("Solid", "Volumenkörper"),
    "support": L("Support", "Lager"), "load": L("Load", "Last"), "material": L("Material", "Material"), "section": L("Cross-section", "Querschnitt"),
    "loadCase": L("Load case", "Lastfall"), "combination": L("Load combination", "Lastfallkombination"),
}
FEM_COMMON = {
    "caseId": target("loadCase"), "loadId": target("load"), "nodeId": target("node"), "elementId": target("element"), "regionId": target("region"), "solidId": target("solid"),
    "materialId": target("material"), "sectionId": target("section"),
    "start": target("node", "Start node", "Anfangsknoten"), "end": target("node", "End node", "Endknoten"),
    "newSelfWeight": toggle("Self weight", "Eigengewicht", description=D("Includes the structure's self weight in this load case.", "Berücksichtigt das Eigengewicht des Tragwerks in diesem Lastfall.")),
    "selfWeight": toggle("Self weight", "Eigengewicht", description=D("Includes the structure's self weight in this load case.", "Berücksichtigt das Eigengewicht des Tragwerks in diesem Lastfall.")),
    "name": text("Name", "Bezeichnung", group="identity"), "newName": text("Name", "Bezeichnung", group="identity"),
    "load": record("Load", "Last"), "newLoad": record("New load", "Neue Last"),
    "loadCase": record("Load case", "Lastfall"), "loads": record("Loads", "Lasten"),
    "combination": record("Load combination", "Lastfallkombination"), "newCombination": record("New load combination", "Neue Lastfallkombination"),
    "terms": record("Load case factors", "Lastfallfaktoren", description=D("Each load case of the combination with its factor.", "Jeder Lastfall der Kombination mit seinem Faktor.")),
    "factor": coordinate("Factor", "Faktor", step=0.05, precision=2, group="load", description=D("Partial or combination factor applied to the load case.", "Teilsicherheits- bzw. Kombinationsbeiwert für den Lastfall.")),
    "dof": S("Degree of freedom", "Freiheitsgrad", widget="select", options=DOF, group="load"),
    "fixed": record("Fixed degrees of freedom", "Gesperrte Freiheitsgrade", options_items=DOF),
    "value": coordinate("Load value", "Lastwert", step=100, precision=2, group="load", description=D("Force in N on a translation, moment in N·m on a rotation.", "Kraft in N bei einer Verschiebung, Moment in N·m bei einer Verdrehung.")),
    "pressure": coordinate("Area load", "Flächenlast", unit="Pa", displayUnit="kN/m²", displayFactor=0.001, step=100, precision=2, group="load"),
    "settings": record("Analysis settings", "Analyseeinstellungen"),
    "modalCount": count("Number of vibration modes", "Anzahl Eigenformen", group="analysis"),
    "bucklingCount": count("Number of buckling modes", "Anzahl Knickformen", group="analysis"),
    "deformationScale": coordinate("Deformation scale", "Verformungsmaßstab", step=1, precision=1, group="analysis", description=D("Magnification the deformed shape is drawn with.", "Überhöhung, mit der die Verformungsfigur gezeichnet wird.")),
    "node": record("Node", "Knoten"), "newNode": record("New node", "Neuer Knoten"),
    "x": coordinate("X", "X", unit="m", step=0.1, precision=3, group="position"), "y": coordinate("Y", "Y", unit="m", step=0.1, precision=3, group="position"), "z": coordinate("Z", "Z", unit="m", step=0.1, precision=3, group="position"),
    "element": record("Element", "Element"), "newElement": record("New element", "Neues Element"),
    "roll": angle("Roll angle", "Verdrehwinkel", group="geometry", description=D("Rotation of the section axes about the member axis.", "Drehung der Querschnittshauptachsen um die Stabachse.")),
    "support": record("Support", "Lager"), "newSupport": record("New support", "Neues Lager"),
    "material": record("Material", "Material"), "newMaterial": record("New material", "Neues Material"),
    "e": coordinate("Young's modulus", "Elastizitätsmodul", unit="Pa", displayUnit="GPa", displayFactor=1e-9, step=1e9, precision=1, group="material"),
    "g": coordinate("Shear modulus", "Schubmodul", unit="Pa", displayUnit="GPa", displayFactor=1e-9, step=1e9, precision=1, group="material"),
    "nu": coordinate("Poisson's ratio", "Querdehnzahl", step=0.01, precision=3, softMin=0, softMax=0.49, group="material"),
    "rho": coordinate("Density", "Dichte", unit="kg/m³", step=10, precision=0, group="material"),
    "section": record("Cross-section", "Querschnitt"), "newSection": record("New cross-section", "Neuer Querschnitt"),
    "area": coordinate("Cross-sectional area", "Querschnittsfläche", unit="m²", displayUnit="cm²", displayFactor=1e4, step=1e-4, precision=2, group="section"),
    "iy": coordinate("Second moment of area Iy", "Flächenträgheitsmoment Iy", unit="m⁴", displayUnit="cm⁴", displayFactor=1e8, step=1e-8, precision=1, group="section"),
    "iz": coordinate("Second moment of area Iz", "Flächenträgheitsmoment Iz", unit="m⁴", displayUnit="cm⁴", displayFactor=1e8, step=1e-8, precision=1, group="section"),
    "j": coordinate("Torsion constant", "Torsionsflächenmoment", unit="m⁴", displayUnit="cm⁴", displayFactor=1e8, step=1e-8, precision=1, group="section", description=D("Saint-Venant torsion constant I_T.", "Torsionsflächenmoment 2. Grades I_T nach Saint-Venant.")),
    "region": record("Plate region", "Scheibenbereich"), "newRegion": record("New plate region", "Neuer Scheibenbereich"),
    "solid": record("Solid", "Volumenkörper"), "newSolid": record("New solid", "Neuer Volumenkörper"),
    "outline": record("Outline", "Umriss", description=D("Closed polygon of x, y vertices in m.", "Geschlossenes Polygon aus x-, y-Eckpunkten in m.")),
    "holes": record("Openings", "Aussparungen", description=D("Polygons cut out of the outline.", "Aus dem Umriss ausgeschnittene Polygone.")),
    "thickness": coordinate("Thickness", "Dicke", unit="m", displayUnit="mm", displayFactor=1000, step=0.001, precision=1, group="geometry"),
    "meshSize": coordinate("Mesh size", "Netzweite", unit="m", step=0.01, precision=3, group="mesh", description=D("Target edge length of the finite elements.", "Angestrebte Kantenlänge der finiten Elemente.")),
    "axis": S("Extrusion axis", "Extrusionsachse", widget="segmented", options=AXES, group="geometry"),
    "baseZ": coordinate("Base elevation", "Basishöhe", unit="m", step=0.1, precision=3, group="geometry", description=D("Coordinate along the extrusion axis where the solid starts.", "Koordinate entlang der Extrusionsachse, an der der Körper beginnt.")),
    "height": coordinate("Extrusion height", "Extrusionshöhe", unit="m", step=0.1, precision=3, group="geometry"),
    "layers": count("Element layers", "Elementlagen", group="mesh", description=D("Number of element layers through the extrusion height.", "Anzahl der Elementlagen über die Extrusionshöhe.")),
    "clock": record("Playback clock", "Wiedergabetakt"),
    "phase": S("Phase", "Phase", widget="slider", step=0.01, precision=2, group="playback", description=D("Position within one animation cycle, 0 to 1.", "Position innerhalb eines Animationszyklus, 0 bis 1.")),
    "reverse": toggle("Reverse", "Rückwärts", group="playback"),
}
FEM_VARIANTS = {
    "nodal": L("Nodal load", "Knotenlast"), "memberUdl": L("Uniform member load", "Gleichstreckenlast"), "area": L("Area load", "Flächenlast"),
    "bar": L("Bar (axial only)", "Fachwerkstab"), "beam": L("Beam", "Balken"), "frame": L("Frame member", "Rahmenstab"),
}
FEM_PATH = {
    **{pointer: {"label": L("Load type", "Lastart")} for pointer in ("/load/kind", "/newLoad/kind", "/loadCase/loads/-/kind")},
    **{pointer: {"label": L("Element type", "Elementtyp")} for pointer in ("/element/kind", "/newElement/kind")},
}
FEM2D_EXTRA = {
    "wx": coordinate("Load wx (along)", "Streckenlast wx (längs)", unit="N/m", displayUnit="kN/m", displayFactor=0.001, step=100, precision=2, group="load", description=D("Uniform load along the member axis (local x).", "Gleichmäßig verteilte Last in Stabachsrichtung (lokal x).")),
    "wy": coordinate("Load wy (across)", "Streckenlast wy (quer)", unit="N/m", displayUnit="kN/m", displayFactor=0.001, step=100, precision=2, group="load", description=D("Uniform load across the member axis (local y).", "Gleichmäßig verteilte Last quer zur Stabachse (lokal y).")),
    "pressure": coordinate("Area load", "Flächenlast", unit="Pa", displayUnit="kN/m²", displayFactor=0.001, step=100, precision=2, group="load", description=D("Uniform pressure on the plate region.", "Gleichmäßige Flächenlast auf den Scheibenbereich.")),
}
FEM3D_EXTRA = {
    "wx": coordinate("Load wx", "Streckenlast wx", unit="N/m", displayUnit="kN/m", displayFactor=0.001, step=100, precision=2, group="load", description=D("Uniform member load component in global X.", "Komponente der Gleichstreckenlast in globaler x-Richtung.")),
    "wy": coordinate("Load wy", "Streckenlast wy", unit="N/m", displayUnit="kN/m", displayFactor=0.001, step=100, precision=2, group="load", description=D("Uniform member load component in global Y.", "Komponente der Gleichstreckenlast in globaler y-Richtung.")),
    "wz": coordinate("Load wz", "Streckenlast wz", unit="N/m", displayUnit="kN/m", displayFactor=0.001, step=100, precision=2, group="load", description=D("Uniform member load component in global Z.", "Komponente der Gleichstreckenlast in globaler z-Richtung.")),
    "pressure": coordinate("Area load", "Flächenlast", unit="Pa", displayUnit="kN/m²", displayFactor=0.001, step=100, precision=2, group="load", description=D("Uniform pressure on the top face of the solid.", "Gleichmäßige Flächenlast auf die Oberseite des Volumenkörpers.")),
}

BLOCK_COMMON = {
    "newName": text("Name", "Name", group="identity"), "name": text("Name", "Name", group="identity"),
    "newLabel": text("Label", "Bezeichnung"), "label": text("Label", "Bezeichnung"),
    "newDescription": multiline("Description", "Beschreibung"), "description": multiline("Description", "Beschreibung"),
    "newUnit": text("Unit", "Einheit", description=D("Unit of measure the kind is modelled in; empty clears it.", "Maßeinheit, in der die Art modelliert ist; leer entfernt sie.")),
    "newVariant": text("Variant", "Variante", description=D("Variant name of the kind; empty clears it.", "Variantenname der Art; leer entfernt ihn.")),
    "newIcon": text("Icon", "Symbol"), "newIconKind": text("Icon", "Symbol", group="appearance"),
    "newColor": text("Color", "Farbe", group="appearance", description=D("CSS color, e.g. #3366ff.", "CSS-Farbe, z. B. #3366ff.")),
    "color": text("Color", "Farbe", group="appearance", description=D("CSS color, e.g. #3366ff.", "CSS-Farbe, z. B. #3366ff.")),
    "newShape": text("Shape", "Form", group="appearance", description=D("Outline shape, e.g. circle or rectangle; empty clears it.", "Umrissform, z. B. circle oder rectangle; leer entfernt sie.")),
    "newRadius": coordinate("Radius", "Radius", step=0.1, precision=2, group="geometry"),
    "newWidth": coordinate("Width", "Breite", step=0.1, precision=2, group="geometry"),
    "newHeight": coordinate("Height", "Höhe", step=0.1, precision=2, group="geometry"),
    "newX": coordinate("Camera X", "Kamera X", step=1, precision=2, group="camera"), "newY": coordinate("Camera Y", "Kamera Y", step=1, precision=2, group="camera"),
    "newZoom": zoom("Zoom", "Zoom", group="camera"),
    "newTarget": S("Camera target", "Kameraziel", widget="vector", group="camera", description=D("Point the camera looks at.", "Punkt, auf den die Kamera blickt.")),
    "rule": record("Compatibility rule", "Kompatibilitätsregel"),
    "bidirectional": toggle("Bidirectional", "Beidseitig", group="kind", description=D("The rule also allows the reverse direction.", "Die Regel erlaubt auch die Gegenrichtung.")),
    "author": record("Author", "Autor"), "email": text("Email", "E-Mail", group="identity"),
    "attribute": record("Attribute", "Attribut"), "attributes": record("Attributes", "Attribute"),
    "key": text("Key", "Schlüssel", group="metadata"), "value": text("Value", "Wert", group="metadata"),
    "definition": text("Definition", "Definition", group="metadata", description=D("Name or URI of the attribute's definition.", "Name oder URI der Attributdefinition.")),
    "tag": text("Tag", "Schlagwort", group="metadata"), "tags": record("Tags", "Schlagwörter"),
    "representation": record("Representation", "Darstellung"),
    "meshUrl": text("Mesh URL", "Mesh-URL", group="source"), "newMeshUrl": text("Mesh URL", "Mesh-URL", group="source"),
    "lod": text("Level of detail", "Detailstufe", group="source"), "newLod": text("Level of detail", "Detailstufe", group="source"),
    "angle": angle("Angle", "Winkel", group="geometry", description=D("Position on the node rim, counter-clockwise.", "Lage auf dem Knotenrand, gegen den Uhrzeigersinn.")),
    "newAngle": angle("Angle", "Winkel", group="geometry", description=D("Position on the node rim, counter-clockwise.", "Lage auf dem Knotenrand, gegen den Uhrzeigersinn.")),
    "radius": coordinate("Radius", "Radius", step=0.01, precision=2, group="geometry"),
    "position": S("Position", "Position", widget="vector", group="geometry"), "newPosition": S("Position", "Position", widget="vector", group="geometry"),
    "direction": S("Direction", "Richtung", widget="vector", group="geometry", description=D("Outward connection direction.", "Nach außen gerichtete Verbindungsrichtung.")),
    "newDirection": S("Direction", "Richtung", widget="vector", group="geometry", description=D("Outward connection direction.", "Nach außen gerichtete Verbindungsrichtung.")),
    "preview": record("Brush preview", "Pinselvorschau"),
}
BLOCK2D = {
    "handle": record("Handle", "Griff"), "handleKind": record("Handle kind", "Griffart"),
    "defaultWireKind": text("Default wire kind", "Standard-Leitungsart", group="kind"), "newDefaultWireKind": text("Default wire kind", "Standard-Leitungsart", group="kind"),
    "newHandleKind": target("handleKind", "Handle kind", "Griffart"),
    "source": target("handleKind", "Source handle kind", "Quell-Griffart"), "target": target("handleKind", "Target handle kind", "Ziel-Griffart"),
    "newX": coordinate("Camera X", "Kamera X", step=1, precision=2, group="camera"),
}
BLOCK2D_PATH = {"/handle/handleKind": target("handleKind", "Handle kind", "Griffart")}
BLOCK3D = {
    "vortex": record("Vortex", "Wirbel"), "vortexKind": record("Vortex kind", "Wirbelart"),
    "defaultCableKind": text("Default cable kind", "Standard-Kabelart", group="kind"), "newDefaultCableKind": text("Default cable kind", "Standard-Kabelart", group="kind"),
    "newVortexKind": target("vortexKind", "Vortex kind", "Wirbelart"),
    "source": target("vortexKind", "Source vortex kind", "Quell-Wirbelart"), "target": target("vortexKind", "Target vortex kind", "Ziel-Wirbelart"),
    "newPosition": S("Position", "Position", widget="vector", group="geometry"),
    "radius": coordinate("Radius", "Radius", step=0.01, precision=3, group="geometry"), "newRadius": coordinate("Radius", "Radius", step=0.01, precision=3, group="geometry"),
}
BLOCK3D_PATH = {
    "/vortex/vortexKind": target("vortexKind", "Vortex kind", "Wirbelart"),
    "/newPosition@move-camera3d": S("Camera position", "Kameraposition", widget="vector", group="camera"),
}
BLOCK5D = {
    "grip": record("Grip", "Griff"), "gripKind": record("Grip kind", "Griffart"),
    "defaultRopeKind": text("Default rope kind", "Standard-Seilart", group="kind"), "newDefaultRopeKind": text("Default rope kind", "Standard-Seilart", group="kind"),
    "newGripKind": target("gripKind", "Grip kind", "Griffart"),
    "source": target("gripKind", "Source grip kind", "Quell-Griffart"), "target": target("gripKind", "Target grip kind", "Ziel-Griffart"),
    "radius2d": coordinate("2D radius", "2D-Radius", step=0.01, precision=2, group="geometry"), "newRadius2d": coordinate("2D radius", "2D-Radius", step=0.01, precision=2, group="geometry"),
    "radius3d": coordinate("3D radius", "3D-Radius", step=0.01, precision=3, group="geometry"), "newRadius3d": coordinate("3D radius", "3D-Radius", step=0.01, precision=3, group="geometry"),
    "position": S("3D position", "3D-Position", widget="vector", group="geometry"), "direction": S("3D direction", "3D-Richtung", widget="vector", group="geometry", description=D("Outward connection direction.", "Nach außen gerichtete Verbindungsrichtung.")),
    "newOrientation": S("Orientation", "Orientierung", widget="vector", group="pose", description=D("Rotation as a unit quaternion (x, y, z, w); empty keeps it.", "Drehung als Einheitsquaternion (x, y, z, w); leer behält sie bei.")),
    "newScale": S("Scale", "Skalierung", widget="vector", group="pose", description=D("Scale factor per axis; empty keeps it.", "Skalierungsfaktor je Achse; leer behält ihn bei.")),
}
BLOCK5D_PATH = {
    "/grip/gripKind": target("gripKind", "Grip kind", "Griffart"),
    "/newPosition@move-camera3d": S("Camera position", "Kameraposition", widget="vector", group="camera"),
    "/newPosition@move-grip3d": S("3D position", "3D-Position", widget="vector", group="geometry"),
    "/newDirection@move-grip3d": S("3D direction", "3D-Richtung", widget="vector", group="geometry", description=D("Outward connection direction.", "Nach außen gerichtete Verbindungsrichtung.")),
}

WFC_COMMON = {
    "tileId": target("tile", "Tile", "Kachel"), "slotId": target("slot", "Slot", "Slot"),
    "weight": coordinate("Weight", "Gewicht", step=0.1, precision=2, group="solver", description=D("Relative frequency of the tile in a solution.", "Relative Häufigkeit der Kachel in einer Lösung.")),
    "seed": count("Seed", "Startwert", group="solver", description=D("Seed of the random choices the solver makes.", "Startwert der Zufallsentscheidungen des Lösers.")),
    "tile": record("Tile", "Kachel"), "label": text("Label", "Bezeichnung"),
    "media": record("Tile media", "Kachelmedien"),
    "rule": record("Adjacency rule", "Nachbarschaftsregel"),
    "tileAId": target("tile", "Tile A", "Kachel A"), "tileBId": target("tile", "Tile B", "Kachel B"),
    "allowed": toggle("Allowed", "Erlaubt", group="rule", description=D("Whether the pair may be neighbours; off forbids it.", "Ob das Paar benachbart sein darf; aus verbietet es.")),
    "index": count("Index", "Index", group="order", description=D("Position in the list; empty appends at the end.", "Position in der Liste; leer hängt am Ende an.")),
    "zoom": zoom("Zoom", "Zoom", group="camera"),
    "config": record("Configuration", "Konfiguration"),
    "cameraX": coordinate("Camera X", "Kamera X", step=1, precision=2, group="camera"), "cameraY": coordinate("Camera Y", "Kamera Y", step=1, precision=2, group="camera"),
    "cameraZoom": zoom("Camera zoom", "Kamerazoom", group="camera"), "activeTileId": target("tile", "Active tile", "Aktive Kachel"),
    "assignments": record("Assignments", "Zuordnungen", description=D("The tile the solver placed in each slot.", "Die Kachel, die der Löser in jeden Slot gesetzt hat.")),
    "contradiction": toggle("Contradiction", "Widerspruch", group="solver", description=D("The solver found no consistent assignment.", "Der Löser fand keine widerspruchsfreie Zuordnung.")),
    "periodicX": toggle("Periodic in X", "Periodisch in X", group="grid", description=D("Neighbourhoods wrap around the grid edge in X.", "Nachbarschaften laufen in X über den Rasterrand um.")),
    "periodicY": toggle("Periodic in Y", "Periodisch in Y", group="grid", description=D("Neighbourhoods wrap around the grid edge in Y.", "Nachbarschaften laufen in Y über den Rasterrand um.")),
    "periodicZ": toggle("Periodic in Z", "Periodisch in Z", group="grid", description=D("Neighbourhoods wrap around the grid edge in Z.", "Nachbarschaften laufen in Z über den Rasterrand um.")),
    "direction": S("Direction", "Richtung", widget="select", group="rule", options={"LEFT": L("Left", "Links"), "RIGHT": L("Right", "Rechts"), "TOP": L("Top", "Oben"), "BOTTOM": L("Bottom", "Unten"), "FRONT": L("Front", "Vorne"), "BACK": L("Back", "Hinten")}),
    "relation": text("Relation", "Relation", group="rule", description=D("Adjacency class, e.g. adjacent, above or beside.", "Nachbarschaftsklasse, z. B. adjacent, above oder beside.")),
    "edge": record("Connection", "Verbindung"),
    "fromSlotId": target("slot", "From slot", "Von Slot"), "toSlotId": target("slot", "To slot", "Zu Slot"),
    "slot": record("Slot", "Slot"), "pinnedTileId": target("tile", "Pinned tile", "Fixierte Kachel"),
    "width": coordinate("Width", "Breite", step=1, precision=2, group="geometry"), "height": coordinate("Height", "Höhe", step=1, precision=2, group="geometry"), "depth": coordinate("Depth", "Tiefe", step=1, precision=2, group="geometry"),
    "x": coordinate("X", "X", step=1, precision=2, group="position"), "y": coordinate("Y", "Y", step=1, precision=2, group="position"), "z": coordinate("Z", "Z", step=1, precision=2, group="position"),
    "color": record("Color", "Farbe"), **RGBA,
    "kind": S("Media kind", "Medienart", widget="segmented", group="kind", options={"bitmap": L("Bitmap", "Bitmap"), "vector": L("Vector", "Vektor"), "image": L("Image", "Bild")}),
    "palette": record("Palette", "Palette"), "pixels": hidden("Pixels", "Pixel", group="source", description=D("Base64-encoded palette indices.", "Base64-codierte Palettenindizes.")),
    "paths": record("Paths", "Pfade"), "segments": record("Segments", "Segmente"),
    "to": record("End point", "Endpunkt"), "ctrl": record("Control point", "Kontrollpunkt"), "ctrl1": record("First control point", "Erster Kontrollpunkt"), "ctrl2": record("Second control point", "Zweiter Kontrollpunkt"),
    "fill": record("Fill color", "Füllfarbe"), "stroke": record("Stroke color", "Konturfarbe"),
    "strokeWidth": coordinate("Stroke width", "Konturbreite", step=0.5, precision=2, group="appearance"),
    "child": record("Child artifact", "Kindartefakt"), "childId": text("Child ID", "Kind-ID", group="identity"),
    "target": record("Target artifact", "Zielartefakt"), "artifactId": text("Artifact ID", "Artefakt-ID", group="identity"),
    "dialect": record("Dialect", "Dialekt"), "artifactKind": text("Artifact kind", "Artefaktart", group="kind"), "standard": text("Standard", "Standard", group="kind"), "subset": text("Subset", "Teilmenge", group="kind"),
    "positions": record("Vertex positions", "Eckpunktkoordinaten", description=D("Flat x, y, z list of the mesh vertices.", "Flache x-, y-, z-Liste der Netzeckpunkte.")),
    "indices": record("Triangle indices", "Dreiecksindizes", description=D("Three vertex indices per triangle.", "Drei Eckpunktindizes je Dreieck.")),
}
WFC_VARIANTS = {"mesh": L("Mesh", "Netz"), "meshChild": L("Child mesh", "Kindnetz")}
WFC_GRAPH_PATH = {
    "/x@change-camera": coordinate("Camera X", "Kamera X", step=1, precision=2, group="camera"), "/y@change-camera": coordinate("Camera Y", "Kamera Y", step=1, precision=2, group="camera"),
    "/tile/media/kind": {"label": L("Media kind", "Medienart")}, "/media/kind": {"label": L("Media kind", "Medienart")},
}
SEGMENT_KINDS = {"moveTo": L("Move to", "Bewegen zu"), "lineTo": L("Line to", "Linie zu"), "quadTo": L("Quadratic curve to", "Quadratische Kurve zu"), "cubicTo": L("Cubic curve to", "Kubische Kurve zu"), "close": L("Close", "Schließen")}
CELL = {
    "x": count("Column", "Spalte", group="cell"), "y": count("Row", "Zeile", group="cell"), "z": count("Layer", "Lage", group="cell"),
}
WFC_GRID = {
    **CELL,
    "cellWidth": coordinate("Cell width", "Zellbreite", step=1, precision=2, group="grid"), "cellHeight": coordinate("Cell height", "Zellhöhe", step=1, precision=2, group="grid"),
    "width": count("Width in cells", "Breite in Zellen", group="grid"), "height": count("Height in cells", "Höhe in Zellen", group="grid"), "depth": count("Depth in cells", "Tiefe in Zellen", group="grid"),
    "axis": S("Axis", "Achse", widget="segmented", options=AXES, group="grid"),
    "sizes": record("Cell sizes", "Zellgrößen", description=D("Size of every cell along the axis, in order.", "Größe jeder Zelle entlang der Achse, der Reihe nach.")),
    "pinned": record("Pinned cell", "Fixierte Zelle"), "cell": record("Cell", "Zelle"),
}
WFC_GRID2D_PATH = {
    **{"/media/paths/-/segments/-/%s/%s" % (point, axis): coordinate(axis.upper(), axis.upper(), step=1, precision=2, group="position") for point in ("to", "ctrl", "ctrl1", "ctrl2") for axis in ("x", "y")},
    **{"/tile/media/paths/-/segments/-/%s/%s" % (point, axis): coordinate(axis.upper(), axis.upper(), step=1, precision=2, group="position") for point in ("to", "ctrl", "ctrl1", "ctrl2") for axis in ("x", "y")},
    "/media/width": count("Width", "Breite", unit="px", group="source"), "/media/height": count("Height", "Höhe", unit="px", group="source"),
    "/tile/media/width": count("Width", "Breite", unit="px", group="source"), "/tile/media/height": count("Height", "Höhe", unit="px", group="source"),
    "/media/paths/-/segments/-/kind": S("Segment", "Segment", widget="select", options=SEGMENT_KINDS, group="kind"),
    "/tile/media/paths/-/segments/-/kind": S("Segment", "Segment", widget="select", options=SEGMENT_KINDS, group="kind"),
}
WFC_BITMAP = {
    "patternSize": count("Pattern size", "Mustergröße", group="model", description=D("Edge length N of the N×N patterns read from the input.", "Kantenlänge N der N×N-Muster, die aus der Eingabe gelesen werden.")),
    "symmetry": count("Symmetry", "Symmetrie", group="model", description=D("Number of rotated and mirrored pattern variants, 1 to 8.", "Anzahl gedrehter und gespiegelter Mustervarianten, 1 bis 8.")),
    "periodicInput": toggle("Periodic input", "Periodische Eingabe", group="model", description=D("Patterns wrap around the input edges.", "Muster laufen über die Ränder der Eingabe um.")),
    "ground": count("Ground", "Boden", group="model", description=D("Palette index forced onto the bottom row; empty disables it.", "Palettenindex, der die unterste Zeile vorgibt; leer schaltet es ab.")),
    "index": count("Palette index", "Palettenindex", group="palette"), "color": record("Color", "Farbe"),
    "x": count("X", "X", unit="px", group="position"), "y": count("Y", "Y", unit="px", group="position"),
    "width": count("Width", "Breite", unit="px", group="size"), "height": count("Height", "Höhe", unit="px", group="size"),
    "periodic": toggle("Periodic output", "Periodische Ausgabe", group="model", description=D("The output wraps around its edges.", "Die Ausgabe läuft über ihre Ränder um.")),
    "pixels": hidden("Pixels", "Pixel", group="source", description=D("Base64-encoded palette indices, row by row.", "Base64-codierte Palettenindizes, zeilenweise.")),
    "outputPixels": hidden("Output pixels", "Ausgabepixel", group="output", description=D("Base64-encoded solved palette indices.", "Base64-codierte gelöste Palettenindizes.")),
    "outputWidth": count("Output width", "Ausgabebreite", unit="px", group="output"), "outputHeight": count("Output height", "Ausgabehöhe", unit="px", group="output"),
}
WFC_BITMAP_PATH = {"/color@pin-pixel": count("Palette index", "Palettenindex", group="palette")}

LAYOUT_ENTITIES = {
    "page": L("Page", "Seite"), "frame": L("Frame", "Rahmen"), "layer": L("Layer", "Ebene"), "story": L("Story", "Textfluss"), "link": L("Link", "Verknüpfung"),
    "paragraphStyle": L("Paragraph style", "Absatzformat"), "characterStyle": L("Character style", "Zeichenformat"),
}
MM = {"unit": "mm", "step": 1, "precision": 1}
PT = {"unit": "pt", "step": 0.5, "precision": 1}
LAYOUT = {
    "pageId": target("page"), "frameId": target("frame"), "layerId": target("layer"), "storyId": target("story"),
    "threadNext": target("frame", "Next frame", "Nächster Rahmen", description=D("The frame the story flows on to; empty ends the thread.", "Der Rahmen, in dem der Textfluss weiterläuft; leer beendet die Verkettung.")),
    "paragraphStyleId": target("paragraphStyle", "Paragraph style", "Absatzformat"), "characterStyleId": target("characterStyle", "Character style", "Zeichenformat"),
    "newWidth": S("Width", "Breite", widget="stepper", group="geometry", **MM), "newHeight": S("Height", "Höhe", widget="stepper", group="geometry", **MM),
    "width": S("Width", "Breite", widget="stepper", group="geometry", **MM), "height": S("Height", "Höhe", widget="stepper", group="geometry", **MM),
    "newX": S("X", "X", widget="stepper", group="position", **MM), "newY": S("Y", "Y", widget="stepper", group="position", **MM),
    "newName": text("Name", "Name", group="identity"), "name": text("Name", "Name", group="identity"),
    "newContent": multiline("Text", "Text", group="text"), "content": multiline("Text", "Text", group="text"),
    "frame": record("Frame", "Rahmen"), "kind": text("Frame kind", "Rahmenart", group="kind", description=D("Kind of frame, e.g. rect, text or image.", "Rahmenart, z. B. rect, text oder image.")),
    "page": record("Page", "Seite"),
    "index": count("Index", "Index", group="order", description=D("Position in the list; empty appends at the end.", "Position in der Liste; leer hängt am Ende an.")),
    "toIndex": count("Target position", "Zielposition", group="order"),
    "newFill": S("Fill color", "Füllfarbe", widget="vector", group="appearance", description=D("RGBA, each channel 0 to 1; empty removes the fill.", "RGBA, jeder Kanal 0 bis 1; leer entfernt die Füllung.")),
    "newStroke": S("Stroke color", "Konturfarbe", widget="vector", group="appearance", description=D("RGBA, each channel 0 to 1; empty removes the stroke.", "RGBA, jeder Kanal 0 bis 1; leer entfernt die Kontur.")),
    "count": count("Columns", "Spalten", group="columns"), "gutter": S("Gutter", "Spaltenabstand", widget="stepper", group="columns", **MM),
    "baselineGrid": S("Baseline grid increment", "Grundlinienraster-Abstand", widget="stepper", group="grid", **PT),
    "baselineOffset": S("Baseline offset", "Grundlinienversatz", widget="stepper", group="grid", **PT),
    "snapToBaseline": toggle("Align to baseline grid", "Am Grundlinienraster ausrichten", group="grid"),
    "top": S("Top margin", "Rand oben", widget="stepper", group="margins", **MM), "right": S("Right margin", "Rand rechts", widget="stepper", group="margins", **MM),
    "bottom": S("Bottom margin", "Rand unten", widget="stepper", group="margins", **MM), "left": S("Left margin", "Rand links", widget="stepper", group="margins", **MM),
    "insetX": S("Inset X", "Innenabstand X", widget="stepper", group="inset", **MM), "insetY": S("Inset Y", "Innenabstand Y", widget="stepper", group="inset", **MM),
    "insetWidth": S("Inset width", "Innenabstand Breite", widget="stepper", group="inset", **MM), "insetHeight": S("Inset height", "Innenabstand Höhe", widget="stepper", group="inset", **MM),
    "fontFamily": text("Font family", "Schriftfamilie", group="type"), "fontSize": S("Font size", "Schriftgrad", widget="stepper", group="type", **PT),
    "fontWeight": S("Font weight", "Schriftstärke", widget="stepper", step=100, precision=0, group="type", description=D("CSS weight, e.g. 400 regular, 700 bold.", "CSS-Stärke, z. B. 400 normal, 700 fett.")),
    "leading": S("Leading", "Zeilenabstand", widget="stepper", group="type", **PT),
    "tracking": S("Tracking", "Laufweite", widget="stepper", step=1, precision=0, group="type", description=D("Letter spacing in thousandths of an em.", "Zeichenabstand in Tausendstel Geviert.")),
    "alignment": S("Alignment", "Ausrichtung", widget="segmented", group="type", options={"left": L("Left", "Links"), "center": L("Centered", "Zentriert"), "right": L("Right", "Rechts"), "justify": L("Justified", "Blocksatz")}),
    "newRotation": angle("Rotation", "Drehung", group="geometry"),
    "locked": toggle("Locked", "Gesperrt", group="state"), "visible": toggle("Visible", "Sichtbar", group="state"),
    "newColumns": count("Columns", "Spalten", group="columns"),
    "newWrapMode": text("Text wrap", "Textumfluss", group="text", description=D("none, box or contour.", "none, box oder contour.")),
    "link": record("Link", "Verknüpfung"), "path": text("Path", "Pfad", group="source"), "hash": text("Checksum", "Prüfsumme", group="source"),
    "dpi": count("Resolution", "Auflösung", unit="dpi", group="source"), "colorProfile": text("Color profile", "Farbprofil", group="source"),
    "state": text("Link state", "Verknüpfungsstatus", group="source"), "proxyDataUrl": hidden("Preview", "Vorschau", group="source"),
    "artifactKind": text("Artifact kind", "Artefaktart", group="source"), "artifactRef": text("Artifact reference", "Artefaktverweis", group="source"),
    "newPrintTarget": text("Print target", "Druckziel", group="output", description=D("Output intent, e.g. cmyk; empty clears it.", "Ausgabeziel, z. B. cmyk; leer entfernt es.")),
    "newJson": multiline("Data fields (JSON)", "Datenfelder (JSON)", group="data", description=D("JSON object of the document's data fields; empty clears them.", "JSON-Objekt der Datenfelder des Dokuments; leer entfernt sie.")),
    "newPath": text("Link path", "Verknüpfungspfad", group="source"),
    "story": record("Story", "Textfluss"), "styleRuns": record("Style runs", "Formatbereiche"),
    "start": count("Start", "Beginn", group="range", description=D("First character offset of the run.", "Zeichenposition, an der der Bereich beginnt.")),
    "end": count("End", "Ende", group="range", description=D("Character offset after the run.", "Zeichenposition nach dem Bereich.")),
}
LAYOUT_PATH = {
    "/newWidth@resize-frame": S("Width", "Breite", widget="stepper", group="geometry", **MM),
    "/newName@rename-layout": text("Layout name", "Layoutname", group="identity"),
    "/pixelWidth": count("Pixel width", "Pixelbreite", unit="px", group="source"),
    "/link/width": count("Pixel width", "Pixelbreite", unit="px", group="source"), "/link/height": count("Pixel height", "Pixelhöhe", unit="px", group="source"),
    "/name@update-layer": text("Layer name", "Ebenenname", group="identity"),
    "/name@update-paragraph-style": text("Style name", "Formatname", group="identity"),
}

OS_CONFIG = {
    "appearance": S("Appearance", "Erscheinungsbild", widget="segmented", options={"system": L("System", "System"), "light": L("Light", "Hell"), "dark": L("Dark", "Dunkel")}, group="shell"),
    "layout": S("Layout", "Layout", widget="segmented", options={"desktop": L("Desktop", "Desktop"), "tablet": L("Tablet", "Tablet")}, group="shell"),
    "driverId": text("Driver", "Treiber", group="shell", description=D("Id of the input driver; empty resets to the default.", "ID des Eingabetreibers; leer setzt auf den Standard zurück.")),
    "driver": record("Driver", "Treiber"),
    "locale": S("Language", "Sprache", widget="segmented", options={"en": L("English", "Englisch"), "de": L("German", "Deutsch")}, group="shell"),
    "terminology": text("Terminology", "Terminologie", group="shell", description=D("Terminology the shell names things with; empty resets it.", "Terminologie, mit der die Oberfläche Dinge benennt; leer setzt sie zurück.")),
    "themeId": text("Theme", "Design", group="shell"), "theme": record("Theme", "Design"),
    "label": text("Label", "Bezeichnung", group="identity"), "config": record("Configuration", "Konfiguration"),
    "controlId": text("Control", "Steuerelement", group="keybinding", description=D("Id of the remappable control.", "ID des umbelegbaren Steuerelements.")),
    "keys": text("Key binding", "Tastenkürzel", group="keybinding", description=D("Chord such as Ctrl+Shift+Z; empty restores the default.", "Tastenkombination wie Strg+Umschalt+Z; leer stellt den Standard wieder her.")),
    "appId": text("App", "App", group="identity"), "layoutId": text("Layout ID", "Layout-ID", group="identity"),
    "root": record("Root", "Wurzel"),
    "documentId": text("Document ID", "Dokument-ID", group="identity"), "schema": text("Schema", "Schema", group="identity"), "name": text("Name", "Name", group="identity"),
    "storage": S("Storage", "Speicherform", widget="segmented", options={"folder": L("Folder", "Ordner"), "file": L("File", "Datei")}, group="storage", description=D("Folder: a folder event log; file: one document file.", "Ordner: ein Ereignisprotokoll im Ordner; Datei: eine einzelne Dokumentdatei.")),
    "target": text("Location", "Speicherort", group="storage"),
    "admittedAtMs": count("Admitted at", "Aufgenommen am", unit="ms", group="metadata", description=D("Unix time in milliseconds.", "Unix-Zeit in Millisekunden.")),
    "policy": record("Merge policy", "Zusammenführungsrichtlinie"),
    "maxRunsBeforeMerge": count("Runs before merge", "Läufe vor dem Zusammenführen", group="policy", description=D("Most runs kept apart before they are merged.", "Höchstzahl getrennt gehaltener Läufe vor dem Zusammenführen.")),
    "dialect": record("Artifact dialect", "Artefaktdialekt"),
    "role": S("App role", "App-Rolle", widget="segmented", options={"viewer": L("Viewer", "Betrachter"), "editor": L("Editor", "Editor")}, group="app"),
    "app": record("App", "App"),
    "userId": text("User ID", "Benutzer-ID", group="identity"), "email": text("Email", "E-Mail", group="identity"), "displayName": text("Display name", "Anzeigename", group="identity"),
    "hubBaseUrl": text("Hub URL", "Hub-URL", group="hub"),
    "issuedAtMs": count("Issued at", "Ausgestellt am", unit="ms", group="metadata", description=D("Unix time in milliseconds.", "Unix-Zeit in Millisekunden.")),
    "kind": S("Direction", "Richtung", widget="segmented", options={"row": L("Row", "Zeile"), "column": L("Column", "Spalte")}, group="layout"),
    "size": coordinate("Size", "Größe", step=0.05, precision=2, group="layout", description=D("Share of the parent's extent.", "Anteil an der Ausdehnung des Elternknotens.")),
    "children": record("Children", "Kinder"),
    "windowKindId": text("Window kind", "Fensterart", group="window"), "title": text("Title", "Titel", group="window"),
    "instanceId": text("Window instance", "Fensterinstanz", group="window"), "templateId": text("Template", "Vorlage", group="window"),
    "corner": S("Corner", "Ecke", widget="select", group="window", options={"topLeft": L("Top left", "Oben links"), "topRight": L("Top right", "Oben rechts"), "bottomLeft": L("Bottom left", "Unten links"), "bottomRight": L("Bottom right", "Unten rechts")}),
}
OS_CONFIG_VARIANTS = {
    "setAppearance": L("Appearance", "Erscheinungsbild"), "setLayout": L("Layout", "Layout"), "setDriver": L("Driver", "Treiber"), "setCustomDriver": L("Custom driver", "Eigener Treiber"),
    "setLocale": L("Language", "Sprache"), "setTerminology": L("Terminology", "Terminologie"), "setTheme": L("Theme", "Design"), "setCustomTheme": L("Custom theme", "Eigenes Design"),
    "setKeybindingOverride": L("Key binding", "Tastenkürzel"), "setNamedLayout": L("Named layout", "Benanntes Layout"),
    "WindowLayoutAxisNode": L("Split", "Aufteilung"), "stack": L("Window stack", "Fensterstapel"),
}
OS_CONFIG_PATH = {
    "/driverId@set-custom-driver": text("Driver ID", "Treiber-ID", group="identity"), "/themeId@set-custom-theme": text("Theme ID", "Design-ID", group="identity"),
    "/driverId@setCustomDriver": text("Driver ID", "Treiber-ID", group="identity"), "/themeId@setCustomTheme": text("Theme ID", "Design-ID", group="identity"),
    "/layout@set-named-layout": record("Named layout", "Benanntes Layout"), "/layout@setNamedLayout": record("Named layout", "Benanntes Layout"), "/layout/layout": record("Window layout", "Fensterlayout"),
    "/layout/label": text("Label", "Bezeichnung", group="identity"),
    "/driver/driverId": text("Driver ID", "Treiber-ID", group="identity"), "/theme/themeId": text("Theme ID", "Design-ID", group="identity"),
    "/mutation@ui-preferences": {"label": L("Preference", "Einstellung")},
}
GIS_WINDOW = {
    "cameraJson": multiline("Camera (JSON)", "Kamera (JSON)", group="camera", description=D("Serialized map camera.", "Serialisierte Kartenkamera.")),
    "layerId": text("Layer", "Ebene", group="layer"), "visible": toggle("Visible", "Sichtbar", group="layer", description=D("Empty follows the layer default.", "Leer folgt der Voreinstellung der Ebene.")),
}
GIS_WINDOW_PATH = {
    "/value@set-vector-style": text("Vector style", "Vektorstil", group="style"),
    "/value@set-layer-stroke-scale": coordinate("Stroke scale", "Linienstärkenfaktor", step=0.1, precision=2, group="layer", description=D("Factor on the layer's line widths; empty resets it.", "Faktor auf die Linienstärken der Ebene; leer setzt ihn zurück.")),
    "/value@set-lod-mode": text("Level-of-detail mode", "Detailstufenmodus", group="view"),
    "/value@set-render-mode": text("Render mode", "Darstellungsmodus", group="view"),
}

DAG = {
    "id": target("node"), "x": coordinate("X", "X", step=1, precision=2, group="position"), "y": coordinate("Y", "Y", step=1, precision=2, group="position"),
    "newId": text("New ID", "Neue ID", group="identity"), "node": record("Node", "Knoten"), "index": record("Index", "Index"),
    "width": coordinate("Width", "Breite", step=1, precision=2, group="geometry"), "height": coordinate("Height", "Höhe", step=1, precision=2, group="geometry"),
    "order": record("Node order", "Knotenreihenfolge", description=D("Every node id in its new order.", "Alle Knoten-IDs in ihrer neuen Reihenfolge.")),
    "newKind": record("Node kind", "Knotenart"), "source": target("node", "Source node", "Quellknoten"), "target": target("node", "Target node", "Zielknoten"),
    "routeStyle": S("Route style", "Linienführung", widget="segmented", group="edge", options={"bezier": L("Curved", "Gebogen"), "sharpSz": L("Orthogonal (S/Z)", "Rechtwinklig (S/Z)")}), "properties": record("Properties", "Eigenschaften"), "newProperties": record("Properties", "Eigenschaften"),
    "newAbbreviation": text("Abbreviation", "Abkürzung"), "newName": text("Name", "Name", group="identity"), "newIcon": text("Icon", "Symbol"),
    "newOperatorKind": text("Operator kind", "Operatorart", group="kind", description=D("Empty makes the node a plain value node.", "Leer macht den Knoten zu einem reinen Wertknoten.")),
}
DAG_PATH = {"/id@connect-nodes": text("Edge ID", "Kanten-ID", group="identity", description=D("Id of the new edge.", "ID der neuen Kante.")), "/id@disconnect-nodes": target("edge", "Edge", "Kante")}
FLOW = {
    "id": target("widget"), "toIndex": record("Target position", "Zielposition"), "hostSnapshot": record("Host snapshot", "Host-Momentaufnahme"),
    "index": record("Index", "Index"), "widget": record("Widget", "Widget"), "entries": record("Layout entries", "Layouteinträge"), "synapse": record("Synapse", "Synapse"),
}
FLOW_PATH = {"/id@remove-synapse": target("synapse"), "/id@move-synapse": target("synapse"), "/id@change-synapse": target("synapse")}
WORKFLOW = {
    "node_id": target("node", "Node", "Knoten"), "x": coordinate("X", "X", step=1, precision=2, group="position"), "y": coordinate("Y", "Y", step=1, precision=2, group="position"),
    "port_id": target("port", "Port", "Anschluss"), "edge_id": target("edge", "Connection", "Verbindung"), "label": text("Label", "Bezeichnung"),
    "node": record("Node", "Knoten"), "binding": record("Binding", "Bindung"), "input": record("Input", "Eingang"), "edge": record("Connection", "Verbindung"),
    "field_path": text("Field path", "Feldpfad", group="binding"), "input_id": target("input", "Input", "Eingang"),
    "parameter": record("Parameter", "Parameter"), "parameter_id": target("parameter", "Parameter", "Parameter"),
}
RUN = {
    "nodeId": target("node", "Node", "Knoten"), "nodeRecord": record("Node record", "Knotenprotokoll"),
    "status": S("Status", "Status", widget="select", group="state", options={
        "computed": L("Computed", "Berechnet"), "cacheHit": L("Cache hit", "Cache-Treffer"), "failed": L("Failed", "Fehlgeschlagen"),
        "pending": L("Pending", "Ausstehend"), "running": L("Running", "Läuft"), "succeeded": L("Succeeded", "Erfolgreich"), "canceled": L("Canceled", "Abgebrochen"),
    }),
    "documentFingerprint": text("Document fingerprint", "Dokument-Fingerabdruck", group="fingerprint"), "configFingerprint": text("Configuration fingerprint", "Konfigurations-Fingerabdruck", group="fingerprint"),
    "inputFingerprints": record("Input fingerprints", "Eingangs-Fingerabdrücke"), "outputFingerprints": record("Output fingerprints", "Ausgangs-Fingerabdrücke"),
    "portId": text("Port", "Anschluss", group="identity"), "fingerprint": text("Fingerprint", "Fingerabdruck", group="fingerprint"),
    "outputs": record("Outputs", "Ausgaben"), "artifactId": text("Artifact ID", "Artefakt-ID", group="identity"), "path": text("Path", "Pfad", group="source"),
    "durationMs": coordinate("Duration", "Dauer", unit="ms", step=1, precision=0, group="timing"),
    "workflowRef": text("Workflow", "Workflow", group="source"), "workflowCheckpointId": text("Workflow checkpoint", "Workflow-Checkpoint", group="source"),
    "inputCollectionRef": text("Input collection", "Eingabesammlung", group="source"), "inputSnapshotId": text("Input snapshot", "Eingabe-Momentaufnahme", group="source"),
    "parameterValues": record("Parameter values", "Parameterwerte"), "parameterId": text("Parameter", "Parameter", group="identity"), "value": text("Value", "Wert"),
    "outputCollectionRef": text("Output collection", "Ausgabesammlung", group="source"), "trigger": record("Trigger", "Auslöser"),
    "actor": text("Actor", "Akteur", group="identity"), "automationRef": text("Automation", "Automatisierung", group="source"),
    "eventFingerprint": text("Event fingerprint", "Ereignis-Fingerabdruck", group="fingerprint"),
    "level": text("Level", "Stufe", group="log"), "message": multiline("Message", "Nachricht", group="log"), "at": text("Time", "Zeitpunkt", group="log"),
}
RUN_VARIANTS = {"manual": L("Manual", "Manuell"), "automation": L("Automation", "Automatisierung")}
RUN_PATH = {"/trigger/kind": {"label": L("Trigger kind", "Auslöserart")},"/status@seal-run": S("Run status", "Laufstatus", widget="select", group="state", options={
    "pending": L("Pending", "Ausstehend"), "running": L("Running", "Läuft"), "succeeded": L("Succeeded", "Erfolgreich"), "failed": L("Failed", "Fehlgeschlagen"), "canceled": L("Canceled", "Abgebrochen")})}
STORE = {
    "alternative": record("Alternative", "Alternative"), "id": text("ID", "ID", group="identity"), "name": text("Name", "Name", group="identity"),
    "checkpointIds": target("checkpoint", "Checkpoints", "Checkpoints"), "alternativeId": target("alternative", "Alternative", "Alternative"),
    "checkpoint": record("Checkpoint", "Checkpoint"), "parentId": target("checkpoint", "Parent checkpoint", "Vorgänger-Checkpoint"),
    "message": multiline("Message", "Nachricht", group="metadata"), "authors": record("Authors", "Autoren"),
    "timestamp": record("Timestamp", "Zeitstempel"), "actor": count("Actor", "Akteur", group="clock"),
    "physicalMs": count("Physical time", "Physikalische Zeit", unit="ms", group="clock"), "logical": count("Logical counter", "Logischer Zähler", group="clock"),
    "members": record("Members", "Mitglieder"), "documentId": text("Document", "Dokument", group="identity"), "checkpointId": target("checkpoint", "Checkpoint", "Checkpoint"),
}
STORE_PATH = {"/checkpoint/members/-/alternativeId": text("Alternative", "Alternative", group="identity"), "/checkpoint/members/-/checkpointId": text("Checkpoint", "Checkpoint", group="identity")}
INTERACTION = {
    "selection": record("Selection", "Auswahl"), "hover": record("Hover", "Hover"),
    "activeMode": record("Active selection mode", "Aktiver Auswahlmodus"), "activeGranularity": record("Active granularity", "Aktive Granularität"),
}
#endregion 🔖️Vocabulary


#region 🔖️Families
class Family:
    """🧭️ One artifact's vocabulary: its interaction domain, the entity kinds it selects, field specs and per-pointer overrides."""

    def __init__(self, name, fields, domain=None, granularities=(), entities=None, paths=None, variants=None, leaf_entities=(), id_entity=None):
        self.name, self.fields, self.domain = name, fields, domain
        self.granularities = granularities if isinstance(granularities, dict) else {kind: kind for kind in granularities}
        self.entities, self.paths, self.variants = entities or {}, paths or {}, variants or {}
        self.leaf_entities, self.id_entity = leaf_entities, id_entity

    def ref(self, kind):
        body = {"kind": kind}
        if self.domain and kind in self.granularities:
            body["domain"] = self.domain
            body["granularity"] = self.granularities[kind]
        return body

    def entity_label(self, kind):
        return self.entities.get(kind) or FALLBACK_ENTITY_LABELS.get(kind) or fail("%s: no label for entity %s" % (self.name, kind))


FALLBACK_ENTITY_LABELS = {
    "handle": L("Handle", "Griff"), "handleKind": L("Handle kind", "Griffart"), "compatibilityRule": L("Compatibility rule", "Kompatibilitätsregel"), "author": L("Author", "Autor"),
    "vortex": L("Vortex", "Wirbel"), "vortexKind": L("Vortex kind", "Wirbelart"), "representation": L("Representation", "Darstellung"),
    "grip": L("Grip", "Griff"), "gripKind": L("Grip kind", "Griffart"), "tile": L("Tile", "Kachel"), "slot": L("Slot", "Slot"), "rule": L("Adjacency rule", "Nachbarschaftsregel"),
    "edge": L("Connection", "Verbindung"), "node": L("Node", "Knoten"), "widget": L("Widget", "Widget"), "synapse": L("Synapse", "Synapse"),
    "port": L("Port", "Anschluss"), "input": L("Input", "Eingang"), "parameter": L("Parameter", "Parameter"), "alternative": L("Alternative", "Alternative"), "checkpoint": L("Checkpoint", "Checkpoint"),
}
FEM2D_KINDS = ("node", "element", "region", "support", "load", "material", "section", "loadCase", "combination")
FEM3D_KINDS = ("node", "element", "solid", "support", "load", "material", "section", "loadCase", "combination")
FEM_LEAF_ENTITIES = (("load-case", "loadCase"), ("combination", "combination"), ("element", "element"), ("node", "node"), ("section", "section"), ("region", "region"), ("solid", "solid"), ("support", "support"), ("material", "material"), ("load", "load"))
BLOCK2D_LEAF_ENTITIES = (("handle-kind", "handleKind"), ("handle", "handle"), ("compatibility-rule", "compatibilityRule"), ("author", "author"))
BLOCK3D_LEAF_ENTITIES = (("vortex-kind", "vortexKind"), ("vortex", "vortex"), ("representation", "representation"), ("compatibility-rule", "compatibilityRule"), ("author", "author"))
BLOCK5D_LEAF_ENTITIES = (("grip-kind", "gripKind"), ("grip2d", "grip"), ("grip3d", "grip"), ("grip", "grip"), ("representation", "representation"), ("compatibility-rule", "compatibilityRule"), ("author", "author"))
WFC_LEAF_ENTITIES = (("slots", "edge"), ("slot", "slot"), ("tile", "tile"), ("rule", "rule"))
LAYOUT_LEAF_ENTITIES = (("paragraph-style", "paragraphStyle"), ("character-style", "characterStyle"), ("page", "page"), ("link", "link"), ("story", "story"), ("frame", "frame"), ("layer", "layer"))
DAG_LEAF_ENTITIES = (("node", "node"),)
FLOW_LEAF_ENTITIES = (("widget", "widget"), ("synapse", "synapse"))


def fail(message):
    raise SystemExit("🚫️ " + message)


def family_of(path):
    """🧭️ The vocabulary family of a leaf payload schema path, or None for a path outside this group."""
    if "/🏗️fem/" in path:
        if "/◻️2d/" in path:
            return Family("fem2d", {**FEM_COMMON, **FEM2D_EXTRA}, "fem2d", FEM2D_KINDS, FEM_ENTITIES, paths=FEM_PATH, variants=FEM_VARIANTS, leaf_entities=FEM_LEAF_ENTITIES)
        return Family("fem3d", {**FEM_COMMON, **FEM3D_EXTRA}, "fem3d", FEM3D_KINDS, FEM_ENTITIES, paths=FEM_PATH, variants=FEM_VARIANTS, leaf_entities=FEM_LEAF_ENTITIES)
    if "/🧱️block/" in path:
        if "/◻️2d/" in path:
            return Family("block2d", {**BLOCK_COMMON, **BLOCK2D}, "handle", ("handle", "handleKind"), paths=BLOCK2D_PATH, leaf_entities=BLOCK2D_LEAF_ENTITIES)
        if "/🧊️3d/" in path:
            return Family("block3d", {**BLOCK_COMMON, **BLOCK3D}, "vortex", ("vortex",), paths=BLOCK3D_PATH, leaf_entities=BLOCK3D_LEAF_ENTITIES)
        return Family("block5d", {**BLOCK_COMMON, **BLOCK5D}, "grip", ("grip", "gripKind"), paths=BLOCK5D_PATH, leaf_entities=BLOCK5D_LEAF_ENTITIES)
    if "/🀄️wfc/" in path:
        if "/🖼️bitmap/" in path:
            return Family("wfcBitmap", {**WFC_COMMON, **WFC_BITMAP}, paths=WFC_BITMAP_PATH)
        if "/🔲️grid2d/" in path:
            return Family("wfcGrid2d", {**WFC_COMMON, **WFC_GRID}, paths=WFC_GRID2D_PATH, leaf_entities=WFC_LEAF_ENTITIES)
        if "/🧱️grid3d/" in path:
            return Family("wfcGrid3d", {**WFC_COMMON, **WFC_GRID}, leaf_entities=WFC_LEAF_ENTITIES)
        return Family("wfcGraph", WFC_COMMON, "slot", ("slot",), paths=WFC_GRAPH_PATH, variants=WFC_VARIANTS, leaf_entities=WFC_LEAF_ENTITIES)
    if "/📏️layout/" in path:
        return Family("layout", LAYOUT, "elements", {"frame": "element"}, LAYOUT_ENTITIES, paths=LAYOUT_PATH, leaf_entities=LAYOUT_LEAF_ENTITIES)
    if "/🌍️gis/" in path:
        return Family("gisWindow", GIS_WINDOW, paths=GIS_WINDOW_PATH)
    if "/💻️os/🎚️config/" in path:
        return Family("osConfig", OS_CONFIG, paths=OS_CONFIG_PATH, variants=OS_CONFIG_VARIANTS)
    if "/🕸️dag/" in path:
        return Family("dag", DAG, paths=DAG_PATH, leaf_entities=DAG_LEAF_ENTITIES)
    if "/🌊️flow/" in path:
        return Family("flow", FLOW, paths=FLOW_PATH, leaf_entities=FLOW_LEAF_ENTITIES)
    if "/🏃️run/" in path:
        return Family("run", RUN, paths=RUN_PATH, variants=RUN_VARIANTS)
    if "/🔁️workflow/" in path:
        return Family("workflow", WORKFLOW)
    if "/🏪️store/" in path:
        return Family("store", STORE, paths=STORE_PATH)
    if "/🕹️interaction/" in path:
        return Family("interaction", INTERACTION)
    return None
#endregion 🔖️Families


#region 🔖️Walker
def kind_of(leaf_dir):
    for at, character in enumerate(leaf_dir):
        if character.isascii() and character.isalpha():
            return leaf_dir[at:]
    return leaf_dir


def leaf_entity(family, leaf):
    """🎯️ The entity kind a leaf's top-level `id` addresses: the longest entity stem right after the verb."""
    rest = leaf.split("-", 1)[1] if "-" in leaf else leaf
    for stem, kind in sorted(family.leaf_entities, key=lambda pair: -len(pair[0])):
        if rest == stem or rest.startswith(stem + "-") or rest == stem + "s":
            return kind
    return None


def input_type(node):
    kind = node.get("type")
    names = [kind] if isinstance(kind, str) else [name for name in kind if name != "null"] if isinstance(kind, list) else ["object"] if "properties" in node else ["array"] if "items" in node else ["string"] if isinstance(node.get("enum"), list) and node["enum"] and all(isinstance(value, str) for value in node["enum"]) else []
    return names[0] if len(names) == 1 else None


class Leaf:
    """🌿️ One leaf payload schema being annotated: the document, its vocabulary family, and the collected edits (schema path → x-semio-ui)."""

    def __init__(self, path, document, family):
        self.path, self.doc, self.family = path, document, family
        self.kind = kind_of(os.path.basename(os.path.dirname(os.path.dirname(path))))
        self.edits = {}
        self.visited = set()
        self.unknown = []
        self.variant = None

    def at(self, spath):
        node = self.doc
        for segment in spath:
            node = node[segment]
        return node

    def emit(self, spath, ui):
        previous = self.edits.get(spath)
        if previous is not None and previous != ui:
            fail("%s: conflicting annotations at %s: %s vs %s" % (self.path, "/".join(map(str, spath)), previous, ui))
        self.edits[spath] = ui

    def resolve(self, node, spath):
        for _ in range(32):
            if not isinstance(node, dict):
                return node, spath, False
            reference = node.get("$ref")
            if isinstance(reference, str):
                if not reference.startswith("#/"):
                    return node, spath, True
                segments = tuple(part.replace("~1", "/").replace("~0", "~") for part in reference[2:].split("/"))
                node, spath = self.at(segments), segments
                continue
            union_key = "oneOf" if isinstance(node.get("oneOf"), list) else "anyOf" if isinstance(node.get("anyOf"), list) else None
            if union_key:
                concrete = [(index, branch) for index, branch in enumerate(node[union_key]) if not (isinstance(branch, dict) and branch.get("type") == "null" and len(branch) == 1)]
                if len(concrete) == 1 and len(concrete) < len(node[union_key]):
                    index, node = concrete[0]
                    spath = spath + (union_key, index)
                    continue
            return node, spath, False
        fail("%s: $ref chain too deep" % self.path)

    def spec(self, key, pointer):
        fam = self.family
        for probe in ("%s@%s" % (pointer, self.variant), "%s@%s" % (pointer, self.kind), pointer, "%s@%s" % ("/" + key, self.kind)):
            if probe in fam.paths:
                return dict(fam.paths[probe])
        if key == "id" and pointer == "/id" and fam.name not in ("store",):
            entity = leaf_entity(fam, self.kind)
            if entity is not None:
                return target(entity)
        if key == "id":
            return text("ID", "ID", group="identity", description=D("Unique id of the record.", "Eindeutige ID des Eintrags."))
        if key in fam.fields:
            return dict(fam.fields[key])
        self.unknown.append(pointer)
        return None

    def annotate_input(self, key, prop, spath, pointer, order):
        node, npath, external = self.resolve(prop, spath)
        if isinstance(node, dict) and "const" in node:
            override = next((self.family.paths[probe] for probe in ("%s@%s" % (pointer, self.kind), pointer) if probe in self.family.paths), None)
            label = override["label"] if override else DISCRIMINATOR["label"] if key == "mutation" else L("Kind", "Art")
            self.emit(spath, {"widget": "hidden", "role": "discriminator", "label": label})
            return
        spec = self.spec(key, pointer)
        if spec is None:
            return
        ui, items_options = self.ui(spec, node if isinstance(node, dict) else {}, key, pointer, order, external)
        self.emit(spath, ui)
        if not external and isinstance(node, dict):
            self.descend(node, npath, pointer, items_options)

    def ui(self, spec, node, key, pointer, order, external):
        kind = None if external else input_type(node)
        body = {}
        entity = spec.pop("target", None)
        items_options = spec.pop("options_items", None)
        if entity is not None:
            if kind not in ("string", "array") and not external:
                fail("%s %s: target %s is not string-shaped" % (self.path, pointer, entity))
            body["widget"] = "reference"
            body["role"] = "target"
            body["label"] = spec.pop("label", None) or self.family.entity_label(entity)
            body["ref"] = self.family.ref(entity)
            spec.setdefault("group", "target")
        body.update({name: value for name, value in spec.items() if value is not None})
        body.setdefault("role", "value")
        if "widget" not in body:
            if kind == "boolean":
                body["widget"] = "toggle"
            elif kind == "integer":
                body.update({"widget": "stepper", "step": body.get("step", 1), "precision": body.get("precision", 0)})
            elif kind == "string" and not node.get("enum"):
                body["widget"] = "text"
        numeric = kind in ("integer", "number")
        if not numeric and not external:
            stray = [name for name in body if name in NUMBER_KEYS and not (name == "step" and kind == "array")]
            if stray:
                fail("%s %s: numeric keys %s on a %s" % (self.path, pointer, stray, kind))
            if "unit" in body and kind != "array":
                fail("%s %s: unit on a %s" % (self.path, pointer, kind))
        if body.get("widget") in ("slider", "stepper", "dial") and not numeric and not external:
            fail("%s %s: widget %s on a %s" % (self.path, pointer, body["widget"], kind))
        if body.get("widget") == "vector" and not (kind == "array" and isinstance(node.get("items"), dict) and node.get("minItems") == node.get("maxItems")):
            fail("%s %s: vector widget on a non-fixed array" % (self.path, pointer))
        enum = node.get("enum") if not external else None
        if enum:
            options = body.get("options") or {}
            missing = [value for value in enum if value not in options]
            if missing:
                fail("%s %s: options lack labels for %s" % (self.path, pointer, missing))
            body["options"] = {value: options[value] for value in enum}
            body["widget"] = body.get("widget") or ("segmented" if len(enum) <= 3 else "select")
        elif "options" in body and not external:
            fail("%s %s: options on a non-enum" % (self.path, pointer))
        if body.get("widget") in ("select", "segmented") and not enum and not external:
            fail("%s %s: %s without enum" % (self.path, pointer, body["widget"]))
        body.setdefault("group", "record" if kind in ("object", "array", None) else "value")
        body["order"] = order
        return {name: body[name] for name in KEY_ORDER if name in body}, items_options

    def descend(self, node, npath, pointer, items_options=None):
        kind = input_type(node)
        if kind == "object" and ("properties" in node or "allOf" in node):
            self.fields(node, npath, pointer)
        elif kind == "array" and isinstance(node.get("items"), dict):
            items, ipath, external = self.resolve(node["items"], npath + ("items",))
            if external or not isinstance(items, dict):
                return
            if isinstance(items.get("enum"), list):
                if items_options is None:
                    fail("%s %s/-: enum items without option labels" % (self.path, pointer))
                self.emit(ipath, {"options": {value: items_options[value] for value in items["enum"]}})
            self.descend(items, ipath, pointer + "/-")
        elif kind is None:
            self.branches(node, npath, pointer)

    def branches(self, node, npath, pointer):
        union_key = "oneOf" if isinstance(node.get("oneOf"), list) else "anyOf" if isinstance(node.get("anyOf"), list) else None
        if union_key is None:
            return
        for index, branch in enumerate(node[union_key]):
            if isinstance(branch, dict) and branch.get("type") == "null" and len(branch) == 1:
                continue
            member, mpath, external = self.resolve(branch, npath + (union_key, index))
            if external or not isinstance(member, dict) or "properties" not in member:
                continue
            tag = next((value["const"] for value in member["properties"].values() if isinstance(value, dict) and isinstance(value.get("const"), str)), None)
            label = self.family.variants.get(tag) or self.family.variants.get(mpath[-1] if mpath else None)
            if label is None:
                fail("%s %s: union branch %s (%s) has no variant label" % (self.path, pointer, tag, mpath[-1] if mpath else None))
            if mpath not in self.visited:
                self.emit(mpath, {"label": label})
            outer, self.variant = self.variant, tag if pointer == "" else self.variant
            self.fields(member, mpath, pointer)
            self.variant = outer

    def fields(self, node, npath, pointer):
        if npath in self.visited:
            return
        self.visited.add(npath)
        for order, (key, prop) in enumerate(node.get("properties", {}).items(), start=1):
            self.annotate_input(key, prop, npath + ("properties", key), "%s/%s" % (pointer, key.replace("~", "~0").replace("/", "~1")), order * 10)
        for index, member in enumerate(node.get("allOf", [])):
            resolved, mpath, external = self.resolve(member, npath + ("allOf", index))
            if not external and isinstance(resolved, dict):
                self.fields(resolved, mpath, pointer)

    def run(self):
        root = self.doc
        if "properties" in root or "allOf" in root:
            self.fields(root, (), "")
        elif isinstance(root.get("oneOf"), list):
            self.branches(root, (), "")
        return self.edits
#endregion 🔖️Walker


#region 🔖️Writer
class Span:
    """✂️ One parsed JSON value with its byte span; objects keep their members (key, value span) in document order."""

    def __init__(self, kind, start):
        self.kind, self.start, self.end, self.members, self.items = kind, start, None, [], []


def parse_spans(text):
    at = 0

    def skip():
        nonlocal at
        while at < len(text) and text[at] in " \t\r\n":
            at += 1

    def string():
        nonlocal at
        start = at
        at += 1
        while text[at] != '"':
            at += 2 if text[at] == "\\" else 1
        at += 1
        return json.loads(text[start:at])

    def value():
        nonlocal at
        skip()
        span = Span({"{": "object", "[": "array"}.get(text[at], "scalar"), at)
        if text[at] == "{":
            at += 1
            skip()
            if text[at] == "}":
                at += 1
            else:
                while True:
                    skip()
                    key = string()
                    skip()
                    at += 1
                    span.members.append((key, value()))
                    skip()
                    at += 1
                    if text[at - 1] == "}":
                        break
        elif text[at] == "[":
            at += 1
            skip()
            if text[at] == "]":
                at += 1
            else:
                while True:
                    span.items.append(value())
                    skip()
                    at += 1
                    if text[at - 1] == "]":
                        break
        elif text[at] == '"':
            string()
        else:
            match = re.compile(r"-?\d+(\.\d+)?([eE][+-]?\d+)?|true|false|null").match(text, at)
            at = match.end()
        span.end = at
        return span

    return value()


def inline(value, spaced):
    if isinstance(value, dict):
        if not value:
            return "{}"
        body = ", ".join("%s: %s" % (json.dumps(key, ensure_ascii=False), inline(item, spaced)) for key, item in value.items())
        return "{ %s }" % body if spaced else "{%s}" % body
    if isinstance(value, list):
        return "[%s]" % ", ".join(inline(item, spaced) for item in value)
    return json.dumps(value, ensure_ascii=False)


def edit_spans(text, edits):
    """✂️ Inserts every `x-semio-ui` member into its object's text in place, keeping the file's own inline/expanded layout."""
    root = parse_spans(text)
    padded_file = '{ "' in text
    patches = []
    for spath, ui in edits.items():
        span = root
        for segment in spath:
            span = dict(span.members)[segment] if isinstance(segment, str) else span.items[segment]
        if span.kind != "object":
            fail("span editor: %s is no object" % (spath,))
        if any(key == "x-semio-ui" for key, _ in span.members):
            fail("span editor: %s already carries x-semio-ui" % (spath,))
        chunk = text[span.start:span.end]
        spaced = padded_file if chunk == "{}" else "\n" in chunk or chunk.startswith("{ ")
        if not span.members:
            patches.append((span.start, span.end, "{ %s: %s }" % (json.dumps("x-semio-ui"), inline(ui, True)) if spaced else '{"x-semio-ui": %s}' % inline(ui, False)))
            continue
        last = span.members[-1][1]
        if "\n" in chunk:
            line_start = text.rfind("\n", 0, last.start) + 1
            indent = re.match(r"[ \t]*", text[line_start:]).group(0)
            patches.append((last.end, last.end, ',\n%s"x-semio-ui": %s' % (indent, inline(ui, True))))
        else:
            patches.append((last.end, last.end, ', "x-semio-ui": %s' % inline(ui, spaced)))
    for start, end, insert in sorted(patches, key=lambda patch: -patch[0]):
        text = text[:start] + insert + text[end:]
    return text


def style_of(text, document):
    for name, render in STYLES.items():
        if render(document) == text:
            return name
    return "mixed"


STYLES = {
    "pretty": lambda document: json.dumps(document, indent=2, ensure_ascii=False) + "\n",
    "pretty-bare": lambda document: json.dumps(document, indent=2, ensure_ascii=False),
    "minified": lambda document: json.dumps(document, separators=(",", ":"), ensure_ascii=False) + "\n",
    "minified-bare": lambda document: json.dumps(document, separators=(",", ":"), ensure_ascii=False),
    "oneline": lambda document: json.dumps(document, ensure_ascii=False) + "\n",
    "oneline-bare": lambda document: json.dumps(document, ensure_ascii=False),
}


def apply(text, document, edits):
    style = style_of(text, document)
    if style == "mixed":
        written = edit_spans(text, edits)
    else:
        for spath, ui in edits.items():
            node = document
            for segment in spath:
                node = node[segment]
            if "x-semio-ui" in node:
                fail("%s already carries x-semio-ui" % (spath,))
            node["x-semio-ui"] = ui
        written = STYLES[style](document)
    return style, written
#endregion 🔖️Writer


def leaf_paths():
    for root in ROOTS:
        for directory, names, files in os.walk(os.path.join(REPO, root)):
            names.sort()
            if "🧫️fixtures" in directory:
                continue
            if directory.endswith("/🧬️schema") and "🔣️.json" in files and os.path.basename(os.path.dirname(os.path.dirname(directory))) == "🧬️mutations":
                yield os.path.join(directory, "🔣️.json")


def main():
    check = "--check" in sys.argv
    preview = sys.argv[sys.argv.index("--preview") + 1] if "--preview" in sys.argv else None
    totals = {}
    unknown = []
    for path in sorted(leaf_paths()):
        family = family_of(path)
        if family is None:
            fail("no family for %s" % path)
        original = open(path, encoding="utf-8").read()
        document = json.loads(original)
        if "x-semio-ui" in original:
            print("skip (already annotated) %s" % os.path.relpath(path, REPO))
            continue
        leaf = Leaf(path, document, family)
        edits = leaf.run()
        unknown += ["%s %s %s" % (family.name, leaf.kind, pointer) for pointer in leaf.unknown]
        if not edits:
            continue
        style, written = apply(original, json.loads(original), edits)
        if json.loads(written) != _expected(json.loads(original), edits):
            fail("%s: written JSON differs from the intended annotation" % path)
        totals.setdefault(family.name, [0, 0])
        totals[family.name][0] += 1
        totals[family.name][1] += len(edits)
        if preview:
            target_path = os.path.join(preview, os.path.relpath(path, REPO))
            os.makedirs(os.path.dirname(target_path), exist_ok=True)
            with open(target_path, "w", encoding="utf-8") as handle:
                handle.write(written)
        elif not check:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(written)
    if unknown:
        print("\n".join("UNKNOWN " + line for line in unknown))
        fail("%d inputs have no annotation rule" % len(unknown))
    for name, (files, annotations) in sorted(totals.items()):
        print("%-12s %4d leaf schemas %5d annotations" % (name, files, annotations))
    print("total %d leaf schemas, %d annotations%s" % (sum(files for files, _ in totals.values()), sum(annotations for _, annotations in totals.values()), " (check only)" if check else ""))


def _expected(document, edits):
    for spath, ui in edits.items():
        node = document
        for segment in spath:
            node = node[segment]
        node["x-semio-ui"] = ui
    return document


if __name__ == "__main__":
    sys.exit(main())
