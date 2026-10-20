/**
 * 🔮 Simulates the effect of the staged batch on the `no-cross-package-relative` rule (overlay of manifests + glue, nothing written).
 * Every current violation edge is classified; edges the batch enables are re-evaluated against the live policy selectors with the
 * post-batch authored export targets (the importer then points at the glue entry file).
 *
 *   bun simulate.ts   (writes generated/simulation.json and prints the table)
 */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, TICKET, posix } from "./lib.ts";
import { abs, entriesOf, model } from "./model.ts";
import { loadPolicy, ownerOfFile } from "./rules.ts";
import type { Group } from "./groups.ts";

type V = { file: string; spec: string; target: string; owner: string; pkgs: string[] };
const viol: V[] = JSON.parse(readFileSync(resolve(GENERATED, "ncpr-now.json"), "utf8"));
const groups: Group[] = JSON.parse(readFileSync(resolve(GENERATED, "groups.json"), "utf8"));
const cands = JSON.parse(readFileSync(resolve(GENERATED, "candidates.json"), "utf8")).candidates as any[];
const moves = JSON.parse(readFileSync(resolve(GENERATED, "moves.json"), "utf8")) as any[];
const batch = JSON.parse(readFileSync(resolve(TICKET, "exports-stage-tools/batch.json"), "utf8"));
const DEFAULT = new Set(["export", "move", "removal", "third-party", "types", "requested", "tst6"]);
const selected = new Set(batch.entries.filter((e: any) => e.apply && DEFAULT.has(e.tier)).map((e: any) => e.id));
const entryOf = new Map<string, any>(batch.entries.map((e: any) => [e.packageJson + "\0" + e.key + "\0" + e.op, e]));
const policy = loadPolicy();

