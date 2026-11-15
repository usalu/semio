import { entryOf, loadSource, meshArea, meshesAt } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const src = loadSource(entry)!;
for (const scale of (process.argv[3] ?? "50,100,200").split(",")) {
  const v = src.volumes[scale];
  const solids = v.thermal.map((s: any) => (s.of === undefined ? s : { ...v.body.find((b: any) => b.id === s.of), id: s.id, region: s.region, layer: s.layer }));
  const ms = meshesAt(asset, Number(scale), solids);
  const sums: Record<string, number> = {};
  console.log("== 1:" + scale);
  solids.forEach((s: any, i: number) => {
    const a = meshArea(ms[i]!);
    const key = s.region + (s.layer ? "/" + s.layer : "");
    sums[key] = (sums[key] ?? 0) + a;
    console.log("  ", s.id.padEnd(28), key.padEnd(24), a.toFixed(4), "tri", ms[i]!.indices.length / 3);
  });
  console.log("   thermal tri total", solids.reduce((n: number, _s: any, i: number) => n + ms[i]!.indices.length / 3, 0));
  for (const r of asset.thermal.regions) {
    const got = Object.entries(sums).filter(([k]) => k.split("/")[0] === r.id).reduce((n, [, a]) => n + a, 0);
    console.log("   region", r.id.padEnd(16), "area", String(r.areaM2).padEnd(7), "got", got.toFixed(4), "dev", r.areaM2 ? ((got / r.areaM2 - 1) * 100).toFixed(0) + "%" : "");
  }
}
