/** 🪝 Turns an in-source vitest registration (a module dynamically importing its own test files) into self-registering test files.
 * Usage: bun 📜️script.ts <module> <apply|dry>   (run from the repository root)
 * Every `if (import.meta.vitest) { ... }` block of the module is read: the dynamically imported test files and their register calls are found, each bag entry is resolved to the module export, a module import, or another dynamic import, and the register call moves to the bottom of the test file.
 * Module-local bag entries that are not exported get an `export` modifier. Module imports that only the removed block used are pruned.
 */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";

const [moduleArg, mode] = process.argv.slice(2) as [string, string];
const modulePath = resolve(moduleArg);
const moduleDir = dirname(modulePath);
const kind = (p: string) => (p.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const parse = (p: string, text: string) => ts.createSourceFile(p, text, ts.ScriptTarget.ESNext, true, kind(p));
const moduleText = readFileSync(modulePath, "utf8");
const msf = parse(modulePath, moduleText);

const rel = (from: string, to: string) => {
  let out = relative(from, to).split(sep).join("/");
  if (!out.startsWith(".")) out = `./${out}`;
  return out;
};
const candidates = (base: string) => [base, `${base}.ts`, `${base}.tsx`, `${base}/🟦️.ts`, `${base}/🟦️.tsx`];
const isRelative = (spec: string) => spec.startsWith(".");
const resolveExisting = (spec: string): string => {
  const base = resolve(moduleDir, spec);
  const found = candidates(base).find((c) => { try { return readFileSync(c) !== undefined; } catch { return false; } });
  if (!found) throw new Error(`cannot resolve ${spec}`);
  return found;
};

type Binding = { spec: string; imported: string };
const moduleImports = new Map<string, Binding>();
for (const st of msf.statements) {
  if (!ts.isImportDeclaration(st) || !ts.isStringLiteral(st.moduleSpecifier) || !st.importClause) continue;
  const spec = st.moduleSpecifier.text;
  const clause = st.importClause;
  if (clause.name) moduleImports.set(clause.name.text, { spec, imported: "default" });
  if (clause.namedBindings) {
    if (ts.isNamespaceImport(clause.namedBindings)) moduleImports.set(clause.namedBindings.name.text, { spec, imported: "*" });
    else for (const el of clause.namedBindings.elements) moduleImports.set(el.name.text, { spec, imported: (el.propertyName ?? el.name).text });
  }
}

const exported = new Set<string>();
const topLevelDecl = new Map<string, ts.Statement>();
for (const st of msf.statements) {
  const isExp = !!(ts.canHaveModifiers(st) && ts.getModifiers(st)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword));
  const names: string[] = [];
  if (ts.isFunctionDeclaration(st) && st.name) names.push(st.name.text);
  else if (ts.isClassDeclaration(st) && st.name) names.push(st.name.text);
  else if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) if (ts.isIdentifier(d.name)) names.push(d.name.text);
  for (const n of names) {
    topLevelDecl.set(n, st);
    if (isExp) exported.add(n);
  }
  if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause) && !st.moduleSpecifier) for (const el of st.exportClause.elements) exported.add(el.name.text);
}

