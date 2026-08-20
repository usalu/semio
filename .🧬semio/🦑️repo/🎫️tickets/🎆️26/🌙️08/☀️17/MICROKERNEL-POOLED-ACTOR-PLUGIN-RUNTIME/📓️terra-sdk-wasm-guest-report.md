# 📓️ terra-sdk-wasm-guest — report

## Packet
`sdk-wasm-guest` executor packet on `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`. Scope:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**` except `🖥️host/**` (packet `host-repair`,
live). `🌐host/**` (guest-side host-capability API — a different directory from `🖥️host`) is mine.

## The design question — answered with evidence, no schema edit needed

The brief asked: are the 90 wasm-guest compile errors (a) a bindgen-mode mismatch (`generate!` not
using `async: true`) or (b) genuine local breakage?

**Neither, precisely — and no schema/world edit was required.** Evidence:

1. Read `🧬️schema/📜️component.wit`: `world actor` (line 1029) `import`s **only** `pure`
   (`log`/`now-ms`/`trace-span`, all plain `func`, no `async`) and `export`s `reactor`/`jobs`/
   `checkpoint`/`describe` — **every one of those exported funcs is a plain WIT `func`, not
   `async func`**. `host-async` (the interface with 24 `async func`s) is imported only by the
   separate, still-unused `world actor-async` (line 1044), not `world actor`. So `world actor` is
   a **fully synchronous** WIT world today — B1/world-collapse (which will make these funcs
   `async func` for real) has not landed yet.
2. Read `wit-bindgen-core-0.57.1`'s `AsyncFilterSet::is_async` (`~/.cargo/registry/.../async_.rs`):
   the macro's `async: true` option **can** force every export into the async ABI regardless of
   what the WIT text says — so it was technically available. But using it would silently change
   the guest's exported component ABI (sync `canon lift` → async) without any matching change on
   the host side or in the WIT text itself — exactly the SDK/host bindgen-surface decision the plan
   (`📋️master-u.md` §B1) explicitly assigns to `world-collapse`, a sol-owned ATOMIC packet ("it
   changes the SDK/host bindgen surface, so it must sit inside the same quiet window"). Not used.
3. Traced every one of the 89 errors to their functions and grepped each body for `.await`: **zero
   real suspension points** in any of them. They are pure WIT↔kernel translation / bookkeeping
   helpers that the site-wide universal-async codemod (this ticket's standing R9 problem) made
   `async fn` with no `.await` ever inserted, now called from the **sync** `Guest` trait
   `wit_bindgen::generate!` produces for the (still-sync) `world actor`. This is **R9**: pure
   computation whose consumer (the WIT-generated `Guest` trait, tool-fixed by the still-sync
   schema) cannot be async — propagated backward through the call graph, exactly per this ticket's
   `number-green`/`3d`/`sdk-final` precedents.
4. Where a callee genuinely could not be de-asyncified in scope (because it transitively depends on
   a real `async fn` in an **out-of-scope** crate — `🏪️store/**`, `📡️spr/**` — or because its
   signature is shared with the `component-guest-async` feature combo, where it legitimately does
   await something real: `⚛️reactor/💼️jobs`'s `spawn_job`/`step_job` when `JobCtx::host()` is live),
   the fix is the already-sanctioned **E5 bridge** `semio_framework::io::resolve_ready` — safe here
   specifically because `world actor` imports no `host-async`, so nothing in this call graph has a
   genuine host suspension point to violate `resolve_ready`'s "must be Ready on first poll"
   contract. Every one of these ~24 sites is tagged `// 🚫️async: E5 executor bridge` inline with a
   reason.

No `.await` was inserted with a name-keyed tool (R10) — every removal/addition was hand-verified
against the actual function body, span by span.

## What changed (all inside `path_scope`, no schema/world edits)

- **`🌐host/🦀️component.rs`**: `outcome_to_result` → sync (R9; `dsl::{decode,encode}_fault_bytes`
  are already plain `fn`). Fixed 25 doubled `.request(...).await.await` call sites down to one
  `.await` (the outer `.await` was resolving `RequestRegistry::request`'s now-removed async-ness;
  the inner one is the genuine suspension on the returned `RequestFuture`, unchanged). Fixed
  `registry.emit(effect).await` → `registry.emit(effect)`.
- **`⚛️reactor/📮️requests/🦀️component.rs`** (`RequestRegistry`): entire public API (`new`,
  `for_instance`, `request`, `resolve`, `append_chunk`, `instance_of`, `emit`, `drain`,
  `pending_ids`, `cancel_instance`) and `Inner::complete` → sync (R9; pure `Rc<RefCell<Inner>>`
  bookkeeping, zero I/O). The only genuine async primitive in this crate's request model is
  `RequestFuture` itself (unchanged) — the registry that allocates/resolves it never needed to be
  async.
- **`⚛️reactor/🧵️executor/🦀️component.rs`** (`LocalExecutor`): entire public API (`new`, `spawn`,
  `spawn_with_id`, `cancel`, `wake`, `run_until_idle`, `has_ready`, `has_pending`, `waker_for`) →
  sync (R9; self-contained `Rc<RefCell<Inner>>` cooperative scheduler using a hand-rolled raw
  waker — its own doc already said "`run_until_idle` handles Pending without ever yielding its own
  future"). Test module (`#[cfg(test)]`, its own separate residue class per this ticket's
  `sdk-final`/`dispatch-group-split` findings) left untouched — out of `--lib` scope.
- **`⚛️reactor/🩹️patches/🦀️component.rs`** (`PatchTracker`): `new`/`diff`/`mark_rejected`/`mark_ack`
  → sync (R9; pure `HashMap` bookkeeping, zero I/O, zero `.await` anywhere in the file).
- **`⚛️reactor/💼️jobs/🦀️component.rs`**: unwrapped the now-stale `resolve_ready(...)` wraps around
  `JOBS_EXECUTOR`'s `spawn`/`wake`/`run_until_idle` calls (the executor itself is sync now, so the
  bridge is redundant there). `start_job`/`restore_job`/`spawn_job`/`step_job`/`cancel_job`/
  `checkpoint_jobs` themselves were **not** touched — confirmed via that module's own doc
  ("`JobCtx::host()`... NEVER ungate this for `world actor`") that they must stay `async fn` to
  keep serving the `component-guest-async` combo; the WIT-boundary Guest impl bridges them instead.
- **`⚛️reactor/🦀️component.rs`**: R9-reverted `poll` and all 18 of its pure WIT↔kernel translation
  helpers (`wit_event_to_kernel`, `kernel_effect_to_wit` + its nested `pack`, `kernel_turn_result_
  to_wit`, `kernel_ui_patch_to_wit`, `kernel_patch_op_to_wit`, `route_app_frame`,
  `drain_task_resumes`, `decode_wire_effect`/`decode_wire_app_event`/`decode_wire_quotas`, etc.) and
  `cancel_instance_tasks`. Applied E5 `resolve_ready` bridges at every point these now-sync helpers
  call a genuinely-still-async out-of-scope function (`store::pack_rt::{encode,decode}_wire_value`,
  `protocol::{decode,encode}_app_frame`, `plugin_runtime::{plugin_exchange,plugin_render,
  plugin_resume_task,plugin_create_app_with_id,set_instance_actor}`). Two of those (lines 379/382,
  `plugin_create_app_with_id`/`set_instance_actor`) were previously **bare dropped futures** (R13) —
  `let _ = ...`/un-awaited statements that silently never ran; now genuinely resolved.
- **`🦀️component.rs`** (crate root): `ensure_plugin_initialized` → sync (R9; body is entirely
  `std::sync::Once::call_once` wrapping a sync `fn()` installer call, zero suspension) — this was
  the packet brief's named dropped-future class; all 6 export-entry-point call sites (`poll`,
  `start_job`, `step_job`, `cancel_job`, `checkpoint`, `restore`, `describe`) now call it as a plain
  synchronous guard, genuinely running on every entry. Removed the 5 now-orphaned `.await`s on its
  other internal callers (`plugin_manifest`, `plugin_wire_list_artifact_inference_services`,
  `plugin_wire_artifact_infer`, `plugin_wire_list_artifact_mutations`,
  `plugin_wire_artifact_mutation_plan`). Rewrote the `JobsGuest` impl block: fixed the stale
  `crate::reactor::jobs::JobOutcome` reference (E0433 — that enum doesn't exist; `step_job` returns
  `jobs::JobStep` directly) and the `step_job(job)` call missing its `budget` argument (E0061) with
  the field-for-field WIT↔jobs `JobBudget`/`JobStep` conversion that module's own doc comment says
  belongs here. Wrapped the `jobs::{start_job,step_job,cancel_job}`, `checkpoint_now`, `restore_now`,
  `describe_plugin` calls in `resolve_ready` (E5 — these stay genuinely async for out-of-scope/
  shared-feature reasons, documented per call site).

## Acceptance (run in the foreground, this turn)

1. `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest`
   → **EXIT 0** (was EXIT 101 / 89 errors). 12 warnings, all pre-existing lint classes (redundant
   `.clone()`, unused import/doc-comment/qualification) — zero dropped-future warnings.
2. `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features
   component-extension-guest` → **EXIT 0**. 13 warnings, same classes, zero dropped-future.
3. **R12 forced-rebuild dropped-future census** (touched the crate root file, re-ran both feature
   builds from cold): **0** occurrences of `unused implementer of \`std::future::Future\`` in
   either build. The six `ensure_plugin_initialized()` call sites specifically: all six now call a
   plain synchronous `fn` — confirmed by reading the current file, not just the warning count.
4. Native regression guards, all green:
   - `cargo check -p semio-framework-plugin --lib` → EXIT 0.
   - `cargo check -p semio-framework-plugin --lib --all-features` → EXIT 0.
   - `cargo check -p semio-framework-os-kernel --lib` → EXIT 0 (57 warnings, unchanged
     `async_fn_in_trait` class, R7-sanctioned).
   - `cargo test -p semio-framework-os-kernel --lib` → **779 passed / 0 failed / 0 ignored**.

All four ran with `CARGO_TARGET_DIR` pointed at the session scratchpad, `-p <crate>`, foreground,
single turn, per this ticket's standing rules.

## Not touched / left as found

- `🖥️host/**` — `host-repair`'s live territory, per brief.
- `🏪️store/**`, `🗣️dsl/**`, `💡️inference/**`, `✏️s/🔌️plugins/🗄️stdio/**` — out of `path_scope`,
  confirmed genuinely still-async and bridged via `resolve_ready` rather than edited.
- `📡️spr/**` (`protocol::{decode,encode}_app_frame`) — not named in the brief's exclusion list but
  clearly outside `🔌️plugin/**`; same treatment (bridged, not edited).
- `#[cfg(test)]` bodies throughout every file touched (e.g. `⚛️reactor/🧵️executor`'s own test module,
  `⚛️reactor/📸️checkpoint`'s tests, `💼️jobs`'s tests, the native fixture at `🦀️component.rs:19653/
  19673/19682/19696`) — these are the **separate, already-flagged** `#[cfg(test)]` residue class
  (`sdk-final`/`dispatch-group-split` cross-packet findings, ~1,381 errors, its own dedicated
  packet) and are out of `--lib` (non-test) scope for this packet regardless.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` shows as modified in
  `git status` but **is not part of this packet's diff** — one line (`register_job_kind(kind,
  run).await`) changed there since this session started, not by me; `git log --date=iso` shows the
  file's last real commit at 2026-08-20 00:52 (before this session), and the working-tree delta is
  a live peer edit consistent with this ticket's standing "concurrent devs, auto-commit" note. Left
  untouched; it did not block any of the four acceptance builds.

## No `lease-request` needed

The fix stayed entirely inside `🔌️plugin/**` (minus `🖥️host/**`). No WIT schema or world-definition
edit was required — confirmed by evidence in the design-question section above, not assumed.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📮️requests/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs`
