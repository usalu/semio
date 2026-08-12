# Wave M — `norm` / `en1991` / `1` / `any` — mutations facet finishing (Job B)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Starting state (wave2)

32 `change-<field>` mutations, all real triads, **already** carrying distinct per-mutation emoji
(this wave-2 agent got emoji-uniqueness right the first time — no reassignment needed). Self-wired
inline (`🔖️LeafWiring` region); a true orphan `📄set-snapshot` stub (never referenced by the
migrated enum, confirmed by its own doc comment) left dangling in both the facet directory and
`📦️glue.rs`'s mount for it; all `.ts` mirrors were stubs.

## What this pass did

1. Reassigned emoji anyway (for lane-wide tooling consistency — the previous assignment was fine
   but re-derived from a fresh deterministic pool slice like every other facet) and renamed all 32
   directories.
2. Deleted the orphan `📄set-snapshot` dir and its dangling `📦️glue.rs` mount (the mount pointed at
   a directory the migrated enum never referenced).
3. Removed the dispatch file's empty `🔖️LeafWiring` region and inline self-wiring in favour of
   `use super::<mod>;` lines; cleaned the stale header prose describing the now-removed self-wiring
   approach and orphan stub.
4. Rewired `📦️glue.rs`: all 32 triads mounted directly.
5. Added real `.ts` mirrors for all 32 triads (field types read from each payload struct).
6. Added `En1991Mutation::from_snapshot(&En1991Snapshot) -> Vec<En1991Mutation>` (32-entry flat
   decomposition — this facet has no collections) and wired it into `import_media`/
   `🎮️commands/📤️set-snapshot`; `evaluate` now returns `Ok(Emit::default())`.

## Tests

Existing `🧪️Tests` region (`round_trip` helper, 20 named `*_round_trips` tests, `semantic_kinds_
cover_every_variant`, two inverse-specific tests) left intact.

## Verification

See lane summary for the combined `cargo check`. Verified independently: 32/32 unique emoji; all
`📦️glue.rs` en1991 `#[path]` strings resolve (126 attrs, 0 missing); zero banned-token hits in
`🗿️artifacts/📘️en1991/**`/`🎛️apps/📘️en1991/**` outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1991/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1991/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Renamed: 32 triad directories. Deleted: `📄set-snapshot/**`. Rewrote: `🧬️mutations/🦀️component.rs`
(self-wiring + orphan-region removed, `from_snapshot` added). Created: 32×2 `.ts` mirror files.
App files rewritten. Plugin-shared: `📦️packages/🦀️rust/📦️glue.rs` (mutations mount block + orphan
mount removed).
