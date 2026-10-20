/** 🧹 Extras: declarations that no source edge needs any more are removed (or demoted to devDependencies when only tests, stories or scripts use them), for the packages that sit in declared production cycles after the carves. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Plan } from "./plan.ts";
import { finish } from "./direct.ts";
import { build } from "./sim.ts";
import { model } from "./lib.ts";
import { scc } from "./pkg-graph.ts";

const W = "extras-stale-declarations";
const parts = ["w1", "carves", "w2d", "w2e"].map(n => JSON.parse(readFileSync(resolve(import.meta.dir, `delta-parts/${n}.json`), "utf8")));
const delta = { manifestOps: parts.flatMap(p => p.manifestOps), fileOps: parts.flatMap(p => p.fileOps) };
const carves = JSON.parse(readFileSync(resolve(import.meta.dir, "carves.json"), "utf8"));
const gone = new Set<string>(delta.fileOps.filter((o: { op: string }) => o.op === "delete").map((o: { path: string }) => o.path));
const es = build(carves, false).filter(e => !gone.has(e.file));
const run = new Map<string, number>(), tst = new Map<string, number>();
for (const e of es) { const m = e.test ? tst : run; const k = e.from + ">" + e.to; m.set(k, (m.get(k) ?? 0) + 1); }
const prod = new Map<string, Set<string>>();
for (const p of model.all) if (p.name) prod.set(p.name, new Set(Object.entries(p.deps).filter(([k, v]) => String(v).startsWith("workspace:") && model.byName.has(k)).map(([k]) => k)));
for (const o of delta.manifestOps) if (o.field === "dependencies" && prod.has(o.package)) { const set = prod.get(o.package)!; if (o.op === "add") set.add(o.key); else set.delete(o.key); }
const plan = new Plan();
const removed: string[] = [];
for (let round = 0; round < 3; round++) {
  const comps = scc([...prod.keys()], prod).filter(c => c.length > 1);
  const inCycle = new Set(comps.flat());
  let changed = false;
  const compOf = new Map<string, number>(); comps.forEach((c, i) => c.forEach(n => compOf.set(n, i)));
  for (const name of inCycle) {
    const P = model.byName.get(name);
    if (!P) continue;
    for (const q of [...prod.get(name)!]) {
      const k = name + ">" + q;
      if (compOf.get(q) !== compOf.get(name)) continue;
      if ((run.get(k) ?? 0) > 0) continue;
      const wasRemovedAlready = removed.includes(k);
      if (wasRemovedAlready) continue;
      const value = P.deps[q];
      if (value === undefined) continue;
      const tests = tst.get(k) ?? 0;
      plan.manifest({ op: "remove", packageJson: P.packageJson, package: name, field: "dependencies", key: q, value, wave: W, why: tests ? `only tests, stories or scripts use ${q}: demoted to devDependencies` : `no source file of ${name} imports ${q} any more` });
      if (tests && !(q in P.dev)) plan.manifest({ op: "add", packageJson: P.packageJson, package: name, field: "devDependencies", key: q, value: "workspace:*", wave: W, why: `tests, stories or scripts of ${name} import ${q}` });
      prod.get(name)!.delete(q); removed.push(k); changed = true;
    }
  }
  if (!changed) break;
}
const comps = scc([...prod.keys()], prod).filter(c => c.length > 1);
console.log("declared production components after cleanup:", comps.map(c => c.length).join(",") || "none");
for (const c of comps) console.log("  ", c.join(" "));
finish(plan, "extras", process.argv);
