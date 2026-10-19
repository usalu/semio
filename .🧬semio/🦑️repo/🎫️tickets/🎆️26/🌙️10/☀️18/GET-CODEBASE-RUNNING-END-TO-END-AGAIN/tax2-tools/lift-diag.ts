// 🔬 Shows why lifted-but-still-rejected scripts fail. usage: bun lift-diag.ts <list.txt> <out.json>
import {readFileSync, writeFileSync} from "node:fs";
import {planLift} from "./lift.ts";
import {ecmaCommandRouterModule, SPOT, WHY} from "./routes-why.ts";
import {ecmaProgram} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🟨️ecma/📦️packages/🟦️typescript/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import ts from "typescript";
const g = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
const out: any[] = [];
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const src = readFileSync("/Users/ueli/Documents/semio/" + p, "utf8");
  const plan = planLift(p, src);
  if (!plan.ok) { out.push({p, lift: plan.reason}); continue; }
  const s = plan.script;
  SPOT.length = 0; WHY.length = 0;
  if (!ecmaProgram(s)) {
    const f = ts.createSourceFile(p, s, ts.ScriptTarget.Latest, true);
    let first = "";
    for (const st of f.statements) if (!ecmaProgram(st.getText())) { first = st.getText().slice(0, 200); break; }
    out.push({p, lift: "ok", result: "unparseable", first});
    continue;
  }
  const ok = ecmaCommandRouterModule(s, g);
  const sp = SPOT.filter((x) => x.line !== 285 && x.line !== 286).at(0) ?? SPOT[0];
  out.push({p, lift: "ok", result: ok ? "accepted" : "rejected", line: sp?.line, expr: sp?.expr ? s.slice(sp.expr[0], sp.expr[1]).slice(0, 200) : null, stmt: sp?.stmt ? s.slice(sp.stmt[0], sp.stmt[1]).slice(0, 200) : null});
}
writeFileSync(process.argv[3]!, JSON.stringify(out, null, 1));
const norm = (x: string) => (x || "").replace(/"[^"\n]*"|'[^'\n]*'|`[^`]*`/g, "S").replace(/\s+/g, " ").slice(0, 90);
const c = new Map<string, number>();
for (const r of out) { const k = r.lift !== "ok" ? "lift:" + r.lift : r.result === "unparseable" ? "unparseable: " + norm(r.first).slice(0, 60) : r.result === "accepted" ? "accepted" : "L" + r.line + " " + norm(r.stmt).slice(0, 70); c.set(k, (c.get(k) ?? 0) + 1); }
for (const [k, v] of [...c].sort((a, b) => b[1] - a[1]).slice(0, 60)) console.log(v, k);
