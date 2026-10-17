const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const fs = require("fs");
for (const file of process.argv.slice(2)) {
  const text = fs.readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
  sf.statements.forEach((s) => {
    const docs = (s.jsDoc || []);
    if (docs.length > 1) console.log(file.split("/").slice(-3).join("/"), "stmt@" + L(s.getStart(sf, false)), "jsdoc blocks", docs.map((d) => L(d.getStart(sf)) + "-" + L(d.getEnd())).join(","));
  });
}
