import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const scale = Number(process.argv[3] ?? 50);
const only = process.argv.slice(4);
const src = loadSource(entry)!;
const v = src.volumes[String(scale)];
const all = v.thermal;
const keep = process.env.KEEP?.split(",");
const probe = (list: any[], label: string) => {
  v.thermal = list;
  if (keep) { v.structure = v.structure.filter((s: any) => false) .concat([]); }
  const m = merged(asset, src);
  m.thermal = { ...m.thermal, regions: m.thermal.regions.map((r: any) => ({ ...r, areaM2: 1e6 })) };
  const ms = measureVolumes(m as never, { scale: scale as never, rules: ["volumes-thermal-coverage"] as never });
  for (const x of ms) if (x.name.startsWith("areaDeviation")) {
    const r = asset.thermal.regions.find((q: any) => "areaDeviation:" + q.id === x.name);
    if (list.some((s: any) => s.region === r.id)) console.log(label.padEnd(28), r.id.padEnd(12), ((1 - x.value) * 1e6).toFixed(4));
  }
};
for (const s of all) if (only.length === 0 || only.includes(s.region)) probe([s], s.id);
