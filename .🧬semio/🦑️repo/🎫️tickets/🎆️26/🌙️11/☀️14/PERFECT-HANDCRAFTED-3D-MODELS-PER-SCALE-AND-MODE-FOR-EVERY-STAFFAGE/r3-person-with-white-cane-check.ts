/** ✅️ Runs the product volume rules and `validateAsset` on the candidate asset of `humans/person-with-white-cane` (before the catalog file is written); prints one line per rule and scale with the failing measurements, `--all` prints every measurement.
 * Usage: `bun r3-person-with-white-cane-check.ts [--scale N] [--all] [--areas]`
 * @see ./volumes-check.ts */
import { checkVolumes, measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { type Json } from "./volumes-lib.ts";
import { candidate } from "./r3-person-with-white-cane-io.ts";

const args = process.argv.slice(2);
const m = candidate();
const scale = args.includes("--scale") ? Number(args[args.indexOf("--scale") + 1]) : undefined;
const options = scale === undefined ? {} : { scale };
const ms = measureVolumes(m as never, options as never) as readonly Json[];
const groups = new Map<string, Json[]>();
for (const x of ms) (groups.get(`${x.rule} 1:${x.scale ?? "-"}`) ?? groups.set(`${x.rule} 1:${x.scale ?? "-"}`, []).get(`${x.rule} 1:${x.scale ?? "-"}`)!).push(x);
let fails = 0;
for (const [key, items] of groups) {
  const bad = items.filter((x) => x.ok === false);
  fails += bad.length;
  const show = args.includes("--all") ? items : bad;
  const areas = args.includes("--areas") && key.includes("thermal") ? items.filter((x) => /outerAreaM2|areaDeviation/.test(x.name)) : [];
  process.stdout.write(`${bad.length === 0 ? "ok  " : "FAIL"} ${key.padEnd(34)} ${[...show, ...areas].map((x) => `${x.name}=${Number(x.value).toFixed(4)}${x.limit === undefined || !Number.isFinite(x.limit) ? "" : `${x.bound === "min" ? ">=" : "<="}${Number(x.limit).toFixed(4)}`}`).join(" ")}\n`);
}
const findings = checkVolumes(m as never, options as never) as readonly Json[];
for (const f of findings) process.stdout.write(`FINDING ${f.rule}${f.scale === undefined ? "" : ` 1:${f.scale}`} ${f.code} ${f.path}: ${f.message.en}\n`);
process.stdout.write(`${fails === 0 && findings.length === 0 ? "GREEN" : `RED: ${fails} measurements, ${findings.length} findings`}\n`);
