/** 🔌 Registers plugin-artifact Rust crates as Cargo-manifest owners named by package, like the 709 registered siblings. Idempotent. */
import { readFileSync, renameSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const root = process.cwd(), dry = process.argv.includes("--dry");
const listFile = process.argv.find((a) => a.endsWith(".json") && !a.startsWith("--"))!;
const REGISTRY = join(root, "📋️build-owners.json");
const read = (p: string) => readFileSync(p, "utf8");
const text = read(REGISTRY), registry = JSON.parse(text);
if (text !== JSON.stringify(registry, null, 2) + "\n") throw Error("registry not canonical");
const roots = new Set<string>(), names = new Set<string>(registry.owners.map((o: any) => o.name));
for (const o of registry.owners) for (const r of [o.root, ...(o.nativeRoots ?? [])]) roots.add(r);
const rows: any[] = [];
for (const dir of JSON.parse(read(listFile)).missing.filter((m: string) => m.startsWith("✏️s/") && !roots.has(m))) {
  const name = (Bun.TOML.parse(read(join(root, dir, "Cargo.toml"))) as any).package.name as string;
  if (names.has(name)) throw Error("owner name collision: " + name + " @ " + dir);
  names.add(name);
  rows.push({ name, root: dir, manifest: `${dir}/Cargo.toml`, packages: [] });
}
console.log("rows", rows.length);
if (dry) process.exit(0);
if (read(REGISTRY) !== text) throw Error("registry changed; rerun");
const owners: any[] = registry.owners;
for (const row of rows.sort((a, b) => (a.name < b.name ? -1 : 1))) {
  let at = owners.findIndex((o) => o.name > row.name);
  if (at < 0) at = owners.length;
  owners.splice(at, 0, row);
}
const tmp = REGISTRY + ".owners-tmp";
writeFileSync(tmp, JSON.stringify(registry, null, 2) + "\n");
renameSync(tmp, REGISTRY);
console.log("applied");
