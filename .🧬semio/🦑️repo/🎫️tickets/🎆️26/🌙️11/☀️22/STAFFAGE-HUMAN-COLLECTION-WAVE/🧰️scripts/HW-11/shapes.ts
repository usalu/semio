/** 🔎️ World bounding boxes of the shapes of a pose at a scale: bun shapes.ts <slug|file> <pose|-> <scale> <view> [idRegex] */
import { evaluate, type Asset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { readAsset } from "./lib.ts";
const [src, pose, scale, view, re] = process.argv.slice(2);
const asset = await readAsset(src!);
if (pose === "-") asset.poses.push({ ...structuredClone(asset.poses[0]), id: "rest", bones: {}, key: true, loads: [] });
const s = evaluate(asset as Asset, { kind: "pose", pose: pose === "-" ? "rest" : pose! }, { kind: "scale", scale: { denominator: Number(scale) as 50 } });
for (const e of (s.shapes as any)[view!]) {
  if (re && !new RegExp(re).test(e.shape.id)) continue;
  const pts = e.rings.flat();
  const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
  console.log(`${e.shape.id.padEnd(22)} ${e.shape.bone.padEnd(14)} x ${Math.min(...xs).toFixed(3)}..${Math.max(...xs).toFixed(3)} y ${Math.min(...ys).toFixed(3)}..${Math.max(...ys).toFixed(3)}`);
}
