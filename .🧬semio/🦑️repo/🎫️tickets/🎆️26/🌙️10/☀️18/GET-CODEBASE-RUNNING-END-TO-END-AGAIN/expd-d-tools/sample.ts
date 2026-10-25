import { readFileSync } from "node:fs";
const { edges } = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const prefix = "🧰️framework/🛍️products/🦑️repo/";
const t = process.argv[3]!;
const sel = edges.filter((e: any) => e.from.startsWith(prefix) && e.to === t);
for (const e of sel.slice(0, Number(process.argv[4] ?? 8))) console.log(e.rule, e.from);
