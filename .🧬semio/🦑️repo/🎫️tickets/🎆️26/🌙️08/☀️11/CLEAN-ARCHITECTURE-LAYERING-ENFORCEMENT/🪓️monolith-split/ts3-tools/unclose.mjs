// usage: node unclose.mjs <spec.json> [APPLY]
// Turns a registrar closure `export async function registerX(vitest, dependencies: Pick<typeof import(".."), "a"|"b">, source)` into a plain vitest module
// (static imports for the dependency bag, the vitest globals and the closure's own dynamic imports; body statements hoisted and dedented) written to `out`.
// spec: { file, out, fn, sourceExpr?: { directory, url }, header?: string }
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = path.resolve(REPO, spec.file);
const out = path.resolve(REPO, spec.out);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);

const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name?.text === spec.fn);
if (!fn || !fn.body) throw new Error("function not found: " + spec.fn);
const params = fn.parameters;
const depsParam = params.find((p) => p.name.getText(sf) === "dependencies");
const sourceParam = params.find((p) => p.name.getText(sf) === "source");

const depSpec = new Map();
if (depsParam && depsParam.type) {
  const visit = (n) => {
    if (ts.isTypeReferenceNode(n) && n.typeName.getText(sf) === "Pick" && n.typeArguments?.length === 2) {
      const q = n.typeArguments[0];
      const names = [];
      const collect = (t) => { if (ts.isLiteralTypeNode(t) && ts.isStringLiteral(t.literal)) names.push(t.literal.text); else if (ts.isUnionTypeNode(t)) t.types.forEach(collect); };
      collect(n.typeArguments[1]);
      if (ts.isImportTypeNode(q) && ts.isLiteralTypeNode(q.argument) && ts.isStringLiteral(q.argument.literal)) for (const nm of names) { if (depSpec.has(nm)) throw new Error("name in two Picks: " + nm); depSpec.set(nm, q.argument.literal.text); }
      else if (spec.host) { for (const nm of names) if (!depSpec.has(nm)) depSpec.set(nm, null); }
      else throw new Error("unsupported Pick source: " + q.getText(sf));
      return;
    }
    ts.forEachChild(n, visit);
  };
  visit(depsParam.type);
}

