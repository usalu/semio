# 📓️ W7c — block5d compile repair

Scope: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/**` only (subset `🏅️standards/🔖️1/🪆️subsets/✳️any`).
Work list: every `🖐️5d` line of `🗑️generated/check-lib-1.txt` (22 errors).
Oracle: the compiling plugin `✏️s/🔌️plugins/🧩️puzzle` 5d subset, plus — for two classes — the
in-plugin sibling `🗿️artifacts/🧊️3d` files W7b migrated in parallel (same crate, same SDK gaps).

Nothing outside `🖐️5d/**` was touched. The crate entry `📦️packages/🦀️rust/🦀️.rs` was NOT edited.

---

## 1. E0433 `cannot find mutation in <leaf>` — 9 sites

**Symptom** (`🧬️schema/🧬️mutations/*/↩️inverse/🦀️.rs`): the inverse body called
`super::super::<leaf>::mutation::<leaf>(…)`. There is no `mutation` module: the crate entry mounts
each leaf as `pub mod <leaf> { pub mod diff; pub mod inverse; mod component; pub use component::*; }`
(`📦️packages/🦀️rust/🦀️.rs:829` for `update_part_2d`), so the builder is re-exported directly on the
leaf module.

**Puzzle precedent**: `🧩️puzzle/…/🖐️5d/…/🧬️mutations/📍move-part2d/↩️inverse/🦀️.rs:9` —
`crate::artifacts::puzzle5d::mutations::move_part_2d::move_part_2d(…)`, i.e. `<leaf module>::<builder>`
with no interposed segment. 32 of block5d's own 41 inverse files already spelled it that way
(`✏️rename-part-kind/↩️inverse/🦀️.rs:8`); only 9 carried the stray segment.

**Change**: dropped the `::mutation` segment in the 9 offenders — `🖌️update-part-2d`, `🧊update-part-3d`,
`📍move-grip-2d`, `🧭move-grip-3d`, `📏resize-grip-3d`, `🎥move-camera2d`, `🔍scale-camera2d`,
`🎬move-camera3d`, `🔎scale-camera3d` (each `↩️inverse/🦀️.rs`).

## 2. E0080 "Mutations descriptor and semantic kind must agree" — `🧬️mutations/🦀️.rs:28`

**Law** (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:1928-1931`), evaluated per variant
inside `dsl::Mutations`' `DESCRIPTORS` const:

1. `MutationKind::SEMANTICS.kind == to_kebab(VariantIdent)`
2. `MutationLeaf::DESCRIPTOR.aggregate_variant == VariantIdent`
3. `MutationLeaf::DESCRIPTOR.semantic_kind == MutationKind::SEMANTICS.kind`  ← the failing one

`DESCRIPTOR` is parsed from the leaf's committed `🔣️.json`. For a **flat** mutation owner (directly
under `🧬️mutations/`, which is block5d's whole vocabulary) the derive sets
`expected_semantic_kind = None` (`✨️derive/🦀️.rs:94-98`), so `semanticKind` is NOT tied to the
directory name — the directory name is free, `semanticKind` is not.

`to_kebab` (`✨️derive/🦀️.rs:2246-2271`) opens a word only at an uppercase letter following a lowercase
letter or a digit, so `MoveGrip2d → "move-grip2d"` — never `"move-grip-2d"`. Rule 1 therefore pins
`SEMANTICS.kind`, and the JSON was the free variable that disagreed.

**Puzzle precedent**: `🧩️puzzle/…/🧬️mutations/📍move-part2d/🔣️.json` has `"semanticKind": "move-part2d"`
matching `🦀️.rs:13`'s `SEMANTICS.kind = "move-part2d"` and `to_kebab("MovePart2d")`.

**Change**: five descriptor JSONs realigned to the derive-pinned spelling (all other 36 already agreed):

| leaf | was | now |
| --- | --- | --- |
| `📍move-grip-2d/🔣️.json` | `move-grip-2d` | `move-grip2d` |
| `🧭move-grip-3d/🔣️.json` | `move-grip-3d` | `move-grip3d` |
| `📏resize-grip-3d/🔣️.json` | `resize-grip-3d` | `resize-grip3d` |
| `🖌️update-part-2d/🔣️.json` | `update-part-2d` | `update-part2d` |
| `🧊update-part-3d/🔣️.json` | `update-part-3d` | `update-part3d` |

