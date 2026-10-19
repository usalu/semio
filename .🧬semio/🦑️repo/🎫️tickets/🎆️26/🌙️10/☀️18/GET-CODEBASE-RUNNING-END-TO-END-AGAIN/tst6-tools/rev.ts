import { readdirSync, readFileSync, writeFileSync, existsSync, realpathSync } from "node:fs";
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
const fileSet = new Set(files);
const rev = new Map<string, Set<string>>();
const cache = new Map<string, string | null>();
for (const f of files) {
  let text: string;
  try { text = readFileSync(f, "utf8"); } catch { continue; }
  if (text.length > 4_000_000) continue;
  const dir = dirname(f);
  const specs = new Set<string>();
  for (const p of PATTERNS) { p.lastIndex = 0; let m; while ((m = p.exec(text))) specs.add(m[1]!); }
  for (const s of specs) {
    if (/[${}*\s]/.test(s) || s.startsWith("node:") || s.startsWith("bun:")) continue;
    const key = dir + "\0" + s;
    let r = cache.get(key);
    if (r === undefined) { try { r = Bun.resolveSync(s, dir); } catch { r = null; } cache.set(key, r); }
    if (!r) continue;
    let target = r;
    if (!fileSet.has(target)) { try { target = realpathSync(r); } catch {} }
    if (!fileSet.has(target)) continue;
    let set = rev.get(target); if (!set) rev.set(target, set = new Set()); set.add(f);
  }
}
const obj: Record<string, string[]> = {};
for (const [k, v] of rev) obj[k.slice(ROOT.length + 1)] = [...v].map(x => x.slice(ROOT.length + 1));
writeFileSync(process.argv[2], JSON.stringify(obj));
console.log("edges targets", rev.size);
