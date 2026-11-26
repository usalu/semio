/** 📏️ The long straight runs a view draws: `bun segments.ts <id|file.json> <scale> <pose> <view> [least metres, default 0.5]` evaluates the pose and prints every straight run of a ring (consecutive segments that leave the line through the first and the last point of the run by less than 2 mm) at least that long with its shape, bone, kind, number of segments and end points in metres, longest first, so a straight cut seen on a sheet can be traced to the shape that draws it.
 * @see ./sheet.sh — the sheets */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
type P = { x: number; y: number };
const [id, scale, pose, view, least] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(fileOf(id!), "utf8")).asset;
const state = evaluate(asset, { kind: "pose", pose }, engine.scaleSelector(Number(scale)));
const found: { length: number; line: string }[] = [];
const off = (p: P, a: P, b: P): number => Math.abs((b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)) / Math.max(1e-12, Math.hypot(b.x - a.x, b.y - a.y));
for (const entry of state.shapes[view!]) for (const ring of entry.rings as P[][]) {
  for (let i = 0; i < ring.length; ) {
    let j = i + 1;
    while (j + 1 < ring.length && ring.slice(i + 1, j + 1).every((p) => off(p, ring[i]!, ring[j + 1]!) < 0.002) && Math.hypot(ring[j + 1]!.x - ring[i]!.x, ring[j + 1]!.y - ring[i]!.y) > Math.hypot(ring[j]!.x - ring[i]!.x, ring[j]!.y - ring[i]!.y)) j++;
    if (j >= ring.length) break;
    const [a, b] = [ring[i]!, ring[j]!];
    const length = Math.hypot(a.x - b.x, a.y - b.y);
    if (length >= Number(least ?? 0.5)) found.push({ length, line: `${length.toFixed(3)} m in ${j - i} segments: ${entry.shape.id} on ${entry.shape.bone} (${entry.shape.kind}${entry.shape.solid === undefined ? "" : ` ${entry.shape.solid.kind}`}, ${entry.shape.filled === true ? "filled" : "open"}, ring of ${ring.length}) from ${a.x.toFixed(3)},${a.y.toFixed(3)} to ${b.x.toFixed(3)},${b.y.toFixed(3)}` });
    i = j;
  }
}
for (const entry of found.sort((p, q) => q.length - p.length).slice(0, 30)) console.log(entry.line);
console.log(`${asset.id} 1:${scale} ${pose} ${view}: ${found.length} straight runs of at least ${least ?? 0.5} m`);
