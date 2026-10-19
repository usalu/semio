// 🗺️ Cache decisions for the cache-rest scope: every edit is keyed by manifest and task; every kept-off task names its reason.
import { existsSync, readFileSync } from "node:fs";

export const ROOT = "/Users/ueli/Documents/semio/";
export type TaskEdit = { cache?: boolean; inputs?: unknown[]; outputs?: string[] };
export type FilePlan = { named?: Record<string, unknown[]>; tasks: Record<string, TaskEdit> };
export type KeepOff = { file: string; task: string; reason: string };

const DEP = { dependentTasksOutputFiles: "**/*" };
const ON: TaskEdit = { cache: true, outputs: [] };
const on = (extra: TaskEdit = {}): TaskEdit => ({ cache: true, outputs: [], ...extra });

const ROOT_PROJECT = "📋️project.json";
const HUB_RS = "🌎️hub/📦️packages/🦀️rust/📋️project.json";
const HUB_TS = "🌎️hub/📦️packages/🟦️typescript/📋️project.json";
const AUTH = "🌎️hub/🔐️auth/🔌️client/📦️packages/🦀️rust/📋️project.json";
const FLOW = "🌎️hub/🧩️compositions/🌊️flow/📦️packages/🦀️rust/📋️project.json";
const GIS = "🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/📋️project.json";
const DRAW = "🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/📋️project.json";
const COMP = "🌎️hub/🧩️compositions/📋️project.json";
const STDIO = "🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📋️project.json";
const ORACLES = "🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/📋️project.json";
const PUZZLE_RETAINED = "🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/📋️project.json";
const SPACE_TESTING = "🌎️hub/🧩️compositions/🪐️space/🧪️testing/📋️project.json";
const SPACE_COMPUTE = "🌎️hub/🧩️compositions/🪐️space/🏛️ownership/🧮️compute/📋️project.json";
const SPACE_RS = "🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📋️project.json";
const PUZZLE_RS = "🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json";
const WGPU = "🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/📋️project.json";
const DEMO = "♻️mit-bestand/🧺️demonstrator/📋️project.json";

const rustWorkspace = [
  "sharedGlobals",
  "{workspaceRoot}/**/*.rs",
  "{workspaceRoot}/**/Cargo.toml",
  "{workspaceRoot}/Cargo.lock",
  "{workspaceRoot}/rust-toolchain.toml",
  "{workspaceRoot}/.cargo/config.toml",
  { env: "RUSTUP_TOOLCHAIN" },
  { env: "RUSTFLAGS" },
  { env: "CARGO_ENCODED_RUSTFLAGS" },
  { env: "CARGO_BUILD_TARGET" },
  { env: "SEMIO_TEST_LEVEL" },
  { env: "SEMIO_TEST_BUDGET_MS" },
  { env: "SEMIO_BUILD_BUDGET_MS" },
];

const stdioSources = ["sharedGlobals", "{projectRoot}/**/*", "{workspaceRoot}/🌎️hub/🧩️compositions/🗄️stdio/**/*", "{workspaceRoot}/✏️s/🔌️plugins/**/*"];

const graphGenerateSources = [
  "{workspaceRoot}/✏️s/🔌️plugins/🌊️flow/**/*manifest.json",
  "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/**/*",
  "{projectRoot}/../../🛂️manifest/📇️outputs.json",
  "{projectRoot}/📜️script.ts",
  "{projectRoot}/📋️project.json",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/**/*.ts",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
];

