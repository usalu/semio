// usage: node stm.cjs <file> <function> [line...] : lists statements of the function body, or of the block (try / if-then / for / arrow body) of the statement starting at each given line, nested in turn
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, name, ...path] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let fn;
const find = (n) => { if (!fn && name.startsWith("@") && ts.isFunctionLike(n) && n.body && L(n.getStart(sf)) === +name.slice(1)) { fn = n; return; } if (fn) return; if (ts.isFunctionDeclaration(n) && n.name && n.name.text === name) { fn = n; return; } if (ts.isMethodDeclaration(n) && n.name && n.name.getText(sf) === name) { fn = n; return; } if (ts.isVariableDeclaration(n) && ts.isIdentifier(n.name) && n.name.text === name && n.initializer && ts.isFunctionLike(n.initializer)) { fn = n.initializer; return; } ts.forEachChild(n, find); };
find(sf);
let block = fn.body;
for (const lineStr of path) {
  const line = +lineStr;
  let hit = null;
  const search = (n) => { if (hit) return; if (ts.isStatement(n) && L(n.getStart(sf)) === line && n !== block) { hit = n; return; } ts.forEachChild(n, search); };
  search(block);
  if (!hit) throw new Error("no statement at " + line);
  let b = null;
  if (ts.isTryStatement(hit)) b = hit.tryBlock;
  else if (ts.isIfStatement(hit)) b = ts.isBlock(hit.thenStatement) ? hit.thenStatement : null;
  else if ((ts.isForStatement(hit) || ts.isForOfStatement(hit) || ts.isWhileStatement(hit)) && ts.isBlock(hit.statement)) b = hit.statement;
  else if (ts.isBlock(hit)) b = hit;
  else if (ts.isVariableStatement(hit)) { const init = hit.declarationList.declarations[0].initializer; b = init && ts.isFunctionLike(init) && init.body && ts.isBlock(init.body) ? init.body : null; }
  if (!b) throw new Error("statement at " + line + " has no block");
  block = b;
}
for (const s of block.statements) console.log(`${L(s.getStart(sf))}-${L(s.getEnd())} (${L(s.getEnd()) - L(s.getStart(sf)) + 1}) ${s.getText(sf).split("\n")[0].slice(0, 110)}`);
