/** 🔲️ Bounds of the evaluated shapes of one view and scale in a pose, filtered by id substring: bun shapebounds.ts <file> <pose> <scale> <view> [filter] */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, pose, scale, view, filter] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const s = evaluate(asset, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: Number(scale) } } as never);
for (const e of (s.shapes as any)[view!]) {
  if (filter && !e.shape.id.includes(filter) && e.shape.bone !== filter) continue;
  let [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity];
  for (const r of e.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
  console.log(`${e.shape.id.padEnd(22)} ${e.shape.bone.padEnd(14)} x ${x0.toFixed(3)}..${x1.toFixed(3)} y ${y0.toFixed(3)}..${y1.toFixed(3)} ${e.shape.solid?.kind ?? ""}`);
}
