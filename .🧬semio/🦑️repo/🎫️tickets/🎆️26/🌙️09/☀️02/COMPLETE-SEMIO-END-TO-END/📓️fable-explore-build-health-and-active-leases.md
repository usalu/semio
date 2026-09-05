# Fable explore: build health and active leases

Read-only host census. Snapshot window: **2026-09-05 03:59 → 04:05 CEST** (host `uptime` says up 1 day 18h31m). No build was run by this exploration; every verdict below is quoted from a receipt already on disk or inferred from `ps`/`lsof` at the timestamps given. Where I could not find a receipt for a crate, I say so explicitly rather than guessing.

Method note: `ps args` on this box renders non-ASCII (the repo's emoji filenames) as BSD `vis`-style `M-x`/`M^x` escapes. I decoded these byte-for-byte (meta-bit + control-char re-assembly, validated as UTF-8) to recover real paths; all paths quoted below are the decoded, real filesystem paths.

## 1. Active builds right now (pid table)

| PID | PPID | Command (crate / target / profile) | CARGO_TARGET_DIR | Owner guess (evidence) | Elapsed | CPU | State |
|---|---|---|---|---|---|---|---|
| 58069 | 58016 | `cargo test -p semio-hub --lib --no-run` | `…COMPLETE-SEMIO-END-TO-END/🗑️generated/space-public-boundary-sol-target` | Codex (ChatGPT) session — ancestry 58069→58016(`bun 📜️script.ts gis-map-frozen-binding-check --native`)→57672→57639→57631→**66250 `codex` app-server** | ~08:09 | 0% | **BLOCKED**: stderr = `Blocking waiting for file lock on build directory` (contending with 90794 below for the same target dir) |
| 90794 | 90793 | `cargo test -p semio-s-plugin-gis --lib --no-default-features --no-run` | same `space-public-boundary-sol-target` (holds the lock) | Codex session — 90794→90793(`bun 📜️script.ts map-create-region-group-native-check`)→90731→90730→**66250 `codex`** | ~41:45 | live | Compiling (wgpu/vello/usvg graphics stack, then `semio-s-plugin-stdio`; gis itself not reached yet as of last stdout write) |
| 60185→(finished) | 59918 | `cargo test -p semio-framework-os-kernel --lib --no-run` | `…/fable-hub-native-qualification/cargo-target` | **genuine Claude Code session** — 59918(`bun 📜️script.ts presence-lease-check native`)→59082→58907→58901→58899→58892(zsh)→**21392 `claude`** | finished 04:04 | — | **Exited 0 (GREEN)** — see §3 |
| (chained) | 59918 | `cargo test -p semio-hub …` (same script, next crate) | same lease dir, `…/01/` | same Claude session | started 04:04 | live | Compiling (still in early dependency stage — serde/syn/libm — no verdict yet) |
| 12147 | 2057 | `cargo check --quiet --workspace --message-format=json --manifest-path Cargo.toml --keep-going --compile-time-deps --all-targets -Zunstable-options` | root `/Users/ueli/Documents/semio/target/debug` | long-lived shell (ppid 2057) | ~30:21 | 0% | running (holds shared lock on root `target/debug`) |
| 16308 | 2057 | `cargo check --workspace --message-format=json-diagnostic-rendered-ansi --manifest-path Cargo.toml --keep-going --all-targets` | same root `target/debug` | **same parent 2057 as 12147** — two overlapping full-workspace checks from one session, contending on the same root target lock | ~28:41 | 0% | running |
| 22180 | 1 | `cargo run --quiet -p semio-framework-os-mcp --bin semio-os-mcp -- stdio` | root `target/debug` | detached (ppid 1) — a long-lived MCP stdio server, not really a "build" | ~26:05 | 0% | running (service, not compiling) |
| 1597 | 1 | `cargo check -p semio-s-plugin-puzzle --message-format short --keep-going` | `target-p3d-e2e/debug` | detached; has an active rustc child (pid 38792, was 11-25% CPU across samples) | ~35:22 | 0% | running |
| 96183 | 93349 | `cargo rustc -p semio-s-plugin-puzzle --target wasm32-wasip2 --profile wasm-dev -- -C link-arg=-zstack-size=8388608` | `target-demonstrator-dev` | **DEMONSTRATOR-END-TO-END-ALL-APPS** ticket's dev-profile pass | ~36:30 | 0% (rustc child varies) | running |
| 86722 | 84252 | `cargo rustc -p semio-s-plugin-stdio --target wasm32-wasip2 --profile wasm-release -- -C link-arg=-zstack-size=8388608` | `target-demonstrator` (release) | **DEMONSTRATOR-END-TO-END-ALL-APPS** — this is exactly the "~3h single-threaded LTO tail" the demonstrator's own status.md describes for `wasm-release` | **~3:00:06** | 9-25% (rustc child) | running, long-haul |
| 94794 | 66250 | `cargo test --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml --bin os-hub tests::document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable -- --exact --test-threads=1` | `…COMPLETE-SEMIO-END-TO-END/🗑️generated/open-plan-issuer-target` (**no longer exists on disk** — swept) | Codex session (parent **66250 `codex` app-server** directly) | **1d 03:56** | 0% (cargo) / test binary 94953 at ~1% | `cargo` itself sleeping (`Ss`); its child **94953 is state `R`**, still executing the compiled test binary `os_hub-a0145cf9870df4e1` even though its `CARGO_TARGET_DIR` was deleted underneath it. This looks like an **orphaned/hung test run**, not live signal — I could not find its target dir to check a verdict, and 27+ hours for one test is not a build in progress, it is stuck. |
| 9392, 15296, 58198, 49588 | — | were: `check -p semio-s-plugin-lowpoly`, `check -p semio-s-plugin-block --lib`, `rustc -p semio-s-plugin-sourcing wasm-dev`, `check -p semio-s-plugin-puzzle` (plain) | `target/debug` (root, shared), `target-block/debug`, `target-sourcing-e2e`, `target-p3d-agentE/debug` | mixed | — | — | **all four exited between my first and second snapshot** (5 min apart) — see per-crate table for the block one, which ties to BLOCK-PLUGIN-END-TO-END's own baseline check |

**rustc concurrency, 3 samples 30s apart:** 04:03:56 → 14, 04:04:27 → 15, 04:04:57 → 15. Stable ~14-15 concurrent `rustc` processes for the whole window, on top of ~10 `cargo` orchestrators.

## 2. Active leases (`.exact-cargo-laws-active-*/lease.json`)

I searched the whole repo for `lease.json` outside `target*` dirs; every hit is inside this ticket's own `🗑️generated`. (`.active` marker files: none found anywhere — this repo's lease mechanism apparently only uses `lease.json`, not a separate `.active` file.)

