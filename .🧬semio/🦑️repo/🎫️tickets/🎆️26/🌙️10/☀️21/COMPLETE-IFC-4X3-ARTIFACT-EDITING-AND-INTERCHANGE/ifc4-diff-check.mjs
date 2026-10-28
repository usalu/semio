import fs from "node:fs/promises";
import path from "node:path";
import {pathToFileURL} from "node:url";
import {createRequire} from "node:module";

/** 🧪️ Executes the checked-in IFC TypeScript law tests and strict compiler validation. */
export async function runDiffChecks(root, output) {
  const require = createRequire(path.join(root, "package.json")), ts = require("typescript");
  const owner = path.join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff");
  const source = path.join(owner, "🟦️.ts"), tests = path.join(owner, "🧪️tests/🟦️.test.ts");
  const options = {strict: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.NodeNext, moduleResolution: ts.ModuleResolutionKind.NodeNext, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: ["node"], typeRoots: [path.join(root, "node_modules/@types")], esModuleInterop: true};
  const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram([source, tests], options)).map(item => ({file: item.file?.fileName, message: ts.flattenDiagnosticMessageText(item.messageText, "\n")}));
  await fs.mkdir(output, {recursive: true});
  const id = Date.now(), modelName = "diff-" + id + ".mjs", testsName = "laws-" + id + ".mjs";
  const compile = text => ts.transpileModule(text, {compilerOptions: {target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ES2022}}).outputText;
  await fs.writeFile(path.join(output, modelName), compile(await fs.readFile(source, "utf8")));
  await fs.writeFile(path.join(output, testsName), compile(await fs.readFile(tests, "utf8")).replace('"../🟦️.ts"', '"./' + modelName + '"'));
  const execution = await import(pathToFileURL(path.join(output, testsName)).href);
  await Promise.all(execution.checks);
  return {diagnostics, tests: execution.records, metrics: execution.metrics, referenceLibrary: "fast-json-patch " + require("fast-json-patch/package.json").version};
}
