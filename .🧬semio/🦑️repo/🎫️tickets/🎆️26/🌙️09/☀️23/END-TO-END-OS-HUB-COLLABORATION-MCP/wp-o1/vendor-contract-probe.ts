
import { loadCatalogTaxonomy, fixedFilenameContractIdsForPath } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const tax = loadCatalogTaxonomy();
const rel = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js';
console.log(rel, fixedFilenameContractIdsForPath(rel, tax));
for (const f of ["common.js","opfs-filesystem.js"]) {
  const p = rel.replace("cli.js", f);
  console.log(p, fixedFilenameContractIdsForPath(p, tax));
}
