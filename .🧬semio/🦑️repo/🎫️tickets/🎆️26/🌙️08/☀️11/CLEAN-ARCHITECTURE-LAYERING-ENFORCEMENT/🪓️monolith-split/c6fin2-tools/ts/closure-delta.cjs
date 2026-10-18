// 🧭 For a split module directory: which files of the module directory does a taxonomy path list (an exact import closure) now reach? usage: node closure-delta.cjs <taxonomy.json> <list-key-path e.g. a.b.c> <module-dir-rel>
const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const R = "/Users/ueli/Documents/semio/";
const [tax, keyPath, modDir] = process.argv.slice(2);
let node = JSON.parse(fs.readFileSync(tax, "utf8"));
for (const k of keyPath.split(".")) node = node[k];
const list = node;
const inDir = (p) => p.startsWith(modDir + "/");
const old = new Set(list);
const imports = (file) => {
  const text = fs.readFileSync(R + file, "utf8");
  const emitted = ts.transpileModule(text, { compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, jsx: ts.JsxEmit.Preserve }, fileName: file }).outputText;
  const sf = ts.createSourceFile(file, emitted, ts.ScriptTarget.Latest, true);
  const out = [];
  for (const n of sf.statements) {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      const s = n.moduleSpecifier.text;
      if (s.startsWith(".")) out.push(path.relative(R, path.resolve(R + path.dirname(file), s)));
    }
  }
  return out;
};
const reached = new Set();
const queue = [];
for (const f of list) {
  if (!/\.(ts|tsx)$/.test(f) || !fs.existsSync(R + f)) continue;
  if (inDir(f)) continue;
  for (const t of imports(f)) if (inDir(t) && fs.existsSync(R + t)) { if (!reached.has(t)) { reached.add(t); queue.push(t); } }
}
while (queue.length) {
  const f = queue.pop();
  if (!/\.(ts|tsx)$/.test(f)) continue;
  for (const t of imports(f)) if (inDir(t) && fs.existsSync(R + t) && !reached.has(t)) { reached.add(t); queue.push(t); }
}
const listedInDir = list.filter(inDir);
console.log("listed in dir:", listedInDir.length, "reached now:", [...reached].filter((x) => /\.(ts|tsx)$/.test(x)).length);
console.log("REACHED:", [...reached].sort().join("\n"));
console.log("LISTED-BUT-NOT-REACHED:", listedInDir.filter((x) => !reached.has(x)).join("\n"));
console.log("REACHED-BUT-NOT-LISTED:", [...reached].filter((x) => !old.has(x)).sort().join("\n"));
