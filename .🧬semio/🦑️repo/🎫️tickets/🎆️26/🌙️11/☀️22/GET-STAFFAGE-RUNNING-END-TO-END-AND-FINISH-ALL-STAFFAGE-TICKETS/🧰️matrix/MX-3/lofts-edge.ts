/** 📏️ The plan extent of the shapes of one asset in the evaluated state (metres, world) and in the drawn scene (paper box / mmPerMetre) at several denominators of one representation: lofts-edge.ts <id> <representation key> <shape id> <denominators…> */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { resolveViews } = await import(join(S, "🔨️modules", "🔭️multiview", "🟦️.ts"));
const [id, key, shapeId, ...dens] = process.argv.slice(2);
const asset = readDoc(id!).doc.asset;
for (const denominator of dens.map(Number)) {
  const multi = resolveViews(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", denominator }, "light-line", engine.drawOptions("light-line", denominator));
  const drawn = multi.views.plan!;
  const k = drawn.scene.metadata.mmPerMetre;
  const e = drawn.state.shapes.plan.find((s: any) => s.shape.id === shapeId)!;
  const xs = e.rings.flat().map((p: any) => p.x);
  const box = engine.paperBox(drawn.scene, [e]);
  console.log(`1:${denominator} ${shapeId}: state x ${Math.min(...xs).toFixed(4)}..${Math.max(...xs).toFixed(4)} (${(Math.max(...xs) - Math.min(...xs)).toFixed(4)} m, ${e.rings.flat().length} pts); paper box ${((box.maxX - box.minX) / k).toFixed(4)} m; k ${k}`);
}
