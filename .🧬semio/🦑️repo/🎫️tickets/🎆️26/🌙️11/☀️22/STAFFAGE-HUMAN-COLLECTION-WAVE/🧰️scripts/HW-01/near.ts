/** 📍️ The drawn shapes of one view of the default pose whose outline passes within `r` of a point.  bun near.ts <asset.json> <scale> <view> <x> <y> [r] */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [path, sc, view, xs, ys, rs] = process.argv.slice(2) as string[];
const asset = (await Bun.file(path!).json()).asset;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } } as never);
const [x, y, r] = [Number(xs), Number(ys), Number(rs ?? 0.01)];
for (const s of (state.shapes as never as Record<string, { shape: { id: string; bone: string; role: string; filled?: boolean; z: number }; rings: { x: number; y: number }[][] }[]>)[view!]!) {
  let best = Infinity;
  for (const ring of s.rings) for (let i = 0; i < ring.length; i++) {
    const a = ring[i]!, b = ring[(i + 1) % ring.length]!;
    const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy;
    const t = l === 0 ? 0 : Math.max(0, Math.min(1, ((x - a.x) * dx + (y - a.y) * dy) / l));
    best = Math.min(best, Math.hypot(x - a.x - t * dx, y - a.y - t * dy));
  }
  if (best <= r) console.log(`${s.shape.id.padEnd(24)} ${s.shape.bone.padEnd(14)} ${s.shape.role} filled ${s.shape.filled} z ${s.shape.z} d ${(best * 1000).toFixed(1)} mm`);
}
