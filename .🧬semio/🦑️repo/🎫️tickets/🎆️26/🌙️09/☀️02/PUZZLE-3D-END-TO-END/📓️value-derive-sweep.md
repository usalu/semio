# Value-Derive Half-Migration Sweep (repo-wide)

Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS. Follow-up to the 15
sites already fixed in `✏️s/🔌️plugins/🧩️puzzle` (see
`🔨️complete-puzzle-value-derive.py` in this ticket folder). This sweep looked for every OTHER
occurrence, repo-wide, of an item-level `#[value(...)]` container attribute (immediately preceding
a `struct`/`enum` declaration) whose enclosing `#[derive(...)]` block registers neither
`ToValue` nor `FromValue`.

## Method

Wrote `🔍️scan-value-derive-gaps.py` (this ticket folder) and ran it over `✏️s/`, `🧰️framework/`,
`🌎️hub/` (excluding `target/`, `node_modules/`, `.🧬semio/`). For each `#[value(...)]` line it:

1. Walks forward past any other stacked attributes/doc-comments to confirm the next real line is a
   `struct`/`enum` declaration (not a field — field-level `#[value(...)]` is legitimate and
   untouched).
2. Walks backward through the *whole* contiguous attribute block above it, collecting every
   `#[derive(...)]` found there (Rust merges multiple stacked `#[derive(...)]` attributes on one
   item, and the puzzle fix pattern itself sometimes leaves two stacked `#[derive(...)]` blocks —
   see `GltfSnapshot` below).
3. Flags the item only if **none** of those derives contain `ToValue` or `FromValue` as a
   substring — confirmed both are independently `#[proc_macro_derive(_, attributes(value))]`
   (`🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/🦀️.rs:10,17`), so either one alone
   registers the `value` attribute and is sufficient; a bare, aliased import (e.g.
   `use dsl::{FromValue, ToValue}` in `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs`) counts
   too, since the substring match doesn't require the `value_derive::` qualifier.

Verified there is no other derive macro in the repo whose name collides with the `ToValue`/
`FromValue` substring (`grep -rn proc_macro_derive`), so the substring check has no false
negatives.

## Result: zero remaining broken sites

The scan covered 2,430 files containing 9,362 total `#[value(` occurrences (field- and item-level)
across the three roots and found **0** item-level `#[value(...)]` attributes still missing both
derives. Every candidate the raw pattern search initially flagged (62 hits before the
backward-scan was corrected to look at the whole stacked-derive block, not just the nearest single
`#[derive(...)]`) turned out to be a false positive of one of two kinds:

- **Stacked `#[derive(...)]` blocks** — e.g.
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:1319-1325`
  (`GltfSnapshot`): a first `#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue,
  value_derive::FromValue)]` already registers `value`; a second, separate
  `#[derive(Serialize, Deserialize)]` immediately above `#[value(rename_all = "camelCase")]` is the
  one the naive nearest-derive scan flagged. Already correct — no change needed.
- **`FromValue`-only test-vector structs** — the many gltf-plugin `🧪️contract/🦀️.rs` files
  (`bind-scene-root-node`, `unbind-node-child`, `create-scene`, `delete-scene`, …) declare
  deserialize-only fixture structs (`Rejected`, `Wire`, `Vector`, `Contract`, `State`, …) with
  `#[derive(value_derive::FromValue)]` only — intentional, since they only ever decode shared test
  vectors and never need `ToValue`. Not the bug pattern; left untouched.
- **Bare/aliased imports** — several framework files (`🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs`,
  `🎨️paint/🦀️.rs`, `✍️editor/🦀️.rs`, `os/🔨️modules/🔌️plugin/🦀️.rs`, `os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`,
  `os/🔨️modules/🌿️vcs/🦀️.rs`, `os/🔨️modules/📖️playbook/🦀️.rs`, `os/🔨️modules/🌊️flow/**`, and others) derive
  a bare `FromValue`/`ToValue` brought in via `use dsl::{FromValue, ToValue};` or similar — already
  correctly wired, just not spelled `value_derive::ToValue`.

**No files were changed** — no site needed a fix, and no Cargo.toml dependency check was needed as
a result (no crate lacked `semio-framework-value-derive` because no crate required the derive
added).

## Compile verification

Ran with `RUSTC_WRAPPER=""` to bypass sccache serialization, foreground, no backgrounding.

| Crate | Result | Notes |
|---|---|---|
| `semio-framework-graph` | **0 errors** (258 pre-existing warnings) | Clean on first run. |
| `semio-framework-surface` | 28 errors, all `E0277 ... serde::Serialize/Deserialize not satisfied` on `JobCheckpoint`, `ActorInstanceOpenRequest`, `ActorInstanceCloseRequest`, `ActorInstanceLifecycleAck`, `ActorInstanceLifecycleReceipt`, `ActorUiPatchReceipt` in `🧰️framework/🔨️modules/🎭️actor/🦀️.rs` / `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🦀️.rs`, surfaced through `🎠️kernel/🦀️.rs` | **Not our pattern** — these types already derive `Serialize, Deserialize, ToValue, FromValue` in source; the error shape (`manifest::_::_serde::Serialize`) points to a duplicate/mismatched `serde` resolution, not a missing derive. Unrelated to the item-level `#[value(...)]` gap this sweep targets. |
| `semio-framework-editor` | 17 errors, all `E0277 ... serde::Serialize/Deserialize/DeserializeOwned not satisfied` on `PropertyValue` (`🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs:29`) and `manifest::Manifest` in several `🤖️generated/*.rs` files, surfaced through `semio-framework-graph` as a dependency | **Confirmed live concurrent edit, not our pattern**: `PropertyValue` has `#[derive(Clone, Debug, Default, PartialEq)]` only — no `#[value(...)]` attribute at all, so it doesn't match this sweep's target pattern. The file `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs` carries a docstring dated "26/09/02 Phase 2" about deleting serde-only hooks for this exact parent ticket, and its mtime was **seconds old** (23:56:26, checked at 23:56:52) — another session is actively mid-edit on it under the same parent migration ticket. The standalone `semio-framework-graph` check had passed cleanly moments earlier; the failure appeared only once `editor`'s dependency rebuild picked up the file after it changed underneath us. Left alone per instructions — not attempting to finish someone else's in-progress refactor. |

`semio-framework-graph` was re-checked once more afterward to see if the live edit had landed on it
directly by then; that re-run did not finish within the 600s foreground window and was moved to
background by the harness (not a deliberate backgrounding) — its output was not used for any
finding in this report and no code decision depended on it.

## Skipped (nothing skipped for cause)

No item-level `#[value(...)]`-without-derive sites were found anywhere outside the already-fixed
puzzle plugin, so there was nothing to skip for a missing `semio-framework-value-derive` Cargo
dependency either — that check never became relevant.

## Files touched

None. This was a pure verification sweep — the earlier report that flagged 62 candidates was a
false-positive set from an initial, over-eager version of the scan script; the corrected script
(final version left in this ticket folder) confirms zero remain.

## Scratch files left in this ticket folder

- `🔍️scan-value-derive-gaps.py` — the corrected scan script (input/tool script, kept per ticket
  rules).
