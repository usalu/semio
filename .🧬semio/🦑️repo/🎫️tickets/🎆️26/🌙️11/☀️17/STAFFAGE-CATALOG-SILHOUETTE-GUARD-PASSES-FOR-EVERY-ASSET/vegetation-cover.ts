/** 🎯 Which evaluated shapes cover a point after rasterisation (fills and pen strokes): bun vegetation-cover.ts <asset> <scale> <view> <x> <y> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, measureView, strokeQuad, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, xs, ys] = process.argv.slice(2) as string[];
const [scale, x, y] = [Number(scaleArg), Number(xs), Number(ys)];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
const rep = (asset.representations as any)[String(scale)];
const box = { minX: x - 0.05, minY: y - 0.05, maxX: x + 0.05, maxY: y + 0.05 };
for (const s of (state.shapes[view] as any[]).filter((q) => ["body", "detail", "glazing"].includes(q.shape.role) && q.rings.length > 0)) {
  const mk = new Mask(box, m.cell);
  if (s.closed && s.shape.filled !== false) mk.fill(s.rings);
  else {
    const pen = (((s.shape.penMm ?? rep.pens[s.shape.role]) ?? 0) * scale) / 1000;
    for (const ring of s.rings) {
      const r = s.closed ? [...ring, ring[0]] : ring;
      for (let i = 0; i + 1 < r.length; i++) mk.fill([strokeQuad(r[i], r[i + 1], Math.max(pen, 1e-4))]);
    }
  }
  if (mk.has(x, y)) console.log("COVERS", s.shape.id, s.shape.bone, s.shape.kind, "filled", s.shape.filled, "closed", s.closed);
}
