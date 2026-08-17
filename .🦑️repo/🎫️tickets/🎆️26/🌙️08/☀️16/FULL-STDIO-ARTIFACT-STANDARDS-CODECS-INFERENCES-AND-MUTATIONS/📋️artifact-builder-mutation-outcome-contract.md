# Artifact Builder Mutation Outcome Contract

## Scope

Migrated the framework-owned `ArtifactBuilder::mutate` contract from `(Self, Self::Diff)` to `(Self, protocol::MutationOutcome<Self::Diff>)`. The stdio plugin subtree was deliberately excluded because its owner migrated it in parallel.

## Updated Sources

- 1 framework module: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`.
- 4 framework contract surfaces in that module: the trait, `DerivedArtifactBuilder`, `derive_artifact_facets!`, and the framework test construction.
- 54 non-stdio manual `ArtifactBuilder` implementations across 54 plugin schema component modules.
- 55 Rust source modules changed by this work; this report is the only ticket artifact added.

Every manual builder now computes one `MutationOutcome`, applies `outcome.diff()` to its snapshot, and returns that unchanged outcome. This removes the four prior builder-local `.into_parts().0` diagnostic discards (forms, playbook, present, and sequence).

## Static Verification

- Non-stdio manual `ArtifactBuilder` implementations: 54.
- Non-stdio `MutationOutcome<Self::Diff>` `mutate` signatures: 54.
- Non-stdio legacy `(Self, Self::Diff)` `mutate` signatures: 0.
- Manual builders that both apply `outcome.diff()` and return `(self, outcome)`: 54/54.
- Framework contract signatures using `MutationOutcome<Self::Diff>`: 4/4.
- Direct non-stdio builder callers: 2 (derived wrapper and generated macro); both forward the unchanged outcome.
- `rustfmt --emit stdout` parse audit: 54/54 plugin modules and the framework module parsed successfully.
- `rustfmt --check` was run without writes over all 55 modules: 18 clean and 37 report pre-existing whole-file formatting drift unrelated to these targeted method edits.
- Scoped `git diff --check` over the 55 changed source modules: passed.
- Cargo was not invoked; shared migrations are still in progress and the parent controls serial Cargo scheduling.

## Coordination

The stdio owner confirmed the identical `protocol::MutationOutcome<Self::Diff>` signature before this framework migration landed, then confirmed its separate stdio implementation work remained aligned.
