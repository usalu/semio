import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const apply = process.argv.includes("--apply");
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const kind = (f: string) => (f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const usedIn = (sf: ts.SourceFile): Set<string> => {
  const used = new Set<string>();
  const visit = (n: ts.Node) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const isName = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isPropertySignature(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || (ts.isMethodDeclaration(p) && p.name === n);
      if (!isName) used.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return used;
};
let total = 0;
for (const f of files) {
  let cur: string; try { cur = readFileSync(f, "utf8"); } catch { continue; }
  const rel = f.slice(root.length + 1);
  const head = Bun.spawnSync(["git", "-c", "core.quotepath=off", "show", `HEAD:${rel}`], { cwd: root });
  if (head.exitCode !== 0) continue;
  const headSf = ts.createSourceFile(f, head.stdout.toString(), ts.ScriptTarget.Latest, true, kind(f));
  const headUsed = usedIn(headSf);
  const sf = ts.createSourceFile(f, cur, ts.ScriptTarget.Latest, true, kind(f));
  const used = usedIn(sf);
  const edits: [number, number, string][] = [];
  for (const s of sf.statements) {
    if (!ts.isImportDeclaration(s) || !s.importClause) continue;
    const c = s.importClause;
    const dead = (name: string) => !used.has(name) && headUsed.has(name);
    const names: { text: string; local: string; kind: string }[] = [];
    if (c.name) names.push({ text: c.name.text, local: c.name.text, kind: "default" });
    let ns: ts.NamespaceImport | undefined;
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) ns = c.namedBindings;
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) names.push({ text: e.getText(), local: e.name.text, kind: "named" });
    const allLocals = [...names.map((n) => n.local), ...(ns ? [ns.name.text] : [])];
    if (!allLocals.some(dead)) continue;
    const keepNames = names.filter((n) => !dead(n.local));
    const keepNs = ns && !dead(ns.name.text) ? ns : undefined;
    let rep = "";
    if (keepNames.length || keepNs) {
      const def = keepNames.find((n) => n.kind === "default");
      const nm = keepNames.filter((n) => n.kind === "named");
      const parts: string[] = [];
      if (def) parts.push(def.text);
      if (keepNs) parts.push(keepNs.getText());
      if (nm.length) parts.push(`{ ${nm.map((n) => n.text).join(", ")} }`);
      const attrs = s.attributes ? ` ${s.attributes.getText()}` : "";
      rep = `import ${c.isTypeOnly ? "type " : ""}${parts.join(", ")} from ${s.moduleSpecifier.getText()}${attrs};`;
      edits.push([s.getStart(), s.getEnd(), rep]);
    } else {
      let en = s.getEnd(); if (cur[en] === "\n") en++;
      edits.push([s.getStart(), en, ""]);
    }
    total += allLocals.filter(dead).length;
    console.log(rel.split("/").slice(-3).join("/"), "removes", allLocals.filter(dead).join(","));
  }
  if (apply && edits.length) { let out = cur; for (const [a, b, r] of edits.sort((x, y) => y[0] - x[0])) out = out.slice(0, a) + r + out.slice(b); writeFileSync(f, out); }
}
console.log("unused bindings:", total);
