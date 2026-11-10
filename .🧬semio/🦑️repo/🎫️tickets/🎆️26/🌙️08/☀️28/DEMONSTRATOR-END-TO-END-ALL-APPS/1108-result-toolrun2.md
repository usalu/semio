# 1108 toolrun2 result

## Step 1 (2026-11-09)
- tick/🛬️receive:100 cause-loss edit re-read: maps like bridge `finish_receiving` (allocation_refusal=child.or(existing), primary=existing.or(producer_refusal), work_refusal=existing.or(failure.work_refusal)); kept.
- semio-framework-tool-machine (EDITED, verifying W-toolrun2-3): ledger reads are reply-shaped. `🔗️read/📤️transfer` macro now generates on Scrub/Typing/Gesture ledgers `retained_read_demands(window,key,body)`, `advance_retained_read_into`/`recover_advance_retained_read_into`, `retain_read_demand`/`retain_read_into` (replaces `retain_original_read`), `return_retained_read_into`/`recover_return_retained_read_into`; the old sync methods are removed from the three ledger files. `OriginalToolLedgerSource` trait: `original_read_demands(..,body)`, `original_read_advance_into`/`recover_original_read_advance_into`, `original_return_into`/`recover_original_return_into`. scrub/🧵️original `owners.step(body,..)` and `owner.step(body,..)` arity.
- plugin tool-run `🎞️tick/🎮️operations/📖️source`: `OriginalToolLedgerRead` stashes `read_reply`/`return_reply` + `read_pending`/`return_pending`; pending returns no progress and keeps state, next turn calls the `recover_*` form; `terminal_is_empty` includes the stash and flags.
- semio-framework-tool-run: lib `admit_sealed` on `SealedShared::admit_from`; `➕️insert` `close.step(body,effect)`.

## Open (not done)
- tool-run `📨️payload/🌊️stream/🧳️update` (the 10 remaining errors): needs a real redesign, not an API rename.
  1. phase 2 `StagedSourceAdmission::borrowed(source,|s|&s.get().pages)` no longer exists; the borrowed admission is `borrowed(original:A)` with `A: OriginalSourceProjection<T>` that takes an UNSEALED A and a driven Source/Work projection action. Use `semio_framework_tool_run_entities::source_hold::SourceHold<T,A>` (needs a tool-run -> tool-run-entities dependency; LOCK NEEDED) with a `PersistentList<Vec<u8>>` projection of `ToolRunPayloadStream` held unsealed; `previous: SealedShared<ToolRunPayloadStream>` must be unsealed first (paid take_authority) or the published directory must be re-modelled.
  2. phases 6/7: `PersistentSplice::{advance_into,take_into,begin_close_paid,close_into}`; the update's own `advance`/`take`/`close_step` become `advance_into/recover_advance_into(body,grant,effect,reply)` with `reply.scope(&mut route,phase)` forwarding exactly like `ToolRunEntityEdit::drive`; local transitions via `local_step`.
  3. `Close` cursor: `RetirementCursor::close_into/recover_close_into` replace `close_step`.
- presentation `🧬️update` (17 phases) + `🗃️slot` + plugin tick `🛬️native:~100` then move onto the same reply form (ToolRunEntityEdit/ProvenanceEdit/Version `advance_into(body,grant,effect,reply)`, `take_output_into`); also needs SourceHold for phase 1 (`StagedSourceAdmission::borrowed` progress source) and `ToolRunProgressClone`.
- Not started: wasm, `--tests` (tool-machine tests still call the removed sync methods in `🔗️read/🧪️tests`, `⌨️typing/✍️edit/🧪️tests`).

## Step 2
- W-toolrun2-3 (native lib, tool-machine): rc 0, FIXED-verified. Plugin 📖️source reader and tool-run lib/insert edits are uncompiled (tool-run still red on stream update).

STATUS: BLOCKED tool-run payload stream update + presentation need the SourceHold/reply redesign above (not an API rename); tool-machine FIXED-verified W-toolrun2-3 (native lib)

