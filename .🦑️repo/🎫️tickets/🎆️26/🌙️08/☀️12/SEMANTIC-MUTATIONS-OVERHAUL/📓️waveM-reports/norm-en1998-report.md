# Wave M — `norm` / `en1998` / `1` / `any` — mutations facet migration (Job A: from scratch)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Vocabulary derived

`En1998Snapshot` is a flat, id-less, document-root parameter form: **49** persistent scalar/boolean
fields (not the ~39 the census guessed) spanning buildings, bridges, retrofit, silos/tanks, towers,
foundations and retaining walls for the EN 1998 seismic check — no id-keyed collections, no
name/identity field. Every field became its own `change-<field>` mutation; none qualified for
`update-<facet>` — each subsystem's fields are independently entered on their own input row (no
documented atomic-bundle validation), matching the `en1991`/`en1994` precedent, not `en1993`'s
per-part grouping. `SetSnapshot` is gone with no replacement; `impl_norm_set_snapshot_ops!` removed.
Note: unlike most other norm facets, `annex` here is a plain `String` (not `AnnexChoice`), and two
fields (`multiple_resisting_systems`, `tower_is_chimney`) are `bool`.

49 mutations (field list, verb `change` throughout): `seismic_zone`, `ground_type`,
`importance_class`, `structural_system`, `t1_s`, `mass_t`, `v_rd_kn`, `drift_mm`, `height_m`,
`multiple_resisting_systems`, `annex`, `en_a_gr`, `en_ground_type`, `en_spectrum_type`,
`period_ratio`, `bridge_v_rd_kn`, `bearing_d_ed_mm`, `bearing_d_rd_mm`, `retrofit_knowledge_level`,
`retrofit_limit_state`, `retrofit_e_d_kn`, `retrofit_r_k_kn`, `retrofit_gamma_el`, `silo_height_m`,
`silo_radius_m`, `silo_n_rd_kn`, `silo_v_ed_kn`, `silo_v_rd_kn`, `silo_q_nominal`, `tank_height_m`,
`tank_radius_m`, `tank_mass_t`, `tank_v_rd_kn`, `tower_m_ed_knm`, `tower_m_rd_knm`,
`tower_is_chimney`, `tower_q_nominal`, `tower_mass_t`, `foundation_area_m2`, `foundation_p_rd_kpa`,
`foundation_h_ed_kn`, `foundation_h_rd_kn`, `k_foundation`, `k_soil`, `wall_height_m`,
`wall_phi_deg`, `wall_soil_gamma_kn_m3`, `wall_r`, `wall_h_rd_kn` (each `change-<kebab-field>`).

## Directory layout / wiring

49 new triad directories, unique emoji within the facet. Old `📄set-snapshot` deleted outright.
`📦️glue.rs`'s `en1998::…::mutations` block mounts all 49 triads directly.

## OpText/OpBinary + `from_snapshot`

Rewrote the text/binary codec with a handcrafted `En1998MutationDsl` + bridge functions (49
variants). Added `En1998Mutation::from_snapshot(&En1998Snapshot) -> Vec<En1998Mutation>` (49-entry
decomposition), wired into `import_media`/`🎮️commands/📤️set-snapshot` (renamed
`SetSnapshot`→`ReplaceSnapshot`). `evaluate` now returns `Ok(Emit::default())`.

## Tests

`every_mutation()` (49 entries), semantic-descriptor/round-trip/`from_snapshot` tests, three law
pairs (`change-seismic-zone` — u8, `change-multiple-resisting-systems` — bool,
`change-ground-type` — String). Text codec: `every_variant_op_text_round_trips` + 3 targeted tests.

## `🟦️component.ts` mirrors

All 49 triads: real, non-stub `.ts` mirrors (boolean fields map to TS `boolean`).

## Verification

See lane summary for the combined `cargo check`. Grepped this facet's own tree: zero banned-token
hits outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1998/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1998/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Created: 49 triad dirs × 3 leaves × 2 files. Rewrote dispatch/text/binary/diff-text-test files and
the three app files as in en1996/en1997. Deleted: `📄set-snapshot/**`.

## Deviation note

The census in `📓️remaining-work-map.md` estimated "~39" fields for this facet; the real snapshot
has 49. Field count was taken from the live `En1998Snapshot` struct, not the estimate.
