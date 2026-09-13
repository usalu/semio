import { readFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = process.cwd();
const discovery = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"));
const taxonomy = discovery.loadCatalogTaxonomy();
const paths = [
  "🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📜️script.ts",
  "♻️mit-bestand/🧺️demonstrator/📜️script.ts",
  "✏️s/🔌️plugins/🗄️stdio/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts",
] as const;
console.log(JSON.stringify(paths.map((path) => ({ path, decision: discovery.fixedSourceDispositionDecision("root-script", readFileSync(join(repoRoot, path), "utf8"), taxonomy) })), null, 2));
