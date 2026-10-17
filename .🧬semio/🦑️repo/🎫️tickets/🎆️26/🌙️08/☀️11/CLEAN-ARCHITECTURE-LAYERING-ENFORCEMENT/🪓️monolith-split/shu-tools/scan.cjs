// usage: node scan.cjs <candidates.txt> -> JSON {importers:[{file,sites:[{kind,spec,line}]}], other:[...]}
const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const R = "/Users/ueli/Documents/semio";
const BARREL = process.env.BARREL;
const cands = fs.readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean).map((p) => path.resolve(R, p));
const barrelDir = path.dirname(BARREL);
const isBarrelSpec = (from, spec) => {
  const s = spec.replace(/\?.*$/, "");
  if (!s.startsWith(".")) return false;
  const base = path.resolve(path.dirname(from), s);
  return [BARREL, BARREL.replace(/x$/, ""), barrelDir, path.join(barrelDir, "🟦️")].includes(base);
};
const out = { importers: [], other: [] };
for (const f of cands) {
  if (!/\.(tsx?|mts|cts|js|mjs|cjs)$/.test(f)) { out.other.push(f); continue; }
  if (f === BARREL) continue;
  const t = fs.readFileSync(f, "utf8");
  const sf = ts.createSourceFile(f, t, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const sites = [];
  const walk = (n) => {
    let lit = null, kind = null;
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) { lit = n.moduleSpecifier; kind = ts.isImportDeclaration(n) ? "import" : "export-from"; }
    else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) {
      const e = n.expression.getText(sf);
      if (n.expression.kind === ts.SyntaxKind.ImportKeyword) { lit = n.arguments[0]; kind = "dynamic-import"; }
      else if (/^(vi|jest)\.(mock|doMock|importActual|importMock|unmock|requireActual)$/.test(e) || e === "require" || e === "import.meta.glob") { lit = n.arguments[0]; kind = e; }
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) { lit = n.argument.literal; kind = "import-type"; }
    else if (ts.isImportEqualsDeclaration(n) && ts.isExternalModuleReference(n.moduleReference) && ts.isStringLiteral(n.moduleReference.expression)) { lit = n.moduleReference.expression; kind = "import="; }
    if (lit && isBarrelSpec(f, lit.text)) sites.push({ kind, spec: lit.text, line: sf.getLineAndCharacterOfPosition(lit.getStart(sf)).line + 1 });
    else if (lit && !lit.text.startsWith(".") && lit.text.includes("ShellHelpers")) sites.push({ kind: "alias-" + kind, spec: lit.text, line: sf.getLineAndCharacterOfPosition(lit.getStart(sf)).line + 1 });
    ts.forEachChild(n, walk);
  };
  walk(sf);
  const rawStrings = [];
  const walk2 = (n) => {
    if (ts.isStringLiteralLike(n) && /ShellHelpers/.test(n.text) && !sites.some((s) => s.line === sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1)) rawStrings.push({ line: sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1, text: n.text.slice(0, 160) });
    ts.forEachChild(n, walk2);
  };
  walk2(sf);
  if (sites.length || rawStrings.length) out.importers.push({ file: f, sites, rawStrings });
}
console.log(JSON.stringify(out, null, 1));
