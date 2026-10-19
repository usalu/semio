import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const clo = new Set(readFileSync(process.argv[3]!, "utf8").split("\n").filter(Boolean));
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/";
const m: Record<string, number> = {};
for (const x of r.summary.violations) {
  if (!x.rule.name.startsWith("no-cross-package-relative-") || !x.from.startsWith(L) || !x.to.startsWith(L)) continue;
  const f = x.from.slice(L.length).split("/").slice(0, 2).join("/"), t = x.to.slice(L.length).split("/").slice(0, 3).join("/");
  const k = `${clo.has(x.from) ? "CLO" : "   "} ${f} -> ${t}`; m[k] = (m[k] ?? 0) + 1;
}
console.log(Object.entries(m).sort((a, b) => b[1] - a[1]).slice(0, 40).map(([k, n]) => `${n}\t${k}`).join("\n"));
