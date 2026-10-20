/** 🔁 Export keys whose glue re-exports modules owned by another package (pass-through surface). */
import { resolve } from "node:path";
import { readFileSync, existsSync } from "node:fs";
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
import { entriesOf, lane, model, originsOf } from "../exports-stage-tools/model.ts";
import { scan } from "../exports-stage-tools/scan.ts";
const policy = loadPolicy();
const pkOf = (f: string): string => { const s = ownerOfFile(policy, f); return s ? lane(s.owner)?.name ?? s.names[0]! : "-"; };
const { edges, bare } = scan();
const rel = new Map<string, string[]>();
for (const e of edges) (rel.get(e.file) ?? rel.set(e.file, []).get(e.file)!).push(e.target);
const users = new Map<string, number>();
for (const b of bare) { const k = b.pkg + "\0" + b.sub; users.set(k, (users.get(k) ?? 0) + 1); }
const only = process.argv[2];
let total = 0;
const rows: string[] = [];
for (const p of model.all) {
  if (!p.name || (only && p.name !== only)) continue;
  for (const en of entriesOf(p)) {
    const targets = rel.get(en.file) ?? [];
    if (!targets.length) continue;
    const owners = new Set(targets.map(pkOf));
    owners.delete("-");
    if (owners.size && !owners.has(p.name)) { total++; rows.push(`${p.name} ${en.sub} -> ${[...owners].join(",")} (importers ${users.get(p.name + "\0" + en.sub) ?? 0})`); }
  }
}
console.log("pass-through keys", total);
console.log(rows.join("\n"));
