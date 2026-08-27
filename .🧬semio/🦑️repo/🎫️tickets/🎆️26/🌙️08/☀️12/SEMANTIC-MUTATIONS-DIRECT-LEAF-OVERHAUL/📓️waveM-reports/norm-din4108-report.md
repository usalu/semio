# Wave M — `norm` / `din4108` / `1` / `any` — mutations facet finishing (Job B)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Starting state (wave2)

22 mutations: 17 `change-<field>` scalars, plus `insert-layer`/`remove-layer`/`reorder-layers`/
`change-layer-thickness`/`change-layer-lambda` addressing `layers: Vec<LayerDocument>` (an
index-keyed, id-less ordered construction build-up — same shape family as `en1990`'s `q_k`). All
self-wired inline; a true orphan `📄set-snapshot` stub; `.ts` mirrors stubs.

## What this pass did

1. Reassigned unique emoji and renamed all 22 directories; deleted the orphan `📄set-snapshot` dir
   and its dangling `📦️glue.rs` mount.
2. Removed the dispatch file's self-wiring.
3. Rewired `📦️glue.rs`: all 22 triads mounted directly.
4. Added real `.ts` mirrors for all 22 triads (17 single-field generated, 5 layer-collection
   triads hand-composed since `InsertLayer { index, layer: LayerDocument }` etc. don't fit the
   generic single-`new_<field>` template).
5. **`from_snapshot(base, target)`**: like `en1990`, needs `base` because `layers` is a real
   ordered collection — every existing layer is `remove-layer`'d (highest BASE index first) before
   `target`'s layers are re-`insert-layer`'d in order. `import_media`'s closure captures
   `doc.snapshot`; `🎮️commands/📤️set-snapshot`'s handler passes `doc.snapshot` directly. `evaluate`
   returns `Ok(Emit::default())`.

## Tests

Existing `🧪️Tests` region left intact.

## Verification

See lane summary for the combined `cargo check`. Verified independently: 22/22 unique emoji; all
`📦️glue.rs` din4108 `#[path]` strings resolve (97 attrs, 0 missing); zero banned-token hits in
`🗿️artifacts/📕️din4108/**`/`🎛️apps/📕️din4108/**` outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📕️din4108/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📕️din4108/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Renamed: 22 triad directories. Deleted: `📄set-snapshot/**`. Rewrote: `🧬️mutations/🦀️component.rs`
(self-wiring removed, `from_snapshot(base, target)` added). Created: 22×2 `.ts` mirror files. App
files rewritten. Plugin-shared: `📦️packages/🦀️rust/📦️glue.rs` (mount block + orphan mount removed).
