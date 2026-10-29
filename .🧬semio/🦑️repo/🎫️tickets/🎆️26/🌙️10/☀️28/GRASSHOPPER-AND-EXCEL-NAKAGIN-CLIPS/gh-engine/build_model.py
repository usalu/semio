"""Authoring aid: emits G/🧱️model/🔣️.json (catalogue, casting rules, capsule types, aggregate spec, five canvases)."""

import json
import sys
from pathlib import Path

OUT = Path(sys.argv[1])
XY = {"plane": {"origin": [0, 0, 0], "x": [1, 0, 0], "y": [0, 1, 0]}}
PT0 = {"point": [0, 0, 0]}
COL, ROW = 260, 140


def P(name, nick, typ, access="item", optional=False, default=None):
    d = {"name": name, "nickname": nick, "type": typ, "access": access}
    if optional:
        d["optional"] = True
    if default is not None:
        d["default"] = default
    return d


def C(nick, kind, cat, sub, en, de, ins, outs):
    return {"nickname": nick, "kind": kind, "category": cat, "subcategory": sub, "description": {"en": en, "de": de}, "inputs": ins, "outputs": outs}


N = lambda n, nick, **k: P(n, nick, "Number", **k)
catalogue = {}
catalogue["Number Slider"] = C("Slider", "slider", "Params", "Input", "Interactive numeric control with a domain and rounding.", "Interaktive Zahlensteuerung mit Wertebereich und Rundung.", [], [P("Number", "N", "Number", "item")])
catalogue["Boolean Toggle"] = C("Toggle", "toggle", "Params", "Input", "Two-state switch that supplies True or False.", "Schalter mit zwei Zuständen: True oder False.", [], [P("Boolean", "B", "Boolean", "item")])
catalogue["Value List"] = C("Value List", "valuelist", "Params", "Input", "Drop-down list of named values.", "Auswahlliste benannter Werte.", [], [P("Value", "V", "Generic", "item")])
catalogue["Panel"] = C("Panel", "panel", "Params", "Input", "Shows data with its paths, or supplies text when unconnected.", "Zeigt Daten mit Pfaden oder liefert Text, wenn nichts verbunden ist.", [P("Stream", "S", "Generic", "tree")], [P("Stream", "S", "Generic", "tree")])
catalogue["Param Viewer"] = C("Viewer", "viewer", "Params", "Util", "Shows the tree structure of the data: branches, paths, item counts.", "Zeigt die Baumstruktur der Daten: Äste, Pfade, Elementanzahl.", [P("Data", "D", "Generic", "tree")], [])
for t, nick, sub in (("Number", "Num", "Primitive"), ("Integer", "Int", "Primitive"), ("Text", "Txt", "Primitive"), ("Boolean", "Bool", "Primitive"), ("Domain", "Dom", "Primitive"),
                     ("Point", "Pt", "Geometry"), ("Vector", "Vec", "Geometry"), ("Plane", "Pln", "Geometry"), ("Circle", "Cir", "Geometry"), ("Line", "Ln", "Geometry"),
                     ("Curve", "Crv", "Geometry"), ("Box", "Box", "Geometry"), ("Brep", "Brep", "Geometry"), ("Geometry", "Geo", "Geometry")):
    catalogue[t + " Param"] = C(nick, "param", "Params", sub, f"Floating {t} parameter: stores or relays {t} data and casts incoming data to {t}.", f"Freier {t}-Parameter: speichert oder leitet {t}-Daten weiter und wandelt eingehende Daten in {t} um.", [P(t, nick, t, "tree")], [P(t, nick, t, "tree")])


def comp(name, nick, cat, sub, en, de, ins, outs):
    catalogue[name] = C(nick, "component", cat, sub, en, de, ins, outs)


op = lambda name, sym, en, de: comp(name, sym, "Maths", "Operators", en, de, [N("A", "A"), N("B", "B")], [N("Result", "R")])
op("Addition", "A+B", "Adds two numbers.", "Addiert zwei Zahlen.")
op("Subtraction", "A-B", "Subtracts B from A.", "Subtrahiert B von A.")
op("Multiplication", "A×B", "Multiplies two numbers.", "Multipliziert zwei Zahlen.")
op("Division", "A/B", "Divides A by B.", "Dividiert A durch B.")
comp("Negative", "Neg", "Maths", "Operators", "Flips the sign of a number.", "Kehrt das Vorzeichen um.", [N("Value", "x")], [N("Result", "y")])
comp("Radians", "Rad", "Maths", "Trig", "Converts degrees to radians.", "Rechnet Grad in Bogenmaß um.", [N("Degrees", "D")], [N("Radians", "R")])
for nm, de in (("Round", "Rundet auf die nächste ganze Zahl."), ("Floor", "Rundet ab."), ("Ceiling", "Rundet auf.")):
    comp(nm, nm, "Maths", "Operators", f"{nm} a number.", de, [N("Number", "x")], [N("Result", "y"), P("Integer", "i", "Integer")])
comp("Series", "Series", "Sets", "Sequence", "Creates a series of numbers.", "Erzeugt eine Zahlenreihe.", [N("Start", "S", default=0), N("Step", "N", default=1), P("Count", "C", "Integer", default=10)], [P("Series", "S", "Number", "list")])
comp("Range", "Range", "Sets", "Sequence", "Divides a domain into equal steps.", "Teilt eine Domain in gleiche Schritte.", [P("Domain", "D", "Domain", default={"domain": [0, 1]}), P("Steps", "N", "Integer", default=10)], [P("Range", "R", "Number", "list")])
comp("Construct Domain", "Dom", "Maths", "Domain", "Creates a domain from two numbers.", "Erzeugt eine Domain aus zwei Zahlen.", [N("Domain start", "S", default=0), N("Domain end", "E", default=1)], [P("Domain", "I", "Domain")])
comp("Divide Domain", "DivDom", "Maths", "Domain", "Divides a domain into equal segments.", "Teilt eine Domain in gleiche Segmente.", [P("Domain", "I", "Domain"), P("Count", "C", "Integer", default=10)], [P("Segments", "S", "Domain", "list")])
comp("Construct Point", "Pt", "Vector", "Point", "Creates a point from coordinates.", "Erzeugt einen Punkt aus Koordinaten.", [N("X coordinate", "X", default=0), N("Y coordinate", "Y", default=0), N("Z coordinate", "Z", default=0)], [P("Point", "Pt", "Point")])
for nm, de in (("XY Plane", "XY-Ebene"), ("XZ Plane", "XZ-Ebene"), ("YZ Plane", "YZ-Ebene")):
    comp(nm, nm[:2], "Vector", "Plane", f"World {nm} at an origin.", f"{de} an einem Ursprung.", [P("Origin", "O", "Point", default=PT0)], [P("Plane", "P", "Plane")])
comp("Plane Origin", "Pl Origin", "Vector", "Plane", "Moves the origin of a plane.", "Verschiebt den Ursprung einer Ebene.", [P("Plane", "P", "Plane", default=XY), P("Origin", "O", "Point")], [P("Plane", "P", "Plane")])
comp("Construct Plane", "Pln", "Vector", "Plane", "Creates a plane from an origin and two axes.", "Erzeugt eine Ebene aus Ursprung und zwei Achsen.", [P("Origin", "O", "Point", default=PT0), P("X-Axis", "X", "Vector", default={"vector": [1, 0, 0]}), P("Y-Axis", "Y", "Vector", default={"vector": [0, 1, 0]})], [P("Plane", "P", "Plane")])
for ax in "XYZ":
    comp(f"Unit {ax}", ax, "Vector", "Vector", f"Unit vector along {ax}, scaled by a factor.", f"Einheitsvektor in {ax}-Richtung, skaliert mit einem Faktor.", [N("Factor", "F", default=1)], [P("Unit vector", "V", "Vector")])
