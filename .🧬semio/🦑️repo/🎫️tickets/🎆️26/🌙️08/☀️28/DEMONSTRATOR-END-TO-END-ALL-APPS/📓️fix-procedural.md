# Fix procedural: serde/serde_json → dsl::DslValue migration to zero wasm errors

## Result

- Starting error count (wasm32-wasip2, `semio-s-plugin-procedural`): **70** (verified: `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --message-format=short 2>&1 | grep -cE ": error"` → `70`).
- Final error count: **0**, verified with the exact command from the task brief:
  ```
  cd /Users/ueli/Documents/semio && CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-engines CARGO_BUILD_JOBS=4 CARGO_PROFILE_DEV_DEBUG=false RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --message-format=short 2>&1 | grep -E ": error"
  ```
  produced no output; `cargo check` exit code was `0`.
- `semio-framework-value-derive`'s full test suite (including the new test file) passes: `14 + 12 + 4 = 30` tests green, 0 failed.

## 1. Shared derive macro extension (root-cause fix)

File: `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs`

Added automatic transparent support for single-field TUPLE structs (`struct Foo(pub u32);`) to both `expand_to_value` and `expand_from_value`, via one new match arm inserted right after the existing `container.transparent` arm and before the plain `Data::Struct(data) => { let fields = named_fields(...)? ... }` arm:

```rust
// expand_to_value
Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
    quote! { #value_crate::ToValue::to_value(&self.0) }
}
// expand_from_value
Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
    quote! { Ok(Self(#value_crate::FromValue::from_value(value)?)) }
}
```

Why this is safe / additive-only:
- Match-arm order guarantees the pre-existing `container.transparent` arm (which already special-cases `Fields::Unnamed` with `#[value(transparent)]`) is tried first; the new arm is only reached when `container.transparent` is `false`, so it never shadows existing behavior.
- Multi-field tuple structs and unit structs still fall through to the unchanged `named_fields(&data.fields, ...)` call, which still produces the same `"supports named-field structs (and #[value(tag = \"…\")] enums), not tuple/unit structs"` error as before — verified by the untouched `named_fields` function and by the full pre-existing test suite staying green.
- Nothing about named-struct or enum code paths was touched.
- Added a module-doc paragraph documenting the new automatic-newtype behavior (after the existing `#[value(transparent)]` paragraph).

New test file (repo idiom for this proc-macro crate is `tests/*.rs` integration tests, registered explicitly in `Cargo.toml` — a proc-macro crate cannot exercise its own derives from `src`; confirmed against the two existing sibling test files):
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/tests/🆔️newtype-transparent.rs` — round-trips a `u32` newtype (`NodeId(pub u32)`) and a `String` newtype (`Slug(pub String)`) through `ToValue`/`FromValue`, and checks the wire form is a bare number (no object wrapper) and that decoding a non-numeric value errors. 4 tests, all passing.
- Registered in `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` as `[[test]] name = "newtype_transparent"`.

This unblocked `id_newtype!`-generated types (`PatternId`, `TileId`, `NodeId`, `RelationId`, `ConstraintId`, …) in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/.../💡️inferences/🧩️wfc-engine/🆔️ids/🦀️.rs`.

**cad regression check**: `cargo check -p semio-s-plugin-cad --target wasm32-wasip2 --message-format=short 2>&1 | grep -cE ": error"` → 4 errors, all pre-existing/unrelated to this derive change:
- `.../✏️editor/🎚️config/🦀️.rs:320` and `.../✏️editor/👥️presence/🦀️.rs:103`: `E0046` "not all trait items implemented, missing: DESCRIPTORS, descriptor".
- `.../🎭️modes/✏️edit/🎚️options/🎥️projection/🦀️.rs:14` and `.../🌞️sun/🦀️.rs:8`: `E0631` closure/argument type mismatch.
- None of these mention `ToValue`/`FromValue`/`DslValue` — they're about an unrelated `ArtifactEditor`-style trait's associated items and a callback signature. `git status --porcelain` shows `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/.../✏️editor/🦀️.rs` (the file that owns that trait impl and the `cad_window_action` callback both error sites depend on) is currently **dirty** — a concurrent session's in-progress edit. Per the ticket's hard rules ("if a file you need is dirty AND its error looks like someone's half-finished edit, skip it and note it"), these 4 errors are left untouched; they are not caused by the additive derive-macro change and fixing them would collide with someone else's live edit.

