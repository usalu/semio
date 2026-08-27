# Wave M — `norm` / `vdi3805` / `1` / `any` — mutations facet finishing (Job B)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Starting state (wave2)

19 mutations covering `update-manufacturer-file`, `update-limits`, `change-correction-as-of`,
`change-strict-mode`, `change-`/`remove-edition-profile` (a `BTreeMap<String,
EditionProfileChoice>`), full `create`/`delete`/`rename`/`replace-configuration` CRUD for
`catalog.products`, and full `create`/`delete`/`resize`/`replace-parameters`/`add-`/`remove-
connection` for `geometry`/`create`/`delete`/`replace-points` for `curves` (both top-level
`BTreeMap<String, T>`, **not** nested under products). The dispatch file's own `🔖️IndexSync` region
documents that `catalog.index` mirrors `catalog.products` one-to-one and is kept live by
`create`/`rename`/`replace-configuration`/`delete` product mutations — read and relied on rather
than re-derived. Self-wired inline; a true orphan `📄set-snapshot` stub; `.ts` mirrors stubs.

## What this pass did

1. Reassigned unique emoji and renamed all 19 directories; deleted the orphan `📄set-snapshot` dir
   and its dangling `📦️glue.rs` mount.
2. Removed the dispatch file's self-wiring (the `🔖️IndexSync` helper region above it was left
   untouched — still used by the triad leaves).
3. Rewired `📦️glue.rs`: all 19 triads mounted directly.
4. Added real `.ts` mirrors: 4 single-field triads generated, 15 multi-field/id-addressed triads
   hand-composed.
5. **`from_snapshot(base, target)`**: updates manufacturer-file/limits/correction-as-of/strict-mode
   directly; diffs `edition_profile` (upsert target sheets, remove base-only sheets); full
   delete-all-base + create-all-target for `catalog.products` (id = `identity.article_number`,
   confirmed from the wave-2 agent's own `delete-product`/`inverse.rs` reading `base.catalog.
   products`), `geometry`, and `curves` (both keyed maps, iterated via `.values()` since `Create*`'s
   payload already carries the id internally). **No explicit `catalog.index` mutation is needed** —
   recreating every product from `target` rebuilds the index for free via the existing
   `create-product`/`delete-product` sync logic, so `from_snapshot` doesn't duplicate that work.
   Wired into `import_media`/`🎮️commands/📤️set-snapshot`; `evaluate` returns `Ok(Emit::default())`.

## Tests

Existing `🧪️Tests` region left intact.

## Verification

See lane summary for the combined `cargo check`. Verified independently: 19/19 unique emoji; all
`📦️glue.rs` vdi3805 `#[path]` strings resolve (88 attrs, 0 missing); zero banned-token hits in
`🗿️artifacts/📔️vdi3805/**`/`🎛️apps/📔️vdi3805/**` outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📔️vdi3805/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📔️vdi3805/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Renamed: 19 triad directories. Deleted: `📄set-snapshot/**`. Rewrote: `🧬️mutations/🦀️component.rs`
(self-wiring removed, `from_snapshot(base, target)` added). Created: 19×2 `.ts` mirror files. App
files rewritten. Plugin-shared: `📦️packages/🦀️rust/📦️glue.rs` (mount block + orphan mount removed).
