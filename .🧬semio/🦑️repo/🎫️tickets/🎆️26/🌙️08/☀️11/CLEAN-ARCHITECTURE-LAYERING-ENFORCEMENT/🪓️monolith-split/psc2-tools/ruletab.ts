/** 📊️ Per-rule counts of cruiser error output: the cross-package family (all / outside tests / distinct edges), not-to-unlisted, no-circular, totals. Usage: bun ruletab.ts <err.txt>... */
import { readFileSync } from "node:fs";
const files = process.argv.slice(2);
const FAMILY = "no-cross-package-relative-";
const isTest = (p: string) => /\/🧪️tests?\/|\/🧪️testing\//u.test(p);
const parse = (p: string) => {
  const rules = new Map<string, { all: number; prod: number }>();
  const family = { lines: 0, prodLines: 0, edges: new Set<string>(), prodEdges: new Set<string>() };
  const byPkg = new Map<string, number>();
  let total = 0;
  for (const line of readFileSync(p, "utf8").normalize("NFC").split("\n")) {
    const x = line.match(/^  error ([^:]+): (.*) → (.*)$/u);
    if (!x) continue;
    total++;
    const r = rules.get(x[1]) ?? { all: 0, prod: 0 };
    r.all++;
    const prod = !isTest(x[2]);
    if (prod) r.prod++;
    rules.set(x[1], r);
    if (x[1].startsWith(FAMILY)) {
      family.lines++;
      if (prod) family.prodLines++;
      const e = x[2] + " → " + x[3];
      family.edges.add(e);
      if (prod) { family.prodEdges.add(e); byPkg.set(x[1].slice(FAMILY.length), (byPkg.get(x[1].slice(FAMILY.length)) ?? 0) + 1); }
    }
  }
  return { rules, family, total, byPkg };
};
const tabs = files.map(parse);
const row = (label: string, f: (t: ReturnType<typeof parse>) => string) => console.log(label.padEnd(58), ...tabs.map(t => f(t).padStart(18)));
row("rule (all / outside tests)", () => "");
row("no-cross-package-relative-* lines", t => `${t.family.lines}/${t.family.prodLines}`);
row("no-cross-package-relative-* distinct edges", t => `${t.family.edges.size}/${t.family.prodEdges.size}`);
for (const r of ["not-to-unlisted", "no-circular", "ui-no-framework-packages", "framework-no-products", "plugins-framework-sdk-only", "renderer-hosts-only-ui", "no-core-path"]) row(r, t => `${t.rules.get(r)?.all ?? 0}/${t.rules.get(r)?.prod ?? 0}`);
row("total error lines", t => String(t.total));
if (process.env.TOP) { const keys = [...new Set(tabs.flatMap(t => [...t.byPkg.keys()]))].sort((a, b) => (tabs[0].byPkg.get(b) ?? 0) - (tabs[0].byPkg.get(a) ?? 0)).slice(0, Number(process.env.TOP)); for (const k of keys) row("  prod " + k, t => String(t.byPkg.get(k) ?? 0)); }
