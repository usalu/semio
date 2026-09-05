# Fix flow-panels-retained slice (semio-s-plugin-flow)

Scope: the file list handed to this agent under
`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/` plus the two
`🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs` files. No `cargo` was run; every fix below is statically
reasoned from sibling compiling code (cad/puzzle/procedural/sourcing) and from the trait/type
definitions the errors point at.

## 1. What replaced serde for `FlowSnapshot` (and the analogous `CsvSnapshot`/`DuplicateWidgetStep`)

`FlowSnapshot` (`🧬️schema/📸️snapshot/🦀️.rs:19`) derives `value_derive::ToValue`/`FromValue` and only
derives `serde::Serialize`/`Deserialize` under `#[cfg(test)]`. The oracle for "what it serializes
through now" is `store::ArtifactStore::snapshot_json` (`🧰️framework/…/🏪️store/🦀️.rs:15828`):
`crate::os_pack::json::to_json_string(&snapshot)`, i.e. `dsl::os_pack::json::to_json_string`/
`from_json_str` (defined in `🎒️pack/🔤️json/🦀️.rs:1418/1424`, generic over `ToValue`/`FromValue`, no
serde involved).

Applied that to every non-test call site still hitting `serde_json`:
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🌊️main/🦀️.rs:87` — `serde_json::to_string(document).ok()` →
  `Some(dsl::os_pack::json::to_json_string(document))`.
- `✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs` — `DuplicateWidgetStep` already derives
  `ToValue`/`FromValue`; replaced `serde_json::to_string`/`from_str`/`json!(payload)` at lines
  173, 177, 188, 219 (and the parallel `#[cfg(test)]` usages at 346-347, 406, 435-436, which
  weren't in the reported error count but shared the identical break and were `serde_json::from_str`
  is not check under a plain build) with `dsl::os_pack::json::to_json_string`/`from_json_str` and
  `dsl::ToValue::to_value(payload)` for the `Effect::DispatchAction.args: Option<DslValue>` field.
- `🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs` and the mirror
  `🚪️io/📥️import/🧩️deserializers/…` file — both `FlowSnapshot` and `semio_s_plugin_stdio`'s
  `CsvSnapshot` derive `ToValue`/`FromValue`, so the `serde_json::Value` round-trip was dropped
  entirely in favor of `dsl::FromValue::from_value(dsl::ToValue::to_value(x))`.

`neural::Dictionary`/`neural::Value` (used from `🧵️retained/🧾️canonical/🦀️.rs`) were never actually a
serde-bound error in my file — the 7 "errors" reported there were all `error: lifetime may not live
long enough` (not `E0277`), see §3.

## 2. `Label` duality

Two distinct `Label` types are in play: `semio_framework_plugin::Label` (re-exported from
`ui_wgpu::wgpu::Label`, has `Label::data(...)`, used for the wire/manifest label system —
`app_labels!`, `LocalizedLabel`, `ui_text`/`built_text_to_component_tree`) and
`semio_framework_plugin::plugin_app_close_prelude::Label` (`semio_framework_ui_contract::Label`,
wraps `UiText`, only `TryFrom<&str>`/`TryFrom<String>`, no `From`), which is what
`PanelTreeBuilder::section`/`tree_item_desc`/`tree_item_with_action(_draggable)` actually want
(`L: TryInto<Label>` or a concrete `Option<Label>` field).

`📌️panels/🛍️catalogue/🦀️.rs` and `📌️panels/📄️artifact/🦀️.rs` were importing the wrong (wgpu) `Label`
and wrapping strings in `Label::data(...)` before handing them to these builder calls. Fix, modeled
on how `✏️s/🔌️plugins/📐️cad/…/📄️artifact/🦀️.rs` imports it (`use
semio_framework_plugin::plugin_app_close_prelude::{…, Label as UiLabel, …}`):
- Added `use semio_framework_plugin::plugin_app_close_prelude::Label;` in both files (dropping the
  wrong one from the `semio_framework_plugin::{…}` import list).
- Everywhere a generic `L: TryInto<Label>` parameter was involved (`tree_item_with_action_draggable`,
  `tree_item_with_action`, `tree_item_desc`), passed the raw `&str`/`String` directly instead of
  `Label::data(...)` — `&str`/`String` already satisfy `TryInto<Label>` via the type's own
  `TryFrom` impls.
