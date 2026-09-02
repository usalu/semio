# Sequence Structural Catalog Merge

## Scope

Repair `direct_owners_descriptors_surfaces_and_catalog_correspond` so it reads the post-split sequence mutation catalogs from their physical subset owners and exercises all eight mutation assertions.

## Findings

- The removed lookup `✳️any/🔣️oracle.json` named a nonexistent file and panicked before the per-mutation assertions.
- The authoritative manifests are `✳️step/🧪️oracle/🔣️.json` with six kinds and `✳️dependency/🧪️oracle/🔣️.json` with two kinds.
- Concatenating the two catalog kind arrays does not preserve the aggregate enum's declaration order because `duplicate-step` belongs to the step catalog but follows the dependency variants in `SequenceMutation`.
- Sorting both vectors before equality keeps the comparison order-independent and duplicate-sensitive.
- The merged catalog vector list supplies fixtures for all eight per-mutation checks.

## Implementation

- Derive the step and dependency subset roots from one standards-level subsets root.
- Read and parse both `🧪️oracle/🔣️.json` manifests.
- Flatten all `mutationCatalogs`, `kinds`, and `vectors`.
- Sort descriptor and catalog kind vectors before asserting equality.

## Verification

- `git diff --check`: passed.
- `cargo test -p semio-s-plugin-sequence structural_correspondence_tests -- --nocapture`: the first foreground run waited on an existing workspace build lock and was stopped after more than ten minutes.
- The isolated foreground retry reached dependency compilation but initially stopped before compiling the sequence crate because the concurrently modified `semio-framework-ui-contract` could not resolve `::protocol` in `🦀️action.rs` (nine E0433 errors).
- After that concurrent UI-contract blocker was repaired, the warmed isolated foreground retry compiled it successfully.
- The retry then stopped before the sequence crate because the concurrently modified core `ProgramContributionEntry` no longer implements `serde::Serialize`, while `semio-s-imperative` still passes `&[ProgramContributionEntry]` to `serde_json::to_string` (one E0277 at `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️.rs:204`).
- Focused test result: pending removal of the unrelated concurrent imperative compile blocker.
- The 2.3 GiB isolated Cargo target created under `🗑️generated/sequence-structural-target` was deleted after the attempts.
- A Bun read-only manifest check loaded both exact paths and confirmed catalog ids `sequence-1-step` and `sequence-1-dependency`, eight combined kinds, eight combined vectors, and identical sorted kind/vector mutation ids.
- `rustfmt --check` parsed the edited source and reported only pre-existing formatting differences outside the scoped block after the new catalog expression was aligned with its suggested form.
