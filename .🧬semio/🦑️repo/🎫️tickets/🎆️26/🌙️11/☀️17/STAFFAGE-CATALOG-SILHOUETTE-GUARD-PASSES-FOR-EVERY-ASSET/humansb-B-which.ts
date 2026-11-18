/** 🔎 Which volume solids put reference silhouette (as the guard builds it, slender members widened) over a point. bun humansb-B-which.ts <asset> <scale> <view> <x> <y> [r] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { drawnFor, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, xs, ys, rs] = process.argv.slice(2) as string[];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const scale = Number(sc);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
const drawn = drawnFor(asset, state, view as never, scale);
const [x, y, r] = [Number(xs), Number(ys), Number(rs ?? 0.02)];
for (const s of solids) {
  const ref = referenceOf(view as never, [s], undefined, drawn);
  const pts = [...ref.cut, ...ref.visible].flat(2) as { x: number; y: number }[];
  if (pts.length === 0) continue;
  const [x0, x1, y0, y1] = [Math.min(...pts.map((p) => p.x)), Math.max(...pts.map((p) => p.x)), Math.min(...pts.map((p) => p.y)), Math.max(...pts.map((p) => p.y))];
  if (x >= x0 - r && x <= x1 + r && y >= y0 - r && y <= y1 + r) console.log(s.id.padEnd(24), "slender", s.slender === true, `x ${x0.toFixed(3)} ${x1.toFixed(3)} y ${y0.toFixed(3)} ${y1.toFixed(3)}`);
}
