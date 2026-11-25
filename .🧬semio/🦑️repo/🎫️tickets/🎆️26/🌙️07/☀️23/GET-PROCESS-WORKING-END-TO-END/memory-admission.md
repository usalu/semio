# Memory admission for Cargo: root cause, fix, carving

Companion: `crate-memory-ranking.md` (full per-crate measurements and the process3d split).

## 1. Root cause

The repo build system already has everything the task asks for, but only for work that enters through `repo cargo`, `repo run` and friends:

| Requirement | Where it already lives |
|---|---|
| machine-wide ledger shared by independent `repo` invocations | `🔨️modules/🏃️process/🏗️build/🌡️host-capacity` (`grants/`, `queue/`, `estimates.json` in `~/Library/Caches/semio/host-capacity`, `$XDG_CACHE_HOME` or `~/.cache`, `%LOCALAPPDATA%`) |
| cross-platform memory probes | `📡️sampling` (macOS `vm_stat` + `kern.memorystatus_vm_pressure_level`, Linux `/proc/meminfo` + cgroup + PSI, Windows native memory port) |
| floor, fair queue, wait with progress and cancellation | `withHostCapacityV1`, `🎫️gate` |
| per-crate learned estimate, persisted | `📐️estimates` (`hostEstimateLineageV1`), `repo cargo` learns under `cargo` > `cargo:<subcommand>` > `cargo:<subcommand>:<package>` from the measured peak tree footprint |
| jobs adaptation | the gate grants "N of M jobs"; `repo cargo` rewrites `--parallelism` |
| per-rustc admission and per-unit peaks | `RUSTC_WRAPPER` scheduler in `📦️artifacts/🏗️native-build/🎟️admission` (broker, `unit history`) |
| pressure reaction on running work | `🛡️warden` (SIGSTOP at guard level, resume when calm, SIGKILL only at danger) |

The incident happened because nothing forced Cargo to enter that system:

1. `.cargo/config.toml` makes the default `rustc` a script that refuses (`unissued default Cargo compiler ingress`), but the refusal is an overridable Cargo setting. The ticket wrappers (`brep-cargo.sh` etc.) set `CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER=` plus a private `CARGO_TARGET_DIR`, which bypasses the ingress, the wrapper and the ledger. At the incident time `repo capacity status` showed `0 running, 0 waiting` while 9 rustc processes ran.
2. The warden only acts on processes below a grant root. Raw rustc children of an unadmitted cargo are invisible to it.
3. The one sanctioned non-compiling entry, `repo tools exec cargo`, was admitted as a fixed `cargo` operation: 1 job, a static 3584 MB, no learning, and it never handed the granted jobs to Cargo (`CARGO_BUILD_JOBS` unset, an explicit `-j 12` was never capped).
4. Peak per crate was never the problem for the named crates (see section 4): the cost is the number of concurrent heavy units, multiplied by five uncoordinated invocations.

## 2. Design

No second ledger. One ledger, every Cargo entry goes into it:

