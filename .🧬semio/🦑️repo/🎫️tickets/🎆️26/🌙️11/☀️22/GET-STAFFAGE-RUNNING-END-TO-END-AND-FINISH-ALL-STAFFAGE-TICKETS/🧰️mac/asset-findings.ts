/** 🩺️ Prints the findings the engine's `validateAsset` has for authored assets of the live tree, one line per asset: the codes and paths. Usage: bun asset-findings.ts <category-folder/asset> ...
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts — `validateAsset` */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { Asset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { validateAsset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

const catalog = resolve(import.meta.dirname, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
for (const name of process.argv.slice(2)) {
  const started = Date.now();
  const asset = (JSON.parse(readFileSync(resolve(catalog, name, "🔣️.json"), "utf8")) as { asset: Asset }).asset;
  const issues = await validateAsset(asset);
  console.log(`${name}: ${issues.length} findings in ${((Date.now() - started) / 1000).toFixed(1)} s${issues.map((issue) => `\n  ${issue.code} ${issue.path}: ${issue.message.en}`).join("")}`);
}
