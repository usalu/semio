import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const roots = ["✏️s/🧑‍💻dev/", "✏️s/🧪️tests/", "♻️mit-bestand/"];
const v = (r.summary.violations as any[]).filter((x) => roots.some((p) => x.from.startsWith(p)));
const byRule: Record<string, number> = {};
const files = new Set<string>();
for (const x of v) { const f = fam(x.rule.name); byRule[f] = (byRule[f] ?? 0) + 1; files.add(x.from); }
console.log(JSON.stringify({ total: v.length, files: files.size, byRule }, null, 1));
if (process.argv[3] === "list") for (const x of v) console.log(x.rule.name, "|", x.from, "->", x.to);
