import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";

const C = "brep.boolean";
const solid = (en, de) => O("shape", "shape", T("Solid", "Körper"), T(en, de), { shapeKinds: ["solid", "compound"] });
const operand = (name, en, de, den, dde) => I("a", { as: name, shapeKinds: ["solid", "compound"], label: T(en, de), description: T(den, dde) });

export default {
  category: {
    id: C, emoji: "🔗", order: 50,
    label: T("Booleans", "Boolesche Operationen"),
    description: T("Combine solids by union, difference and intersection.", "Körper durch Vereinigung, Differenz und Schnittmenge kombinieren."),
  },
  kinds: [
    K(C, {
      was: "brep.bool.fuse", id: "brep.boolean.fuse", emoji: "➕", quality: "exact-numerical", preview: true,
      label: T("Union", "Vereinigen"),
      description: T("Merges two solids into one that covers both.", "Verschmilzt zwei Körper zu einem, der beide abdeckt."),
      in: [operand("a", "Solid A", "Körper A", "The first solid.", "Der erste Körper."), operand("b", "Solid B", "Körper B", "The second solid.", "Der zweite Körper.")],
      out: [solid("The united solid.", "Der vereinigte Körper.")],
      interaction: { pick: [pick("shape", "a")] },
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.bool.cut", id: "brep.boolean.cut", emoji: "➖", quality: "exact-numerical", preview: true,
      label: T("Difference", "Subtrahieren"),
      description: T("Removes the second solid from the first.", "Entfernt den zweiten Körper aus dem ersten."),
      in: [operand("a", "Base", "Basis", "The solid to cut into.", "Der Körper, in den geschnitten wird."), operand("b", "Tool", "Werkzeug", "The solid that is removed.", "Der Körper, der entfernt wird.")],
      out: [solid("The solid after the cut.", "Der Körper nach dem Schnitt.")],
      interaction: { pick: [pick("shape", "a")] },
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.bool.intersect", id: "brep.boolean.intersect", emoji: "✖", quality: "exact-numerical", preview: true,
      label: T("Intersection", "Schnittmenge"),
      description: T("Keeps only the volume shared by both solids.", "Behält nur das Volumen, das beiden Körpern gemeinsam ist."),
      in: [operand("a", "Solid A", "Körper A", "The first solid.", "Der erste Körper."), operand("b", "Solid B", "Körper B", "The second solid.", "Der zweite Körper.")],
      out: [solid("The common volume.", "Das gemeinsame Volumen.")],
      interaction: { pick: [pick("shape", "a")] },
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.bool.compoundCut", id: "brep.boolean.compoundCut", emoji: "🔪", quality: "exact-numerical", preview: true,
      label: T("Difference with many tools", "Mehrfach subtrahieren"),
      description: T("Removes several tool solids from a target in one step.", "Entfernt mehrere Werkzeugkörper in einem Schritt aus einem Zielkörper."),
      in: [
        operand("target", "Target", "Ziel", "The solid to cut into.", "Der Körper, in den geschnitten wird."),
        P("tools", "shapes", T("Tools", "Werkzeuge"), T("The solids that are removed.", "Die Körper, die entfernt werden."), { shapeKinds: ["solid", "compound"], minItems: 1 }),
      ],
      out: [solid("The target after all cuts.", "Das Ziel nach allen Schnitten.")],
      interaction: { pick: [pick("shape", "target")] },
      ren: { out: { solid: "shape" } },
    }),
  ],
};
