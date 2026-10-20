// usage: node titles.cjs <file> <fnName> : in-order flat list of describe/it/test titles of a registration function, descending into calls of relative-imported functions
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const [file, name] = process.argv.slice(2);
const cache = new Map();
const parse = (f) => { if (!cache.has(f)) { const t = fs.readFileSync(f, "utf8"); cache.set(f, ts.createSourceFile(f, t, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS)); } return cache.get(f); };
const findFn = (sf, nm) => { let r = null; const w = (n) => { if (r) return; if (ts.isFunctionDeclaration(n) && n.name && n.name.text === nm) r = n; else if (ts.isVariableDeclaration(n) && ts.isIdentifier(n.name) && n.name.text === nm && n.initializer && ts.isFunctionLike(n.initializer)) r = n.initializer; else ts.forEachChild(n, w); }; w(sf); return r; };
const imports = (sf) => { const m = new Map(); for (const s of sf.statements) if (ts.isImportDeclaration(s) && s.importClause?.namedBindings && ts.isNamedImports(s.importClause.namedBindings) && ts.isStringLiteral(s.moduleSpecifier) && s.moduleSpecifier.text.startsWith(".")) for (const e of s.importClause.namedBindings.elements) m.set(e.name.text, path.resolve(path.dirname(sf.fileName), s.moduleSpecifier.text)); return m; };
const out = [];
const walkFn = (f, nm, seen) => {
  const sf = parse(f);
  const fn = findFn(sf, nm);
  if (!fn) { out.push("<<missing " + nm + ">>"); return; }
  const imp = imports(sf);
  const visit = (n) => {
    if (ts.isCallExpression(n)) {
      const ex = n.expression.getText(sf);
      if (/^(describe|it|test)(\.\w+)*$/.test(ex) || /^(describe|it|test)\.\w+\(/.test(ex)) { const a = n.arguments[0]; out.push(ex + ":" + (a && ts.isStringLiteralLike(a) ? a.text : a ? a.getText(sf).slice(0, 40) : "")); }
      else if (/^(describe|it|test)\./.test(ex)) { const a = n.arguments[0]; out.push(ex + ":" + (a && ts.isStringLiteralLike(a) ? a.text : "")); }
      if (ts.isIdentifier(n.expression)) {
        const id = n.expression.text;
        const target = imp.get(id);
        const local = findFn(sf, id);
        const key = f + "#" + id + "#" + n.getStart(sf);
        if (target && !seen.has(key)) { seen.add(key); const tf = [target + ".ts", target + ".tsx", target].find((p) => fs.existsSync(p) && fs.statSync(p).isFile()); if (tf) walkFn(tf, id, seen); }
        else if (local && local !== fn && /^(register|create|use)/.test(id) && !seen.has(key)) { seen.add(key); walkFn(f, id, seen); }
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(fn.body);
};
walkFn(path.resolve(file), name, new Set());
console.log(JSON.stringify(out));
