# terra / scene-surface — report

## Crate created

`semio-framework-ui-scene` at `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/` — layout mirrors `ui-contract`/`ui-runtime` (`Cargo.toml`, `📦️glue.rs`, `📋️project.json`, `📜️script.ts`). Files:

- `🦀️scenes.rs` (824 lines) — the 15 `SceneDoc` structs + `NodeGraphScene`'s nested plain records + the `SceneDoc` trait.
- `🦀️math.rs` (1856 lines) — the pre-existing `🖱️ui/🎬️scene/🦀️component.rs` (1671 lines), incorporated as the `math` region, not duplicated. Original file deleted after `ui_wgpu` stopped mounting it directly.
- `🦀️pack.rs` (645 lines) — self-contained generic `serde` binary codec (see "Encode/decode" below).
- `🦀️surface.rs` (83 lines) — `encode`/`decode` bridging `SceneDoc` ↔ `ui_contract::SurfaceProps`.

Added to the root workspace by sol (registrar) partway through — no longer a lease-request.

## Structs moved (15/15, matches `SurfaceKind`'s 15 variants)

| Struct | Schema | Note |
|---|---|---|
| `Canvas2dScene` | `canvas-2d@1` | verbatim |
| `World3dScene` | `world-3d@1` | verbatim, `.base()` moved sync |
| `NodeGraphScene` + 9 nested records (`NodeGraphPortRecord`/`NodeGraphNodeRecord`/`NodeGraphEdgeRecord`/`NodeGraphViewport`/`NodeGraphFindItem`/`NodeGraphHover`/`NodeGraphOperatorVariadicRecord`/`NodeGraphOperatorChannelRecord`/`NodeGraphOperatorRecord`) | `node-graph@1` | moved as one unit (typed fields); `.base()` moved sync |
| `TextEditorScene` | `text-editor@1` | verbatim, `.base()` moved sync |
| `TableScene` | `table@1` | `drop_action: Option<ActionDescriptor>` → `drop_action_json: Option<String>` (see below); `.base()` moved sync |
| `Paint2dScene` | `paint-2d@1` | verbatim |
| `IconRenderScene` | `icon-render@1` | verbatim |
| `VirtualFileSystemScene` | `virtual-file-system@1` | schema tracks `ui-w4-core`'s kebab-case rename of the wire tag (landed after I first wrote this; caught and fixed) |
| `TiledMapScene` | `tiled-map@1` | verbatim, defaults + `.base()` moved sync |
| `Board2dScene` | `board-2d@1` | verbatim, defaults + `.base()` moved sync |
| `InkCanvasScene` | `ink-canvas@1` | verbatim, default + `.base()` moved sync |
| `GraphTimelineScene` | `graph-timeline@1` | verbatim |
| `DiffViewScene` | `diff-view@1` | verbatim |
| `EventFeedScene` | `event-feed@1` | verbatim |
| `BlockListScene` | `block-list@1` | verbatim |

