import { readdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { builtinModules } from "node:module";
import { dirname, join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const SCOPES = ["♻️mit-bestand", "🌎️hub", "🎓️teaching", "🏢️semio-tech", "🧰️framework", "✏️s"];
const SKIP = new Set(["node_modules", "target", "dist", "🗑️generated", ".cache", ".git", "bazel-out", ".vite", ".vite-temp", "storybook-static", "build"]);
const EXT = /\.(ts|tsx|mts|cts|js|mjs|cjs)$/;
const PATTERNS = [
  /\b(?:import|export)\s+(?:type\s+)?(?:[^"'`;]*?\s+from\s+)?["']([^"'\n]+)["']/g,
  /\bimport\s*\(\s*["']([^"'\n]+)["']\s*\)/g,
  /\brequire\s*\(\s*["']([^"'\n]+)["']\s*\)/g,
  /\bcreateRequire\s*\([^)]*\)\s*\(\s*["']([^"'\n]+)["']\s*\)/g,
];
const files: string[] = [];
const walk = (dir: string) => {
  let ents;
  try { ents = readdirSync(dir, { withFileTypes: true }); } catch { return; }
  for (const e of ents) {
    if (e.isDirectory()) { if (!SKIP.has(e.name)) walk(join(dir, e.name)); }
    else if (EXT.test(e.name)) files.push(join(dir, e.name));
  }
};
for (const s of SCOPES) walk(join(ROOT, s));
const builtin = new Set(builtinModules);
const bare = (s: string) => /^(@[a-z0-9][a-z0-9._-]*\/)?[a-z0-9][a-z0-9._-]*(\/[A-Za-z0-9._/@-]+)?$/.test(s) && !s.startsWith("node:") && !builtin.has(s.split("/")[0]!) && !/^(bun|bun:.*)$/.test(s);
const pkgCache = new Map<string, { dir: string; deps: Set<string>; name: string } | null>();
const owner = (dir: string) => {
  let d = dir;
  const chain: string[] = [];
  while (d.length >= ROOT.length) {
    if (pkgCache.has(d)) { const r = pkgCache.get(d)!; for (const c of chain) pkgCache.set(c, r); return r; }
    chain.push(d);
    const p = join(d, "package.json");
    if (existsSync(p)) {
      let r = null;
      try { const j = JSON.parse(readFileSync(p, "utf8")); r = { dir: d, name: j.name ?? "", deps: new Set([...Object.keys(j.dependencies ?? {}), ...Object.keys(j.devDependencies ?? {}), ...Object.keys(j.peerDependencies ?? {}), ...Object.keys(j.optionalDependencies ?? {})]) }; } catch {}
      for (const c of chain) pkgCache.set(c, r);
      return r;
    }
    d = dirname(d);
  }
  return null;
};
const out: Record<string, { pkg: string; kind: string; example: string; count: number }> = {};
for (const f of files) {
  let text: string;
  try { text = readFileSync(f, "utf8"); } catch { continue; }
  if (text.length > 4_000_000) continue;
  const dir = dirname(f);
  const specs = new Set<string>();
  for (const p of PATTERNS) { p.lastIndex = 0; let m; while ((m = p.exec(text))) if (bare(m[1]!)) specs.add(m[1]!); }
  if (!specs.size) continue;
  const o = owner(dir);
  if (!o) continue;
  for (const s of specs) {
    const name = s.startsWith("@") ? s.split("/").slice(0, 2).join("/") : s.split("/")[0]!;
    if (name === o.name || o.deps.has(name)) continue;
    let resolves = true;
    try { Bun.resolveSync(s, dir); } catch { resolves = false; }
    const key = o.dir.slice(ROOT.length + 1) + "|" + name;
    const e = (out[key] ??= { pkg: o.dir.slice(ROOT.length + 1) || ".", kind: resolves ? "undeclared-resolves" : "undeclared-unresolved", example: f.slice(ROOT.length + 1), count: 0 });
    e.count++;
  }
}
writeFileSync(process.argv[2], JSON.stringify(out, null, 1));
console.log(Object.keys(out).length, "undeclared pairs");
