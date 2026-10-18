import { mkdirSync, writeFileSync, readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { analyze, relSpec, ROOT, ts } from "./ast.ts";
import { resolveRel } from "./exports.ts";
import { seg } from "./split.ts";

export type Group = { dir: string; className: string; doc: string; members: string[]; abstractMembers?: string[] };
export type ClassPlan = { source: string; className: string; groups: Group[]; baseClass: string };

type Member = { name: string; start: number; end: number; node: import("typescript").ClassElement; text: string };

export function planClass(cp: ClassPlan, write: boolean): { files: { file: string; text: string }[]; sourceText: string } {
  const a = analyze(cp.source);
  const cls = a.sf.statements.find((s) => ts.isClassDeclaration(s) && s.name?.text === cp.className) as import("typescript").ClassDeclaration;
  const members: Member[] = [];
  let prevEnd = cls.members.pos;
  for (const m of cls.members) {
    const full = m.getFullStart();
    const ranges = ts.getLeadingCommentRanges(a.text, Math.max(full, prevEnd)) ?? [];
    let start = ranges.length ? ranges[0].pos : m.getStart(a.sf);
    while (start > 0 && (a.text[start - 1] === " " || a.text[start - 1] === "\t")) start--;
    const name = m.name && (ts.isIdentifier(m.name) || ts.isStringLiteral(m.name)) ? m.name.text : "<anon>";
    members.push({ name, start, end: m.getEnd(), node: m, text: a.text.slice(start, m.getEnd()) });
    prevEnd = m.getEnd();
  }
  const importBy = new Map<string, { spec: string; imported: string; typeOnly: boolean; kind: string }>();
  for (const imp of a.imports) for (const b of imp.bindings) importBy.set(b.local, { spec: imp.spec, imported: b.imported, typeOnly: b.typeOnly, kind: b.kind });
  const exact = analyze(cp.source, a.text, true);
  const depsOf = (m: Member): Set<string> => {
    const decl = exact.decls.find((d) => d.names.includes(cp.className))!;
    void decl;
    const out = new Set<string>();
    const checkerFile = exactFor(cp.source, a.text);
    collect(checkerFile.checker, checkerFile.sf, m.node, out);
    return out;
  };
  const groupOf = new Map<string, Group>();
  for (const g of cp.groups) for (const n of g.members) {
    if (groupOf.has(n)) throw new Error(`member ${n} in two groups`);
    groupOf.set(n, g);
  }
  for (const n of groupOf.keys()) if (!members.some((m) => m.name === n)) throw new Error(`unknown member ${n}`);
  const files: { file: string; text: string }[] = [];
  const baseDir = dirname(cp.source);
  const fileOf = (g: Group) => join(baseDir, ...g.dir.split("/").map(seg), "🟦️.ts");
  const makeImports = (file: string, deps: Set<string>, extra: { spec: string; id: string; typeOnly?: boolean }[]): string => {
    const grouped = new Map<string, Map<string, boolean>>();
    const add = (spec: string, id: string, typeOnly: boolean) => {
      const e = grouped.get(spec) ?? grouped.set(spec, new Map()).get(spec)!;
      e.set(id, (e.get(id) ?? true) && typeOnly);
    };
    for (const name of deps) {
      const imp = importBy.get(name);
      if (!imp) continue;
      const r = imp.spec.startsWith(".") ? resolveRel(cp.source, imp.spec) : null;
      const spec = r ? relSpec(file, r) : imp.spec;
      add(spec, imp.imported === name ? name : `${imp.imported} as ${name}`, imp.typeOnly);
    }
    for (const e of extra) add(e.spec, e.id, e.typeOnly ?? false);
    const lines: string[] = [];
    const sorted = [...grouped].sort((x, y) => (x[0].startsWith(".") ? 1 : 0) - (y[0].startsWith(".") ? 1 : 0));
    for (const [spec, ids] of sorted) {
      const all = [...ids.entries()];
      const allType = all.every(([, t]) => t);
      const items = all.map(([k, t]) => (!allType && t ? `type ${k}` : k)).sort((x, y) => x.replace(/^type /u, "").localeCompare(y.replace(/^type /u, "")));
      lines.push(`import ${allType ? "type " : ""}{ ${items.join(", ")} } from "${spec}";`);
    }
    return lines.join("\n");
  };
  let previous: { className: string; file: string } | null = null;
  const abstractAll = cp.groups.flatMap((g) => g.abstractMembers ?? []);
  void abstractAll;
  for (const g of cp.groups) {
    const file = fileOf(g);
    const ms = members.filter((m) => groupOf.get(m.name) === g);
    const deps = new Set<string>();
    for (const m of ms) for (const d of depsOf(m)) deps.add(d);
    if (!previous) deps.add(cp.baseClass);
    const base = previous ? previous.className : cp.baseClass;
    const extra: { spec: string; id: string; typeOnly?: boolean }[] = [];
    if (previous) extra.push({ spec: relSpec(file, previous.file), id: previous.className });
    const imports = makeImports(file, deps, extra);
    const respec = (t: string) => t.replace(/\bimport\(\s*(["'])(\.\.?\/[^"']+)\1\s*\)/gu, (_m, q: string, spec: string) => {
      const r = resolveRel(cp.source, spec);
      return `import(${q}${r ? relSpec(file, r) : spec}${q})`;
    });
    const body = ms.map((m) => respec(m.text).replace(/^(\s*(?:\/\*\*[\s\S]*?\*\/\s*)?)private (async )?/mu, (_x, pre: string, asy: string | undefined) => `${pre}protected ${asy ?? ""}`)).join("\n\n");
    const header = ms.length === 0 ? "" : "";
    void header;
    const text = `${imports}\n\n/** ${g.doc} */\nexport abstract class ${g.className} extends ${base} {\n${body}\n}\n`;
    files.push({ file, text });
    previous = { className: g.className, file };
  }
  const remaining = members.filter((m) => !groupOf.has(m.name));
  const remDeps = new Set<string>();
  for (const m of remaining) for (const d of depsOf(m)) remDeps.add(d);
  const sourceText = rebuildSource(cp, a, cls, remaining, remDeps, previous!, importBy);
  if (write) {
    for (const f of files) {
      if (existsSync(join(ROOT, f.file))) throw new Error(`refusing to overwrite ${f.file}`);
      mkdirSync(dirname(join(ROOT, f.file)), { recursive: true });
      writeFileSync(join(ROOT, f.file), f.text);
    }
    writeFileSync(join(ROOT, cp.source), sourceText);
  }
  return { files, sourceText };
}

function rebuildSource(cp: ClassPlan, a: ReturnType<typeof analyze>, cls: import("typescript").ClassDeclaration, remaining: Member[], remDeps: Set<string>, tail: { className: string; file: string }, importBy: Map<string, { spec: string; imported: string; typeOnly: boolean; kind: string }>): string {
  const firstImport = a.imports[0];
  const lastImport = a.imports[a.imports.length - 1];
  const headerText = a.text.slice(0, firstImport.start);
  const between = a.text.slice(lastImport.end, cls.getStart(a.sf));
  const keep: string[] = [];
  for (const imp of a.imports) {
    const used = imp.bindings.filter((b) => remDeps.has(b.local));
    if (!used.length) continue;
    const q = imp.text.includes("'") && !imp.text.includes('"') ? "'" : '"';
    const items = used.map((b) => (b.typeOnly && !imp.declTypeOnly ? "type " : "") + (b.imported === b.local ? b.local : `${b.imported} as ${b.local}`));
    keep.push(`import ${imp.declTypeOnly ? "type " : ""}{ ${items.join(", ")} } from ${q}${imp.spec}${q};`);
  }
  keep.push(`import { ${tail.className} as VerifyChecks } from "${relSpec(cp.source, tail.file)}";`.replace(" as VerifyChecks", ""));
  const classStart = (() => {
    const ranges = ts.getLeadingCommentRanges(a.text, cls.getFullStart()) ?? [];
    return ranges.length ? ranges[0].pos : cls.getStart(a.sf);
  })();
  void classStart;
  const members = remaining.map((m) => m.text).join("\n\n");
  const headLine = a.text.slice(cls.getStart(a.sf), cls.members.pos);
  const newHead = headLine.replace(new RegExp(`extends ${cp.baseClass}\\b`, "u"), `extends ${tail.className}`);
  const tailText = a.text.slice(cls.getEnd());
  return `${headerText}${keep.join("\n")}${between}${newHead}\n${members}\n}${tailText}`;
}

function exactFor(path: string, text: string) {
  const options: import("typescript").CompilerOptions = { noResolve: true, noLib: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, skipLibCheck: true, types: [] };
  const host = ts.createCompilerHost(options, true);
  const original = host.getSourceFile.bind(host);
  host.getSourceFile = (fileName, languageVersion, ...rest) => (fileName === path ? ts.createSourceFile(path, text, languageVersion, true, ts.ScriptKind.TS) : original(fileName, languageVersion, ...rest));
  host.fileExists = (f) => f === path;
  host.readFile = (f) => (f === path ? text : undefined);
  const program = ts.createProgram({ rootNames: [path], options, host });
  return { checker: program.getTypeChecker(), sf: program.getSourceFile(path)! };
}

function collect(checker: import("typescript").TypeChecker, sf: import("typescript").SourceFile, node: import("typescript").Node, into: Set<string>): void {
  const memberNode = sf.getChildAt(0) && findSame(sf, node);
  const visit = (n: import("typescript").Node) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = (p && ts.isPropertyAccessExpression(p) && p.name === n) || (p && ts.isQualifiedName(p) && p.right === n) || (p && (ts.isPropertyAssignment(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p) || ts.isEnumMember(p)) && p.name === n) || (p && (ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isGetAccessor(p) || ts.isSetAccessor(p)) && p.name === n) || (p && ts.isBindingElement(p) && p.propertyName === n) || (p && ts.isLabeledStatement(p)) || (p && ts.isBreakOrContinueStatement(p));
      if (!skip) {
        const sym = p && ts.isShorthandPropertyAssignment(p) && p.name === n ? checker.getShorthandAssignmentValueSymbol(p) : checker.getSymbolAtLocation(n);
        if (sym) {
          const decls = sym.declarations ?? [];
          if (decls.some((d) => d.getSourceFile() === sf && (ts.isImportSpecifier(d) || ts.isImportClause(d) || ts.isNamespaceImport(d)))) into.add(n.text);
        }
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(memberNode ?? node);
}

function findSame(sf: import("typescript").SourceFile, node: import("typescript").Node): import("typescript").Node | undefined {
  let found: import("typescript").Node | undefined;
  const visit = (n: import("typescript").Node) => {
    if (found) return;
    if (n.pos === node.pos && n.end === node.end && n.kind === node.kind) {
      found = n;
      return;
    }
    if (n.pos <= node.pos && n.end >= node.end) ts.forEachChild(n, visit);
  };
  visit(sf);
  return found;
}
