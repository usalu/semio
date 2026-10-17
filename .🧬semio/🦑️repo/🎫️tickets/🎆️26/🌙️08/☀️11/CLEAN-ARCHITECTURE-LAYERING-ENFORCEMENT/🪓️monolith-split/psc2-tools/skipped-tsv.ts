/** 📄️ Writes every still-flagged edge with its reason: skipped slice, cross-scope, or open. Usage: bun skipped-tsv.ts <scan.json> <out.tsv> */
import { readFileSync, writeFileSync } from "node:fs";
import { skipReason, crossScope } from "./skip.ts";
const edges = JSON.parse(readFileSync(process.argv[2], "utf8")) as any[];
const rows = edges.map(e => [skipReason(e.file) ?? skipReason(e.resolved) ?? crossScope(e.file, e.resolved) ?? (e.resolved.includes("/🖱️ui/") ? "open: ui internals (owner decision)" : /\.(ts|tsx)$/u.test(e.resolved) ? "open" : "open: unsupported asset or JavaScript target"), e.test ? "test" : "prod", e.file, e.line, e.resolved, e.sourceOwner].join("\t"));
rows.sort();
writeFileSync(process.argv[3], "reason\tkind\timporter\tline\ttarget\ttarget owner\n" + rows.join("\n") + "\n");
const c = new Map<string, number>(); for (const r of rows) { const k = r.split("\t").slice(0, 2).join(" "); c.set(k, (c.get(k) ?? 0) + 1); }
for (const [k, v] of [...c].sort()) console.log(v, k);
