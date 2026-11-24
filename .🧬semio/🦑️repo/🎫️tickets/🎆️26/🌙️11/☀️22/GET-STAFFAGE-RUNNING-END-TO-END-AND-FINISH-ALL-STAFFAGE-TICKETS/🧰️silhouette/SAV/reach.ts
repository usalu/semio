/** 📏️ The world extents (x ahead, y up, z lateral; default pose) of the body solids of an asset whose id matches a pattern, per authored scale.
 *  bun reach.ts <id|file.json> <regex> [scale…] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";

const [name, pattern, ...only] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
for (const d of [50, 100, 200, 500]) {
  if (asset.volumes[String(d)]?.status !== "authored" || (only.length > 0 && !only.includes(String(d)))) continue;
  for (const w of worldSolids(asset, state.bones as never, d)) {
    if (!new RegExp(pattern!, "u").test(w.id)) continue;
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < w.positions.length; i += 3) for (let k = 0; k < 3; k++) { min[k] = Math.min(min[k]!, w.positions[i + k]!); max[k] = Math.max(max[k]!, w.positions[i + k]!); }
    console.log(`1:${d} ${w.id}@${w.bone}${w.slender ? " slender" : ""}${w.texture ? " texture" : ""} x ${min[0]!.toFixed(3)}..${max[0]!.toFixed(3)} y ${min[1]!.toFixed(3)}..${max[1]!.toFixed(3)} z ${min[2]!.toFixed(3)}..${max[2]!.toFixed(3)}`);
  }
}