Deliberately **not** moved (stay in `ui_wgpu`, unmoved): `TableCell`/`table_row_json` (build `TableScene.rows_json`'s STRING content, never a typed field of `TableScene`), `BlockPaletteEntry` (same relationship to `BlockListScene.palette_json`), `WorldMeshLodEntry`/`WorldLodRecord`/`WorldChunkingRecord` + their default fns (documentation-shape helpers for `World3dScene`'s own `_json` fields, not typed fields themselves), `ActionDescriptor`/`UiTreeItemAction` (generic `ui_wgpu` UI-tree primitives, not product scene payloads).

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`: **5141 → 4444 lines** (760 deleted, 63 inserted — re-export block + retained-content notes). `ui_wgpu` now does `pub use ui_scene::{Board2dScene, Canvas2dScene, ..., BlockListScene};` inside `pub mod ui`, so every existing `ui_wgpu::wgpu::TableScene`/`World3dScene`/... reference compiles unchanged.

### Two fields could not move byte-identical
- `TableScene.drop_action: Option<ActionDescriptor>` → `drop_action_json: Option<String>` — `ActionDescriptor` is a genuine `ui_wgpu` type (pulls `dsl::DslValue`), not reachable from a crate that must depend on nothing beyond `ui_contract`/`serde`. Renamed (not just retyped) to be honest about the shape change, matching the `_json` convention every sibling field on the same struct already uses.
- `NodeGraphOperatorChannelRecord.default: Option<serde_json::Value>` → `default_json: Option<String>` — same reasoning; `serde_json` would have been this crate's first dependency beyond `ui_contract`/`serde`.

Both are documented in `🦀️scenes.rs`'s own header. Neither field had any live construction site outside its own struct definition / test fixtures anywhere in the repo (grepped repo-wide before deciding).

## `math` region — one real architectural finding

The pre-existing `🖱️ui/🎬️scene/🦀️component.rs` had every fn marked `pub async fn` (survivor of a repo-wide async-ification codemod — confirmed by this ticket's own `.py` scripts in this folder) with **zero internal `.await` call sites**, including into `semio_framework_geometry::{Vec3, Mat4}`'s own (correctly, consistently `.await`-chained) async API. That combination cannot compile as async OR as sync-calling-through: awaiting inside a sync `fn` (E6's decree) is illegal, and the geometry crate is outside this packet's path scope so I can't strip its async either.

Resolved by porting the same formulas (verified byte-for-byte against `📐️geometry`'s own `⚙️engine/🦀️component.rs`) as local sync free-functions/trait methods suffixed `_m` (`vec3_new_m`, `.dot_m()`, `mat4_perspective_m()`, ...) inside `math.rs`, so `Vec3`/`Mat4` stay the real geometry-crate types (no wrapper/newtype) but every call goes through the sync port instead of the geometry crate's async inherent methods. All 40 pre-existing math tests (frustum culling, ray/AABB, mat4 inverse round-trips, orbit camera, LOD grids, ...) pass unchanged — strong correctness signal for the port. Flagging to sol: `📐️geometry`'s own `Vec3`/`Mat4` carry the identical bug (async with zero internal `.await`) and are outside my scope; worth its own cleanup packet since other `wgpu-engine`-feature code may hit the same wall.

## `SceneDoc` / encode / decode — as landed

```rust
pub trait SceneDoc: Clone + Serialize + DeserializeOwned {
    const SCHEMA: &'static str; // "<surface-kind-wire-tag>@<version>", e.g. "table@1"
}

pub fn encode<T: SceneDoc>(kind: SurfaceKind, doc: &T) -> SurfaceProps;
pub fn decode<T: SceneDoc>(props: &SurfaceProps) -> Result<T, SurfaceDocError>;

