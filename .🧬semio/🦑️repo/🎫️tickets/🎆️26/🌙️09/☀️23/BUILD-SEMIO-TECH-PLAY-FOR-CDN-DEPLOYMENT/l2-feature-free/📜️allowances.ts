/** 📜️ Wave L2: writes the documented G3 allow-list (`firstPartyFeaturesAllowed` of the hub unit census fixture) from `cargo metadata` of the
 * hub workspace. Every first-party feature still declared there gets exactly one classified row; anything this table cannot classify fails.
 * Usage: `bun 📜️allowances.ts <hub-metadata.json | --manifests> [--apply]`; `--manifests` derives the same first-party package set from the
 * manifests (members with every dependency kind, other path packages with normal and build dependencies, as Cargo resolves them). */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { PEER_OWNED, repoRoot } from "./🗺️inventory.ts";

type Allowance = { package: string; feature: string; classification: "test-only" | "peer-owned" | "peer-blocked"; reason: string };

const TEST_ONLY: Record<string, string> = {
  "semio-framework-artifact-flow-flow/protocol-laws": "protocol laws for dependents' tests; extraction into a law crate pending",
  "semio-framework-async/testkit": "ManualRuntime test double for dependents' tests; peer crates enable it in their dev-dependencies",
  "semio-framework-graph-layout-run/testing": "fdg-sim reference oracle for dependents' layout tests",
  "semio-framework-job/testkit": "forwards the async test double; peer crates enable it in their dev-dependencies",
  "semio-framework-os-kernel-db/claimed-backend-laws": "claimed shared-server laws for the hub's live directory lanes",
  "semio-framework-pack/corruption-testing": "corruption fixtures; the peer-owned os-pack forwards it",
  "semio-framework-plugin/artifact-app-testing": "artifact app harness for every artifact crate's tests; moves with the plugin crate cut",
  "semio-framework-plugin/media-turn-testing": "hooks for the plugin's own [[test]] target; becomes a unit test with the plugin crate cut",
  "semio-framework-plugin/reserved-producer-testing": "hooks for the plugin's own [[test]] target; becomes a unit test with the plugin crate cut",
  "semio-framework-plugin/original-actor-context-testing": "hooks for the plugin's own [[test]] target; becomes a unit test with the plugin crate cut",
  "semio-framework-server/conformance": "server conformance laws for dependents' tests",
  "semio-framework-value/testkit": "value test helpers; the peer-owned os-kernel enables it in a normal dependency (reported)",
  "semio-hub/integration-fixtures": "fixture-carrying os-hub binary for cross-target integration laws; needs a fixture library plus a test-server binary crate",
  "semio-s-plugin-norm-app-surface/compliance-testing": "norm app-surface compliance oracle for every norm artifact's tests",
  "semio-s-plugin-norm-compliance/compliance-testing": "norm compliance oracle for every norm artifact's tests",
  "semio-s-artifact-stdio-contract/testing": "stdio contract law kit for every stdio artifact's tests",
  "semio-s-artifact-stdio-gltf-model/testing": "test-only serde derives on glTF model types (production types stay serde-free)",
  "semio-s-artifact-stdio-semio-base/testing": "test-only serde derives on semio base types (production types stay serde-free)",
  "semio-s-plugin-block/test-serde": "test-only serde derives on block plugin types (production types stay serde-free)",
};
const PEER_BLOCKED: Record<string, string> = {
  "semio-framework-async/entrypoint": "block_on gate enabled by peer-owned crates (os-kernel, os-run, ui, plugin-host, renderer); an entrypoint crate replaces it once they move",
  "semio-framework-pack/deflate": "default feature forwarded by the peer-owned os-pack, os-history and os-kernel",
  "semio-framework-pack/default": "selects deflate by default for the peer-owned consumers that do not name it",
  "semio-framework-replication/deflate": "forwarded by the peer-owned os-pack, os-history and os-kernel",
};

type Toml = Record<string, any>;
type MetadataPackage = { name: string; source: string | null; features: Record<string, string[]>; manifest_path: string };

/** 🧭️ The `[workspace]` root manifest a package inherits from: its `package.workspace` key, else the nearest ancestor declaring `[workspace]`. */
function workspaceRoot(manifest: string, toml: Toml): string {
  if (toml.workspace) return manifest;
  if (typeof toml.package?.workspace === "string") return join(resolve(dirname(manifest), toml.package.workspace), "Cargo.toml");
  for (let dir = dirname(dirname(manifest)); dir.length >= repoRoot.length; dir = dirname(dir)) {
    const candidate = join(dir, "Cargo.toml");
    if (existsSync(candidate) && (Bun.TOML.parse(readFileSync(candidate, "utf8")) as Toml).workspace) return candidate;
  }
  return manifest;
}

