# Wave M — `norm` / `en1995` / `1` / `any` — mutations facet finishing (Job B)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Starting state (wave2) — **including real, pre-existing compile-breaking bugs**

20 mutations (19 self-wired `change-*` + `change-annex` repurposed under `set_snapshot`). Two
independent defects predating this pass, both confirmed by `cargo check` before any edits:

1. **`SEMANTICS.kind` kebab bugs, 4 variants**: `ChangeFC0K`, `ChangeFMK`, `ChangeAVertMS2`,
   `ChangeFVK` had hand-typed `kind`/`entity` strings (`"change-f-c-0-k"`, `"change-f-m-k"`,
   `"change-a-vert-m-s2"`, `"change-f-v-k"`) that do **not** match `#[derive(dsl::Mutations)]`'s own
   compile-time `to_kebab(variant name)` assertion. `cargo check` panicked at these four with
   `evaluation panicked: … MutationKind::SEMANTICS.kind must equal "…" (its own kebab form)`. Traced
   the derive's actual `to_kebab` algorithm by hand (`🗣️dsl/✨️derive/🦀️component.rs`) and confirmed
   the correct forms are `change-fc0-k`, `change-fmk`, `change-a-vert-ms2`, `change-fvk` (adjacent
   all-caps runs with no lowercase letter between them merge into one kebab word — the same rule
   documented in `en1991`'s own header prose for its `EnVBMS`-style fields). Fixed all four
   `kind`/`entity` values and renamed their triad directories to match (`🔧change-f-c-0-k` →
   `🔧change-fc0-k`, etc.) **before** running this lane's generic emoji-reassignment tooling, so the
   tool picked up the corrected kebab, not the buggy one.
2. **Text/binary codec never migrated**: `🧬️mutations/📝️text/🦀️component.rs` still carried the
   pre-migration placeholder claiming blanket `OpText`/`OpBinary` impls "for free" via
   `impl_norm_set_snapshot_ops!`'s macro-provided trait bound — but the dispatch enum had already
   gone semantic, so `En1995Mutation` satisfied neither trait. `cargo check` failed with `En1995Mutation:
   OpText`/`OpBinary is not satisfied` plus missing `encode_op`/`decode_op`. Rewrote both files with a
   handcrafted `En1995MutationDsl` mirror + bridge functions, matching this lane's established
   pattern.
3. **Directory-loss near-miss** (documentation, not a defect in the final state): this lane's
   generic restructuring tool initially treated `set_snapshot` as an unreferenced legacy directory
   (because en1995's `set_snapshot` mount is a `📦️glue.rs`-level mount, not a self-wired block, so it
   never appeared in the dispatch file's self-wiring scan) and deleted `📄set-snapshot` — while the
   dispatch enum's `ChangeAnnex(set_snapshot::mutation::ChangeAnnex)` variant still referenced it.
   Caught immediately (the enum literally names the module), recovered the deleted files via
   `git checkout -- <path>` (tracked, uncommitted-deletion, no history rewrite), renamed the
   recovered directory to `📐change-annex`, and re-added its `📦️glue.rs` mount. The generic tool
   (`restructure_generic.py`'s `restructure()`) was hardened afterward to detect
   `"set_snapshot::mutation::" in src` before ever treating that dir as legacy, and every other
   facet already processed at that point (`en1990`, `en1992`) was re-verified clean.

## What this pass did (beyond the bug fixes above)

Reassigned unique emoji and renamed all 20 directories (including the recovered `set_snapshot` slot);
removed self-wiring; rewired `📦️glue.rs` (20 direct mounts); added real `.ts` mirrors for all 20
triads; added `En1995Mutation::from_snapshot(&En1995Snapshot) -> Vec<En1995Mutation>`, wired into
`import_media`/`🎮️commands/📤️set-snapshot`; `evaluate` returns `Ok(Emit::default())`.

## Tests

Existing `🧪️Tests` region left intact (unaffected by directory renames or the kebab fix, since
tests reference module paths, not the `kind` string literals).

## Verification

See lane summary for the combined `cargo check`. Verified independently: 20/20 unique emoji, zero
`E0080` kebab-mismatch panics remain for this facet, all `📦️glue.rs` en1995 `#[path]` strings
resolve (87 attrs, 0 missing after the recovery), zero banned-token hits in
`🗿️artifacts/📘️en1995/**`/`🎛️apps/📘️en1995/**` outside the out-of-scope app-command variant name.

## `sharedFileRequests`

None outstanding.

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1995/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1995/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Renamed: 20 triad directories (4 with corrected kebab slugs). Rewrote: `🧬️mutations/🦀️component.rs`
(self-wiring removed, `from_snapshot` added), `📝️text/🦀️component.rs`/`💾️binary/🦀️component.rs`
(genuinely new OpText/OpBinary implementations, not touch-ups), `🔺️diff/📝️text/🦀️component.rs`
(broken `SetSnapshot` test fixture replaced with a real `Change*` diff test), 4 `🦠️mutation/
🦀️component.rs` files (`kind`/`entity` fixes). Created: 20×2 `.ts` mirror files. App files
rewritten. Plugin-shared: `📦️packages/🦀️rust/📦️glue.rs` (mount block, including the recovered
`set_snapshot` slot).
