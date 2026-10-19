/**
 * ↩️ Reverts the applied specifier rewrites that introduce a violation of another boundary rule or a new import cycle.
 *
 *   bun revert-regress.ts <plan.json> <beforeDir> <afterDir> <out.json> [apply]
 */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";

const [planPath, dir, beforePrefix, afterPrefix, outPath, mode] = process.argv.slice(2) as [string, string, string, string, string, string | undefined];
const beforeDir = dir, afterDir = dir;
const ROOT = "/Users/ueli/Documents/semio";
const policy = require(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs`);
const plan = JSON.parse(readFileSync(planPath, "utf8"));
const edits = (plan.edits as any[]).filter(e => !plan.unresolved?.some((u: any) => u.file === e.file && u.after === e.after));

const arr = (v: unknown): string[] => (v === undefined ? [] : Array.isArray(v) ? v : [v as string]);
const rx = (p: string): RegExp => { try { return new RegExp(p, "u"); } catch { return new RegExp(p); } };
const rules = (policy.forbidden as any[]).filter(r => !r.name.startsWith("no-cross-package-relative-") && r.name !== "no-circular" && !r.to?.dependencyTypes && !r.to?.dependencyTypesNot && !r.to?.couldNotResolve && !r.to?.circular && !r.to?.orphan && !r.from?.orphan && !r.module && r.name !== "not-to-unlisted").map(r => ({
  name: r.name,
  fromPath: arr(r.from?.path).map(rx), fromNot: arr(r.from?.pathNot).map(rx), toPath: arr(r.to?.path).map(rx), toNot: arr(r.to?.pathNot).map(rx),
}));
const violates = (from: string, to: string): string[] => rules.filter(r => (!r.fromPath.length || r.fromPath.some(x => x.test(from))) && !r.fromNot.some(x => x.test(from)) && (!r.toPath.length || r.toPath.some(x => x.test(to))) && !r.toNot.some(x => x.test(to))).map(r => r.name);

const resolveNew = (e: any): string | null => {
  try {
    const r = e.after.startsWith(".") ? resolve(dirname(resolve(ROOT, e.file)), e.after) : Bun.resolveSync(e.after, dirname(resolve(ROOT, e.file)));
    return relative(ROOT, r).split("\\").join("/");
  } catch { return null; }
};

const graph = (dir: string, prefix: string): Map<string, Set<string>> => {
  const g = new Map<string, Set<string>>();
  for (const f of readdirSync(dir).filter(f => f.startsWith(prefix) && f.endsWith(".json") && !/merged|plan|edges|revert/u.test(f))) {
    const r = JSON.parse(readFileSync(`${dir}/${f}`, "utf8"));
    for (const m of r.modules) {
      const s = g.get(m.source) ?? new Set<string>();
      for (const d of m.dependencies) if (d.resolved && !d.coreModule && !d.couldNotResolve) s.add(d.resolved);
      g.set(m.source, s);
    }
  }
  return g;
};
const before = graph(beforeDir, beforePrefix), after = graph(afterDir, afterPrefix);

const sccOf = (g: Map<string, Set<string>>): Map<string, number> => {
  const index = new Map<string, number>(), low = new Map<string, number>(), onStack = new Set<string>(), stack: string[] = [], comp = new Map<string, number>();
  let i = 0, c = 0;
  for (const root of g.keys()) {
    if (index.has(root)) continue;
    const work: { v: string; it: Iterator<string> }[] = [{ v: root, it: (g.get(root) ?? new Set()).values() }];
    index.set(root, i); low.set(root, i++); stack.push(root); onStack.add(root);
    while (work.length) {
      const top = work[work.length - 1]!;
      const nx = top.it.next();
      if (!nx.done) {
        const w = nx.value;
        if (!g.has(w) && !index.has(w)) { index.set(w, i); low.set(w, i++); comp.set(w, -1 - c++); continue; }
        if (!index.has(w)) { index.set(w, i); low.set(w, i++); stack.push(w); onStack.add(w); work.push({ v: w, it: (g.get(w) ?? new Set()).values() }); }
        else if (onStack.has(w)) low.set(top.v, Math.min(low.get(top.v)!, index.get(w)!));
      } else {
        work.pop();
        if (work.length) { const p = work[work.length - 1]!.v; low.set(p, Math.min(low.get(p)!, low.get(top.v)!)); }
        if (low.get(top.v) === index.get(top.v)) { let w: string; do { w = stack.pop()!; onStack.delete(w); comp.set(w, c); } while (w !== top.v); c++; }
      }
    }
  }
  return comp;
};
const sccSizes = (comp: Map<string, number>): Map<number, number> => { const m = new Map<number, number>(); for (const v of comp.values()) m.set(v, (m.get(v) ?? 0) + 1); return m; };
const beforeComp = sccOf(before), beforeSize = sccSizes(beforeComp);
const wasCyclic = (a: string, b: string): boolean => { const x = beforeComp.get(a), y = beforeComp.get(b); return x !== undefined && x === y && (beforeSize.get(x) ?? 0) > 1; };

const reverted = new Map<any, string>();
const info = edits.map(e => ({ e, newTo: resolveNew(e), oldTo: (e.to as string) }));
for (const x of info) {
  if (!x.newTo) { reverted.set(x.e, "unresolvable"); continue; }
  const now = violates(x.e.file, x.newTo), was = new Set(violates(x.e.file, x.oldTo));
  const fresh = now.filter(n => !was.has(n));
  if (fresh.length) reverted.set(x.e, "rule:" + fresh.join(","));
}
for (let round = 0; round < 12; round++) {
  const g = new Map<string, Set<string>>();
  for (const [k, v] of after) g.set(k, new Set(v));
  for (const x of info) if (reverted.has(x.e) && x.newTo) { g.get(x.e.file)?.delete(x.newTo); (g.get(x.e.file) ?? g.set(x.e.file, new Set()).get(x.e.file)!).add(x.oldTo); }
  const comp = sccOf(g), size = sccSizes(comp);
  let n = 0;
  for (const x of info) {
    if (reverted.has(x.e) || !x.newTo) continue;
    const a = comp.get(x.e.file), b = comp.get(x.newTo);
    if (a !== undefined && a === b && (size.get(a) ?? 0) > 1 && !wasCyclic(x.e.file, x.oldTo)) { reverted.set(x.e, "cycle"); n++; }
  }
  console.log("round", round, "cycle reverts", n);
  if (!n) break;
}

const byReason: Record<string, number> = {};
for (const r of reverted.values()) byReason[r.replace(/^rule:.*/u, "rule")] = (byReason[r.replace(/^rule:.*/u, "rule")] ?? 0) + 1;
const ruleCounts: Record<string, number> = {};
for (const r of reverted.values()) if (r.startsWith("rule:")) for (const n of r.slice(5).split(",")) ruleCounts[n] = (ruleCounts[n] ?? 0) + 1;
let done = 0, missing = 0;
if (mode === "apply") {
  const byFile = new Map<string, any[]>();
  for (const e of reverted.keys()) (byFile.get(e.file) ?? byFile.set(e.file, []).get(e.file)!).push(e);
  for (const [file, list] of byFile) {
    const abs = resolve(ROOT, file);
    let text = readFileSync(abs, "utf8");
    for (const e of list) {
      const q = [`"${e.after}"`, `'${e.after}'`, `\`${e.after}\``].find(s => text.includes(s));
      if (!q) { missing++; continue; }
      text = text.replace(q, q[0] + e.before + q[0]);
      done++;
    }
    writeFileSync(abs, text);
  }
}
writeFileSync(outPath, JSON.stringify({ reverted: [...reverted].map(([e, r]) => ({ file: e.file, before: e.before, after: e.after, reason: r })), byReason, ruleCounts, done, missing }, null, 1));
console.log(JSON.stringify({ edits: edits.length, reverted: reverted.size, byReason, ruleCounts, done, missing }));
