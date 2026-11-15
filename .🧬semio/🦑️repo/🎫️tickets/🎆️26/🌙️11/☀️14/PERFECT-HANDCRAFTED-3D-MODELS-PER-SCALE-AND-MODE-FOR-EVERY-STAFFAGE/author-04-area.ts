import { entryOf, loadSource, merged, scaleMeshes, meshArea, isAuthored } from "./volumes-lib.ts";

const spec = process.argv[2]!;
const { entry, asset } = entryOf(spec);
const source = loadSource(entry)!;
const a = merged(asset, source);
const out: Record<string, Record<string, number>> = {};
for (const sc of [50, 100, 200, 500]) {
  const v = a.volumes[String(sc)];
  if (!isAuthored(v)) continue;
  const sm = scaleMeshes(a, sc, v);
  for (const r of a.thermal?.regions ?? []) {
    const layers: string[] = (r.layers ?? []).map((l: any) => l.id);
    const items = sm.solids.thermal.map((s: any, i: number) => ({ s, m: sm.thermal[i]! })).filter((x: any) => x.s.region === r.id);
    if (items.length === 0) continue;
    const top = Math.max(...items.map((x: any) => (x.s.layer === undefined ? -1 : layers.indexOf(x.s.layer))));
    const area = items.filter((x: any) => (x.s.layer === undefined ? -1 : layers.indexOf(x.s.layer)) === top).reduce((n: number, x: any) => n + meshArea(x.m), 0);
    (out[r.id] ??= { areaM2: r.areaM2 })[String(sc)] = Number(area.toFixed(4));
  }
}
console.log(JSON.stringify(out));
