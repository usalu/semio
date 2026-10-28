/** 🧪️ Reproducible source-backed strict check and independent XML typography fixture execution. */
import fs from "node:fs/promises";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";

/** 🔎️ Uses installed test libraries through the same owned XML port as the durable laws. */
export async function verifyLayoutSimpleDecision() {
  const ticket = path.dirname(fileURLToPath(import.meta.url));
  let root = ticket;
  while (!(await fs.stat(path.join(root, "AGENTS.md")).catch(() => undefined))?.isFile()) {
    const parent = path.dirname(root);
    if (parent === root) throw new Error("repository root missing");
    root = parent;
  }
  const require = createRequire(path.join(root, "package.json"));
  const ts = require("typescript");
  const text = path.join(root, "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🎚️simple");
  const tests = path.join(text, "🧪️tests");
  const targets = [path.join(text, "🟦️.ts"), path.join(tests, "🟦️.ts"), path.join(path.dirname(text), "🔺️output/🟦️.ts"), path.join(path.dirname(text), "📓️spread/🔺️output.ts")];
  const options = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, esModuleInterop: true, resolveJsonModule: true, types: ["node"] };
  const host = ts.createCompilerHost(options);
  host.getCurrentDirectory = () => root;
  const program = ts.createProgram(targets, options, host);
  const diagnostics = targets.flatMap((target) => { const source = program.getSourceFile(target); return [...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)]; });
  if (diagnostics.length) throw new Error(diagnostics.map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")).join("\n"));
  const jsonPatch = require("fast-json-patch");
  const base = path.resolve(ticket, "🗑️generated", "layout-simple-check");
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
      if (!specifier.startsWith(".") && !specifier.startsWith("@semio-tech/")) continue;
      const input = specifier.startsWith(".") ? path.resolve(path.dirname(file), specifier) : await fs.realpath(require.resolve(specifier, { paths: [path.dirname(file)] }));
      const resolved = /\.(ts|json)$/.test(input) ? await compile(input) : input;
      emitted = emitted.replaceAll(match[0], "require(" + JSON.stringify(resolved.replaceAll("\\", "/")) + ")");
    }
    await fs.writeFile(target, emitted);
    return target;
  }
  try {
    const law = require(await compile(path.join(tests, "🟦️.ts")));
    return { strictTargetDiagnostics: 0, actualSourceFiles: program.getSourceFiles().length, ...law.verifyLayoutSimpleDecision({apply(before,ops){return jsonPatch.applyPatch(structuredClone(before),structuredClone(ops),true,false).newDocument}}), generatedModules: modules.size };
  } finally {
    const relative = path.relative(base, output);
    if (!relative || relative.startsWith("..") || path.isAbsolute(relative)) throw new Error("refused compiler output cleanup outside ticket verification directory");
    await fs.rm(output, { recursive: true, force: true });
    if ((await fs.readdir(base)).length === 0) await fs.rmdir(base);
  }
}
