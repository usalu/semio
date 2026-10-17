/** 🧹️ Lists the exports this slice created that no file imports by specifier yet (prepared for hand-off owners or left over). Usage: bun unused.ts <plan.json>... */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT } from "../sdkb-tools/lib.ts";
import { listSourceFiles } from "./model.ts";
const facadeFiles = new Set<string>();
for (const p of process.argv.slice(2)) for (const f of JSON.parse(readFileSync(p, "utf8")).facades) facadeFiles.add(f.facade.normalize("NFC"));
const used = new Map<string, number>();
const re = /["'`]((?:@semio-tech\/|os-hub-ts|semio-)[^"'`\s]*)["'`]/gu;
for (const f of listSourceFiles()) {
  let text: string; try { text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
  if (facadeFiles.has(f)) continue;
  if (!text.includes("@semio-tech/") && !text.includes("os-hub-ts")) continue;
  for (const m of text.matchAll(re)) used.set(m[1], (used.get(m[1]) ?? 0) + 1);
}
const rows: { plan: string; spec: string }[] = [];
for (const p of process.argv.slice(2)) {
  const plan = JSON.parse(readFileSync(p, "utf8"));
  for (const f of plan.facades) if (f.created) rows.push({ plan: p.split("-plan-")[1].replace(".json", ""), spec: f.pkg + (f.subpath === "." ? "" : f.subpath.slice(1)) });
  for (const d of plan.directExports) rows.push({ plan: p.split("-plan-")[1].replace(".json", ""), spec: d.pkg + (d.subpath === "." ? "" : d.subpath.slice(1)) });
}
const unused = rows.filter(r => !used.has(r.spec));
const per = new Map<string, number>(); for (const r of rows) per.set(r.plan, (per.get(r.plan) ?? 0) + 1);
const perU = new Map<string, number>(); for (const r of unused) perU.set(r.plan, (perU.get(r.plan) ?? 0) + 1);
console.log({ createdExports: rows.length, unused: unused.length });
for (const [k, v] of per) console.log(k.padEnd(6), `created ${v}, unused ${perU.get(k) ?? 0}`);
for (const r of unused.slice(0, 80)) console.log("  UNUSED", r.plan, r.spec);
