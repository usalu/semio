/**
 * 📦 Assembles the staged manifest batch: `batch.json` (entries + glue + rewrites + baselines) from the analysis outputs.
 *
 *   bun assemble.ts   (reads generated/{candidates,moves,deps,health}.json; writes batch.json next to this file)
 */
import { createHash } from "node:crypto";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";
import { GENERATED, ROOT, TICKET, posix } from "./lib.ts";
import { abs, entriesOf, model, originsOf, parseFile } from "./model.ts";
import { extensionLines, relSpec, type Reexport, reexportsOf } from "./glue-ops.ts";
import type { Candidate } from "./build-candidates.ts";
import type { Move } from "./moves.ts";

const read = <T>(name: string): T => JSON.parse(readFileSync(resolve(GENERATED, name), "utf8")) as T;
const sha = (path: string): string => createHash("sha256").update(readFileSync(abs(path))).digest("hex");

type Tier = "tst6" | "export" | "move" | "removal" | "third-party" | "types" | "requested" | "hygiene" | "physical";
export type Entry = {
  id: string; op: "add" | "remove"; packageJson: string; package: string; field: "exports" | "dependencies" | "devDependencies"; key: string; value: string;
  tier: Tier; apply: boolean; optional?: boolean; blocked?: string; justification: Record<string, unknown>; glueRef?: string[];
};
export type GlueOp =
  | { id: string; kind: "create"; path: string; content: string; entry: string | null; forKey?: string }
  | { id: string; kind: "extend"; path: string; lines: string[]; values: string[]; types: string[]; baselineSha256: string; entry: string | null; forKey?: string }
  | { id: string; kind: "delete"; path: string; baselineSha256: string; entry: string | null };
export type Rewrite = { file: string; from: string; to: string; entry: string };

const candidates = read<{ candidates: Candidate[]; skipped: unknown[] }>("candidates.json");
const moves = read<Move[]>("moves.json");
const deps = read<any[]>("deps.json");
const health = read<{ missing: { pkg: string; packageJson: string; key: string; target: string }[] }>("health.json");

const restructured = new Set(existsSync(resolve(TICKET, "tax-stage-tools/library-43.txt")) ? readFileSync(resolve(TICKET, "tax-stage-tools/library-43.txt"), "utf8").split("\n").filter(Boolean) : []);
const entries: Entry[] = [];
const glue: GlueOp[] = [];
const rewrites: Rewrite[] = [];
let seq = 0;
const nid = (p: string): string => p + String(++seq).padStart(4, "0");
const glueByPath = new Map<string, GlueOp>();
const addGlue = (g: GlueOp): void => { glueByPath.set(g.path + g.kind, g); glue.push(g); };

for (const c of candidates.candidates) {
  const P = model.byName.get(c.pkg)!;
  const live = (P.exports as Record<string, unknown>)[c.key];
  const blocked: string[] = [];
  for (const f of c.flags) if (f.startsWith("glue-violates:")) blocked.push(f);
  if (c.flags.some(f => f.startsWith("names-missing-in-target:"))) blocked.push(c.flags.find(f => f.startsWith("names-missing-in-target:"))!);
  if (restructured.has(c.target)) blocked.push("target is restructured by tax-stage (library-43: body moves into the sibling owner module); re-stage after it lands");
  if (live !== undefined && live !== c.value) blocked.push("manifest key exists with another value: " + JSON.stringify(live));
  const id = nid("E");
  const justification: Record<string, unknown> = {
    importerFiles: c.importers.length, importerEdges: c.edges, target: c.target, names: c.names === "*" ? "*" : (c.names as string[]).slice(0, 12).concat((c.names as string[]).length > 12 ? ["…" + ((c.names as string[]).length - 12) + " more"] : []),
    sources: [...new Set(["fresh-scan:no-cross-package-relative", ...c.proposals.map(p => p.source)])],
    proposedByInputs: c.proposals.filter(p => p.subpath).map(p => `${p.source}:${p.pkg ?? ""}${p.subpath}`),
  };
  if (c.carriedRules.length) justification.alsoFires = c.carriedRules;
  if (c.conflicts.length) justification.rewriteConflict = [...new Set(c.conflicts.map(x => x.split(":")[0]))];
  if (c.flags.some(f => f.startsWith("key-collision"))) justification.keyCollision = c.flags.filter(f => f.startsWith("key-collision"));
  if (live === undefined) entries.push({ id, op: "add", packageJson: c.packageJson, package: c.pkg, field: "exports", key: c.key, value: c.value, tier: "export", apply: !blocked.length, ...(blocked.length ? { blocked: blocked.join("; ") } : {}), justification, glueRef: [] });
  const ref = entries.find(e => e.id === id);
  if (c.glue.mode === "create") { const g: GlueOp = { id: nid("G"), kind: "create", path: c.glue.path, content: c.glue.content!, entry: id }; addGlue(g); ref?.glueRef?.push(g.id); }
  if (c.glue.mode === "extend") {
    const origins = c.target.endsWith(".json") ? null : originsOf(c.target);
    const want = c.names === "*" ? [...(origins?.keys() ?? [])].filter(k => !k.startsWith("*bare:")) : (c.names as string[]);
    const wanted = new Map<string, { target: string; type: boolean }>();
    for (const n of want) { const o = origins?.get(n); if (o) wanted.set(n, { target: c.target, type: o.typeKind || o.via }); }
    const ext = extensionLines(c.glue.path, wanted);
    if (ext.lines.length) { const g: GlueOp = { id: nid("G"), kind: "extend", path: c.glue.path, lines: ext.lines, values: ext.values, types: ext.types, baselineSha256: sha(c.glue.path), entry: live === undefined ? id : null, forKey: c.pkg + c.key }; addGlue(g); ref?.glueRef?.push(g.id); }
  }
}

