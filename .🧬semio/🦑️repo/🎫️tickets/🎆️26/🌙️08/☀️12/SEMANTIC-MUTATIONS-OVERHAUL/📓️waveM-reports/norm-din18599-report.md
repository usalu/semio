# Wave M — `norm` / `din18599` / `1` / `any` — mutations facet migration (Job A: from scratch)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Vocabulary derived

`Din18599Snapshot` has 13 persistent fields: 12 flat document-root scalars/enums plus one nested
`climate: MonthlyClimate` struct (two twelve-month arrays, `theta_e_c`/`g_h_w_m2`). Per
derivation-rules rule 1, the 12 scalars each became their own `change-<field>` mutation. `climate`
is this lane's one deliberate `update-<facet>` exception: both arrays are entered together as one
climate dataset (typically loaded via `MonthlyClimate::german_reference(ClimateZoneDe)`), never
meaningfully edited one month/array at a time from this app's input surface — an inseparable
≥2-field facet per the recipe's own worked example, not two independently-set scalars. `SetSnapshot`
is gone with no replacement; `impl_norm_set_snapshot_ops!` removed.

13 mutations: `change-use-class`, `change-heated-area-m2`, `change-occupants`, `change-ht`
(field `h_t`), `change-hv` (field `h_v`), `update-climate`, `change-internal-gains-wm2`,
`change-solar-gains-kwh`, `change-system-losses-kwh`, `change-renewable-kwh`,
`change-annual-limit-kwh`, `change-energy-carrier`, `change-reference-qp-kwh`.

## Directory layout / wiring

13 new triad directories, unique emoji within the facet — `update-climate` was hand-authored (not
generator output) since it's the lane's only multi-field payload:
`UpdateClimate { new_climate: MonthlyClimate }`, `diff()` writes `Din18599Diff.climate` directly
(that field already existed pre-migration as `Option<MonthlyClimate>`), `inverse()` restores
`base.climate.clone()`. Old `📄set-snapshot` deleted outright. `📦️glue.rs`'s
`din18599::…::mutations` block mounts all 13 triads directly.

## OpText/OpBinary + `from_snapshot`

Rewrote the text/binary codec with a handcrafted `Din18599MutationDsl` (the `UpdateClimate` variant
uses `#[dsl(block)]` on its `new_climate: MonthlyClimate` field, matching the snapshot's own
`#[dsl(block)]` convention for the same nested type) + bridge functions. **Self-inflicted bug found
and fixed during this pass**: the generated text codec initially omitted `use
crate::artifacts::din18599::UseClass;`, causing `error[E0425]: cannot find type 'UseClass'` — fixed
by adding the import; confirmed clean on the next targeted read. Added
`Din18599Mutation::from_snapshot(&Din18599Snapshot) -> Vec<Din18599Mutation>` (12 scalar pushes +
one `UpdateClimate`), wired into `import_media`/`🎮️commands/📤️set-snapshot`. `evaluate` returns
`Ok(Emit::default())`.

## Tests

`every_mutation()` (13 entries incl. a full `MonthlyClimate` fixture), semantic-descriptor/round-
trip/`from_snapshot` tests, three law pairs (`update-climate` — the nested facet, `change-use-class`
— enum, `change-heated-area-m2` — f64). Text codec: `every_variant_op_text_round_trips` (13) plus
`op_text_round_trips_change_use_class`/`op_text_round_trips_update_climate` targeted tests.

## `🟦️component.ts` mirrors

All 13 triads: real, non-stub `.ts` mirrors — `update-climate`'s carries a structured
`{ thetaEC: number[]; gHWM2: number[] }` shape, not a bare `unknown`.

## Verification

See lane summary for the combined `cargo check`. Grepped this facet's own tree: zero banned-token
hits outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📙️din18599/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📙️din18599/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Created: 13 triad dirs × 3 leaves × 2 files (`update-climate` hand-authored, the other 12
generated). Rewrote: `🧬️mutations/🦀️component.rs`, `📝️text/🦀️component.rs`. App files rewritten.
Deleted: `📄set-snapshot/**`.
