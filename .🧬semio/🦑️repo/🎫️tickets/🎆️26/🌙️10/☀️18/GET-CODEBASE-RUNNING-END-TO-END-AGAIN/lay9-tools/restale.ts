import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
const ROOT = "/Users/ueli/Documents/semio/";
const OS = ROOT + "🧰️framework/🛍️products/💻️os/";
const apply = process.argv[3] === "apply";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const repl = new Map<string, string>();
for (const f of plan.fixes) repl.set(ROOT + f.from + "\u0000" + f.specifier, f.replacement);
const cfg = ts.getParsedCommandLineOfConfigFile(OS + "tsconfig.json", {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: (d) => { throw new Error(String(d.messageText)); } })!;
const program = ts.createProgram({ rootNames: cfg.fileNames, options: { ...cfg.options, noEmit: true, incremental: false, tsBuildInfoFile: undefined } });
const checker = program.getTypeChecker();
const W = OS + "🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/";
let fixed = 0, unresolved = 0;
for (const sf of program.getSourceFiles()) {
  if (!sf.fileName.startsWith(W)) continue;
  const diags = program.getSemanticDiagnostics(sf).filter((d) => [2305, 2724, 2459, 2614].includes(d.code) && d.start !== undefined);
  if (!diags.length) continue;
  const perDecl = new Map<ts.ImportDeclaration, { node: ts.ImportSpecifier; file: string }[]>();
  for (const d of diags) {
    let node: ts.Node | undefined = (function find(n: ts.Node): ts.Node | undefined { if (n.getStart(sf) <= d.start! && d.start! < n.getEnd()) { return ts.forEachChild(n, find) ?? n; } return undefined; })(sf);
    while (node && !ts.isImportSpecifier(node)) node = node.parent;
    if (!node || !ts.isImportSpecifier(node)) { console.log("skip non-specifier diag", sf.fileName.slice(W.length), d.code); unresolved++; continue; }
    const decl = node.parent.parent.parent;
    const spec = (decl.moduleSpecifier as ts.StringLiteral).text;
    const bare = repl.get(sf.fileName + "\u0000" + spec);
    if (!bare) { console.log("no replacement known", sf.fileName.slice(W.length), spec.slice(-60)); unresolved++; continue; }
    const res = ts.resolveModuleName(bare, sf.fileName, cfg.options, ts.sys).resolvedModule;
    const entry = res && program.getSourceFile(res.resolvedFileName);
    const sym = entry && checker.getSymbolAtLocation(entry);
    const name = (node.propertyName ?? node.name).text;
    const ex = sym && checker.getExportsOfModule(sym).find((s) => s.name === name);
    const real = ex && (ex.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(ex) : ex);
    const file = real?.declarations?.[0]?.getSourceFile().fileName;
    if (!file) { console.log("cannot relocate", name, bare, sf.fileName.slice(W.length)); unresolved++; continue; }
    (perDecl.get(decl) ?? perDecl.set(decl, []).get(decl)!).push({ node, file });
  }
  const text = sf.text; const edits: { start: number; end: number; text: string }[] = [];
  for (const [decl, items] of perDecl) {
    const nb = decl.importClause!.namedBindings as ts.NamedImports;
    const rest = nb.elements.filter((e) => !items.some((i) => i.node === e));
    const byFile = new Map<string, string[]>();
    for (const i of items) (byFile.get(i.file) ?? byFile.set(i.file, []).get(i.file)!).push(i.node.getText(sf));
    const adds = [...byFile].map(([file, names]) => { let r = relative(dirname(sf.fileName), file); if (!r.startsWith(".")) r = "./" + r; return `\nimport { ${names.join(", ")} } from "${r}";`; }).join("");
    if (!rest.length && !decl.importClause!.name) edits.push({ start: decl.getStart(sf), end: decl.getEnd(), text: adds.slice(1) });
    else { edits.push({ start: nb.getStart(sf), end: nb.getEnd(), text: "{ " + rest.map((e) => e.getText(sf)).join(", ") + " }" }); edits.push({ start: decl.getEnd(), end: decl.getEnd(), text: adds }); }
    fixed += items.length;
  }
  console.log(apply ? "edit" : "would edit", sf.fileName.slice(W.length), edits.length);
  if (apply) { let out = text; for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end); writeFileSync(sf.fileName, out); }
}
console.log({ fixed, unresolved });
