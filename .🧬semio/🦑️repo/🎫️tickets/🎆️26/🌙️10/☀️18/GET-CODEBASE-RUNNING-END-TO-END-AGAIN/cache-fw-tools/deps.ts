import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, projectFiles, readJson, tracked } from "./lib.ts";
const plan = JSON.parse(readFileSync(join(import.meta.dir, "../🗑️generated/cache-fw-plan.json"), "utf8"));
const flips = new Set(plan.changed.map((c: any) => c[0]));
const cen = JSON.parse(readFileSync(join(import.meta.dir, "../🗑️generated/cache-fw-census-before.json"), "utf8"));
const byId = new Map<string, any>(); for (const t of cen.targets) byId.set(t.project + ":" + t.name, t);
// all manifests incl. other owners for outputs lookup
const outs = new Map<string, string[] | undefined>();
for (const t of cen.targets) outs.set(t.project + ":" + t.name, undefined);
const rows: string[] = [];
for (const f of projectFiles) { const j = readJson(f); for (const [name, t] of Object.entries<any>(j.targets ?? {})) { const id = j.name + ":" + name; if (!flips.has(id)) continue; const deps = (t.dependsOn ?? []).filter((d: any) => typeof d === "string"); if (!deps.length) continue; rows.push(id + " <- " + deps.join(", ") + (t.inputs && JSON.stringify(t.inputs).includes("dependentTasks") ? " [has DTO]" : "")); } }
console.log(rows.length); console.log(rows.join("\n").slice(0, 9000));
