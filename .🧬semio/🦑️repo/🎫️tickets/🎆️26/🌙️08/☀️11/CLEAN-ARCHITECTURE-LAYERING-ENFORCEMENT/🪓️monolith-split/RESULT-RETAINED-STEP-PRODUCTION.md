# 🏭 Result RETAINED-STEP-PRODUCTION (slice `rsp`, wave 7, 2026-10-17)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `ENG` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `WGPU` = `ENG/🎯️targets/🧊️wgpu`. Logs: `F/🗑️generated/rsp-*` (`rsp-scan0.txt` before, `rsp-scan1.txt` after, `rsp-run1.log` killed by the 590 s cap, `rsp-run2.log` finished oracle run, label `rsp1`).

## 0. Verdict

**DONE.** All 26 production call sites listed in `RESULT-RETAINED-STEP-TEST-DRIFT.md` section 6 are on the current retained-step API. `rsd.py` over the whole tree: 1,509 sites, **0 wrong arity** (before: 26). No shim, no compat constructor, no zero-fabricated grant except one honest fuel-only grant (section 3, decision D3). The real `bun repo cargo` check was not possible (`📓️FLEET.md` has no `COMPILE ADMITTED` line); exact command in section 6.

## 1. Canonical pattern (cited)

Current API (`🧰️framework/🔨️modules/🧵️job/🦀️.rs`): `StepBudget::new(fuel, deadline_us, retained)`, `StepBudget::from_duration(fuel, start_us, duration_us, retained)`, `StepContext::new(operation, generation, budget, cancel, now_us, &mut preview_sequence, &mut RetainedCloneProgress)`.

