import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";

const C = "brep.curve";
const PI = Math.PI;
const curve = (en, de) => O("shape", "shape", T("Curve", "Kurve"), T(en, de), { shapeKinds: ["curve"] });
const wire = (en, de) => O("shape", "shape", T("Wire", "Kantenzug"), T(en, de), { shapeKinds: ["wire"] });
const normal = (extra = {}) => I("normal", { description: T("Normal of the plane that contains the shape.", "Normale der Ebene, in der die Form liegt."), ...extra });

export default {
  category: {
    id: C, emoji: "〰", order: 20,
    label: T("Curves and wires", "Kurven und Kantenzüge"),
    description: T("Lines, arcs, circles, ellipses, splines, polygons and helices as building blocks for profiles and paths.", "Linien, Bögen, Kreise, Ellipsen, Splines, Vielecke und Schraubenlinien als Bausteine für Profile und Pfade."),
  },
  kinds: [
    K(C, {
      was: "brep.curve.line", id: "brep.curve.line", emoji: "📏", quality: "exact-analytic", preview: true,
      label: T("Line", "Linie"),
      description: T("Creates a straight line segment between two points.", "Erzeugt eine gerade Strecke zwischen zwei Punkten."),
      in: [I("point", { as: "start", label: T("Start", "Start"), description: T("First end of the line.", "Erstes Ende der Linie.") }), I("point", { as: "end", default: [1, 0, 0], label: T("End", "Ende"), description: T("Second end of the line.", "Zweites Ende der Linie.") })],
      out: [curve("The line as a curve.", "Die Linie als Kurve.")],
      ren: { out: { curve: "shape" } },
    }),
    K(C, {
      was: "brep.curve.circle", id: "brep.curve.circle", emoji: "⭕", quality: "exact-analytic", preview: true,
      label: T("Circle", "Kreis"),
      description: T("Creates a full circle around a center in the plane given by its normal.", "Erzeugt einen Vollkreis um einen Mittelpunkt in der durch die Normale gegebenen Ebene."),
      in: [I("center"), normal(), I("radius")],
      out: [curve("The circle as a closed curve.", "Der Kreis als geschlossene Kurve.")],
      ren: { in: {}, out: { curve: "shape" } },
      note: "normal is a vector port (was a point port)",
    }),
    K(C, {
      was: "brep.curve.arc", id: "brep.curve.arc", emoji: "🌙", quality: "exact-analytic", preview: true,
      label: T("Arc", "Kreisbogen"),
      description: T("Creates a circular arc between a start and an end angle.", "Erzeugt einen Kreisbogen zwischen einem Start- und einem Endwinkel."),
      in: [
        I("center"), normal(), I("radius"),
        I("angle", { as: "startAngle", default: 0, label: T("Start angle", "Startwinkel"), description: T("Angle at which the arc begins, measured from the local X axis.", "Winkel, bei dem der Bogen beginnt, gemessen von der lokalen X-Achse.") }),
        I("angle", { as: "endAngle", default: PI / 2, label: T("End angle", "Endwinkel"), description: T("Angle at which the arc ends; must differ from the start angle.", "Winkel, bei dem der Bogen endet; muss vom Startwinkel abweichen.") }),
      ],
      out: [curve("The arc as a curve.", "Der Kreisbogen als Kurve.")],
      ren: { out: { curve: "shape" } },
    }),
    K(C, {
      was: "brep.curve.ellipse", id: "brep.curve.ellipse", emoji: "🥚", quality: "exact-analytic", preview: true,
      label: T("Ellipse", "Ellipse"),
      description: T("Creates a full ellipse around a center; elliptical edges cannot be extruded or revolved yet.", "Erzeugt eine vollständige Ellipse um einen Mittelpunkt; elliptische Kanten lassen sich noch nicht extrudieren oder rotieren."),
      in: [
        I("center"), normal(),
        I("radius", { as: "semiMajor", default: 2, label: T("Major radius", "Große Halbachse"), description: T("Half of the longer diameter.", "Hälfte des längeren Durchmessers.") }),
        I("radius", { as: "semiMinor", default: 1, label: T("Minor radius", "Kleine Halbachse"), description: T("Half of the shorter diameter.", "Hälfte des kürzeren Durchmessers.") }),
      ],
      out: [curve("The ellipse as a closed curve.", "Die Ellipse als geschlossene Kurve.")],
      ren: { out: { curve: "shape" } },
    }),
    K(C, {
      was: "brep.curve.polyline", id: "brep.curve.polyline", emoji: "📈", quality: "exact-analytic", preview: true,
      label: T("Polyline", "Polylinie"),
      description: T("Connects the points with straight segments into an open wire.", "Verbindet die Punkte mit geraden Strecken zu einem offenen Kantenzug."),
      in: [I("points", { description: T("The corner points in order; at least two.", "Die Eckpunkte in Reihenfolge; mindestens zwei.") })],
      out: [wire("The open polyline.", "Die offene Polylinie.")],
      ren: { out: { wire: "shape" } },
    }),
    K(C, {
      was: "brep.curve.rectangle", id: "brep.curve.rectangle", emoji: "⬜", quality: "exact-analytic", preview: true,
      label: T("Rectangle", "Rechteck"),
      description: T("Creates a closed rectangle in the XY plane with its corner at the origin.", "Erzeugt ein geschlossenes Rechteck in der XY-Ebene mit einer Ecke im Ursprung."),
      in: [I("width"), I("height", { description: T("Size along the Y axis.", "Ausdehnung entlang der Y-Achse.") })],
      out: [wire("The closed rectangle.", "Das geschlossene Rechteck.")],
      ren: { out: { wire: "shape" } },
    }),
    K(C, {
      was: "brep.curve.polygon", id: "brep.curve.polygon", emoji: "⬡", quality: "exact-analytic", preview: true,
      label: T("Regular polygon", "Regelmäßiges Vieleck"),
      description: T("Creates a closed regular polygon in the XY plane around the origin.", "Erzeugt ein geschlossenes regelmäßiges Vieleck in der XY-Ebene um den Ursprung."),
      in: [
        I("radius", { description: T("Distance from the center to each corner.", "Abstand vom Mittelpunkt zu jeder Ecke.") }),
        P("sides", "integer", T("Sides", "Seiten"), T("Number of sides; at least three.", "Anzahl der Seiten; mindestens drei."), { default: 6, min: 3, max: 256 }),
      ],
      out: [wire("The closed polygon.", "Das geschlossene Vieleck.")],
      ren: { out: { wire: "shape" } },
    }),
    K(C, {
      was: "brep.curve.interpolate", id: "brep.curve.interpolate", emoji: "🪢", quality: "approximate", preview: true,
      label: T("Interpolated curve", "Interpolierte Kurve"),
      description: T("Fits a smooth spline that passes exactly through every point.", "Legt einen glatten Spline, der genau durch jeden Punkt verläuft."),
      in: [
        I("points", { default: [[0, 0, 0], [1, 1, 0], [2, 0, 0], [3, 1, 0]], description: T("The points the curve must pass through; at least two.", "Die Punkte, durch die die Kurve verlaufen muss; mindestens zwei.") }),
        P("degree", "integer", T("Degree", "Grad"), T("Polynomial degree of the spline; 3 gives a smooth cubic curve.", "Polynomgrad des Splines; 3 ergibt eine glatte kubische Kurve."), { default: 3, min: 1, max: 9 }),
      ],
      out: [curve("The interpolating spline.", "Der interpolierende Spline.")],
      ren: { out: { curve: "shape" } },
    }),
    K(C, {
      was: "brep.curve.approximate", id: "brep.curve.approximate", emoji: "🎢", quality: "approximate", preview: true,
      label: T("Approximated curve", "Approximierte Kurve"),
      description: T("Fits a smooth spline with a limited number of control points close to the points.", "Legt einen glatten Spline mit begrenzter Kontrollpunktzahl nahe an die Punkte."),
      in: [
        I("points", { default: [[0, 0, 0], [1, 1, 0], [2, 0, 0], [3, 1, 0], [4, 0, 0]], description: T("The points to approximate; at least two.", "Die zu approximierenden Punkte; mindestens zwei.") }),
        P("degree", "integer", T("Degree", "Grad"), T("Polynomial degree of the spline.", "Polynomgrad des Splines."), { default: 3, min: 1, max: 9 }),
        P("controlPoints", "integer", T("Control points", "Kontrollpunkte"), T("Number of control points of the result; fewer gives a smoother curve.", "Anzahl der Kontrollpunkte des Ergebnisses; weniger ergibt eine glattere Kurve."), { default: 4, min: 2 }),
      ],
      out: [curve("The approximating spline.", "Der approximierende Spline.")],
      ren: { out: { curve: "shape" } },
    }),
    K(C, {
      was: "brep.curve.helix", id: "brep.curve.helix", emoji: "🌀", quality: "approximate", preview: true,
      label: T("Helix", "Schraubenlinie"),
      description: T("Creates a helix around an axis, built as a polyline with 32 segments per turn.", "Erzeugt eine Schraubenlinie um eine Achse, aufgebaut als Polylinie mit 32 Segmenten je Windung."),
      in: [
        I("origin", { description: T("Start point of the axis.", "Startpunkt der Achse.") }),
        I("axis", { description: T("Direction of the helix axis.", "Richtung der Schraubenachse.") }),
        I("radius"),
        P("pitch", "length", T("Pitch", "Steigung"), T("Distance travelled along the axis per full turn.", "Weg entlang der Achse pro voller Windung."), { default: 1, min: 0, exclusiveMin: true }),
        P("turns", "number", T("Turns", "Windungen"), T("Number of full turns; fractions are allowed.", "Anzahl voller Windungen; Bruchteile sind erlaubt."), { default: 1, min: 0, exclusiveMin: true, step: 0.25 }),
      ],
      out: [wire("The helix as an open wire.", "Die Schraubenlinie als offener Kantenzug.")],
      ren: { out: { curve: "shape" } },
      note: "the kernel returns a polyline wire (quality tag lowered from exact-analytic to approximate), the old registration declared a curve",
    }),
  ],
};