type Call = { fn: string; testPath: string; bag: { key: string; value: string }[]; third: ts.Expression | undefined };
const calls: Call[] = [];
const blocks: { start: number; end: number }[] = [];
const dynModules = new Map<string, Binding>();
for (const st of msf.statements) {
  if (!ts.isIfStatement(st) || st.expression.getText(msf) !== "import.meta.vitest" || !ts.isBlock(st.thenStatement)) continue;
  blocks.push({ start: st.getStart(msf), end: st.getEnd() });
  const fnToTest = new Map<string, string>();
  for (const inner of st.thenStatement.statements) {
    if (!ts.isVariableStatement(inner)) continue;
    for (const d of inner.declarationList.declarations) {
      if (!ts.isObjectBindingPattern(d.name) || !d.initializer) continue;
      const init = ts.isAwaitExpression(d.initializer) ? d.initializer.expression : d.initializer;
      if (!ts.isCallExpression(init) || init.expression.kind !== ts.SyntaxKind.ImportKeyword || !init.arguments[0] || !ts.isStringLiteral(init.arguments[0])) throw new Error(`unsupported block statement ${inner.getText(msf).slice(0, 100)}`);
      const spec = init.arguments[0].text;
      const isTest = /(^|\/)🧪️tests\//u.test(spec) || /(^|\/)🧪️[^/]+\/🟦️\.tsx?$/u.test(spec);
      for (const el of d.name.elements) {
        if (!ts.isIdentifier(el.name)) throw new Error("unsupported binding");
        const imported = (el.propertyName && ts.isIdentifier(el.propertyName) ? el.propertyName : el.name).text;
        if (isTest && /^registerTests\d+$/u.test(el.name.text)) fnToTest.set(el.name.text, resolveExisting(spec));
        else dynModules.set(el.name.text, { spec, imported });
      }
    }
  }
  for (const inner of st.thenStatement.statements) {
    if (!ts.isExpressionStatement(inner)) continue;
    const e = ts.isAwaitExpression(inner.expression) ? inner.expression.expression : inner.expression;
    if (!ts.isCallExpression(e) || !ts.isIdentifier(e.expression) || !fnToTest.has(e.expression.text)) throw new Error(`unsupported block expression ${inner.getText(msf).slice(0, 100)}`);
    const bagArg = e.arguments[1];
    if (!bagArg || !ts.isObjectLiteralExpression(bagArg)) throw new Error(`bag is not an object literal for ${e.expression.text}`);
    const bag: { key: string; value: string }[] = [];
    for (const p of bagArg.properties) {
      if (ts.isShorthandPropertyAssignment(p)) bag.push({ key: p.name.text, value: p.name.text });
      else if (ts.isPropertyAssignment(p) && ts.isIdentifier(p.name) && ts.isIdentifier(p.initializer)) bag.push({ key: p.name.text, value: p.initializer.text });
      else throw new Error(`unsupported bag entry ${p.getText(msf).slice(0, 100)}`);
    }
    calls.push({ fn: e.expression.text, testPath: fnToTest.get(e.expression.text)!, bag, third: e.arguments[2] });
  }
}
if (calls.length === 0) throw new Error("no in-source registrations found");

const needsExport = new Set<string>();
const exportedHere = (name: string) => exported.has(name);
const usedAfterRemoval = new Map<string, number>();
{
  const inBlock = (pos: number) => blocks.some((b) => pos >= b.start && pos < b.end);
  const visit = (n: ts.Node) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n) && !inBlock(n.getStart(msf))) usedAfterRemoval.set(n.text, (usedAfterRemoval.get(n.text) ?? 0) + 1);
    ts.forEachChild(n, visit);
  };
  visit(msf);
}

