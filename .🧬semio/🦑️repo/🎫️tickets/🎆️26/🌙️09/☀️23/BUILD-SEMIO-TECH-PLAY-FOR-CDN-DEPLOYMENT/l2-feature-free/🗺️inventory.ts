/** 🗺️ Wave L2 inventory: every first-party Cargo feature (declared `[features]` rows and implicit optional-dependency features), who
 * enables it, and how many source cfgs read it. Usage: `bun 🗺️inventory.ts [--json] [--package <name>]`. */
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { execSync } from "node:child_process";

export const repoRoot = resolve(import.meta.dir, "../../../../../../../..");

/** 🧭️ Fleet A packages (peer-scope map §1, including the UI backends, plus every crate carrying Fleet A's
 * `[package.metadata.semio.rust-source-boundary]` block except the plugin crate this wave owns): listed with a classification, never edited. */
export const PEER_OWNED = [
  /^semio-framework-ui($|-)/, /^semio-framework-font-/, /^semio-framework-icon-contract$/, /^semio-framework-os-identity$/, /^semio-framework-os-history/,
  /^semio-framework-os-(channel|directory|directory-client|vcs|command|inference|pack)$/, /^semio-framework-pack-record-token$/,
  /^semio-framework-plugin-(component-receipt|host|interpreter|interpreter-runtime|describe|guest-contract)$/, /^semio-framework-value-retirement-owner$/,
  /^semio-framework-os-store-/, /^semio-framework-os-kernel$/, /^semio-framework-os-(run|mcp|renderer-wgpu)$/,
];

export type Feature = { name: string; implicit: boolean; enables: string[] };
export type Consumer = { manifest: string; section: string; features: string[]; defaultFeatures: boolean };
export type Package = { name: string; manifest: string; owner: string; peer: boolean; defaults: string[]; features: Feature[]; consumers: Consumer[]; cfgHits: Record<string, number> };

/** 📋️ Every first-party manifest, tracked or untracked, outside the repository cache. */
export function manifests(): string[] {
  return execSync("git ls-files -co --exclude-standard -- '*Cargo.toml' ':!.🧬semio' ':!**/node_modules/**' ':!**/target/**'", { cwd: repoRoot, encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }).trim().split("\n").filter(Boolean).filter((path) => existsSync(join(repoRoot, path)));
}

type Toml = Record<string, any>;
const dependencyTables = (toml: Toml): [string, Toml][] => {
  const rows: [string, Toml][] = [];
  for (const section of ["dependencies", "dev-dependencies", "build-dependencies"]) if (toml[section]) rows.push([section, toml[section]]);
  for (const [target, table] of Object.entries<Toml>(toml.target ?? {})) for (const section of ["dependencies", "dev-dependencies", "build-dependencies"]) if (table[section]) rows.push([`target.${target}.${section}`, table[section]]);
  if (toml.workspace?.dependencies) rows.push(["workspace.dependencies", toml.workspace.dependencies]);
  return rows;
};

/** 🌳️ The `.rs` files of one owner, without the trees of nested crate owners (directories with their own `📦️packages/🦀️rust/Cargo.toml`). */
export function crateSources(owner: string): string[] {
  const files = execSync(`rg --files --no-messages -g '*.rs' -g 'Cargo.toml' ${JSON.stringify(owner)}`, { cwd: repoRoot, encoding: "utf8", shell: "/bin/bash", maxBuffer: 1 << 28 }).trim().split("\n").filter(Boolean);
  const nested = files.filter((file) => file.endsWith("/📦️packages/🦀️rust/Cargo.toml")).map((file) => file.slice(0, file.lastIndexOf("/📦️packages/"))).filter((dir) => dir !== owner);
  return files.filter((file) => file.endsWith(".rs") && !nested.some((dir) => file.startsWith(`${dir}/`)));
}

/** 🧭️ Every `.rs` file a crate compiles on native (test) and wasm32 targets with the given features on: the mounted-module closure of its
 * lib, bin, test, bench and example roots, computed by the framework's own cfg-selecting source walker (mounts outside the owner included). */
