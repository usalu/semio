/** 📏 Round 5, furniture: world extents (x length, y height, z depth, metres) of every solid of one list of one scale of a catalog asset (volumes source merged) in a pose, plus the whole list; the model side of the model-against-reference tables.
 * Usage: `bun r5-furniture-extent.ts <category/slug> <scale> [body|thermal|structure] [pose] [filter-regex]`
 * @see ./r5-brief.md */
import { load, meshesIn, rigAt } from "./r3-lib.ts";

const [spec, scale, list = "body", pose, filter] = process.argv.slice(2);
const { asset } = load(spec, "r5-furniture-extent.ts <category/slug> <scale> [list] [pose] [regex]");
const frames = rigAt(asset, Number(scale), { kind: "pose", pose: pose ?? asset.presentation.defaultPose }).frames;
const re = filter ? new RegExp(filter) : null;
const all = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
const f = (x: number): string => x.toFixed(3).padStart(7);
for (const s of asset.volumes[scale!][list]) {
  const b = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  for (const m of meshesIn(asset, frames, [s], Number(scale))) for (let i = 0; i < m.world.length; i++) { const k = i % 3; b[k] = Math.min(b[k]!, m.world[i]!); b[k + 3] = Math.max(b[k + 3]!, m.world[i]!); }
  for (let k = 0; k < 3; k++) { all[k] = Math.min(all[k]!, b[k]!); all[k + 3] = Math.max(all[k + 3]!, b[k + 3]!); }
  if (re && !re.test(s.id)) continue;
  console.log(`${String(s.id).padEnd(26)} x ${f(b[0]!)}..${f(b[3]!)} (${f(b[3]! - b[0]!)})  y ${f(b[1]!)}..${f(b[4]!)} (${f(b[4]! - b[1]!)})  z ${f(b[2]!)}..${f(b[5]!)} (${f(b[5]! - b[2]!)})`);
}
console.log(`${"ALL".padEnd(26)} x ${f(all[0]!)}..${f(all[3]!)} (${f(all[3]! - all[0]!)})  y ${f(all[1]!)}..${f(all[4]!)} (${f(all[4]! - all[1]!)})  z ${f(all[2]!)}..${f(all[5]!)} (${f(all[5]! - all[2]!)})`);
