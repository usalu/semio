# goal-facetsplit-sgpfa — report

Scope: `shooting`, `gis`, `procedural`, `flow`, `animate` — restore `🔺️diff`/`↩️inverse` facet
directories for every mutation whose diff/inverse behavior was inlined back into the direct leaf.

## Method

1. Enumerated mutation direct leaves per plugin (`git ls-files ... | grep 🧬️mutations/[^/]+/🦀️(\.|component\.)rs$`),
   95 total across the 5 plugins.
2. Applied the behavioural predicate (top-level `pub (async )?fn diff(`/`fn inverse(` with no sibling
   `🔺️diff`/`↩️inverse` dir) — 70 inlined mutations found.
3. Scripted restoration (`📜️goal-facetsplit-sgpfa-restore.py`, brace-matching Rust parser, no regex
   brace-counting inside string/char literals): for each inlined mutation dir `D`, pulled
   `D/🔺️diff/🦀️component.rs` and `D/↩️inverse/🦀️component.rs` from pinned commit
   `bb06c41f73f0122fbed315b7487428b976f99921`, collapsed every `::mutation::` module segment to `::`
   (the pinned commit's dedicated `🦠️mutation` facet/module no longer exists anywhere — struct+impl
   now live directly in the kind-only leaf, for both this mutation's own self-reference and any
   sibling mutation referenced cross-mutation), and wrote the result to `D/🔺️diff/🦀️.rs` /
   `D/↩️inverse/🦀️.rs` (kind-only names, not `component.rs`).
4. Stripped the corresponding standalone `fn diff`/`fn inverse` (+ attached doc comment + tightly
   -wrapping region markers where present) from the direct leaf, rewrote the `MutationKind` impl's
   `diff`/`inverse` methods to delegate (`super::diff::diff(self, base)` / `super::inverse::inverse(self, base)`,
   preserving each plugin's existing sync/async convention), and pruned now-unused `use` imports.
5. Verified restored facet bodies are byte-for-byte logic matches (whitespace/module-path normalized)
   of the pre-edit inlined bodies for all 70 — zero silent content drift. 6 shooting `inverse` cases
   needed the generalized `::mutation::` → `::` rule (cross-mutation refs like
   `delete_asset::mutation::DeleteAsset`); all others matched with just `super::mutation::` → `super::`.
6. Rewired all 5 `📦️glue.rs` crate roots (`📜️goal-facetsplit-sgpfa-patch-glue.py`): inserted
   `pub mod diff;` / `pub mod inverse;` `#[path]` mounts before each mutation's `mod component;`,
   copying the vcs/`add-tag` exemplar's mount style exactly. 70/70 mount points matched, 0 missing.

## Before/after (behavioural predicate)

| plugin | before | after |
|---|---:|---:|
| 🎥️shooting | 31 | 0 |
| 🌍️gis | 12 | 0 |
| 🌀️procedural | 9 | 0 |
| 🌊️flow | 9 | 0 |
| 🎞️animate | 9 | 0 |
| **total** | **70** | **0** |

No semantic directory was deleted or flattened; only new `🔺️diff`/`↩️inverse` dirs were created.
`🦠️mutation` facets were left untouched (never restored) — out of this ticket's scope per the
target shape (`add-tag`'s struct+impl live in the direct leaf, its `🦠️mutation` dir is empty).

## Verification

- Behavioural predicate re-run: 0/0/0/0/0 (table above) — confirmed via filesystem walk, not just
  `git ls-files` (new facet files are untracked).
- `rustfmt --check --edition 2021` (stdin, single-file, no crate-tree traversal to avoid noise from
  unrelated pre-existing files pulled in transitively via `glue.rs`'s `#[path]` mod tree):
  - All 140 new facet files (`🔺️diff/🦀️.rs` + `↩️inverse/🦀️.rs`): **0 diffs**, fully clean.
  - All 70 edited direct leaves: compared before/after diff-count per file — **0 files regressed**
    (several were already non-clean pre-edit from unrelated content elsewhere in the file; none of
    my edits added new diff hunks).
  - All 5 `glue.rs`: my inserted 3-line mount blocks trigger the same reordering complaint
    (`mod component;` before the `diff`/`inverse` mounts) that the **target exemplar itself**
    (`vcs`/`add-tag`, `📦️packages/🦀️rust/📦️glue.rs`) already has at HEAD — pre-existing, accepted
    convention for this hand-authored file family, not a regression.
- `cargo check -p semio-s-plugin-{gis,shooting,procedural,flow,animate}`: launched in parallel,
  blocked for the full session on `Blocking waiting for file lock on package cache/build directory`
  (concurrent sessions building the same workspace) and never reached a rustc invocation for these
  crates before the ticket's time budget ran out. Per this ticket's own prior report
  (`📓️goal-rustjoin-report.md`), `cargo check` for every plugin crate is additionally blocked
  transitively by pre-existing, unrelated compile errors in `semio-framework-os-kernel`
  (`self.store.detach_backbone().await` on a non-future `Result`) and `semio-framework-plugin-host`
  (missing `diff`/`inverse` on `MergePolicyConfigMutation`/`OpeningConfigMutation`) — both predate
  this session. Falling back to `rustfmt --check` (above) as instructed.
- Cross-checked the pinned commit's cross-mutation references (e.g. shooting's `create-asset`
  inverse constructs `delete_asset::...::DeleteAsset`) resolve correctly after the `::mutation::`
  collapse — confirmed by exact-body-match diffing against the pre-edit inlined text (item 5 above).
- `🌊️flow`/`🌍️gis` mutation files: re-read live at edit time (script reads the file fresh at
  process time, not a cached snapshot), so the other worker's unrelated Rust-path-join fix in these
  files (see `📓️goal-rustjoin-report.md`) was preserved automatically — no reconciliation needed.

## Files

- Restore scripts (kept, ticket-scoped):
  `📜️goal-facetsplit-sgpfa-restore.py`, `📜️goal-facetsplit-sgpfa-patch-glue.py`,
  `📜️goal-facetsplit-sgpfa-classify.py`
- 70 mutation dirs × 3 files each: new `🔺️diff/🦀️.rs`, new `↩️inverse/🦀️.rs`, rewritten `🦀️.rs`
  (delegating impl, pruned imports) across
  `✏️s/🔌️plugins/{🎥️shooting,🌍️gis,🌀️procedural,🌊️flow,🎞️animate}/…/🧬️mutations/*/`.
- 5 rewired crate roots: `✏️s/🔌️plugins/{🎥️shooting,🌍️gis,🌀️procedural,🌊️flow,🎞️animate}/📦️packages/🦀️rust/📦️glue.rs`.
