/** 🧊 Mismatch clusters of the default pose. bun spread-clu.ts <asset> <scale> <view> */
import { clustersOf, loadAsset } from "./humansa-lib.ts";
const [name, scale, view] = process.argv.slice(2);
const r = clustersOf(await loadAsset(name), Number(scale), view as any, 10);
console.log(`iou ${r.iou.toFixed(4)} h ${r.h.toFixed(4)}`);
for (const c of r.clusters) console.log(`  ${c.side} (${c.x}, ${c.y}) d ${c.d}`);
