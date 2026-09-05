# MP4 Emoji Repair

## Scope

Hand-reviewed `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4` only. The ISO BMFF `✳️any` subset remains the established semantic owner; conventional reserved names were not renamed.

## Baseline

- 186 files and 97 directories.
- 32 statute findings: 24 missing emojis across eight fixture triplets, two generic options directories, one generic oracle directory, four incomplete presentation selectors, and one snapshot schema sibling collision.
- One additional generic snapshot operation directory was repaired during content review, for 33 physical moves total.

## Handpicked decisions

- `🎚️options → ☑️options`: selectable editor/viewer configuration.
- `🧪️oracle → 🔮️oracle`: expected-result authority.
- Fixture operations: `🧱insert-sample`, `➕insert-track`, `🗑️remove-sample`, `➖remove-track`, `🏷️set-ftyp`, `⭐set-sample-sync`, `🎛️set-track-codec`, and `📐set-track-dimensions`.
- Every fixture pair uses `⬅️before.json` and `➡️after.json`.
- `⏱duration → ⏱️duration`: completed the selector presentation without changing its identity.
- Production operation directories were completed as `🎛️set-track-codec`, `🏷️set-ftyp`, and `🗑️remove-sample`.
- `🟤️set-snapshot → 📸️set-snapshot`: snapshot capture, replacing a generic color marker.
- Snapshot `🔣️.schema.json → 🧬️.schema.json`: schema identity, unique beside its descriptor and implementation siblings.

## Exact reference repairs

- Updated MP4 oracle manifest fixture coordinates.
- Updated the MP4 aggregate Rust mounts for the four changed production operations and the snapshot test.
- Updated the Stdio oracle barrel and main Rust barrel mounts.
- Updated inference documentation for `⏱️duration`.
- Updated all four affected mutation descriptors, including owner, emoji, and the snapshot payload schema coordinate.

## Verification

- All scoped JSON documents parse.
- Exact stale-coordinate scan found no old MP4 path references.
- The scoped post-repair audit found 186 files, 97 directories, and zero missing, generic, presentation, spacing, duplicate, multiple-emoji, reserved-name, or oracle findings.
- `bun nx run @semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.mp4` passed: 8 fixtures, 0 file problems.

## Remaining issues

None within the MP4 subtree. Repository-wide findings outside this scope remain owned by their respective repair lanes.