comp("Rotate", "Rotate", "Transform", "Euclidean", "Rotates geometry about the Z axis of a plane.", "Dreht Geometrie um die Z-Achse einer Ebene.", [P("Geometry", "G", "Generic"), N("Angle", "A"), P("Plane", "P", "Plane", default=XY)], [P("Geometry", "G", "Generic"), P("Transform", "X", "Transform")])
comp("Move", "Move", "Transform", "Euclidean", "Translates geometry by a vector.", "Verschiebt Geometrie um einen Vektor.", [P("Geometry", "G", "Generic"), P("Motion", "T", "Vector")], [P("Geometry", "G", "Generic"), P("Transform", "X", "Transform")])
comp("Orient", "Orient", "Transform", "Euclidean", "Moves geometry from a source plane onto a target plane.", "Bewegt Geometrie von einer Quellebene auf eine Zielebene.", [P("Geometry", "G", "Generic"), P("Source", "A", "Plane", default=XY), P("Target", "B", "Plane")], [P("Geometry", "G", "Generic"), P("Transform", "X", "Transform")])
comp("List Item", "Item", "Sets", "List", "Picks one item of a list by index.", "Wählt ein Element einer Liste per Index.", [P("List", "L", "Generic", "list"), P("Index", "i", "Integer", default=0), P("Wrap", "W", "Boolean", default=False)], [P("Item", "i", "Generic")])
comp("List Length", "Lng", "Sets", "List", "Counts the items of a list.", "Zählt die Elemente einer Liste.", [P("List", "L", "Generic", "list")], [P("Length", "L", "Integer")])
comp("Repeat Data", "Repeat", "Sets", "Sequence", "Repeats a list cyclically to a length.", "Wiederholt eine Liste zyklisch auf eine Länge.", [P("Data", "D", "Generic", "list"), P("Length", "L", "Integer")], [P("Data", "D", "Generic", "list")])
comp("Center Box", "Box", "Surface", "Primitive", "Creates a box centered on a base plane from three sizes.", "Erzeugt eine Box, zentriert auf einer Grundebene, aus drei Größen.", [P("Base", "B", "Plane", default=XY), N("X", "X", default=1), N("Y", "Y", default=1), N("Z", "Z", default=1)], [P("Box", "B", "Box")])
comp("Domain Box", "DBox", "Surface", "Primitive", "Creates a box from a base plane and three domains.", "Erzeugt eine Box aus Grundebene und drei Domains.", [P("Base", "B", "Plane", default=XY), P("X", "X", "Domain", default={"domain": [-0.5, 0.5]}), P("Y", "Y", "Domain", default={"domain": [-0.5, 0.5]}), P("Z", "Z", "Domain", default={"domain": [-0.5, 0.5]})], [P("Box", "B", "Box")])
comp("Cylinder", "Cyl", "Surface", "Primitive", "Creates a cylinder along the normal of a base plane.", "Erzeugt einen Zylinder entlang der Normalen einer Grundebene.", [P("Base", "B", "Plane", default=XY), N("Radius", "R", default=1), N("Length", "L", default=1)], [P("Cylinder", "C", "Brep")])
comp("Circle", "Cir", "Curve", "Primitive", "Creates a circle on a plane.", "Erzeugt einen Kreis auf einer Ebene.", [P("Plane", "P", "Plane", default=XY), N("Radius", "R", default=1)], [P("Circle", "C", "Circle")])
comp("Rectangle", "Rec", "Curve", "Primitive", "Creates a rectangle centered on a plane.", "Erzeugt ein Rechteck, zentriert auf einer Ebene.", [P("Plane", "P", "Plane", default=XY), N("X Size", "X", default=1), N("Y Size", "Y", default=1)], [P("Rectangle", "R", "Curve"), N("Length", "L")])
comp("Extrude", "Extr", "Surface", "Freeform", "Extrudes a curve along a vector.", "Extrudiert eine Kurve entlang eines Vektors.", [P("Base", "B", "Curve"), P("Direction", "D", "Vector", default={"vector": [0, 0, 1]})], [P("Extrusion", "E", "Brep")])
comp("Fillet Edge", "FilEdge", "Surface", "Freeform", "Rounds the edges of a solid with a radius.", "Rundet die Kanten eines Körpers mit einem Radius.", [P("Shape", "S", "Brep"), P("Edges", "E", "Integer", "list", optional=True), N("Radius", "R")], [P("Result", "R", "Brep")])
comp("Solid Difference", "SDiff", "Intersect", "Shape", "Subtracts solids from a solid.", "Zieht Körper von einem Körper ab.", [P("Brep A", "A", "Brep", "list"), P("Brep B", "B", "Brep", "list")], [P("Result", "R", "Brep", "list")])
comp("Volume", "Vol", "Surface", "Analysis", "Measures the volume and centroid of a closed solid.", "Misst Volumen und Schwerpunkt eines geschlossenen Körpers.", [P("Geometry", "G", "Geometry")], [N("Volume", "V"), P("Centroid", "C", "Point")])
comp("Dispatch", "Dispatch", "Sets", "Sets", "Splits a list in two by a cyclic True/False pattern.", "Teilt eine Liste anhand eines zyklischen True/False-Musters in zwei.", [P("List", "L", "Generic", "list"), P("Pattern", "P", "Boolean", "list")], [P("List A", "A", "Generic", "list"), P("List B", "B", "Generic", "list")])
comp("Merge", "Merge", "Sets", "Tree", "Merges data streams into one.", "Fügt Datenströme zu einem zusammen.", [P(f"Data {i}", f"D{i}", "Generic", "tree", optional=True) for i in range(1, 5)], [P("Result", "R", "Generic", "tree")])
comp("Entwine", "Entwine", "Sets", "Tree", "Combines data streams as separate branches of one tree.", "Verbindet Datenströme als getrennte Äste eines Baums.", [P(f"Branch {i}", f"{i}", "Generic", "tree", optional=True) for i in range(1, 4)], [P("Result", "R", "Generic", "tree")])
comp("Graft Tree", "Graft", "Sets", "Tree", "Puts every item into its own branch.", "Legt jedes Element in einen eigenen Ast.", [P("Tree", "T", "Generic", "tree")], [P("Tree", "T", "Generic", "tree")])
comp("Flatten Tree", "Flatten", "Sets", "Tree", "Moves all items into one branch.", "Legt alle Elemente in einen Ast.", [P("Tree", "T", "Generic", "tree"), P("Path", "P", "Text", optional=True)], [P("Tree", "T", "Generic", "tree")])
comp("Simplify Tree", "Simplify", "Sets", "Tree", "Removes the path indices all branches share.", "Entfernt die Pfadindizes, die alle Äste teilen.", [P("Tree", "T", "Generic", "tree"), P("Front", "F", "Boolean", optional=True)], [P("Tree", "T", "Generic", "tree")])
comp("Path Mapper", "Mapper", "Sets", "Tree", "Rewrites paths with a lexical mapping such as {A;B;C;D}(i) -> {A;B;C}.", "Schreibt Pfade mit einer lexikalischen Abbildung um, z. B. {A;B;C;D}(i) -> {A;B;C}.", [P("Tree", "T", "Generic", "tree"), P("Mapping", "M", "Text", optional=True)], [P("Tree", "T", "Generic", "tree")])
comp("Flip Matrix", "Flip", "Sets", "Tree", "Swaps branches and items of a tree.", "Vertauscht Äste und Elemente eines Baums.", [P("Data", "D", "Generic", "tree")], [P("Data", "D", "Generic", "tree")])
comp("Explode Tree", "Explode", "Sets", "Tree", "Returns each branch as its own list (the first four here).", "Gibt jeden Ast als eigene Liste aus (hier die ersten vier).", [P("Data", "D", "Generic", "tree")], [P(f"Branch {i}", f"{i}", "Generic", "list") for i in range(1, 5)])
comp("Tree Statistics", "TStat", "Sets", "Tree", "Reports paths, item counts and the number of branches.", "Meldet Pfade, Elementanzahlen und die Zahl der Äste.", [P("Tree", "T", "Generic", "tree")], [P("Paths", "P", "Text", "list"), P("Length", "L", "Integer", "list"), P("Count", "C", "Integer")])
comp("Tree Item", "TItem", "Sets", "Tree", "Picks one item by path and index.", "Wählt ein Element per Pfad und Index.", [P("Tree", "T", "Generic", "tree"), P("Path", "P", "Text", default="{0}"), P("Index", "i", "Integer", default=0), P("Wrap", "W", "Boolean", default=False)], [P("Item", "i", "Generic")])
comp("Custom Preview", "Preview", "Display", "Preview", "Previews geometry with a material (here a colour name).", "Zeigt Geometrie mit einem Material (hier ein Farbname).", [P("Geometry", "G", "Geometry"), P("Material", "M", "Generic")], [P("Preview", "P", "Brep")])

