/** 🔦️ Author 28: product-metric blame: removing each body solid, delta of IoU per view. bun author-28-blame.ts <asset> <scale> */
import { entryOf, loadSource, merged, featureM } from "./volumes-lib.ts";
import { measureAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [spec, sc] = process.argv.slice(2);
const scale = Number(sc);
const { entry, asset: raw } = entryOf(spec!);
const asset: any = merged(raw, loadSource(entry)!);
const f = featureM(scale, asset);
const V = ["elevation", "front", "plan"];
const run = () => { const r: any = measureAsset(asset, scale, asset.presentation.defaultPose, undefined, f); return V.map((v) => r.views[v].iou); };
const base = run();
console.log("base", base.map((x) => x.toFixed(4)).join(" "));
const body = asset.volumes[sc!].body;
const rows: [string, number[]][] = [];
for (let i = 0; i < body.length; i++) {
  const s = body.splice(i, 1)[0];
  const r = run();
  body.splice(i, 0, s);
  rows.push([s.id, r.map((x, k) => x - base[k]!)]);
}
for (let k = 0; k < 3; k++) {
  rows.sort((a, b) => b[1][k]! - a[1][k]!);
  console.log(V[k], "removing improves:", rows.slice(0, 7).map((r) => `${r[0]} ${r[1][k]!.toFixed(4)}`).join(", "));
}
