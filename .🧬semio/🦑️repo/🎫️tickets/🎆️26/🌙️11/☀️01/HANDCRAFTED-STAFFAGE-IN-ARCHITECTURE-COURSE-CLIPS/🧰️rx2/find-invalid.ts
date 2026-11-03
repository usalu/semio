/** 🔎️ Lists every catalog asset that fails `validateAsset`, with its issues. */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { validateAsset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

const catalog = join(import.meta.dirname, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
let total = 0;
for (const category of readdirSync(catalog, { withFileTypes: true }).filter((entry) => entry.isDirectory())) {
  const folder = join(catalog, category.name);
  for (const asset of readdirSync(folder, { withFileTypes: true }).filter((entry) => entry.isDirectory())) {
    const file = join(folder, asset.name, "🔣️.json");
    let document: unknown;
    try {
      document = JSON.parse(readFileSync(file, "utf8"));
    } catch {
      continue;
    }
    total += 1;
    const issues = validateAsset((document as { asset: never }).asset ?? (document as never));
    if (issues.length > 0) console.log(`✗ ${category.name}/${asset.name}`, JSON.stringify(issues.map((issue) => [issue.path, issue.code, issue.message.en])));
  }
}
console.log(`checked ${total}`);
