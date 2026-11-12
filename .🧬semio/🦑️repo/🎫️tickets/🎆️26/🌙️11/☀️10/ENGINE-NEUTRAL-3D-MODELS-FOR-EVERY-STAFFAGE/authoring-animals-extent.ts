import { loadAsset, resolveAsset, restState } from "./volume-lib.ts";
const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const st = restState(asset, 50);
for (const v of ["elevation", "front", "plan"] as const) {
  const pts = st[v].filter((e) => e.closed && ["body", "detail", "glazing"].includes(e.shape.role)).flatMap((e) => e.rings.flat());
  const r = (x: number) => Math.round(x * 1000) / 1000;
  console.log(v, `x ${r(Math.min(...pts.map((p) => p.x)))}..${r(Math.max(...pts.map((p) => p.x)))} (${r(Math.max(...pts.map((p) => p.x)) - Math.min(...pts.map((p) => p.x)))})  y ${r(Math.min(...pts.map((p) => p.y)))}..${r(Math.max(...pts.map((p) => p.y)))} (${r(Math.max(...pts.map((p) => p.y)) - Math.min(...pts.map((p) => p.y)))})`);
}
console.log(JSON.stringify(asset.dimensions));
