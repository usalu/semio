import { entryOf, merged, loadSource } from "./volumes-lib.ts";
import { extentOf, meshes } from "./volumes-base.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const src = loadSource(entry)!;
const scale = process.argv[3] ?? "50";
const list = process.argv[4] ?? "structure";
const v = src.volumes[scale];
const solids = v[list].map((s: any) => (s.of === undefined ? s : v.body.find((b: any) => b.id === s.of)));
const ms = meshes(asset, solids, Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM));
solids.forEach((s: any, i: number) => {
  const e = extentOf([ms[i]!]);
  console.log(v[list][i].id.padEnd(34), e ? `x ${e.min[0]!.toFixed(4)}..${e.max[0]!.toFixed(4)}  y ${e.min[1]!.toFixed(4)}..${e.max[1]!.toFixed(4)}  z ${e.min[2]!.toFixed(4)}..${e.max[2]!.toFixed(4)}` : "empty");
});
