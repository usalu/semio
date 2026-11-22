/** 🩺️ Checks one PW-13 asset file: the catalog validation and the product volume rules (`checkVolumes`, `measureVolumes`) with the measurements that fail and the thermal area infos.
 * Usage: repo tools exec bun -- check.ts <asset json> [--scale N] [--rules a,b] [--info]
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts */
import { readFileSync } from "node:fs";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { checkVolumes, measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const args = process.argv.slice(2);
const flag = (n: string) => (args.includes(n) ? args[args.indexOf(n) + 1] : undefined);
const asset = JSON.parse(readFileSync(args[0]!, "utf8")).asset;
const issues = validateAsset(asset);
console.log(`[DEBUG] validate: ${issues.length} issue(s)`);
for (const i of issues.slice(0, 60)) console.log(`  ${i.code} ${i.path}: ${i.message.en}`);
if (args.includes("--no-volumes")) process.exit(issues.length ? 1 : 0);
const scale = flag("--scale") ? (Number(flag("--scale")) as 50) : undefined;
const rules = flag("--rules")?.split(",") as never;
const t0 = Date.now();
const found = checkVolumes(asset, { ...(scale ? { scale } : {}), ...(rules ? { rules } : {}) });
console.log(`[DEBUG] volumes: ${found.length} finding(s) (${((Date.now() - t0) / 1000).toFixed(1)} s)`);
for (const f of found) console.log(`  ! ${f.rule} ${f.path}: ${f.message.en.slice(0, 400)}${f.measurements.map((m) => ` [${m.name}=${m.value.toFixed(4)} ${m.bound} ${m.limit}]`).join("").slice(0, 400)} ${JSON.stringify(f.data ?? {}).slice(0, 600)}`);
if (args.includes("--info")) for (const m of measureVolumes(asset, { ...(scale ? { scale } : {}), ...(rules ? { rules } : {}) }).filter((m) => m.name.startsWith("info:") || m.rule === "volumes-silhouette" || m.rule === "volumes-bounds")) console.log(`  i ${m.rule} 1:${m.scale ?? "-"} ${m.list ?? ""} ${m.name} = ${m.value.toFixed(4)}${m.ok ? "" : " FAIL"}`);
process.exitCode = issues.length || found.length ? 1 : 0;
