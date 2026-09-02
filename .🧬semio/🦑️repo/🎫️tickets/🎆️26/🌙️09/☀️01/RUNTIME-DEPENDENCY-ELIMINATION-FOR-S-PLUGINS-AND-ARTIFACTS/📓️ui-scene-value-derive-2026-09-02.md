# 🌱️ `semio-framework-ui-scene` hand-written `ToValue`/`FromValue`

Crate owned exclusively for this slice: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust`.
Continuation of `📓️ui-primitives-and-tree-nodes-value-derive-2026-09-02.md`'s "Next step for whoever
picks this up".

## What changed

Hand-written (NOT `#[derive(ToValue, FromValue)]` — that macro hardcodes
`::semio_framework_os_kernel::…` paths, and this crate must never depend on `os-kernel`; mirrors the
`NodeGraph*Record::FromValue` precedent already in this file and the `MeshData` precedent in
`🔺️mesh-engine`) `impl ToValue`/`impl FromValue` against `protocol::value::` for every scene type
`UiComponentSceneNode` embeds, plus the two nested snapshot-lease types:

**`🦀️scenes.rs`** (added `ToValue` to the existing `use protocol::value::{...}` import; added a new
`🔖️ValueCodecHelpers` region of small shared object-entry helpers — `value_field`/`value_required`/
`value_decode`/`value_decode_option`/`value_decode_default`/`value_push`/`value_push_option`/
`value_push_if_nonempty` — every impl below builds on):

- Both directions, newly added: `Canvas2dScene`, `World3dScene`, `NodeGraphScene`,
  `NodeGraphViewport`, `NodeGraphHover`, `TextEditorScene`, `TableScene`, `Paint2dScene`,
  `IconRenderScene`, `VirtualFileSystemScene`, `TiledMapScene`, `Board2dScene`, `InkCanvasScene`,
  `GraphTimelineScene`, `DiffViewScene`, `EventFeedScene`, `BlockListScene` (17 types).
- `ToValue` only added (their `FromValue` already existed from an earlier packet):
  `NodeGraphPortRecord`, `NodeGraphNodeRecord`, `NodeGraphEdgeRecord`, `NodeGraphFindItem`,
  `NodeGraphOperatorVariadicRecord`, `NodeGraphOperatorChannelRecord`, `NodeGraphOperatorRecord`
  (7 types).

**`🦀️canvas2d_snapshot.rs`** / **`🦀️world3d_snapshot.rs`**: both directions for
`Canvas2dSnapshotLease` / `World3dSnapshotLease` (the two `Option<...>` fields `Canvas2dScene`/
`World3dScene` embed) — local closures, matching the existing `NodeGraph*Record` idiom, since these
two small leaf structs didn't warrant importing the shared helpers cross-module.

**Total: 26 types, 45 new `impl` blocks** (24 types needed both directions, 2 types — the two
`NodeGraph*` records with pre-existing `FromValue` needing only `ToValue` — wait: 7 needed only
`ToValue` added; 17+2=19 needed both; 19×2 + 7 = 45).

## Correctness approach

Every impl mirrors this struct's own `#[serde(rename_all = "camelCase", default, skip_serializing_if
= ...)]` shape field-by-field, not the stricter `#[value(...)]`-derive contract: an `Option<T>` field
decodes to `None` when its key is absent regardless of whether an explicit `default` attribute is
spelled out (matching `serde`'s own implicit-optionality behaviour for `Option`, not the derive's
literal "no `default` attribute → required" rule) — same precedent already set by the existing
`NodeGraphOperatorVariadicRecord::from_value`'s `max` field. A field with `#[serde(default = "fn")]`
falls back to that same `fn` on decode. A field with neither is required and errors `"missing field
`x`"` if absent. Encode-side `skip_serializing_if` is mirrored by simply omitting the key (never
emitting `null`) via `value_push_option`/`value_push_if_nonempty`.

Integer fidelity: every scalar field goes through the field's own typed `ToValue`/`FromValue` (via
the generic `value_push`/`value_decode*` helpers calling `T::to_value`/`T::from_value`), never a
manually-constructed `DslValue::Number`, so `u8`/`u16`/`u32`/`u64`/`usize` fields (slot/epoch/
byte counts, `NodeGraphOperatorVariadicRecord.min`, etc.) automatically encode as `Number::UInt` and
`f64` coordinate/size fields as `Number::Float` — verified explicitly in
`world3d_scene_round_trips_dense_and_bare_and_keeps_integers_as_integers`.

