import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";

const file = process.argv[2]!;
const out = process.argv[3]!;
const text = readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const line = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line + 1;

type Decl = { id: number; names: string[]; kind: string; start: number; end: number; region: string; refs: string[]; hits: string[] };
const decls: Decl[] = [];
const nameOwner = new Map<string, number>();

const namesOf = (s: ts.Statement): string[] => {
  const names: string[] = [];
  if (ts.isVariableStatement(s)) {
    for (const d of s.declarationList.declarations) {
      if (ts.isIdentifier(d.name)) names.push(d.name.text);
      else if (ts.isObjectBindingPattern(d.name) || ts.isArrayBindingPattern(d.name)) for (const e of d.name.elements) if (ts.isBindingElement(e) && ts.isIdentifier(e.name)) names.push(e.name.text);
    }
  } else if ((ts.isFunctionDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isClassDeclaration(s) || ts.isInterfaceDeclaration(s)) && s.name) names.push(s.name.text);
  return names;
};

const regionAt = (pos: number): string => {
  let region = "";
  const stack: string[] = [];
  for (const m of text.slice(0, pos).matchAll(/^\s*\/\/#(end)?region\s+(.*)$/gmu)) {
    if (m[1]) stack.pop();
    else stack.push(m[2]!.trim());
  }
  region = stack.join("/");
  return region;
};

const ids = (n: ts.Node, set: Set<string>) => {
  if (ts.isIdentifier(n)) {
    const p = n.parent;
    const prop = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || (ts.isPropertySignature(p) && p.name === n);
    if (!prop) set.add(n.text);
  }
  n.forEachChild((c) => ids(c, set));
};

const HITS = /puzzle|2d-overview|data-board|dumpBoard2d|board2d|create-node|drag-selection|rotate-selection|scale-selection|lock-node|duplicate|vitals|:6012|:6112|PLUGIN_VARIANT/giu;

sf.statements.forEach((s, i) => {
  const names = namesOf(s);
  const refs = new Set<string>();
  ids(s, refs);
  const textOf = text.slice(s.getStart(sf), s.end);
  const hits = [...new Set([...textOf.matchAll(HITS)].map((m) => m[0]!.toLowerCase()))];
  decls.push({ id: i, names, kind: ts.SyntaxKind[s.kind], start: line(s.getStart(sf)), end: line(s.end), region: regionAt(s.getStart(sf)), refs: [...refs], hits });
  for (const n of names) nameOwner.set(n, i);
});

const rows = decls.map((d) => ({ ...d, uses: [...new Set(d.refs.filter((r) => nameOwner.has(r) && nameOwner.get(r) !== d.id).map((r) => r))] }));
writeFileSync(out, rows.map((d) => `${d.id}\t${d.start}-${d.end}\t${d.kind}\t${d.region}\t${d.names.join(",")}\tHITS:${d.hits.join("|")}\tUSES:${d.uses.join(",")}`).join("\n") + "\n");
console.log(rows.length);
