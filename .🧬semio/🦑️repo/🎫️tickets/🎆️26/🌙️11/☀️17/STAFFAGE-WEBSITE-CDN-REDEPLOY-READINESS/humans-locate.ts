/** 🔍 Which drawn shapes and volume solids lie near a point of a view. bun humans-locate.ts <asset> <scale> <view> <x> <y> [radius] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { distanceToTriangle, project, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, xs, ys, rs] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string | undefined];
const scale = Number(sc), P = { x: Number(xs), y: Number(ys) }, R = Number(rs ?? 0.05);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const dseg = (a: any, b: any) => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; let t = l === 0 ? 0 : ((P.x - a.x) * dx + (P.y - a.y) * dy) / l; t = Math.max(0, Math.min(1, t)); return Math.hypot(P.x - a.x - t * dx, P.y - a.y - t * dy); };
console.log("DRAWN");
for (const s of (state.shapes as any)[view]) {
  let d = Infinity;
  for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) d = Math.min(d, dseg(ring[i], ring[(i + 1) % ring.length]));
  if (d < R) console.log(`  ${s.shape.id} role=${s.shape.role} bone=${s.shape.bone} closed=${s.closed} edgeDist=${d.toFixed(3)}`);
}
console.log("VOLUME");
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
for (const w of solids) {
  let d = Infinity;
  for (let k = 0; k < w.indices.length; k += 3) {
    const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [any, any, any];
    d = Math.min(d, distanceToTriangle(P, t));
  }
  if (d < R) console.log(`  ${w.id} bone=${w.bone} dist=${d.toFixed(3)}`);
}
