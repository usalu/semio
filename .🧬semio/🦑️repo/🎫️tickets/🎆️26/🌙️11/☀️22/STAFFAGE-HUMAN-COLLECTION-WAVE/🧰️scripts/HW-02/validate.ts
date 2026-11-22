/** 🩺️ Validates human assets (given slugs, or a JSON file path) with the library's `validateAsset` and prints their issues.
 *  bun validate.ts <slug|file.json> [...] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const limit = Number(process.env.LIMIT ?? 40);
let failing = 0;
for (const arg of process.argv.slice(2)) {
  const file = arg.endsWith(".json") ? arg : join(catalog, arg, "🔣️.json");
  const t0 = performance.now();
  const asset = (await Bun.file(file).json()).asset;
  const issues = validateAsset(asset);
  console.log(`${arg}\t${issues.length} issues\t${((performance.now() - t0) / 1000).toFixed(1)} s`);
  if (issues.length) failing++;
  for (const issue of issues.slice(0, limit)) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 400));
}
process.exit(failing ? 1 : 0);
