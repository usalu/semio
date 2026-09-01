# Facet-split undo — `✏️s/🔌️plugins/📕️norm`

## Method

1. Work list built two ways and cross-checked identical (371 mutations):
   - Marker grep: `git grep -lE '//#region 🔖️(Diff|Inverse)' -- '…/🧬️mutations/*/🦀️.rs'`
   - Corrected predicate (per coordinator correction): `pub (async )?fn diff(` / `fn inverse(` present with no sibling `🔺️diff/`/`↩️inverse/` dir. Both gave the **same 371** paths — `norm` has no unmarked-inlining cases like `🏛️architect`.
2. For each of the 371 mutation dirs: pulled `🔺️diff/🦀️component.rs` and `↩️inverse/🦀️component.rs` from pinned commit `bb06c41f73f0122fbed315b7487428b976f99921` (0 missing — no fallback extraction needed), rewrote every `::mutation::` segment to `::` (payload now lives directly in the leaf, not a `🦠️mutation/` subdir — target shape has no `mutation` facet, matching the vcs `🏷️add-tag` exemplar), and wrote them as kind-only `D/🔺️diff/🦀️.rs` / `D/↩️inverse/🦀️.rs`.
3. Rewrote each direct leaf: removed the `//#region 🔖️Diff…//#endregion 🔖️Diff` and `//#region 🔖️Inverse…//#endregion 🔖️Inverse` blocks; the `MutationKind` impl's `fn diff`/`fn inverse` bodies now delegate `super::diff::diff(self, base)` / `super::inverse::inverse(self, base)` (previously either a bare local call or a fully-qualified `crate::…::diff::diff` call to a module that didn't exist yet — both forms normalized to `super::…`).
4. `📦️glue.rs`: for each of the 371, inserted `#[path=".../🔺️diff/🦀️.rs"] pub mod diff;` and `#[path=".../↩️inverse/🦀️.rs"] pub mod inverse;` immediately before the existing `mod component; pub use component::*;`, matching the vcs exemplar's mount order exactly.
5. **Extra pass, not in the original brief but required for correctness**: the inlining sweep had also left ~38 self-referential `use …mutations::<name>::mutation::<Type>;` imports inside affected leaves themselves, plus ~700 more in each artifact's shared `📝️text`/`💾️binary` mutation-codec files and the top-level mutation-enum `🦀️component.rs` (they still addressed sibling payloads via the now-gone `mutation` submodule). Fixed via a script scoped **per artifact directory** using the *actual* glue.rs module alias (not the directory-derived name — 42 of 371 mutations have a name mangling mismatch, e.g. `change-as-mm2` → alias `change_a_s_mm2`, and en1995/en1996/en1997/en1998/en1999/din16798 each have one `change-annex` mutation mounted under the historical alias `set_snapshot`). Scoping by directory avoided the 3 genuine name collisions with iso16757 (`create_product`/`rename_product`/`delete_product`, which legitimately still has `🦠️mutation/` subdirs and was correctly left untouched).

## Counts

- Direct-leaf `//#region 🔖️(Diff|Inverse)` matches: **371 → 0** (`git grep -cE … -- '…/🧬️mutations/*/🦀️.rs'`).
- New facet files created: **742** (`371 × {🔺️diff/🦀️.rs, ↩️inverse/🦀️.rs}`), all untracked (`git status --porcelain` confirms exactly 742 `??` under the plugin).
- `glue.rs`: 371 new `pub mod diff;`/`pub mod inverse;` pairs added → plugin-wide total 392 each (371 new + 21 pre-existing iso16757, which was never touched — it still legitimately uses a `🦠️mutation/` subdir shape).
- Stale `::mutation::` references fixed: 588 (first pass, derived names) + 214 (second pass, true-alias correction) = **~700 replacements across 55 files**, plus the 38 self-imports folded into the same fix. Zero remaining stale `::mutation::` references anywhere in the 371 mutations' own trees (verified plugin-wide; the only surviving `::mutation::` hits are legitimately iso16757's).
- Nothing fell back to same-file region extraction — pinned-commit content was available for all 742 facet files.

## Verification (real output)

- **Structural**: all 371 leaves have exactly one `super::diff::diff(self, base)` and one `super::inverse::inverse(self, base)`, zero remaining `#region 🔖️Diff|Inverse`, exactly one `fn diff(&self`/`fn inverse(&self` each. Brace-balance checked clean on all 1113 touched/created files.
- **rustfmt**: `rustfmt --check --config-path rustfmt.toml` is **clean (0 diffs)** on all 742 newly-created facet files. The leaf files still show diffs, but `git diff` confirms every diff hunk is in lines I never touched (pre-existing double-blank-line/import-order style already in the repo). `glue.rs` itself shows a diff at every inserted mount block (rustfmt wants `mod component;` before `pub mod diff/inverse;`) — but the **vcs exemplar's own glue.rs has the identical non-compliance at its `add_tag` block**, confirming this mount order is the established (if not rustfmt-clean) convention, correctly replicated.
- **cargo check**: `cargo check -p semio-s-plugin-norm` (both on the shared target dir and a second run with an isolated `CARGO_TARGET_DIR`) never reaches `norm`'s own code — its dependency `semio-s-plugin-stdio` fails first with **65 pre-existing `E0046` errors** ("not all trait items implemented: missing `DESCRIPTORS`, `descriptor`") across unrelated stdio format artifacts (wav, mp4, html, binary, tsv, mp3, avi, epw, json, semio…). Confirmed unrelated to this work — no file under `✏️s/🔌️plugins/🗄️stdio` was touched. Per the brief's fallback, relied on rustfmt --check + the structural checks above instead of a full type-check.

## Skipped / not done

- Nothing was skipped within the 371-mutation scope.
- Did not touch iso16757 (21 mutations) — it already has the correct `🦠️mutation/🔺️diff/↩️inverse` triad shape and was out of scope.
- Did not attempt to get `cargo check` past the pre-existing `semio-s-plugin-stdio` breakage (out of scope, not caused by this change).

## Files touched

- 371 mutation leaves: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/🏅️standards/*/🪆️subsets/✳️any/🧬️schema/🧬️mutations/*/🦀️.rs`
- 742 new files: same dirs' `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs`
- 55 shared codec/enum files fixed for stale `::mutation::` refs (each artifact's `🧬️mutations/🦀️component.rs`, `🧬️mutations/📝️text/🦀️component.rs`, `🧬️mutations/💾️binary/🦀️component.rs`, and 3 `🔺️diff/📝️text/🦀️component.rs`)
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs`
