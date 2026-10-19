import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
const ROOT = "/Users/ueli/Documents/semio/";
const [, , specPath, mode] = process.argv;
const spec = JSON.parse(readFileSync(specPath!, "utf8"));
const root = JSON.parse(readFileSync(ROOT + "package.json", "utf8"));
const published = new Map<string, Set<string>>();
for (const w of root.workspaces as string[]) {
  const f = ROOT + w + "/package.json";
  if (!existsSync(f)) continue;
  const j = JSON.parse(readFileSync(f, "utf8"));
  if (spec.byPackage[j.name]) published.set(j.name, new Set(Object.keys(j.exports ?? {})));
}
const dry = mode !== "apply";
let rewritten = 0, pending = 0;
const edits = new Map<string, { start: number; end: number; text: string }[]>();
for (const [pkg, rows] of Object.entries<any>(spec.byPackage)) {
  const have = published.get(pkg) ?? new Set<string>();
  for (const row of Object.values<any>(rows)) {
    if (!have.has(row.proposedSubpath)) { pending += row.importers.length; continue; }
    const text = pkg + row.proposedSubpath.slice(1);
    for (const importer of row.importers as string[]) {
      const abs = ROOT + importer;
      const sf = ts.createSourceFile(abs, readFileSync(abs, "utf8"), ts.ScriptTarget.Latest, true);
      const visit = (n: ts.Node): void => {
        let lit: ts.StringLiteralLike | undefined;
        if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) lit = n.moduleSpecifier;
        else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) lit = n.arguments[0];
        else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) lit = n.argument.literal;
        if (lit && lit.text.startsWith(".") && resolve(dirname(abs), lit.text) === ROOT + row.targetFile) {
          const list = edits.get(abs) ?? []; list.push({ start: lit.getStart(sf) + 1, end: lit.getEnd() - 1, text }); edits.set(abs, list); rewritten++;
        }
        ts.forEachChild(n, visit);
      };
      visit(sf);
    }
  }
}
if (!dry) for (const [file, list] of edits) {
  let s = readFileSync(file, "utf8");
  for (const e of list.sort((a, b) => b.start - a.start)) s = s.slice(0, e.start) + e.text + s.slice(e.end);
  writeFileSync(file, s);
}
console.log(JSON.stringify({ mode: dry ? "dry" : "apply", rewritten, pendingEntries: pending, files: edits.size }));
