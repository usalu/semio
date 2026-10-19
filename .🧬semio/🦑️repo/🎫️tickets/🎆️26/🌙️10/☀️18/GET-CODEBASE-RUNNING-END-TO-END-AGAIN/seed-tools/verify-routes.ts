import { resolveAll } from "./graph.ts";
const file = process.argv[2]!;
const doc = Bun.JSONC.parse(await Bun.file(file).text());
const routes = resolveAll(doc.configurations, doc.inputs);
const c: Record<string, number> = {};
for (const r of routes) c[r.status] = (c[r.status] ?? 0) + 1;
console.log(file.slice(-40), routes.length, JSON.stringify(c));
const bad = routes.filter((r) => !["ok", "ok-noTarget", "ticket-project-only"].includes(r.status));
const g = new Map<string, number>();
for (const r of bad) { const k = `${r.status} ${r.project}${r.target ? ":" + r.target : ""}`; g.set(k, (g.get(k) ?? 0) + 1); }
for (const [k, n] of g) console.log(n, k);
const tp = routes.filter((r) => r.status === "ticket-project-only");
const tg = new Map<string, number>();
for (const r of tp) tg.set(r.project, (tg.get(r.project) ?? 0) + 1);
console.log("ticket-project-only projects", tg.size, [...tg].sort((a, b) => b[1] - a[1]).slice(0, 8));
