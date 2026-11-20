/** 📦️ Prints the bounding box of the body list of the candidate document at every scale in the default pose (world metres) and, with `--write`, states `dimensions` (height, forward length as width, lateral breadth as depth) from the 1:50 body in `🗑️generated/r3-runner-with-blade-prosthesis/dimensions.json`.
 * Usage: `bun r3-runner-bbox.ts [--write]` */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { scaleMeshes } from "./volumes-lib.ts";
import { OUT } from "./r3-runner-roots.ts";
const m = JSON.parse(readFileSync(join(OUT, "🔣️.json"), "utf8")).asset;
for (const scale of [50, 100, 200, 500]) {
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const mesh of scaleMeshes(m, scale, m.volumes[String(scale)]).body) for (let i = 0; i < mesh.world.length; i++) [lo[i % 3], hi[i % 3]] = [Math.min(lo[i % 3]!, mesh.world[i]!), Math.max(hi[i % 3]!, mesh.world[i]!)];
  const e = hi.map((x, k) => x - lo[k]!);
  console.log(scale, "min", lo.map((x) => x.toFixed(4)).join(" "), "max", hi.map((x) => x.toFixed(4)).join(" "), "extent", e.map((x) => x.toFixed(4)).join(" "));
  if (scale === 50 && process.argv.includes("--write")) writeFileSync(join(OUT, "dimensions.json"), JSON.stringify({ heightM: Math.round(hi[1]! * 1000) / 1000, widthM: Math.round(e[0]! * 1000) / 1000, depthM: Math.round(e[2]! * 1000) / 1000 }));
}
