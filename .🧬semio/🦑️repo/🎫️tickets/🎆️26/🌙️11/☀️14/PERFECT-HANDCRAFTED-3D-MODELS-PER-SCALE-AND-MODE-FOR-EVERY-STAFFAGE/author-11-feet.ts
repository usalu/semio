import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const { entry, asset } = entryOf("appliances/electric-fan-heater");
const src = loadSource(entry)!;
for (const sc of ["50", "100"]) {
  const v = src.volumes[sc];
  const keepBody = v.body.filter((b: any) => b.id === "foot-back");
  const save = [v.body, v.thermal, v.structure];
  v.body = keepBody; v.thermal = v.thermal.filter((t: any) => t.id === "th-foot-back"); 
  const m = merged(asset, src);
  m.thermal = { ...m.thermal, regions: m.thermal.regions.map((r: any) => ({ ...r, areaM2: 1e6 })) };
  const ms = measureVolumes(m as never, { scale: Number(sc) as never, rules: ["volumes-thermal-coverage"] as never });
  console.log(sc, ms.filter((x: any) => x.name === "areaDeviation:feet").map((x: any) => (1 - x.value) * 1e6));
  [v.body, v.thermal, v.structure] = save;
}
