# History read retirement selected work map

Source at review time selects active ticket first, then derived snapshot (registry alias, derived alias birth, derived split), plan retirement or plan handoff, preview, raw replay, factory authorities, then terminal. Preview actions after draft/registry owners are checkpoint, derived operation (terminal shell/cancel/child), operation close ticket, displaced snapshot admission, identity, draft handoff, state admission and preview-to-registry handoff. Raw replay selects finished, loaded, derived replay, then its registry handoff.

Concrete discrepancy: preview_demands selects nonempty drafts before derived_operation, while close_preview selects derived_operation first. The quoted demand can therefore describe a different action. requires_original_control is an aggregate OR over several later owners, not the actual selected action, and omits other potentially controlled child slots. It must follow precisely the same priority as demand and close.

Current result-only edges discard actual child work: active ticket; derived registry close; plan and draft controlled owners; preview derived operation and operation_close ticket; raw replay owners; factory authority. Snapshot admissions, cancellation, fixed shell and registry handoffs need their own selected work, not a physical-progress proxy. Current erased close adapters infer one work from nonempty successful progress and zero on error, losing reported child work and refused work. Rich outcome migration must propagate original primary/work causes and actual work through every edge and preserve owner and physical receipt on refusal; no narrowing into ValueError.

The owner is exported by store read, admitted into boxed erased ownership by history planning admission, and used through StoreOwners/Store prepare_history_read_retirement and plugin time-travel discard. Public callers consuming erased closure outcomes must preserve rich failure and work; selecting a package that merely depends on the lib does not run its tests. Source-only review; no build.

## Partial migration follow-up

Current preview demand order now matches derived/operation/identity before drafts. Raw replay retirement start is a separate bool-copy turn. Preview guard covers Store-owned history source/input/issuer/issuer-close; replay guard covers replacement wire/target/replacement/failed operation and the corresponding history owners. Plan and replay children have distinct cleanup. Engine upstream retirement methods drain the guarded owners before shell admission. No further concrete priority defect was identified in this partial source pass; original-loan dispatch and runtime verification remain pending.


Current partial implementation aligns selected control order, forwards child rich outcomes and actual work, quotes snapshot source worst-branch work without a fabricated grant, and separates replay-start and terminal factory authority shell turns. It refuses preview/replay shell retirement if original Store-owned history/replacement fields remain. Controlled/original dispatch is authored; replay producer rich-close API, real caller law and compilation are still pending. This file is not source-ready.

## Current integration state

The selected close path now forwards child work and original-control loans, preserves rich retirement failures, and quotes separately moved inline shells and raw replay start. Replay and derived-operation producers are still being migrated by their owner, so this source has not compiled as a complete unit. The existing planner law is mounted in semio-os-store-tests-10; it directly observes physical heap effects but does not yet exercise an OriginalJobFixtureCaller. A genuine original-caller law and native compilation remain required.
