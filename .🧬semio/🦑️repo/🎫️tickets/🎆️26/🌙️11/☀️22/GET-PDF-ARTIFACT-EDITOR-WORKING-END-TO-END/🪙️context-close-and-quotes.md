# Context close, clone quote and Pack oracle fixes

Open items 16 and 17 of `📋️open-items-round-6.md`, plus the two red framework Pack tests.

## 1. `closeActorContext` of a retired program (row 16)

### Cause
`close_plugin_step` (`🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs`) answers `Progress(default)` (zero spend, neither complete nor blocked) while the entry's job state is `registered`, has a source or holds an output. `registered` is set by the first `start_job` and is cleared only by `close_jobs_step` (the `closeJobs` export, which closes every registered job under paid steps). The native host calls `closeJobs` before the context close; the browser component bridge never did. A PDF program starts jobs (page workers), so every retirement hit this answer, `closeOriginal` returned `sourceClosed: false` and the kernel stock kept the original source held (funds forfeited per open/close cycle).

### Fix
- `🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts`: `OriginalGuestJobs` gains `closeJobs`; `OriginalJobFunds.close(jobs)` loops `closeJobs` under paid crossings until `admitted` (a stall counter that only counts crossings that spent nothing).
- `🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`: the bridge remembers whether a job was ever started and, only then, runs `fundedJob(..., funds.close(jobs), cleanup)` before the `closeActorContext` loop in `closeOriginal`. Receipts flow through the same source, so the stock sees every spend.

### Law (language-agnostic case plus executable test)
- Schema `🔨️modules/🎭️actor/🎟️retained-turn/🔁️succession/🧬️schema/🔣️.json` gains `retirement` (`jobs`, `liveSourcesAfterClose`); case in `🧪️tests/🧫️fixtures/🔣️.json`: `{ "jobs": 1, "liveSourcesAfterClose": 0 }`.
- `🔌️plugin/🌐️browser-bundle/🏗️materialization/🧪️tests/🔁️succession/🟦️.ts`: the stand-in guest now answers the real zero-spend "neither" for `closeActorContext` while a job is registered and clears it in `closeJobs`; new test "an actor that ran a job closes its context on retirement and returns its whole source to the stock" opens, runs the fixture's jobs to completion, retires through the real worker (`dispose`) and the real kernel stock and asserts `liveSources` goes from 1 to the fixture's 0.
- Bun: `🔁️succession` 4/4 green with the fix; the new test is red (stock keeps its live source) with the fix switched off.

## 2. Clone quote under-quotes the paged octet reservation (row 17)

### Cause
The test (`original_causal_envelope_clone_preserves_wire_value_and_shared_actor_identity`) stalled at an exact-fit grant with quote `copy 0, capacity 392, depth 1`. Instrumenting the field cursor's child selection showed the quote came from `PagedBytesCursor::advance_demands` (`🌱️value/🧬️retained-clone/📦️paged/🦀️.rs`), the cursor of the causal payload (`CausalPayload = PagedBytes<...>`). Its capacity branch quoted only `next_capacity_allocation_bytes` and depth 1. The step it funds, `PagedList::reserve_capacity_one_funded`, refuses unless the grant also covers `next_capacity_copy_byte_demand` (the page header writes: 424 B here) and `next_capacity_depth_demand`; with only the quote it answers `Denied` and the step is `Progress(default)` forever. `PagedList`'s own clone engine (`🧬️retained-clone/📋️paged-list/⚙️engine/🦀️.rs`) already quoted all three axes.

### Fix
`PagedBytesCursor::advance_demands` quotes `copy_bytes = next_capacity_copy_byte_demand`, `capacity_bytes = next_capacity_allocation_bytes`, `depth = next_capacity_depth_demand.max(1)`. All callers of the quote (record rows, vec, option, field cursors, test drivers) pick it up.

### Test change
`grant()` in the test asserted `copy + capacity <= 4096` for every indivisible quote. The frames now legitimately quote 3056/3008 up to 4832/4784 (the envelope is 424 B, each page header write is quoted exactly), so the bound is per axis, `<= 2 * BODY`.

### Results
`original_causal_envelope_clone_preserves_wire_value_and_shared_actor_identity`: green (exact-fit grants progress). `semio-framework-replication-causal --lib`: 44/44.

Neighbours that could not be run: `semio-framework-value-paged --lib` (lib tests do not compile: missing `effect_fixture` and `semio_framework_value_testing_allocator` in the receive-work test) and `semio-framework-value-tests` (346 compile errors), both independent of this change. The Pack cursor crate (17/17) was last run before the `PagedBytes` quote change; the final re-run was left queued behind memory pressure and cancelled, so that combination is unverified.

### Not changed: reply-shaped close ticket (second half of row 17)
The note that a close grant equal to its quote does not move a reply-shaped ticket (`closing(d)` slack of one item, two depth levels, 4096 B capacity in `🔌️plugin/🫧️transient/🧵️publication/🧬️native/🧪️tests/🦀️.rs`) was not re-verified or fixed: the paged octet quote was its own, independent defect, and that crate was not run in this slice.

## 3. Red framework Pack tests (crate `semio-framework-pack-record-encode-cursor`)

- `borrowed_pack_document_derived_expression_joins_original_symbol_and_default_codec`: the failure was in the test's reference oracle, not in the cursor. The oracle helper `encode_document` calls `encode_document_controlled` with a `NativeEncodeControl` that carries no Work recipient. Printing an expression is paid original Work (`control.with_effect`), so the oracle refused with "original work requires its original caller loan". The expression test now builds its expected bytes through `encode_document_with_loan`, which runs the same controlled encoder under the existing `staged_rig::EncodeBank` (a real Source turn and a finite stock). `staged_rig` is exported `pub(crate)` from the cursor tests module for this.
- `borrowed_projected_pack_native_paged_key_and_text_cross_unicode_pages`: the rig call `encode_success::<_, 8194>(..., maximum = 65536, ...)` passed the control's capacity (65536) as the text maximum, but the text formatter refuses a maximum above its page capacity N = 8194 ("text formatter exceeds its original capacity"). The maximum is the fixture's `length` (8194 B). The failure was the test's argument, not the formatter.
- Result: crate `--lib`: 17/17.

## Files
- `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🔁️succession/🧬️schema/🔣️.json`, `🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧪️tests/🔁️succession/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/🔗️causal/🧬️clone/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🦀️.rs`, `…/🎒️document/⏳️cursor/🧪️tests/🦀️.rs`

Temporary tracing used in the field engine and the clone test (marker `CCDBG`) was removed again; the field engine file is byte-identical to its start state. Scripts: `chain-cc.sh`, `chain-cc2.sh`.
