import { entryOf, loadSource, merged, scaleMeshes, meshArea } from "./volumes-lib.ts";
const [spec, only] = process.argv.slice(2);
const { entry, asset: raw } = entryOf(spec!);
const asset: any = merged(raw, loadSource(entry)!);
const out: any = {};
for (const scale of [50, 100, 200, 500]) {
  const v = asset.volumes[String(scale)];
  if (v?.status !== "authored" || (only && String(scale) !== only)) continue;
  const sm = scaleMeshes(asset, scale, v);
  const byRegion = new Map<string, any[]>();
  sm.solids.thermal.forEach((s: any, i: number) => { if (sm.thermal[i]) (byRegion.get(s.region) ?? byRegion.set(s.region, []).get(s.region)!).push({ s, m: sm.thermal[i] }); });
  out[scale] = {};
  for (const r of asset.thermal.regions) {
    const items = byRegion.get(r.id) ?? [];
    const layers: string[] = (r.layers ?? []).map((l: any) => l.id);
    const idx = (x: any) => (x.s.layer === undefined ? -1 : layers.indexOf(x.s.layer));
    const top = items.length ? Math.max(...items.map(idx)) : -1;
    const area = items.filter((x) => idx(x) === top).reduce((n, x) => n + meshArea(x.m), 0);
    out[scale][r.id] = { area: +area.toFixed(4), target: r.areaM2 ?? null, n: items.length };
  }
}
console.log(JSON.stringify(out));
