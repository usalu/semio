import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const LOGGERS = new Set(["log", "info", "warn", "error", "debug", "trace"]);
const PURE = (n) => ts.isIdentifier(n) || ts.isPropertyAccessExpression(n) && PURE(n.expression) || ts.isLiteralExpression(n) || n.kind === ts.SyntaxKind.TrueKeyword || n.kind === ts.SyntaxKind.FalseKeyword || n.kind === ts.SyntaxKind.ThisKeyword || ts.isPrefixUnaryExpression(n) && PURE(n.operand) || ts.isParenthesizedExpression(n) && PURE(n.expression) || ts.isBinaryExpression(n) && n.operatorToken.kind !== ts.SyntaxKind.EqualsToken && !ts.isAssignmentOperator?.(n.operatorToken.kind) && PURE(n.left) && PURE(n.right) || ts.isElementAccessExpression(n) && PURE(n.expression) && PURE(n.argumentExpression) || ts.isNonNullExpression(n) && PURE(n.expression);
const hasDebug = (n, sf) => n.getText(sf).includes("[DEBUG]");
const isLogStatement = (n, sf) => ts.isExpressionStatement(n) && ts.isCallExpression(n.expression) && ts.isPropertyAccessExpression(n.expression.expression) && ts.isIdentifier(n.expression.expression.expression) && n.expression.expression.expression.text === "console" && LOGGERS.has(n.expression.expression.name.text) && hasDebug(n, sf);
export function stripDebugTs(text, name, notes) {
  const sf = ts.createSourceFile(name, text, ts.ScriptTarget.Latest, true, name.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const cuts = [];
  const cut = (node) => cuts.push([node.getFullStart(), node.end, node]);
  const outer = (stmt) => {
    let node = stmt;
    for (;;) {
      const p = node.parent;
      if (ts.isBlock(p) && p.statements.length === 1 && p.parent && (ts.isIfStatement(p.parent) || ts.isForStatement(p.parent) || ts.isForOfStatement(p.parent) || ts.isForInStatement(p.parent) || ts.isWhileStatement(p.parent) || ts.isDoStatement(p.parent))) node = p.parent;
      else if ((ts.isIfStatement(p) && p.thenStatement === node && !p.elseStatement) || ts.isForStatement(p) || ts.isForOfStatement(p) || ts.isForInStatement(p) || ts.isWhileStatement(p)) node = p;
      else if (ts.isIfStatement(p) && p.elseStatement === node) { notes.push("else-branch debug at " + (sf.getLineAndCharacterOfPosition(node.getStart(sf)).line + 1)); return null; }
      else if (ts.isIfStatement(p) && p.thenStatement === node && p.elseStatement) { notes.push("then-branch with else at " + (sf.getLineAndCharacterOfPosition(node.getStart(sf)).line + 1)); return null; }
      else if (ts.isBlock(p) && p.statements.length === 1 && (ts.isTryStatement(p.parent) || ts.isFunctionLike(p.parent) || ts.isCatchClause(p.parent))) { return node; }
      else return node;
      if (node === stmt) return node;
    }
  };
  const visit = (n) => {
    if (isLogStatement(n, sf)) {
      const o = outer(n);
      if (!o) return;
      if (o !== n) {
        const cond = ts.isIfStatement(o) ? o.expression : ts.isWhileStatement(o) ? o.expression : null;
        if (cond && !PURE(cond)) { notes.push("impure condition kept as statement at " + (sf.getLineAndCharacterOfPosition(o.getStart(sf)).line + 1) + ": " + cond.getText(sf).slice(0, 80)); }
        else if (!cond) notes.push("loop removed at " + (sf.getLineAndCharacterOfPosition(o.getStart(sf)).line + 1));
      }
      cut(o);
      return;
    }
    if (ts.isArrowFunction(n) && !ts.isBlock(n.body) && ts.isCallExpression(n.body) && hasDebug(n.body, sf) && ts.isPropertyAccessExpression(n.body.expression) && n.body.expression.expression.getText(sf) === "console") {
      notes.push("arrow-expression debug at " + (sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1));
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  cuts.sort((a, b) => a[0] - b[0]);
  let out = "", pos = 0;
  for (const [fs, end, node] of cuts) {
    let start = node.getStart(sf);
    const lineStart = text.lastIndexOf("\n", start - 1) + 1;
    const lineEnd = text.indexOf("\n", end);
    const before = text.slice(lineStart, start), after = text.slice(end, lineEnd < 0 ? text.length : lineEnd);
    if (/^\s*$/.test(before) && /^\s*$/.test(after)) { out += text.slice(pos, lineStart); pos = lineEnd < 0 ? text.length : lineEnd + 1; }
    else { out += text.slice(pos, start); pos = end; }
  }
  out += text.slice(pos);
  return out;
}