Note: `🗑️generated/exact-cargo-lease-red/exact-cargo-fixture-*/…` (≈70 dirs, all timestamped 00:47-00:50) are **test fixtures for the lease mechanism's own test suite**, not real leases — excluded from the table below.

| Lane | Lease dir | pid in lease.json | Alive? | What it actually is |
|---|---|---|---|---|
| `root-home-directory-projection` | `.exact-cargo-laws-active-ZnGAXK` | 14373 | **DEAD** (no such process) | Stale/orphaned lease from an earlier, already-ended run; lease file itself last written 01:56. |
| `gis-map-create-region-group-exact` | `.exact-cargo-laws-active-iUGZzU` | 90793 | **ALIVE** | `bun 📜️script.ts map-create-region-group-native-check` (Codex session, see §1) — holds the `space-public-boundary-sol-target` build lock right now. |
| `gis-map-frozen-binding-exact` | `.exact-cargo-laws-active-H28pV2` | 58016 | **ALIVE** | `bun 📜️script.ts gis-map-frozen-binding-check --native` (Codex session) — its cargo child (58069) is the one **blocked** waiting on the lock above. |
| `fable-hub-native-qualification/presence-lease-native-exact` | `.exact-cargo-laws-active-ETGmbk` | 59918 | **ALIVE** | `bun 📜️script.ts presence-lease-check native` (**genuine Claude Code session**) — just produced the GREEN `semio-framework-os-kernel` verdict below and is now chaining into `semio-hub`. |

