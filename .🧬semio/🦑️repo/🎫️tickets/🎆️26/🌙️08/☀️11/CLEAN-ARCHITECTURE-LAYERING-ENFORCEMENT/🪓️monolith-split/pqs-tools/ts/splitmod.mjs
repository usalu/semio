import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const cfg = (await import(pathToFileURL(path.resolve(process.argv[2])).href)).default;
const WRITE = process.argv.includes("--write");
const text = fs.readFileSync(cfg.source, "utf8");
const sf = ts.createSourceFile(cfg.source, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const host = ts.createCompilerHost({});
const orig = host.getSourceFile.bind(host);
host.getSourceFile = (n, ...r) => (path.resolve(n) === path.resolve(cfg.source) ? sf : orig(n, ...r));
const program = ts.createProgram([cfg.source], { noResolve: true, noLib: true, target: ts.ScriptTarget.ESNext, skipLibCheck: true, types: [], allowImportingTsExtensions: true }, host);
const checker = program.getTypeChecker();
const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;
const lines = text.split("\n");
const errors = [];
const fail = (m) => errors.push(m);

const ownerOfLine = (line) => {
  for (const c of cfg.children) for (const [a, b] of c.ranges) if (line >= a && line <= b) return c.folder;
  return "parent";
};
const importDecls = sf.statements.filter(ts.isImportDeclaration);
const body = sf.statements.filter((s) => !ts.isImportDeclaration(s));
const ownerOfStmt = new Map();
for (const s of body) {
  const o = ownerOfLine(lineOf(s.getStart()));
  ownerOfStmt.set(s, o);
  const e = ownerOfLine(lineOf(s.end));
  if (e !== o) fail(`statement ${lineOf(s.getStart())}-${lineOf(s.end)} straddles ${o}/${e}`);
}
const topStmtOf = (d) => {
  let n = d;
  while (n && n.parent && n.parent !== sf) n = n.parent;
  return n && n.parent === sf ? n : null;
};
const declNames = (s) => {
  if (ts.isVariableStatement(s)) return s.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : []));
  if (s.name && ts.isIdentifier(s.name)) return [s.name.text];
  return [];
};
const isTypeDecl = (s) => ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s);