The already-migrated production code takes the grant from the owner of the turn and the recipient as a fresh local `RetainedCloneProgress::default()` declared before the context (so it drops after it):
* `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🦀️.rs:653,766` and `🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1514,1521` (grant `ui_contract::UI_WORKER_RETIREMENT_POLICY`, local `actual_retained_progress`).
* `ENG/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:244,461,494,500` (grant `input.grant` issued by the host, local `recipient`).
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🦀️.rs:1269` and `🔔️wake/🎟️root/🦀️.rs:31` (grant in scope, local `receipt`).

## 2. Sites fixed (26 rows, 9 files)

| Crate / file (under `WGPU` unless noted) | Sites | Grant source | Recipient |
| --- | --- | --- | --- |
| `🏠️os-host/🦀️.rs` world close (278, 282) | 2 | the `grant` already in scope (line 277), passed to `from_duration` as well | local `recipient` |
| `🧊️renderer/📮️mailbox-impl/🦀️.rs` `close_engine_surface_step` (178, 181) | 2 | **new 4th parameter `grant`**, threaded from the host | local `retained_progress` |
| `🏠️os-host/🦀️.rs` callers of it (2 CPU phases) | 2 call sites updated | see D1 | n/a |
| `🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs` (357-360, 389-392, 417-420, 482-485, 625-628) | 5 contexts, 15 rows | `JOB_REPLAY_STEP_GRANT` (D2), also in the `StepBudget::new(0, 0, ..)` no-clock fallback | local `retained_progress` per context |
| `🧊️renderer/📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs` (690, 693) | 1 context, 3 rows | `JOB_REPLAY_STEP_GRANT` | local `retained_progress` |
| `ENG/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🎲️board2d/🦀️.rs` (491, 494) | 1 context, 2 rows | `BOARD_POINTER_SETTLE_GRANT` (D3) | local `retained_progress` |
| `ENG/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🖼️chrome-frame-and-main-window/🦀️.rs` (25, 29) | 1 context, 2 rows | `terrain_grant` hoisted above the budget (it was defined 9 lines later) | local `retained_progress` |

Wiring edits for the new names: `🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs` (new const next to `TURN_BUDGET`, re-exported by `kernel-protocol` through its existing `pub use kernel_core::*`), `🏊️kernel-pool/🦀️.rs` (crate-root `use` list gains `JOB_REPLAY_STEP_GRANT`), `🗃️kernel-retained/🦀️.rs` (crate-root `use` gains it; `kernel-retained-state` reaches it through `use super::*`).

## 3. Design decisions (the minimal ports)

* **D1, engine surface close grant (no owner had one).** The CPU close demand of an `EngineCanvasPacket` scales with its scene, so `RuntimeMailbox::close_engine_surface_step(token, operation, sequence, grant)` now takes the grant and the host funds it. `os-host` has two owners of that ladder:
  * `ComponentSurfaceCloseOwner::close_step(runtime, presenter, retained)` and `OsHost::advance_component_surface_close` pass **`self.retained`**, the host root grant `OsHost::new` is constructed with (`FRAME_BOOTSTRAP_ROOT_GRANT`; the field was stored but never read).
  * `PairedEngineSurfaceClose::close_step(runtime, presenter, retained)` is driven from `OsHostRetirementState::close_step(cx)`, which passes **`cx.retained_grant()`**, the grant of the retirement turn that funds it.
  No new constant was invented for the host. The grant is a ceiling per step, not a stock (every close step is one item and pages its copy demand by the grant's copy limit), so the 64 KiB copy / 16 MiB release root grant is sufficient for page-wise retirement.
* **D2, job replay log grant (`JOB_REPLAY_STEP_GRANT`).** `JobReplayLog::begin_capture`/`acknowledge_publication`/`maintenance_step`/`close_step` page payload bytes through `cx.retained_grant()` (`🎭️actor/💼️job/📨️publication/🦀️.rs:1607-1624`: `grant.maximum_copy_bytes >= page.len()`, `maximum_capacity_bytes >= original_extent`), so a zero grant would stall. One shared constant in `kernel-core` beside `TURN_BUDGET`, derived from the owners' own limits and equal to the declared actor bridge grant (`🎭️actor/🧪️tests/📄️checkpoint/🔣️.json` `originalGrant`: 4096 items, 8 MiB copy/capacity/release, depth 64): `maximum_items = semio_framework_actor::JOB_REPLAY_PROCESS_PAGE_CAPACITY` (4096), copy/capacity/release `= 2 * semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES` (state plus output of one operation), `maximum_depth = 64`. `kernel-protocol/🦀️.rs` already imported `RetainedCloneGrant` (unused until now), so no new dependency.
* **D3, board pointer settle (`BOARD_POINTER_SETTLE_GRANT`).** `settle_board_pointer_authority_into` is called from seven input handlers plus tests with no turn context, and the board pointer authority (`step_pointer_commit`, `step_event_authority`, `step_pointer_cancel` in `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host`) never reads or consumes the retained grant (`/usr/bin/grep` for `retained_grant|consume_retained|RetainedClone` over `♾️infinite/🎲️board` finds only schema code, none in the host/engine steppers). Threading a grant through the public input API would invent an owner, so the grant is a documented const beside `BOARD_POINTER_SETTLE_STEPS`: one item per step it may spend, depth 1, **byte limits zero**; an authority that ever consumed bytes is refused (fail closed) instead of overspending. The frame phase that drives the same authority (`drive_board_authority_step` from `🧾️frame-build/🧮️frame-transaction`) keeps the frame's own context. This is the one place a grant is not a copy of a larger owner grant; if the owner prefers the frame root grant, `engine-board` would need a dependency on `frame-job` (it has none today).

## 4. Static evidence

* `python3 rsd-tools/rsd.py /Users/ueli/Documents/semio 🗑️generated/rsp-scan1.txt` -> `1509 0` (before, `rsp-scan0.txt`: `1509 26`, exactly the 26 listed rows).
* `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout --quiet` on the 9 edited files (`os-host`, kernel-pool main and crate root, `kernel-retained-state`, `kernel-core`, `kernel-retained` root, `mailbox-impl`, `board2d`, `chrome-frame-and-main-window`): exit 0 each, 0 parse errors.
* Renderer typeck oracle (family `rii-tools/rii.json`, 104 crates, `ulimit -n 65536`, `POS_PREFIX=rsd`, `pos.py run ../rii-tools/rii.json --cfgs lib --label rsp1 --no-base`, log `rsp-run2.log`): `lib` section row diff against the `wa2` run (`python3 rsd-tools/rsd_rowdiff.py rsd-wa2-run.log rsp-run2.log`): **base 366 rows, post 366 rows, gone 0, new 0**; `it0` error total 1,779 in both. No E0425/E0432/E0599 for `JOB_REPLAY_STEP_GRANT`, the new `grant` parameter or the hoisted `terrain_grant`. Note the `test` cfg was not re-run (the edits are library code; the cap on one command is 10 minutes, a lib run takes about 11 minutes under machine load 57, so `rsp-run1.log` hit the cap and the finished run was `rsp-run2.log` under `nohup` while I polled in the foreground turn).
* Oracle limit, unchanged from the drift slice: the `job` and `value` crates are stubs in this family (the stubbed `StepBudget`/`StepContext` constructors take 0 arguments, `E0061/stub=2484` identical before and after), so no oracle can display the arity fix. The arity scanner is the proof; the real compile is the final proof.
* No `[DEBUG]` lines added; no comments inside definitions; every new item carries a docstring with a unique emoji.

## 5. Not done / caveats

* `JOB_REPLAY_STEP_GRANT` and `BOARD_POINTER_SETTLE_GRANT` are policy values chosen from the owners' limits (D2) and from the fuel-only nature of the authority (D3); the real compile cannot confirm that the 8 MiB state-plus-output size covers every product replay publication at run time. A run with a larger publication would stall the replay log close, exactly as the drift slice predicted for a too-small grant; the limit is therefore tied to `JOB_PAYLOAD_OPERATION_BYTES` (the admission ceiling of one operation) so the two cannot drift apart.
* The world close grant in `os-host` (literal, line 277) and the terrain grant in `chrome-frame-and-main-window` are the same literal in two crates; left as found (outside this slice, no shared owner crate).

## 6. Commands

Run:
* `python3 rsd-tools/rsd.py /Users/ueli/Documents/semio 🗑️generated/rsp-scan0.txt` -> `1509 26`; after the edits `.../rsp-scan1.txt` -> `1509 0`.
* `rustfmt ...` on 9 files -> exit 0 each.
* `cd pos-tools && ulimit -n 65536 && POS_PREFIX=rsd python3 pos.py run ../rii-tools/rii.json --cfgs lib --label rsp1 --no-base` (first run killed by the 590 s alarm, second finished) -> row diff vs `wa2`: lib 366/366, gone 0, new 0.
* Compile admission: `/usr/bin/grep -n "COMPILE ADMITTED" 📓️FLEET.md` -> no match. No `bun repo cargo` call made.

Real check for the compile queue (one call under the mutex; renderer crates are members of the framework workspace):
`cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-renderer-wgpu-host-winit -p semio-framework-os-renderer-wgpu-runtime-mailbox -p semio-framework-os-renderer-wgpu-kernel-protocol -p semio-framework-os-renderer-wgpu-kernel-pool -p semio-framework-os-renderer-wgpu-kernel-retained -p semio-framework-os-renderer-wgpu-engine-board -p semio-framework-os-renderer-wgpu-shell-surfaces --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`
(package names read from the `Cargo.toml` files of `🫧️shell-surfaces` and `📨️runtime-mailbox`; the `chrome-frame-and-main-window` staging file is mounted by the shell-surfaces crate family, and `mailbox-impl` by the runtime-mailbox crate).
