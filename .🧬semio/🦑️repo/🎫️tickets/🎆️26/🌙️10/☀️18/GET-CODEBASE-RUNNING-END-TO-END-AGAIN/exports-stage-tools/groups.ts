/**
 * 📊 Current `no-cross-package-relative` violations grouped by (owner, target): imported names, importers, existing covering entries.
 *
 *   bun groups.ts   (writes generated/groups.json)
 */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, ROOT, listAll } from "./lib.ts";
import { edgesOf } from "../repoapi-tools/lib.ts";
import { coveringEntries, lane, siblingsOf } from "./model.ts";
import { computeMoves } from "./moves.ts";

type V = { file: string; spec: string; target: string; kind: string; owner: string; pkgs: string[] };
const allViol: V[] = JSON.parse(readFileSync(resolve(GENERATED, "ncpr-now.json"), "utf8"));
export const queryEdges = allViol.filter(v => v.spec.includes("?"));
const movedGlue = new Set(computeMoves().map(m => m.glue));
export const movedEdges = allViol.filter(v => movedGlue.has(v.file));
const viol = allViol.filter(v => !v.spec.includes("?") && !movedGlue.has(v.file));
const known = new Set(listAll());
const byFile = new Map<string, V[]>();
for (const v of viol) (byFile.get(v.file) ?? byFile.set(v.file, []).get(v.file)!).push(v);

const BARRELS = new Set(["./artifacts", "./authoring", "./build", "./cargo", "./dev", "./inventory", "./laws", "./policy", "./process", "./storage", "./testing"]);
export type Group = { owner: string; target: string; pkg: string | null; importers: { file: string; spec: string; names: string[] | "*"; kind: string }[]; names: string[] | "*"; covering: { pkg: string; sub: string; file: string }[]; barrelOnly?: boolean };
const groups = new Map<string, Group>();
for (const [file, list] of byFile) {
  let edges: ReturnType<typeof edgesOf> = [];
  try { edges = edgesOf(file, readFileSync(resolve(ROOT, file), "utf8"), known); } catch { /* unreadable */ }
  for (const v of list) {
    const mine = edges.filter(e => e.spec === v.spec && e.target === v.target);
    const names: string[] | "*" = !mine.length || mine.some(e => e.star || e.kind === "side-effect" || e.kind === "require") ? "*" : [...new Set(mine.flatMap(e => (e.json ? ["default"] : e.names)))];
    const k = v.owner + "\0" + v.target;
    const g = groups.get(k) ?? groups.set(k, { owner: v.owner, target: v.target, pkg: lane(v.owner)?.name ?? null, importers: [], names: [], covering: [] }).get(k)!;
    g.importers.push({ file, spec: v.spec, names, kind: v.kind });
  }
}
for (const g of groups.values()) {
  g.names = g.importers.some(i => i.names === "*") ? "*" : [...new Set(g.importers.flatMap(i => i.names as string[]))].sort();
  const full = g.names === "*" ? coveringEntries(g.owner, g.target, "*") : coveringEntries(g.owner, g.target, g.names);
  g.covering = full.filter(e => !(e.pkg.name === "@semio-tech/repo-lib" && BARRELS.has(e.sub))).map(e => ({ pkg: e.pkg.name, sub: e.sub, file: e.file }));
  g.barrelOnly = full.length > 0 && !g.covering.length;
}
const list = [...groups.values()];
writeFileSync(resolve(GENERATED, "query-edges.json"), JSON.stringify(queryEdges));
writeFileSync(resolve(GENERATED, "groups.json"), JSON.stringify(list));
const covered = list.filter(g => g.covering.length);
console.log(`query-import edges (irreducible by manifest) ${queryEdges.length}; edges inside misplaced glue (removed by the moves) ${movedEdges.length}`);
console.log(`groups ${list.length} importer edges ${list.reduce((n, g) => n + g.importers.length, 0)} | fully covered by an existing entry ${covered.length} (${covered.reduce((n, g) => n + g.importers.length, 0)} edges) | uncovered ${list.length - covered.length} (${list.filter(g => !g.covering.length).reduce((n, g) => n + g.importers.length, 0)} edges)`);
console.log("owners without package:", [...new Set(list.filter(g => !g.pkg).map(g => g.owner))].join(" | "), "siblings check", siblingsOf(list[0]!.owner).length);
