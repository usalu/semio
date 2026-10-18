import { parseFile, sccs } from "./lib.ts";
const p = parseFile(process.argv[2]);
const anchor = process.argv[3];
const n = p.units.length;
const seed = p.units.find((u) => u.names.has(anchor))!;
const cs = sccs(n, (i) => p.units[i].deps).find((c) => c.includes(seed.id))!;
const phase: string[] = new Array(n).fill("pre");
const coreSet = new Set(cs);
cs.forEach((u) => (phase[u] = "core"));
const rev: number[][] = p.units.map(() => []);
p.units.forEach((u) => u.deps.forEach((d) => rev[d].push(u.id)));
const st = [...cs];
while (st.length) { const x = st.pop()!; for (const y of rev[x]) if (!coreSet.has(y) && phase[y] !== "post") { phase[y] = "post"; st.push(y); } }
// print runs of consecutive phase
let i = 0;
while (i < n) {
  let j = i;
  while (j + 1 < n && phase[j + 1] === phase[i]) j++;
  const lines = p.units.slice(i, j + 1).reduce((a, u) => a + u.lines, 0);
  console.log(`${i}-${j}\t${phase[i]}\t${lines}L\t${[...p.units[i].names][0]}..${[...p.units[j].names][0]}`);
  i = j + 1;
}
