import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd();
const moduleAt = (path: string) => import(pathToFileURL(resolve(root, path)).href);
const { specificCatalogPublicationV1 } = await moduleAt("✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts");
const { renderCatalogFiles, publishCatalogFilesV1 } = await moduleAt("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts");
const publication = specificCatalogPublicationV1((path: string) => readFileSync(resolve(root, path), "utf8"));
const {files, entries, playgrounds, diagnostics} = renderCatalogFiles(root, undefined, "exclude", publication);
publishCatalogFilesV1(join(root, publication.outputRoot), files);
console.log(JSON.stringify({ files: Object.keys(files).length, plugins: entries.length, playgrounds: playgrounds.length, bim: entries.find(row => row.pluginId === "bim")?.pluginId, diagnostics: diagnostics.filter(row => row.pluginId === "bim") }));