comp("Cull Index", "Cull i", "Sets", "List", "Removes the items at the given indices from a list.", "Entfernt die Elemente an den angegebenen Indizes aus einer Liste.", [P("List", "L", "Generic", "list"), P("Indices", "I", "Integer", "list", optional=True), P("Wrap", "W", "Boolean", default=False)], [P("List", "L", "Generic", "list")])
comp("Sine", "Sin", "Maths", "Trig", "Sine of an angle in radians.", "Sinus eines Winkels im Bogenmaß.", [N("Value", "x")], [N("Result", "y")])
comp("Absolute", "Abs", "Maths", "Operators", "Absolute value.", "Betrag.", [N("Value", "x")], [N("Result", "y")])
comp("Vector XYZ", "Vec", "Vector", "Vector", "Creates a vector from components.", "Erzeugt einen Vektor aus Komponenten.", [N("X component", "X", default=0), N("Y component", "Y", default=0), N("Z component", "Z", default=0)], [P("Vector", "V", "Vector"), N("Length", "L")])
comp("Deconstruct Plane", "DePlane", "Vector", "Plane", "Splits a plane into origin and axes.", "Zerlegt eine Ebene in Ursprung und Achsen.", [P("Plane", "P", "Plane")], [P("Origin", "O", "Point"), P("X-Axis", "X", "Vector"), P("Y-Axis", "Y", "Vector"), P("Z-Axis", "Z", "Vector")])
comp("Amplitude", "Amp", "Vector", "Vector", "Sets the length of a vector.", "Setzt die Länge eines Vektors.", [P("Vector", "V", "Vector"), N("Amplitude", "A")], [P("Vector", "V", "Vector")])


def R(frm, to, basis, source, en, de):
    return {"from": frm, "to": to, "ok": True, "basis": basis, "source": source, "en": en, "de": de}


def F(frm, to, basis, source, en, de):
    return {"from": frm, "to": to, "ok": False, "basis": basis, "source": source, "en": en, "de": de}


U, V, CM = "unverified", "verified", "community"
D, K, T_, RU, QZ, L = "mcneel-data-types", "forum-136013", "teaching", "rutten-2010", "quiz", "forum-plane-of-a-line"
rules = {
    "same": R("any", "same", V, D, "Same type: the data passes through unchanged.", "Gleicher Typ: Die Daten gehen unverändert durch."),
    "Integer>Number": R("Integer", "Number", U, T_, "An Integer widens to a Number without loss.", "Eine Ganzzahl wird verlustfrei zur Zahl."),
    "Boolean>Number": R("Boolean", "Number", U, T_, "False becomes 0, True becomes 1.", "False wird 0, True wird 1."),
    "Text>Number": R("Text", "Number", U, K, "Numeric text becomes a number.", "Zahlentext wird zur Zahl."),
    "Text>Number!": F("Text", "Number", V, K, "Text that is not a number cannot be read.", "Der Text lässt sich nicht als Zahl lesen; Grasshopper meldet „{gh}“ und lässt den Platz leer."),
    "Text>Number!comma": F("Text", "Number", U, K, "A decimal comma in Text may not parse as a Number; this is locale dependent.", "Ein Dezimalkomma im Text lässt sich womöglich nicht als Zahl lesen (abhängig von der Spracheinstellung); Grasshopper meldet „{gh}“ und lässt den Platz leer."),
    "Domain>Number": R("Domain", "Number", U, T_, "A Domain gives its length.", "Eine Domain liefert ihre Länge."),
    "Point>Number": R("Point", "Number", CM, "community-digest", "A Point gives its distance from the origin.", "Ein Punkt liefert seinen Abstand vom Ursprung."),
    "Number>Integer": R("Number", "Integer", "conflicting", RU, "A Number rounds to the nearest Integer; exact ties go to the even number (2.5 gives 2, 3.5 gives 4).", "Eine Zahl wird auf die nächste Ganzzahl gerundet; genau in der Mitte gewinnt die gerade Zahl (2,5 wird 2, 3,5 wird 4)."),
    "Boolean>Integer": R("Boolean", "Integer", V, D, "False becomes 0, True becomes 1.", "False wird 0, True wird 1."),
    "Text>Integer": R("Text", "Integer", U, "forum-154859", "Numeric text becomes an Integer.", "Zahlentext wird zur Ganzzahl."),
    "Text>Integer!": F("Text", "Integer", V, "forum-154859", "Text that is not a number cannot be read.", "Der Text lässt sich nicht als Ganzzahl lesen; Grasshopper meldet „{gh}“ und lässt den Platz leer."),
    "Number>Boolean": R("Number", "Boolean", U, T_, "0 becomes False, any other number True.", "0 wird False, jede andere Zahl True."),
    "Integer>Boolean": R("Integer", "Boolean", U, T_, "0 becomes False, any other number True.", "0 wird False, jede andere Zahl True."),
    "Text>Boolean": R("Text", "Boolean", U, T_, "The text True becomes True, False becomes False.", "Der Text True wird True, False wird False."),
    "Number>Text": R("Number", "Text", U, T_, "A Number becomes its text.", "Eine Zahl wird zu ihrem Text."),
    "Integer>Text": R("Integer", "Text", U, T_, "An Integer becomes its text.", "Eine Ganzzahl wird zu ihrem Text."),
    "Boolean>Text": R("Boolean", "Text", V, D, "A Boolean becomes the text false or true.", "Ein Boolean wird zum Text false oder true."),
    "Point>Text": R("Point", "Text", U, T_, "A Point becomes its text.", "Ein Punkt wird zu seinem Text."),
    "Vector>Text": R("Vector", "Text", U, T_, "A Vector becomes its text.", "Ein Vektor wird zu seinem Text."),
    "Domain>Text": R("Domain", "Text", U, T_, "A Domain becomes its text.", "Eine Domain wird zu ihrem Text."),
    "Text>Point": R("Text", "Point", U, T_, "The text 1,2,3 becomes the point (1, 2, 3).", "Der Text 1,2,3 wird zum Punkt (1, 2, 3)."),
    "Vector>Point": R("Vector", "Point", V, QZ, "A Vector keeps its coordinates as a Point.", "Ein Vektor behält seine Koordinaten als Punkt."),
    "Plane>Point": R("Plane", "Point", U, T_, "A Plane gives its origin.", "Eine Ebene liefert ihren Ursprung."),
    "Point>Vector": R("Point", "Vector", V, QZ, "A Point keeps its coordinates as a Vector.", "Ein Punkt behält seine Koordinaten als Vektor."),
    "Line>Vector": R("Line", "Vector", U, T_, "A Line gives its direction from start to end.", "Eine Linie liefert ihre Richtung von Anfang zu Ende."),
    "Text>Vector": R("Text", "Vector", U, T_, "The text 1,2,3 becomes the vector (1, 2, 3).", "Der Text 1,2,3 wird zum Vektor (1, 2, 3)."),
    "Point>Plane": R("Point", "Plane", CM, L, "A Point becomes a plane at the point with the world XY axes.", "Ein Punkt wird zur Ebene am Punkt mit den Weltachsen XY."),
    "Line>Plane": R("Line", "Plane", CM, L, "A Line becomes a plane whose Z axis follows the line; X and Y are guessed.", "Eine Linie wird zur Ebene, deren Z-Achse der Linie folgt; X und Y werden geraten."),
    "Circle>Plane": R("Circle", "Plane", U, T_, "A Circle gives its plane.", "Ein Kreis liefert seine Ebene."),
    "Curve>Plane": R("Curve", "Plane", U, T_, "A planar curve gives its plane.", "Eine ebene Kurve liefert ihre Ebene."),
    "Number>Domain": R("Number", "Domain", U, T_, "A Number x becomes the Domain 0 To x.", "Eine Zahl x wird zur Domain 0 bis x."),
    "Integer>Domain": R("Integer", "Domain", U, T_, "An Integer x becomes the Domain 0 To x.", "Eine Ganzzahl x wird zur Domain 0 bis x."),
    "Text>Domain": R("Text", "Domain", U, T_, "The text 0 To 5 becomes the Domain 0 To 5.", "Der Text 0 To 5 wird zur Domain 0 bis 5."),
    "Circle>Curve": R("Circle", "Curve", U, T_, "A Circle is accepted as a Curve.", "Ein Kreis wird als Kurve akzeptiert."),
    "Line>Curve": R("Line", "Curve", U, T_, "A Line is accepted as a Curve.", "Eine Linie wird als Kurve akzeptiert."),
    "Curve>Circle": R("Curve", "Circle", U, T_, "A circular Curve becomes a Circle.", "Eine kreisförmige Kurve wird zum Kreis."),
    "Curve>Line": R("Curve", "Line", U, T_, "A straight Curve becomes a Line.", "Eine gerade Kurve wird zur Linie."),
    "Box>Brep": R("Box", "Brep", U, T_, "A Box becomes a Brep with six faces.", "Eine Box wird zum Brep mit sechs Flächen."),
    "Brep>Box": R("Brep", "Box", U, T_, "A box-shaped Brep becomes a Box.", "Ein quaderförmiges Brep wird zur Box."),
    "*>*!": F("any", "any", V, D, "No conversion exists: Grasshopper reports an error and leaves the slot blank.", "Eine Umwandlung von {source} nach {target} gibt es nicht; Grasshopper meldet „{gh}“ und lässt den Platz leer."),
}
rules["Number>Integer"]["sources"] = ["rutten-2010", "rutten-2012"]
for g in ("Point", "Circle", "Line", "Curve", "Box", "Brep"):
    rules[f"{g}>Geometry"] = R(g, "Geometry", U, T_, f"A {g} is Geometry and passes through.", f"{g} ist Geometrie und geht durch.")
