# store-tests-06 compile status (2026-11-23)

## Replacement of `arc_custody::admit_shared_retirement`
- The old Arc-in, grant-in admission is gone. Two successors exist:
  - `arc_custody::admit_shared_retirement_into(&mut Option<SharedOriginalStorage<T>>, &mut Option<Box<dyn ErasedSnapshotRetirement>>, effect, reply)` for resident `SharedOriginalStorage` roots (used by `SharedValueRetirementFactory`, plugin admission).
  - For a returned `Arc<T>` root (what `try_admit_one_returned` hands the closure): `semio_framework_value_snapshot_read::admit_arc_lease(&mut Option<Arc<T>>, effect)` (store read retirement), built on `ArcLease` + `controlled::admit_typed_controlled_retirement`. `try_admit_one_returned` now takes `&mut RetainedAllocationEffect` and a closure `(&mut Option<Arc<T>>, &mut effect) -> RetainedWorkOutcome<Option<(R, Progress)>, OriginalRetirementFailure>`.

## Changed
- `🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/🧪️testing/👥️presence-retirement/🦀️.rs`: added `admit_returned_arc` (ArcLease based, same shape as `admit_arc_lease`) and used it in `admit_returned_string`. This library blocked every dependent crate (testkit feature) and now compiles.

## Not done: store-tests-06 itself
After that fix `check --tests -p semio-os-store-tests-06` still fails with 113 errors, all from the same unfinished peer migration (grant -> `&mut RetainedAllocationEffect`, `RetainedWorkOutcome` -> `RetainedWorkPoll`, `close_step` gained an effect parameter, `SnapshotRetirementFactory`/`ArtifactOwnedValueRetirementFactory` gained `*_demands`/`recover_*` and take `&mut Option<..>`, `SharedControlledRetirement::lease` removed, `PersistedDocumentHydrationIdentity` and `ArtifactGroupVisibility::retirement` missing). It is a rewrite of whole test files, not three call sites:
- `🏪️store/🧵️canonical-edit/🧳️source/🧪️tests/🦀️.rs` (BoolFactory/RootFactory traits, `admit_shared_retirement` at line 47, `lease` calls)
- `🏪️store/🧪️tests/🌴️bin-06/🔬️unit/` : dispatch-group-phase-policy-laws, registry-generation-grant-coverage-and-decode-fault-laws, member-publication-denied-axes-and-original-allocation-laws, refused-reload-and-erased-close-demand-laws, replica-identity-genesis-and-actor-ownership-laws, retained-backing-and-terminal-cursor-demand-laws
- `🏪️store/🔗️backbone/✂️detach/🧪️tests/✂️detach/🦀️.rs`
Other `admit_shared_retirement` callers (not in bin-06 errors): store read/retirement unit tests, canonical-edit unit tests, value presence tests, field-clone, flow copy tests, plugin part-7 tests.

Tests were not run (crate does not compile).
