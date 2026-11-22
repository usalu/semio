/** 🐞️ [DEBUG] Prints the world extents of every tessellated solid of one asset, scale and list in the default pose. Usage: dbg-solids.ts <asset json> <scale> [list] */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [file, scale, list] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }) as any;
for (const w of worldSolids(asset, st.bones, Number(scale), (list ?? "body") as never)) {
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < w.positions.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, w.positions[i]!); hi[i % 3] = Math.max(hi[i % 3]!, w.positions[i]!); }
  console.log(`[DEBUG] ${w.id.padEnd(18)} x ${lo[0]!.toFixed(3)}..${hi[0]!.toFixed(3)} y ${lo[1]!.toFixed(3)}..${hi[1]!.toFixed(3)} z ${lo[2]!.toFixed(3)}..${hi[2]!.toFixed(3)}`);
}
