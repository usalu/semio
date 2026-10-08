import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";

const C = "brep.transform";
const PI = Math.PI;
const same = (en, de) => O("shape", "shape", T("Shape", "Form"), T(en, de));
const compound = (en, de) => O("shape", "shape", T("Pattern", "Muster"), T(en, de), { shapeKinds: ["compound"] });
const grab = { pick: [pick("shape", "shape")] };

export default {
  category: {
    id: C, emoji: "🔁", order: 70,
    label: T("Transforms and patterns", "Transformationen und Muster"),
    description: T("Move, rotate, scale, mirror and copy shapes, and repeat them in linear, circular and grid patterns.", "Formen verschieben, drehen, skalieren, spiegeln und kopieren sowie in linearen, kreisförmigen und Rastermustern wiederholen."),
  },
  kinds: [
    K(C, {
      was: "brep.xform.translate", id: "brep.transform.translate", emoji: "➡", quality: "exact-analytic", preview: true,
      label: T("Move", "Verschieben"),
      description: T("Moves a shape by an offset vector.", "Verschiebt eine Form um einen Versatzvektor."),
      in: [I("shape"), I("offset", { description: T("Displacement along X, Y and Z.", "Verschiebung entlang X, Y und Z.") })],
      out: [same("The moved shape.", "Die verschobene Form.")],
      interaction: { ...grab, gumball: [gumball("translate", "offset")] },
      ren: { in: { geometry: "shape" }, out: { geometryOut: "shape" } },
      note: "offset is a vector port (was a point port)",
    }),
    K(C, {
      was: "brep.xform.rotate", id: "brep.transform.rotate", emoji: "🔃", quality: "exact-analytic", preview: true,
      label: T("Rotate about origin", "Um Ursprung drehen"),
      description: T("Rotates a shape around an axis through the world origin.", "Dreht eine Form um eine Achse durch den Weltursprung."),
      in: [I("shape"), I("axis", { description: T("Direction of the rotation axis.", "Richtung der Drehachse.") }), I("angle")],
      out: [same("The rotated shape.", "Die gedrehte Form.")],
      interaction: { ...grab, gumball: [gumball("rotate", "angle", { axisPort: "axis" })] },
      ren: { in: { geometry: "shape" }, out: { geometryOut: "shape" } },
    }),
    K(C, {
      was: "brep.xform.rotateAbout", id: "brep.transform.rotateAbout", emoji: "🔄", quality: "exact-analytic", preview: true,
      label: T("Rotate about point", "Um Punkt drehen"),
      description: T("Rotates a shape around an axis through a chosen point.", "Dreht eine Form um eine Achse durch einen gewählten Punkt."),
      in: [I("shape"), I("origin", { label: T("Pivot", "Drehpunkt"), description: T("A point on the rotation axis.", "Ein Punkt auf der Drehachse.") }), I("axis", { description: T("Direction of the rotation axis.", "Richtung der Drehachse.") }), I("angle")],
      out: [same("The rotated shape.", "Die gedrehte Form.")],
      interaction: { ...grab, gumball: [gumball("rotate", "angle", { axisPort: "axis", originPort: "origin" })] },
      ren: { in: { geometry: "shape" }, out: { geometryOut: "shape" } },
    }),
    K(C, {
      was: "brep.xform.scale", id: "brep.transform.scale", emoji: "↔", quality: "exact-numerical", preview: true,
      label: T("Scale", "Skalieren"),
      description: T("Scales a shape independently along each axis about a center point.", "Skaliert eine Form unabhängig entlang jeder Achse um einen Mittelpunkt."),
      in: [I("shape"), I("factor"), I("center", { label: T("Center", "Zentrum"), description: T("The point that stays fixed.", "Der Punkt, der fest bleibt.") })],
      out: [same("The scaled shape.", "Die skalierte Form.")],
      interaction: { ...grab, gumball: [gumball("scale", "factor", { originPort: "center" })] },
      ren: { in: { geometry: "shape" }, out: { geometryOut: "shape" } },
    }),
    K(C, {
      was: "brep.xform.mirror", id: "brep.transform.mirror", emoji: "🪞", quality: "exact-analytic", preview: true,
      label: T("Mirror", "Spiegeln"),
      description: T("Reflects a shape across a plane.", "Spiegelt eine Form an einer Ebene."),
      in: [I("shape"), I("plane", { default: { origin: [0, 0, 0], normal: [1, 0, 0] }, description: T("The mirror plane.", "Die Spiegelebene.") })],
      out: [same("The mirrored shape.", "Die gespiegelte Form.")],
      interaction: { ...grab, gumball: [gumball("translate", "plane"), gumball("rotate", "plane")] },
      ren: { in: { geometry: "shape", "origin, normal": "plane" }, out: { geometryOut: "shape" } },
      note: "origin and normal merged into one plane port",
    }),
    K(C, {
      was: "brep.xform.copy", id: "brep.transform.copy", emoji: "📋", quality: "exact-analytic", preview: true,
      label: T("Copy", "Kopieren"),
      description: T("Creates an independent copy of a shape.", "Erzeugt eine unabhängige Kopie einer Form."),
      in: [I("shape")],
      out: [same("The copy.", "Die Kopie.")],
      interaction: grab,
      ren: { in: { geometry: "shape" }, out: { geometryOut: "shape" } },
    }),
    K(C, {
      was: "brep.xform.linearPattern", id: "brep.transform.linearPattern", emoji: "📶", quality: "mesh-derived-brep", preview: true,
      label: T("Linear pattern", "Lineares Muster"),
      description: T("Repeats a shape along a direction at a fixed spacing.", "Wiederholt eine Form entlang einer Richtung mit festem Abstand."),
      in: [I("shape"), I("direction"), I("distance", { as: "spacing", default: 1, min: 0, exclusiveMin: true, label: T("Spacing", "Abstand"), description: T("Distance between neighbouring copies.", "Abstand zwischen benachbarten Kopien.") }), I("count", { default: 3, min: 1, max: 1024 })],
      out: [compound("All copies as one compound.", "Alle Kopien als ein Verbund.")],
      interaction: grab,
      ren: { in: { geometry: "shape" }, out: { compound: "shape" } },
      note: "direction is a vector port (was a point port)",
    }),
    K(C, {
      was: "brep.xform.circularPattern", id: "brep.transform.circularPattern", emoji: "🎡", quality: "mesh-derived-brep", preview: true,
      label: T("Circular pattern", "Kreismuster"),
      description: T("Repeats a shape evenly around an axis through the origin.", "Wiederholt eine Form gleichmäßig um eine Achse durch den Ursprung."),
      in: [I("shape"), I("axis"), I("count", { default: 4, min: 1, max: 1024 })],
      out: [compound("All copies as one compound.", "Alle Kopien als ein Verbund.")],
      interaction: grab,
      ren: { in: { geometry: "shape" }, out: { compound: "shape" } },
    }),
    K(C, {
      was: "brep.xform.gridPattern", id: "brep.transform.gridPattern", emoji: "🔳", quality: "mesh-derived-brep", preview: true,
      label: T("Grid pattern", "Rastermuster"),
      description: T("Repeats a shape in a two-dimensional grid.", "Wiederholt eine Form in einem zweidimensionalen Raster."),
      in: [
        I("shape"),
        I("direction", { as: "dirX", label: T("Direction X", "Richtung X"), description: T("First grid direction.", "Erste Rasterrichtung.") }),
        I("direction", { as: "dirY", default: [0, 1, 0], label: T("Direction Y", "Richtung Y"), description: T("Second grid direction.", "Zweite Rasterrichtung.") }),
        I("distance", { as: "spacingX", default: 1, min: 0, exclusiveMin: true, label: T("Spacing X", "Abstand X"), description: T("Distance between copies along the first direction.", "Abstand der Kopien entlang der ersten Richtung.") }),
        I("distance", { as: "spacingY", default: 1, min: 0, exclusiveMin: true, label: T("Spacing Y", "Abstand Y"), description: T("Distance between copies along the second direction.", "Abstand der Kopien entlang der zweiten Richtung.") }),
        I("count", { as: "countX", default: 2, max: 1024, label: T("Count X", "Anzahl X"), description: T("Copies along the first direction.", "Kopien entlang der ersten Richtung.") }),
        I("count", { as: "countY", default: 2, max: 1024, label: T("Count Y", "Anzahl Y"), description: T("Copies along the second direction.", "Kopien entlang der zweiten Richtung.") }),
      ],
      out: [compound("All copies as one compound.", "Alle Kopien als ein Verbund.")],
      interaction: grab,
      ren: { in: { geometry: "shape" }, out: { compound: "shape" } },
    }),
  ],
};
