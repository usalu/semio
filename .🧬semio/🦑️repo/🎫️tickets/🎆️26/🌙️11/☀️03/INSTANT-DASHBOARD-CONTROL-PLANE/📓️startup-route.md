# Instant Dashboard Control Plane

`repo dashboard` is rewritten to `repo run @semio-tech/repo-dashboard-rs:run --output=stream`. Two stacked costs made that start take minutes and then fail.

1. The build door sent every `@semio-tech/repo-dashboard-rs:` target, including `run`, `daemon` and `preferences`, to the full implementation. That admits the compiler source closure (`source.served`) before any dashboard code runs.
2. A missing prototype install logged `Rebuilding prototype because it is not installed` and spawned `repo run @semio-tech/repo-dashboard-rs:build`. That child is not inside the dashboard task, so it nests another full run. The nested run lists parser sources, then walks `dependencies:store-ownership` and floods `dependencies:current-input`.

## Route

Native control-plane targets take a new door route, `dashboard`, before the full-implementation prefix. The target after `@semio-tech/repo-dashboard-rs:` must be exactly `run`, `daemon` or `preferences`. `--graph`, `--batch`, `--parallel` and `--exclude-dependencies` stay on the full implementation, as do `:build` and `:test`. `--lang` is a required global, so it is consumed into the plan language and `SEMIO_LANGUAGE`; it is not forwarded as an operation argument.

The route handler builds a native caller envelope and calls `launchDashboard`. It does not load the build script, admit sources, or open the dependency store. A stale or missing installation compiles the dashboard crate with Cargo in this process from the compile target's `rustBuildIntent`, then installs that binary. Explicit `repo run @semio-tech/repo-dashboard-rs:build` still uses the graph.

## Checks

- Door planner: 9 pass, 438 expects. The language-agnostic fixture covers run, daemon, preferences and the build target. bun:sqlite is the independent oracle for the dashboard route. yargs-parser remains the independent argument reader.
- Dashboard launch law: 1 pass, 834 expects. A rebuild is an in-process compile plus install.
- The control module imports. The Cargo imports used by a cold rebuild resolve on disk.

`repo dashboard` itself was not started from this session. A warm start is the door plan plus the installed `semio` process. A cold start is one Cargo compile of the dashboard binary, not a workspace graph sync. `dev` and `build` issued from inside the running dashboard still use the graph.

Grok 4.7 Medium and Grok 4.7 Light are not in the model list for this session. The available Grok 4.7 slug is grok-4.7-high. No substitute fleet was launched.

## Files

- `framework/products/repo/modules/library/build/door/package.ts`
- `framework/products/repo/modules/library/build/door/control/package.ts`
- `framework/products/repo/modules/library/build/door/tests/package.ts`
- `framework/products/repo/modules/library/build/door/tests/fixtures/json`
- `framework/products/repo/modules/library/caching/bootstrap/dashboard/installation/package.ts`
- `framework/products/repo/modules/library/build/host-capacity/package.ts`
- `framework/products/repo/modules/dashboard/tests/execution/launch/package.ts`
## Launcher argv

`repo dashboard` is planned by the launcher as `run @semio-tech/repo-dashboard-rs:run --output=stream --`, then any further arguments. The `--` keeps that command off the resident build service (`ResidentEligible` returns false at `--`). The door plans that argv as `dashboard` and `dashboardInvocation` returns an empty launch list, which is the installed binary with no extra arguments.

The door test now runs that argv through `runBuildDoorV1`. The result is `handled`. The light-command loader is not called, and the route does not return `full`, which is the only result that loads the build implementation and its source admission. Door tests: 9 pass, 447 expects.

The dashboard command registry files `dev` and `build` as verbs, so those commands are part of the binary that this route starts. A cold install still compiles that one crate in this process. This session did not start `repo dashboard`.

## Issued Cargo proof

`repo run` of a dashboard test target reentered source admission and `dependencies:current-input`, then exited 1 before Cargo. That target was removed.

The known-command proof uses the same native process envelope as the dashboard door and calls `runRepositoryCargoTests` for `semio-framework-repo-dashboard-inventory`. That path does not enter `dependencies:current-input`.

On Windows the issued compiler publication then failed three times, each fixed and covered by a bun test:

- Directory fsync returns EPERM. `synchronizeRustupDirectoryV1` accepts that refusal.
- `tar -tf` rows end in a carriage return. `rustupArchiveNamesV1` strips it before path admission.
- The compiler Path and PATH keys hid System32. `rustupArchiveListEnvironmentV1` keeps one PATH that still contains the system tar.

The inventory test is running through that publication. The toolchain `nightly-2026-07-20-x86_64-pc-windows-msvc` is being extracted. The `workspace:dev` and `workspace:build` assertion has not finished.
## Compiler store envelope

`runRepositoryCargoTests` opened the compiler store with the caller environment. `withRepositoryCompilerStoreV1` reads `SEMIO_SCRIPT_PROCESS_INVOCATION` from that environment before any compile. The known-command proof kept the envelope only in async context, so workspace preparation finished and the store open then refused with `Original parent Script process invocation required`. The compiler lane now receives `scriptInvocationEnvironment(invocation, environment)`. A nested receive of that wire keeps the same invocation.

## Cargo test without nextest

The issued lane reached Cargo. `cargoNextestAvailableV1` saw `cargo-nextest.exe` on the store PATH, then the compiler command ran with a PATH that does not expose that executable, so Cargo reported `no such command: nextest`. The known-command proof now runs `cargo test --lib the_job_publishes_the_known_commands_first` through `runRepositoryCargoCommandV1`.

## Known commands passed

`cargo test --lib the_job_publishes_the_known_commands_first` finished `ok`. 1 passed, 55 filtered out, 0.05s. The first published phase is `known` and contains `workspace:dev` and `workspace:build`. `hub:dev` is absent until the walk. The run did not enter `dependencies:current-input`.

The build door tests, re-run after that, passed 9 tests and 447 expects. The launcher argv stays on the dashboard route. The dashboard installation file compiles the control plane in process and does not call `runOwnedCommand`.
