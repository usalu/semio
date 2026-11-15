import { entryOf, loadSource, merged, scaleMeshes, extentOf, featureM } from "./volumes-lib.ts";
const [spec, scaleArg, lst = "thermal"] = process.argv.slice(2);
const scale = Number(scaleArg);
const { entry, asset: raw } = entryOf(spec!);
const asset: any = merged(raw, loadSource(entry)!);
const sm: any = scaleMeshes(asset, scale, asset.volumes[String(scale)]);
const be = extentOf(sm.body)!;
const f = featureM(scale, asset);
console.log("body", be.min.map((x: number) => +x.toFixed(3)), be.max.map((x: number) => +x.toFixed(3)));
sm[lst].forEach((m: any, i: number) => {
  const e = extentOf([m])!;
  const over = [0, 1, 2].map((k) => Math.max(be.min[k] - e.min[k], e.max[k] - be.max[k]));
  if (Math.max(...over) > f * 0.5) console.log(sm.solids[lst][i].id, over.map((x: number) => +(x * 1000).toFixed(1)), "mm  min", e.min.map((x: number) => +x.toFixed(3)), "max", e.max.map((x: number) => +x.toFixed(3)));
});