## 2. `dsl_value_to_json` (E0425) fix

File: `✏️s/🔌️plugins/🌀️procedural/🦀️.rs`, `generation_form` (was line 163, now shifted slightly).

`flow::playbook` never exposed `dsl_value_to_json` — it exposes `default_value_for_block(question) -> DslValue` directly (already migrated) and `is_block_visible(block, values: &PlaybookValues) -> bool`. Fixed by:
- Changing `generation_form`'s `values` parameter from `&serde_json::Map<String, serde_json::Value>` to `&flow::playbook::PlaybookValues` (`= HashMap<String, DslValue>`).
- Dropping the bogus `dsl_value_to_json(...)` wrap: `values.get(&question.id).cloned().unwrap_or_else(|| flow::playbook::default_value_for_block(question))`.
- `DslValue` has the same `as_str`/`as_f64`/`as_bool` accessor names as `serde_json::Value`, so most match arms were unchanged.
- `"vector"` arm: `DslValue::as_array()` returns `Option<&[DslValue]>` (a slice, not `Vec`), so `.cloned()` (which needs `T: Clone`, and slices aren't) became `.map(<[dsl::DslValue]>::to_vec)`; the fallback branch now builds `Vec<DslValue>` via `dsl::DslValue::float(...)` instead of `serde_json::json!(...)`.
- Fallback `_` arm: `DslValue` has no `Display`/`to_string()`, so `value.to_string()` became `serde_json::Value::from(&value).to_string()`, reusing the repo's existing sanctioned `From<&DslValue> for serde_json::Value` bridge (`🧰️framework/🔨️modules/🌱️value/🦀️.rs:218`) purely for this rare human-readable-text fallback — no new serde dependency, `serde_json` was already a direct dependency of this crate.
- Two callers (`generation2d`'s and `generation3d`'s `.../🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs:42`, calling `crate::generation_form(&spec, &current.values, ...)`) needed no change — `current.values` was already `PlaybookValues` on the `FormGeneration` side.

## 3. `GenerationPlayRoot` hand-written `ToValue`/`FromValue` (unblocked by, but not covered by, item 1)

File: `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🧬️generation/🦀️.rs` (shared framework file, confirmed clean via `git status --porcelain` before editing).

The ticket assumed the derive extension alone would unblock `GenerationPlayRoot: ToValue/FromValue`, but `GenerationPlayRoot` turned out to be `pub struct GenerationPlayRoot(ManuallyDrop<Option<Arc<GenerationPlayState>>>);` with HAND-WRITTEN `impl Serialize`/`impl Deserialize` that delegate to `self.as_state()` (not a plain field-forward) — the new transparent-newtype derive arm doesn't apply here because deriving on the raw tuple field would try to serialize the `ManuallyDrop<Option<Arc<...>>>` itself, which is nonsensical.

Added hand-written `impl ToValue for GenerationPlayRoot` / `impl FromValue for GenerationPlayRoot`, directly mirroring the existing `Serialize`/`Deserialize` impls right above them (same file, same pattern already used for every other struct in the module which derives `Serialize, Deserialize, ToValue, FromValue` together during this in-flight migration):
```rust
impl ToValue for GenerationPlayRoot {
    fn to_value(&self) -> DslValue { self.as_state().to_value() }
}
impl FromValue for GenerationPlayRoot {
    fn from_value(value: DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> { GenerationPlayState::from_value(value).map(Self::from) }
}
```
`GenerationPlayState` (the `Deref` target) already derives `ToValue, FromValue` in the same file. This is purely additive — the existing `Serialize`/`Deserialize` impls are untouched (full serde removal from this module is out of scope for this ticket; the module is mid dual-impl migration, matching every other struct there).

## 4. serde_json ↔ DslValue call-site migrations (file:line)

`ChangeGenerationValue` payload field type changed from `serde_json::Value` to `dsl::DslValue` in both facets, plus every call site that built/consumed it:
- `.../generation2d/.../🧬️mutations/🔢change-generation-value/🦀️.rs` — `value: serde_json::Value` → `dsl::DslValue`, builder signature updated.
- `.../generation2d/.../🔢change-generation-value/🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs` — no `.value` conversions needed once the field type matched; only the inverse fallback (`unwrap_or(serde_json::Value::Null)` → `unwrap_or(dsl::DslValue::Null)`).
- `.../generation3d/.../🧬️mutations/🔧change-generation-value/🦀️.rs` — `new_value: Value` → `dsl::DslValue`.
- `.../generation3d/.../🔧change-generation-value/🔺️diff/🦀️.rs:13` — the no-op warning `format!(...)` used `payload.new_value` in a `Display` position; `DslValue` has no `Display`, so wrapped with `serde_json::Value::from(&payload.new_value)` (same bridge as item 2).
- `.../generation3d/.../🔧change-generation-value/↩️inverse/🦀️.rs` — `unwrap_or(Value::Null)` → `unwrap_or(dsl::DslValue::Null)`.

`generation_mutation_to_generation{2,3}d` bridge functions (`.../🧬️mutations/🦀️.rs`) needed no further edits once the payload field types above matched `flow::playbook::GenerationMutation::UpdateValues { value: DslValue }`.

`generation{2,3}d_operation_to_dsl`/`_from_dsl` (op-text `Generation{2,3}dOperationDsl::ChangeGenerationValue` variants, both already declared `value/new_value: dsl::DslValue`) in `.../🧬️mutations/💾️binary/🦀️.rs`: removed now-wrong `dsl::DslValue::from(&payload.value)` (E0277 `DslValue: From<&DslValue>` not satisfied — there is no such impl, only `serde_json::Value ↔ DslValue` bridges exist) in favor of `payload.value.clone()`/plain `value`, in both generation2d (already had this fixed for 3 of these but one at the `13 =>` binary-decode arm was missed) and generation3d.

`Generation2dMutationJsonFrame`/`Generation2dReplayDisplaced::Json`/`generation2d_copy_json`/`generation2d_copy_generation`/`generation2d_observe_json` (generation2d `.../🧬️mutations/💾️binary/🦀️.rs`): these were ALREADY migrated to `dsl::DslValue` by a prior pass; only a handful of leftover call sites still wrapped an already-`DslValue` in a stale `dsl::DslValue::from(&x)`/`serde_json::Value::from(&x)` — fixed at lines ~130, 154, 711, 1596, 1646, 2540ish (`generation2d_copy_generation`, retyped its local map to `flow::playbook::PlaybookValues`), 2883.

`generation3d_copy_json`/`generation3d_observe_json`/`generation3d_copy_generation`/`Generation3dReplayDisplaced::Json` (generation3d `.../🧬️mutations/💾️binary/🦀️.rs`): this whole mirror subsystem was **still fully on `serde_json::Value`/`Map`** (never migrated, unlike its generation2d sibling). Rewrote both recursive functions and the `Generation3dReplayDisplaced::Json` variant field to `dsl::DslValue`/`Vec<(String, DslValue)>`, matching the generation2d reference implementation exactly (same recursive shape, same `dsl::json::to_json_string` use for hashing a `Number` variant). Also fixed `generation3d_operation_to_dsl`/`_from_dsl` (lines ~140, 161) and the binary-decode `13 =>` arm (line ~1657) the same way as generation2d. Two `#[cfg(test)]` fixtures (line ~256 and the `generation3d_all_retained_mutation_fixtures_for_test` fixture) were rebuilt from `serde_json::json!(...)` literals into explicit `dsl::DslValue::object([...])`/`Array(vec![...])` construction.

`Generation3dMutationFrame::Generation`/`Generation3dMutationJsonFrame`/`self.json`/`assign_json` and every `Token::*` arm feeding it, plus the `Container::List|Map` begin/end handling (`.../generation3d/.../🧬️mutations/💾️binary/🦀️.rs`, the wire-decode state machine for `Generation3dMutation`): this was the largest single piece — a full binary-decode JSON sub-state-machine still entirely on `serde_json::Value`/`Map`, sitting right next to an ALREADY-migrated sibling `Generation3dMutationDslFrame`/`assign_dsl` system in the same file. Migrated the `json_*` half of the state machine to mirror the `dsl_*` half exactly: frame field types, `Vec<(String, DslValue)>` (push, not `.insert`) for objects, `dsl::DslValue::{float,int,uint,Bool,Null,String,Array,Object}` constructors in place of every `serde_json::Value::*`/`serde_json::Number::*` construction, and `.into_iter().collect()` where a decoded `Vec<(String, DslValue)>` needed to become the `HashMap<String, DslValue>` `FormGeneration.values` expects.

`.../generation2d/.../🧬️schema/📸️snapshot/💾️binary/🦀️.rs` and the analogous `.../generation3d/.../🧬️schema/📸️snapshot/💾️binary/🦀️.rs`: a SEPARATE, independent snapshot-decode binary state machine (`Generation{2,3}dMountedContainerOwner`/`Generation{2,3}dMountedGenerationOwner`/`Generation{2,3}dMountedJsonFrame`/`assign_json`) with the exact same still-on-serde_json shape as the mutations one above. Migrated identically (frame types, constructors, `.push` not `.insert`, `.into_iter().collect()` at the `FormGeneration` construction site, two `#[cfg(test)]` fixtures rebuilt with `dsl::DslValue::object`/`Array`).

`.../generation2d/.../🧬️schema/📸️snapshot/📝️text/🦀️.rs` (`FormGenerationDsl`, the op-text twin — already `values: BTreeMap<String, dsl::DslValue>`): `form_generation_to_dsl`/`form_generation_from_dsl` had stale `dsl::DslValue::from(value)` / `serde_json::Value::from(value)` wraps around already-`DslValue` values; simplified to plain clones/moves.

`.../generation3d/.../🧬️schema/🦀️.rs`: `generation_values_to_pack_object`/`evaluate_generation_preview` retyped from `&serde_json::Map<String, Value>` to `&flow::playbook::PlaybookValues`, body simplified (`dsl::DslValue::object(values.clone())` instead of round-tripping through a synthetic `serde_json::Value::Object`). Same fix mirrored in `.../generation2d/.../🧬️schema/🦀️.rs`'s `evaluate_generation_preview`.

`.../generation2d/.../✏️editor/🦀️.rs:652` (`export_media`'s `"drawing:out"` port): call site `evaluate_generation_preview(&doc.snapshot.fixture, &serde_json::Map::new())` → `&flow::playbook::PlaybookValues::new()`.

Generate-mode command files — `handle_generation`'s `args: Option<&serde_json::Value>` → `Option<&dsl::DslValue>`, and every JSON-args builder call site rebuilt with `dsl::DslValue::object([...])`/`dsl::DslValue::String(...)`/`dsl::DslValue::Null` instead of `serde_json::json!(...)`:
- generation2d: `add-generation`, `remove-generation`, `rename-generation`, `select-generation` (payload's `id` is `Option<String>` here, unlike generation3d's plain `String` — handled with `.map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null)`), `update-generation-values` (three-key args object, `generationId` optional).
- generation3d: the same five files, mirrored (plain `String` ids here).

`.../generation3d/.../✏️editor/🦀️.rs` — three call sites decoding `MeshData` from JSON text/value. `MeshData` (defined in `🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs`) only derives `serde::{Serialize,Deserialize}` under `#[cfg(test)]` now; in production it has its own first-party `impl pack::value::FromValue for MeshData` (confirmed `pack::value` = `pub use protocol::value;` = the exact same `🧰️framework/🔨️modules/🌱️value/🦀️.rs` file compiled into both `protocol` and, via `os_dsl::schema`'s `pub use protocol::value::{DslValue, FromValue, ToValue, ValueError, ...}`, into `dsl` — i.e. one canonical `DslValue`/`FromValue` type across the whole dependency graph). Fixed:
- Two `serde_json::from_str::<MeshData>(json)` call sites → `dsl::json::from_json_str::<MeshData>(json)` (the repo's own `serde_json::from_str` analog over `FromValue`, `🎒️pack/🔤️json/🦀️.rs`).
- One `serde_json::from_value::<MeshData>(serde_json::Value::from(dsl::json::to_dsl_value(&data))).ok()` → `dsl::FromValue::from_value(dsl::json::to_dsl_value(&data)).ok()`, dropping the pointless double bridge through `serde_json::Value`.
- Updated the two stale doc-comments that said "MeshData has no FromValue" / "decoding stays on serde_json" to reflect the current state.

## Skipped / out of scope

- The 4 remaining `semio-s-plugin-cad` errors (E0046 `DESCRIPTORS`/`descriptor`, E0631 closure mismatch in `.../✏️editor/🎚️config/🦀️.rs`, `.../👥️presence/🦀️.rs`, `.../🎭️modes/✏️edit/🎚️options/🎥️projection/🦀️.rs`, `.../🌞️sun/🦀️.rs`) — left untouched. `git status --porcelain` shows `.../📐️cad/.../✏️editor/🦀️.rs` (and two command files) dirty; these errors are a concurrent session's in-progress edit to an unrelated `ArtifactEditor`-style trait and a callback signature, not caused by the additive derive-macro change (no `ToValue`/`FromValue`/`DslValue` involvement). Per the ticket's rule on dirty half-finished edits, not fixed here.
- Full serde/serde_json removal from `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🧬️generation/🦀️.rs` and `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs` (both still carry `Serialize`/`Deserialize` impls/derives alongside `ToValue`/`FromValue`) — out of scope for `semio-s-plugin-procedural`; every struct in that module is mid dual-impl migration and this ticket only needed the missing `ToValue`/`FromValue` half added, additively.
- `serde_json` remains a direct dependency of `semio-s-plugin-procedural` (`Cargo.toml`, `serde_json = { workspace = true, features = ["float_roundtrip"] }`) — used legitimately for: the sanctioned `DslValue ↔ serde_json::Value` bridge (`serde_json::Value::from(&dsl_value)`) in a few human-readable-text/no-op-message fallback spots, and genuine test-oracle JSON fixture parsing (`generation3d_all_retained_mutation_fixtures_for_test`'s neighbor test helpers at `.../mutations/💾️binary/🦀️.rs:~3505`). Not removed — that's the wider repo migration's job, not this ticket's.

## Files changed

- `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs` (shared derive macro — additive)
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` (new test registration)
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/tests/🆔️newtype-transparent.rs` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🧬️generation/🦀️.rs` (hand-written ToValue/FromValue for GenerationPlayRoot)
- `✏️s/🔌️plugins/🌀️procedural/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-generation-value/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-generation-value/↩️inverse/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️{add,remove,rename,select}-generation/🦀️.rs`, `🧬️update-generation-values/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧change-generation-value/🦀️.rs`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️{add,remove,rename,select}-generation/🦀️.rs`, `🧬️update-generation-values/🦀️.rs`

No mutating git commands were run. No ticket was opened/closed/reopened by this work (per instruction, this fix is reported directly to the `DEMONSTRATOR-END-TO-END-ALL-APPS` ticket folder without touching its ticket lifecycle).
