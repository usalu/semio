/** 📦️ Prints the bounding box of the body list of `humans/jogger` at every scale in the default pose (world metres) and the outer thermal areas the model alone gives are left to `volumes-check.ts`.
 * Usage: `bun r3-jogger-bbox.ts` */
import { entryOf, loadSource, merged, scaleMeshes } from "./volumes-lib.ts";
const { entry, asset } = entryOf("humans/jogger");
const m = merged(asset, loadSource(entry)!);
for (const scale of [50, 100, 200, 500]) {
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const mesh of scaleMeshes(m, scale, m.volumes[String(scale)]).body) for (let i = 0; i < mesh.world.length; i++) [lo[i % 3], hi[i % 3]] = [Math.min(lo[i % 3]!, mesh.world[i]!), Math.max(hi[i % 3]!, mesh.world[i]!)];
  console.log(scale, "min", lo.map((x) => x.toFixed(4)).join(" "), "max", hi.map((x) => x.toFixed(4)).join(" "), "extent", hi.map((x, k) => (x - lo[k]!).toFixed(4)).join(" "));
}
