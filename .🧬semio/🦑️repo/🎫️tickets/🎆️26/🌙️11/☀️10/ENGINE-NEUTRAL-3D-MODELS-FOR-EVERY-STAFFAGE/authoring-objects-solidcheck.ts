import { loadAsset, resolveAsset, meshes } from "./volume-lib.ts";
const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const d = asset.dimensions;
const list = meshes(asset, asset.volume.solids, Math.max(d.widthM, d.heightM, d.depthM));
for (const m of list) {
  let vol = 0;
  const min = [1e9, 1e9, 1e9], max = [-1e9, -1e9, -1e9];
  const w = m.world, ix = m.indices;
  for (let k = 0; k < ix.length; k += 3) {
    const a = ix[k]! * 3, b = ix[k + 1]! * 3, c = ix[k + 2]! * 3;
    vol += (w[a]! * (w[b + 1]! * w[c + 2]! - w[b + 2]! * w[c + 1]!) - w[a + 1]! * (w[b]! * w[c + 2]! - w[b + 2]! * w[c]!) + w[a + 2]! * (w[b]! * w[c + 1]! - w[b + 1]! * w[c]!)) / 6;
  }
  for (let i = 0; i < w.length; i += 3) for (let k = 0; k < 3; k++) { min[k] = Math.min(min[k]!, w[i + k]!); max[k] = Math.max(max[k]!, w[i + k]!); }
  const f = (a: number[]) => a.map((x) => x.toFixed(3)).join(",");
  console.log(`${vol < 0 ? "INVERTED " : ""}${m.id.padEnd(24)} vol ${(vol * 1e6).toFixed(0).padStart(9)} cm3  [${f(min)}] .. [${f(max)}]`);
}
