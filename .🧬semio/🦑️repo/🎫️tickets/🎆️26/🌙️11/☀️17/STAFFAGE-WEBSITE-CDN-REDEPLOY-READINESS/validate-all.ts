/** 🩺️ Validates every catalog asset with the library's `validateAsset` and prints one line per failing asset with its first issues. */
import { Glob } from "bun";
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
let failing = 0, total = 0;
for (const path of [...new Glob("*/*/🔣️.json").scanSync(catalog)].sort()) {
  total++;
  const asset = (await Bun.file(join(catalog, path)).json()).asset;
  const issues = validateAsset(asset);
  if (issues.length === 0) continue;
  failing++;
  console.log(`${path.split("/").slice(0, 2).join("/")}\t${issues.length}\t${issues.slice(0, 3).map((issue) => `${issue.code}: ${issue.message.en}`).join(" | ").slice(0, 400)}`);
}
console.log(`failing ${failing} of ${total}`);
