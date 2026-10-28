# Energy Mutation and Diff Audit

## Current Infrastructure

The current native implementation has 306 authored mutation diff leaves and 306 inverse mutation leaves. No mutation diff leaf clones `base.model`. The October 8 audit describing whole-model replacement diffs is stale.

`EnergyModelDiff` carries a sparse typed `ModelPatch` and two explicit attached/detached link deltas. Its central cold applier prepares every affected model field and both links before installing anything. The patch macro prepares changed fields through `PreparedField`; existing late-field refusal tests verify unchanged original snapshots and allocations. Leaf inverse mutations read the original entity fields and return semantic operations; diff algebra inverses separately restore affected base fields, keyed rows, lists, and links.

TypeScript exports the same sparse schema and three-state option/link representations. It does not implement a separate TypeScript mutation executor in the artifact entrypoint, so these fixes belong in the native central path. Bridge/runtime parity still requires integration execution.

## Corrections

1. Central keyed collection application previously allowed inserted entities to duplicate surviving or newly inserted stable keys. `Rows::commit_onto` now checks the resulting keys and refuses collisions. Removing and reinserting an entity with the same key remains valid. New tests assert refusal is atomic even when the same diff also renames another entity, and legal replacement inverts exactly.
2. `Slots::absorb` previously binary-searched slots received from the wire without validating their ordering. A decoded unsorted earlier patch could override a later assignment with an old value. Absorption now collects assignments by index with the later value winning and emits sorted unique slots.

## Validation

- Added shared language-neutral `identity.json` (five cases) and `slots.json` under the existing diff unit-test directory.
- Added native regression tests for duplicate stable IDs, valid identity-preserving replacement, language-neutral matrix apply/refusal/inverse, and decoded unordered slot composition.
- Executed `energy-identity-oracle.py` successfully with Python's SQLite library, backed by SQLite 3.53.3. SQL unique constraints independently reproduce all five accepted/refused row outcomes; SQL upsert independently reproduces later slot assignment wins.
- Invoked the shared `📜️energy-cargo.py test --manifest-path` energy-model workspace `-p semio-s-artifact-energy-model-diff --lib`. The first attempt exited 101 before compiling the energy tests because the framework UI styling package lacks generated `🔤️tokens/🦀️.rs`. The parent coordinates the actual owner generator; native tests remain unverified at this point.

## Remaining Verification

Run the native diff package unit tests and mutation corpus once the shared Rust toolchain is ready. Verify production editor history uses the central native path for semantic inverse operations. Full Energy simulation, IDF compliance, editor controls, and end-to-end runtime completeness are outside this mutation subtask and remain part of the parent integration audit.

## Native Compilation Follow-up

After the parent generated the missing UI styling artifacts using their owner generator, the second invocation exposed two shared protocol-law helper errors from the cold diff trait split. The helper called `absorb` while importing/bounding only `MutationDiff`. Updated the helper import and canonicalization bound to `ColdMutationDiff` in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🧮️operation-and-mutation-laws/🦀️.rs`. This is a minimal test-infrastructure correction required by energy's inverse-law tests.

The third invocation passed that point and failed in the concurrent shared `semio-framework-os-store-replay` dependency with 23 existing API drift errors involving retained projection APIs, close demand method names, projection associated output types, and replay signatures. No changes to those replay internals were made by this lane. Parent integration owns coordination of the shared compile repair. The energy regression tests remain authored but not yet executed; the independent SQLite oracle passes.

## Native Production Patch Harness: Passing

Ran persistent `energy-patch-native.py` successfully (exit 0). It creates an isolated generated Cargo workspace whose library path points directly to the actual production energy `🩹️patch/🦀️.rs`, and whose splice dependency points to the production splice package. Its only dependency-name substitution resolves the wide OS kernel import directly to the actual lower-level `semio-framework-mutation-diff` owner of `MutationApplyError` and `PreparedField`; no mock protocol types or source copies are involved. All other framework value/codec crates remain the production packages.

The durable harness `energy-patch-native.rs` uses a minimal generic keyed row to test the production generic collection vocabulary, avoiding the currently broken unrelated OS replay dependency. Four tests pass:

- All five language-neutral identity fixtures: legal insertions and identity-preserving replacement apply and invert; duplicate surviving/inserted keys refuse.
- The language-neutral unordered slot fixture honors the later assignment, emits sorted unique indices, and inverts exactly.
- A rename folded into a newly inserted keyed entity survives composition and inverts exactly.
- An out-of-range fixed-array slot refuses.

This verifies the changed production generic patch algorithms natively. It does not replace the full energy artifact central-applier atomicity tests, which still need the shared store/replay compilation fixes before they can execute.
