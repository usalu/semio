/** 📊 Static oracle for rule `ui-no-framework-packages`: lists every (UI file → framework file) import edge without running dependency-cruiser.
 * Usage: bun 📜️script.ts   (run from the repository root; prints `from → to` lines, then a per-folder summary on stderr)
 */
import ts from "typescript";
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";
import { execFileSync } from "node:child_process";

const repo = process.cwd();
const UI = "🧰️framework/🔨️modules/🖱️ui/";
const git = (args: string[]) => execFileSync("git", ["-c", "core.fsmonitor=false", "-c", "core.quotepath=false", ...args], { cwd: repo, maxBuffer: 1 << 30 }).toString("utf8").split("\n").filter(Boolean);
const tracked = new Set([...git(["ls-files", "--", "🧰️framework", "✏️s"]), ...git(["ls-files", "--others", "--exclude-standard", "--", "🧰️framework", "✏️s"])]);
const live = (p: string) => existsSync(resolve(repo, p));
const packages = new Map<string, string>();
for (const manifest of [...tracked].filter(p => p.endsWith("/package.json") && !p.includes("node_modules/"))) {
  try {
    const json = JSON.parse(readFileSync(resolve(repo, manifest), "utf8")) as { name?: string; exports?: Record<string, unknown> | string; main?: string };
    if (!json.name?.startsWith("@semio-tech/")) continue;
    const exp = typeof json.exports === "string" ? json.exports : (json.exports?.["."] as unknown);
    const target = typeof exp === "string" ? exp : typeof (exp as { default?: string } | undefined)?.default === "string" ? (exp as { default: string }).default : json.main;
    if (target) packages.set(json.name, relative(repo, resolve(dirname(resolve(repo, manifest)), target)).split(sep).join("/"));
  } catch {}
}
const kindOf = (p: string) => (p.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const resolveRel = (from: string, spec: string): string | null => {
  const base = relative(repo, resolve(dirname(resolve(repo, from)), spec)).split(sep).join("/");
  for (const c of [base, `${base}.ts`, `${base}.tsx`, `${base}/🟦️.ts`, `${base}/🟦️.tsx`]) if (live(c) && statSync(resolve(repo, c)).isFile()) return c;
  return null;
};
const sourceFiles = [...tracked].filter(p => p.startsWith(UI) && /\.(ts|tsx)$/.test(p) && !p.includes("node_modules/") && !p.includes("🗑️generated/") && live(p));
const edges = new Set<string>();
for (const file of sourceFiles) {
  const sf = ts.createSourceFile(file, readFileSync(resolve(repo, file), "utf8"), ts.ScriptTarget.ESNext, true, kindOf(file));
  const specs: string[] = [];
  const visit = (n: ts.Node) => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) specs.push(n.moduleSpecifier.text);
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) specs.push(n.argument.literal.text);
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0])) specs.push(n.arguments[0].text);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  for (const spec of specs) {
    let target: string | null = null;
    if (spec.startsWith(".")) target = resolveRel(file, spec);
    else if (packages.has(spec)) target = packages.get(spec)!;
    else if (spec.startsWith("@semio-tech/")) { const [scope, name] = spec.split("/"); const key = `${scope}/${name}`; if (packages.has(key)) target = packages.get(key)!; }
    if (!target) continue;
    if (target.startsWith("🧰️framework/") && !target.startsWith(UI) && !spec.startsWith("@semio-tech/ui-")) edges.add(`${file} → ${target}`);
  }
}
const sorted = [...edges].sort();
console.log(sorted.join("\n"));
const folders = new Map<string, number>();
for (const e of sorted) { const rest = e.split(" → ")[0]!.slice(UI.length).split("/")[0]!; folders.set(rest, (folders.get(rest) ?? 0) + 1); }
console.error([...folders].sort().map(([k, v]) => `${v}\t${k}`).join("\n"));
console.error(`total ${sorted.length}`);
