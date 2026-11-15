/** 🔪️ Vertical slices of the drawn shapes of one bone in one view: bun author-69-slice.ts <asset> <scale> <view> <bone> x1 x2 ... (prints the y-intervals inside the shape at each x) */
import { entryOf, loadSource, merged, poseState } from "./volumes-lib.ts";
const [spec, sc, view, bone, ...xs] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const st = poseState(m, Number(sc)) as any;
for (const x of xs.map(Number)) {
  const ys: number[] = [];
  for (const s of st[view!] as any[]) {
    if (s.shape.bone !== bone) continue;
    for (const ring of s.rings) for (let i = 0; i < ring.length; i++) {
      const a = ring[i], b = ring[(i + 1) % ring.length];
      if ((a.x <= x && b.x > x) || (b.x <= x && a.x > x)) ys.push(a.y + ((x - a.x) / (b.x - a.x)) * (b.y - a.y));
    }
  }
  ys.sort((p, q) => p - q);
  const pairs: string[] = [];
  for (let i = 0; i + 1 < ys.length; i += 2) pairs.push(`${ys[i]!.toFixed(3)}..${ys[i + 1]!.toFixed(3)}`);
  console.log(`x=${x}: ${pairs.join(" ")}`);
}
