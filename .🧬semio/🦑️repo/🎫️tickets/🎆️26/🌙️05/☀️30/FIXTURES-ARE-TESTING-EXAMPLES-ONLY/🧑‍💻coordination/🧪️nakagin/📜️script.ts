import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import ts from "typescript";
import { build } from "esbuild";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, output = process.env.SEMIO_FIXTURE_GENERATED!;
const source = readFileSync(join(output, "nakagin-original.ts"), "utf8");
const reference: Record<string, unknown> = {};
new Function("exports", ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS } }).outputText)(reference);
const fixture = "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧫️fixtures/🏢️nakagin/🔣️.json";
const value = JSON.parse(readFileSync(join(root, fixture), "utf8"));
assert.deepEqual(value, { nakagin: reference.nakagin, architects: reference.architects });
for (const element of ["📊️Table", "🔤️Textarea"]) {
  const result = await build({ absWorkingDir: root, entryPoints: [`🧰️framework/🔨️modules/🖱️ui/🧱️elements/${element}/🧪️tests/📚️storybook/🟦️.tsx`], bundle: true, write: false, metafile: true, format: "esm", logLevel: "silent", plugins: [{ name: "fixture-consumer-boundary", setup(builder) { builder.onResolve({ filter: /.*/ }, args => args.kind === "entry-point" || args.path.endsWith("🏢️nakagin/🔣️.json") ? undefined : { path: args.path, external: true }); } }] });
  assert(Object.keys(result.metafile!.inputs).includes(fixture));
  console.log(`[DEBUG] ${element}: canonical fixture resolved by esbuild`);
}
console.log("[DEBUG] Nakagin fixture values match the original TypeScript module through the independent compiler");
