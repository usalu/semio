import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = process.cwd();
const { specificCatalogPublicationV1 } = await import(resolve(root, "✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts"));
const { renderCatalogFiles, publishCatalogFilesV1 } = await import(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts"));
const publication = specificCatalogPublicationV1((path: string) => readFileSync(resolve(root, path), "utf8"));
const catalog = renderCatalogFiles(root, undefined, "exclude", publication);
if (process.argv.includes("--publish")) publishCatalogFilesV1(resolve(root, publication.outputRoot), catalog.files);
const stale = Object.entries(catalog.files).filter(([name, expected]) => {
  try { return readFileSync(resolve(root, publication.outputRoot, name), "utf8") !== expected; }
  catch { return true; }
});
const puzzle = catalog.entries.find((entry: { pluginId: string }) => entry.pluginId === "puzzle");
console.error("[DEBUG] Plugins:", catalog.entries.length, "puzzle:", puzzle?.wasmOut, "withheld:", catalog.diagnostics, "stale:", stale.map(([name]) => name));
