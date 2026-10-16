/** ↪️ Repoints named imports and re-exports of moved symbols to their new module, in every tracked or untracked TS file.
 * Usage: bun 📜️script.ts <spec.json> [--apply]   (run from the repository root)
 * Spec: { moves: [{ names: string[], target: string, scope?: string[], from: { specifiers?: string[], files?: string[] } }], exclude?: string[] }
 *   `scope`: optional repo-relative path prefixes; the move only applies to files below one of them.
 *   `via`: optional bare specifier written instead of the relative path (a package entry that re-exports the target).
 *   `from.specifiers`: bare module specifiers that currently supply the names (e.g. "@semio-tech/framework").
 *   `from.files`: absolute or repo-relative module files that currently declare them (relative imports of these are repointed).
 *   A file whose own path equals `target` or a `from.files` entry is skipped. Everything is written only with --apply.
 */
import ts from "typescript";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";
import { execFileSync } from "node:child_process";

type Move = { names: string[]; target: string; via?: string; scope?: string[]; from: { specifiers?: string[]; files?: string[] } };
type Spec = { moves: Move[]; exclude?: string[] };
const [specPath, ...flags] = process.argv.slice(2) as [string, ...string[]];
const apply = flags.includes("--apply");
const repo = process.cwd();
const spec: Spec = JSON.parse(readFileSync(specPath, "utf8"));
const excluded = (spec.exclude ?? []).map(p => resolve(p));
const kindOf = (p: string) => (p.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const git = (args: string[]) => execFileSync("git", ["-c", "core.fsmonitor=false", "-c", "core.quotepath=false", ...args], { cwd: repo, maxBuffer: 1 << 30 }).toString("utf8").split("\n").filter(Boolean);
const files = [...new Set([...git(["ls-files", "--", "*.ts", "*.tsx"]), ...git(["ls-files", "--others", "--exclude-standard", "--", "*.ts", "*.tsx"])])]
  .filter(p => !p.startsWith(".🧬semio/") && existsSync(resolve(repo, p))).map(p => resolve(repo, p));
const moves = spec.moves.map(m => ({ ...m, scopes: (m.scope ?? []).map(x => resolve(x)), target: resolve(m.target), files: (m.from.files ?? []).map(f => resolve(f)), specifiers: new Set(m.from.specifiers ?? []), names: new Set(m.names) }));
const candidates = (base: string) => [base, `${base}.ts`, `${base}.tsx`, `${base}/🟦️.ts`, `${base}/🟦️.tsx`].map(p => resolve(p));
const resolveRelative = (fromFile: string, specifier: string): string[] => (specifier.startsWith(".") ? candidates(resolve(dirname(fromFile), specifier)) : []);
const specTo = (fromFile: string, target: string): string => {
  let out = relative(dirname(fromFile), target).split(sep).join("/");
  if (!out.startsWith(".")) out = `./${out}`;
  return out;
};
type Edit = { start: number; end: number; text: string };
const specText = (el: ts.ImportSpecifier | ts.ExportSpecifier, typeOnly: boolean) => `${el.isTypeOnly && !typeOnly ? "type " : ""}${el.propertyName ? `${el.propertyName.text} as ` : ""}${el.name.text}`;
let touched = 0, moved = 0;
const report: string[] = [];
for (const file of files) {
  if (excluded.some(e => file === e || file.startsWith(`${e}${sep}`))) continue;
  if (moves.some(m => file === m.target)) continue;
  const text = readFileSync(file, "utf8");
  if (!moves.some(m => [...m.names].some(n => text.includes(n)))) continue;
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, kindOf(file));
  const edits: Edit[] = [];
  const additions = new Map<string, { typeOnlyAll: boolean; parts: string[] }[]>();
  const existingTargetImport = new Map<string, ts.ImportDeclaration>();
  for (const st of sf.statements) {
    if (ts.isImportDeclaration(st) && ts.isStringLiteral(st.moduleSpecifier) && st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings) && !st.importClause.name) {
      const rs = resolveRelative(file, st.moduleSpecifier.text);
      for (const m of moves) if ((rs.includes(m.target) || (m.via !== undefined && st.moduleSpecifier.text === m.via)) && !st.importClause.isTypeOnly) existingTargetImport.set(m.target, st);
    }
  }
  const pendingNew = new Map<string, { kind: "import" | "export"; typeOnly: boolean; parts: string[]; anchor: ts.Statement }[]>();
  for (const st of sf.statements) {
    const isImport = ts.isImportDeclaration(st);
    const isExport = ts.isExportDeclaration(st) && !!st.moduleSpecifier;
    if (!isImport && !isExport) continue;
    const modSpec = (st as ts.ImportDeclaration | ts.ExportDeclaration).moduleSpecifier;
    if (!modSpec || !ts.isStringLiteral(modSpec)) continue;
    const bindings = isImport ? (st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings) ? st.importClause.namedBindings : undefined) : ((st as ts.ExportDeclaration).exportClause && ts.isNamedExports((st as ts.ExportDeclaration).exportClause!) ? ((st as ts.ExportDeclaration).exportClause as ts.NamedExports) : undefined);
    if (!bindings) continue;
    const rs = resolveRelative(file, modSpec.text);
    const stTypeOnly = isImport ? !!st.importClause?.isTypeOnly : !!(st as ts.ExportDeclaration).isTypeOnly;
    const keep: string[] = [];
    const goByTarget = new Map<string, string[]>();
    for (const el of bindings.elements as ts.NodeArray<ts.ImportSpecifier | ts.ExportSpecifier>) {
      const imported = (el.propertyName ?? el.name).text;
      const owner = moves.find(m => {
        const inScope = !m.scopes.length || m.scopes.some(x => file === x || file.startsWith(`${x}${sep}`));
        return inScope && m.names.has(imported) && (m.specifiers.has(modSpec.text) || m.files.some(f => rs.includes(f)));
      });
      if (!owner) keep.push(specText(el, stTypeOnly));
      else goByTarget.set(owner.target, [...(goByTarget.get(owner.target) ?? []), specText(el, stTypeOnly)]);
    }
    if (!goByTarget.size) continue;
    for (const [target, go] of goByTarget) {
      moved += go.length;
      const list = pendingNew.get(target) ?? [];
      list.push({ kind: isImport ? "import" : "export", typeOnly: stTypeOnly, parts: go, anchor: st });
      pendingNew.set(target, list);
      report.push(`${relative(repo, file)}: ${go.join(", ")} <- ${modSpec.text}`);
    }
    const lineEnd = text[st.getEnd()] === "\n" ? st.getEnd() + 1 : st.getEnd();
    if (keep.length) {
      const clauseHead = isImport ? `import ${stTypeOnly ? "type " : ""}` : `export ${stTypeOnly ? "type " : ""}`;
      edits.push({ start: st.getStart(), end: st.getEnd(), text: `${clauseHead}{ ${keep.join(", ")} } from ${modSpec.getText()};` });
    } else edits.push({ start: st.getStart(), end: lineEnd, text: "" });
  }
  if (!pendingNew.size) continue;
  for (const [target, list] of pendingNew) {
    const specifier = moves.find(m => m.target === target && m.via)?.via ?? specTo(file, target);
    for (const kind of ["import", "export"] as const) {
      const rows = list.filter(r => r.kind === kind);
      if (!rows.length) continue;
      const allTypeOnly = rows.every(r => r.typeOnly);
      const parts = [...new Set(rows.flatMap(r => r.parts.map(p => (allTypeOnly || !r.typeOnly ? p : p.startsWith("type ") ? p : `type ${p}`))))];
      const anchor = rows[0]!.anchor;
      const existing = kind === "import" ? existingTargetImport.get(target) : undefined;
      if (existing && !allTypeOnly) {
        const nb = existing.importClause!.namedBindings as ts.NamedImports;
        const have = nb.elements.map(e => e.getText());
        const merged = [...have, ...parts.filter(p => !have.includes(p))];
        edits.push({ start: existing.getStart(), end: existing.getEnd(), text: `import { ${merged.join(", ")} } from ${existing.moduleSpecifier.getText()};` });
      } else {
        edits.push({ start: anchor.getStart(), end: anchor.getStart(), text: `${kind} ${allTypeOnly ? "type " : ""}{ ${parts.join(", ")} } from "${specifier}";\n` });
      }
    }
  }
  const sorted = edits.sort((a, b) => b.start - a.start || b.end - a.end);
  for (let i = 1; i < sorted.length; i += 1) if (sorted[i]!.end > sorted[i - 1]!.start && !(sorted[i]!.start === sorted[i]!.end)) throw new Error(`overlapping edits in ${file}`);
  let out = text;
  for (const e of sorted) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  touched += 1;
  if (apply) writeFileSync(file, out);
}
console.log(report.join("\n"));
console.log(`${apply ? "applied" : "dry"}: ${touched} files, ${moved} names`);
