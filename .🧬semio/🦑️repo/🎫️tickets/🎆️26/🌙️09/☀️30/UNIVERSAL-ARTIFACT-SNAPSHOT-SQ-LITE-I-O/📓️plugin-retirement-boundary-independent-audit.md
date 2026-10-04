# Plugin Retirement Boundary Independent Audit

## Verdict

No actionable regression found in the typed caller prerequisite repair. This is a source boundary and control-flow review, not a successful native build or runtime result. Root's JSON18 lane must establish current compilation and executed behavior.

## Scope and Evidence

Reviewed the prerequisite report, original compiler diagnostic ledger, the changed retirement/preparation/query/window/initializer portions of all 15 listed files, and the canonical Store owner-close and Value paged-list definitions. Unrelated concurrent inference-context and macro dependency changes in the large Plugin root are outside this verdict. No Cargo, runtime check, source mutation, modifying Git command or worktree operation ran in this audit. Read-only Git diff inspection was used to distinguish prerequisite edits from unrelated existing changes. No parser result is used as behavioral evidence.

## Findings

- Retained-root, zero-payload and captured-root retirement implementations now expose canonical `ValueError`. Capture reader, private query capture, query close, initializer pumps, preparation and private erased window publications forward that type directly. No new message extraction or category reconstitution occurs between these private stages.
- Locally created failures have explicit categories: exact registry/base-return rejection and initializer grant/terminal violations use `InvariantViolated`; four serde input bridges use `InvalidValue`; the two absent controlled metadata producers use `UnsupportedOwner`.
- Message projection occurs at actual existing job fault bytes, Fault/SDK diagnostics, String live-query/retained-load diagnostics or VCS validation terminals. The SDK publication dispatch preserves each publication's typed result until its Fault-returning dispatcher. No blanket conversion or message classifier was introduced.
- The initializer's replacement for `retire_envelope_uninstalled` preserves the one-step owner-close prerequisite and terminal witness while avoiding the helper's String projection. `bounded_document_store_owners` creates the bounded catalog; the canonical fresh cursor disposer completes `close_uninstalled_step(1)` and reports its empty witness. The envelope remains incrementally retired by its existing retained retirement cursor. The two old helper refusal messages are consolidated into one invariant diagnostic; this changes diagnostic wording, not the accepted close states.
- Current `PagedListError` has `kind` and static `reason`. Retained catalog close projects `reason` at its existing `Result<PluginCloseStep, String>` terminal. That agrees with its current Display implementation, which writes only `reason`; it does not introduce a new textual classifier or accidental debug rendering.
- The repair does not add close loops, destructor-based cleanup, queue removal before terminal witness, grant widening, cancellation removal, or success-on-error branches. Existing Pending/Blocked/Complete handling and cancellation calls remain in place.

## Limits

This verdict is confined to changes attributable to these prerequisites. Existing retained allocation hint `.ok()` projections and the apply-ops terminal failure `mem::forget(owner)` remain visible in surrounding source but are not introduced by this repair. Existing fail-closed exceptional owner-close behavior likewise is not newly added. None of these observations certifies universal retirement correctness or feature completion. Native diagnostics and behavioral receipts remain root's responsibility.
