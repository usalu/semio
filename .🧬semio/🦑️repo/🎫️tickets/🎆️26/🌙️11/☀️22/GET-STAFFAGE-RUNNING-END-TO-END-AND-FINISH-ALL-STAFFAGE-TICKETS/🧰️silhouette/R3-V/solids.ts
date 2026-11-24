/** 📦️ The world box of every body solid of an object at a scale in the default pose, and per axis the solids that reach furthest on both sides: which solid makes the bounding box.
 *  bun solids.ts <id|file.json> <scale> [count] [id-substring] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { fileOf } from "./people.ts";
const [name, sc, cnt, sub] = process.argv.slice(2);
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const boxes = worldSolids(asset, state.bones as never, Number(sc)).map((w: any) => {
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < w.positions.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, w.positions[i]); hi[i % 3] = Math.max(hi[i % 3]!, w.positions[i]); }
  return { id: w.id as string, bone: w.bone as string, slender: w.slender === true, lo, hi };
});
const f = (n: number): string => n.toFixed(4);
console.log(`${asset.id} 1:${sc} declared ${JSON.stringify(asset.dimensions)}`);
for (const k of [0, 1, 2]) {
  const n = Number(cnt ?? 3);
  console.log(`axis ${"xyz"[k]}: extent ${f(Math.max(...boxes.map((b) => b.hi[k]!)) - Math.min(...boxes.map((b) => b.lo[k]!)))}`);
  console.log(`  min: ${[...boxes].sort((a, b) => a.lo[k]! - b.lo[k]!).slice(0, n).map((b) => `${b.id}[${b.bone}]${b.slender ? "~" : ""} ${f(b.lo[k]!)}`).join("  ")}`);
  console.log(`  max: ${[...boxes].sort((a, b) => b.hi[k]! - a.hi[k]!).slice(0, n).map((b) => `${b.id}[${b.bone}]${b.slender ? "~" : ""} ${f(b.hi[k]!)}`).join("  ")}`);
}
if (sub !== undefined) for (const b of boxes) if (b.id.includes(sub)) console.log(`${b.id}[${b.bone}] x ${f(b.lo[0]!)} ${f(b.hi[0]!)} y ${f(b.lo[1]!)} ${f(b.hi[1]!)} z ${f(b.lo[2]!)} ${f(b.hi[2]!)}`);