casting = {
    "types": ["Number", "Integer", "Text", "Boolean", "Point", "Vector", "Plane", "Domain", "Circle", "Line", "Curve", "Box", "Brep", "Geometry"],
    "sources": {
        "mcneel-data-types": "https://developer.rhino3d.com/guides/grasshopper/grasshopper-data-types/",
        "rutten-2010": "https://www.grasshopper3d.com/xn/detail/2985220:Comment:42581",
        "rutten-2012": "https://grasshopper3d.com/forum/topics/the-int-component",
        "forum-136013": "https://discourse.mcneel.com/t/data-conversion-failed-from-text-to-number/136013",
        "forum-154859": "https://discourse.mcneel.com/t/data-conversion-failed-from-text-to-integer/154859",
        "forum-plane-of-a-line": "https://www.grasshopper3d.com/forum/topics/plane-of-a-line",
        "community-digest": "https://www.grasshopper3d.com/forum/topics/plane-of-a-line",
        "quiz": "🎓️teaching/🏛️architecture/🦗️grasshopper/❓️quiz/🔣️.json",
        "teaching": "Teaching assumption: check in Rhino 8",
    },
    "rules": rules,
}
TOPICS = {"anatomy": [], "capsule": [], "tower": [], "casting": [], "trees": []}


def topic(canvas):
    def wrap(fn):
        TOPICS[canvas].append(fn)
        return fn

    return wrap


class Canvas:
    def __init__(self, key, en, de):
        self.key, self.title, self.objects, self.wires, self.views = key, {"en": en, "de": de}, {}, [], {}

    def obj(self, oid, component, **settings):
        self.objects[oid] = {"id": oid, "component": component, "at": None, "settings": settings}

    def slider(self, oid, label, lo, hi, value, rounding="float", decimals=2, bind=None):
        s = dict(label=label, min=lo, max=hi, value=value, rounding=rounding)
        if rounding == "float":
            s["decimals"] = decimals
        if bind:
            s["bind"] = bind
        self.obj(oid, "Number Slider", **s)

    def panel(self, oid, lines):
        self.obj(oid, "Panel", lines=lines)

    def param(self, oid, typ, label, data=None):
        s = {"label": label}
        if data is not None:
            s["data"] = data
        self.obj(oid, typ + " Param", **s)

    def wire(self, *ws):
        self.wires.extend(ws)

    def view(self, name, en, de, rows):
        layout = {}
        for oid, c, r in rows:
            assert oid in self.objects, (self.key, name, oid)
            layout[oid] = [round(c * COL), round(r * ROW)]
            if self.objects[oid]["at"] is None:
                self.objects[oid]["at"] = layout[oid]
        assert len(rows) <= 12, (self.key, name, len(rows))
        self.views[name] = {"title": {"en": en, "de": de}, "objects": [r[0] for r in rows], "layout": layout}

    def done(self):
        for fn in TOPICS[self.key]:
            fn(self)
        for o in self.objects.values():
            assert o["at"] is not None, o["id"]
        return {"title": self.title, "objects": list(self.objects.values()), "wires": self.wires, "views": self.views}


def anatomy():
    c = Canvas("anatomy", "Anatomy of a definition", "Anatomie einer Definition")
    c.slider("w", "Width", 1.0, 5.0, 2.5, decimals=1, bind="capsule.width")
    c.panel("wp", [""])
    c.slider("bl", "Length", 2.0, 6.0, 4.0, decimals=1, bind="capsule.length")
    c.slider("bh", "Height", 1.5, 4.0, 2.5, decimals=1, bind="capsule.height")
    c.obj("xy", "XY Plane")
    c.obj("cb", "Center Box")
    c.obj("vol", "Volume")
    c.panel("vp", [""])
    c.param("fl", "Number", "Width")
    c.panel("flp", [""])
    c.slider("st", "Storey height", 2.0, 4.0, 2.75, bind="core.storey_height")
    c.slider("cnt", "Count", 1, 12, 5, rounding="integer")
    c.obj("se", "Series")
    c.panel("sp", [""])
    c.panel("cp", ["2", "3"])
    c.obj("se2", "Series")
    c.panel("tp", [""])
    c.obj("pv", "Param Viewer")
    c.obj("tg", "Boolean Toggle", label="Window", value=True)
    c.panel("tgp", [""])
    c.obj("vl", "Value List", items=[{"name": f"Type {k}", "value": i} for i, k in enumerate("ABCD")], selected=0)
    c.panel("vlp", [""])
    c.panel("wn", ["2.5", "tall"])
    c.panel("er", ["tall"])
    c.obj("cbw", "Center Box")
    c.obj("cbe", "Center Box")
    c.panel("cbwp", [""])
    c.panel("cbep", [""])
    c.wire("tg.Boolean>tgp.Stream", "vl.Value>vlp.Stream", "w.Number>cbw.X", "bl.Number>cbw.Y", "wn.Stream>cbw.Z", "cbw.Box>cbwp.Stream",
           "w.Number>cbe.X", "bl.Number>cbe.Y", "er.Stream>cbe.Z", "cbe.Box>cbep.Stream")
    c.wire("w.Number>wp.Stream","w.Number>cb.X", "bl.Number>cb.Y", "bh.Number>cb.Z", "xy.Plane>cb.Base", "cb.Box>vol.Geometry", "vol.Volume>vp.Stream", "fl.Number>flp.Stream", "w.Number>fl.Number",
           "st.Number>se.Step", "cnt.Number>se.Count", "se.Series>sp.Stream", "cp.Stream>se2.Count", "se2.Series>tp.Stream", "se2.Series>pv.Data")
    c.view("slider", "Slider and Panel", "Schieberegler und Panel", [("w", 0, 0), ("wp", 1, 0)])
    c.view("box", "Sliders into Center Box", "Schieberegler in Center Box", [("w", 0, 0), ("bl", 0, 1), ("bh", 0, 2), ("xy", 0, 3), ("cb", 1, 1), ("vol", 2, 1), ("vp", 3, 1)])
    c.view("param", "A floating parameter", "Ein freier Parameter", [("w", 0, 0), ("fl", 1, 0), ("flp", 2, 0)])
    c.view("list", "A series is a list", "Eine Reihe ist eine Liste", [("st", 0, 0), ("cnt", 0, 1), ("se", 1, 0.5), ("sp", 2, 0.5)])
    c.view("tree", "Lists in a tree", "Listen in einem Baum", [("cp", 0, 0), ("se2", 1, 0), ("tp", 2, 0), ("pv", 2, 1)])
    c.view("warning", "Orange: a warning", "Orange: eine Warnung", [("w", 0, 0), ("bl", 0, 1), ("wn", 0, 2), ("cbw", 1, 1), ("cbwp", 2, 1)])
    c.view("error", "Red: an error", "Rot: ein Fehler", [("w", 0, 0), ("bl", 0, 1), ("er", 0, 2), ("cbe", 1, 1), ("cbep", 2, 1)])
    c.view("inputs", "Toggle and Value List", "Toggle und Werteliste", [("tg", 0, 0), ("tgp", 1, 0), ("vl", 0, 1), ("vlp", 1, 1)])
    return c.done()