export async function crateSourceClosure(manifest: string, features: readonly string[]): Promise<string[]> {
  const { readRustSourceClosureV1 } = await import("../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
  const toml = Bun.TOML.parse(readFileSync(join(repoRoot, manifest), "utf8")) as Toml;
  const directory = dirname(join(repoRoot, manifest));
  const roots = [toml.lib?.path ?? "src/lib.rs", ...["bin", "test", "bench", "example"].flatMap((kind) => (toml[kind] ?? []).map((row: Toml) => row.path).filter(Boolean))].map((path) => join(directory, path)).filter((path) => existsSync(path));
  const common = ["test", "debug_assertions", "panic=unwind", "target_pointer_width=64", "target_endian=little", ...features.map((feature) => `feature=${feature}`)];
  const snapshots = [[...common, "unix", "target_family=unix", "target_os=macos", "target_vendor=apple", "target_arch=aarch64", "target_env="], [...common, "target_family=wasm", "target_os=wasi", "target_arch=wasm32", "target_env=p2"]];
  const read = async (path: string): Promise<Uint8Array | null> => existsSync(path) ? new Uint8Array(readFileSync(path)) : null;
  const files = new Set<string>();
  for (const cfg of snapshots) for (const root of roots) {
    try { for (const input of await readRustSourceClosureV1([root], repoRoot, cfg, read, { manifestDirectory: directory, generated: () => {} })) if (input.kind === "source") files.add(input.path); }
    catch (error) { files.add(`!${(error as Error).message}`); }
  }
  return [...files].sort();
}

/** 🔎️ Builds the inventory of first-party packages that declare at least one feature. */
export function inventory(): Package[] {
  const parsed = manifests().map((manifest) => ({ manifest, toml: Bun.TOML.parse(readFileSync(join(repoRoot, manifest), "utf8")) as Toml }));
  const packages = new Map<string, Package>();
  for (const { manifest, toml } of parsed) {
    const name = toml.package?.name;
    if (!name) continue;
    const declared = Object.entries<string[]>(toml.features ?? {}).filter(([feature]) => feature !== "default").map(([feature, enables]) => ({ name: feature, implicit: false, enables }));
    const explicitDeps = new Set(Object.values<string[]>(toml.features ?? {}).flat().filter((entry) => entry.startsWith("dep:")).map((entry) => entry.slice(4)));
    const implicit = dependencyTables(toml).filter(([section]) => !section.startsWith("workspace")).flatMap(([, table]) => Object.entries<Toml>(table).filter(([, spec]) => typeof spec === "object" && spec.optional).map(([dep]) => dep)).filter((dep) => !explicitDeps.has(dep) && !declared.some((row) => row.name === dep)).map((dep) => ({ name: dep, implicit: true, enables: [`dep:${dep}`] }));
    const features = [...declared, ...implicit];
    if (!features.length && !(toml.features?.default ?? []).length) continue;
    const owner = manifest.includes("/📦️packages/") ? manifest.slice(0, manifest.lastIndexOf("/📦️packages/")) : dirname(manifest);
    packages.set(name, { name, manifest, owner, peer: PEER_OWNED.some((pattern) => pattern.test(name)) || (name !== "semio-framework-plugin" && toml.package?.metadata?.semio?.["rust-source-boundary"] !== undefined), defaults: toml.features?.default ?? [], features, consumers: [], cfgHits: {} });
  }
  for (const { manifest, toml } of parsed) {
    for (const [section, table] of dependencyTables(toml)) for (const [dep, spec] of Object.entries<Toml>(table)) {
      if (typeof spec !== "object") continue;
      const target = packages.get(spec.package ?? dep);
      if (!target) continue;
      const features = spec.features ?? [];
      if (features.length || spec["default-features"] === false) target.consumers.push({ manifest, section, features, defaultFeatures: spec["default-features"] !== false });
    }
    const localName = (dep: string): string => { for (const [, table] of dependencyTables(toml)) { const spec = table[dep]; if (spec && typeof spec === "object" && spec.package) return spec.package; } return dep; };
    for (const [feature, enables] of Object.entries<string[]>(toml.features ?? {})) for (const entry of enables) {
      const match = entry.match(/^([^/:]+)\??\/(.+)$/);
      if (!match) continue;
      const target = packages.get(localName(match[1]!));
      if (target) target.consumers.push({ manifest, section: `features.${feature}`, features: [match[2]!], defaultFeatures: true });
    }
  }
  for (const row of packages.values()) for (const feature of row.features) {
    row.cfgHits[feature.name] = crateSources(row.owner).reduce((sum, file) => sum + (readFileSync(join(repoRoot, file), "utf8").split(`feature = "${feature.name}"`).length - 1), 0);
  }
  return [...packages.values()].sort((a, b) => a.name.localeCompare(b.name));
}

if (import.meta.main) {
  const rows = inventory();
  const only = process.argv.includes("--package") ? process.argv[process.argv.indexOf("--package") + 1] : undefined;
  const selected = only ? rows.filter((row) => row.name === only) : rows;
  if (process.argv.includes("--json")) console.log(JSON.stringify(selected, null, 1));
  else for (const row of selected) {
    console.log(`${row.peer ? "PEER " : ""}${row.name}  [${row.manifest}]  default=${JSON.stringify(row.defaults)}`);
    for (const feature of row.features) {
      const users = row.consumers.filter((consumer) => consumer.features.includes(feature.name)).map((consumer) => `${consumer.section}@${consumer.manifest.split("/").slice(-5, -3).join("/")}`);
      console.log(`  ${feature.implicit ? "~" : "-"} ${feature.name} = ${JSON.stringify(feature.enables)} cfg=${row.cfgHits[feature.name]} users=${users.length}${users.length ? " " + users.slice(0, 4).join(", ") : ""}${row.defaults.includes(feature.name) ? " DEFAULT" : ""}`);
    }
    const nodefault = row.consumers.filter((consumer) => !consumer.defaultFeatures).length;
    if (nodefault) console.log(`  (default-features=false by ${nodefault})`);
  }
  console.log(`[DEBUG] packages=${selected.length} features=${selected.reduce((sum, row) => sum + row.features.length, 0)} peer=${selected.filter((row) => row.peer).length}`);
}
