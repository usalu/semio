/** 🔎️ Prints the extent of every evaluated 2D shape of the candidate document in the default pose, to find the shape that sets a drawing extent.
 * Usage: `bun r3-runner-probe2d.ts <scale> <view>` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { poseState } from "./volumes-lib.ts";
import { OUT } from "./r3-runner-roots.ts";
const st = poseState(JSON.parse(readFileSync(join(OUT, "🔣️.json"), "utf8")).asset, Number(process.argv[2])) as any;
const rows: [string, number, number, number, number][] = [];
for (const e of st[process.argv[3]!]) {
  const p = e.rings.flat();
  if (p.length === 0) continue;
  rows.push([e.shape.id, Math.min(...p.map((q: any) => q.x)), Math.max(...p.map((q: any) => q.x)), Math.min(...p.map((q: any) => q.y)), Math.max(...p.map((q: any) => q.y))]);
}
for (const k of [1, 2, 3, 4]) {
  const sorted = [...rows].sort((a, b) => (k % 2 === 1 ? a[k]! - b[k]! : b[k]! - a[k]!)).slice(0, 3);
  console.log(["min x", "max x", "min y", "max y"][k - 1], sorted.map((r) => `${r[0]} ${r[k]!.toFixed(4)}`).join(" | "));
}
