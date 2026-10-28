// Streams test files, parses each with the TypeScript compiler API and prints one JSON row per cwd-based root call outside string literals.
import ts from "typescript";
import { readFileSync } from "node:fs";
const files = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean);
const forbidden = (node: ts.Node): string | undefined => {
  if (!ts.isCallExpression(node)) return undefined;
  const text = node.expression.getText();
  if (node.arguments.length === 0 && (text === "process.cwd" || text === "Deno.cwd" || text === "Bun.cwd")) return text + "()";
  if (node.arguments.length === 0 && text === "findWorkspaceRoot") return "findWorkspaceRoot()";
  return undefined;
};
for (const file of files) {
  let text: string;
  try { text = readFileSync(file, "utf8"); } catch { continue; }
  const kind = file.endsWith(".tsx") ? ts.ScriptKind.TSX : file.endsWith(".mjs") || file.endsWith(".js") ? ts.ScriptKind.JS : ts.ScriptKind.TS;
  const source = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, kind);
  const chdir = /process\.chdir\(/.test(text);
  const visit = (node: ts.Node): void => {
    const hit = forbidden(node);
    if (hit) {
      const { line, character } = source.getLineAndCharacterOfPosition(node.getStart());
      let parent: ts.Node = node.parent, callee = "", index = -1, decl = "", prop = "";
      if (ts.isCallExpression(parent) && parent.arguments.includes(node as ts.Expression)) { callee = parent.expression.getText().slice(0, 80); index = parent.arguments.indexOf(node as ts.Expression); }
      else if (ts.isVariableDeclaration(parent)) decl = parent.name.getText().slice(0, 40);
      else if (ts.isPropertyAssignment(parent)) prop = parent.name.getText().slice(0, 40);
      else if (ts.isBinaryExpression(parent)) { const g = parent.parent; if (ts.isVariableDeclaration(g)) decl = g.name.getText().slice(0, 40); else if (ts.isPropertyAssignment(g)) prop = g.name.getText().slice(0, 40); }
      else if (ts.isPropertyAccessExpression(parent) || ts.isElementAccessExpression(parent)) callee = "member";
      console.log(JSON.stringify({ f: file, l: line + 1, c: character + 1, hit, callee, index, decl, prop, chdir, kind: ts.ScriptKind[kind] }));
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
}
