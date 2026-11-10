# jsonwriter result (11/09)

## Root cause (W-jsonwriter-4 trace)
The failing wave step is writer phase 1 (measuring), state 2 (object, next key): quote 57, receipt 73. `JsonWriteFrame` is 40 bytes. The step receipts `advance_source` frame copies `40 + u8 + 3 words (position, source_index, next_cursor)` = 65 plus 1 word of metadata (the opening quote) = 73. `writer_metadata_demand` quoted state 2 with `u8 + 1 word` (57): two words short.

## Fix (pack-json `🎛️controlled-and-borrowed-bridge/🦀️.rs`, `writer_metadata_demand`)
- state 2, next key: `u8 + 3*word` (was `u8 + word`).
- state 1, next element: `2*word + child` (was `word + child`; the receipt copies `index` and `next_cursor`).
- state 0 `Float32` / `QuotedUnsigned`: were missing in the quote match (would refuse as "source state changed"); now quoted like the other scalars.
- All other states audited against `advance_source`: equal.

## Law
`🧪️tests/📏️quote-equals-receipt/🦀️.rs` (own `[[test]] quote-equals-receipt-law` in `📦️packages/🦀️rust/Cargo.toml`, no dependency change so no LOCK) over corpus `🧪️tests/🧫️fixtures/📏️quote-equals-receipt.json` (16 cases: scalars, escapes, multibyte, long string, empty/flat/nested arrays and objects, escaped keys) plus Float32 and QuotedUnsigned sources. For copy 64/128/4096 and every step: quote == receipt (copy, capacity, release), the quote is unchanged when asked again with the grant it implies, output equals serde_json. W-jsonwriter-8: rc 0 (2 new tests + pending-quote-law pass).
Run: `SLOT_HANG_SECS=100 1105b-tools/runtime/tst.sh W-jsonwriter-<n> <pack-json 📦️packages/🦀️rust/Cargo.toml> native --test quote-equals-receipt-law --test pending-quote-law -- --test-threads=1`
(first attempt placed the law in the companion `retirement` target; its `retire_json_with_independent_grants` harness is a turn-less fixture that fails with "retirement field requires its original producer invocation", so the law lives in its own target without retirement helpers.)

## Upstream reds fixed on the way (companion `retirement` target did not compile)
`close_step` made `pub` (`🧮️parser-and-grammar-cursor`), and the body argument added to 5 call sites in `retirement-laws` / `borrowed-source-laws`. The companion target now compiles (W-jsonwriter-7); its tests still fail on the turn-less retirement fixture (`fp`/`fx` helpers) - open, not mine.

## flowhost3 stable_quote review: second production fault found and fixed
Root cause of the eval phase 33 / input owned-entry re-quote: `native_reads(body)` in value `🗂️ordered/⚙️engine/🦀️.rs` capped compare reads at `min(body,64)` but the quote adds the fixed metadata (744 B) on top, so a grant of `quote(1)=745` re-quoted to 744+64=808 and the step was never admitted (stall). Now `reads = min(remaining,64,body-fixed)` with a floor of 1 while bytes remain, used by both the copy quote and the work quote (`native_work_demand` had its own copy of the formula) and the step; a quote at body 1 equals the quote at its own grant.
`stable_quote` removed from `⏱️evaluation/🧪️tests/🦀️.rs` and `📥️input/🧪️tests/🦀️.rs`: both quote once with `copy`, grant `copy.max(quote)`.

## Verification
W-jsonwriter-10: `semio-os-kernel-neural-engine-budgeted --lib` 13 passed / 0 failed (rerun: `SLOT_HANG_SECS=100 1105b-tools/runtime/tst.sh W-jsonwriter-<n> Cargo.toml native -p semio-os-kernel-neural-engine-budgeted --lib -- --test-threads=1`).

Ordered package regression: W-jsonwriter-14 (`🗂️ordered/📦️packages/🦀️rust/Cargo.toml`, all targets) rc 0, all green after the quote change; stale `once(..|effect|owner.step(effect))` helper at `🗂️ordered/🦀️.rs:507` got its body argument (it blocked the lib test build).

Debts: companion `retirement` target (`semio-framework-pack-json-tests`) compiles again but its retirement fixture (`fp`/`fx` turn-less helpers) fails with "retirement field requires its original producer invocation" (needs the shared driven harness); not touched. Temporary [DEBUG] instrumentation removed (verified by grep).

