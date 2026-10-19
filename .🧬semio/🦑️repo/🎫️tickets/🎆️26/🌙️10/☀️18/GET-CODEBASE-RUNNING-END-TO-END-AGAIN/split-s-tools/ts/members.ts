import { analyze, ts } from "./ast.ts";
const [path, cls] = process.argv.slice(2);
const a = analyze(path);
const c = a.sf.statements.find((s) => ts.isClassDeclaration(s) && s.name?.text === cls) as import("typescript").ClassDeclaration;
for (const m of c.members) {
  const name = m.name ? (m.name as any).text ?? m.name.getText(a.sf) : "<ctor>";
  const s = a.sf.getLineAndCharacterOfPosition(m.getStart(a.sf)).line + 1, e = a.sf.getLineAndCharacterOfPosition(m.getEnd()).line + 1;
  const mods = (ts.canHaveModifiers(m) ? ts.getModifiers(m) ?? [] : []).map((x) => x.getText(a.sf)).join(" ");
  console.log(`${s}-${e} ${e - s + 1} ${ts.SyntaxKind[m.kind]} ${mods} ${name}`);
}
