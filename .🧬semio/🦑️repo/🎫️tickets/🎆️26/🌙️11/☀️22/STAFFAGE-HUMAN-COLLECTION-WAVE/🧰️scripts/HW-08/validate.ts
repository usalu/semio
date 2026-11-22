/** 🩺️ Validates humans (default: the HW-08 slugs, or the given slugs, or `all`) with the library's `validateAsset` and prints their issues.
 *  bun validate.ts [slug ...|all] */
import { join, resolve } from "node:path";
import { readdirSync } from "node:fs";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const humans = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const own = ["hiker-with-trekking-poles", "person-stretching", "inline-skater"];
const args = process.argv.slice(2);
const slugs = args[0] === "all" ? readdirSync(humans).sort() : args.length ? args : own;
const max = Number(process.env.MAX ?? 40);
let failing = 0;
for (const slug of slugs) {
  const t0 = performance.now();
  const asset = (await Bun.file(join(humans, slug, "🔣️.json")).json()).asset;
  const issues = validateAsset(asset);
  const ms = Math.round(performance.now() - t0);
  console.log(`${slug}\t${issues.length}\t${ms} ms`);
  if (issues.length) failing++;
  for (const issue of issues.slice(0, max)) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 600));
}
console.log(`failing ${failing} of ${slugs.length}`);
