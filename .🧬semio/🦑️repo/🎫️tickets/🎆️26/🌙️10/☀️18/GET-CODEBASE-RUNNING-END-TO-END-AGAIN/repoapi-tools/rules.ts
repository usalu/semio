import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { LIB_REL, ROOT, readJson } from "./lib.ts";
import { exportsOf } from "./exports.ts";

/** Domain entries of `@semio-tech/repo-lib`: the legitimate public API of the repository product, grouped by what a consumer wants to do. */
export type DomainId = "laws" | "inventory" | "authoring" | "testing" | "process" | "cargo" | "artifacts" | "storage" | "build" | "dev" | "policy";
export const DOMAINS: Record<DomainId, { key: string; dir: string; purpose: string }> = {
  laws: { key: "./laws", dir: "⚖️laws", purpose: "law engines, inventories and self-tests that products bind into their verify lanes" },
  inventory: { key: "./inventory", dir: "📇️inventory", purpose: "repository inventory: taxonomy authority, package discovery, owner markers, catalog projections, schema field extraction, dependency inventory, workspace facts" },
  authoring: { key: "./authoring", dir: "🏗️authoring", purpose: "artifact authoring: generator previews and their contracts, scaffolding, projection adapters, canonical serialization, launch and collection catalogs" },
  testing: { key: "./testing", dir: "🧪️testing", purpose: "test levels, Go plans, coverage, lcov, shared test values and inference test sources" },
  process: { key: "./process", dir: "🏃️process", purpose: "process helpers of dev scripts that have no framework port yet: daemons, owner-only files, bun, environment, route entry points" },
  cargo: { key: "./cargo", dir: "🦀️cargo", purpose: "Cargo and WASM execution, workspaces, preparation and native builds for packages of the repository product" },
  artifacts: { key: "./artifacts", dir: "📦️artifacts", purpose: "artifact reading, storage, destinations, groups, executables, bundles and their schemas" },
  storage: { key: "./storage", dir: "💾️storage", purpose: "bounded repository storage: cache partitions, budget stores, Nx and compiler custody, bootstrap tooling, development state" },
  build: { key: "./build", dir: "🏗️build", purpose: "build graph, selection, inputs, caller capabilities, TypeScript check, observation and inference plugin facets" },
  dev: { key: "./dev", dir: "🎮️playground", purpose: "dev servers: Vite services, sessions and plugins, playground catalog, ports and native host" },
  policy: { key: "./policy", dir: "🚨️policy", purpose: "policy linters, breach records, runner and CLI resolution" },
};

/** Purpose line of the JavaScript sibling entries that Nx, dependency-cruiser and other node-loaded tooling import. */
export const SUB_PURPOSE: Record<string, string> = {
  "./build/tooling": "Nx plugin facets in JavaScript: build mode, dependency observation and build-graph fact storage",
  "./build/inference": "Nx inference plugin, argument shapes and the build dependency implementation in JavaScript",
  "./inventory/tooling": "runtime component closure and generated-source contracts in JavaScript",
  "./inventory/commonjs": "dependency-direction policy bootstrap that dependency-cruiser loads as CommonJS",
};

