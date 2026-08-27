# Wave M — `norm` / `en1999` / `1` / `any` — mutations facet migration (Job A: from scratch)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Vocabulary derived

`En1999Snapshot` is a flat, id-less, document-root parameter form: 26 persistent scalar/enum fields
(aluminium design actions/resistances, fatigue, weld, sheet and shell parameters) for the EN 1999
check — no id-keyed collections, no name/identity field. Every field became its own `change-<field>`
mutation; none qualified for `update-<facet>`. `SetSnapshot` is gone with no replacement;
`impl_norm_set_snapshot_ops!` removed.

26 mutations: `change-a-mm2`, `change-alloy`, `change-annex`, `change-beta-w`, `change-chi`,
`change-delta-sigma-c`, `change-delta-sigma-ed`, `change-fatigue-m`, `change-i-t-mm4`,
`change-l-cr-mm`, `change-m-ed-knm`, `change-n-cycles`, `change-n-ed-kn`, `change-sheet-bm-mm`
(field `sheet_b_mm`), `change-sheet-k-sigma`, `change-sheet-m-ed-knm`, `change-sheet-tm-mm`
(field `sheet_t_mm`), `change-sheet-w-el-mm3`, `change-shell-rm-mm` (field `shell_r_mm`),
`change-shell-tm-mm` (field `shell_t_mm`), `change-sigma-ed-shell-mpa`, `change-theta-c`,
`change-v-weld-ed-kn`, `change-weld-length-mm`, `change-weld-throat-mm`, `change-w-el-mm3`.

## Directory layout / wiring

26 new triad directories, unique emoji within the facet. Old `📄set-snapshot` deleted outright.
`📦️glue.rs`'s `en1999::…::mutations` block mounts all 26 triads directly.

## OpText/OpBinary + `from_snapshot`

Rewrote the text/binary codec with a handcrafted `En1999MutationDsl` + bridge functions. Added
`En1999Mutation::from_snapshot(&En1999Snapshot) -> Vec<En1999Mutation>`, wired into
`import_media`/`🎮️commands/📤️set-snapshot` (renamed `SetSnapshot`→`ReplaceSnapshot`). `evaluate`
now returns `Ok(Emit::default())`.

## Tests

`every_mutation()` (26 entries), semantic-descriptor/round-trip/`from_snapshot` tests, three law
pairs (`change-annex` — enum, `change-n-ed-kn` — f64, `change-alloy` — String). Text codec:
`every_variant_op_text_round_trips` + 3 targeted tests.

## `🟦️component.ts` mirrors

All 26 triads: real, non-stub `.ts` mirrors.

## Verification

See lane summary for the combined `cargo check`. Grepped this facet's own tree: zero banned-token
hits outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1999/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1999/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Created: 26 triad dirs × 3 leaves × 2 files. Rewrote dispatch/text/binary/diff-text-test files and
the three app files. Deleted: `📄set-snapshot/**`.
