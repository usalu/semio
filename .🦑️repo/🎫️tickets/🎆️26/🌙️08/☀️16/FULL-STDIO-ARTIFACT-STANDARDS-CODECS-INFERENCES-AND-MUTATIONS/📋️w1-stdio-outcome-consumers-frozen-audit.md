# Frozen Stdio Mutation Outcome Consumer Audit

## Scope

Read-only audit of the exact non-GLTF stdio files named by `📋️w1-stdio-test-patch-independent-audit.md` and `📋️w1-stdio-outcome-test-geometry-media.md`. The audit did not inspect glTF or active runtime-writer files and did not run Cargo or Nx.

The two reports cover 19 and 14 changed files respectively, for 33 unique files. The XML path in the first report contains an extra variation selector (`📰️xml`); the repository path is `📰xml`. The audit normalized that spelling only for read-only inspection.

## Verdict

No P0 or P1 outcome-adoption defect was found in the assigned files. The frozen source preserves `MutationOutcome` values, projects `.diff()` only at raw diff algebra boundaries, and contains no detected template corruption or declaration/test loss.

This is a static/readiness verdict only. Cargo/Nx execution remains outside this audit and is not implied by the checks below.

## Evidence

- All 33 normalized paths exist and are non-empty.
- `rustfmt --edition 2021 --check`: 33/33 passed.
- `git diff --check` over the exact 33 paths: passed.
- Scans over the exact paths found zero `${...}` artifacts, literal `++` path markers, merge-conflict markers, and double-outcome `.diff().diff()` chains.
- No legacy `fn diff(...) -> Self::Diff` or `ArtifactBuilder::mutate(...) -> (Self, Self::Diff)` signature remains in the exact paths.
- No `MutationOutcome` compatibility implementation or external-type passthrough implementation is present in the exact paths.
- Every inspected public `apply_*_mutation` helper follows the same outcome-preserving shape: compute one outcome, apply `outcome.diff()`, return the original outcome.
- Semio any-level dispatch maps child outcomes through `MutationOutcome::map`; the map implementation retains the child message vector. The wrong-kind branch returns a typed `mutation.target-kind-mismatch` error outcome, and its test asserts the diagnostic.
- `.diff()` calls in the inspected tests are confined to applying, inverting, absorbing, encoding, or checking the underlying diff. Raw `Diff::between` values are used directly and were not given an erroneous second `.diff()` call.
- Test-attribute counts are unchanged in all 33 files versus `HEAD`. One private nested `find_mut` helper count decreases in IFC 4 because the imperative apply path was replaced by the diff-driven path; no public or test declaration was lost.

## Findings

### P0

None.

### P1

None within the assigned outcome-consumer scope.

### P2

- The active geometry/media report has a machine-visible path spelling mismatch for XML (`📰️xml` versus the repository's `📰xml`). This did not hide a source file after normalization, but should be corrected before reports are consumed as manifests.

## Broad-plan Carryovers

The inspected files still contain pre-existing `NoMutation` vocabulary and several missing-target branches that return an empty outcome rather than a typed rejection. Those are broader mutation taxonomy/rejection-policy work and were not changed or counted as outcome-adoption regressions in this read-only shard.

## Bounded Compiler Fix

The parent reported five current PDF 1.7 test errors and six current SVG 1.1 test errors. Only these two exact files were edited:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

The PDF field-sweep assertions now access `PdfDiff` fields through `.diff()`. SVG absorb-law calls now pass underlying diffs, the exact-native absorb test clones underlying diffs before `absorb`, and the set-snapshot codec test retains the outcome before applying its underlying diff. Residual targeted-pattern scan is empty.

`rustfmt --edition 2021 --check` and `git diff --check` both pass for the two files. Cargo/Nx was not run. Source is frozen pending the parent-controlled compiler gate.
