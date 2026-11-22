/** 🩺️ Validates the given human assets (catalog slugs or asset JSON paths) with the library's `validateAsset` and prints their issues.
 *  bun validate.ts <slug|path.json> ... */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
let failing = 0;
for (const arg of process.argv.slice(2)) {
  const path = arg.endsWith(".json") ? arg : join(catalog, arg, "🔣️.json");
  const asset = (await Bun.file(path).json()).asset;
  const t0 = performance.now();
  const issues = validateAsset(asset);
  console.log(`${asset.id}\t${issues.length} issues\t${((performance.now() - t0) / 1000).toFixed(1)} s`);
  if (issues.length > 0) failing++;
  const counts = new Map<string, number>();
  for (const issue of issues) counts.set(issue.code, (counts.get(issue.code) ?? 0) + 1);
  for (const [code, n] of counts) console.log(`  ${code} x${n}`);
  for (const issue of issues.slice(0, Number(process.env.N ?? 40))) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, Number(process.env.W ?? 400)));
}
console.log(`failing ${failing}`);
