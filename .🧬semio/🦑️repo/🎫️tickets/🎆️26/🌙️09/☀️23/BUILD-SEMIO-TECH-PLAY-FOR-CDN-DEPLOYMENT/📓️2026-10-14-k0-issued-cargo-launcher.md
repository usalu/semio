# K0 — Issued Repository Cargo Launcher (2026-10-14/15)

## Exact Usage for Agents

```
bun repo cargo --store=shared --configuration=prototype -- <cargo subcommand> <args…>
```

- `--store=shared|isolated` is required. `shared` uses the fixed store `cargo/<configuration>/{target,build}` inside the issued compiler-workspace reservation. `isolated` creates a private `cargo/<configuration>/isolated/<nonce>/{target,build,tmp}` inside the same reservation and removes it when the command settles.
- `--configuration=prototype|release` is required. It selects the compiler store and the Cargo profile (`prototype`/`wasm-prototype`, `release`/`wasm-release`). It must match `SEMIO_BUILD_MODE` when that variable is set.
- `--parallelism=<cores>` is optional. The default comes from the policy (`2`). This is the number of host cores the command is admitted for; it must not exceed the host's core count.
- The first argument after `--` must be the Cargo subcommand. Global flags before the subcommand and a literal `cargo` program name are refused.
  - compile: `build check test bench run doc clippy rustc rustdoc`
  - resolve: `metadata tree fetch pkgid locate-project verify-project read-manifest`
  - lock: `generate-lockfile update`
- Relative paths (such as `--manifest-path`) resolve against the repository root, because `bun repo` runs there.
- Do not set `RUSTC`, `RUSTC_WRAPPER` or `CARGO_TARGET_DIR`. The launcher sets them, and `--target-dir`, foreign `--config`, `--artifact-dir`, `--lockfile-path` and `--root` are refused.

The fleet wrapper keeps its old usage: `<ticket>/🚦️cargo-slot.sh <agent> -- cargo <args>`. It translates the call into the line above:

- store: `isolated` when the caller set `CARGO_TARGET_DIR`, otherwise `shared`;
- configuration: `SEMIO_BUILD_MODE` when set, otherwise `release` for `--release` or `--profile (wasm-)release`, otherwise `prototype`;
- a relative `--manifest-path` is made absolute against the caller's directory.

It then `exec`s the launcher, so stdout, stderr and the exit status pass through unchanged. The 6-slot mkdir gate is removed. The launcher's own host-capacity gate, jobserver and memory broker already share the machine, which made the slot gate redundant, so the script is now a thin pass-through.

Examples:

```
bun repo cargo --store=shared --configuration=prototype -- check --manifest-path 🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/Cargo.toml -p semio-framework-diagnostic
bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --target wasm32-wasip2
bun repo cargo --store=isolated --configuration=release --parallelism=4 -- build -p <crate>
bun repo cargo --store=shared --configuration=prototype -- test -p <crate> -- <filter> --nocapture
bun repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 --no-deps
bun repo cargo --store=shared --configuration=prototype -- update -p <dep> --offline
```

## Finding: The Half-Built Entry

An issued ad-hoc entry already existed, half-built. `runRepositoryCargoCommandV1` (`🏃️process/🦀️cargo/🎟️admission/🟦️.ts`) is called from about 35 framework `📜️script.ts` sites. It already went through:

- `withRepositoryCompilerStoreV1`: issues or recovers the `compiler-workspace` reservation `repo:compiler:<mode>` and calls `repositoryCompilerLaunchEnvironmentV1`;
- bounded registry custody;
- unit-graph capture and Source boundary admission;
- the target/build lease.

What it lacked:

- a CLI;
- jobserver, host-capacity and memory admission;
- a store choice;
- exit-status forwarding (it threw);
- metadata, tree and lock commands (`fetch --locked` and the unit graph made sense only for compilers).

I finished that entry and added no parallel one. `runRepositoryCargoCommandV1` now delegates to the new canonical core `launchRepositoryCargoV1`.

Nothing in the peer fleet was a shared jobserver or a shared memory broker. Both existed only per A1 driver run (`runNativeBuildDriverV1`: a private jobserver plus `startAdmissionBrokerV1`). The only cross-process host gate was `acquireCompilerCapacityLeaseV1`, and it offered all cores (exclusive) or one core. The launcher therefore composes these primitives as described below.

## Design

`launchRepositoryCargoV1(invocation, workspace, request, cwd, env, {maximumMilliseconds, output})` returns Cargo's exit status. It runs these steps in order:

