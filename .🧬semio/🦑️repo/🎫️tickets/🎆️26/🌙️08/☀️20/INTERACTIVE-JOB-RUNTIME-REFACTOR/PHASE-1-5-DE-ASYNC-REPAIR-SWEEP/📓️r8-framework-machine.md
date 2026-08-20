# R8 — De-Async Repair: `semio-framework-machine`

Packet R8 of Phase 1.5. Ownership boundary: `semio-framework-machine` only
(`🧰️framework/🔨️modules/🔄️machine/🦀️component.rs` + its package glue). This is the
crate R3 unmasked and flagged as "extensively broken by the identical bug class" once
the proc-macro entry-point fix let `rustc` reach it for the first time.

## 1. Error trajectory

Measured with `cargo check -p semio-framework-machine --all-targets` only (per the
measurement caveat — workspace totals are non-deterministic here).

| Target | Before | After |
|---|---|---|
| lib | 90 | **0** |
| lib test | 318 | **3** (all one cross-boundary class, see §3) |

Error-code profile before (matches the ticket's predicted signature exactly):
E0308×126, E0599×83, E0609×68, E0277×29, E0600×7, E0369×5 — all resolved by the
mechanical de-async pass. The 3 remaining errors are all `E0053` (trait-impl signature
mismatch), a different class, caused by a deliberate trait-shape decision — see §3.

## 2. What changed

**File:** `🧰️framework/🔨️modules/🔄️machine/🦀️component.rs` (2601 lines, single file,
no `#[path]` submodules — safe to edit directly).

Every `async fn` in the file had a genuinely-synchronous body: dense-table lookups,
`Vec`/bitset mutation, arithmetic, `match`. Grep for `.await` before any edit found
exactly **3** call sites in the whole 2601-line file (`Migration::source_fingerprint`/
`migrate` inside `persist::restore`, and `persist::restore` inside `step::step`) — i.e.
161 of 162 `async fn` had zero suspension anywhere downstream, and the 3rd was awaiting
another non-suspending `async fn` in the same file, not a real timer/I-O/channel.

Decision: **full de-async, zero `.await` added.** Unlike R3's `semio-s-plugin-draw-fsm`
sibling (which kept its `Migration`-shaped trait async because that crate ships a real
`#[async_test]` harness and genuinely-suspending custom migrations were plausible), this
crate's own `Cargo.toml` has an **empty `[dev-dependencies]`** block — no async-test
harness exists here at all, and every `#[test] async fn` in this file was itself
uncompilable under plain `#[test]` regardless of the kernel bug. Keeping any part of the
call graph async (even just `Migration`/`restore`/`step`) would have forced every test
and every kernel entry point that touches `M::definition()` or an event's `event_id()`
back into `async fn`, undoing the whole point of the refactor for a crate whose own
`Host` trait doc-comment already stated the design intent explicitly: *"No `async fn` —
hosts own their own tasks/timers and report completion back as ordinary events."*
Chose the reading that matches that stated intent and the ticket's stated direction.

- **160** `async fn` → `fn` (mechanical, span-keyed: only lines where `async fn` opens a
  real declaration were touched — the 2 remaining `async fn` matches in the file are
  both inside doc-comments quoting the phrase, correctly left alone).
- **0** `.await` added.
- **3** `.await` removed: the `Migration` lookup loop in `persist::restore` (also
  simplified from a hand-rolled loop back to `migrations.iter().find(...)` now that
  nothing needs awaiting inside it — the comment explaining the R10-residue hand-loop
  was stale and removed with it), and the `persist::restore(...)` call inside
  `step::step`.
- `📦️glue.rs`: removed the stale `#![allow(async_fn_in_trait)]` and its explanatory
  comment — no trait in the crate declares an `async fn` any more, so the allow (and the
  comment claiming `Host`/`Inspector`/`Migration`/`Configuration`/`StatechartEvent`/
  `Machine` are an "async-fn-in-trait family") was dead and actively misleading.
- Reformatted both touched files with `rustfmt` (explicit paths only).

## 3. Public trait methods whose asyncness changed — macro-crate follow-up required

**Every** public trait in the crate lost `async` from every method:

| Trait | Methods | Macro-crate impact |
|---|---|---|
| `Host<M>` | `execute_effect`, `schedule`, `cancel_timer`, `start_task`, `cancel_task`, `now_ms` | none — no derive touches `Host` |
| `Inspector<M>` | `observe` | none |
| `Configuration` | `set`, `clear`, `contains`, `iter_ones`, `clear_all`, `is_empty` | none |
| `Migration` | `source_fingerprint`, `migrate` | none — user-implemented directly, never derive-generated |
| **`StatechartEvent`** | `event_id`, `event_name` | **yes — see below** |
| **`Machine`** | `definition` | **yes — see below** |

`StatechartEvent` and `Machine::definition` are the exact 5 generated methods R3's
report flagged as deliberately left `async` in `semio-framework-machine-derive` "because
they must match the async trait declarations in `semio-framework-machine`." Those
declarations are no longer async, so the derive crate is now the one out of sync. It is
outside my boundary (report, not edit); the fix it needs, precisely:

**File:** `🧰️framework/🔨️modules/🔄️machine/✨️derive/🦀️component.rs`

- In `emit()`: drop `async` from the generated `fn event_id`, `fn event_name`
  (`impl StatechartEvent`) and `fn definition` (`impl Machine`).
- In `expand_statechart_event()`: drop `async` from the generated `fn event_id`,
  `fn event_name`.
- None of these 5 generated bodies contain a nested `.await` (R3 already confirmed
  this), so dropping `async` from the signature is the entire fix — mechanically
  identical to what R3 already did for the crate's other 47 internal functions.

**Additionally found while checking the wasm32 targets (§5):** the `export_wasm_machine!`
macro's generated inherent-method block (`new`/`send_json`/`tick`/`snapshot_json`/
`restore_json`/`manifest_json`/`on_effect`) is *also* out of sync, and has a second,
independent bug:

