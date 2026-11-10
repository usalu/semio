# mutapply3 (takes over mutapply2)

## Step 1 receiving lib tests (original_member*)
Faults found and fixed (all in `store/🧩️composition/📨️emission/🧳️receiving/🦀️.rs` unless noted):
1. Source admission dropped after the FIRST take_output step (publication destination created, owner/lease not yet transferred) -> "original source admission retains unpaid custody". Now dropped only when `terminal_is_empty` (advance_local + close_active).
2. While a published source is mid-handoff, `demands`, `advance_local` and the Work quote must keep using the admission (`source_admission.is_some()||source.is_none()`); previously fell to `retained_alias_demand` -> "retained source original owner is absent".
3. Test grant: the alias step reports two hops (bind + publish, copied_items 2); test `grant()` now has `maximum_items:2` (quote has no item axis).
4. Source witness: nothing ever bound `NativeDecodeControl::source`, so `control.source_witness()` failed for the JSON receiving path. INTERFACE: `ArtifactOriginalOperationReceiving::advance` and `MemberOriginalBatchReceiving::advance` now take an explicit `OriginalSourceTurnWitness` (lifted/operation-wire/receiving + binary impl ignores it). Callers must pass the witness of their Source turn.
- Note: close path for `projectionStart` needed no special case (return_inputs accepts it); the real faults were 1-2.

## Step 2 (witness, settled with coordinator)
- bind_source_witness removed; `NativeDecodeControl::with_source_witness(Option<witness>, op)` scopes the binding and restores the previous one. Used by job/context/decode (production, line 126) and `Authorities::nested` (testing-admission driven.rs). Explicit witness parameters were reverted (lifted receiving, binary, replacement, emission receiving back to the single control.source_witness() path).
- Compile green W-mutapply3-15 (lib test, native).

## Step 3 (open: receiver spins in the pack-json parse/phase stage)
- W-mutapply3-15..17: both `original_member*` lib tests now fail with `assertion failed: turns < 100000` (test.rs loop). Rows are tiny (<=28 bytes).
- From turn 34 the owner demand is constant (copy 2849, capacity 0) and each `advance` reports progress {items 1, bytes 1}: bytes 1 = `size_of::<Phase>()` added by the post-processing in `pack/json/decode/receiving` advance_turn (`p.copied_items.max(1)`, `+size_of::<Phase>()`), which turns an inner `Complete(Progress(default))` (denied/idle step from `Self::local`) into fake progress. So some local step is refused each turn and the wrapper masks it. Candidate: `Self::local(.. q ..)` refuses because the effect cannot fund quote q (Phase::Parse/Canonical quote = `maximum_work_units` = test WORK 1<<20, ConstructionWork admits always) or the construction grant (= grant(d) with copy exactly d.copy_bytes, child grant subtracts size_of::<Phase>()=1) is 1 byte short. Next probe: print inside `Self::local` which refusal returns default, or fund the test grant with `+size_of::<Phase>()` and WORK = exact quote; if the quote `demands()` omits the Phase header for this stage that is a production bug (receiver.demands must include the header it later subtracts).
- Fix the masking too: the post-processing must not turn a denied step into progress (it hides every refusal as a spin).
- Not started: mutation_apply laws, member_owned_batch_* (6), prepared_candidate test, PostFixture dev-dep (LOCK NEEDED for the 4 replication retained test crates).
- DEBUG logs removed from the test file; receiving.rs edits stay (drop-on-terminal-empty, admission-keyed demands/advance_local/quote); test grant maximum_items 2.
- Rerun: `"$C/1105b-tools/runtime/tst.sh" W-mutapply3-N Cargo.toml native -p semio-os-store-one-item-batch --lib -- --test-threads=1 original_member_wire --nocapture`

STATUS: BLOCKED original_member lib tests spin at turns>=34 (receiver stage returns a refused step masked as progress by pack-json receiving's Phase-header post-processing); priorities 2-5 not started