## 3. Per-crate compiler verdict (latest known, no build run by me)

| Crate | Verdict | Errors | First signatures (file:line) | Receipt | Timestamp |
|---|---|---|---|---|---|
| **semio-hub** (lib, `--no-run`) | **RED** (latest completed) | 1 | `error: couldn't read …/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🚪️io/….rs: No such file or directory (os error 2)` — missing generated artifact consumed via `semio-s-plugin-stdio`'s generated include tree | `🗑️generated/root-directory-ordering/exact-cargo-laws-h1yzSy/00/build.json` (`status:101`) | 2026-09-05 01:48 |
| | *in-flight, no verdict yet* | — | (early dependency stage only) | `🗑️generated/fable-hub-native-qualification/…/8A0g3Y/01/build.stderr` | started 04:04, still running |
| | *in-flight, blocked* | — | waiting on file lock, not compiling | `🗑️generated/gis-map-frozen-binding-exact/…/cAzqZk/00/build.stderr` | since 03:56 |
| **semio-s-plugin-gis** (lib, `--no-default-features --no-run`) | **UNKNOWN — no completed receipt found anywhere searched** | — | — | in-flight only: `🗑️generated/gis-map-create-region-group-exact/…/2hMTLS/00/build.stdout` | still compiling as of 04:05 |
| **semio-s-plugin-stdio** (wasm32-wasip2) | **FLAPPING** | 0 broken `#[path]` mounts at the demonstrator's own audit, but 3 *different* downstream builds hit 3 *different* missing/broken generated-include subsets afterward | audit said `ok 🗄️stdio 0/4690` (structurally sound) at ~01:55; then 01:48-adjacent semio-hub build hit `brep/io`; 03:07 space-plugin build hit `drawing/io/…/dxf/r12`; 03:10 flow-plugin build hit `pdf/1.4/base/…mutations` (15 errors incl. `E0277: ReplacePageText: MutationLeaf not satisfied`) | `…/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️status.md` (audit) + the semio-hub/space/flow receipts above | audit ~01:55; failures 01:48, 03:07, 03:10 |
| **semio-framework-os-kernel** (lib, `--no-run`) | **GREEN** — freshest, most trustworthy verdict in this whole set | 0 | — | `🗑️generated/fable-hub-native-qualification/presence-lease-native-exact/exact-cargo-laws-8A0g3Y/00/build.json` (`status:0`) | **2026-09-05 04:04** |
| **semio-framework-os-kernel-db** | **UNKNOWN — no standalone `-p` receipt found**; only appears as a transitive dependency inside other builds (e.g. the semio-hub 01:48 graph) | — | — | — | — |
| **semio-framework-plugin-host** | **STALE / UNKNOWN** — dedicated cache `⚡️cache/agents/plugin-host-lifecycle-sol/cargo-test-hosts/` has only per-dependency build-script stderr (wasmtime, cranelift, object, …), no top-level verdict file, and nothing has touched it since | 32h+ stale, no verdict | — | `⚡️cache/agents/plugin-host-lifecycle-sol/cargo-test-hosts/debug/build/*/stderr` | last touched 2026-09-03 20:08 |
| **semio-s-plugin-flow** (lib, `-j1 --no-run`) | **RED** | 15 | `pdf/1.4/…/replace-page-text/….rs:12` MutationLeaf source-authority mismatch; `…/bachelor-thesis/….rs:15` missing example asset PDF; `…/replace-page-text/….rs:27` `E0277: ReplacePageText: MutationLeaf` not satisfied | `🗑️generated/flow-add-widget-retained-sol-run/exact-cargo-laws-eDmuyC/00/build.json` (`status:101`) | 2026-09-05 03:10 |
| **semio-s-plugin-norm** | **RED** (per peer audit, not a receipt I read directly) | 33/2413 broken `#[path]` mounts | tied to a `🌬️din16798` artifact tree; explicitly flagged "not a demonstrator dependency, left to its owner" | `…/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️status.md` | audit ~01:55, no evidence of a fix since |

