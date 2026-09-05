# Home Directory Projection Persistence

## Outcome

The Home artifact now preserves the complete hub directory projection instead of silently dropping each space's `documents` collection. Corrupt persisted projection JSON is an explicit `Fault`; it is no longer converted into an empty directory.

This is the persistence prerequisite for the bounded authenticated directory event-page work. It does not claim that paging, sealing, replacement, or terminal acknowledgement is implemented.

## Contract

- `DirectorySpaceWire` persists `view`, `members`, and `documents` with unknown fields denied.
- `DirectoryReadModelWire` persists `spaces`, `cursor`, and `users` with unknown fields denied.
- decoding returns `Result<DirectoryReadModel, Fault>`.
- editor, viewer, rename, and fold call sites propagate or contain corruption without inventing empty state.
- the neutral JSON fixture proves a Unicode document identity survives structural round-trip and malformed/missing-field inputs are rejected.

## Permanent Gates

- `@semio-tech/space-plugin:home-directory-projection-persistence-check`
- `@semio-tech/space-plugin:home-directory-projection-persistence-native-check`
- launch entries `⚖️gate📇️home-directory-projection-persistence` and `⚖️gate📇️home-directory-projection-persistence🦀️native`
- exact Rust law `editor::home::config::tests::directory_projection_round_trip_preserves_documents_and_rejects_corruption`

## Evidence

- TDD RED: the independent oracle rejected the previous source because `documents` was absent and corruption used `unwrap_or_default`.
- source/Nx gate: green, 11 checks.
- project JSON: parsed successfully.
- Rust parsing: all four touched Rust files parsed; `rustfmt --check` also reported broad existing formatting drift, which this packet intentionally did not rewrite.
- native exact law: the queued run was cancelled cleanly with exit 130 before discovery because the shared Cargo target remained occupied by unrelated long-running Stdio compilation. No native assertion ran, so this gate remains pending until that external prerequisite clears.

## Files

- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️fixtures/📇️projection-persistence-v1/`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-space/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc`
