/**
 * 🚨 Current `no-cross-package-relative` violations from the scan, evaluated with the exact live policy selectors (`bun ncpr-now.ts`).
 */
import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED } from "./lib.ts";
import { scan } from "./scan.ts";
import { loadPolicy, ncprViolation } from "./rules.ts";
const t0 = Date.now();
const policy = loadPolicy();
console.log("selectors", policy.selectors.length, "others", policy.others.length, Date.now() - t0, "ms");
const { edges } = scan();
const viol: any[] = [];
for (const e of edges) { const s = ncprViolation(policy, e.file, e.target); if (s) viol.push({ ...e, owner: s.owner, pkgs: s.names }); }
writeFileSync(resolve(GENERATED, "ncpr-now.json"), JSON.stringify(viol));
console.log("edges", edges.length, "ncpr violations", viol.length, "importers", new Set(viol.map(v => v.file)).size, "targets", new Set(viol.map(v => v.target)).size, Date.now() - t0, "ms");
const byOwner = new Map<string, number>();
for (const v of viol) byOwner.set(v.owner, (byOwner.get(v.owner) ?? 0) + 1);
console.log([...byOwner].sort((a, b) => b[1] - a[1]).slice(0, 25).map(([o, n]) => `${n} ${o}`).join("\n"));
