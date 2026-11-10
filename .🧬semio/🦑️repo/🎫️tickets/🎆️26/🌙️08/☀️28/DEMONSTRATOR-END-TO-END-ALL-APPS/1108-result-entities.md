# 1108 entities result

## Step 1 (lib redesigned onto the 733 reply contract)
- W-entities-4 native lib rc 0, W-entities-5 wasm32-wasip2 lib rc 0, no warnings in the crate.
- `ToolRunEntityEdit`: Editor wraps Insert/Remove cursors; phases IDLE/ADVANCING/TAKING/CLOSING_BEGIN/CLOSING/DONE, normalised from cursor state (`output_ready`, `closing`). Every turn is `advance_into`/`recover_advance_into(grant,effect,reply)`; child cursor calls forward the caller reply through `reply.scope(&mut state.route,phase)` (route key = phase, recover iff route is set); local transitions are `local_step` (funded check, `effect.performed`, reply completed). No sync wrapper, no stash needed because cursor replies are forwarded unchanged.
- `take_output_into` replaces `take_output(grant)` (paid, reply-shaped).
- SUPERSEDED (coordinator ruling): see Step 3. Previously: Source admission dropped (the 733 `borrowed` needs a projection frame driven by a Source/Work action). The original stays resident in State and is read; the candidate is rebuilt by one Append `ToolRunEntityEdit` (retract: filtered scan; append: scan + targets). Child edit closes through `ControlledRetirement::step_into/recover_step_into`, forwarded the same way. Phases are explicit per sub-step.
- Sealed helpers: `admit_sealed` on `SealedShared::admit_from`; `publish_sealed` on `prepare_birth` + `SealedSharedBirth::publish`, returns `Option` (None = unfunded).
- Posted FIXED-verified + INTERFACE in claims.

## Step 2 (tests ported, unverified)
- `🧪️tests/🦀️.rs` rewritten to the reply contract (5 laws + sealed quote law) over a new `🧪️tests/⏱️effect/🦀️.rs` harness (EffectBank + OriginalEffectAuthority drive loop, heap observed with semio-framework-trace). `sealed_quote_tests` replaced by `sealed_birth_is_funded_by_its_exact_quote`.
- Cargo.toml: dev-dependency `semio-framework-value-testing-effect-bank` added; LOCK NEEDED posted. W-entities-6 (`native --tests`) stops at unresolved dev-deps (serde_json, semio_framework_trace and effect-bank are all absent from the hub lock for this package); no test code error is visible until the lock is regenerated.
- Rerun after lock: `chk.sh W-entities-7 🌎️hub/Cargo.toml native --tests -p semio-framework-tool-run-entities`, then `tst.sh W-entities-8 🌎️hub/Cargo.toml native -p semio-framework-tool-run-entities`.

## Open
- Debt: Version rebuild is O(n) appends instead of an ordered-map clone cursor (clone needs a driven Source projection frame; restore when a drive helper for `StagedSourceAdmission::borrowed` exists).
- Rerun: `chk.sh W-entities-<n> 🌎️hub/Cargo.toml native -p semio-framework-tool-run-entities` (then wasm, then `--tests`).

STATUS: FIXED-verified W-entities-4 (native lib), W-entities-5 (wasm lib); tests written, BLOCKED on lock (dev-deps) at W-entities-6

## Step 3 (coordinator: clone-based edits restored)
- New `🔗️source-hold/🦀️.rs`: `SourceHold<T,A>` = staged original + `OriginalSourceOwner` + projection action + custom `RetirementCursor` (`HoldRetirement`) that hands the Source back (Unique prepare/return) and retires the original through `ControlledRetirement`. `ToolRunEntityVersion`/`ToolRunEntityProvenance` implement `OriginalSourceProjection`.
- Version edit phases: ADMIT (capture + projection drive) -> CLONE (`RetainedOrderedMapCloneCursor::advance_into` over `hold.original()`) -> TAKE_CLONE -> BEGIN/CLOSE/DROP_CLONE -> CONSTRUCT (edit over the clone with the targets only) -> ADVANCE -> TAKE -> WRAP -> CLOSE -> DROP. Provenance gets ADMIT in front and reads marks through the admitted Source.
- Verified: native lib W-entities-7 rc 0; wasm lib W-entities-13 rc 0 (W-entities-8 failed upstream in native-owned peer edit, W-entities-10 was the same family).
- New law `original_tool_run_entity_version_edit_cost_is_independent_of_the_cloned_size` (n=16 vs 2048: clone work scales >=8x, edit work <=4x). Unrun.
- Blocked: `--tests` (W-entities-10) still lacks dev-deps in `🌎️hub/Cargo.lock`; lock-120/sync-120 did not touch that file. LOCK NEEDED re-posted.

## Step 4 (tests run through 🧰️framework/Cargo.toml)
- `--tests` W-entities-17 rc 0 (fixed two borrow errors in the harness). Run: 5 of 7 pass: edit law, sealed quote law, version cancel-after-every-turn, provenance cancel-after-every-turn, `..cost_is_independent_of_the_cloned_size` (n=16 vs 2048: clone work >= 8x, edit work <= 4x).
- API: `advance_into`/`recover_advance_into` of ToolRunEntityEdit and ProvenanceEdit take an explicit `body` (as Version does); a body derived from a denied grant changed the quote. `publication_demands()` is the exact `publication_demand::<T>()`.
- Peer fix (claimed + released): `source-owner` guard so a cancelled Drive-selected projection without a pending action reaches `projection_step` instead of failing.
- Failing 2 (value-lane defects, posted NEEDS-OWNER): version law (unaccounted realloc when the insert cursor inserts into an exact-capacity cloned page) and provenance law (`ControlledRetirement<ToolRunEntityEdit<u128>>` UnsupportedOwner; u64 works).
- Libs re-verified after all edits: native W-entities-48, wasm W-entities-47 rc 0. No [DEBUG] lines remain.

STATUS: FIXED-verified libs W-entities-48 (native), W-entities-47 (wasm); tests 5/7 (W-entities-46 and earlier runs), 2 BLOCKED on value-lane defects (insert into cloned page, u128 controlled retirement)
