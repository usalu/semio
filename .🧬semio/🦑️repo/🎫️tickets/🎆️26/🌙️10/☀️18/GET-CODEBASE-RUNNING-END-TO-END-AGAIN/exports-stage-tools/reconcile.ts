/**
 * 🧮 Reconciles every earlier proposal (lay13, lay14, lay15, tax2b, glue, tsc-stage, tsc5) with the staged batch.
 *
 *   bun reconcile.ts   (writes generated/reconciliation.json)
 */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, TICKET } from "./lib.ts";
import { model } from "./model.ts";
import { proposals, lay13NotExportable, lay15Narrow } from "./inputs.ts";
import type { Group } from "./groups.ts";

const batch = JSON.parse(readFileSync(resolve(TICKET, "exports-stage-tools/batch.json"), "utf8"));
const cands = JSON.parse(readFileSync(resolve(GENERATED, "candidates.json"), "utf8")).candidates as any[];
const groups: Group[] = JSON.parse(readFileSync(resolve(GENERATED, "groups.json"), "utf8"));
const moves = JSON.parse(readFileSync(resolve(GENERATED, "moves.json"), "utf8")) as any[];
const ncpr = JSON.parse(readFileSync(resolve(GENERATED, "ncpr-now.json"), "utf8")) as any[];
const deps = batch.entries.filter((e: any) => e.field !== "exports");
const selected = new Set(batch.entries.filter((e: any) => e.apply).map((e: any) => e.id));
const candBy = new Map<string, any>(); for (const c of cands) { candBy.set(c.target, c); candBy.set(c.glue.path, c); for (const g of groups) if (g.owner === c.owner && c.importers.some((i: string) => g.importers.some(x => x.file === i))) candBy.set(g.target, c); }
const covered = new Map<string, Group>(groups.filter(g => g.covering.length).map(g => [g.target, g]));
const movedTargets = new Set(moves.flatMap(m => m.sources));
const movedGlue = new Set(moves.map(m => m.glue));
const status = (t: string): { s: string; key?: string } => {
  const c = candBy.get(t);
  if (c) { const e = batch.entries.find((x: any) => x.package === c.pkg && x.key === c.key && x.op === "add"); const live = (model.byName.get(c.pkg)!.exports as Record<string, unknown>)[c.key]; return { s: !e && live !== undefined ? "glue-extension-only" : e?.apply ? "in-batch" : "in-batch-blocked", key: c.pkg + c.key }; }
  const g = covered.get(t); if (g) return { s: "covered-by-existing-entry", key: g.covering[0]!.pkg + g.covering[0]!.sub };
  if (movedTargets.has(t) || movedGlue.has(t)) return { s: "handled-by-move" };
  return { s: "stale-no-current-violation" };
};
const out: Record<string, any> = {};
const tally = (rows: { s: string }[]): Record<string, number> => { const o: Record<string, number> = {}; for (const r of rows) o[r.s] = (o[r.s] ?? 0) + 1; return o; };
const l13 = [...proposals].filter(([, ps]) => ps.some(p => p.source === "lay13")).map(([t, ps]) => ({ target: t, ...status(t), proposed: ps.filter(p => p.source === "lay13").map(p => p.pkg + p.subpath) }));
out.lay13 = { targets: l13.length, ...tally(l13) };
const l14 = [...proposals].filter(([, ps]) => ps.some(p => p.source === "lay14")).map(([t]) => ({ target: t, ...status(t) }));
out.lay14 = { targets: l14.length, ...tally(l14) };
const l15 = lay15Narrow.map(k => ({ key: k.key, target: k.target, ...status(k.target), glueStatus: status(k.glue).s }));
out.lay15 = { keys: l15.length, ...tally(l15.map(x => ({ s: x.s === "stale-no-current-violation" && x.glueStatus !== "stale-no-current-violation" ? x.glueStatus : x.s }))) };
const t2 = readFileSync(resolve(TICKET, "result-tax2b.md"), "utf8");
const t2rows = [...t2.matchAll(/^\| `(FW[^`]*)` \| (\d+) \|/gmu)].map(m => ({ target: m[1]!.replace(/^FW\/P\//u, "🧰️framework/🛍️products/").replace(/^FW\//u, "🧰️framework/"), edges: +m[2]! }));
const t2s = t2rows.map(r => ({ ...r, ...status(r.target) }));
out.tax2b = { targets: t2s.length, edges: t2rows.reduce((n, r) => n + r.edges, 0), ...tally(t2s) };
const l13ne: Record<string, { total: number; stillViolating: number }> = {};
const violKeys = new Set(ncpr.map(v => v.file + "\0" + v.target));
for (const n of lay13NotExportable) { const r = n.reason.replace(/:.*/u, ""); const row = l13ne[r] ?? (l13ne[r] = { total: 0, stillViolating: 0 }); row.total++; if (violKeys.has(n.from + "\0" + n.to)) row.stillViolating++; }
out.lay13NotExportable = l13ne;
out.glueDeadEntries = { sixFrameworkRepoEntries: Object.keys(model.byName.get("@semio-tech/framework")!.exports).filter(k => k.startsWith("./repo/modules/library")).length, stylingInventory: "builder/python/inventory" in model.byName.get("@semio-tech/ui-styling")!.exports ? 1 : (model.byName.get("@semio-tech/ui-styling")!.exports as Record<string, unknown>)["./builder/python/inventory"] ? 1 : 0, configSchemaMutationsImporters: 0 };
const tsc5Names = ["d3-shape", "d3-interpolate", "d3-time", "d3-chord", "d3-contour", "d3-hexbin", "d3-delaunay", "d3-format", "d3-time-format", "d3-sankey", "d3-random", "d3-regression", "d3-quadtree", "dagre", "suncalc", "fft.js", "@asamuzakjp/css-color", "@types/d3-path", "@types/d3-polygon", "@types/d3-geo", "@types/d3-force", "unified", "remark-gfm", "remark-parse", "remark-rehype", "rehype-stringify", "@types/reveal.js", "dotenv", "@semio-tech/nx-bootstrap-tools", "@types/lines-and-columns"];
out.tsc5 = tsc5Names.map(n => ({ name: n, entries: deps.filter((d: any) => d.key === n).map((d: any) => `${d.packageJson.length > 36 ? d.packageJson.slice(0, 36) + "…" : d.packageJson} [${d.tier}${d.apply ? "" : ",not applied"}]`) }));
out.depsByTier = Object.fromEntries([...new Set(deps.map((d: any) => d.tier))].map(t => [t, deps.filter((d: any) => d.tier === t).length]));
void selected;
writeFileSync(resolve(GENERATED, "reconciliation.json"), JSON.stringify(out, null, 1));
console.log(JSON.stringify(out, null, 1).slice(0, 6000));
