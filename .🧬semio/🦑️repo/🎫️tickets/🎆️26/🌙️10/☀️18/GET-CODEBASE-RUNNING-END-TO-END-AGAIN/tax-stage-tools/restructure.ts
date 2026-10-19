// 🏛️ Splits a rejected 📜️script.ts into a router skeleton and an owner module (classes intact, declarations, exports, imperative program).
// usage: bun restructure.ts <list.txt> [--write] [--only <path>] [--dump] [--reuse]
import {existsSync, mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {basename, dirname, join, relative, resolve} from "node:path";
import ts from "typescript";
import {accepts, ageMinutes, aliasSpecifier, GEN, ledger, read, recordOwn, REPO, rel, ROUTING, syntaxOk} from "./lib.ts";
import {applyRepoint, buildIndex, planRepoint, termsOf} from "./repoint.ts";

type Plan = {ok: true; script: string; module: string; modulePath: string; mergedInto: boolean; exported: boolean; commands: string[]; moved: string[]} | {ok: false; reason: string};
const FROZEN = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"];
const frozen = (p: string): boolean => p === "📜️script.ts" || /(^|\/)(package\.json|bun\.lock|🔣️taxonomy\.json)$/u.test(p);
const EMOJI = ["🚪️", "🧭️", "🏭️", "🧪️", "📦️", "🔧️", "🗂️", "🧰️", "🔬️", "🛠️"];
const TERMINAL = /^(?:await\s+)?(?:receiveScriptProcessInvocation|runScriptMain|runRepoScriptMain|runWorkspaceScriptMain|runPolicyOnlyMain|runArtifactRustPackageMain|runArtifactTypeScriptPackageMain)\b/u;
const SCRIPT_BASES = new Set(["BundleScript", "Script"]);
const myModules = (): Set<string> => { const out = new Set<string>(); try { for (const l of readFileSync(`${GEN}/ledger.jsonl`, "utf8").split("\n")) { if (!l) continue; const r = JSON.parse(l); if (r.module && String(r.result).startsWith("written")) out.add(r.module); } } catch { /* none */ } return out; };

function ids(node: ts.Node): Set<string> {
  const out = new Set<string>();
  const visit = (n: ts.Node): void => {
    if (ts.isIdentifier(n)) {
      const par = n.parent;
      const prop = par && ((ts.isPropertyAccessExpression(par) && par.name === n) || (ts.isPropertyAssignment(par) && par.name === n) || (ts.isQualifiedName(par) && par.right === n) || (ts.isMethodDeclaration(par) && par.name === n) || (ts.isPropertyDeclaration(par) && par.name === n));
      if (!prop) out.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
  return out;
}
function locals(node: ts.Node): Set<string> {
  const out = new Set<string>();
  const bind = (b: ts.BindingName): void => { if (ts.isIdentifier(b)) out.add(b.text); else for (const e of b.elements) if (!ts.isOmittedExpression(e)) bind(e.name); };
  const visit = (n: ts.Node): void => {
    if (ts.isParameter(n) || ts.isVariableDeclaration(n)) bind(n.name);
    if ((ts.isFunctionDeclaration(n) || ts.isFunctionExpression(n)) && n.name) out.add(n.name.text);
    ts.forEachChild(n, visit);
  };
  visit(node);
  return out;
}
const names = (st: ts.Statement): string[] => {
  if (ts.isVariableStatement(st)) return st.declarationList.declarations.flatMap((d) => { const out: string[] = []; const walk = (b: ts.BindingName): void => { if (ts.isIdentifier(b)) out.push(b.text); else for (const e of b.elements) if (!ts.isOmittedExpression(e)) walk(e.name); }; walk(d.name); return out; });
  if (ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isEnumDeclaration(st)) return st.name ? [st.name.text] : [];
  return [];
};
const isDeclaration = (st: ts.Statement): boolean => ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isEnumDeclaration(st) || ts.isVariableStatement(st);
const isExported = (st: ts.Statement): boolean => !!(ts.canHaveModifiers(st) && ts.getModifiers(st)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword));
const exportsDeclaration = (st: ts.Statement): boolean => ts.isExportDeclaration(st) || ts.isExportAssignment(st);

function importBindings(st: ts.ImportDeclaration): {name: string; text: string; imported: string}[] {
  const c = st.importClause;
  if (!c) return [];
  const rows: {name: string; text: string; imported: string}[] = [];
  if (c.name) rows.push({name: c.name.text, text: c.name.text, imported: "default"});
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) rows.push({name: c.namedBindings.name.text, text: `* as ${c.namedBindings.name.text}`, imported: "*"});
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) rows.push({name: e.name.text, text: e.getText(), imported: (e.propertyName ?? e.name).text});
  return rows;
}

