/** 📏 One row or column of the masks with metre coordinates: bun vegetation-row.ts <asset> <scale> <view> <row|col> <fixed> <from> <to> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, mode, fixed, from, to] = process.argv.slice(2) as string[];
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
let line = "";
let last = "";
let start = Number(from);
for (let t = Number(from); t <= Number(to); t += m.cell) {
  const [x, y] = mode === "row" ? [t, Number(fixed)] : [Number(fixed), t];
  const c = (drawn.has(x, y) ? 1 : 0) + (vol.has(x, y) ? 2 : 0);
  const ch = "." + "D" + "V" + "#";
  const s = ch[c]!;
  if (s !== last) {
    if (last !== "") line += `${last}[${start.toFixed(4)}..${t.toFixed(4)}] `;
    last = s;
    start = t;
  }
}
console.log(line + `${last}[${start.toFixed(4)}..${to}]`);
