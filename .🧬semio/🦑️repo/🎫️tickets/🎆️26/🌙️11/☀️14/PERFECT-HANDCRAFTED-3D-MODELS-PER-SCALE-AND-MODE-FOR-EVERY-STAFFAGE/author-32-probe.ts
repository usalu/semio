/** 🔬️ Prints every measurement (with note) of given rules for one scale, product engine. Usage: bun author-32-probe.ts cat/slug --scale N [--source f] [--rules a,b] [--all] */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { entryOf, loadSource, merged, sourcePath } from "./volumes-lib.ts";
const a = process.argv.slice(2);
const g = (n: string) => (a.includes(n) ? a[a.indexOf(n) + 1] : undefined);
const spec = a[0]!;
const scale = Number(g("--scale"));
const { entry, asset } = entryOf(spec);
const src = g("--source") ? JSON.parse(readFileSync(g("--source")!, "utf8")) : loadSource(entry);
const m = merged(asset, src);
if (a.includes("--per-solid")) {
  const vol = m.volumes[String(scale)];
  const base = m.thermal.regions[0];
  const ids = new Set<string>();
  vol.thermal = vol.thermal.map((s: any) => ({ ...s, region: s.id, layer: undefined }));
  m.thermal = { ...m.thermal, regions: vol.thermal.map((s: any) => ({ ...base, id: s.id, areaM2: 1, layers: [], hidden: undefined })) };
  void ids;
}
const rules = (g("--rules") ?? "volumes-thermal-coverage,volumes-structure-coverage").split(",");
const out = measureVolumes(m as never, { scale: scale as never, rules: rules as never });
for (const x of out as any[]) {
  if (!a.includes("--all") && x.ok && !x.name.startsWith("areaDeviation") && !x.name.startsWith("outside")) continue;
  console.log(`${x.ok ? "ok  " : "FAIL"} ${x.rule} ${x.list ?? ""} ${x.name} = ${typeof x.value === "number" ? x.value.toFixed(4) : x.value} (limit ${x.limit})  ${x.note ?? ""}`);
}
if (a.includes("--owned")) {
  const { rulingMeasurements } = await import("./volumes-rulings.ts");
  const o = rulingMeasurements(m, { volumes: m.volumes }, { scale, rules });
  for (const x of o as any[]) if (x.name.startsWith("areaDeviation:") || x.name.startsWith("outside") || x.name.startsWith("coverage")) console.log(`[owned] ${x.name} = ${typeof x.value === "number" ? x.value.toFixed(3) : x.value}  ${x.note ?? ""}`);
}
