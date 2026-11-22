/** 🩺️ Validates HW-07 humans (or the given slugs or asset files) with the library's `validateAsset` and prints their issues.
 *  bun validate.ts [slug|file.json ...] [--all] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const args = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const all = process.argv.includes("--all");
const targets = args.length > 0 ? args : ["tennis-player", "basketball-player", "yoga-practitioner"];
for (const t of targets) {
  const path = t.endsWith(".json") ? t : join(catalog, t, "🔣️.json");
  const asset = (await Bun.file(path).json()).asset;
  const t0 = performance.now();
  const issues = validateAsset(asset);
  console.log(`${asset.id}\t${issues.length} issues\t${((performance.now() - t0) / 1000).toFixed(1)} s`);
  const codes: Record<string, number> = {};
  for (const i of issues) codes[i.code] = (codes[i.code] ?? 0) + 1;
  console.log("  codes", JSON.stringify(codes));
  for (const issue of all ? issues : issues.slice(0, 40)) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 400));
}
