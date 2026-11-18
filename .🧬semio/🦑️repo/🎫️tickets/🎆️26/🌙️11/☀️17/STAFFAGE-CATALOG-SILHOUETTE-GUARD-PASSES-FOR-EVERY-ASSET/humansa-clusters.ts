/** 🔬 Worst mismatch clusters of one asset, scale and view with the nearest drawn shapes and volume solids per cluster; env OPS applies an ops json in memory first, env ASCII="x,y,r" prints the D/V/# patch.
 *  bun humansa-clusters.ts <asset> <scale> [view|-] [max] [radius] */
import { existsSync, readFileSync } from "node:fs";
import { VIEWPOINTS, clustersOf, loadAsset, nearOf, patchOf, shapesOf, type Viewpoint } from "./humansa-lib.ts";
const [name, scaleArg, only, maxArg, radArg] = process.argv.slice(2);
const asset = await loadAsset(name!);
const scale = Number(scaleArg);
if (process.env.OPS && existsSync(process.env.OPS)) for (const o of JSON.parse(readFileSync(process.env.OPS, "utf8"))) {
  if (o.scale !== String(scale)) continue;
  const list: any[] = o.op.startsWith("shape") ? shapesOf(asset, scale, o.view) : (asset as any).volumes[o.scale].body;
  const find = (id: string): number => list.findIndex((x) => x.id === id);
  if (o.op.endsWith("-add")) list.splice(find(o.after) + 1, 0, o.shape ?? o.solid);
  else if (o.op.endsWith("-set")) Object.assign(list[find(o.id)], o.set);
  else list[find(o.id)] = o.shape ?? o.solid;
}
if (process.env.ASCII && only !== undefined && only !== "-") { const [x, y, r] = process.env.ASCII.split(",").map(Number); console.log(patchOf(asset, scale, only as Viewpoint, x!, y!, r!).join("\n")); process.exit(0); }
for (const view of VIEWPOINTS) {
  if (only !== undefined && only !== "-" && only !== view) continue;
  const r = clustersOf(asset, scale, view as Viewpoint, Number(maxArg ?? 6));
  console.log(`== ${view} 1:${scale} iou ${r.iou.toFixed(3)} h ${(r.h * 1000).toFixed(1)}mm excess ${r.excess.toFixed(2)}`);
  for (const c of r.clusters) {
    const n = nearOf(asset, scale, view as Viewpoint, c, Number(radArg ?? 0.03));
    console.log(`  ${c.side} (${c.x}, ${c.y}) d=${c.d}`);
    console.log(`    drawn: ${n.shapes.map((s) => `${s.id}${s.d.toFixed(3)}`).join(" ")}`);
    console.log(`    solids: ${n.solids.map((s) => `${s.id}${s.d.toFixed(3)}`).join(" ")}`);
  }
}
