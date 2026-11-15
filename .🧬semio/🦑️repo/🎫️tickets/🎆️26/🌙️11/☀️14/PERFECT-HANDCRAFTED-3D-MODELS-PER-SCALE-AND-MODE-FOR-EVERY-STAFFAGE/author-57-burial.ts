import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
// usage: bun author-57-burial.ts <asset> <scale> <region>  : exposed area of the region when each other body/structure/thermal solid is removed alone
const { entry, asset } = entryOf(process.argv[2]!);
const scale = Number(process.argv[3] ?? 50);
const region = process.argv[4]!;
const exposed = (src: any) => {
  const m = merged(asset, src);
  m.thermal = { ...m.thermal, regions: m.thermal.regions.map((r: any) => ({ ...r, areaM2: 1e6 })) };
  const ms = measureVolumes(m as never, { scale: scale as never, rules: ["volumes-thermal-coverage"] as never });
  const x = ms.find((q: any) => q.name === "areaDeviation:" + region);
  return x ? (1 - x.value) * 1e6 : NaN;
};
const base = loadSource(entry)!;
const b0 = exposed(base);
console.log("all", b0.toFixed(4));
for (const list of ["body", "structure", "thermal"]) {
  for (const s of base.volumes[String(scale)][list]) {
    const src = JSON.parse(JSON.stringify(base));
    src.volumes[String(scale)][list] = src.volumes[String(scale)][list].filter((q: any) => q.id !== s.id);
    if (list === "body") src.volumes[String(scale)].thermal = src.volumes[String(scale)].thermal.filter((q: any) => q.of !== s.id);
    const e = exposed(src);
    if (Math.abs(e - b0) > 2e-4) console.log(list.padEnd(10), s.id.padEnd(22), e.toFixed(4), "(+" + (e - b0).toFixed(4) + ")");
  }
}
