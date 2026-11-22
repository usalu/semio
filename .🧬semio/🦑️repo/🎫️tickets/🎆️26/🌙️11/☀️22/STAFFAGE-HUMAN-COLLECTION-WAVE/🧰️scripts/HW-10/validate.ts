/** 🩺️ Validates human assets with the library's `validateAsset` and prints every issue.
 *  bun validate.ts <slug | path/to/asset.json> [...]   (LIMIT=n caps the printed issues per asset) */
import { join, resolve } from "node:path";
import { validateAsset } from "./lib.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const limit = Number(process.env.LIMIT ?? 60);
let failing = 0;
for (const arg of process.argv.slice(2)) {
  const started = performance.now();
  const path = arg.endsWith(".json") ? arg : join(catalog, arg, "🔣️.json");
  const asset = (await Bun.file(path).json()).asset;
  const issues = validateAsset(asset);
  console.log(`${asset.id}\t${issues.length} issues\t${((performance.now() - started) / 1000).toFixed(1)} s`);
  if (issues.length > 0) failing++;
  const codes = new Map<string, number>();
  for (const issue of issues) codes.set(issue.code, (codes.get(issue.code) ?? 0) + 1);
  for (const [code, n] of codes) console.log(`  ${code} × ${n}`);
  for (const issue of issues.slice(0, limit)) console.log(`   ${issue.code} ${issue.path ?? ""}: ${issue.message.en}`.slice(0, 600));
}
console.log(`failing ${failing}`);
