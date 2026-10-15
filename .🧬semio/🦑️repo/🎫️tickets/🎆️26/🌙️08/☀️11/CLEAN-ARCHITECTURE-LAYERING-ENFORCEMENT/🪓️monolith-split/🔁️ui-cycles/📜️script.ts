/** 🔁 Strongly connected components of a dependency-cruiser JSON report.
 * Usage: bun 📜️script.ts <report.json> [prefix] [fromRegex=>toRegex ...]  (drop edges before computing)
 */
import { readFileSync } from "node:fs";

const [reportPath, prefix = "", ...drops] = process.argv.slice(2);
const dropRules = drops.map((d) => d.split("=>") as [string, string]).map(([f, t]) => [new RegExp(f), new RegExp(t)] as const);
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const graph = new Map<string, string[]>();
for (const m of report.modules) {
  if (m.coreModule || m.couldNotResolve) continue;
  const targets: string[] = [];
  for (const d of m.dependencies) {
    if (d.coreModule || d.couldNotResolve || !d.resolved) continue;
    if (dropRules.some(([f, t]) => f.test(m.source) && t.test(d.resolved))) continue;
    targets.push(d.resolved);
  }
  graph.set(m.source, targets);
}
let index = 0;
const idx = new Map<string, number>();
const low = new Map<string, number>();
const onStack = new Set<string>();
const stack: string[] = [];
const components: string[][] = [];
const visit = (root: string) => {
  const work: [string, number][] = [[root, 0]];
  idx.set(root, index);
  low.set(root, index++);
  stack.push(root);
  onStack.add(root);
  while (work.length > 0) {
    const frame = work[work.length - 1]!;
    const [node, i] = frame;
    const edges = graph.get(node) ?? [];
    if (i < edges.length) {
      frame[1] = i + 1;
      const next = edges[i]!;
      if (!graph.has(next)) continue;
      if (!idx.has(next)) {
        idx.set(next, index);
        low.set(next, index++);
        stack.push(next);
        onStack.add(next);
        work.push([next, 0]);
      } else if (onStack.has(next)) {
        low.set(node, Math.min(low.get(node)!, idx.get(next)!));
      }
      continue;
    }
    work.pop();
    if (work.length > 0) {
      const parent = work[work.length - 1]![0];
      low.set(parent, Math.min(low.get(parent)!, low.get(node)!));
    }
    if (low.get(node) === idx.get(node)) {
      const component: string[] = [];
      let member: string;
      do {
        member = stack.pop()!;
        onStack.delete(member);
        component.push(member);
      } while (member !== node);
      if (component.length > 1 || (graph.get(node) ?? []).includes(node)) components.push(component);
    }
  }
};
for (const node of graph.keys()) if (!idx.has(node)) visit(node);
let total = 0;
components.sort((a, b) => b.length - a.length);
for (const [n, component] of components.entries()) {
  if (prefix && !component.some((c) => c.startsWith(prefix))) continue;
  const set = new Set(component);
  const edges: string[] = [];
  for (const c of component) for (const t of graph.get(c) ?? []) if (set.has(t)) edges.push(`${c} -> ${t}`);
  total += edges.length;
  console.log(`\n### SCC ${n} size=${component.length} edges=${edges.length}`);
  for (const e of edges.sort()) console.log(`  ${e}`);
}
console.log(`\nTOTAL components=${components.length} intra-edges=${total}`);
