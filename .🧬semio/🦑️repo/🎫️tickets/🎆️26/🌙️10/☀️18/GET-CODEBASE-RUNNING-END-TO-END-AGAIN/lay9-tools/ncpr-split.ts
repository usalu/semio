import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const pkgFilter = process.argv[3]!;
const R = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/";
const OS = "🧰️framework/🛍️products/💻️os/";
const v = (r.summary.violations as any[]).filter((x) => x.from.startsWith(R) && x.rule.name === "no-cross-package-relative-" + pkgFilter);
const m = new Map<string, number>();
for (const x of v) { const k = x.from.replace(R, "").split("/").slice(0, 3).join("/") + "  ->  " + x.to.replace(OS, "").split("/").slice(0, 6).join("/"); m.set(k, (m.get(k) ?? 0) + 1); }
for (const [k, n] of [...m].sort((a, b) => b[1] - a[1]).slice(0, 40)) console.log(n, k);
