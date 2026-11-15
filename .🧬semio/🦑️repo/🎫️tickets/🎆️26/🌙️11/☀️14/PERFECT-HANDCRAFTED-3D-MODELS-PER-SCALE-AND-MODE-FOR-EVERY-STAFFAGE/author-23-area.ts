import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";

const spec = process.argv[2]!;
const scale = process.argv[3] === undefined ? undefined : Number(process.argv[3]);
const { entry, asset } = entryOf(spec);
const src = process.env.SRC === undefined ? loadSource(entry)! : JSON.parse(readFileSync(process.env.SRC, "utf8"));
const m = merged(asset, src) as any;
const orig: Record<string, number> = {};
for (const r of m.thermal.regions) { orig[r.id] = r.areaM2; r.areaM2 = 1e-9; }
const out: Record<string, any> = {};
for (const x of measureVolumes(m, { rules: ["volumes-thermal-coverage"], ...(scale === undefined ? {} : { scale }) } as never)) {
  if (!x.name.startsWith("areaDeviation:")) continue;
  const id = x.name.slice(14);
  (out[id] ??= { areaM2: orig[id] })[String(x.scale)] = Number(((x.value + 1) * 1e-9).toPrecision(4));
}
for (const [k, v] of Object.entries(out)) console.log(k.padEnd(22), JSON.stringify(v));
