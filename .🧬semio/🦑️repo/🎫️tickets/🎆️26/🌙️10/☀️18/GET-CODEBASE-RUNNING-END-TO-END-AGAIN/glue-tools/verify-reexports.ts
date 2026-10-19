/**
 * 🧪️ Statically checks every exports target: each relative `export ... from` / `import ... from` specifier resolves to a file and
 * every explicitly named member exists in that module (following `export *`). Bun's bundler accepts missing re-exported names, this check does not.
 *
 *   bun glue-tools/verify-reexports.ts
 */
import ts from "typescript";
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { exportsOf } from "../repoapi-tools/exports.ts";
import { ROOT, listTargets, markBuildOutputs } from "./check-exports.ts";

const isFile = (p: string): boolean => existsSync(p) && statSync(p).isFile();
const resolveStrict = (from: string, spec: string): string | undefined => {
  const base = resolve(dirname(from), spec.split("?")[0]!);
  return [base, base.replace(/\.js$/u, ".ts"), base.replace(/\.js$/u, ".tsx"), base.replace(/\.mjs$/u, ".mts")].find(isFile);
};

const problems: string[] = [];
const opaque = new Set<string>();
const all = listTargets();
markBuildOutputs(all);
const files = [...new Set(all.filter(t => t.exists && /\.(ts|tsx|mts|js|mjs|cjs)$/u.test(t.file) && !t.target.includes("*") && !t.buildOutput && !/(^|\/)(dist|🕸️bindings)\//u.test(t.file)).map(t => resolve(ROOT, t.file)))];
let reexports = 0;
for (const file of files) {
  const text = readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true);
  const shown = file.slice(ROOT.length + 1);
  for (const st of sf.statements) {
    const spec = (ts.isExportDeclaration(st) || ts.isImportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) ? st.moduleSpecifier.text : undefined;
    if (!spec || !spec.startsWith(".")) continue;
    reexports++;
    const target = resolveStrict(file, spec);
    if (!target) { problems.push(`${shown}: unresolved ${spec}`); continue; }
    if (target.endsWith(".json") || /\?/u.test(spec) || /🕸️bindings\//u.test(spec)) continue;
    if (/^export\b[^;]*\bfrom\s*"(?![./])/mu.test(readFileSync(target, "utf8"))) { opaque.add(shown); continue; }
    const have = exportsOf(target);
    const names: string[] = [];
    if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) names.push((e.propertyName ?? e.name).text);
    if (ts.isImportDeclaration(st) && st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings)) for (const e of st.importClause.namedBindings.elements) names.push((e.propertyName ?? e.name).text);
    for (const n of names) if (!have.values.has(n) && !have.types.has(n)) problems.push(`${shown}: ${n} not exported by ${spec}`);
  }
}
console.log(`targets ${files.length}, relative re-export/import specifiers ${reexports}, name checks skipped because the target re-exports from a bare package ${opaque.size} glue files, problems ${problems.length}`);
if (process.argv.includes("--list-opaque")) for (const f of opaque) console.log(f);
const byFile = new Map<string, string[]>();
for (const p of problems) { const [f, rest] = p.split(": ", 2) as [string, string]; (byFile.get(f) ?? byFile.set(f, []).get(f)!).push(p.slice(f.length + 2)); }
for (const [f, list] of byFile) console.log(`${f}\n   ${list.length} problems, first: ${list.slice(0, 3).join(" | ")}`);
process.exit(problems.length ? 1 : 0);
