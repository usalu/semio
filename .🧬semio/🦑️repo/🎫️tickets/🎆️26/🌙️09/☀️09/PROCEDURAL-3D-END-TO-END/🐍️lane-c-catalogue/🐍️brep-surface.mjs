import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";

const C = "brep.surface";
const face = (en, de) => O("shape", "shape", T("Face", "Fläche"), T(en, de), { shapeKinds: ["face"] });
const surf = (en, de) => O("shape", "shape", T("Surface", "Fläche"), T(en, de), { shapeKinds: ["surface"] });
const edge = (name, en, de, def) => P(name, "point", T(en, de), T(`The ${en.toLowerCase()} boundary curve as ordered points; it must meet the neighbouring boundaries at its ends.`, `Die ${de.toLowerCase()} Randkurve als geordnete Punkte; sie muss an ihren Enden auf die Nachbarränder treffen.`), { list: true, minItems: 2, default: def });

export default {
  category: {
    id: C, emoji: "🏳", order: 30,
    label: T("Surfaces and faces", "Flächen"),
    description: T("Planes, faces from points or wires, free-form patches and face offsets.", "Ebenen, Flächen aus Punkten oder Kantenzügen, Freiformflächen und Flächenversatz."),
  },
  kinds: [
    K(C, {
      was: "brep.surf.plane", id: "brep.surface.plane", emoji: "▫", quality: "exact-analytic", preview: true,
      label: T("Plane surface", "Ebene"),
      description: T("Creates an unbounded plane surface from an origin and a normal.", "Erzeugt eine unbegrenzte Ebene aus Ursprung und Normale."),
      in: [I("plane")],
      out: [surf("The plane as a surface.", "Die Ebene als Fläche.")],
      ren: { in: { origin: "plane.origin", normal: "plane.normal" }, out: { surface: "shape" } },
      note: "origin and normal merged into one plane port",
      interaction: { gumball: [gumball("translate", "plane"), gumball("rotate", "plane")] },
    }),
    K(C, {
      was: "brep.surf.planarFace", id: "brep.surface.planarFace", emoji: "🔲", quality: "exact-analytic", preview: true,
      label: T("Face from points", "Fläche aus Punkten"),
      description: T("Creates a planar face bounded by a polygon through the points.", "Erzeugt eine ebene Fläche, die von einem Polygon durch die Punkte begrenzt wird."),
      in: [I("points", { minItems: 3, default: [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]], description: T("The corner points in order; at least three, all in one plane.", "Die Eckpunkte in Reihenfolge; mindestens drei, alle in einer Ebene.") })],
      out: [face("The planar face.", "Die ebene Fläche.")],
      ren: { out: { face: "shape" } },
    }),
    K(C, {
      was: "brep.surf.planarFaceWire", id: "brep.surface.planarFaceFromWire", emoji: "🖼", quality: "exact-analytic", preview: true,
      label: T("Planar face from wire", "Ebene Fläche aus Kantenzug"),
      description: T("Fills a closed wire that lies in a plane parallel to the XY plane with a planar face.", "Füllt einen geschlossenen Kantenzug, der in einer zur XY-Ebene parallelen Ebene liegt, mit einer ebenen Fläche."),
      in: [I("wire", { description: T("The closed wire to fill.", "Der zu füllende geschlossene Kantenzug.") })],
      out: [face("The planar face bounded by the wire.", "Die vom Kantenzug begrenzte ebene Fläche.")],
      interaction: { pick: [pick("shape", "wire")] },
      ren: { out: { face: "shape" } },
    }),
    K(C, {
      was: "brep.util.faceFromWire", id: "brep.surface.faceFromWire", emoji: "🧾", quality: "exact-analytic", preview: true,
      label: T("Face from wire", "Fläche aus Kantenzug"),
      description: T("Creates a face bounded by a closed wire.", "Erzeugt eine Fläche, die von einem geschlossenen Kantenzug begrenzt wird."),
      in: [I("wire", { description: T("The closed wire that bounds the face.", "Der geschlossene Kantenzug, der die Fläche begrenzt.") })],
      out: [face("The face bounded by the wire.", "Die vom Kantenzug begrenzte Fläche.")],
      interaction: { pick: [pick("shape", "wire")] },
      ren: { out: { face: "shape" } },
    }),
    K(C, {
      was: "brep.surf.nurbsGrid", id: "brep.surface.nurbsGrid", emoji: "🕸", quality: "approximate", preview: true,
      label: T("NURBS surface from grid", "NURBS-Fläche aus Punktgitter"),
      description: T("Fits a smooth NURBS surface through a rectangular grid of points.", "Legt eine glatte NURBS-Fläche durch ein rechteckiges Punktgitter."),
      in: [
        I("points", { minItems: 4, default: [[0, 0, 0], [1, 0, 0], [2, 0, 0], [0, 1, 0], [1, 1, 0.5], [2, 1, 0], [0, 2, 0], [1, 2, 0], [2, 2, 0]], description: T("All grid points row by row.", "Alle Gitterpunkte Zeile für Zeile.") }),
        P("rows", "integer", T("Rows", "Zeilen"), T("Number of rows; the point count must be a multiple of it.", "Anzahl der Zeilen; die Punktzahl muss ein Vielfaches davon sein."), { default: 3, min: 2 }),
        P("degreeU", "integer", T("Degree U", "Grad U"), T("Polynomial degree along the rows.", "Polynomgrad entlang der Zeilen."), { default: 2, min: 1, max: 9 }),
        P("degreeV", "integer", T("Degree V", "Grad V"), T("Polynomial degree along the columns.", "Polynomgrad entlang der Spalten."), { default: 2, min: 1, max: 9 }),
      ],
      out: [surf("The fitted surface.", "Die angepasste Fläche.")],
      ren: { out: { surface: "shape" } },
      note: "defaults changed to a 3x3 grid with degree 2 so a new widget evaluates",
    }),
    K(C, {
      was: "brep.surf.coons", id: "brep.surface.coons", emoji: "🧩", quality: "approximate", preview: true,
      label: T("Coons patch", "Coons-Fläche"),
      description: T("Spans a smooth surface between four boundary curves.", "Spannt eine glatte Fläche zwischen vier Randkurven auf."),
      in: [
        edge("bottom", "Bottom", "Untere", [[0, 0, 0], [0.5, 0, 0.3], [1, 0, 0]]),
        edge("right", "Right", "Rechte", [[1, 0, 0], [1, 1, 0]]),
        edge("top", "Top", "Obere", [[0, 1, 0], [0.5, 1, 0.3], [1, 1, 0]]),
        edge("left", "Left", "Linke", [[0, 0, 0], [0, 1, 0]]),
      ],
      out: [surf("The Coons patch.", "Die Coons-Fläche.")],
      ren: { in: { curves: "bottom, right, top, left" }, out: { surface: "shape" } },
      note: "the single nested list port becomes four point-list ports, in kernel order bottom, right, top, left",
    }),
    K(C, {
      was: "brep.surf.offset", id: "brep.surface.offset", emoji: "↔", quality: "exact-numerical", preview: true,
      label: T("Offset face", "Fläche versetzen"),
      description: T("Moves a face along its normal by a distance.", "Versetzt eine Fläche um einen Abstand entlang ihrer Normalen."),
      in: [I("face"), I("distance", { default: 0.1, description: T("Offset along the face normal; negative values move against it.", "Versatz entlang der Flächennormale; negative Werte verschieben entgegen.") })],
      out: [face("The offset face.", "Die versetzte Fläche.")],
      interaction: { pick: [pick("face", "face")], gumball: [gumball("translate", "distance", { along: "normal" })] },
      ren: { out: { faceOut: "shape" } },
    }),
  ],
};
