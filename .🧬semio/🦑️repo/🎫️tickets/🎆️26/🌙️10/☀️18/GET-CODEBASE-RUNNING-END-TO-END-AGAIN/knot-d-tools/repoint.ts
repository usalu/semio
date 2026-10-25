import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, relative } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";

const root = "/Users/ueli/Documents/semio";
const [mode, source, dest, ...names] = process.argv.slice(2) as string[];
const apply = mode === "apply";
let moved = new Set(names);
const rel = (from: string, to: string) => { let r = relative(dirname(join(root, from)), join(root, to)).split("\\").join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const text = readFileSync(join(root, source!), "utf8");
const sf = ts.createSourceFile(source!, text, ts.ScriptTarget.Latest, true);
const declNames = (st: ts.Statement): string[] => {
  if (ts.isVariableStatement(st)) return st.declarationList.declarations.flatMap((d) => ts.isIdentifier(d.name) ? [d.name.text] : []);
  if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) return [st.name.text];
  return [];
};
const isTypeDecl = (st: ts.Statement) => ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st);
const topDecl = new Map<string, ts.Statement>();
for (const st of sf.statements) for (const n of declNames(st)) topDecl.set(n, st);
const importOf = new Map<string, { st: ts.ImportDeclaration; spec: ts.ImportSpecifier | null; kind: "named" | "default" | "ns"; typeOnly: boolean }>();
for (const st of sf.statements) if (ts.isImportDeclaration(st)) {
  const c = st.importClause; if (!c) continue;
  if (c.name) importOf.set(c.name.text, { st, spec: null, kind: "default", typeOnly: c.isTypeOnly });
  if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) importOf.set(c.namedBindings.name.text, { st, spec: null, kind: "ns", typeOnly: c.isTypeOnly }); else for (const e of c.namedBindings.elements) importOf.set(e.name.text, { st, spec: e, kind: "named", typeOnly: c.isTypeOnly || e.isTypeOnly }); }
}
const statements: ts.Statement[] = [];
let moved0 = new Set(names);
moved = moved0;
const allNames = [...moved];
const topDeclNone = 0;
const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter((f) => /\.(?:[cm]?[jt]sx?|d\.mts)$/u.test(f) && !f.includes("/node_modules/") && !f.startsWith(".🧬semio/") && existsSync(join(root, f)));
const edits = new Map<string, { start: number; end: number; text: string }[]>();
const manual: string[] = [];
for (const from of files) {
  if (from === source || from === dest) continue;
  const t = readFileSync(join(root, from), "utf8");
  if (!allNames.some((n) => t.includes(n))) continue;
  const f = ts.createSourceFile(from, t, ts.ScriptTarget.Latest, true);
  const newSpec = rel(from, dest!);
  const visit = (st: ts.Node) => {
    if ((ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && st.moduleSpecifier.text.startsWith(".") && normalize(join(dirname(from), st.moduleSpecifier.text)) === source) {
      const elements = ts.isImportDeclaration(st) ? (st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings) ? st.importClause.namedBindings.elements : null) : (st.exportClause && ts.isNamedExports(st.exportClause) ? st.exportClause.elements : null);
      if (!elements) { if (allNames.some((n) => t.includes(n))) manual.push(from + ": non-named import/export of source"); return; }
      const mv = elements.filter((e) => moved.has((e.propertyName ?? e.name).text)); if (!mv.length) return;
      const keep = elements.filter((e) => !mv.includes(e));
      const quote = t[st.moduleSpecifier.getStart(f)]!;
      const origSpec = t.slice(st.moduleSpecifier.getStart(f), st.moduleSpecifier.getEnd());
      const attr = (st as ts.ImportDeclaration).attributes ? " " + (st as ts.ImportDeclaration).attributes!.getText(f) : "";
      let replacement: string;
      if (ts.isImportDeclaration(st)) {
        const c = st.importClause!; const typeOnly = c.isTypeOnly; const parts: string[] = [];
        if (keep.length || c.name) parts.push(`import ${typeOnly ? "type " : ""}${c.name ? c.name.text + (keep.length ? ", " : "") : ""}${keep.length ? "{ " + keep.map((e) => e.getText(f)).join(", ") + " }" : ""} from ${origSpec}${attr};`);
        parts.push(`import ${typeOnly ? "type " : ""}{ ${mv.map((e) => e.getText(f)).join(", ")} } from ${quote}${newSpec}${quote}${attr};`);
        replacement = parts.join(" ");
      } else {
        const parts: string[] = [];
        if (keep.length) parts.push(`export ${st.isTypeOnly ? "type " : ""}{ ${keep.map((e) => e.getText(f)).join(", ")} } from ${origSpec};`);
        parts.push(`export ${st.isTypeOnly ? "type " : ""}{ ${mv.map((e) => e.getText(f)).join(", ")} } from ${quote}${newSpec}${quote};`);
        replacement = parts.join(" ");
      }
      (edits.get(from) ?? edits.set(from, []).get(from)!).push({ start: st.getStart(f), end: st.getEnd(), text: replacement });
    } else if (ts.isCallExpression(st) && st.arguments[0] && ts.isStringLiteralLike(st.arguments[0]) && st.arguments[0].text.startsWith(".") && normalize(join(dirname(from), st.arguments[0].text)) === source && (st.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(st.expression) && st.expression.text === "require"))) {
      manual.push(from + ": dynamic/require of source (line " + (f.getLineAndCharacterOfPosition(st.getStart(f)).line + 1) + ")");
    } else if (ts.isImportTypeNode(st) && ts.isLiteralTypeNode(st.argument) && ts.isStringLiteral(st.argument.literal) && st.argument.literal.text.startsWith(".") && normalize(join(dirname(from), st.argument.literal.text)) === source) {
      manual.push(from + ": import() type of source (line " + (f.getLineAndCharacterOfPosition(st.getStart(f)).line + 1) + ")");
    }
    ts.forEachChild(st, visit);
  };
  visit(f);
}

if (manual.length) console.log("MANUAL:\n" + manual.join("\n"));
console.log("importers re-pointed:", edits.size);
if (apply) { for (const [from, list] of edits) { let t = readFileSync(join(root, from), "utf8"); for (const e of list.sort((a, b) => b.start - a.start)) t = t.slice(0, e.start) + e.text + t.slice(e.end); writeFileSync(join(root, from), t); } }
