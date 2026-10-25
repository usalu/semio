import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const [pkgsPath, scope, out] = process.argv.slice(2) as [string, string, string];
const packages: any[] = JSON.parse(readFileSync(pkgsPath, "utf8"));
const entryFiles = (value: unknown): string[] => typeof value === "string" ? [value] : Array.isArray(value) ? value.flatMap(entryFiles) : value && typeof value === "object" ? Object.values(value).flatMap(entryFiles) : [];
const bare = new Map<string, Map<string, string>>();
for (const pkg of packages) {
  const m = new Map<string, string>();
  try { const manifest = JSON.parse(readFileSync(join(root, pkg.owner, "package.json"), "utf8")); for (const [key, value] of Object.entries(manifest.exports ?? {})) { const t = entryFiles(value).find((x) => x.startsWith("./")); if (t && !key.includes("*")) m.set(key, normalize(join(pkg.owner, t))); } } catch {}
  bare.set(pkg.name, m);
}
const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter((f) => /\.(?:[cm]?[jt]sx?)$/u.test(f) && !f.includes("/node_modules/") && !f.startsWith(".🧬semio/") && !f.includes("/🗑️generated/") && existsSync(join(root, f)));
const fileSet = new Set(files);
const ignore = new Set<string>(process.env.IGNORE ? (JSON.parse(readFileSync(process.env.IGNORE, "utf8")) as [string, string][]).map(([a, b]) => a + "\0" + b) : []);
const graph = new Map<string, Set<string>>();
const kindOf = new Map<string, Set<string>>();
for (const from of files) {
  const text = readFileSync(join(root, from), "utf8");
  const sf = ts.createSourceFile(from, text, ts.ScriptTarget.Latest, true, from.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const specs: string[] = [];
  const kinds = new Map<string, Set<string>>();
  const mark = (spec: string, kind: string) => { specs.push(spec); if (!kinds.has(spec)) kinds.set(spec, new Set()); kinds.get(spec)!.add(kind); };
  const visit = (node: ts.Node) => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) {
      let typeOnly = false;
      if (ts.isImportDeclaration(node)) { const c = node.importClause; typeOnly = !!c && (c.isTypeOnly || (!c.name && !!c.namedBindings && ts.isNamedImports(c.namedBindings) && c.namedBindings.elements.length > 0 && c.namedBindings.elements.every((e) => e.isTypeOnly))); }
      else typeOnly = node.isTypeOnly;
      mark(node.moduleSpecifier.text, typeOnly ? "type" : "static");
    } else if (ts.isCallExpression(node) && node.arguments[0] && ts.isStringLiteralLike(node.arguments[0]) && (node.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(node.expression) && node.expression.text === "require"))) mark(node.arguments[0].text, node.expression.kind === ts.SyntaxKind.ImportKeyword ? "dynamic" : "require");
    else if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument) && ts.isStringLiteral(node.argument.literal)) mark(node.argument.literal.text, "type");
    ts.forEachChild(node, visit);
  };
  visit(sf);
  const set = new Set<string>();
  for (const spec of specs) {
    let to: string | undefined;
    if (spec.startsWith(".")) to = normalize(join(dirname(from), spec.split("?")[0]!));
    else if (spec.startsWith("@semio-tech/")) { const parts = spec.split("/"); const name = parts.slice(0, 2).join("/"); const key = parts.length > 2 ? "./" + parts.slice(2).join("/") : "."; to = bare.get(name)?.get(key); }
    if (to && fileSet.has(to) && to !== from && !ignore.has(from + "\0" + to)) { set.add(to); const key = from + "\0" + to; const row = kindOf.get(key) ?? new Set<string>(); kinds.get(spec)!.forEach((k) => row.add(k)); kindOf.set(key, row); }
  }
  graph.set(from, set);
}
let index = 0; const idx = new Map<string, number>(), low = new Map<string, number>(), on = new Set<string>(), stack: string[] = [], sccs: string[][] = [];
const strong = (v: string) => {
  const work: [string, Iterator<string>][] = [[v, graph.get(v)![Symbol.iterator]()]]; idx.set(v, index); low.set(v, index); index++; stack.push(v); on.add(v);
  while (work.length) {
    const [node, it] = work[work.length - 1]!; const next = it.next();
    if (!next.done) { const w = next.value; if (!idx.has(w)) { idx.set(w, index); low.set(w, index); index++; stack.push(w); on.add(w); work.push([w, graph.get(w)![Symbol.iterator]()]); } else if (on.has(w)) low.set(node, Math.min(low.get(node)!, idx.get(w)!)); }
    else { work.pop(); if (work.length) { const parent = work[work.length - 1]![0]; low.set(parent, Math.min(low.get(parent)!, low.get(node)!)); } if (low.get(node) === idx.get(node)) { const comp: string[] = []; let w: string; do { w = stack.pop()!; on.delete(w); comp.push(w); } while (w !== node); if (comp.length > 1) sccs.push(comp); } }
  }
};
for (const v of graph.keys()) if (!idx.has(v)) strong(v);
const rows = sccs.map((comp) => { const set = new Set(comp); const edges: [string, string, string][] = []; for (const a of comp) for (const b of graph.get(a)!) if (set.has(b)) edges.push([a, b, [...kindOf.get(a + "\0" + b)!].sort().join("+")]); return { size: comp.length, edges, files: comp.sort() }; }).sort((a, b) => b.size - a.size);
const mine = rows.filter((r) => r.files.some((f) => f.startsWith(scope)));
writeFileSync(out, JSON.stringify(mine, null, 1));
console.log("files", files.length, "sccs", rows.length, "in scope", mine.length, mine.map((r) => r.size + "/" + r.edges.length).join(" "));
