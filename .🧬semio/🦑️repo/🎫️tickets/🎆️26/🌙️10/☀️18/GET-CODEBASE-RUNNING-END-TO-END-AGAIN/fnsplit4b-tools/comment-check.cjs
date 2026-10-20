// 💬 usage: node comment-check.cjs <orig> <rootFile> <spec.json|-> : every comment line of the original must occur in the root or in the children listed by the spec (modulo indentation and the leading star)
const ts = require(process.env.TYPESCRIPT_PATH || "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const [orig, root, specPath] = process.argv.slice(2);
const files = [root];
if (specPath && specPath !== "-") {
  const spec = JSON.parse(fs.readFileSync(specPath, "utf8"));
  const dirs = [];
  for (const k of ["kit", "shared", "helpers"]) if (spec[k]) dirs.push(spec[k].dir);
  for (const g of spec.groups || []) { dirs.push(g.dir); if (g.helpers) dirs.push(g.helpers.dir); for (const p of g.parts || []) dirs.push(p.dir); }
  for (const p of spec.parts || []) dirs.push(p.dir);
  for (const f of spec.files || []) dirs.push(f.dir);
  for (const d of dirs) files.push(path.join(path.dirname(spec.file), d, "🟦️" + path.extname(spec.file)));
}
const lines = (file) => {
  const text = fs.readFileSync(file, "utf8");
  const out = [];
  const sc = ts.createScanner(ts.ScriptTarget.Latest, false, file.endsWith("x") ? ts.LanguageVariant.JSX : ts.LanguageVariant.Standard, text);
  for (let t = sc.scan(); t !== ts.SyntaxKind.EndOfFileToken; t = sc.scan()) {
    if (t === ts.SyntaxKind.SingleLineCommentTrivia || t === ts.SyntaxKind.MultiLineCommentTrivia) for (const l of sc.getTokenText().split("\n")) out.push(l.trim().replace(/^\*\s?/, "").replace(/\s+/g, " "));
  }
  return out.filter(Boolean);
};
const have = new Map();
for (const f of files) for (const l of lines(f)) have.set(l, (have.get(l) || 0) + 1);
const missing = [];
for (const l of lines(orig)) { const n = have.get(l) || 0; if (n === 0) missing.push(l); else have.set(l, n - 1); }
console.log(`comment lines missing from the new tree: ${missing.length}`);
for (const l of missing.slice(0, 12)) console.log("  -", l.slice(0, 110));
