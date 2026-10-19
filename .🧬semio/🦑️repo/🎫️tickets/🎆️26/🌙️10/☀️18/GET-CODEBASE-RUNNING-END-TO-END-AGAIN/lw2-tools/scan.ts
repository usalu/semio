import { readFileSync, readdirSync, statSync, existsSync, realpathSync, writeFileSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import ts from "typescript";

const ROOT = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", "dist", "target", "pkg", ".cache", "🗑️generated"]);
const sdkPkg = JSON.parse(readFileSync(join(ROOT, "🧰️framework/📦️packages/🟦️typescript/package.json"), "utf8"));
const sdkDir = join(ROOT, "🧰️framework/📦️packages/🟦️typescript");
const sdkTargets = new Set<string>();
const walkExports = (v: any) => { if (typeof v === "string") sdkTargets.add(resolve(sdkDir, v)); else if (v && typeof v === "object") Object.values(v).forEach(walkExports); };
walkExports(sdkPkg.exports);

const scope = process.argv[2] ?? "✏️s/🔌️plugins";
const files: string[] = [];
const walk = (d: string) => {
  for (const e of readdirSync(d, { withFileTypes: true })) {
    if (SKIP.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (/\.(ts|tsx|mts|cts|js|mjs|cjs|jsx)$/u.test(e.name) && !e.name.endsWith(".d.ts")) files.push(p);
  }
};
walk(join(ROOT, scope));

const specs = (src: string, file: string): string[] => {
  const sf = ts.createSourceFile(file, src, ts.ScriptTarget.Latest, false);
  const out: string[] = [];
  const visit = (n: ts.Node) => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) out.push(n.moduleSpecifier.text);
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) out.push((n.arguments[0] as ts.StringLiteral).text);
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) out.push(n.argument.literal.text);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return out;
};

const rows: any[] = [];
for (const f of files) {
  let src: string;
  try { src = readFileSync(f, "utf8"); } catch { continue; }
  for (const s of specs(src, f)) {
    let target: string | null = null;
    if (s.startsWith(".")) target = resolve(dirname(f), s);
    else if (s.startsWith("@semio-tech/")) {
      try { target = realpathSync(Bun.resolveSync(s, dirname(f))); } catch { target = null; }
    } else continue;
    if (!target) { if (s.startsWith("@semio-tech/")) rows.push({ from: relative(ROOT, f), spec: s, to: null }); continue; }
    const rel = relative(ROOT, target);
    if (!rel.startsWith("🧰️framework/")) continue;
    rows.push({ from: relative(ROOT, f), spec: s, to: rel, sdk: sdkTargets.has(target) });
  }
}
writeFileSync(process.argv[3] ?? "/dev/stdout", JSON.stringify(rows, null, 1));
console.error(files.length, "files", rows.length, "edges to framework");
