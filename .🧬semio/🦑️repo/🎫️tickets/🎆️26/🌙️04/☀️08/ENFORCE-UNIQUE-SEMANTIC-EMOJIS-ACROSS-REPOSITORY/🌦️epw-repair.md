# EPW Artifact Emoji Repair

## Scope and baseline

This review covers `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw`, its exact Rust inference mount, and its exact Stdio oracle-barrel mount. The pre-repair read-only statute audit found 189 files, 102 audit directories, and 41 findings: 33 missing identities in 11 fixture triplets, four presentation-selector defects, three sibling collisions, and one snapshot descriptor/schema collision.

No automatic naming or rewrite tool, Git mutation, compatibility alias, or payload transformation was used. All 42 physical changes were explicit destination-absent `mv -n` operations.

## Handpicked decisions

The fixture owners now use the same semantic identities as their corresponding, individually reviewed production operations:

- `insert-record → 📥insert-record` and `remove-record → 📤remove-record` distinguish records entering and leaving the weather series.
- `set-comments1 → 💬set-comments1` and `set-comments2 → 🗨️set-comments2` retain the two distinct speech/comment fields.
- `set-data-periods → 📅set-data-periods` and `set-typical-extreme-periods → 📆set-typical-extreme-periods` distinguish ordinary data periods from the separate typical/extreme-period collection.
- `set-design-conditions → 🌡️set-design-conditions` and `set-ground-temperatures → 🌍set-ground-temperatures` identify design weather conditions and ground-temperature records.
- `set-holidays-dst → 🎉set-holidays-dst`, `set-location → 📍set-location`, and `set-record-field → 🎚️set-record-field` identify calendar exceptions, station location, and a selected record field.
- Every operation fixture uses `⬅️before.json` and `➡️after.json` for its input/output pair.

The remaining handpicked changes are `🎚️options → ☑️options` in both UI windows, `🧪️oracle → 🔮️oracle`, `🌡climate → 🌡️climate`, and completed presentation forms for the temperature, field-control, and second-comment mutation owners. The complete-state mutation changes from placeholder `🟤️set-snapshot` to meaningful `📸️set-snapshot`; its sibling schema changes from duplicate `🔣️.schema.json` to `🧬️.schema.json`.

All wire-level semantic IDs remain unchanged. Exact fixture catalog paths, production directory coordinates, descriptor owners/emojis, the snapshot payload-schema authority, Rust `#[path]`/`include_str!` mounts, and inference documentation were repaired to the new physical names.

## Verification

- All JSON files in the EPW subtree parse with `jq`.
- The mutation manifest retains `🔣️.schema.json` for the 11 unchanged operation schemas and points only `set-snapshot` at `🧬️.schema.json`.
- The post-repair read-only ticket audit reports 189 files, 102 directories, and zero `missing`, `generic`, `presentation`, `spacing`, `duplicate`, `multiple`, `reserved-emoji`, or independent-oracle findings.
- The exact central mapping requested from the root lane is owner `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any` to `🔮️oracle`.
- After the root lane added that exact override, `bun nx run @semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.epw` passed with 11 fixtures and 0 file problems.
