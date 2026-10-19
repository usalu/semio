import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad";
const ROOT = process.cwd();
const files = readFileSync(S + "/changed-files.txt", "utf8").split("\n").filter(Boolean);
const need = new Map<string, Set<string>>();
const re = /(?:from\s*|import\s*\(\s*|require\(\s*|import\s+)["'](@semio-tech\/[^"'/]+|repo-vscode)[^"']*["']/gu;
const depsCache = new Map<string, Set<string>>();
const depsOf = (dir: string): Set<string> | undefined => {
  if (depsCache.has(dir)) return depsCache.get(dir);
  const pj = resolve(ROOT, dir, "package.json");
  if (!existsSync(pj)) return undefined;
  const j = JSON.parse(readFileSync(pj, "utf8"));
  const s = new Set<string>([j.name, ...Object.keys(j.dependencies ?? {}), ...Object.keys(j.devDependencies ?? {}), ...Object.keys(j.peerDependencies ?? {}), ...Object.keys(j.optionalDependencies ?? {})]);
  depsCache.set(dir, s); return s;
};
const boundDir = (f: string): string => { let d = dirname(f); while (d !== "." && d !== "/") { if (existsSync(resolve(ROOT, d, "node_modules"))) return d; d = dirname(d); } return "."; };
const unbound: string[] = [];
for (const f of files) {
  let t: string; try { t = readFileSync(f, "utf8"); } catch { continue; }
  const b = readFileSync(resolve(S, "before", f), "utf8");
  const before = new Set([...b.matchAll(re)].map(m => m[1]!));
  for (const m of t.matchAll(re)) {
    const name = m[1]!; if (before.has(name)) continue;
    const d = boundDir(f);
    const deps = depsOf(d);
    const key = d + "/package.json";
    if (!deps) { unbound.push(`${f} (${d}) ${name}`); continue; }
    if (!deps.has(name)) (need.get(key) ?? need.set(key, new Set()).get(key)!).add(name);
  }
}
console.log(JSON.stringify({ needDeclarations: Object.fromEntries([...need].map(([k, v]) => [k, [...v].sort()])), noManifestAtBoundDir: unbound.slice(0, 20), unboundCount: unbound.length }, null, 1));