These are exactly the five spellings `KINDS` (`🧬️mutations/🦀️.rs`) and the committed oracle catalog
(`🔮️oracle/🔣️.json`) already used, so `kinds_match_the_enum_and_the_catalog` stays honest.
`textOpcode`/`#[dsl(keyword)]`/grammar/TS were left alone — they are a separate axis the derive does
not constrain, and changing them would move the text/binary wire format.

## 3. E0046 missing `DESCRIPTORS`/`descriptor` — config + presence

`protocol::Mutation` grew a required `DESCRIPTORS` const and `descriptor()` method. Both block5d
lane enums are hand-written (`dsl::DslOps`, not `dsl::Mutations`), so nothing supplies them.

**Puzzle precedent**: `🧩️puzzle/…/✏️editor/🎚️config/🦀️.rs:203-238` and `👥️presence/🦀️.rs:103-112` —
one `MutationLeafDescriptor` literal per variant with a `PROVISIONAL` owner path (these enums own no
`🧬️mutations/<slug>` triad on disk), plus a `descriptor()` match returning `&Self::DESCRIPTORS[i]`.

**Change**: mirrored that verbatim.
- `✏️editor/👥️presence/🦀️.rs` — one entry, `snapshot`.
- `✏️editor/🎚️config/🦀️.rs` — two entries, `snapshot` + `set-locale`, matching
  `Block5dConfigMutation`'s two variants in declaration order.

## 4. `render` must return `Result<ComponentTree, …>` / `UiNode` → `BuiltNode`

`ArtifactEditor::render` and `ArtifactViewer::render` now return
`UiAssemblyResult<ComponentTree>`; the leaf renders return `UiAssemblyResult<BuiltNode>` and the
top-level assembles with `built_to_component_tree`.

**Puzzle precedent**: `🧩️puzzle/…/✏️editor/🦀️.rs:9155-9173` (`let node = …?; Ok(built_to_component_tree(node))`,
unknown-body arm via `built_text_node(...).map_err(...)`), and `◻️2d/🦀️.rs:174` / `📌️panels/*` for the
leaf signature. In-plugin twin: `🧊️3d/…/👁️viewer/🦀️.rs:71-77`.

**Changes**:
- `✏️editor/🦀️.rs::render` — arms `?`-propagate, fallback uses `built_text_node`, tail returns
  `Ok(built_to_component_tree(node))`.
- `👁️viewer/🦀️.rs::render` — same shape; return type changed from `UiNode`; `UiNode` import dropped.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs:46` — `MeshWindowKit::render` (framework,
  `🔌️plugin/🦀️.rs:26208`) now returns `UiAssemblyResult<BuiltNode>`; this window's `render` return type
  changed from `UiNode` to `UiAssemblyResult<BuiltNode>` and just forwards it. Body unchanged.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs` and `…/🌐️world/🦀️.rs` — the two lightweight text
  summaries moved off the retained `ui_stack_vertical(vec![ui_text(Label::data(…))])` onto the
  contract builders: `ui::text(ui_label(…)?).try_build()` per line, `ui::column().try_children([…])`
  for the stack, each admission failure surfaced as a `PluginAssemblyError`.

## 5. `with_ports` on `impl Future<Output = AppIo>`

