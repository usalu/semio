/** 🚛 Relocates one TS module file and rewrites every relative specifier that points at it, plus the relative specifiers inside it.
 * Usage: bun 📜️script.ts <oldFile> <newFile> <apply|dry>   (run from the repository root)
 */
import ts from "typescript";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmdirSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";
import { execFileSync } from "node:child_process";

const [oldArg, newArg, mode] = process.argv.slice(2) as [string, string, string];
const repo = process.cwd();
const oldPath = resolve(oldArg);
const newPath = resolve(newArg);
const kind = (p: string) => (p.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const candidatesFor = (fromFile: string, specifier: string): string[] => {
  const base = resolve(dirname(fromFile), specifier);
  return [base, `${base}.ts`, `${base}.tsx`, `${base}/🟦️.ts`, `${base}/🟦️.tsx`];
};
const specTo = (fromFile: string, target: string): string => {
  let out = relative(dirname(fromFile), target).split(sep).join("/");
  if (!out.startsWith(".")) out = `./${out}`;
  return out;
};

type Edit = { start: number; end: number; text: string };
const literalsOf = (sf: ts.SourceFile): ts.StringLiteral[] => {
  const out: ts.StringLiteral[] = [];
  const visit = (n: ts.Node) => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) out.push(n.moduleSpecifier);
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) out.push(n.argument.literal);
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0])) out.push(n.arguments[0]);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return out;
};
const apply = (text: string, edits: Edit[]) => {
  let out = text;
  for (const e of [...edits].sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  return out;
};

const ownText = readFileSync(oldPath, "utf8");
const ownSf = ts.createSourceFile(oldPath, ownText, ts.ScriptTarget.ESNext, true, kind(oldPath));
const ownEdits: Edit[] = [];
for (const lit of literalsOf(ownSf)) {
  if (!lit.text.startsWith(".")) continue;
  const targetAbs = resolve(dirname(oldPath), lit.text);
  const next = (() => {
    let out = relative(dirname(newPath), targetAbs).split(sep).join("/");
    if (!out.startsWith(".")) out = `./${out}`;
    return out;
  })();
  if (next !== lit.text) ownEdits.push({ start: lit.getStart(ownSf) + 1, end: lit.getEnd() - 1, text: next });
}
const newOwnText = apply(ownText, ownEdits);

const listed = execFileSync("git", ["ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*.ts", "*.tsx", "*.mts"], { cwd: repo, maxBuffer: 1 << 28 }).toString("utf8").split("\0").filter(Boolean);
const results: { path: string; text: string }[] = [];
for (const f of listed) {
  if (f.startsWith(".🧬semio/") || f.endsWith(".d.ts")) continue;
  const abs = resolve(repo, f);
  if (abs === oldPath || abs === newPath || !existsSync(abs)) continue;
  const text = readFileSync(abs, "utf8");
  let touches = false;
  for (const m of text.matchAll(/["'](\.{1,2}\/[^"'\n]*)["']/g)) if (candidatesFor(abs, m[1]!).includes(oldPath)) { touches = true; break; }
  if (!touches) continue;
  const sf = ts.createSourceFile(abs, text, ts.ScriptTarget.ESNext, true, kind(abs));
  const edits: Edit[] = [];
  for (const lit of literalsOf(sf)) {
    if (!lit.text.startsWith(".") || !candidatesFor(abs, lit.text).includes(oldPath)) continue;
    edits.push({ start: lit.getStart(sf) + 1, end: lit.getEnd() - 1, text: specTo(abs, newPath) });
  }
  if (edits.length) results.push({ path: abs, text: apply(text, edits) });
}
console.log(`${relative(repo, oldPath)} -> ${relative(repo, newPath)}: ${ownEdits.length} own specifiers, ${results.length} importers`);
for (const r of results) console.log(`  ${relative(repo, r.path)}`);
if (mode === "apply") {
  mkdirSync(dirname(newPath), { recursive: true });
  writeFileSync(newPath, newOwnText);
  unlinkSync(oldPath);
  let dir = dirname(oldPath);
  while (existsSync(dir) && readdirSync(dir).length === 0 && dir !== dirname(newPath)) {
    rmdirSync(dir);
    dir = dirname(dir);
    if (!dir.includes("🖱️ui")) break;
  }
  for (const r of results) writeFileSync(r.path, r.text);
  console.log("APPLIED");
}
