import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { specificCatalogPublicationV1 } from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts";
import { renderCatalogFiles } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
import { runtimeComponentClosure } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { demonstratorRuntimeComponentIds } from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts";
import { demonstratorShippedComponents } from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📇️catalog/🟦️.ts";
import { DEMONSTRATOR_RUNTIME_TARGETS } from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🟦️.ts";

const workspace = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const read = (path: string): string => readFileSync(join(workspace, path), "utf8");
const publication = specificCatalogPublicationV1(read);
const projection = renderCatalogFiles(workspace, undefined, "exclude", publication);
const current = demonstratorRuntimeComponentIds();
const plugins = projection.entries.filter(row => row.role === "plugin");
const extensions = demonstratorShippedComponents(projection.entries.filter(row => row.role === "extension"));
const variants = new Set(DEMONSTRATOR_RUNTIME_TARGETS.map(row => row.variant));
const roots = [...new Set(projection.playgrounds.filter(row => variants.has(row.variant)).map(row => row.pluginId))];
const projected = runtimeComponentClosure([...plugins, ...extensions], roots);
const manifestChanged = Object.entries(projection.files).filter(([path, bytes]) => {
  try { return read(join(publication.outputRoot, path)) !== bytes; } catch { return true; }
}).map(([path]) => path);
const summary = {
  current, projected, added: projected.filter(id => !current.includes(id)), removed: current.filter(id => !projected.includes(id)),
  gis: plugins.find(row => row.pluginId === "gis")?.dependsOn,
  standaloneGis: runtimeComponentClosure([...plugins, ...extensions], ["gis"]),
  manifestChanged, diagnostics: projection.diagnostics,
};
writeFileSync(new URL("./🗑️generated/stdio-catalog-render.json", import.meta.url), JSON.stringify(projection.files));
console.log(JSON.stringify(summary, null, 2));
