# Wave M — `norm` / `en1990` / `1` / `any` — mutations facet finishing (Job B)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Starting state (wave2)

10 mutations, real triads: `change-annex` (repurposed `set_snapshot` slot), `change-permanent-
action`, `change-resistance`, `change-consequence-class`, `change-seismic-action`, plus
`insert-variable-action`/`remove-variable-action`/`change-variable-action-{category,value}`/
`reorder-variable-actions` addressing the `q_k: Vec<En1990QkEntry>` table. Nine dirs self-wired
inline; `set_snapshot` mounted by `📦️glue.rs` outside the wave-2 agent's reach; all `.ts` mirrors
were stubs.

## What this pass did

1. Reassigned unique emoji to all 10 triads and renamed their directories; the repurposed
   `set_snapshot` dir stayed named `set_snapshot`/`📄set-snapshot` (mod name unchanged — this
   facet's own tests already reference `set_snapshot::mutation::ChangeAnnex` extensively, so the mod
   name was kept to avoid unnecessary churn, only its glue mount got a fresh unique emoji-prefixed
   path via a straight `#[path]` string update, no directory rename needed).
2. Removed the dispatch file's self-wiring (`🔖️NewLeaves`/`🔖️RepurposedLeaves` regions) in favour
   of plain `use super::<mod>;` lines.
3. Rewired `📦️glue.rs`: all 10 triads mounted as `mutations`-sibling modules.
4. Added real `.ts` mirrors for the 5 single-field triads (generated from each payload struct) and
   hand-authored `.ts` mirrors for the 5 multi-field/collection triads
   (`insert-variable-action`/`remove-variable-action`/`change-variable-action-{category,value}`/
   `reorder-variable-actions`), since their payloads don't fit the generic single-`new_<field>`
   template.
5. **`from_snapshot(base, target)`** — the one facet in this lane where the single-arg
   `from_snapshot(&Snapshot) -> Vec<Mutation>` shape used everywhere else doesn't work: `q_k` is a
   real ordered collection, so a full-document replace must know the *current* document too (to
   `remove-variable-action` every existing entry, highest index first, before re-`insert`ing
   `target`'s entries). `import_media`'s app-level closure captures `doc.snapshot` to supply `base`;
   `🎮️commands/📤️set-snapshot`'s handler takes `doc.snapshot` directly. This required no
   `crate::app_surface::import_media` signature change beyond what's already shared by every facet
   (`F: Fn(D) -> Vec<M>`) — the extra `base` argument is captured by the closure, not threaded
   through the generic helper.
6. `evaluate` now returns `Ok(Emit::default())`.

## Tests

Existing `🧪️Tests` region (`every_mutation()`, round-trip, `insert_remove_variable_action_round_trips`,
out-of-range empty-inverse tests, `reorder_variable_actions_round_trips`,
`change_variable_action_category_and_value_round_trip`, three law pairs) left intact — unaffected
by directory renames.

## Verification

See lane summary for the combined `cargo check`. Verified independently: 10/10 unique emoji; all
`📦️glue.rs` en1990 `#[path]` strings resolve; zero banned-token hits in
`🗿️artifacts/📘️en1990/**`/`🎛️apps/📘️en1990/**` outside the out-of-scope `En1990Command::SetSnapshot`
app-command variant name.

## `sharedFileRequests`

None outstanding — wave2's own three sharedFileRequests (import_media/set-snapshot/evaluate
architectural decision) are resolved directly by this pass's `from_snapshot(base, target)` design.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1990/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1990/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Renamed: 9 triad directories. Rewrote: `🧬️mutations/🦀️component.rs` (self-wiring removed,
`from_snapshot(base, target)` added). Created: 10×2 `.ts` mirror files. App files rewritten:
`🎛️apps/📘️en1990/🦀️component.rs`, `🎮️commands/📤️set-snapshot/🦀️component.rs`,
`🎮️commands/🧮️evaluate/🦀️component.rs`. Plugin-shared: `📦️packages/🦀️rust/📦️glue.rs`.
