/** 📐 Static coverage of the v3 checks against feature-inventory-v3.md (no browser). Usage: node coverage.mjs */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { APPS } from "./apps.mjs";
import { coreChecks } from "./checks-core.mjs";
import { modelChecks } from "./checks-model.mjs";
import { resultsChecks } from "./checks-results.mjs";
import { viewerChecks } from "./checks-viewer.mjs";

const inv = readFileSync(fileURLToPath(new URL("../feature-inventory-v3.md", import.meta.url)), "utf8");
const rows = [...inv.matchAll(/^\| ([EV][23]D-\d+) \| (.*?) \|/gm)].map(m => ({ id: m[1], feature: m[2] }));
const covered = new Map(); const total = {};
for (const k of ["2d", "3d", "viewer2d", "viewer3d"]) {
  const a = APPS[k]; const cs = a.kind === "viewer" ? viewerChecks(a) : [...coreChecks(a), ...modelChecks(a), ...resultsChecks(a)];
  total[k] = cs.length;
  for (const c of cs) for (const i of c.inv?.[a.dim] ?? []) { const id = typeof i === "number" ? `${a.inv}-${String(i).padStart(2, "0")}` : i; covered.set(id, [...(covered.get(id) ?? []), `${k}:${c.id}`]); }
}
const gaps = rows.filter(r => !covered.has(r.id));
console.log(`checks per app: ${JSON.stringify(total)}`);
console.log(`inventory rows ${rows.length}, covered ${rows.length - gaps.length}, gaps ${gaps.length}`);
for (const g of gaps) console.log(`GAP ${g.id}: ${g.feature}`);
