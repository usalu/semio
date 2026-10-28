/** 🧪️ Typecheck and execute the permanent IFC4 geometry source suite. */
import fs from "node:fs/promises";
import path from "node:path";
import {createRequire, stripTypeScriptTypes} from "node:module";
import {pathToFileURL} from "node:url";
export async function runIfc2DetailsChecks(root, ticket) {
  const subset = path.join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any"), owner = path.join(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪟️details"), source = path.join(owner, "🟦️.ts"), test = path.join(owner, "🧪️tests/🟦️.test.ts");
  const require = createRequire(path.join(root, "package.json")), ts = require("typescript");
  const config = ts.readConfigFile(path.join(root, "tsconfig.json"), ts.sys.readFile), options = ts.convertCompilerOptionsFromJson(config.config.compilerOptions, root).options;
  const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram([source, test], options)).map(item => ({file: item.file?.fileName, message: ts.flattenDiagnosticMessageText(item.messageText, "\n")}));
  const output = path.join(ticket, "🗑️generated/ifc2-details-check-" + Date.now());
  await fs.mkdir(output, {recursive: true});
  const cache = new Map();
  async function compile(sourcePath) {
    sourcePath = path.resolve(sourcePath);
    if (sourcePath.endsWith(".json")) {
      const target = path.join(output, "asset" + cache.size + ".mjs"); cache.set(sourcePath, target);
      await fs.writeFile(target, "export default " + await fs.readFile(sourcePath, "utf8")); return target;
    }
    if (cache.has(sourcePath)) return cache.get(sourcePath);
    const target = path.join(output, "module" + cache.size + ".mjs"); cache.set(sourcePath, target);
    let source = stripTypeScriptTypes(await fs.readFile(sourcePath, "utf8"), {mode: "transform"});
    for (const match of [...source.matchAll(/from\s*["']([^"']+)["']/g)]) {
      let dependency = match[1];
      if (dependency === "@semio-tech/framework/value/ieee754") dependency = path.join(root, "🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts");
      else if (dependency.startsWith(".")) dependency = path.resolve(path.dirname(sourcePath), dependency);
      else continue;
      source = source.replace(match[0], "from " + JSON.stringify(pathToFileURL(await compile(dependency)).href));
    }
    source = source.replace(/with\s*\{\s*type:\s*["']json["']\s*\}/g, "");
    source = source.replaceAll("import.meta.url", JSON.stringify(pathToFileURL(sourcePath).href));
    await fs.writeFile(target, source); return target;
  }
  const execution = await import(pathToFileURL(await compile(test)).href);
  await Promise.all(execution.checks);
  if (execution.records.some(record => !record.passed)) return {diagnostics, tests: execution.records, passed: false, transformedModules: cache.size, output};
  return {diagnostics, tests: execution.records, transformedModules: cache.size, output};
}
