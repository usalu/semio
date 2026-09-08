import assert from "node:assert/strict";
import { build } from "esbuild";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd(), product = "🧰️framework/🛍️products/📓️print";
const contract = JSON.parse(readFileSync(join(root, product, "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️command-boundaries.json"), "utf8"));
for (const entry of contract.entries) {
  const result = await build({ absWorkingDir: root, entryPoints: [join(product, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
  const files = Object.keys(result.metafile!.inputs);
  for (const forbidden of contract.forbidden) assert.equal(files.some(path => path.includes(forbidden)), false, `${entry} loads ${forbidden}`);
  assert.ok(files.length <= contract.maxSourceFiles, `${entry}: ${files.length} source files`);
  console.log(`[DEBUG] Print command boundary ${entry}: ${files.length} source files PASS`);
}
