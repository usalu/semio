/**
 * 🧾️ M-1a hand-written owner declarations (ticket input, not codebase code).
 *
 * Each entry names one owner manifest and what `m1a-apply.ts` inserts into it: `project` is the value of
 * `metadata.semio.dashboard`, `targets[<name>]` the value of `targets.<name>.metadata.semio.dashboard`, and
 * `removeEnv` the dead `options.env` keys taken out of the listed targets. The manifests are the source of
 * truth once applied; `m1a-coverage.ts` reads the manifests, never this file.
 *
 * @see ./fleet-plan.md §2.2
 * @see ../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json
 */
export type Declared = Readonly<Record<string, unknown>>;
export type OwnerEdit = Readonly<{
  file: string;
  name: string;
  project?: Declared;
  targets?: Readonly<Record<string, Declared>>;
  removeEnv?: Readonly<Record<string, readonly string[]>>;
}>;

const HUB = "http://127.0.0.1:8787";
const SHELL = "http://127.0.0.1:6070/";
const storybook = { ready: { port: 6010, portEnv: "STORYBOOK_PORT" } };
const inspector = { ready: { port: 6274, portEnv: "CLIENT_PORT" } };
const hubReady = { port: 8787, portEnv: "OS_HUB_PORT" };
const locale = (how: (value: string) => Declared): Declared => ({ id: "locale", kind: "choice", values: ["en", "de"].map((id) => ({ id, ...how(id) })) });
const localeFlag = locale((id) => ({ args: ["--locale", id] }));
const backend = (ids: readonly string[], rest: Declared): Declared => ({ id: "backend", kind: "choice", ...rest, values: ids.map((id) => ({ id, args: [id] })) });
const sqliteGroup = ["@semio-tech/block-2d-rs", "@semio-tech/block-3d-rs", "@semio-tech/block-5d-rs", "@semio-tech/puzzle-3d-rs", "@semio-tech/process-process3d-rs", "@semio-tech/writer-writer-rs", "@semio-tech/mathematical-equation-rs", "@semio-tech/gis-gisterrain-rs", "@semio-tech/gis-gismap-rs", "@semio-tech/trinity-jack-rs", "@semio-tech/layout-layout-rs", "@semio-tech/procedural-generation2d-rs", "@semio-tech/procedural-generation3d-rs", "@semio-tech/imperative-procedure-rs", "@semio-tech/playbook-playbook-rs", "@semio-tech/puzzle-5d-rs", "@semio-tech/trinity-rewriting-rs", "@semio-tech/dag-dag-rs"];
const processExtensions = ["@semio-tech/process-extension-wood-rust", "@semio-tech/process-extension-metal-rust", "@semio-tech/process-extension-concrete-rust", "@semio-tech/process-extension-robotic-rust"];
const sourcingExtensions = ["@semio-tech/sourcing-extension-beams-rust", "@semio-tech/sourcing-extension-slabs-rust", "@semio-tech/sourcing-extension-windows-rust"];

