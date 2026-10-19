import { readdirSync, readFileSync, writeFileSync } from "node:fs";
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
  /\b(?:resolveSync|require\.resolve|import\.meta\.resolve)\s*\(\s*["']([^"'\n]+)["']/g,
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
const bare = (s: string) => !s.startsWith(".") && !s.startsWith("/") && !s.startsWith("node:") && !s.startsWith("bun:") && !/^[a-z]+:/.test(s) && !s.startsWith("#");
const cache = new Map<string, boolean>();
const bad: Record<string, string[]> = {};
let scanned = 0;
for (const f of files) {
  let text: string;
  try { text = readFileSync(f, "utf8"); } catch { continue; }
  if (text.length > 4_000_000) continue;
  scanned++;
  const dir = dirname(f);
  const specs = new Set<string>();
  for (const p of PATTERNS) { p.lastIndex = 0; let m; while ((m = p.exec(text))) if (bare(m[1])) specs.add(m[1]); }
  for (const s of specs) {
    if (/[${}*\s]/.test(s)) continue;
    const key = dir + "\0" + s;
    let ok = cache.get(key);
    if (ok === undefined) { try { Bun.resolveSync(s, dir); ok = true; } catch { ok = false; } cache.set(key, ok); }
    if (!ok) (bad[s] ??= []).push(f.slice(ROOT.length + 1));
  }
}
writeFileSync(process.argv[2], JSON.stringify({ scanned, bad }, null, 1));
console.log(`scanned ${scanned}/${files.length}, unresolved specifiers ${Object.keys(bad).length}`);
