/** 📏 Per-solid world extents of one list of one scale of an asset in its default pose (candidate document with the volumes source merged), sorted by the chosen axis; for finding the part that sets an overall extent.
 * Usage: `bun r5-extent.ts <io module> <scale> [body|thermal|structure] [x|y|z]` (io module: `./r3-leaning-person-io.ts` or `./r3-person-cooking-io.ts`) */
import { meshesIn, rigAt } from "./r3-lib.ts";

const [io, scale, list = "body", axis = "x"] = process.argv.slice(2);
const { candidate } = await import(io!);
const asset = candidate(true);
const k = { x: 0, y: 1, z: 2 }[axis as "x" | "y" | "z"];
const frames = rigAt(asset, Number(scale), { kind: "pose", pose: asset.presentation.defaultPose }).frames;
const rows: [string, number, number][] = [];
for (const s of asset.volumes[scale!][list].filter((q: any) => q.bone !== "wall" && !(q.of ?? "").startsWith("wall"))) {
  let lo = Infinity, hi = -Infinity;
  for (const m of meshesIn(asset, frames, [s])) for (let i = k; i < m.world.length; i += 3) { lo = Math.min(lo, m.world[i]!); hi = Math.max(hi, m.world[i]!); }
  rows.push([s.id, lo, hi]);
}
rows.sort((a, b) => a[1] - b[1]);
for (const r of [...rows.slice(0, 4), ...rows.sort((a, b) => b[2] - a[2]).slice(0, 4)]) console.log(r[0].padEnd(20), r[1].toFixed(4), r[2].toFixed(4));
