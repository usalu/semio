# 📌️ Binding rules — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME

**Empty this file before `ticket_close`.**

## Hard prohibitions (every agent)

1. **No git-modifying commands.** No `commit`, `stash`, `checkout`, `reset`, `worktree`, `add`. Other sessions are live in this tree and an auto-commit bot runs. `git status` is NOT a churn detector — use `git log --oneline -3 -- <path>` and file hashes.
2. **No `ticket_close` / `ticket_reopen` by anyone but sol.** A subagent closing this ticket closes the whole umbrella.
3. **Never edit outside your packet's `path_scope`.** A region name inside a shared file is not ownership. Need a shared-file change → emit a `lease-request` block and stop.
4. **Never run bare workspace cargo.** Always `CARGO_TARGET_DIR=<ticket>/🎯️target` and `-p <crate>`. A slow build is not a hung build.
5. **Scratch files are `.txt`/`.md`/`.json` inside the ticket folder.** Never `.log` (repo-wide gitignored — `ticket_close` silently drops them).
6. **Do not touch `.cargo/config.toml` or add per-crate `RUSTFLAGS`** — the uniform 512 MiB wasm limit is deliberate; per-crate flags churn cargo fingerprints across the whole fleet.
7. **Never claim a test passed without pasting its output and exit code.**
8. Temporary logs carry the `[DEBUG] ` prefix and are removed before a packet reports done.

## Registrar-only files (sol edits these; everyone else sends a `lease-request`)

