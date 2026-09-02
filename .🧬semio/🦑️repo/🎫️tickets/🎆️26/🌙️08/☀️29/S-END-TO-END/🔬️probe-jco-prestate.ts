// 🔬️ Surfaces the exception that `nestedCargoGeneratedPrestate`'s `catch { return false; }` swallows
// for the `jco-package-adapter` contract, which is what makes its tracked-output check fail.
import { semanticPackageAdapterPreview, loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
try {
  const taxonomy = loadCatalogTaxonomy();
  const adapters = semanticPackageAdapterPreview(root, "jcoprobe-guest", taxonomy);
  console.log("adapters:", adapters.map((a: { path: string }) => a.path));
} catch (error) {
  console.log("THREW:", error instanceof Error ? error.message : String(error));
  if (error instanceof Error && error.stack) console.log(error.stack.split("\n").slice(0, 6).join("\n"));
}
