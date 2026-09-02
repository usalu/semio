# 🌳️ The recursive `UiNode` family (7 types) now has `ToValue`/`FromValue`

File: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`
(`pub mod ui`, `#[cfg(feature = "wgpu")]`-mounted; `semio-framework-ui` with `--features wgpu` is
the crate that actually mounts this file — `semio-framework` re-exports the whole target tree, both
were checked).

## What landed

All 7 types were unblocked, additively (`Serialize`/`Deserialize` kept on every one), because
`semio-framework-ui-scene` (verified) already carries hand-written `ToValue`/`FromValue` for all 15
embedded scene payload types (`Canvas2dScene`, `World3dScene`, `NodeGraphScene`,
`TextEditorScene`, `TableScene`, `Paint2dScene`, `VirtualFileSystemScene`, `TiledMapScene`,
`Board2dScene`, `IconRenderScene`, `InkCanvasScene`, `GraphTimelineScene`, `BlockListScene`,
`DiffViewScene`, `EventFeedScene` — confirmed via `grep -n "impl ToValue for\|impl FromValue for"`
in `🦀️scenes.rs`, all 15 present).

- `UiStackNode`, `UiGroupNode`, `UiFieldNode`, `UiSectionNode`, `UiInspectorFieldGroup` — plain
  `#[derive(ToValue, FromValue)]` + `#[value(rename_all = "camelCase")]` twin, field-by-field
  `#[value(...)]` mirrors of each `#[serde(...)]`. All five `🚧️ BLOCKED
  (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS)` comments (and their
  associated older `🚧️ NOT typegen-derived` doc comments) deleted.
- `UiSectionNode.label: Option<Label>` carries `#[serde(alias = "title")]`. The first-party derive
  has no `alias` attribute (unrecognized key = hard compile error), so per the established pattern
  already in this same file (`WindowLayoutWindowNode::active_window_kind_id`,
  `UiTreeItemNode::icon_id`/`default_open`), the `#[value(...)]` twin drops `alias` and keeps only
  `skip_serializing_if`, with an explanatory comment. `ToValue`/`FromValue` round-trips on the
  canonical `label` key only; serde keeps accepting the legacy `title` alias unchanged (no serde
  behavior change).
- `UiComponentSceneNode` — had no `🚧️ BLOCKED` marker of its own (only referenced by the other
  five's comments) but was itself un-derived; added the same twin treatment across all 15
  `Option<...Scene>` fields plus `surface_id`/`controller_id`/`component_kind`/`pane_id`/
  `binding_id`/`presence`/`menu`.
- `UiNode` (the recursive enum itself) — internally tagged (`#[serde(tag = "type", rename_all =
  "camelCase")]`), 19 tuple variants each wrapping a struct. Every one of those 19 struct types
  already had (or now has, after this pass) `ToValue`/`FromValue`, so the plain derive applies
  unchanged: `#[derive(ToValue, FromValue)]` + `#[value(tag = "type", rename_all = "camelCase")]`
  — an exact twin, same pattern already proven at `UiControlNode` (a sibling internally-tagged
  enum with tuple-of-struct variants) a few hundred lines above. No hand-written impl was needed —
  the derive expresses genuine tree recursion (via `Vec<UiNode>`/`Box<UiNode>` in the child
  structs) correctly on its own, since each field just calls `ToValue`/`FromValue` on the child
  value, which terminates naturally at leaf nodes (this is a tree, not a cycle).

## Verify pattern used (proof, not assumption)

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=.../scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-framework-ui --features wgpu --message-format short   # 0 errors, WITH changes
cargo check -p semio-framework --message-format short                       # 0 errors, WITH changes
cargo check -p semio-framework-ui-scene --message-format short              # 0 errors, untouched
git diff -- .../🦀️component.rs > my_change.diff
git apply -R my_change.diff                                                 # revert
cargo check -p semio-framework-ui --features wgpu --message-format short   # 0 errors, baseline confirmed (6 BLOCKED markers present)
git apply my_change.diff                                                    # restore
cargo check -p semio-framework-ui --features wgpu --message-format short   # 0 errors again, 0 BLOCKED markers
cargo check -p semio-framework --message-format short                       # 0 errors again
```

All four `cargo check` runs (pre-change baseline, post-change ui, post-change framework,
post-change ui-scene) used `grep -cE ': error(\[|:)'` (anchored `^error` undercounts, per the
packet's warning) — every count was **0**.

## Round-trip test added

Added `ui_node_round_trips_with_nested_children` to the existing `mod value_round_trip_tests`
(same file, `#[cfg(test)]`), nesting **three** levels deep: `Stack > Section > Group > [Text,
Field > Text]` — exercises `Vec<UiNode>` recursion (Stack/Section/Group) AND `Box<UiNode>`
recursion (Field) in one tree. Also added `ui_component_scene_node_round_trips` and
`ui_inspector_field_group_round_trips` for the other two newly-unblocked types not otherwise
covered by the nested tree.

**Could not execute the test binary** — confirmed pre-existing, NOT caused by this change:
`cargo test -p semio-framework-ui --features wgpu --lib -- value_round_trip_tests` fails to link
with exactly the two `E0308`s the packet warned about, both in the peer-owned
`🎯️targets/🧊️wgpu/🦀️prepared.rs` (line 3752: `PreparedRenderUpload::GlyphAtlas` matched against
`Option<&PreparedRenderUpload>`; line 3513: closure passed where `drive_step` expects an `fn()
-> Option<u64>`), unrelated to `component.rs`. Grepped the full test-build output for
`component.rs` and confirmed zero errors from this file — only warnings, and only the 2 real
errors, both in `prepared.rs`. Per the packet's instruction, reporting this rather than chasing it.
The new test **compiles cleanly** (it is part of the same crate that reached the 0-error `cargo
check`, and the failing test-build got past `component.rs` with zero errors before failing at
`prepared.rs`), but its assertions were not executed at runtime.

## Confirmed: no serde removed

`Serialize`/`Deserialize` kept on all 7 types; `git diff` only adds `ToValue, FromValue` to derive
lists, adds `#[value(...)]` twins, and deletes `🚧️ BLOCKED`/stale `🚧️ NOT typegen-derived` comment
blocks. No field renamed, no field removed, no `#[serde(...)]` attribute changed or removed.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` (only file
  touched this task)
