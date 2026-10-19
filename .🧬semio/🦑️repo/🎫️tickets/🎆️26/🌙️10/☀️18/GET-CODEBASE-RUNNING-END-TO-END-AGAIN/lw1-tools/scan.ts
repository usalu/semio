import { readdirSync, readFileSync, statSync, existsSync, lstatSync } from "node:fs";
import { join, dirname, resolve, relative } from "node:path";
import ts from "typescript";
const ROOT = "/Users/ueli/Documents/semio";
const SDKDIR = join(ROOT, "🧰️framework/📦️packages/🟦️typescript");
const sdkPkg = JSON.parse(readFileSync(join(SDKDIR, "package.json"), "utf8"));
const sdkTargets = new Map<string, string>();
for (const [k, v] of Object.entries<any>(sdkPkg.exports)) if (typeof v === "string") sdkTargets.set(resolve(SDKDIR, v), k);
const skip = new Set(["node_modules", "target", "dist", "pkg", ".git", "🗑️generated"]);
function* walk(d: string): Generator<string> {
  for (const e of readdirSync(d, { withFileTypes: true })) {
    if (skip.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) yield* walk(p);
    else if (/\.(ts|tsx|mts|cts|mjs|js|cjs)$/u.test(e.name)) yield p;
  }
}
const fwPkgNames = new Map<string, string>();
function loadPkgs(dir: string, depth = 0) {
  if (depth > 12) return;
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    if (skip.has(e.name) || !e.isDirectory()) continue;
    const p = join(dir, e.name);
    const pj = join(p, "package.json");
    if (existsSync(pj)) { try { const j = JSON.parse(readFileSync(pj, "utf8")); if (j.name) fwPkgNames.set(j.name, p); } catch {} }
    loadPkgs(p, depth + 1);
  }
}
loadPkgs(join(ROOT, "🧰️framework"));
export type Edge = { from: string; spec: string; target: string; kind: "relative" | "bare"; ok: boolean; pkg?: string };
function specifiersOf(file: string, txt: string): string[] {
  const sf = ts.createSourceFile(file, txt, ts.ScriptTarget.Latest, false, /\.tsx$/u.test(file) ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const out: string[] = [];
  const visit = (n: ts.Node) => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) out.push(n.moduleSpecifier.text);
    else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) out.push((n.arguments[0] as ts.StringLiteralLike).text);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return out;
}
export function scan(scopeDir: string): Edge[] {
  const out: Edge[] = [];
  for (const f of walk(scopeDir)) {
    let txt: string; try { txt = readFileSync(f, "utf8"); } catch { continue; }
    if (!/framework|semio-tech/u.test(txt)) continue;
    for (const spec of specifiersOf(f, txt)) {
      if (spec.startsWith(".")) {
        const t = resolve(dirname(f), spec);
        const rel = relative(ROOT, t);
        if (!rel.startsWith("🧰️framework/")) continue;
        out.push({ from: relative(ROOT, f), spec, target: rel, kind: "relative", ok: sdkTargets.has(t) });
      } else if (spec.startsWith("@semio-tech/")) {
        const parts = spec.split("/"); const name = parts.slice(0, 2).join("/");
        const dir = fwPkgNames.get(name);
        if (!dir || name === "@semio-tech/framework") continue;
        out.push({ from: relative(ROOT, f), spec, target: relative(ROOT, dir), kind: "bare", ok: false, pkg: name });
      }
    }
  }
  return out;
}
if (import.meta.main) {
  const edges = scan(join(ROOT, process.argv[2] ?? "✏️s/🔌️plugins")).filter(e => !e.ok);
  console.log(JSON.stringify(edges));
}
