import ts from "typescript";
import { dirname, relative, resolve } from "node:path";
import { LIB_REL, ROOT, parse, posix, rel, resolveSpec } from "./lib.ts";
import { PKG_NAME } from "./lib.ts";
import { isBootstrapFile, type Dest, type FwName, type FwRules, entryOf, destHas, isConfigFile, isFixture, isRepoProduct, libRelOf, needsProvider } from "./rules.ts";

export type Ctx = {
  known: Set<string>;
  fw: FwRules;
  closure: Set<string>;
  relative: boolean;
  jsonNames: Map<string, string>;
  blocked: Set<string>;
};

export type Report = {
  file: string;
  changed: boolean;
  text: string;
  /** names kept on the repository product, per library target */
  b: Map<string, Set<string>>;
  a: { target: string; name: string; to: string; mode: FwName["mode"] }[];
  dead: { target: string; name: string }[];
  open: { target: string; name: string; reason: string }[];
  failures: string[];
};

const blockKey = (file: string, target: string, name: string): string => `${file}\0${target}\0${name}`;

function specTo(file: string, ctx: Ctx, glueAbs: string, bare: string): string {
  const fileAbs = resolve(ROOT, file);
  const forceRelative = ctx.relative || ctx.closure.has(file) || /\.(mjs|cjs)$/u.test(file) || file.endsWith("📜️script.ts") || isConfigFile(file) || isBootstrapFile(file);
  if (!forceRelative) return bare;
  const r = posix(relative(dirname(fileAbs), glueAbs));
  return r.startsWith(".") ? r : "./" + r;
}

export const domainSpec = (file: string, ctx: Ctx, target: string): string => {
  const e = entryOf(libRelOf(target));
  return specTo(file, ctx, e.glueAbs, `${PKG_NAME}/${e.key.slice(2)}`);
};
export const destSpec = (file: string, ctx: Ctx, dest: Dest): string => specTo(file, ctx, resolve(ROOT, dest.glue), `${dest.pkg}/${dest.key.slice(2)}`);

type Edit = { start: number; end: number; text: string };

const importedName = (e: ts.ImportSpecifier): string => (e.propertyName ?? e.name).text;

