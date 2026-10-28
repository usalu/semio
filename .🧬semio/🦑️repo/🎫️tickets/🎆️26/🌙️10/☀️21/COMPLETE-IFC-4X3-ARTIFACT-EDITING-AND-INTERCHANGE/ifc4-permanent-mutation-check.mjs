/** 🧪️ Executes the actual checked-in mutation test callbacks through transformed owned modules. */
import fs from "node:fs/promises";
import path from "node:path";
import {createRequire} from "node:module";
import {pathToFileURL} from "node:url";
import assert from "node:assert/strict";

export async function run(root, ticket, relativeTest = "🧬️schema/🧬️mutations/🧪️tests/🟦️.test.ts", expectedGroups = 4) {
  const require = createRequire(path.join(root, "package.json")), ts = require("typescript");
  const subset = path.join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any");
  const testPath = path.join(subset, relativeTest);
  const configuration = JSON.parse(await fs.readFile(path.join(root, "tsconfig.json"), "utf8"));
  const options = ts.convertCompilerOptionsFromJson(configuration.compilerOptions, root).options;
  const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram([testPath], options));
  assert.equal(diagnostics.length, 0, diagnostics.map(diagnostic => ts.flattenDiagnosticMessageText(diagnostic.messageText, " ")).join("\n"));
  const output = path.join(ticket, "🗑️generated", "ifc4-mutation-source-" + Date.now());
  await fs.mkdir(output, {recursive: true});
  const files = [], cache = new Map();
  const registryPath = path.join(output, "registry.mjs"), patchPath = path.join(output, "patch.mjs"), graphqlPath = path.join(output, "graphql.mjs"), protobufPath = path.join(output, "protobuf.mjs");
  try {
    await fs.writeFile(registryPath, "export const checks=[]; export function test(name, callback){checks.push([name, callback]);}\n"); files.push(registryPath);
    await fs.writeFile(patchPath, 'import {createRequire} from "node:module"; const patch=createRequire(' + JSON.stringify(pathToFileURL(path.join(root, "package.json")).href) + ')("fast-json-patch"); export const applyPatch=patch.applyPatch;\n'); files.push(patchPath);
    const requireSource = 'import {createRequire} from "node:module"; const require=createRequire(' + JSON.stringify(pathToFileURL(path.join(root, "package.json")).href) + ');\n';
    await fs.writeFile(graphqlPath, requireSource + 'const lib=require("graphql"); export const {buildSchema,GraphQLObjectType,GraphQLScalarType,Kind,parseValue,validateSchema,valueFromAST}=lib;\n'); files.push(graphqlPath);
    await fs.writeFile(protobufPath, requireSource + 'export default require("protobufjs");\n'); files.push(protobufPath);
    async function compile(sourcePath) {
      sourcePath = path.resolve(sourcePath);
      if (cache.has(sourcePath)) return cache.get(sourcePath);
      const target = path.join(output, "module" + cache.size + ".mjs"); cache.set(sourcePath, target);
      const result = ts.transpileModule(await fs.readFile(sourcePath, "utf8"), {compilerOptions: {module: ts.ModuleKind.ESNext, target: ts.ScriptTarget.ES2022}, reportDiagnostics: true});
      assert.equal(result.diagnostics.length, 0);
      let source = result.outputText;
      for (const match of [...source.matchAll(/from\s*["']([^"']+)["']/g)]) {
        let targetPath;
        if (match[1] === "node:test") targetPath = registryPath;
        else if (match[1] === "fast-json-patch") targetPath = patchPath;
        else if (match[1] === "graphql") targetPath = graphqlPath;
        else if (match[1] === "protobufjs") targetPath = protobufPath;
        else if (match[1].startsWith(".")) targetPath = await compile(path.resolve(path.dirname(sourcePath), match[1]));
        else continue;
        source = source.replace(match[0], "from " + JSON.stringify(pathToFileURL(targetPath).href));
      }
      source = source.replaceAll("import.meta.url", JSON.stringify(pathToFileURL(sourcePath).href));
      await fs.writeFile(target, source); files.push(target); return target;
    }
    await import(pathToFileURL(await compile(testPath)).href);
    const registry = await import(pathToFileURL(registryPath).href), passed = [];
    assert.equal(registry.checks.length, expectedGroups);
    for (const [name, callback] of registry.checks) { await callback(); passed.push(name); }
    return {typecheck: "passed", groups: passed.length, passed, transformedModules: cache.size};
  } finally { for (const file of files.reverse()) await fs.unlink(file); await fs.rmdir(output); }
}
