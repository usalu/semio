# replay3 result (takes over from replay2)

Lib tests: 4/9 (was 3/9). Re-run: `SLOT_HANG_SECS=100 tst.sh W-replay3-N Cargo.toml native -p semio-os-store-replay-operation --lib` (last full run W-replay3-15). Compile: W-replay3-1 rc0.

## Done
1. **ArtifactMutationSource ported** (`native/🔗️input/🦀️.rs`): now `SourceHold<M,FactoryOwnedSourceAuthority<M>>` (semio-framework-value-source-hold = OriginalSourceOwner + projection-frame drive) plus a resident `OriginalEffectAuthority<Producer<M>>` whose receive/recover call `hold.step_into(body,effect,reply,recover)`; pending quote/body kept until the reply is recorded; alias (`retained_alias`) bound directly once `hold.ready()`; close = `RetainedCloneClose::begin_granted(&mut hold)` (HoldRetirement: Unique handback, then ControlledRetirement of the returned authority). Same public API (admit/demands/advance/take/ready/begin_close/close_demand/close_step/terminal_is_empty). Cargo.toml +semio-framework-value-source-hold (LOCK NEEDED logged). The two semantic tests now pass the mutation-source phase (exact 29 advance calls asserted via `MUTATION_SOURCE_STEPS`, derived from quote-exact grants; 8 of them are 1032-byte admission-birth blocks).
2. **Root-retention stall fixed** (`native/🔗️history/🦀️.rs` `birth`): `drive_into`/scope demotes the final Complete to Progress, so `ready=matches!(step,Complete)` never flipped. Now `ready` = the ledger's birth quote is zero after the step. `original_history_input_returns_same_backing_and_captured_issuer` passes.
3. **terminal-ticket test**: quote was capacity-only; now `controlled_retirement_birth_demand(&7u64)`. The old STARVED(12) turn now opens an effect at probe ceiling 8 and closes (work 1); the unadmitted-quote law is asserted with `UNADMITTED=7` (no effect opens, ticket kept). Test then reaches `caller.finish()`.
4. Test helper `grant` cap `1<<14` removed (coordinator: fixtures follow quotes).

## Open (5 red)
A. `original_semantic_factory...` (86:307) and `original_zero_work_loan...` (116:78): "original projection must be driven or returned through its resident frame". Now NOT from ArtifactMutationSource but from `NativeSemanticPreparation` (`native/🦀️.rs`): its own `source_admission: RetainedCloneSourceAdmission<P,CapturedSnapshotSource<P>>` (admission_borrowed, stages SourceCapture/Source, close_admission, `source`, `source_authority`, `base` close paths, delegates_original_close, close_original loan paths) needs the same SourceHold port: `SourceHold<P,CapturedSnapshotSource<P>>` + resident producer frame (generalize my Producer/Call/run into a generic `HoldCustody<T,A>` in native/🔗️input and use it in both; replace `self.source.as_ref()...try_borrow()` with `custody.original()`; close via `retirement.begin_granted(&mut custody.hold,..)`). **Perf debt:** `NativeSemanticPreparation` holds the 18528-byte `RetainedCloneSourceAdmission` inline (Producer ctor quote 19.6 KB copy / 18.5 KB capacity); the hold port removes it.
B. `original_history_source_birth...` (62:398) and `original_retirement_preserves_child_and_work_commit_failures` (155:100 `failure.primary()` None): undiagnosed. B1: `caller.close(failure)` of the denied turn's output still does not close (needs failure-storage birth like prepare_failure_storage). B2: the preserved close failure arrives without `primary` (ValueError message pointer law).
C. `original_terminal_ticket...` fails in `OriginalLawCaller::finish` (63:308, "decoder did not close") after the ticket law passes.
D. Wide-grant chaining law (coordinator, after reds): a wide production grant must finish `ArtifactMutationSource` in <=2 turns; not implemented (quote-exact grants take 29 turns by construction).

## Hygiene
- No [DEBUG] left (grep verified over replay/operation). No mem::forget, no AuthoringWallet. Backups only in the scratchpad.
- Claims: CLAIMED input, Cargo.toml, native tests/history/probe; to be RELEASED below.

STATUS: BLOCKED lib tests 4/9: preparation SourceHold port (A), failure-output close + primary-less failure (B), decoder finish (C), wide-grant law (D); mutation source + history stall fixed