`/📜️script.ts`, `/Cargo.toml`, `/Cargo.lock`, `/📋️project.json`, `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json`, `🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `🔌️plugin/📦️packages/🟦️typescript/📇️registry/**`, all `🤖️generated/**`, `Shell/🧊️component.rs` (shared with live hover/selection tickets), `ShellHost/🟦️component.tsx`.

## Replace, never wrap — these must not exist at exit

`exchange` (WIT + all callers) · `PluginWorkerClient` (BOTH copies: `🎠️kernel/🟦️component.ts`, `🧊️wgpu/🟦️typescript/🟦️boot.ts`) · `LeasePool`/`PluginModuleLease` in the kernel (the generic `createLeasePool` relocates to `📦️packages/🟦️typescript/🟦️glue.ts` for its 3 non-plugin users) · `WasmPluginRuntime` · `ExtensionRuntime` · **both** `ProgramSupervisorState` definitions · `PLUGIN_FUEL_BUDGET` · `PLUGIN_WORKER_UNRESPONSIVE_MS` · `INSTANCE_GUARD`/`clear-instance-guard` · `host_port` · `component::host_*` · `install_io_fallback_dispatcher` · `set_host_backbone_channel` (process-global) · `runSerialized` retry/reload loop · `loadPluginModuleUncached`.

## Naming hazards

- `kernel::ActorId` **already exists** (re-export of `protocol_core::ActorId`, the presence/collab actor, `🎠️kernel/🦀️component.rs` L40). The runtime actor id is re-exported as **`RuntimeActorId`**. Never shadow.
- `🎭️actor` crate must stay pure: no `wasm_bindgen`, `web_sys`, `winit`, `tokio`, `std::thread` in the crate core — transports are injected. This is what keeps mobile open.

## Sequencing constraints

- The ABI flip is **big-bang**: A2/A3/B1 land and the fleet rebuilds before W3 fans out. The SDK crate is frozen during W3.
- `🗄️stdio` migrates alone and first in W3 (every plugin depends on it). `🎪️demonstrator` migrates last (bundles panes from cad/process/puzzle/procedural/gis/sourcing).
- Linked extension mode is feature-gated to avoid the `semio-framework-os-flow` ↔ extension crate cycle.
- Descriptor `extends` gates extension-actor activation on parent activity — a linked extension must not also run as an actor.

## ⚠️ Live peer ticket contending for our core files (2026-08-17 21:05)

`26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` slice **W1-D** is running RIGHT NOW (its `📓️w1-d-report.md` was written 21:02) and holds large **uncommitted** work in files this ticket must rewrite:

| file | peer's uncommitted delta | our packet |
|---|---|---|
| `🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit` | +37 (guest `list-io-entries`/`io-run`/`io-sniff`; host `io-routes`/`io-run`/`io-identify`) | `A2` |
| `🔌️plugin/🖥️host/🦀️component.rs` | +459 (`IoRouter` route resolution) | `B1` |
| `🔌️plugin/🦀️component.rs` | +1259 / −96 (io mechanism in the guest SDK) | `A2` |
| `🎠️kernel/🟦️component.ts` | +341 (`//#region 🔖️IoRouter`, `IoEntryGraph`, `ioRun`) | `A3` |

Rules that follow:

- **User decision 21:10: proceed now, absorbing the current working tree.** The hold on `A2-abi-sdk` / `B1-host-native` is lifted. The peer's uncommitted work **is the baseline** — treat the working tree, never `HEAD`, as the state to build on. Any agent in those files re-reads from disk immediately before every edit, edits surgically by region, and must be able to show the peer's io mechanism still present (as absorbed job kinds / effects) at the end.
- When A2/B1 do run, they **absorb** rather than delete the io mechanism: guest `io-run`/`io-sniff` become the cold job kinds `semio.io-run`/`semio.io-sniff`; host `io-routes`/`io-identify`/`io-run` become the `RegistryQuery` and `IoCompose`/`IoRun` effect variants with completions. The route-resolution algorithm, the ≤3-hop cycle-free rule, the ranking order (highest minimum fidelity → fewest hops → lexicographic) and the self-owned-hop reentrancy guard are all preserved semantics — they map onto host-side routing after a turn, which is exactly where the new design already puts cross-plugin routing.
- Any agent editing a file in that table makes **surgical region-scoped edits only**, never a full-file rewrite, and re-reads from disk immediately before each edit.

## Environment

- Disk was at 100% on 2026-08-17; freed by removing the `🎯️target` dirs of the two CLOSED tickets `☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` and `☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` (user-approved). sol checks `df -h` at every wave start and asks the user before deleting anything further.
- Ports: bench/parity use the 7300+ pool via `findFreeParityPortPair`, never the catalog ports 6012–6205.
- ≤6 concurrent building agents (cargo lock + disk).
- **One ticket target dir serializes our own builds.** Observed 2026-08-17: two of this ticket's own commands sat on `Blocking waiting for file lock on build directory` against our shared `🎯️target`, while a third peer ticket saturated the global `~/.cargo` package-cache lock — 54 cargo processes, a wasm check that compiles in seconds taking 23 minutes. Consequences, binding from W2 on:
  - **Only ONE packet at a time may hold a cargo build.** Parallel *editing* is free; parallel *building* is not. Stagger acceptance runs, or give each concurrently-building packet its own `🎯️target-<packet>` dir (prior tickets did exactly this — e.g. `🎯️target-w3-cad`, `🎯️target-verify`) and accept the disk cost.
  - Prefer one `-p <crate> --all-targets` check per packet over a suite of narrower ones; never `--workspace` from an executor.
- **Executors must run cargo in the FOREGROUND, in a single turn.** All four W1 executors independently stalled in wake/idle loops on backgrounded builds that cannot survive a subagent turn boundary (~1.4M tokens spent collecting nothing). Acceptance runs belong to the coordinator session; executors report what they actually observed.
- **After any atomic rename, the coordinator re-greps the tree.** A3's 132-file sweep missed a live `type HostEffect` import in the React renderer entry — an executor's own file count is not proof of completeness.

## W5+ additions (async-first rewrite) — measured this session, binding on every packet

9. **An API that exists is not an API that is implemented.** wasmtime 34.0.2 exposes the whole `component-model-async` surface — feature flag, `Config` knobs, types — and its engine is ~35 bare `todo!()` bodies with `StreamReader<T>` carrying zero trait impls. It compiles, links, and panics. Before adopting any new external capability, read its source or execute it; version availability is not feature availability. (The certified working version is **wasmtime 47.0.3**.)
10. **`cmd | tail -N; echo $?` reports tail's exit code, not the command's.** Read the pass/fail summary line, or run the command without a pipe. A confidently-pasted wrong exit code is worse than none — and rule 7 demands exit codes.
11. **Baselines are named sets, never counts.** Failure counts are meaningless across a suite that has grown (the plugin SDK suite went 230→247 tests, so "4 failures" became uncomparable and cost a packet real effort proving innocence). Record *which* tests fail, and settle attribution by running suspects **in isolation** — deterministic-alone means pre-existing; passes-alone-fails-in-suite means shared global state.
12. **After fixing one variant of a serde-shape defect, sweep every sibling.** `#[serde(tag = "kind")]` cannot serialize a newtype variant whose payload is not a map (string/int/`Vec<u8>` all fail at RUNTIME, compiling clean). The `JobStep` fix recorded this instruction in W4 and nobody executed it; six more instances were sitting in `🎭️actor` (`Payload::Event`/`Cancel`, `Origin::Actor`, `TurnStatus::Faulted`, `FailureSignal::Trap`, `Backpressure::Dropped`). They are **latent, not live** — that crate has no `serde_json` dependency and the wire uses the hand-rolled `pack_encode` — but the generated TS mirror renders them as impossible `object & string` intersections, so the mirror cannot type those variants.
13. **A vitest config with explicit filename arrays silently ignores new files.** `🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts` lists names in `include`/`coverage.include`/`includeSource` rather than globbing, so a new test file **does not run while the suite still reports green**. Add the filename, then re-run with `--reporter=verbose` and confirm your tests appear **by name**. Several packages also double-count in-source suites (91 unique → 182 reported); divide before comparing to a baseline.
14. **Never name the hidden library in the interface that hides it.** `semio-framework-async` briefly had a `tokio_workers` field and a `ThreadRole::TokioWorker` — in a serialized, ts-rs-mirrored type, so the leak would have reached the TypeScript wire. Now `io_workers` / `IoWorker`. Doc-comment prose naming today's concrete choice is fine; identifiers are not.
15. **W5+ packet ids are descriptive slugs, not letter-numbers.** `A1`, `H2`, `P1`, `M1`, `R1`, `G1` and `W0` all collide with ids/waves/gates this ticket already used; one packet nearly overwrote a finalized `📓️terra-R1-report.md`. Use `spike`, `async-iface`, `params`, `wasmtime-upgrade`, `services`, `shard-grants`, `kernel-loop`, `effects-async`, `shell-unpark`, `directory-and-run`, `lifecycle`, `sdk-async`, `async-worlds`, `packaging`, `e2e-proof`, `web-*`. Reports are `📓️terra-<slug>-report.md`.
16. **Reconnect backoff must reset after SUSTAINED health**, never on socket-open alone (open-only resetting defeats the backoff against an accept-then-drop server, which is the failure it exists for). Required by the "support short connection-shortages without freezing" rule: a monotonically growing counter makes a healthy session wait ~`maxMs/2` after a momentary blip.
17. **Do not put two packets in one file.** `shard-grants` was held out of `🖥️host/🦀️component.rs` while `wasmtime-upgrade` rewrote it, and the TurnResult bridge was relocated to `🧵️shard/` so the collision cannot recur. This ticket has already absorbed four half-landed peer changes with the same signature — *the artifact moved, its registration did not*.

18. **`include` + `includeSource` naming the same file makes vitest collect it TWICE.** Every TS baseline recorded in this ticket before 2026-08-18 ~20:30 was inflated 2×. Fixed in the four packages this ticket touches (`🧰️framework`, `💻️os`, `🧑️‍💻️dev`, `🎭️actor`) by setting `include: []` and keeping `includeSource`. **In-source suites belong in `includeSource` only.** Other packages still carry the bug (`mcp`, `shell`, 4 cad extensions, `animate` — see `📓️terra-web-kernel-package-report.md`). Corollary: a file absent from `includeSource` does not run at all while the suite still reports green, so adding a file means editing that list.

### Current verified baselines — **RE-MEASURED after the double-count fix** (measure again before trusting)

| target | baseline |
|---|---|
| `semio-framework-actor` (test) | **60 passed / 0 failed** |
| `semio-framework-plugin-host --lib` | **86 passed / 0 failed / 1 ignored** |
| `semio-framework-plugin --lib` | 242-ish passed, **5 known failures** — 4 fail in isolation (pre-existing), `a_child_survives_…channel_frames` passes alone (global-state interference). Compare NAMED SETS, not counts |
| `semio-framework-async` (test) | **16 passed / 0 failed** |
| `semio-framework-os-services` (test) | **26 passed / 0 failed** |
| `🧰️framework/📦️packages/🟦️typescript` | **87 passed** (was reported 174 pre-fix) |
| `🎭️actor/📦️packages/🟦️typescript` | **29 passed** (was reported 58 pre-fix) |
| `🎠️kernel/📦️packages/🟦️typescript` | **29 passed** — NEW package; these tests were in no gate at all before |
| `💻️os/📦️packages/🟦️typescript` | **184 passed / 2 failed** (was reported 370/2 pre-fix). The 2 failures are **two DISTINCT pre-existing** Rust-fixture/wasm tests, not one doubled: `🟦️component.ts` → `matches the Rust plan_workflow … decoded via wasm`, and `🟦️backbone-worker.ts` → `decodes the Rust-generated binary wire fixtures byte-identically`. I previously mis-recorded these as a single doubled failure because I grepped for only one of the two names — a narrow grep is not a census |
| `🧑️‍💻️dev/📦️packages/🟦️typescript` | **17 passed** (was reported 34 pre-fix) |
| repo-wide `tsc --noEmit` | **19 pre-existing errors** in trinity / stdio schemas / vscode extension — routed to a separate task, not this ticket's. Exit code observed as both 1 and 2 by different runs; report what you see |

## W4 additions — measured 2026-08-18, binding on every packet

These are not advice. Each cost a packet real time this session, and several were discovered twice
because the first discovery stayed buried in a report nobody else read.

1. **`--features component-guest` is NOT a plugin-crate feature.** Plugin crates declare no
   `[features]` section at all; `component-guest` is a *dependency* feature each enables on
   `semio-framework-plugin`. Passing it to `cargo -p <plugin>` fails with "does not contain this
   feature". Found by D0, re-found by Z1 after it blocked an entire target, and present in sol's own
   `verify rust-warnings` verb until Z1 hit it.
2. **Descriptors live at the plugin OWNER ROOT**, sibling of `🛂️manifest.json` — never under
   `🤖️generated/`, which is globally gitignored and therefore cannot hold a committed artifact.
3. **A descriptor is only ratcheted after its `descriptor_is_fresh` test passes.** Emitting is safe;
   ratcheting a plugin whose declarations may still move turns the tree red for every session.
   Unratcheted descriptors still feed the generated catalog, so a stale one is a silent
   data-correctness bug — that is the trade, and it is deliberate.
4. **`[DEBUG] ` means DELETE ME.** It has been repurposed for permanent operator diagnostics
   (312+ repo-wide); a blind sweep would strip the bench's entire error reporting. Re-prefix
   permanent diagnostics; only genuinely temporary lines carry the marker.
5. **Fuel exhaustion and pooling caps surface as a bare "error while executing"** with no mention of
   fuel or of which pool. Measure, never estimate: `🗒️note`'s `describe()` alone burns ~92M fuel in
   an unoptimized wasip2 build, and wasmtime meters component instances, core instances, memories,
   tables and GC heaps from FIVE separate pools that each default to 1000.
6. **Native builds never compile `#[cfg(target_arch = "wasm32")]` code.** A signature change can
   leave the wasm bindings broken behind a green native build AND a green test suite. `verify gate`
   now compiles the actor kernel's wasm bindings for exactly this reason.
7. **A test that passes against `MockGuestRuntime` is not a test of the runtime.** Every one of the
   ten defects found this session was covered by a green `cargo check` plus mock-backed tests.
8. **Cross-packet findings must be lifted HERE or into a coordinator message the moment they are
   read.** A finding left in a packet report does not reach a sibling packet. Item 1 above is the
   proof: correct, written down, and still cost a second packet a fully blocked target.
9. **Executors: run cargo in the FOREGROUND, in one turn.** Background watchers do not survive a
   subagent turn boundary. Six packets have now lost budget to this; briefs alone do not prevent it.
10. **Prune `🎯️target-*/**/incremental/` and stale `.wasm` between plugins.** One packet reached
    84 GB before doing so; after pruning it held ~12 GB for the rest of its run.
19. **Pass an explicit long `timeout` to every build command — the Bash tool auto-backgrounds at ~120 s by default.** This, not carelessness, is the mechanism behind the wake/idle trap: three packets in one wave "chose" to background builds because the harness detached them at the default, then idled across a turn boundary where the result can never arrive. Executors: set `timeout` to the maximum (600000 ms) on every cargo command, and if a build still exceeds it, report it unrun rather than detaching. Coordinator: your `run_in_background` tasks DO survive turn boundaries and notify you — subagents' do not. That asymmetry is why acceptance runs belong to the coordinator.
20. **A `use`d WIT type is an ALIAS, not the same `TypeId`.** `wit-parser` materialises `use effects.{x}` as a fresh `TypeDef` whose kind is `TypeDefKind::Type(original)`, so genuinely-shared types compare UNEQUAL by raw id. Any schema test comparing types must resolve alias chains to their root first (`canonical_type` in `🖥️host/🧪️schema-parity/`). This produced a false "the async world copied the payload records" report — the schema was correct.
21. **A negative result from a query that cannot report its own failure is not evidence of absence.** Four times in one wave a too-narrow or silently-failing query produced a confident wrong picture: a grep that turned two distinct test failures into "one doubled"; a `find -newermt` returning nothing while `ls` showed a one-minute-old file; a `grep wit_bindgen::generate!` that missed the scale fixture's private generator and cost the bench a run; and a file-existence gate that counted `#[path = "."]` directory anchors as missing files and so could never pass. Three would have produced a wrong conclusion about ANOTHER session's work. **Where a negative would change a judgement, reproduce it with a differently-implemented tool** — shell globbing/`find`/`grep` over emoji paths have all silently under-reported; python over explicit absolute paths has not.
22. **Acceptance must run the command the CONSUMER runs, feature flags included.** `cargo build -p semio-framework-os-scale-fixture --target wasm32-wasip2` succeeded while the bench's own `--features component-guest` build failed on W0 fallout — I verified an artifact built from a code path that excluded the defect, then reported the bench unblocked. A build without the feature gating the code under test is not evidence about that code.
23. **Executors must not run acceptance builds at all when the machine is loaded — the COORDINATOR owns every build.** Five packets in one wave ended a turn idling on a detached build. The mechanism is structural, not a lapse: the Bash tool auto-backgrounds at ~120 s, a subagent's detached job cannot report across its turn boundary, and above ~20 concurrent cargo processes even the 600 s maximum timeout will not finish a wgpu build — so "run it in the foreground" stops being available and detaching looks like the only option. A coordinator's `run_in_background` task DOES survive and notify. Therefore: briefs should ask executors to **write code and reasoning**, run only cheap checks, and mark acceptance **UNRUN**; the coordinator runs the real gates and pastes the numbers. This costs nothing — the coordinator was re-running every packet's acceptance anyway, because an executor's own figure has never been accepted as evidence on this ticket.

