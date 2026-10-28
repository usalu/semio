/** 🧪️ Reproducible source-backed strict check and independent XML typography fixture execution. */
import fs from "node:fs/promises";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";

/** 🔎️ Uses installed test libraries through the same owned XML port as the durable laws. */
export async function verifyOverrideTypescript() {
  const ticket = path.dirname(fileURLToPath(import.meta.url));
  let root = ticket;
  while (!(await fs.stat(path.join(root, "AGENTS.md")).catch(() => undefined))?.isFile()) {
    const parent = path.dirname(root);
    if (parent === root) throw new Error("repository root missing");
    root = parent;
  }
  const require = createRequire(path.join(root, "package.json"));
  const ts = require("typescript");
  const interchange = path.join(root, "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange");
  const text = path.join(interchange, "📦️idml/📎️overrides");
  const tests = path.join(text, "🧪️tests");
  const targets = [path.join(interchange, "🟦️.ts"), path.join(text, "🟦️.ts"), path.join(tests, "🟦️.ts")];
  const options = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, esModuleInterop: true, resolveJsonModule: true, types: ["node"] };
  const host = ts.createCompilerHost(options);
  host.getCurrentDirectory = () => root;
  const program = ts.createProgram(targets, options, host);
  const diagnostics = targets.flatMap((target) => { const source = program.getSourceFile(target); return [...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)]; });
  if (diagnostics.length) throw new Error(diagnostics.map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")).join("\n"));
  let xml;
  try { xml = require("fast-xml-parser"); } catch {
    for (const cache of [".bun", ".pnpm"]) {
      const directory = path.join(root, "node_modules", cache);
      const packages = await fs.readdir(directory).catch(() => []);
      const match = packages.find((name) => name.startsWith("fast-xml-parser@"));
      if (match) { xml = createRequire(path.join(directory, match, "node_modules/fast-xml-parser/package.json"))("fast-xml-parser"); break; }
    }
  }
  if (!xml) throw new Error("test-only fast-xml-parser is not installed");
  const adapter = await import(pathToFileURL(path.join(interchange, "📦️idml/🔤️text/🧪️tests/oracle.mjs")).href);
  const base = path.resolve(ticket, "🗑️generated", "override-ts-verification");
  const output = path.resolve(base, "run-" + Date.now().toString(36));
  await fs.mkdir(output, { recursive: true });
  const modules = new Map();
  async function compile(file) {
    file = path.resolve(file);
    if (modules.has(file)) return modules.get(file);
    const target = path.join(output, "m" + modules.size + ".cjs");
    if (target.length > 256) throw new Error("compiler output path exceeds repository limit");
    modules.set(file, target);
    const source = await fs.readFile(file, "utf8");
    if (file.endsWith(".json")) { await fs.writeFile(target, "module.exports = " + JSON.stringify(JSON.parse(source)) + ";\n"); return target; }
    let emitted = ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS, esModuleInterop: true } }).outputText;
    for (const match of [...emitted.matchAll(/require\("((?:\\.|[^"\\])*)"\)/g)]) {
      const specifier = JSON.parse('"' + match[1] + '"');
      if (!specifier.startsWith(".")) continue;
      const input = path.resolve(path.dirname(file), specifier);
      const resolved = /\.(ts|json)$/.test(input) ? await compile(input) : input;
      emitted = emitted.replaceAll(match[0], "require(" + JSON.stringify(resolved.replaceAll("\\", "/")) + ")");
    }
    await fs.writeFile(target, emitted);
    return target;
  }
  try {
    const law = require(await compile(path.join(tests, "🟦️.ts")));
    return { strictTargetDiagnostics: 0, actualSourceFiles: program.getSourceFiles().length, ...law.verifyIdmlOverrides(adapter.createTextXmlOracle(xml)), generatedModules: modules.size };
  } finally {
    const relative = path.relative(base, output);
    if (!relative || relative.startsWith("..") || path.isAbsolute(relative)) throw new Error("refused compiler output cleanup outside ticket verification directory");
    await fs.rm(output, { recursive: true, force: true });
    if ((await fs.readdir(base)).length === 0) await fs.rmdir(base);
  }
}
