/** 🧊️ Runs the measured volume rules (`checkVolumes`) on the given human slugs and prints every finding.
 *  bun vcheck.ts <slug> [scale] [rule ...] */
import { join, resolve } from "node:path";
import { checkVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [slug, scaleArg, ...rules] = process.argv.slice(2);
const asset = (await Bun.file(join(catalog, slug!, "🔣️.json")).json()).asset;
const t0 = performance.now();
const options: Record<string, unknown> = {};
if (scaleArg && scaleArg !== "all") options.scale = Number(scaleArg);
if (rules.length) options.rules = rules;
const findings = checkVolumes(asset, options, { progress: (done: number, total: number, label: { en: string }) => process.stderr.write(`[DEBUG] ${done}/${total} ${label?.en ?? ""}\n`) } as never);
for (const f of findings) console.log(`${f.rule} 1:${f.scale ?? ""} ${f.list ?? ""}: ${f.message.en}`.slice(0, 600));
console.log(`${slug}: ${findings.length} findings in ${((performance.now() - t0) / 1000).toFixed(0)} s`);
