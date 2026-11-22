/** 🩺️ Validates the given human slugs (default: the HW-16 assets) with the library's `validateAsset` and prints their issues.
 *  bun validate-some.ts [slug ...] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const args = process.argv.slice(2);
const slugs = args.length > 0 ? args : ["person-in-rain-poncho", "person-in-winter-coat", "sunbather-on-towel"];
let failing = 0;
for (const slug of slugs) {
  const t0 = performance.now();
  const asset = (await Bun.file(join(catalog, slug, "🔣️.json")).json()).asset;
  const issues = validateAsset(asset);
  const ms = Math.round(performance.now() - t0);
  console.log(`${slug}\t${issues.length} issues\t${ms} ms`);
  if (issues.length === 0) continue;
  failing++;
  const counts = new Map<string, number>();
  for (const i of issues) counts.set(i.code, (counts.get(i.code) ?? 0) + 1);
  console.log(`   codes: ${[...counts].map(([c, n]) => `${c}×${n}`).join(", ")}`);
  for (const issue of issues.slice(0, Number(process.env.N ?? 40))) console.log(`   ${issue.code} @ ${issue.path ?? ""}: ${issue.message.en}`.slice(0, 400));
}
console.log(`failing ${failing} of ${slugs.length}`);