const post = new Set<string>();
for (const p of model.all) for (const e of entriesOf(p)) post.add(e.file);
for (const e of batch.entries.filter((x: any) => selected.has(x.id) && x.field === "exports")) {
  const dir = e.packageJson.slice(0, e.packageJson.lastIndexOf("/"));
  const file = posix(resolve(abs(dir), e.value).slice(abs("x").length - 1)).replace(/^\//u, "");
  if (e.op === "add") post.add(file); else post.delete(file);
}
const candById = new Map(cands.map(c => [c.pkg + "\0" + c.key, c]));
const candByTarget = new Map<string, any>();
for (const g of groups) { const c = cands.find(x => x.owner === g.owner && (x.target === g.target || x.glue.path === g.target)); if (c) candByTarget.set(g.owner + "\0" + g.target, c); }
const movedGlue = new Map(moves.map(m => [m.glue, m]));
const coveredByKey = new Map<string, Group>();
for (const g of groups) coveredByKey.set(g.owner + "\0" + g.target, g);

const ncprPost = (importer: string, entryFile: string): boolean => {
  const sel = ownerOfFile(policy, entryFile);
  if (!sel) return false;
  if (sel.fromNot?.test(importer)) return false;
  return !post.has(entryFile);
};

const AREAS = readFileSync(resolve(TICKET, "lay15-tools/areas.txt"), "utf8").split("\n").filter(Boolean).map(l => { const [n, ...r] = l.split("|"); return [n!, r] as const; });
const areaOf = (f: string): string => { for (const [n, ps] of AREAS) if (ps.some(p => f === p || f.startsWith(p + "/"))) return n; if (f === "📜️script.ts") return "root-script"; return f.startsWith("🎓️teaching") ? "teaching" : f.startsWith("🌎️hub") ? "hub" : f.startsWith("🏢️semio-tech") ? "play" : "other"; };

type Cls = "enabled-by-batch" | "enabled-by-existing-entry" | "removed-with-misplaced-glue" | "irreducible-query" | "blocked-candidate" | "residual" | "unclassified";
const rows: { file: string; target: string; cls: Cls; area: string; detail?: string; weight: number }[] = [];
let postResidual = 0;
const pushRow = (r: (typeof rows)[number]): void => { rows.push(r); };
for (const v of viol) {
  const area = areaOf(v.file);
  if (v.spec.includes("?")) { pushRow({ file: v.file, target: v.target, weight: v.pkgs.length, cls: "irreducible-query", area, detail: "?raw/?url style import reads the file itself" }); continue; }
  const mv = movedGlue.get(v.file);
  if (mv) {
    const ok = batch.entries.some((e: any) => e.tier === "move" && e.packageJson && e.value !== undefined && selected.has(e.id) && e.justification?.misplacedGlue === v.file);
    pushRow({ file: v.file, target: v.target, weight: v.pkgs.length, cls: ok ? "removed-with-misplaced-glue" : "blocked-candidate", area }); continue;
  }
  const g = coveredByKey.get(v.owner + "\0" + v.target);
  if (!g) { pushRow({ file: v.file, target: v.target, weight: v.pkgs.length, cls: "unclassified", area }); continue; }
  if (g.covering.length) { const en = g.covering[0]!; if (ncprPost(v.file, en.file)) postResidual++; pushRow({ file: v.file, target: v.target, weight: v.pkgs.length, cls: "enabled-by-existing-entry", area, detail: en.pkg + en.sub }); continue; }
  const c = candByTarget.get(v.owner + "\0" + v.target);
  if (!c) { pushRow({ file: v.file, target: v.target, weight: v.pkgs.length, cls: "residual", area, detail: "no candidate (asset or no package lane)" }); continue; }
  const entry = entryOf.get(c.packageJson + "\0" + c.key + "\0add");
  const live = (model.byName.get(c.pkg)!.exports as Record<string, unknown>)[c.key];
  const applied = live !== undefined ? true : !!entry && selected.has(entry.id);
  if (!applied) { pushRow({ file: v.file, target: v.target, weight: v.pkgs.length, cls: "blocked-candidate", area, detail: c.pkg + c.key }); continue; }
  const entryFile = c.glue.mode === "none" ? c.target : c.glue.path;
  if (ncprPost(v.file, entryFile)) postResidual++;
  pushRow({ file: v.file, target: v.target, weight: v.pkgs.length, cls: "enabled-by-batch", area, detail: c.pkg + c.key });
}
const classes: Cls[] = ["enabled-by-batch", "enabled-by-existing-entry", "removed-with-misplaced-glue", "irreducible-query", "blocked-candidate", "residual", "unclassified"];
const total = rows.length;
const count = (f: (r: (typeof rows)[number]) => boolean): number => rows.filter(f).length;
const byArea = new Map<string, Record<string, number>>();
for (const r of rows) { const a = byArea.get(r.area) ?? byArea.set(r.area, Object.fromEntries(classes.map(c => [c, 0]))).get(r.area)!; a[r.cls]!++; }
const weighted = (f: (r: (typeof rows)[number]) => boolean): number => rows.filter(f).reduce((n, r) => n + r.weight, 0);
writeFileSync(resolve(GENERATED, "simulation.json"), JSON.stringify({ total, postResidual, cruiseStyleNow: weighted(() => true), cruiseStyleAfterBatchAndRewrite: weighted(r => r.cls === "irreducible-query" || r.cls === "blocked-candidate" || r.cls === "residual" || r.cls === "unclassified"), byClass: Object.fromEntries(classes.map(c => [c, count(r => r.cls === c)])), byArea: Object.fromEntries(byArea), rows }));
console.log(`current no-cross-package-relative violation edges: ${total}`);
for (const c of classes) console.log(`  ${c.padEnd(30)} ${String(count(r => r.cls === c)).padStart(5)}`);
const reducible = count(r => r.cls === "enabled-by-batch" || r.cls === "removed-with-misplaced-glue");
console.log(`edges the batch enables or removes: ${reducible}; already enabled by existing entries (pending importer rewrite): ${count(r => r.cls === "enabled-by-existing-entry")}`);
console.log(`after the batch AND the importer rewrite: ${total - reducible - count(r => r.cls === "enabled-by-existing-entry")} edges remain (${total - reducible} if the existing-entry rewrite is not run)`);
console.log(`cruise-style count (one violation per matching package rule, e.g. the framework owner carries a TypeScript and a Rust package): now ${weighted(() => true)}, after the batch and the importer rewrite ${weighted(r => r.cls === "irreducible-query" || r.cls === "blocked-candidate" || r.cls === "residual" || r.cls === "unclassified")}`);
console.log(`post-batch residual check on enabled edges (rule still firing when importers point at the entry file): ${postResidual}`);
console.log("| area | now | existing entry | batch | misplaced glue | query | blocked | residual |");
for (const [a, r] of [...byArea].sort((x, y) => Object.values(y[1]).reduce((n, v) => n + v, 0) - Object.values(x[1]).reduce((n, v) => n + v, 0))) console.log(`| ${a} | ${Object.values(r).reduce((n, v) => n + v, 0)} | ${r["enabled-by-existing-entry"]} | ${r["enabled-by-batch"]} | ${r["removed-with-misplaced-glue"]} | ${r["irreducible-query"]} | ${r["blocked-candidate"]} | ${r["residual"] + r["unclassified"]} |`);
