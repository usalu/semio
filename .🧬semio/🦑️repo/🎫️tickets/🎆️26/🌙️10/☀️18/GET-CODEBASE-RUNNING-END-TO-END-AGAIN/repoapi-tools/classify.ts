import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, LIB_REL, ROOT, cliClosure, libTargets, readJson, writeJson, type Edge } from "./lib.ts";
import { plan, type Ctx, type Report } from "./engine.ts";
import { DOMAINS, domainOf, entryOf, frameworkRules, jsonNames, libRelOf } from "./rules.ts";
import { exportsOf } from "./exports.ts";
import { ensureScan } from "./scan.ts";

export type ClassifyResult = {
  reports: Report[];
  bTargets: Map<string, { names: Set<string>; star: boolean; json: boolean }>;
  jsonNames: Map<string, string>;
  ctx: Ctx;
};

/** Plans every importer file against the current tree: no file is written. */
export function classify(): ClassifyResult {
  const scan = ensureScan();
  const known = libTargets();
  const targets = [...new Set(scan.edges.map(e => libRelOf(e.target)))];
  for (const t of targets) if (!domainOf(t)) throw Error("no domain for " + t);
  const allJson = [...known].map(libRelOfSafe).filter((t): t is string => !!t && t.endsWith(".json"));
  const names = jsonNames([...new Set([...targets, ...allJson.filter(t => targets.includes(t))])]);
  const ctx: Ctx = { known, fw: frameworkRules(), closure: cliClosure(), relative: false, jsonNames: names, blocked: new Set() };
  const files = [...new Set(scan.edges.map(e => e.file))].sort();
  const reports: Report[] = [];
  for (const file of files) {
    try { reports.push(plan(file, readFileSync(resolve(ROOT, file), "utf8"), ctx)); }
    catch (error) { reports.push({ file, changed: false, text: "", b: new Map(), a: [], dead: [], open: [], failures: [String(error).split("\n")[0]!] }); }
  }
  const bTargets = new Map<string, { names: Set<string>; star: boolean; json: boolean }>();
  const edgesBy = new Map<string, Edge[]>();
  for (const e of scan.edges) { const k = e.file + "\0" + e.target; (edgesBy.get(k) ?? edgesBy.set(k, []).get(k)!).push(e); }
  for (const r of reports) {
    for (const [target, set] of r.b) {
      const lib = libRelOf(target);
      const row = bTargets.get(lib) ?? { names: new Set<string>(), star: false, json: lib.endsWith(".json") };
      bTargets.set(lib, row);
      for (const n of set) {
        if (n === "*" || n === "default" && !row.json) {
          if (n === "*") for (const e of edgesBy.get(r.file + "\0" + target) ?? []) { if (e.star) row.star = true; e.names.forEach(x => x !== "default" && row.names.add(x)); }
          else row.names.add("default");
        } else if (n !== "default") row.names.add(n);
      }
    }
  }
  return { reports, bTargets, jsonNames: names, ctx };
}
function libRelOfSafe(p: string): string | undefined { return p.startsWith(LIB_REL + "/") ? p.slice(LIB_REL.length + 1) : p.startsWith(LIB_REL) ? undefined : p; }

if (import.meta.main) {
  const t0 = Date.now();
  const r = classify();
  const kinds = { filesChanged: 0, a: 0, dead: 0, open: 0, failures: 0 };
  const byDest = new Map<string, number>();
  const fails: string[] = [];
  for (const rep of r.reports) {
    if (rep.changed) kinds.filesChanged++;
    kinds.a += rep.a.length; kinds.dead += rep.dead.length; kinds.open += rep.open.length; kinds.failures += rep.failures.length;
    for (const a of rep.a) byDest.set(a.to, (byDest.get(a.to) ?? 0) + 1);
    for (const f of rep.failures) fails.push(rep.file + " :: " + f);
  }
  console.log(kinds, `${Date.now() - t0} ms`);
  console.log([...byDest].sort((a, b) => b[1] - a[1]));
  console.log("failures:", fails.slice(0, 20));
  const byDomain = new Map<string, number>();
  for (const [t, row] of r.bTargets) byDomain.set(domainOf(t)!, (byDomain.get(domainOf(t)!) ?? 0) + row.names.size);
  console.log("b names by domain", [...byDomain]);
  void DOMAINS; void entryOf; void exportsOf;
}
