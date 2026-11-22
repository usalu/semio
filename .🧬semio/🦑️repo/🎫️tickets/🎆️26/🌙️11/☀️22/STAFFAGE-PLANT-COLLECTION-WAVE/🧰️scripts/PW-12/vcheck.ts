/** 🧊️ PW-12 volume probe: every volume rule (`checkVolumes`) of one catalog asset, with the IoU, p95 and area measurements, so the authored volumes can be tuned.
 *
 * `repo tools exec bun -- <this file> <asset id> [scale] [rule,rule]`
 *
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts */
import { loadCatalogFromFiles } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { checkVolumes, measureVolumes, VOLUME_RULES } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [id, scaleArg, rulesArg] = process.argv.slice(2);
const catalog = await loadCatalogFromFiles(undefined, undefined, new Set([id!]));
const asset = catalog.assets.find((a) => a.id === id)!;
const scale = scaleArg && scaleArg !== "all" ? (Number(scaleArg) as 50) : undefined;
const rules = (rulesArg ? rulesArg.split(",") : VOLUME_RULES) as never;
const options = { ...(scale === undefined ? {} : { scale }), rules };
const control = { progress: (done: number, total: number, label: { en: string }) => console.error(`[DEBUG] ${done}/${total} ${label.en}`) };
if (process.env.PW12_MEASURE === "1") {
  const measured = measureVolumes(asset, options, control as never);
  for (const m of measured) if (/^(iou|p95|p95Soft|areaDeviation|info:outerAreaM2|bboxDeviation|areaDeviationTotal|outsideBody)/.test(m.name) || !m.ok) console.log(`[DEBUG] ${m.ok ? "ok " : "BAD"} ${m.rule} 1:${m.scale} ${m.list ?? ""} ${m.name} ${m.value.toFixed(4)} limit ${m.limit === Number.MAX_VALUE ? "-" : m.limit.toFixed(4)}`);
}
const findings = checkVolumes(asset, options, control as never);
for (const f of findings) console.log(`[DEBUG] FINDING ${f.rule} ${f.path}: ${f.message.en}`);
console.log(`[DEBUG] ${findings.length} findings`);
