// 🧷 usage: node absorig.cjs <orig.mjs> <real-root.mjs> <out.mjs>: rewrites the original monolith so it can run from any directory: relative imports become absolute, import.meta.url names the real module location.
const ts = require("/Users/ueli/.cache/jsi/nm/node_modules/typescript");
const fs = require("fs"), path = require("path"), { pathToFileURL } = require("url");
const [source, real, out] = process.argv.slice(2);
const text = fs.readFileSync(source, "utf8");
const sf = ts.createSourceFile(source, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
const base = path.dirname(path.resolve(real));
const edits = [];
for (const s of sf.statements) {
  if (ts.isImportDeclaration(s) && s.moduleSpecifier.text.startsWith(".")) {
    const abs = path.resolve(base, s.moduleSpecifier.text);
    edits.push({ at: s.moduleSpecifier.getStart(sf), end: s.moduleSpecifier.getEnd(), to: JSON.stringify(abs) });
  }
}
const visit = (n) => {
  if (ts.isPropertyAccessExpression(n) && ts.isMetaProperty(n.expression) && n.name.text === "url") edits.push({ at: n.getStart(sf), end: n.getEnd(), to: "ORIGINAL_MODULE_URL" });
  ts.forEachChild(n, visit);
};
visit(sf);
edits.sort((a, b) => b.at - a.at);
let result = text;
for (const e of edits) result = result.slice(0, e.at) + e.to + result.slice(e.end);
result = `const ORIGINAL_MODULE_URL = ${JSON.stringify(pathToFileURL(path.resolve(real)).href + "?original=1")};\n` + result;
fs.writeFileSync(out, result);
console.log("rewrote", edits.length);
