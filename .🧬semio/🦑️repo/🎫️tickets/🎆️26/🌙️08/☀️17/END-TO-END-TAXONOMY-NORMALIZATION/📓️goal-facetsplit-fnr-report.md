# 🔖️ Facet-split restoration — fem / remodel / note

Coordinator's corrected predicate (`fn diff(`/`fn inverse(` with no sibling facet dir, not just
the `//#region` marker) re-derived to the identical 118-file set the marker grep already found —
no undercounting in this scope. Script: `📜️goal-facetsplit-fnr-extract.ts` (this dir).

## Before / after

- Region-marker grep (`//#region 🔖️(Diff|Inverse)`) over the three plugins' mutation leaves:
  **118 → 0**.
- Processed 118/118, 0 skipped. Per plugin: 🏗️fem 50, 📸️remodel 35, 🗒️note 33 (matches brief).
- 236 new `🔺️diff/🦀️component.rs` + `↩️inverse/🦀️component.rs` facet files, sourced verbatim from
  pinned commit `bb06c41f73f0122fbed315b7487428b976f99921` with `::mutation::` → `::` applied
  (removed-submodule path shift — confirmed present in all 236/236 pinned originals, no other
  qualifier shapes found).
- 3 `📦️glue.rs` files rewired: `pub mod diff;` / `pub mod inverse;` mounted beside each mutation's
  existing `mod component;`, matching the `vcs`/`add-tag` exemplar mount style exactly.
- Direct leaves: stripped both inlined bodies (outer `//#region 🔺️diff`/`↩️inverse` wrapper where
  present — 🏗️fem only, 50 files; inner-marker-only elsewhere — 68 files), retargeted the impl's
  `diff(self, base)`/`inverse(self, base)` calls to `super::diff::diff(...)`/`super::inverse::inverse(...)`.
  Payload struct + `MutationKind` impl untouched, left inline (matches exemplar; `🦠️mutation` facet
  not required — contract only forbids inlining for `🔺️diff`/`↩️inverse`).

## Verification

- 0 leftover `::mutation::` refs, 0 `super::super::` refs across all 236 new facet files.
- `rustfmt --check` on all 354 touched/created files: 118 diffs, all identical in shape — pure
  pre-existing `use`-import ordering in the direct leaf, confirmed via `git diff` to predate my
  edit (I never touch `use` lines). 0 diffs in the 236 new facet files.
- `cargo check -p semio-s-plugin-fem`: blocked — `semio-s-plugin-stdio` (a required dependency,
  outside my scope) fails with 65 pre-existing errors. `git status` on `✏️s/🔌️plugins/🗄️stdio`
  shows ~2,900 lines of uncommitted changes — another session is mid-flight on that plugin right
  now (last real commit to it is `bb06c41f73`, the same pin I read from). `remodel`/`note` both
  depend on `stdio` identically, so the same blocker applies transitively; their `cargo check`
  runs were still queued behind heavy concurrent build load at report time and did not finish —
  not claiming pass/fail for them, only that the dependency graph is identical to fem's.
- `git status --porcelain` restricted to exactly the 118 leaves + 3 glue.rs: 118× `AM`, 3× `MM` —
  no other files touched (the wider plugin-tree noise in full `git status` is concurrent workers
  on unrelated facets, not mine, per the "ignore unrelated recent changes" rule).

## Not run to completion

`cargo check` for 📸️remodel and 🗒️note did not finish before this report (machine under heavy
concurrent-session load; `stdio` itself was still uncompilable when last polled). Textual/structural
verification above stands in for it per the brief's fallback. Re-run `cargo check -p semio-s-plugin-remodel`
/ `-p semio-s-plugin-note` once `stdio` is fixed by its owning session.

Files touched: 118 direct-leaf `🦀️.rs` (region strip + delegate calls), 236 new facet `🦀️component.rs`
files, 3 `📦️glue.rs`. Script and this report kept at ticket root; nothing under `🗑️temp/` needs to
survive (run log deleted).

## Correction — kind-only leaf naming (coordinator defect report)

Initial pass wrongly kept the pinned commit's `🦀️component.rs` stem for the 236 restored
`🔺️diff`/`↩️inverse` facet files. Fixed: `mv` (not `git mv`) renamed all 236 to kind-only `🦀️.rs`,
`glue.rs` mounts updated to match. One mistake caught during the fix: a blanket sed over
`🔺️diff/🦀️component.rs` in `glue.rs` also touched the unrelated *schema-level* diff-type mount
(`🧬️schema/🔺️diff/🦀️component.rs` — the `Fem2dDiff`/etc. component, not a mutation facet); reverted
those 4 lines (2× fem, 1× remodel, 1× note) back to `component.rs`, matching their still-correctly-named
files on disk.

Verified: `find` kind-only vs named — fem 100/0, remodel 70/0, note 66/0 (2×50/35/33, diff+inverse).
`git grep '🔺️diff/🦀️component\.rs\|↩️inverse/🦀️component\.rs'` over the three plugins now returns
only schema-level hits (outside `🧬️mutations/`), zero inside any mutation directory.
