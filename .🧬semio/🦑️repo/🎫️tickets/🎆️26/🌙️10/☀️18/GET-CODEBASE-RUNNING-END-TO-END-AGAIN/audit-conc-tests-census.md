# Census of concurrency-unsafe tests (wave 1020e, executor conc-tests)

Scope: owners are the directories holding a project manifest (`📋️project.json`), a Cargo manifest, a `go.mod` or a `package.json` (3,943 owners, index listing only, no repository-wide content grep). Test sources are tracked files below a `🧪️tests` segment or named `*.test.*`, `*.spec.*`, `*_test.go` outside fixture/generated trees: 16451 (ts/tsx/js, go, rs, py, sh); of them 9807 are runnable suites and were scanned, 6644 are provers/helpers that a caller drives (not scanned). Rules: `conc-tests-tools/staged/sb-protocol.json` (23 rules, EN/DE texts). The TypeScript scanner and the Python reference census give the identical finding set on the whole tree (1,042 of 1,042 findings, same file, rule and line).

## Before the staged fixes

errors 54 in 36 files (law-enforced, ratcheted); warnings 988 (reported, not enforced)

| severity | rule | count |
| --- | --- | ---: |
| error | ticket-scratch-root | 21 |
| error | fixed-tmp-dir | 19 |
| error | checkout-write | 6 |
| error | go-unsandboxed-writer | 4 |
| error | persistent-native-state | 2 |
| error | rust-checkout-write | 1 |
| error | fixed-port-bind | 1 |
| warn | fixed-port-url | 227 |
| warn | real-cache-path | 227 |
| warn | fixed-tmp-literal | 170 |
| warn | ticket-state-path | 150 |
| warn | rust-unsandboxed-writer | 96 |
| warn | unsandboxed-writer | 63 |
| warn | checkout-write-named | 27 |
| warn | port-variable-bind | 10 |
| warn | real-home | 9 |
| warn | fixed-port-config | 5 |
| warn | process-global-state | 4 |

## After the staged fixes (post-apply tree, ratchet baseline)

errors 43 in 31 files (law-enforced, ratcheted); warnings 977 (reported, not enforced)

| severity | rule | count |
| --- | --- | ---: |
| error | fixed-tmp-dir | 19 |
| error | ticket-scratch-root | 10 |
| error | checkout-write | 6 |
| error | go-unsandboxed-writer | 4 |
| error | persistent-native-state | 2 |
| error | rust-checkout-write | 1 |
| error | fixed-port-bind | 1 |
| warn | fixed-port-url | 227 |
| warn | real-cache-path | 227 |
| warn | fixed-tmp-literal | 170 |
| warn | ticket-state-path | 139 |
| warn | rust-unsandboxed-writer | 96 |
| warn | unsandboxed-writer | 63 |
| warn | checkout-write-named | 27 |
| warn | port-variable-bind | 10 |
| warn | real-home | 9 |
| warn | fixed-port-config | 5 |
| warn | process-global-state | 4 |

## Findings by product area (before / after, errors and warnings)

| area | errors before | errors after | warnings before | warnings after |
| --- | ---: | ---: | ---: | ---: |
| repo product | 25 | 23 | 563 | 561 |
| framework os product | 2 | 2 | 194 | 194 |
| semio plugins (✏️s) | 11 | 11 | 77 | 77 |
| hub | 1 | 1 | 59 | 59 |
| framework modules | 11 | 2 | 43 | 34 |
| framework other | 0 | 0 | 22 | 22 |
| teaching | 0 | 0 | 18 | 18 |
| semio-tech | 4 | 4 | 8 | 8 |
| root tests | 0 | 0 | 4 | 4 |

## Error-class violators before the fixes (every file, with status)

