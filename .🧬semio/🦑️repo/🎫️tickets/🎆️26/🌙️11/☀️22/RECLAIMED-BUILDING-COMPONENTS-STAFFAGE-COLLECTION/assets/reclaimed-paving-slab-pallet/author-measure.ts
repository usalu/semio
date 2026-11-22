/** 📐️ Prints the product's own volume measurements of this candidate whose name contains a filter (default `area`), for one scale and rule set: `$BUN author-measure.ts <scale> <rules,…> [filter]`.
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-validate.ts */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../../../..");
const { measureVolumes } = await import(join(root, "🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const [scale, rules, filter] = [Number(process.argv[2] ?? 50), (process.argv[3] ?? "thermal-coverage").split(",").map((rule) => (rule.startsWith("volumes-") ? rule : `volumes-${rule}`)), (process.argv[4] ?? "area").toLowerCase()];
for (const m of measureVolumes(asset, { scale, rules }) as readonly { readonly name: string; readonly value: number; readonly limit?: number; readonly ok?: boolean }[]) {
  if (m.name.toLowerCase().includes(filter)) console.log(`[DEBUG] ${m.ok === false ? "!" : " "} ${m.name} ${typeof m.value === "number" ? m.value.toFixed(4) : m.value}${m.limit === undefined ? "" : ` limit ${m.limit}`}`);
}
