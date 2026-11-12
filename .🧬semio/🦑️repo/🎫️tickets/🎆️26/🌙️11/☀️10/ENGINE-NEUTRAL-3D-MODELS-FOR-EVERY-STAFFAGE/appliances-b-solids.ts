import { readFileSync } from "node:fs";
import { loadAsset, meshes, resolveAsset } from "./volume-lib.ts";
const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const vol = process.argv[3] ? JSON.parse(readFileSync(process.argv[3], "utf8")).volume : asset.volume;
const maxDim = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
for (const m of meshes(asset, vol.solids, maxDim)) {
  const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
  for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, m.world[i + k]!); mx[k] = Math.max(mx[k]!, m.world[i + k]!); }
  console.log(m.id.padEnd(24), mn.map((x) => x.toFixed(3)).join(" "), "..", mx.map((x) => x.toFixed(3)).join(" "));
}
