/** 🔬️ Rest frames and rest-world coordinates: `bun silhouette-rest.ts <slug> <bone> [u,v,w ...]` prints the rest frame of the bone and the rest-world (x ahead, y up, z left) of each local point. */
import { catalogEntries, frames, loadAsset, toWorld } from "./volume-lib.ts";
const a = process.argv.slice(2);
const asset = loadAsset(catalogEntries().find((x) => x.slug === a[0])!.path);
const f = frames(asset)[a[1]!]!;
console.log("o", f.o.map((x) => x.toFixed(4)).join(","), "u", f.u.map((x) => x.toFixed(4)).join(","), "v", f.v.map((x) => x.toFixed(4)).join(","), "w", f.w.map((x) => x.toFixed(4)).join(","));
for (const p of a.slice(2)) console.log(p, "->", toWorld(f, p.split(",").map(Number) as never).map((x) => x.toFixed(4)).join(","));
