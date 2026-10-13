# 🚀️ Wave A1: release build driver, jobserver, memory admission, telemetry (2026-10-13)

Executor: Lane A (A1). Design: `📓️2026-10-13-scalable-build-architecture.md` §6.1–6.5, §8 and §10 A1.

Lane B (A2) landed its single-profile change while this wave ran. It rebased `componentTargets`, the plan's profile list, the `native component` admission and the nx contract fixture onto this wave's versions, and those versions are what is in the tree now. Nothing in this note was run against the full release; that run is M0.

## 1. Outcome

- **One owner per release.** `workspace:release` is a root target with `cache: false`. It runs `bun ./📜️script.ts release`, which calls `runReleaseDriverV1`.
  - It is declared `metadata.semio.orchestrator: true`. The Nx plugin therefore never wraps it in `native owner-command`, so the 1 h child deadline that killed build #10 (§2.1) cannot apply to it.
  - Proved on the constructed graph: `workspace:release` resolves to `nx:run-commands`, while every other root target is `@semio-tech/repo-lib:owner-command`.
- **No pass inside a consumer.** The prebuild plan, claim, ready markers, passes and jobs logic (`ensureCargoPrebuildV1`, `cargoPrebuild*`, `SEMIO_CARGO_PREBUILD_*`, schema `PrebuildPlan`) are deleted, together with their tests and fixtures.
  - Component nx targets only admit what the driver staged. In Lane B's current version this is the single `component` target, `dependsOn: ["workspace:release"]`.
- **No nested Nx.** `build-fresh` is gone as a script command, as an Nx target and as a package alias. The fresh build is exactly one invocation, launched from launch.json:
  `bun nx run @semio-tech/semio-tech-play:build --skip-nx-cache --skip-remote-cache --parallel=8 --output-style=stream`
  with the environment `SEMIO_RELEASE_GENERATION=fresh`, `SEMIO_BUILD_MODE=ship` and no daemon. The driver compiles cold into a new generation `⚡️cache/fXXXXXX/{t,b,r,l,j}`.
- **No wall-clock deadline on the pass.** A stall watchdog stops a run only when no unit has started or finished for 15 min while no rustc runs. It first writes a `sample` of every Cargo process into `l/stall-<pid>.txt`.

## 2. Architecture as built

