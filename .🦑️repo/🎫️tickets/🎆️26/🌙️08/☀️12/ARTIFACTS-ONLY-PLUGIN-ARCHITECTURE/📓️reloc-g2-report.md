# reloc-g2: declaration() + pilot_languages() relocation — gis / dag / sourcing

Scope: move `declaration()` (and its private helper `pilot_languages()`, kept private) out of
`⚙️engine/🦀️component.rs` and onto the artifact root `🦀️component.rs`, per the REVISED instructions
(move BOTH, widen neither). `📕️norm` and `🧱️block` are owned by another session and were not touched.

## 🌍️ gis (crate `semio-s-plugin-gis`) — two artifacts

Both artifacts had **already been relocated by an earlier pass** — `declaration()` was already living
at the artifact root, with the call site (`✏️s/🔌️plugins/🌍️gis/🦀️component.rs`) already pointing at
`crate::artifacts::<x>::declaration()`. The one thing the earlier pass got wrong (the exact defect this
REVISED dispatch exists to correct): `pilot_languages()` had been **left behind in `⚙️engine`, marked
`pub`**, and `declaration()` called it qualified as `…::engine::pilot_languages()`. Fixed by moving
`pilot_languages()` (private) alongside `declaration()` and switching the call to unqualified.

### 🗺️ gismap
- Moved: `pilot_languages()` — before `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:443` (`pub fn`) → after `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️component.rs:97` (private `fn`, no other edits — `declaration()` itself was already at root:83, untouched in shape, only its `.languages(...)` call switched from `…engine::pilot_languages()` to `pilot_languages()`).
- move-both: N/A (declaration() pre-relocated); pilot_languages()-only move-and-privatize applied cleanly. No deviation — single caller confirmed (`declaration()` at root), no second caller, fully qualified body.

### 🏔️ gisterrain
- Moved: `pilot_languages()` — before `.../🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:233` (`pub fn`) → after `.../🏔️gisterrain/🦀️component.rs:63` (private `fn`); `declaration()` (already at root:49) call switched from `…engine::pilot_languages()` to `pilot_languages()`.
- Same pattern as gismap, no deviation.

### Stale-comment cleanup (not code, but false hits on the verify greps)
- `✏️s/🔌️plugins/🌍️gis/🦀️component.rs:10` — doc comment said "moved onto each artifact's own `engine::declaration()`"; corrected to "moved onto each artifact's own root `declaration()`" (declaration was never in engine by the time this dispatch started; only the comment was stale).

### Verify — gis
```
grep -rn "fn declaration" ✏️s/🔌️plugins/🌍️gis
  🗿️artifacts/🗺️gismap/🦀️component.rs:83:pub fn declaration()
  🗿️artifacts/🏔️gisterrain/🦀️component.rs:49:pub fn declaration()
grep -rn "engine::declaration" ✏️s/🔌️plugins/🌍️gis   → 0 hits
grep -rn "pub fn pilot_languages" ✏️s/🔌️plugins/🌍️gis → 0 hits
grep -rn "fn pilot_languages" ✏️s/🔌️plugins/🌍️gis
  🗿️artifacts/🗺️gismap/🦀️component.rs:97:fn pilot_languages()   (private)
  🗿️artifacts/🏔️gisterrain/🦀️component.rs:63:fn pilot_languages() (private)
```
`#[path]` check (📦️glue.rs): all entries resolve on disk — PASS (no file was moved/renamed, only
in-file function relocation).

### cargo check — gis
`RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-gis --all-targets`
→ **FAILS**, 3 errors, **none attributable to this transform** (none mention `declaration`,
`pilot_languages`, or any file this dispatch touched):
```
error[E0433]: cannot find `modules` in `crate`
  --> .../🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:10:12
  |  use crate::modules::terrain::{TerrainDescriptorJson, TerrainPositionData, TerrainProjectOrigin};

error[E0433]: cannot find `modules` in `crate`
  --> .../🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs:11:12
  |  use crate::modules::terrain::{build_terrain_scene_json, TerrainDescriptorJson};

error[E0432]: unresolved imports `super::TerrainDescriptorJson`, `super::TerrainPositionData`, `super::TerrainProjectOrigin`
  --> .../🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:33:17
```
`crate::modules::terrain` is not declared anywhere in this crate's `📦️glue.rs` (only
`crate::artifacts::gisterrain::standards::v1::engine::terrain` and
`crate::apps::gis3d::…::windows::terrain` exist as `pub mod terrain` under other parents) — a broken
import pre-existing in the working tree at both sites, in files this dispatch never opened for editing.
`git diff` on the one file I did edit here (`⚙️engine/🦀️component.rs`) confirms the only hunks are the
`pilot_languages()` removal; line 10's import statement is untouched. Classed as **pre-existing /
concurrent-session churn, not upstream-stdio, not mine** — full raw output:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-g2-gis-check.txt`.

## 🕸️ dag (crate `semio-s-plugin-dag`) — one artifact

`declaration()` and `pilot_languages()` were still both in `⚙️engine`
(`pilot_languages()` already private there — no revert needed). No other local dependency in either
body; qualified-only refs throughout (`crate::artifacts::dag::…`, `dsl::…`).

- Moved both — before `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:24` (`declaration()`) / `:37` (`pilot_languages()`, private) → after `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️component.rs:72` (`declaration()`) / `:86` (`pilot_languages()`, private).
- Call site updated: `✏️s/🔌️plugins/🕸️dag/🦀️component.rs:15` `crate::artifacts::dag::engine::declaration()` → `crate::artifacts::dag::declaration()`.
- move-both held cleanly. No deviation.

### Verify — dag
```
grep -rn "fn declaration" ✏️s/🔌️plugins/🕸️dag       → 🗿️artifacts/🕸️dag/🦀️component.rs:72:pub fn declaration()
grep -rn "engine::declaration" ✏️s/🔌️plugins/🕸️dag  → 0 hits
grep -rn "pub fn pilot_languages" ✏️s/🔌️plugins/🕸️dag → 0 hits
grep -rn "fn pilot_languages" ✏️s/🔌️plugins/🕸️dag    → 🗿️artifacts/🕸️dag/🦀️component.rs:86:fn pilot_languages() (private)
```
`#[path]` check (📦️glue.rs): all entries resolve — PASS.

