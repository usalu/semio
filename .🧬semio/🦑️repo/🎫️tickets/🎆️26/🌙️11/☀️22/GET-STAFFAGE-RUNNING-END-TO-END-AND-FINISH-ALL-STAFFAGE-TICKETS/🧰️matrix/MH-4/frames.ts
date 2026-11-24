/** 🧭️ The rest frames of an asset that are not sagittal: `bun frames.ts <id|file.json>…` prints every bone whose lateral axis w leaves the world z by more than 8 degrees at rest (its u, v, w), since a profile in the (u, v) plane of such a bone is not the side elevation. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
for (const arg of process.argv.slice(2)) {
  const asset = readDoc(arg).doc.asset;
  const frames = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
  const odd = Object.entries(frames).filter(([, { frame }]) => Math.abs(frame.w[2]!) < 0.99);
  console.log(`${asset.id}: ${Object.keys(frames).length} bones, ${odd.length} not sagittal`);
  for (const [bone, { frame }] of odd) console.log(`   ${bone} u ${frame.u.map((x) => x.toFixed(2))} v ${frame.v.map((x) => x.toFixed(2))} w ${frame.w.map((x) => x.toFixed(2))}`);
}
