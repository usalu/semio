/** ✂️ For the given Vite configs: inline the relative graph like the config bundler, collect the workspace packages left to Node's strip-only loader,
 * and judge every file of their runtime closures with Node's own type stripper and TypeScript's erasableSyntaxOnly (same oracles as the node-native-typescript law).
 * Usage: `bun check-native-surface.ts <config>...` */
import { spawnSync } from "node:child_process";
import { readFileSync, realpathSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import ts from "typescript";

const root = "/Users/ueli/Documents/semio";
const configs = process.argv.slice(2).map((p) => realpathSync(resolve(root, p)));
const EXT = new Set([".ts", ".tsx", ".mts", ".js", ".mjs"]);
const STRIP = new Set([1294, 1484, 1485]);
const runtimeSpecifiers = (file: string): string[] => {
  const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, false, extname(file) === ".tsx" ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  return source.statements.flatMap((s) => {
    if (ts.isImportDeclaration(s) && !s.importClause?.isTypeOnly) return [(s.moduleSpecifier as ts.StringLiteral).text];
    if (ts.isExportDeclaration(s) && !s.isTypeOnly && s.moduleSpecifier) return [(s.moduleSpecifier as ts.StringLiteral).text];
    return [];
  }).filter((x) => !x.startsWith("node:") && !x.startsWith("bun:"));
};
const target = (spec: string, from: string): string | null => {
  try { const r = realpathSync(Bun.resolveSync(spec, dirname(from))); return r.startsWith(root + "/") && !r.includes("/node_modules/") ? r : null; } catch { return null; }
};
const owners = new Map<string, Set<string>>();
for (const config of configs) {
  const seen = new Set<string>(), queue = [config];
  while (queue.length) {
    const f = queue.pop()!;
    if (seen.has(f) || !EXT.has(extname(f))) continue;
    seen.add(f);
    for (const spec of runtimeSpecifiers(f)) {
      const t = target(spec, f);
      if (t === null) continue;
      if (spec.startsWith(".") || spec.startsWith("/")) queue.push(t);
      else (owners.get(relative(root, t)) ?? owners.set(relative(root, t), new Set()).get(relative(root, t))!).add(relative(root, config));
    }
  }
}
const parent = new Map<string, string>();
const closure = (entry: string): string[] => {
  const seen = new Set<string>(), queue = [realpathSync(join(root, entry))];
  while (queue.length) {
    const f = queue.shift()!;
    if (seen.has(f) || !EXT.has(extname(f))) continue;
    seen.add(f);
    for (const spec of runtimeSpecifiers(f)) { const t = target(spec, f); if (t !== null) { queue.push(t); if (!parent.has(t) && t !== f) parent.set(t, f); } }
  }
  return [...seen].sort();
};
const all = new Set<string>();
for (const entry of owners.keys()) for (const f of closure(entry)) all.add(f);
const files = [...all].sort();
const tsFiles = files.filter((f) => !/\.m?js$/.test(f));
const program = ts.createProgram(files, { noEmit: true, erasableSyntaxOnly: true, verbatimModuleSyntax: true, noResolve: true, noLib: true, types: [], target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, skipLibCheck: true });
const findings: string[] = [];
for (const file of tsFiles) {
  const source = program.getSourceFile(file)!;
  for (const d of [...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)]) if (STRIP.has(d.code)) findings.push(`${relative(root, file)}:${d.file ? d.file.getLineAndCharacterOfPosition(d.start ?? 0).line + 1 : 0} TS${d.code}`);
}
const nodeScript = 'import { stripTypeScriptTypes } from "node:module"; import { readFileSync } from "node:fs"; const files = JSON.parse(readFileSync(0, "utf8")); process.stdout.write(JSON.stringify(files.map((f) => { try { stripTypeScriptTypes(readFileSync(f, "utf8")); return null; } catch (e) { return String(e.message).split("\\n")[0]; } })));';
const ranTs = files.filter((f) => f.endsWith(".ts") || f.endsWith(".mts") || f.endsWith(".tsx"));
const run = spawnSync("node", ["--no-warnings", "--input-type=module", "-e", nodeScript], { input: JSON.stringify(ranTs), encoding: "utf8", maxBuffer: 1 << 28 });
const verdicts: (string | null)[] = run.status === 0 ? JSON.parse(run.stdout) : [];
const nodeRefusals = ranTs.map((f, i) => [f, verdicts[i]] as const).filter(([, v]) => v !== null && v !== undefined);
console.log(JSON.stringify({ configs: configs.map((c) => relative(root, c)), nativeEntries: [...owners].map(([e, by]) => ({ entry: e, by: [...by] })), closureFiles: files.length, tsFindings: findings, nodeRefusals: nodeRefusals.map(([f, v]) => { const chain = [relative(root, f)]; for (let c = f, n = 0; parent.has(c) && n < 25; n++) { c = parent.get(c)!; chain.push(relative(root, c)); } return `${relative(root, f)} :: ${v}\n      CHAIN ${chain.map((x) => x.split("/").slice(-3).join("/")).join(" <- ")}`; }), nodeStatus: run.status }, null, 1));
