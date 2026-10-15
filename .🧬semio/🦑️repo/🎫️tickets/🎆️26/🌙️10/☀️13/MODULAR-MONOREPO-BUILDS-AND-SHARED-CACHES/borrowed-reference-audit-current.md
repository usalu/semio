# Borrowed Reference Read-Only Audit

Audited 2026-10-15. Source was read only; no compiler, build or test was run for this audit. Only this requested ticket report was written. Concurrent unrelated refactors were excluded.

## Result

No concrete implementation regression found in the borrowed-owner extraction or admission rewiring. One meaningful native integration validation gap remains, already acknowledged by the owner report: the changed owned `ArtifactRef::borrowed()` implementation and its seven real callers have not yet been compiled together. The tiny-owner/projection native passes cannot establish that integration result.

## Verified From Current Source

- Admission's physical Cargo manifest has precisely two production dependencies: `semio-framework-schema-composition` and `semio-framework-artifact-reference-borrow`. Both dependency owners have empty production dependency tables. Thus this production closure has three crates total, no owned reference/value/registry/codec/Kernel/Store edge, and no cycle or backedge. Dev-only `serde_json` is separate.
- Owned reference's physical manifest directly declares the borrowed owner. Owned Source explicitly reexports the canonical borrowed type and predicate. The new accessor borrows artifact ID, artifact kind, standard and subset directly from their original owned fields.
- The extracted Rust predicate is textually identical to the removed owned predicate and helper in the current Git diff. It still requires exactly three dot-separated ASCII lowercase/digit kebab segments, rejecting empty segments, edge hyphens and doubled hyphens. No grammar broadening found. The independent Ajv oracle also checks the final newline case.
- Membership checks exact slot plus all four borrowed target fields. It intentionally does not conflate child ID with artifact ID. The projection native test changes each target field separately and rejects each substitution; the architect fixture explicitly carries independent child `a` and target `b` identities.
- Repository Rust search finds seven real `admits_member` call sites outside the admission test. Every one now supplies `target.borrowed()` (writer, reasoning wires, playbook artifact, playbook editor, Trinity graph, architect snapshot and DIN18599). Callers use the owned crate's public return type, so they do not themselves require a direct dependency on the borrowed crate to call this method.
- Borrowed-owner metadata declares 32KiB/four Source files/400 active lines. Admission declares 64KiB/eight files/1000 lines. The borrowed owner's actual library is one short dependency-free file; its contract test checks byte size and manifest bounds. Framework workspace and build-owner registrations are present. Project targets call their local script, expose prototype/release configurations, and have no inline permanent build implementation. The local script uses the existing repository Cargo admission route. Ticket headroom routes and editor seed contain borrowed contract/type/compiler routes for both modes.

## Evidence Boundaries and Next Meaningful Check

The current owner report records actual prototype borrowed-owner native two-test success (92154), projection three-test success (42518), and release successes 75775/58047 with the same two/three counts. It also records successful independent Bun contracts and type checking. These are retained owner outcomes, not test executions independently repeated by this audit; settled raw transcripts were deleted by the owner. The report's final release update supersedes its earlier statement that release successors were running.

The native floor test constructs `BorrowedArtifactRef` directly; the projection native test likewise constructs that type directly. Neither executes `ArtifactRef::borrowed()`, because deliberately compiling the owned crate expands beyond this floor. The next meaningful check is an actual owned-reference native test calling `borrowed()` with four distinct owned strings and checking values and original string addresses, followed by the affected real caller tests. Existing floor/projection success should not be represented as owned-reference or Kernel/Plugin compile success. This is a validation gap, not evidence that the inspected accessor is wrong.

No Source changes or extra compiler runs are warranted merely to add redundant grammar or membership tests: the present neutral corpus, independent oracle, field-substitution cases, exact reference and byte bounds, and sparse walk boundary already exercise the relevant floor behavior.
