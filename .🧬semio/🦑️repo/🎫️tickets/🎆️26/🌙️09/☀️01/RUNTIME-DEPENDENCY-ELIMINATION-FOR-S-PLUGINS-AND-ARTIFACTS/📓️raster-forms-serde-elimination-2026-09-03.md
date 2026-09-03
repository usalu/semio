# Raster + Forms serde elimination — 2026-09-03

Target: `✏️s/🔌️plugins/🖨️raster` and `✏️s/🔌️plugins/📋️forms`. Zero cargo commands run, zero sub-agents
spawned. Verified by re-reading every edited region on disk and by grep after each file.

## Real before/after counts

Audit command (excludes 🧪/🏭/🔬 dirs and comment-only lines):
```
grep -rn 'serde_json\|use serde\|derive([^)]*Serialize' <plugin> --include='*.rs' \
  | grep -vE '🧪|🏭|🔬' | grep -vE ':\s*(///|//!|//|\*)'
```

| Plugin | Before | After | Delta |
|---|---|---|---|
| 🖨️raster | 83 | 7 | -76 |
| 📋️forms | 84 | 16 | -68 |

**Every remaining raster ref (7) is `#[cfg(test)]`-gated** — either a genuine oracle test comparing
against a hand-authored fixture (`🗿️artifacts/🖨️raster/🦀️.rs`'s `RasterAssetChild` wire-roundtrip test)
or a `#[cfg_attr(test, derive(Serialize, Deserialize))]` + matching `#[cfg(test)] use serde::{...}`
pair kept for that same reason (`🧬️schema/🦀️.rs`).

**15 of the 16 remaining forms refs are the same pattern** (cfg-gated oracle tests / cfg_attr derives
in `🧬️schema/🦀️.rs`, `🧬️schema/💡️inferences/🦀️.rs`, `✏️editor/🎚️config/🦀️.rs`, and the wire-roundtrip
oracle test in the main `🗿️artifacts/📋️forms/🦀️.rs`).

**1 forms ref is a genuine, unavoidable production blocker**, not something I could convert:
`✏️editor/📌️panels/🛍️catalogue/🦀️.rs` still has `use serde_json::json;` because
`semio_framework_plugin::tree_item_with_action_draggable`'s `drag_data` parameter is typed
`&serde_json::Value` in the framework itself (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5796`,
importing `serde_json::Value` at its own line 288). Framework is DO-NOT-TOUCH. This is the one
call site left; flagging for whoever eventually converts that framework signature.

## `git diff --name-only` (mine only — two extra hits were concurrent, unrelated edits by other
sessions and are excluded below: `📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
(a `MathematicalDiff`→`EquationDiff` comment rename) and `🖨️raster/…/🚪️io/🦀️.rs` (a
`subsets::any::schema::geometry` → `subsets::base::schema::geometry` import path rename) — neither
touches serde and neither was made by me.)

Raster (24 files):
```
🗿️artifacts/🖨️raster/🦀️.rs
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️patch-layer/🦀️.rs
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️patch-layers/🦀️.rs
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs
🗿️artifacts/🖨️raster/🏅️标准/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs
+ 12 mutation-leaf files (🗑️delete-layer, 🗂️remove-layer-asset, 🖇️add-layer-asset, 🔀reorder-layers,
  📐resize-layer, 👁️change-layer-visible, 🎨change-layer-blend-mode, 🎚️change-layer-adjustment-kind,
  🌱create-layer, 🌫️change-layer-opacity, ✏️rename-layer, ↔️move-layer) under 🧬️schema/🧬️mutations/
🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs
```

Forms (30 files):
```
🗿️artifacts/📋️forms/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/▶️try/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs (note only — see below)
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❓️add-question/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❓️drop-question-kind/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❓️move-question/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❓️patch-questions/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️patch-vector-field/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️set-spec-json/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔘️patch-question-options/🦀️.rs
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs
+ 8 mutation-leaf files (✏️rename-step, ➕create-block, ➖delete-block, 🌱create-step,
  🏷️change-form-title, 📝change-step-description, 📦move-block-to-step, 🔀reorder-step,
  🔁replace-block, 🗑️delete-step) under 🧬️schema/🧬️mutations/*/🦠️mutation/
```
NOT converted (left as-is, production, DO-NOT-TOUCH boundary):
```
🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs
```

## Notable defects found and fixed along the way (not pure mechanical swaps)

1. **Bulk-strip script briefly orphaned three `#[cfg(test)]` attributes** (raster
   `🧬️schema/🦀️.rs`, forms `🧬️schema/🦀️.rs`, forms `🧬️schema/💡️inferences/🦀️.rs`) — my first pass
   removed only the exact-match `use serde::{Deserialize, Serialize};` line, leaving a bare
   `#[cfg(test)]` attribute that then silently re-attached to the NEXT item (in one case gating the
   whole `RasterArtifact`/`FormsArtifact` struct behind `#[cfg(test)]`, in another gating forms'
   production `use serde_json::Value;` that `initial_try_values` needs unconditionally). Caught by
   scanning every file for `#[cfg(test)]` immediately followed by a blank/comment line, and by
   diffing against the struct's own `#[cfg_attr(test, derive(Serialize, Deserialize))]` to see which
   derives had no matching import. Fixed by restoring the intended
   `#[cfg(test)] use serde::{Deserialize, Serialize};` line in all three files.
2. **`RasterOwnedMap<V>` and `FormsTryValues`** both already had a hand-written serde
   `Serialize`/`Deserialize` pair living alongside an ALREADY-PRESENT first-party `dsl::DslField`
   impl (raster) or none at all (forms). Raster's `dsl::ToValue`/`dsl::FromValue` impls already
   existed too (someone else's earlier pass) — I only deleted the now-redundant serde half. Forms'
   `FormsTryValues` had NO `dsl::ToValue`/`dsl::FromValue` impl at all despite `FormsConfig` deriving
   `dsl::ToValue, dsl::FromValue, dsl::DslArtifact` unconditionally over a `try_values: FormsTryValues`
   field — I wrote the missing `impl dsl::ToValue for FormsTryValues` / `impl dsl::FromValue for
   FormsTryValues`, mirroring the deleted serde impl's shape 1:1.
3. **`forms_config/🦀️.rs`'s `#[cfg(test)] use std::collections::BTreeMap;`** was gating an import
   that `TryValueBlob`/`TryValuesBatch` registries use UNCONDITIONALLY (`static ... OnceLock<Mutex<
   BTreeMap<...>>>` at module scope) — a pre-existing bug unrelated to serde, only surfaced because I
   was touching the same import block. Made the import unconditional.
4. **`schema/🦀️.rs`'s `initial_try_values`** was declared as `overrides: &serde_json::Map<String,
   Value>` but forwarded straight into `crate::playbook::initial_values(&spec, overrides)`, whose real
   signature (`🧰️framework/…/📖️playbook/🦀️.rs:305`) is `&HashMap<String, DslValue>` — a
   pre-existing type mismatch, not something serde removal could leave untouched. Rewrote it to
   bridge through `dsl::os_pack::json::to_dsl_value`/`from_dsl_value` explicitly (build the
   `HashMap<String, DslValue>`, call the real fn, convert the result back to `Object`).
5. **Legacy `dsl::to_dsl_value(&serde_json::json!(...))` bridge calls** — found in raster's
   `🧬️mutations/🦀️.rs`, `📸️snapshot/💾️binary/🦀️.rs`, `📸️snapshot/📝️text/🦀️.rs`, `🧬️schema/🦀️.rs`
   (same fixture-params block copy-pasted four times) — replaced with direct `dsl::DslValue`
   literal construction (`DslValue::float`, `DslValue::Bool`, `DslValue::Array`, `DslValue::Object`),
   removing the serde_json round-trip entirely rather than just re-typing it.
6. **A test that asserted on the OLD serde behaviour** (raster `🧬️mutations/💾️binary/🦀️.rs`, the
   "populated serde output …" test): the removed serde `Serialize` returned a graceful `Err` for a
   populated `RasterOwnedMap`, but the ALREADY-EXISTING `dsl::ToValue` impl `assert!`-panics instead.
   Updated the test's assertion from `matches!(output, Ok(Err(_)))` to `output.is_err()` to match the
   real (panicking) dsl behaviour, and swapped the exercised call from `serde_json::to_vec(&layer)` to
   `dsl::ToValue::to_value(&layer)`.

## Pre-existing issues discovered but NOT touched (out of scope / can't verify without cargo)

- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml` has no `semio-framework-value-derive`
  dependency, yet 16 raster files (e.g. `✏️editor/🎮️commands/🎥️set-camera/🦀️.rs`,
  `🖼️add-layer/🦀️.rs`, …) already do `use semio_framework_value_derive::{FromValue, ToValue};`
  directly. These files carry zero serde refs so they were outside my grep's scope; left untouched.
  Forms' equivalent Cargo.toml DOES list `semio-framework-value-derive` explicitly, so the same
  pattern in ~28 forms command files is fine.
- `✏️s/🔌️plugins/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/
  ▶️try/🦀️.rs`'s `try_value_action`/`render_try_question` pass a JSON `Value`
  (`object([("key", …)])`) where `forms_action`'s real signature
  (`✏️editor/🦀️.rs:46`) expects `Option<semio_framework_plugin::UiValue>` — `UiValue` has no
  `From<Value>`/`From<serde_json::Value>` impl anywhere in the framework. This looks like a
  pre-existing type mismatch (present before my edit, same shape either with `serde_json::Value` or
  the new `dsl::os_pack::json::Value` — the swap is neutral, not a regression). Not fixed; would need
  either a `UiValue` conversion path or the JSON payload to be plumbed differently, and is outside
  this ticket's serde-removal remit.

## Verification commands run (results clean)

```
grep -rn '\.get([0-9]' <plugin>          → no hits in either plugin (no array-index-as-object-key bugs)
grep -rn 'unwrap_or_else' <plugin>       → reviewed every hit; none are stale post-infallible-swap leftovers
```

Zero `cargo` commands run. Zero sub-agents spawned. All work done directly via Read/Edit/Bash+python
text substitution, each file re-read after edit to confirm on-disk state.
