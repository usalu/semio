/** 🧾 Feature-aware manifest reader (Bun TOML): per crate dependency flags (kind, optional, features, default-features, target cfg), `[features]` table and the owning workspace's `[workspace.dependencies]` flags. Usage: `bun manifests.ts dirs.json out.json`. Read-only. */
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const dirs: string[] = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
type Any = Record<string, any>;
const cache = new Map<string, Any | null>();
const load = (d: string): Any | null => {
  if (cache.has(d)) return cache.get(d)!;
  const p = join(ROOT, d, "Cargo.toml");
  let v: Any | null = null;
  try { v = existsSync(p) ? (Bun.TOML.parse(readFileSync(p, "utf8")) as Any) : null; } catch { v = null; }
  cache.set(d, v);
  return v;
};
const owner = (d: string, doc: Any): string => {
  const w = doc.package?.workspace;
  if (typeof w === "string") return normalize(join(d, w));
  let p = d;
  while (p && p !== ".") {
    p = dirname(p);
    const t = load(p);
    if (t && t.workspace) return p;
  }
  return ".";
};
const KIND: Record<string, string> = { dependencies: "normal", "build-dependencies": "build", "dev-dependencies": "dev" };
const out: Any = {};
let bad = 0;
for (const d of dirs) {
  const doc = load(d);
  if (!doc) { bad++; continue; }
  const root = owner(d, doc);
  const wdoc = load(root);
  const wdeps: Any = wdoc?.workspace?.dependencies ?? {};
  const deps: Any[] = [];
  const push = (kind: string, target: string | null, table: Any | undefined) => {
    for (const [key, raw] of Object.entries(table ?? {})) {
      const v: Any = typeof raw === "string" ? { version: raw } : (raw as Any);
      let base: Any = {};
      if (v.workspace === true) {
        const w = wdeps[key];
        base = typeof w === "string" ? {} : (w ?? {});
      }
      const features = [...(base.features ?? []), ...(v.features ?? [])];
      const dflt = v.workspace === true ? (v["default-features"] ?? true) && (base["default-features"] ?? base["default_features"] ?? true) : (v["default-features"] ?? v["default_features"] ?? true);
      deps.push({ key, pkg: v.package ?? base.package ?? key, kind, target, optional: v.optional === true, features, defaultFeatures: dflt, ws: v.workspace === true, path: v.path ?? null });
    }
  };
  for (const [k, kind] of Object.entries(KIND)) push(kind, null, doc[k]);
  for (const [cfg, t] of Object.entries((doc.target ?? {}) as Any)) for (const [k, kind] of Object.entries(KIND)) push(kind, cfg, (t as Any)[k]);
  out[d] = { wsroot: root, name: doc.package?.name ?? null, features: doc.features ?? {}, lib: doc.lib?.name ?? null, deps };
}
const roots: Any = {};
for (const d of Object.keys(out)) {
  const r = out[d].wsroot;
  if (roots[r]) continue;
  const w = load(r)?.workspace?.dependencies ?? {};
  roots[r] = Object.fromEntries(Object.entries(w as Any).map(([k, v]) => [k, typeof v === "string" ? { version: v } : { path: (v as Any).path ?? null, package: (v as Any).package ?? null, features: (v as Any).features ?? [], optional: (v as Any).optional === true, defaultFeatures: (v as Any)["default-features"] ?? true }]));
}
writeFileSync(process.argv[3]!, JSON.stringify(out));
writeFileSync(process.argv[3]!.replace(/\.json$/, "-roots.json"), JSON.stringify(roots));
console.log("manifests", Object.keys(out).length, "unparsed", bad);
