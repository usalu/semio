/** 🔬️ Silhouette of one asset at one scale with parts removed (shapes and solids of bones matching a regex). */
import { loadDoc, dropShapes, dropSolids, measureAsset, featureOf } from "./lib.ts";
const [slug, scale, re] = process.argv.slice(2);
const a = loadDoc(slug!).asset;
if (re) { const rx = new RegExp(re); dropShapes(a, (s) => rx.test(s.bone)); dropSolids(a, (s) => rx.test(s.bone ?? "") || rx.test(s.member ?? "")); }
const m = measureAsset(a, Number(scale), undefined, undefined, featureOf(Number(scale) as never));
for (const v of ["elevation", "front", "plan"] as const) console.log(v, m.views[v].iou.toFixed(3), (m.views[v].hausdorffM * 1000).toFixed(0));
