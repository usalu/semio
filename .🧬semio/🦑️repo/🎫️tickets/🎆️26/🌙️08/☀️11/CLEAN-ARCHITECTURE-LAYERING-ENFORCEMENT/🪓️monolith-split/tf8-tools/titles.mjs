import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync } from "node:fs";
const out = [];
for (const file of process.argv.slice(2)) {
  const sf = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true);
  const title = (n) => (n && (ts.isStringLiteral(n) || ts.isNoSubstitutionTemplateLiteral(n)) ? n.text : n ? "<<" + n.getText(sf) + ">>" : "?");
  const visit = (n, stack) => {
    if (ts.isCallExpression(n)) {
      let callee = n.expression;
      if (ts.isPropertyAccessExpression(callee)) callee = callee.expression;
      const name = ts.isIdentifier(callee) ? callee.text : "";
      if (name === "describe") { const t = title(n.arguments[0]); out.push(`describe\t${stack.join(" > ")}${stack.length ? " > " : ""}${t}`); n.arguments.slice(1).forEach((a) => visit(a, [...stack, t])); return; }
      if (name === "test" || name === "it") { out.push(`test\t${stack.join(" > ")}${stack.length ? " > " : ""}${title(n.arguments[0])}`); return; }
    }
    ts.forEachChild(n, (c) => visit(c, stack));
  };
  visit(sf, []);
}
console.log(out.sort().join("\n"));
