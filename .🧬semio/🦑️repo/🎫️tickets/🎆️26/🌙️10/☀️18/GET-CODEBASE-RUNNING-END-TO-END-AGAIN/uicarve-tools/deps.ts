// 🔎️ uicarve: every manifest dependency edge onto a package: bun deps.ts <package> [out.json]
import {readdirSync, readFileSync, writeFileSync} from "node:fs";
import {join} from "node:path";
const root = "/Users/ueli/Documents/semio";
const target = process.argv[2]!;
const skip = new Set(["node_modules", "target", ".git", "dist", ".cache"]);
const manifests = readFileSync("/Users/ueli/.cache/uicarve/manifests.txt", "utf8").split("\n").filter(Boolean).map(p => join(root, p));
const out: any[] = [];
for (const m of manifests) {
  let t: any; try { t = Bun.TOML.parse(readFileSync(m, "utf8")); } catch { continue; }
  const scan = (tbl: any, kind: string, tgt = "") => { for (const [k, v] of Object.entries(tbl ?? {}) as [string, any][]) { const pkg = typeof v === "object" && v.package ? v.package : k; if (pkg === target) out.push({manifest: m.slice(root.length + 1), alias: k, kind, tgt, spec: typeof v === "object" ? v : {}}); } };
  for (const kind of ["dependencies", "dev-dependencies", "build-dependencies"]) { scan(t[kind], kind); for (const [tg, tt] of Object.entries(t.target ?? {}) as [string, any][]) scan(tt[kind], kind, tg); }
  scan(t.workspace?.dependencies, "ws-dependencies");
}
if (process.argv[3]) writeFileSync(process.argv[3], JSON.stringify(out, null, 1));
const count = (f: (r: any) => string) => { const c = new Map<string, number>(); for (const r of out) c.set(f(r), (c.get(f(r)) ?? 0) + 1); return [...c.entries()].sort((a, b) => b[1] - a[1]); };
console.log(out.length, new Set(out.map(r => r.manifest)).size);
console.log(count(r => JSON.stringify((r.spec.features ?? []).slice().sort())));
console.log(count(r => r.alias));
console.log(count(r => r.kind + (r.tgt ? " " + r.tgt : "")));
console.log(count(r => r.manifest.split("/")[0]));
