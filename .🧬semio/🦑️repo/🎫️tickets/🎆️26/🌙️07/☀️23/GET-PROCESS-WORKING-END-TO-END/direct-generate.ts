import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { renderCatalogFiles, publishCatalogFilesV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
import { specificCatalogPublicationV1 } from "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const publication = specificCatalogPublicationV1((path: string) => readFileSync(resolve(repoRoot, path), "utf8"));
const { files, entries } = renderCatalogFiles(repoRoot, undefined, "exclude", publication);
publishCatalogFilesV1(join(repoRoot, publication.outputRoot), files);
console.log("generated", entries.length);
