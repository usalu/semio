# Wave M — `norm` / `en1992` / `1` / `any` — mutations facet finishing (Job B)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Starting state (wave2)

35 `change-<field>` mutations (34 self-wired + `change-annex` repurposed under the `set_snapshot`
mount), real triads, distinct emoji already (uniform `🔧` was *not* this facet's problem — that was
`din16798`'s wave-2 agent; en1992's was already varied). Self-wired inline; the repurposed
`set_snapshot` slot mounted by `📦️glue.rs` outside the wave-2 agent's reach; `.ts` mirrors stubs.
This is one of only two remaining-Job-B facets whose optional `fem` Cargo feature (`cross-fem`)
couples it to the FEM plugin for a shared-kernel beam solve — untouched by this pass, no mutation
in this facet's vocabulary reaches that code path.

## What this pass did

1. Reassigned unique emoji (fresh deterministic pool slice) and renamed all 35 directories,
   including the repurposed `set_snapshot`/`📄set-snapshot` slot (mod name kept, directory
   renamed, glue mount's `#[path]` string updated).
2. Removed the dispatch file's self-wiring in favour of `use super::<mod>;` lines; reworded the
   header prose that literally spelled the banned `SetSnapshot` token while describing its removal.
3. Rewired `📦️glue.rs`: all 35 triads mounted directly.
4. Added real `.ts` mirrors for all 35 triads.
5. Added `En1992Mutation::from_snapshot(&En1992Snapshot) -> Vec<En1992Mutation>` (35-entry flat
   decomposition), wired into `import_media`/`🎮️commands/📤️set-snapshot`; `evaluate` returns
   `Ok(Emit::default())`.

## Tests

Existing `🧪️Tests` region left intact.

## Verification

See lane summary for the combined `cargo check`. Verified independently: 35/35 unique emoji; all
`📦️glue.rs` en1992 `#[path]` strings resolve; zero banned-token hits in
`🗿️artifacts/📘️en1992/**`/`🎛️apps/📘️en1992/**` outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1992/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1992/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Renamed: 35 triad directories. Rewrote: `🧬️mutations/🦀️component.rs` (self-wiring removed,
prose fixed, `from_snapshot` added). Created: 35×2 `.ts` mirror files. App files rewritten.
Plugin-shared: `📦️packages/🦀️rust/📦️glue.rs`.
