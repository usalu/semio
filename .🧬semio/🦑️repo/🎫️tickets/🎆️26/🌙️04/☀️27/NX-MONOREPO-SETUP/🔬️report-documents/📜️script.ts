import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
const root = process.cwd(), product = "♻️mit-bestand/📋️bericht", modulePath = join(root, product, "🔨️modules/📄️documents");
const require = createRequire(import.meta.url), catalog = JSON.parse(readFileSync(join(modulePath, "🔣️.json"), "utf8"));
assert.ok(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(modulePath, "🧬️schema.json"), "utf8")), catalog));
const fixture = JSON.parse(readFileSync(join(root, product, "🧫️tests/🔣️report-family.json"), "utf8"));
assert.deepEqual(catalog.documents.map((document: any) => ({ id: document.id, path: document.texPath })), fixture.documents);
const project = JSON.parse(readFileSync(join(root, product, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
for (const document of catalog.documents) {
  const target = project.targets[`build-${document.id}`];
  assert.deepEqual(target.outputs, [`{projectRoot}/dist/documents/${document.id}`]);
  assert.equal(target.cache, true);
  assert.equal(target.options.forwardAllArgs, false);
  assert.ok(target.dependsOn.includes("@semio-tech/print:deps-tex"));
}
assert.deepEqual(project.targets.build.dependsOn, catalog.documents.map((document: any) => `build-${document.id}`));
console.log("[DEBUG] Report schema, independent family fixture and fixed Nx output ownership PASS");

const ts = require("typescript"), source = ts.createSourceFile("root.ts", readFileSync(join(root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
const declaration = source.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "resolveNxInvocation");
const route = ts.transpileModule(declaration.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const resolveInvocation = new Function("process", "readFileSync", "join", "WORKSPACE_ROOT", `${route}; return resolveNxInvocation;`)({ env: {} }, readFileSync, join, root);
const vectors = JSON.parse(readFileSync(join(modulePath, "🧫️invocations.json"), "utf8"));
for (const vector of vectors.valid) { const result = resolveInvocation(vector.input); assert.deepEqual(result.args, vector.args); assert.equal(result.watch, vector.watch); }
for (const vector of vectors.invalid) assert.throws(() => resolveInvocation(vector));
console.log("[DEBUG] Report root invocation and Nx watch selection PASS");
