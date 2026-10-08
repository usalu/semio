import assert from "node:assert/strict";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import ts from "typescript";

const ticket = dirname(import.meta.dir);
let root = ticket;
while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
const inputs = readdirSync(ticket).filter(name => /^📥️current-plugin-oct8-(?:corpus-facets|extended-.*)-before\.json$/u.test(name)).sort();
assert(inputs.length > 0);
const expected = new Map<string, string>(), absent = new Set<string>();
for (const name of inputs) {
  const input = JSON.parse(readFileSync(join(ticket, name), "utf8"));
  for (const row of input.sources) expected.set(row.path, row.after);
  for (const row of input.documents) absent.add(row.path);
}

if (process.argv[2] === "source-integrity") {
  for (const [path, after] of expected) {
    const source = readFileSync(join(root, path), "utf8");
    assert.equal(source, after, path);
    if (path.endsWith(".json")) JSON.parse(source);
    else assert.equal((ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true) as any).parseDiagnostics.length, 0, path);
  }
  for (const path of absent) assert(!existsSync(join(root, path)), path);
  console.log("[DEBUG] Exact authored current plugin facet sources", expected.size, "retired corpus documents", absent.size);
} else if (process.argv[2] === "tests") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  const paths = [...expected.keys()].filter(path => path.endsWith(".ts") && path.includes("🧪️tests"));
  assert(paths.length > 0);
  const catalog = paths.filter(path => path.includes("✅️catalog-complete")), local = paths.filter(path => !catalog.includes(path));
  await runBudgetedTestCommand(process.execPath, ["test", ...local.map(path => join(root, path))], { cwd: root, env: process.env, budgetMs: 120_000, throwOnFailure: true });
  await runBudgetedTestCommand(process.execPath, ["test", ...catalog.map(path => join(root, path)), "-t", "validates the neutral contract and withholds every publication|admits only schema-owned module routes"], { cwd: root, env: process.env, budgetMs: 120_000, throwOnFailure: true });
  const { documentBackboneBindingOracle } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts"));
  const { testRetainedWindowInputOracle } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🧪️tests/🪟️retained-window-input/🟦️.ts"));
  const bindings = documentBackboneBindingOracle(root);
  assert(bindings > 0);
  testRetainedWindowInputOracle();
  const { compositionLawGroups, testCompositionOwnership } = await import(join(root, "✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️ownership/🟦️.ts"));
  assert(compositionLawGroups().length > 0);
  testCompositionOwnership();
  console.log("[DEBUG] Actual current plugin facet Bun test owners", paths.length, "binding oracle cases", bindings, "retained window helper", 1, "composition ownership helper", 1);
} else if (process.argv[2] === "catalog-tests") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts")], { cwd: root, env: process.env, budgetMs: 120_000, throwOnFailure: true });
} else throw Error("Expected source-integrity, tests or catalog-tests");
