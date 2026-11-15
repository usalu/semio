import { entryOf, loadSource, meshesAt } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const src = loadSource(entry)!;
for (const scale of (process.argv[3] ?? "50,100,200").split(",")) {
  const v = src.volumes[scale];
  const out: string[] = [];
  for (const l of ["body", "thermal", "structure"]) {
    const solids = v[l].map((s: any) => (s.of === undefined ? s : { ...v.body.find((b: any) => b.id === s.of), id: s.id }));
    const ms = meshesAt(asset, Number(scale), solids);
    out.push(`${l} ${solids.length} solids ${ms.reduce((n, m) => n + m.indices.length / 3, 0)} tri`);
  }
  console.log("1:" + scale, out.join(" | "));
}