const PREFIX: [string, DomainId][] = [
  ["⚖️laws/", "laws"],
  ["🚨️policy/", "policy"],
  ["🧬️schema/", "inventory"],
  ["🧾️serialization/", "authoring"],
  ["🏭️generator/", "authoring"],
  ["🏗️authoring/", "authoring"],
  ["🏗️builder/", "authoring"],
  ["📽️projection/", "authoring"],
  ["🗂️workspaces/🦀️cargo/", "cargo"],
  ["🏃️process/🦀️cargo/", "cargo"],
  ["🏃️process/🕸️wasm/", "cargo"],
  ["⚡️caching/🦀️cargo/", "cargo"],
  ["⚡️caching/📦️artifacts/🏗️native-build/", "cargo"],
  ["⚡️caching/📦️artifacts/🦀️rust/", "cargo"],
  ["⚡️caching/📦️artifacts/🕸️wasm/", "cargo"],
  ["🏗️build/🕸️graph/🦀️rust/", "cargo"],
  ["🏗️build/🎚️mode/🦀️profile/", "cargo"],
  ["🏗️build/🎯️host-platform/", "cargo"],
  ["🏗️build/🕸️graph/📥️inference/🧪️tests/", "testing"],
  ["⚡️caching/🧪️tests/", "testing"],
  ["🧪️testing/", "testing"],
  ["⚡️caching/📦️artifacts/", "artifacts"],
  ["⚡️caching/📦️bundle/", "artifacts"],
  ["⚡️caching/💾️storage/", "storage"],
  ["⚡️caching/🚀️bootstrap/", "storage"],
  ["💾️development/", "storage"],
  ["⚡️caching/🌐️vite/", "dev"],
  ["🎮️playground/", "dev"],
  ["🌐️network/", "dev"],
  ["🏃️process/", "process"],
  ["⚡️caching/🟦️.ts", "process"],
  ["🏗️build/", "build"],
  ["🟨️.mjs", "build"],
  ["🔍️discovery/", "inventory"],
  ["🧹️normalization/", "inventory"],
  ["📇️catalog/", "inventory"],
  ["🤖️agent-clients/", "inventory"],
  ["🕸️dependencies/", "inventory"],
  ["🗂️workspaces/", "inventory"],
  ["🎛️commands/", "inventory"],
];

export function domainOf(libRelTarget: string): DomainId | undefined {
  for (const [prefix, domain] of PREFIX) if (libRelTarget === prefix || libRelTarget.startsWith(prefix)) return domain;
  return undefined;
}

export type Glue = "ts" | "mjs" | "cjs";
export const glueOf = (libRelTarget: string): Glue => (libRelTarget.endsWith(".cjs") ? "cjs" : libRelTarget.endsWith(".mjs") ? "mjs" : "ts");

/** Where a framework destination lives: package, export key, glue file (repository relative). */
export type Dest = { pkg: string; key: string; glue: string };
const PROCESS_DIR = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript";

let processExports: Record<string, string> | undefined;
export function processDest(key: string): Dest {
  processExports ??= readJson(resolve(ROOT, PROCESS_DIR, "package.json")).exports;
  const target = processExports![key];
  if (!target) throw Error(`process export ${key} missing`);
  return { pkg: "@semio-tech/process", key, glue: `${PROCESS_DIR}/${target.replace(/^\.\//u, "")}` };
}

export type FwName = { dest: Dest; as?: string; mode: "specifier" | "rename" | "cargo-script" };

/** Tool configuration is evaluated when an editor or runner loads it, outside the script envelope that issues the provider designation. */
export const isConfigFile = (file: string): boolean => /(?:^|\/)(?:vitest|vite)\.config\.[cm]?[jt]s$|\/🧪️tests\/🎚️config\/|\/🏗️builder\/🌐️vite\/|^\.storybook\//u.test(file);
/** Bootstrap tooling runs before any dependency is installed: it may only import relative paths. */
export const isBootstrapFile = (file: string): boolean => file.startsWith(`${LIB_REL}/⚡️caching/🚀️bootstrap/`);
/** Destinations whose functions load a designated provider at call time. */
export const needsProvider = (dest: Dest): boolean => dest.key !== "./routing";
export type FwRules = Record<string, Record<string, FwName>>;

const EXEC = () => processDest("./execution");
const ROUTING = () => processDest("./routing");
const VIT = () => processDest("./vitest");
const TEXEC = () => processDest("./test-execution");
const CARGO = () => processDest("./cargo-test");
const TSC = () => processDest("./typescript-check");
const NATIVE = () => processDest("./native-build");
const WASM = () => processDest("./wasm-build");

const spec = (dest: () => Dest, ...names: string[]): Record<string, FwName> => Object.fromEntries(names.map(n => [n, { dest: dest(), mode: "specifier" as const }]));
const ren = (dest: () => Dest, from: string, to: string): Record<string, FwName> => ({ [from]: { dest: dest(), as: to, mode: "rename" } });
const cargo = (from: string, to: string): Record<string, FwName> => ({ [from]: { dest: CARGO(), as: to, mode: "cargo-script" } });

