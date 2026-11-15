import { entryOf, loadSource, merged, scaleMeshes } from "./volumes-lib.ts";
const [spec, sc] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const src = loadSource(entry)!;
const m = merged(asset, src);
const vol = m.volumes[sc!];
const sm = scaleMeshes(m, Number(sc), vol);
for (const l of ["body", "thermal", "structure"] as const) {
  console.log(l, sm.solids[l].map((s: any, i: number) => `${s.id}:${s.kind ?? "ref"}x${s.repeat?.count ?? 1}=${(sm as any)[l][i]?.indices.length / 3}`).join("  "));
}