for (const m of moves) {
  const P0 = model.byName.get(m.pkg0)!;
  const blocked = m.flags.length ? m.flags.join("; ") : undefined;
  const justification = { movedFrom: `${m.pkg0}${m.keys0.join(",")}`, misplacedGlue: m.glue, edgesInsideTheMisplacedGlue: m.edges, importerRewrites: m.rewrites.length, rule: "no-cross-package-relative / framework-no-products: a package must not carry glue that re-exports modules owned by another package", sources: ["lay13:misplacedGlue", "fresh-scan:no-cross-package-relative"] };
  const removeIds: string[] = [];
  for (const k of m.keys0) {
    const rid = nid("E");
    entries.push({ id: rid, op: "remove", packageJson: P0.packageJson, package: m.pkg0, field: "exports", key: k, value: (P0.exports as Record<string, string>)[k] ?? "", tier: "move", apply: !blocked, ...(blocked ? { blocked } : {}), justification: { ...justification, replacedBy: `${m.pkg}${m.destKey === "." ? "" : m.destKey.slice(1)}` }, glueRef: [] });
    removeIds.push(rid);
  }
  const anchor = removeIds[0]!;
  if (!m.destKeyExists) entries.push({ id: nid("E"), op: "add", packageJson: m.pkgJson, package: m.pkg, field: "exports", key: m.destKey, value: m.destValue, tier: "move", apply: !blocked, ...(blocked ? { blocked } : {}), justification, glueRef: [] });
  const destEntry = m.destKeyExists ? entries.find(e => e.id === anchor)! : entries[entries.length - 1]!;
  if (!m.destExists) { const g: GlueOp = { id: nid("G"), kind: "create", path: m.dest, content: m.content, entry: anchor }; addGlue(g); destEntry.glueRef?.push(g.id); }
  else {
    const wanted = new Map<string, { target: string; type: boolean }>();
    const gOrigins = originsOf(m.glue);
    for (const r of reexportsOf(m.glue)) {
      if (!r.target) continue;
      const tOrigins = r.target.endsWith(".json") ? null : originsOf(r.target);
      if (r.star) { for (const [k, o] of tOrigins ?? []) if (k !== "default" && !k.startsWith("*bare:")) wanted.set(k, { target: r.target, type: o.typeKind || o.via || r.typeOnly }); }
      else for (const nm of r.names) { wanted.set(nm, { target: r.target, type: r.typeOnly || !!(gOrigins?.get(nm)?.typeKind) || !!(gOrigins?.get(nm)?.via) }); }
    }
    const ext = extensionLines(m.dest, wanted);
    if (ext.lines.length) { const g: GlueOp = { id: nid("G"), kind: "extend", path: m.dest, lines: ext.lines, values: ext.values, types: ext.types, baselineSha256: sha(m.dest), entry: anchor, forKey: m.pkg + m.destKey }; addGlue(g); destEntry.glueRef?.push(g.id); }
  }
  addGlue({ id: nid("G"), kind: "delete", path: m.glue, baselineSha256: sha(m.glue), entry: anchor });
  for (const r of m.rewrites) rewrites.push({ file: r.file, from: r.from, to: r.to, entry: anchor });
}

