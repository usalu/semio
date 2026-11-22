import { S } from "./lib.ts";
const { flattenGeometry, solidDrawingProjector } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const a = (await Bun.file(process.argv[2]!).json()).asset;
const project = solidDrawingProjector(a, { kind: "pose", pose: a.presentation.defaultPose });
for (const s of ["500", "200", "100", "50"]) for (const view of ["elevation", "front", "plan"]) for (const shape of a.representations[s].geometry[view]) {
  if (!shape.solid) continue;
  try { project(shape, flattenGeometry(shape).rings, view); } catch (e) { console.log("FAIL", s, view, shape.id, JSON.stringify(shape.points).slice(0, 300), JSON.stringify(shape.solid)); }
}
