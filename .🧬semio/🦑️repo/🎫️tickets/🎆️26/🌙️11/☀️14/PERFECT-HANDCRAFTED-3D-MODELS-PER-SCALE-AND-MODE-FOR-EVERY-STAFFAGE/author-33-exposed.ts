/** 🔬️ Exposed area per thermal solid: bun author-33-exposed.ts <asset> <scale> [--source file]. One pseudo-region per outermost thermal solid. */
import { readFileSync } from "node:fs";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
import { rulingMeasurements } from "./volumes-rulings.ts";
const a0 = process.argv.slice(2);
const spec = a0[0]!;
const scale = Number(a0[1]);
const si = a0.indexOf("--source");
const { entry, asset } = entryOf(spec);
const source = si >= 0 ? JSON.parse(readFileSync(a0[si + 1]!, "utf8")) : loadSource(entry)!;
const m = merged(asset, source);
const vol = m.volumes[String(scale)];
const regs: any[] = [];
const thermal = vol.thermal.map((s: any, i: number) => {
  const r = m.thermal.regions.find((x: any) => x.id === s.region);
  const rid = `p${i}`;
  regs.push({ ...r, id: rid, layers: [], areaM2: 1000 });
  const { layer, ...rest } = s;
  void layer;
  return { ...rest, region: rid };
});
const probe = { ...m, thermal: { ...m.thermal, regions: regs }, volumes: { ...m.volumes, [String(scale)]: { ...vol, thermal } } };
const ms = rulingMeasurements(probe, { volumes: probe.volumes }, { scale, rules: ["volumes-thermal-coverage"] });
const rows: string[] = [];
for (const x of ms) {
  if (!x.name.startsWith("areaDeviation:p")) continue;
  const i = Number(x.name.slice(15));
  const e = /exposed ([\d.]+) m2/.exec(x.note ?? "");
  rows.push(`${vol.thermal[i].region.padEnd(16)} ${(vol.thermal[i].layer ?? "").padEnd(10)} ${String(vol.thermal[i].id).padEnd(34)} ${e ? e[1] : "?"}`);
}
console.log(rows.sort().join("\n"));
