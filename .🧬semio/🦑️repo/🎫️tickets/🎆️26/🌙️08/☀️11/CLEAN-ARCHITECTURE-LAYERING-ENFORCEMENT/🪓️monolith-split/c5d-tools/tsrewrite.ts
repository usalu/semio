// 🔁 c5d importer rewrite: every static import and `const { a } = await import(...)` of the split module is re-pointed, name by name, to the leaf that now declares the name (explicit named imports, no barrel).
// usage: bun tsrewrite.ts <tag> [--apply]    (needs the stage map written by `tssplit.ts stage <tag>`)
import {execFileSync} from "node:child_process";
import {readFileSync, writeFileSync, existsSync} from "node:fs";
import {dirname, relative, resolve} from "node:path";
const ts = (await import("/Users/ueli/.bun/install/cache/typescript@5.9.3@@@1/lib/typescript.js")).default as typeof import("typescript");
const REPO = "/Users/ueli/Documents/semio";
const SCR = process.env.C5D_SCR ?? "/Users/ueli/.cache/c5d";
const tag = process.argv[2]!;
const apply = process.argv.includes("--apply");
const {old, map} = JSON.parse(readFileSync(`${SCR}/stage-ts/${tag}.map.json`, "utf8")) as {old: string; map: Record<string, string>};
const list = (args: string[]) => execFileSync("git", ["-c", "core.fsmonitor=false", "-c", "core.quotepath=false", ...args], {cwd: REPO, encoding: "utf8", maxBuffer: 1 << 28}).split("\n").filter(Boolean);
const files = [...new Set([...list(["ls-files", "--", "*.ts", "*.tsx", "*.mts", "*.mjs"]), ...list(["ls-files", "-o", "--exclude-standard", "--", "*.ts", "*.tsx", "*.mts", "*.mjs"])])].filter(f => !f.startsWith(".🧬semio/") && !f.includes("node_modules/"));
const relSpec = (fromDir: string, target: string): string => { const r = relative(fromDir, target); return r.startsWith(".") ? r : "./" + r; };
let changed = 0, manual = 0;
for (const rel of files) {
  const abs = resolve(REPO, rel);
  if (!existsSync(abs) || abs === old) continue;
  const text = readFileSync(abs, "utf8");
  if (!ts.preProcessFile(text, true, true).importedFiles.some(f => f.fileName.startsWith(".") && resolve(dirname(abs), f.fileName) === old)) continue;
  const sf = ts.createSourceFile(abs, text, ts.ScriptTarget.Latest, true);
  const dir = dirname(abs);
  const edits: {start: number; end: number; text: string}[] = [];
  const indentOf = (pos: number) => { const ls = text.lastIndexOf("\n", pos - 1) + 1; return text.slice(ls, pos).match(/^\s*/)![0]; };
  const group = (names: {name: string; type: boolean; alias?: string}[]): Map<string, typeof names> => {
    const g = new Map<string, typeof names>();
    for (const n of names) {
      const target = map[n.name];
      if (!target) throw new Error(`${rel}: no new home for ${n.name}`);
      if (!g.has(target)) g.set(target, []);
      g.get(target)!.push(n);
    }
    return g;
  };
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && resolve(dir, node.moduleSpecifier.text) === old) {
      const c = node.importClause;
      if (!c || c.name || !c.namedBindings || !ts.isNamedImports(c.namedBindings)) { console.log("MANUAL", rel, node.getText().slice(0, 100)); manual++; return; }
      const names = c.namedBindings.elements.map(e => ({name: (e.propertyName ?? e.name).text, alias: e.propertyName ? e.name.text : undefined, type: e.isTypeOnly}));
      const multi = /\n/.test(c.namedBindings.getText());
      const g = group(names);
      if ([...g.keys()].every(k => k === old)) return;
      const ind = indentOf(node.getStart());
      const stmts = [...g.entries()].map(([target, ns]) => {
        const all = ns.every(n => n.type) || c.isTypeOnly;
        const items = ns.map(n => `${n.type && !all ? "type " : ""}${n.alias ? `${n.name} as ${n.alias}` : n.name}`);
        const body = multi ? `{\n${items.map(i => `${ind}  ${i},`).join("\n")}\n${ind}}` : `{ ${items.join(", ")} }`;
        return `import ${all ? "type " : ""}${body} from "${relSpec(dir, target)}";`;
      });
      edits.push({start: node.getStart(), end: node.getEnd(), text: stmts.join("\n" + ind)});
      return;
    }
    if (ts.isVariableStatement(node)) {
      if (node.declarationList.declarations.length > 1) {
        if (node.getText().includes("await import(")) { console.log("MANUAL(multi-declarator)", rel + ":" + (sf.getLineAndCharacterOfPosition(node.getStart()).line + 1)); manual++; }
        ts.forEachChild(node, visit);
        return;
      }
      const d = node.declarationList.declarations[0]!;
      const init = d.initializer;
      const call = init && ts.isAwaitExpression(init) && ts.isCallExpression(init.expression) ? init.expression : undefined;
      if (call && call.expression.kind === ts.SyntaxKind.ImportKeyword && call.arguments[0] && ts.isStringLiteral(call.arguments[0]!) && resolve(dir, (call.arguments[0] as ts.StringLiteral).text) === old) {
        if (!ts.isObjectBindingPattern(d.name)) { console.log("MANUAL", rel, node.getText().slice(0, 120)); manual++; return; }
        const names = d.name.elements.map(e => ({name: (e.propertyName ?? e.name as ts.Identifier).getText(), alias: e.propertyName ? (e.name as ts.Identifier).getText() : undefined, type: false}));
        const g = group(names);
        if ([...g.keys()].every(k => k === old)) return;
        const ind = indentOf(node.getStart());
        const kw = node.declarationList.flags & ts.NodeFlags.Const ? "const" : "let";
        const stmts = [...g.entries()].map(([target, ns]) => `${kw} { ${ns.map(n => (n.alias ? `${n.name}: ${n.alias}` : n.name)).join(", ")} } = await import("${relSpec(dir, target)}");`);
        edits.push({start: node.getStart(), end: node.getEnd(), text: stmts.join("\n" + ind)});
        return;
      }
    }
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0]!) && resolve(dir, (node.arguments[0] as ts.StringLiteral).text) === old) {
      const aw = node.parent, par = aw?.parent, pa = par?.parent;
      if (aw && ts.isAwaitExpression(aw) && par && ts.isParenthesizedExpression(par) && pa && ts.isPropertyAccessExpression(pa) && pa.expression === par) {
        const target = map[pa.name.text];
        if (!target) throw new Error(`${rel}: no new home for ${pa.name.text}`);
        const lit = node.arguments[0] as ts.StringLiteral;
        edits.push({start: lit.getStart(), end: lit.getEnd(), text: JSON.stringify(relSpec(dir, target))});
        return;
      }
    }
    if ((ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0]!) && resolve(dir, (node.arguments[0] as ts.StringLiteral).text) === old)) {
      const parent = node.parent?.parent;
      if (!(parent && ts.isVariableDeclaration(parent) && ts.isObjectBindingPattern(parent.name))) { console.log("MANUAL(dynamic)", rel + ":" + (sf.getLineAndCharacterOfPosition(node.getStart()).line + 1), node.parent?.parent?.getText().slice(0, 100)); manual++; }
    }
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && resolve(dir, node.moduleSpecifier.text) === old) { console.log("MANUAL", rel); manual++; }
    ts.forEachChild(node, visit);
  };
  visit(sf);
  if (!edits.length) continue;
  let out = text;
  for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  console.log(`${rel}: ${edits.length} edit(s)`);
  if (apply) writeFileSync(abs, out);
  changed++;
}
console.log("files", changed, "manual", manual, apply ? "APPLIED" : "dry run");
