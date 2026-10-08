import { T, I, O, P, K, pick, gumball, sel } from "./🐍️lib.mjs";

const C = "brep.topology";
const grab = (port = "shape") => ({ pick: [pick("shape", port)] });

export default {
  category: {
    id: C, emoji: "🐚", order: 100,
    label: T("Topology and repair", "Topologie und Reparatur"),
    description: T("Take shapes apart, combine them, and sew, heal or convert them.", "Formen zerlegen, zusammenfassen sowie vernähen, heilen oder umwandeln."),
  },
  kinds: [
    K(C, {
      was: "brep.util.vertex", id: "brep.topology.vertex", emoji: "📍", quality: "exact-analytic", preview: true,
      label: T("Vertex", "Eckpunkt"),
      description: T("Creates a vertex shape at a position.", "Erzeugt eine Eckpunkt-Form an einer Position."),
      in: [I("point")],
      out: [O("shape", "shape", T("Vertex", "Eckpunkt"), T("The vertex.", "Der Eckpunkt."), { shapeKinds: ["vertex"] })],
      ren: { out: { vertex: "shape" } },
    }),
    K(C, {
      was: "brep.brep", id: "brep.topology.deconstruct", emoji: "🧩", quality: "exact-analytic",
      label: T("Deconstruct", "Zerlegen"),
      description: T("Splits a shape into its vertices, edges, faces and shells, and extracts the selected edges and faces.", "Zerlegt eine Form in Eckpunkte, Kanten, Flächen und Schalen und liefert die ausgewählten Kanten und Flächen."),
      in: [
        I("shape"),
        P("edges", "selection", T("Edges", "Kanten"), T("Edges to extract as a separate output.", "Kanten, die als eigene Ausgabe extrahiert werden."), { ...sel("edge", "shape"), default: [], optional: true }),
        P("faces", "selection", T("Faces", "Flächen"), T("Faces to extract as a separate output.", "Flächen, die als eigene Ausgabe extrahiert werden."), { ...sel("face", "shape"), default: [], optional: true }),
      ],
      out: [
        O("vertices", "shapes", T("Vertices", "Eckpunkte"), T("All vertices.", "Alle Eckpunkte."), { shapeKinds: ["vertex"] }),
        O("edges", "shapes", T("Edges", "Kanten"), T("All edges.", "Alle Kanten."), { shapeKinds: ["edge"] }),
        O("faces", "shapes", T("Faces", "Flächen"), T("All faces.", "Alle Flächen."), { shapeKinds: ["face"] }),
        O("shells", "shapes", T("Shells", "Schalen"), T("All shells.", "Alle Schalen."), { shapeKinds: ["shell"] }),
        O("selectedEdges", "shapes", T("Selected edges", "Gewählte Kanten"), T("The edges picked in the selection.", "Die in der Auswahl gewählten Kanten."), { shapeKinds: ["edge"], optional: true }),
        O("selectedFaces", "shapes", T("Selected faces", "Gewählte Flächen"), T("The faces picked in the selection.", "Die in der Auswahl gewählten Flächen."), { shapeKinds: ["face"], optional: true }),
      ],
      interaction: { pick: [pick("shape", "shape"), pick("edge", "edges"), pick("face", "faces")] },
      ren: { in: { brep: "shape", edgeLabels: "edges", faceLabels: "faces", sourceHandle: "(dropped)" }, out: { V: "vertices", E: "edges", F: "faces", S: "shells", SE: "selectedEdges", SF: "selectedFaces", SI: "(dropped)" } },
      note: "label texts become selection ports",
    }),
    K(C, {
      was: "brep.topology.shells", id: "brep.topology.shells", emoji: "🐚", quality: "exact-analytic", preview: true,
      label: T("Shells of solid", "Schalen eines Körpers"),
      description: T("Lists the outer shell and every inner void shell of a solid.", "Listet die äußere Schale und jede innere Hohlraumschale eines Körpers auf."),
      in: [I("solid")],
      out: [O("shells", "shapes", T("Shells", "Schalen"), T("One shape per shell.", "Eine Form pro Schale."), { shapeKinds: ["shell"] })],
      interaction: grab("solid"),
    }),
    K(C, {
      was: "brep.topology.compound", id: "brep.topology.compound", emoji: "🗃", quality: "exact-analytic", preview: true,
      label: T("Compound", "Verbund"),
      description: T("Groups several solids into one compound without merging them.", "Fasst mehrere Körper zu einem Verbund zusammen, ohne sie zu verschmelzen."),
      in: [P("solids", "shapes", T("Solids", "Körper"), T("The solids to group.", "Die zusammenzufassenden Körper."), { shapeKinds: ["solid", "compound"], minItems: 1 })],
      out: [O("shape", "shape", T("Compound", "Verbund"), T("The compound of all solids.", "Der Verbund aller Körper."), { shapeKinds: ["compound"] })],
      ren: { out: { compound: "shape" } },
    }),
    K(C, {
      was: "brep.topology.explode", id: "brep.topology.explode", emoji: "💥", quality: "exact-analytic", preview: true,
      label: T("Explode compound", "Verbund auflösen"),
      description: T("Splits a compound into its member solids.", "Zerlegt einen Verbund in seine Einzelkörper."),
      in: [I("shape", { as: "compound", shapeKinds: ["compound"], label: T("Compound", "Verbund"), description: T("The compound to split.", "Der aufzulösende Verbund.") })],
      out: [O("solids", "shapes", T("Solids", "Körper"), T("One shape per member solid.", "Eine Form pro enthaltenem Körper."), { shapeKinds: ["solid"] })],
      interaction: grab("compound"),
    }),
    K(C, {
      was: "brep.topology.label", id: "brep.topology.label", emoji: "🏷", quality: "exact-analytic",
      label: T("Persistent label", "Persistente Kennung"),
      description: T("Reads the stable label that identifies a shape across edits.", "Liest die stabile Kennung, die eine Form über Änderungen hinweg identifiziert."),
      in: [I("shape")],
      out: [O("label", "text", T("Label", "Kennung"), T("The label as a decimal number in text form.", "Die Kennung als Dezimalzahl in Textform."))],
      interaction: grab(),
    }),
    K(C, {
      was: "brep.util.sew", id: "brep.topology.sew", emoji: "🧵", quality: "exact-analytic", preview: true,
      label: T("Sew faces", "Flächen vernähen"),
      description: T("Joins faces that share edges into one closed solid.", "Verbindet Flächen mit gemeinsamen Kanten zu einem geschlossenen Körper."),
      in: [P("faces", "shapes", T("Faces", "Flächen"), T("The faces to sew; they must enclose a volume.", "Die zu vernähenden Flächen; sie müssen ein Volumen umschließen."), { shapeKinds: ["face"], minItems: 1 }), I("tolerance", { description: T("Largest gap between edges that is still closed.", "Größte Lücke zwischen Kanten, die noch geschlossen wird.") })],
      out: [O("shape", "shape", T("Solid", "Körper"), T("The sewn solid.", "Der vernähte Körper."), { shapeKinds: ["solid"] })],
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.util.heal", id: "brep.topology.heal", emoji: "🩹", quality: "exact-numerical", preview: true,
      label: T("Heal", "Heilen"),
      description: T("Repairs small gaps and inconsistencies in a solid.", "Repariert kleine Lücken und Unstimmigkeiten in einem Körper."),
      in: [I("shape", { shapeKinds: ["solid"], label: T("Solid", "Körper"), description: T("The solid to repair.", "Der zu reparierende Körper.") }), I("tolerance", { description: T("Largest gap that is still repaired.", "Größte Lücke, die noch repariert wird.") })],
      out: [O("shape", "shape", T("Solid", "Körper"), T("The repaired solid.", "Der reparierte Körper."), { shapeKinds: ["solid"] })],
      interaction: grab(),
      ren: { in: { geometry: "shape" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.util.convertToNurbs", id: "brep.topology.convertToNurbs", emoji: "〰", quality: "exact-analytic", preview: true,
      label: T("Convert to NURBS", "In NURBS umwandeln"),
      description: T("Rewrites every curve and surface of a shape as NURBS without changing its geometry.", "Schreibt jede Kurve und Fläche einer Form als NURBS um, ohne die Geometrie zu ändern."),
      in: [I("shape")],
      out: [O("shape", "shape", T("Shape", "Form"), T("The shape in NURBS form.", "Die Form in NURBS-Darstellung."))],
      interaction: grab(),
      ren: { in: { geometry: "shape" }, out: { geometryOut: "shape" } },
    }),
  ],
};
