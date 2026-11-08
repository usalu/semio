# jobsplit (b): continuation family crate split (staged plan, nothing swapped)

STATUS: PLAN ONLY. Live tree untouched by (b). Measured with `surface.py` (output `surface.txt`).

## What moves
`job/📬️outcome/⏸️continuation/` non-test sources: 9 files, ~101 KB on disk (about 75 KB of it counted in the job-context closure; 📒️book is already its own crate). New crate: `semio-framework-job-continuation` at `📬️outcome/⏸️continuation/📦️packages/🦀️rust/` (framework workspace; entry `../../🦀️.rs` like job-recorded-clock). Manifest goes into `🧰️framework/Cargo.toml` [workspace.dependencies] only; coordinator must get `LOCK NEEDED` before the swap.

## Cycle and how it is cut
job-context depends on the new crate (StepContext stores `OriginalReturnBudgetReference`), so the new crate cannot name `StepContext`. Surface it uses (surface.txt):
- 31 StepContext methods (22x original_source_parent, 52 direct `return_budget` field uses, retained_grant, consume_fuel/consume_retained, should_yield, retirement_should_yield, original_clock_binding, clock_source, original_absolute_deadline_us, original_total_*, original_source_epoch, preflight_original_reserved_return, receive_original_reserved_return, capture_return_frame/budget, admit_original_return_table, ...).
- 2 parent types: OriginalCallerSourceParent (validate_borrowed_context 14x, source_identity 9x, admit_return_origin, into_source_loan) and OriginalCallerSourceLoan; plus OriginalCallerReturnPlace, OriginalSourceBinding, close_original_retained_step, receive_original_retained_work, JobOutcome{Borrow,Descriptor,Admission}, InteractiveJobCloseStep, OriginalTurnFailure (job-failure crate, already lower), OperationId, Generation.

Cut: new crate defines `trait OriginalReturnCx` (the 31 methods; `return_budget()`/`return_budget_mut()` accessors replace the 52 field uses; `type Parent: OriginalReturnParent` with the 8 parent methods) and every `cx:&mut StepContext<'_>` in the family becomes `cx:&mut impl OriginalReturnCx` (mechanical). job-context implements the trait for StepContext (about 4 KB) and re-exports the public family items at their current names (the crate's own facade; no shim). Types needing StepContext internals but not stored in it (OriginalCallerSourceLoan, ReturnPlace, SourceBinding) stay in job-context behind the `Parent` trait.

## Estimated effect
Closure: -75 KB + about 6 KB (trait impl + glue) => production about 190 KB, testkit about 196 KB, i.e. 60+ KB margin on both.

## Steps (compile-atomic)
1. Build the new crate in this dir (`stage/`), copy family files, mechanically genericise, check it alone with a stub impl test crate (native + wasm32-wasip2).
2. Stage job-context edits as patches (`patches/`): trait impl, `pub use`, removal of the 3 `#[path]` mods, Cargo dep.
3. Coordinator: LOCK NEEDED for the new Cargo.toml + root workspace deps; sync.
4. Swap in one step (manifest + mods), then check job-context + job-receiving + one wasm32-wasip2 dependent + one native testkit dependent under W-jobsplit-<n>.
5. Update external importers of moved paths (list: `git grep original_job_continuation` outside the crate) in the same step.

## Risks
- Generic functions on a trait with `with_original_work_loan<R>(…run:impl FnOnce(&mut Self)…)`: object-unsafe, fine (static dispatch), but needs `Self:Sized` bounds on a few methods.
- Hot-path monomorphisation: only one impl (StepContext), no cost.
- Peers touching the family files during the staging: freeze announcement needed from the coordinator before step 2.
- Blocked now by upstream reds (value-native-decoding); do not swap until job-context compiles.

## CORRECTION after reading the family code (step 1 attempt)
The trait cut above does NOT work as a file move. job-context is one strongly connected component, not a core with a removable family:
- `OriginalJobReturnOrigin`/`OriginalJobReturnReceiver` (continuation/origin) STORE concrete `OriginalCallerSourceParent` and `OriginalCallerSourceLoan`; `OriginalCallerSourceParent` stores `OriginalReturnSource` (continuation/source). A new crate holding the family would have to be generic over parent/loan types (public API change on every consumer), and the parent in turn is generic over its own `StepContext`.
- `⏸️park` defines inherent `impl OriginalCallerSourceParent` (park_owned_turn, close_*), which Rust forbids outside the defining crate; `🎟️source` parent calls `OriginalOwnedCallerTurn::from_admitted_source` back. Same for capture/child/place/weak.
- Measured: of the 101 KB continuation family only 2.6 KB is StepContext-free; the rest is `cx:&mut StepContext` glue. Existing precedent trait `OriginalFailureContext` (job-failure) covers 14 of 31 methods, but it works because failure code stores no parent/loan.
- Verdict: no mechanical split. A real margin needs a redesign: shrink StepContext (a god object with ~190 methods) by moving state with its logic into owned sub-objects living in lower crates (as done for SourceClock and ReceivingScope), or genericise Parent/Loan/StepContext. Days, not hours; breaks consumer types.
- Cheap leftovers (about 1 KB each, test-only `internal` testkit split; shared budget accessor helpers in continuation) do not bridge the 2.6 KB testkit gap alone.
