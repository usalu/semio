// 🧱 Turns imperative 📜️script.ts programs into the router shape: the program moves verbatim into a sibling module and every
// command the program recognises is registered on the router. usage: bun plain-lift.ts <list.txt> [--write] [--only <path>] [--dump]
import {existsSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import ts from "typescript";
import {accepts, ageMinutes, aliasImportText, GEN, ledger, read, recordOwn, REPO, rel, ROUTING, syntaxOk} from "./lib.ts";

type Plan = {ok: true; script: string; module: string; modulePath: string; commands: string[]} | {ok: false; reason: string};
const EMOJI = ["🚪️", "🧭️", "🏭️", "🧪️", "📦️", "🔧️", "🗂️", "🧰️", "🔬️", "🛠️"];

function ids(node: ts.Node): Set<string> {
  const out = new Set<string>();
  const visit = (n: ts.Node): void => {
    if (ts.isIdentifier(n)) {
      const par = n.parent;
      const prop = par && ((ts.isPropertyAccessExpression(par) && par.name === n) || (ts.isPropertyAssignment(par) && par.name === n) || (ts.isQualifiedName(par) && par.right === n));
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
const isDeclaration = (st: ts.Statement): boolean => ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isEnumDeclaration(st);
const mentionsArgv = (n: ts.Node): boolean => /\bprocess\.argv\b/u.test(n.getText()) || /\bawait\b/u.test(n.getText());

function commandNames(tail: ts.Statement[]): string[] {
  const found = new Set<string>();
  const subject = (e: ts.Expression): boolean => {
    if (ts.isIdentifier(e)) return /^(command|cmd|subcommand|mode|verb|action|task|op)$/u.test(e.text);
    if (ts.isElementAccessExpression(e)) { const t = e.expression.getText(); const a = e.argumentExpression.getText(); return (t === "process.argv" && (a === "2")) || (/^(argv|args|rest)$/u.test(t) && a === "0"); }
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

export function planPlain(p: string, src: string): Plan {
  const f = ts.createSourceFile(p, src, ts.ScriptTarget.Latest, true);
  const st = [...f.statements];
  if (st.some((s) => ts.canHaveModifiers(s) && ts.getModifiers(s)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword) || ts.isExportDeclaration(s) || ts.isExportAssignment(s))) return {ok: false, reason: "has-exports"};
  if (st.some((s) => ts.isClassDeclaration(s) && s.heritageClauses?.length)) return {ok: false, reason: "has-classes"};
  if (/\bimport\.meta\.(url|path|vitest)\b/u.test(src)) return {ok: false, reason: "import.meta-use"};
  const modulePath = join(dirname(p), "🟦️.ts");
  if (existsSync(join(REPO, modulePath))) return {ok: false, reason: "no-module-slot"};
  if (/📦️packages\/[^/]+$/u.test(dirname(p))) return {ok: false, reason: "package-dir"};
  const imports = st.filter(ts.isImportDeclaration);
  const rest = st.filter((s) => !ts.isImportDeclaration(s));
  let tailStart = rest.findIndex((s) => !isDeclaration(s) && (!ts.isVariableStatement(s) ? true : mentionsArgv(s)));
  if (tailStart < 0) return {ok: false, reason: "no-program"};
  const head = rest.slice(0, tailStart), tailAll = rest.slice(tailStart);
  const tail = tailAll.filter((s) => !isDeclaration(s)), movedDecl = tailAll.filter(isDeclaration);
  const tailNames = new Set(tail.flatMap(names));
  for (const d of [...head, ...movedDecl]) { const own = locals(d); for (const id of ids(d)) if (tailNames.has(id) && !own.has(id) && !names(d).includes(id)) return {ok: false, reason: `module-level-uses-program-name:${id}`}; }
  const commands = commandNames(tail);
  const fallback = commands.length === 0;
  const first = imports[0] ?? rest[0]!;
  const headerEnd = first.getStart();
  const header = src.slice(0, headerEnd).replace(/\s+$/u, "");
  const bodyText = (s: ts.Statement): string => src.slice(s.getFullStart(), s.end);
  const importLines = imports.map((i) => i.getText());
  const moduleBody = [...head, ...movedDecl].map(bodyText).join("");
  const tailText = tail.map((s) => bodyText(s).replace(/\bimport\.meta\.main\b/gu, "true")).join("");
  const usedInModule = new Set<string>([...head, ...movedDecl, ...tail].flatMap((s) => [...ids(s)]));
  const keptImports: string[] = [];
  for (const i of imports) {
    const c = i.importClause;
    if (!c) { keptImports.push(i.getText()); continue; }
    const binds: {name: string; text: string}[] = [];
    if (c.name) binds.push({name: c.name.text, text: c.name.text});
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) binds.push({name: c.namedBindings.name.text, text: `* as ${c.namedBindings.name.text}`});
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) binds.push({name: e.name.text, text: e.getText()});
    const keep = binds.filter((b) => usedInModule.has(b.name));
    if (!keep.length) continue;
    if (keep.length === binds.length) { keptImports.push(i.getText()); continue; }
    const spec = (i.moduleSpecifier as ts.StringLiteral).text, attrs = i.attributes ? ` ${i.attributes.getText()}` : "";
    const def = c.name && usedInModule.has(c.name.text) ? c.name.text : null;
    const ns = keep.find((b) => b.text.startsWith("* as "));
    const named = keep.filter((b) => b !== ns && b.name !== def);
    keptImports.push(`import ${[def, ns?.text, named.length ? `{ ${named.map((b) => b.text).join(", ")} }` : null].filter(Boolean).join(", ")} from "${spec}"${attrs};`);
  }
  const doc = "/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */\n";
  const module = `${keptImports.map((text) => aliasImportText(text, dirname(join(REPO, p)))).join("\n")}\n${moduleBody}\n${doc}export async function runScriptProgram(): Promise<void> {${tailText.replace(/\n/g, "\n  ")}\n}\n`;
  const routingImport = rel(p, ROUTING);
  const list = fallback ? ["run"] : commands;
  const cls = (c: string, i: number): string => {
    const name = c.split(/[^A-Za-z0-9]+/u).filter(Boolean).map((w) => w[0]!.toUpperCase() + w.slice(1)).join("") + "Script";
    return `/** ${EMOJI[i % EMOJI.length]} \`${c}\` — runs the program of this script. */\nclass ${/^[0-9]/u.test(name) ? "Command" + name : name} extends BundleScript {\n  async run(): Promise<void> {\n    await runScriptProgram();\n  }\n}\n`;
  };
  const clsNames = list.map((c) => { const n = c.split(/[^A-Za-z0-9]+/u).filter(Boolean).map((w) => w[0]!.toUpperCase() + w.slice(1)).join("") + "Script"; return /^[0-9]/u.test(n) ? "Command" + n : n; });
  if (new Set(clsNames).size !== clsNames.length) return {ok: false, reason: "command-class-collision"};
  const script = `${header ? header + "\n" : ""}import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "${routingImport}";\nimport { runScriptProgram } from "./🟦️.ts";\n\n${list.map(cls).join("")}const router = new ScriptRouter(import.meta.dir)${list.map((c, i) => `.register(${JSON.stringify(c)}, ${clsNames[i]})`).join("")};\nif (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original${fallback ? ', defaultCommand: "run"' : ""} }));\n`;
  return {ok: true, script, module, modulePath, commands: list};
}

if (import.meta.main) {
  const write = process.argv.includes("--write"), dump = process.argv.includes("--dump");
  const only = process.argv.includes("--only") ? process.argv[process.argv.indexOf("--only") + 1] : null;
  const tally: Record<string, number> = {};
  const count = (k: string) => (tally[k] = (tally[k] ?? 0) + 1);
  for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
    if (only && p !== only) continue;
    const src = read(p);
    let plan: Plan;
    try { plan = planPlain(p, src); } catch (e) { count("error"); ledger({tool: "plain-lift", p, result: "error", message: String(e).slice(0, 200)}); continue; }
    if (!plan.ok) { count(plan.reason.split(":")[0]!); ledger({tool: "plain-lift", p, result: plan.reason}); continue; }
    if (dump) { writeFileSync(`${GEN}/dump-plain-script.ts`, plan.script); writeFileSync(`${GEN}/dump-plain-module.ts`, plan.module); }
    const sOk = syntaxOk(p, plan.script), mOk = syntaxOk(plan.modulePath, plan.module);
    const accepted = sOk && accepts(plan.script);
    if (!sOk || !mOk) { count("syntax-fail"); ledger({tool: "plain-lift", p, result: "syntax-fail", sOk, mOk}); continue; }
    if (!accepted) { count("still-rejected"); ledger({tool: "plain-lift", p, result: "still-rejected"}); continue; }
    if (!write) { count("ok-accepted"); continue; }
    if (ageMinutes(p) < 15) { count("skip-recent"); continue; }
    if (read(p) !== src) { count("skip-changed"); continue; }
    writeFileSync(join(REPO, plan.modulePath), plan.module);
    writeFileSync(join(REPO, p), plan.script); recordOwn(p);
    count("written-accepted");
    ledger({tool: "plain-lift", p, result: "written-accepted", module: plan.modulePath, commands: plan.commands});
  }
  console.log(tally);
}
