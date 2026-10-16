import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const cfgPath = path.resolve(process.argv[2]);
const WRITE = process.argv.includes("--write");
const cfg = (await import(pathToFileURL(cfgPath).href)).default;
const outArg = process.argv.indexOf("--out");
if (outArg > 0) cfg.testsDir = path.resolve(process.argv[outArg + 1]);

const fileName = cfg.source;
let text = fs.readFileSync(fileName, "utf8");
if (cfg.blankHeader) text = text.split("\n").map((l, i) => (i < cfg.blankHeader ? "" : l)).join("\n");
const sf = ts.createSourceFile(fileName, text, ts.ScriptTarget.Latest, true, cfg.isTsx ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const host = ts.createCompilerHost({});
const origGetSourceFile = host.getSourceFile.bind(host);
host.getSourceFile = (name, ...rest) => (path.resolve(name) === path.resolve(fileName) ? sf : origGetSourceFile(name, ...rest));
const program = ts.createProgram([fileName], { noResolve: true, noLib: true, jsx: ts.JsxEmit.Preserve, target: ts.ScriptTarget.ESNext, skipLibCheck: true, types: [], allowImportingTsExtensions: true, resolveJsonModule: true }, host);
const checker = program.getTypeChecker();
const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;
const errors = [];
const fail = (m) => errors.push(m);
const SHARED = "__shared__";
const closure = cfg.mode === "closure";

const closureFn = closure ? sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name?.text === cfg.fnName) : null;
const container = closure ? closureFn.body.statements : sf.statements;
const importDecls = sf.statements.filter(ts.isImportDeclaration);
const unitByNode = new Map();
const containerBodies = new Set([sf, closure ? closureFn.body : sf]);

