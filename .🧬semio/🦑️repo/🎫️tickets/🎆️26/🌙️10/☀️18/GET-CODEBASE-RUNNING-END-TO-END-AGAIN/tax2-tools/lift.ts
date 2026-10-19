// 🏗️ Lifts command-class bodies of rejected 📜️script.ts routers into an owner module and leaves one-line delegations.
// usage: bun lift.ts <list.txt> [--write] [--only <path>] [--dump]
import {existsSync, mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
import ts from "typescript";
import {accepts, ageMinutes, aliasSpecifier, GEN, ledger, read, recordOwn, REPO, rel, ROUTING, syntaxOk} from "./lib.ts";

type Plan = {ok: true; script: string; module: string; modulePath: string; lifted: string[]} | {ok: false; reason: string};
const SCRIPT_BASES = new Set(["BundleScript", "Script"]);

/** 📍️ The owner module slot of a package script: `<owner>/🧪️tests/🔬️<lang>-package-commands`. It cannot sit in `📦️packages/<lang>` because the taxonomy classifies implementation there as `package-implementation-file` (probed in tax2b); cross-technology imports of the module are emitted as aliased workspace specifiers (`aliasSpecifier`). */
const ownerOf = (p: string): {dir: string; slug: string} | null => {
  const parts = p.split("/");
  const lang = parts.at(-2);
  if (parts.at(-3) === "📦️packages" && lang) {
    const owner = parts.slice(0, -3).join("/");
    const name = ({"🦀️rust": "rust", "🟦️typescript": "typescript", "🐹️go": "go", "🐍️python": "python"} as Record<string, string>)[lang];
    return name ? {dir: owner, slug: `🔬️${name}-package-commands`} : null;
  }
  return null;
};

function usedIdentifiers(node: ts.Node, skip?: (n: ts.Node) => boolean): Set<string> {
  const out = new Set<string>();
  const visit = (n: ts.Node): void => {
    if (skip?.(n)) return;
    if (ts.isIdentifier(n)) {
      const parent = n.parent;
      const isPropName = parent && ((ts.isPropertyAccessExpression(parent) && parent.name === n) || (ts.isPropertyAssignment(parent) && parent.name === n) || (ts.isPropertySignature(parent) && parent.name === n) || (ts.isQualifiedName(parent) && parent.right === n) || (ts.isMethodDeclaration(parent) && parent.name === n) || (ts.isPropertyDeclaration(parent) && parent.name === n));
      if (!isPropName) out.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
  return out;
}

function declaredNames(st: ts.Statement): string[] {
  if (ts.isVariableStatement(st)) return st.declarationList.declarations.flatMap((d) => ts.isIdentifier(d.name) ? [d.name.text] : d.name.elements.flatMap((e) => ts.isIdentifier((e as ts.BindingElement).name) ? [((e as ts.BindingElement).name as ts.Identifier).text] : []));
  if (ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isEnumDeclaration(st)) return st.name ? [st.name.text] : [];
  return [];
}

function importBindings(st: ts.ImportDeclaration): {name: string; text: string; typeOnly: boolean; imported: string}[] {
  const c = st.importClause;
  if (!c) return [];
  const rows: {name: string; text: string; typeOnly: boolean; imported: string}[] = [];
  if (c.name) rows.push({name: c.name.text, text: c.name.text, typeOnly: c.isTypeOnly, imported: "default"});
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) rows.push({name: c.namedBindings.name.text, text: `* as ${c.namedBindings.name.text}`, typeOnly: c.isTypeOnly, imported: "*"});
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) rows.push({name: e.name.text, text: e.getText(), typeOnly: c.isTypeOnly || e.isTypeOnly, imported: (e.propertyName ?? e.name).text});
  return rows;
}

export function planLift(p: string, src: string): Plan {
  const owner = ownerOf(p);
  const f = ts.createSourceFile(p, src, ts.ScriptTarget.Latest, true);
  const imports = f.statements.filter(ts.isImportDeclaration);
  const bindings = new Map<string, {decl: ts.ImportDeclaration; text: string; typeOnly: boolean; imported: string}>();
  for (const d of imports) for (const b of importBindings(d)) bindings.set(b.name, {decl: d, ...b});
  const declOf = new Map<string, ts.Statement>();
  for (const st of f.statements) for (const n of declaredNames(st)) declOf.set(n, st);
  const stubs = (used: Set<string>, self: ts.Statement): string[] => [...used].filter((n) => declOf.has(n) && declOf.get(n) !== self && !bindings.has(n)).map((n) => ts.isClassDeclaration(declOf.get(n)!) ? `class ${n} extends __BS { async run(): Promise<void> { const __x = "x"; } }` : `const ${n} = "x";`);
  if (f.statements.some((st) => ts.canHaveModifiers(st) && ts.getModifiers(st)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword) || ts.isExportDeclaration(st) || ts.isExportAssignment(st))) return {ok: false, reason: "has-exports"};
  const classes = f.statements.filter((s): s is ts.ClassDeclaration => ts.isClassDeclaration(s));
  const routingSpecifier = [...bindings.values()].find((b) => b.imported === "ScriptRouter")?.decl.moduleSpecifier;
  const sRouting = routingSpecifier && ts.isStringLiteral(routingSpecifier) ? routingSpecifier.text : rel(p, ROUTING);

  const needsLift: ts.ClassDeclaration[] = [];
  for (const c of classes) {
    const base = c.heritageClauses?.[0]?.types[0]?.expression;
    if (!base || !ts.isIdentifier(base) || !bindings.has(base.text) || !SCRIPT_BASES.has(bindings.get(base.text)!.imported)) continue;
    const used = usedIdentifiers(c);
    const synthetic = [
      ...imports.map((i) => i.getText()),
      `import {ScriptRouter as __SR, runScriptMain as __RM, receiveScriptProcessInvocation as __RP, BundleScript as __BS} from "${sRouting}";`,
      ...stubs(used, c),
      c.getText(),
      `const __router = new __SR(import.meta.dir).register("x", ${c.name!.text});`,
      `await __RP(process.env, original => __RM(__router, { invocation: original }));`,
    ].join("\n");
    if (!accepts(synthetic)) needsLift.push(c);
  }
  const invalidConsts: ts.Statement[] = [];
  for (const st of f.statements) {
    if (!ts.isVariableStatement(st)) continue;
    const used = usedIdentifiers(st);
    const synthetic = [...imports.map((i) => i.getText()), `import {BundleScript as __BS} from "${sRouting}";`, ...stubs(used, st), st.getText(), `const __router = new __SR(import.meta.dir)${[...used].filter((n) => declOf.has(n) && declOf.get(n) !== st && ts.isClassDeclaration(declOf.get(n)!)).map((n, i) => `.register("k${i}", ${n})`).join("")};`, `import {ScriptRouter as __SR, runScriptMain as __RM, receiveScriptProcessInvocation as __RP} from "${sRouting}";`, `await __RP(process.env, original => __RM(__router, { invocation: original }));`].join("\n");
    if (!accepts(synthetic)) invalidConsts.push(st);
  }
  if (!needsLift.length && !invalidConsts.length) return {ok: false, reason: "no-class-to-lift"};
  const scriptDir = dirname(join(REPO, p));
  let modulePath: string;
  const ownerInfo = owner;
  if (ownerInfo) modulePath = join(ownerInfo.dir, "🧪️tests", ownerInfo.slug, "🟦️.ts");
  else if (!existsSync(join(scriptDir, "🟦️.ts"))) modulePath = join(dirname(p), "🟦️.ts");
  else return {ok: false, reason: "no-module-slot"};
  if (existsSync(join(REPO, modulePath))) return {ok: false, reason: "module-exists"};
  const moduleDir = dirname(join(REPO, modulePath));
  let usesScriptDir = false;
  const rebase = (spec: string): string => {
    if (!spec.startsWith(".")) return spec;
    const abs = resolve(scriptDir, spec);
    const alias = aliasSpecifier(moduleDir, abs);
    if (alias) return alias;
    const r = relative(moduleDir, abs);
    return r.startsWith(".") ? r : "./" + r;
  };

  const liftedNames = new Set(needsLift.map((c) => c.name!.text));
  const moved = new Set<ts.Statement>();
  for (const st of invalidConsts) moved.add(st);
  const queue: ts.Node[] = [...needsLift, ...invalidConsts];
  const seenNames = new Set<string>();
  while (queue.length) {
    const n = queue.pop()!;
    for (const id of usedIdentifiers(n)) {
      if (seenNames.has(id)) continue;
      seenNames.add(id);
      const st = declOf.get(id);
      if (st && !liftedNames.has(id) && !moved.has(st) && !(ts.isClassDeclaration(st) && st.heritageClauses?.some((h) => h.types.some((t) => ts.isIdentifier(t.expression) && SCRIPT_BASES.has(bindings.get(t.expression.text)?.imported ?? ""))))) { moved.add(st); queue.push(st); }
    }
  }

  const bodyEdit = (node: ts.Node, topLevel = false): string | null => {
    const start = node.getStart();
    const edits: {s: number; e: number; t: string}[] = [];
    let bail: string | null = null;
    const visit = (n: ts.Node): void => {
      if (bail) return;
      if (n.kind === ts.SyntaxKind.ThisKeyword) {
        const par = n.parent;
        if (ts.isPropertyAccessExpression(par) && par.expression === n && ["root", "repoRoot", "invocation"].includes(par.name.text)) { edits.push({s: par.getStart(), e: par.end, t: `context.${par.name.text}`}); return; }
        bail = "this-use"; return;
      }
      if (ts.isMetaProperty(n)) {
        const par = n.parent;
        if (ts.isPropertyAccessExpression(par) && par.name.text === "dir") { edits.push({s: par.getStart(), e: par.end, t: topLevel ? "SCRIPT_DIR" : "context.root"}); usesScriptDir = topLevel || usesScriptDir; return; }
        bail = "import.meta"; return;
      }
      if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0]!)) {
        const lit = n.arguments[0] as ts.StringLiteral;
        edits.push({s: lit.getStart(), e: lit.end, t: JSON.stringify(rebase(lit.text))});
      }
      if (ts.isCallExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === "require") { bail = "require"; return; }
      if (ts.isIdentifier(n) && n.text === "__dirname") { bail = "__dirname"; return; }
      ts.forEachChild(n, visit);
    };
    visit(node);
    if (bail) return null;
    let text = src.slice(start, node.end);
    for (const e of edits.sort((a, b) => b.s - a.s)) text = text.slice(0, e.s - start) + e.t + text.slice(e.e - start);
    return text;
  };

  const lifted: string[] = [], functions: string[] = [];
  for (const c of needsLift) {
    const members = c.members;
    if (members.length !== 1 || !ts.isMethodDeclaration(members[0]!) || (members[0]!.name as ts.Identifier).text !== "run" || !members[0]!.body) return {ok: false, reason: `class-shape:${c.name!.text}`};
    if (c.typeParameters || c.heritageClauses![0]!.types[0]!.typeArguments) return {ok: false, reason: `class-generics:${c.name!.text}`};
    const m = members[0]!;
    const rawBody = bodyEdit(m.body!);
    if (rawBody === null) return {ok: false, reason: `body-bail:${c.name!.text}`};
    const body = rawBody.split("\n").map((line, i) => i === 0 ? line : line.replace(/^ {2}/u, "")).join("\n");
    const params = m.parameters.map((q) => bodyEdit(q) ?? q.getText()).join(", ");
    const ret = m.type ? `: ${m.type.getText()}` : "";
    const isAsync = !!m.modifiers?.some((x) => x.kind === ts.SyntaxKind.AsyncKeyword);
    const fn = `run${c.name!.text}`;
    functions.push(`/** 🏃️ Runs the \`${c.name!.text}\` command under the caller's roots and invocation. */\nexport ${isAsync ? "async " : ""}function ${fn}(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>${params ? ", " + params : ""})${ret} ${body}`);
    lifted.push(c.name!.text);
  }
  const movedStatements = [...moved].sort((a, b) => a.pos - b.pos);
  const movedText: string[] = [];
  for (const st of movedStatements) {
    const t = bodyEdit(st, true);
    if (t === null) return {ok: false, reason: "moved-bail"};
    movedText.push(t);
  }

  const moduleUses = new Set<string>();
  for (const t of [...movedText, ...functions]) { const sf = ts.createSourceFile("m.ts", t, ts.ScriptTarget.Latest, true); for (const id of usedIdentifiers(sf)) moduleUses.add(id); }
  const moduleImports: string[] = [`import type { ScriptInvocation } from "${rebase(sRouting)}";`];
  for (const d of imports) {
    const rows = importBindings(d).filter((b) => moduleUses.has(b.name));
    const spec = (d.moduleSpecifier as ts.StringLiteral).text;
    const attrs = d.attributes ? ` ${d.attributes.getText()}` : "";
    if (!rows.length) continue;
    const def = rows.find((b) => b.imported === "default"), ns = rows.find((b) => b.imported === "*"), named = rows.filter((b) => b.imported !== "default" && b.imported !== "*");
    const parts = [def?.name, ns?.text, named.length ? `{ ${named.map((b) => b.text).join(", ")} }` : null].filter(Boolean);
    moduleImports.push(`import ${parts.join(", ")} from "${rebase(spec)}"${attrs};`);
  }
  let mutableShared = false;
  const exportedMoved = new Set<string>();
  const usedByRest = (() => {
    const rest = new Set<string>();
    for (const st of f.statements) { if (ts.isImportDeclaration(st) || moved.has(st) || (ts.isClassDeclaration(st) && st.name && liftedNames.has(st.name.text))) continue; for (const id of usedIdentifiers(st)) rest.add(id); }
    return rest;
  })();
  const movedFinal = movedText.map((t, i) => {
    const names = declaredNames(movedStatements[i]!);
    const shared = names.filter((n) => usedByRest.has(n));
    shared.forEach((n) => exportedMoved.add(n));
    if (shared.length && ts.isVariableStatement(movedStatements[i]!) && (movedStatements[i] as ts.VariableStatement).declarationList.flags & ts.NodeFlags.Let) mutableShared = true;
    return shared.length ? "export " + t : t;
  });
  const scriptDirConst = usesScriptDir ? `const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, ${JSON.stringify(relative(moduleDir, scriptDir))});\n\n` : "";
  if (usesScriptDir) moduleImports.push(`import { resolve as scriptDirectoryResolve } from "node:path";`);
  if (mutableShared) return {ok: false, reason: "let-shared"};
  const moduleText = `${moduleImports.join("\n")}\n\n${scriptDirConst}${movedFinal.length ? movedFinal.join("\n") + "\n\n" : ""}${functions.length ? functions.join("\n\n") + "\n" : ""}`;

  const scriptEdits: {s: number; e: number; t: string}[] = [];
  const moduleSpec = (() => { const r = relative(scriptDir, join(REPO, modulePath)); return r.startsWith(".") ? r : "./" + r; })();
  for (const c of needsLift) {
    const m = c.members[0] as ts.MethodDeclaration;
    const names = m.parameters.map((q) => ts.isIdentifier(q.name) ? q.name.text : null);
    const header = m.parameters.map((q, i) => ts.isIdentifier(q.name) ? q.getText() : `segments${i || ""}${q.type ? ": " + q.type.getText() : ": string[]"}`);
    const args = m.parameters.map((q, i) => names[i] ?? `segments${i || ""}`);
    const ret = m.type ? `: ${m.type.getText()}` : "";
    const modifiers = c.modifiers?.filter((x) => x.kind !== ts.SyntaxKind.AbstractKeyword).map((x) => x.getText()).join(" ") ?? "";
    const replacement = `${modifiers ? modifiers + " " : ""}class ${c.name!.text} extends ${c.heritageClauses![0]!.types[0]!.getText()} {\n  async run(${header.join(", ")})${ret.includes("Promise") ? ret : ": Promise<void>"} {\n    await run${c.name!.text}({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }${args.length ? ", " + args.join(", ") : ""});\n  }\n}`;
    scriptEdits.push({s: c.getStart(), e: c.end, t: replacement});
  }
  for (const st of movedStatements) scriptEdits.push({s: st.getFullStart(), e: st.end, t: ""});
  let script = src;
  for (const e of scriptEdits.sort((a, b) => b.s - a.s)) script = script.slice(0, e.s) + e.t + script.slice(e.e);
  const newImport = `import { ${[...liftedNames].map((n) => `run${n}`).concat([...exportedMoved]).join(", ")} } from "${moduleSpec}";\n`;
  const lastImport = [...script.matchAll(/^import\s[^\n]*?;[ \t]*$/gmu)].at(-1);
  script = lastImport ? script.slice(0, lastImport.index! + lastImport[0].length) + "\n" + newImport.trimEnd() + script.slice(lastImport.index! + lastImport[0].length) : newImport + script;
  const sf2 = ts.createSourceFile(p, script, ts.ScriptTarget.Latest, true);
  const restUsed = new Set<string>();
  for (const st of sf2.statements) if (!ts.isImportDeclaration(st)) for (const id of usedIdentifiers(st)) restUsed.add(id);
  const importEdits: {s: number; e: number; t: string}[] = [];
  for (const d of sf2.statements.filter(ts.isImportDeclaration)) {
    const rows = importBindings(d);
    if (!rows.length) continue;
    const keep = rows.filter((b) => restUsed.has(b.name));
    if (keep.length === rows.length) continue;
    if (!keep.length) { importEdits.push({s: d.getFullStart(), e: d.end, t: ""}); continue; }
    const spec = (d.moduleSpecifier as ts.StringLiteral).text, attrs = d.attributes ? ` ${d.attributes.getText()}` : "";
    const def = keep.find((b) => b.imported === "default"), ns = keep.find((b) => b.imported === "*"), named = keep.filter((b) => b.imported !== "default" && b.imported !== "*");
    const typeOnly = d.importClause?.isTypeOnly ? "type " : "";
    importEdits.push({s: d.getStart(), e: d.end, t: `import ${typeOnly}${[def?.name, ns?.text, named.length ? `{ ${named.map((b) => b.text).join(", ")} }` : null].filter(Boolean).join(", ")} from "${spec}"${attrs};`});
  }
  for (const e of importEdits.sort((a, b) => b.s - a.s)) script = script.slice(0, e.s) + e.t + script.slice(e.e);
  return {ok: true, script, module: moduleText, modulePath, lifted};
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
    try { plan = planLift(p, src); } catch (e) { count("error"); ledger({tool: "lift", p, result: "error", message: String(e).slice(0, 200)}); continue; }
    if (!plan.ok) { count(plan.reason); ledger({tool: "lift", p, result: plan.reason}); continue; }
    const scriptOk = syntaxOk(p, plan.script), accepted = scriptOk && accepts(plan.script);
    if (dump) { mkdirSync(GEN, {recursive: true}); writeFileSync(`${GEN}/dump-lift-script.ts`, plan.script); writeFileSync(`${GEN}/dump-lift-module.ts`, plan.module); }
    const modOk = syntaxOk(plan.modulePath, plan.module);
    if (!scriptOk || !modOk) { count("syntax-fail"); ledger({tool: "lift", p, result: "syntax-fail", scriptOk, modOk}); continue; }
    if (!write) { count(accepted ? "ok-accepted" : "ok-still-rejected"); continue; }
    if (!accepted && process.argv.includes("--accepted-only")) { count("skip-not-accepted"); continue; }
    if (ageMinutes(p) < 15) { count("skip-recent"); continue; }
    if (read(p) !== src) { count("skip-changed"); continue; }
    mkdirSync(dirname(join(REPO, plan.modulePath)), {recursive: true});
    writeFileSync(join(REPO, plan.modulePath), plan.module);
    writeFileSync(join(REPO, p), plan.script); recordOwn(p);
    count(accepted ? "written-accepted" : "written-still-rejected");
    ledger({tool: "lift", p, result: accepted ? "written-accepted" : "written-still-rejected", module: plan.modulePath, lifted: plan.lifted});
  }
  console.log(tally);
}