| Facet (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/`) | Role | Schema / fixture / tests |
|---|---|---|
| `🎰️jobserver` | GNU make compatible pool of N = cores slots. The driver keeps the implicit slot and publishes N−1 tokens: a fifo on Unix, read without blocking; a named semaphore on Windows through `bun:ffi`. Cargo, rustc and every stage draw from it. Waiters are served FIFO, and a cancelled waiter leaves no token behind. | `JobserverV1`; MAKEFLAGS and auth vectors (Cargo precedence); pool laws; the `jobserver` crate oracle; the GNU make ≥ 4.4 oracle |
| `🎟️admission` | The broker (Unix socket or named pipe) and the `semio-build-scheduler` rustc wrapper (`🦀️.rs`, standalone workspace, std only, hand-declared FFI: `wait4`, `proc_pid_rusage`, `/proc/<pid>/statm`, `K32GetProcessMemoryInfo`, `GetProcessTimes`, `OpenSemaphoreW`). Prediction is `max(1.15 × p95(R-7, last 8 peaks), 256 MiB)`. Without history: 0.6 GiB + 40 MiB × kloc, or 6 GiB for a component final unit. The budget is physical − 6 GiB − the rolling 10 s maximum of non-rustc RSS. EASY backfilling uses recorded walls. Measured overshoot or swap-out above 64 MiB/s freezes admission, with hysteresis at 90 %. Priority: remaining path, then predicted peak, then arrival. The store is `⚡️cache/build-telemetry/units.jsonl`: compacted to 8 per key, rewritten atomically under a lease, never cleared by fresh builds. | `AdmissionV1` (policy, wire protocol `AdmitRequestV1` / `VerdictV1` / `RssReportV1` / `DoneReportV1`, `ObservationV1`); vectors for keys, edges, R-7, prediction, order, decisions, freeze and store; the d3-array oracle; real Cargo laws |
| `📈️telemetry` | Timeline rows: CPU from `os.cpus()` deltas; memory and swap from `vm_stat`/`sysctl` or `/proc`; rustc running, admission waiting, rustc RSS, frozen, slots, stages. The critical path is recomputed from the wrapper's real unit graph: `--extern` rmeta vs rlib edges, plus build-script runs via `OUT_DIR`. Also idle windows with cause, L9 relink fan-out, `report.md` and `units.json`. | `TelemetryV1`; critical-path, idle and fan-out vectors; the cargo `--timings` HTML oracle |
| `🧩️component-pass` | The driver. Each Cargo pass covers one workspace, profile and target; profiles run in parallel lanes, and the workspaces of one profile run in sequence so that no two Cargo processes contend for one unit. Each pass holds the per-profile lease on its build-dir. Each component is staged (header-checked, `stageArtifacts` with the narrow builder's owner string) the moment Cargo links it. Stale outputs of unlinked packages are removed. One provenance receipt is written per pass (skipped only for a no-op pass whose identical receipt exists). Output is bounded: JSONL and log capped at 128 MiB, a progress row every 30 s, ≤ 200 failure lines per unit and ≤ 2,000 in total. History is recorded with remaining paths. In a fresh generation, t/b are deleted after a green run. | `ReleasePlanV1` and driver policy; plan, generation, args and artifact vectors; stub-Cargo laws; a real wasm32-wasip2 run |

Plan source: `componentWorkspacesV1` and `componentPassPlanV1` in `⚡️caching/🦀️cargo/🟦️.ts`. They walk the same manifests the Nx component targets read and resolve `package.workspace` or the nearest `[workspace]`. Today that is 28 workspaces (hub 44, 26 `✏️s` extensions, root 1): 71 components and one pass per workspace for `wasm-release` (Lane B's region `RELEASE_COMPONENT_PROFILES`).

## 3. Verification (all run)

| Check | Result |
|---|---|
| `bun nx run @semio-tech/framework-process:test-native-artifacts` (`SEMIO_TEST_ARTIFACT_DIR=⚡️cache/a1/tna`) | **61 pass, 1 skip, 0 fail**, 62 tests in 6 files, 17.3 s. The suite budget moved from fundamental (15 s) to quick, because it now carries real Cargo laws. |
| Jobserver oracle: the `jobserver` 0.1.35 crate (Cargo's and rustc's client) joins the pool from MAKEFLAGS | `maxConcurrent 4` = tokens, `acquired 28` = (8 − 1) × 4, all tokens returned |
| Real Cargo through wrapper, broker and pool (4 crates, 2 tokens) | Tight budget (6.9 GiB physical → 0.9 GiB): exactly 1 unit admitted at a time, with real waits. Roomy: 2 concurrent = tokens. `top`'s edges are rmeta (pipelined). |
| Telemetry critical path vs cargo `--timings` HTML (`unblocked_units` / `unblocked_rmeta_units`) | Same chain `base → right → top`. Lengths: 352 ms vs 360 ms (other runs 341/350, 965/980). |
| Driver laws (stub Cargo) | Streaming before the pass ends; release lanes sequential and dev lane overlapping; jobserver, wrapper and `package` unification in every Cargo environment; unlinked component fails the release, its stale output is removed and the report is still written; cancellation kills the Cargo tree; stall watchdog fires. |
| Real driver run (wasm32-wasip2, two profiles, 2 components) | 6 units, 4 staged, report, timeline and history written, fresh t/b deleted |
| `bun nx run semio-build-scheduler:test` (new launch gate) | 4/4 Rust unit tests. `test = false` was removed so that the gate runs them. |
| Play fresh-build contract (`🧪️playfreshbuild`, vitest through nx) | **7/7**. Launch entries in seed and launch.json equal the contract (jsonc-parser), one Nx invocation, no `runOwnedNxInvocationV1`, cold generation env, orchestrator target, component producer `workspace:release`, parallel rows. |
| `repo:test-cargo-build-lease` (lease facet after deletion) | Green with an artifact dir of about 75 UTF-16 units. The pre-existing resource-lease boundary law needs a dir long enough that the owner length is ≤ 64, but short enough to stay under 256. |
| Dry wiring probe: `SEMIO_RELEASE_GENERATION=dry-wiring-probe bun nx run workspace:release` | nx → root script `release` → plan built (repo walk) → `Invalid SEMIO_RELEASE_GENERATION` before anything compiles |
| GNU make 4.4 `--jobserver-style=fifo` oracle | **Not run.** The law is written (`test.skipIf`), but only GNU Make 3.81 is installed and installing 4.4 needs a download approval. The `jobserver` crate oracle carries the third-party requirement. |
| Windows paths (named semaphore via `bun:ffi`; wrapper `OpenSemaphoreW` / `K32GetProcessMemoryInfo`) | **Not compiled or run.** No Windows host is available. |
| `workspace:verify-taxonomy-report` (admission of the new slugs `🎰️jobserver`, `🎟️admission`, `📈️telemetry`, `🧩️component-pass`, `🔮️oracles`, `🧪️tests/🦀️cargo`) | **Not completed.** The inventory of all 155,487 files finished in about 10 min. After that the gate printed nothing but heartbeats for 33 more minutes, and I stopped it at 44 min. The coordinator should run it before M0. |

## 4. Demonstration: a cold driver run over 5 hub components

The command was `build-driver/📜️script.ts subset wasm-release 560000 semio-hub-stdio-{media,binary,mesh,office,image}`: a fresh generation `⚡️cache/a1/fvdpKd2`, staged into `dist/a1-demo` (deleted afterwards). The full report is in `build-driver/📓️2026-10-13-a1-demonstration-report.md`.

These components link `semio-framework-os-kernel` but not stdio-semio. The cold-start predictions were active because the store was empty.

| Metric | Value |
|---|---|
| Wall | **530 s**. The pass took 517 s; provenance of the pass read 10.1 GB in about 9 s. |
| Units, components | 144 units, 5/5 staged, 0 failures |
| Machine CPU, compile phase | 95.5 % mean, no idle window (≥ 30 s below 75 %) |
| rustc CPU | 3,857 s over a 516 s compile window, which is **7.5 cores on average**. Max 10 concurrent rustc. |
| Peak rustc RSS (sum) | 14.5 GiB, against a 25.7 GiB budget. Largest unit: `semio-hub-stdio-image` final, 9.28 GiB (predicted 6.0). |
| Admission | 3 waits, all component final units at the cold 6 GiB prediction: 133.6 s in total, max 70.5 s. Machine CPU stayed at about 100 % while they waited. |
| Streaming | Components staged at 311, 340, 405, 443 and 516 s while the pass kept compiling until 516 s |
| Critical path | 516 s: … → os-kernel (28.5 s, full edge, it links a cdylib) → framework-plugin (31.2 s) → stdio-contract → stdio-binary → stdio-gif → stdio-image final (245.6 s, 82 s ready-to-start, of which 45.9 s was admission) |

CPU timeline (30 s buckets, from `r/timeline.jsonl`, 264 rows):

| t (s) | CPU % | rustc running (max) | admission waiting (max) | rustc RSS GiB (max) |
|---:|---:|---:|---:|---:|
| 0–30 | 97.0 | 10 | 0 | 2.4 |
| 30–60 | 90.8 | 4 | 0 | 4.4 |
| 60–90 | 97.6 | 9 | 0 | 7.6 |
| 90–150 | 100.0 | 10 | 0 | 9.2 |
| 150–210 | 99.7 | 10 | 1 | 11.4 |
| 210–330 | 99.9 | 4 | 1 | 12.0 |
| 330–390 | 100.0 | 3 | 0 | 14.5 |
| 390–420 | 95.5 | 2 | 0 | 10.6 |
| 420–450 | 70.3 | 2 | 0 | 9.1 |
| 450–480 | 100.0 | 1 | 0 | 8.9 |
| 480–540 | 79.7 | 1 | 0 | 6.7 |

Reading:

- The head is no longer serial: 10 rustc ran in the first 30 s.
- The tail is set by the component final links. Each is a thin-LTO, cgu-16 link that alone draws up to every token.
- The admission waits were caused by the cold 6 GiB component allowance. Measured RSS stayed at 8–12 GiB. From the second build the recorded peaks (3.7–9.3 GiB × 1.15) replace that allowance, and priority uses the recorded remaining path; M0 seeds both.

## 5. Decisions and deviations, with reasons

1. **Orchestrator exemption, not a policy change.** `📋️native-orchestration/🔣️policy.json` is unchanged, so every other native task keeps its 1 h child cap. Only a target declared `orchestrator` (the driver, which has its own cancellation and stall watchdog) skips the owner wrapper (`🟨️.mjs` `projectWithDefaults`).
2. **Lanes per profile, workspaces of one profile in sequence.** Concurrent Cargo processes that share units on one build-dir can deadlock in `prebuild_lock_exclusive` (memory `feedback-fine-grain-locking-wasm-deadlock`). Different profiles never share units. Today this matters little, because Lane B left a single profile.
   - The 26 `✏️s` extension workspaces still recompile their closure under their own profile set and run after the hub pass. WS1 merges them.
3. **Provenance once per pass**, at O(units) instead of O(components × closure). `cargoProvenancePhysicalControlV1` gained `scale` (the number of components a pass stands for), which multiplies bytes, work and deadline.
   - Measured: 10.1 GB for 5 components. The fixed 16 GiB budget would refuse a 44-component hub pass. With scale 44 the budget is 704 GiB and 44 × 600 s.
4. **Feature unification stays `package`** in the pass environment until WS1. Components resolve exactly as narrow builds did.
5. **Fresh nx parallelism** is `playFreshParallel` for the reference build machine (10 cores, 32 GiB → 8), kept as a fixture row. A launch entry cannot compute it; N1 makes it irrelevant once stages hold jobserver tokens.
6. **Dropped:** `SEMIO_PLUGIN_SYMBOLS` (`cargo rustc -- -C strip=none`) has no multi-package form. Symbol builds belong to A2's `wasm-iterate` profile (`strip = "none"`).
7. **Deferred, per the plan:**
   - lanes (b) host tools and (c) wasm32-unknown-unknown modules: their nx targets live outside this wave's files (N1);
   - t = 0 work (catalog, Vite, typecheck) and pages assembly (N1);
   - streaming describe, wasm-opt and jco: they stay the Nx `describe` / `materialize` tasks after the driver (A2/N1);
   - the stage cache (N1); staging relies on `stageArtifacts`' identical-tree check.

## 6. Risks for M0

- **Cold-start admission.** The full hub pass has 44 component finals at 6 GiB each, so at most about 4 run concurrently until history exists. Expect waits like those in §4; they resolve after M0.
- **Provenance time for the full hub pass.** About 25–40 GB read is estimated. At about 1 GB/s that is 30–60 s, it runs as a token-holding stage, and its budget is scaled.
- **The stall watchdog** treats a single rustc that runs longer than 15 min as alive, by design. Only "nothing running and nothing finishing" counts as a stall.
- **Dev flows** (`materialize` → `component` → `workspace:release`) now run the whole warm driver on every launch, per D8. This is a fresh check of 28 passes plus identical-tree staging; a no-op pass with an existing receipt writes no receipt.

## 7. Files

Created:

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/{🟦️.ts, 🧬️schema/🔣️.json, 🧫️fixtures/🔣️.json, 🧪️tests/🟦️.ts, 🔮️oracles/🦀️.rs}`
- `…/🏗️native-build/🎟️admission/{🟦️.ts, 🦀️.rs, 🧬️schema/🔣️.json, 🧫️fixtures/🔣️.json, 🧪️tests/🟦️.ts, 📦️packages/🦀️rust/Cargo.toml, 📦️packages/🦀️rust/Cargo.lock}`
- `…/🏗️native-build/📈️telemetry/{🟦️.ts, 🧬️schema/🔣️.json, 🧫️fixtures/🔣️.json, 🧪️tests/🟦️.ts}`
- `…/🏗️native-build/🧩️component-pass/{🟦️.ts, 🧬️schema/🔣️.json, 🧫️fixtures/🔣️.json, 🧪️tests/🟦️.ts}`
- `…/🏗️native-build/🧪️tests/🦀️cargo/🟦️.ts` (the shared Cargo test kit; it contains no tests)
- Ticket: `build-driver/📜️script.ts`, `build-driver/📓️2026-10-13-a1-demonstration-report.md`

Changed:

- `…/🏗️native-build/🔒️lease/{🟦️.ts, 🧬️schema/🔣️.json, 🧫️fixtures/🔣️.json, 🧪️tests/🟦️.ts}`: prebuild deleted
- `…/🏗️native-build/🟦️.ts`: no prebuild option or re-exports
- `…/🏗️native-build/🧾️receipt/🟦️.ts`: provenance `scale`
- `🧰️framework/🔨️modules/🏃️process/📜️script.ts`: facet suites in `native-artifacts`, quick budget
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`: orchestrator exemption; `componentTargets` → `workspace:release`, region `🧩️ComponentProfiles` (since rebased by Lane B)
- `…/📚️library/⚡️caching/🦀️cargo/🟦️.ts`: `componentWorkspacesV1`, `componentPassPlanV1`, region `🧩️ReleaseComponentPlan`
- `…/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`
- `…/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`: `native component` admits the driver output (since narrowed by Lane B)
- `…/📚️library/⚡️caching/🧫️fixtures/nx-contract/🔣️.json` and `…/🧪️tests/⚡️cache-contracts/🟦️.ts` (since superseded by Lane B)
- `📜️script.ts` (`ReleaseScript`, `release`) and `📋️project.json` (`release` target)
- `🏢️semio-tech/🎡️play/{🔨️modules/📦️site/📜️script.ts, 🔨️modules/📦️site/🆕️fresh-build/🟦️.ts, 📋️project.json, package.json, 🧪️tests/🧪️playfreshbuild/🟦️.ts, 🧪️tests/🧪️playfreshbuild/🧫️fixtures/🔣️.json}`
- `🌎️hub/🧪️tests/🌳️unit-census/🟦️.ts` (Lane C's file: import and two references only)
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`:
  - `📦️build🏢️semio-tech🎡️play🆕️fresh` (changed)
  - `📦️build🦑️workspace🚀️release` (new)
  - `📦️build🦑️workspace🚀️release🆕️fresh` (new; the M0 command)
  - `⚖️gate🧰️framework🏃️process🏗️native-artifacts` (new)
  - `⚖️gate🧰️framework🏃️process🎟️build-scheduler` (new)

All private directories under `⚡️cache/a1` were deleted after the runs.
