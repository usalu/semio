/** @emoji 🌱️ One-off: seed builtin typology assets from interaction folders. */
import { mkdirSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/compose/spatial/assets/extension/builtin";
const interactions: { category: string; id: string; label: string }[] = [
  { category: "primitive", id: "box", label: "Box" },
  { category: "curve", id: "arc", label: "Arc" },
  { category: "curve", id: "circle", label: "Circle" },
  { category: "curve", id: "control-point-curve", label: "Control Point Curve" },
  { category: "curve", id: "interpolate-curve", label: "Interpolate Curve" },
  { category: "curve", id: "line", label: "Line" },
  { category: "curve", id: "polyline", label: "Polyline" },
  { category: "edit", id: "chamfer", label: "Chamfer" },
  { category: "edit", id: "explode", label: "Explode" },
  { category: "edit", id: "fillet", label: "Fillet" },
  { category: "edit", id: "join", label: "Join" },
  { category: "edit", id: "split", label: "Split" },
  { category: "edit", id: "trim", label: "Trim" },
  { category: "feature", id: "extrude-wire", label: "Extrude Wire" },
  { category: "feature", id: "offset-surface", label: "Offset Surface" },
  { category: "measure", id: "area", label: "Area" },
  { category: "measure", id: "length", label: "Length" },
  { category: "solid", id: "boolean-difference", label: "Boolean Difference" },
  { category: "solid", id: "boolean-intersection", label: "Boolean Intersection" },
  { category: "solid", id: "boolean-union", label: "Boolean Union" },
  { category: "solid", id: "cylinder", label: "Cylinder" },
  { category: "solid", id: "sphere", label: "Sphere" },
  { category: "surface", id: "extrude-crv", label: "Extrude Curve" },
  { category: "surface", id: "loft", label: "Loft" },
  { category: "surface", id: "network-srf", label: "Network Surface" },
  { category: "surface", id: "plane", label: "Plane" },
  { category: "surface", id: "sweep1", label: "Sweep1" },
  { category: "surface", id: "sweep2", label: "Sweep2" },
  { category: "transform", id: "copy", label: "Copy" },
  { category: "transform", id: "mirror", label: "Mirror" },
  { category: "transform", id: "move", label: "Move" },
  { category: "transform", id: "rotate", label: "Rotate" },
  { category: "transform", id: "scale1d", label: "Scale 1D" },
  { category: "transform", id: "scale3d", label: "Scale 3D" },
];

for (const row of interactions) {
  const dir = join(root, "typology", row.category);
  if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
  const interactionId = `${row.category}.${row.id}`;
  const typologyId = `builtin.${row.category}.${row.id}`;
  const doc = {
    schema: "spatial.typology/v1",
    id: typologyId,
    version: "1.0.0",
    label: row.label,
    description: `Typology produced by ${interactionId}.`,
    actions: [`builtin.${interactionId}`],
    interactions: [interactionId],
  };
  writeFileSync(join(dir, `${row.id}.json`), JSON.stringify(doc, null, 2) + "\n");
}
