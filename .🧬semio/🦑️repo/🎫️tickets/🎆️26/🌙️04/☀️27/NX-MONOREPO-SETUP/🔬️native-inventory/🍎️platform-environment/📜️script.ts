import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
const workspace = process.env.SEMIO_REPO_ROOT!, require = createRequire(import.meta.url);
const fixture = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/nx-contract");
const vectors = JSON.parse(readFileSync(join(fixture, "🔣️.json"), "utf8"));
assert.equal(require("jsonschema").validate(vectors, JSON.parse(readFileSync(join(fixture, "🛂️schema/🔣️.json"), "utf8"))).valid, true);
const ts = require("typescript"), source = ts.createSourceFile("owner.ts", readFileSync(join(workspace, vectors.platformEnvironment.source), "utf8"), ts.ScriptTarget.Latest, true);
const selector = source.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "ensureAppleDeveloperDir");
assert.ok(selector);
const code = ts.transpileModule(selector.getText(source), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
for (const platform of vectors.platformEnvironment.platforms) {
  const host = { platform, env: {} };
  new Function("process", "existsSync", code + "; ensureAppleDeveloperDir();")(host, () => true);
  assert.deepEqual(Object.keys(host.env).sort(), platform === "darwin" ? [...vectors.platformEnvironment.keys].sort() : []);
}
console.log("[DEBUG] Current Apple tooling owner matches all neutral platform environment vectors PASS");
