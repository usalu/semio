# Wave M — `norm` / `en1994` / `1` / `any` — mutations facet finishing (Job B)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Starting state (wave2)

22 `change-<field>` mutations, real triads with distinct emoji already. Notably: this facet's
`🧬️mutations/📝️text/🦀️component.rs`/`💾️binary/🦀️component.rs` were **already** a real handcrafted
`OpText`/`OpBinary` implementation (not the stale blanket-impl comment several other facets still
carried) — left untouched, no `SetSnapshot` references found in it. Self-wired inline; a true
orphan `📄set-snapshot` stub left dangling; `.ts` mirrors stubs.

## What this pass did

1. Reassigned unique emoji and renamed all 22 directories; deleted the orphan `📄set-snapshot` dir
   and its dangling `📦️glue.rs` mount.
2. Removed the dispatch file's self-wiring.
3. Rewired `📦️glue.rs`: all 22 triads mounted directly.
4. Added real `.ts` mirrors for all 22 triads.
5. Added `En1994Mutation::from_snapshot(&En1994Snapshot) -> Vec<En1994Mutation>` (22-entry flat
   decomposition), wired into `import_media`/`🎮️commands/📤️set-snapshot`; `evaluate` returns
   `Ok(Emit::default())`.

## Tests

Existing `🧪️Tests` region left intact; the pre-existing real text-codec tests were not touched.

## Verification

See lane summary for the combined `cargo check`. Verified independently: 22/22 unique emoji; all
`📦️glue.rs` en1994 `#[path]` strings resolve; zero banned-token hits in
`🗿️artifacts/📘️en1994/**`/`🎛️apps/📘️en1994/**` outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1994/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1994/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Renamed: 22 triad directories. Deleted: `📄set-snapshot/**`. Rewrote: `🧬️mutations/🦀️component.rs`
(self-wiring removed, `from_snapshot` added). Created: 22×2 `.ts` mirror files. App files
rewritten. Plugin-shared: `📦️packages/🦀️rust/📦️glue.rs` (mount block + orphan mount removed).
