/**
 * 🔗️ Finds every bare import of a workspace package (static, dynamic, export-from, require) whose subpath is not covered by that package's `exports`
 * (exact keys and `*` patterns), and every import of an unknown `@semio-tech/*` package.
 *
 *   bun glue-tools/check-importers.ts [--json]
 */
import ts from "typescript";
import { existsSync, readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { ROOT, workspaceManifests } from "./check-exports.ts";

type Pkg = { dir: string; exports: Record<string, unknown> | undefined; hasMain: boolean };
const packages = new Map<string, Pkg>();
for (const m of workspaceManifests()) {
  const json = JSON.parse(readFileSync(m, "utf8"));
  if (!json.name) continue;
  const exp = json.exports;
  const map = exp === undefined ? undefined : typeof exp === "string" || Array.isArray(exp) || Object.keys(exp).every(k => !k.startsWith(".")) ? { ".": exp } : exp;
  packages.set(json.name, { dir: dirname(m).slice(ROOT.length + 1), exports: map, hasMain: !!json.main });
}

const covered = (map: Record<string, unknown>, sub: string): boolean => {
  if (sub in map) return true;
  return Object.keys(map).some(k => { const i = k.indexOf("*"); return i >= 0 && sub.startsWith(k.slice(0, i)) && sub.endsWith(k.slice(i + 1)) && sub.length >= k.length - 1; });
};

const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: ROOT, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter(f => /\.(ts|tsx|mts|cts|mjs|js|cjs)$/u.test(f) && !f.startsWith(".🧬semio/") && !/(^|\/)(node_modules|target|dist|📤️dist|🗑️generated|🤖️generated|pkg|\.cache)\//u.test(f) && existsSync(resolve(ROOT, f)));
const names = [...packages.keys()].sort((a, b) => b.length - a.length);
const problems: { file: string; spec: string; reason: string }[] = [];
let checked = 0;
for (const file of files) {
  const text = readFileSync(resolve(ROOT, file), "utf8");
  if (!names.some(n => text.includes(n))) continue;
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, false, file.endsWith("x") ? ts.ScriptKind.TSX : undefined);
  const specs: string[] = [];
  const visit = (n: ts.Node): void => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) specs.push(n.moduleSpecifier.text);
    else if (ts.isCallExpression(n) && n.arguments.length >= 1 && ts.isStringLiteralLike(n.arguments[0]!) && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) specs.push((n.arguments[0] as ts.StringLiteral).text);
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) specs.push(n.argument.literal.text);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  for (const raw of specs) {
    const spec = raw.split("?")[0]!;
    const name = names.find(n => spec === n || spec.startsWith(n + "/"));
    if (!name) { if (/^@semio-tech\/[^/]+/u.test(spec) && !spec.startsWith("@semio-tech/") === false) { const scoped = spec.split("/").slice(0, 2).join("/"); if (!packages.has(scoped)) problems.push({ file, spec: raw, reason: "unknown workspace package" }); } continue; }
    checked++;
    const pkg = packages.get(name)!;
    const sub = "." + spec.slice(name.length);
    if (!pkg.exports) { if (sub !== "." || !pkg.hasMain) problems.push({ file, spec: raw, reason: `package ${name} has no exports` }); continue; }
    if (!covered(pkg.exports, sub)) problems.push({ file, spec: raw, reason: `no exports key ${sub} in ${name}` });
  }
}
const grouped = new Map<string, { reason: string; files: string[] }>();
for (const p of problems) { const g = grouped.get(p.spec) ?? grouped.set(p.spec, { reason: p.reason, files: [] }).get(p.spec)!; g.files.push(p.file); }
console.log(`source files ${files.length}, workspace packages ${packages.size}, bare workspace imports checked ${checked}, uncovered imports ${problems.length} (${grouped.size} distinct specifiers)`);
for (const [spec, g] of [...grouped].sort()) console.log(`${spec}  [${g.reason}]  importers ${g.files.length}: ${g.files.slice(0, 2).join(" | ")}`);
if (process.argv.includes("--json")) { const out = resolve(import.meta.dir, "../🗑️generated/glue/uncovered-imports.json"); mkdirSync(dirname(out), { recursive: true }); writeFileSync(out, JSON.stringify(problems, null, 1)); }
process.exit(problems.length ? 1 : 0);
