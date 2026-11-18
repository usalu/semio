/** 🧪 Rasterise one solid alone and test points: bun vegetation-solidrow.ts <asset> <scale> <view> <solid id> <y> <x0> <x1> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, id, ys, x0s, x1s] = process.argv.slice(2) as string[];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const f = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solid = worldSolids(asset, state.bones as never, scale).find((s) => s.id === id)!;
const groups = referenceOf(view as never, [solid], undefined, undefined).cut;
const [y, x0, x1] = [Number(ys), Number(x0s), Number(x1s)];
const m = new Mask({ minX: x0 - 0.2, minY: y - 0.2, maxX: x1 + 0.2, maxY: y + 0.2 }, f / 4);
for (const g of groups) m.fill(g, true);
m.seal();
let line = "";
let last = "";
let st = x0;
for (let x = x0; x <= x1; x += f / 4) { const c = m.has(x, y) ? "#" : "."; if (c !== last) { if (last) line += `${last}[${st.toFixed(4)}..${x.toFixed(4)}] `; last = c; st = x; } }
console.log(line + `${last}[${st.toFixed(4)}..${x1}]`);
