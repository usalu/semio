/** 📊️ Per-rule counts (all / outside 🧪️tests / distinct production edges) for the rules this slice moves. Usage: bun ruletab.ts <err.txt> [<err2.txt>] */
import { readFileSync } from "node:fs";
const files = process.argv.slice(2);
const RULES = ["no-cross-package-relative-@semio-tech/framework-schema", "no-cross-package-relative-@semio-tech/schema-ts", "no-cross-package-relative-@semio-tech/schema-validator-ts", "no-cross-package-relative-@semio-tech/schema-validator-rs", "no-cross-package-relative-@semio-tech/value-ts", "no-cross-package-relative-@semio-tech/value-rs", "not-to-unlisted", "no-circular", "framework-no-products", "plugins-framework-sdk-only"];
const parse = (p: string) => {
  const m = new Map<string, { all: number; prod: number; edges: Set<string> }>();
  let total = 0;
  for (const line of readFileSync(p, "utf8").split("\n")) {
    const x = line.match(/^  error ([^:]+): (.*)$/u);
    if (!x) continue;
    total++;
    const r = m.get(x[1]) ?? { all: 0, prod: 0, edges: new Set() };
    r.all++;
    if (!/\/🧪️tests?\//u.test(x[2].split(" → ")[0])) { r.prod++; r.edges.add(x[2]); }
    m.set(x[1], r);
  }
  return { m, total };
};
const tabs = files.map(parse);
console.log("rule".padEnd(64), ...files.map((_, i) => `all/prod[${i}]`.padStart(16)));
for (const rule of RULES) console.log(rule.replace("no-cross-package-relative-", "ncpr:").padEnd(64), ...tabs.map(t => { const r = t.m.get(rule); return `${r?.all ?? 0}/${r?.prod ?? 0}`.padStart(16); }));
console.log("total error lines".padEnd(64), ...tabs.map(t => String(t.total).padStart(16)));
const ours = RULES.slice(0, 6);
console.log("distinct prod edges across the 5 original rules".padEnd(64), ...tabs.map(t => { const s = new Set<string>(); for (const r of ours) for (const e of t.m.get(r)?.edges ?? []) s.add(e); return String(s.size).padStart(16); }));