- **Managed entry** (`🚀️launcher/🚪️shims/cargo`, `cargo.cmd`): compiling subcommands (`build check test run bench clippy doc rustc rustdoc fix miri install`) become `repo cargo --store=isolated --configuration=prototype -- <args>` (full per-unit admission, unit cache, private isolated store, bypass environment ignored). Everything else becomes `repo tools exec cargo -- <args>`. `SEMIO_CARGO_STORE` and `SEMIO_BUILD_MODE` override the defaults. Put the directory first on `PATH`.
- **Registry admission learns per crate** (`createToolAdmissionV1`): lineage from the arguments (`cargoGrantLineageV1`), request scaled by the learned ratio of that class (`estimateHostRatioV1` + `scaleHostRequestV1` + `rampHostRequestV1`, same arithmetic as `repo cargo`), run observed by the gate heartbeat, outcome recorded with `recordHostOutcomeV1` only when the tool exited 0 (failed, signalled, cancelled and pressured runs teach nothing).
- **Jobs adaptation**: preferred jobs = the command's own `-j/--jobs`, else `CARGO_BUILD_JOBS`, else cores; the gate grants what the memory above the floor carries (at least one); the grant is handed over as `CARGO_BUILD_JOBS` and any explicit job flag in the command line is capped at the grant (`capCargoGrantJobsV1`; arguments after `--`, the test binary's, are untouched).
- **Toolchain**: the grant environment sets `RUSTUP_TOOLCHAIN` from the nearest `rust-toolchain.toml` (`<channel>-<host triple>`, triples for macOS, Linux and Windows) unless the caller chose one, so a toolchain binary called directly resolves `rustc` like the rustup proxy (the coordinator's finding about the stopgap). The registry pin of `cargo` stays `rust-toolchain.toml` `channel`.
- **Waiting and cancellation**: the gate's own fair queue (`[capacity] waiting Ns for cargo: ...`, German too), aborted through the `AbortSignal` of the tool run; tested.
- **Pressure**: unchanged and now reaches every Cargo run, since every one is below a grant root: the warden suspends rustc units first (no new memory is taken while stopped), resumes when calm.

## 3. Implementation

Schema first: `🦀️cargo-grant/🧬️schema/🔣️.json` (policy constants and the case definitions the fixtures are validated against), then `🦀️cargo-grant/🟦️.ts` (pure functions: lineage, jobs, cap, host triple, pinned toolchain, grant environment), then the wiring in `🧰️tools/🏃️run/🟦️.ts`.

New optional ports: `ToolCapacityAskV1.arguments/cwd`, `createToolAdmissionV1(... { gate })` to inject sample/observe ports.

### Tests (`🧪️tests/🦀️cargo-grant`, 11 tests, all pass)

Run: `SEMIO_BUILD_GRAPH_POLICY=current repo test-file <path>`.

- fixtures validate against the schema (Ajv);
- fixtures (lineage 8, jobs 9, caps 6, toolchains 6, shims 6) equal the TS output;
- the same cases plus a generated grid of 2304 argument/environment/core/grant combinations equal an independent Python model (`🐍️oracle/🐍️.py`, TOML parsed by Python's own `tomllib`, a third-party parser);
- a cap never exceeds the grant and never touches the test binary's arguments;
- the repo's real `rust-toolchain.toml` resolves to `nightly-2026-07-20-<triple>` from any directory below it, a caller's `RUSTUP_TOOLCHAIN` wins, triples of macOS/Linux/Windows, a temp repo without a pin yields none;
- admission arithmetic against a simulated host: roomy host grants more jobs than a tight one, at least one, `CARGO_BUILD_JOBS` equals the grant and `RUSTUP_TOOLCHAIN` is present;
- learning: success teaches `cargo`, `cargo:check`, `cargo:check:<package>`; exit 101 and SIGKILL teach nothing; a heavy package's ratio exceeds a light one's;
- a learned heavy crate waits behind a learned light one on the same host (light finishes first);
- waiting reports progress and cancellation ends the wait without starting the work;
- two independent invocations share one ledger (the second sees the first's reservation, the first was granted fewer than the requested jobs);
- the shim: six command lines map to the exact `repo` argv (including a spaced argument and a `+toolchain` selector).

Also run: `🧰️tools/🧪️tests/🏃️run` 15/15 pass. `🦀️cargo-scaling` 8 pass, 2 fail: the two fleet cases fail with `--offload takes never, distribute or pin:<origin>, once` (an `always` mode rename by another agent); unrelated to this change (the door file `host-capacity/🟦️.ts` is untouched by it).

### Live verification

- `repo tools exec cargo -- config get -Zunstable-options --format json build.jobs` printed `{"build":{"jobs":2}}`, then `1` under heavier load: `[capacity] admitted cargo with 2 of 10 job(s): the host cannot carry more right now`.
- `repo cargo --store=isolated --configuration=prototype -- check -p semio-framework-value --lib`: exit 0 in 55 s through the full ingress. This is the retirement path.
- A compile started via `repo tools exec cargo` is refused by the ingress script by design (use `repo cargo`).
- One learned sample from a failing probe polluted `cargo` and `cargo:metadata` in the shared `estimates.json` before the failure guard existed; I removed exactly those two single-sample classes.

## 4. Crate ranking (summary; details in `crate-memory-ranking.md`)

Measured with `T/rustc-mem-wrap.sh` (`/usr/bin/time -l` per rustc) via `T/mem-cargo.sh`, through the gate, 1085 units of `cargo test --no-run -p semio-s-artifact-process-process3d-editor`.

| # | unit | peak RSS MB | wall s | diagnosis |
|---|---|---|---|---|
| 1 | `process3d_workpiece_window` (test) | 3358 | 222 | 418-line harness mounted by `#[path]` into 10 units, each with the history law |
| 2 | `semio_s_artifact_process_process3d_editor` (lib test) | 3336 | 288 | same harness in-crate |
| 3 | `semio_s_artifact_stdio_binary_io_registry` | 2000 | 42 | 4450 LOC in 65 files, `editor`/`io`/`diff` in one unit |
| 4 | `semio_s_artifact_stdio_txt_tests` | 1730 | 32 | re-instantiates the typed-store generics |
| 5 | `semio_plugin_actor_input` | 1725 | 26 | 788 LOC, macro/generic expansion |
| 6-15 | `*_member` stdio crates, `gltf_model_snapshot`, `process3d_host`, `framework_kernel` | 1.2-1.5 GB | 11-58 | 14-line `space_members!` crates (monomorphization), derive-heavy records, 9.3 kLOC kernel |

Not a problem: `semio_framework_job` 274 MB; value-native crates 160-364 MB; `semio_framework_dsl_record_model` 313 MB. `semio-plugin-tool-run` and `semio-s-artifact-energy-model-mutations` are outside the measured closure and were blocked by other agents' compile breakage (`brep-snapshot` higher-ranked lifetime error), so they are unranked.

### Splits done

New crate `semio-s-artifact-process-process3d-harness` (`✏️editor/🧪️harness`) holds the shared test app once; eight editor test targets `use` it; the history law runs once. A/B back to back for `process3d_workpiece_window`: 6609 MB / 144 s before, 4203 MB / 120 s after. The harness crate costs 3.4 GB once.

Coordinator note on the unresolved import in `pane-smoke-target` and `🪜️composition`: this split is mine (via my helper). The wiring is complete in the tree (workspace dependency, editor `[dev-dependencies]`, `Cargo.lock`, all 9 mounts), and `cargo check --tests -p semio-s-artifact-process-process3d-editor` no longer reports an unresolved import. It stops earlier on a different agent's error in `semio-s-artifact-stdio-semio-brep-snapshot` (higher-ranked lifetime error in the `NativeReceive` derive), so the seven other editor targets are still not compile-verified. The two `f64` dereference errors are not caused by the split: `WorkingSolid::Box { width }` is `f64` in `📐️document-records` now, so those errors came from a state of the model crate that no longer exists.

### Proposed, not done

`binary_io_registry` split into `editor`/`io`/`diff` subset crates; `txt_tests` to depend on the subset member crate; erased (non-generic) core under `space_members!` for the `*_member` family; `process3d_host`, `framework_kernel` (43 files) along the taxonomy; `-Z self-profile` for `plugin_actor_input`. Disk is a second shared resource: the volume hit 287 MB free from ticket build dirs of 3.8-25 GB; the door path has `admitBuildDiskV1`, the registry path does not.

## 5. Retiring the stopgap `T/bin/cargo` (done, see 7.3)

1. Put `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims` first on `PATH` (agents and the ticket wrappers; `repo tools setup` is the natural owner of that line, not wired yet).
2. Replace `CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=...` in the ticket wrappers with nothing: `repo cargo --store=isolated` owns the target and ignores the bypass variables.
3. Delete `T/bin/cargo` and `/Users/ueli/Documents/semio/.cache/cargo-gate` once a ticket wrapper (for example `brep-cargo.sh`) has run green through the shim.

Not done and not verified: the shim end to end for a ticket wrapper (only the argv mapping is tested), the Windows `cargo.cmd` (written, untested), and the warden still cancels with SIGKILL at its danger level; the requested "pause only, never kill" would need `cancel` removed from the `local-unit` kind actions of the warden schema, a policy decision left open.

## 6. Changed files

All under `/Users/ueli/Documents/semio/`:

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🦀️cargo-grant/🟦️.ts` (new)
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🦀️cargo-grant/🧬️schema/🔣️.json` (new)
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🦀️cargo-grant/🟦️.ts` (new)
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🦀️cargo-grant/🧫️fixtures/🔣️.json` (new)
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🦀️cargo-grant/🐍️oracle/🐍️.py` (new)
- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts` (per-crate sizing, learning, jobs, toolchain, cap, `gate` ports)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims/cargo` and `cargo.cmd` (new)
- Carving (details in `crate-memory-ranking.md`): the process3d `✏️editor/🧪️harness` crate, the editor `Cargo.toml`, the process3d workspace `Cargo.toml`/`Cargo.lock`, nine test mounts, removed `🧪️tests/🧰️support/🦀️.rs`
- Ticket: `rustc-mem-wrap.sh`, `mem-cargo.sh`, `mem-chain.sh`, `crate-memory-ranking.md`, this file. Private build dir and tool output deleted.

## 7. Round 2 (coordinator decisions)

### 7.1 Warden: pause first, kill only as a last resort

Files: `🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🐍️.py` (oracle), `🧪️tests/🧫️fixtures/🔣️.json`, `🧪️tests/🟦️.ts`.

- Already the law and kept: at the guard level the warden holds new admissions (`hold`) and SIGSTOPs the cheapest-to-redo running units, the newest and biggest first (`order`: elapsed seconds per megabyte, ascending); `keepRunning` leaves the oldest one running; recovery resumes one paused unit per tick, the one that ran longest (`order` last).
- New: `Policy.pauseDeadlineSeconds` (10). At severe danger a unit is cancelled only when it cannot be paused (`canSuspend` false) or it has been paused for at least the deadline and pressure persists; cancels go newest first and stop as soon as the freed megabytes cover the deficit. A running unit is first suspended in the same tick, so one tick of pressure never kills. Per-unit runaway ceilings and caps (a single unit above its own declared ceiling) are unchanged: they are not pressure relief.
- Clean cancel: the `local-unit` and `remote-unit` kinds now end with SIGTERM instead of SIGKILL (the loop sends SIGCONT first, so a paused unit can exit), so Cargo reports an ordinary failed unit and the work can be retried.
- Fixtures: 5 new decision cases (suspend before any cancel, units paused under the deadline survive, cancelled newest-first after the deadline, an unpausable unit is cancelled at once, a calm host resumes the oldest paused unit); 11 stored expectations changed from immediate cancel to suspend-first and were regenerated by the Python oracle after review. The mounted tests that run real children were updated to the same story (paused at the first critical tick, cancelled at the next).
- Result: 46 of 46 warden tests pass (oracle, fixtures, mounted real processes).

### 7.2 Windows `cargo.cmd`

`🧪️tests/🦀️cargo-grant/🟦️.ts`: the batch file is parsed, its subcommand list must equal the POSIX `case` list, its two command lines must be exactly the expected ones, and a small interpreter applies cmd's rules (percent expansion of `%*`, `%~1` stripping quotes, case-sensitive string comparison without `/i`) to the six fixture command lines and to `Build` and `"build"`. On a Windows host an additional test runs `cmd.exe /d /c cargo.cmd` against a fake `repo.cmd` (skipped elsewhere, not run here).

### 7.3 Stopgap retired: every ticket wrapper uses the repo shim

- `T/bin/cargo` and `.cache/cargo-gate` are deleted. No process held a slot; a text search of `.🧬semio` finds the old path only in another ticket's markdown notes and in this ticket's historical reports.
- 41 wrappers were migrated mechanically (`migrate.py`, kept out of the ticket): the `T/bin` PATH entry became the repo shim directory (still first on `PATH`), and `CARGO_BUILD_RUSTC`, `CARGO_BUILD_RUSTC_WRAPPER`, `CARGO_TARGET_DIR`, `CARGO_BUILD_BUILD_DIR`, `CARGO_PROFILE_*`, `CARGO_INCREMENTAL` and `CARGO_BUILD_JOBS` were removed from their exports. The last four are refused by `repo cargo` (`Cargo profile environment belongs to issued consumption`) and belong to the grant; the jobs are now the grant's. `replay-cargo.sh` was rewritten by hand first.
- Not mechanical, by design:
  - `hub-build.sh`: the wasm profile and the artifact path that `publish-hub.ts` reads (`$N/target/wasm32-wasip2/...`) are outside `repo cargo`. It keeps its own target, now runs as `repo capacity run --operation=cargo --jobs=3 -- hub-cargo.sh build ...`, and `hub-cargo.sh` turns the granted jobs (`SEMIO_BUILD_PARALLEL`) into `CARGO_BUILD_JOBS`. Verified: `hub-cargo.sh config get ... build.jobs` printed 1 under a grant of 1 of 3 jobs.
  - `mem-cargo.sh` (measurement; its `RUSTC_WRAPPER` needs the bare compiler) now runs `repo capacity run --operation=cargo --jobs=2 -- $HOME/.cargo/bin/cargo`.
- Green run: `replay-cargo.sh` (process3d workspace) `check -p semio-s-artifact-process-process3d-model --lib` exited 0 through the shim into `repo cargo --store=isolated` (`[repo cargo] isolated store, 4 of 10 cores admitted, rustc memory budget 6.6 GiB`). A second wrapper, `stale-cargo.sh ... metadata`, went through the shim into the registry entry and exited 0.
- The full editor check (`check --tests -p semio-s-artifact-process-process3d-editor`, the coordinator's example) was started twice. The first run was cancelled by the warden because the host was critically short of memory (then the cargo driver had no pause step; this is why the unpausable-driver law of 7.1 was added: pausable units are paused first, the unpausable driver is cancelled only after they have been paused for the deadline). The second reached 543 of about 1800 crates in 40 minutes while the host swapped (8.7 GB swap, 78 suspended units, `cancelled 0`) and was stopped by me to give the memory back. It has not run green end to end; a later run of the harness check stops at `semio-plugin-ownership exceeds its source boundary before the compiler` (262796 of 262144 bytes, another agent's growing edit), so the editor and harness checks are blocked until that file is trimmed.

### 7.4 Observed behaviour worth keeping

The ledger and the new warden law were exercised live by the run above: `[capacity] admitted cargo with 4 of 10 job(s)`, sized from learned classes (`sized at 37% of the default from 1 earlier run(s) of its class`), `Memory guard: tight ... suspended 78, resumed 72, cancelled 0`. No unit was killed during the second run.

### 7.5 Changed files of round 2

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🐍️.py`, `🧪️tests/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json` (pause-first law, SIGTERM for units, 7 new decision cases, 11 regenerated expectations).
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🦀️cargo-grant/🟦️.ts` and `🧫️fixtures/🔣️.json` (Windows entry mapping tests, shim fixtures).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims/cargo`, `cargo.cmd` (unchanged since round 1).
- process3d editor harness: `✏️editor/🧪️harness/🦀️.rs` (facades).
- Ticket wrappers: 41 migrated `*.sh`, `replay-cargo.sh`, `hub-build.sh`, new `hub-cargo.sh`, `mem-cargo.sh`. Deleted: `T/bin/cargo`, `/Users/ueli/Documents/semio/.cache/cargo-gate`. Docs: this file, `crate-memory-ranking.md`.