const hostMap = new Map();
if (spec.host) {
  const hostFile = path.resolve(REPO, spec.host);
  const hostText = fs.readFileSync(hostFile, "utf8");
  const hsf = ts.createSourceFile(hostFile, hostText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const rebase = (sp) => (sp.startsWith(".") ? (() => { let r = path.relative(path.dirname(out), path.resolve(path.dirname(hostFile), sp)).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; })() : sp);
  for (const s of hsf.statements) if (ts.isImportDeclaration(s) && s.importClause?.namedBindings && ts.isNamedImports(s.importClause.namedBindings)) for (const el of s.importClause.namedBindings.elements) if (!(el.propertyName && el.propertyName.text !== el.name.text)) hostMap.set(el.name.text, rebase(s.moduleSpecifier.text));
  const visitHost = (n) => {
    if (ts.isVariableDeclaration(n) && n.initializer && ts.isAwaitExpression(n.initializer) && ts.isCallExpression(n.initializer.expression) && n.initializer.expression.expression.kind === ts.SyntaxKind.ImportKeyword && ts.isObjectBindingPattern(n.name) && ts.isStringLiteralLike(n.initializer.expression.arguments[0])) for (const el of n.name.elements) if (!hostMap.has(el.name.getText(hsf))) hostMap.set(el.name.getText(hsf), rebase(n.initializer.expression.arguments[0].text));
    ts.forEachChild(n, visitHost);
  };
  visitHost(hsf);
  for (const [nm, sp] of depSpec) if (sp === null || spec.hostWins) { const h = hostMap.get(nm); if (!h) throw new Error("host does not import " + nm); depSpec.set(nm, h); }
}

const templateRanges = [];
const markTemplates = (n) => {
  if (ts.isNoSubstitutionTemplateLiteral(n) || ts.isTemplateExpression(n) || (ts.isStringLiteral(n) && n.getText(sf).includes("\n"))) templateRanges.push([n.getStart(sf), n.getEnd()]);
  ts.forEachChild(n, markTemplates);
};
markTemplates(fn.body);

const vitestNames = [];
const depNames = [];
const dynamicImports = [];
const uses = { source: false, dependencies: false };
const lineStarts = [0];
for (let i = 0; i < text.length; i++) if (text[i] === "\n") lineStarts.push(i + 1);
const inTemplate = (pos) => templateRanges.some(([x, y]) => pos > x && pos < y);
function dedentedText(from, to) {
  const segment = text.slice(from, to);
  let pos = from;
  const outLines = [];
  for (const ln of segment.split("\n")) {
    const startsInside = inTemplate(pos) && pos !== from;
    if (!startsInside && ln.startsWith("  ")) outLines.push(ln.slice(2));
    else outLines.push(ln);
    pos += ln.length + 1;
  }
  return outLines.join("\n");
}
const pieces = [];
let cursor = fn.body.getStart(sf) + 1;
let keptCount = 0;
for (const st of fn.body.statements) {
  let removed = false;
  if (ts.isVariableStatement(st) && st.declarationList.declarations.length === 1) {
    const d = st.declarationList.declarations[0];
    const init = d.initializer;
    if (init && ts.isIdentifier(init) && init.text === "dependencies" && ts.isObjectBindingPattern(d.name)) { for (const el of d.name.elements) depNames.push(el.name.getText(sf)); removed = true; }
    else if (init && ts.isIdentifier(init) && init.text === "vitest" && ts.isObjectBindingPattern(d.name)) { for (const el of d.name.elements) vitestNames.push(el.getText(sf)); removed = true; }
    else if (init && ts.isAwaitExpression(init) && ts.isCallExpression(init.expression) && init.expression.expression.kind === ts.SyntaxKind.ImportKeyword && ts.isObjectBindingPattern(d.name) && ts.isStringLiteralLike(init.expression.arguments[0])) {
      dynamicImports.push({ spec: init.expression.arguments[0].text, names: d.name.elements.map((el) => el.getText(sf)) });
      removed = true;
    }
  }
  if (!removed) { pieces.push(dedentedText(cursor, st.getEnd()).replace(/^\s*\n/, "")); keptCount++; }
  cursor = st.getEnd();
}
const kept = { length: keptCount };
let bodyText = pieces.join("\n\n");
bodyText = bodyText.replace(/\n{3,}/g, "\n\n").replace(/^\s+/, "");

const ref = (name) => new RegExp(`(?<![\\w$.])${name}\\b`).test(bodyText);
if (/(?<![\w$.'"])dependencies\b(?!\s*:)/.test(bodyText)) uses.dependencies = true;
if (uses.dependencies) throw new Error("body still references `dependencies`");
const usesSource = /(?<![\w$.])source\.(url|directory)/.test(bodyText) || /\bsource\b[^:]/.test(bodyText) && spec.sourceExpr;

const importStatements = sf.statements.filter(ts.isImportDeclaration).map((s) => s.getText(sf));
const depBySpec = new Map();
for (const nm of depNames) {
  const sp = depSpec.get(nm);
  if (!sp) throw new Error("dependency without Pick source: " + nm);
  if (!depBySpec.has(sp)) depBySpec.set(sp, []);
  depBySpec.get(sp).push(nm);
}
const lines2 = [];
lines2.push(...importStatements);
if (vitestNames.length) lines2.push(`import { ${vitestNames.join(", ")} } from "vitest";`);
for (const di of dynamicImports) lines2.push(`import { ${di.names.join(", ")} } from "${di.spec}";`);
for (const [sp, names] of depBySpec) lines2.push(`import { ${names.join(", ")} } from "${sp}";`);
let sourceDecl = "";
if (sourceParam && spec.sourceExpr && usesSource) {
  const typeAlias = sf.statements.find((s) => ts.isTypeAliasDeclaration(s) && s.name.text === "TestSource");
  if (typeAlias) sourceDecl += typeAlias.getText(sf) + "\n";
  sourceDecl += `const source: TestSource = { directory: ${spec.sourceExpr.directory}, url: ${spec.sourceExpr.url} };\n`;
}
const extraImports = spec.extraImports ?? [];
const result = `${spec.header ?? "/** stage */"}\n${[...lines2, ...extraImports].join("\n")}\n\n${sourceDecl}${sourceDecl ? "\n" : ""}${bodyText.replace(/\s+$/, "")}\n`;
void ref;
const outSf = ts.createSourceFile(out, result, ts.ScriptTarget.Latest, true, out.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const diag = outSf.parseDiagnostics ?? [];
console.log(JSON.stringify({ fn: spec.fn, bodyStatements: kept.length, vitestNames, depNames: depNames.length, dynamicImports: dynamicImports.length, usesSource: !!sourceDecl, parseDiagnostics: diag.length, lines: result.split("\n").length - 1 }));
if (diag.length) for (const d of diag.slice(0, 5)) console.log("  PARSE:", ts.flattenDiagnosticMessageText(d.messageText, "\n"), "at line", outSf.getLineAndCharacterOfPosition(d.start).line + 1);
const lineOf = (pos) => outSf.getLineAndCharacterOfPosition(pos).line + 1;
console.log("top-level statement starts (line: kind name):");
for (const s of outSf.statements) {
  if (ts.isImportDeclaration(s)) continue;
  const nm = ts.isVariableStatement(s) ? s.declarationList.declarations.map((d) => d.name.getText(outSf)).join(",").slice(0, 50) : s.name ? s.name.getText(outSf) : ts.isExpressionStatement(s) ? s.expression.getText(outSf).slice(0, 80).replace(/\n/g, " ") : "";
  console.log(`  ${lineOf(s.getStart(outSf))}-${lineOf(s.getEnd())} ${ts.SyntaxKind[s.kind]} ${nm}`);
}
if (APPLY) { fs.writeFileSync(out, result); console.log("WROTE", path.relative(REPO, out)); }
