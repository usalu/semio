// 🧬 Generates the crate-boundary corpus of the carved db family from the staged manifests (design input: the layered DAG).
import {readFileSync, writeFileSync} from "node:fs";
import {resolve, dirname} from "node:path";
const SPD = process.argv[2]!, OUT = process.argv[3]!;
const STAGE = SPD + "/stage/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db";
const crates: {lib: string; root: string}[] = JSON.parse(readFileSync(SPD + "/crates.json", "utf8"));
type Manifest = {package: {name: string}; dependencies?: Record<string, {package?: string; path?: string}>; target?: Record<string, {dependencies?: Record<string, {package?: string; path?: string}>}>};
const rows = crates.map(c => {
  const manifest = c.root.replace(/\/🦀️\.rs$/u, "").replace(/^🦀️\.rs$/u, "") + "/📦️packages/🦀️rust/Cargo.toml";
  const path = (manifest.startsWith("/") ? manifest.slice(1) : manifest);
  const own = Bun.TOML.parse(readFileSync(STAGE + "/" + path, "utf8")) as unknown as Manifest;
  const declared = Object.assign({}, own.dependencies, ...Object.values(own.target ?? {}).map(t => t.dependencies ?? {}));
  const dependencies = Object.entries(declared).map(([alias, edge]) => edge.package ?? alias).sort();
  return {package: own.package.name, manifest: path, entry: c.root, dependencies};
});
const byName = new Map(rows.map(r => [r.package, r]));
const dependents = new Map<string, Set<string>>();
for (const r of rows) dependents.set(r.package, new Set());
const reach = new Map<string, Set<string>>();
const down = (n: string): Set<string> => {
  if (reach.has(n)) return reach.get(n)!;
  const s = new Set<string>(); reach.set(n, s);
  for (const d of byName.get(n)!.dependencies) if (byName.has(d)) { s.add(d); for (const x of down(d)) s.add(x); }
  return s;
};
for (const r of rows) for (const d of down(r.package)) dependents.get(d)!.add(r.package);
const outside = ["semio-framework-plugin", "semio-framework-ui", "semio-framework-replication-hub"];
const fixture = {
  version: 1,
  crates: rows.map(r => ({...r, forbiddenDependencies: [...new Set([...dependents.get(r.package)!, ...(r.package === "semio-framework-os-kernel-db" ? [] : ["semio-framework-os-kernel-db"]), ...outside])].filter(x => x !== r.package).sort()}))
};
writeFileSync(OUT, JSON.stringify(fixture, null, 2) + "\n");
console.log("rows", rows.length);