1. **Reservation.** `withRepositoryCompilerStoreV1(workspace, configuration, …)` issues or recovers the live `compiler-workspace` NativeStorageReservation, rechecks it, and gets the environment from `repositoryCompilerLaunchEnvironmentV1` (`RUSTC=rustc`, `RUSTC_WRAPPER=""`, CARGO_HOME, target, build and TMP inside the fixed image). Nx graph construction is never involved: the `cargo` branch in `runOwnedBuildInLeasesV1` runs before `ensureBuildEnvironmentV1` and before any graph load.
2. **Store.** For `shared`, it uses the store's fixed target and build. For `isolated`, `withRepositoryCargoIsolatedStoreV1` first sweeps abandoned stores. It acquires each store's exclusive SQLite claim (`<isolated>/<sha256(nonce)>.sqlite`) with timeout 0; success means the owner is dead, because the kernel released its lock. It then takes its own claim before `mkdir`, and on return, throw or signal it removes the directory, releases the claim and unlinks the claim file. The claim files live on the Rust image, not on the 64 MiB lease image, so nothing accumulates there.
3. **Registry custody by class.**
   - compile and resolve: `withRepositoryCargoInputsV1` runs `fetch --locked` under the writer lease, then the command under the reader lease.
   - lock: the new `withRepositoryCargoResolutionV1` holds exclusive writer custody without first fetching the stale locked graph.
   - Both share the private `admitRepositoryCargoInputsV1`, so argument and storage admission are one code path.