Everything in the RED rows traces back to the **same root cause**: `semio-s-plugin-stdio` mounts generated Rust files via `#[path]` into artifact-standard subset trees (`🗿️artifacts/…`), and those subset directories are being renamed/edited live by other concurrent sessions while builds run — exactly the failure mode the demonstrator ticket's own status.md documents ("the peer is renaming subsets one at a time, and each rename breaks stdio until its references catch up"). None of the three RED receipts above necessarily reflect the *current* tree — they reflect the tree as of their own start time (the "queued-cargo-check-reports-stale" trap).

## 4. Host capacity

- **Disk**: `/` is 926Gi total, 279Gi avail (was 286Gi five minutes earlier — ~7GB consumed by the builds above in that window). Not urgent.
- **`target/` sizes**: root `target/`=19G; `target-demonstrator-dev`=**18G**; `target-gen3d`=5.6G; `target-sourcing-e2e`=2.0G; `target-demonstrator`=1.7G; `target-p3d-e2e`=1.4G; `target-block`=1.1G; `target-p3d-agentE`=308M; `target-block-io`=103M; `target-block-3d`=20M. Plus several ticket-local `CARGO_TARGET_DIR`s under this ticket's `🗑️generated/*` (space-public-boundary-sol-target, fable-hub-native-qualification/cargo-target, home-directory-projection-target, flow-add-widget-retained-sol-target, root-directory-ordering/target — plus one now-deleted one, open-plan-issuer-target). Expected under the isolated-target policy, but it is a lot of duplicated dependency compilation.
- **Load**: `uptime` load averages **106.17 / 136.63 / 147.82** — very high, consistent with the ~14-15 concurrent `rustc` + ~10 `cargo` orchestrators + everything else on the box.
- **Memory**: `vm_stat` shows only **3834 free pages** (16KB pages ⇒ ~63MB free) out of a large working set; 368k active + 341k inactive + 208k wired pages; compressor holds 5.9M pages. Real pressure, though the swapin/swapout counters (437M/469M) are cumulative since the 1-day-18h boot, not a spike I can attribute to this exact window.

## 5. What the three concurrent peer tickets/sessions are doing (five lines each)

**S-END-TO-END** (`…/🎆️26/🌙️09/☀️05/S-END-TO-END`, opened same day, exploring only so far):
1. Read-only census of the `s` plugin catalog (59 registry rows: 33 plugins + 26 extensions) against the dev shell's built-module cache.
2. Finding: 57/59 rows have real `.core.wasm` + component JS; only `draw` and `layout` have zero build output at all.
3. `plugin-registry:check` crashed ~20min in on `ENOENT …target-block/debug/deps/rustcAWEOX6` — attributed to a concurrent agent's `target-block/` CARGO_TARGET_DIR racing the scan, not a catalog defect.
4. Documents exactly which crates ship without an owner descriptor pair (`block`, `playbook`, `stdio`, `trinity`, 15 extensions) — a registry-check warning source, not a compile error.
5. Not building any of our 8 target crates itself; it is auditing the plugin catalog/registry layer that sits above them.

**BLOCK-PLUGIN-END-TO-END** (`…/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END`, opened 2026-09-05 by a Fable coordinator session):
1. Driving `semio-s-plugin-block` (block2d/3d/5d editors) to green: native+wasm check, lib test, descriptor regen, all three playgrounds booting.
2. Exploration fleet found block2d and block3d have **no** `bounded_first_step_tool_proofs!`/factory wiring at all (dispatch dead), block5d's dispatch is healthy.
3. TS/io exploration: block's IO layer is mostly stub (`export {}`), only JSON import is real; implementer W3 launched to fix io registration.
4. Its own native baseline check (`block-check-native-baseline`, exit 101) is called out **in its own status.md** as stale/non-live: `semio-s-plugin-stdio`'s `🪆️subsets/✳️base/🧬️schema` mount was renamed to `🧱️base` by its owner *after* that check started — the same stdio-rename-race pattern seen above.
5. Not one of our 8 target crates directly, but its `target-block` CARGO_TARGET_DIR is the one that broke S-END-TO-END's registry-check scan (cross-ticket collision, both read-only for each other).

