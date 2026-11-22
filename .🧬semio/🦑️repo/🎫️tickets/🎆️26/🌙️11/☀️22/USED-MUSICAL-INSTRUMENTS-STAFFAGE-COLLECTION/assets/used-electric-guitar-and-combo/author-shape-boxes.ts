/** 🔬️ Prints the paper bounding box (metres) of every evaluated shape of one view and scale of the candidate, for comparison with the solids (author debug aid). */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [file, scale, view, pose] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const state = evaluate(asset, { kind: "pose", pose: pose ?? asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: Number(scale) } } as never);
for (const s of (state.shapes as any)[view!]) {
  const pts = s.rings.flat();
  if (!pts.length) continue;
  const xs = pts.map((p: any) => p.x ?? p[0]), ys = pts.map((p: any) => p.y ?? p[1]);
  console.log(`[DEBUG] ${s.shape.id.padEnd(28)} x ${Math.min(...xs).toFixed(3)}..${Math.max(...xs).toFixed(3)} y ${Math.min(...ys).toFixed(3)}..${Math.max(...ys).toFixed(3)}`);
}