## Companion `retirement` target (coordinator follow-up): 30/31 green (W-jsonwriter-28), 1 open
Support (`🧪️tests/🥣️companion/🧰️support/🦀️.rs`): `fx` now runs the closure inside a driven producer action (`admission::driven`, caller's own grant, so sealed retirement finds its invocation); new `retire_step` drives `ControlledRetirement::step_into` under `driven` inside `observe_retirement_allocations` and posts the receipt to the bank (reference: admission `receiver-driver.rs::retire`). `retirement-laws` uses `retire_step` for the zero-grant and granted retirement steps.
Production faults found by it, fixed in `🧮️parser-and-grammar-cursor/🦀️.rs` (`JsonProjectionIteratorRetirement`, also used by the writer cursors):
- child cursors were born by `ChildProgress(.., Default::default(), ..)` without a physical allocation, which the controller refuses ("original child receipt differs from its admitted physical transfer"); now the child is staged and admitted with `admit_retirement_cursor` (real allocation, receipt carries `copied_items=1`, `retained_capacity=birth`).
- its quote omitted the copy bytes of that birth (`size_of::<Option<T>>() + capacity`), so retirement never got admitted (stall); the quote now carries them.
Fixture: `🧫️fixtures/🎟️normal/🔣️.json` `maximumCopyBytes` 32 -> 1024. The first parser step quotes copy 424 (capacity 392 frontier + header), so 32 could never admit it. This is test data, not a production limit, but it is a raise: coordinator please confirm.
Open (1 test): `retained_json_writer_exposes_metadata_before_panicking_observer` (`🔬️unit/🫳️borrowed-source-laws/🦀️.rs` ~1027-1052). It still drives the settlement by hand with `record_effect` + `settle_effect`; `NativeEncodeEffectSettlement` now gets `submitted_progress` only from `record_performed_before_post`, called inside `NativeEncodeControl::effect_allocated_with_work(grant, local, demand, bytes, work, marker, failure, effect)` (which also catches the panic). `settle_effect` fails with "original submitted effect progress is absent", the `?` returns early and the unresolved settlement's Drop masks the error. Needs a rewrite of the test tail onto `effect_allocated_with_work` with a separate writer control (reference rig: `🌱️value/📝️text/📦️paged/🧪️tests/encoder-rig.rs` `Payment`).
Other companion targets (`tree`, `laws`) do not compile (stale `project_owned`, arity) - not touched.

## Round 3 (coordinator: no literal 1024) and HANDOVER (context low)
- Done: fixture `🎟️normal/🔣️.json` `maximumCopyBytes` = 424 and the test asserts per source `grant.maximum_copy_bytes == JsonParseCursor::new(Reject).normal_step_demands(text).copy_bytes` ("exact first-step quote"). Rerun W-jsonwriter-29 was queued behind the capacity gate when I stopped: read `$G/W-jsonwriter-29.rc/.txt`. If a later parser step quotes more than 424 the normal-frontiers test stalls (turns<100000): then the quote of that step is the thing to fix or the grant must be the maximum quote over the run (still derived from the quote API, never a literal).
- NOT done: panicking-observer test tail onto `NativeEncodeControl::effect_allocated_with_work` (value `🛫️encode/🎟️effect/🦀️.rs` ~84-102; marker = `NativeEncodeEffectState::view()`, failure = `Option<NativeEncodeEffectFailure>`; working use in `🚪️io/⏱️control/🛫️snapshot/🦀️.rs` ~247 and `🚪️io/⏱️control/🧺️backing/🦀️.rs`; rig `🌱️value/📝️text/📦️paged/🧪️tests/encoder-rig.rs` `Payment`). The writer needs its own control (the effect closure cannot also borrow `native`). Test lines ~1027-1052 of `🔬️unit/🫳️borrowed-source-laws/🦀️.rs`; unwrap-instrumented copy is not kept, file is restored.
- NOT done: companion `tree` and `laws` targets (stale `project_owned`, arity errors; see W-jsonwriter-16 error list).

STATUS: BLOCKED companion retirement 30/31 (panicking-observer test), tree/laws untouched; fixture now quote-derived (424, W-jsonwriter-29 pending); main task FIXED-verified W-jsonwriter-10