function fixtureReferences(file: string): string[] {
  const found = new Set<string>();
  const walk = (value: unknown) => {
    if (typeof value === "string") {
      const path = value.replace(/^\.\//u, "");
      if (path.includes("/") && !path.includes(" ") && existsSync(ROOT + path)) found.add(path);
    } else if (Array.isArray(value)) value.forEach(walk);
    else if (value && typeof value === "object") Object.values(value).forEach(walk);
  };
  walk(JSON.parse(readFileSync(ROOT + file, "utf8")));
  return [...found].sort().map(path => path.endsWith("/") || !/\.[A-Za-z0-9]+$/u.test(path) ? `{workspaceRoot}/${path.replace(/\/$/u, "")}/**/*` : `{workspaceRoot}/${path}`);
}

export function buildPlan(rootVerifyTasks: string[], rootTidyTasks: string[]): { files: Record<string, FilePlan>; keepOff: KeepOff[] } {
  const files: Record<string, FilePlan> = {};
  const keepOff: KeepOff[] = [];
  const off = (file: string, tasks: string[], reason: string) => tasks.forEach(task => keepOff.push({ file, task, reason }));

  const rootTasks: Record<string, TaskEdit> = {};
  for (const task of rootVerifyTasks) rootTasks[task] = on({ inputs: ["default"] });
  for (const task of rootTidyTasks) rootTasks[task] = on();
  Object.assign(rootTasks, {
    "agents-check": on({ inputs: ["sharedGlobals", "{workspaceRoot}/📋️project.json", "{workspaceRoot}/.mcp.json", "{workspaceRoot}/.codex/config.toml"] }),
    "generate-config-check": on(),
    "schema-fixture-boundary": on({ inputs: ["default", "schemaSources"] }),
    "schema-verify": on({ inputs: ["default", "schemaSources"] }),
    "schema-entries": on(),
    "schema-parsers-report": on(),
    "schema-parsers-check": on(),
    "test-canvas-native": on({ inputs: ["default", { env: "SEMIO_TEST_LEVEL" }, { env: "SEMIO_TEST_BUDGET_MS" }, { env: "SEMIO_BUILD_BUDGET_MS" }] }),
    "test-canvas-react": on({ inputs: ["default", { env: "SEMIO_TEST_LEVEL" }, { env: "SEMIO_TEST_BUDGET_MS" }, { env: "SEMIO_BUILD_BUDGET_MS" }] }),
    "test-command-entrypoint": on(),
    "test-runtime-build-producer": on(),
    "cpp-custody": on(),
    "cpp-test-toolchain": on(),
    examples: on(),
  });
  files[ROOT_PROJECT] = { tasks: rootTasks, named: { default: ["{projectRoot}/**/*", "sharedGlobals", "{workspaceRoot}/🧪️tests/🎚️config/🟦️.ts", "{workspaceRoot}/🧰️framework/🔨️modules/🧬️schema/🧪️tests/🎚️config/🟦️.ts"] } };
  off(ROOT_PROJECT, ["agents-write"], "rewrites the MCP client files from the root declaration in place");
  off(ROOT_PROJECT, ["generate-config"], "rewrites tracked scope nextest files from the root nextest file in place (read-modify-write of tracked sources)");
  off(ROOT_PROJECT, ["schema-parsers-apply", "schema-parsers-restate"], "projects parsers into tracked sources (writes what it reads)");
  off(ROOT_PROJECT, ["verify-canonical-architecture"], "orchestrator: re-enters the build tool (`run-many -t canonical-architecture --skip-cache`) over every project");
  off(ROOT_PROJECT, ["build", "release", "components"], "orchestrator or release pipeline; products live in leased stores (plugin modules, vite store), not in restorable outputs");
  off(ROOT_PROJECT, ["build-storybook"], "storybook static build, leased vite store output, port env");
  off(ROOT_PROJECT, ["build-canvas-font-assets"], "cargo build whose artifacts are captured in the shared compiler store");
  off(ROOT_PROJECT, ["cpp", "cpp-setup", "cpp-configure", "cpp-build", "cpp-test"], "CMake chain: installs toolchain (setup), build tree lives in the native store consumed downstream, host compiler probes");
  off(ROOT_PROJECT, ["cargo-prepare"], "cargo workspace preparation writes generated manifests and store observations");
  off(ROOT_PROJECT, ["runtime-fixture-graph"], "builds a production fixture graph in a private cargo store and runs guest checks (compiler captures)");
  off(ROOT_PROJECT, ["test-build-tool-bazel", "test-build-tool-bazel-bootstrap"], "runs real Bazel actions and bootstraps the pinned Bazel binary (downloads, output_user_root)");
  off(ROOT_PROJECT, ["format", "clean-test", "clean-coverage", "clean-marker-only-folders", "clean-taxonomy-inventory", "clean-taxonomy-plan", "clean-taxonomy-apply", "clean-taxonomy-verify", "new-taxonomy-mutation"], "policy-pinned or side effecting: `clean*`/`format`/`new` names are forced uncached by the workspace policy");
  off(ROOT_PROJECT, ["bench-plugins", "os", "semio"], "benchmark or launches the OS shell");
  off(ROOT_PROJECT, ["setup", "setup-storybook", "setup-git", "setup-native", "setup-devcontainer-start", "setup-devcontainer-attach", "cpp-setup", "publish", "deps-javascript", "deps-python", "deps-cargo", "deps-go", "deps-dotnet", "deps-cpp", "deps-browsers", "deps-wasm", "deps-trunk", "deps-wasm-opt", "deps-tools", "deps-cargo-lock", "deps-js-lock", "deps-js-contract", "deps-js-all", "deps-build-tools"], "installs or publishes: mutates the host, the lockfiles or the network");

  files[HUB_RS] = {
    tasks: {
      "unit-census-check": on({ inputs: ["default", "^default", "sharedGlobals", "{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/**/Cargo.lock", "{workspaceRoot}/rust-toolchain.toml", "{workspaceRoot}/.cargo/config.toml", { env: "RUSTFLAGS" }, { env: "RUSTUP_TOOLCHAIN" }] }),
      "component-codec-check": on({ inputs: ["default", "^default", DEP, "{workspaceRoot}/🌎️hub/🧪️tests/🧩️component-codecs/**/*", "{workspaceRoot}/🌎️hub/🧬️schema/🧩️component-codecs/**/*"] }),
      "component-codec-budget-check": on({ inputs: ["default", "^default", DEP, "{workspaceRoot}/🌎️hub/🧪️tests/🧩️component-codecs/**/*", "{workspaceRoot}/🌎️hub/🧬️schema/🧩️component-codecs/**/*"] }),
      test: on(),
      "test-all-features": on({ inputs: ["default", "^default", DEP, { env: "SEMIO_TEST_LEVEL" }, { env: "SEMIO_TEST_BUDGET_MS" }, { env: "SEMIO_BUILD_BUDGET_MS" }] }),
      "test-inference-client-contract": on(),
    },
  };
  off(HUB_RS, ["runtime-fixture-graph"], "builds a production fixture graph in a private cargo store (compiler captures)");
  off(HUB_RS, ["setup"], "cargo fetch: network and host mutation");
  off(HUB_RS, ["publish"], "publishes the hub distribution");
  off(HUB_RS, ["live-sign-in-check", "admin-live-journey-check"], "live: boots a real hub child and signs in over HTTP");
  off(HUB_RS, ["browser-actor-child-worker-containment-check", "browser-actor-gis-describe-check", "hub-document-sweep", "io-matrix"], "drives Chromium against a loopback server and writes timestamped acceptance records");
  off(HUB_RS, ["trusted-catalog-preflight"], "reads built plugin components from the products store, outside declared inputs");
  off(HUB_RS, ["trusted-catalog-bootstrap"], "publishes a trusted catalog generation and starts a local hub");
  off(HUB_RS, ["secure-local-smoke", "dev-collaboration"], "smoke or collaboration run against a live local hub");
  off(HUB_RS, ["hostile-input-check"], "publishes a timestamped acceptance record and runs seeded generative laws");
  off(HUB_RS, ["canonical-architecture"], "policy.targetDefaults pins canonical-architecture uncached");

  off(HUB_TS, ["two-client-e2e", "document-growth-e2e"], "e2e: boots the real hub binary with two clients");
  off(HUB_TS, ["backend-up", "backend-down", "backend-status", "backend-run", "docker-image-build", "docker-image-check"], "Docker daemon side effects");
  off(HUB_TS, ["backup-restore-drill", "shutdown-drill"], "operational drill against a built local hub with timing assertions");
  off(HUB_TS, ["residency-watch", "boot-watch", "hub-freshness", "agent-ceiling-check"], "needs a running hub URL (live observation)");

  files[AUTH] = {
    tasks: {
      check: on(),
      test: on(),
      "test-source": on({ inputs: ["default", "^default", "{workspaceRoot}/🌎️hub/🔐️auth/🔌️client/**/*", "{workspaceRoot}/🌎️hub/🔐️auth/🧬️schema/🔣️.json", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🤖️agent-credential/**/*", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/**/*", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/⌨️entrypoint/🦀️.rs"] }),
    },
  };

  files[FLOW] = {
    named: { graphGenerateSources },
    tasks: {
      "graph-generate": { inputs: ["graphGenerateSources"] },
      "preview-generated": on({ inputs: ["graphGenerateSources", "{workspaceRoot}/🌎️hub/🧩️compositions/🌊️flow/🤖️generated/**/*"] }),
      "graph-wire-check": on({ inputs: ["default", "^default", DEP] }),
    },
  };

  files[GIS] = {
    tasks: {
      "installed-service-source": on({ inputs: ["default", "^default", "{workspaceRoot}/🌎️hub/🧩️compositions/🌍️gis/🧪️tests/**/*"] }),
    },
  };
  off(GIS, ["hub-edit-durability-check"], "restarts a hub process to observe durability (live)");

  files[DRAW] = { tasks: { "guest-instance-check": on({ inputs: ["default", "^default", DEP] }) } };

  files[COMP] = {
    named: { rustWorkspace },
    tasks: { ownership: on(), typescript: on(), native: on({ inputs: ["default", "rustWorkspace"] }) },
  };

  files[STDIO] = { tasks: { "composition-contribution-check": on({ inputs: ["default", "^default", "{workspaceRoot}/🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/**/*"] }), "mutation-inventory": on({ inputs: ["default", "^default", "{workspaceRoot}/🌎️hub/🧩️compositions/🗄️stdio/🏭️bridge/**/*"] }) } };
  off(STDIO, ["artifact-directory-wiring-generate", "subset-directory-wiring-generate"], "rewrites tracked wiring sources in place (the -check twin is cached)");
  off(STDIO, ["native-codec-projection"], "read-modify-write of the tracked factory, definition and catalog files it also reads");
  off(STDIO, ["catalog-root"], "needs a caller-owned empty build root and runs a cargo release build");
  off(STDIO, ["editor-component-check"], "links wasm components into the shared cargo target (compiler store output)");
  off(STDIO, ["composition-prepare"], "rewrites tracked manifests and generated sources in place (reads what it writes)");
  off(STDIO, ["parent-removal-test-check", "parent-removal-check"], "needs a copied workspace snapshot argument and writes proofs inside it");

  files[ORACLES] = {
    named: { stdioSources, rustWorkspace },
    tasks: {
      "test-composition": on({ inputs: ["stdioSources"] }),
      "test-drawing-reader": on({ inputs: ["stdioSources"] }),
      "test-private-reader": on({ inputs: ["stdioSources"] }),
      "test-type-ownership": on({ inputs: ["stdioSources"] }),
      "test-production-preparation": on({ inputs: ["stdioSources"] }),
      "test-native-oracles": on({ inputs: ["stdioSources", "rustWorkspace"] }),
      "test-native-drawing-reader": on({ inputs: ["stdioSources", "rustWorkspace"] }),
    },
  };

  const puzzleRefs = fixtureReferences("🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/🧪️tests/🧫️fixtures/🔮️ownership/🔣️.json");
  files[PUZZLE_RETAINED] = { tasks: { "test-ownership": on({ inputs: ["default", "sharedGlobals", ...puzzleRefs] }) } };
  off(PUZZLE_RETAINED, ["test-native"], "orchestrator: re-enters the build tool with --skip-cache for every artifact test project (those tests are cached themselves)");

  const spaceRefs = fixtureReferences("🌎️hub/🧩️compositions/🪐️space/🧪️testing/🧪️tests/🧫️fixtures/🔮️ownership/🔣️.json");
  files[SPACE_TESTING] = { tasks: { "test-ownership": on({ inputs: ["default", "sharedGlobals", ...spaceRefs] }) } };
  off(SPACE_TESTING, ["test-native"], "orchestrator: re-enters the build tool with --skip-cache for every Space native test project (those tests are cached themselves)");

  const censusFixture = JSON.parse(readFileSync(ROOT + "🌎️hub/🧩️compositions/🪐️space/🧪️tests/🧫️fixtures/🪶️snapshot-owner-census/🔣️.json", "utf8")) as { owners: { sqlPath: string }[] };
  const censusInputs = ["default", "^default", "{workspaceRoot}/🌎️hub/🧩️compositions/🪐️space/🧪️tests/🧫️fixtures/🪶️snapshot-owner-census/**/*", ...[...new Set(censusFixture.owners.map(owner => owner.sqlPath))].sort().map(path => `{workspaceRoot}/${path}`)];
  files[SPACE_RS] = { tasks: { "snapshot-owner-census-source": on({ inputs: censusInputs }), "snapshot-owner-census-native": on({ inputs: censusInputs }) } };

  files[SPACE_COMPUTE] = { tasks: { test: on() } };

  const wgpuGeneral = /general="([^"]+)"/u.exec(readFileSync(ROOT + "🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🧪️tests/🟦️.ts", "utf8"))![1]!;
  files[WGPU] = { tasks: { test: on({ inputs: ["default", "^default", `{workspaceRoot}/${wgpuGeneral}/**/*`] }) } };
  off(WGPU, ["journey-check", "creation-check", "collaboration-check", "native-guest-journey-check"], "live: boots a hub and mounts the staged native runtime");
  off(WGPU, ["collaboration-acceptance"], "live two-human collaboration journey with acceptance records");

  off(PUZZLE_RS, ["canonical-architecture"], "policy.targetDefaults pins canonical-architecture uncached");

  files[DEMO] = { tasks: { "activate-lanes-prototype": on() } };
  off(DEMO, ["build"], "vite site build lands in the leased products store and downloads map tiles (network, see result-bzl-man)");
  off(DEMO, ["test", "test-runtime-catalog"], "reads repository-wide files (plugin descriptors, sibling manifests, ticket folders) that no project-scoped input set covers (see result-bzl-man)");
  off(DEMO, ["canonical-architecture"], "policy.targetDefaults pins canonical-architecture uncached");
  off(DEMO, ["prepare-e2e", "test-e2e"], "e2e: browser and ports");

  const COURSE = "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json";
  const PROCTOR = "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json";
  const PLAY = "🏢️semio-tech/🎡️play/📋️project.json";
  off(COURSE, ["preflight", "ci-browser-prepare"], "host introspection and system package installation (bzl-man)");
  off(COURSE, ["build", "publish"], "vite site output lands in the leased products store; test-pinned cache:false (bzl-man)");
  off(COURSE, ["check", "docker-image-build", "docker-image-check", "docker-image-publish", "docker-stack-check", "docker-stack-bundle", "deploy-check"], "builds and launches the proctor or talks to the Docker daemon (bzl-man)");
  off(COURSE, ["test-e2e"], "e2e: browser, ports, processes");
  off(PROCTOR, ["check", "rebuild", "health", "backup", "restore", "erase", "prune", "capacity"], "operator verbs on the dev data directory; capacity is a timing gate (bzl-man)");
  off(PLAY, ["build"], "orchestrator publishing pages into the leased products store (bzl-man)");
  off(PLAY, ["test", "test-release", "test-release-routing", "release-verify"], "reads repository-wide files or the release pages of the products store, outside declared inputs (bzl-man)");
  off(PLAY, ["prepare-e2e", "test-e2e", "test-e2e-enduser"], "e2e: browser, ports, processes");
  off("👴️leutwiler/💤️realparts-of-powers-z-n/📋️project.json", ["deps"], "installs the Lean toolchain");
  return { files, keepOff };
}