function commandNames(tail: ts.Statement[]): string[] {
  const found = new Set<string>();
  const subject = (e: ts.Expression): boolean => {
    if (ts.isIdentifier(e)) return /^(command|cmd|subcommand|mode|verb|action|task|op)$/u.test(e.text);
    if (ts.isElementAccessExpression(e)) { const t = e.expression.getText(), a = e.argumentExpression.getText(); return (t === "process.argv" && a === "2") || (/^(argv|args|rest)$/u.test(t) && a === "0"); }
    if (ts.isBinaryExpression(e) && e.operatorToken.kind === ts.SyntaxKind.QuestionQuestionToken) return subject(e.left);
    if (ts.isParenthesizedExpression(e)) return subject(e.expression);
    return false;
  };
  const visit = (n: ts.Node): void => {
    if (ts.isBinaryExpression(n) && [ts.SyntaxKind.EqualsEqualsEqualsToken, ts.SyntaxKind.EqualsEqualsToken].includes(n.operatorToken.kind)) {
      if (subject(n.left) && ts.isStringLiteral(n.right)) found.add(n.right.text);
      else if (subject(n.right) && ts.isStringLiteral(n.left)) found.add(n.left.text);
    }
    if (ts.isCaseClause(n) && ts.isStringLiteral(n.expression)) found.add(n.expression.text);
    if (ts.isCallExpression(n) && ts.isPropertyAccessExpression(n.expression) && n.expression.name.text === "includes" && ts.isArrayLiteralExpression(n.expression.expression) && n.arguments[0] && subject(n.arguments[0] as ts.Expression)) for (const e of n.expression.expression.elements) if (ts.isStringLiteral(e)) found.add(e.text);
    ts.forEachChild(n, visit);
  };
  for (const st of tail) visit(st);
  return [...found].filter((c) => /^[A-Za-z0-9][\w-]*$/u.test(c));
}

