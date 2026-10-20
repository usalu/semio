/**
 * ✅ Validates the staged batch against the live tree on a virtual overlay (nothing is written).
 *
 *   bun validate.ts   (prints the verdict, writes generated/validation.json)
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import ts from "typescript";
import { GENERATED, TICKET, posix } from "./lib.ts";
import { abs, entriesOf, model, originsOf, resolveRel } from "./model.ts";
import { loadPolicy, otherViolations } from "./rules.ts";
import { reaches } from "./cycles.ts";
import { scan } from "./scan.ts";

const batch = JSON.parse(readFileSync(resolve(TICKET, "exports-stage-tools/batch.json"), "utf8"));
const DEFAULT = new Set(["export", "move", "removal", "third-party", "types", "requested", "tst6"]);
const entries = batch.entries.filter((e: any) => e.apply && DEFAULT.has(e.tier));
const selected = new Set(entries.map((e: any) => e.id));
const glue = batch.glue.filter((g: any) => g.entry === null || selected.has(g.entry));
const rewrites = batch.rewrites.filter((r: any) => selected.has(r.entry));
const problems: Record<string, string[]> = {};
const warn = (k: string, m: string): void => { (problems[k] ??= []).push(m); };

const virtual = new Map<string, string>();
for (const g of glue) {
  if (g.kind === "create") virtual.set(g.path, g.content);
  if (g.kind === "extend") virtual.set(g.path, readFileSync(abs(g.path), "utf8").replace(/\n*$/u, "\n") + g.lines.join("\n") + "\n");
}
const deleted = new Set(glue.filter((g: any) => g.kind === "delete").map((g: any) => g.path));
const exists = (p: string): boolean => (virtual.has(p) || existsSync(abs(p))) && !deleted.has(p);

const exportedNames = (text: string, file: string): Map<string, { type: boolean }> => {
  const out = new Map<string, { type: boolean }>();
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  for (const st of sf.statements) {
    if (ts.isExportDeclaration(st) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier)) {
      const spec = st.moduleSpecifier.text;
      const target = resolveRel(file, spec);
      if (!target) { warn("glue-specifier-unresolved", `${file}: ${spec}`); continue; }
      const to = originsOf(target);
      if (st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) {
        const orig = (e.propertyName ?? e.name).text;
        const o = to?.get(orig);
        if (!o && !target.endsWith(".json") && !(to && [...to.keys()].some(k => k.startsWith("*bare:")))) warn("glue-name-missing-in-target", `${file}: ${orig} not exported by ${target}`);
        const typeKind = !!o && (o.typeKind || o.via);
        const declaredType = st.isTypeOnly || e.isTypeOnly;
        if (o && typeKind && !declaredType) warn("glue-type-exported-as-value", `${file}: ${orig}`);
        out.set(e.name.text, { type: declaredType });
      } else if (!st.exportClause) for (const [k, o] of to ?? []) if (k !== "default" && !k.startsWith("*bare:")) out.set(k, { type: o.typeKind || o.via });
    } else if (ts.isImportDeclaration(st) && ts.isStringLiteral(st.moduleSpecifier) && st.moduleSpecifier.text.endsWith(".json")) {
      const target = resolveRel(file, st.moduleSpecifier.text);
      if (!target) warn("glue-json-unresolved", `${file}: ${st.moduleSpecifier.text}`); else out.set("default", { type: false });
    }
  }
  return out;
};
const glueNames = new Map<string, Map<string, { type: boolean }>>();
for (const [p, text] of virtual) glueNames.set(p, exportedNames(text, p));

// V1/V2 manifest adds
let adds = 0, removes = 0;
const finalExports = new Map<string, Map<string, string>>();
for (const p of model.all) finalExports.set(p.packageJson, new Map(Object.entries(p.exports).filter(([, v]) => typeof v === "string") as [string, string][]));
for (const e of entries) {
  if (e.field !== "exports") continue;
  const m = finalExports.get(e.packageJson)!;
  if (e.op === "add") {
    adds++;
    if (m.has(e.key) && m.get(e.key) !== e.value) warn("key-exists-with-other-value", `${e.packageJson} ${e.key}`);
    const file = posix(resolve(abs(dirname(e.packageJson)), e.value).slice(abs("x").length - 1));
    const rel = posix(file).replace(/^\//u, "");
    if (!exists(rel)) warn("export-target-missing", `${e.package} ${e.key} -> ${e.value}`);
    m.set(e.key, e.value);
  } else { removes++; m.delete(e.key); }
}
// V4 alias duplicates
let preexisting = 0;
const dupNow = (map: Map<string, string>): string[][] => { const by = new Map<string, string[]>(); for (const [k, v] of map) (by.get(v) ?? by.set(v, []).get(v)!).push(k); return [...by.values()].filter(x => x.length > 1); };
const before = new Map<string, number>();
for (const p of model.all) { const d = dupNow(new Map(Object.entries(p.exports).filter(([, v]) => typeof v === "string") as [string, string][])); before.set(p.packageJson, d.length); preexisting += d.length; }
let introduced = 0;
for (const [pj, m] of finalExports) { const d = dupNow(m); if (d.length > (before.get(pj) ?? 0)) { introduced += d.length - (before.get(pj) ?? 0); warn("alias-duplicate-introduced", `${pj}: ${d.map(x => x.join(" = ")).join("; ").slice(0, 300)}`); } }
// V6 path budget
const budget = (p: string): string[] => { const out: string[] = []; if ([...p].length > 256) out.push(">256 chars"); if (p.split("/").some(s => Buffer.byteLength(s) > 255)) out.push("component >255 bytes"); return out; };
for (const g of glue.filter((x: any) => x.kind === "create")) for (const m of budget(g.path)) warn("path-budget", `${g.path}: ${m}`);
// V7 rewrites
for (const r of rewrites) {
  const mv = batch.entries.find((e: any) => e.id === r.entry);
  const parts = r.to.split("/"); const pkgName = parts.slice(0, 2).join("/"); const key = parts.length > 2 ? "./" + parts.slice(2).join("/") : ".";
  const P = model.byName.get(pkgName);
  const value = finalExports.get(P?.packageJson ?? "")?.get(key);
  if (!P || !value) { warn("rewrite-target-key-missing", `${r.file}: ${r.to}`); continue; }
  const entryFile = posix(resolve(abs(P.dir), value).slice(abs("x").length - 1)).replace(/^\//u, "");
  const names = virtual.has(entryFile) ? glueNames.get(entryFile)! : (() => { const o = originsOf(entryFile); return new Map([...(o ?? [])].map(([k, v]) => [k, { type: v.typeKind || v.via }] as const)); })();
  const sf = ts.createSourceFile(r.file, readFileSync(abs(r.file), "utf8"), ts.ScriptTarget.Latest, true, r.file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const used: string[] = []; let star = false;
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === r.from) { const c = n.importClause; if (c?.name) used.push("default"); if (c?.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) used.push((e.propertyName ?? e.name).text); else if (c?.namedBindings) star = true; }
    if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === r.from) { if (n.exportClause && ts.isNamedExports(n.exportClause)) n.exportClause.elements.forEach(e => used.push((e.propertyName ?? e.name).text)); else star = true; }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  const lacking = used.filter(u => !names.has(u));
  if (lacking.length) warn("rewrite-names-missing", `${r.file}: ${r.from} -> ${r.to}: ${lacking.join(",")}`);
  void mv; void star;
}
// V8 deleted glue still referenced
const { edges } = scan();
for (const g of deleted) { const refs = edges.filter(e => e.target === g && !deleted.has(e.file)); if (refs.length) warn("deleted-glue-still-imported", `${g}: ${refs.map(r => r.file).slice(0, 3).join(", ")}`); if ([...finalExports.values()].some(m => [...m.values()].length && false)) void 0; }
for (const g of deleted) for (const p of model.all) for (const en of entriesOf(p)) if (en.file === g && !entries.some((e: any) => e.op === "remove" && e.packageJson === p.packageJson && e.key === en.sub)) warn("deleted-glue-still-exported", `${p.name} ${en.sub}`);
// V9 dependency entries
let depAdds = 0;
for (const e of entries.filter((x: any) => x.field !== "exports")) {
  depAdds++;
  const m = model.byJson.get(e.packageJson)!;
  if (e.key in m.deps || e.key in m.dev || e.key in m.peer) warn("dependency-already-declared", `${e.packageJson}: ${e.key}`);
  if (String(e.value).startsWith("workspace:")) { if (e.field === "dependencies" && reaches(e.key, m.name)) warn("dependency-cycle", `${m.name} -> ${e.key}`); if (!model.byName.has(e.key)) warn("dependency-unknown-workspace-package", e.key); }
}
// V10 glue path policy
const policy = loadPolicy();
for (const g of glue.filter((x: any) => x.kind === "create")) {
  const r = otherViolations(policy, "🧰️framework/📦️packages/x.ts", g.path, "aliased-workspace").filter(n => n === "no-core-path" || n === "no-impl-segment");
  const t = otherViolations(policy, "🧰️framework/📦️packages/x.ts", g.path, "local").filter(n => n === "no-core-path" || n === "no-impl-segment");
  if (r.length || t.length) warn("glue-path-policy", `${g.path}: ${[...new Set([...r, ...t])].join(",")}`);
}
void GENERATED;
const summary = Object.fromEntries(Object.entries(problems).map(([k, v]) => [k, v.length]));
writeFileSync(resolve(GENERATED, "validation.json"), JSON.stringify({ summary, problems }, null, 1));
console.log(JSON.stringify({ selectedEntries: entries.length, exportAdds: adds, exportRemoves: removes, dependencyAdds: depAdds, glueCreate: glue.filter((g: any) => g.kind === "create").length, glueExtend: glue.filter((g: any) => g.kind === "extend").length, glueDelete: deleted.size, rewrites: rewrites.length, aliasDuplicatesPreexisting: preexisting, aliasDuplicatesIntroduced: introduced }));
console.log("problems:", JSON.stringify(summary));
for (const [k, v] of Object.entries(problems)) for (const m of v.slice(0, 5)) console.log(" ", k, m.slice(0, 220));
