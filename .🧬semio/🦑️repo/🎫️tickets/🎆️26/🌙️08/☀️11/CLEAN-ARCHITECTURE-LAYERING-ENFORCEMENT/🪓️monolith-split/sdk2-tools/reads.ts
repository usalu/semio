/** 📖️ Rewrites literal framework-path reads in plugin test files to the SDK source-text API (`frameworkSourcePath` / `frameworkSourceUrl`). Dry-run by default.
 * Usage: bun reads.ts <files.txt> [--write] [--snapshot <out.json>] [--report <out.json>]
 * Re-reads every file right before editing. Anything it cannot prove (non-root bases, dynamic import of a framework module, split literals) is reported as MANUAL and left untouched. */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const MARK = "🧰️framework/";
const SPEC = "@semio-tech/framework/test/source-text";
const args = process.argv.slice(2);
const write = args.includes("--write");
const opt = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
const files = readFileSync(args[0], "utf8").split("\n").map(s => s.trim()).filter(Boolean);
const snapshot: Record<string, string> = {};
const report: { file: string; line: number; kind: string; tail: string; note?: string }[] = [];
let edits = 0, manual = 0;

const ROOTISH = new Set(["resolve", "join", "path.resolve", "path.join"]);
const callName = (c: ts.CallExpression) => c.expression.getText();
const isRootBase = (a: ts.Expression): boolean => {
  if (ts.isIdentifier(a)) return true;
  if (ts.isPropertyAccessExpression(a) || ts.isElementAccessExpression(a)) return true;
  if (ts.isCallExpression(a)) return a.expression.getText() === "process.cwd" || a.expression.getText() === "getWorkspaceRoot" || a.expression.getText() === "fileURLToPath";
  if (ts.isStringLiteralLike(a)) return /^(\.\.?\/?)*$/.test(a.text);
  if (ts.isSpreadElement(a)) return /fill\(\s*["']\.\.["']\s*\)/.test(a.getText());
  if (ts.isNewExpression(a) || ts.isTemplateExpression(a)) return false;
  return false;
};

for (const f of files) {
  const text = readFileSync(ROOT + f, "utf8");
  if (!text.includes(MARK)) continue;
  const kind = f.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const sf = ts.createSourceFile(f, text, ts.ScriptTarget.ESNext, true, kind);
  const patches: { start: number; end: number; text: string; used: "path" | "url" }[] = [];
  const claimed: [number, number][] = [];
  const lineOf = (n: ts.Node) => sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1;
  const tailOf = (n: ts.Node): string | null => {
    if (ts.isStringLiteralLike(n)) { const i = n.text.indexOf(MARK); return i < 0 ? null : n.text.slice(i + MARK.length); }
    if (ts.isTemplateExpression(n)) {
      const last = n.templateSpans[n.templateSpans.length - 1]!.literal.text;
      const i = last.indexOf(MARK);
      if (i < 0) { return null; }
      return last.slice(i + MARK.length);
    }
    return null;
  };
  const insideImport = (n: ts.Node): boolean => { for (let p: ts.Node | undefined = n; p; p = p.parent) if (ts.isImportDeclaration(p) || ts.isExportDeclaration(p) || ts.isImportTypeNode(p)) return true; return false; };
  const isDynamicImportArg = (n: ts.Node): boolean => { for (let p: ts.Node | undefined = n.parent; p && !ts.isBlock(p) && !ts.isSourceFile(p); p = p.parent) { if (ts.isCallExpression(p) && p.expression.kind === ts.SyntaxKind.ImportKeyword) return true; if (ts.isStatement(p)) break; } return false; };
  const visit = (n: ts.Node) => {
    if ((ts.isStringLiteralLike(n) || ts.isTemplateExpression(n)) && !insideImport(n)) {
      const tail = tailOf(n);
      if (tail !== null && (!ts.isTemplateExpression(n) || !n.getText().includes("${") || true)) {
        const line = lineOf(n);
        if (tail.includes("${") || tail.length === 0) { report.push({ file: f, line, kind: "MANUAL", tail, note: "tail not static" }); manual++; }
        else if (isDynamicImportArg(n) && /\.(ts|tsx|js|mjs)$/.test(tail)) { report.push({ file: f, line, kind: "MANUAL", tail, note: "dynamic import of a framework module" }); manual++; }
        else {
          let target: ts.Node = n, used: "path" | "url" = "path", ok = true, note = "";
          const p = n.parent;
          if (p && ts.isNewExpression(p) && p.expression.getText() === "URL" && p.arguments && p.arguments[0] === n) { target = p; used = "url"; }
          else if (p && ts.isCallExpression(p) && ROOTISH.has(callName(p)) && p.arguments.includes(n as ts.Expression)) {
            const others = p.arguments.filter(a => a !== n);
            if (others.every(isRootBase)) { target = p; used = "path"; } else { ok = false; note = "non-root base: " + others.map(o => o.getText()).join(", ").slice(0, 80); }
          } else if (p && ts.isCallExpression(p) && callName(p) === "dirname") { ok = false; note = "dirname of framework path"; }
          if (ok) {
            if (isDynamicImportArg(target) && /\.(ts|tsx|js|mjs)$/.test(tail)) { report.push({ file: f, line, kind: "MANUAL", tail, note: "dynamic import of a framework module" }); manual++; }
            else {
              const s = target.getStart(sf), e = target.getEnd();
              if (!claimed.some(([a, b]) => s < b && e > a)) {
                claimed.push([s, e]);
                patches.push({ start: s, end: e, text: `${used === "url" ? "frameworkSourceUrl" : "frameworkSourcePath"}(${JSON.stringify(tail)})`, used });
                report.push({ file: f, line, kind: used.toUpperCase(), tail });
              }
            }
          } else { report.push({ file: f, line, kind: "MANUAL", tail, note }); manual++; }
        }
      }
      if (ts.isTemplateExpression(n)) { /* nested spans cannot hold marker usefully */ }
      return;
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  if (!patches.length) continue;
  patches.sort((a, b) => b.start - a.start);
  let next = text;
  for (const p of patches) next = next.slice(0, p.start) + p.text + next.slice(p.end);
  const names = [...new Set([...(patches.some(p => p.used === "path") ? ["frameworkSourcePath"] : []), ...(patches.some(p => p.used === "url") ? ["frameworkSourceUrl"] : [])])].sort();
  const sf2 = ts.createSourceFile(f, next, ts.ScriptTarget.ESNext, true, kind);
  const existing = sf2.statements.find((s): s is ts.ImportDeclaration => ts.isImportDeclaration(s) && ts.isStringLiteral(s.moduleSpecifier) && s.moduleSpecifier.text === SPEC);
  if (existing) { report.push({ file: f, line: 0, kind: "MANUAL", tail: "", note: "already imports the source-text API" }); manual++; continue; }
  const imports = sf2.statements.filter(ts.isImportDeclaration);
  const line = `import { ${names.join(", ")} } from "${SPEC}";`;
  next = imports.length ? next.slice(0, imports[imports.length - 1]!.getEnd()) + "\n" + line + next.slice(imports[imports.length - 1]!.getEnd()) : line + "\n" + next;
  edits += patches.length;
  if (write) { snapshot[f] = text; writeFileSync(ROOT + f, next); }
}
const sp = opt("--snapshot"); if (write && sp) writeFileSync(sp, JSON.stringify(snapshot));
const rp = opt("--report"); if (rp) writeFileSync(rp, JSON.stringify(report, null, 1));
console.log(`${write ? "WROTE" : "DRY"}: edits=${edits} manual=${manual}`);
for (const r of report.filter(r => r.kind === "MANUAL")) console.log("MANUAL", r.file.replace(/.*plugins\//, "P/").slice(-100), r.line, r.tail.slice(0, 70), "|", r.note);
