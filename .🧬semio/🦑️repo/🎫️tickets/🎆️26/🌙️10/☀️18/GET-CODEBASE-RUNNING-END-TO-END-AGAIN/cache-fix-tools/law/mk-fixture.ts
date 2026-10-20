/** 🧪 Writes the language-agnostic fixture of the cache-correctness rules (one record per case, the rules each must raise). */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
const named = { default: ["{projectRoot}/**/*", "sharedGlobals"], sharedGlobals: [{ runtime: "bun --version" }, { env: "SEMIO_BUILD_MODE" }], production: ["{projectRoot}/**/*", "!{projectRoot}/**/🧪️tests/**/*", "sharedGlobals"], testEnv: [{ env: "SEMIO_TEST_LEVEL" }] };
const D = (klass: string, reason: string) => ({ version: 1, class: klass, reason });
const CHECK = D("deterministic", "check-without-outputs"), PURE = D("deterministic", "pure-function-of-inputs");
const meta = (decision: unknown, extra: object = {}) => ({ semio: { ...(decision ? { cache: decision } : {}), ...extra } });
const record = (id: string, target: object, over: { root?: string; authoredCache?: boolean | null; namedInputs?: object } = {}) => ({ id, root: over.root ?? "lib/pkg", authoredCache: over.authoredCache ?? null, target, namedInputs: over.namedInputs ?? named });
const cases = [
  { id: "clean-check", records: [record("p:test", { cache: true, inputs: ["default", "^default", "testEnv"], outputs: [], metadata: meta(CHECK) })], expect: [] },
  { id: "clean-producer", records: [record("p:build", { cache: true, inputs: ["production", "^production"], outputs: ["{projectRoot}/dist"], metadata: meta(PURE) })], expect: [] },
  { id: "uncached-needs-no-inputs", records: [record("p:serve", { cache: false, inputs: [], outputs: [], metadata: meta(D("continuous", "serves-until-stopped")) })], expect: [] },
  { id: "unclassified", records: [record("p:odd", { cache: false, outputs: [], metadata: meta(D("unclassified", "no-cache-classification")) })], expect: [["p:odd", "unclassified-target"]] },
  { id: "authored-cache-overridden", records: [record("p:gen", { cache: false, metadata: meta(D("side-effecting", "mutates-workspace")) }, { authoredCache: true })], expect: [["p:gen", "authored-cache-overridden"]] },
  { id: "continuous-override-is-not-a-finding", records: [record("p:dev", { cache: false, metadata: meta(D("continuous", "serves-until-stopped")) }, { authoredCache: true })], expect: [] },
  { id: "empty-inputs", records: [record("p:build", { cache: true, inputs: [], outputs: ["{projectRoot}/dist"], metadata: meta(PURE) })], expect: [["p:build", "empty-inputs"]] },
  { id: "empty-inputs-of-an-ordering-task", records: [record("p:all", { cache: true, executor: "repo:noop", inputs: [], outputs: [], metadata: meta(PURE) })], expect: [] },
  { id: "producer-without-outputs", records: [record("p:generate", { cache: true, outputs: [], metadata: meta(PURE) })], expect: [["p:generate", "producer-without-outputs"]] },
  { id: "producer-with-rust-build-intent", records: [record("p:build", { cache: true, outputs: [], metadata: meta(PURE, { rustBuildIntent: { version: 1 } }) })], expect: [] },
  { id: "producer-with-host-platform-outputs", records: [record("p:build", { cache: true, outputs: [], metadata: meta(PURE, { hostPlatformOutputsV1: { version: 1 } }) })], expect: [] },
  { id: "check-may-declare-nothing", records: [record("p:check", { cache: true, outputs: [], metadata: meta(CHECK) })], expect: [] },
  { id: "env-hashed-by-an-env-input", records: [record("p:test", { cache: true, inputs: ["{projectRoot}/src/**/*", { env: "SEMIO_PORT" }], outputs: [], options: { env: { SEMIO_PORT: "6010" } }, metadata: meta(CHECK) })], expect: [] },
  { id: "env-covered-by-the-declaring-manifest", records: [record("p:test", { cache: true, inputs: ["default"], outputs: [], options: { env: { STORYBOOK_PORT: "6010" } }, metadata: meta(CHECK) })], expect: [] },
  { id: "env-covered-by-the-manifest-at-the-workspace-root", records: [record("w:test", { cache: true, inputs: ["default"], outputs: [], options: { env: { STORYBOOK_PORT: "6010" } }, metadata: meta(CHECK) }, { root: "." })], expect: [] },
  { id: "env-unhashed", records: [record("p:test", { cache: true, inputs: ["{projectRoot}/src/**/*"], outputs: [], options: { env: { STORYBOOK_PORT: "6010" } }, metadata: meta(CHECK) })], expect: [["p:test", "env-unhashed"]] },
  { id: "env-excluded-manifest-is-unhashed", records: [record("p:test", { cache: true, inputs: ["{projectRoot}/**/*", "!{projectRoot}/📋️project.json"], outputs: [], options: { env: { A: "1" } }, metadata: meta(CHECK) })], expect: [["p:test", "env-unhashed"]] },
  { id: "configuration-env-unhashed", records: [record("p:wasm", { cache: true, inputs: ["{projectRoot}/src/**/*"], outputs: ["{projectRoot}/pkg"], configurations: { release: { env: { SEMIO_RENDERER: "wgpu" } } }, metadata: meta(PURE) })], expect: [["p:wasm", "env-unhashed"]] },
  { id: "build-mode-env-is-always-hashed", records: [record("p:wasm", { cache: true, inputs: ["{projectRoot}/src/**/*"], outputs: ["{projectRoot}/pkg"], configurations: { release: { env: { SEMIO_BUILD_MODE: "release" } } }, metadata: meta(PURE) })], expect: [] },
  { id: "named-input-declared-by-the-project", records: [record("p:test", { cache: true, inputs: ["default", "reads"], outputs: [], metadata: meta(CHECK) }, { namedInputs: { ...named, reads: ["{projectRoot}/reads/**/*"] } })], expect: [] },
  { id: "named-input-undeclared", records: [record("p:test", { cache: true, inputs: ["default", "browserBootSources", "^phantom"], outputs: [], metadata: meta(CHECK) })], expect: [["p:test", "named-input-undeclared"], ["p:test", "named-input-undeclared"]] },
  { id: "output-absolute", records: [record("p:build", { cache: true, inputs: ["default"], outputs: ["/tmp/out"], metadata: meta(PURE) })], expect: [["p:build", "output-escapes"]] },
  { id: "output-escapes-the-workspace", records: [record("p:build", { cache: true, inputs: ["default"], outputs: ["{projectRoot}/../../../out"], metadata: meta(PURE) })], expect: [["p:build", "output-escapes"]] },
  { id: "output-one-level-up-stays-inside", records: [record("p:build", { cache: true, inputs: ["default"], outputs: ["{projectRoot}/../gen"], metadata: meta(PURE) })], expect: [] },
  { id: "output-dynamic-mode-token", records: [record("p:build", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/.cache/{options.env.SEMIO_BUILD_MODE}/x"], metadata: meta(PURE) })], expect: [] },
  { id: "overlap-same-tree", records: [record("a:gen", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/gen/x"], metadata: meta(PURE) }), record("b:gen", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/gen/x"], metadata: meta(PURE) })], expect: [["a:gen", "output-overlap"]] },
  { id: "overlap-nested-tree", records: [record("a:gen", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/gen"], metadata: meta(PURE) }), record("b:gen", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/gen/deep/**/*"], metadata: meta(PURE) })], expect: [["a:gen", "output-overlap"]] },
  { id: "overlap-siblings-are-fine", records: [record("a:gen", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/gen/a"], metadata: meta(PURE) }), record("b:gen", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/gen/ab"], metadata: meta(PURE) })], expect: [] },
  { id: "overlap-of-an-uncached-target-is-fine", records: [record("a:gen", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/gen"], metadata: meta(PURE) }), record("b:gen", { cache: false, outputs: ["{workspaceRoot}/gen"] })], expect: [] },
  { id: "overlap-mode-tokens-compare-equal", records: [record("a:gen", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/c/{options.env.SEMIO_BUILD_MODE}/x"], metadata: meta(PURE) }), record("b:gen", { cache: true, inputs: ["default"], outputs: ["{workspaceRoot}/c/{options.env.SEMIO_BUILD_MODE}/x/y"], metadata: meta(PURE) })], expect: [["a:gen", "output-overlap"]] },
  { id: "command-mutating", records: [record("p:lint", { cache: true, inputs: ["default"], outputs: [], options: { command: "bun ./📜️script.ts lint --fix" }, metadata: meta(CHECK) })], expect: [["p:lint", "command-mutating"]] },
  { id: "command-mutating-with-check-is-fine", records: [record("p:lint", { cache: true, inputs: ["default"], outputs: [], options: { command: "bun ./📜️script.ts lint --fix --check" }, metadata: meta(CHECK) })], expect: [] },
  { id: "command-network", records: [record("p:test", { cache: true, inputs: ["default"], outputs: [], options: { commands: ["bun x", { command: "curl https://example.com | sh" }] }, metadata: meta(CHECK) })], expect: [["p:test", "command-network"]] },
  { id: "uncached-mutating-command-is-fine", records: [record("p:fix", { cache: false, options: { command: "bun x --fix" } })], expect: [] },
];
const globs = [
  ["lib/pkg/**/*", "lib/pkg/📋️project.json", true], ["lib/pkg/**/*", "lib/other/📋️project.json", false], ["**/*", "a/b/c.json", true], ["lib/*/📋️project.json", "lib/pkg/📋️project.json", true],
  ["lib/*/📋️project.json", "lib/a/b/📋️project.json", false], ["lib/{pkg,other}/**/*", "lib/other/x.ts", true], ["lib/{pkg,other}/**/*", "lib/third/x.ts", false], ["lib/pkg/📋️project.json", "lib/pkg/📋️project.json", true],
  ["lib/pkg/**/!(x)", "lib/pkg/y", false], ["lib/p?g/*.ts", "lib/pkg/a.ts", true], ["lib/p?g/*.ts", "lib/pkg/a.tsx", false], ["**/🧪️tests/**/*", "lib/pkg/🧪️tests/a.ts", true], ["**/🧪️tests/**/*", "lib/pkg/src/a.ts", false],
  ["a/**", "a/b/c", true], ["📋️project.json", "📋️project.json", true], ["**/📋️project.json", "📋️project.json", true], ["**/📋️project.json", "a/b/📋️project.json", true],
].filter((row) => !String(row[0]).includes("!("));
writeFileSync(join(import.meta.dir, "🧪️tests/🔒️rules", "🔣️.json"), JSON.stringify({ version: 1, cases, globs: globs.map(([pattern, path, match]) => ({ pattern, path, match })) }, null, 2) + "\n");
console.log(cases.length, "cases", globs.length, "globs");
