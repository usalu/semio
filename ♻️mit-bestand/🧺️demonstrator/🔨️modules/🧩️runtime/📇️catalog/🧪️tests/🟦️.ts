import { testDemonstratorE2eArtifactPaths } from "../../../🧪️e2e/📁️artifacts/🧪️tests/🟦️.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, relative } from "node:path";
import { pathToFileURL } from "node:url";

/** 🌐️ Proves the authored catalog and brand import graph stay safe for a browser. */
export async function testDemonstratorBrowserCatalog(workspace: string): Promise<void> {
  await testDemonstratorE2eArtifactPaths(workspace);
  const require = createRequire(import.meta.url), root = join(workspace, "♻️mit-bestand/🧺️demonstrator"), directory = join(root, "🔨️modules/🧩️runtime/📇️catalog");
  const fixture = JSON.parse(readFileSync(join(directory, "🧫️cases.json"), "utf8")), schema = JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8"));
  const validator = new (require("ajv").default)({ strict: false });
  assert.ok(validator.validate(schema, fixture), JSON.stringify(validator.errors));
  const typescript = require("typescript"), entry = typescript.createSourceFile("🟦️.tsx", readFileSync(join(root, "🟦️.tsx"), "utf8"), typescript.ScriptTarget.Latest, true, typescript.ScriptKind.TSX);
  const authorities: Record<string, number>[] = [];
  const visit = (node: any): void => {
    if (typescript.isCallExpression(node) && node.expression.getText(entry) === "composeSpecificOsCatalogV1") {
      const authority = node.arguments[1];
      assert.ok(typescript.isObjectLiteralExpression(authority));
      authorities.push(Object.fromEntries(authority.properties.filter((property: any) => typescript.isPropertyAssignment(property) && typescript.isNumericLiteral(property.initializer)).map((property: any) => [property.name.getText(entry), Number(property.initializer.text)])));
    }
    typescript.forEachChild(node, visit);
  };
  visit(entry);
  const { durationMs, ...limits } = fixture.admission.authority;
  assert.deepEqual(authorities, [limits], "The page must admit its full registry with explicit finite authority");
  const registrySchema = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🔣️.json"), "utf8"));
  const policy = registrySchema["x-semio-admission"];
  assert.equal(limits.maxRows, policy.maxRows);
  for (const [value, maximum] of [[limits.maxBytes, policy.maxJsonBytes], [limits.maxEdges, policy.maxEdges], [limits.maxWork, policy.maxWork], [durationMs, policy.maxDurationMs]]) assert.ok(value > 0 && value <= maximum);
  const { composeSpecificOsCatalogV1 } = await import(pathToFileURL(join(workspace, "✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts")).href);
  const progress: { completed: number; total: number; work: number }[] = [];
  const control = { ...limits, deadlineMs: durationMs, now: () => 0, cancelled: () => false, progress: (value: { completed: number; total: number; work: number }) => progress.push(value) };
  const composed = composeSpecificOsCatalogV1(fixture.admission.baseUrl, control);
  assert.ok(validator.validate(registrySchema, composed.rows), JSON.stringify(validator.errors));
  assert.deepEqual(Object.fromEntries(Object.keys(fixture.admission.expectedCounts).map(key => [key, composed.rows[key].length])), fixture.admission.expectedCounts);
  const total = Object.values(fixture.admission.expectedCounts).reduce<number>((sum, value) => sum + Number(value), 0);
  assert.equal(progress.length, total);
  assert.equal(progress.at(-1)?.completed, total);
  assert.equal(progress.at(-1)?.total, total);
  for (const pane of fixture.panes) assert.ok(composed.catalog.playgrounds.some((row: any) => row.variant === pane.runtimeVariant));
  assert.throws(() => composeSpecificOsCatalogV1(fixture.admission.baseUrl, { ...control, maxRows: fixture.admission.rejectedMaxRows }), /plugin-catalog-invalid: row (count|credits)/);
  const catalog = await import(pathToFileURL(join(directory, "🟦️.ts")).href), build = require("esbuild").build;
  const browser = await build({ entryPoints: [join(directory, "🟦️.ts")], absWorkingDir: root, bundle: true, write: false, platform: "browser", format: "iife", globalName: "DemonstratorCatalog", metafile: true, logLevel: "silent" });
  const normalizedInputs = (metafile: any): string[] => Object.keys(metafile.inputs).map(path => relative(root, join(root, path)).replaceAll("\\", "/")).sort();
  assert.deepEqual(normalizedInputs(browser.metafile), [...fixture.catalogInputs].sort());
  assert.deepEqual(Object.values(browser.metafile.outputs).flatMap((output: any) => output.imports), []);
  const bundled = new Function(browser.outputFiles[0].text + ";return DemonstratorCatalog;")();
  for (const implementation of [catalog, bundled]) {
    assert.equal(implementation.DEMONSTRATOR_HOST, fixture.host);
    assert.equal(implementation.DEMONSTRATOR_ASSETS_DIR, fixture.assetsDirectory);
    assert.deepEqual(implementation.DEMONSTRATOR_RUNTIME_PANES, fixture.panes);
    for (const row of fixture.panes) assert.equal(implementation.demonstratorPaneRuntimeVariant(row.variant), row.runtimeVariant);
    for (const variant of fixture.invalidVariants) assert.throws(() => implementation.demonstratorPaneRuntimeVariant(variant), /Unknown demonstrator pane variant:/);
  }
  const brand = await build({ entryPoints: [join(root, "🪧️brand.ts")], absWorkingDir: root, bundle: true, write: false, platform: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, packages: "external", metafile: true, logLevel: "silent" });
  assert.deepEqual(normalizedInputs(brand.metafile), [...fixture.brandInputs].sort());
  assert.deepEqual([...new Set(Object.values(brand.metafile.outputs).flatMap((output: any) => output.imports.map((entry: any) => entry.path)))].sort(), [...fixture.brandImports].sort());
}
