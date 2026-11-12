import { readFileSync } from "node:fs";
import { loadAsset, resolveAsset, meshes } from "./volume-lib.ts";
const slug = process.argv[2]!;
const file = process.argv[3];
const asset = loadAsset(resolveAsset(slug).path);
const vol = file ? JSON.parse(readFileSync(file, "utf8")).volume : asset.volume;
const maxDim = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
const list = meshes(asset, vol.solids, maxDim);
const rows = list.map((m) => {
  const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
  for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, m.world[i + k]!); mx[k] = Math.max(mx[k]!, m.world[i + k]!); }
  return { id: m.id, mn, mx };
});
const f = (a: number[]) => a.map((x) => x.toFixed(3)).join(",");
for (const [k, name] of [[0, "x"], [1, "y"], [2, "z"]] as const) {
  const hi = [...rows].sort((a, b) => b.mx[k]! - a.mx[k]!).slice(0, 3);
  const lo = [...rows].sort((a, b) => a.mn[k]! - b.mn[k]!).slice(0, 3);
  console.log(`${name} max: ` + hi.map((r) => `${r.id} ${r.mx[k]!.toFixed(3)}`).join(" | ") + `   min: ` + lo.map((r) => `${r.id} ${r.mn[k]!.toFixed(3)}`).join(" | "));
}