### ⏱️ LATEST coordinator-verified baselines — supersedes the table above (2026-08-19, W5)

| target | verified |
|---|---|
| `semio-framework-actor` | **70 passed / 0 failed** (60 → 69 shard-grants → 70 interactive-isolation) |
| `semio-framework-plugin-host --lib -- --skip schema_parity` | **115 passed / 0 failed / 1 ignored** |
| `semio-framework-plugin-host --lib schema_parity` | **4 passed / 0 failed** (the 3 that failed were the TEST comparing raw `TypeId`s across `use` aliases — see rule 20) |
| `semio-framework-async` | **16 / 0** · `semio-framework-os-services` **26 / 0** |
| `semio-framework-plugin --lib` | ~242 passed, **5 known failures BY NAME** (4 fail in isolation, `a_child_survives_…channel_frames` passes alone) |
| `semio-framework-os-renderer-wgpu --lib` | **exit 0** (`--all-targets` still fails on another session's `Dock` test-module break — not ours) |
| `🧰️framework/📦️packages/🟦️typescript` **87** · `🎭️actor/…/🟦️typescript` **40** · `🎠️kernel/…/🟦️typescript` **29** · `💻️os/…/🟦️typescript` **206 / 1** · `🧑️‍💻️dev/…/🟦️typescript` **17** · react-renderer **325 / 336** (11 = exact subset of the 15-name baseline) |
| native bench, `--shards 4` | **7 of 8**; only budget 5 fails, and it is an **instrument** defect under correction — see the W5 consolidation entry |

The single remaining `💻️os` failure (`matches the Rust plan_workflow … decoded via wasm`) is **not** ambient: `pkg/semio_framework_os.js` cannot build because `RUSTFLAGS` replaces `.cargo/config.toml`'s wasm32 `getrandom_backend` cfg. Routed out-of-band. Do not re-label it "pre-existing" — that word cost this ticket two days of carrying a fixable bug.
