# AVI Hand Repair

## Scope and method

Reviewed `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi` manually by sibling group. Exact `mv -n` operations were used only after selecting the semantic identity for each physical node. No generated rename plan, normalization writer, palette, hash, Git mutation, or compatibility alias was used.

## Baseline

The scoped ticket audit reported 228 files, 129 directories, 57 missing identities, 4 presentation-selector findings, and 7 sibling-duplicate findings. The findings were concentrated in three subset roots, three oracle roots, two options roots, four selector-deficient semantic owners, and 22 fixture cases with 35 unprefixed binary leaves.

## Handpicked decisions

- `✳️hdrl -> 🎛️hdrl`: AVI header/control records.
- `✳️idx1 -> 📇️idx1`: the stream/chunk index.
- `✳️movi -> 🎞️movi`: interleaved media chunks.
- Each subset's `🧪️oracle -> 🔮️oracle`; `🎚️options -> ☑️options` beside the distinct `🎚️config` owner.
- `⏱duration -> ⏱️duration` retained the duration meaning and corrected emoji presentation.
- Production operations were handpicked as `📥️insert-stream`, `📤️remove-stream`, `🎞️set-stream-header`, `📇️set-idx1-present`, `🗑️remove-chunk`, and `📸️set-snapshot`. Existing meaningful, unique operation identities such as `🎨set-stream-format`, `🎬set-main-header`, `🔑set-chunk-keyframe`, `🧩insert-chunk`, `🧱add-unknown-chunk`, and `🧹remove-unknown-chunk` were preserved.
- Applied fixture owners use their operation meaning; rejected cases use distinct `⛔️`, `🚫️`, `❓️`, `⚠️`, or `🔍️` identities matching boundary, unavailable-target, or missing-target outcomes. The identity case is `⏸️no-mutation-applied`.
- Every binary fixture input/output is now exactly `⬅️before.avi` or `➡️after.avi`.

All 22 public fixture IDs and mutation IDs remain unchanged. The generator now carries an explicit, reviewed `id -> directoryName + subset` authority instead of deriving a physical basename from a wire ID. Its codec writes directly to the declared fixture directory and declared emoji leaf names.

## Consumers repaired

- AVI descriptor owners and emoji fields, Rust mutation mounts, inference documentation, generator/probe commands, and all three oracle manifests.
- Stdio's main Rust barrel, oracle barrel, native codec registry, and protocol registry.
- The standalone RIFF AVI codec's internal build interface and output leaf names.
- One exact BMP documentation reference to the AVI codec.
- Root dependency and plugin-policy coordinates, plus exact oracle overrides for `🎛️hdrl`, `📇️idx1`, and `🎞️movi`, were coordinated with the root owner.

Whole-workspace stale-coordinate searches found no live non-ticket AVI reference to the old subset, oracle, options, operation, fixture-directory, or fixture-leaf identities.

## Verification

- Every AVI JSON file parsed successfully.
- Final scoped audit: 228 files, 129 directories, 350 governed nodes, zero findings in every statute category.
- `bun nx run @semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.avi`: 22 fixtures, 0 file problems; Nx target passed.
- Isolated generator execution under the ticket produced 22/22 manifests and 35 binary fixture leaves at the new coordinates.
- All 35 generated binaries were byte-for-byte identical to the committed source fixtures.
- The isolated generated output was deleted after comparison; no generated build product was left in the source tree.

Literal Cargo files and `src/main.rs` remain conventional reserved names.
