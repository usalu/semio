// 🔬 usage: node lossless.cjs <orig.mjs> <inference-dir>: every top-level declaration of the monolith reappears verbatim in exactly one component, modulo the declared rewrites (export keyword, `loaded.` state access, revision identity, root URL).
const ts = require("/Users/ueli/.cache/jsi/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const [origPath, dir] = process.argv.slice(2);
const parse = (file) => { const text = fs.readFileSync(file, "utf8"); return { text, sf: ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS) }; };
const names = (s) => {
  if (ts.isVariableStatement(s) && s.declarationList.declarations.every((d) => d.initializer === undefined)) return [];
  if (ts.isVariableStatement(s)) return s.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : d.name.elements.filter(ts.isBindingElement).map((e) => e.name.getText())));
  if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s)) && s.name) return [s.name.text];
  return [];
};
const LOADED = ["commandInputHash", "commandSourceImports", "moduleSourceImports", "productionModuleSourceImports", "declaredBrowserSessionEnginesV1", "playgroundCompositionPathV1", "admitPlaygroundNativeHostV1", "nativeHostSourceFactsV1", "nativeHostScriptAdmissionV1", "runtimeComponentClosure", "runtimeInputAdmissionV1", "readSourceInputContract", "relativeSourceInputs", "relativeSourceInputsAsyncV1", "componentDeploymentDirectoryV1", "inferProjectInputs", "compactSourceInputs", "cargoLockInput", "deduplicateProjectInputs", "targetBuildModes"];
const normalize = (text) => {
  let t = text.replace(/^export\s+/u, "");
  t = t.replace(/\bloaded\.([A-Za-z0-9_$]+)/g, "$1");
  t = t.replace(/\b([A-Za-z0-9_$]+): ([A-Za-z0-9_$]+)\b/g, (m, a, b) => (a === b ? a : m));
  t = t.replaceAll("revisionIdentity.revision", "IMPLEMENTATION_REVISION").replaceAll("revisionIdentity.url", "import.meta.url");
  return t.replace(/\s+/g, " ").trim();
};
const orig = parse(origPath);
const index = new Map();
const files = [];
(function walk(d) { for (const e of fs.readdirSync(d, { withFileTypes: true })) { const p = path.join(d, e.name); if (e.isDirectory()) { if (!/^🧪️tests$|^🛡️policy$/u.test(e.name)) walk(p); } else if (p.endsWith(".mjs")) files.push(p); } })(dir);
for (const f of files) { if (f === path.join(dir, "🟨️.mjs")) continue; const { text, sf } = parse(f); for (const s of sf.statements) for (const n of names(s)) { if (["wire", "component"].includes(n)) continue; if (!index.has(n)) index.set(n, []); index.get(n).push({ file: path.relative(dir, f), text: s.getText(sf) }); } }
let missing = 0, changed = 0, same = 0, multi = 0;
for (const s of orig.sf.statements) {
  for (const n of names(s)) {
    if (["IMPLEMENTATION_REVISION", "libraryBootstrap"].includes(n)) continue;
    const found = index.get(n);
    if (!found) { console.log("MISSING", n); missing++; continue; }
    if (found.length > 1) { console.log("MULTI", n, found.map((f) => f.file).join(",")); multi++; }
    const a = normalize(s.getText(orig.sf)), b = normalize(found[0].text);
    if (a === b) same++;
    else { console.log("CHANGED", n, found[0].file); if (process.env.SHOW) { let i = 0; while (a[i] === b[i]) i++; console.log("  orig:", a.slice(Math.max(0, i - 60), i + 120)); console.log("  next:", b.slice(Math.max(0, i - 60), i + 120)); } changed++; }
  }
}
console.log({ same, changed, missing, multi });
