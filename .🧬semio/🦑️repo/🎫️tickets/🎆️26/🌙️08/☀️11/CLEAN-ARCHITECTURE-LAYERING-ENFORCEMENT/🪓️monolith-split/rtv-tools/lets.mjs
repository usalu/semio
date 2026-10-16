import { load } from "./graph.mjs";
const { tops, checker, ts, sf } = load(process.argv[2]);
const lets = new Map();
for (const t of tops) if (ts.isVariableStatement(t.st) && (t.st.declarationList.flags & ts.NodeFlags.Let)) for (const d of t.st.declarationList.declarations) lets.set(checker.getSymbolAtLocation(d.name), { t, name: d.name.text });
const isWrite = (n) => { const p = n.parent; if (ts.isBinaryExpression(p) && p.left === n && p.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && p.operatorToken.kind <= ts.SyntaxKind.LastAssignment) return true; if ((ts.isPrefixUnaryExpression(p) || ts.isPostfixUnaryExpression(p)) && (p.operator === ts.SyntaxKind.PlusPlusToken || p.operator === ts.SyntaxKind.MinusMinusToken)) return true; return false; };
const writers = new Map();
const visit = (n, t) => { if (ts.isIdentifier(n)) { const s = checker.getSymbolAtLocation(n); const l = lets.get(s); if (l && isWrite(n)) { const k = l.name; if (!writers.has(k)) writers.set(k, new Set()); writers.get(k).add(t.names[0] ?? t.kind); } } ts.forEachChild(n, (c) => visit(c, t)); };
for (const t of tops) if (!t.isImport) visit(t.st, t);
for (const [s, l] of lets) console.log(`let ${l.name} declared in ${l.t.names.join(",")}@${l.t.region}; written by: ${[...(writers.get(l.name) ?? [])].join(", ")}`);
