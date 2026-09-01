# Facet-split restoration — small plugins (process, mathematical, raster, reasoning, playbook)

## Scope
`🏭️process` (16), `➗️mathematical` (15), `🖨️raster` (12), `💡️reasoning` (10), `📖️playbook` (9) — 62 mutations total, matching the coordinator's corrected repo-wide census for these five plugins.

## Method
Corrected predicate applied (not the naive `//#region` grep, which happened to agree for these five plugins but was not trusted blind): a mutation leaf counts as inlined when it has a freestanding `pub (async) fn diff(`/`fn inverse(` with no sibling `🔺️diff/`/`↩️inverse/` directory. For each of the 62: extracted the `//#region 🔖️Diff` / `//#region 🔖️Inverse` bodies out of the direct leaf into `D/🔺️diff/🦀️.rs` and `D/↩️inverse/🦀️.rs` (kind-only leaf names, per the ticket's explicit override of the vcs exemplar's `component.rs` naming); rewrote the leaf's impl to `super::diff::diff(self, base)` / `super::inverse::inverse(self, base)` (`.await`-suffixed where the impl method is `async fn`, matching each plugin's own async-ness — mathematical/reasoning are async, process/raster/playbook are not); qualified the payload type as `super::Payload` both in signatures and in any in-body reconstruction (e.g. a "replace"/"resize"/"move" mutation's inverse that rebuilds its own payload type) — 25 such in-body occurrences were caught and fixed beyond the signature-only pass. Doc-comment headers were recovered from the pinned pre-collapse commit `bb06c41f73f0122fbed315b7487428b976f99921` where available. `use` lines were pruned per-file to only what each facet body actually references (including dropping the payload type's now-redundant self-import, superseded by inline `super::` qualification). Rewired all five `📦️glue.rs` files to mount `pub mod diff;` / `pub mod inverse;` beside each mutation's `mod component;`, `#[path]`-relative, matching the `🌿️vcs/…/🏷️add-tag` exemplar's mounting style exactly.

Scripts (kept, per ticket root convention): `📜️goal-facetsplit-small-extract.py` (main extraction pass). Ad hoc one-off fixups (self-import pruning, unused-item pruning, body-qualification of in-body payload-type references, glue.rs patching) were run from `/tmp` and are not retained as they were single-use over a worklist, not reusable tooling.

## Verification
- **Before**: `git grep -cE '//#region 🔖️(Diff|Inverse)'` over the five plugins' mutation leaves = 62 files × 2 markers = 124 region blocks on direct leaves.
- **After**: 0. Re-verified with the corrected predicate (freestanding `fn diff`/`fn inverse` + missing sibling dir) directly against the filesystem — 0 residual inlined leaves in all five plugins.
- Structural: all 62 mutation dirs have `🦀️.rs` + `🔺️diff/🦀️.rs` + `↩️inverse/🦀️.rs`; no file under 20 bytes (no truncation).
- Every leaf's impl now calls exactly `super::diff::diff(self, base)` / `super::inverse::inverse(self, base)` (verified by scan — 0 files missing either delegate, 0 files with a stray leftover freestanding `pub fn diff(`/`fn inverse(`).
- `rustfmt` (project `rustfmt.toml`, not stock defaults) applied and clean across all 186 touched/created files (62 × 3) — 0 errors, confirming syntactic validity of every file.
- `git grep --untracked -l '//#region 🔖️(Diff|Inverse)'` across the five plugins now only matches inside `🔺️diff/`/`↩️inverse/` facet files (where the marker legitimately belongs, matching the vcs exemplar) — never on a direct leaf.
- `📦️glue.rs` mount counts match exactly: process 16, mathematical 15, raster 12, reasoning 10, playbook 9 (`pub mod diff;` occurrences).
- `cargo check -p semio-s-plugin-mathematical`: reached real compilation (isolated `CARGO_TARGET_DIR` to dodge shared-lock contention) but never reached my crate — it stopped at a transitive dependency, `semio-s-plugin-stdio`, which fails with 65 pre-existing errors unrelated to this ticket's slice. Exact tail:
  ```
  error: could not compile `semio-s-plugin-stdio` (lib) due to 65 previous errors; 917 warnings emitted
  ```
  Representative error (all 65 are this same shape, all inside `🗄️stdio`, none inside my five plugins):
  ```
  ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🧬️mutations/🦀️.rs:146:1: error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
  ```
  `grep -c "^error" output` = 1 unique error class repeated 65×, all under `✏️s/🔌️plugins/🗄️stdio/`; zero errors or mentions of `➗️mathematical` (or any of my five plugins) anywhere in the output. Not a build I can claim green, and not mine to fix.

## Verification against sibling `🦀️component.rs` naming regression
Sibling workers (🏗️fem, 📸️remodel, 🗒️note, 🧩️puzzle, 📕️norm) were found restoring facet leaves under the original `🦀️component.rs` name instead of the kind-only `🦀️.rs` the contract requires (`physicalLeafRendering.filename = "file-kind-emoji-and-extension-chain"`). Checked mine are clean:
  ```
  for p in 🏭️process ➗️mathematical 🖨️raster 💡️reasoning 📖️playbook; do
    find "✏️s/🔌️plugins/$p" -path '*🧬️mutations*' \( -path '*🔺️diff/🦀️component.rs' -o -path '*↩️inverse/🦀️component.rs' \)
  done
  ```
  Output: empty. All 124 of my facet files (62 × diff + inverse) are correctly named `🦀️.rs`.

## Residual sweep (repo-wide, outside my five plugins)
Per the coordinator's correction mid-task: **skipped**. The coordinator ran the repo-wide census with the corrected predicate directly (1,167 inlined leaves across 20 plugins) and is routing the residual outside my five plugins to dedicated workers (🏛️architect, 🎥️shooting/🌍️gis/🌀️procedural/🌊️flow/🎞️animate). Confirmed independently that all five of my own plugins are at 0 residual under that same corrected predicate.

## Files touched
- 62 × `D/🦀️.rs` (rewritten: regions removed, delegate calls added)
- 62 × `D/🔺️diff/🦀️.rs` (new)
- 62 × `D/↩️inverse/🦀️.rs` (new)
- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs`
- `📜️goal-facetsplit-small-extract.py` (this file, kept as the reusable extraction script)
