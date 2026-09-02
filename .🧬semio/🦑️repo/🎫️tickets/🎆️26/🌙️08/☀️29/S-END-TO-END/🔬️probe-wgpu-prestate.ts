// 🔬️ Surfaces what `nestedCargoGeneratedPrestate` computes for the `wgpu-frame-worker` contract,
// whose bare `catch` otherwise hides the reason its tracked outputs read as missing.
import { semanticPackageSourceOutputPhase, loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
try {
  const phase = semanticPackageSourceOutputPhase(root, "wgpu-frame-worker", loadCatalogTaxonomy());
  console.log("phase paths:");
  for (const p of phase) console.log("  ", p);
} catch (error) {
  console.log("THREW:", error instanceof Error ? error.message : String(error));
  if (error instanceof Error && error.stack) console.log(error.stack.split("\n").slice(0, 5).join("\n"));
}
