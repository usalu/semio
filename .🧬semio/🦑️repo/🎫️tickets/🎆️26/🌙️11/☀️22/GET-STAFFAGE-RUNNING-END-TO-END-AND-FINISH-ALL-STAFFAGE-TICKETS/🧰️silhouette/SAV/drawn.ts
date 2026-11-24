/** 📐️ The paper extents (model metres, default pose) of the evaluated drawn shapes of an asset whose id matches a pattern, per view, at one scale.
 *  bun drawn.ts <id|file.json> <scale> <regex> [view] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { fileOf } from "./files.ts";

const [name, scaleArg, pattern, only] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } });
for (const view of ["elevation", "front", "plan"]) {
  if (only !== undefined && only !== view) continue;
  for (const s of state.shapes[view] as any[]) {
    if (!new RegExp(pattern!, "u").test(s.shape.id)) continue;
    const pts = s.rings.flat();
    if (pts.length === 0) continue;
    const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
    console.log(`${view} ${s.shape.id}@${s.shape.bone}[${s.shape.role}] x ${Math.min(...xs).toFixed(4)}..${Math.max(...xs).toFixed(4)} y ${Math.min(...ys).toFixed(4)}..${Math.max(...ys).toFixed(4)} (${s.rings.length} rings)`);
  }
}
