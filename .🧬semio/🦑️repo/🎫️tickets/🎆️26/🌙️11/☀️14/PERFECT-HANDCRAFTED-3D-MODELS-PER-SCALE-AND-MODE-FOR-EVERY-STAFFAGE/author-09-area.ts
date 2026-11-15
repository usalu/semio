import { entryOf, loadSource, merged, scaleMeshes, meshArea, extentOf, type Json } from "./volumes-lib.ts";

const [spec, scaleArg, list] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const source = loadSource(entry)!;
const m = merged(asset, source);
const scale = Number(scaleArg);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
const meshes = (sm as Json)[list ?? "body"] as Json[];
const solids = sm.solids[(list ?? "body") as "body"];
const rows: string[] = [];
const regionArea = new Map<string, number>();
meshes.forEach((mesh, i) => {
  const s = solids[i]!;
  const e = extentOf([mesh])!;
  const a = meshArea(mesh);
  const key = s.region === undefined ? "" : `${s.region}${s.layer === undefined ? "" : "/" + s.layer}`;
  if (key) regionArea.set(key, (regionArea.get(key) ?? 0) + a);
  rows.push(`${s.id.padEnd(28)} ${s.bone?.padEnd(12) ?? ""} area=${a.toFixed(4)} min=[${e.min.map((x) => x.toFixed(3))}] max=[${e.max.map((x) => x.toFixed(3))}] ${key}`);
});
process.stdout.write(rows.join("\n") + "\n");
for (const [k, v] of regionArea) process.stdout.write(`REGION ${k} ${v.toFixed(4)}\n`);
