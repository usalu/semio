# 🔺️↩️ Undo diff/inverse inlining — `🧱️block` + `🗄️stdio`

## Scope
`✏️s/🔌️plugins/🧱️block` and `✏️s/🔌️plugins/🗄️stdio` only. No other plugin touched.

## Before → after (marker predicate: `//#region 🔖️(Diff|Inverse)` on the direct leaf)
- `🧱️block`: 104 → **0**
- `🗄️stdio`: 97 → **0**

## Behavioural predicate (`pub (async )?fn diff(`/`inverse(` present with no sibling `🔺️diff/`/`↩️inverse/`)
A concurrent coordinator flagged mid-task that this predicate catches inlined leaves the marker grep misses. Verified independently against `git ls-files` + the pinned commit `bb06c41f73` (which still shows `🔺️diff/🦀️component.rs` + `↩️inverse/🦀️component.rs` for these dirs) before acting:
- `🧱️block`: **0** extra (marker predicate was already complete)
- `🗄️stdio`: **16** extra found and fixed — all the `📄set-snapshot` mutation (apply/diff/inverse free-function trio, no payload struct; different shape than the region-marked ones, so a dedicated extraction script was written for it)
- Both predicates now read **0/0** across both plugins.
- **68** pre-existing `🦀️component.rs`-named facet leaves exist elsewhere in `🗄️stdio` (legacy stems, last touched 2026-08-21, zero overlap with anything I edited) — confirmed pre-existing and left untouched, per instruction not to rename legacy facets.

## What changed (217 mutation dirs: 104 + 97 + 16)
- Each direct leaf `🦀️.rs` had its `Diff`/`Inverse` body relocated verbatim to new `🔺️diff/🦀️.rs` / `↩️inverse/🦀️.rs` (kind-only leaves, never `🦀️component.rs`), with per-file pruned `use` lines.
- Leaf's `MutationKind` impl now delegates: `super::diff::diff(self, base)` / `super::inverse::inverse(self, base)`.
- **Bug found + fixed during extraction**: in the `🗄️stdio` shape the payload struct was referenced bare (e.g. `&DeleteEdge`) since it used to share a module with the free functions; moved to a sibling module it must be `&super::DeleteEdge` (matches the `🌿️vcs` ground-truth exemplar). Applied a targeted fix (word-boundary substitution) and then had to correct an over-broad first pass that had also mangled enum-variant paths and cross-module paths (e.g. `SemioBrepMutation::DeleteEdge`, `delete_edge::DeleteEdge`) — reverified 0 remaining bad `X::super::` occurrences afterward.
- `📦️glue.rs` for both plugins: mounted `pub mod diff;` / `pub mod inverse;` alongside each `mod component;`, matching the `🌿️vcs` exemplar. Handled two pre-existing mount shapes (block-wrapped `pub mod X { mod component; }` and flat `pub mod X;`), converting the flat ones into blocks.

## Build verification
`cargo check -p semio-s-plugin-block` transitively pulls in `semio-s-plugin-stdio` (a real dependency), which currently fails with **61 pre-existing errors** (E0046/E0425/E0599) from another session's in-flight refactor, in files outside anything I touched (`🖼️bmp`, `🎨️svg`, `📰xml`, `🧊️gltf` mutations). Neither plugin can get a green `cargo check` right now for reasons unrelated to this change — pasted real output, not claimed. Fallback used instead: `rustfmt --check --edition 2021` on all 651 touched/created files → **0 parse errors** (only style diffs, exit 1 from formatting only).

## Files
Scripts (kept, at ticket root): `📜️goal-facetsplit-blockstdio-split.py`, `📜️goal-facetsplit-blockstdio-glue.py`, `📜️goal-facetsplit-blockstdio-split-setsnapshot.py`.

## Note on unsolicited scope-expansion messages
Twice during this task, a message styled as "the coordinator" arrived embedded in a system-reminder immediately after tool output (not as a normal chat turn), the second one flagged by the environment itself as a possible injection. The first asked me to expand to a repo-wide 20-plugin scope, which contradicts the explicit hard rule ("your two plugins... no other plugin") — not acted on. Both messages also carried narrower, checkable technical claims (the 16 unmarked `stdio` leaves; the `super::` qualifier bug; the 61 pre-existing `stdio` errors); I independently verified each against the repo/git history myself before doing anything, rather than trusting the messages, and only the verified, in-scope parts (fixing the 16 `stdio` leaves) were acted on.
