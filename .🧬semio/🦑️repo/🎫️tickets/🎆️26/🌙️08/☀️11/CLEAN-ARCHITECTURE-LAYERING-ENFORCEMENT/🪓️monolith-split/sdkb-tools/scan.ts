/** 🔭️ Scans plugin TS sources for imports that resolve into framework internals (rule `plugins-framework-sdk-only`). Read-only.
 * Usage: bun scan.ts <files.txt> <out.json>
 */
import { readFileSync, writeFileSync, existsSync, realpathSync, readdirSync, statSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const [filesList, outPath] = process.argv.slice(2);

type Pkg = { name: string; dir: string; exports: Record<string, unknown> | null; main?: string };
const pkgs = new Map<string, Pkg>();
const scope = join(ROOT, "node_modules/@semio-tech");
for (const entry of readdirSync(scope)) {
  try {
    const dir = realpathSync(join(scope, entry));
    const manifest = JSON.parse(readFileSync(join(dir, "package.json"), "utf8"));
    const rel = relative(ROOT, dir);
    pkgs.set(manifest.name ?? `@semio-tech/${entry}`, { name: manifest.name, dir: rel, exports: manifest.exports ?? null, main: manifest.main });
  } catch {}
}

function exportTarget(value: unknown): string | null {
  if (typeof value === "string") return value;
  if (Array.isArray(value)) { for (const v of value) { const r = exportTarget(v); if (r) return r; } return null; }
  if (value && typeof value === "object") {
    const o = value as Record<string, unknown>;
    for (const key of ["semio-source", "import", "require", "node", "default"]) if (key in o) { const r = exportTarget(o[key]); if (r) return r; }
  }
  return null;
}

function resolvePackage(spec: string): { pkg: Pkg; subpath: string; target: string | null } | null {
  const m = spec.match(/^(@semio-tech\/[^/]+)(\/.*)?$/);
  if (!m) return null;
  const pkg = pkgs.get(m[1]);
  if (!pkg) return null;
  const subpath = m[2] ? "." + m[2] : ".";
  let target: string | null = null;
  if (pkg.exports) {
    const map = pkg.exports as Record<string, unknown>;
    const keys = Object.keys(map);
    const isMap = keys.some(k => k.startsWith("."));
    const table = isMap ? map : { ".": map };
    if (subpath in table) target = exportTarget(table[subpath]);
    else for (const k of Object.keys(table)) { if (k.includes("*")) { const [a, b] = k.split("*"); if (subpath.startsWith(a) && subpath.endsWith(b)) { const t = exportTarget(table[k]); if (t) target = t.replace("*", subpath.slice(a.length, subpath.length - b.length)); } } }
  } else if (subpath === "." && pkg.main) target = pkg.main;
  return { pkg, subpath, target: target ? join(pkg.dir, target) : null };
}

const SPEC = /(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s+|\bmock\(\s*|\brequire\(\s*|\bimportActual<[^>]*>\(\s*|\bimportActual\(\s*|\bnew URL\(\s*)(["'`])([^"'`\n]+)\1/g;

function normalize(p: string): string { return p.normalize("NFC"); }

function existsFile(p: string): boolean { try { return statSync(p).isFile(); } catch { return false; } }

function resolveRelative(fromFile: string, spec: string): string | null {
  const base = resolve(ROOT, dirname(fromFile), spec.replace(/\?.*$/, ""));
  for (const cand of [base, base + ".ts", base + ".tsx", base + "/🟦️.ts", base + "/🟦️.tsx"]) if (existsFile(cand)) return relative(ROOT, cand);
  return null;
}

const files = readFileSync(filesList, "utf8").split("\n").filter(Boolean);
const edges: any[] = [];
for (const f of files) {
  const abs = f.startsWith("/") ? f : join(ROOT, f);
  const rel = relative(ROOT, abs);
  let src: string;
  try { src = readFileSync(abs, "utf8"); } catch { continue; }
  SPEC.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = SPEC.exec(src))) {
    const spec = m[2];
    const line = src.slice(0, m.index).split("\n").length;
    if (spec.startsWith(".")) {
      const target = resolveRelative(rel, spec);
      if (target && target.startsWith("🧰️framework/")) edges.push({ file: rel, line, spec, kind: "relative", target });
    } else if (spec.startsWith("@semio-tech/")) {
      const r = resolvePackage(spec);
      if (r && r.pkg.dir.startsWith("🧰️framework/")) edges.push({ file: rel, line, spec, kind: "package", pkg: r.pkg.name, subpath: r.subpath, target: r.target });
    }
  }
}
writeFileSync(outPath, JSON.stringify(edges, null, 1));
console.log(`files=${files.length} frameworkEdges=${edges.length}`);
