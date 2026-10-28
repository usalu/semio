/** 🧪️ Executes the checked-in IFC4 TypeScript suite without changing production module boundaries. */
import fs from "node:fs/promises";
import path from "node:path";
import {createRequire, stripTypeScriptTypes} from "node:module";
import {fileURLToPath, pathToFileURL} from "node:url";

export async function checkIfc4TypeScript() {
  const root = fileURLToPath(new URL("../../../../../../../", import.meta.url));
  const ticket = path.dirname(fileURLToPath(import.meta.url));
  const subset = path.join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any");
  const parser = path.join(root, "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🟦️.ts");
  const test = path.join(subset, "🧪️tests/🔬️step-io/🟦️.ts");
  const require = createRequire(path.join(root, "package.json"));
  const ts = require("typescript");
  const config = ts.readConfigFile(path.join(root, "tsconfig.json"), ts.sys.readFile);
  const options = ts.convertCompilerOptionsFromJson(config.config.compilerOptions, root).options;
  const sources = [parser, path.join(subset, "🚪️io/🟦️.ts"), test];
  for (const direction of ["📥️import/🧩️deserializers", "📤️export/🧵️serializers"]) for (const format of ["🔤️txt/🔖️utf-8", "💾️binary/🔖️raw"]) sources.push(path.join(subset, "🚪️io", direction, "🗿️artifacts", format, "✳️any/🟦️.ts"));
  const errors = ts.getPreEmitDiagnostics(ts.createProgram(sources, options));
  if (errors.length) throw new Error(ts.formatDiagnosticsWithColorAndContext(errors, {getCanonicalFileName: file => file, getCurrentDirectory: () => root, getNewLine: () => "\n"}));
  const output = path.join(ticket, "🗑️generated/ifc4-source-check-" + Date.now());
  await fs.mkdir(output, {recursive: true});
  const cache = new Map();
  async function compile(sourcePath) {
    sourcePath = path.resolve(sourcePath);
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
    source = source.replaceAll("import.meta.url", JSON.stringify(pathToFileURL(sourcePath).href));
    await fs.writeFile(target, source); return target;
  }
  const checks = await import(pathToFileURL(await compile(test)).href);
  const passed = [];
  for (const name of ["checkOfficial", "checkBoundaries", "checkScalars", "checkLimits", "checkOptionalComplexAndProgress"]) {checks[name](); passed.push(name);}
  return {typecheck: "passed", passed, transformedModules: cache.size};
}
