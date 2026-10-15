/** 🧮 Transitive declaration closure inside one module for a set of seed symbols.
 * Usage: bun 📜️script.ts <tsconfig> <module.tsx> <seeds.txt> [--ignore-types]
 * Prints the top-level statements (line ranges) required, plus the imports they consume.
 */
import ts from "typescript";
import { dirname, resolve } from "node:path";
import { readFileSync } from "node:fs";

const [tsconfigPath, moduleFile, seedsFile] = process.argv.slice(2) as [string, string, string];
const cfg = ts.readConfigFile(tsconfigPath, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, dirname(resolve(tsconfigPath)));
const program = ts.createProgram(parsed.fileNames, parsed.options);
const checker = program.getTypeChecker();
const sf = program.getSourceFile(resolve(moduleFile))!;
const seeds = new Set(readFileSync(seedsFile, "utf8").split("\n").map((s) => s.trim()).filter(Boolean));
const topOf = new Map<ts.Symbol, ts.Statement>();
const nameOfStatement = (st: ts.Statement): string[] => {
  const out: string[] = [];
  if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) if (ts.isIdentifier(d.name)) out.push(d.name.text);
  if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) out.push(st.name.text);
  return out;
};
const byName = new Map<string, ts.Statement>();
for (const st of sf.statements) {
  for (const n of nameOfStatement(st)) byName.set(n, st);
}
const importSymbols = new Map<ts.Symbol, ts.ImportDeclaration>();
for (const st of sf.statements) {
  if (!ts.isImportDeclaration(st) || !st.importClause) continue;
  const c = st.importClause;
  const names: ts.Identifier[] = [];
  if (c.name) names.push(c.name);
  if (c.namedBindings) {
    if (ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) names.push(e.name);
    else names.push(c.namedBindings.name);
  }
  for (const n of names) {
    const s = checker.getSymbolAtLocation(n);
    if (s) importSymbols.set(s, st);
  }
}
const needed = new Set<ts.Statement>();
const importsOf = new Map<ts.Statement, Set<string>>();
const depsOf = new Map<ts.Statement, Set<ts.Statement>>();
const neededImports = new Map<ts.ImportDeclaration, Set<string>>();
const queue: ts.Statement[] = [];
const add = (st: ts.Statement) => { if (!needed.has(st)) { needed.add(st); queue.push(st); } };
for (const seed of seeds) {
  const st = byName.get(seed);
  if (st) add(st);
  else console.error(`seed not declared at top level: ${seed}`);
}
let current: ts.Statement | undefined;
const visit = (node: ts.Node) => {
  if (ts.isIdentifier(node)) {
    const s = checker.getSymbolAtLocation(node) ?? checker.getShorthandAssignmentValueSymbol(node.parent);
    if (s) {
      const imp = importSymbols.get(s);
      if (imp && current) {
        const mods = importsOf.get(current) ?? new Set<string>();
        mods.add((imp.moduleSpecifier as ts.StringLiteral).text);
        importsOf.set(current, mods);
      }
      if (imp) {
        const set = neededImports.get(imp) ?? new Set<string>();
        set.add(node.text);
        neededImports.set(imp, set);
      }
      for (const d of s.declarations ?? []) {
        if (d.getSourceFile() !== sf) continue;
        let p: ts.Node = d;
        while (p.parent && p.parent !== sf) p = p.parent;
        if (p.parent === sf && !ts.isImportDeclaration(p)) {
          add(p as ts.Statement);
          if (current && current !== p) {
            const d2 = depsOf.get(current) ?? new Set<ts.Statement>();
            d2.add(p as ts.Statement);
            depsOf.set(current, d2);
          }
        }
      }
    }
  }
  ts.forEachChild(node, visit);
};
while (queue.length) { current = queue.shift()!; visit(current); }
const lc = (n: ts.Node) => sf.getLineAndCharacterOfPosition(n.getStart()).line + 1;
const le = (n: ts.Node) => sf.getLineAndCharacterOfPosition(n.getEnd()).line + 1;
const sorted = [...needed].sort((a, b) => a.getStart() - b.getStart());
let total = 0;
for (const st of sorted) {
  total += le(st) - lc(st) + 1;
  console.log(`STMT ${lc(st)}-${le(st)} ${nameOfStatement(st).join(",") || ts.SyntaxKind[st.kind]}`);
}
console.log(`TOTAL statements=${sorted.length} lines=${total}`);
for (const [imp, names] of neededImports) console.log(`IMPORT ${lc(imp)} ${(imp.moduleSpecifier as ts.StringLiteral).text} :: ${[...names].join(",")}`);

if (process.argv.includes("--graph")) {
  const lines: string[] = [];
  for (const st of sorted) {
    const name = nameOfStatement(st).join(",") || ts.SyntaxKind[st.kind];
    const deps = [...(depsOf.get(st) ?? [])].map((d) => nameOfStatement(d).join(",")).join(" ");
    const mods = [...(importsOf.get(st) ?? [])].join(" ");
    lines.push(`NODE ${lc(st)}-${le(st)} ${name} | deps: ${deps} | imports: ${mods}`);
  }
  console.log(lines.join("\n"));
}
