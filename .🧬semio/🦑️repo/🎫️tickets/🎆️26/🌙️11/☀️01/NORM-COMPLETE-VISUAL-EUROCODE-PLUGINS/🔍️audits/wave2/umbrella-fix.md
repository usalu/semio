# 🧩 Umbrella Fix (wave 2)

Source: `🔍️audits/wave2/rule-guard.md` Pass 15, check 13, routes "from: rule-guard pass 15" in `📌️important/requests.md`.

## Findings

- The shared norm crates build and test through `@semio-tech/norm-artifact-contract-rs`, whose umbrella is `📇️registry/🧬️contract/📦️packages/🦀️rust/📜️script.ts`.
- That umbrella called `runArtifactRustPackageMain` with only `commands`, so it ran the single crate `semio-s-artifact-norm-contract` and no sibling crate.
- No compliance or structural-scene umbrella exists on disk. The `🗿️artifacts/*/📦️packages/🦀️rust/📜️script.ts` umbrellas with a `packages` list (for example `🏋️en1991`) are artifact-only, so the contract umbrella is the registration point for the shared crates.
- The 26 crates are all members of the norm workspace `✏️s/🔌️plugins/📕️norm/Cargo.toml` (`check-cargo-authority.py`: 28 crates, 28 members, 0 errors).

## Changes

1. `📇️registry/🧬️contract/📦️packages/🦀️rust/📜️script.ts`: `runArtifactRustPackageMain` now receives `packages` (same shape as `🏋️en1991`, main crate included first). Added 26 names:
   - 12 ndp: `semio-s-plugin-norm-ndp`, `-ndp-model`, `-ndp-en1990` … `-ndp-en1999` (10 tables).
   - 2 compliance: `semio-s-plugin-norm-compliance`, `-compliance-core`.
   - 2 config: `semio-s-plugin-norm-results-config`, `semio-s-plugin-norm-viewport-config`.
   - 3 structural-scene: `semio-s-plugin-norm-structural-scene`, `-structural-scene-meshing`, `-structural-scene-foundation`.
   - 7 app-surface: `semio-s-plugin-norm-app-surface`, `-app-surface-foundation`, `-app-surface-live-core`, `-app-surface-live-editor`, `-app-surface-visual-chrome`, `-app-surface-panels`, `-app-surface-viewport3d`.
   - Total list: 27 names (contract + 26), all unique.
2. `📇️registry/🧬️contract/📦️packages/🦀️rust/📋️project.json`: added `{workspaceRoot}/✏️s/🔌️plugins/📕️norm/🎬️structural-scene/**/*` to `nativeSources`. The structural-scene crates were not covered by the cache inputs of the contract project.

## Verification

- `python3 -I 🧰️scripts/check-cargo-authority.py`: `checked 28 crates, 28 members, 0 errors`, exit 0.
- `SEMIO_LANGUAGE=en repo ./📜️script.ts verify build-owners sync`: `removed=0 added=0 renamed=0 relocated=0 repackaged=0`, passed, exit 0.
- No cargo, rustc or build was run. No git writes.

## Open

- Build of `@semio-tech/norm-artifact-contract-rs` now compiles 27 crates. Requested via `🗑️generated/integrate/latest.md` ("ready for rebuild: umbrella fix").
- The scanner (`pass15-scan.py`) check 13 still reports the 19 crates as having no umbrella, because it looks only for `packages` lists in separate umbrellas. Its result should be read against the contract umbrella.
- The `🎬️structural-scene` `nativeSources` line is an addition beyond the packages list. Owner should confirm.
