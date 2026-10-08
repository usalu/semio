import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";
import { meshIn, meshOut, pickMany, pickOne, modePort, modeSelection, pivotPort, pivotPoint } from "./🐍️mesh-common.mjs";

const PI = Math.PI;
const grab = { pick: [pick("mesh", "mesh")] };
const axisOptions = [
  { value: "x", label: T("X axis", "X-Achse") },
  { value: "y", label: T("Y axis", "Y-Achse") },
  { value: "z", label: T("Z axis", "Z-Achse") },
];

export const transform = {
  category: {
    id: "mesh.transform", emoji: "🔁", order: 220,
    label: T("Mesh transforms", "Netz-Transformationen"),
    description: T("Move, rotate, scale, mirror or apply a matrix to a whole mesh.", "Ein ganzes Netz verschieben, drehen, skalieren, spiegeln oder mit einer Matrix transformieren."),
  },
  kinds: [
    K("mesh.transform", {
      was: "brep.mesh.translate", id: "mesh.transform.translate", emoji: "➡", quality: "polygon-mesh", preview: true,
      label: T("Move mesh", "Netz verschieben"),
      description: T("Moves every vertex by an offset vector.", "Verschiebt jeden Eckpunkt um einen Versatzvektor."),
      in: [meshIn(), I("offset", { description: T("Displacement along X, Y and Z.", "Verschiebung entlang X, Y und Z.") })],
      out: [meshOut("The moved mesh.", "Das verschobene Netz.")],
      interaction: { ...grab, gumball: [gumball("translate", "offset")] },
      ren: { out: { meshOut: "mesh" } },
    }),
    K("mesh.transform", {
      was: "brep.mesh.rotate", id: "mesh.transform.rotate", emoji: "🔃", quality: "polygon-mesh", preview: true,
      label: T("Rotate mesh", "Netz drehen"),
      description: T("Rotates the mesh around an axis through the origin.", "Dreht das Netz um eine Achse durch den Ursprung."),
      in: [meshIn(), I("axis", { description: T("Direction of the rotation axis.", "Richtung der Drehachse.") }), I("angle")],
      out: [meshOut("The rotated mesh.", "Das gedrehte Netz.")],
      interaction: { ...grab, gumball: [gumball("rotate", "angle", { axisPort: "axis" })] },
      ren: { out: { meshOut: "mesh" } },
      note: "angle default changed from 0 to a quarter pi",
    }),
    K("mesh.transform", {
      was: "brep.mesh.scale", id: "mesh.transform.scale", emoji: "↔", quality: "polygon-mesh", preview: true,
      label: T("Scale mesh", "Netz skalieren"),
      description: T("Scales each axis about the origin; negative factors mirror the mesh and keep the faces facing outward.", "Skaliert jede Achse um den Ursprung; negative Faktoren spiegeln das Netz und halten die Flächen nach außen gerichtet."),
      in: [meshIn(), I("factor")],
      out: [meshOut("The scaled mesh.", "Das skalierte Netz.")],
      interaction: { ...grab, gumball: [gumball("scale", "factor")] },
      ren: { out: { meshOut: "mesh" } },
    }),
    K("mesh.transform", {
      was: "brep.mesh.transform", id: "mesh.transform.matrix", emoji: "🧮", quality: "polygon-mesh", preview: true,
      label: T("Transform by matrix", "Mit Matrix transformieren"),
      description: T("Applies a 4x4 affine matrix given column by column; normals and face winding follow.", "Wendet eine affine 4x4-Matrix an, spaltenweise angegeben; Normalen und Flächenorientierung folgen."),
      in: [meshIn(), P("matrix", "number", T("Matrix", "Matrix"), T("The 16 matrix entries in column-major order.", "Die 16 Matrixeinträge in spaltenweiser Reihenfolge."), { list: true, minItems: 16, maxItems: 16, default: [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1] })],
      out: [meshOut("The transformed mesh.", "Das transformierte Netz.")],
      interaction: grab,
      ren: { out: { meshOut: "mesh" } },
    }),
    K("mesh.transform", {
      was: "brep.mesh.mirror", id: "mesh.transform.mirror", emoji: "🪞", quality: "polygon-mesh", preview: true,
      label: T("Mirror mesh half", "Netzhälfte spiegeln"),
      description: T("Reflects a one-sided mesh across an axis plane through the origin and welds the seam.", "Spiegelt ein einseitiges Netz an einer Achsenebene durch den Ursprung und verschweißt die Naht."),
      in: [meshIn(), P("axis", "enum", T("Mirror plane", "Spiegelebene"), T("The axis whose perpendicular plane through the origin is the mirror.", "Die Achse, deren senkrechte Ebene durch den Ursprung die Spiegelebene ist."), { default: "x", options: axisOptions }), I("tolerance", { default: 0.0001, description: T("Largest distance at which seam vertices are welded.", "Größter Abstand, bei dem Nahtpunkte verschweißt werden.") })],
      out: [meshOut("The mirrored mesh.", "Das gespiegelte Netz.")],
      interaction: grab,
      ren: { out: { meshOut: "mesh" } },
      note: "axis is now an enum port (was free text)",
    }),
  ],
};

