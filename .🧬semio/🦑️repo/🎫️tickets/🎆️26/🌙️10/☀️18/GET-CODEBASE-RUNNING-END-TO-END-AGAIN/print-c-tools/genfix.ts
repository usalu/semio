import { build } from "esbuild";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio", own = "🧰️framework/🛍️products/📓️print/", product = join(root, own.slice(0, -1));
const path = join(product, "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json");
const contract = JSON.parse(readFileSync(path, "utf8"));
const identities = (files: string[]) => ({ sources: files.filter(f => f.startsWith(own)).sort(), owners: [...new Set(files.filter(f => !f.startsWith(own)).map(f => f.split("/").slice(0, 3).join("/")))].sort() });
const sourceInputs: Record<string, unknown> = {}, externalDependencies: Record<string, string[]> = {};
for (const entry of contract.entries as string[]) {
  const r = await build({ absWorkingDir: root, entryPoints: [join(product, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
  sourceInputs[entry] = identities(Object.keys(r.metafile!.inputs));
  const external = [...new Set(Object.values(r.metafile!.outputs).flatMap(o => o.imports).filter(i => i.external && !/^(node|bun):/u.test(i.path) && i.path !== "bun").map(i => i.path.startsWith("@") ? i.path.split("/").slice(0, 2).join("/") : i.path.split("/")[0]))].sort();
  externalDependencies[entry] = external;
}
contract.sourceInputs = sourceInputs;
contract.externalDependencies = externalDependencies;
contract.sourceRefusals = [
  { id: "extra-source", remove: [], add: ["🧰️framework/🛍️products/🧪️extra-owner/🟦️.ts"], lengthDelta: 1, accepted: false },
  { id: "missing-source", remove: ["$entry"], add: [], lengthDelta: -1, accepted: false },
  { id: "same-count-substitution", remove: ["$entry"], add: ["🧰️framework/🛍️products/📓️print/🔨️modules/🧪️substitute/🟦️.ts"], lengthDelta: 0, accepted: false },
];
writeFileSync(path, JSON.stringify(contract, null, 2) + "\n");
console.log("written", Object.entries(sourceInputs).map(([k, v]: any) => `${k}: ${v.sources.length} sources, ${v.owners.length} owners`).join("; "));
