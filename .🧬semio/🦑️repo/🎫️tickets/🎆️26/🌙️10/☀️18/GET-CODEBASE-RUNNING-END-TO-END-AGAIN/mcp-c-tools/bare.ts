import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, resolve, join } from "node:path";
const root = resolve(process.argv[2]!); const tsconfig = JSON.parse(readFileSync(join(root, "tsconfig.json"), "utf8"));
const paths = Object.keys(tsconfig.compilerOptions.paths ?? {});
const aliased = (s: string) => paths.some((p) => { const i = p.indexOf("*"); return i < 0 ? p === s : s.startsWith(p.slice(0, i)) && s.endsWith(p.slice(i + 1)); });
const files: string[] = []; const walk = (d: string) => { for (const e of readdirSync(d)) { if (e === "node_modules") continue; const p = join(d, e); statSync(p).isDirectory() ? walk(p) : /\.tsx?$/.test(p) && files.push(p); } };
walk(join(root, "app"));
const seen = new Set<string>(); const bare = new Map<string, string[]>();
const re = /(?:from\s*|import\s*\(\s*|import\s+)["']([^"']+)["']/g;
const visit = (f: string) => { if (seen.has(f)) return; seen.add(f); for (const m of readFileSync(f, "utf8").matchAll(re)) { const s = m[1]!; if (s.startsWith("node:")) continue;
  if (s.startsWith(".")) { const t = resolve(dirname(f), s); const r = [t, t + ".ts", t + ".tsx", t + "/index.ts"].find((x) => existsSync(x) && statSync(x).isFile()); if (r) visit(r); continue; }
  if (aliased(s)) { const key = Object.keys(tsconfig.compilerOptions.paths).find((p) => p === s); if (key) { const t = resolve(root, tsconfig.compilerOptions.paths[key][0]); if (existsSync(t)) visit(t); } continue; }
  const pkg = s.startsWith("@") ? s.split("/").slice(0, 2).join("/") : s.split("/")[0]!; (bare.get(pkg) ?? bare.set(pkg, []).get(pkg)!).push(f.slice(-60)); } };
files.forEach(visit);
console.log("visited", seen.size); for (const [k, v] of bare) console.log(k, v.length, v[0]);
