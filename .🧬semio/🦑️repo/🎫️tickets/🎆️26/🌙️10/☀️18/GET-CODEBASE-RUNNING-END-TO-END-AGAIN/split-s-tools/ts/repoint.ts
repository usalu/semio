import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { analyze, relSpec, ROOT, ts } from "./ast.ts";
import { resolveRel } from "./exports.ts";

export type Resolver = (name: string) => string | undefined;
export type Edit = { start: number; end: number; text: string };
export type Report = { file: string; kind: string; detail: string };

export type Options = {
  oldTargets: Set<string>;
  bare?: Set<string>;
  resolver: Resolver;
  dry: boolean;
  skip?: (file: string) => boolean;
};

function quoteOf(text: string, node: import("typescript").Node, sf: import("typescript").SourceFile): string {
  return text[node.getStart(sf)] === "'" ? "'" : '"';
}

function matches(file: string, spec: string, o: Options): boolean {
  if (o.bare?.has(spec)) return true;
  const r = resolveRel(file, spec);
  return r !== null && o.oldTargets.has(r);
}

export function repointFile(file: string, o: Options, report: Report[]): Edit[] | null {
  const text = readFileSync(join(ROOT, file), "utf8");
  const a = analyze(file, text, false);
  const sf = a.sf;
  const edits: Edit[] = [];
  const target = (name: string) => {
    const t = o.resolver(name);
    if (!t) report.push({ file, kind: "unresolved-name", detail: name });
    return t;
  };
  const newSpec = (t: string) => relSpec(file, t);
  for (const st of sf.statements) {
    if (ts.isImportDeclaration(st) && ts.isStringLiteral(st.moduleSpecifier) && matches(file, st.moduleSpecifier.text, o)) {
      const clause = st.importClause;
      if (!clause || clause.name || !clause.namedBindings || !ts.isNamedImports(clause.namedBindings)) {
        report.push({ file, kind: "manual-import-shape", detail: st.getText(sf).slice(0, 120) });
        continue;
      }
      const groups = new Map<string, string[]>();
      let bad = false;
      for (const el of clause.namedBindings.elements) {
        const t = target((el.propertyName ?? el.name).text);
        if (!t) {
          bad = true;
          continue;
        }
        (groups.get(t) ?? groups.set(t, []).get(t)!).push(el.getText(sf));
      }
      if (bad) continue;
      const q = quoteOf(text, st.moduleSpecifier, sf);
      if (groups.size === 1) {
        const [t] = [...groups.keys()];
        edits.push({ start: st.moduleSpecifier.getStart(sf), end: st.moduleSpecifier.getEnd(), text: `${q}${newSpec(t)}${q}` });
      } else {
        const lineStart = text.lastIndexOf("\n", st.getStart(sf)) + 1;
        const indent = text.slice(lineStart, st.getStart(sf));
        const lines = [...groups].map(([t, els]) => `import ${clause.isTypeOnly ? "type " : ""}{ ${els.join(", ")} } from ${q}${newSpec(t)}${q};`);
        edits.push({ start: st.getStart(sf), end: st.getEnd(), text: lines.join("\n" + indent) });
      }
    } else if (ts.isExportDeclaration(st) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && matches(file, st.moduleSpecifier.text, o)) {
      if (!st.exportClause || !ts.isNamedExports(st.exportClause)) {
        report.push({ file, kind: "manual-export-shape", detail: st.getText(sf).slice(0, 120) });
        continue;
      }
      const groups = new Map<string, string[]>();
      let bad = false;
      for (const el of st.exportClause.elements) {
        const t = target((el.propertyName ?? el.name).text);
        if (!t) {
          bad = true;
          continue;
        }
        (groups.get(t) ?? groups.set(t, []).get(t)!).push(el.getText(sf));
      }
      if (bad) continue;
      const q = quoteOf(text, st.moduleSpecifier, sf);
      const lines = [...groups].map(([t, els]) => `export ${st.isTypeOnly ? "type " : ""}{ ${els.join(", ")} } from ${q}${newSpec(t)}${q};`);
      const lineStart = text.lastIndexOf("\n", st.getStart(sf)) + 1;
      edits.push({ start: st.getStart(sf), end: st.getEnd(), text: lines.join("\n" + text.slice(lineStart, st.getStart(sf))) });
    }
  }
  const visit = (n: import("typescript").Node) => {
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments.length >= 1 && ts.isStringLiteralLike(n.arguments[0]) && matches(file, (n.arguments[0] as import("typescript").StringLiteral).text, o)) {
      const lit = n.arguments[0] as import("typescript").StringLiteral;
      const q = quoteOf(text, lit, sf);
      const aw = n.parent;
      const decl = aw && ts.isAwaitExpression(aw) ? aw.parent : undefined;
      if (decl && ts.isVariableDeclaration(decl) && ts.isObjectBindingPattern(decl.name)) {
        const groups = new Map<string, string[]>();
        let bad = false;
        for (const el of decl.name.elements) {
          const t = target((el.propertyName && ts.isIdentifier(el.propertyName) ? el.propertyName.text : (el.name as import("typescript").Identifier).text));
          if (!t) {
            bad = true;
            continue;
          }
          (groups.get(t) ?? groups.set(t, []).get(t)!).push(el.getText(sf));
        }
        if (bad) return;
        if (groups.size === 1) {
          const [t] = [...groups.keys()];
          edits.push({ start: lit.getStart(sf), end: lit.getEnd(), text: `${q}${newSpec(t)}${q}` });
        } else {
          const parts = [...groups].map(([t, els]) => `{ ${els.join(", ")} } = await import(${q}${newSpec(t)}${q})`);
          edits.push({ start: decl.getStart(sf), end: decl.getEnd(), text: parts.join(", ") });
        }
      } else if (aw && ts.isParenthesizedExpression(aw.parent ?? aw)) {
        report.push({ file, kind: "manual-dynamic", detail: n.parent.parent.getText(sf).slice(0, 120) });
      } else {
        report.push({ file, kind: "manual-dynamic", detail: (decl ?? n).getText(sf).slice(0, 120) });
      }
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && matches(file, n.argument.literal.text, o)) {
      const qn = n.qualifier;
      const name = qn && ts.isIdentifier(qn) ? qn.text : undefined;
      const t = name ? target(name) : undefined;
      if (t) {
        const lit = n.argument.literal;
        const q = quoteOf(text, lit, sf);
        edits.push({ start: lit.getStart(sf), end: lit.getEnd(), text: `${q}${newSpec(t)}${q}` });
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return edits.length ? edits : null;
}

export function applyEdits(text: string, edits: Edit[]): string {
  const sorted = [...edits].sort((x, y) => y.start - x.start);
  let out = text;
  for (const e of sorted) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  return out;
}

export function repoint(files: string[], o: Options): { changed: string[]; report: Report[] } {
  const report: Report[] = [];
  const changed: string[] = [];
  for (const file of files) {
    if (o.skip?.(file)) continue;
    let edits: Edit[] | null;
    let before: string;
    try {
      before = readFileSync(join(ROOT, file), "utf8");
      edits = repointFile(file, o, report);
    } catch (e) {
      report.push({ file, kind: "error", detail: String(e) });
      continue;
    }
    if (!edits) continue;
    const after = applyEdits(before, edits);
    if (!o.dry) {
      const now = readFileSync(join(ROOT, file), "utf8");
      if (now !== before) {
        report.push({ file, kind: "raced", detail: "file changed during rewrite; rerun" });
        continue;
      }
      writeFileSync(join(ROOT, file), after);
    }
    changed.push(file);
  }
  return { changed, report };
}
