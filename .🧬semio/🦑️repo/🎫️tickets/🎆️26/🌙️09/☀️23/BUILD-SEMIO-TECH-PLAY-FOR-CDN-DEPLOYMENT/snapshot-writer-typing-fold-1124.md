# Snapshot Writer Typing Fold (1124)

Crate: `semio-s-artifact-writer-writer-typing-fold` (wasm32-wasip2, release), 25 errors in
`/Users/ueli/.sinc/play/.cache/b/r/m/components/runs/wVzfDkM/l/cargo-release-00-b1.log`.
Tree edited: `/Users/ueli/.sinc/play` only. **Nothing was compiled** (no cargo/rustc run).

The live repo copy of `🔗️fold/🦀️.rs` is byte-identical to the snapshot copy, so it is stale too and there was no live port to take. The live framework has the same new API, so the port below applies to live as well.

## Causes per error class

| Class | Count | Cause |
|---|---|---|
| E0061 (6 args, 4 given) + E0308 `Result<OriginalConstructionStatus>` vs `Result<bool>` + E0282 closure param | 4 + 4 + 1 | `admit_original_construction` (`🧵️job/🏭️construction/🦀️.rs:184`) is now `(cx, output, &mut OriginalConstructionAdmission, demand, work:u64, construct(scope,&mut Option<usize>)->RetainedWorkOutcome<(O,RetainedCloneProgress)>)` and returns `OriginalConstructionStatus`. Its bool-returning, settle-driving form is `admit_original_construction_settled(cx, admission, output, demand, work, construct)` (`:242`). |
| E0308 `(NativeAllocationIdentity,u64)` vs `(u64,u64)` | 2 | `StepContext::native_receiving_identity()` now yields `(NativeAllocationIdentity,u64)`; the `decoder` field kept the old `(u64,u64)`. |
| E0308 `RetainedWorkOutcome` vs `Result<RetainedCloneStep,_>` | 3 | `close_original_retained_step`'s closure must now return `RetainedWorkOutcome` (result and work), and the function returns `InteractiveJobCloseStep`. |
| E0277 `?` in a closure returning `RetainedWorkOutcome`, E0277 `?` on `RetainedWorkOutcome`, E0599 `unwrap_or` on `_`, E0308 `RetainedAllocationEffect` vs `RetainedCloneGrant`, E0308 `RetainedWorkPoll` vs `RetainedWorkOutcome` | 3 + 2 + 2 + 3 + 1 | `RetirementSelection::step` and `RetainedCloneClose::begin_granted` now take `&mut RetainedAllocationEffect` and return `OriginalRetirementPoll` and `RetainedWorkOutcome<Option<RetainedCloneStep>>`. They must run under `close_original_effect_step` (closure `&mut RetainedAllocationEffect -> RetainedWorkPoll`), not under the grant-shaped `close_original_retained_step`. |

## Model followed

