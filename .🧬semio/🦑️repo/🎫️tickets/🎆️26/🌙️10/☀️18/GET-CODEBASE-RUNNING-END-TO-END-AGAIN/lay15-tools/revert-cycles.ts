import { readdirSync, readFileSync, statSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import ts from "typescript";

const root = "/Users/ueli/Documents/semio";
const planPath = process.argv[2]!, outPath = process.argv[3]!, apply = process.argv[4] === "apply";
const roots = ["🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];
const exts = [".ts", ".tsx", ".mts", ".cts", ".js", ".jsx", ".mjs", ".cjs"];
const skip = new Set(["node_modules", "target", "pkg", "dist", "📤️dist", ".git", "🗑️generated", ".cache", "bazel-out", "temp", "🎫️tickets", "captures"]);
const testPath = /(^|\/)🧪️tests\//u;
const files: string[] = [];
const pkgFiles: string[] = [];
const walk = (d: string) => {
  for (const e of readdirSync(d, { withFileTypes: true })) {
    if (skip.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.name === "package.json") pkgFiles.push(p);
    else if (/\.(ts|tsx|mts|cts|js|jsx|mjs|cjs)$/u.test(e.name) && !e.name.endsWith(".d.ts") && !testPath.test(p.slice(root.length))) files.push(p);
  }
};
for (const r of roots) walk(join(root, r));
const set = new Set(files);
const re = /(?:import|export)\s+(?:type\s+)?(?:[^'"`;]*?\sfrom\s+)?["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)/gu;
const pkgs = new Map<string, { dir: string; exports: unknown }>();
for (const f of pkgFiles) { try { const j = JSON.parse(readFileSync(f, "utf8")); if (typeof j.name === "string" && j.exports) pkgs.set(j.name, { dir: dirname(f), exports: j.exports }); } catch {} }
const pickTarget = (v: unknown): string | undefined => { if (typeof v === "string") return v; if (v && typeof v === "object") for (const k of ["bun", "import", "default", "types", "node"]) { const r = pickTarget((v as Record<string, unknown>)[k]); if (r) return r; } return undefined; };
const resolveBare = (spec: string): string | undefined => {
  if (spec.startsWith("node:") || spec.startsWith("bun:")) return undefined;
  const parts = spec.split("/");
  const name = spec.startsWith("@") ? parts.slice(0, 2).join("/") : parts[0]!;
  const pkg = pkgs.get(name); if (!pkg) return undefined;
  const sub = "." + spec.slice(name.length);
  const ex = pkg.exports as Record<string, unknown>;
  const table: Record<string, unknown> = ex && typeof ex === "object" && Object.keys(ex).some((k) => k.startsWith(".")) ? ex : { ".": ex };
  let t = pickTarget(table[sub]);
  if (!t) for (const [k, v] of Object.entries(table)) { const i = k.indexOf("*"); if (i < 0) continue; if (sub.startsWith(k.slice(0, i)) && sub.endsWith(k.slice(i + 1))) { const mid = sub.slice(i, sub.length - (k.length - i - 1)); const tv = pickTarget(v); if (tv) { t = tv.replace("*", mid); break; } } }
  if (!t) return undefined;
  const abs = resolve(pkg.dir, t);
  return set.has(abs) ? abs : undefined;
};
const resolveSpec = (from: string, spec: string): string | undefined => {
  if (!spec.startsWith(".")) return resolveBare(spec);
  const base = resolve(dirname(from), spec);
  for (const c of [base, ...exts.map((e) => base + e), ...exts.map((e) => join(base, "index" + e))]) if (set.has(c)) return c;
  return undefined;
};
const graph = new Map<string, Set<string>>();
for (const f of files) {
  const src = readFileSync(f, "utf8");
  const out = new Set<string>();
  const pre = ts.preProcessFile(src, true, true);
  for (const imp of pre.importedFiles) {
    const t = resolveSpec(f, imp.fileName);
    if (t) out.add(t);
  }
  graph.set(f, out);
}

import { writeFileSync } from "node:fs";
const plan = JSON.parse(readFileSync(planPath, "utf8"));
const applied: any[] = (plan.edits as any[]).filter(e => !(plan.staleList ?? []).some((x: any) => x.file === e.file && x.after === e.after));
const rel = (p: string) => p.slice(root.length + 1);
const sccOf = (g: Map<string, Set<string>>): Map<string, number> => {
  let index = 0, comp = 0;
  const idx = new Map<string, number>(), low = new Map<string, number>(), on = new Set<string>(), stack: string[] = [], out = new Map<string, number>();
  const strong = (v: string) => {
    const work: [string, Iterator<string>][] = [[v, (g.get(v) ?? new Set<string>())[Symbol.iterator]()]];
    idx.set(v, index); low.set(v, index); index++; stack.push(v); on.add(v);
    while (work.length) {
      const [u, it] = work[work.length - 1]!;
      const n = it.next();
      if (!n.done) {
        const w = n.value;
        if (!g.has(w)) continue;
        if (!idx.has(w)) { idx.set(w, index); low.set(w, index); index++; stack.push(w); on.add(w); work.push([w, g.get(w)![Symbol.iterator]()]); }
        else if (on.has(w)) low.set(u, Math.min(low.get(u)!, idx.get(w)!));
      } else {
        work.pop();
        if (work.length) { const p = work[work.length - 1]![0]; low.set(p, Math.min(low.get(p)!, low.get(u)!)); }
        if (low.get(u) === idx.get(u)) { let w: string; const members: string[] = []; do { w = stack.pop()!; on.delete(w); members.push(w); } while (w !== u); const id = comp++; if (members.length > 1) for (const m of members) out.set(m, id); }
      }
    }
  };
  for (const f of g.keys()) if (!idx.has(f)) strong(f);
  return out;
};
const baseline = sccOf(new Map([...graph].map(([k, v]) => [k, new Set(v)])));
const absOf = (e: any) => resolve(root, e.file);
const info = applied.map(e => { const newTo = resolveSpec(absOf(e), e.after); const oldTo = resolve(root, e.to); return { e, newTo, oldTo }; });
const g0 = new Map([...graph].map(([k, v]) => [k, new Set(v)]));
const reverted = new Set<any>();
const state = new Map<string, Set<string>>([...graph].map(([k, v]) => [k, new Set(v)]));
for (let round = 0; round < 20; round++) {
  const comp = sccOf(state);
  let n = 0;
  for (const x of info) {
    if (reverted.has(x.e) || !x.newTo) continue;
    const a = comp.get(absOf(x.e)), b = comp.get(x.newTo);
    if (a !== undefined && a === b) {
      reverted.add(x.e); n++;
      const set = state.get(absOf(x.e));
      if (set) { set.delete(x.newTo); if (set.has(x.oldTo) === false && graph.get(absOf(x.e))?.has(x.oldTo)) set.add(x.oldTo); }
    }
  }
  console.log("round", round, "reverted", n);
  if (!n) break;
}
let done = 0, missing = 0;
if (apply) {
  const byFile = new Map<string, any[]>();
  for (const e of reverted) (byFile.get(e.file) ?? byFile.set(e.file, []).get(e.file)!).push(e);
  for (const [file, list] of byFile) {
    const p = resolve(root, file);
    let text = readFileSync(p, "utf8");
    for (const e of list) {
      const q = [`"${e.after}"`, `'${e.after}'`, `\`${e.after}\``].find(s => text.includes(s));
      if (!q) { missing++; continue; }
      text = text.replace(q, q[0] + e.before + q[0]);
      done++;
    }
    writeFileSync(p, text);
  }
}
writeFileSync(outPath, JSON.stringify({ reverted: [...reverted].map(e => ({ file: e.file, before: e.before, after: e.after })), done, missing }, null, 1));
console.log(JSON.stringify({ edits: applied.length, reverted: reverted.size, done, missing }));
