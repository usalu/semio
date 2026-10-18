import { readFileSync, writeFileSync } from "node:fs";
import { readEntityCatalog } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📥️source/🟦️.ts";
import { generatedTargets } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📋️plan/🟦️.ts";
const write = process.argv.includes("--write");
const source = readEntityCatalog("/Users/ueli/Documents/semio");
for (const target of generatedTargets(source)) {
  const current = readFileSync(target.path, "utf8");
  console.log(target.path.replace("/Users/ueli/Documents/semio/", ""), current === target.content ? "fresh" : "STALE", target.content.length, current.length);
  if (write && current !== target.content) writeFileSync(target.path, target.content);
}
