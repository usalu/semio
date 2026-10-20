/** 🖱️ Evaluates the ui boundary against the carved ownership: which packages the ui reaches at runtime, and whether that set is closed under the ui-foundation rule. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { build, isTest } from "./sim.ts";
const plan = JSON.parse(readFileSync(resolve(import.meta.dir, "carves.json"), "utf8"));
const es = build(plan);
const BUILD = /\/🏗️builder\//u;
const isUi = (n: string): boolean => /^@semio-tech\/ui(-|$)/u.test(n);
const runtime = es.filter(e => !e.test && !BUILD.test(e.file));
const reach = new Set<string>();
const via = new Map<string, Set<string>>();
for (const e of runtime) if (isUi(e.from) && !isUi(e.to)) { reach.add(e.to); (via.get(e.to) ?? via.set(e.to, new Set()).get(e.to)!).add(e.file); }
console.log("packages reached by ui at runtime:", reach.size);
for (const p of [...reach].sort()) console.log("  ", p, via.get(p)!.size);
const outside = new Map<string, Map<string, number>>();
for (const e of runtime) if (reach.has(e.from) && !reach.has(e.to) && !isUi(e.to)) { const m = outside.get(e.from) ?? outside.set(e.from, new Map()).get(e.from)!; m.set(e.to, (m.get(e.to) ?? 0) + 1); }
console.log("closure violations (foundation -> outside):", [...outside.values()].reduce((a, m) => a + m.size, 0));
for (const [f, m] of outside) for (const [t, n] of m) console.log("  ", f, "->", t, n);
const tests = new Map<string, Set<string>>();
for (const e of es) if (isUi(e.from) && !isUi(e.to) && !reach.has(e.to) && !(e.test || BUILD.test(e.file))) (tests.get(e.to) ?? tests.set(e.to, new Set()).get(e.to)!).add(e.file);
console.log("ui runtime edges to non-reached:", tests.size);