`AppIo::from_document` and `AppIo::with_ports` are both `async fn` now
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:5337,5343`).

**Puzzle precedent**: `🧩️puzzle/…/✏️editor/🦀️.rs:9119-9126` — `resolve_ready(AppIo::from_document(…))`
then `resolve_ready(io.with_ports(vec![…]))`.

**Change**: `✏️editor/🦀️.rs::block5d_io` wraps both stages in
`semio_framework_plugin::resolve_ready` (the SDK re-export block already uses at
`✏️editor/🦀️.rs:839`), keeping the `"catalog:out"` port declaration byte-identical.

## 6. `command_from_action` takes `dsl::DslValue`

**Puzzle precedent**: `🧩️puzzle/…/✏️editor/🦀️.rs:9074-9078` — signature takes `Option<&dsl::DslValue>`
and converts once with `args.map(dsl::os_pack::json::from_dsl_value)` before reading fields off the
resulting json `Value`.

**Change**: `✏️editor/🦀️.rs::command_from_action` signature switched to `Option<&dsl::DslValue>`; the
body converts once and `str_field` now reads through `args.as_ref()`. All seven action arms are
untouched, so the retained/typed command bridge behaves identically.

## 7. `ActionDescriptor` vs `Result<(ActionId, Option<UiValue>), …>` — `📌️panels/🔍️inspection`

`block5d_action` (`✏️editor/🦀️.rs:46`) already returned the new
`UiAssemblyResult<(ActionId, Option<UiValue>)>`; the inspector still built retained
`UiNode::Field(UiFieldNode { child: UiNode::Input(UiInputNode { on_change: ActionDescriptor … }) })`
via `ui_inspector_groups_to_tree`.

**Precedent**: the in-plugin sibling `🧊️3d/…/📌️panels/🔍️inspection/🦀️.rs:32-98` (W7b's parallel
migration of the identical panel), itself following puzzle's
`🧩️puzzle/…/📌️panels/🔍️inspection/🦀️.rs:33-40` `PanelTreeBuilder` shape.

**Change**: rewrote `📌️panels/🔍️inspection/🦀️.rs` onto the contract builders — `field_row` wraps a
control in `ui::field`, `text_field` binds `Trigger::Change` to the `patchPartKind` action through
`try_on_with`, `readonly_field` is a `disabled(true)` input, and `render` assembles them with
`PanelTreeBuilder::section`. The three rows (name, label, grip count) and the `patchPartKind`
`{field}` payload are preserved exactly.

## 8. `Label: From<Label>` / `From<LabelText>` — `📌️panels/🗿️artifact`

Two distinct `Label` types exist: the SDK's retained authoring `semio_framework_plugin::Label`
(= `ui_wgpu::wgpu::Label`, has `::data`) and the wire/contract
`semio_framework_plugin::plugin_app_close_prelude::Label` (= `semio_framework_ui_contract::Label`,
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🧩️component.rs:36`, `TryFrom<&str>` only — its
docstring says the two are deliberately unbridged). `tree_item_desc` /
`PanelTreeBuilder::section_or_placeholder` (`🔌️plugin/🦀️.rs:5805,5929`) take the contract one; the
panel was passing the retained one and `LabelText`.

**Precedent**: puzzle's `ui_label` helper (`🧩️puzzle/…/✏️editor/🦀️.rs:4337`) and its artifact panel's
`ui_label(labels.parts.as_str())?`; in-plugin twin `🧊️3d/…/📌️panels/🗿️artifact/🦀️.rs:26-48`.

**Changes**:
- added `ui_label` to `✏️editor/🦀️.rs` (returns the contract `Label`, admission failure →
  `PluginAssemblyError`), the block5d twin of block3d's `✏️editor/🦀️.rs:78`;
- `📌️panels/🗿️artifact/🦀️.rs` — `icon_item` now takes `label: &str` and converts internally; the four
  `labels.*.into()` call sites became `ui_label(labels.*.as_str())?`.

---

## Files changed (all under `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

- `🧬️schema/🧬️mutations/{🖌️update-part-2d,🧊update-part-3d,📍move-grip-2d,🧭move-grip-3d,📏resize-grip-3d,🎥move-camera2d,🔍scale-camera2d,🎬move-camera3d,🔎scale-camera3d}/↩️inverse/🦀️.rs`
- `🧬️schema/🧬️mutations/{📍move-grip-2d,🧭move-grip-3d,📏resize-grip-3d,🖌️update-part-2d,🧊update-part-3d}/🔣️.json`
- `✏️editor/🦀️.rs`
- `✏️editor/🎚️config/🦀️.rs`
- `✏️editor/👥️presence/🦀️.rs`
- `✏️editor/📌️panels/🗿️artifact/🦀️.rs`
- `✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs`
- `👁️viewer/🦀️.rs`
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs`

The retained-factory / boot-snapshot (`default_block5d_snapshot`) / `io()` work from W1-W3 is intact —
none of it was rewritten, only the `AppIo` construction stage was wrapped in `resolve_ready`.

---

## ✅️ Verification

PENDING — see `🗑️generated/w7c-check.txt` and `🗑️generated/w7c-test.txt`.