No bridge-through-the-same-trait recursion risk: none of these 26 types are self-referential or
mutually call back through `ToValue`/`FromValue` in a cycle (`NodeGraphOperatorRecord` embeds
`NodeGraphOperatorChannelRecord`/`Variadic`, a one-way DAG, not a cycle). `UiNode`'s own eventual
recursion (children) is the *next* owner's concern in `component.rs`, not this crate's.

## Tests added (all pass at runtime, not just type-check)

`🦀️scenes.rs`, new `value_round_trip_tests` module (7 tests): `Canvas2dScene` with/without its
nested lease, `World3dScene` dense/bare + explicit integer-fidelity assertion on the nested
`World3dSnapshotLease.slot`, a missing-required-field error-message test, `NodeGraphScene` round-
tripping through every nested `NodeGraph*Record` type at once (port → node → edge → viewport → hover
→ operator → channel → variadic, the deepest nesting in this crate), `NodeGraphViewport`/
`NodeGraphHover` including the all-`None` empty-object case, `TableScene` + `TiledMapScene` (the
latter proving every `#[serde(default = "fn")]` field falls back correctly on a decode missing only
the two genuinely-required keys), `Board2dScene` + `BlockListScene`.

`🦀️canvas2d_snapshot.rs` (new test module) / `🦀️world3d_snapshot.rs` (existing test module): one
round-trip test each for the two lease types.

**`UiNode` itself has no test here** — it lives in the sibling `ui_wgpu` crate
(`🎯️targets/🧊️wgpu/🦀️component.rs`), owned by a different agent this session; per this ticket's own
instruction that file was read-only for this slice. Coverage instead concentrates on this crate's
own most-nested type (`NodeGraphScene`, exercising 6 further nested record types) plus two more
scene types, satisfying the spirit of "UiNode and two nested scene types" within the file boundary
this slice actually owns.

## Which of the 6 blocked `🖱️ui` types this frees

`UiComponentSceneNode` (the type embedding all 15 top-level scene payloads as `Option<T>` fields) can
now derive/hand-write `ToValue`/`FromValue` — every field type it touches has the traits. That
cascades: `UiComponentSceneNode` unblocks `UiNode` (the enum wrapping it), which unblocks the five
types that were left serde-only with an explicit `🚧️ BLOCKED (26/09/01/...)` comment in
`component.rs` because they're recursive through `UiNode`: **`UiStackNode`, `UiGroupNode`,
`UiFieldNode`, `UiSectionNode`, `UiInspectorFieldGroup`**. That is 6 named types
(`UiComponentSceneNode` + the 5) plus `UiNode` itself as the 7th link in the chain — all now
unblocked from this crate's side. Wiring the actual `#[derive(ToValue, FromValue)]` (or hand-written
twin) onto `UiComponentSceneNode`/`UiNode`/the five in `component.rs` is the next agent's job — not
done here, per this ticket's explicit "do NOT edit `🖱️ui`'s `component.rs`/`label.rs`" instruction.

## Verification

Isolated target dir (`CARGO_TARGET_DIR=.../scratchpad/iso3`), `RUSTC_WRAPPER=""`:

```
cargo check -p semio-framework-ui-scene --message-format short          → 0 errors
cargo test  -p semio-framework-ui-scene --lib                           → 108 passed; 0 failed (was ~99 before; +9 new)
cargo check -p semio-framework-ui --features wgpu --message-format short → 0 errors (unchanged)
cargo check -p semio-framework --message-format short                    → 0 errors (unchanged)
```

Before/after proof: `git diff` of the three touched files saved, `git apply -R`'d, re-checked
`semio-framework-ui-scene` (still 0 errors — genuine pre-existing baseline, confirmed
`grep -c "impl ToValue for"` = 0 on the reverted files, i.e. the revert actually took), then
`git apply`'d back to restore — confirmed clean re-apply and 108/108 tests passing again.

Confirmed no `serde`/`Serialize`/`Deserialize` was removed (27 remaining references to
`Serialize`/`Deserialize` in `🦀️scenes.rs`, all pre-existing derives, untouched) and no
`semio-framework-os-kernel` dependency was added (`git diff` on this crate's `Cargo.toml` is empty;
the only `os-kernel`/`os_kernel` text in that file is the pre-existing explanatory comment, not a
dependency line).

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs` (edited: import, new
  `🔖️ValueCodecHelpers` region, 17 types gained both directions, 7 gained `ToValue`, new
  `value_round_trip_tests` module)
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️canvas2d_snapshot.rs` (edited: import +
  both directions for `Canvas2dSnapshotLease` + new test module)
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️world3d_snapshot.rs` (edited: import +
  both directions for `World3dSnapshotLease` + one test in the existing test module)
- No `Cargo.toml` change. No files under `component.rs`/`label.rs` touched.
