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
