import { entryOf, loadSource, meshesAt } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const src = loadSource(entry)!;
const scale = process.argv[3]!;
const v = src.volumes[scale];
for (const l of (process.argv[4] ?? "body,thermal,structure").split(",")) {
  const solids = v[l].map((s: any) => (s.of === undefined ? s : { ...v.body.find((b: any) => b.id === s.of), id: s.id }));
  const ms = meshesAt(asset, Number(scale), solids);
  console.log("--", l);
  solids.forEach((s: any, i: number) => console.log("  ", s.id.padEnd(30), s.kind.padEnd(10), ms[i]!.indices.length / 3));
}