const TEST_HEADS = new Set(["describe", "it", "test"]);
function containsTestCall(node) {
  let found = false;
  const visit = (n) => {
    if (found) return;
    if (ts.isCallExpression(n)) {
      let callee = n.expression;
      while (ts.isCallExpression(callee)) callee = callee.expression;
      const head = (x) => (ts.isIdentifier(x) ? x.text : ts.isPropertyAccessExpression(x) ? head(x.expression) : "");
      if (TEST_HEADS.has(head(callee))) {
        found = true;
        return;
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
  return found;
}
const isDeclStmt = (s) => ts.isVariableStatement(s) || ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isEnumDeclaration(s);

function makeUnits(stmts, parent) {
  const out = [];
  let prev = null;
  let carry = "";
  for (const stmt of stmts) {
    const isImp = !parent && ts.isImportDeclaration(stmt);
    const full = stmt.getFullStart();
    const start = stmt.getStart();
    const trivia = text.slice(full, start);
    const nl = trivia.indexOf("\n");
    let sameLine;
    let rest;
    if (nl >= 0) {
      sameLine = trivia.slice(0, nl);
      rest = trivia.slice(nl + 1);
    } else {
      sameLine = trivia;
      rest = "";
    }
    if (prev && sameLine.trim().startsWith("//")) prev.trail += " " + sameLine.trim();
    else if (sameLine.trim()) rest = sameLine + "\n" + rest;
    const restOffset = trivia.length - rest.length;
    const lines = rest.split("\n");
    let i = 0;
    while (i < lines.length && lines[i].trim() === "") i++;
    const endRegions = [];
    while (i < lines.length && /^\s*\/\/#endregion/.test(lines[i])) {
      endRegions.push(lines[i].trim());
      i++;
    }
    if (endRegions.length && prev) prev.trail += "\n" + endRegions.join("\n");
    else if (endRegions.length) fail(`endregion without previous unit at line ${lineOf(start)}`);
    const kept = lines.slice(i);
    const keptOffset = restOffset + lines.slice(0, i).reduce((a, l) => a + l.length + 1, 0);
    const leadSrc = kept.join("\n");
    const hasLead = leadSrc.trim() !== "";
    if (isImp) {
      if (hasLead) carry += (carry ? "\n" : "") + leadSrc.trimEnd();
      continue;
    }
    const rawStart = hasLead ? full + keptOffset : text.lastIndexOf("\n", start - 1) + 1;
    const u = { node: stmt, parent, pre: carry, rawStart, head: text.slice(rawStart, start), trail: "", startLine: lineOf(start), endLine: lineOf(stmt.end), kind: null, children: null };
    carry = "";
    if (isDeclStmt(stmt)) u.kind = "decl";
    else if (containsTestCall(stmt)) u.kind = "test";
    else u.kind = "other";
    unitByNode.set(stmt, u);
    out.push(u);
    prev = u;
  }
  return out;
}

const allUnits = [];
function register(list) {
  for (const u of list) {
    allUnits.push(u);
    const sp = cfg.splits?.[u.startLine];
    if (sp) {
      const call = u.node.expression;
      const cb = call.arguments.find((a) => ts.isArrowFunction(a) || ts.isFunctionExpression(a));
      u.splitBody = cb.body;
      containerBodies.add(cb.body);
      u.header = text.slice(u.node.getStart(), cb.body.getStart() + 1);
      u.footer = text.slice(text.lastIndexOf("\n", cb.body.end - 1) + 1, u.node.end);
      u.children = makeUnits(cb.body.statements, u);
      register(u.children);
    }
  }
}
const top = makeUnits(container, null);
const modTop = closure ? makeUnits(sf.statements.filter((s) => s !== closureFn), null) : [];
for (const u of modTop) u.module = true;
register(top);
register(modTop);

const splitSpec = (u) => (u.parent ? cfg.splits?.[u.parent.startLine] : null);
const isReplicate = (u) => !!u.parent && !!splitSpec(u)?.replicate?.includes(u.startLine);

function topicOfLine(line) {
  for (const t of cfg.topics) for (const [a, b] of t.ranges) if (line >= a && line <= b) return t.folder;
  return null;
}

const closureParams = closure ? new Map(closureFn.parameters.map((p) => [p, p.name.text])) : new Map();
const preamble = new Map();
if (closure) {
  for (const u of top) {
    if (u.kind === "decl" && ts.isVariableStatement(u.node)) {
      const decls = u.node.declarationList.declarations;
      if (decls.length === 1 && ts.isObjectBindingPattern(decls[0].name) && decls[0].initializer && ts.isIdentifier(decls[0].initializer) && closureParams.size && [...closureParams.values()].includes(decls[0].initializer.text)) {
        u.preamble = decls[0].initializer.text;
        for (const el of decls[0].name.elements) preamble.set(el, { from: decls[0].initializer.text });
      }
    }
  }
}

{
  let carryPre = "";
  for (const u of top) {
    if (u.preamble) {
      const head = u.head.trimEnd();
      const c = [u.pre, head].filter((x) => x && x.trim()).join("\n");
      if (c) carryPre += (carryPre ? "\n" : "") + c;
      if (u.trail) carryPre += u.trail;
    } else if (carryPre) {
      u.pre = carryPre + (u.pre ? "\n" + u.pre : "");
      carryPre = "";
    }
  }
  if (carryPre) fail("preamble comments left over");
}
const leaves = allUnits.filter((u) => !u.children && !u.preamble);
for (const u of leaves) {
  if (isReplicate(u)) {
    u.replicate = true;
    continue;
  }
  if (cfg.pin?.[u.startLine]) {
    u.topic = cfg.pin[u.startLine];
    continue;
  }
  if (u.kind === "test") {
    u.topic = topicOfLine(u.startLine);
    if (!u.topic) fail(`test unit at line ${u.startLine} has no topic`);
  }
}

function declNames(u) {
  const n = u.node;
  if (ts.isVariableStatement(n)) {
    const names = [];
    const walk = (b) => {
      if (ts.isIdentifier(b)) names.push(b.text);
      else if (ts.isObjectBindingPattern(b) || ts.isArrayBindingPattern(b)) for (const el of b.elements) if (ts.isBindingElement(el)) walk(el.name);
    };
    for (const d of n.declarationList.declarations) walk(d.name);
    return names;
  }
  if (n.name && ts.isIdentifier(n.name)) return [n.name.text];
  return [];
}
for (const u of allUnits) u.names = u.kind === "decl" ? declNames(u) : [];

function ownerUnitOf(d) {
  let n = d;
  while (n && n.parent) {
    if (unitByNode.has(n) && containerBodies.has(n.parent)) return unitByNode.get(n);
    n = n.parent;
  }
  return null;
}
function importOf(d) {
  let n = d;
  while (n) {
    if (ts.isImportDeclaration(n)) return n;
    n = n.parent;
  }
  return null;
}
const inside = (inner, outer) => inner.pos >= outer.pos && inner.end <= outer.end;

function refsOf(u) {
  return refsOfNode(u.node, u);
}
function refsOfNode(node, u) {
  const res = { imports: new Map(), units: new Set(), pre: new Set(), params: new Set() };
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      if (p) {
        if (ts.isPropertyAccessExpression(p) && p.name === n) return;
        if (ts.isQualifiedName(p) && p.right === n) return;
        if ((ts.isPropertyAssignment(p) || ts.isPropertySignature(p) || ts.isMethodDeclaration(p) || ts.isMethodSignature(p) || ts.isPropertyDeclaration(p) || ts.isGetAccessorDeclaration(p) || ts.isSetAccessorDeclaration(p) || ts.isEnumMember(p)) && p.name === n) return;
        if (ts.isBindingElement(p) && p.propertyName === n) return;
        if (ts.isJsxAttribute(p) && p.name === n) return;
        if (ts.isLabeledStatement(p) || ts.isBreakOrContinueStatement(p) || ts.isMetaProperty(p)) return;
      }
      let sym;
      if (p && ts.isShorthandPropertyAssignment(p) && p.name === n) sym = checker.getShorthandAssignmentValueSymbol(p);
      else sym = checker.getSymbolAtLocation(n);
      if (sym?.declarations) {
        for (const d of sym.declarations) {
          if (inside(d, node)) continue;
          const imp = importOf(d);
          if (imp) {
            const set = res.imports.get(imp) ?? res.imports.set(imp, new Set()).get(imp);
            set.add(d);
            continue;
          }
          if (closure && closureParams.has(d)) {
            res.params.add(d);
            continue;
          }
          if (ts.isBindingElement(d) && preamble.has(d)) {
            res.pre.add(d);
            continue;
          }
          if (ts.isBindingElement(d) || ts.isVariableDeclaration(d) || ts.isFunctionDeclaration(d) || ts.isClassDeclaration(d) || ts.isTypeAliasDeclaration(d) || ts.isInterfaceDeclaration(d) || ts.isEnumDeclaration(d)) {
            const owner = ownerUnitOf(d);
            if (owner && owner !== u && !owner.preamble) res.units.add(owner);
          }
        }
      }
    }
    ts.forEachChild(n, visit);
  };
  if (u?.children) {
    const call = u.node.expression;
    const hdr = (n) => {
      if (n === u.splitBody) return;
      visit(n);
    };
    ts.forEachChild(call, (c) => {
      if (ts.isArrowFunction(c) || ts.isFunctionExpression(c)) ts.forEachChild(c, hdr);
      else visit(c);
    });
  } else visit(node);
  return res;
}
for (const u of allUnits) u.refs = refsOf(u);

const paramTypeRefs = closure ? closureFn.parameters.map((p) => refsOfNode(p.type, null)) : [];
for (const r of paramTypeRefs) for (const d of r.units) d.forceSharedByParam = true;
const users = new Map(allUnits.map((u) => [u, new Set()]));
for (const u of allUnits) for (const d of u.refs.units) users.get(d)?.add(u);

for (const u of leaves) {
  if (u.topic || u.replicate) continue;
  if (u.kind === "test") continue;
  if (u.names.some((n) => cfg.forceShared?.includes(n)) || u.kind === "other" || u.forceSharedByParam) {
    u.topic = SHARED;
    continue;
  }
  const set = new Set();
  const seen = new Set();
  const stack = [u];
  while (stack.length) {
    const x = stack.pop();
    if (seen.has(x)) continue;
    seen.add(x);
    for (const user of users.get(x)) {
      if (user.replicate) continue;
      const owner = user.children ? null : user;
      if (!owner) continue;
      if (owner.kind === "test") {
        if (owner.topic) set.add(owner.topic);
      } else stack.push(owner);
    }
  }
  u.userTopics = set;
  if (set.size === 1) u.topic = [...set][0];
  else {
    u.topic = SHARED;
    if (set.size === 0) u.unused = true;
  }
}
let changed = true;
while (changed) {
  changed = false;
  for (const u of leaves) {
    if (u.topic === SHARED) {
      for (const dep of u.refs.units) {
        if (dep.kind === "decl" && dep.topic !== SHARED && !dep.replicate && !dep.children) {
          dep.topic = SHARED;
          changed = true;
        }
      }
    }
  }
}
for (const u of leaves) {
  if (!u.topic && !u.replicate) fail(`unit ${u.startLine} (${u.kind}) has no topic`);
  if (u.topic && u.topic !== SHARED) {
    for (const dep of u.refs.units) {
      if (dep.topic && dep.topic !== SHARED && dep.topic !== u.topic && !dep.replicate && !dep.children) fail(`unit ${u.startLine} (${u.names.join(",")}) in ${u.topic} depends on unit ${dep.startLine} (${dep.names.join(",")}) in ${dep.topic}`);
    }
  }
}
for (const u of allUnits) {
  if (u.children) {
    u.topics = new Set(u.children.filter((c) => c.topic && c.topic !== SHARED).map((c) => c.topic));
    for (const c of u.children) {
      if (c.replicate) for (const d of c.refs.units) if (d.topic && d.topic !== SHARED && d.kind === "decl") fail(`replicated unit ${c.startLine} depends on local decl ${d.startLine}`);
    }
  }
}

function siblingsOf(u) {
  return u.parent ? u.parent.children : u.module ? modTop : top;
}
const MARK = /^\s*\/\/#(region|endregion)\b/;
function moveMarkers() {
  const nameOf = (l) => l.replace(MARK, "").trim();
  const groups = new Map();
  for (const u of leaves) {
    const key = u.parent ? u.parent : u.module ? "mod" : "top";
    (groups.get(key) ?? groups.set(key, []).get(key)).push(u);
  }
  const sibsOf = (u) => siblingsOf(u);
  const seen = new Set();
  for (const u of leaves) {
    const sibs = sibsOf(u);
    if (seen.has(sibs)) continue;
    seen.add(sibs);
    const stack = [];
    const moves = [];
    sibs.forEach((x, i) => {
      if (x.preamble) return;
      const shared = x.topic === SHARED;
      const lines = [];
      for (const part of ["pre", "head", "trail"]) for (const l of x[part].split("\n")) if (MARK.test(l)) lines.push({ part, l: l.trim() });
      for (const m of lines) {
        const kind = MARK.exec(m.l)[1];
        if (kind === "region") stack.push({ ...m, name: nameOf(m.l), x, i, shared });
        else {
          let k = stack.length - 1;
          while (k >= 0 && stack[k].name !== nameOf(m.l)) k--;
          if (k < 0) {
            if (shared) moves.push({ kind: "end", x, l: m.l, part: m.part });
            continue;
          }
          const st = stack.splice(k, 1)[0];
          if (st.shared && !shared) moves.push({ kind: "start", x: st.x, l: st.l, part: st.part });
          else if (!st.shared && shared) moves.push({ kind: "end", x, l: m.l, part: m.part });
        }
      }
    });
    for (const st of stack) if (st.shared) moves.push({ kind: "start", x: st.x, l: st.l, part: st.part });
    for (const mv of moves) {
      const idx = sibs.indexOf(mv.x);
      const dropFrom = (str) => {
        let dropped = false;
        return str
          .split("\n")
          .filter((ln) => {
            if (!dropped && MARK.test(ln) && ln.trim() === mv.l) {
              dropped = true;
              return false;
            }
            return true;
          })
          .join("\n");
      };
      mv.x[mv.part] = dropFrom(mv.x[mv.part]);
      if (mv.kind === "start") {
        const next = sibs.slice(idx + 1).find((y) => y.topic !== SHARED && !y.preamble);
        if (!next) fail(`cannot relocate region start of shared unit ${mv.x.startLine}`);
        else next.pre = mv.l + (next.pre ? "\n" + next.pre : "");
      } else {
        const prev = sibs.slice(0, idx).reverse().find((y) => y.topic !== SHARED && !y.preamble);
        if (!prev) fail(`cannot relocate region end of shared unit ${mv.x.startLine}`);
        else prev.trail += "\n" + mv.l;
      }
    }
  }
}
moveMarkers();

const hasAwait = (u) => {
  let f = false;
  const visit = (n) => {
    if (f) return;
    if (ts.isAwaitExpression(n)) {
      f = true;
      return;
    }
    if (ts.isFunctionLike(n)) return;
    ts.forEachChild(n, visit);
  };
  visit(u.node);
  return f;
};
const closureBound = new Map();
function needsClosure(u, seen = new Set()) {
  if (closureBound.has(u)) return closureBound.get(u);
  if (seen.has(u)) return false;
  seen.add(u);
  let r = u.refs.pre.size > 0 || u.refs.params.size > 0 || (closure && u.kind === "decl" && hasAwait(u));
  if (!r) for (const d of u.refs.units) if (needsClosure(d, seen)) { r = true; break; }
  closureBound.set(u, r);
  return r;
}
const sharedUnits = leaves.filter((u) => u.topic === SHARED);
const factoryUnits = closure ? sharedUnits.filter((u) => needsClosure(u)) : [];
const pureShared = sharedUnits.filter((u) => !factoryUnits.includes(u));

function unitIndent(u) {
  const ls = text.slice(text.lastIndexOf("\n", u.node.getStart() - 1) + 1, u.node.getStart());
  return /^\s*$/.test(ls) ? ls.length : 0;
}
function dedent(s, n) {
  if (!n) return s;
  const pad = " ".repeat(n);
  return s.split("\n").map((l) => (l.startsWith(pad) ? l.slice(n) : l)).join("\n");
}
function indentText(s, n) {
  if (!n) return s;
  const pad = " ".repeat(n);
  return s.split("\n").map((l) => (l.trim() ? pad + l : l)).join("\n");
}
function unitText(u, { exportIt = false, normalize = false } = {}) {
  const n = u.node;
  const ns = n.getStart();
  const head = u.head;
  let body = text.slice(ns, n.end);
  if (exportIt) body = "export " + body;
  let out = (u.pre ? u.pre + "\n" : "") + head + body + (u.trail ?? "");
  if (normalize) {
    const ind = unitIndent(u);
    const pre = u.pre ? u.pre + "\n" : "";
    out = pre + dedent(head + body + (u.trail ?? ""), ind);
  }
  return out;
}

function regionProblems(txt, label) {
  const stack = [];
  txt.split("\n").forEach((l, i) => {
    const m = /^\s*\/\/#(region|endregion)\b(.*)$/.exec(l);
    if (!m) return;
    if (m[1] === "region") stack.push([m[2].trim(), i + 1]);
    else if (!stack.length) fail(`${label}: unmatched #endregion at ${i + 1}: ${l.trim()}`);
    else stack.pop();
  });
  for (const [name, ln] of stack) if (!cfg.allowUnmatchedRegions?.includes(name)) fail(`${label}: unmatched #region ${name} at ${ln}`);
}

function importLine(decl, usedSet) {
  const clause = decl.importClause;
  const spec = decl.moduleSpecifier.getText();
  const attrs = decl.attributes ? ` ${decl.attributes.getText()}` : "";
  if (!clause) return null;
  const pieces = [];
  if (clause.name && usedSet.has(clause)) pieces.push(clause.name.text);
  const nb = clause.namedBindings;
  if (nb && ts.isNamespaceImport(nb) && usedSet.has(nb)) pieces.push(`* as ${nb.name.text}`);
  if (nb && ts.isNamedImports(nb)) {
    const picked = nb.elements.filter((e) => usedSet.has(e)).map((e) => e.getText());
    if (picked.length) pieces.push(`{ ${picked.join(", ")} }`);
  }
  if (!pieces.length) return null;
  return `import ${clause.isTypeOnly ? "type " : ""}${pieces.join(", ")} from ${spec}${attrs};`;
}
function importLinesFor(unitList) {
  const used = new Map();
  for (const u of unitList) for (const [decl, set] of u.refs.imports) {
    const s = used.get(decl) ?? used.set(decl, new Set()).get(decl);
    for (const x of set) s.add(x);
  }
  const out = [];
  for (const decl of importDecls) {
    const s = used.get(decl);
    if (!s) continue;
    const l = importLine(decl, s);
    if (l) out.push(l);
  }
  return out;
}

const sharedUsedBy = new Map();
for (const u of leaves) {
  const t = u.replicate ? "*" : u.topic;
  if (!t || t === SHARED) continue;
  for (const d of u.refs.units) if (d.topic === SHARED) (sharedUsedBy.get(d) ?? sharedUsedBy.set(d, new Set()).get(d)).add(t);
}
for (const t of cfg.topics) {
  const parts = leaves.filter((u) => u.topic === t.folder || (u.replicate && u.parent.topics.has(t.folder)));
  for (const u of parts) for (const d of u.refs.units) if (d.topic === SHARED) (sharedUsedBy.get(d) ?? sharedUsedBy.set(d, new Set()).get(d)).add(t.folder);
}

const kebabToPascal = (s) => s.split("-").map((x) => x[0].toUpperCase() + x.slice(1)).join("");
const slugOf = (folder) => folder.replace(/^[^\p{L}\p{N}]+/u, "");
const supportName = cfg.supportName;
const helperRel = (ext) => `../${cfg.helperFolder}/🟦️${ext ?? cfg.ext}`;

function paramNamesFor(unitList) {
  const names = new Set();
  for (const u of unitList) {
    for (const p of u.refs.params) names.add(closureParams.get(p));
    for (const el of u.refs.pre) names.add(preamble.get(el).from);
  }
  return names;
}
function destructureLines(unitList, indent) {
  const byFrom = new Map();
  for (const u of unitList) for (const el of u.refs.pre) {
    const from = preamble.get(el).from;
    (byFrom.get(from) ?? byFrom.set(from, new Set()).get(from)).add(el);
  }
  const out = [];
  for (const [from, els] of byFrom) out.push(`${indent}const { ${[...els].sort((a, b) => a.pos - b.pos).map((e) => e.getText()).join(", ")} } = ${from};`);
  return out;
}

const files = new Map();
const topicMeta = [];
const docFor = (e, d, see) => `/** ${e.endsWith("\ufe0f") ? e : e + "\ufe0f"} ${d}${see ? `\n * @see {@link ${see}}` : ""} */`;

const factoryParamNames = closure ? [...closureFn.parameters.map((p) => p.name.text)].filter((n) => paramNamesFor(factoryUnits).has(n)) : [];
const factoryAsync = factoryUnits.some(hasAwait);
const factoryReturned = new Set();
for (const [d] of sharedUsedBy) if (factoryUnits.includes(d)) for (const n of d.names) factoryReturned.add(n);

for (const t of cfg.topics) {
  const folder = t.folder;
  const slug = slugOf(folder);
  const regName = closure ? `register${kebabToPascal(slug)}Tests` : null;
  const body = [];
  const flat = [];
  for (const u of top) {
    if (u.children) {
      const keep = u.children.filter((c) => c.topic === folder || (c.replicate && u.topics.has(folder)));
      if (!keep.some((c) => c.topic === folder)) continue;
      const inner = keep.map((c) => unitText(c)).join("\n\n");
      body.push((u.pre ? u.pre + "\n" : "") + u.head + u.header + "\n" + inner + "\n" + u.footer + (u.trail ?? ""));
      for (const c of keep) flat.push(c);
      flat.push(u);
    } else if (u.topic === folder) {
      body.push(unitText(u));
      flat.push(u);
    }
  }
  const importLines = importLinesFor(flat);
  const sharedPureNames = new Set();
  const sharedPureTypes = new Set();
  const sharedFactoryNames = new Set();
  for (const u of flat) for (const d of u.refs.units) {
    if (d.topic !== SHARED) continue;
    if (factoryUnits.includes(d)) for (const n of d.names) sharedFactoryNames.add(n);
    else for (const n of d.names) (ts.isTypeAliasDeclaration(d.node) || ts.isInterfaceDeclaration(d.node) ? sharedPureTypes : sharedPureNames).add(n);
  }
  const modLocal = modTop.filter((u) => u.topic === folder);
  const modImports = importLinesFor([...flat, ...modLocal]);
  importLines.length = 0;
  importLines.push(...modImports);
  for (const u of modLocal) for (const d of u.refs.units) {
    if (d.topic !== SHARED) continue;
    if (factoryUnits.includes(d)) continue;
    for (const n of d.names) (ts.isTypeAliasDeclaration(d.node) || ts.isInterfaceDeclaration(d.node) ? sharedPureTypes : sharedPureNames).add(n);
  }
  let out = docFor(t.emoji, t.doc, t.see) + "\n";
  out += importLines.join("\n") + (importLines.length ? "\n" : "");
  const helperImports = [...[...sharedPureNames].sort(), ...[...sharedPureTypes].sort().map((n) => `type ${n}`)];
  if (closure) {
    const paramsUsed = paramNamesFor(flat);
    if (sharedFactoryNames.size) for (const n of factoryParamNames) paramsUsed.add(n);
    const sigParts = closureFn.parameters.filter((p) => paramsUsed.has(p.name.text)).map((p) => `${p.name.text}: ${cfg.paramTypes[p.name.text].name}`);
    const typeImports = closureFn.parameters.filter((p) => paramsUsed.has(p.name.text)).map((p) => `type ${cfg.paramTypes[p.name.text].name}`);
    if (sharedFactoryNames.size) helperImports.push(supportName);
    for (const x of typeImports) helperImports.push(x);
    if (helperImports.length) out += `import { ${helperImports.join(", ")} } from "${helperRel()}";\n`;
    out += "\n";
    if (modLocal.length) out += modLocal.map((u) => unitText(u)).join("\n\n") + "\n\n";
    out += `export async function ${regName}(${sigParts.join(", ")}): Promise<void> {\n`;
    const dl = destructureLines(flat, "  ");
    out += dl.join("\n") + (dl.length ? "\n" : "");
    if (sharedFactoryNames.size) out += `  const { ${[...sharedFactoryNames].sort().join(", ")} } = ${factoryAsync ? "await " : ""}${supportName}(${factoryParamNames.join(", ")});\n`;
    out += body.join("\n\n") + "\n}\n";
    t.regName = regName;
    t.params = [...paramsUsed].filter((n) => closureParams.size && closureFn.parameters.some((p) => p.name.text === n));
    t.params = closureFn.parameters.map((p) => p.name.text).filter((n) => paramsUsed.has(n));
  } else {
    out += (helperImports.length ? `import { ${helperImports.join(", ")} } from "${helperRel()}";\n` : `import "${helperRel()}";\n`);
    out += "\n" + body.join("\n\n") + "\n";
  }
  files.set(folder, out);
  regionProblems(out, folder);
  topicMeta.push({ folder, lines: out.split("\n").length, units: flat.map((u) => u.startLine) });
}

{
  let out = docFor(cfg.helper.emoji, cfg.helper.doc, cfg.helper.see) + "\n";
  const paramPseudo = closure ? closureFn.parameters.map((p) => ({ refs: refsOfNode(p.type, null) })) : [];
  const importLines = importLinesFor([...sharedUnits, ...paramPseudo]);
  const hasTypes = closure;
  const typeDecls = [];
  if (closure) {
    for (const p of closureFn.parameters) {
      const typeText = p.type.getText();
      typeDecls.push(`export type ${cfg.paramTypes[p.name.text].name} = ${typeText};`);
    }
  }
  out += importLines.join("\n") + (importLines.length ? "\n" : "");
  out += "\n";
  if (closure) out += typeDecls.join("\n") + "\n\n";
  const pureText = pureShared.map((u) => unitText(u, { exportIt: sharedUsedBy.has(u) && u.kind === "decl", normalize: true })).join("\n\n");
  out += pureText + (pureText ? "\n" : "");
  if (closure && factoryUnits.length) {
    const ps = closureFn.parameters.filter((p) => factoryParamNames.includes(p.name.text)).map((p) => `${p.name.text}: ${cfg.paramTypes[p.name.text].name}`);
    out += `\nexport ${factoryAsync ? "async " : ""}function ${supportName}(${ps.join(", ")}) {\n`;
    const dl = destructureLines(factoryUnits, "  ");
    out += dl.join("\n") + (dl.length ? "\n" : "");
    out += factoryUnits.map((u) => indentText(unitText(u, { normalize: true }), 2)).join("\n\n") + "\n";
    out += `  return { ${[...factoryReturned].sort().join(", ")} };\n}\n`;
  }
  files.set(cfg.helperFolder, out);
  regionProblems(out, cfg.helperFolder);
}

if (closure && cfg.registrar) {
  const r = cfg.registrar;
  const sigParts = closureFn.parameters.map((p) => `${p.name.text}: ${cfg.paramTypes[p.name.text].name}`);
  let out = docFor(r.emoji, r.doc, r.see) + "\n";
  out += `import { ${closureFn.parameters.map((p) => `type ${cfg.paramTypes[p.name.text].name}`).join(", ")} } from "${helperRel()}";\n`;
  for (const t of cfg.topics) out += `import { ${t.regName} } from "../${t.folder}/🟦️${cfg.ext}";\n`;
  out += `\nexport async function ${r.name}(${sigParts.join(", ")}): Promise<void> {\n`;
  for (const t of cfg.topics) out += `  await ${t.regName}(${t.params.join(", ")});\n`;
  out += "}\n";
  files.set(r.folder, out);
}

const unused = allUnits.filter((u) => u.unused).map((u) => `${u.startLine}:${u.names.join(",")}`);
if (errors.length) console.log("ERRORS:\n" + errors.join("\n"));
for (const m of topicMeta) console.log(`${m.folder}: ${m.lines} lines`);
console.log(`helper: ${files.get(cfg.helperFolder).split("\n").length} lines`);
console.log(`shared pure: ${pureShared.map((u) => `${u.startLine}:${u.names.join(",") || u.kind}`).join(" ")}`);
console.log(`shared factory: ${factoryUnits.map((u) => `${u.startLine}:${u.names.join(",") || u.kind}`).join(" ")}`);
console.log(`unused decls: ${unused.join(" ")}`);
if (WRITE && (!errors.length || process.argv.includes("--force"))) {
  for (const [folder, content] of files) {
    const dir = path.join(cfg.testsDir, folder);
    fs.mkdirSync(dir, { recursive: true });
    fs.writeFileSync(path.join(dir, `🟦️${cfg.ext}`), content);
  }
  console.log("written");
}
