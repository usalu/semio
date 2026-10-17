// usage: BARREL=... node ownermap.cjs <tsconfig> <out.json>
// For every export of the barrel (named `export {..} from`), resolve to the true owner via the checker.
const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const BARREL = process.env.BARREL;
const cfgPath = process.argv[2];
const cfg = ts.getParsedCommandLineOfConfigFile(cfgPath, {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: (d) => { throw new Error(ts.flattenDiagnosticMessageText(d.messageText, "\n")); } });
const program = ts.createProgram({ rootNames: [BARREL], options: { ...cfg.options, noEmit: true, incremental: false, tsBuildInfoFile: undefined } });
const checker = program.getTypeChecker();
const sf = program.getSourceFile(BARREL);
if (!sf) throw new Error("barrel not in program");
const text = sf.text;
const entries = new Map();
for (const s of sf.statements) {
  if (!ts.isExportDeclaration(s) || !s.moduleSpecifier || !s.exportClause || !ts.isNamedExports(s.exportClause)) { if (ts.isExportDeclaration(s)) console.error("non-named export", s.getText(sf).slice(0, 80)); else if (!ts.isEmptyStatement(s)) console.error("non-export statement", ts.SyntaxKind[s.kind]); continue; }
  const spec = s.moduleSpecifier.text;
  for (const el of s.exportClause.elements) entries.set(el.name.text, { spec, declType: !!(el.isTypeOnly || s.isTypeOnly), original: (el.propertyName ?? el.name).text, node: el });
}
const modSym = checker.getSymbolAtLocation(sf);
const exps = checker.getExportsOfModule(modSym);
const out = {};
const problems = [];
const exportNameInModule = (file, target) => {
  const ms = checker.getSymbolAtLocation(file);
  if (!ms) return null;
  const names = [];
  for (const e of checker.getExportsOfModule(ms)) {
    const r = e.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(e) : e;
    if (r === target) names.push(e.name);
  }
  return names;
};
const isFinal = (s) => !(s.flags & ts.SymbolFlags.Alias);
for (const e of exps) {
  const ent = entries.get(e.name);
  if (!ent) { problems.push("export without entry: " + e.name); continue; }
  if (!ent.spec.startsWith(".")) { out[e.name] = { kind: "external", spec: ent.spec, original: ent.original, typeOnly: ent.declType }; continue; }
  const target = e.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(e) : e;
  const decl = target.declarations && target.declarations[0];
  if (!decl || target.name === "unknown") { problems.push("unresolved: " + e.name); continue; }
  const file = decl.getSourceFile();
  const names = exportNameInModule(file, target);
  const valueFlags = target.flags & (ts.SymbolFlags.Value);
  out[e.name] = { kind: "relative", file: file.fileName, original: names && names.includes(e.name) ? e.name : names && names[0], exportNames: names, typeOnly: !valueFlags, declType: ent.declType, viaSpec: ent.spec, viaFile: path.resolve(path.dirname(BARREL), ent.spec) };
  if (!names || !names.length) problems.push("owner exports no such name: " + e.name + " in " + file.fileName);
  else if (!names.includes(e.name)) problems.push("renamed: " + e.name + " -> " + names.join(","));
}
fs.writeFileSync(process.argv[3], JSON.stringify({ problems, names: Object.keys(out).length, out }, null, 1));
console.log(JSON.stringify({ names: Object.keys(out).length, problems }, null, 1));