for (const h of health.missing) {
  const P = model.byJson.get(h.packageJson)!;
  const live = (P.exports as Record<string, unknown>)[h.key];
  if (live === undefined) continue;
  entries.push({ id: nid("E"), op: "remove", packageJson: h.packageJson, package: h.pkg, field: "exports", key: h.key, value: String(live), tier: "removal", apply: true, justification: { reason: `export target ${h.target} no longer exists (module body moved into the sibling owner module)`, importers: ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts (re-pointed relatively by tsc-stage patch 8)"], sources: ["result-tsc-stage.md Open", "fresh-scan:export-health"] } });
}

for (const d of deps) {
  const tier: Tier = d.tier === "tst6" ? "tst6" : d.tier === "third-party" ? "third-party" : d.tier === "types" ? "types" : d.tier === "requested" ? "requested" : d.tier === "physical" ? "physical" : "hygiene";
  const apply = (tier === "third-party" || tier === "types" || tier === "requested" || tier === "tst6") && !d.blocked;
  const optional = !d.blocked && (tier === "hygiene" || tier === "physical");
  entries.push({ id: nid("D"), op: "add", packageJson: d.packageJson, package: d.manifest, field: d.field, key: d.name, value: d.version, tier, apply, ...(optional ? { optional: true } : {}), ...(d.blocked ? { blocked: d.blocked } : tier === "hygiene" ? { blocked: "optional (--include=hygiene): declaration hygiene, the package already resolves through the auto-linked workspace members" } : tier === "physical" ? { blocked: "optional decision (--include=physical): ✏️s must not depend on a hub package (puzzle-wasm); tsc5 resolves it through the ✏️s/tsconfig path" } : {}), justification: { importerFiles: d.importers.length, sampleImporters: d.importers.slice(0, 3), sources: d.sources, ...(d.note ? { note: d.note } : {}) } });
}

const specTargets = (path: string, text: string): { path: string; sha256: string; names: string[] }[] => {
  const out: { path: string; sha256: string; names: string[] }[] = [];
  const sf = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  for (const st of sf.statements) {
    if (!ts.isExportDeclaration(st) && !ts.isImportDeclaration(st)) continue;
    const ms = st.moduleSpecifier;
    if (!ms || !ts.isStringLiteral(ms) || !ms.text.startsWith(".")) continue;
    const target = resolve(abs(path), "..", ms.text);
    const rel = posix(target.slice(abs("x").length - 1)).replace(/^\//u, "");
    if (!existsSync(abs(rel))) continue;
    const names = ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause) ? st.exportClause.elements.map(e => (e.propertyName ?? e.name).text) : [];
    out.push({ path: rel, sha256: sha(rel), names });
  }
  return out;
};
for (const g of glue) if (g.kind === "create") (g as any).targets = specTargets(g.path, g.content); else if (g.kind === "extend") (g as any).targets = specTargets(g.path, g.lines.join("\n"));
const targetJsons = [...new Set(entries.filter(e => e.apply).map(e => e.packageJson))].sort();
const baseline: Record<string, { sha256: string; bytes: number }> = {};
for (const pj of [...new Set(entries.map(e => e.packageJson))].sort()) baseline[pj] = { sha256: sha(pj), bytes: readFileSync(abs(pj)).length };
const out = {
  schema: "exports-stage/batch@1",
  stagedAt: new Date().toISOString(),
  note: "STAGED ONLY. No package.json, bun.lock, library or glue file was edited. Apply with apply.ts after the freeze is lifted and the dependency sync has exited.",
  baseline,
  summary: {
    entries: entries.length, apply: entries.filter(e => e.apply).length, blocked: entries.filter(e => !e.apply).length,
    byTier: Object.fromEntries([...new Set(entries.map(e => e.tier))].map(t => [t, { total: entries.filter(e => e.tier === t).length, apply: entries.filter(e => e.tier === t && e.apply).length }])),
    manifests: targetJsons.length, glueCreate: glue.filter(g => g.kind === "create").length, glueExtend: glue.filter(g => g.kind === "extend").length, glueDelete: glue.filter(g => g.kind === "delete").length, rewrites: rewrites.length,
  },
  entries, glue, rewrites,
};
writeFileSync(resolve(TICKET, "exports-stage-tools/batch.json"), JSON.stringify(out, null, 1) + "\n");
console.log(JSON.stringify(out.summary));
void ROOT; void posix; void existsSync; void relSpec; void ts; void parseFile; void entriesOf; void (null as unknown as Reexport);
