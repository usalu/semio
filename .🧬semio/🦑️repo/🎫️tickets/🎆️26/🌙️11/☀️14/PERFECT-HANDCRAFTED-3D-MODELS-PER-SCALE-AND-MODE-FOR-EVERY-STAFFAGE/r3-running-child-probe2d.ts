/** 🔎️ Prints the forward (x) and second-axis extent of every evaluated 2D shape of the candidate document of `humans/running-child` in the default pose for one scale, side elevation against plan, to find where the constructed plan departs from the elevation.
 * Usage: `bun r3-running-child-probe2d.ts <scale>` */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { poseState } from "./volumes-lib.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "r3-running-child", "🔣️.json"), "utf8")).asset;
const st = poseState(asset, Number(process.argv[2])) as any;
const box = (rings: any[][]): number[] => {
  const p = rings.flat();
  return [Math.min(...p.map((q) => q.x)), Math.max(...p.map((q) => q.x)), Math.min(...p.map((q) => q.y)), Math.max(...p.map((q) => q.y))];
};
const rows = new Map<string, Record<string, number[]>>();
for (const view of ["elevation", "plan", "front"]) for (const e of st[view]) if (e.rings.length > 0 && e.shape.filled) rows.set(e.shape.id, { ...(rows.get(e.shape.id) ?? {}), [view]: box(e.rings) });
for (const [id, r] of rows) console.log(id.padEnd(16), ["elevation", "plan", "front"].map((v) => (r[v] === undefined ? "-".padEnd(34) : r[v]!.map((x) => x.toFixed(3)).join(" ").padEnd(34))).join(" | "));