function skeletonKind(st: ts.Statement): "skeleton" | "program" | "declaration" | "import" {
  if (ts.isImportDeclaration(st)) return "import";
  if (ts.isVariableStatement(st)) {
    const text = st.getText();
    if (!isExported(st) && /\bnew\s+ScriptRouter\b/u.test(text) && !/\bawait\b/u.test(text)) return "skeleton";
    if (/\bprocess\.argv\b|\bawait\b/u.test(text)) return "program";
    return "declaration";
  }
  if (isDeclaration(st)) return "declaration";
  if (ts.isExpressionStatement(st)) {
    const text = st.expression.getText();
    if (TERMINAL.test(text) || /^[A-Za-z_$][\w$]*\s*\.\s*register\s*\(/u.test(text) || /^process\.env\.[A-Z0-9_]+\s*\?\?=/u.test(text)) return "skeleton";
    return "program";
  }
  if (ts.isIfStatement(st)) {
    const c = st.expression.getText();
    if (c === "import.meta.main" && !st.elseStatement) {
      const inner = ts.isBlock(st.thenStatement) ? [...st.thenStatement.statements] : [st.thenStatement];
      if (inner.every((i) => ts.isExpressionStatement(i) && TERMINAL.test(i.expression.getText()))) return "skeleton";
    }
    return "program";
  }
  return "program";
}

/** 📍️ The owner module slot of a package script: `<owner>/🧪️tests/🔬️<lang>-package-commands`. It cannot sit in `📦️packages/<lang>` because the taxonomy classifies implementation there as `package-implementation-file` (probed in tax2b); cross-technology imports of the module are emitted as aliased workspace specifiers (`aliasSpecifier`). */
const ownerOf = (p: string): {dir: string; slug: string} | null => {
  const parts = p.split("/");
  const lang = parts.at(-2);
  if (parts.at(-3) === "📦️packages" && lang) {
    const name = ({"🦀️rust": "rust", "🟦️typescript": "typescript", "🐹️go": "go", "🐍️python": "python"} as Record<string, string>)[lang];
    return name ? {dir: parts.slice(0, -3).join("/"), slug: `🔬️${name}-package-commands`} : null;
  }
  return null;
};

export function planRestructure(p: string, src: string): Plan {
  const f = ts.createSourceFile(p, src, ts.ScriptTarget.Latest, true);
  const st = [...f.statements];
  const stripped = src.replace(/dirname\(\s*fileURLToPath\(\s*import\.meta\.url\s*\)\s*\)/gu, "").replace(/new URL\([^()]*(?:\([^()]*\)[^()]*)*import\.meta\.url\s*\)/gu, "").replace(/dirname\(\s*import\.meta\.path\s*\)/gu, "");
  if (/\bimport\.meta\.(url|path|vitest)\b/u.test(stripped)) return {ok: false, reason: "import.meta-use"};
  if (st.some(exportsDeclaration)) return {ok: false, reason: "export-declaration"};
  const owner = ownerOf(p);
  const sibling = !owner;
  const modulePath = owner ? join(owner.dir, "🧪️tests", owner.slug, "🟦️.ts") : join(dirname(p), "🟦️.ts");
  if (frozen(modulePath)) return {ok: false, reason: "frozen-module"};
  const exists = existsSync(join(REPO, modulePath));
  if (!owner && /📦️packages\/[^/]+$/u.test(dirname(p))) return {ok: false, reason: "package-dir"};
  const scriptDir = dirname(join(REPO, p)), moduleDir = dirname(join(REPO, modulePath));
  const rebaseSpec = (spec: string): string => { if (!spec.startsWith(".")) return spec; const abs = resolve(scriptDir, spec), alias = aliasSpecifier(moduleDir, abs); if (alias) return alias; if (sibling) return spec; const r = relative(moduleDir, abs); return r.startsWith(".") ? r : "./" + r; };
  const imports = st.filter(ts.isImportDeclaration);
  const bindings = new Map<string, {decl: ts.ImportDeclaration; imported: string}>();
  for (const d of imports) for (const b of importBindings(d)) bindings.set(b.name, {decl: d, imported: b.imported});
  const kinds = new Map<ts.Statement, ReturnType<typeof skeletonKind>>();
  for (const s of st) kinds.set(s, skeletonKind(s));
  const skeleton = st.filter((s) => kinds.get(s) === "skeleton");
  const program = st.filter((s) => kinds.get(s) === "program");
  const decls = st.filter((s) => kinds.get(s) === "declaration");
  const terminals = skeleton.filter((s) => (ts.isExpressionStatement(s) && TERMINAL.test(s.expression.getText())) || ts.isIfStatement(s));
  if (program.length && terminals.length) return {ok: false, reason: "program-and-terminal"};
  const declOf = new Map<string, ts.Statement>();
  for (const d of decls) for (const n of names(d)) declOf.set(n, d);
  const sRouting = [...bindings.values()].find((b) => b.imported === "ScriptRouter")?.decl.moduleSpecifier;
  const routingSpec = sRouting && ts.isStringLiteral(sRouting) ? sRouting.text : rel(p, ROUTING);
  const isScriptClass = (d: ts.Statement): d is ts.ClassDeclaration => ts.isClassDeclaration(d) && !!d.heritageClauses?.[0] && ts.isIdentifier(d.heritageClauses[0].types[0]!.expression) && (SCRIPT_BASES.has(bindings.get((d.heritageClauses[0].types[0]!.expression as ts.Identifier).text)?.imported ?? "") || declOf.has((d.heritageClauses[0].types[0]!.expression as ts.Identifier).text));
  const stub = (n: string, self: ts.Statement): string => { const d = declOf.get(n); if (!d || d === self || bindings.has(n)) return ""; return ts.isClassDeclaration(d) && isScriptClass(d) ? `class ${n} extends __BS { async run(): Promise<void> { const __x = "x"; } }` : `const ${n} = "x";`; };
  const synth = (d: ts.Statement): string => {
    const used = [...ids(d)];
    const clsStubs = used.filter((n) => declOf.has(n) && declOf.get(n) !== d && ts.isClassDeclaration(declOf.get(n)!) && isScriptClass(declOf.get(n) as ts.ClassDeclaration));
    return [...imports.map((i) => i.getText()), `import {ScriptRouter as __SR, runScriptMain as __RM, receiveScriptProcessInvocation as __RP, BundleScript as __BS} from "${routingSpec}";`, ...used.map((n) => stub(n, d)).filter(Boolean), d.getText(), `const __router = new __SR(import.meta.dir)${(ts.isClassDeclaration(d) ? [d.name!.text] : []).concat(clsStubs).map((n, i) => `.register("k${i}", ${n})`).join("")};`, `await __RP(process.env, original => __RM(__router, { invocation: original }));`].join("\n");
  };
  const movedSet = new Set<ts.Statement>();
  for (const d of decls) {
    if (isExported(d)) { movedSet.add(d); continue; }
    if (ts.isClassDeclaration(d) && !isScriptClass(d)) { movedSet.add(d); continue; }
    if (ts.isFunctionDeclaration(d) || ts.isTypeAliasDeclaration(d) || ts.isInterfaceDeclaration(d) || ts.isEnumDeclaration(d)) { movedSet.add(d); continue; }
    if (!accepts(synth(d))) movedSet.add(d);
  }
  let changed = true;
  while (changed) {
    changed = false;
    for (const d of decls) {
      if (movedSet.has(d)) continue;
      if (ts.isClassDeclaration(d)) {
        const base = d.heritageClauses?.[0]?.types[0]?.expression;
        if (base && ts.isIdentifier(base) && declOf.has(base.text) && movedSet.has(declOf.get(base.text)!)) { movedSet.add(d); changed = true; continue; }
      }
      for (const m of [...movedSet]) for (const id of ids(m)) if (declOf.get(id) === d && !movedSet.has(d)) { movedSet.add(d); changed = true; }
    }
  }
  const moved = decls.filter((d) => movedSet.has(d));
  if (!moved.length && !program.length) return {ok: false, reason: "nothing-to-move"};
  const programNames = new Set(program.flatMap(names));
  for (const d of decls) { if (!movedSet.has(d)) continue; const own = locals(d); for (const id of ids(d)) if (programNames.has(id) && !own.has(id) && !names(d).includes(id)) return {ok: false, reason: `module-level-uses-program-name:${id}`}; }
  for (const d of decls) { if (!movedSet.has(d) && ts.isVariableStatement(d) && d.declarationList.flags & ts.NodeFlags.Let) { for (const m of moved) for (const id of ids(m)) if (names(d).includes(id)) return {ok: false, reason: "let-shared"}; } }
  for (const d of moved) if (ts.isVariableStatement(d) && d.declarationList.flags & ts.NodeFlags.Let) { const n = names(d); for (const s of [...skeleton, ...decls.filter((x) => !movedSet.has(x))]) for (const id of ids(s)) if (n.includes(id)) return {ok: false, reason: "let-shared"}; }
  const skeletonIds = new Set([...skeleton, ...decls.filter((d) => !movedSet.has(d))].flatMap((s) => [...ids(s)]));
  const movedNames = new Set(moved.flatMap(names));
  const needExport = new Set([...skeletonIds].filter((n) => movedNames.has(n)));
  let usesScriptDir = false;
  const edit = (node: ts.Node): string | null => {
    const start = node.getFullStart();
    const edits: {s: number; e: number; t: string}[] = [];
    let bail: string | null = null;
    const inClass = (n: ts.Node): boolean => { for (let q: ts.Node | undefined = n; q; q = q.parent) if (ts.isClassDeclaration(q) || ts.isClassExpression(q)) return true; return false; };
    const visit = (n: ts.Node): void => {
      if (bail) return;
      if (ts.isMetaProperty(n)) {
        const par = n.parent;
        if (sibling) return;
        if (ts.isPropertyAccessExpression(par) && par.name.text === "dir") { if (inClass(n)) edits.push({s: par.getStart(), e: par.end, t: "this.root"}); else { edits.push({s: par.getStart(), e: par.end, t: "SCRIPT_DIR"}); usesScriptDir = true; } return; }
        if (ts.isPropertyAccessExpression(par) && par.name.text === "main") return;
        bail = "import.meta-relocated"; return;
      }
      if (!sibling && ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0]!)) { const lit = n.arguments[0] as ts.StringLiteral; edits.push({s: lit.getStart(), e: lit.end, t: JSON.stringify(rebaseSpec(lit.text))}); }
      if (ts.isIdentifier(n) && n.text === "__dirname") { bail = "__dirname"; return; }
      ts.forEachChild(n, visit);
    };
    visit(node);
    if (bail) return null;
    let text = src.slice(start, node.end);
    for (const e of edits.sort((a, b) => b.s - a.s)) text = text.slice(0, e.s - start) + e.t + text.slice(e.e - start);
    return text;
  };
  const exportize = (s: ts.Statement, t: string): string => (isExported(s) || !names(s).some((n) => needExport.has(n))) ? t : t.replace(/^(\s*(?:\/\*[\s\S]*?\*\/\s*|\/\/[^\n]*\n\s*)*)/u, (m) => m + "export ");
  const movedTexts: string[] = [];
  for (const d of moved) { const t = edit(d); if (t === null) return {ok: false, reason: "moved-bail"}; movedTexts.push(exportize(d, t)); }
  const tailTexts: string[] = [];
  for (const s of program) { const t = edit(s); if (t === null) return {ok: false, reason: "program-bail"}; tailTexts.push(t.replace(/\bimport\.meta\.main\b/gu, "true")); }
  const moduleIds = new Set<string>([...moved, ...program].flatMap((s) => [...ids(s)]));
  const doc = "/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */\n";
  const programFn = program.length ? `${doc}export async function runScriptProgram(): Promise<void> {${tailTexts.join("").replace(/\n/g, "\n  ")}\n}\n` : "";
  const keepImports = (used: Set<string>, rebase: boolean): string[] => {
    const out: string[] = [];
    for (const i of imports) {
      const c = i.importClause, spec = (i.moduleSpecifier as ts.StringLiteral).text, newSpec = rebase ? rebaseSpec(spec) : spec, attrs = i.attributes ? ` ${i.attributes.getText()}` : "";
      if (!c) { out.push(`import "${newSpec}"${attrs};`); continue; }
      const keep = importBindings(i).filter((b) => used.has(b.name));
      if (!keep.length) continue;
      const def = keep.find((b) => b.imported === "default"), ns = keep.find((b) => b.imported === "*"), named = keep.filter((b) => b !== def && b !== ns);
      out.push(`import ${c.isTypeOnly ? "type " : ""}${[def?.name, ns?.text, named.length ? `{ ${named.map((b) => b.text).join(", ")} }` : null].filter(Boolean).join(", ")} from "${newSpec}"${attrs};`);
    }
    return out;
  };
  const moduleImports = keepImports(moduleIds, true);
  if (usesScriptDir) moduleImports.push(`import { resolve as scriptDirectoryResolve } from "node:path";`);
  const scriptDirConst = usesScriptDir && !exists ? `const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, ${JSON.stringify(relative(moduleDir, scriptDir) || ".")});\n\n` : "";
  let moduleText: string, merged = false;
  if (exists) {
    merged = true;
    const existing = readFileSync(join(REPO, modulePath), "utf8");
    const ef = ts.createSourceFile(modulePath, existing, ts.ScriptTarget.Latest, true);
    const have = new Map<string, string>();
    const declared = new Set<string>();
    for (const s of ef.statements) { if (ts.isImportDeclaration(s)) for (const b of importBindings(s)) have.set(b.name, `${(s.moduleSpecifier as ts.StringLiteral).text}#${b.imported}`); for (const n of names(s)) declared.add(n); }
    for (const n of movedNames) if (declared.has(n) || have.has(n)) return {ok: false, reason: `merge-conflict:${n}`};
    if (usesScriptDir) return {ok: false, reason: "merge-script-dir"};
    const newImports: string[] = [];
    try {
      for (const line of moduleImports) {
        const d = ts.createSourceFile("i.ts", line, ts.ScriptTarget.Latest, true).statements[0] as ts.ImportDeclaration;
        const spec = (d.moduleSpecifier as ts.StringLiteral).text;
        const all = importBindings(d);
        const need = all.filter((b) => { const k = `${spec}#${b.imported}`; const cur = have.get(b.name); if (cur === k) return false; if (cur || declared.has(b.name)) throw new Error("import-conflict"); return true; });
        if (need.length === all.length) newImports.push(line);
        else if (need.length) newImports.push(`import { ${need.map((b) => b.text).join(", ")} } from "${spec}";`);
      }
    } catch { return {ok: false, reason: "merge-import-conflict"}; }
    const lastImp = ef.statements.filter(ts.isImportDeclaration).at(-1);
    const insertAt = lastImp ? lastImp.end : 0;
    moduleText = existing.slice(0, insertAt) + (newImports.length ? "\n" + newImports.join("\n") : "") + existing.slice(insertAt).replace(/\s*$/u, "\n") + "\n" + movedTexts.join("") + (programFn ? "\n" + programFn : "");
  } else {
    moduleText = `${moduleImports.join("\n")}\n\n${scriptDirConst}${movedTexts.join("")}\n${programFn}`;
  }
  const first = imports[0] ?? st[0]!;
  const header = src.slice(0, first.getStart()).replace(/\s+$/u, "");
  const kept = st.filter((s) => skeleton.includes(s) || (kinds.get(s) === "declaration" && !movedSet.has(s)));
  const keptTexts: string[] = [];
  const routerConsts = new Map<string, number>();
  for (const s of kept) {
    let text = src.slice(s.getFullStart(), s.end);
    if (ts.isVariableStatement(s) && s.declarationList.declarations.length === 1 && ts.isIdentifier(s.declarationList.declarations[0]!.name) && /\bnew\s+ScriptRouter\b/u.test(text)) routerConsts.set((s.declarationList.declarations[0]!.name as ts.Identifier).text, keptTexts.length);
    if (ts.isExpressionStatement(s)) {
      const m = /^([A-Za-z_$][\w$]*)(\s*\.\s*register\s*\([\s\S]*\))\s*;?\s*$/u.exec(s.expression.getText());
      if (m && routerConsts.has(m[1]!)) { const at = routerConsts.get(m[1]!)!; keptTexts[at] = keptTexts[at]!.replace(/;?\s*$/u, "") + m[2] + ";"; continue; }
      const env = /^(process\.env\.[A-Z0-9_]+\s*\?\?=\s*)String\(\s*([0-9_*+\-() ]+)\s*\)\s*;?$/u.exec(s.expression.getText());
      if (env) { try { const v = Function(`"use strict"; return (${env[2]!.replace(/_/g, "")});`)(); text = text.replace(/process\.env[\s\S]*$/u, `${env[1]}${JSON.stringify(String(v))};`); } catch { /* keep */ } }
    }
    keptTexts.push(text);
  }
  const skeletonText = keptTexts.join("");
  let extra = "", commands: string[] = [];
  const usedForScript = new Set<string>(kept.flatMap((s) => [...ids(s)]));
  const importNames = [...needExport];
  if (program.length) {
    commands = commandNames(program);
    const fallback = commands.length === 0, list = fallback ? ["run"] : commands;
    const cn = (c: string): string => { const n = c.split(/[^A-Za-z0-9]+/u).filter(Boolean).map((w) => w[0]!.toUpperCase() + w.slice(1)).join("") + "Script"; return /^[0-9]/u.test(n) ? "Command" + n : n; };
    const clsNames = list.map(cn);
    if (new Set(clsNames).size !== clsNames.length) return {ok: false, reason: "command-class-collision"};
    extra = `${list.map((c, i) => `/** ${EMOJI[i % EMOJI.length]} \`${c}\` — runs the program of this script. */\nclass ${clsNames[i]} extends BundleScript {\n  async run(): Promise<void> {\n    await runScriptProgram();\n  }\n}\n`).join("")}const router = new ScriptRouter(import.meta.dir)${list.map((c, i) => `.register(${JSON.stringify(c)}, ${clsNames[i]})`).join("")};\nif (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original${fallback ? ', defaultCommand: "run"' : ""} }));\n`;
    for (const n of ["BundleScript", "ScriptRouter", "receiveScriptProcessInvocation", "runScriptMain"]) usedForScript.add(n);
    if (fallback) commands = ["run"];
    importNames.push("runScriptProgram");
  }
  const neededRouting = ["BundleScript", "ScriptRouter", "receiveScriptProcessInvocation", "runScriptMain"].filter((n) => program.length && !bindings.has(n));
  const routingImport = neededRouting.length ? `import { ${neededRouting.join(", ")} } from "${rel(p, ROUTING)}";\n` : "";
  const scriptImports = keepImports(usedForScript, false);
  const moduleSpec = (() => { const r = relative(scriptDir, join(REPO, modulePath)); return r.startsWith(".") ? r : "./" + r; })();
  const moduleImportLine = importNames.length ? `import { ${importNames.join(", ")} } from "${moduleSpec}";\n` : "";
  const script = `${header ? header + "\n" : ""}${scriptImports.join("\n")}${scriptImports.length ? "\n" : ""}${routingImport}${moduleImportLine}${skeletonText}${extra}`;
  return {ok: true, script, module: moduleText, modulePath, mergedInto: merged, exported: st.some(isExported), commands, moved: [...movedNames]};
}

