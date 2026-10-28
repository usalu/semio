# Retained Apply Audit

Read-only source audit of the native Layout retained central apply and Page cache mounting on 2026-10-21. No Rust compiler or native execution was available. Production files were not modified.

## Findings

- **P1: Missing direct diagnostic dependency.** `diff/🧳️retained/🪡️apply/🦀️.rs` references `semio_framework_diagnostic::Severity::Fatal`; the diff package manifest does not declare `semio-framework-diagnostic`. Rust transitive dependencies are not available by crate name. Add the workspace dependency or use a verified public reexport. Sent to the implementation owner.
- **P1: Child completion can escape an unfinished parent close.** Central `RetainedLayoutApply::close_step` directly returns child close steps; `NestedWork`, `PageWork`, `StoryWork`, `FinalList`, `RootScalars`, and `DataWork` also have direct child returns. A child can emit `Complete` while sibling owners and parent numeric allocations remain. Existing `RetainedCollection` explicitly maps child steps to `Progress`. Normalize nested completion until the parent is terminal, or establish and test a protocol guarantee that callers ignore Complete until terminal emptiness. Sent to the implementation owner.

## Reviewed Properties

The diff crate mounts retained/apply and all eight child modules using existing paths. Page cache preparation retains allocated owners in `ManuallyDrop<Option<Owners>>`; reservation errors preserve that custody. Cache text copy validates borrowed pointer and length after registration, slices at UTF-8 boundaries, and installs via vector swaps so displaced cache owners remain in retirement custody. Final nested row extraction uses allocation-free vacant typed rows and keeps authored rows in explicit owners before mover capture. Nested root changes precede root list movement, so existing row provenance indexes still address original post rows during nested commit. No definite borrow/type mismatch beyond the manifest finding was established by inspection. This does not establish successful compilation.

## Follow-Up

The implementation owner reports normalizing child completion across eight owners and adding a close completion regression assertion. A fresh source reread confirms the direct diagnostic dependency is now declared and central nested close forwarding now maps child steps rather than returning directly. Native compilation remains unverified.
