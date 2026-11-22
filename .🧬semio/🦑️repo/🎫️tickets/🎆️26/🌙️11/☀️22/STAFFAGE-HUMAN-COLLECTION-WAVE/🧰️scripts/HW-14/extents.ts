/** 📏️ Default-pose (or --pose) extents of every view and scale of an asset document, with the shapes at each extreme, and the volume bbox per scale: bun extents.ts <file.json> [--pose id] */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids, bodyDimensions } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const args = process.argv.slice(2);
const asset = JSON.parse(readFileSync(args[0]!, "utf8")).asset;
const pose = args.includes("--pose") ? args[args.indexOf("--pose") + 1]! : asset.presentation.defaultPose;
console.log("declared", JSON.stringify(asset.dimensions));
for (const scale of [500, 200, 100, 50]) {
  const s = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } } as never);
  const line: string[] = [];
  for (const view of ["elevation", "front", "plan"] as const) {
    let [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity];
    const who = ["", "", "", ""];
    for (const e of s.shapes[view]) for (const r of e.rings) for (const p of r) {
      if (p.x < x0) { x0 = p.x; who[0] = e.shape.id; } if (p.x > x1) { x1 = p.x; who[1] = e.shape.id; }
      if (p.y < y0) { y0 = p.y; who[2] = e.shape.id; } if (p.y > y1) { y1 = p.y; who[3] = e.shape.id; }
    }
    line.push(`${view} x ${x0.toFixed(3)}..${x1.toFixed(3)} (${(x1 - x0).toFixed(3)}; ${who[0]}|${who[1]}) y ${y0.toFixed(3)}..${y1.toFixed(3)} (${(y1 - y0).toFixed(3)}; ${who[2]}|${who[3]})`);
  }
  const st = evaluate(asset, { kind: "pose", pose });
  const bb = bodyDimensions(worldSolids(asset, st.bones as never, scale));
  console.log(`1:${scale} volume w/h/d ${bb.map((b) => b.toFixed(4)).join("/")}\n  ${line.join("\n  ")}`);
}
