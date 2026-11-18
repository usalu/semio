/** 📍 Volume solids and drawn shapes near a point of one view: bun vegetation-at.ts <asset> <scale> <view> <x> <y> <r> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { boundsOf, drawnFor, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, xs, ys, rs] = process.argv.slice(2) as string[];
const [x, y, r] = [Number(xs), Number(ys), Number(rs)];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } }) as any;
const solids = worldSolids(asset, state.bones as never, Number(scaleArg)).filter((s) => s.texture !== true);
for (const s of solids) {
  const g = referenceOf(view as never, [s], undefined, drawnFor(asset, state, view as never, Number(scaleArg))).cut;
  const b = g.reduce((a, q) => { const c = boundsOf(q); return [Math.min(a[0], c[0]), Math.max(a[1], c[1]), Math.min(a[2], c[2]), Math.max(a[3], c[3])]; }, [Infinity, -Infinity, Infinity, -Infinity]);
  if (b[0] - r <= x && x <= b[1] + r && b[2] - r <= y && y <= b[3] + r) console.log("SOLID", s.id, s.bone, s.slender === true ? "slender" : "", b.map((v) => +v.toFixed(3)));
}
for (const s of state.shapes[view] as any[]) {
  const pts = s.rings.flat();
  if (pts.length === 0) continue;
  const bx = [Math.min(...pts.map((p: any) => p.x)), Math.max(...pts.map((p: any) => p.x)), Math.min(...pts.map((p: any) => p.y)), Math.max(...pts.map((p: any) => p.y))];
  const inside = s.rings.reduce((n: number, ring: any[]) => { let c = false; for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) { const [a, b] = [ring[k], ring[i]]; if ((a.y <= y) !== (b.y <= y) && x < a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x)) c = !c; } return n + (c ? 1 : 0); }, 0) % 2 === 1;
  if (bx[0] - r <= x && x <= bx[1] + r && bx[2] - r <= y && y <= bx[3] + r) console.log(inside ? "SHAPE*" : "SHAPE", s.shape.id, s.shape.bone, s.shape.role, s.shape.kind, s.shape.filled, bx.map((v) => +v.toFixed(3)));
}