def capsule():
    c = Canvas("capsule", "One parametric capsule", "Eine parametrische Kapsel")
    c.slider("w", "Width", 1.5, 3.5, 2.5, bind="capsule.width")
    c.slider("l", "Length", 2.5, 5.5, 4.0, bind="capsule.length")
    c.slider("h", "Height", 1.5, 3.5, 2.5, bind="capsule.height")
    c.panel("two", ["2"])
    c.obj("hl", "Division")
    c.obj("hh", "Division")
    c.obj("xy", "XY Plane")
    c.obj("bp", "Construct Point")
    c.obj("bo", "Plane Origin")
    c.obj("box", "Center Box")
    c.slider("r", "Corner radius", 0.0, 0.5, 0.25, bind="capsule.corner_radius")
    c.obj("fil", "Fillet Edge")
    c.obj("fvol", "Volume")
    c.panel("fvp", [""])
    c.slider("d", "Window diameter", 0.5, 2.0, 1.3, bind="capsule.window_diameter")
    c.slider("wu", "Window offset", -0.8, 0.8, 0.0, bind="types.A.window_u")
    c.panel("twow", ["2"])
    c.panel("wall", [{"model": "capsule_wall"}])
    c.obj("rad", "Division")
    c.obj("wpt", "Construct Point")
    c.obj("xz", "XZ Plane")
    c.obj("wpl", "Plane Origin")
    c.obj("cir", "Circle")
    c.obj("cyl", "Cylinder")
    c.obj("cut", "Solid Difference")
    c.slider("bu", "Bolt offset", 0.2, 1.1, 0.9, bind="capsule.bolt_u")
    c.slider("bvl", "Bolt low", 0.2, 1.2, 0.5, bind="capsule.bolt_v_low")
    c.slider("bvh", "Bolt high", 1.3, 2.3, 2.0, bind="capsule.bolt_v_high")
    c.obj("nbu", "Negative")
    c.obj("mx", "Merge")
    c.obj("mz", "Merge")
    c.obj("bolt", "Construct Point")
    c.param("types", "Brep", "Types", [{"capsule": k} for k in "ABCD"])
    c.panel("tp", [""])
    c.obj("vl", "Value List", items=[{"name": f"Type {k}", "value": i} for i, k in enumerate("ABCD")], selected=1)
    c.obj("li2", "List Item")
    c.panel("lp", [""])
    c.slider("side", "Core side", 3.0, 8.0, 5.5, decimals=1, bind="core.side")
    c.panel("two2", ["2"])
    c.obj("hs", "Division")
    c.obj("pp", "Construct Point")
    c.obj("pxy", "XY Plane")
    c.obj("p0", "Plane Origin")
    c.panel("ang", ["0", "-90", "-180", "-270"])
    c.obj("rad4", "Radians")
    c.obj("rot", "Rotate")
    c.panel("idx", ["0", "1", "2", "0"])
    c.obj("li", "List Item")
    c.obj("ori", "Orient")
    c.wire("l.Number>hl.A", "two.Stream>hl.B", "h.Number>hh.A", "two.Stream>hh.B", "hl.Result>bp.Y coordinate", "hh.Result>bp.Z coordinate", "xy.Plane>bo.Plane", "bp.Point>bo.Origin",
           "bo.Plane>box.Base", "w.Number>box.X", "l.Number>box.Y", "h.Number>box.Z",
           "box.Box>fil.Shape", "r.Number>fil.Radius", "fil.Result>fvol.Geometry", "fvol.Volume>fvp.Stream",
           "d.Number>rad.A", "twow.Stream>rad.B", "l.Number>wpt.Y coordinate", "hh.Result>wpt.Z coordinate", "wu.Number>wpt.X coordinate", "xz.Plane>wpl.Plane", "wpt.Point>wpl.Origin", "wpl.Plane>cyl.Base",
           "rad.Result>cyl.Radius", "wall.Stream>cyl.Length", "fil.Result>cut.Brep A", "cyl.Cylinder>cut.Brep B", "wpl.Plane>cir.Plane", "rad.Result>cir.Radius",
           "bu.Number>nbu.Value", "nbu.Result>mx.Data 1", "bu.Number>mx.Data 2", "bu.Number>mx.Data 3", "nbu.Result>mx.Data 4",
           "bvl.Number>mz.Data 1", "bvl.Number>mz.Data 2", "bvh.Number>mz.Data 3", "bvh.Number>mz.Data 4",
           "mx.Result>bolt.X coordinate", "mz.Result>bolt.Z coordinate",
           "types.Brep>tp.Stream", "types.Brep>li2.List", "vl.Value>li2.Index", "li2.Item>lp.Stream",
           "side.Number>hs.A", "two2.Stream>hs.B", "hs.Result>pp.Y coordinate", "pxy.Plane>p0.Plane", "pp.Point>p0.Origin", "ang.Stream>rad4.Degrees", "p0.Plane>rot.Geometry", "rad4.Radians>rot.Angle",
           "types.Brep>li.List", "idx.Stream>li.Index", "li.Item>ori.Geometry", "pxy.Plane>ori.Source", "rot.Geometry>ori.Target")
    c.view("body", "Capsule body from three sliders", "Kapselkörper aus drei Schiebereglern", [("w", 0, 0), ("l", 0, 1), ("h", 0, 2), ("two", 0, 3), ("hl", 1, 1), ("hh", 1, 2), ("xy", 1, 3), ("bp", 2, 1.5), ("bo", 3, 2), ("box", 4, 1)])
    c.view("round", "Rounded corners", "Gerundete Kanten", [("box", 0, 0), ("r", 0, 1), ("fil", 1, 0.5), ("fvol", 2, 0.5), ("fvp", 3, 0.5)])
    c.view("window", "Window cut", "Fensterausschnitt", [("l", 0, 0), ("hh", 0, 1), ("d", 0, 2), ("twow", 0, 3), ("wall", 0, 4), ("wpt", 1, 0), ("xz", 1, 1), ("rad", 1, 3), ("wpl", 2, 0.5), ("fil", 2, 3), ("cyl", 3, 1.5), ("cut", 4, 2.2)])
    c.view("window-circle", "Window circle", "Fensterkreis", [("l", 0, 2), ("hh", 0, 3), ("d", 0, 0), ("twow", 0, 1), ("rad", 1, 0), ("wu", 0, 4), ("wpt", 1, 2), ("xz", 1, 3), ("wpl", 2, 2), ("cir", 3, 1)])
    c.view("bolts", "Four bolts", "Vier Bolzen", [("bu", 0, 0), ("bvl", 0, 1.5), ("bvh", 0, 3), ("nbu", 1, 0), ("mx", 2, 0.5), ("mz", 2, 2.5), ("bolt", 3, 1.5)])
    c.view("types", "Capsule types", "Kapseltypen", [("types", 0, 0), ("tp", 1, 0), ("vl", 0, 1.5), ("li2", 1, 1.5), ("lp", 2, 1.5)])
    c.view("planes", "Planes around the core", "Ebenen um den Kern", [("side", 0, 0), ("two2", 0, 1), ("hs", 1, 0), ("pp", 2, 0), ("pxy", 2, 1.5), ("p0", 3, 0.5), ("ang", 2, 3), ("rad4", 3, 3), ("rot", 4, 1.5)])
    c.view("orient", "Orient a type onto each plane", "Einen Typ auf jede Ebene orientieren", [("types", 0, 0), ("idx", 0, 1), ("li", 1, 0.5), ("rot", 0, 2.5), ("pxy", 1, 2.5), ("ori", 2, 1.5)])
    return c.done()


