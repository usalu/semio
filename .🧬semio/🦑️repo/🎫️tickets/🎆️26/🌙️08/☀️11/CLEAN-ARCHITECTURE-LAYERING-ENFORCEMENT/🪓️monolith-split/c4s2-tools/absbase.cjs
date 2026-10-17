// usage: node absbase.cjs <original-repo-path> <snapshot> <out-dir>  -> writes <out-dir>/<basename(dir)>-<ext copy> whose relative specifiers are absolute, for type-checking a pre-split snapshot from outside the repo
const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const [orig, snap, outDir] = process.argv.slice(2);
const text = fs.readFileSync(snap, "utf8");
const sf = ts.createSourceFile(orig, text, ts.ScriptTarget.Latest, true, orig.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const edits = [];
const abs = (s) => (s.startsWith(".") ? path.resolve(path.dirname(orig), s) : s);
const visit = (n) => {
  if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) edits.push([n.moduleSpecifier.getStart(sf) + 1, n.moduleSpecifier.getEnd() - 1, abs(n.moduleSpecifier.text)]);
  else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) edits.push([n.argument.literal.getStart(sf) + 1, n.argument.literal.getEnd() - 1, abs(n.argument.literal.text)]);
  else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0])) edits.push([n.arguments[0].getStart(sf) + 1, n.arguments[0].getEnd() - 1, abs(n.arguments[0].text)]);
  ts.forEachChild(n, visit);
};
visit(sf);
let out = text;
for (const [a, b, rep] of edits.sort((x, y) => y[0] - x[0])) out = out.slice(0, a) + rep + out.slice(b);
fs.mkdirSync(outDir, { recursive: true });
const target = path.join(outDir, path.basename(path.dirname(orig)) + path.extname(orig));
fs.writeFileSync(target, out);
console.log(target);
