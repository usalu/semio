/** 🔬️ Prints the extent of every evaluated 2D shape of one view, scale and pose of the candidate: `$BUN author-probe.ts <scale> <view> [pose]`. */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const asset = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")).asset;
const [scale, view, pose] = [Number(process.argv[2]), process.argv[3] as "plan", process.argv[4] ?? asset.presentation.defaultPose];
const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
for (const s of state.shapes[view]) {
  const pts = s.rings.flat();
  const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y);
  console.log("[DEBUG]", s.shape.id.padEnd(22), "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3));
}
import { extentOf, project, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
if (process.argv[5] === "3d") {
  for (const w of worldSolids(asset, state.bones as any, scale, "body")) {
    const e = extentOf([w]);
    const a = project(view, e.min[0], e.min[1], e.min[2]), b = project(view, e.max[0], e.max[1], e.max[2]);
    console.log("[DEBUG] 3d", w.id.padEnd(22), "x", Math.min(a.x, b.x).toFixed(3), Math.max(a.x, b.x).toFixed(3), "y", Math.min(a.y, b.y).toFixed(3), Math.max(a.y, b.y).toFixed(3));
  }
}