def tower():
    c = Canvas("tower", "Core, capsules, capital and podium", "Kern, Kapseln, Kopf und Sockel")
    c.slider("side", "Core side", 3.0, 8.0, 5.5, decimals=1, bind="core.side")
    c.slider("sh", "Storey height", 2.0, 4.0, 2.75, bind="core.storey_height")
    c.slider("n", "Capsule storeys", 1, 13, 11, rounding="integer", bind="towers.B.capsule_storeys")
    c.slider("ps", "Podium storeys", 0, 4, 2, rounding="integer", bind="podium.storeys")
    c.panel("two", ["2"])
    c.obj("tn", "Addition")
    c.obj("tot", "Multiplication")
    c.obj("z0", "Multiplication")
    c.obj("hc", "Division")
    c.obj("cp", "Construct Point")
    c.obj("xy", "XY Plane")
    c.obj("co", "Plane Origin")
    c.obj("core", "Center Box")
    c.obj("ser", "Series")
    c.obj("gz", "Graft Tree")
    c.obj("uz", "Unit Z")
    c.panel("gp", [""])
    c.obj("hs", "Division")
    c.slider("pitch", "Slot pitch", 2.0, 4.0, 2.75, bind="items.pitch")
    c.obj("hp", "Division")
    c.obj("nhp", "Negative")
    c.obj("sx", "Merge")
    c.obj("sp0", "Construct Point")
    c.obj("p0", "Plane Origin")
    c.slider("stg", "Item stagger", 0.0, 1.5, 0.917, decimals=3, bind="items.stagger")
    c.panel("zero", ["0"])
    c.obj("sgm", "Merge")
    c.obj("uzs", "Unit Z")
    c.obj("p0s", "Move")
    c.panel("ang", ["0", "-90", "-180", "-270"])
    c.obj("rad", "Radians")
    c.obj("ga", "Graft Tree")
    c.obj("rot", "Rotate")
    c.obj("fl", "Flatten Tree")
    c.slider("cx", "Core centre X", -40.0, 40.0, -11.1, decimals=2, bind="towers.B.center.x")
    c.slider("cy", "Core centre Y", -40.0, 40.0, 0.0, decimals=2, bind="towers.B.center.y")
    c.obj("vc", "Vector XYZ")
    c.obj("mc", "Move")
    c.obj("mv", "Move")
    c.param("cullp", "Integer", "Cull indices", {"slot_tree": {"tower": "B", "part": "cull"}})
    c.obj("cull", "Cull Index")
    c.param("yawp", "Number", "Capsule yaw (degrees)", {"slot_tree": {"tower": "B", "part": "yaw"}})
    c.obj("yrad", "Radians")
    c.obj("yrot", "Rotate")
    c.obj("dpl", "Deconstruct Plane")
    c.obj("sin", "Sine")
    c.obj("abs", "Absolute")
    c.slider("cw2", "Capsule width", 1.5, 3.5, 2.5, bind="capsule.width")
    c.obj("hw", "Division")
    c.obj("amp", "Multiplication")
    c.obj("ampv", "Amplitude")
    c.obj("slots", "Move")
    c.param("capp", "Brep", "Capsule per slot", {"slot_tree": {"tower": "B", "part": "capsule"}})
    c.obj("ori", "Orient")
    c.obj("st", "Tree Statistics")
    c.panel("stl", [""])
    c.panel("stc", [""])
    c.slider("ow", "Capital overhang", 0.0, 1.0, 0.25, bind="capital.overhang")
    c.slider("ch", "Capital height", 1.0, 5.0, 2.75, bind="capital.height")
    c.obj("ow2", "Multiplication")
    c.obj("cw", "Addition")
    c.obj("chh", "Division")
    c.obj("czz", "Addition")
    c.obj("cpt", "Construct Point")
    c.obj("cpl", "Plane Origin")
    c.obj("capital", "Center Box")
    c.slider("pw", "Podium length", 8.0, 40.0, 31.4, decimals=1, bind="podium.length")
    c.slider("pd", "Podium width", 6.0, 30.0, 20.7, decimals=1, bind="podium.width")
    c.obj("rect", "Rectangle")
    c.obj("uzp", "Unit Z")
    c.obj("ext", "Extrude")
    c.wire("ps.Number>tn.A", "n.Number>tn.B", "sh.Number>tot.A", "tn.Result>tot.B", "ps.Number>z0.A", "sh.Number>z0.B",
           "tot.Result>hc.A", "two.Stream>hc.B", "hc.Result>cp.Z coordinate", "xy.Plane>co.Plane", "cp.Point>co.Origin", "co.Plane>core.Base",
           "side.Number>core.X", "side.Number>core.Y", "tot.Result>core.Z",
           "z0.Result>ser.Start", "sh.Number>ser.Step", "n.Number>ser.Count", "ser.Series>gz.Tree", "gz.Tree>uz.Factor", "gz.Tree>gp.Stream",
           "side.Number>hs.A", "two.Stream>hs.B", "pitch.Number>hp.A", "two.Stream>hp.B", "hp.Result>nhp.Value", "nhp.Result>sx.Data 1", "hp.Result>sx.Data 2", "sx.Result>sp0.X coordinate", "hs.Result>sp0.Y coordinate",
           "xy.Plane>p0.Plane", "sp0.Point>p0.Origin",
           "zero.Stream>sgm.Data 1", "stg.Number>sgm.Data 2", "sgm.Result>uzs.Factor", "p0.Plane>p0s.Geometry", "uzs.Unit vector>p0s.Motion",
           "ang.Stream>rad.Degrees", "rad.Radians>ga.Tree", "p0s.Geometry>rot.Geometry", "ga.Tree>rot.Angle", "rot.Geometry>fl.Tree",
           "cx.Number>vc.X component", "cy.Number>vc.Y component", "fl.Tree>mc.Geometry", "vc.Vector>mc.Motion",
           "mc.Geometry>mv.Geometry", "uz.Unit vector>mv.Motion", "mv.Geometry>cull.List", "cullp.Integer>cull.Indices",
           "yawp.Number>yrad.Degrees", "cull.List>yrot.Geometry", "yrad.Radians>yrot.Angle", "cull.List>yrot.Plane",
           "cull.List>dpl.Plane", "yrad.Radians>sin.Value", "sin.Result>abs.Value", "cw2.Number>hw.A", "two.Stream>hw.B", "hw.Result>amp.A", "abs.Result>amp.B",
           "dpl.Y-Axis>ampv.Vector", "amp.Result>ampv.Amplitude", "yrot.Geometry>slots.Geometry", "ampv.Vector>slots.Motion",
           "capp.Brep>ori.Geometry", "slots.Geometry>ori.Target", "ori.Geometry>st.Tree", "st.Length>stl.Stream", "st.Count>stc.Stream",
           "ow.Number>ow2.A", "two.Stream>ow2.B", "side.Number>cw.A", "ow2.Result>cw.B", "ch.Number>chh.A", "two.Stream>chh.B", "tot.Result>czz.A", "chh.Result>czz.B", "czz.Result>cpt.Z coordinate",
           "cpt.Point>cpl.Origin", "cpl.Plane>capital.Base", "cw.Result>capital.X", "cw.Result>capital.Y", "ch.Number>capital.Z",
           "xy.Plane>rect.Plane", "pw.Number>rect.X Size", "pd.Number>rect.Y Size", "z0.Result>uzp.Factor", "rect.Rectangle>ext.Base", "uzp.Unit vector>ext.Direction")
    c.view("core", "The core", "Der Kern", [("side", 0, 0), ("sh", 0, 1), ("n", 0, 2), ("ps", 0, 3), ("two", 0, 4), ("tn", 1, 2.5), ("tot", 2, 1.5), ("hc", 2, 3.5), ("cp", 3, 3.5), ("co", 4, 2.5), ("core", 5, 2.5), ("xy", 3, 0.5)])
    c.view("levels", "Heights per storey", "Höhen je Geschoss", [("sh", 0, 0), ("n", 0, 1), ("ps", 0, 2), ("z0", 1, 1.5), ("ser", 2, 0.8), ("gz", 3, 0.8), ("uz", 4, 0.8), ("gp", 3, 2.4)])
    c.view("facades", "Two slots on the first facade", "Zwei Plätze an der ersten Fassade", [("side", 0, 0), ("two", 0, 1), ("pitch", 0, 2), ("hs", 1, 0.5), ("hp", 1, 2), ("nhp", 2, 3), ("sx", 3, 2.5), ("xy", 3, 0), ("sp0", 4, 1.2), ("p0", 5, 1.2)])
    c.view("stagger", "The second capsule sits higher", "Die zweite Kapsel sitzt höher", [("zero", 0, 0), ("stg", 0, 1), ("sgm", 1, 0.5), ("uzs", 2, 0.5), ("p0", 2, 2), ("p0s", 3, 1.2)])
    c.view("rotate", "Rotate to every facade", "Zu jeder Fassade drehen", [("p0s", 0, 0), ("ang", 0, 2), ("rad", 1, 2), ("ga", 2, 2), ("rot", 3, 1), ("fl", 4, 1)])
    c.view("centre", "Move to the core centre", "Zum Kernmittelpunkt verschieben", [("fl", 0, 0.5), ("cx", 0, 1.5), ("cy", 0, 2.5), ("vc", 1, 2), ("mc", 2, 1)])
    c.view("storeys", "Copy the planes to every storey", "Die Ebenen auf jedes Geschoss kopieren", [("mc", 0, 0), ("uz", 0, 1.5), ("mv", 1, 0.8)])
    c.view("cull", "Cull Index removes the west planes of the top storey", "Cull Index entfernt die Westebenen des obersten Geschosses", [("mv", 0, 0.5), ("cullp", 0, 2), ("cull", 1, 1)])
    c.view("yaw", "Yaw every capsule by its type", "Jede Kapsel nach ihrem Typ drehen", [("cull", 0, 0.5), ("yawp", 0, 2), ("yrad", 1, 2), ("yrot", 2, 1)])
    c.view("clearance", "Push the yawed capsule out to touch the facade", "Die gedrehte Kapsel bis an die Fassade schieben", [("cull", 0, 0), ("yrad", 0, 1.5), ("yrot", 3, 0), ("dpl", 1, 0), ("sin", 1, 1.5), ("abs", 2, 1.5), ("cw2", 0, 3), ("two", 1, 3), ("hw", 2, 3), ("amp", 3, 2.2), ("ampv", 4, 1.2), ("slots", 5, 0.5)])
    c.view("orient", "Orient the right capsule onto every slot", "Die richtige Kapsel auf jeden Platz orientieren", [("slots", 0, 0.5), ("capp", 0, 2), ("ori", 1, 1.2), ("st", 2, 1.2), ("stl", 3, 0.5), ("stc", 3, 2)])
    c.view("capital", "The capital", "Der Kopf", [("ow", 0, 0), ("ch", 0, 1), ("side", 0, 2), ("tot", 0, 3), ("two", 0, 4), ("ow2", 1, 0), ("cw", 2, 0.5), ("chh", 1, 1.5), ("czz", 2, 2.5), ("cpt", 3, 2.5), ("cpl", 4, 2.5), ("capital", 5, 1.5)])
    c.view("podium", "The podium", "Der Sockel", [("pw", 0, 0), ("pd", 0, 1), ("ps", 0, 2), ("sh", 0, 3), ("z0", 1, 2.5), ("rect", 1, 0.5), ("uzp", 2, 2.5), ("ext", 3, 1.5)])
    return c.done()


