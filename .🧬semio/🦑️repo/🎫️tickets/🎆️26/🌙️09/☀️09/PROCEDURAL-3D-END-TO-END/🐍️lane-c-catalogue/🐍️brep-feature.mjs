import { T, I, O, P, K, pick, gumball, sel } from "./🐍️lib.mjs";

const C = "brep.feature";
const solid = (en, de) => O("shape", "shape", T("Solid", "Körper"), T(en, de), { shapeKinds: ["solid"] });
const body = () => I("shape", { shapeKinds: ["solid"], label: T("Solid", "Körper"), description: T("The solid to modify.", "Der zu verändernde Körper.") });
const edges = (extra = {}) => P("edges", "selection", T("Edges", "Kanten"), T("The edges to treat.", "Die zu bearbeitenden Kanten."), { ...sel("edge", "shape"), default: [], minItems: 1, ...extra });
const faces = (en, de, extra = {}) => P("faces", "selection", T("Faces", "Flächen"), T(en, de), { ...sel("face", "shape"), default: [], minItems: 1, ...extra });
const round = (name, en, de, den, dde, def) => P(name, "length", T(en, de), T(den, dde), { default: def, min: 0, exclusiveMin: true });
const wholeBody = { pick: [pick("shape", "shape")] };

export default {
  category: {
    id: C, emoji: "🛠", order: 60,
    label: T("Features", "Features"),
    description: T("Local edits of a solid: fillets, chamfers, shelling, draft, offset, thickening and defeaturing.", "Lokale Änderungen an einem Körper: Verrunden, Fasen, Aushöhlen, Entformungsschräge, Versatz, Verdicken und Merkmale entfernen."),
  },
  kinds: [
    K(C, {
      was: "brep.solid.fillet", id: "brep.feature.fillet", emoji: "🔘", quality: "exact-analytic", preview: true,
      label: T("Fillet all edges", "Alle Kanten verrunden"),
      description: T("Rounds every edge of a solid with a constant radius; faces with holes are not supported yet.", "Rundet jede Kante eines Körpers mit konstantem Radius ab; Flächen mit Löchern werden noch nicht unterstützt."),
      in: [body(), round("radius", "Radius", "Radius", "Radius of the rounding.", "Radius der Verrundung.", 0.1)],
      out: [solid("The solid with rounded edges.", "Der Körper mit verrundeten Kanten.")],
      interaction: wholeBody,
      ren: { in: { geometry: "shape" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.solid.filletEdges", id: "brep.feature.filletEdges", emoji: "⭕", quality: "exact-analytic", preview: true,
      label: T("Fillet edges", "Kanten verrunden"),
      description: T("Rounds only the selected edges with a constant radius.", "Rundet nur die ausgewählten Kanten mit konstantem Radius ab."),
      in: [body(), edges({ description: T("The edges to round.", "Die zu rundenden Kanten.") }), round("radius", "Radius", "Radius", "Radius of the rounding.", "Radius der Verrundung.", 0.1)],
      out: [solid("The solid with the selected edges rounded.", "Der Körper mit verrundeten Auswahlkanten.")],
      interaction: { pick: [pick("shape", "shape"), pick("edge", "edges")] },
      ren: { in: { geometry: "shape", edges: "edges (selection of persistent labels)" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.solid.filletVariable", id: "brep.feature.filletVariable", emoji: "🌗", quality: "exact-numerical", preview: true,
      label: T("Variable fillet", "Verrundung mit variablem Radius"),
      description: T("Rounds the edges with a radius that changes from a start to an end value.", "Rundet die Kanten mit einem Radius ab, der sich von einem Start- zu einem Endwert ändert."),
      in: [body(), round("radiusStart", "Start radius", "Startradius", "Radius at the start of each edge.", "Radius am Anfang jeder Kante.", 0.1), round("radiusEnd", "End radius", "Endradius", "Radius at the end of each edge.", "Radius am Ende jeder Kante.", 0.2)],
      out: [solid("The solid with variably rounded edges.", "Der Körper mit variabel verrundeten Kanten.")],
      interaction: wholeBody,
      ren: { in: { geometry: "shape" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.solid.chamfer", id: "brep.feature.chamfer", emoji: "🔻", quality: "exact-analytic", preview: true,
      label: T("Chamfer all edges", "Alle Kanten fasen"),
      description: T("Bevels every edge of a solid by the same distance on both faces.", "Versieht jede Kante eines Körpers mit einer Fase, die auf beiden Flächen gleich breit ist."),
      in: [body(), round("distance", "Distance", "Abstand", "Distance of the bevel from the edge on both faces.", "Abstand der Fase von der Kante auf beiden Flächen.", 0.1)],
      out: [solid("The solid with bevelled edges.", "Der Körper mit Fasen an allen Kanten.")],
      interaction: wholeBody,
      ren: { in: { geometry: "shape" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.solid.chamferEdges", id: "brep.feature.chamferEdges", emoji: "🔶", quality: "exact-analytic", preview: true,
      label: T("Chamfer edges", "Kanten fasen"),
      description: T("Bevels only the selected edges by the same distance on both faces.", "Versieht nur die ausgewählten Kanten mit einer Fase, die auf beiden Flächen gleich breit ist."),
      in: [body(), edges({ description: T("The edges to bevel.", "Die Kanten, die eine Fase erhalten.") }), round("distance", "Distance", "Abstand", "Distance of the bevel from the edge on both faces.", "Abstand der Fase von der Kante auf beiden Flächen.", 0.1)],
      out: [solid("The solid with the selected edges bevelled.", "Der Körper mit Fasen an den gewählten Kanten.")],
      interaction: { pick: [pick("shape", "shape"), pick("edge", "edges")] },
      ren: { in: { geometry: "shape", edges: "edges (selection of persistent labels)" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.solid.chamferAsymmetric", id: "brep.feature.chamferAsymmetric", emoji: "📐", quality: "exact-analytic", preview: true,
      label: T("Asymmetric chamfer", "Asymmetrische Fase"),
      description: T("Bevels every edge with two different distances.", "Versieht jede Kante mit einer Fase, deren beide Schenkel unterschiedlich lang sind."),
      in: [body(), round("d1", "Distance 1", "Abstand 1", "Distance of the bevel on the first face.", "Abstand der Fase auf der ersten Fläche.", 0.1), round("d2", "Distance 2", "Abstand 2", "Distance of the bevel on the second face.", "Abstand der Fase auf der zweiten Fläche.", 0.1)],
      out: [solid("The solid with asymmetrically bevelled edges.", "Der Körper mit asymmetrischen Fasen.")],
      interaction: wholeBody,
      ren: { in: { geometry: "shape" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.solid.shell", id: "brep.feature.shell", emoji: "🥚", quality: "exact-numerical", preview: true,
      label: T("Shell", "Aushöhlen"),
      description: T("Hollows out a solid to a uniform wall thickness, optionally opening some faces.", "Höhlt einen Körper auf eine gleichmäßige Wandstärke aus und öffnet auf Wunsch einzelne Flächen."),
      in: [body(), P("thickness", "length", T("Thickness", "Wandstärke"), T("Wall thickness; positive values grow inward.", "Wandstärke; positive Werte wachsen nach innen."), { default: 0.1, min: 0, exclusiveMin: true }), faces("The faces that stay open; leave empty for a closed hollow.", "Die Flächen, die offen bleiben; leer lassen für einen geschlossenen Hohlraum.", { as: undefined, name: "openFaces", minItems: undefined, optional: true })],
      out: [solid("The hollowed solid.", "Der ausgehöhlte Körper.")],
      interaction: { pick: [pick("shape", "shape"), pick("face", "openFaces")] },
      ren: { in: { geometry: "shape", openFaces: "openFaces (selection of persistent labels)" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.solid.draft", id: "brep.feature.draft", emoji: "📏", quality: "exact-numerical", preview: true,
      label: T("Draft", "Entformungsschräge"),
      description: T("Tilts the selected faces against a pull direction; only planar and cylindrical faces taper exactly.", "Neigt die ausgewählten Flächen gegen eine Entformungsrichtung; nur ebene und zylindrische Flächen werden exakt konisch."),
      in: [
        body(), faces("The faces to tilt.", "Die zu neigenden Flächen."),
        I("direction", { as: "pullDirection", default: [0, 0, 1], label: T("Pull direction", "Entformungsrichtung"), description: T("Direction in which the part is pulled out of the mold.", "Richtung, in der das Teil aus der Form gezogen wird.") }),
        I("point", { as: "neutralPoint", label: T("Neutral point", "Neutralpunkt"), description: T("A point on the neutral plane that stays fixed.", "Ein Punkt der neutralen Ebene, der fest bleibt.") }),
        I("angle", { default: 0.1, min: -1.5, max: 1.5, description: T("Draft angle against the pull direction.", "Entformungswinkel gegen die Entformungsrichtung.") }),
      ],
      out: [solid("The solid with drafted faces.", "Der Körper mit geneigten Flächen.")],
      interaction: { pick: [pick("shape", "shape"), pick("face", "faces")] },
      ren: { in: { geometry: "shape", faces: "faces (selection of persistent labels)" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.solid.offsetSolid", id: "brep.feature.offsetSolid", emoji: "↔", quality: "exact-numerical", preview: true,
      label: T("Offset solid", "Körper versetzen"),
      description: T("Grows or shrinks a solid by moving all faces along their normals.", "Vergrößert oder verkleinert einen Körper, indem alle Flächen entlang ihrer Normalen verschoben werden."),
      in: [body(), I("distance", { default: 0.1, description: T("Offset distance; negative values shrink the solid.", "Versatzabstand; negative Werte verkleinern den Körper.") })],
      out: [solid("The offset solid.", "Der versetzte Körper.")],
      interaction: wholeBody,
      ren: { in: { geometry: "shape" }, out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.surf.thicken", id: "brep.feature.thicken", emoji: "🧈", quality: "exact-numerical", preview: true,
      label: T("Thicken face", "Fläche verdicken"),
      description: T("Gives a face a thickness and turns it into a solid.", "Gibt einer Fläche eine Dicke und macht sie zu einem Körper."),
      in: [I("face"), I("thickness", { description: T("Thickness along the face normal.", "Dicke entlang der Flächennormale.") })],
      out: [solid("The thickened solid.", "Der verdickte Körper.")],
      interaction: { pick: [pick("face", "face")], gumball: [gumball("translate", "thickness", { along: "normal" })] },
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.solid.defeature", id: "brep.feature.defeature", emoji: "🧽", quality: "mesh-derived-brep", preview: true,
      label: T("Remove faces", "Merkmale entfernen"),
      description: T("Removes selected faces such as holes and bosses and heals the gap; the result is rebuilt from a mesh.", "Entfernt ausgewählte Flächen wie Bohrungen und Erhebungen und schließt die Lücke; das Ergebnis wird aus einem Netz neu aufgebaut."),
      in: [body(), faces("The faces to remove.", "Die zu entfernenden Flächen.")],
      out: [solid("The solid without the selected features.", "Der Körper ohne die gewählten Merkmale.")],
      interaction: { pick: [pick("shape", "shape"), pick("face", "faces")] },
      ren: { in: { geometry: "shape", faces: "faces (selection of persistent labels)" }, out: { solid: "shape" } },
    }),
  ],
};
