# 🔎️ 1107 Audit Review 3 (aud-review-3)

Read-only review of `git -c core.fsmonitor=false diff HEAD` (1036 files, 40.6k insertions) for the recently ported areas: store decode ladder and close conversions, store codec and plugin sqlite hosts, tick-wire, tool-run, dsl-record-model `📥️owned`, pack-record, job-native-encoding, ui-prepared, replication-retained and history. Owners come from `C/1107-lanes.md` and the addenda in `C/1107-brief.md`, applying runtime2's rulings W1 to W4 and R1 to R7 from `C/1107-result-runtime2.md`. No runs, no edits outside C, no sub-agents, no miri run.

Method: a Rust-only diff (`/private/tmp/.../scratchpad/full.diff`), greps over added lines for `reborrow(..maximum_work_units())`, raw pointers, `AuthoringWallet`, constant `work:`, zero-progress `Pending`/`Progress(default)`, and `unwrap/expect/panic/assert`, then targeted reads of the source around each hit. Test code was excluded from the unwrap count. dsl-record-model `📥️owned` and the store codec host helpers were not read line by line; their coverage here is limited to the grep hits.

## Findings, ranked by runtime impact

### 1. Raw-pointer witness in job-side source encoding is unsound (high impact if it bites, confidence high on the aliasing reasoning, not run under miri)

File: `🧰️framework/🔨️modules/🧵️job/⏱️context/🛫️encoding/🦀️.rs`

- L215: `let token=std::ptr::from_ref(self.original_source_turn()?);`. L220: `NativeEncodeControl::new_recorded_source(maximum,depth,unsafe{(*token).witness()},..)`. Between them, L217 `Mutex::new(&mut*self)` takes a new unique borrow of the whole `StepContext`.
- The token is not behind a reference that outlives `self`. `SourceTurnReference::Owned(OriginalSourceTurn)` (L78) stores it inline in `StepContext`, so the pointee lives inside `*self`. Creating `&mut *self` after the shared borrow that produced `token` pops that borrow's tag, so the read at L220 is UB under Stacked Borrows and Tree Borrows as written.
- Same class in `EncodingEffectReceipt` (L201–205): `context` is `addr_of_mut!(*self)` created before the `&mut *self` at L217 and dereferenced in `settle` (L203) after `drop(source)` at L222. `output as *const _` (L217) is read in `settle` after `encode(control,output)` (L221) wrote through `output`, which is the write pattern of L110 and L114.
- Why it is not a crash today: the value read is a plain `u64` identity, and no optimizer assumption has been observed to break it. It is still not sound, and the brief asked for exactly this check.
- Fix: `new_recorded_source` only uses `source.identity()` (value `🛫️encode/🦀️.rs:176–177`). Take a `Copy` `OriginalSourceTurnIdentity` captured before any `&mut` borrow, and delete the `unsafe` block. For the receipt, hold `&Mutex<&mut StepContext>` and `&Mutex<EncodingHop>` created before the lending, or `&Cell`, and drop the raw pointers.
- Owner: jobsplit (`acdff90e240d22566`, the only editor of `job/⏱️context/` per the 03:30 addendum). The value-side `new_recorded_source` signature change belongs to the value lane (`a1bf0499e1a315f51`). Runtime (`a8f570504dc3f4b48`) decides the signature.

### 2. `reborrow(.., effect.maximum_work_units())` is wrong by construction at 57 live call sites (high impact, fail-closed, confidence high on the code)

