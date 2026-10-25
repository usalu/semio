import { build } from "esbuild";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio", own = "🧰️framework/🛍️products/📓️print/", product = join(root, own.slice(0, -1));
const path = join(product, "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json");
const contract = JSON.parse(readFileSync(path, "utf8"));
import { printCommandSourceIdentitiesV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🛡️command-and-publication-verification/🟦️.ts";
const sourceInputs: Record<string, unknown> = {}, externalDependencies: Record<string, string[]> = {};
for (const entry of contract.entries as string[]) {
  const r = await build({ absWorkingDir: root, entryPoints: [join(product, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
  sourceInputs[entry] = printCommandSourceIdentitiesV1(r.metafile!.inputs as never);
  const external = [...new Set(Object.values(r.metafile!.outputs).flatMap(o => o.imports).filter(i => i.external && !/^(node|bun):/u.test(i.path) && i.path !== "bun").map(i => i.path.startsWith("@") ? i.path.split("/").slice(0, 2).join("/") : i.path.split("/")[0]))].sort();
  externalDependencies[entry] = external;
}
contract.sourceInputs = sourceInputs;
contract.externalDependencies = externalDependencies;
contract.sourceRefusals = [
  { id: "extra-owner-import", remove: [], addSources: [], addImports: ["🧰️framework/🛍️products/🧪️extra-owner/🟦️.ts"], lengthDelta: 0, accepted: false },
  { id: "extra-source", remove: [], addSources: ["🧰️framework/🛍️products/📓️print/🔨️modules/🧪️extra/🟦️.ts"], addImports: [], lengthDelta: 1, accepted: false },
  { id: "missing-source", remove: ["$entry"], addSources: [], addImports: [], lengthDelta: -1, accepted: false },
  { id: "same-count-substitution", remove: ["$entry"], addSources: ["🧰️framework/🛍️products/📓️print/🔨️modules/🧪️substitute/🟦️.ts"], addImports: [], lengthDelta: 0, accepted: false },
];
writeFileSync(path, JSON.stringify(contract, null, 2) + "\n");
console.log("written", Object.entries(sourceInputs).map(([k, v]: any) => `${k}: ${v.sources.length} sources, ${v.owners.length} owners`).join("; "));
