// usage: node fnlen.cjs <file>... : longest outermost function-like body per file, and every one over 300 lines
const fs = require("fs");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
for (const f of process.argv.slice(2)) {
  const text = fs.readFileSync(f, "utf8");
  const sf = ts.createSourceFile(f, text, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const rows = [];
  const visit = (n) => {
    if (ts.isFunctionLike(n) && n.body && ts.isBlock(n.body)) {
      const a = sf.getLineAndCharacterOfPosition(n.getStart(sf)).line, b = sf.getLineAndCharacterOfPosition(n.getEnd()).line;
      rows.push({ len: b - a + 1, name: (n.name && n.name.getText(sf)) || ts.SyntaxKind[n.kind], line: a + 1 });
      return;
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  rows.sort((x, y) => y.len - x.len);
  const top = rows[0];
  console.log(`${String(top ? top.len : 0).padStart(4)} ${top ? top.name : "-"}@${top ? top.line : 0}  ${f.split("/").slice(-3).join("/")}`);
}
