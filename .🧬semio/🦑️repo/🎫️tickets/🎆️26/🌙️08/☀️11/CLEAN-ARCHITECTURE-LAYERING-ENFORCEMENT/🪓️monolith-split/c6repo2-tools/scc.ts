import { parseFile, sccs } from "./lib.ts";
const p = parseFile(process.argv[2]);
const comps = sccs(p.units.length, (i) => p.units[i].deps).filter((c) => c.length > 1);
const nm = (i: number) => [...p.units[i].names][0] ?? "?";
for (const c of comps) {
  const set = new Set(c);
  console.log(`SCC size=${c.length} lines=${c.reduce((s, i) => s + p.units[i].lines, 0)}`);
  for (const i of c.sort((a, b) => a - b)) console.log(`  ${i} L${p.units[i].startLine} ${p.units[i].lines} ${nm(i)} -> ${[...p.units[i].deps].filter((d) => set.has(d)).map(nm).join(",")}`);
}
