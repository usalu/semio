/** 🔎️ Reports the Specific registry taxonomy gate without changing generated output. */
import { readFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { publicationPackageRootsV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🟦️.ts";
import { findNewContractPluginRoots, validateTaxonomyTree } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts";
import { specificCatalogPublicationV1 } from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts";
const root = process.cwd(), publication = specificCatalogPublicationV1(path => readFileSync(resolve(root, path), "utf8"));
const roots = publicationPackageRootsV1(publication);
const plugins = roots.length ? findNewContractPluginRoots(root).filter(row => roots.some(path => relative(root, row.pluginRoot).replaceAll("\\", "/").startsWith(path + "/"))) : [];
const findings = plugins.flatMap(({ pluginId, pluginRoot }) => validateTaxonomyTree(pluginRoot, pluginId));
console.log(JSON.stringify({ plugins: plugins.length, findings }, null, 2));
if (findings.length) process.exitCode = 1;
