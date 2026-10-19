import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const v = (r.summary.violations as any[]).filter(x => x.from.startsWith("✏️s/🔌️plugins/") && x.to.startsWith("🧰️framework/") && !x.to.startsWith(OS));
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
console.log("WP1 violations", v.length);
for (const x of v) console.log(fam(x.rule.name).padEnd(30), x.from.slice(-80), "->", x.to.slice(-70));
