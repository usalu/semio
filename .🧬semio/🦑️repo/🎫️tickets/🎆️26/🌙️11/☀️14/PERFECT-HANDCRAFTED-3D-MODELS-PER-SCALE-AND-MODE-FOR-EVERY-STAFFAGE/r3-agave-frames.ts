/** 🌵 Round 3, vegetation/agave: prints every bone frame (origin, tip, axes), azimuth and elevation of the bone in a pose or clip frame. Usage: bun r3-agave-frames.ts [pose] [--file doc.json] */
import { readFileSync } from "node:fs";
import { rigAt } from "./r3-lib.ts";
const args = process.argv.slice(2);
const fi = args.indexOf("--file");
const file = fi >= 0 ? args[fi + 1] : "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/agave/🔣️.json";
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const pose = args.find((a, i) => !a.startsWith("--") && args[i - 1] !== "--file") ?? asset.presentation.defaultPose;
const rig = rigAt(asset, 50, { kind: "pose", pose });
const f = (v: number[]) => v.map((x) => x.toFixed(3)).join(",");
for (const b of rig.bones) {
  const F = rig.frames[b.id]!;
  const az = (Math.atan2(F.u[2], F.u[0]) * 180) / Math.PI;
  const el = (Math.asin(F.u[1]) * 180) / Math.PI;
  console.log(`${b.id.padEnd(11)} o ${f(b.o)} tip ${f(b.tip)} az ${az.toFixed(1)} el ${el.toFixed(1)} v ${f(F.v)} w ${f(F.w)}`);
}
