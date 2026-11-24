/** 🧿️ The evaluated rings of drawn shapes of one view (count, points, box, first points), optional ops applied first: bun rings.ts <id|file.json> <scale> <view> <id-substring> [ops.json|-] [pose] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./people.ts";
const [name, sc, view, sub, opsArg, pose] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsArg && opsArg !== "-") applyOps(asset, JSON.parse(await Bun.file(opsArg).text()));
const state: any = evaluate(asset, { kind: "pose", pose: pose ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
for (const s of state.shapes[view!]) if (String(s.shape.id).includes(sub!)) {
  console.log(`${s.shape.id} closed=${s.closed} rings=${s.rings.length}`);
  for (const ring of s.rings) console.log(`   ${ring.length} points x ${Math.min(...ring.map((p: any) => p.x)).toFixed(4)} ${Math.max(...ring.map((p: any) => p.x)).toFixed(4)} y ${Math.min(...ring.map((p: any) => p.y)).toFixed(4)} ${Math.max(...ring.map((p: any) => p.y)).toFixed(4)}  ${ring.slice(0, 4).map((p: any) => `(${p.x.toFixed(4)},${p.y.toFixed(4)})`).join(" ")}`);
}
