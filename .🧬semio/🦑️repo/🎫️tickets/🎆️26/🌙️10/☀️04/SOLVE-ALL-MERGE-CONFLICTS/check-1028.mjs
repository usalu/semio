import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../..");
const require = createRequire(resolve(root, "package.json"));
const ts = require("typescript");
const Ajv = require("ajv").default;
const contract = JSON.parse(readFileSync(resolve(import.meta.dir, "merge-1028.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"), "utf8"));
const project = JSON.parse(readFileSync(resolve(root, contract.files[1]), "utf8"));
const validate = new Ajv({ strict: false }).compile(schema);
assert.ok(validate(project), JSON.stringify(validate.errors));
assert.deepEqual(project.targets.build.metadata.semio.closure.modules, contract.buildModules);
const includes = [];
for (const name of [contract.files[0], contract.files[2]]) {
  const source = ts.createSourceFile(name, readFileSync(resolve(root, name), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  assert.deepEqual(source.parseDiagnostics.map(d => ts.flattenDiagnosticMessageText(d.messageText, "\n")), []);
  const visit = node => {
    if (name === contract.files[2] && ts.isPropertyAssignment(node) && node.name.getText(source) === "include") {
      assert.ok(ts.isArrayLiteralExpression(node.initializer));
      for (const item of node.initializer.elements) includes.push(item.text);
    }
    if (name === contract.files[0] && ts.isImportDeclaration(node) && node.moduleSpecifier.text === contract.routingImport) {
      assert.ok(readFileSync(resolve(dirname(resolve(root, name)), node.moduleSpecifier.text), "utf8"));
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
}
assert.deepEqual(includes, contract.testIncludes);
console.log("PASS: TypeScript syntax and AST test union, Ajv project schema, and merged build closure");