function renameIdentifiers(sf: ts.SourceFile, from: string, to: string, skip: (n: ts.Node) => boolean): Edit[] {
  const edits: Edit[] = [];
  const visit = (n: ts.Node): void => {
    if (skip(n)) return;
    if (ts.isIdentifier(n) && n.text === from) {
      const p = n.parent;
      const isName = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isBindingElement(p) && p.propertyName === n)
        || (ts.isMethodDeclaration(p) && p.name === n) || (ts.isPropertySignature(p) && p.name === n) || (ts.isPropertyDeclaration(p) && p.name === n) || (ts.isEnumMember(p) && p.name === n) || (ts.isImportSpecifier(p)) || (ts.isExportSpecifier(p) && p.propertyName === n);
      if (!isName) {
        if (ts.isShorthandPropertyAssignment(p) && p.name === n) edits.push({ start: n.getStart(sf), end: n.getEnd(), text: `${from}: ${to}` });
        else edits.push({ start: n.getStart(sf), end: n.getEnd(), text: to });
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return edits;
}

const lineEnd = (text: string, end: number): number => (text[end] === "\n" ? end + 1 : text[end] === "\r" && text[end + 1] === "\n" ? end + 2 : end);

/** Rewrites one file's imports of repository-library files: framework ports, domain entries, dead imports removed. */
export function rewriteFile(file: string, text: string, ctx: Ctx): Report {
  const report: Report = { file, changed: false, text, b: new Map(), a: [], dead: [], open: [], failures: [] };
  const abs = resolve(ROOT, file);
  const sf = parse(file, text);
  const edits: Edit[] = [];
  const statements: { start: number; end: number; lineEnd: number; typeAll: boolean; groups: { spec: string; items: string[] }[] }[] = [];
  const renames = new Map<string, string>();
  const dropped = new Set<string>();
  const cargoLocals = new Set<string>();
  const awaitCalls = new Set<string>();
  const skipRanges: [number, number][] = [];
  const repoImporter = isRepoProduct(file);

  const usedOutside = new Set<string>();
  const collect = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) usedOutside.add(n.text);
    ts.forEachChild(n, collect);
  };
  collect(sf);

  const addB = (target: string, name: string): void => {
    if (!report.b.has(target)) report.b.set(target, new Set());
    report.b.get(target)!.add(name);
  };

  const ruleFor = (target: string, name: string): FwName | undefined => {
    if (repoImporter) return undefined;
    const rule = ctx.fw[libRelOf(target)]?.[name];
    if (!rule) return undefined;
    if (ctx.blocked.has(blockKey(file, target, name))) return undefined;
    if (isConfigFile(file) && needsProvider(rule.dest)) return undefined;
    if (!destHas(rule, name)) return undefined;
    return rule;
  };

  for (const st of sf.statements) {
    if (!ts.isImportDeclaration(st) || !ts.isStringLiteral(st.moduleSpecifier)) continue;
    const spec = st.moduleSpecifier.text;
    const target = resolveSpec(abs, spec, ctx.known);
    if (!target) continue;
    const lib = libRelOf(target);
    const clause = st.importClause;
    if (spec.includes("?")) { report.open.push({ target, name: "*", reason: "query import reads the source file itself" }); continue; }
    skipRanges.push([st.getStart(sf), st.getEnd()]);
    const typeAll = !!clause?.isTypeOnly;

    if (!clause) { addB(target, "*"); edits.push({ start: st.moduleSpecifier.getStart(sf), end: st.moduleSpecifier.getEnd(), text: JSON.stringify(domainSpec(file, ctx, target)) }); continue; }

    if (isFixture(lib)) { report.open.push({ target, name: "*", reason: "test fixture of another package" }); continue; }

    if (target.endsWith(".json")) {
      const local = clause.name?.text;
      if (!local) { report.failures.push(`${spec}: JSON import without default binding`); continue; }
      if (!usedOutside.has(local)) { report.dead.push({ target, name: "default" }); edits.push({ start: st.getStart(sf), end: lineEnd(text, st.getEnd()), text: "" }); continue; }
      const jn = ctx.jsonNames.get(lib);
      if (!jn) { report.failures.push(`${spec}: no exported name for JSON`); continue; }
      addB(target, "default");
      edits.push({ start: st.getStart(sf), end: st.getEnd(), text: `import { ${jn} as ${local} } from ${JSON.stringify(domainSpec(file, ctx, target))};` });
      continue;
    }

    if (clause.namedBindings && ts.isNamespaceImport(clause.namedBindings) && !clause.name) {
      const local = clause.namedBindings.name.text;
      if (!usedOutside.has(local)) { report.dead.push({ target, name: "*" }); edits.push({ start: st.getStart(sf), end: lineEnd(text, st.getEnd()), text: "" }); continue; }
      addB(target, "*");
      edits.push({ start: st.moduleSpecifier.getStart(sf), end: st.moduleSpecifier.getEnd(), text: JSON.stringify(domainSpec(file, ctx, target)) });
      continue;
    }

    if (clause.name) {
      if (!usedOutside.has(clause.name.text) && !clause.namedBindings) { report.dead.push({ target, name: "default" }); edits.push({ start: st.getStart(sf), end: lineEnd(text, st.getEnd()), text: "" }); continue; }
      addB(target, "default");
      edits.push({ start: st.moduleSpecifier.getStart(sf), end: st.moduleSpecifier.getEnd(), text: JSON.stringify(domainSpec(file, ctx, target)) });
      continue;
    }

    if (!clause.namedBindings || !ts.isNamedImports(clause.namedBindings)) continue;
    const groups = new Map<string, { spec: string; items: string[] }>();
    const push = (id: string, spec2: string, item: string): void => { if (!groups.has(id)) groups.set(id, { spec: spec2, items: [] }); groups.get(id)!.items.push(item); };
    for (const el of clause.namedBindings.elements) {
      const imported = importedName(el), local = el.name.text, typeOnly = typeAll || el.isTypeOnly;
      const used = usedOutside.has(local);
      const prefix = typeOnly && !typeAll ? "type " : "";
      const rule = ruleFor(target, imported);
      if (rule && rule.mode === "cargo-script" && rule.as === "") {
        dropped.add(local);
        report.a.push({ target, name: imported, to: "dropped (provider supplies it)", mode: rule.mode });
        continue;
      }
      if (!used) { report.dead.push({ target, name: imported }); continue; }
      if (rule) {
        if (rule.mode === "cargo-script") cargoLocals.add(local);
        const newName = rule.as ?? imported;
        const aliased = el.propertyName !== undefined;
        if (!aliased && newName !== imported) renames.set(local, newName);
        if (rule.mode === "cargo-script" && newName === "admittedCargoTestPolicyV1") awaitCalls.add(newName);
        push(`a\0${rule.dest.glue}`, destSpec(file, ctx, rule.dest), `${prefix}${newName}${aliased ? ` as ${local}` : ""}`);
        report.a.push({ target, name: imported, to: `${rule.dest.pkg}${rule.dest.key.slice(1)}${newName !== imported ? " as " + newName : ""}`, mode: rule.mode });
        continue;
      }
      addB(target, imported);
      push(`b\0${entryOf(lib).key}`, domainSpec(file, ctx, target), `${prefix}${imported}${el.propertyName ? ` as ${local}` : ""}`);
    }
    statements.push({ start: st.getStart(sf), end: st.getEnd(), lineEnd: lineEnd(text, st.getEnd()), typeAll, groups: [...groups.values()] });
  }

  const owner = new Map<string, { items: string[] }>();
  for (const st of statements) for (const g of st.groups) {
    const key = `${st.typeAll}\0${g.spec}`;
    const first = owner.get(key);
    if (!first) { owner.set(key, { items: [...g.items] }); (g as { first?: boolean }).first = true; }
    else { for (const item of g.items) if (!first.items.includes(item)) first.items.push(item); }
  }
  for (const st of statements) {
    const lines = st.groups.filter(g => (g as { first?: boolean }).first).map(g => `import ${st.typeAll ? "type " : ""}{ ${owner.get(`${st.typeAll}\0${g.spec}`)!.items.join(", ")} } from ${JSON.stringify(g.spec)};`);
    edits.push({ start: st.start, end: lines.length === 0 ? st.lineEnd : st.end, text: lines.join("\n") });
  }

  const inImport = (n: ts.Node): boolean => skipRanges.some(([s, e]) => n.getStart(sf) >= s && n.getEnd() <= e);

  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      const target = resolveSpec(abs, n.moduleSpecifier.text, ctx.known);
      if (target && n.moduleSpecifier.text.includes("?")) report.open.push({ target, name: "*", reason: "query import reads the source file itself" });
      else if (target) {
        if (isFixture(libRelOf(target))) report.open.push({ target, name: "*", reason: "test fixture of another package" });
        else {
          const names = n.exportClause && ts.isNamedExports(n.exportClause) ? n.exportClause.elements.map(e => (e.propertyName ?? e.name).text) : ["*"];
          for (const name of names) addB(target, name);
          edits.push({ start: n.moduleSpecifier.getStart(sf), end: n.moduleSpecifier.getEnd(), text: JSON.stringify(domainSpec(file, ctx, target)) });
        }
      }
    } else if (ts.isCallExpression(n) && ts.isIdentifier(n.expression) && cargoLocals.has(n.expression.text) && dropped.size) {
      for (const argument of n.arguments) {
        if (!ts.isObjectLiteralExpression(argument)) continue;
        for (const property of argument.properties) {
          if (!ts.isPropertyAssignment(property) || !ts.isCallExpression(property.initializer) || !ts.isIdentifier(property.initializer.expression) || !dropped.has(property.initializer.expression.text)) continue;
          const after = text.slice(property.getEnd()).match(/^\s*,\s*/u), before = text.slice(0, property.getStart(sf)).match(/,\s*$/u);
          if (after) edits.push({ start: property.getStart(sf), end: property.getEnd() + after[0].length, text: "" });
          else if (before) edits.push({ start: property.getStart(sf) - before[0].length, end: property.getEnd(), text: "" });
          else edits.push({ start: property.getStart(sf), end: property.getEnd(), text: "" });
        }
      }
    }
    if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) {
      const arg = n.arguments[0] as ts.StringLiteralLike;
      const isDynamic = n.expression.kind === ts.SyntaxKind.ImportKeyword;
      const isRequire = ts.isIdentifier(n.expression) && n.expression.text === "require";
      if (isDynamic || isRequire) {
        const target = resolveSpec(abs, arg.text, ctx.known);
        if (target && arg.text.includes("?")) report.open.push({ target, name: "*", reason: "query import reads the source file itself" });
        else if (target) {
          const lib = libRelOf(target);
          if (isFixture(lib)) report.open.push({ target, name: "*", reason: "test fixture of another package" });
          else {
            const jn = ctx.jsonNames.get(lib);
            let ok = true;
            const jsonEdits: Edit[] = [];
            if (jn) {
              const awaited = n.parent && ts.isAwaitExpression(n.parent) ? n.parent : undefined;
              const holder = awaited?.parent;
              if (holder && ts.isParenthesizedExpression(holder) && ts.isPropertyAccessExpression(holder.parent) && holder.parent.expression === holder && holder.parent.name.text === "default") jsonEdits.push({ start: holder.parent.name.getStart(sf), end: holder.parent.name.getEnd(), text: jn });
              else if (holder && ts.isVariableDeclaration(holder) && ts.isObjectBindingPattern(holder.name)) {
                for (const e of holder.name.elements) {
                  if (ts.isIdentifier(e.name) && !e.propertyName && e.name.text === "default") ok = false;
                  else if (e.propertyName && ts.isIdentifier(e.propertyName) && e.propertyName.text === "default") jsonEdits.push({ start: e.propertyName.getStart(sf), end: e.propertyName.getEnd(), text: jn });
                }
              } else if (holder && ts.isVariableDeclaration(holder) && ts.isIdentifier(holder.name)) {
                const local = holder.name.text;
                const walk = (m: ts.Node): void => {
                  if (ts.isIdentifier(m) && m.text === local && m !== holder.name) {
                    const pr = m.parent;
                    if (ts.isPropertyAccessExpression(pr) && pr.expression === m && pr.name.text === "default") jsonEdits.push({ start: pr.name.getStart(sf), end: pr.name.getEnd(), text: jn });
                    else if (!(ts.isPropertyAccessExpression(pr) && pr.name === m) && !(ts.isPropertyAssignment(pr) && pr.name === m)) ok = false;
                  }
                  ts.forEachChild(m, walk);
                };
                let scope: ts.Node = holder;
                while (scope.parent && !ts.isBlock(scope) && !ts.isSourceFile(scope)) scope = scope.parent;
                walk(scope);
              } else if (holder && ts.isPropertyAccessExpression(holder) && holder.name.text === "default") jsonEdits.push({ start: holder.name.getStart(sf), end: holder.name.getEnd(), text: jn });
              else if (n.parent && ts.isPropertyAccessExpression(n.parent) && n.parent.name.text === "then") {
                const fn = ts.isCallExpression(n.parent.parent) ? n.parent.parent.arguments[0] : undefined;
                const p = fn && (ts.isArrowFunction(fn) || ts.isFunctionExpression(fn)) ? fn.parameters[0]?.name : undefined;
                if (p && ts.isObjectBindingPattern(p)) {
                  for (const e of p.elements) if (e.propertyName && ts.isIdentifier(e.propertyName) && e.propertyName.text === "default") jsonEdits.push({ start: e.propertyName.getStart(sf), end: e.propertyName.getEnd(), text: jn });
                  else if (ts.isIdentifier(e.name) && !e.propertyName && e.name.text === "default") ok = false;
                } else ok = false;
              } else ok = false;
              if (n.arguments[1]) jsonEdits.push({ start: n.arguments[0]!.getEnd(), end: n.arguments[1].getEnd(), text: "" });
            }
            if (!ok) report.failures.push(`${arg.text}: dynamic JSON import shape not rewritable`);
            else {
              addB(target, jn ? "default" : "*");
              edits.push({ start: arg.getStart(sf), end: arg.getEnd(), text: JSON.stringify(domainSpec(file, ctx, target)) }, ...jsonEdits);
            }
          }
        }
      }
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) {
      const target = resolveSpec(abs, n.argument.literal.text, ctx.known);
      if (target && !isFixture(libRelOf(target))) {
        addB(target, n.qualifier ? (ts.isIdentifier(n.qualifier) ? n.qualifier.text : (n.qualifier as ts.QualifiedName).left.getText(sf)) : "*");
        edits.push({ start: n.argument.literal.getStart(sf), end: n.argument.literal.getEnd(), text: JSON.stringify(domainSpec(file, ctx, target)) });
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);

  for (const [from, to] of renames) {
    const conflict = usedOutside.has(to);
    if (conflict) { report.failures.push(`rename ${from} -> ${to} collides with an existing name`); continue; }
    edits.push(...renameIdentifiers(sf, from, to, inImport));
  }
  for (const name of awaitCalls) {
    for (const e of edits.filter(x => x.text === name)) e.text = `await ${name}`;
  }

  if (!edits.length) return report;
  const sorted = edits.sort((x, y) => y.start - x.start || y.end - x.end);
  let out = text;
  for (const e of sorted) out = out.slice(0, e.start) + e.text + out.slice(e.end);

  if (dropped.size) {
    const body = out.split("\n").filter(line => !/^\s*import\b/u.test(line)).join("\n");
    for (const name of dropped) if (new RegExp(`(?<![\\w$])${name}(?![\\w$])`, "u").test(body)) report.failures.push(`dropped ${name} still referenced`);
  }
  if (out !== text) { report.changed = true; report.text = out; }
  return report;
}

export const blocked = blockKey;

/** Runs the rewrite, blocking framework names that could not be rewritten safely, until stable. */
export function plan(file: string, text: string, ctx: Ctx): Report {
  let report = rewriteFile(file, text, ctx);
  for (let round = 0; round < 3 && report.failures.some(f => f.startsWith("dropped ") || f.startsWith("rename ")); round++) {
    const fresh = new Set(ctx.blocked);
    for (const a of report.a) if (a.mode === "cargo-script" || report.failures.some(f => f.startsWith("rename ") && f.includes(` ${a.name} `))) fresh.add(blockKey(file, a.target, a.name));
    ctx = { ...ctx, blocked: fresh };
    report = rewriteFile(file, text, ctx);
  }
  return report;
}

export { rel, LIB_REL };
