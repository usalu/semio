# Retained Post Call-Site Completion — 2026-10-28

Actual proctor20245 terminated1 with two sequence E0308 errors: typed preflight now requires the original RetainedCloneRef<P>, while the sequence passed bare &P. Backend owns the sequence/canonical handback implementation, mounted sequence law and shared PostFixture import correction. This report covers the independently reviewed external callers and the subsequently assigned mounted root test setup. No Cargo process was launched here.

## External Production Callers

Full repository Rust preflight call/signature inspection found two additional concrete production mismatches in Energy:
- PreparedEnergyBranchesApply still declared bare &EnergyModelState post parameters for the RetainedDiffApply preflight methods.
- EnergyDocumentApply still declared bare &EnergyModelSnapshot and forwarded &p.model to its typed child.

Both now accept the actual retained post loan required by the canonical trait. Document forwarding uses p.project(0, |p| &p.model), matching the actual child projection of the diff and base in both quotation and execution. No cold reconstruction, new owner alias or fabricated retained reference is involved. Commit remains the canonical mutable &P/&mut P interface; it was not mechanically changed.

Other similarly named layout helpers intentionally accept borrowed rows, scalar indices or ordinary read views inside their own local algorithms. Their wrappers already receive genuine retained post references and extract these views locally. They were inspected and left unchanged. OS mutation application forwards its actual post.original() loan; IDML/drawing typed implementations already use the canonical retained type.

## Existing-Law Test Ownership

Energy's half-migrated tests contained two unconditional central_zone definitions and a generic bare-&mut-P preflight helper. Consolidated them into the one generic existing-law helper with PostFixture<P>. Actual projection outputs enter through PostFixture::projected, which uses the canonical funded placement; preflight borrows its genuine post loan; commit requests the canonical funded mutable handback. Changed owners return through into_inner and paid source/sealed-owner constructors. Original pointer/identity, every denied currency, physical birth/release, sparse forward/inverse and terminal closure assertions remain. No clone-based post proxy or raw source-reference constructor was added.

The Energy test root already imports SnapshotProjection from semio_framework_value_snapshot_projection::snapshot_projection. Its schema Cargo manifest lacked that direct internal dependency, so added the explicit canonical local package path. No compatibility re-export or obsolete value::snapshot_projection facade was introduced. Backend separately corrected the shared PostFixture helper's four old paths.

After53462 revealed two concrete mounted root test errors, the parent assigned retained/tests to this agent. Its existing owned Projection setup now uses PostFixture::new(Projection{...}); the preflight helper's borrow and commit mutable_read therefore refer to that actual owned fixture. Its untouched-vector pointer and existing decision/inverse/commit assertions remain unchanged.

## Evidence and Limits

Retained retained-post-oracle.mjs independently applied all four existing Energy forward/inverse fixture cases through fast-json-patch and Lodash; actual managed Bun output was `[DEBUG] {"energyForwardInverseCases":4,"originalInputUnchanged":true}`. The oracle preserves the original input and compares unchanged semantic entities by their authored IDs after index shifts. Its initial same-array-index assumption failed on the authored remove case; that diagnostic was a comparison mistake, not a source/fixture failure. The corrected oracle tracks the original entity identity. It does not claim native pointer or owner-provenance proof.

Backend mounted sequence run12080 stopped at stale PostFixture imports before execution. After its canonical import repair,53462 stopped at the root test setup errors described above. Backend then confirmed authoritative80829 terminal0: one actual mounted ordered sequence law passed, zero failed,18 filtered. This demonstrates that mounted sequence law and compilation of the root setup, not runtime execution of every retained test or the external Energy tests.

Sole proctor release41361 is now live. External Energy production/test candidate is stable; no heavy gate or additional Cargo job was launched. Exact separate owning Rust test commands are retained in the completion ledger and remain pending. All frontend semantic/unit/watcher/full-browser and persistent runtime requirements remain unchanged.