pub enum SurfaceDocError {
    SchemaMismatch { expected: &'static str, actual: String },
    Decode(PackError),
}
```

Schema strings agree with `ui_contract::SurfaceKind`'s own `#[serde(rename = ...)]` wire tags (verified against `Interpreter/🟦️component.tsx`'s `resolveComponentSceneHost`/`SURFACE_KIND_SCENE_FIELD` switch, which uses the same 15-entry, same-tag convention) with `@1` appended.

**Known deviation from `ui_contract::🦀️surface.rs`'s documented target signature**, flagged to sol mid-task: that file's module doc specifies `encode<T: Serialize>(kind: SurfaceKind, version: u32, value: &T) -> SurfaceProps` / `decode<T: DeserializeOwned>(props: &SurfaceProps) -> Result<T, DecodeFault>` (kind+version as separate args, no `SceneDoc` bound, error type named `DecodeFault`). I kept the ticket brief's own `SceneDoc`-bound shape (tested, working, and the contract file itself says "this crate defines only the opaque envelope... the scene crate owns `DecodeFault`, the per-kind `kind_slug` strings, and the actual pack encode/decode" — i.e. it explicitly leaves the shape to this crate). Renaming `SurfaceDocError`→`DecodeFault` and splitting `SCHEMA`→`KIND`+`VERSION` to match the documented signature exactly is a small mechanical follow-up if sol wants byte-for-byte alignment; not done here to avoid more churn on a crate that just came back from a repo-wide build break.

Landed `SurfaceProps` (confirmed from disk, differs from ticket brief's abbreviated version): `{ kind: SurfaceKind, doc_schema: String, doc: SurfaceDoc, bindings: Vec<ActionBinding> }` — no `surface_id`/`controller_id`/`pane_id`/`binding_id`/`domain_id`/`domain_granularity_id` (dropped by `ui-w4-core`, now `ui_render`'s `SurfacePlacement` concern). `encode`/`decode` adapted accordingly (`..Default::default()` for `bindings`).

## Pack codec

No existing repo pack encoder is usable here without an OS/DSL dependency: `🎒️pack::encode_record_body`/`decode_record_body` and `store::encode_pack_value`/`decode_pack_value` are all `RecordSpec`/`DslValue`-typed (os-kernel layer); `pack::encode_json_value` wraps a full `.spk` container (confirmed heavyweight per prior project note). Followed `semio_framework_actor::pack`'s own precedent instead — "hand-rolled, self-contained... no dependency on `🎒️pack`" — but generalized via `serde::Serializer`/`Deserializer` rather than per-type hand-written `pack_encode`/`pack_decode` (15 structs × up to 20 fields would have meant ~15 hand-written codecs otherwise). Tag-based binary format: LEB128 varints, length-prefixed strings/bytes, no `serialize_map`/enum-variant support (documented as intentionally out of scope — no `SceneDoc` struct needs either after the `_json` opaque-string treatment). Returns `PackError`, never panics, including on truncated input (tested).

## Acceptance

All commands run foreground, `CARGO_TARGET_DIR=.../scratchpad/target-scene`:

- `cargo check -p semio-framework-ui-scene --lib` → **EXIT 0**
- `cargo check -p semio-framework-ui-scene --lib --target wasm32-unknown-unknown` → **EXIT 0**
- `cargo check -p semio-framework-ui-scene --lib --target wasm32-wasip2` → **EXIT 0**
- `cargo test -p semio-framework-ui-scene --lib` → **82 passed, 0 failed** (40 pre-existing math tests unchanged, 3 pack codec tests, 2 surface encode/decode tests incl. both acceptance-named cases: `TableScene` round-trips byte-identical; `World3dScene` decoded against `doc_schema = "world3d@99"` returns `Err(SchemaMismatch)`, never panics)
- `cargo clean -p semio-framework-ui-scene && cargo check -p semio-framework-ui-scene --lib`, grep `unused implementer of` → **0 matches** (R12/R17 census; expected under E6, verified rather than assumed)
- `cargo check -p semio-framework-ui --lib --features wgpu` → **EXIT 0** (this is the flag every one of its 15 dependents actually enables — per sol's rule 22, checked with the feature, not bare `--lib`)
- `cargo test -p semio-framework-ui --lib --features wgpu` → 45 passed, 1 failed. The failure (`action_descriptor_and_style_spec_serialize_to_golden_json`, a `42` vs `42.0` JSON-number mismatch at line 1330) is in code I never touched — confirmed via `git diff --stat`, every one of my hunks starts at line 3071 or later. Pre-existing, unrelated to this packet.

## Incident: broke `semio-framework-ui` mid-flight, now fixed

While mid-surgery on `🎯️targets/🧊️wgpu/🦀️component.rs` I left two orphaned inherent `impl` blocks (`impl NodeGraphScene`, `impl TextEditorScene`) behind after their types moved — an inherent `impl` for a now-foreign type is an orphan-rule error (E0116), and it also meant their `.base()` constructors vanished (E0599 at the two call sites). Sol caught this from the consumer side (`--features wgpu`) while I was still mid-task; fixed by moving both `.base()`s into the scene crate (sync) and turning `TextEditorScene`'s `ui_wgpu`-side `json_view`/`code_input` (which embed a live `ActionDescriptor`, so legitimately stay put) into free functions `text_editor_json_view`/`text_editor_code_input` — renamed since they can no longer be inherent methods; grepped repo-wide first, nothing else called the old names. Also fixed 6 leftover `X::base(...).await` call sites in `ui_wgpu`'s own test module (the `.base()`s are sync now). Both `cargo check -p semio-framework-ui --lib --features wgpu` and `-p semio-framework-ui-scene --lib` are green as of this report.

## Housekeeping
- Deleted a stray `Cargo.lock` in the scene crate directory (leftover from a temporary standalone-`[workspace]` trick I used to `cargo check` the crate before it had a registrar-added workspace-member line; confirmed dead once the crate resolved fine via the main workspace's own graph — sol, no action needed, already gone).
- Deleted the original `🖱️ui/🎬️scene/🦀️component.rs` (its content now lives as `math.rs`; grepped repo-wide first, nothing still `#[path]`-mounts it).

## Files touched
- Created: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs,🦀️scenes.rs,🦀️math.rs,🦀️pack.rs,🦀️surface.rs,📋️project.json,📜️script.ts}`
- Deleted: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🦀️component.rs`
- Edited: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` (added `ui_scene` dep — path corrected by sol), `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` (`kernel_3d_scene` now aliases `ui_scene::math`), `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` (15 struct defs + `NodeGraphRecords` region + two orphaned impls + 2 orphaned default fns removed, replaced with re-exports; `TextEditorScene::json_view`/`code_input` → free fns)
- Registrar (sol): root `Cargo.toml` member line added; a path-typo in my own `ui_scene` dependency line fixed.
