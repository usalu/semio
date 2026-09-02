# 🌱️ `ui_wgpu` primitive/control/tree node types now have `ToValue`/`FromValue`

Additive-phase pass over `🧰️framework/🔨️modules/🖱️ui/` (my module alone), continuing on from the
keystone-seven packet (`📓️ui-wgpu-keystone-seven-value-derive-2026-09-02.md`). No `Serialize`/
`Deserialize` was removed anywhere — every change is `#[derive(..., ToValue, FromValue)]` plus a
`#[value(...)]` twin of the existing `#[serde(...)]` attribute, or (for `Label`) a
`#[value(transparent)]` twin of `#[serde(transparent)]`.

## What landed (34 types gained the traits this pass)

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` (33 types, all
`#[derive(ToValue, FromValue)]` + `#[value(...)]` twins):

- **`pub mod layout`**: `StyleSpec`, `UiState`, `UiStatus`, `UiPeerMark`, `UiPresence`
- **`pub mod utilities`**: `UtilityNode` (internally tagged, `tag = "kind"` +
  `rename_all_fields = "camelCase"` — both ARE supported container attributes, contrary to a stale
  claim; see `🌱️value/✨️derive/🦀️.rs`'s own docstring)
- **`pub mod role_chrome`**: `ChromeRole` (needed a new `use dsl::{FromValue, ToValue};` import —
  this submodule only had `use dsl::DslValue;` before)
- **`pub mod ui`** (declarative node primitives, NOT recursive through `UiNode`): `UiDropOverlaySpec`,
  `UiTextNode`, `UiButtonNode`, `UiSeparatorNode`, `UiImageNode`, `UiInputNode`, `UiSelectItem`,
  `UiSelectNode`, `UiToggleNode`, `UiKeyValueEntry`, `UiKeyValueNode`, `UiSliderNode`,
  `UiNumberStepperNode`, `UiRingNode`, `UiIconSelectNode`, `UiControlNode` (internally tagged,
  single-unnamed-field variants — the derive's documented internally-tagged newtype-variant support
  covers this exactly), `UiTreeActionPlacement`, `UiTreeItemAction`, `UiTreeItemNode`,
  `UiTreeSectionNode`, `UiTreeNode`, `WorldMeshLodEntry`, `WorldLodRecord`, `WorldChunkingRecord`,
  `TableCell` (tagged, struct variants), `BlockPaletteEntry`, `UiExternalSlotNode`

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️label.rs` (1 type, hand-written
derive since the struct is a newtype): `Label` — `#[value(transparent)]` twin of
`#[serde(transparent)]`, forwards straight to/from the inner `String`'s own `ToValue`/`FromValue`.

`alias` divergence (same documented precedent as the keystone-seven packet — `#[value(...)]` has no
`alias` key): `UiSectionNode.label` (`alias = "title"`), `UiTreeItemNode.icon_id`
(`alias = "icon"`), `UiTreeItemNode.default_open` (`alias = "expanded"`) all keep
`skip_serializing_if` in their `#[value(...)]` twin but drop `alias` — the serde path is untouched
and still accepts the legacy alias; only the new `ToValue`/`FromValue` path does not.

## Why `Label` mattered so much

`Label` (a `#[serde(transparent)]` newtype around `String`, defined in `label.rs` — NOT the same
type as `LocalizedLabel`, which the keystone-seven packet already converted) is a field on nearly
every `Ui*Node` type (`label: Label`, `text: Option<Label>`, etc.). None of the 33 types above would
have compiled with the derive until `Label` itself got `ToValue`/`FromValue` — this was the first
concrete blocker hit and fixed in this pass.

## Deliberately NOT converted (blocked, left serde-only, documented in-line)

`UiStackNode`, `UiGroupNode`, `UiFieldNode`, `UiSectionNode`, `UiInspectorFieldGroup` — each embeds
`Vec<UiNode>` or `Box<UiNode>`, and `UiNode` itself embeds `UiComponentSceneNode`, which embeds
`Option<T>` for 15 scene payload types (`World3dScene`/`Canvas2dScene`/`TableScene`/…) owned by the
sibling crate `semio-framework-ui-scene`. That crate's own `Cargo.toml` docstring is explicit: it
must never depend on `os-kernel`, and `#[derive(ToValue, FromValue)]` hardcodes its expansion at the
literal `::semio_framework_os_kernel::…` path regardless of local aliasing — so the derive is
UNUSABLE there. Its ~24 types (`🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs`) would need HAND-WRITTEN
`ToValue`/`FromValue` against `protocol::value::` directly, mirroring the existing
`NodeGraph*Record::FromValue` impls already in that file (decode-only, ~130 lines for 6 small
record types) — scaled to all 24 types plus `ToValue` in both directions, this is a substantial
separate effort, not a quick derive-and-twin pass, so I left it out of scope this round. I added an
explicit `🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS)` comment
above each of the five reverted types naming the exact chain (`UiNode` → `UiComponentSceneNode` →
`semio-framework-ui-scene`'s scene types). **Next step for whoever picks this up**: hand-write
`ToValue`/`FromValue` for the 24 types in `🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs` against
`protocol::value::`, which unblocks `UiComponentSceneNode`, which unblocks `UiNode` itself, which
unblocks all five reverted types above in one follow-on pass.

`ui_wgpu::{Locale, Terminology}` were left alone per the packet's own instruction (concurrent agent
owns them) — confirmed still done (hand-written, in `locale_terminology_value.rs`) as of this
session, untouched by me.

## Tests added

Round-trip tests (`FromValue(ToValue(x)) == x`), all under `#[cfg(test)]`, `async_test`:
- `pub mod layout`'s existing `value_round_trip_tests` mod: `style_spec_round_trips`,
  `ui_presence_round_trips` (covers `UiState`/`UiStatus`/`UiPeerMark` transitively).
- `pub mod utilities`'s existing test mod: `utility_node_round_trips` (all 4 variants, one nested).
- `pub mod role_chrome`'s existing test mod: `chrome_role_round_trips`.
- `pub mod ui`: new `value_round_trip_tests` mod — `ui_text_node_round_trips`,
  `ui_button_node_round_trips`, `ui_control_node_round_trips` (all 9 variants),
  `ui_tree_item_node_round_trips`, `ui_tree_section_and_tree_node_round_trip`,
  `table_cell_round_trips` (all 4 variants), `block_palette_entry_and_external_slot_round_trip`.

## Verification

- `cargo check -p semio-framework-ui --features wgpu --message-format short`: **0 errors** (was 0
  before too, and stays 0 — confirmed via `grep -cE ': error(\[|:)'`, not an anchored `^error`).
- `cargo check -p semio-framework --message-format short`: **0 errors**, unchanged.
- `cargo test -p semio-framework-ui --features wgpu --lib value_round_trip --no-run`: my new test
  code type-checks clean. The link/run step is still blocked by the SAME two pre-existing,
  peer-owned `E0308`s in `🎯️targets/🧊️wgpu/🦀️prepared.rs` (lines 3752/3513) the keystone-seven
  packet already documented — confirmed via `git log --date=iso` that file is untouched by me (last
  commit `f394df99d4`, 2026-09-01 18:10:11, unrelated author-session). **Tests are unverified at
  runtime**, only confirmed to type-check, same caveat as the prior packet.

## Concurrent-editing note

`component.rs` is under heavy concurrent edit (this repo auto-commits periodically across the whole
fleet — my edits landed inside auto-commit `a807c0706c`, not a commit I made myself). I confirmed no
other agent's changes were clobbered: diffed `f394df99d4` (last commit before my session touched
this file) against `a807c0706c` and every change is accounted for by my own edits (derive
additions, `#[value(...)]` twins, the two new imports, the five `🚧️ BLOCKED` comments, and the new
test code). All edits were purely additive — no existing code was restructured or moved.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` (edited)
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️label.rs` (edited)
