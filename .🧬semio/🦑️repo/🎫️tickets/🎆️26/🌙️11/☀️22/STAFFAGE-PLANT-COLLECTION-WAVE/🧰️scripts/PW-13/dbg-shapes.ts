/** 🐞️ [DEBUG] Prints the world extents of every evaluated shape of one asset, scale and view in the default pose. Usage: dbg-shapes.ts <asset json> <scale> <view> */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, scale, view] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } }) as any;
for (const e of st.shapes[view!]) {
  const ps = e.rings.flat();
  const xs = ps.map((p: any) => p.x), ys = ps.map((p: any) => p.y);
  console.log(`[DEBUG] ${e.shape.id.padEnd(16)} ${e.shape.role.padEnd(8)} x ${Math.min(...xs).toFixed(3)}..${Math.max(...xs).toFixed(3)} y ${Math.min(...ys).toFixed(3)}..${Math.max(...ys).toFixed(3)} rings ${e.rings.length}`);
}