/** Symbols that belong to the framework: non-repo importers reach them through the framework ports, never through the repository product. */
export function frameworkRules(): FwRules {
  return {
    "⚡️caching/🟦️.ts": spec(EXEC, "repoCacheDirectory"),
    "⚡️caching/🦀️cargo/🗂️directories/🟦️.ts": spec(EXEC, "cargoTargetDirectory", "cargoBuildDirectory", "cargoDirectories"),
    "🏃️process/🌿️environment/🧪️test-output/🟦️.ts": spec(EXEC, "repoTestArtifactEnvironment"),
    "🏃️process/🌿️environment/🟦️.ts": spec(EXEC, "devToolingEnv", "semioBuildParallel", "repoToolCacheEnv"),
    "🏃️process/🟦️.ts": spec(EXEC, "runCmd", "orchestratorBudgetOpts", "runCmdStatus", "runNodeBinStatus", "resolveWorkspaceBin", "daemonBudgetOpts", "cargoProfileDir"),
    "🏃️process/🍞️bun/🟦️.ts": spec(EXEC, "runBun"),
    "🏃️process/🔍️probe/🟦️.ts": spec(EXEC, "runProbe"),
    "🏃️process/🎛️owned-execution/🟦️.ts": spec(EXEC, "runRepositoryCommand"),
    "🏃️process/🧭️routing/🧩️contributions/🟦️.ts": spec(EXEC, "dispatchOwnedScriptRoute"),
    "🏃️process/🦀️cargo/🏭️inventory/🟦️.ts": spec(EXEC, "runRepositoryMutationBridgeScriptV1"),
    "🏃️process/🦀️cargo/🏃️run/🟦️.ts": spec(EXEC, "runCargo"),
    "🗂️workspaces/🧬️metadata/🟦️.ts": spec(EXEC, "getRepoMetaDir"),
    "🗂️workspaces/🟦️.ts": ren(ROUTING, "getWorkspaceRoot", "findWorkspaceRoot"),
    "🧪️testing/🧪️vitest/🟦️.ts": spec(VIT, "runVitest"),
    "🧪️testing/🎛️repository-command/🟦️.ts": spec(TEXEC, "runRepositoryTestCommand"),
    "🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts": spec(NATIVE, "observeCargoPreparationSourceV1", "observeCargoPreparationInputV1", "observeCargoPreparationOutputV1", "completeCargoPreparationObservationV1"),
    "🗂️workspaces/🦀️cargo/🟦️.ts": spec(NATIVE, "cargoWorkspaceForManifest", "cargoRepositoryPackages"),
    "⚡️caching/📦️artifacts/🦀️rust/🟦️.ts": spec(CARGO, "runArtifactRustPackageMain", "runArtifactRustTests", "runArtifactRustPackagesTests"),
    "⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts": spec(TSC, "runArtifactTypeScriptPackageMain"),
    "🏗️build/🟦️typescript/🟦️.ts": ren(TSC, "typecheckRepositoryProjectV1", "typecheckAdmittedProjectV1"),
    "🏃️process/🕸️wasm/🔣️policy/🟦️.ts": ren(WASM, "buildRepositoryWasmWebV1", "buildAdmittedWasmWebV1"),
    "🏃️process/🦀️cargo/🎟️admission/🟦️.ts": {
      ...cargo("runRepositoryCargoTestsV1", "runAdmittedCargoTestsV1"),
      ...cargo("runRepositoryCargoLintV1", "runAdmittedCargoLintV1"),
      ...cargo("runRepositoryCargoCommandV1", "runAdmittedCargoCommandV1"),
      ...cargo("runRepositoryCargoTests", "runAdmittedCargoPackageTestsV1"),
    },
    "🏃️process/🦀️cargo/🎯️exact/🟦️.ts": {
      ...cargo("runRepositoryExactCargoLaws", "runAdmittedExactCargoLawsV1"),
      ...cargo("configuredRepositoryCargoPreparationPolicyV1", ""),
    },
    "🏃️process/🦀️cargo/🔣️policy/🟦️.ts": cargo("repositoryCargoTestPolicyV1", "admittedCargoTestPolicyV1"),
    "⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts": cargo("buildRepositoryCargoArtifacts", "buildAdmittedCargoArtifactsV1"),
    "🏃️process/🦀️cargo/📥️dependencies/🟦️.ts": cargo("repositoryCargoNetworkV1", ""),
  };
}