| rule | n | file (L: repo library/modules, F: framework modules, OS: os product) | status |
| --- | ---: | --- | --- |
| checkout-write | 2 | `L:lib/🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts` | ledger: passes process.cwd() as workspaceRoot to a read-only project inference inside a child script text |
| checkout-write | 3 | `L:lib/🏗️build/📊️reporting/🧪️tests/🟦️.ts` | ledger: publishes build reports into the real diagnostics partition under unique random names and removes them; needs a sandbox workspace root with its own partition |
| checkout-write | 1 | `L:lib/🔍️discovery/🕸️runtime/🧪️tests/🏗️build-producer/🟦️.ts` | ledger: child script text writes next to its own temp directory |
| fixed-port-bind | 1 | `L:lib/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts` | ledger: asserts the declared `--port` argument of a target command; nothing binds |
| fixed-tmp-dir | 1 | `S:plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs` | ledger: Rust test names a constant directory below temp_dir(): collision-free across agents only while the runner points TMPDIR at the per-run sandbox; two tests of one binary may still share the name (make it unique per test, then drop this row) |
| fixed-tmp-dir | 1 | `S:plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | ledger: Rust test names a constant directory below temp_dir(): collision-free across agents only while the runner points TMPDIR at the per-run sandbox; two tests of one binary may still share the name (make it unique per test, then drop this row) |
| fixed-tmp-dir | 4 | `S:plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs` | ledger: Rust test names a constant directory below temp_dir(): collision-free across agents only while the runner points TMPDIR at the per-run sandbox; two tests of one binary may still share the name (make it unique per test, then drop this row) |
| fixed-tmp-dir | 1 | `S:plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | ledger: Rust test names a constant directory below temp_dir(): collision-free across agents only while the runner points TMPDIR at the per-run sandbox; two tests of one binary may still share the name (make it unique per test, then drop this row) |
| fixed-tmp-dir | 2 | `S:plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs` | ledger: Rust test names a constant directory below temp_dir(): collision-free across agents only while the runner points TMPDIR at the per-run sandbox; two tests of one binary may still share the name (make it unique per test, then drop this row) |
| fixed-tmp-dir | 1 | `S:plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🔮️oracles/🧪️tests/🔬️unit/🦀️.rs` | ledger: Rust test names a constant directory below temp_dir(): collision-free across agents only while the runner points TMPDIR at the per-run sandbox; two tests of one binary may still share the name (make it unique per test, then drop this row) |
| fixed-tmp-dir | 1 | `🌎️hub/🧪️tests/🏷️hub-freshness/🟦️.ts` | ledger: probes a path that must stay absent; nothing is written |
| fixed-tmp-dir | 1 | `F:🏃️process/🔌️providers/🧪️tests/🟦️.ts` | ledger: builds provider-map strings only; nothing is written |
| fixed-tmp-dir | 1 | `L:🎛️dashboard/⚙️preferences/🧪️tests/🔬️unit/🦀️.rs` | ledger: Rust test names a constant directory below temp_dir(): collision-free across agents only while the runner points TMPDIR at the per-run sandbox; two tests of one binary may still share the name (make it unique per test, then drop this row) |
| fixed-tmp-dir | 1 | `L:🎛️dashboard/🧭️cli/🧪️tests/🔬️unit/🦀️.rs` | ledger: Rust test names a constant directory below temp_dir(): collision-free across agents only while the runner points TMPDIR at the per-run sandbox; two tests of one binary may still share the name (make it unique per test, then drop this row) |
| fixed-tmp-dir | 2 | `L:🏠️workspace/🧪️tests/🏖️sandbox/🐹️.go` | ledger: observes a path outside the checkout on purpose; nothing is written |
| fixed-tmp-dir | 1 | `L:lib/🏗️build/📏️incremental/🧪️tests/🟦️.ts` | ledger: string/path construction under the shared temp name; the SUT reads it (use testScratchRootV1) |
| fixed-tmp-dir | 1 | `L:lib/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️workspace/🟦️.ts` | ledger: string/path construction under the shared temp name; the SUT reads it (use testScratchRootV1) |
| fixed-tmp-dir | 1 | `L:lib/🧪️tests/🔬️workspace-contract/🧬️mutation-taxonomy/🟦️.ts` | ledger: string/path construction under the shared temp name; the SUT reads it (use testScratchRootV1) |
| go-unsandboxed-writer | 1 | `L:⌨️cli/🧪️tests/🔬️unit/🐹️.go` | ledger: Go unit writes a helper script below a path derived by the test; use t.TempDir() |
| go-unsandboxed-writer | 1 | `L:🔌️mcp/🧪️tests/📋️capability-listing/🐹️.go` | ledger: adapter builds the repo binary into the shared cache `🗃️bin` of the real checkout (concurrent builds race); needs a per-run output directory in the build script |
| go-unsandboxed-writer | 1 | `L:🔌️mcp/🧪️tests/🤝️jsonrpc-handshake/🐹️.go` | ledger: adapter builds the repo binary into the shared cache `🗃️bin` of the real checkout (concurrent builds race); needs a per-run output directory in the build script |
| go-unsandboxed-writer | 1 | `L:🖥️server/🎛️coordinator/🧪️tests/🧯️g3-hostile-source/🐹️.go` | ledger: lists forbidden call names as strings; nothing is written |
| persistent-native-state | 1 | `F:🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️receiver/🧪️tests/🟦️.ts` | ledger: refusal proof: copied and expired controls must be refused before any physical intent; its scratch area is the sandbox since this change |
| persistent-native-state | 1 | `OS:🔨️modules/🧑‍💻dev/🚀️local-hub/🎫️session/🧪️tests/🟦️.ts` | ledger: dev local-hub session callback proof under S_LOCAL_ONLY; no partition birth is requested by the test |
| rust-checkout-write | 1 | `S:plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` | ledger: bless-style generator test rewrites a checked-in fixture next to the crate on every run (gate it behind an explicit bless switch) |
| ticket-scratch-root | 4 | `🏢️semio-tech/🎡️play/🧪️tests/🧪️playpages/🟦️.ts` | ledger: vitest lane: scratch lives in the generated folder of a foreign ticket; vite cannot load the sandbox module (JSON import attributes) until vitest gets its own scratch helper |
| ticket-scratch-root | 1 | `F:🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🧪️tests/🟦️.ts` | FIXED (staged: scratch root becomes the per-run sandbox) |
| ticket-scratch-root | 6 | `F:🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts` | FIXED (staged: scratch root becomes the per-run sandbox) |
| ticket-scratch-root | 1 | `F:🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️receiver/🧪️tests/🟦️.ts` | FIXED (staged: scratch root becomes the per-run sandbox) |
| ticket-scratch-root | 1 | `F:🏃️process/📦️artifacts/📤️publication/📥️sources/🧪️tests/🟦️.ts` | FIXED (staged: scratch root becomes the per-run sandbox) |
| ticket-scratch-root | 1 | `OS:🔨️modules/🧑‍💻dev/🧪️tests/🧩️contribution/🟦️.ts` | ledger: symlink-boundary proof must run below the workspace root; needs a sandbox workspace root |
| ticket-scratch-root | 1 | `L:lib/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/⏳️phase/🧪️tests/🟦️.ts` | FIXED (staged: scratch root becomes the per-run sandbox) |
| ticket-scratch-root | 1 | `L:lib/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🟦️.ts` | ledger: scratch must lie below the workspace (inputs are workspace-relative); needs a sandbox workspace root, then testScratchRootV1 |
| ticket-scratch-root | 1 | `L:lib/⚡️caching/🦀️cargo/📦️units/📥️inputs/🧪️tests/🟦️.ts` | ledger: scratch must lie below the workspace (inputs are workspace-relative); needs a sandbox workspace root, then testScratchRootV1 |
| ticket-scratch-root | 2 | `L:lib/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🧪️tests/🟦️.ts` | ledger: scratch must lie below the workspace (inputs are workspace-relative); needs a sandbox workspace root, then testScratchRootV1 |
| ticket-scratch-root | 1 | `L:lib/🏗️build/🎚️mode/🦀️profile/🧪️tests/📥️consumption/🟦️.ts` | ledger: scratch must lie below the workspace (inputs are workspace-relative); needs a sandbox workspace root, then testScratchRootV1 |
| ticket-scratch-root | 1 | `L:lib/🧪️tests/💠️inventory-artifact-shards/🟦️.ts` | FIXED (staged: scratch root becomes the per-run sandbox) |

