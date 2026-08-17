# MutationDiff Result Migration — Stdio Document and Media Slice

## Scope

This source-first handoff covers PDF 1.7, DOCX ECMA-376, PPTX ECMA-376, and XLSX ECMA-376. It excludes glTF, Semio, and the geometry/CAD families as requested. Cargo/Nx were not run because the runtime host owns the serialized build lane.

## Changes

- Each owned `MutationDiff::apply` now returns `MutationApplyResult<Snapshot>`.
- PDF validates index/key target existence, duplicate/conflicting edits, final insertion positions, recursive value kind compatibility, nested dictionary/array/object edits, and stages the candidate after validation. The prior PDF native transport and sparse-state preservation fixes remain intact.
- DOCX/PPTX indexed and named collection engines reject missing, duplicate, conflicting, and out-of-range persisted edits; nested kind mismatches are typed; OPC relationship/content-type/part paths are prefixed through nested errors; relationship owner removals, modifications, and additions are fully preflighted before candidate mutation; the candidate snapshot is cloned before mutation.
- XLSX named sheet/cell/shared-string/OPC collection engines and relationship owners reject invalid persisted targets and return typed errors; candidate state is staged in the top-level apply.
- PDF/DOCX/PPTX/XLSX mutation consumers now preserve the typed rejection as an empty-diff `MutationOutcome::error` with the original outcome diagnostics, and only commit a successful candidate.
- Added absorb-only helpers where the fallible apply callbacks cannot be used by the framework's total `absorb` contract; these helpers do not participate in persisted apply and are not compatibility shims.

## Static verification

- `rustfmt --edition 2021` completed on all owned diff/mutation Rust leaves plus the PDF native I/O test consumer.
- `git diff --check` passed for all owned files.
- Exact owned trait inventory reports four migrated implementations and zero legacy return signatures in the four owned artifact revisions.
- Mutation-law and native-I/O test consumers now unwrap only valid-result paths; invalid-target adversarial tests assert typed rejection and unchanged base snapshots.
- Cargo/Nx intentionally not run under the runtime-host serialization rule.

## Pending integration

The remaining stdio artifact families are outside this lane and must be migrated by their owning shards. Runtime validation remains pending the serialized Cargo/Nx lane; this handoff is source-stable for that gate.
