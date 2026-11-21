/** 📦️ World bounding boxes of the body solids of a candidate document in the default pose (debug aid for the round 3 sleeping person): `bun r3-sleeping-person-bbox.ts <document> <scale> [id regex] [--list body|thermal|structure]`.
 * @see ./r3-lib.ts */
import { readFileSync } from "node:fs";
import { meshesIn, rigAt } from "./r3-lib.ts";
const [file, scale, re] = process.argv.slice(2);
const li = process.argv.indexOf("--list");
const list = li < 0 ? "body" : process.argv[li + 1]!;
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const pi = process.argv.indexOf("--pose");
const frames = rigAt(asset, Number(scale), { kind: "pose", pose: pi < 0 ? asset.presentation.defaultPose : process.argv[pi + 1]! }).frames;
const solids = (asset.volumes[scale!][list] as any[]).filter((s) => s.of === undefined && new RegExp(re ?? ".").test(s.id));
for (const m of meshesIn(asset, frames, solids)) {
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < m.world.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, m.world[i]!); hi[i % 3] = Math.max(hi[i % 3]!, m.world[i]!); }
  console.log((m as any).solid?.id ?? (m as any).id, "x", lo[0]!.toFixed(3), hi[0]!.toFixed(3), "y", lo[1]!.toFixed(3), hi[1]!.toFixed(3), "z", lo[2]!.toFixed(3), hi[2]!.toFixed(3));
}