def casting_canvas():
    c = Canvas("casting", "Casting: one demo per rule", "Casting: ein Beispiel je Regel")
    c.param("wid", "Number", "Width", [{"nakagin": "capsule.width"}])
    c.param("sto", "Integer", "Storeys", [{"nakagin": "towers.B.capsule_storeys"}])
    c.param("pln", "Plane", "Plane", [XY])
    c.panel("t25", [{"nakagin": "capsule.width"}])
    c.param("tn", "Number", "Width")
    c.panel("tno", [""])
    c.slider("sl", "Storeys", 0.0, 14.0, 10.6, decimals=1)
    c.panel("slp", [""])
    c.param("sto2", "Integer", "Storeys")
    c.panel("sto2p", [""])
    c.obj("fl", "Floor")
    c.param("sto3", "Integer", "Storeys")
    c.panel("sto3p", [""])
    c.param("pt", "Point", "Slot origin", [{"slot": {"index": [0, 1, 3, 3, 0], "part": "origin"}}])
    c.param("plc", "Plane", "Plane")
    c.panel("plp", [""])
    c.param("capc", "Brep", "Capsule", [{"capsule": "A"}])
    c.obj("orw", "Orient")
    c.param("vx", "Vector", "X-Axis", [{"slot": {"index": [0, 1, 3, 3, 0], "part": "x"}}])
    c.param("vy", "Vector", "Y-Axis", [{"slot": {"index": [0, 1, 3, 3, 0], "part": "y"}}])
    c.obj("cpl", "Construct Plane")
    c.obj("ore", "Orient")
    c.param("ln", "Line", "Core to slot", [{"slot": {"index": [0, 1, 3, 3, 0], "part": "line"}}])
    c.param("lpl", "Plane", "Plane")
    c.panel("lpp", [""])
    c.param("flen", "Number", "Length", [{"nakagin": "capsule.length"}])
    c.param("dom", "Domain", "Domain")
    c.panel("domp", [""])
    c.panel("cnt4", ["4"])
    c.obj("dd", "Divide Domain")
    c.panel("ddp", [""])
    c.param("cir", "Circle", "Window", [{"capsule_window": "A"}])
    c.param("crv", "Curve", "Curve")
    c.panel("crvp", [""])
    c.param("bx", "Box", "Capsule body", [{"capsule_box": "A"}])
    c.param("brp", "Brep", "Brep")
    c.panel("brpp", [""])
    c.obj("bxv", "Volume")
    c.panel("bxvp", [""])
    c.obj("tg", "Boolean Toggle", label="Window", value=True)
    c.param("bi", "Integer", "Window")
    c.panel("bip", [""])
    c.slider("isl", "Window", 0, 3, 1, rounding="integer")
    c.param("win", "Boolean", "Window")
    c.panel("winp", [""])
    c.panel("typa", ["Type A"])
    c.panel("tde", ["2,5"])
    c.param("tdn", "Number", "Width")
    c.panel("tdo", [""])
    c.param("fhei", "Number", "Height", [{"nakagin": "capsule.height"}])
    c.obj("cbx", "Center Box")
    c.panel("cbxp", [""])
    c.obj("vl", "Value List", items=[{"name": f"Type {k}", "value": 2.5, "bind": "capsule.width"} for k in "ABCD"], selected=0)
    c.obj("cbx2", "Center Box")
    c.panel("cbx2p", [""])
    c.wire("t25.Stream>tn.Number", "tn.Number>tno.Stream", "sl.Number>slp.Stream", "sl.Number>sto2.Integer", "sto2.Integer>sto2p.Stream", "sl.Number>fl.Number", "fl.Result>sto3.Integer", "sto3.Integer>sto3p.Stream",
           "pt.Point>plc.Plane", "plc.Plane>plp.Stream", "capc.Brep>orw.Geometry", "pln.Plane>orw.Source", "plc.Plane>orw.Target",
           "pt.Point>cpl.Origin", "vx.Vector>cpl.X-Axis", "vy.Vector>cpl.Y-Axis", "capc.Brep>ore.Geometry", "pln.Plane>ore.Source", "cpl.Plane>ore.Target",
           "ln.Line>lpl.Plane", "lpl.Plane>lpp.Stream", "flen.Number>dom.Domain", "dom.Domain>domp.Stream", "flen.Number>dd.Domain", "cnt4.Stream>dd.Count", "dd.Segments>ddp.Stream",
           "cir.Circle>crv.Curve", "crv.Curve>crvp.Stream", "bx.Box>brp.Brep", "brp.Brep>brpp.Stream", "brp.Brep>bxv.Geometry", "bxv.Volume>bxvp.Stream",
           "tg.Boolean>bi.Integer", "bi.Integer>bip.Stream", "isl.Number>win.Boolean", "win.Boolean>winp.Stream",
           "tde.Stream>tdn.Number", "tdn.Number>tdo.Stream", "typa.Stream>cbx.X", "flen.Number>cbx.Y", "fhei.Number>cbx.Z", "cbx.Box>cbxp.Stream", "vl.Value>cbx2.X", "flen.Number>cbx2.Y", "fhei.Number>cbx2.Z", "cbx2.Box>cbx2p.Stream")
    c.view("types", "Every parameter has a type", "Jeder Parameter hat einen Typ", [("wid", 0, 0), ("sto", 0, 1), ("pln", 0, 2)])
    c.view("text-number", "Text into Number", "Text in Zahl", [("t25", 0, 0), ("tn", 1, 0), ("tno", 2, 0)])
    c.view("storeys", "Number into Integer", "Zahl in Ganzzahl", [("sl", 0, 0), ("slp", 1, 1), ("sto2", 1, 0), ("sto2p", 2, 0)])
    c.view("deliberate", "Floor before the Integer", "Floor vor der Ganzzahl", [("sl", 0, 0), ("fl", 1, 0), ("sto3", 2, 0), ("sto3p", 3, 0)])
    c.view("point-plane", "Point into Plane", "Punkt in Ebene", [("pt", 0, 0), ("plc", 1, 0), ("plp", 2, 0), ("capc", 0, 1.3), ("pln", 0, 2.3), ("orw", 2, 1.8)])
    c.view("explicit", "Build the plane explicitly", "Ebene ausdrücklich konstruieren", [("pt", 0, 0), ("vx", 0, 1), ("vy", 0, 2), ("cpl", 1, 1), ("capc", 1, 2.5), ("pln", 2, 3), ("ore", 2, 1.5)])
    c.view("line-plane", "Line into Plane", "Linie in Ebene", [("ln", 0, 0), ("lpl", 1, 0), ("lpp", 2, 0)])
    c.view("domain", "Number into Domain", "Zahl in Domain", [("flen", 0, 0), ("dom", 1, 0), ("domp", 2, 0), ("cnt4", 0, 2), ("dd", 1, 1.5), ("ddp", 2, 1.5)])
    c.view("circle-curve", "Circle into Curve", "Kreis in Kurve", [("cir", 0, 0), ("crv", 1, 0), ("crvp", 2, 0)])
    c.view("box-brep", "Box into Brep", "Box in Brep", [("bx", 0, 0), ("brp", 1, 0), ("brpp", 2, 0), ("bxv", 2, 1), ("bxvp", 3, 1)])
    c.view("bool-int", "Boolean into Integer", "Boolean in Ganzzahl", [("tg", 0, 0), ("bi", 1, 0), ("bip", 2, 0)])
    c.view("int-bool", "Number into Boolean", "Zahl in Boolean", [("isl", 0, 0), ("win", 1, 0), ("winp", 2, 0)])
    c.view("locale", "A decimal comma in Text", "Ein Dezimalkomma im Text", [("tde", 0, 0), ("tdn", 1, 0), ("tdo", 2, 0)])
    c.view("fail", "A label is no number", "Eine Bezeichnung ist keine Zahl", [("typa", 0, 0), ("flen", 0, 1), ("fhei", 0, 2), ("cbx", 1, 1), ("cbxp", 2, 1)])
    c.view("fix", "A Value List fixes it", "Eine Werteliste behebt es", [("vl", 0, 0), ("flen", 0, 1), ("fhei", 0, 2), ("cbx2", 1, 1), ("cbx2p", 2, 1)])
    return c.done()