export const OWNER_EDITS: readonly OwnerEdit[] = [
  {
    file: "📋️project.json",
    name: "workspace",
    project: {
      parameters: [
        {
          id: "cache",
          kind: "choice",
          appliesTo: { verbs: ["test", "check", "build", "verify", "lint", "serve", "activate", "task"], playground: false },
          default: "use",
          values: [{ id: "use" }, { id: "skip-local", nxFlags: ["--skip-nx-cache"] }, { id: "skip-all", nxFlags: ["--skip-nx-cache", "--skip-remote-cache"] }],
        },
        {
          id: "test-level",
          kind: "choice",
          appliesTo: { verbs: ["test", "verify", "check"], playground: false },
          values: ["quick", "long", "exhaustive"].map((id) => ({ id, env: { SEMIO_TEST_LEVEL: id } })),
        },
        { id: "dependencies", kind: "flag", appliesTo: { verbs: ["test", "verify", "check", "task"], playground: false }, nxFlags: ["--excludeTaskDependencies"] },
        {
          id: "build-mode",
          kind: "choice",
          appliesTo: { verbs: ["test", "verify", "build", "task"], playground: false },
          values: [{ id: "ship", env: { SEMIO_BUILD_MODE: "ship" }, nxFlags: ["--skip-nx-cache", "--skip-remote-cache"] }],
        },
        {
          id: "nextest-output",
          kind: "choice",
          appliesTo: { verbs: ["test"], playground: false },
          values: ["immediate", "immediate-final", "final", "never"].map((id) => ({ id, env: { NEXTEST_SUCCESS_OUTPUT: id } })),
        },
        { id: "cargo-jobs", kind: "text", appliesTo: { verbs: ["test", "check", "task"], playground: false }, valueEnv: "CARGO_BUILD_JOBS" },
        { id: "build-budget", kind: "text", appliesTo: { verbs: ["test", "serve", "activate", "task"], playground: false }, valueEnv: "SEMIO_BUILD_BUDGET_MS" },
      ],
      tools: [
        { id: "repo-mcp", verb: "dev", command: ["bun", "📜️script.ts", "dev", "mcp", "stdio"], continuous: true, parameters: [{ id: "client", kind: "text", default: "client", valuePositional: true }] },
        { id: "os-mcp-stdio", verb: "dev", command: ["bun", "📜️script.ts", "dev", "mcp", "stdio", "os"], continuous: true },
        { id: "os-mcp-http", verb: "dev", command: ["bun", "nx", "run", "workspace:dev", "--", "mcp", "http", "os"], continuous: true },
        { id: "mcp-inspector-os", verb: "dev", command: ["bun", "x", "@modelcontextprotocol/inspector", "--config", ".mcp.json", "--server", "semio"], continuous: true, ...inspector },
        {
          id: "bun-test",
          verb: "test",
          command: ["bun", "nx", "exec", "--projects={project}", "--", "bun", "test"],
          parameters: [
            { id: "project", kind: "text", default: "workspace" },
            { id: "file", kind: "text", required: true, valuePositional: true },
            { id: "build-mode", kind: "choice", values: [{ id: "ship", env: { SEMIO_BUILD_MODE: "ship" } }] },
          ],
        },
        { id: "native-cargo", verb: "run", command: ["bun", "nx", "exec", "--projects=workspace", "--", "bun", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts", "native", "cargo"] },
        { id: "gemini", verb: "run", command: ["gemini", "--yolo"], continuous: true },
        { id: "kiro", verb: "run", command: ["kiro-cli", "chat", "--trust-all-tools"], continuous: true },
        { id: "f3d", verb: "run", command: ["f3d"], continuous: true },
        { id: "gitkraken", verb: "run", command: ["gitkraken", "--path", "{workspace}"], continuous: true },
      ],
      compounds: [
        { id: "s-with-hub", verb: "dev", stop: "together", members: [{ run: "os-hub:dev" }, { run: "playground:s", parameters: { renderer: "react" } }] },
        { id: "s-with-os-mcp", verb: "dev", stop: "together", members: [{ run: "tool:workspace/os-mcp-http" }, { run: "playground:s", parameters: { renderer: "react" } }] },
        {
          id: "s-users-with-hub",
          verb: "dev",
          stop: "together",
          members: [{ run: "os-hub:dev" }, { run: "playground:s", parameters: { renderer: "react", "user-slot": 1 } }, { run: "playground:s", parameters: { renderer: "react", "user-slot": 2 } }],
        },
      ],
      groups: [
        { id: "process-extension-catalogs", target: "describe", projects: processExtensions },
        { id: "sourcing-extension-catalogs", target: "describe", projects: sourcingExtensions },
        { id: "extension-catalogs", target: "describe", projects: [...processExtensions, ...sourcingExtensions] },
        { id: "process-extension-tests", target: "test", projects: processExtensions },
        { id: "sourcing-extension-tests", target: "test", projects: sourcingExtensions },
        { id: "stdio-artifact-tests", target: "test", projects: ["@semio-tech/stdio-*-rs", "!@semio-tech/stdio-artifact-contract-rs"] },
        { id: "puzzle-spatial-tests", target: "test", projects: ["@semio-tech/puzzle-3d-rs", "@semio-tech/puzzle-5d-rs"] },
        { id: "snapshot-sqlite-parent-baselines", target: "test-snapshot-sqlite-native", projects: sqliteGroup },
        { id: "snapshot-sqlite-native", target: "test-snapshot-sqlite-native", projects: ["*"] },
        { id: "snapshot-sqlite-source", target: "test-snapshot-sqlite-source", projects: ["*"] },
        { id: "dag-actor-wasm", target: "wasm", projects: ["semio-framework-os-flow-core", "@semio-tech/framework-surface-rs", "@semio-tech/s-cad-composition-rs", "@semio-tech/s-spatial-kernel-semio-session-rs"] },
      ],
    },
    targets: {
      "dev-storybook": storybook,
      "dev-storybook-ui": storybook,
      "dev-storybook-puzzle": storybook,
      "dev-storybook-puzzle-2d": storybook,
      "dev-storybook-styling": storybook,
      "dev-storybook-puzzle-3d": storybook,
      "dev-storybook-puzzle-5d": storybook,
      "dev-storybook-framework": storybook,
      "dev-storybook-framework-hosts": storybook,
      "dev-storybook-framework-os": storybook,
      "dev-storybook-infinite": storybook,
      "dev-storybook-cad": storybook,
      "dev-storybook-animate": storybook,
      "dev-mcp": inspector,
      "dev-mcp-repo": inspector,
      verify: { parameters: [{ id: "check", kind: "text", valuePositional: true }] },
    },
  },
  {
    file: "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json",
    name: "@semio-tech/repo-dashboard-rs",
    targets: {
      run: {
        parameters: [
          { id: "repo-implementation", kind: "choice", values: [{ id: "rust", env: { SEMIO_REPO_IMPLEMENTATION: "rust" } }, { id: "go", env: { SEMIO_REPO_IMPLEMENTATION: "go", GOWORK: "{workspace}/go.work" } }] },
        ],
      },
      daemon: { parameters: [{ id: "action", kind: "choice", default: "status", values: ["start", "stop", "status", "attach"].map((id) => ({ id, args: [id] })) }] },
      preferences: { parameters: [{ id: "action", kind: "choice", default: "show", values: ["show", "set"].map((id) => ({ id, args: [id] })) }] },
    },
  },
  {
    file: "🌎️hub/📦️packages/🦀️rust/📋️project.json",
    name: "os-hub",
    targets: {
      dev: { ready: { ...hubReady, path: "/admin" } },
      "dev-postgres": { ready: { ...hubReady, path: "/admin" }, parameters: [{ id: "data", kind: "text", default: "{workspace}/.🧬semio/🌐hub/hub-dev-postgres", valueEnv: "OS_HUB_DATA" }] },
      "dev-neo4j": { ready: { ...hubReady, path: "/admin" }, parameters: [{ id: "data", kind: "text", default: "{workspace}/.🧬semio/🌐hub/hub-dev-neo4j", valueEnv: "OS_HUB_DATA" }] },
      "dev-secure-suite": { ready: hubReady },
      "dev-secure-native": { ready: hubReady },
      "dev-secure-mcp": { ready: hubReady },
      "dev-secure-admin": { ready: hubReady },
      "trusted-catalog-preflight": { parameters: [{ id: "packages", kind: "text", valueFlag: "--packages" }] },
      "trusted-catalog-bootstrap": { parameters: [{ id: "packages", kind: "text", valueFlag: "--packages" }] },
    },
  },
  {
    file: "🌎️hub/📦️packages/🟦️typescript/📋️project.json",
    name: "os-hub-ts",
    targets: {
      "two-client-e2e": { parameters: [backend(["sqlite", "postgres", "neo4j"], { required: true })] },
      "document-growth-e2e": { parameters: [backend(["sqlite", "postgres", "neo4j"], { required: true })] },
      "backend-up": { parameters: [backend(["all", "postgres", "neo4j"], { default: "all" })] },
      "backend-down": { parameters: [backend(["all", "postgres", "neo4j"], { default: "all" })] },
      "backend-status": { parameters: [backend(["all", "postgres", "neo4j"], { default: "all" })] },
      "backend-run": { parameters: [backend(["postgres", "neo4j"], { required: true })] },
      "residency-watch": { requires: ["os-hub:dev"], parameters: [{ id: "hub", kind: "text", default: HUB, required: true, valueFlag: "--hub" }] },
      "hub-freshness": { requires: ["os-hub:dev"], parameters: [{ id: "hub", kind: "text", default: HUB, required: true, valueFlag: "--hub" }] },
      "agent-ceiling-check": { requires: ["os-hub:dev"], parameters: [{ id: "hub", kind: "text", default: HUB, required: true, valueFlag: "--hub" }, localeFlag] },
    },
  },
  {
    file: "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📋️project.json",
    name: "os-hub-admin",
    targets: { dev: { ready: { port: 8790, portEnv: "OS_HUB_ADMIN_DEV_PORT" }, requires: ["os-hub:dev"] } },
  },
  {
    file: "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json",
    name: "@semio-tech/framework-os-dev",
    targets: {
      "program-matrix": { parameters: [{ id: "serve", kind: "text", default: SHELL, required: true, valueFlag: "--serve" }, localeFlag, { id: "roles", kind: "text", valueFlag: "--roles" }] },
      "tool-run-matrix": { parameters: [{ id: "serve", kind: "text", default: SHELL, required: true, valueFlag: "--serve" }, localeFlag] },
      "io-matrix": { parameters: [{ id: "serve", kind: "text", default: SHELL, required: true, valueFlag: "--serve" }, localeFlag] },
      "hub-document-sweep": {
        requires: ["os-hub:dev"],
        parameters: [{ id: "serve", kind: "text", default: "http://127.0.0.1:6071/", required: true, valueFlag: "--serve" }, { id: "hub", kind: "text", default: HUB, required: true, valueFlag: "--hub" }, localeFlag],
      },
      "two-human": {
        requires: ["os-hub:dev"],
        parameters: [{ id: "hub", kind: "text", default: HUB, required: true, valueFlag: "--hub" }, { id: "serve", kind: "text", default: SHELL, required: true, valueFlag: "--serve" }, { id: "users", kind: "text", valueFlag: "--users" }, localeFlag],
      },
      "connection-budget": { requires: ["playground:s"], parameters: [{ id: "serve", kind: "text", default: SHELL, required: true, valuePositional: true }] },
      "idle-budget": { requires: ["playground:s"], parameters: [{ id: "serve", kind: "text", default: SHELL, required: true, valuePositional: true }] },
      "memory-soak": { requires: ["playground:s"], parameters: [{ id: "serve", kind: "text", default: SHELL, required: true, valuePositional: true }] },
      "interaction-latency": { requires: ["playground:s"], parameters: [{ id: "serve", kind: "text", default: SHELL, required: true, valuePositional: true }] },
      "time-travel": { parameters: [{ id: "renderer", kind: "choice", default: "react", values: ["react", "wgpu"].map((id) => ({ id, args: ["--renderer", id] })) }] },
      "s-host-foreign-kind-s": { requires: ["playground:s"] },
      "s-host-pinch-diagram-contrast-s": { requires: ["playground:s"] },
    },
  },
  {
    file: "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json",
    name: "@semio-tech/framework-os-kernel",
    targets: {
      "wal-writer-fence-live": { parameters: [{ id: "lane", kind: "choice", values: ["sqlite", "postgres", "neo4j"].map((id) => ({ id, args: [id] })) }] },
      "reopen-storm-check": { parameters: [{ id: "law", kind: "choice", values: ["all", "unit", "fs", "sqlite", "postgres", "neo4j"].map((id) => ({ id, args: [id] })) }] },
    },
  },
  {
    file: "✏️s/🧑‍💻dev/💡️services/📦️packages/🦀️rust/📋️project.json",
    name: "@semio-tech/s-services-native",
    targets: {
      "live-agent-loop-check": { parameters: [{ id: "serve", kind: "text", valueEnv: "S_OS_MCP_LIVE_SHELL_URL" }, locale((id) => ({ env: { S_OS_MCP_LIVE_LOCALE: id } }))] },
      "user-path-check": {
        requires: ["os-hub:dev", "playground:s"],
        parameters: [{ id: "hub", kind: "text", default: HUB, valueEnv: "OS_MCP_HUB_ORIGIN" }, { id: "serve", kind: "text", default: SHELL, valueEnv: "S_OS_MCP_LIVE_SHELL_URL" }, locale((id) => ({ env: { S_OS_MCP_LIVE_LOCALE: id } }))],
      },
      "security-check": {
        requires: ["os-hub:dev"],
        parameters: [{ id: "hub", kind: "text", default: HUB, valueEnv: "OS_MCP_HUB_ORIGIN" }, { id: "admin-capability", kind: "text", required: true, valueEnv: "OS_HUB_ADMIN_CAPABILITY_FILE" }],
      },
      "inference-quartet-check": { requires: ["os-hub:dev"], parameters: [{ id: "hub", kind: "text", default: HUB, valueEnv: "OS_MCP_HUB_ORIGIN" }] },
    },
  },
  {
    file: "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json",
    name: "@semio-tech/repo-test-domain",
    targets: {
      "acceptance-goal": {
        parameters: [
          { id: "hub", kind: "text", valueFlag: "--hub" },
          { id: "serve", kind: "text", valueFlag: "--serve" },
          { id: "local-serve", kind: "text", valueFlag: "--local-serve" },
          { id: "users", kind: "text", valueFlag: "--users" },
        ],
      },
    },
  },
  {
    file: "🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📋️project.json",
    name: "@semio-tech/print",
    targets: {
      "test-native-grammar": {
        parameters: [
          { id: "phase", kind: "text", valueEnv: "PRINT_NATIVE_GRAMMAR_PHASE" },
          { id: "diagram-family", kind: "text", valueEnv: "PRINT_NATIVE_DIAGRAM_FAMILY" },
          { id: "geo-palette-phase", kind: "text", valueEnv: "PRINT_NATIVE_GEO_PALETTE_PHASE" },
          { id: "geo-planar-phase", kind: "choice", values: ["guard", "admission"].map((id) => ({ id, env: { PRINT_NATIVE_GEO_PLANAR_PHASE: id } })) },
          { id: "geo-planar-source", kind: "choice", values: [{ id: "rust", env: { PRINT_NATIVE_GEO_PLANAR_SOURCE: "rust" } }] },
        ],
      },
    },
  },
  {
    file: "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json",
    name: "@teaching/proctor",
    targets: {
      dev: { ready: { port: 8791, portEnv: "PROCTOR_PORT" } },
      restore: { parameters: [{ id: "file", kind: "text", required: true, valuePositional: true }] },
      erase: {
        parameters: [
          { id: "handle", kind: "text", valueFlag: "--handle" },
          { id: "tag", kind: "text", valueFlag: "--tag" },
          { id: "learner", kind: "text", valueFlag: "--learner" },
          { id: "dry-run", kind: "flag", args: ["--dry-run"] },
        ],
      },
      prune: { parameters: [{ id: "older-than", kind: "text", default: "7d", required: true, valueFlag: "--older-than" }, { id: "dry-run", kind: "flag", args: ["--dry-run"] }] },
    },
  },
  {
    file: "🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📋️project.json",
    name: "@teaching/architecture-quiz",
    project: { compounds: [{ id: "quiz-with-proctor", verb: "dev", stop: "together", members: [{ run: "@teaching/proctor:dev" }, { run: "@teaching/architecture-quiz:dev" }] }] },
    targets: {
      dev: { ready: { port: 6061, portEnv: "TEACHING_ARCHITECTURE_QUIZ_PORT" } },
      "dev-site": { ready: { port: 6061, portEnv: "TEACHING_ARCHITECTURE_QUIZ_PORT" } },
    },
  },
  {
    file: "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json",
    name: "@semio-tech/pets-react",
    targets: {
      dev: { ready: { portEnv: "PETS_STORIES_PORT" }, parameters: [{ id: "port", kind: "text", default: "6069", valueEnv: "PETS_STORIES_PORT" }, { id: "menagerie", kind: "text", valueEnv: "PETS_MENAGERIE" }] },
    },
  },
  {
    file: "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📋️project.json",
    name: "@semio-tech/mit-bestand-praesentation-projektetage",
    targets: { dev: { ready: { port: 6050, portEnv: "PRAESENTATION_PROJEKTETAGE_PORT" } } },
  },
  {
    file: "♻️mit-bestand/🧺️demonstrator/📋️project.json",
    name: "@semio-tech/mit-bestand-demonstrator",
    targets: { dev: { ready: { port: 6029, portEnv: "MIT_BESTAND_DEMONSTRATOR_PORT" } }, serve: { ready: { port: 6029, portEnv: "MIT_BESTAND_DEMONSTRATOR_PORT" } } },
  },
  {
    file: "🏢️semio-tech/🎡️play/📋️project.json",
    name: "@semio-tech/semio-tech-play",
    targets: { dev: { ready: { port: 6033, portEnv: "SEMIO_TECH_PLAY_PORT" } }, serve: { ready: { port: 6033, portEnv: "SEMIO_TECH_PLAY_PORT" } } },
  },
  {
    file: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json",
    name: "@semio-tech/framework-renderer-wgpu",
    targets: { "native-release": { parameters: [{ id: "variant", kind: "text", valuePositional: true }] } },
  },
  {
    file: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📋️project.json",
    name: "@semio-tech/flow-extension-brep-rust",
    targets: { "canonical-architecture": { parameters: [{ id: "oracle-only", kind: "flag", args: ["--oracle-only"] }] } },
  },
  {
    file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📋️project.json",
    name: "@semio-tech/procedural-generation3d-rs",
    targets: { "semantic-wire-check": { parameters: [{ id: "native", kind: "flag", args: ["native"] }] } },
  },
  {
    file: "✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/📋️project.json",
    name: "@semio-tech/cad-js",
    removeEnv: { test: ["CAD_JS_RENDERER_PLAY_PORT"], "test-quick": ["CAD_JS_RENDERER_PLAY_PORT"], "test-long": ["CAD_JS_RENDERER_PLAY_PORT"], "test-exhaustive": ["CAD_JS_RENDERER_PLAY_PORT"] },
  },
];