- It still generates `async fn` bodies that call `.await` on `machine::init`/
  `macrostep`/`route_command`/`timer_elapsed`/`persist`/`restore` — all now sync in this
  crate, so every one of those `.await`s needs to go, same as everywhere else.
- It calls `restore::<M>(...)` supplying only **one** of `restore`'s two generic
  parameters (`M`, `Mg: Migration`) — `E0107`, pre-existing, independent of the async
  bug class (confirmed: `restore`'s two-generic signature predates this packet's edits
  entirely). The macro needs to either thread a caller-supplied `Mg` through
  `export_wasm_machine!`'s own macro arguments, or default to `NoMigrations` the way the
  hand-written `checkout_integration`/native tests already do.
- The generated code's use of `wasm_bindgen_futures` to bridge the (formerly async, now
  sync) wasm-bindgen methods back to JS promises is no longer needed once the bodies it
  wraps are sync — `wasm_bindgen_futures` is not even a dependency of this crate
  (`Cargo.toml`'s `wasm32` deps are `wasm-bindgen`/`js-sys` only), so today it's also a
  hard `E0433` (unresolved crate), on top of being architecturally unnecessary once the
  wrapped calls stop being async.

This block was never exercised or compiler-verified by anyone before this packet — R3
explicitly said so for the derive crate ("nothing in my three owned crates exercises
`export_wasm_machine!`... left untouched rather than guess") and no consumer crate in
the repo invokes `export_wasm_machine!` at all outside this crate's own `wasm_smoke`
compile-only fixture (checked below).

## 4. Deriving crates — blast radius

Grepped the whole repo for `Cargo.toml` dependencies on `semio-framework-machine-derive`
and for `statechart!`/`derive(StatechartEvent)`/`derive(StatechartSchema)`/
`export_wasm_machine!` call sites:

- **Only `semio-framework-machine` itself** depends on `semio-framework-machine-derive`.
  No other crate in the repository does.
- `semio-s-plugin-draw-fsm` (draw plugin) uses `statechart!`-shaped DSL too, but through
  its own, entirely separate proc-macro crate `semio-s-plugin-draw-fsm-macros` (R3
  confirmed: "a sibling, not a reuse" of this crate's kernel) — unaffected by anything
  in this packet.
- Within this crate, exactly **two** call sites actually go through the derive macro:
  `checkout_integration::checkout` (native + wasm, `#[cfg(test)]`) and
  `wasm_smoke::toggle` (`#[cfg(target_arch = "wasm32")]` only, not test-gated). Every
  other `Machine`/`StatechartEvent` impl in the file (`ToggleMachine`, `PlayerMachine`,
  `RecorderMachine`, `DummyMachine`, `UnitToggleMachine`) is **hand-written**, not
  macro-derived, and was updated directly in this packet — no follow-up needed for those.

So the blast radius of the trait-shape decision is fully contained: **zero external
crates**, and exactly the 2 in-crate macro fixtures reported in §3 pending the derive
crate follow-up.

## 5. Verification actually run

- `cargo check -p semio-framework-machine --all-targets` (native, default features) —
  **0 lib errors, 3 lib-test errors** (the known E0053 trio, §3). Re-ran after
  `rustfmt` to confirm formatting didn't shift anything — same 3.
- `cargo check -p semio-framework-machine --target wasm32-unknown-unknown --all-targets`
  — **5 lib errors** (3× the same E0053 trio via `wasm_smoke::toggle`, plus the
  independent E0107 and E0433 in §3), **8 lib-test errors** (5 lib + the native 3 from
  `checkout_integration`, which is also compiled for wasm32 test builds).
- `cargo check -p semio-framework-machine --target wasm32-wasip2 --all-targets` — same
  5 lib / 8 lib-test shape (`wasm32-wasip2` shares `target_arch = "wasm32"`, so the same
  gated code compiles). Confirms the wasm findings are target-arch-general, not
  `wasm32-unknown-unknown`-specific.
- **Temporary, reverted validation only:** to confirm the rest of the suite is actually
  correct and not just "fewer type errors," I added `#[cfg(any())]` above
  `mod checkout_integration` (the only native module blocked by §3), ran the full test
  suite, then removed the attribute and re-ran `cargo check --all-targets` to confirm
  the file returned to the exact same 3-error state before leaving it. No other file was
  touched during this window; this was not left in the tree.
  - `cargo test -p semio-framework-machine` (debug): **23/23 pass**, 0 failed.
  - `cargo test -p semio-framework-machine --release`: **23/23 pass**, 0 failed.
  - (The 6 tests inside `checkout_integration` cannot run at all — including in the
    *original* 318-error baseline — until the macro-crate follow-up in §3 lands; this is
    a pre-existing gap this packet cannot close from inside its boundary.)
- `cargo clippy -p semio-framework-machine --all-targets` — 0 errors, 4 pre-existing
  style warnings (`clippy::derivable_impls` on `StepInspector`, 2×
  `clippy::needless_pass_by_value` on `MachineStep::of`, `clippy::manual_contains` on
  `Snapshot::matches`/`MachineStep::is_active`). None are part of the async bug class,
  none were introduced by this packet (spot-checked: same code shape existed before,
  just wrapped in `async fn`); left untouched per R3's precedent of not scope-creeping
  into unrelated pre-existing lints.
- `cargo check -p semio-framework-machine-derive --all-targets` — clean, unaffected (its
  own test suite only parses/string-compares macro output, never type-checks against
  this crate's actual trait, so it stays green regardless of this packet's trait-shape
  change).
- `cargo test -p semio-framework-machine-derive` — 10/10 pass (debug); confirmed
  unaffected.
- Checked deriving crates (§4): only this crate itself; no other crate to verify.
- `bun ./📜️script.ts verify dependencies` — clean, 238/238, no new third-party deps
  (nothing was added to any `Cargo.toml`).

## 6. This crate's state machine vs. the Phase 2 `InteractiveJob` protocol

Genuine assessment, not a one-liner: this crate is a strong **foundation**, not a ready
implementation and not orthogonal.

**What already matches the protocol's shape, closely:**

- `PersistedSnapshot` + `step::step()`'s restore → macrostep → persist cycle is
  *architecturally* the same idea as a resumable job: nothing mutable survives across
  calls (the module doc says this explicitly: *"The working `Snapshot` is born and dies
  inside `step`... machine state is never a durable field"*), a host reads a persisted
  record, gets a result back, writes a new persisted record. That is precisely the shape
  `InteractiveJob::step(&mut StepContext) -> StepOutcome` needs underneath it.
- The `Command<M>` enum (`Effect`/`Raise`/`Send`/`Emit`/`StartInvoke`/`StopInvoke`/
  `Schedule`/`CancelTimer`) already cleanly separates *what happened* (kernel, pure)
  from *what to do about it* (host, effectful) — exactly the split Phase 2 wants between
  a fast bounded step and the effects it requests.
- `Inspector`/`InspectionEvent` (`MacrostepStart`/`Microstep`/`CommandIssued`/`Settled`)
  is a genuinely reusable *observable* channel — it already streams structured progress
  out of a running step at exactly microstep granularity. This maps naturally onto
  `StepOutcome::Yield`-style progress reporting with very little new code.
- `Status<O>` (`Running`/`Done(O)`/`Stopped`) already distinguishes in-flight from
  terminal states, which is the right starting taxonomy for `Complete`/`Cancelled`.

**Where it falls short of the protocol today, concretely:**

- **No mid-operation yield.** `run_to_completion` always drains its microstep queue
  (`VecDeque<ActiveTrigger<M>>`) to full quiescence — bounded only by
  `MICROSTEP_LIMIT` (a *count* safety cap against unguarded eventless cycles, not a
  *time* budget) before returning. `InteractiveJob::step` needs to return control after
  a bounded slice (<8 ms) even if the queue isn't empty, and be resumable from where it
  left off. The queue is already reified as a value (`VecDeque<ActiveTrigger<M>>`) —
  that is the encouraging part: turning `run_to_completion` into a budget-aware loop
  that returns `(StepReport, Option<VecDeque<ActiveTrigger<M>>>)` and folding the
  leftover queue into `Snapshot`/`PersistedSnapshot` as an explicit "in-progress
  macrostep" field is a targeted extension of the existing loop, not a rewrite or a
  parallel state machine layered on top.
- **No preview channel.** `MachineStep`/`Command` have nothing analogous to
  `PreviewReady` — the closest fit is another `Inspector` event or `Command` variant,
  which the existing plumbing could carry without restructuring anything.
- **No fault channel through a step.** Errors currently only surface at the
  `restore()` boundary (`RestoreError`) — mid-macrostep, action/guard functions are
  infallible fn pointers (`ActionFn`/`GuardFn` return `()`/`bool`, no `Result`). A
  `Fault` outcome would need either a fallible action-fn shape (real design change,
  bigger than this packet) or restricting faults to the host layer only (effects can
  fail; the kernel itself stays infallible) — the latter fits the existing "kernel is
  pure, host is effectful" split much more cleanly and is the reading I'd recommend.
- **One step = one external event, not one bounded slice of arbitrary work.** The
  kernel's unit of work is "one `Event` or `Timer` firing, plus every eventless/`on_done`
  follow-on" — a domain concept, not a time-boxed compute budget. Phase 2's protocol is
  the reverse: an arbitrary long-running operation sliced into <8 ms chunks regardless of
  its internal structure. These are complementary, not the same axis: a *statechart*
  instance is a natural `InteractiveJob` (its `PersistedSnapshot` round-trip already
  fits), but an `InteractiveJob` is not necessarily a statechart — most interactive
  operations (a long compute, a file import) have no natural state-node structure to
  hang microsteps off of. So: **this crate's kernel is the right foundation for jobs
  that are naturally state-machine-shaped**, and the `step()` entry point is very close
  to being reusable as `InteractiveJob::step`'s inner engine for that subset once the
  budget/yield extension above lands — but it should not be stretched to be the *only*
  or *universal* implementation of `InteractiveJob` for every kind of interactive
  operation Phase 2 lists.

## 7. Files touched

- `🧰️framework/🔨️modules/🔄️machine/🦀️component.rs` — 160 `async fn` → `fn`, 3
  `.await` removed (2 in a simplified `persist::restore` migration lookup, 1 in
  `step::step`), reformatted with `rustfmt`.
- `🧰️framework/🔨️modules/🔄️machine/📦️packages/🦀️rust/📦️glue.rs` — removed the now-stale
  `#![allow(async_fn_in_trait)]` and its explanatory comment, reformatted.

No other file in or outside this crate's boundary was modified. The temporary
`#[cfg(any())]` used to validate the rest of the suite around the known §3 blocker was
added and removed within this session (see §5) and is not present in the final state.

## 8. Cross-boundary summary for the coordinator / whoever owns `semio-framework-machine-derive`

1. `emit()` and `expand_statechart_event()` in `✨️derive/🦀️component.rs`: drop `async`
   from the 5 generated `event_id`/`event_name`/`definition` methods (§3) — mechanical,
   no body changes needed.
2. `export_wasm_machine!`'s generated inherent-method block: drop `async`/`.await`
   throughout (bodies now call sync kernel functions), fix the `restore::<M>(...)` call
   to supply its `Mg` generic (currently `E0107`, pre-existing and independent of the
   async class), and remove the now-unnecessary `wasm_bindgen_futures` dependency this
   generated code implicitly requires (currently `E0433`, since the crate never declared
   that dependency). Nothing in the repo currently exercises this macro outside this
   crate's own `wasm_smoke` fixture, so it was never verified before this packet.