- `RetainedAllocationEffect::reborrow` (value `🎟️control/🧬️contract/📦️effect/🦀️.rs:33–36`) computes `remaining = min(receipt remainder, maximum − spent)` and refuses with `OwnershipLimit` when the requested maximum exceeds it. Passing the parent's own `maximum_work_units()` therefore refuses as soon as the parent has performed any work, or whenever the receipt remainder is below the authored maximum. This is the defect review-2 finding 1 described.
- The check is pre-existing in HEAD. The diff only adds `nested`/`nested_within` to that file (+7 lines).
- Live count: 57 occurrences in 19 framework files (`grep reborrow(…effect.maximum_work_units()`). Ones added in this diff (12 lines): `📋️list/🪢️concat/🦀️.rs` (5 sites: L59, L82–87), `📦️aliases/🦀️.rs:54`, `📽️projection/📦️owned/🦀️.rs:30`, `🛬️decode/🧳️owned/🌳️value/⚠️refusal/🦀️.rs:154,169`, `🧬️retained-clone/🔗️source/🧪️testing/🦀️.rs:71` (test helper with `.expect("retirement child exceeds original effect")`), and `♻️retirement/📋️queue/🧪️tests/🎮️controlled/🦀️.rs`.
- Impact is refusal, not double charge: a failed reborrow returns `Err` before any producer runs. But on close and retirement paths it turns valid multi-step closes into `OwnershipLimit`/`WorkLimit`. The `.expect` sites panic in tests.
- Fix: `effect.nested(child)` at every site (the pattern already used in deflate `🛫️encode/⏳️cursor/🦀️.rs:49`, window `📬️mutation/🦀️.rs:32` and value-snapshot-read). Review-2 finding 1 is fixed in those two files only.
- Owner: value lane (`a1bf0499e1a315f51`) for the value sites; runtime (`a8f570504dc3f4b48`) for the shared primitive.

### 3. Presence JSON store decode still runs authoring wallets inside a native receiving turn (high, confidence high)

File: `🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📸️snapshot/🔤️json/🦀️.rs`

- L10–13: `effect_turn` and `effect_poll` build `AuthoringWallet::new(body.remaining_grant(), TURN_UNITS)` and `run`/`run_settled`. The `decode_presence_json` loop (L35–53) also builds inline wallets.
- Ruling R4 (`C/1107-result-runtime2.md`) said to delete these wallets and use `native.receive_effect` with a resident `NativeReceivingEffect`. The file still has them, so R4 is not applied. Under W1 this work is uncharged against the turn's fuel and must be refused in review.
- Owner: store (`a9d14ef1566792fdf`).

### 4. SQLite schema validation builds a wallet inside a native control turn (medium-high, confidence high)

File: `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🏛️validation/🦀️.rs`, L38, `SchemaValidationStorage::parse`

