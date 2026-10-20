/**
 * 🔬 Static evaluation of the dependency-boundary policy (every forbidden rule except ncpr/circular) without dependency-cruiser.
 *
 *   bun scan.ts <policy.json> <out.tsv> [--roots a,b,c]
 */
import { existsSync, readFileSync, readdirSync, realpathSync, statSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, relative, resolve } from "node:path";
import ts from "typescript";

const ROOT = "/Users/ueli/Documents/semio";
const [policyFile, outFile] = [process.argv[2]!, process.argv[3]!];
const rootsArg = process.argv.indexOf("--roots");
const only = rootsArg > 0 ? process.argv[rootsArg + 1]!.split(",") : [];
const policy = JSON.parse(readFileSync(policyFile, "utf8"));
const arr = (v: unknown): string[] => (v === undefined ? [] : Array.isArray(v) ? v : [v as string]);
const rx = (v: unknown): RegExp[] => arr(v).map(s => new RegExp(s, "u"));
const EXCLUDE = rx(policy.options.exclude.path);
const rules = policy.forbidden
  .filter((r: any) => !/^no-cross-package-relative/u.test(r.name) && r.name !== "no-circular" && !r.to.circular && !/^not-to-unlisted|^no-escaping/u.test(r.name))
  .map((r: any) => ({
    name: r.name,
    fromPath: rx(r.from?.path),
    fromNot: rx(r.from?.pathNot),
    toPath: rx(r.to?.path),
    toNot: rx(r.to?.pathNot),
    types: arr(r.to?.dependencyTypes),
    typesNot: arr(r.to?.dependencyTypesNot),
  }));
const any = (rs: RegExp[], s: string) => rs.some(r => r.test(s));

const listCache = outFile.replace(/[^/]*$/u, "files.cache");
let all: string[];
if (!process.argv.includes("--refresh") && existsSync(listCache)) all = readFileSync(listCache, "utf8").split("\0").filter(f => f && existsSync(resolve(ROOT, f)));
else {
  const listed = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: ROOT, maxBuffer: 1 << 30, encoding: "utf8" });
  if (listed.status !== 0) throw Error("git ls-files failed");
  all = listed.stdout.split("\0").filter(Boolean);
  writeFileSync(listCache, all.join("\0"));
}
const known = new Set(all);
const SRC = /\.(ts|tsx|mts|cts|js|jsx|mjs|cjs)$/u;
const files = all.filter(f => SRC.test(f) && !f.endsWith(".d.ts") && !any(EXCLUDE, f) && (only.length === 0 || only.some(o => f === o || f.startsWith(o + "/"))));

const nm = resolve(ROOT, "node_modules/@semio-tech");
const pkgs = new Map<string, { dir: string; exports: any }>();
for (const e of readdirSync(nm)) {
  try {
    const dir = realpathSync(resolve(nm, e));
    const j = JSON.parse(readFileSync(resolve(dir, "package.json"), "utf8"));
    pkgs.set(`@semio-tech/${e}`, { dir, exports: j.exports });
  } catch {}
}
const CONDITIONS = ["semio-source", "import", "require", "node", "default"];
const pick = (v: any): string | undefined => {
  if (typeof v === "string") return v;
  if (v && typeof v === "object" && !Array.isArray(v)) for (const [k, x] of Object.entries(v)) if (CONDITIONS.includes(k)) { const r = pick(x); if (r) return r; }
  return undefined;
};
const relRoot = (abs: string) => relative(ROOT, abs).split("\\").join("/");
const exts = ["", ".ts", ".tsx", ".mts", ".js", ".mjs", ".cjs", ".json"];
const fileOf = (abs: string): string | undefined => {
  const r = relRoot(abs);
  for (const e of exts) if (known.has(r + e) || (e === "" && existsSync(abs) && statSync(abs).isFile())) return r + e;
  const stem = r.replace(/\.(js|mjs|cjs)$/u, "");
  for (const e of [".ts", ".tsx", ".mts"]) if (known.has(stem + e)) return stem + e;
  for (const d of ["/index.ts", "/index.js", "/🟦️.ts", "/🟦️.tsx"]) if (known.has(r + d)) return r + d;
  return undefined;
};
const resolveBare = (spec: string): string | undefined => {
  const clean = spec.split("?")[0]!;
  const parts = clean.split("/");
  const name = parts.slice(0, 2).join("/");
  const pkg = pkgs.get(name);
  if (!pkg) return undefined;
  const sub = "." + clean.slice(name.length);
  const table = pkg.exports && typeof pkg.exports === "object" && Object.keys(pkg.exports).some(k => k.startsWith(".")) ? pkg.exports : { ".": pkg.exports };
  let t = pick(table[sub]);
  if (!t) for (const [k, v] of Object.entries<any>(table)) { const i = k.indexOf("*"); if (i < 0 || !sub.startsWith(k.slice(0, i)) || !sub.endsWith(k.slice(i + 1))) continue; const mid = sub.slice(i, sub.length - (k.length - i - 1)); const tv = pick(v); if (tv) { t = tv.replace("*", mid); break; } }
  if (!t) return undefined;
  try { return relRoot(realpathSync(resolve(pkg.dir, t))); } catch { return undefined; }
};
const resolveSpec = (from: string, spec: string): { to: string; types: string[] } | undefined => {
  const clean = spec.split("?")[0]!;
  if (clean.startsWith(".")) { const to = fileOf(resolve(ROOT, dirname(from), clean)); return to ? { to, types: ["local"] } : undefined; }
  if (clean.startsWith("@semio-tech/")) { const to = resolveBare(clean); return to ? { to, types: ["aliased", "aliased-workspace", "local"] } : undefined; }
  return undefined;
};

const out: string[] = [];
const graph: string[] = [];
const counts = new Map<string, number>();
for (const f of files) {
  let text: string;
  try { text = readFileSync(resolve(ROOT, f), "utf8"); } catch { continue; }
  const seen = new Set<string>();
  for (const imp of ts.preProcessFile(text, true, true).importedFiles) {
    const r = resolveSpec(f, imp.fileName);
    if (!r || any(EXCLUDE, r.to)) continue;
    const key = imp.fileName + "\0" + r.to;
    if (seen.has(key)) continue;
    seen.add(key);
    graph.push(f + "\t" + r.to);
    for (const rule of rules) {
      if (!any(rule.fromPath, f) || any(rule.fromNot, f)) continue;
      if ((rule.toPath.length && !any(rule.toPath, r.to)) || any(rule.toNot, r.to)) continue;
      if (rule.types.length && !rule.types.some(t => r.types.includes(t))) continue;
      if (rule.typesNot.some(t => r.types.includes(t))) continue;
      out.push([rule.name, f, r.to, imp.fileName, r.types.join(",")].join("\t"));
      counts.set(rule.name, (counts.get(rule.name) ?? 0) + 1);
    }
  }
}
writeFileSync(outFile, out.join("\n") + "\n");
writeFileSync(outFile.replace(/\.tsv$/u, ".graph"), graph.join("\n") + "\n");
const fam = new Map<string, number>();
for (const [n, c] of counts) { const k = n.replace(/^(no-cross-technology|plugin-no-extension-or-artifact|no-cross-plugin)-.*/u, "$1"); fam.set(k, (fam.get(k) ?? 0) + c); }
console.log(`files ${files.length}`);
for (const [k, c] of [...fam].sort((a, b) => b[1] - a[1])) console.log(String(c).padStart(7), k);
