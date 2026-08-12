# Wave M — `norm` / `en1997` / `1` / `any` — mutations facet migration (Job A: from scratch)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Vocabulary derived

`En1997Snapshot` is a flat, id-less, document-root parameter form: 22 persistent scalar/enum fields
(shallow-footing and pile geotechnical design actions, resistances and ground parameters) for the
EN 1997 check — no id-keyed collections, no name/identity field. Every field became its own
`change-<field>` mutation (derivation-rules rule 1); none qualified for `update-<facet>` (no
documented inseparable multi-field group). `SetSnapshot` (sole pre-migration variant) is gone with
no replacement; the old `impl_norm_set_snapshot_ops!` macro call is removed with it.

All 22 mutations: `change-alpha-s`, `change-annex`, `change-bm`, `change-c-kpa`, `change-design-
approach`, `change-dfm`, `change-es-mpa`, `change-footing-area-m2`, `change-gamma-kn-m3`,
`change-h-ed-kn`, `change-nu`, `change-n-pile-ed-kn`, `change-phi-deg`, `change-pile-base-area-m2`,
`change-pile-dm`, `change-pile-lm`, `change-pile-n-profiles`, `change-qb-kpa`, `change-qs-kpa`,
`change-settlement-limit-mm`, `change-v-ed-kn`, `change-z-investigated-m`. `kind`s were computed
with the derive's own `to_kebab` algorithm (verified by hand against several irregular-acronym
fields — `b_m`→`change-bm`, `d_f_m`→`change-dfm`, `e_s_mpa`→`change-es-mpa`, `pile_d_m`→
`change-pile-dm`, `pile_l_m`→`change-pile-lm` — no lowercase letter separates the merged single-
letter segments, matching the algorithm's own boundary rule).

## Directory layout / wiring

22 new triad directories, unique emoji within the facet (offset slice of the shared 150-emoji pool).
Old `📄set-snapshot` deleted outright (no repurposing needed — this lane's agent owns `📦️glue.rs`).
`📦️glue.rs`'s `en1997::…::mutations` block mounts all 22 triads directly as `component`-sibling
modules.

## OpText/OpBinary + `from_snapshot`

Rewrote `🧬️mutations/📝️text/🦀️component.rs` / `💾️binary/🦀️component.rs` with a handcrafted
`En1997MutationDsl` + `OpText`/`OpBinary` bridge (`din16798` pattern). Added
`En1997Mutation::from_snapshot(&En1997Snapshot) -> Vec<En1997Mutation>`, used by
`import_media`/`🎮️commands/📤️set-snapshot` (payload struct renamed `SetSnapshot`→`ReplaceSnapshot`).
`🎮️commands/🧮️evaluate/🦀️component.rs` now returns `Ok(Emit::default())` (no persisted state to
re-commit).

## Tests

Extended the existing `🧪️Tests` region: `every_mutation()` (22 entries), semantic-descriptor +
round-trip-via-inverse + `from_snapshot` round-trip tests, three
`assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` pairs (`change-annex` — enum,
`change-v-ed-kn` — f64, `change-design-approach` — String). Text codec got
`every_variant_op_text_round_trips` plus targeted tests.

## `🟦️component.ts` mirrors

All 22 triads: real `interface`/type-alias `.ts` mirrors, no `export {};`.

## Verification

See `📓️waveM-reports/norm-lane-summary.md` for the combined `cargo check -p semio-s-plugin-norm`
output covering the whole lane. Grepped `🗿️artifacts/📘️en1997/**` + `🎛️apps/📘️en1997/**` for the
banned tokens: zero hits outside the out-of-scope `En1997Command::SetSnapshot` app-command variant
name (manifest action id unchanged).

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1997/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1997/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Created: 22 triad dirs × 3 leaves × 2 files. Rewrote: `🧬️mutations/🦀️component.rs`, `📝️text/🦀️component.rs`,
`💾️binary/🦀️component.rs` (tests), `🔺️diff/📝️text/🦀️component.rs` (test fix). App files rewritten
per facet as above. Deleted: `🧬️mutations/📄set-snapshot/**`. Plugin-shared files (glue.rs, app-surface,
📄️artifact macro removal) are covered once in the lane summary.
