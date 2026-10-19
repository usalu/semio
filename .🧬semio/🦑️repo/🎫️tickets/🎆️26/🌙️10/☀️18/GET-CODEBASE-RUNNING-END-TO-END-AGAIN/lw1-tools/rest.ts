import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const fam = (n: string) => n.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const v = (r.summary.violations as any[]).filter(x => x.from.startsWith("✏️s/🔌️plugins/") && x.to.startsWith("🧰️framework/") && !x.to.startsWith(OS));
const m = new Map<string, string[]>();
for (const x of v) { const k = x.from.replace("✏️s/🔌️plugins/", "") + " -> " + x.to.replace("🧰️framework/", "FW/").replace("🛍️products/🦑️repo/🔨️modules/📚️library/", "R/"); (m.get(k) ?? m.set(k, []).get(k)!).push(fam(x.rule.name)); }
for (const [k, f] of m) console.log(f.join(","), "|", k.slice(0, 330));