if (import.meta.main) {
  const write = process.argv.includes("--write"), dump = process.argv.includes("--dump"), reuse = process.argv.includes("--reuse");
  const only = process.argv.includes("--only") ? process.argv[process.argv.indexOf("--only") + 1] : null;
  const tally: Record<string, number> = {};
  const count = (k: string) => (tally[k] = (tally[k] ?? 0) + 1);
  const ready: {p: string; src: string; plan: Extract<Plan, {ok: true}>}[] = [];
  const claimed = new Set<string>();
  for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
    if (only && p !== only) continue;
    if (frozen(p)) { count("frozen"); continue; }
    const src = read(p);
    let plan: Plan;
    try { plan = planRestructure(p, src); } catch (e) { count("error"); ledger({tool: "restructure", p, result: "error", message: String(e).slice(0, 200)}); continue; }
    if (!plan.ok) { count(plan.reason.split(":")[0]!); ledger({tool: "restructure", p, result: plan.reason}); continue; }
    if (dump) { writeFileSync(`${GEN}/dump-restr-script.ts`, plan.script); writeFileSync(`${GEN}/dump-restr-module.ts`, plan.module); }
    const sOk = syntaxOk(p, plan.script), mOk = syntaxOk(plan.modulePath, plan.module);
    if (!sOk || !mOk) { count("syntax-fail"); ledger({tool: "restructure", p, result: "syntax-fail", sOk, mOk}); continue; }
    if (!accepts(plan.script)) { count("still-rejected"); ledger({tool: "restructure", p, result: "still-rejected"}); continue; }
    if (claimed.has(plan.modulePath)) { count("module-claimed"); continue; }
    claimed.add(plan.modulePath);
    ready.push({p, src, plan});
  }
  if (!write) { console.log({...tally, "ok-accepted": ready.length}); process.exit(0); }
  buildIndex(ready.filter((r) => r.plan.exported).flatMap((r) => termsOf(r.p)), basename(process.argv[2]!), reuse);
  for (const {p, src, plan} of ready) {
    if (ageMinutes(p) < 15) { count("skip-recent"); continue; }
    if (read(p) !== src) { count("skip-changed"); continue; }
    const repoint = plan.exported ? planRepoint(p, plan.modulePath, new Set([p]), true) : {edits: [], recent: []};
    if (repoint.recent.length || repoint.edits.some((e) => frozen(e.file))) { count("skip-referencer-recent-or-frozen"); ledger({tool: "restructure", p, result: "skip-referencer-recent-or-frozen", recent: repoint.recent, frozenRefs: repoint.edits.filter((e) => frozen(e.file)).map((e) => e.file)}); continue; }
    mkdirSync(dirname(join(REPO, plan.modulePath)), {recursive: true});
    writeFileSync(join(REPO, plan.modulePath), plan.module);
    writeFileSync(join(REPO, p), plan.script); recordOwn(p);
    applyRepoint(repoint);
    count("written-accepted");
    ledger({tool: "restructure", p, result: "written-accepted", module: plan.modulePath, merged: plan.mergedInto, commands: plan.commands, referencers: repoint.edits.map((e) => e.file)});
  }
  console.log(tally);
}