## Step 3 (coordinator-approved redesign)
- tool-run: new path dep on semio-framework-tool-run-entities (LOCK NEEDED posted). `📨️payload/🌊️stream/🧳️update` rewritten: `PayloadOriginal` (projection of the page directory), `SourceHold` for the borrowed original, splice `advance_into/take_into/begin_close_paid/close_into`, `advance_into/recover_advance_into(body,effect,reply)` with `reply.scope(&mut route,key)` forwarding, `Close` cursor relays with route+stash. FIXED-verified native W-toolrun2-4 rc 0.
- presentation: update/slot are reply-shaped (`advance_into/recover_advance_into(body,grant,effect,reply)`), progress clone holds a `SourceHold`, closes forwarded through `step_into/recover_step_into`; `OriginalSourceProjection<ToolRunProgress>` for `ToolRunPresentation`; seed publish_sealed Option. Native W-toolrun2-7: presentation compiled with no errors.
- tick-wire: `owner.step(body,effect)` arity (2 sites); W-toolrun2-8 stopped on peer red `RetirementHeaderRelease` in replication-mutation-messages.
- plugin tick `🛬️native`: `advance_presentation` and `close_effect_step` stash reply + pending flag (`adv_*`, `close_*`), recover form on the next turn.
- Rerun: `chk.sh W-toolrun2-9 Cargo.toml native -p semio-framework-tool-run-tick-wire -p semio-plugin-tool-machine -p semio-plugin-tool-run -p semio-plugin-tool-run-operations -p semio-plugin-tool-run-turns -p semio-os-store-engine-close -p semio-os-store-engine-bundle` (in flight), then wasm, then `--tests` (tool-run/presentation/tool-machine tests still call removed sync APIs).

STATUS: IN PROGRESS, tool-machine/tool-run/presentation native libs verified; tick-wire + plugin crates W-toolrun2-9

## Step 4
- W-toolrun2-9: stops upstream at semio-os-store-one-item-publication (17: 📥️lifted/🧵️canonical-edit close_step/close_ladder, operation-wire binary arity) and semio-os-store-backbone (3: ♻️retirement + 📤️original-outbox close_step arity). Neither is in my claim; tick-wire and the six crates were not reached, so plugin 🛬️native, tool-machine plugin, tool-run plugin, operations, turns, engine-close, engine-bundle have no verdict.
- Rerun W-toolrun2-10 (same list) when store-one-item-publication and store-backbone are green; then wasm and --tests.

STATUS: BLOCKED upstream semio-os-store-one-item-publication + semio-os-store-backbone (W-toolrun2-9); framework tool-machine/tool-run/presentation native libs FIXED-verified

## Step 5
- semio-os-store-backbone FIXED-verified (close_step body arity in ♻️retirement, 📤️original-outbox; gone from W-toolrun2-10). W-toolrun2-10 (tick-wire + six crates) then stops upstream at store `🧬️snapshot-clone` (40), store `🧵️operation-wire` (32), `🗣️dsl` (1): not mine.
- `--tests` must use `🧰️framework/Cargo.toml` (root manifest lacks the dev-deps). W-toolrun2-12 (tool-machine, tool-run, presentation, native --tests): libs green, only test files red: tool-machine `🔗️read/🧪️tests/🦀️.rs` 58 + `🧵️scrub` 13, tool-run `🧪️tests/🔬️unit` 15, `📨️payload/🌊️stream/🧪️tests` 15 (presentation tests not reached).
- Test port plan (not done): replace the old `paid(caller,g,d,|grant|outcome)` with `paid_into(caller,g,d,|effect,reply|...)`: inside `caller.turn`, check `cx.should_yield()||!cx.retained_work_can_enter(d)`, run `cx.with_original_effect(cx.retained_grant(),|_,effect|{let mut slot=None;drop(call(effect,OriginalEffectReply::borrow(&mut slot)?)?);Ok(slot)})`, then `cx.consume_retained(progress)` and `cx.consume_fuel(performed)`; return (progress, step-complete). Call sites: `retained_read_demands(..,4096)`, `advance_retained_read_into`, `retain_read_into` (needs a `RetainedOwnedProjectionAdmission`), `return_retained_read_into`, `RetainedOwnedProjection` close via its `*_into`; stream tests use `advance_into(body,effect,reply)`/`close_into` with the same helper.

STATUS: BLOCKED upstream store snapshot-clone/operation-wire/dsl (plugin crates unverified); tests not yet ported (token budget); libs tool-machine, tool-run, presentation, store-backbone FIXED-verified native

## Step 6 (tests)
- Shared reply-shaped harness `⏯️tool-run/🧪️tests/⏱️effect/🦀️.rs` (`paid(bank,grant,|effect,reply,recover|..)`, `primitive`, `retire`, `bank`, `grant`, `Observed{progress,work,heap,complete}`); included by `#[path]` in tool-run stream tests, presentation tests and tool-machine read tests. Dev-deps: effect-bank on tool-run, presentation, tool-machine (LOCK NEEDED each); stale `clone-source-testing` removed from presentation.
- Ported: tool-run `📨️payload/🌊️stream/🧪️tests` and presentation `🧪️tests/🔬️unit` (turn/slot_turn/drain via harness, `admit_sealed`, foreign-original refusal through `advance_into`). `chk W-toolrun2-15 🧰️framework/Cargo.toml native --tests -p semio-framework-tool-run -p semio-framework-tool-run-presentation`: rc 0. Test run `tst.sh W-toolrun2-16 ... --lib` queued on capacity (result unread).
- tool-machine `🔗️read/🧪️tests/🦀️.rs`: `read_law!` rewritten onto the harness (retained lease via `retain_read_into` + `RetainedOwnedProjectionAdmission`), NOT compiled yet; still old: `transfer_law!` closing loop, typing growth test, `🧵️scrub`, `⌨️typing/✍️edit/🧪️tests`.
- Rerun next: `chk.sh W-toolrun2-17 🧰️framework/Cargo.toml native --tests -p semio-framework-tool-machine`, fix by error list; read W-toolrun2-16; then six crates when store snapshot-clone/operation-wire/dsl clear.

