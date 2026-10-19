// 🔬 Prints the first rejecting construct of each script: statement, expression and validator line. usage: bun diag.ts <list.txt> <out.json>
import {readFileSync, writeFileSync} from "node:fs";
import {ecmaCommandRouterModule, SPOT, WHY} from "./routes-why.ts";
import {ecmaProgram} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🟨️ecma/📦️packages/🟦️typescript/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import ts from "typescript";
const g = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
const out: any[] = [];
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  let s = readFileSync("/Users/ueli/Documents/semio/" + p, "utf8"); if (process.argv.includes("--flip")) s = s.replace(/throw Error\(/g, "throw new Error(").replace(/(from\s*"[^"]+")\s*with\s*\{[^}]*\}/g, "$1");
  WHY.length = 0; SPOT.length = 0;
  if (!ecmaProgram(s)) {
    const f = ts.createSourceFile(p, s, ts.ScriptTarget.Latest, true);
    let bad = "";
    for (const st of f.statements) {
      const k = ts.SyntaxKind[st.kind];
      if (ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st)) { bad = k; break; }
      if (ts.isTryStatement(st) || ts.isSwitchStatement(st) || ts.isWhileStatement(st) || ts.isForStatement(st) || ts.isForInStatement(st)) { bad = k; break; }
      if (ts.isExportDeclaration(st) || ts.isExportAssignment(st)) { bad = k; break; }
      const t = st.getText();
      if (/^(export\s+)?(default\s+)?(async\s+)?function\s*\*/.test(t)) { bad = "generator"; break; }
      if (/^import\b/.test(t) && /\bwith\s*\{/.test(t)) { bad = "import-attributes"; break; }
      if (/^(export\s+)?abstract\s+class/.test(t)) { bad = "abstract-class"; break; }
      if (ts.isClassDeclaration(st) && !/extends/.test(t.split("{")[0]!)) { bad = "class-without-extends"; break; }
    }
    let first = "";
    if (!bad) for (const st of f.statements) { if (!ecmaProgram(st.getText())) { first = st.getText().slice(0, 220); bad = "other:" + ts.SyntaxKind[st.kind]; break; } }
    out.push({p, parsed: false, kind: bad || "other-syntax", first});
    continue;
  }
  const ok = ecmaCommandRouterModule(s, g);
  const sp = SPOT.filter((x) => x.line !== 285 && x.line !== 286).at(-1) ?? SPOT[0];
  out.push({p, parsed: true, ok, line: sp?.line ?? null, expr: sp?.expr ? s.slice(sp.expr[0], sp.expr[1]).slice(0, 200) : null, stmt: sp?.stmt ? s.slice(sp.stmt[0], sp.stmt[1]).slice(0, 160) : null});
}
writeFileSync(process.argv[3]!, JSON.stringify(out, null, 1));
const c = new Map<string, number>();
for (const r of out) { const k = r.parsed ? "L" + r.line : "U:" + r.kind; c.set(k, (c.get(k) ?? 0) + 1); }
console.log([...c].sort((a, b) => b[1] - a[1]));