/** 🗺️ The first-party packages of one workspace with their declared features (implicit optional-dependency features included). */
function manifestPackages(workspaceManifest: string): MetadataPackage[] {
  const parse = (path: string): Toml => Bun.TOML.parse(readFileSync(path, "utf8")) as Toml;
  const root = parse(workspaceManifest);
  const members = new Set((root.workspace.members as string[]).map((member) => join(dirname(workspaceManifest), member, "Cargo.toml")));
  const seen = new Map<string, MetadataPackage>(), queue = [...members];
  while (queue.length) {
    const manifest = queue.shift()!;
    if (seen.has(manifest)) continue;
    const toml = parse(manifest);
    const inherited = parse(workspaceRoot(manifest, toml)).workspace?.dependencies ?? {};
    const inheritedRoot = dirname(workspaceRoot(manifest, toml));
    const sections = members.has(manifest) ? ["dependencies", "dev-dependencies", "build-dependencies"] : ["dependencies", "build-dependencies"];
    const tables = [...sections.map((section) => toml[section] ?? {}), ...Object.values<Toml>(toml.target ?? {}).flatMap((target) => sections.map((section) => target[section] ?? {}))];
    const explicit = new Set(Object.values<string[]>(toml.features ?? {}).flat().filter((entry) => entry.startsWith("dep:")).map((entry) => entry.slice(4)));
    const features: Record<string, string[]> = { ...(toml.features ?? {}) };
    for (const table of tables) for (const [key, spec] of Object.entries<Toml>(table)) {
      if (typeof spec !== "object") continue;
      const base = spec.workspace ? inherited[key] : spec;
      const path = typeof base === "object" && base.path ? (spec.workspace ? resolve(inheritedRoot, base.path) : resolve(dirname(manifest), base.path)) : undefined;
      if (path) queue.push(join(path, "Cargo.toml"));
      if (spec.optional && !explicit.has(key) && !(key in features) && (table === toml.dependencies || table === toml["build-dependencies"] || Object.values<Toml>(toml.target ?? {}).some((target) => target.dependencies === table || target["build-dependencies"] === table))) features[key] = [`dep:${key}`];
    }
    seen.set(manifest, { name: toml.package.name, source: null, features, manifest_path: manifest });
  }
  return [...seen.values()];
}

const metadata = process.argv[2] === "--manifests" ? { packages: manifestPackages(join(repoRoot, "🌎️hub/Cargo.toml")) } : JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { packages: MetadataPackage[] };
const rows: Allowance[] = [];
const unknown: string[] = [];
for (const pkg of metadata.packages.filter((row) => row.source === null).sort((a, b) => a.name.localeCompare(b.name))) {
  const manifest = Bun.TOML.parse(readFileSync(pkg.manifest_path, "utf8")) as { package?: { metadata?: { semio?: Record<string, unknown> } } };
  const peer = PEER_OWNED.some((pattern) => pattern.test(pkg.name)) || (pkg.name !== "semio-framework-plugin" && manifest.package?.metadata?.semio?.["rust-source-boundary"] !== undefined);
  for (const feature of Object.keys(pkg.features).sort()) {
    const key = `${pkg.name}/${feature}`;
    if (peer) rows.push({ package: pkg.name, feature, classification: "peer-owned", reason: "owned by peer fleet A (peer-scope map §1); classified in 📓️2026-10-14-wave-l2-feature-free.md, not edited by this wave" });
    else if (key in TEST_ONLY) rows.push({ package: pkg.name, feature, classification: "test-only", reason: TEST_ONLY[key]! });
    else if (key in PEER_BLOCKED) rows.push({ package: pkg.name, feature, classification: "peer-blocked", reason: PEER_BLOCKED[key]! });
    else unknown.push(key);
  }
}
for (const row of rows) console.log(`${row.classification.padEnd(12)} ${row.package}/${row.feature}`);
console.log(`[DEBUG] rows=${rows.length} unknown=${unknown.length} ${unknown.join(" ")}`);
if (unknown.length) process.exit(1);
if (process.argv.includes("--apply")) {
  const path = join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🌳️unit-census/🔣️.json");
  const text = readFileSync(path, "utf8");
  const start = text.indexOf('  "firstPartyFeaturesAllowed": ['), end = text.indexOf('  "featureCases": [');
  if (start < 0 || end < 0) throw new Error("fixture lacks the G3 sections");
  const body = rows.length ? `  "firstPartyFeaturesAllowed": [\n${rows.map((row) => `    ${JSON.stringify(row)}`).join(",\n")}\n  ],\n` : '  "firstPartyFeaturesAllowed": [],\n';
  writeFileSync(path, text.slice(0, start) + body + text.slice(end));
  console.log("[DEBUG] applied");
}
