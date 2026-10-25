import { build } from "esbuild";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { printCommandSourceIdentitiesV1, type PrintCommandInputsV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🛡️command-and-publication-verification/🟦️.ts";
const workspaceRoot = "/Users/ueli/Documents/semio", productRoot = join(workspaceRoot, "🧰️framework/🛍️products/📓️print");
const contract = JSON.parse(readFileSync(join(productRoot, "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json"), "utf8"));
for (const entry of contract.entries as string[]) {
  const result = await build({ absWorkingDir: workspaceRoot, entryPoints: [join(productRoot, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
  const inputs = result.metafile!.inputs as PrintCommandInputsV1, files = Object.keys(inputs), entryKey = "🧰️framework/🛍️products/📓️print/" + entry;
  const expected = contract.sourceInputs[entry];
  assert.deepEqual(printCommandSourceIdentitiesV1(inputs), expected, entry);
  const validate = new (createRequire(import.meta.url)("ajv").default)().compile({ const: expected });
  assert.equal(validate(printCommandSourceIdentitiesV1(inputs)), true);
  for (const vector of contract.sourceRefusals) {
    const removed = new Set<string>((vector.remove as string[]).map(path => path === "$entry" ? entryKey : path));
    const kept = Object.entries(inputs).filter(([path]) => !removed.has(path)).map(([path, input]) => [path, path === entryKey ? { imports: [...input.imports, ...(vector.addImports as string[]).map(path => ({ path }))] } : input] as const);
    const candidate: PrintCommandInputsV1 = { ...Object.fromEntries(kept), ...Object.fromEntries((vector.addSources as string[]).map(path => [path, { imports: [] }])) };
    assert.equal(Object.keys(candidate).length - files.length, vector.lengthDelta, `${entry}/${vector.id}: cardinality`);
    assert.equal(validate(printCommandSourceIdentitiesV1(candidate)), vector.accepted, `${entry}/${vector.id}`);
    assert.throws(() => assert.deepEqual(printCommandSourceIdentitiesV1(candidate), expected), assert.AssertionError, `${entry}/${vector.id}`);
  }
  console.log("boundary ok", entry);
}