def trees():
    c = Canvas("trees", "The aggregated tower as a data tree", "Der gesamte Turm als Datenbaum")
    c.param("src", "Brep", "Capsules", {"aggregate": True})
    c.obj("pnl", "Panel")
    c.obj("pv", "Param Viewer")
    c.obj("st", "Tree Statistics")
    c.panel("stl", [""])
    c.panel("stc", [""])
    c.obj("fl", "Flatten Tree")
    c.obj("fls", "Tree Statistics")
    c.panel("flc", [""])
    c.obj("gr", "Graft Tree")
    c.obj("grs", "Tree Statistics")
    c.panel("grc", [""])
    c.obj("sm", "Simplify Tree")
    c.obj("sms", "Tree Statistics")
    c.panel("smp", [""])
    c.obj("pm", "Path Mapper", mapping="{A;B;C;D}(i) -> {A;B;C}")
    c.obj("pms", "Tree Statistics")
    c.panel("pml", [""])
    c.obj("pt", "Path Mapper", mapping="{A;B;C;D}(i) -> {A;B}")
    c.obj("pts", "Tree Statistics")
    c.panel("ptl", [""])
    c.obj("fp", "Flip Matrix")
    c.obj("fps", "Tree Statistics")
    c.panel("fpl", [""])
    c.panel("tip", ["{0;1;4;2}"])
    c.panel("tii", ["1"])
    c.obj("ti", "Tree Item")
    c.panel("tio", [""])
    c.panel("cols", ["teal", "orange"])
    c.obj("pvk", "Param Viewer")
    c.obj("cp1", "Custom Preview")
    c.obj("pvc", "Param Viewer")
    c.obj("gc", "Graft Tree")
    c.obj("cp2", "Custom Preview")
    c.obj("pvd", "Param Viewer")
    c.wire("src.Brep>pnl.Stream", "src.Brep>pv.Data", "src.Brep>st.Tree", "st.Length>stl.Stream", "st.Count>stc.Stream",
           "src.Brep>fl.Tree", "fl.Tree>fls.Tree", "fls.Length>flc.Stream", "src.Brep>gr.Tree", "gr.Tree>grs.Tree", "grs.Count>grc.Stream",
           "src.Brep>sm.Tree", "sm.Tree>sms.Tree", "sms.Paths>smp.Stream", "src.Brep>pm.Tree", "pm.Tree>pms.Tree", "pms.Length>pml.Stream",
           "src.Brep>pt.Tree", "pt.Tree>pts.Tree", "pts.Length>ptl.Stream", "src.Brep>fp.Data", "fp.Data>fps.Tree", "fps.Length>fpl.Stream",
           "src.Brep>ti.Tree", "tip.Stream>ti.Path", "tii.Stream>ti.Index", "ti.Item>tio.Stream",
           "cols.Stream>pvk.Data", "src.Brep>cp1.Geometry", "cols.Stream>cp1.Material", "cp1.Preview>pvc.Data",
           "cols.Stream>gc.Tree", "pt.Tree>cp2.Geometry", "gc.Tree>cp2.Material", "cp2.Preview>pvd.Data")
    c.view("source", "The aggregated tree", "Der aggregierte Baum", [("src", 0, 0), ("pnl", 1, 0), ("pv", 1, 1.3), ("st", 1, 2.6), ("stl", 2, 2.2), ("stc", 2, 3.2)])
    c.view("flatten", "Flatten Tree", "Flatten Tree", [("src", 0, 0), ("fl", 1, 0), ("fls", 2, 0), ("flc", 3, 0)])
    c.view("graft", "Graft Tree", "Graft Tree", [("src", 0, 0), ("gr", 1, 0), ("grs", 2, 0), ("grc", 3, 0)])
    c.view("simplify", "Simplify Tree", "Simplify Tree", [("src", 0, 0), ("sm", 1, 0), ("sms", 2, 0), ("smp", 3, 0)])
    c.view("mapper", "Path Mapper per storey", "Path Mapper je Geschoss", [("src", 0, 0), ("pm", 1, 0), ("pms", 2, 0), ("pml", 3, 0)])
    c.view("tower-map", "Path Mapper per tower", "Path Mapper je Turm", [("src", 0, 0), ("pt", 1, 0), ("pts", 2, 0), ("ptl", 3, 0)])
    c.view("flip", "Flip Matrix", "Flip Matrix", [("src", 0, 0), ("fp", 1, 0), ("fps", 2, 0), ("fpl", 3, 0)])
    c.view("item", "One item by path and index", "Ein Element per Pfad und Index", [("src", 0, 0), ("tip", 0, 1.5), ("tii", 0, 2.5), ("ti", 1, 1.5), ("tio", 2, 1.5)])
    c.view("mistake", "A short list repeats in every branch", "Eine kurze Liste wiederholt sich in jedem Ast", [("src", 0, 0), ("cols", 0, 1.5), ("cp1", 1, 0.8), ("pvc", 2, 0.8)])
    c.view("why", "Why the paths do not meet", "Warum die Pfade sich nicht treffen", [("src", 0, 0), ("cols", 0, 1.5), ("pv", 1, 0), ("pvk", 1, 1.5)])
    c.view("fix", "Map the capsules, graft the colours", "Kapseln abbilden, Farben graften", [("src", 0, 0), ("pt", 1, 0), ("cols", 0, 1.5), ("gc", 1, 1.5), ("cp2", 2, 0.8), ("pvd", 3, 0.8)])
    return c.done()


model = {"version": 1, "languages": ["en", "de"], "revision": 1, "catalogue": catalogue, "casting": casting, "capsule_wall": {"value": 0.3, "unit": "m", "basis": "assumed"},
         "canvases": {"anatomy": anatomy(), "capsule": capsule(), "tower": tower(), "casting": casting_canvas(), "trees": trees()}}
OUT.write_text(json.dumps(model, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print("[DEBUG] wrote", len(json.dumps(model)))