/** True when the destination glue exports the name the rule promises. */
export function destHas(rule: FwName, imported: string): boolean {
  const abs = resolve(ROOT, rule.dest.glue);
  if (!existsSync(abs)) return false;
  const e = exportsOf(abs);
  const name = rule.as ?? imported;
  return name === "" || e.values.has(name) || e.types.has(name);
}

export const libRelOf = (repoRel: string): string => repoRel.slice(LIB_REL.length + 1);
export const isRepoProduct = (file: string): boolean => file.startsWith("🧰️framework/🛍️products/🦑️repo/");
export const isScript = (file: string): boolean => file.endsWith("📜️script.ts");
export const isFixture = (libTarget: string): boolean => /\/🧪️tests\/🧫️fixtures\//u.test(libTarget);

/** ASCII slug of one taxonomy path segment: `🧬️schema` becomes `schema`, `🔣️event.json` becomes `event`. */
export const ascii = (segment: string): string => segment.replace(/\.[a-z]+$/u, "").replace(/[^\x00-\x7f]/gu, "");
export const camel = (parts: string[]): string => parts.filter(Boolean).map((p, i) => p.split(/[^A-Za-z0-9]+/u).filter(Boolean).map((w, j) => (i === 0 && j === 0 ? w.charAt(0).toLowerCase() + w.slice(1) : w.charAt(0).toUpperCase() + w.slice(1))).join("")).join("");

export function readText(abs: string): string {
  return readFileSync(abs, "utf8");
}

import { PKG_DIR } from "./lib.ts";

export type Entry = { domain: DomainId; glue: Glue; key: string; glueAbs: string };
/** Domain entry module (key and glue file) that exposes one library file. */
export function entryOf(libTarget: string): Entry {
  const domain = domainOf(libTarget);
  if (!domain) throw Error(`no domain for ${libTarget}`);
  const glue = glueOf(libTarget);
  const d = DOMAINS[domain];
  const inference = glue === "mjs" && /^(?:🏗️build\/🕸️graph\/|🏗️build\/📤️arguments\/|🟨️\.mjs$)/u.test(libTarget);
  const key = glue === "ts" ? d.key : glue === "mjs" ? `${d.key}/${inference ? "inference" : "tooling"}` : `${d.key}/commonjs`;
  const file = glue === "ts" ? "🟦️.ts" : glue === "mjs" ? "🟨️.mjs" : "🟨️.cjs";
  const dir = inference ? resolve(PKG_DIR, d.dir, "🕸️graph/📥️inference") : resolve(PKG_DIR, d.dir);
  return { domain, glue, key, glueAbs: resolve(dir, file) };
}

/** Deterministic, collision-free names for JSON documents exposed by a domain entry. */
export function jsonNames(libTargets: string[]): Map<string, string> {
  const out = new Map<string, string>();
  const used = new Map<string, Set<string>>();
  for (const t of [...libTargets].filter(x => x.endsWith(".json")).sort()) {
    const domain = domainOf(t)!;
    const dirs = t.split("/").slice(0, -1).map(ascii).filter(p => p && p !== "caching");
    const stem = ascii(t.split("/").pop()!);
    const isSchema = dirs[dirs.length - 1] === "schema";
    const base = isSchema ? dirs.slice(0, -1) : dirs;
    const suffix = isSchema ? "schema" : stem ? "" : "document";
    const set = used.get(domain) ?? new Set<string>();
    used.set(domain, set);
    let name = "";
    for (let take = Math.min(2, base.length); take <= base.length; take++) {
      name = camel([...base.slice(base.length - take), stem, suffix]);
      if (!set.has(name)) break;
    }
    if (set.has(name)) name = camel([...base, stem, suffix]) + set.size;
    set.add(name);
    out.set(t, name);
  }
  return out;
}
