export type Manual = { off?: string; skip?: string; outputs?: string[]; inputs?: (base: unknown[]) => unknown[]; forceInputs?: boolean };
export const MANUAL: Record<string, Manual> = {};
const off = (reason: string, ...ids: string[]) => { for (const id of ids) MANUAL[id] = { off: reason }; };
const skip = (reason: string, ...ids: string[]) => { for (const id of ids) MANUAL[id] = { skip: reason }; };
const out = (outputs: string[], ...ids: string[]) => { for (const id of ids) MANUAL[id] = { ...(MANUAL[id] ?? {}), outputs }; };

const LIVE = "live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs";
const HOST = "host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs";
const LAUNCH = "launcher: starts a long-lived tool or UI, no cacheable result";
const WRITER = "mutating writer: rewrites tracked repository state in place";

off("measures live event-loop deadlines and chained scheduling on the current host", "@semio-tech/framework-async:twin");
off("downloads fonts over the network", "@semio-tech/ui-styling-tokens:fonts");
off("tree-wide projection writer: rewrites about 2600 generated schema leaves across the workspace, outputs unbounded", "@semio-tech/plugin-registry:surface-schema");
off("rebuilds and republishes every generated catalog artifact in place (mutating)", "@semio-tech/plugin-registry:rebuild-all");
off("emits descriptors into the leased products store on every run; consumers re-run it", "@semio-tech/os-plugin-describe-rs:describe");
off("publishes a produced logical owner into the artifact store (mutating)", "@semio-tech/print-rs:produce-logical-owner");
off("writes declaration files into the leased artifact store", "repo:emit-typescript-declarations");

for (const t of ["program-matrix", "tool-run-matrix", "two-human", "two-human-viewer", "two-human-cross-undo", "connection-budget", "idle-budget", "interaction-latency", "memory-soak", "boot-budget", "time-travel", "parity-journey", "cold-boot-check-s", "s-host-foreign-kind-s", "s-host-pinch-diagram-contrast-s", "plugin-size"])
  off(LIVE, "@semio-tech/framework-os-dev:" + t);
for (const t of ["browser-dock-acceptance", "browser-media-acceptance", "browser-embedded-acceptance", "native-scale"]) off(LIVE + " (browser or GPU)", "@semio-tech/framework-renderer-wgpu:" + t);
for (const t of ["git-show", "git-blame", "git-diff", "git-history", "git-status", "open-source", "debuggers"]) off("reads live git state or opens the editor; not a function of declared inputs", "@semio-tech/repo-developer-tools:" + t);
off(LAUNCH, "@semio-tech/repo-cli-rs:repo", "@semio-tech/repo-cli-rs:mcp", "@semio-tech/repo-dashboard-rs:run", "@semio-tech/repo-dashboard-rs:preferences", "@semio-tech/repo-mcp-rs:run", "@semio-tech/repo-coordinator-rs:run");
off("mutates Cargo workspace state", "repo:cargo-prepare", "repo:cargo-synchronize", "repo:cargo-provenance-repair");
off("debug bundle inspection of the live bundle store", "repo:debug-bun-bundle");
off("prints digests of the whole workspace source inventory; unbounded input set", "repo:digest-source-inventory", "repo:digest-source-closure", "repo:digest-cargo-lock");
off(WRITER, "@semio-tech/repo-lib:workspaces-write", "@semio-tech/cargo-workspaces:members-write");
off("runs the Go toolchain over Go modules outside the declared inputs", "@semio-tech/repo-lib:go-test");
off("probes the live nx owner-argument path", "@semio-tech/repo-lib:probe-nx-owner-arguments");
off("wrapper that spawns the owner command; result is the child's", "@semio-tech/repo-lib:owner-command");
off("reads schema trees across the whole workspace; unbounded input set", "@semio-tech/repo-lib:schema-read-inventory");
off("builds and runs Go executables outside the declared inputs", "@semio-tech/repo-lib:test-go-executable-acceptance");
off("bootstrap acceptance: installs and probes tooling on the host", "@semio-tech/repo-lib:test-cross-platform-bootstrap", "@semio-tech/repo-lib:test-dependency-policy-bootstrap");
off("exercises the nx cache directory itself", "repo-next-storage:test-nx-cache");
off("dashboard UI journeys against a running dashboard", "@semio-tech/repo-dashboard-rs:test-journeys", "@semio-tech/repo-dashboard-rs:test-smoke");
off("probe of the live test domain", "@semio-tech/repo-test-domain:test-probe");

for (const t of ["test-cargo-build-lease", "test-next-runtime", "test-next-dev-runtime", "test-graph-cache-waiter", "test-bazel-storage-runtime", "test-bazel-storage", "test-tools-partition", "test-daemon-journal", "test-daemon-journal-workspace", "test-storage-budget", "test-leases-partition", "test-lease-bootstrap", "test-lease-queue", "test-javascript-storage", "test-runtime-verification-storage", "test-runtime-verification-storage-runtime", "test-rustup-toolchain", "test-bootstrap-original-invocation-source", "test-runtime-bootstrap"])
  off(HOST, "repo:" + t);
for (const t of ["test-process-tree-termination", "test-budget", "test-execution-budget", "test-resource-leases", "test-native-storage-platform", "test-cargo-test-leases", "test-runtime-environment", "test-cargo-build-lease", "test-compiler-capacity", "test-compiler-capacity-inheritance", "test-compiler-capacity-custodian", "test-exact-cargo-storage", "test-resource-lease-waits"])
  off(HOST, "@semio-tech/framework-process:" + t);
off(HOST, "@semio-tech/framework-os-kernel:test-directory-lease-fixture");

skip("owner-declared fingerprint of the live toolchain; leave to its owner", "repo:toolchain");

out([], "@semio-tech/framework-playbook-playbook-rs:generation-root-policy", "@semio-tech/plugin-registry:generate", "@semio-tech/framework-os-dev:channel-version-generate", "@semio-tech/print:generate-viz", "@semio-tech/print:generate-font-metrics", "@semio-tech/print:build", "@semio-tech/print-viz-inference:build", "@semio-tech/repo-dashboard-rs:compile", "@semio-tech/repo-test-runner-go:build", "@semio-tech/repo-metrics-go:build");