const outputs = new Map<string, { path: string; text: string }>();
const byTest = new Map<string, Call[]>();
for (const c of calls) byTest.set(c.testPath, [...(byTest.get(c.testPath) ?? []), c]);
const prunedImports = new Set<string>();
for (const [testPath, group] of byTest) {
  const testText = readFileSync(testPath, "utf8");
  const tsf = parse(testPath, testText);
  const testDir = dirname(testPath);
  const header = new Map<string, string>();
  const nsFor = (absOrBare: string, spec: string, isDefault: boolean): string => {
    const key = `${isDefault ? "d:" : "n:"}${absOrBare}`;
    let name = header.get(key);
    if (!name) {
      name = isDefault ? `Default${header.size}` : `Dep${header.size}`;
      header.set(key, name);
      headerSpec.set(name, { spec, isDefault });
    }
    return name;
  };
  const headerSpec = new Map<string, { spec: string; isDefault: boolean }>();
  const relSpecForTest = (spec: string): { id: string; out: string } => {
    if (!isRelative(spec)) return { id: spec, out: spec };
    const abs = resolve(moduleDir, spec);
    return { id: abs, out: rel(testDir, abs) };
  };
  const moduleNs = "ModuleUnderTest";
  const moduleTarget = rel(testDir, modulePath);
  const lines: string[] = [];
  const wantsUrl = group.some((c) => c.third?.getText(msf).includes("import.meta.url"));
  const wantsDir = group.some((c) => /import\.meta\.dir(name)?\b/u.test(c.third?.getText(msf) ?? ""));
  for (const c of group) {
    const entries: string[] = [];
    for (const { key, value } of c.bag) {
      const dyn = dynModules.get(value);
      const imp = moduleImports.get(value);
      if (dyn) {
        const { id, out } = relSpecForTest(dyn.spec);
        entries.push(`${key}: ${nsFor(id, out, false)}.${dyn.imported}`);
      } else if (imp) {
        const { id, out } = relSpecForTest(imp.spec);
        if (imp.imported === "default") entries.push(`${key}: ${nsFor(id, out, true)}`);
        else if (imp.imported === "*") entries.push(`${key}: ${nsFor(id, out, false)}`);
        else entries.push(`${key}: ${nsFor(id, out, false)}.${imp.imported}`);
        prunedImports.add(value);
      } else {
        if (!exportedHere(value)) {
          if (!topLevelDecl.has(value)) throw new Error(`bag entry ${value} is neither exported, imported nor declared at top level`);
          needsExport.add(value);
        }
        entries.push(`${key}: ${moduleNs}.${value}`);
      }
    }
    let third = "";
    if (c.third) {
      let text = c.third.getText(msf);
      text = text.replace(/import\.meta\.url\b/gu, "moduleUnderTestUrl").replace(/import\.meta\.dir(name)?\b/gu, "moduleUnderTestDirectory");
      third = `, ${text}`;
    }
    lines.push(`await ${c.fn}(vitestApi, { ${entries.join(", ")} }${third});`);
  }
  const importLines: string[] = [`import * as ${moduleNs} from "${moduleTarget}";`];
  for (const [name, { spec, isDefault }] of headerSpec) importLines.push(isDefault ? `import ${name} from "${spec}";` : `import * as ${name} from "${spec}";`);
  const needResolve = wantsUrl || wantsDir;
  const hasNamed = (spec: string, name: string) => tsf.statements.some((s) => ts.isImportDeclaration(s) && ts.isStringLiteral(s.moduleSpecifier) && s.moduleSpecifier.text === spec && s.importClause?.namedBindings && ts.isNamedImports(s.importClause.namedBindings) && s.importClause.namedBindings.elements.some((el) => el.name.text === name));
  const consts: string[] = [];
  if (needResolve && !hasNamed("node:path", "resolve")) importLines.push(`import { resolve } from "node:path";`);
  if (wantsUrl && !hasNamed("node:url", "pathToFileURL")) importLines.push(`import { pathToFileURL } from "node:url";`);
  const moduleRel = rel(testDir, modulePath);
  const dirRel = rel(testDir, moduleDir);
  if (wantsUrl) consts.push(`const moduleUnderTestUrl = pathToFileURL(resolve(import.meta.dirname, "${moduleRel}")).href;`);
  if (wantsDir) consts.push(`const moduleUnderTestDirectory = resolve(import.meta.dirname, "${dirRel}");`);
  for (const taken of ["moduleUnderTestUrl", "moduleUnderTestDirectory", "vitestApi"]) if (new RegExp(`\\b${taken}\\b`, "u").test(testText)) throw new Error(`identifier ${taken} already used in ${testPath}`);
  const lastImport = [...tsf.statements].reverse().find((s) => ts.isImportDeclaration(s));
  let insertAt = lastImport ? lastImport.getEnd() : 0;
  if (!lastImport) {
    const ranges = ts.getLeadingCommentRanges(testText, 0) ?? [];
    if (ranges.length) insertAt = ranges[ranges.length - 1]!.end;
  }
  const head = `${insertAt === 0 ? "" : "\n"}${importLines.join("\n")}${insertAt === 0 ? "\n" : ""}`;
  const tail = `\n${consts.length ? `${consts.join("\n")}\n` : ""}const vitestApi = await import("vitest");\n${lines.join("\n")}\n`;
  const newTest = testText.slice(0, insertAt) + head + testText.slice(insertAt);
  outputs.set(testPath, { path: testPath, text: newTest.replace(/\s*$/u, "\n") + tail });
}

