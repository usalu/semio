# result ts-verify (wave 1022a, executor ts-verify) - wound down at coordinator request (quota 90 %)
Tools: `C/ts-verify-tools/one.sh <nx|bazel> <project>` runs `SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo [--tool=bazel] run <p>:typecheck`; `chain.sh`, `direct*.sh` (direct tsc via ts-health `run1.sh`). Logs/rc: `C/🗑️generated/ts-verify/`, direct logs `C/🗑️generated/ts-health-run/logs/*-a<N>.txt`.

## Table (target -> nx -> bazel; "direct" = last direct tsc of the same tsconfig, NOT the repo CLI)
| project | nx | bazel | direct tsc (final) |
| --- | --- | --- | --- |
| @semio-tech/framework | BLOCKED (nx spin, below) | GREEN rc 0 (75 s, after fixing 12 test-type errors) | 0 errors |
| @semio-tech/repo-lib | not run | NOT RUN (fixed 70 errors in 28 files after ts-health) | 0 errors (a5) |
| @semio-tech/framework-os | not run | NOT RUN (dependsOn wasm declared, untested) | 1 error = stale flow bindings (env.) |
| @semio-tech/s-dev-composition | not run | NOT RUN | only TS2307 for dist-only packages (env.; built by its dependsOn) |
| @semio-tech/pets, pets-react, ui-react, framework-renderer-react, course-react | not run | NOT RUN (chain killed at wind-down) | 0 errors each (a1) |
| root `workspace` | no typecheck target | - | not run |

## Findings
- Nx (20:14-21:03, framework): after `Original daemon child exited before acquisition` / `Original native caller capabilities required` the parent `bun --eval` spins at ~150 % CPU for 45+ min without a task (same as demo-verify/course-rest2). Killed only my pids. Not fixed (demo-verify owns).
- Bazel works: framework:typecheck green; each queued 10-25 min on the capacity gate.

## Changes (live edits)
- Declared dependency: `@semio-tech/framework-os:typecheck` dependsOn `semio-framework-os-flow-browser-package:wasm` (`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json`). UNTESTED (wasm build not run).
- Jack LSP worker (`✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/{🟦️.ts,🔖️protocol/🟦️.ts,🧪️tests/📡️worker/🟦️.ts}`): its Rust crate was deleted (folded into dsl_lsp), the worker imported a nonexistent/stale wasm pkg. Now `serveJackLspV1(scope, engine)` with injected `JackLspEngineV1`; test uses a fake engine (no vi.mock). Removed the `JackLspSession` stub from `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🪝️playground-stubs/🟦️.ts`. OPEN: no wasm adapter exists; stale manifest entries remain (`rust/pkg` workspaces in 🏢️semio-tech/♻️mit-bestand/🌎️hub package.json, engines catalog.json, build-owners, ownership fixtures) - need a user decision (delete the dead package or build the dsl_lsp wasm bridge); I did not touch manifests/locks.
- stdio `bundleRuntime: "bun"` -> `"runtime"` in 11 artifact `📜️script.ts`; same stale `"bun"` in 4 repo-lib tests (compileNxAuthorizerSnapshotV1, bundleJavaScript).
- Real bug: `repo remote-cache ...` was routed to the runtime kernel which has no handler; `🏗️build/⌨️command/🧭️selection` now routes it to the cache kernel (guard in `💾️cache/🟦️.ts`, corpus rows + SQL oracle in `⌨️command/🧪️tests`).
- `loadBuildInvocationGraphV1` wrongly destructured from the wrong module in `⌨️command/⚙️runtime/🟦️.ts` (static import already exists); `buildEnvironmentPlanV1(tool: string)`; `ensureDaemon(process.execPath, …)` in caller-environment test; tools-resolve test host (`runtimeCommand`); reset-protocol `attached` text type; resident `.d.mts` fixes (`ResidentPolicyV1` as type alias + `>>>` syntax error, optional request fields, `DerivedStateConfigV1`, new `project-farm/🏊️pool/🟨️.d.mts`), JSON-fixture row typing in ~12 tests (`as unknown as` row types / `present()` filters, no any/ts-ignore).

## Open
- Run `repo --tool=bazel run <p>:typecheck` for the other 9 targets (repo-lib first) and the Nx runs once the Nx spin is fixed; framework-os needs the flow wasm build; s-dev-composition needs its dist/wasm deps built.
- No processes of mine remain (chain 52935 tree killed); no [DEBUG] lines added.
