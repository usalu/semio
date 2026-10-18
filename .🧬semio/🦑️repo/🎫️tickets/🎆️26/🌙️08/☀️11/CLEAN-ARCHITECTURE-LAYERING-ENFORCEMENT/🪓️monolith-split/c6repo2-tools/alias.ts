import { parseFile, ts } from "./lib.ts";
import { assign, exportTable, type Plan } from "./split.ts";
import { rewriteFile, lookupFrom, type Lookup } from "./rewrite.ts";
import { readFileSync, copyFileSync, mkdirSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const [planFile, backupParent, rootFile, sitesFile, mode] = process.argv.slice(2);
const plan: Plan = JSON.parse(readFileSync(planFile, "utf8"));
const realParent = plan.file;
const a = assign({ ...plan, file: backupParent, dir: plan.dir ?? dirname(realParent) });
const table = exportTable(a);
const discoveryLookup = lookupFrom(table, realParent, a.chunkFiles);
const root = parseFile(rootFile);
const own = new Set<string>();
for (const u of root.units) for (const s of u.stmts) if (root.stmtInfo[s].exported) root.stmtInfo[s].names.forEach((n) => own.add(n));
for (const ex of root.reexports) {
  const spec = (ex.moduleSpecifier as import("typescript").StringLiteral).text;
  if (ex.exportClause && ts.isNamedExports(ex.exportClause)) ex.exportClause.elements.forEach((el) => own.add(el.name.text));
}
for (const ex of root.localExportLists) if (ex.exportClause && ts.isNamedExports(ex.exportClause)) ex.exportClause.elements.forEach((el) => own.add(el.name.text));
console.log(`root own exports: ${own.size}`);
const rootLookup: Lookup = (name) => {
  if (own.has(name)) return { kind: "file", abs: rootFile, imported: name };
  return discoveryLookup(name);
};
const sites: any[] = JSON.parse(readFileSync(sitesFile, "utf8"));
const files = [...new Set(sites.map((s) => s.file as string))].filter((f) => f !== rootFile);
const parents = new Map([[rootFile, rootLookup]]);
const write = mode === "write";
const manual: string[] = [];
let n = 0;
for (const f of files) {
  if (write) { const d = join("/Users/ueli/.cache/c6repo2/backup/root", relative("/Users/ueli/Documents/semio", f)); mkdirSync(dirname(d), { recursive: true }); copyFileSync(f, d); }
  const r = rewriteFile(f, parents, write);
  if (r.edits) n++;
  manual.push(...r.manual);
  if (!write && r.edits) console.log(`${f}: ${r.edits} edits`);
}
console.log(`edited ${n}/${files.length}; manual ${manual.length}`);
console.log(manual.join("\n"));
