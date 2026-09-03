# Imperative + Reasoning — driven to 0 real production serde refs

Measured with `python3 /tmp/prodserde.py <dir> <lines>` (strips `#[cfg(test)] mod`, `#[cfg(test)]` on
single items, `cfg_attr(test`, and comments).

## Before/after

| Plugin | Before | After |
|---|---|---|
| `✏️s/🔌️plugins/📜️imperative` | 61 | **0** |
| `✏️s/🔌️plugins/💡️reasoning` | 75 | **2** (one framework-blocked file, see below) |

## Remaining reasoning refs — framework blocker, not bridged

`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:40,141`
(`wires_select_action_args`) bottoms out in
`🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:803`:

```rust
pub fn optional_json_to_dsl(args: Option<serde_json::Value>) -> Option<DslValue>
```

This is a genuine framework signature requiring `serde_json::Value` — per the ticket's hazard list,
NOT bridged (no `serde_json::Value::from(...)`, no bare `From<&DslValue>`). Left as-is and reported;
fixing `optional_json_to_dsl` to accept a first-party value type at the framework end would clear
this last ref, same pattern that unblocked other plugins tonight.

## Conversion approach

Both plugins already had the `dsl`/`protocol`/`store`/`schema` extern-crate aliases declared at
their real crate roots (`📦️packages/🦀️rust/🦀️.rs`, distinct from the taxonomy-root `🦀️.rs` stub
files most editing happens in) — confirmed before writing any import, per the ticket's warning about
non-resolving paths. No Cargo.toml touched.

- `#[derive(Serialize, Deserialize, X)]` → `#[derive(dsl::ToValue, dsl::FromValue, X)]` for every
  schema/diff/mutation/config/presence type (`X` ∈ `ArtifactSchema`, `DslRecord`, `DslArtifact`,
  `DslEnum`, `DslOps`, `MutationLeaf`, `Mutations` — all proven to coexist with `ToValue`/`FromValue`
  in `🖨️raster`/`📖️playbook`/`🧱️block`).
- `#[serde(...)]` → `#[value(...)]` (`rename_all`, `default`, `default = "fn"`,
  `skip_serializing_if` all carry over verbatim).
- `serde_json::to_string`/`from_str` → `dsl::os_pack::json::to_json_string`/`from_json_str`
  (infallible `to_json_string` — every `.unwrap_or_default()`/`.unwrap_or_else(...)` fallback around
  it deleted, matching the ticket's INFALLIBLE rule).
- Bare `dsl::to_dsl_value(&serde_json::json!({...}))` (broken in both plugins — `to_dsl_value<T:
  ToValue>` doesn't accept `serde_json::Value`, so this was pre-existing dead-on-arrival code, not
  something I introduced) → hand-built via `DslValue::object([...])`/`DslValue::uint/int/float/String`
  literals, matching the plugins' own existing idiom (`wires/🦀️.rs`'s `empty_board_fixture`).
- `serde_json::Value` used as a generic dynamic-JSON scratch type (canvas layer building, inspection
  fixture parsing) → `dsl::os_pack::json::Value` (`pack::json::Value`, first-party, has
  `object()`/`array()`/`From<&str|String|u64|i64|f64|bool>` constructors mirroring `serde_json::json!`).
- `WiresError`/`ImperativeCoreError`'s `Json(serde_json::Error)` variant → `Json(dsl::ValueError)` or
  `Json(dsl::os_pack::json::JsonError)` depending on whether the call site decodes into a typed
  `FromValue` target (`from_json_str`, error = `ValueError`) or a dynamic tree (`parse`, error =
  `JsonError`).
- `ProcedureSnapshot`/`WiresSnapshot`/`ProcedureArtifact`/`WiresArtifact` (referenced from
  `ProcedureDiff`/`WiresDiff` via `Option<Box<...>>`) kept `#[cfg_attr(test, derive(serde::Serialize,
  serde::Deserialize))]` + matching `#[cfg_attr(test, serde(...))]` — confirmed live serde-oracle
  fixture tests (`serde_json::from_str::<ProcedureDiff/WiresDiff>`) under each mutation's
  `🧪️tests/<slug>/🦀️.rs`, never deleted.
- `neural_engine::{Dictionary, Value, Atom}` (imperative) already carry production `ToValue`/
  `FromValue` (a prior pass, `📓️orderedmap-tenth-seam.md`) — used directly, no bridging needed.
  `imperative_engine::{Path, Step}` likewise already derive `ToValue`/`FromValue`.
- `seed_dictionary` (imperative `🗿️artifacts/📜️procedure/🦀️.rs`) rewritten from a
  `serde_json::to_value`/`from_value` round trip to a direct `Dictionary::insert` fold — faster, no
  JSON detour at all.
