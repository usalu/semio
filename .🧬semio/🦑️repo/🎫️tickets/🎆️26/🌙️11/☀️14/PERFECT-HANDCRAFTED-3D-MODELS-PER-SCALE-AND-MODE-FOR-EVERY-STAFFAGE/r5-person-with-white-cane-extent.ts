/** 📏 Prints the drawn extent of every shape of a view of the candidate `humans/person-with-white-cane` (the widest and narrowest first), to find what sets an extent.
 * Usage: `bun r5-person-with-white-cane-extent.ts <scale> <elevation|front|plan> [pose]` */
import { rigAt } from "./r3-lib.ts";
import { type Json } from "./volumes-lib.ts";
import { candidate } from "./r5-person-with-white-cane-io.ts";
const m = candidate(true);
const [scale, view, pose] = [Number(process.argv[2] ?? 50), process.argv[3] ?? "front", process.argv[4] ?? m.presentation.defaultPose];
const st = rigAt(m, scale, { kind: "pose", pose }).state.shapes as Json;
const rows: [string, number, number, number, number][] = [];
for (const s of st[view]) {
  let b = [Infinity, Infinity, -Infinity, -Infinity];
  for (const r of s.rings) for (const p of r) b = [Math.min(b[0]!, p.x), Math.min(b[1]!, p.y), Math.max(b[2]!, p.x), Math.max(b[3]!, p.y)];
  rows.push([s.shape.id, b[0]!, b[2]!, b[1]!, b[3]!]);
}
rows.sort((a, b) => a[1] - b[1]);
for (const r of rows.slice(0, 4)) console.log("min x", r[0], r[1].toFixed(3), r[2].toFixed(3));
rows.sort((a, b) => b[2] - a[2]);
for (const r of rows.slice(0, 4)) console.log("max x", r[0], r[1].toFixed(3), r[2].toFixed(3));
rows.sort((a, b) => b[4] - a[4]);
for (const r of rows.slice(0, 3)) console.log("max y", r[0], r[3].toFixed(3), r[4].toFixed(3));
