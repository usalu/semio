/** 📦️ The world box of the body solids of an asset at a scale in the default pose (metres; x ahead, y up, z left), optionally only those whose id contains a substring or whose top is the highest over their footprint centre.
 *  bun solids.ts <id|file.json> <scale> [id-substring] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";
const [name, sc, sub] = process.argv.slice(2);
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const f = (n: number): string => n.toFixed(4);
for (const w of worldSolids(asset, state.bones as never, Number(sc)) as any[]) {
  if (sub !== undefined && !String(w.id).includes(sub)) continue;
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < w.positions.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, w.positions[i]); hi[i % 3] = Math.max(hi[i % 3]!, w.positions[i]); }
  console.log(`${w.id}[${w.bone}]${w.texture === true ? " texture" : ""} x ${f(lo[0]!)} ${f(hi[0]!)} y ${f(lo[1]!)} ${f(hi[1]!)} z ${f(lo[2]!)} ${f(hi[2]!)}`);
}
