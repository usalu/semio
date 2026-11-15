import { entryOf, loadSource, merged, scaleMeshes, extentOf } from "./volumes-lib.ts";
const [spec, sc, list] = [process.argv[2]!, Number(process.argv[3] ?? 50), process.argv[4] ?? "thermal"];
const { entry, asset } = entryOf(spec);
const source = loadSource(entry)!;
const a = merged(asset, source);
const sm: any = scaleMeshes(a, sc, source.volumes[String(sc)]);
const e = extentOf(sm.body)!;
console.log("body", e.min.map((x: number) => x.toFixed(3)), e.max.map((x: number) => x.toFixed(3)));
(sm as any)[list].forEach((m: any, i: number) => {
  const b = extentOf([m])!;
  const over = [0, 1, 2].map((k) => Math.max(e.min[k]! - b.min[k]!, b.max[k]! - e.max[k]!));
  if (Math.max(...over) > 0.004) console.log(sm.solids[list][i].id, over.map((x: number) => x.toFixed(3)));
});
