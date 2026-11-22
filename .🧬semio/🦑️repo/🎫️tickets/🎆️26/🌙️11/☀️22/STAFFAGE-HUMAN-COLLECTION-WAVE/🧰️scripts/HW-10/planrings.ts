/** 🧭️ The evaluated plan outlines of the shapes of some bones of an asset file at a pose, with the world frames of those bones: bun planrings.ts <file> <pose> <bone,...>
 *  Prints { scale: { shapeId: { bone, ring: [[x, y], ...] } }, frames: { bone: { o, u, v, w } }, probe: plan centroid of the left hand (to read the sign of the lateral axis) }. */
import { evaluateDrawing } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { framesOf, loadAsset } from "./lib.ts";

const [file, pose, bones] = process.argv.slice(2);
const asset = await loadAsset(file!);
const keep = new Set(bones!.split(","));
const out: Record<string, unknown> = { frames: Object.fromEntries(Object.entries(framesOf(asset, pose!).bones).filter(([id]) => keep.has(id) || id === "hand-l")) };
for (const scale of [50, 100, 200, 500]) {
  const state = evaluateDrawing(asset, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: scale } });
  const shapes: Record<string, unknown> = {};
  for (const e of state.shapes.plan) {
    if (keep.has(e.shape.bone) && e.shape.filled && e.rings.length > 0) shapes[e.shape.id] = { bone: e.shape.bone, ring: e.rings[0]!.map((p) => [p.x, p.y]) };
    if (e.shape.bone === "hand-l" && e.rings.length > 0 && out.probe === undefined) out.probe = [e.rings[0]![0]!.x, e.rings[0]![0]!.y];
  }
  out[String(scale)] = shapes;
}
console.log(JSON.stringify(out));