- Everywhere the API wanted a concrete `Option<Label>` (`PanelTreeBuilder::section`,
  `section_or_placeholder`'s `label` param), added a small local `ui_label(value: impl AsRef<str>)
  -> UiAssemblyResult<Label>` helper (`Label::try_from(value.as_ref().to_string()).map_err(...)`) in
  each file, mirroring the `ui_label`/`PluginAssemblyError` pattern every already-compiling sibling
  plugin's own crate-root `ui_label` helper uses (cad/puzzle/procedural's `✏️editor/🦀️.rs`) — could not
  reuse theirs directly since it lives in the crate root, which is out of this agent's scope.
- `flow_extension_label`/`flow_extension_action_title_label` (in `🗣️terminology/🦀️.rs`, out of
  scope, untouched) return the wgpu `Label` on purpose (their own tests assert `Label::data(...)`
  equality) — call sites now convert with `.into_string()` before handing them to
  `tree_item_with_action`'s generic `TryInto<Label>` slot.
- `section_or_placeholder`'s trailing generic `placeholder_label: L` slot (`labels.none_placeholder`,
  a `LabelText`) is passed as `.as_str()` (`&'static str`), which satisfies `TryInto<Label>` directly
  without a fallible wrapper.

## 3. Lifetime "errors" in `🧾️canonical/🦀️.rs` (not actually a serde-bound issue)

All 7 reported errors were `error: lifetime may not live long enough` on
`ArtifactCanonicalJsonValue<'a>` (invariant over `'a` because `Object`/`Array` hold
`Box<dyn Iterator<Item = ArtifactCanonicalJsonValue<'a>> + …>`, and `Item` is an invariant
associated-type position). `number`/`index`/`boolean`/`null` were declared as returning
`Value<'static>`; mixed into an `object([...])`/`array(...)` literal alongside borrowed
`text(...)`/`dictionary(...)` results (`Value<'1>`), invariance forces `'1 == 'static`, which fails
whenever the enclosing function's elided lifetime isn't actually `'static`. Fix: made those four
helpers generic over the return lifetime (`fn number<'a>(value: f64) -> Value<'a>`, etc.) instead of
hard-coding `'static` — matches the pattern the one already-compiling sibling call site
(`SetGraphParameter::canonical_json_borrowed_root` in
`🧰️framework/…/🌊️flow/🎚️parameter/📨️intent/🦀️.rs:38`) uses: it builds `Value::Scalar(Json::F64(...))`
inline at the call site's own inferred lifetime rather than through a `'static`-returning helper.

## 4. `DESCRIPTORS`/`descriptor` (E0046, `Mutation<P>` trait)

`protocol::Mutation<P>` (`🧰️framework/…/📡️replication/🎮️mutation/🦀️.rs:145`) gained two required
items: `const DESCRIPTORS: &'static [MutationLeafDescriptor]` and `fn descriptor(&self) -> &'static
MutationLeafDescriptor`. Modeled on `CadPresenceMutation`'s impl
(`✏️s/🔌️plugins/📐️cad/…/✏️editor/👥️presence/🦀️.rs:103-114`, confirmed compiling), which has the same
single-variant `Snapshot { presence: … }` shape as flow's `FlowPresenceMutation`:

```rust
const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
    protocol::MutationLeafDescriptor { schema_version: 1, owner: "<own file path>/📄snapshot",
        semantic_kind: "snapshot", display_name: "Snapshot", emoji: "📄",
        aggregate_variant: "Snapshot", payload_schema: "🔣️.schema.json", text_opcode: None,
        binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust,
            protocol::MutationLanguageSurface::JsonSchema] },
];
fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
    match self { Self::Snapshot { .. } => &Self::DESCRIPTORS[0] }
}
```

Applied verbatim (with flow's own path in `owner`) to `✏️editor/👥️presence/🦀️.rs`'s
`impl Mutation<FlowPresence> for FlowPresenceMutation`. `✏️editor/🎚️config/🦀️.rs`'s own
`impl Mutation<FlowConfig> for FlowConfigMutation` (14 variants) has the *same* E0046, but that file
is **not** in this agent's scope (it's a sibling of, not the same file as, the in-scope
`✏️editor/🧵️retained/🎚️config/🦀️.rs`) — left untouched, flagged below.

## 5. `preparation` module privacy (E0603)

`✏️editor/🦀️.rs` (crate root, out of scope) reaches `retained::artifact::preparation::PreparationFactory`.
`✏️editor/🧵️retained/🗿️artifact/🦀️.rs` declared `pub(super) mod preparation;` (visible only one level up,
to `🧵️retained`), while its sibling `recipe`/`snapshot` modules use the same `pub(super)` and are
*not* reached from the crate root. `PreparationFactory` itself was already declared
`pub(in super::super::super)` (i.e. exactly editor-level) inside `📬️preparation/🦀️.rs` — only the
wrapping `mod` declaration was one visibility notch too narrow. Fixed by widening just that one
declaration to `pub(in super::super) mod preparation;` (editor-level, matching the struct's own
already-correct visibility), leaving `recipe`/`snapshot` as `pub(super)` since nothing outside
`🧵️retained` needs them.

## 6. `sequence: i32` vs `u64` (E0308, not one of the named error families)

`store::ArtifactStoreOneItemLiveAuthority::next_sequence_number() -> i32`, but
`🧵️retained/🔤️bytes/🦀️.rs`'s `edit_id_length`/`edit_id_byte` (owned by another agent, out of scope)
both take `sequence: u64`. Fixed at the two in-scope call sites
(`🧵️retained/🎚️config/🦀️.rs:138`, `🧵️retained/🗿️artifact/📬️preparation/🦀️.rs:150`) by widening once at
the `let sequence = …next_sequence_number()` binding with `as u64` (sequence numbers are logically
non-negative counters; no other use of the local variable in either scope needed the narrower type).

## 7. `recipe/🦀️.rs` — stale `::mutation::` import segment (E0432)

`connect_widgets`/`create_widget`/`move_widgets`/`replace_widget` (under `🧬️schema/🧬️mutations/`, out
of scope) no longer nest their mutation struct under a `mutation` submodule — confirmed by reading
`🧬️mutations/🔗️connect-widgets/🦀️.rs`, which now defines `pub struct ConnectWidgets` directly at the
module root. Fixed the import in `🧵️retained/🗿️artifact/🧬️recipe/🦀️.rs:6` to drop the `::mutation::`
segment for all four.

## 8. `👁️viewer/🦀️.rs` + `🪟️windows/🌊️main/🦀️.rs` — `ArtifactViewer::render` return-type flip

`ArtifactViewer::render` now returns `UiAssemblyResult<ComponentTree>` (was `UiNode`). Flow's main
window built its node-graph scene through the retired `build_node_graph_scene(...) -> UiNode` (old
wgpu-target builder). There is no adapter from the old `UiNode` to `BuiltNode`/`ComponentTree` — per
`🧰️framework/…/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️surface.rs`'s own docstring, the old
`ComponentScene`'s `surface_id`/`controller_id`/… fields are gone outright, and per
`.🧬semio/…/26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY/📓️recipe-plugin.md` §7, "world3d,
node-graph, …" scene surfaces are explicitly out of that recipe's mechanical scope and need bespoke
handling per plugin. Used `semio_framework_plugin::scene_surface::<T: SceneDoc>(id, kind, &scene) ->
UiAssemblyResult<BuiltNode>` (`🔌️plugin/🦀️.rs:339`, itself a thin wrapper over
`semio_framework_ui_scene::encode` + `semio_framework_ui_contract::surface(...).try_id(...).try_build()`,
reachable purely via the already-depended-on `semio_framework_plugin` re-export — no new Cargo
dependency needed) in place of `build_node_graph_scene`. `NodeGraphScene` itself is unchanged (single
canonical definition in `semio_framework_ui_scene`, already re-exported through `ui_wgpu::wgpu` — no
duality). Dropped the now-meaningless `FLOW_VIEW_APP_ID` controller-id constant (the new contract has
no controller-id field; this window declares zero actions/utilities, so nothing needed it). Updated
`main::render`'s return type to `UiAssemblyResult<BuiltNode>` and its one test accordingly, and
`👁️viewer/🦀️.rs`'s `render()` body to `main::render(doc.snapshot).map(built_to_component_tree)` and
`built_text_to_component_tree(Label::data(...))` for the fallback arm — mirrors
`✏️s/🔌️plugins/📐️cad/…/👁️viewer/🦀️.rs`'s confirmed-compiling `render()` body exactly.

## What remains unverified

No `cargo check`/`build` was run (per instructions — the coordinator verifies centrally). Every fix
above is statically reasoned from trait/function definitions and from sibling code that the task
brief or my own reading confirmed compiles (cad/puzzle/procedural/sourcing), but I did **not**
execute the compiler, so:
- Exact field/variant names, generic bounds, and trait paths were read directly from source, not
  compiler-checked.
- The `MutationLeafDescriptor.owner` string I wrote for `FlowPresenceMutation` does **not** contain
  the `/🧬️mutations/` marker `mutation_leaf_descriptor_owner`'s validator requires — but neither does
  the CadPresenceMutation descriptor I copied it from, so I believe `.validate()` is a separate
  test/lint pass, not part of normal compilation; unconfirmed without running that check.
- `✏️editor/🎚️config/🦀️.rs:253`'s `impl Mutation<FlowConfig> for FlowConfigMutation` has the identical
  E0046 as `👥️presence/🦀️.rs`, but that file is not in this agent's file list (distinct from the
  in-scope `🧵️retained/🎚️config/🦀️.rs`) and was left untouched.
- The three `Arc` E0433/E0425 errors ("family #4" in the brief) are all in `✏️editor/🦀️.rs` (crate
  root, lines 1270/1361/1409) — out of scope, none found in any file on this agent's list.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🌊️main/🦀️.rs`'s sibling in the mutation-capable module
  (`✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs`) has the exact same `serde_json::to_string(fixture)`
  break and would presumably need the identical `dsl::os_pack::json::to_json_string` fix plus the
  same `build_node_graph_scene` → `scene_surface` migration — untouched, out of scope (owned by
  another agent under `✏️editor/🎭️modes/`).
