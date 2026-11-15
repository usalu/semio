import { readFileSync } from "node:fs";
import { entryOf, merged, scaleMeshes, extentOf } from "./volumes-lib.ts";
const a = process.argv.slice(2);
const f = (n: string) => (a.indexOf(n) < 0 ? undefined : a[a.indexOf(n) + 1]);
const { asset } = entryOf(a[0]!);
const src = JSON.parse(readFileSync(f("--source")!, "utf8"));
const m = merged(asset, src);
const scale = Number(f("--scale"));
const sm = scaleMeshes(m, scale, src.volumes[String(scale)]);
const e = extentOf(sm.body)!;
console.log("body", e.min.map((x: number) => x.toFixed(3)), e.max.map((x: number) => x.toFixed(3)));
sm.thermal.forEach((mesh: any, i: number) => {
  const x = extentOf([mesh])!;
  const out = Math.max(...[0, 1, 2].map((k) => Math.max(e.min[k]! - x.min[k]!, x.max[k]! - e.max[k]!, 0)));
  if (out > 0.02) console.log(sm.solids.thermal[i]!.id, out.toFixed(3), x.min.map((v: number) => v.toFixed(3)), x.max.map((v: number) => v.toFixed(3)));
});
