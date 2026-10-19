// 🔎️ uicarve: every manifest dependency edge onto a package (resolving workspace=true entries): bun deps.ts <package> [out.json]
import {existsSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
const root = "/Users/ueli/Documents/semio";
const target = process.argv[2]!;
const manifests = readFileSync("/Users/ueli/.cache/uicarve/manifests.txt", "utf8").split("\n").filter(Boolean).map(p => join(root, p));
const cache = new Map<string, any>();
const parse = (m: string) => { if (!cache.has(m)) { try { cache.set(m, Bun.TOML.parse(readFileSync(m, "utf8"))); } catch { cache.set(m, null); } } return cache.get(m); };
const wsOf = (m: string): string | undefined => {
  const t = parse(m);
  if (t?.package?.workspace) { const w = resolve(dirname(m), t.package.workspace, "Cargo.toml"); if (existsSync(w)) return w; }
  let d = dirname(m);
  for (;;) { const c = join(d, "Cargo.toml"); if (existsSync(c) && parse(c)?.workspace) return c; const up = dirname(d); if (up === d) return undefined; d = up; }
};
const out: any[] = [];
for (const m of manifests) {
  const t = parse(m); if (!t) continue;
  const ws = t.workspace ? m : undefined;
  const wsDeps = () => { const w = wsOf(m); return w ? parse(w)?.workspace?.dependencies ?? {} : {}; };
  const scan = (tbl: any, kind: string, tgt = "") => {
    for (const [k, v] of Object.entries(tbl ?? {}) as [string, any][]) {
      let pkg = typeof v === "object" && v.package ? v.package : k;
      let spec = typeof v === "object" ? v : {};
      if (typeof v === "object" && v.workspace === true && !v.package) { const w = wsDeps()[k]; if (w && typeof w === "object" && w.package) pkg = w.package; }
      if (pkg === target) out.push({manifest: m.slice(root.length + 1), alias: k, kind, tgt, spec});
    }
  };
  for (const kind of ["dependencies", "dev-dependencies", "build-dependencies"]) { scan(t[kind], kind); for (const [tg, tt] of Object.entries(t.target ?? {}) as [string, any][]) scan(tt[kind], kind, tg); }
  scan(t.workspace?.dependencies, "ws-dependencies");
}
if (process.argv[3]) writeFileSync(process.argv[3], JSON.stringify(out, null, 1));
const count = (f: (r: any) => string) => { const c = new Map<string, number>(); for (const r of out) c.set(f(r), (c.get(f(r)) ?? 0) + 1); return [...c.entries()].sort((a, b) => b[1] - a[1]); };
console.log(out.length, new Set(out.map(r => r.manifest)).size);
console.log(count(r => r.alias));
console.log(count(r => r.kind));