STATUS: IN PROGRESS tool-run + presentation tests compile (W-toolrun2-15), tool-machine tests partly ported

## Step 7 (first test runs)
- W-toolrun2-16 (tool-run + presentation `--lib`): tool-run 3 pass / 2 FAIL; presentation 0 pass / 6 FAIL.
  - tool-run stream tests: `phase quote=RetirementDemand{copy_bytes:5290,depth:1}` exceeds the fixture policy `maximumCopyBytes` 4096 (`🧫️fixtures/🔣️.json`); that quote is the `SourceHold` capture/projection frontier (`OriginalSourceOwner::capture_demand`, value lane). Not raised (brief: never raise limits). The second failure (`owner.complete()`) is the same denial loop. Needs a value-lane ruling (shrink capture_demand or split the capture frame) or an explicit policy decision.
  - presentation: five failures were fixture grants without copy bytes for cold `admit_from`/`admit` setup (fixed by a `setup()` grant in the test, not a product limit).
- W-toolrun2-17 (presentation `--lib` after that fix): 4 FAILED quickly, 2 tests (`captures_keep_joint_original_body`, `update_is_atomic_and_physically_owned`) ran over 60 s (likely a never-completing denial loop at the same capture quote); panic text was lost when the run was cut. Rerun with `--lib -- --test-threads=1` and read the panics before further edits.
- Open: tool-machine tests (`read_law!` ported, rest old), six plugin/store crates (upstream red), wasm.

STATUS: BLOCKED value-lane SourceHold capture quote (5290 B copy) above the 4096 B fixture policy; tests compile but 2+6 fail (W-toolrun2-16/17)

## Step 8 (final, context limit)
- Coordinator ruling applied: stream fixture cap derived from the exact capture quote: `📨️payload/🌊️stream/🧪️tests/🧫️fixtures/🔣️.json` policy.maximumCopyBytes 4096 -> 5290 and the three literal `maximum_copy_bytes:4096` in `🧪️tests/🦀️.rs` -> 5290 (no schema const exists for it; `🧪️tests/🟦️.ts` has none). Production quote untouched.
- W-toolrun2-18 (tool-run + presentation `--lib -- --test-threads=1`): tool-run 1 pass (group) / 4 FAIL, no longer copy-denied but stalled:
  - `..retains_native_pages..`: `assert!(turns<100000)` (advance never completes), `..keeps_complete_capture..`: `owner.complete()` false, `..cancel_after_every_turn..`: `close` never reaches terminal, `..returns_every_cancelled_native_prefix`: `owner.terminal_is_empty()` false.
  - Probable cause (unverified): the harness wallet gives each `paid` run `UNITS=64` Work, but the `SourceHold` capture/projection frontier or the splice turn needs more original Work per turn than 64, so every turn is denied with no progress. Check: print `effect.remaining_work_units()` vs `hold.quote`/work in `drive` phase 2, or raise `UNITS` in `⏯️tool-run/🧪️tests/⏱️effect/🦀️.rs` (test harness constant, not a product limit).
  - Presentation `--lib` results did not print panics before the run was cut (it follows the same stalls); rerun after the UNITS check: `tst.sh W-toolrun2-19 🧰️framework/Cargo.toml native -p semio-framework-tool-run -p semio-framework-tool-run-presentation --lib -- --test-threads=1`.
- Exact next steps:
  1. Test UNITS (harness) and read the stall; fix `drive` in `📨️payload/🌊️stream/🧳️update/🦀️.rs` if the stall is a missing state transition.
  2. Presentation tests: read panics, repeat the same fix for `update` phases.
  3. `chk.sh W-toolrun2-N 🧰️framework/Cargo.toml native --tests -p semio-framework-tool-machine`; finish `transfer_law!`, typing growth test, `🧵️scrub`, `⌨️typing/✍️edit/🧪️tests` on the `paid` harness (`read_law!` is already ported).
  4. Six crates + tick-wire native/wasm/--tests when store snapshot-clone / operation-wire / dsl are green.
- Release: tool-machine, tool-run, presentation, tick-wire, store-backbone released; nothing else held.

STATUS: BLOCKED tool-run stream/presentation tests stall (probable Work UNITS in harness), tool-machine tests partly ported, six crates upstream-red