let newModule = moduleText;
const edits: { start: number; end: number; text: string }[] = [];
for (const b of blocks) {
  let start = b.start;
  let end = b.end;
  if (newModule[end] === "\n") end += 1;
  const before = newModule.slice(0, start);
  const after = newModule.slice(end);
  const open = /(^|\n)([ \t]*\/\/ ?#region [^\n]*\n)(?:[ \t]*\n)*$/u.exec(before);
  const close = /^(?:[ \t]*\n)*([ \t]*\/\/ ?#endregion [^\n]*(?:\n|$))/u.exec(after);
  if (open && close) {
    start = before.length - open[0].length + open[1].length;
    end += close[0].length;
  }
  edits.push({ start, end, text: "" });
}
for (const name of needsExport) {
  const st = topLevelDecl.get(name)!;
  edits.push({ start: st.getStart(msf), end: st.getStart(msf), text: "export " });
}
for (const st of msf.statements) {
  if (!ts.isImportDeclaration(st) || !st.importClause) continue;
  const clause = st.importClause;
  const keep: string[] = [];
  let defaultName: string | undefined = clause.name?.text;
  const named: string[] = [];
  let nsName: string | undefined;
  let changed = false;
  if (defaultName && prunedImports.has(defaultName) && !usedAfterRemoval.get(defaultName)) { defaultName = undefined; changed = true; }
  if (clause.namedBindings) {
    if (ts.isNamespaceImport(clause.namedBindings)) {
      const n = clause.namedBindings.name.text;
      if (prunedImports.has(n) && !usedAfterRemoval.get(n)) changed = true;
      else nsName = n;
    } else {
      for (const el of clause.namedBindings.elements) {
        if (prunedImports.has(el.name.text) && !usedAfterRemoval.get(el.name.text)) changed = true;
        else named.push(el.getText(msf));
      }
    }
  }
  if (!changed) continue;
  const typeOnly = clause.isTypeOnly ? "type " : "";
  const parts: string[] = [];
  if (defaultName) parts.push(defaultName);
  if (nsName) parts.push(`* as ${nsName}`);
  if (named.length) parts.push(`{ ${named.join(", ")} }`);
  void keep;
  const spec = (st.moduleSpecifier as ts.StringLiteral).getText(msf);
  const replacement = parts.length ? `import ${typeOnly}${parts.join(", ")} from ${spec};` : "";
  let end = st.getEnd();
  if (!replacement && newModule[end] === "\n") end += 1;
  edits.push({ start: st.getStart(msf), end, text: replacement });
}
edits.sort((a, b) => b.start - a.start);
for (const e of edits) newModule = newModule.slice(0, e.start) + e.text + newModule.slice(e.end);
if (!/import\.meta\.vitest/u.test(newModule)) newModule = newModule.replace(/^\/\/\/ <reference types="vitest\/importMeta" \/>\n(?:[ \t]*\n)*/u, "");

console.log(`module blocks removed: ${blocks.length}; register calls: ${calls.map((c) => c.fn).join(", ")}; tests: ${[...byTest.keys()].map((p) => rel(moduleDir, p)).join(", ")}`);
console.log(`exports added: ${[...needsExport].join(", ") || "-"}; imports pruned (candidates): ${[...prunedImports].filter((n) => !usedAfterRemoval.get(n)).join(", ") || "-"}`);
for (const o of outputs.values()) console.log(`--- ${rel(moduleDir, o.path)} tail ---\n${o.text.slice(o.text.lastIndexOf("\nconst vitestApi") - 400).slice(0, 1800)}`);
if (mode === "apply") {
  for (const o of outputs.values()) writeFileSync(o.path, o.text);
  writeFileSync(modulePath, newModule);
  console.log("APPLIED");
}
