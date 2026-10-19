import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
const ts = createRequire("/Users/ueli/Documents/semio/package.json")("typescript");
const MODS = new Set([ts.SyntaxKind.PublicKeyword, ts.SyntaxKind.PrivateKeyword, ts.SyntaxKind.ProtectedKeyword, ts.SyntaxKind.ReadonlyKeyword, ts.SyntaxKind.OverrideKeyword]);
for (const file of process.argv.slice(2)) {
  const text = readFileSync(file, "utf8");
  const source = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true);
  const edits = [];
  const visit = (node) => {
    if (ts.isConstructorDeclaration(node) && node.body && node.parameters.some((p) => (p.modifiers ?? []).some((m) => MODS.has(m.kind)))) {
      const fields = [], names = [];
      for (const p of node.parameters) {
        const mods = (p.modifiers ?? []).filter((m) => MODS.has(m.kind));
        if (!mods.length) continue;
        if (!ts.isIdentifier(p.name)) throw new Error("destructured parameter property in " + file);
        const name = p.name.text, type = p.type ? p.type.getText(source) : undefined;
        if (!type) throw new Error("untyped parameter property " + name + " in " + file);
        fields.push(`${mods.map((m) => m.getText(source)).join(" ")} ${name}${p.questionToken ? "?" : ""}: ${type};`);
        names.push(name);
        edits.push({ start: mods[0].getStart(source), end: p.name.getStart(source), text: "" });
      }
      const indent = text.slice(text.lastIndexOf("\n", node.getStart(source)) + 1, node.getStart(source));
      edits.push({ start: node.getStart(source), end: node.getStart(source), text: fields.map((f) => f + "\n" + indent).join("") });
      const statements = node.body.statements;
      const superCall = statements.find((s) => ts.isExpressionStatement(s) && ts.isCallExpression(s.expression) && s.expression.expression.kind === ts.SyntaxKind.SuperKeyword);
      const assigns = names.map((n) => `this.${n} = ${n};`).join(" ");
      if (superCall) edits.push({ start: superCall.end, end: superCall.end, text: (text.slice(superCall.getStart(source), superCall.end).endsWith(";") ? " " : "; ") + assigns });
      else if (statements.length) edits.push({ start: statements[0].getStart(source), end: statements[0].getStart(source), text: assigns + " " });
      else edits.push({ start: node.body.getStart(source) + 1, end: node.body.getStart(source) + 1, text: " " + assigns + " " });
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  if (!edits.length) { console.log(file, 0); continue; }
  edits.sort((a, b) => b.start - a.start || b.end - a.end);
  let out = text;
  for (const e of edits) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  writeFileSync(file, out);
  console.log(file, edits.length);
}
