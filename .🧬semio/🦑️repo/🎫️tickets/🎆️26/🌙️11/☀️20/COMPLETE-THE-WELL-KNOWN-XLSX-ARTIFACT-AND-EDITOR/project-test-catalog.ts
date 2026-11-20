import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { specificCatalogPublicationV1 } from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts";
import { renderCatalogFiles } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
import { readFileSync } from "node:fs";

const root = process.cwd();
const publication = specificCatalogPublicationV1(path => readFileSync(join(root, path), "utf8"));
const { files, playgrounds } = renderCatalogFiles(root, undefined, "exclude", publication);
const destination = join(import.meta.dir, "🗑️generated", "test-catalog");
await mkdir(destination, { recursive: true });
const names = ["🔌️plugins.json", "🎠️playgrounds.json", "🚀️playgrounds.json", "🗂️sources.json"];
for (const name of names) await writeFile(join(destination, name), files[name]!);
if (playgrounds.length < 2) throw new Error("The production time-travel tests require two deployed playgrounds");
console.log(`Projected ${playgrounds.length} deployed playgrounds into ${destination}`);
