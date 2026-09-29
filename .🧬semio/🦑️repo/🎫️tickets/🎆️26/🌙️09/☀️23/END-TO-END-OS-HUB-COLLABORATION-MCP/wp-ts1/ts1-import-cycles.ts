/** 🔁️ TS1: static-import cycles reachable from one module (relative + workspace `@semio-tech/*` packages resolved through the vitest aliases). */
import ts from "typescript";
import { resolve, relative } from "node:path";
const repo = "/Users/ueli/Documents/semio";
const [configPath, entry, target] = process.argv.slice(2);
const parsed = ts.getParsedCommandLineOfConfigFile(resolve(configPath!), {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: () => {} })!;
const host = ts.createCompilerHost(parsed.options);
const edges = new Map<string, string[]>();
const imports = (file: string): string[] => {
  const cached = edges.get(file); if (cached) return cached;
  const source = ts.createSourceFile(file, ts.sys.readFile(file) ?? "", ts.ScriptTarget.ESNext, true);
  const out: string[] = [];
  for (const statement of source.statements) {
    if (!(ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) || !statement.moduleSpecifier) continue;
    if (ts.isImportDeclaration(statement) && statement.importClause?.isTypeOnly) continue;
    if (ts.isExportDeclaration(statement) && statement.isTypeOnly) continue;
    if (ts.isImportDeclaration(statement) && statement.importClause && !statement.importClause.name && statement.importClause.namedBindings && ts.isNamedImports(statement.importClause.namedBindings) && statement.importClause.namedBindings.elements.every((e) => e.isTypeOnly)) continue;
    const spec = (statement.moduleSpecifier as ts.StringLiteral).text;
    const resolved = ts.resolveModuleName(spec, file, parsed.options, host).resolvedModule?.resolvedFileName;
    if (resolved && !resolved.includes("node_modules") && /\.(ts|tsx)$/.test(resolved) && !resolved.endsWith(".d.ts")) out.push(resolved);
  }
  edges.set(file, out); return out;
};
const start = resolve(entry!), goal = resolve(target ?? entry!);
const seen = new Set<string>(); const stack: string[] = [];
const paths: string[][] = [];
const dfs = (file: string): void => {
  if (paths.length >= 5) return;
  stack.push(file);
  for (const next of imports(file)) {
    if (next === goal && stack.length > 0) { paths.push([...stack, next]); continue; }
    if (seen.has(next)) continue; seen.add(next); dfs(next);
  }
  stack.pop();
};
seen.add(start); dfs(start);
for (const path of paths) console.log(path.map((f) => relative(repo, f).replace(/^.*?(✏️editor\/⚙️engine\/|spatial-kernel\/⚙️engine\/|🧩️extensions\/|🧰️framework\/)/, "")).join("\n  → ") + "\n");
if (!paths.length) console.log("no cycle");
