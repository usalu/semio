/** 🪝 Turns an in-source vitest registration (a module dynamically importing its own test file) into a self-registering test file.
 * Usage: bun 📜️script.ts <module> <test> <apply|dry>   (run from the repository root)
 * The module's `if (import.meta.vitest) { ... await import(test) ... }` blocks are removed; the test file imports the module and calls its register functions with a bag built from the `Pick<typeof import(...)>` parameter types.
 */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";

const [moduleArg, testArg, mode] = process.argv.slice(2) as [string, string, string];
const modulePath = resolve(moduleArg);
const testPath = resolve(testArg);
const kind = (p: string) => (p.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const moduleText = readFileSync(modulePath, "utf8");
const testText = readFileSync(testPath, "utf8");
const msf = ts.createSourceFile(modulePath, moduleText, ts.ScriptTarget.ESNext, true, kind(modulePath));
const tsf = ts.createSourceFile(testPath, testText, ts.ScriptTarget.ESNext, true, kind(testPath));
const rel = (from: string, to: string) => {
  let out = relative(dirname(from), to).split(sep).join("/");
  if (!out.startsWith(".")) out = `./${out}`;
  return out;
};
const candidates = (from: string, spec: string) => {
  const base = resolve(dirname(from), spec);
  return [base, `${base}.ts`, `${base}.tsx`, `${base}/🟦️.ts`, `${base}/🟦️.tsx`];
};

type Call = { fn: string; bagKeys: string[]; hasSource: boolean; blockStart: number; blockEnd: number };
const calls: Call[] = [];
const blocks: { start: number; end: number }[] = [];
for (const st of msf.statements) {
  if (!ts.isIfStatement(st) || st.getText(msf).slice(0, 40).replace(/\s+/g, " ").indexOf("import.meta.vitest") < 0 || !ts.isBlock(st.thenStatement)) continue;
  let importsTest = false;
  const localCalls: Call[] = [];
  const registerFns = new Map<string, string>();
  for (const inner of st.thenStatement.statements) {
    inner.forEachChild(function visit(n) {
      if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0]) && candidates(modulePath, n.arguments[0].text).includes(testPath)) importsTest = true;
      ts.forEachChild(n, visit);
    });
  }
  if (!importsTest) continue;
  for (const inner of st.thenStatement.statements) {
    if (ts.isVariableStatement(inner)) {
      for (const d of inner.declarationList.declarations) if (ts.isObjectBindingPattern(d.name)) for (const el of d.name.elements) if (ts.isIdentifier(el.name)) registerFns.set(el.name.text, el.name.text);
    }
    if (ts.isExpressionStatement(inner)) {
      const e = ts.isAwaitExpression(inner.expression) ? inner.expression.expression : inner.expression;
      if (ts.isCallExpression(e) && ts.isIdentifier(e.expression) && registerFns.has(e.expression.text)) {
        const bagArg = e.arguments[1];
        if (!bagArg || !ts.isObjectLiteralExpression(bagArg)) throw new Error(`bag is not an object literal for ${e.expression.text}`);
        const keys: string[] = [];
        for (const p of bagArg.properties) {
          if (!ts.isShorthandPropertyAssignment(p)) throw new Error(`non-shorthand bag entry: ${p.getText(msf)}`);
          keys.push(p.name.text);
        }
        localCalls.push({ fn: e.expression.text, bagKeys: keys, hasSource: e.arguments.length > 2, blockStart: st.getFullStart(), blockEnd: st.getEnd() });
      }
    }
  }
  calls.push(...localCalls);
  blocks.push({ start: st.getStart(msf), end: st.getEnd() });
}
if (calls.length === 0) throw new Error("no in-source registrations of the test found");

const namespaces = new Map<string, string>();
const nsFor = (spec: string): string => {
  let name = namespaces.get(spec);
  if (!name) {
    name = `Dep${namespaces.size}`;
    namespaces.set(spec, name);
  }
  return name;
};
const moduleNs = "ModuleUnderTest";
const moduleSpec = rel(testPath, modulePath);
namespaces.set(moduleSpec, moduleNs);