**DEMONSTRATOR-END-TO-END-ALL-APPS** (`…/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`, oldest of the three, status.md last written 02:15):
1. Building all demonstrator-dependency plugins across `wasm-dev` (7 crates, `target-demonstrator-dev`) and `wasm-release` (`stdio` only, `target-demonstrator`, because only stdio is known to breach a size ceiling).
2. `wasm-release` for stdio is a deliberate ~3h single-threaded LTO tail — this is the `86722` process in §1, currently at 3h00m elapsed, not stuck, just slow by design.
3. Ran a preemptive `#[path]` audit across all 33 plugin crates (~01:55) after catching two more mid-build renames; result at that moment: stdio/puzzle/gis/process/flow/demonstrator/cad/sourcing all clean (0 broken mounts).
4. Still broken at that audit and explicitly **not** a demonstrator dependency: `semio-s-plugin-norm` (33/2413, a `din16798` artifact tree) and `semio-framework/…/vcs` (1/158) — left to their owners.
5. Diagnosed and dismissed an apparent `sourcing`/`process` failure as a stale mid-edit read (compared descriptor owners against real `os.listdir` bytes, then confirmed a later re-run after the fix landed reported 0 failures) — a live example of the "queued check reports stale" trap this whole report keeps running into.

## Advice to implementers

**Green, trust it (for now):** `semio-framework-os-kernel` — GREEN as of 04:04, produced 1 minute before this report was finalized, by a live Claude Code session that is still actively working in the same lane. This is the one crate here you can currently build on top of with confidence.

**Unsafe to depend on right now:**
- `semio-s-plugin-stdio` — flapping; multiple different generated-artifact subsets have broken and been "fixed" within the last 2.5 hours by unrelated concurrent renames. Any dependent build's result is only as good as the instant it started.
- `semio-hub` — RED at its last completed receipt (01:48, missing brep/io artifact via stdio); two newer attempts are either blocked on a lock or still mid-dependency-compile with no verdict yet. Do not treat "no red receipt in the last hour" as green — nothing has finished.
- `semio-s-plugin-flow` — RED, 15 errors, freshest failure (03:10), rooted in stdio's pdf/1.4 mutation subset plus one missing example asset.
- `semio-s-plugin-norm` — RED per the demonstrator ticket's own audit (33 broken mounts), no evidence of a fix since ~01:55.
- `semio-s-plugin-gis` — status unknown; it's the crate currently *holding* the shared lock in `space-public-boundary-sol-target` and has never been seen to finish in any receipt I could find. Wait for its current run (started ~41 min ago) to conclude before trusting it either way.

**No current evidence either way (don't assume green OR red):**
- `semio-framework-os-kernel-db` — never tested standalone in any receipt found; only seen as a transitive dependency.
- `semio-framework-plugin-host` — its dedicated build cache is 32+ hours stale with no top-level verdict file; nothing is building it right now.

**Process hygiene flag (not a build-health item, but worth surfacing):** pid 94794/94953 (a Codex-driven `cargo test --bin os-hub …` for one specific test) has been running **27+ hours**, and its `CARGO_TARGET_DIR` (`open-plan-issuer-target`, inside this ticket's own `🗑️generated`) no longer exists on disk — the folder was swept while the process kept running. The test binary child (94953) is still in run state consuming ~1% CPU. This is very likely an orphaned/hung process rather than a real in-flight build; it belongs to a Codex/ChatGPT session (not a Claude session), so it is not something this exploration can or should kill, but the coordinator may want to flag it to that session or to the dev.

**Lock contention flag:** two Codex-driven lanes (`gis-map-create-region-group-exact` and `gis-map-frozen-binding-exact`) are sharing the same `CARGO_TARGET_DIR` (`space-public-boundary-sol-target`) and one has been blocked on the other's build lock for 8+ minutes. Separately, two `cargo check --workspace` invocations (pids 12147, 16308) from the *same* parent session are both sitting on the shared root `target/debug` lock simultaneously — likely redundant/duplicate work from one session rather than two different lanes.
