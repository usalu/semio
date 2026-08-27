# Wave M — `norm` / `iso16757` / `1` / `any` — mutations facet finishing (Job B)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Starting state (wave2)

21 mutations covering document-root scalars (`change-exchange-process`, `update-script-limits`),
catalogue/manufacturer naming (`rename-catalogue`, `rename-manufacturer`), part-number
rule/inputs (`replace-part-number-rule`, `change-`/`remove-part-number-input`), the selection
facet (`change-selection-class`/`-series`, `add-`/`remove-selection-constraint`), and full
create/delete(+rename) CRUD for `product_groups`/`products`/`property_definitions`/`subjects` — the
facet's own header prose **explicitly and deliberately** defers `product_classes`, `product_series`,
`product_indexes`, `descriptive_objects`, `accessories`/`compositions` edges, dictionary
`relationships`/`properties`/`controlled_lists`/`meta_subjects`, and `geometry` to a follow-up
ticket; this pass respects that documented scope boundary rather than second-guessing it. Self-wired
inline; a true orphan `📄set-snapshot` stub; `.ts` mirrors stubs.

## What this pass did

1. Reassigned unique emoji and renamed all 21 directories; deleted the orphan `📄set-snapshot` dir
   and its dangling `📦️glue.rs` mount.
2. Removed the dispatch file's self-wiring; reworded the header comment's wiring-mechanism prose
   (the vocabulary-scope prose explaining the deliberate deferral was left as-is — still accurate).
3. Rewired `📦️glue.rs`: all 21 triads mounted directly.
4. Added real `.ts` mirrors: 6 single-field triads generated, 15 multi-field/id-addressed triads
   (create/delete/rename/replace/add/remove) hand-composed from each triad's own payload struct.
5. **`from_snapshot(base, target)`**, scoped to exactly this facet's migrated vocabulary (matching
   its own documented deferral — the un-migrated collections have no mutation to decompose into and
   so are correctly absent from a whole-document replace too, same gap `evaluate`/direct editing
   already has for those fields): renames catalogue/manufacturer, changes exchange-process/script-
   limits/part-number-rule, diffs `part_number_inputs` (upsert target keys, remove base-only keys),
   diffs `selection.constraints` (remove all BASE, `add` all target), and full delete-all-base +
   create-all-target (with explicit re-`index`ing) for `product_groups`, `products`,
   `property_definitions`, `subjects` — verified every field path (`catalogue.metadata.names.
   preferred.text`, `catalogue.manufacturer.names.preferred.text`, `catalogue.product_groups[i].id`,
   `dictionary.subjects[i].id`, …) directly against the wave-2 agent's own `inverse.rs` bodies
   (which already read `base.catalogue.product_groups`/`base.dictionary.subjects`/etc. for the same
   purpose), not guessed. Wired into `import_media`/`🎮️commands/📤️set-snapshot`; `evaluate` returns
   `Ok(Emit::default())`.

## Tests

Existing `🧪️Tests` region left intact — unaffected by directory renames.

## Verification

See lane summary for the combined `cargo check`. Verified independently: 21/21 unique emoji; all
`📦️glue.rs` iso16757 `#[path]` strings resolve (94 attrs, 0 missing); zero banned-token hits in
`🗿️artifacts/📓️iso16757/**`/`🎛️apps/📓️iso16757/**` outside the out-of-scope app-command variant
name.

## `sharedFileRequests`

None outstanding for this pass's own scope. Carried forward from wave2 (not this pass's to solve,
explicitly out of scope by the facet's own documented boundary): a follow-up ticket to migrate
`product_classes`/`product_series`/`product_indexes`/`descriptive_objects`/`accessories`/
`compositions`/dictionary `relationships`/`properties`/`controlled_lists`/`meta_subjects`/`geometry`
— once that lands, `from_snapshot` should be extended to cover the new mutations the same way.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📓️iso16757/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📓️iso16757/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Renamed: 21 triad directories. Deleted: `📄set-snapshot/**`. Rewrote: `🧬️mutations/🦀️component.rs`
(self-wiring removed, `from_snapshot(base, target)` added). Created: 21×2 `.ts` mirror files. App
files rewritten. Plugin-shared: `📦️packages/🦀️rust/📦️glue.rs` (mount block + orphan mount removed).
