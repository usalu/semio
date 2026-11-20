/** 📐️ Prints, for one scale of the candidate document, the lateral and forward reach of every drawn shape in the default pose (front and plan), widest first: finds the shape that makes two views disagree.
 * Usage: `bun r3-standing-adult-extents.ts 50` */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { poseState, type Json } from "./volumes-lib.ts";
import { OUT } from "./r3-standing-adult-view.ts";

const scale = Number(process.argv[2] ?? 50);
const asset = JSON.parse(readFileSync(join(OUT, "🔣️.json"), "utf8")).asset as Json;
const st = poseState(asset, scale) as Json;
for (const view of ["elevation", "front", "plan"]) {
  const rows = (st[view] as Json[]).map((s) => {
    const p = (s.rings as { x: number; y: number }[][]).flat();
    return { id: s.shape.id as string, x0: Math.min(...p.map((q) => q.x)), x1: Math.max(...p.map((q) => q.x)), y0: Math.min(...p.map((q) => q.y)), y1: Math.max(...p.map((q) => q.y)) };
  });
  const f = (n: number): string => n.toFixed(4);
  console.log(view, "x", f(Math.min(...rows.map((r) => r.x0))), f(Math.max(...rows.map((r) => r.x1))), "y", f(Math.min(...rows.map((r) => r.y0))), f(Math.max(...rows.map((r) => r.y1))));
  for (const r of rows.sort((a, b) => Math.max(b.x1, -b.x0, b.y1, -b.y0) - Math.max(a.x1, -a.x0, a.y1, -a.y0)).slice(0, Number(process.argv[3] ?? 6))) console.log("  ", r.id.padEnd(16), f(r.x0), f(r.x1), f(r.y0), f(r.y1));
}
