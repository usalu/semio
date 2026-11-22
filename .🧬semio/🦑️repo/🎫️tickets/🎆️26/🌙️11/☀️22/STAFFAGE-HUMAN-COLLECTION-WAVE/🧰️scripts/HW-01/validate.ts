/** 🩺️ Validates HW-01's human assets (or the given slugs, or `--file <path>` JSON documents) with `validateAsset` and prints every issue.
 *  bun validate.ts [slug ...] [--file path ...] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const args = process.argv.slice(2);
const files: string[] = [];
const slugs: string[] = [];
for (let i = 0; i < args.length; i++) args[i] === "--file" ? files.push(args[++i]!) : slugs.push(args[i]!);
if (files.length === 0 && slugs.length === 0) slugs.push("standing-desk-worker", "presenter-at-flipchart", "office-worker-carrying-laptop");
for (const slug of slugs) files.push(join(catalog, slug, "🔣️.json"));
let failing = 0;
for (const path of files) {
  const asset = (await Bun.file(path).json()).asset;
  const issues = validateAsset(asset);
  console.log(`${asset.id}\t${issues.length}`);
  if (issues.length > 0) failing++;
  for (const issue of issues) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 400));
}
console.log(`failing ${failing} of ${files.length}`);
