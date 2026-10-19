import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const src = process.argv[3] ?? "🧰️framework/🛍️products/🦑️repo/";
const tgt = process.argv[4] ?? "🧰️framework/🔨️modules/🏃️process/";
const byRule: Record<string, number> = {};
const unique = new Set<string>();
for (const v of r.summary.violations) {
  if (!v.from.startsWith(src)) continue;
  if (tgt && !v.to.startsWith(tgt)) continue;
  const rule = v.rule.name.replace(/^no-cross-package-relative-.*/, "no-cross-package-relative");
  byRule[rule] = (byRule[rule] ?? 0) + 1;
  unique.add(v.from + "\0" + v.to);
}
console.log(byRule, "unique edges", unique.size, "files", new Set([...unique].map((x) => x.split("\0")[0])).size);
