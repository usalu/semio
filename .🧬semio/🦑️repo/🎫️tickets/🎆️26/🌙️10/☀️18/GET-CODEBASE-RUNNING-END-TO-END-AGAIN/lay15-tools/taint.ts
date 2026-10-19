import ts from "typescript";
import { readFileSync } from "node:fs";
const [file, upwardList, wantedList] = process.argv.slice(2) as [string, string, string];
const sf = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true);
const upward = new Set(upwardList.split(","));
const wanted = wantedList.split(",");
const decl = new Map<string, ts.Statement>();
for (const st of sf.statements) {
  if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) decl.set(d.name.getText(), st);
  else if ((ts.isFunctionDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isClassDeclaration(st)) && st.name) decl.set(st.name.text, st);
}
const refs = new Map<ts.Statement, Set<string>>();
const idsOf = (st: ts.Statement): Set<string> => {
  const out = new Set<string>();
  const v = (n: ts.Node): void => { if (ts.isIdentifier(n)) out.add(n.text); ts.forEachChild(n, v); };
  v(st);
  return out;
};
for (const st of new Set(decl.values())) refs.set(st, idsOf(st));
const usesUpward = (st: ts.Statement): string[] => [...refs.get(st)!].filter(i => upward.has(i));
const tainted = new Set<ts.Statement>();
let changed = true;
for (const st of refs.keys()) if (usesUpward(st).length) tainted.add(st);
while (changed) {
  changed = false;
  for (const [st, ids] of refs) { if (tainted.has(st)) continue; for (const id of ids) { const d = decl.get(id); if (d && d !== st && tainted.has(d)) { tainted.add(st); changed = true; break; } } }
}
const name = (st: ts.Statement): string => [...decl].filter(([, s]) => s === st).map(([n]) => n).join(",");
for (const w of wanted) {
  const st = decl.get(w);
  if (!st) { console.log("MISSING", w); continue; }
  console.log(w, tainted.has(st) ? "TAINTED via " + usesUpward(st).join("|") : "clean");
}
const closure = new Set<ts.Statement>();
const stack = wanted.map(w => decl.get(w)).filter((s): s is ts.Statement => !!s);
while (stack.length) { const st = stack.pop()!; if (closure.has(st)) continue; closure.add(st); for (const id of refs.get(st)!) { const d = decl.get(id); if (d && d !== st) stack.push(d); } }
console.log("closure of wanted:", [...closure].map(name).join(" ").slice(0, 3000));
console.log("closure tainted:", [...closure].filter(s => tainted.has(s)).map(name).join(" "));
console.log("tainted overall:", [...tainted].map(name).join(" "));
