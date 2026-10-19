// usage: node fsplit.cjs <spec.json> [APPLY]
// Factory splitter: a factory function whose body is `const {...} = param; [type aliases]; function declarations; return {...}` becomes a root that composes child factories.
// spec: { file, factory, param, paramType (text), extraParams?: [{name,type}], destructureLine, aliasLines: [line], children: [{ dir, doc, name (factory name), emoji, fnDoc, fns: [names], needs: [{child:index, names:[...]}] , returns:[names] }], rootDoc, rootEmoji }
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const VS = "️";
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const REPO = "/Users/ueli/Documents/semio";
const file = path.isAbsolute(spec.file) ? spec.file : path.join(REPO, spec.file);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const ext = path.extname(file);
const errors = [];
const factory = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === spec.factory);
const stmts = factory.body.statements;
const destr = stmts.find((s) => L(s.getStart(sf)) === spec.destructureLine);
const destrNames = destr.declarationList.declarations[0].name.elements.map((e) => e.name.text);
const aliases = (spec.aliasLines || []).map((l) => stmts.find((s) => L(s.getStart(sf)) === l));
const fnDecls = new Map(stmts.filter((s) => ts.isFunctionDeclaration(s) && s.name).map((s) => [s.name.text, s]));
const importStmts = sf.statements.filter(ts.isImportDeclaration);
const identsOf = (code) => {
  const f = ts.createSourceFile("x" + ext, code, ts.ScriptTarget.Latest, true);
  const used = new Set();
  const visit = (n) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = p && ((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n));
      if (!skip) used.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(f);
  return used;
};
const rebase = (fromFile) => (s) => {
  if (!s.startsWith(".")) return s;
  let r = path.relative(path.dirname(fromFile), path.resolve(path.dirname(file), s)).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};
const rebaseDynamic = (code, fromFile) => code.replace(/import\((["'])(\.[^"']+)\1\)/g, (m, q, s) => `import(${q}${rebase(fromFile)(s)}${q})`);
const prunedImports = (used, fix) => {
  const out = [];
  for (const n of importStmts) {
    const c = n.importClause;
    const q = n.moduleSpecifier.getText(sf)[0];
    const sp = fix(n.moduleSpecifier.text);
    if (!c) continue;
    const def = c.name && used.has(c.name.text) ? c.name.text : null;
    let ns = null;
    const named = [];
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && used.has(c.namedBindings.name.text)) ns = c.namedBindings.name.text;
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (used.has(el.name.text)) named.push(el.getText(sf));
    if (!def && !ns && !named.length) continue;
    const bits = [];
    if (def) bits.push(def);
    if (ns) bits.push(`* as ${ns}`);
    if (named.length) bits.push(`{ ${named.join(", ")} }`);
    out.push(`import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${sp}${q};`);
  }
  return out;
};
const fnText = (name) => {
  const s = fnDecls.get(name);
  const lead = ts.getLeadingCommentRanges(text, s.getFullStart()) || [];
  const start = lead.length ? lead[0].pos : s.getStart(sf);
  return text.slice(text.lastIndexOf("\n", start) + 1, s.getEnd());
};
const outputs = [];
const rootCalls = [];
const rootImportsUsed = new Set();
const paramDecl = `${spec.param}: ${spec.paramType}` + (spec.extraParams || []).map((p) => `, ${p.name}: ${p.type}`).join("");
const typeOfChild = (i) => `ReturnType<typeof ${spec.children[i].name}>`;
for (const [ci, ch] of spec.children.entries()) {
  const outPath = path.join(path.dirname(file), ch.dir, "🟦️" + ext);
  const bodies = ch.fns.map((n) => fnText(n));
  const bodyText = bodies.join("\n\n");
  const usedAll = identsOf(bodyText);
  const dnames = destrNames.filter((n) => usedAll.has(n));
  const aliasTexts = aliases.filter((a) => usedAll.has(a.name.text)).map((a) => "  " + a.getText(sf));
  const needParams = (ch.needs || []).flatMap((nd) => nd.names);
  const needParamText = (ch.needs || []).length ? ", " + (ch.needs || []).map((nd) => `{ ${nd.names.join(", ")} }: Pick<${typeOfChild(nd.child)}, ${nd.names.map((x) => `"${x}"`).join(" | ")}>`).join(", ") : "";
  const retObj = `  return { ${ch.returns.join(", ")} };`;
  const decl = dnames.length ? `  const { ${dnames.join(", ")} } = ${spec.param};\n` : "";
  let code = `/** ${ch.emoji + VS} ${ch.fnDoc} */\nexport function ${ch.name}(${paramDecl}${needParamText}) {\n${decl}${aliasTexts.length ? aliasTexts.join("\n") + "\n" : ""}${bodyText}\n${retObj}\n}`;
  code = rebaseDynamic(code, outPath);
  const used = identsOf(code);
  const imps = prunedImports(used, rebase(outPath));
  for (const nd of ch.needs || []) imps.push(`import type { ${spec.children[nd.child].name} } from "${path.relative(path.dirname(outPath), path.join(path.dirname(file), spec.children[nd.child].dir, "🟦️" + ext)).split(path.sep).join("/").replace(/^(?!\.)/, "./")}";`);
  outputs.push({ path: outPath, text: `/** ${ch.doc.replace(/^\/\*\*\s*/, "").replace(/\s*\*\/$/, "")} */\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${code}\n`, ci });
  const argNames = (ch.needs || []).map((nd) => `{ ${nd.names.join(", ")} }`);
  rootCalls.push({ ci, name: ch.name, returns: ch.returns, args: argNames });
}
const rootBody = rootCalls.map((c) => `  const { ${c.returns.join(", ")} } = ${c.name}(${[spec.param, ...(spec.extraParams || []).map((p) => p.name), ...c.args.map((a, i) => a)].join(", ")});`).join("\n");
const allReturns = spec.rootReturns || [...new Set(rootCalls.flatMap((c) => c.returns))];
const rootFn = `/** ${spec.rootEmoji + VS} ${spec.rootDocText} */\nexport function ${spec.factory}(${paramDecl}) {\n${rootBody}\n  return { ${allReturns.join(", ")} };\n}`;
const rootUsed = identsOf(rootFn);
const rootImps = prunedImports(rootUsed, (s) => s);
for (const c of rootCalls) rootImps.push(`import { ${c.name} } from "${path.relative(path.dirname(file), path.join(path.dirname(file), spec.children[c.ci].dir, "🟦️" + ext)).split(path.sep).join("/").replace(/^(?!\.)/, "./")}";`);
const rootText = `${rootImps.join("\n")}\n\n${rootFn}\n`;
const lc = (t) => t.split("\n").length;
console.log(JSON.stringify({ root: lc(rootText), children: outputs.map((o) => ({ p: path.basename(path.dirname(o.path)), lines: lc(o.text) })), errors }, null, 1));
if (process.env.DUMP) { fs.mkdirSync(process.env.DUMP, { recursive: true }); fs.writeFileSync(path.join(process.env.DUMP, "root" + ext), rootText); outputs.forEach((o, i) => fs.writeFileSync(path.join(process.env.DUMP, `${i}-${path.basename(path.dirname(o.path))}${ext}`), o.text)); }
if (APPLY) {
  if (errors.length) { console.error("REFUSED"); process.exit(2); }
  for (const o of outputs) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: root changed"); process.exit(5); }
  for (const o of outputs) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, rootText);
  console.log("APPLIED");
}