const files = new Map();
for (const f of ["parent", ...cfg.children.map((c) => c.folder)]) files.set(f, { imports: new Map(), cross: new Map(), stmts: [] });
const exportsNeeded = new Set();
for (const s of body) {
  const me = ownerOfStmt.get(s);
  files.get(me).stmts.push(s);
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      let skip = false;
      if (p) {
        if (ts.isPropertyAccessExpression(p) && p.name === n) skip = true;
        else if (ts.isQualifiedName(p) && p.right === n) skip = true;
        else if ((ts.isPropertyAssignment(p) || ts.isPropertySignature(p) || ts.isMethodDeclaration(p) || ts.isMethodSignature(p) || ts.isPropertyDeclaration(p) || ts.isGetAccessorDeclaration(p) || ts.isSetAccessorDeclaration(p) || ts.isEnumMember(p)) && p.name === n) skip = true;
        else if (ts.isBindingElement(p) && p.propertyName === n) skip = true;
        else if (ts.isLabeledStatement(p) || ts.isBreakOrContinueStatement(p)) skip = true;
      }
      if (!skip) {
        const sym = p && ts.isShorthandPropertyAssignment(p) && p.name === n ? checker.getShorthandAssignmentValueSymbol(p) : checker.getSymbolAtLocation(n);
        for (const d of sym?.declarations ?? []) {
          let imp = d;
          while (imp && !ts.isImportDeclaration(imp)) imp = imp.parent;
          if (imp) {
            const m = files.get(me).imports;
            (m.get(imp) ?? m.set(imp, new Set()).get(imp)).add(d);
            continue;
          }
          const top = topStmtOf(d);
          if (!top || !ownerOfStmt.has(top)) continue;
          const other = ownerOfStmt.get(top);
          if (other === me) continue;
          if (other === "parent") fail(`${me} references parent-owned ${n.text} at line ${lineOf(n.getStart())}`);
          const m = files.get(me).cross;
          const set = m.get(other) ?? m.set(other, new Map()).get(other);
          set.set(n.text, isTypeDecl(top));
          exportsNeeded.add(top);
        }
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(s);
}

const spec = (decl, rel) => {
  let v = decl.moduleSpecifier.text;
  if (rel && v.startsWith("../")) v = "../" + v;
  else if (rel && v.startsWith("./")) v = "." + v;
  return v;
};
function fmt(names, from, typeOnly) {
  const one = `import ${typeOnly ? "type " : ""}{ ${names.join(", ")} } from "${from}";`;
  if (one.length <= 150) return one;
  return `import ${typeOnly ? "type " : ""}{\n${names.map((n) => "  " + n + ",").join("\n")}\n} from "${from}";`;
}
function importBlock(file, isChild, baseFolderOf) {
  const out = [];
  const f = files.get(file);
  for (const decl of importDecls) {
    const set = f.imports.get(decl);
    if (!set) continue;
    const c = decl.importClause;
    const pieces = [];
    if (c?.name && set.has(c)) pieces.push(c.name.text);
    const nb = c?.namedBindings;
    if (nb && ts.isNamespaceImport(nb) && set.has(nb)) pieces.push(`* as ${nb.name.text}`);
    if (nb && ts.isNamedImports(nb)) {
      const picked = nb.elements.filter((e) => set.has(e)).map((e) => e.getText());
      if (picked.length) {
        if (pieces.length) out.push(`import ${c.isTypeOnly ? "type " : ""}${pieces.join(", ")}, { ${picked.join(", ")} } from "${spec(decl, isChild)}";`);
        else out.push(fmt(picked, spec(decl, isChild), c.isTypeOnly));
        pieces.length = 0;
        continue;
      }
    }
    if (pieces.length) out.push(`import ${c.isTypeOnly ? "type " : ""}${pieces.join(", ")} from "${spec(decl, isChild)}";`);
  }
  for (const [other, names] of f.cross) {
    const list = [...names].sort((a, b) => a[0].localeCompare(b[0])).map(([n, t]) => (t ? `type ${n}` : n));
    out.push(fmt(list, `${isChild ? "../" : "./"}${other}/🟦️${cfg.ext}`, false));
  }
  return out;
}
function bodyText(c) {
  const edits = [];
  for (const s of exportsNeeded) if (ownerOfStmt.get(s) === c.folder && !(ts.getCombinedModifierFlags?.(s) & ts.ModifierFlags.Export) && !s.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)) edits.push(s.getStart());
  const parts = [];
  for (const [a, b] of c.ranges) {
    const start = lines.slice(0, a - 1).join("\n").length + (a > 1 ? 1 : 0);
    const end = lines.slice(0, b).join("\n").length;
    let seg = text.slice(start, end);
    for (const pos of edits.filter((p) => p >= start && p < end).sort((x, y) => y - x)) seg = seg.slice(0, pos - start) + "export " + seg.slice(pos - start);
    parts.push(seg);
  }
  return parts.join("\n\n");
}
const outFiles = new Map();
for (const c of cfg.children) {
  const imps = importBlock(c.folder, true);
  const doc = `/** ${c.emoji} ${c.doc} */`;
  outFiles.set(`${c.folder}/🟦️${cfg.ext}`, doc + "\n\n" + imps.join("\n") + (imps.length ? "\n\n" : "") + bodyText(c) + "\n");
}
{
  const [ia, ib] = cfg.importsRange;
  const removed = new Set();
  for (const c of cfg.children) for (const [a, b] of c.ranges) for (let l = a; l <= b + 1 && l <= lines.length; l++) if (l <= b || lines[l - 1].trim() === "") removed.add(l);
  const out = [];
  for (let l = 1; l <= lines.length; l++) {
    if (l === ia) {
      const imps = importBlock("parent", false);
      out.push(...imps);
    }
    if (l >= ia && l <= ib) continue;
    if (removed.has(l)) continue;
    out.push(lines[l - 1]);
  }
  let parentText = out.join("\n");
  const edits = [];
  outFiles.set(`🟦️${cfg.ext}`, parentText);
}
if (errors.length) console.log("ERRORS:\n" + errors.join("\n"));
for (const [f, t] of outFiles) console.log(`${f}: ${t.split("\n").length} lines`);
const owners = {};
for (const s of body) for (const n of declNames(s)) owners[n] = ownerOfStmt.get(s);
fs.writeFileSync(path.join(cfg.outDir, "owners.json"), JSON.stringify(owners, null, 1));
if (WRITE && !errors.length) {
  for (const [f, t] of outFiles) {
    const p = path.join(cfg.outDir, f);
    fs.mkdirSync(path.dirname(p), { recursive: true });
    fs.writeFileSync(p, t);
  }
  console.log("written");
}