export const component = {
  category: {
    id: "mesh.component", emoji: "🎚", order: 230,
    label: T("Mesh components", "Netzelemente"),
    description: T("Move, rotate and scale selected vertices, edges or faces.", "Ausgewählte Eckpunkte, Kanten oder Flächen verschieben, drehen und skalieren."),
  },
  kinds: [
    K("mesh.component", {
      was: "brep.mesh.moveVertices", id: "mesh.component.moveVertices", emoji: "📍", quality: "polygon-mesh", preview: true,
      label: T("Move vertices", "Eckpunkte verschieben"),
      description: T("Moves the selected vertices by an offset vector.", "Verschiebt die ausgewählten Eckpunkte um einen Versatzvektor."),
      in: [meshIn(), pickMany("vertices", "vertex", "The vertices to move.", "Die zu verschiebenden Eckpunkte."), I("offset")],
      out: [meshOut("The mesh with moved vertices.", "Das Netz mit verschobenen Eckpunkten.")],
      interaction: { pick: [pick("mesh", "mesh"), pick("vertex", "vertices")], gumball: [gumball("translate", "offset")] },
      ren: { out: { meshOut: "mesh" } },
    }),
    K("mesh.component", {
      was: "brep.mesh.translateComponents", id: "mesh.component.translate", emoji: "➡", quality: "polygon-mesh", preview: true,
      label: T("Move components", "Elemente verschieben"),
      description: T("Moves selected vertices, edges or faces; shared vertices move once.", "Verschiebt ausgewählte Eckpunkte, Kanten oder Flächen; gemeinsame Eckpunkte werden nur einmal bewegt."),
      in: [meshIn(), modePort(), modeSelection("The components to move.", "Die zu verschiebenden Elemente."), I("offset")],
      out: [meshOut("The mesh with moved components.", "Das Netz mit verschobenen Elementen.")],
      interaction: { pick: [pick("mesh", "mesh"), pick("vertex", "selection"), pick("edge", "selection"), pick("face", "selection")], gumball: [gumball("translate", "offset")] },
      ren: { out: { meshOut: "mesh" } },
    }),
    K("mesh.component", {
      was: "brep.mesh.rotateComponents", id: "mesh.component.rotate", emoji: "🔃", quality: "polygon-mesh", preview: true,
      label: T("Rotate components", "Elemente drehen"),
      description: T("Rotates selected vertices, edges or faces around their center or a chosen point.", "Dreht ausgewählte Eckpunkte, Kanten oder Flächen um ihre Mitte oder einen gewählten Punkt."),
      in: [meshIn(), modePort(), modeSelection("The components to rotate.", "Die zu drehenden Elemente."), pivotPort(), pivotPoint(), I("axis", { description: T("Direction of the rotation axis.", "Richtung der Drehachse.") }), I("angle")],
      out: [meshOut("The mesh with rotated components.", "Das Netz mit gedrehten Elementen.")],
      interaction: { pick: [pick("mesh", "mesh"), pick("vertex", "selection"), pick("edge", "selection"), pick("face", "selection")], gumball: [gumball("rotate", "angle", { axisPort: "axis", originPort: "center" })] },
      ren: { out: { meshOut: "mesh" } },
      note: "angle default changed from 0 to a quarter pi",
    }),
    K("mesh.component", {
      was: "brep.mesh.scaleComponents", id: "mesh.component.scale", emoji: "↔", quality: "polygon-mesh", preview: true,
      label: T("Scale components", "Elemente skalieren"),
      description: T("Scales selected vertices, edges or faces about their center or a chosen point.", "Skaliert ausgewählte Eckpunkte, Kanten oder Flächen um ihre Mitte oder einen gewählten Punkt."),
      in: [meshIn(), modePort(), modeSelection("The components to scale.", "Die zu skalierenden Elemente."), pivotPort(), pivotPoint(), I("factor")],
      out: [meshOut("The mesh with scaled components.", "Das Netz mit skalierten Elementen.")],
      interaction: { pick: [pick("mesh", "mesh"), pick("vertex", "selection"), pick("edge", "selection"), pick("face", "selection")], gumball: [gumball("scale", "factor", { originPort: "center" })] },
      ren: { out: { meshOut: "mesh" } },
    }),
    K("mesh.component", {
      was: "brep.mesh.moveProportional", id: "mesh.component.moveProportional", emoji: "🌊", quality: "polygon-mesh", preview: true,
      label: T("Move proportionally", "Proportional verschieben"),
      description: T("Moves the selected vertices fully and nearby vertices with a linear falloff.", "Verschiebt die ausgewählten Eckpunkte vollständig und nahe Eckpunkte mit linearem Abfall."),
      in: [meshIn(), pickMany("vertices", "vertex", "The vertices that move fully.", "Die Eckpunkte, die sich vollständig bewegen."), I("offset"), I("center", { label: T("Falloff center", "Abfallzentrum"), description: T("Center from which the influence radius is measured.", "Zentrum, von dem aus der Einflussradius gemessen wird.") }), I("radius", { label: T("Influence radius", "Einflussradius"), description: T("Distance beyond which vertices stay unmoved.", "Abstand, ab dem Eckpunkte unbewegt bleiben.") })],
      out: [meshOut("The deformed mesh.", "Das verformte Netz.")],
      interaction: { pick: [pick("mesh", "mesh"), pick("vertex", "vertices")], gumball: [gumball("translate", "offset")] },
      ren: { in: { selection: "vertices" }, out: { meshOut: "mesh" } },
    }),
    K("mesh.component", {
      was: "brep.mesh.snapVertices", id: "mesh.component.snapToGrid", emoji: "🔲", quality: "polygon-mesh", preview: true,
      label: T("Snap vertices to grid", "Eckpunkte am Raster einrasten"),
      description: T("Rounds the selected vertices to the nearest point of an origin-aligned grid.", "Rundet die ausgewählten Eckpunkte auf den nächsten Punkt eines am Ursprung ausgerichteten Rasters."),
      in: [meshIn(), pickMany("vertices", "vertex", "The vertices to snap.", "Die einzurastenden Eckpunkte."), P("grid", "length", T("Grid spacing", "Rasterweite"), T("Distance between grid lines.", "Abstand der Rasterlinien."), { default: 1, min: 0, exclusiveMin: true })],
      out: [meshOut("The snapped mesh.", "Das eingerastete Netz.")],
      interaction: { pick: [pick("mesh", "mesh"), pick("vertex", "vertices")] },
      ren: { in: { selection: "vertices" }, out: { meshOut: "mesh" } },
    }),
  ],
};