4. **Compile class only.**
   - `cargoArtifactArgumentsV1`: profile from the configuration, `--config .cargo/config.toml`, and `build.build-dir`.
   - Unit-graph capture plus `admitCargoSourceBoundariesV1`.
   - `acquireCargoBuildLeaseV1` for the selected target/build pair.
   - Then admission:
     - `acquireCompilerCapacityLeaseV1(requestedParallelism=k)` on the driver's own gate directory `.cache/framework/leases`. Ad-hoc commands and A1 drivers therefore queue on one fair FIFO host gate. An exclusive driver waits for shared holders and vice versa.
     - A `createJobserverV1` pool of exactly `k` tokens (fifo inside the store's tmp). Cargo and rustc inherit it through `MAKEFLAGS`/`CARGO_MAKEFLAGS`.
     - `RUSTC_WRAPPER=semio-build-scheduler`, built with `buildSchedulerV1` into the shared store, plus a `startAdmissionBrokerV1` sized to the same share of memory: `reserve + (total − reserve)·k/N`.
     - A 1 s `memoryCountersV1` sampler feeds swap-out into the broker freeze.
     - Children get `SEMIO_COMPILER_CAPACITY_HELD=true` with `SEMIO_COMPILER_PARALLELISM=k`.
     - Nested inside a driver job (capacity already held), the inherited jobserver and broker are used unchanged.
5. **Output and status.** `runOwnedCommand` pipes stdout and stderr straight through, with stdin inherited and no buffering. Its 10 s heartbeat goes to stderr. Launcher progress is rate-limited to one line per second. An `OwnedCommandFailure` becomes Cargo's status (or 128 + signal number), which the CLI sets as `process.exitCode`. SIGINT/SIGTERM abort the original invocation: the Cargo tree is terminated, every `finally` releases the claim, jobserver, broker, capacity, build lease and image custody, and an isolated store is removed.
6. **Shared reservation.** `repo:compiler:<mode>` is the persistent shared working set by design. Each command releases its custody (image reader, leases); the durable charge stays, exactly as for every other repository compiler.

### Capacity Extension (Peer Module)

`acquireCompilerCapacityLeaseV1` now admits `requestedParallelism: k` for any k from 1 to N, with schema `integer ≥ 1`. Without `requestedParallelism` it still takes all cores exclusively. A k-core claim holds the shared gate, then takes the new FIFO picker `semio.compiler.cpu-picker/v1` and acquires `k` core leases all-or-nothing. A partial set is released and polled again, so the claim cannot deadlock, and claims stay first come, first served. k = 1 behaves as before. Fixture: `{requestedParallelism:2}` is now accepted and `1.5` is refused. A new law covers whole-claim waiting and FIFO order.

### Unit-Capture Fix (Peer Module)

`captureCargoCompilationGraphV1` appended everything after `--` to the root units' rustc flags for every subcommand. As a result, `cargo test -p X -- <filter>|--nocapture` died in Source admission with `rustc --print cfg … --nocapture`. Those arguments are now rustc flags only for `cargo rustc`. Test and bench pass-through arguments go to the test binary. A new actual-Cargo law covers test, bench and rustc: RED before the fix (`Expected false, Received true`), GREEN after.

### Coordinator Findings Addressed

1. Metadata, tree and lock resync gate on `rustc -vV`. They are now the launcher's `resolve` and `lock` classes, and proof runs are below.
2. The launcher never touches Nx graph inference or publication.

## Changed Files

- New `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/`: `🟦️.ts`, `🔣️policy.json`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🔣️.json`.
- `…/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts`: `runRepositoryCargoCommandV1` delegates to the launcher (shared store) and throws `OwnedCommandFailure` on a nonzero status.
- `…/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts`: shared `admitRepositoryCargoInputsV1`; new `withRepositoryCargoResolutionV1`; the fetch `[DEBUG]` line moved from stdout to stderr, so `metadata` stdout is pure JSON.
- `…/📚️library/🏗️build/📜️script.ts`: the `bun repo cargo …` branch and the usage text.
- `…/📚️library/⚡️caching/📜️script.ts` and `📋️project.json`: the `test cargo-launcher` route and the `repo:test-cargo-launcher` target.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/`: `🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json` (k-core claims).
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/`: `📥️capture/🟦️.ts`, `🧪️tests/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json` (pass-through fix).
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`:
  - `🦀️ Repo Cargo prototype|release` (inputs `repoCargoStore` and `repoCargoArguments`);
  - `🦀️ Repo Cargo Launcher prototype|release`;
  - all placed next to `🚪️ Cargo Ingress`.
- Ticket: `🚦️cargo-slot.sh` (now a pass-through), `k0-rustc-sampler.py` (the concurrency probe input).

## Tests (All Actually Run)

| Gate | Result |
|---|---|
| `⚡️caching/📜️script.ts test cargo-launcher` (registered route) | 3 pass / 0 fail. 21 argument cases agree with commander, Ajv and a SQLite `json_each` subcommand classifier. 5 isolated lifecycles on a real 32 MiB fixed image: return, throw and abort remove the store; a SIGKILLed owner's store is swept by the next command; a live peer's store is kept until it is released. Python `os.path.isdir` and SQLite expectations agree. |
| Capacity suite (`🏗️build/🎟️capacity/🧪️tests`) | 5 pass, including the new multi-core FIFO law. |
| Unit-capture suite (`SEMIO_BUILD_MODE=prototype`) | 9 pass, including the new pass-through law (shown RED first). |
| `cargo-ingress` route, dependencies tests | 5 pass; 1 pass. |
| Strict tsc (ES2022, `noUncheckedIndexedAccess`) over launcher, launcher tests, admission, capacity and capacity tests | exit 0. `--listFiles` confirms the launcher and the dependencies module were compiled. |

## Proof Runs (Real Cargo Through `bun repo cargo`)

| Command | Result |
|---|---|
| `check -p semio-framework-diagnostic` (shared, prototype) | exit 0; `2 of 10 cores admitted, rustc memory budget 5.2 GiB`; first run 49 s, warm 13 s (Cargo 0.31 s) |
| `check -p semio-framework-diagnostic --target wasm32-wasip2` (shared) | exit 0; cold 52 s at 2 cores; warm 11 s |
| the same check, isolated, concurrently with the wasm check | exit 0; the isolated directory existed during the run and was gone (directory and claim file) afterwards |
| two isolated cold checks at `--parallelism=2` and `=3`, sampled by `k0-rustc-sampler.py` | peak admitted rustc units (children of the scheduler) 2 and 3, equal to the jobserver tokens |
| `metadata --format-version 1 --no-deps` | stdout is pure JSON, parsed by Python (164 workspace members) |
| `tree -p semio-framework-diagnostic --depth 1` | exit 0 |
| `update -p serde --offline --dry-run` (lock class) | exit 0; `framework/Cargo.lock` sha unchanged |
| `test … --lib -- --k0-unknown-libtest-flag` | exit **101** forwarded from Cargo |
| SIGINT to the launcher during an isolated cold check | the isolated store and claim were removed, no rustc or scheduler survived, exit 1 (`stopped: SIGINT`) |
| `check -p semio-framework-value` (large root) | exit 0; Source admission passed |
| `🚦️cargo-slot.sh k0 -- cargo check --manifest-path <relative>` from a subdirectory, and with `CARGO_TARGET_DIR` set (→ isolated) for `metadata` | exit 0; translated lines logged; isolated `target_directory` inside the reservation, and no `/tmp` target created |

## Known Limits

- **Pre-compile overhead.** Each compile-class command spends about 10–20 s before Cargo starts: registry `fetch --locked` about 4 s, unit-graph plus Source admission about 5–17 s, scheduler freshness about 2 s. That is the cost of custody. A warm `check` totals about 11–13 s.
- **Isolated stores are cold.** They are deleted on settlement, so every isolated command compiles from scratch inside the 16 GiB Rust image. Use `shared` unless you really need isolation.
- **Errors before Cargo starts exit with 1.** An admission failure that happens before the real Cargo run (for example `-p` naming an unknown package, refused during unit-graph capture) is reported with Cargo's message and exit 1, not 101. An interrupted command also exits 1.
- **SIGKILL orphans Cargo.** When the launcher is SIGKILLed, its detached Cargo child keeps running. The next isolated command sweeps the dead owner's store, and the orphan then fails.
- **Old slot directory.** `⚡️cache/play-fleet/slots` is no longer used by the script. It was left in place for still-running old wrapper instances.
- **macOS only.** Only macOS was qualified (fixed HFS+ images, a fifo jobserver).
