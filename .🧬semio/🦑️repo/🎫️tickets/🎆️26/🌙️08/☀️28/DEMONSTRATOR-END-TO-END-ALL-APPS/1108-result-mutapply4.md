# mutapply4 (takes over mutapply3)

## Step 1 fix (a): zero-progress passthrough (done, compiled in W-mutapply4-3)
- `pack/json/decode/receiving` advance_turn: the post-processing closure now returns the inner step unchanged when its progress is `RetainedCloneProgress::default()` (denied/refused/idle). No Phase header or item is added. Effect: the spin now shows up as `original member receiver made no progress at turn=34` (test.rs:42) instead of looping to 100000.

## Step 2 finding (b): the quote omitted the Source `ready` stage cost (fixed, unverified)
- W-mutapply4-10 (POLL zero print): the zero step at turn 34 was `tag=9` = `admission.take_output` in `Phase::Source` with admission phase `ready`, grant == demand == 2849 exactly. `demands()` quoted `header::<Option<RetainedCloneSource<DslValue>>>()` but `take_output` prices `take_output_demand()` (`size_of::<Option<RetainedCloneSource<T,A>>>() + size_of::<Option<u64>>()` for the first hop, `take_output_completion_demand()` for the second). So the quote was short and `take_output` refused (progress default); the wrapper had masked it with the Phase header.
- Fix: `pack/json/decode/receiving` `demands()` Source/ready now quotes `admission.take_output_demand()` (the Phase header is added by `phase()` as for every stage). The zero-progress passthrough only applies when the phase did not advance (`self.phase==phase`), so a legitimate transition with no inner copy still charges its header.
- After the fix the two original_member tests got from turn 34 to turn 132 (W-mutapply4-11, stall assert fired at demand 2849 again); second stall under investigation (probe prints in W-mutapply4-12+ blocked by a peer's dsl-record-model source budget refusal).
- Lesson: any eprintln inside the observed advance allocates 64 bytes on first use and trips the test's `physical` heap assertion; print only on the refusal path.

## Step 3: more under-quotes found by W-mutapply4-18/19 (all fixed, verification pending W-mutapply4-20)
- pack json receiving `TypedSource` ready stage had the same under-quote (`header::<Option<RetainedCloneSource<..>>>` vs `take_output_demand`): now `admission.take_output_demand()` (turn 132 -> 218).
- `store/ledger/♻️retirement` `ArtifactStoreDecodedEditRetirement`: the pending-mutation row quoted only `2*size_of::<Mutation>()` copy while the inner `factory_retire_owned` needs `controlled_retirement_birth_demand` (Option<T> + ControlledRetirement copy) and the child grant had that remainder subtracted. Now quote = transfer + `factory.retire_owned_demands(mutation)` (copy, capacity, release, depth+1); child grant subtracts only the transfer; receipt adds only the transfer. Fixes "typed factory completed without publishing its retirement owner" (prepared_candidate test).
- `store/prepared/♻️retirement` `strings` stage quote was capacity-only; `admit_owned_retirement` refused (`Ok(None)` -> Progress(default) spin, the prepared test hit turn<999999). Now `nested(controlled_retirement_birth_demand(string))`.
- `store/composition/emission/owned` `TypedBatch::demands` / `next_copy_byte_demand` omitted the birth copy bytes (`size_of::<Option<Vec<M>>>()+birth_bytes`); `admit_frame` refused every funded close, so the six `member_owned_batch_*` tests never closed. Fixed: `birth_copy_bytes()`.
- Tests: receiving test.rs fail-fast stall assert (64 identical demands); emission/owned tests got `quoted(batch, grant)` (exact-quote grants) at all funded closes, `close_exact*` panics when the original effect quote is refused (was silent Progress(default)); copy-query law now expects birth copy = `size_of::<Option<Vec<i32>>>()+capacity` and subtracts it from the row-copy total.

## Step 4 (W-mutapply4-22 findings, fixes in W-mutapply4-23 pending)
- Receiving stall at turn 218 (demand copy 73 / release 3600): `last progress {copied_bytes 9}` = Canonical stage charging `size_of::<usize>()+Phase` for a canonical cursor step that wrote 0 bytes: `ArtifactCanonicalJsonTreeCursor::next_demand` final branch (frames empty, pages left to release) quoted only release bytes/depth 1, but `release_empty_page` denies unless the grant also covers `next_release_copy_byte_demand` and `next_release_depth_demand`. Fixed in `pack/json/encode/tree`.
- Same class in `ArtifactStoreDecodedEditRetirement::demands` (inverse page release): now copy + depth from the PagedList quotes (this removed the prepared_candidate spin at demand release 4080).
- emission/owned copy-query law rewritten: every stage with copy != 0 charges exactly its quoted copy; row-copy total (stages whose copy == fixture minimumCopyBytes) == rows*4; header stages (40, 424, birth) are exact-charged but not row copies.
- 4 of the 6 member_owned_batch tests are green (W-mutapply4-22: borrowed_admission, child_depth, constructor_refusals, phase_queries, delegate_work, exact_type_source); copy_query is pending W-23.

## Step 5 (W-mutapply4-24: compiled, cleaned tree, no [DEBUG] left)
- Evidence that quotes now equal actual cost: copy_query trace for rows=1 shows every stage charging exactly its quoted copy (856, 424, 48, 56, 40, 12, 24, 32, 48, 32, 0, 0). The only failing assertion there was the old row law; the row-copy stage costs `minimumCopyBytes + size_of::<usize>()` (12). Test rewritten accordingly (`row_stage`, total == rows*row_stage; fixture minimumCopyBytes stays 4 for the TS oracle). Unverified until the next tst run.
- Receiving lib tests (both `original_member*`): last known failure `made no progress at turn=175, settling=false, demand copy 81 / release 48 / depth 2` (W-mutapply4-23/24). 81 = 1 (Phase) + 72 (Canonical comparison extent 64+8) + 8; this is the `Phase::Canonical` stage again, now at the cursor's `release_empty_page` branch with copy 8 / release 48 / depth 2. The zero step means `canonical.advance` returned Progress(default) at the exact grant: look at `pack/json/encode/tree` `advance` -> `admitted(demand, grant)` vs the nested `canonical_child` grant (copy = child.copy - 72) and at `flat_run` / `release_empty_page` gates (items, depth `next_release_depth_demand`, copy `next_release_copy_byte_demand`); print the three numbers on the refusing branch only (any print in the observed advance trips the heap-receipt assertion at its first use).
- Note the progress passthrough in pack json receiving only covers fully zero inner progress; the Canonical arm still adds `size_of::<usize>()` receipt bytes even when `written_bytes==0` and the cursor made no progress: the arm should return zero progress when the cursor returned `Progress(default)` with `written_bytes==0` (same masking class).

## Open
- Receiving stall above (turn 175) and the Canonical-arm masking.
- prepared_candidate_original_owners_retire_with_actual_allocator_receipts: after the quote fixes it fails with `UnsupportedOwner "shared retirement requires its borrowed producer"`. The legacy `close_step` of `SharedControlledRetirement` (value/retirement/contract/arc-custody) deliberately errors; the prepared box reaches it through `artifact_retirement_box_close_step` (test) / `ArtifactStorePreparedRetirement::close_step` (`post` -> `ArcRootRetirement::new_snapshot`). Needs a `close_into`/`close_original_into` port of `ArtifactStorePreparedRetirement` (+ the test driven by `clone_source_testing::drive_reply`), not a quote fix. The test also has a fail-fast stall assert now (64 zero-progress turns).
- mutation_apply laws one at a time (--test-threads=1) and the PostFixture dev-dep LOCK (4 replication retained test crates) are not started.
- Rerun: `"$C/1105b-tools/runtime/tst.sh" W-mutapply4-N Cargo.toml native -p semio-os-store-one-item-batch --lib -- --test-threads=1 --nocapture` (full lib, ~35 s run once built; build 20-40 min under fleet load).

## Files changed (all compile-atomic, W-mutapply4-24 built)
- pack/json/decode/receiving/🦀️.rs (zero-progress passthrough when the phase did not advance; Source/ready and TypedSource/ready quotes = `take_output_demand()`)
- pack/json/encode/tree/🦀️.rs (cursor release stage quote: copy + depth)
- store/composition/emission/receiving/🧪️tests/🦀️.rs (stall assert)
- store/composition/emission/owned/🦀️.rs + tests/🦀️.rs (TypedBatch birth copy quote; quoted()/close_exact grants; exact-charge and row law)
- store/ledger/♻️retirement/🦀️.rs (pending-mutation row quote; inverse page release quote)
- store/prepared/♻️retirement/🦀️.rs + tests/🦀️.rs (strings quote; stall assert)
- value/retained-clone/source/admission and value/decode/owned/protocol: debug prints added and fully removed (net zero)

STATUS: BLOCKED receiving lib tests stall at turn 175 (Canonical arm), prepared_candidate needs a close_into port, mutation_apply laws and PostFixture LOCK not started
