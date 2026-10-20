// 🔬 cargo metadata (the real Cargo) against the engine on the dep-info overlay: owning workspace, path dependencies by kind, inherited package fields and inherited dependency requirements.
import {readFileSync} from "node:fs";
import {relative, dirname} from "node:path";
import {readCargoResolutionPolicyV1, owningWorkspaceV1, cargoDependenciesV1, inheritedSliceV1} from "/private/tmp/gk/sb/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution/🟦️.ts";
const OV = "/private/tmp/gk/di/overlay", meta = JSON.parse(readFileSync("/private/tmp/gk/di/metadata.json", "utf8"));
const cache = new Map<string, any>();
const host = {read: async (p: string) => { if (!cache.has(p)) { try { cache.set(p, Bun.TOML.parse(readFileSync(`${OV}/${p}`, "utf8"))); } catch { cache.set(p, null); } } return cache.get(p); }};
let packages = 0, pathDeps = 0, wsMismatch: string[] = [], depMismatch: string[] = [], fieldMismatch: string[] = [], reqMismatch: string[] = [];
const kindName = (k: string) => (k === "dev-dependencies" ? "dev" : k === "build-dependencies" ? "build" : null);
for (const pkg of meta.packages) {
  packages++;
  const dir = relative(OV, dirname(pkg.manifest_path)), manifest = await host.read(`${dir}/Cargo.toml`), ws = await owningWorkspaceV1(host, dir);
  if (relative(OV, meta.workspace_root) !== (ws?.root ?? "")) wsMismatch.push(dir);
  const rows = cargoDependenciesV1(dir, manifest, ws, true), mine = new Set(rows.filter((r) => r.path !== undefined).map((r) => `${r.alias}|${kindName(r.kind)}|${r.path}`));
  const real = new Set<string>(pkg.dependencies.filter((d: any) => d.path).map((d: any) => `${d.rename ?? d.name}|${d.kind}|${relative(OV, d.path)}`));
  const norm = (s: Set<string>) => [...s].map((x) => x.split("|")).map(([a, k, p]) => `${a}|${k}|${p}`).sort();
  // cargo reports the package name (not the alias) in `name` unless renamed; compare by (kind, path) multiset
  const key = (s: Set<string>) => [...s].map((x) => x.split("|").slice(1).join("|")).sort();
  pathDeps += real.size;
  if (JSON.stringify(key(mine)) !== JSON.stringify(key(real))) depMismatch.push(dir);
  const slice: any = inheritedSliceV1(manifest, (ws?.manifest.workspace as any) ?? null, rows);
  const wsPkg: any = (ws?.manifest.workspace as any)?.package ?? {};
  if (slice.package.edition !== undefined && slice.package.edition !== pkg.edition) fieldMismatch.push(`${dir} edition`);
  if (slice.package.version !== undefined && slice.package.version !== pkg.version) fieldMismatch.push(`${dir} version`);
  if (slice.package["rust-version"] !== undefined && slice.package["rust-version"] !== pkg.rust_version) fieldMismatch.push(`${dir} rust-version`);
  void wsPkg;
  for (const [tableName, kind, alias, entry] of slice.dependencies as [string, string, string, any][]) {
    if (tableName !== "") continue;
    const want = typeof entry === "string" ? entry : entry?.version;
    const found = pkg.dependencies.find((d: any) => (d.rename ?? d.name) === alias && d.kind === kindName(kind));
    if (found && want && found.req.replace(/^\^/, "") !== want && !found.req.includes(want)) reqMismatch.push(`${dir} ${alias} ${found.req} vs ${want}`);
  }
}
console.log(JSON.stringify({packages, pathDeps, wsMismatch, depMismatch, fieldMismatch, reqMismatch}, null, 1));