- Each reservation loop creates `AuthoringWallet::new(..,1).run(grant,1,|e| self.units.reserve_one(e))`, then calls `control.record_performed_work(done.performed)`. That is two ledgers beside `control.admit_allocation_bytes(quote)`.
- W1: a control reachable from a native turn gets its effect from that turn, not a wallet. Use the control's `with_effect` (runtime2 encode-side primitive, owner port-e) or the receive-effect frame.
- Work `1` per call is a single action, which is consistent with W2.
- Owner: port-f (no agent id in `C/1107-lanes.md`; the brief's addendum 08:25 names port-f for this crate).

### 5. Pending and zero-progress returns that can spin (medium, confidence medium, driver stall guard not reviewed)

- `ui-prepared` `🎟️prepared/📤️staging/♻️retirement/🦀️.rs`: `prepared_close_gate` (returns `Pending{default}` when `bytes > maximum_release_bytes` or `maximum_items == 0`), `prepared_close_bytes` (same test on `release > maximum_release_bytes`, and `Pending` for a non-empty buffer when `maximum_copy_bytes == 0`), and `prepared_close_authored` (maps a `None` from the wallet to `Pending{default}`). A backing larger than any turn's release budget never closes and never refuses. W4 asks for a typed refusal or `Blocked`.
- `prepared_close_authored` is called from `🛍️products/…/📺️renderer/…/🖥️frame-presentation/🧯️retirement-and-present-steps/🦀️.rs:123` per frame, with a 64-unit stock and no `cx`. That is a W2 candidate (per-call constant standing in for a turn). Owner: unassigned in the lanes table; closest is reacthost (`a4d1224fa2b0af2e6`).
- Tool-run tick receive `🛬️receive/🦀️.rs:96–101`: `Pending{work:0}` maps to `Progress(default)` without a refusal. The backpressure path is `frame.cancel_unstarted(control)?` at L97, which returns before `self.frame=frame` at L98, so the frame is dropped unrestored on error. Owner: tickwire (`af3f16d327c436e5a`) and port-d (`ac184d5a0692b2b49`).
- Same `Progress(default)`-on-`!funds` pattern: tool-run `📼️trace/🦀️.rs` (several), `📨️payload/🌊️stream/🧳️update/🦀️.rs`, value `✨️derive/🛬️receive/🦀️.rs` and `🔀️enum/🦀️.rs`, and `🗂️ordered`/`📝️text` zero-work returns. Each is a no-admission signal that loops if the grant never grows.
- Pack canonical tree `🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs:520` (`output.is_empty()` returns `Progress(default)` with work 0). Still present from review-2 finding 10. Owner: aggregator (`a78c08da65901d641`).
- Quote-vs-record check: tick receive calls `receive_effect(demand, 1, …)` (L92), so any `advance_admitted` step that records more than one unit through the shared effect, including the nested `trace.advance` and parser calls, is refused `WorkLimit`. Owners need to confirm each phase records at most one unit. Confidence medium-low.

### 6. Turn-less wallets whose callers were not verified (medium-low, need caller check)

These are cold or standalone entries by their own docs. Each needs a check that no production caller is inside a `cx` or native turn (W1), and that the stock is not an admit-all constant (W2).

- Replication fold `🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs`: `authored` (L59–61, fixed `1<<16` work), `HistoryFoldJob::step` (L269–271) and `close_cold` (L373–374). `close_cold` reports `work:0` on any error (the spent wallet work is dropped from the report), and `last_work` saturates to `u64::MAX` (L263). Owner: bearbeiten (`a02fc4a3e9a4931c6`).
- SPR history schema `🛍️products/…/📡️spr/📜️history/🧬️schema/🦀️.rs` L228: `fold()` builds `AuthoringWallet::new(stock, 1 << 24)` and runs with `1 << 16`. `1<<24` is an admit-scale constant. Owner: unassigned (store area).
- Pack JSON `🔤️json/🧮️parser-and-grammar-cursor/🦀️.rs` L10: `parse()` sizes its stock from node count, with `unwrap_or(u128::MAX)`. Owner: unassigned (review-2 flagged it).
- Pixels `🖼️image/📥️decode/🦀️.rs` L81–93: `drive_png`/`authoring_wallet` with `1<<20` budget, and `ImageDecodeJob::step` (review-2 finding 5: non-PNG phases still call `advance(grant)` without the effect; error kinds collapse to `Invalid`). Pixels compositing `🗂️layers` (`into_result`) and surface paint compositing use wallets too. Owner: proc (`a9416d3c03e6a69e5`).
- UI scene mesh `📐️math/🕸️mesh/🧱️mesh-owner/🦀️.rs:33` (`authored_step`). Owner: unassigned.

### 7. Store decode-page close conversion: meaning preserved, two items to verify (low-medium, confidence medium)

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/📄️decode-page-state-and-authority/🦀️.rs`, `close_granted` L223–300.

- Checked against HEAD: the empty-grant, depth, `copy/capacity/release/depth` over-budget, `Contended → Blocked`, `return_now()` false → `Blocked`, `checked_field_close_demand`, `grant.maximum_depth <= demand.depth → DepthLimit`, `artifact_retirement_grant_covers`, `progress.fits(child)` and the terminal check are all retained. The child now runs as `owner.close_step(&mut effect.nested(child))` (L289), which is the correct shared-effect form.
- Verify 1: `artifact_envelope_close_charge` (L239) must `performed()` the reported work of the grant-shaped `close_step_granted` child, or that work is never charged. The helper's definition is in the same owned-schema folder; I did not read its body.
- Verify 2: `artifact_envelope_close_resume_with` discards the work of `RetainedWorkPoll::Pending{..}`. That is fine for nested children (work is already on the shared effect) and wrong for any child that records work elsewhere.
- The `.expect("closing job holds its field decoder lease")` calls predate this diff.
- Owner: store (`a9d14ef1566792fdf`).

### 8. Panics and asserts reachable from production paths (medium-low, confidence high on the code)

- Tool-run presentation `📊️progress/🪟️presentation/🗃️slot/🦀️.rs`: `admit(..).unwrap_or_else(|_| panic!("original presentation update birth was fully preflighted"))` and `self.state.published.as_ref().unwrap()`. Owner: port-d.
- Window-config registry `🗂️registry/🦀️.rs`: `assert!(self.insertion.is_none(), …)` panics on an insert while a pending insertion exists. Owner: port-b (`a5090211a70df05f5`).
- io-control `🛫️snapshot/🦀️.rs`: `value.expect("completed snapshot retirement step")`. Safe only if `post` never turns `Err` into `Ok`. Owner: koordinator (`a35a27af5fbf1186e`).
- SQLite-snapshot receiving `🫴️receiving/🦀️.rs`: `checked_sub(before_work).expect("original SQL producer work is monotone")` and `slot.as_mut().unwrap()`. Owner: port-f.
- Store owned-schema: `decode-owner-bundle-and-bounded-array-authority` (`reservation.take().expect("observed live history reservation")`, `retirement_factory.as_ref().expect(…)`, `slot.as_mut().expect("observed original schema fact…")`), `…/🎟️field-decoder-registry…` (`owner.as_mut().expect("returned field decoder remain…")`), and the native encoding/decoding `with_settlement_refusal(error).expect("record body has its original…")`. Owner: store.
- Pack tree `expect("checked immutable native canonical child")` inside `project`: guarded by the `canonical_tree_child(ordinal)?` precheck. Low risk. Owner: aggregator.
- Scan count: about 871 added lines in the whole diff match `unwrap/expect/panic/assert`, including tests. Most in tool-run entity and update cursors (`👥️entities`, `🧬️update`, `🧾️provenance`, `📸️version`) are phase-machine `.as_mut().unwrap()`. They are reachable only through phase bugs, but any phase bug panics rather than refuses. Owner: port-d.

### 9. Dropped checks and refusals: not systematically verified

I did not run a removed-line census. The one dropped-check case I confirmed is the test weakening that review-2 finding 6 reported (ui-locale, port-c). The store decode-page conversion kept its checks (item 7).

## Resolved since review-2 (verified in the current tree)

- Review-2 finding 1 (deflate `initialize_one`, window `close_step`): now `effect.nested(..)` (deflate `⏳️cursor/🦀️.rs:49`, window `📬️mutation/🦀️.rs:32`). The same pattern persists elsewhere (item 2).
- Review-2 finding 2 (tool-run tick receive constant `work:1`): now `work: effect.performed_work()` (`🛬️receive/🦀️.rs:94`).
- Review-2 finding 3 (pack tree `last_turn_work=1`): now derived from the effect, with `effect.performed(1)` when the turn made progress and recorded no work (tree `🦀️.rs:558`).
- Review-2 finding 4 (tool-run native failure paths): phase 3 error arm now debits `performed`; fuel is debited before the retained `?` (`🛬️native/🦀️.rs:62–63, 83`).
- Review-2 finding 8 (pixels flood `expect`): removed.

Not re-verified here: review-2 finding 5 (PNG decode), finding 7 (io-control Pending progress) and finding 10 (zero-progress spins beyond item 5).

## Owners (from `C/1107-lanes.md` and the addenda)

| Item | Owner |
| --- | --- |
| 1 job encoding raw pointers | jobsplit `acdff90e240d22566`; value `a1bf0499e1a315f51`; runtime `a8f570504dc3f4b48` |
| 2 reborrow sites (value) | value `a1bf0499e1a315f51`; runtime `a8f570504dc3f4b48` |
| 3 presence JSON wallets | store `a9d14ef1566792fdf` |
| 4 sqlite validation wallet | port-f (no agent id in lanes) |
| 5 ui-prepared close, tick receive, tick trace | reacthost `a4d1224fa2b0af2e6` (ui-prepared, unassigned in table); tickwire `af3f16d327c436e5a`; port-d `ac184d5a0692b2b49`; pack tree aggregator `a78c08da65901d641` |
| 6 cold wallets | replication fold bearbeiten `a02fc4a3e9a4931c6`; pixels proc `a9416d3c03e6a69e5`; spr history, pack parser, mesh: unassigned |
| 7 decode-page close | store `a9d14ef1566792fdf` |
| 8 panics | port-d, port-b `a5090211a70df05f5`, koordinator `a35a27af5fbf1186e`, port-f, store, aggregator |

## Confidence and limits

- Item 1: high on the aliasing analysis, read from source. Not run under miri. Not reproduced.
- Item 2: high on the code (`reborrow` check and call sites). Reachability into a specific caller was traced for the close-path sites only.
- Items 3 and 4: high. Read the code directly.
- Items 5 and 6: medium. The driver stall guard and the production callers were not read.
- Item 7: medium. The helper bodies in the owned-schema folder were not read.
- No command was run. No test reproduces any finding.
