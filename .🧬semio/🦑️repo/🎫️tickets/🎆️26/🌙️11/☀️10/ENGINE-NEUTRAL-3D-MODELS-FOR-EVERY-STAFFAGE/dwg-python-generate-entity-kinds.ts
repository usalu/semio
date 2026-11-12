/** Writes the git-ignored entity-kind catalog files the schema crate includes, with the exact plan of the `generate` target (`@semio-tech/framework-schema:generate`), without the graph runner. */
import { writeGeneratedFileIfChanged } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { readEntityCatalog } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📥️source/🟦️.ts";
import { generatedTargets } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📋️plan/🟦️.ts";
const root = process.cwd();
for (const target of generatedTargets(readEntityCatalog(root))) {
  writeGeneratedFileIfChanged(target.path, target.content);
  console.log("wrote", target.path);
}
