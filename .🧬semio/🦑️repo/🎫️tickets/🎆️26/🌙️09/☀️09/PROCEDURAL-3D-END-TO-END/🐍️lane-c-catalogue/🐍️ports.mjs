import { T, def, lenOpts, RAD } from "./🐍️lib.mjs";

const PI = Math.PI;

def("shape", { type: "shape", label: T("Shape", "Form"), description: T("The shape to operate on.", "Die Form, auf die der Vorgang angewendet wird.") });
def("solid", { type: "shape", shapeKinds: ["solid", "compound"], label: T("Solid", "Körper"), description: T("The solid to operate on.", "Der Körper, auf den der Vorgang angewendet wird.") });
def("face", { type: "shape", shapeKinds: ["face"], label: T("Face", "Fläche"), description: T("The planar or curved face to operate on.", "Die ebene oder gekrümmte Fläche, auf die der Vorgang angewendet wird.") });
def("wire", { type: "shape", shapeKinds: ["wire"], label: T("Wire", "Kantenzug"), description: T("A chain of connected edges.", "Eine Kette zusammenhängender Kanten.") });
def("curve", { type: "shape", shapeKinds: ["curve", "wire", "edge"], label: T("Curve", "Kurve"), description: T("The curve to evaluate.", "Die auszuwertende Kurve.") });
def("surface", { type: "shape", shapeKinds: ["surface", "face"], label: T("Surface", "Fläche"), description: T("The surface to evaluate.", "Die auszuwertende Fläche.") });
def("profile", { type: "shape", shapeKinds: ["wire", "face"], label: T("Profile", "Profil"), description: T("The closed profile that is moved along the operation.", "Das geschlossene Profil, das durch den Vorgang bewegt wird.") });
def("path", { type: "shape", shapeKinds: ["wire", "curve", "edge"], label: T("Path", "Pfad"), description: T("The curve the profile follows.", "Die Kurve, der das Profil folgt.") });
def("a", { type: "shape", label: T("Shape A", "Form A"), description: T("The first operand.", "Der erste Operand.") });
def("b", { type: "shape", label: T("Shape B", "Form B"), description: T("The second operand.", "Der zweite Operand.") });
def("points", { type: "point", list: true, minItems: 2, default: [[0, 0, 0], [1, 0, 0], [1, 1, 0]], label: T("Points", "Punkte"), description: T("The ordered points.", "Die geordneten Punkte.") });

def("width", { type: "length", ...lenOpts, default: 1, label: T("Width", "Breite"), description: T("Size along the X axis.", "Ausdehnung entlang der X-Achse.") });
def("depth", { type: "length", ...lenOpts, default: 1, label: T("Depth", "Tiefe"), description: T("Size along the Y axis.", "Ausdehnung entlang der Y-Achse.") });
def("height", { type: "length", ...lenOpts, default: 1, label: T("Height", "Höhe"), description: T("Size along the Z axis.", "Ausdehnung entlang der Z-Achse.") });
def("radius", { type: "length", ...lenOpts, default: 1, label: T("Radius", "Radius"), description: T("Distance from the center to the outline.", "Abstand vom Mittelpunkt zum Rand.") });
def("distance", { type: "length", default: 0.1, label: T("Distance", "Abstand"), description: T("Distance of the operation.", "Abstand des Vorgangs.") });
def("thickness", { type: "length", ...lenOpts, default: 0.1, label: T("Thickness", "Dicke"), description: T("Wall thickness.", "Wandstärke.") });
def("tolerance", { type: "length", ...lenOpts, default: 0.001, step: 0.0001, label: T("Tolerance", "Toleranz"), description: T("Largest deviation that is still accepted.", "Größte noch akzeptierte Abweichung.") });
def("deflection", { type: "length", ...lenOpts, default: 0.1, step: 0.01, label: T("Deflection", "Abweichung"), description: T("Largest distance between the surface and its triangles; smaller values give finer meshes.", "Größter Abstand zwischen Oberfläche und Dreiecken; kleinere Werte ergeben feinere Netze.") });
def("angle", { type: "angle", default: PI / 4, label: T("Angle", "Winkel"), description: T("Rotation angle; positive values turn counter-clockwise around the axis.", "Drehwinkel; positive Werte drehen gegen den Uhrzeigersinn um die Achse.") });
def("count", { type: "integer", min: 1, default: 3, label: T("Count", "Anzahl"), description: T("Number of copies, including the original.", "Anzahl der Kopien einschließlich des Originals.") });
def("parameter", { type: "number", default: 0, label: T("Parameter", "Parameter"), description: T("Position along the curve in its own parameter domain.", "Position entlang der Kurve in ihrem eigenen Parameterbereich.") });
def("u", { type: "number", default: 0, label: T("U", "U"), description: T("First parameter of the surface.", "Erster Parameter der Fläche.") });
def("v", { type: "number", default: 0, label: T("V", "V"), description: T("Second parameter of the surface.", "Zweiter Parameter der Fläche.") });

def("origin", { type: "point", default: [0, 0, 0], label: T("Origin", "Ursprung"), description: T("Reference point of the operation.", "Bezugspunkt des Vorgangs.") });
def("center", { type: "point", default: [0, 0, 0], label: T("Center", "Mittelpunkt"), description: T("Center point.", "Mittelpunkt.") });
def("point", { type: "point", default: [0, 0, 0], label: T("Point", "Punkt"), description: T("A position in space.", "Eine Position im Raum.") });
def("normal", { type: "vector", default: [0, 0, 1], label: T("Normal", "Normale"), description: T("Direction perpendicular to the plane of the shape.", "Richtung senkrecht zur Ebene der Form.") });
def("axis", { type: "vector", default: [0, 0, 1], label: T("Axis", "Achse"), description: T("Direction of the axis.", "Richtung der Achse.") });
def("axisOrigin", { type: "point", default: [0, 0, 0], label: T("Axis origin", "Achsenursprung"), description: T("A point on the axis.", "Ein Punkt auf der Achse.") });
def("axisDirection", { type: "vector", default: [0, 0, 1], label: T("Axis direction", "Achsenrichtung"), description: T("Direction of the axis.", "Richtung der Achse.") });
def("offset", { type: "vector", default: [0, 0, 1], label: T("Offset", "Verschiebung"), description: T("Displacement along each axis.", "Verschiebung entlang jeder Achse.") });
def("direction", { type: "vector", default: [1, 0, 0], label: T("Direction", "Richtung"), description: T("Direction of the operation.", "Richtung des Vorgangs.") });
def("plane", { type: "plane", default: { origin: [0, 0, 0], normal: [0, 0, 1] }, label: T("Plane", "Ebene"), description: T("The plane given by an origin and a normal.", "Die Ebene aus Ursprung und Normale.") });
def("factor", { type: "vector", default: [1, 1, 1], label: T("Scale factor", "Skalierfaktor"), description: T("Scale factor per axis; negative values mirror.", "Skalierfaktor je Achse; negative Werte spiegeln.") });
def("pivot", { type: "point", default: [0, 0, 0], label: T("Pivot", "Drehpunkt"), description: T("Fixed point of the transformation.", "Fester Punkt der Transformation.") });

def("text", { type: "text", default: "", label: T("Text", "Text"), description: T("The text content.", "Der Textinhalt.") });
def("data", { type: "text", default: "", label: T("Data", "Daten"), description: T("The file content as text.", "Der Dateiinhalt als Text.") });
