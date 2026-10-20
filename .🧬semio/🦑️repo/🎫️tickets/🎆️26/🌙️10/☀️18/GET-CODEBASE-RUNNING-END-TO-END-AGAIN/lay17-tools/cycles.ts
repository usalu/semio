/**
 * 🔁 Strongly connected components of the production import graph (no-circular semantics: paths with `🧪️tests/` are excluded).
 *
 *   bun cycles.ts <scan.graph>
 */
import { readFileSync } from "node:fs";
const TESTS = /(^|\/)🧪️tests\//u;
const adj = new Map<string, Set<string>>();
for (const l of readFileSync(process.argv[2]!, "utf8").split("\n")) {
  if (!l) continue;
  const [a, b] = l.split("\t") as [string, string];
  if (TESTS.test(a) || TESTS.test(b)) continue;
  (adj.get(a) ?? adj.set(a, new Set()).get(a)!).add(b);
  if (!adj.has(b)) adj.set(b, new Set());
}
let index = 0;
const idx = new Map<string, number>(), low = new Map<string, number>(), on = new Set<string>(), stack: string[] = [];
const sccs: string[][] = [];
for (const root of adj.keys()) {
  if (idx.has(root)) continue;
  const work: [string, Iterator<string>][] = [[root, adj.get(root)![Symbol.iterator]()]];
  idx.set(root, index); low.set(root, index); index++; stack.push(root); on.add(root);
  while (work.length) {
    const [u, it] = work[work.length - 1]!;
    const n = it.next();
    if (!n.done) {
      const w = n.value;
      if (!idx.has(w)) { idx.set(w, index); low.set(w, index); index++; stack.push(w); on.add(w); work.push([w, adj.get(w)![Symbol.iterator]()]); }
      else if (on.has(w)) low.set(u, Math.min(low.get(u)!, idx.get(w)!));
    } else {
      work.pop();
      if (work.length) { const p = work[work.length - 1]![0]; low.set(p, Math.min(low.get(p)!, low.get(u)!)); }
      if (low.get(u) === idx.get(u)) {
        const c: string[] = []; let w: string;
        do { w = stack.pop()!; on.delete(w); c.push(w); } while (w !== u);
        if (c.length > 1 || adj.get(u)!.has(u)) sccs.push(c);
      }
    }
  }
}
console.log(`nodes ${adj.size} sccs ${sccs.length} members ${sccs.reduce((a, c) => a + c.length, 0)}`);
for (const c of sccs.sort((a, b) => b.length - a.length)) { console.log(`--- scc ${c.length}`); for (const m of c.sort().slice(0, 12)) console.log("  " + m.replace(/🧰️framework\/🛍️products\//u, "P/").replace(/🧰️framework\/🔨️modules\//u, "M/")); }
