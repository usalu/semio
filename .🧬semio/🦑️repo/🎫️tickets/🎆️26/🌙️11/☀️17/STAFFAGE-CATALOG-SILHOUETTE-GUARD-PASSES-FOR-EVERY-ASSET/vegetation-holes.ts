/** 🕳️ The enclosed holes that the guard fills (at most (4 features)^2) in the drawn and volume masks of one view, as components with their bounding box in view metres: bun vegetation-holes.ts <asset> <scale> <view> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view] = process.argv.slice(2) as string[];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
const drawn = new Mask(m.box, m.cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
const vol = new Mask(m.box, m.cell);
for (const g of m.reference) vol.fill(g, true);
vol.seal();
const [w, h] = [vol.width, vol.height];
const holes = Math.round((4 * feature / m.cell) ** 2);
for (const [label, mask] of [["drawn", drawn], ["volume", vol]] as const) {
  const filled = fillSmallHoles(mask.cells, w, h, holes);
  const seen = new Uint8Array(w * h);
  const comps: string[] = [];
  for (let k = 0; k < w * h; k++) {
    if (!filled[k] || mask.cells[k] || seen[k]) continue;
    const stack = [k];
    seen[k] = 1;
    let [n, x0, x1, y0, y1] = [0, Infinity, -Infinity, Infinity, -Infinity];
    while (stack.length > 0) {
      const c = stack.pop()!;
      const [i, j] = [c % w, Math.floor(c / w)];
      n++;
      x0 = Math.min(x0, i); x1 = Math.max(x1, i); y0 = Math.min(y0, j); y1 = Math.max(y1, j);
      for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) {
        const [a, b] = [i + di, j + dj];
        if (a < 0 || b < 0 || a >= w || b >= h) continue;
        const q = b * w + a;
        if (filled[q] && !mask.cells[q] && !seen[q]) { seen[q] = 1; stack.push(q); }
      }
    }
    comps.push(`cells ${n} x ${(m.box.minX + x0 * m.cell).toFixed(3)}..${(m.box.minX + (x1 + 1) * m.cell).toFixed(3)} y ${(m.box.minY + y0 * m.cell).toFixed(3)}..${(m.box.minY + (y1 + 1) * m.cell).toFixed(3)}`);
  }
  console.log(label, "filled holes", comps.length);
  for (const c of comps.slice(0, 40)) console.log("  ", c);
}
