/** expc-d: synthesizes `no-cross-package-relative-*` edges for the given importer files (every relative import that lands inside another source owner). bun scan-edges.ts <out.json> <file...> */
import ts from "typescript";
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { GENERATED, ROOT, posix } from "../repoapi-tools/lib.ts";
import { packageIndex } from "../repoapi-tools/packages.ts";
mkdirSync(GENERATED, { recursive: true });
const [out, ...files] = process.argv.slice(2) as string[];
const index = packageIndex();
const SRC = [".ts", ".tsx", ".mts", ".js", ".mjs", ".cjs", ".json", "/🟦️.ts", "/🟦️.tsx", "/🟨️.mjs", "/🟨️.cjs"];
const isFile = (p: string): boolean => { try { return statSync(p).isFile(); } catch { return false; } };
const resolveRel = (from: string, spec: string): string | null => {
  const base = resolve(dirname(resolve(ROOT, from)), spec.split("?")[0]!);
  for (const c of [base, ...SRC.map(s => base + s), base.replace(/\.js$/u, ".ts")]) if (isFile(c)) return posix(relative(ROOT, c));
  return null;
};
const ownerOf = (target: string) => index.all.filter(p => target.startsWith(p.sourceOwner + "/")).sort((a, b) => b.sourceOwner.length - a.sourceOwner.length)[0];
const edges: { rule: string; from: string; to: string }[] = [];
for (const file of files) {
  const sf = ts.createSourceFile(file, readFileSync(resolve(ROOT, file), "utf8"), ts.ScriptTarget.Latest, true);
  const specs = new Set<string>();
  const visit = (n: ts.Node): void => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) specs.add(n.moduleSpecifier.text);
    else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) specs.add(n.arguments[0].text);
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) specs.add(n.argument.literal.text);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  const mine = ownerOf(file)?.sourceOwner;
  for (const spec of specs) {
    if (!spec.startsWith(".")) continue;
    const target = resolveRel(file, spec);
    if (!target) continue;
    const owner = ownerOf(target);
    if (!owner || owner.sourceOwner === mine) continue;
    if (target.startsWith(owner.dir + "/")) { const entry = Object.values(owner.exports).some(v => typeof v === "string" && posix(resolve(ROOT, owner.dir, v)) === posix(resolve(ROOT, target))); if (entry) continue; }
    edges.push({ rule: "no-cross-package-relative-" + owner.name, from: file, to: target });
  }
}
writeFileSync(out!, JSON.stringify({ ok: true, edges }, null, 1));
console.log(edges.length, "edges");
