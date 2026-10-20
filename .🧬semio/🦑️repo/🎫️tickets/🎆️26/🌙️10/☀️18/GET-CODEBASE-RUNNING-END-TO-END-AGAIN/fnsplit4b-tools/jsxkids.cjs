// usage: node jsxkids.cjs <file> <function>: lists the children of the root JSX element returned by a function component (line ranges, tag names)
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, name] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === name);
const ret = fn.body.statements.find((s) => ts.isReturnStatement(s));
let e = ret.expression; while (ts.isParenthesizedExpression(e)) e = e.expression;
const show = (el, depth) => {
  const kids = el.children || [];
  for (const k of kids) {
    if (ts.isJsxText(k) && !k.text.trim()) continue;
    const a = L(k.getStart(sf)), b = L(k.getEnd());
    console.log(`${"  ".repeat(depth)}${a}-${b} (${b - a + 1}) ${ts.SyntaxKind[k.kind]} ${ts.isJsxElement(k) ? k.openingElement.tagName.getText(sf) : ts.isJsxExpression(k) ? k.getText(sf).slice(0, 60).replace(/\n.*/s, "") : ""}`);
  }
};
console.log("root", ts.SyntaxKind[e.kind], e.openingElement ? e.openingElement.tagName.getText(sf) : "");
if (e.children) show(e, 1);
if (e.children) for (const k of e.children) if (ts.isJsxElement(k) && L(k.getEnd()) - L(k.getStart(sf)) > 80) { console.log("--", L(k.getStart(sf))); show(k, 2); }