### cargo check — dag
`cargo check -p semio-s-plugin-dag --all-targets` → **FAILS**, 5 lib errors (8 incl. tests), **none
attributable to this transform**:
```
error[E0560]: struct `MdSnapshot` has no field named `body`          (.../🚪️io/📤️export/…/📝️md/…)
error[E0609]: no field `body` on type `&MdSnapshot`                  (.../🚪️io/📥️import/…/📝️md/…)
error[E0063]: missing fields `properties`,`route_style` on DagFixtureEdge (×2, .../🧬️schema/💡️inferences/…)
error[E0599]: no method `apply` on DagDiff                           (.../🧬️schema/🧬️mutations/…)
error[E0308]: JsonValue vs Value mismatch (×2, .../🚪️io/{export,import}/…/🔣️json/…)
error[E0599]: no method `inverse` on DagMutation (missing `use crate::store::Mutation`) (.../🧬️schema/🧬️mutations/…)
```
None touch `declaration`, `pilot_languages`, or any file this dispatch edited (root/engine
`🦀️component.rs`, plugin-root `🦀️component.rs`) — schema/mutation/io churn from another session mid
refactor. Classed as **pre-existing / concurrent-session churn, not upstream-stdio, not mine**. Full
raw output: `.../scratch-g2-dag-check.txt`.

## 🪵️ sourcing (crate `semio-s-plugin-sourcing`) — one artifact (`🗂️curate`)

`declaration()` and `pilot_languages()` were still both in `⚙️engine` (`pilot_languages()` already
private — no revert needed). Body fully qualified (`crate::artifacts::curate::…`, `dsl::…`); no other
local dependency.

- Moved both — before `.../🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:25` (`declaration()`) / `:38` (`pilot_languages()`, private) → after `.../🗂️curate/🦀️component.rs:148` (`declaration()`) / `:162` (`pilot_languages()`, private).
- Call site updated: `✏️s/🔌️plugins/🪵️sourcing/🦀️component.rs:11` `crate::artifacts::curate::engine::declaration()` → `crate::artifacts::curate::declaration()`.
- Stale-comment cleanup (false hit on verify grep): `.../🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:2` said "declared once by ⚙️engine::declaration" — corrected to "declared once by the artifact root's `declaration()`".
- move-both held cleanly. No deviation.

### Verify — sourcing
```
grep -rn "fn declaration" ✏️s/🔌️plugins/🪵️sourcing        → 🗿️artifacts/🗂️curate/🦀️component.rs:148:pub fn declaration()
grep -rn "engine::declaration" ✏️s/🔌️plugins/🪵️sourcing    → 0 hits
grep -rn "pub fn pilot_languages" ✏️s/🔌️plugins/🪵️sourcing → 0 hits
grep -rn "fn pilot_languages" ✏️s/🔌️plugins/🪵️sourcing     → 🗿️artifacts/🗂️curate/🦀️component.rs:162:fn pilot_languages() (private)
```
`#[path]` check (📦️glue.rs, plugin + 3 extensions `🪵️beams`/`🧱️slabs`/`🪟️windows`): all entries resolve
— PASS.

### cargo check — sourcing
`cargo check -p semio-s-plugin-sourcing --all-targets` → **PASSES**:
```
warning: `semio-s-plugin-sourcing` (lib test) generated 14 warnings (12 duplicates)
warning: `semio-s-plugin-sourcing` (lib) generated 13 warnings
    Finished `dev` profile [unoptimized] target(s) in 5m 21s
```
Zero errors (13/14 pre-existing lint warnings only, none new from this move — e.g. an unused
`ArtifactAnalyzer` import and an unused `item` binding, both in code untouched by this dispatch). Full
raw output: `.../scratch-g2-sourcing-check.txt`.

## apa-status
COMPLETE for all three plugins:
- **gis** (2/2 artifacts: gismap, gisterrain) — relocation done, verify greps clean, `cargo check`
  FAILS but with 0 errors attributable to this transform (pre-existing `crate::modules::terrain`
  breakage in files never touched here).
- **dag** (1/1 artifact) — relocation done, verify greps clean, `cargo check` FAILS but with 0 errors
  attributable to this transform (pre-existing schema/mutation/io churn from another session, in files
  never touched here).
- **sourcing** (1/1 artifact) — relocation done, verify greps clean, `cargo check` **PASSES clean**
  (warnings only).

All plugin-root call sites now point at `crate::artifacts::<x>::declaration()` (no `engine::` in the
path). No file was moved, renamed, or had its directory touched; `🧬️mutations/**` untouched; no
artifact-kind ids renamed; `📕️norm`/`🧱️block` untouched. Two stray stale doc-comments (not code, but
false hits on the `engine::declaration` verify grep) were corrected in passing — noted above per plugin.