## Go (hermetic harness already present)

Every Go test owner that resolves a repository root (`os.Getwd`, `workspace.FindRepoRoot`/`GetRootDir`/`ResolveRepoRoot`, `runtime.Caller`) already runs under `sandbox.Run` (TestMain bound to a temporary git repository, root guard fails the run when anything resolves or holds the checkout). The scan found 0 owners with such a lookup and no `sandbox.Run`; the 7 owners without it (`cli/internal/command`, `cli/internal/eventstore`, `events`, `search`, `yaml`, `identity`, `coordinator`) contain no root lookup (scan); the adapter cases of `events` and `coordinator` write below the harness `ctx.WorkDir`. Remaining Go rows are the 4 ledger entries above.

## Ports

No test binds a fixed port: 0 `listen(<literal>)` errors in TypeScript, Go or Rust (the one error row is an assertion on a `--port` argument text). The real exposure is the probe-then-rebind pattern (10 sites: `port-variable-bind`, bind to 0, read the number, close, bind it again later) and 5 configured literals in dev-hub lease tests (`fixed-port-config`, data only). `reserveTestPortV1` replaces both: one atomic lease directory per port shared by all sandboxes of the host, bind-checked, released on disposal, dead owners reclaimed.

## Warnings kept as census only (top files by count)

| warnings | file |
| ---: | --- |
| 39 | `L:⌨️cli/🧪️tests/🧷️hook/🐹️.go` |
| 35 | `L:💻️client/⌨️cli/🧪️tests/🍎️native-hook-event-mapping/🐹️.go` |
| 25 | `OS:🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🔭️config-discovery/🟦️.ts` |
| 23 | `L:lib/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts` |
| 22 | `L:lib/🧪️tests/📜️historical-document-evidence/🟦️.ts` |
| 15 | `OS:🔨️modules/🌉️mcp/🧪️tests/🔬️bin-quick/🦀️.rs` |
| 13 | `OS:🔨️modules/🧑‍💻dev/🧪️tests/🚀️local-hub/🟦️.ts` |
| 12 | `L:lib/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts` |
| 10 | `L:lib/🏗️build/🟢️readiness/🧪️tests/🟦️.ts` |
| 9 | `OS:🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔐️HubSignIn/🧪️tests/🧩️component/🟦️.tsx` |
| 8 | `L:🎛️dashboard/🧪️tests/🧊️execution/🟦️.ts` |
| 8 | `L:💻️client/⌨️cli/🧪️tests/📡️track-hook-events/🐹️.go` |
| 8 | `L:lib/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts` |
| 7 | `OS:🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔀️wgpu-document-relay/🦀️.rs` |
| 7 | `OS:🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs` |
