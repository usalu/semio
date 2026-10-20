import { scan } from "../exports-stage-tools/scan.ts";
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
const [prefix, skip] = process.argv.slice(2);
const policy = loadPolicy();
const { edges } = scan();
const by = new Map<string, { n: number; owner: string; ex: string }>();
for (const e of edges) {
  if (!e.file.startsWith(prefix!)) continue;
  if (e.target.startsWith(prefix!)) continue;
  const o = ownerOfFile(policy, e.target);
  const k = e.target;
  const c = by.get(k) ?? by.set(k, { n: 0, owner: o ? o.names.join(",") : "-", ex: e.file }).get(k)!;
  c.n++;
}
const rows = [...by].sort((a, b) => b[1].n - a[1].n);
console.log(rows.length, "targets", rows.reduce((s, r) => s + r[1].n, 0), "edges");
const byOwner = new Map<string, number>();
for (const [, v] of rows) byOwner.set(v.owner, (byOwner.get(v.owner) ?? 0) + v.n);
console.log([...byOwner].sort((a, b) => b[1] - a[1]).map(([o, n]) => `${n} ${o}`).join("\n"));
if (skip !== "owners") for (const [k, v] of rows.slice(0, 80)) console.log(v.n, v.owner, k);
