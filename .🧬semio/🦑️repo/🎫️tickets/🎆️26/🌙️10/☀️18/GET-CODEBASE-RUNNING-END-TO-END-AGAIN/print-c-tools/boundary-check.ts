import { build } from "esbuild";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { printCommandSourceIdentitiesV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🛡️command-and-publication-verification/🟦️.ts";
const workspaceRoot = "/Users/ueli/Documents/semio", productRoot = join(workspaceRoot, "🧰️framework/🛍️products/📓️print");
const contract = JSON.parse(readFileSync(join(productRoot, "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json"), "utf8"));
for (const entry of contract.entries as string[]) {
  const result = await build({ absWorkingDir: workspaceRoot, entryPoints: [join(productRoot, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
  const files = Object.keys(result.metafile!.inputs);
  for (const forbidden of contract.forbidden) assert.equal(files.some(path => path.includes(forbidden)), false, `${entry} loads ${forbidden}`);
  const expected = contract.sourceInputs[entry];
  assert.deepEqual(printCommandSourceIdentitiesV1(files), expected, entry);
  const validate = new (createRequire(import.meta.url)("ajv").default)().compile({ const: expected });
  assert.equal(validate(printCommandSourceIdentitiesV1(files)), true);
  for (const vector of contract.sourceRefusals) {
    const removed = new Set<string>((vector.remove as string[]).map(path => path === "$entry" ? "🧰️framework/🛍️products/📓️print/" + entry : path));
    const candidate = [...files.filter(path => !removed.has(path)), ...vector.add as string[]];
    assert.equal(candidate.length - files.length, vector.lengthDelta, `${entry}/${vector.id}: cardinality`);
    assert.equal(validate(printCommandSourceIdentitiesV1(candidate)), vector.accepted, `${entry}/${vector.id}`);
  }
  const external = [...new Set(Object.values(result.metafile!.outputs).flatMap(output => output.imports).filter(item => item.external && !/^(node|bun):/u.test(item.path) && item.path !== "bun").map(item => item.path.startsWith("@") ? item.path.split("/").slice(0, 2).join("/") : item.path.split("/")[0]))].sort();
  assert.deepEqual(external, contract.externalDependencies[entry], `${entry}: external imports`);
  console.log("[DEBUG] ok", entry);
}