type KeySource = { spec: string; key: string } | { literal: string };
const keySources = new Map<string, Map<string, KeySource>>();
let sawSource = false;
for (const st of tsf.statements) {
  if (!ts.isFunctionDeclaration(st) || !st.name || !calls.some((c) => c.fn === st.name!.text)) continue;
  const depParam = st.parameters[1];
  const sources = new Map<string, KeySource>();
  const visit = (t: ts.TypeNode) => {
    if (ts.isIntersectionTypeNode(t)) {
      t.types.forEach(visit);
      return;
    }
    if (ts.isParenthesizedTypeNode(t)) {
      visit(t.type);
      return;
    }
    if (ts.isTypeReferenceNode(t) && t.typeName.getText(tsf) === "Pick" && t.typeArguments?.length === 2) {
      const q = t.typeArguments[0]!;
      if (!ts.isTypeQueryNode(q) && !ts.isImportTypeNode(q)) throw new Error("unsupported Pick source");
      const spec = ts.isImportTypeNode(q) ? ((q.argument as ts.LiteralTypeNode).literal as ts.StringLiteral).text : (() => { throw new Error("typeof expr unsupported"); })();
      const keyNode = t.typeArguments[1]!;
      const keys = ts.isUnionTypeNode(keyNode) ? keyNode.types : [keyNode];
      for (const k of keys) sources.set(((k as ts.LiteralTypeNode).literal as ts.StringLiteral).text, { spec, key: ((k as ts.LiteralTypeNode).literal as ts.StringLiteral).text });
      return;
    }
    if (ts.isTypeLiteralNode(t)) {
      for (const m of t.members) {
        if (!ts.isPropertySignature(m) || !m.type || !ts.isTypeQueryNode(m.type) && !(m.type as ts.TypeNode).getText(tsf).startsWith("typeof import(")) throw new Error(`unsupported literal member ${m.getText(tsf)}`);
        const spec = /typeof import\("([^"]+)"\)/.exec(m.type!.getText(tsf))![1]!;
        sources.set((m.name as ts.Identifier).text, { spec, key: "*" });
      }
      return;
    }
    throw new Error(`unsupported dependency type node ${ts.SyntaxKind[t.kind]}: ${t.getText(tsf).slice(0, 80)}`);
  };
  visit(depParam!.type!);
  keySources.set(st.name.text, sources);
  sawSource ||= st.parameters.length > 2;
}

const lines: string[] = [];
const importDecls: string[] = [];
const bagLines: string[] = [];
for (const c of calls) {
  const sources = keySources.get(c.fn);
  if (!sources) throw new Error(`test function ${c.fn} not found`);
  const entries: string[] = [];
  for (const key of c.bagKeys) {
    const src = sources.get(key);
    if (!src) {
      entries.push(`${key}: ${moduleNs}.${key}`);
      continue;
    }
    const specForTest = src.spec;
    if (src.key === "*") {
      const alias = `${key}Namespace`;
      importDecls.push(`import * as ${alias} from "${specForTest}";`);
      entries.push(`${key}: ${alias}`);
    } else entries.push(src.spec === moduleSpec ? `${key}: ${moduleNs}.${key}` : `${key}: ${nsFor(specForTest)}.${key}`);
  }
  const dir = rel(testPath, dirname(modulePath));
  const sourceArg = c.hasSource ? `, { directory: resolve(import.meta.dirname, "${dir}"), url: pathToFileURL(resolve(import.meta.dirname, "${moduleSpec}")).href }` : "";
  bagLines.push(`await ${c.fn}(vitestApi, { ${entries.join(", ")} }${sourceArg});`);
}
const header: string[] = [`import * as ${moduleNs} from "${moduleSpec}";`];
for (const [spec, ns] of namespaces) if (spec !== moduleSpec) header.push(`import * as ${ns} from "${spec}";`);
const usesPath = bagLines.some((l) => l.includes("resolve(") );
if (usesPath) header.push(`import { resolve } from "node:path";`, `import { pathToFileURL } from "node:url";`);
header.push(...importDecls);

const tail = `\nconst vitestApi = await import("vitest");\n${bagLines.join("\n")}\n`;
let newTest = testText;
const firstNonImportIdx = (() => {
  const last = [...tsf.statements].reverse().find((s) => ts.isImportDeclaration(s));
  return last ? last.getEnd() : 0;
})();
newTest = newTest.slice(0, firstNonImportIdx) + "\n" + header.join("\n") + newTest.slice(firstNonImportIdx);
newTest = newTest.replace(/\s*$/, "\n") + tail;

let newModule = moduleText;
for (const b of blocks.sort((a, c) => c.start - a.start)) {
  let end = b.end;
  if (newModule[end] === "\n") end += 1;
  newModule = newModule.slice(0, b.start) + newModule.slice(end);
}
newModule = newModule.replace(/^[ \t]*\/\/ ?#region ([^\n]*)\n(?:[ \t]*\n)*[ \t]*\/\/ ?#endregion [^\n]*\n?/gm, "");

console.log(`module blocks removed: ${blocks.length}; register calls: ${calls.map((c) => c.fn).join(", ")}`);
console.log("--- header ---\n" + header.join("\n"));
console.log("--- tail ---" + tail.slice(0, 1200));
if (mode === "apply") {
  writeFileSync(testPath, newTest);
  writeFileSync(modulePath, newModule);
  console.log("APPLIED");
}
