/** 🔎 Round 3 dairy cow: prints the bone frames of the default pose (joint, direction, tip) and the bounding box of every scale's body.
 *
 * Usage: bun r3-dairy-cow-probe.ts */
import { entryOf, loadSource, merged, meshesAt, poseState, type Json } from "./volumes-lib.ts";

const { entry, asset: catalog } = entryOf("animals/dairy-cow");
const asset = merged(catalog, loadSource(entry)!);
const frames = poseState(asset, 50).frames as Record<string, { o: number[]; u: number[]; v: number[] }>;
for (const b of asset.rig.bones as Json[]) {
  const f = frames[b.id]!;
  const tip = [f.o[0]! + f.u[0]! * b.lengthM, f.o[1]! + f.u[1]! * b.lengthM];
  console.log(b.id.padEnd(15), "joint", f.o.slice(0, 2).map((c) => c.toFixed(3)).join(","), "deg", ((Math.atan2(f.u[1]!, f.u[0]!) * 180) / Math.PI).toFixed(1), "tip", tip.map((c) => c.toFixed(3)).join(","));
}
for (const scale of [50, 100, 200, 500]) {
  const body: Json[] = asset.volumes[String(scale)].body;
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const m of meshesAt(asset, scale, body)) for (let i = 0; i < m.world.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, m.world[i]!); hi[i % 3] = Math.max(hi[i % 3]!, m.world[i]!); }
  console.log(`1:${scale} box`, lo.map((c) => c.toFixed(3)).join(","), "to", hi.map((c) => c.toFixed(3)).join(","));
}
