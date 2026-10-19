/**
 * 🧷 Reverts applied specifier rewrites inside the zero-touch CLI closure that pull a repo-lib barrel or a bare package specifier.
 *
 *   bun revert-closure.ts <plan.json> <revert.json> <entry> <out.json> [apply]
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
const [planPath, revertPath, entry, outPath, mode] = process.argv.slice(2) as string[];
const ROOT = "/Users/ueli/Documents/semio";
const plan = JSON.parse(readFileSync(planPath!, "utf8")), done = JSON.parse(readFileSync(revertPath!, "utf8"));
const gone = new Set((done.reverted as any[]).map(e => e.file + "\0" + e.after));
const kept = (plan.edits as any[]).filter(e => !gone.has(e.file + "\0" + e.after));
const pattern = /(?:import|export)\b[^;]*?from\s*["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)|^\s*import\s*["']([^"']+)["']/gm;
const cand = (p: string): string[] => (/\.(ts|tsx|mts|js|mjs)$/u.test(p) ? [p] : []);
const closure = (): Set<string> => {
  const seen = new Set<string>(), queue = [resolve(ROOT, entry!)];
  while (queue.length) {
    const f = queue.pop()!;
    if (seen.has(f) || !existsSync(f)) continue;
    seen.add(f);
    if (!/\.(ts|tsx|mts|mjs|cjs|js)$/u.test(f)) continue;
    const text = readFileSync(f, "utf8");
    for (const m of text.matchAll(pattern)) {
      const s = m[1] ?? m[2] ?? m[3];
      if (!s || !s.startsWith(".")) continue;
      const base = resolve(dirname(f), s.split("?")[0]!);
      const hit = cand(base).find(c => { try { return existsSync(c) && !require("node:fs").statSync(c).isDirectory(); } catch { return false; } });
      if (hit) queue.push(hit);
    }
  }
  return seen;
};
const rel = (p: string): string => relative(ROOT, p).split("\\").join("/");
const out: { file: string; before: string; after: string; reason: string }[] = [];
const dropped = new Set<any>();
for (let round = 0; round < 10; round++) {
  const live = kept.filter(e => !dropped.has(e));
  const cl = new Set([...closure()].map(rel));
  let n = 0;
  for (const e of live) {
    if (!cl.has(e.file)) continue;
    const barrel = e.after.includes("📦️packages/🟦️typescript/") && e.after.startsWith(".") || e.after.startsWith("@semio-tech/repo-lib");
    if (!e.after.startsWith(".") || barrel) { dropped.add(e); out.push({ file: e.file, before: e.before, after: e.after, reason: barrel ? "barrel" : "bare" }); n++; }
  }
  console.log("round", round, "closure", cl.size, "dropped", n);
  if (!n) break;
  if (mode === "apply") for (const e of out.filter(o => !(o as any).done)) {
    const abs = resolve(ROOT, e.file);
    let text = readFileSync(abs, "utf8");
    const q = [`"${e.after}"`, `'${e.after}'`].find(s => text.includes(s));
    (e as any).done = true;
    if (!q) { (e as any).missing = true; continue; }
    writeFileSync(abs, text.replace(q, q[0] + e.before + q[0]));
  }
}
writeFileSync(outPath!, JSON.stringify(out, null, 1));
console.log(out.length, out.filter(o => (o as any).missing).length);
