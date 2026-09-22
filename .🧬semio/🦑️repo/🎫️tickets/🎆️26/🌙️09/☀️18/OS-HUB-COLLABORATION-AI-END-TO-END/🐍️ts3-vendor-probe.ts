/** 🔎️ Measures the vendor dir the deployed-vendor-transport law reads and how the taxonomy names each file. */
import { readdirSync } from "node:fs";
import { join, relative } from "node:path";
import { loadCatalogTaxonomy, fixedFilenameContractIdsForPath } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { pluginOutRoot } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const vendorPath = "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim";
const taxonomy = loadCatalogTaxonomy();
const vendorRoot = join(pluginOutRoot, vendorPath);
console.log("pluginOutRoot =", relative(repoRoot, pluginOutRoot));
for (const name of readdirSync(vendorRoot).sort()) {
  const rel = relative(repoRoot, join(vendorRoot, name)).replace(/\\/g, "/");
  console.log(name, "->", JSON.stringify(fixedFilenameContractIdsForPath(rel, taxonomy)));
}
