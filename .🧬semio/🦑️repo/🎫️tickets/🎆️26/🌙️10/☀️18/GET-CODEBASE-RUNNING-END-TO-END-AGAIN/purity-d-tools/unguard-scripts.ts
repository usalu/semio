import { readFileSync, writeFileSync } from "node:fs";
import { classifyPackageSourceDisposition } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🏷️package-source-classification/🟦️.ts";
import { loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const t = loadCatalogTaxonomy();
const d = t.packageSourceDispositions["root-script"]!;
const g = t.packageGlueGrammar[d.grammarId ?? "typescript"]!;
const root = "/Users/ueli/Documents/semio/";
const apply = process.argv[3] === "apply";
for (const file of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const src = readFileSync(root + file, "utf8");
  const before = classifyPackageSourceDisposition(src, d, g);
  let out = src.replace(/^import ?\{ ?isMainModuleV1 ?\} ?from ?"[^"]*";?\n/m, "");
  out = out.replace(/if ?\(isMainModuleV1\(import\.meta\.url\)\) ?\{\n([\s\S]*?)\n\}\s*$/m, (_m, body) => body.replace(/^  /gm, "") + "\n");
  out = out.replace(/if ?\(isMainModuleV1\(import\.meta\.url\)\) ?(await )/m, "$1");
  const after = classifyPackageSourceDisposition(out, d, g);
  console.log(before, "->", after, file.replace(/^🧰️framework\/🛍️products\//, "P/"));
  if (apply && after === "tool-metadata" && before !== "tool-metadata") writeFileSync(root + file, out);
}
