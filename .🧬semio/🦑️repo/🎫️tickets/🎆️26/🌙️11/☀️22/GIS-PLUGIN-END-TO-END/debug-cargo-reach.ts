import { discoverCargoWorkspaces, cargoWorkspaceMembers, cargoWorkspaceForManifest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import { readFileSync } from "node:fs"; import { resolve, relative } from "node:path";
const root = process.cwd(), name = "semio-s-artifact-gis-gismap";
const doc = (m: string) => Bun.TOML.parse(readFileSync(resolve(root, m), "utf8")) as any;
const scope = discoverCargoWorkspaces(root).find(s => s.directory === ".")!;
const initial = cargoWorkspaceMembers(root, scope);
console.log("[DEBUG] root members", initial.length);
const parent = new Map<string, string>(); const reach = new Set(initial.map(p => p.manifest));
for (const m of reach) {
  const cur = doc(m); const pkgDir = resolve(root, m, "..");
  const owner = cargoWorkspaceForManifest(root, m); const auth = doc(owner.manifest).workspace;
  for (const g of [cur, ...Object.values<any>(cur.target ?? {})]) for (const kind of ["dependencies", "dev-dependencies", "build-dependencies"]) for (const [alias, v] of Object.entries<any>(g[kind] ?? {})) {
    const dep = v?.workspace ? auth.dependencies?.[alias] : v; if (typeof dep?.path !== "string") continue;
    const base = v.workspace ? resolve(root, owner.directory) : pkgDir;
    const l = relative(root, resolve(base, dep.path, "Cargo.toml")); if (!reach.has(l)) { parent.set(l, m); reach.add(l); }
  }
}
const target = [...reach].filter(m => { try { return doc(m).package?.name === name; } catch { return false; } });
console.log("[DEBUG] reach", reach.size, "candidates", target);
for (let m = target[0]; m; m = parent.get(m)!) console.log("[DEBUG] chain", m);
