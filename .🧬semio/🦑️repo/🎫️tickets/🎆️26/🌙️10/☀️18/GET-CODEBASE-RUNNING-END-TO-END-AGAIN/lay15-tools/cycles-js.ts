import { readdirSync, readFileSync, statSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import ts from "typescript";

const root = "/Users/ueli/Documents/semio";
const roots = process.argv.slice(2).length ? process.argv.slice(2) : ["✏️s/🔌️plugins", "✏️s/🧑‍💻dev"];
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
let index = 0;
const idx = new Map<string, number>(), low = new Map<string, number>(), on = new Set<string>(), stack: string[] = [];
const sccs: string[][] = [];
const strong = (v: string) => {
  const work: [string, Iterator<string>][] = [[v, graph.get(v)![Symbol.iterator]()]];
  idx.set(v, index); low.set(v, index); index++; stack.push(v); on.add(v);
  while (work.length) {
    const [u, it] = work[work.length - 1]!;
    const n = it.next();
    if (!n.done) {
      const w = n.value;
      if (!idx.has(w)) { idx.set(w, index); low.set(w, index); index++; stack.push(w); on.add(w); work.push([w, graph.get(w)![Symbol.iterator]()]); }
      else if (on.has(w)) low.set(u, Math.min(low.get(u)!, idx.get(w)!));
    } else {
      work.pop();
      if (work.length) { const p = work[work.length - 1]![0]; low.set(p, Math.min(low.get(p)!, low.get(u)!)); }
      if (low.get(u) === idx.get(u)) {
        const c: string[] = []; let w: string;
        do { w = stack.pop()!; on.delete(w); c.push(w); } while (w !== u);
        if (c.length > 1 || graph.get(u)!.has(u)) sccs.push(c);
      }
    }
  }
};
for (const f of files) if (!idx.has(f)) strong(f);
const rel = (p: string) => p.slice(root.length + 1);
console.log(JSON.stringify({ files: files.length, sccs: sccs.length, members: sccs.reduce((a, c) => a + c.length, 0), scc: sccs.map((c) => c.map(rel).sort()) }, null, 1));
if (process.env.EDGES) {
  const want = new Set(sccs.flat());
  const short = (p: string) => rel(p).replace(/️/gu, "").replace("✏s/🔌plugins/📐cad/", "").replace("🗿artifacts/📐cad/🏅standards/🔖1/🪆subsets/✳any/", "…/").replace("✏s/🧑‍💻dev/📐cad/", "dev/");
  for (const f of want) for (const t of graph.get(f)!) if (want.has(t)) console.error(short(f), "->", short(t));
}
