import { readFileSync } from "node:fs";
const { edges } = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const prefix = process.argv[4] ?? "🧰️framework/🛍️products/🦑️repo/";
const pkg = process.argv[3]!;
const sel = edges.filter((e: any) => e.from.startsWith(prefix) && e.rule === "no-cross-package-relative-" + pkg);
const by: Record<string, number> = {};
for (const e of sel) by[e.to] = (by[e.to] ?? 0) + 1;
console.log(sel.length, "edges", Object.keys(by).length, "targets");
for (const [t, n] of Object.entries(by).sort((a, b) => b[1] - a[1]).slice(0, Number(process.argv[5] ?? 40))) console.log(n, t);
