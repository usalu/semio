/** 🔬️ Evaluated shapes of one view with their world bbox: `bun silhouette-shapes.ts <slug> <view> [--match substr] [--pose id] [--scale 50]`. */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
const a = process.argv.slice(2);
const opt = (k: string) => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : undefined; };
const asset = loadAsset(catalogEntries().find((x) => x.slug === a[0])!.path);
const state = evaluate(asset, { kind: "pose", pose: opt("--pose") ?? asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: Number(opt("--scale") ?? 50) } } as never) as any;
const m = opt("--match");
for (const s of state.shapes[a[1]!] as any[]) {
  if (m && !s.shape.id.includes(m) && s.shape.bone !== m) continue;
  let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity;
  for (const r of s.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
  console.log(s.shape.id.padEnd(22), s.shape.role.padEnd(7), s.closed ? "closed" : "open  ", `x ${x0.toFixed(3)}..${x1.toFixed(3)}  y ${y0.toFixed(3)}..${y1.toFixed(3)}`, s.shape.bone);
}
