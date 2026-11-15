/** 📏️ Prints the mesh area of every thermal solid and the target areaM2 per region, per scale. Usage: bun author-14-area.ts <category>/<slug> [--body] */
import { entryOf, loadSource, merged, meshArea, scaleMeshes } from "./volumes-lib.ts";

const spec = process.argv[2]!;
const { entry, asset } = entryOf(spec);
const source = loadSource(entry)!;
const m = merged(asset, source);
const out: any = {};
for (const d of [50, 100, 200, 500]) {
  const v = m.volumes[String(d)];
  if (v?.status !== "authored" || v.body.length === 0) continue;
  const sm = scaleMeshes(m, d, v);
  const regions: any = {};
  sm.solids.thermal.forEach((s: any, i: number) => {
    const r = (regions[s.region] ??= { target: (asset.thermal.regions.find((x: any) => x.id === s.region) ?? {}).areaM2, layers: (asset.thermal.regions.find((x: any) => x.id === s.region) ?? {}).layers?.map((l: any) => l.id) ?? [], solids: {} });
    r.solids[s.id] = { layer: s.layer ?? null, area: +meshArea(sm.thermal[i]!).toFixed(4) };
  });
  const body: any = {};
  if (process.argv.includes("--body")) sm.solids.body.forEach((s: any, i: number) => (body[s.id] = +meshArea(sm.body[i]!).toFixed(4)));
  out[d] = { regions, body };
}
process.stdout.write(JSON.stringify(out));
