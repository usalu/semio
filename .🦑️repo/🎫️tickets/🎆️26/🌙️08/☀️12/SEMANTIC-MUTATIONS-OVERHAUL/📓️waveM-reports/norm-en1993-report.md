# Wave M — `norm` / `en1993` / `1` / `any` — mutations facet finishing (Job B)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Starting state (wave2)

17 mutations: `change-annex` (its own self-wired dir, **not** a repurposed `set_snapshot` slot —
this facet's `📄set-snapshot` was a genuine orphan, unreferenced by the migrated enum) plus 16
`update-<family>-inputs` mutations, one per EN 1993 part, each an inseparable multi-field group
(the facet's own header prose justifies this at length: every group's fields are consumed as one
unit by exactly one EN 1993 check function — this lane's agent reviewed and agrees with that
justification, so no attempt was made to split these into per-field `change-*` mutations). All 17
already had distinct emoji. Self-wired inline; `.ts` mirrors stubs.

## What this pass did

1. Reassigned unique emoji and renamed all 17 directories; deleted the genuine orphan
   `📄set-snapshot` dir and its dangling `📦️glue.rs` mount.
2. Removed the dispatch file's self-wiring; reworded stale prose.
3. Rewired `📦️glue.rs`: all 17 triads mounted directly.
4. Added a real `.ts` mirror for `change-annex` (single-field) and hand-composed real `.ts` mirrors
   for all 16 `update-*` triads (multi-field payloads, field lists read directly from each
   triad's own `mutation.rs`).
5. **`from_snapshot`**: this facet needed the "multi-field mixed" generator — each `update-*`
   mutation's payload bundles every field its EN 1993 part group reads (e.g.
   `UpdateMemberProperties` alone carries 11 fields: `n_ed_kn`, `m_ed_knm`, `v_ed_kn`, `a_mm2`,
   `a_v_mm2`, `w_pl_mm3`, `f_y_mpa`, `f_u_mpa`, `chi`, `a_net_mm2`, `tension_n_ed_kn`). Added
   `En1993Mutation::from_snapshot(&En1993Snapshot) -> Vec<En1993Mutation>` (17 entries: 1
   `ChangeAnnex` + 16 fully-populated `Update*` constructions, one field-initializer per struct
   field, read straight off the target snapshot). Wired into `import_media`/`🎮️commands/
   📤️set-snapshot`; `evaluate` returns `Ok(Emit::default())`.

## Tests

Existing `🧪️Tests` region left intact.

## Verification

See lane summary for the combined `cargo check`. Verified independently: 17/17 unique emoji; all
`📦️glue.rs` en1993 `#[path]` strings resolve; zero banned-token hits in
`🗿️artifacts/📘️en1993/**`/`🎛️apps/📘️en1993/**` outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1993/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1993/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Renamed: 17 triad directories. Deleted: `📄set-snapshot/**`. Rewrote: `🧬️mutations/🦀️component.rs`
(self-wiring + prose fixed, `from_snapshot` added). Created: 17×2 `.ts` mirror files. App files
rewritten. Plugin-shared: `📦️packages/🦀️rust/📦️glue.rs` (mount block + orphan mount removed).