- **Main model:** `OriginalTextSpliceComposer` (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🦀️.rs`). The Writer fold already drives this sibling.
  - Its `admit` calls `admit_original_construction_settled(cx,admission,output,construction(d),1,|native,actual|construct(actual,||…))`.
  - When settled, it swaps the non-terminal admission into the born owner's resident `construction`.
  - `advance` and `take_into` reuse the owner's resident admission with work `1`.
  - The close metadata closure returns `RetainedWorkOutcome{result:Ok(..),work:1}`.
  - The private `construct` helper reports `actual` capacity.
- **Plugin helpers:** the same shape is in `🔌️plugin/👷️job/🛬️receiving/🏗️steps/🦀️.rs` (`built`, `built_with`, `adopted`, `begin`). I inlined it instead of using these helpers, so no new dependency was added.
- **Effect closes:** `FundedText::close_step` (`🧵️job/📝️text/🧳️owned/🦀️.rs:72`) wraps `begin_granted(..,effect)` as `RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(Progress(default))),work:turn.work})`. The closes go through `close_original_effect_step`, as in `🛠️tool-machine/⌨️typing/✍️edit` and the composer.
- **Where the new arguments come from:**
  - Admission: the producer's existing resident `construction: OriginalConstructionAdmission` field, which was already closed in `close_step`.
  - Work: `1`, which is the composer's and the `built` helper's one-unit constructions.
  - No dummy budgets were invented.

## Changes

`/Users/ueli/.sinc/play/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⌨️typing/🔗️fold/🦀️.rs`

- **Imports:** `admit_original_construction_settled`, `close_original_effect_step`, `RetainedWorkOutcome`, `RetainedWorkPoll` and `retained_clone::NativeAllocationIdentity`. The stale `admit_original_construction` import was dropped.
- **Helpers:**
  - `construct`, copied from the composer.
  - `closed(step)`, which builds the work-1 close outcome.
  - `begun(turn)`, which builds the `FundedText` begin poll.
- **`decoder`:** now `(NativeAllocationIdentity,u64)`.
- **Factory `admit`:**
  - It refuses an occupied output, as the composer does.
  - It settles into a local admission and a local `Option<Box<WriterOriginalTypingFold>>`.
  - The trait gives no resident admission slot, so this is the only place it can live, and the settled call commits only when the whole quote is funded.
  - Whenever an owner was born, the admission is swapped into `owner.construction` (the composer and `adopted` pattern) and the owner is moved into `output` as the trait object. The born owner never drops unclosed, even on an error path.
- **Producer constructions:** `transition`, `make_decision`, the phase-0 Split/ReplaceIncoming decision and `take_into` now use `admit_original_construction_settled(cx,&mut self.construction,<output>,demand(d),1,|_,actual|construct(actual,||…))`. The bodies are unchanged.
- **`close_step` metadata closes:** the closing flag, composer release and final phase now return `closed(..)`.
- **Retirement steps:**
  - The retirement-ticket step is now `close_original_effect_step(cx,d,|effect|match self.retirement.select(self.body){Ok(s)=>s.step(effect),Err(e)=>Complete{Err(e.into()),work:0}})`. This keeps the old select-then-step semantics.
  - The composition and decision retirement births are now `close_original_effect_step(cx,d,|effect|begun(self.retirement.begin_granted(&mut slot,effect)))`.

`/Users/ueli/.sinc/play/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⌨️typing/🔗️fold/🧪️tests/🦀️.rs` (cfg(test) only)

- `retire` now uses `close_original_effect_step(cx,d,|effect|owner.step(32768,effect))`, because `ControlledRetirement::step` is now `(body,effect)`.
- The producer-shell drop now returns `closed(..)`.

## Consistency checks

- No `🪞️mirror` or other crate in the writer plugin mounts `🔗️fold/🦀️.rs`. Only its own package and the operations unit-test crate depend on it, and that test crate uses only `original_typing_fold()` and `WRITER_ORIGINAL_TYPING_FOLD`.
- No other writer source calls `admit_original_construction` or `close_original_retained_step`.
- The trait `ArtifactOriginalTypingFoldFactory`/`Producer` is unchanged, so the trait signatures were kept.
- No new dependencies and no Cargo.lock changes.

## Unresolved / risks

- Not compiled. Two places rely on rustc deref-coercing `&mut ManuallyDrop<Option<T>>` to a generic `&mut Option<O>`: `&mut self.decision` as the output, and `&mut self.composition`/`&mut self.decision` in `begin_granted`. The old code passed these the same way and the log reported no error on those arguments.
- Factory `admit` holds its admission locally because the framework trait has no admission slot. If a committed settle ever returns `Ok(false)` before any owner is born, that admission's bound metadata is dropped rather than resumed. The framework documents that a committed settled quote binds, constructs and settles in the same turn, so this should not happen.
- The test file was not otherwise re-validated against the snapshot framework (for example the `OriginalJobFixtureCaller` and decoder helpers). It is not part of the release build.
