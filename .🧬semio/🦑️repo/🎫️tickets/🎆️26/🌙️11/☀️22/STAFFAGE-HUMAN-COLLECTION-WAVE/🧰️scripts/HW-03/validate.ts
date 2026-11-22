/** 🩺️ Validates the given human assets (catalog slugs or asset files) with the library's `validateAsset` and prints their issues.
 *  bun validate.ts <slug|file.json> ... */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const limit = Number(process.env.LIMIT ?? 40);
for (const arg of process.argv.slice(2)) {
  const path = arg.endsWith(".json") ? arg : join(catalog, arg, "🔣️.json");
  const t0 = performance.now();
  const asset = (await Bun.file(path).json()).asset;
  const issues = validateAsset(asset);
  console.log(`${asset.id}\t${issues.length} issues\t${((performance.now() - t0) / 1000).toFixed(1)} s`);
  const counts: Record<string, number> = {};
  for (const issue of issues) counts[issue.code] = (counts[issue.code] ?? 0) + 1;
  console.log("   codes", JSON.stringify(counts));
  for (const issue of issues.slice(0, limit)) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 400));
}
