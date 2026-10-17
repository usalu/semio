// usage: node hookcheck.cjs <orig.tsx> <fnName> <tailFrom> <hookFile...>
const fs = require("fs");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const [origPath, fnName, tailFromS, ...hookFiles] = process.argv.slice(2);
const parse = (p) => { const text = fs.readFileSync(p, "utf8"); return { text, sf: ts.createSourceFile(p, text, ts.ScriptTarget.Latest, true, p.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) }; };
const norm = (s) => s.replace(/\s+/g, "");
const o = parse(origPath);
const fn = o.sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === fnName);
const stmts = fn.body.statements;
const tailFrom = +tailFromS;
const origBody = stmts.slice(0, tailFrom).map((s) => o.text.slice(s.getFullStart(), s.getEnd())).join("");
let hooksBody = "";
hookFiles.forEach((hp, k) => {
  const h = parse(hp);
  const f = h.sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && /^use/.test(s.name.text));
  let st = [...f.body.statements];
  if (st.length && ts.isReturnStatement(st[st.length - 1])) st.pop();
  if (k > 0) st.shift();
  hooksBody += st.map((s) => h.text.slice(s.getFullStart(), s.getEnd())).join("");
});
const a = norm(origBody), b = norm(hooksBody);
console.log("orig body statements 0.." + tailFrom + " chars:", a.length, " hooks bodies chars:", b.length, " equal:", a === b);
if (a !== b) { let i = 0; while (a[i] === b[i]) i++; console.log("first diff at", i, JSON.stringify(a.slice(i - 80, i + 80)), "|||", JSON.stringify(b.slice(i - 80, i + 80))); }