- `dsl_to_json`/`fixture_json_string` (reasoning `🧬️schema/🦀️.rs`) were calling the WRONG generic
  helper (`dsl::from_dsl_value::<T: FromValue>` used to target `serde_json::Value`, which isn't
  `FromValue` — pre-existing broken code) — `fixture_json_string` now goes straight through
  `to_json_string` (drops the intermediate `Value` entirely); `dsl_to_json` now returns
  `dsl::os_pack::json::Value` via the real `from_dsl_value(&DslValue) -> pack::json::Value` bridge.
  Its four consumer files (two canvas windows, inspection panel, its own test module) updated in
  lockstep since the return type changed.

## Files touched (this session only — do not attribute other hunks in these directories to me)

`git diff --name-only` against this plugin tree also shows unrelated concurrent edits from other
sessions (e.g. reasoning's `📦️packages/🦀️rust/🦀️.rs`, svg/png import-export serializer leaves) —
NOT touched by me, listed below only the files I personally edited.

### `✏️s/🔌️plugins/📜️imperative/` (23 files)
- `🗿️artifacts/📜️procedure/🦀️.rs`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `.../🧬️schema/📸️snapshot/🦀️.rs`
- `.../🧬️schema/📸️snapshot/📝️text/🦀️.rs`
- `.../🧬️schema/🧬️mutations/🦀️.rs`
- `.../🧬️mutations/🔧edit-step-params/🦀️.rs`
- `.../🧬️mutations/🔀reorder-steps/🦀️.rs`
- `.../🧬️mutations/🌱create-step/🦀️.rs`
- `.../🧬️mutations/🗑️delete-step/🦀️.rs`
- `.../🧬️schema/🔺️diff/🦀️.rs`
- `.../🧬️schema/⚙️operations/🦀️.rs`
- `.../🧬️schema/💡️inferences/🦀️.rs`
- `.../🧬️schema/💡️inferences/🧭topology/🦀️.rs`
- `.../✏️editor/🦀️.rs`
- `.../✏️editor/⚙️engine/🦀️.rs`
- `.../✏️editor/🎚️config/🦀️.rs`
- `.../✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `.../✏️editor/👥️presence/🦀️.rs`
- `.../✏️editor/👥️presence/🧬️schema/🦀️.rs`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📋️main/🦀️.rs`
- `.../✏️editor/🎮️commands/👁️run/🦀️.rs`
- `.../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`
- `.../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`

### `✏️s/🔌️plugins/💡️reasoning/` (31 files)
- `🗿️artifacts/🔌️wires/🦀️.rs`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`
- `.../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`
- `.../🧬️schema/🦀️.rs`
- `.../🧬️schema/📸️snapshot/🦀️.rs`
- `.../🧬️schema/🔺️diff/🦀️.rs`
- `.../🧬️schema/🧬️mutations/🦀️.rs`
- `.../🧬️mutations/📐resize-node/🦀️.rs`
- `.../🧬️mutations/🏷️change-node-kind/🦀️.rs`
- `.../🧬️mutations/🚩set-node-root/🦀️.rs`
- `.../🧬️mutations/🗑️delete-node/🦀️.rs`
- `.../🧬️mutations/🌱create-node/🦀️.rs`
- `.../🧬️mutations/🧭move-node/🦀️.rs`
- `.../🧬️mutations/✂️disconnect-nodes/🦀️.rs`
- `.../🧬️mutations/🔗connect-nodes/🦀️.rs`
- `.../🧬️mutations/✏️edit-node-text/🦀️.rs`
- `.../🧬️mutations/🔷change-node-shape/🦀️.rs`
- `.../🧬️schema/💡️inferences/🦀️.rs`
- `.../🧬️schema/💡️inferences/🧭topology/🦀️.rs`
- `.../👁️viewer/🎭️modes/👁️view/🪟️windows/🕸️canvas/🦀️.rs`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🦀️.rs`
- `.../✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `.../✏️editor/🎚️config/🦀️.rs`
- `.../✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `.../✏️editor/👥️presence/🦀️.rs`
- `.../✏️editor/👥️presence/🧬️schema/🦀️.rs`
- `.../✏️editor/📌️panels/📄️artifact/🦀️.rs`
- `.../✏️editor/🎮️commands/🔵️add-node/🦀️.rs`
- `.../✏️editor/🎮️commands/🔗️add-relationship/🦀️.rs`
- `.../🚪️io/📸️snapshot/💾️binary/🦀️.rs`
- `.../🚪️io/📸️snapshot/📝️text/🦀️.rs`

## Confirmations

- Zero `cargo` commands run this session.
- Zero sub-agents spawned.
- No Cargo.toml edited.
- No `git commit`/`stash`/`checkout`/worktree used.
- Every region re-read on disk after editing (via the Edit tool's own post-edit diff, plus targeted
  `grep`/`sed -n` spot checks) before moving to the next file.
