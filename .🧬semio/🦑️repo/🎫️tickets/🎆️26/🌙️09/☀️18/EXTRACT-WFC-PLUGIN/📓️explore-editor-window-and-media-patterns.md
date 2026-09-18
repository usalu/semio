# Editor window, media & tool patterns for the `🌊️wfc` plugin

Read-only survey of proven patterns in the monorepo, for engineers building the five `wfc`
artifacts: **bitmap** (input/output bitmap windows), **2d-grid** (grid canvas + 2d preview),
**2d** (graph-with-slots + 2d preview), **3d-grid** (grid box + 3d preview), **3d** (graph-with-slots
+ 3d preview). All paths are repo-relative from `/Users/ueli/Documents/semio`.

---

## 0. The module-per-directory convention (read this first)

Every plugin/artifact is authored as **one Rust module per directory**: a folder holds a single
`🦀️.rs` file that *is* that module's body, and its subfolders are its submodules, wired in with
inline `#[path = "…"]` mod declarations (not a codegen tool). Example, from the generation3d
editor's `edit` mode (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs`):

```rust
use crate::editor::generation3d::modes::edit::windows::{flow, preview};
...
pub fn layout() -> WindowLayout {
    create_default_layout(&[flow::GENERATION_3D_PLAY_WINDOW_MAIN.into(), preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into()], "row", Some(&[68.0, 32.0]), Some(&["Flow".into(), "Preview".into()]))
}
```

and inside a window file, e.g. `…/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`:

```rust
#[path = "🫧️transient/🦀️.rs"]
pub mod transient;
```

The standard directory shape under an editor is: `🎭️modes/<mode>/🪟️windows/<window>/🦀️.rs`
(window kind definition + render), optionally with `☑️options/`, `🎚️config/`, `🎬️actions/`,
`🫧️transient/`, `👥️presence/` subfolders per window; `🎮️commands/<command>/🦀️.rs` (one command per
folder); `🧬️schema/🧬️mutations/<mutation>/🦠️mutation, 🔺️diff, ↩️inverse, 🧪️tests` (one folder per
mutation with its own diff/inverse); `📌️panels/`; `🗣️terminology/`; `👥️presence/`; `🌉️wasm/`.

For `wfc`, the closest existing sibling-plugin precedent is **`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly`**
— an artifact whose document is literally a WFC-style problem spec (seed/slots/adjacency-edges/
weights/rules, solve never persisted). See §5.

---

## 0.5. CRITICAL: a working WFC engine + slot schema already exists in this repo

Before designing anything new: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly` is not just "a good analog" —
its schema is explicitly a WFC problem spec and it ships a real solver. **Read this artifact first.**

Schema (`…/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:23-90`):

```rust
pub struct AssemblySlot { pub id: String, pub x: f64, pub y: f64, pub z: f64, pub pinned_module_id: Option<String> }
/// 🔗 One adjacency EDGE between two slots — the generic graph topology WFC propagates constraints
/// over (`semio_framework_graph::GraphView`), independent of any regular-grid assumption.
pub struct AssemblySlotEdge { pub id: String, pub from_slot_id: String, pub to_slot_id: String }
pub struct AssemblyModuleWeight { pub module_id: String, pub weight: f64 }
pub struct AssemblyRule { pub id: String, pub module_a_id: String, pub module_b_id: String, pub allowed: bool, pub params: SemioValue }
pub struct AssemblySnapshot { pub schema: String, pub seed: u64, pub slots: Vec<AssemblySlot>, pub edges: Vec<AssemblySlotEdge>, /* rules, weights, modules */ }
```

An **actual WFC solver package** already lives right next to it:
`…/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/`, with 14+
submodules: `🌐️domain`, `⛓️constraint`, `🕳️sparse-3d`, `📣️propagate`, `🗺️topology`, `🀄️tiled`,
`🪜️hierarchy`, `🪶️soft`, `⚠️error`, `🎼️motif`, `🧬️evolve`, `🐾️trail`, `🚫️nogood`, `🔧️repair`.

Its editor (§5.3) is a **tree window** (`TreeWindowKit`), not a spatial canvas — its own doc comment
flags a spatial `NodeGraphScene`/`Board2dScene` view over `slots`' raw x/y/z as "a plausible
follow-up." **The `wfc` plugin's `2d`/`3d` graph-with-slots artifacts should very likely reuse or
extend `assembly`'s schema and solver rather than reinvent slot/rule/weight/seed modeling** — verify
with whoever scoped `wfc` whether it supersedes, wraps, or is meant to stay independent of `assembly`
before writing new mutation/schema code.

---

## 1. Multi-window editors: window kinds + layouts

### 1.1 The core framework types

`WindowKindDefinition` — `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3288`:

```rust
pub type WindowKinds = NonEmptyVec<WindowKindDefinition>;   // every app has ≥1 window kind

pub struct WindowKindDefinition {
    pub id: String,
    pub label: LocalizedLabel,
    pub body_key: String,
    pub surface_kind: SurfaceKind,
    pub icon_id: IconName,
    pub options: WindowOptions,          // chrome facets (measures/engagement)
    pub actions: Vec<ActionDefinition>,  // actions owned by this window kind
    pub utilities: Vec<UtilityRef>,      // references AppDefinition.utilities
    pub interactions: Vec<InteractionRef>,
    pub params_schema: Option<String>,
    pub artifact_snapshot_schema: Option<String>,
    pub input_event_schema: Option<String>,
    pub output_schema: Option<String>,
    pub capabilities: Vec<kernel::CapabilityRequirement>,
}
```

`SurfaceKind` — `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:81` — 15 tags, the ones that
matter for wfc:

| tag | used by |
|---|---|
| `canvas-2d` (default) | draw's vector canvas, generation2d/3d preview windows' form/tile windows, architect's adjacency window, animate's tile-cropper |
| `paint-2d` | raster's bitmap composite/navigator windows |
| `world-3d` | generation3d/generation2d/puzzle-3d preview & main windows |
| `node-graph` | dag/flow/generation3d/generation2d flow-graph windows |
| `board-2d` | puzzle-2d, block's 2d/5d, gis-free "board" games — free-placed nodes/edges/handles on a snap-grid |
| `tiled-map` | gis's tile-based map |

`WindowLayout` / `WindowLayoutNode` — `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📐️layout/🦀️.rs:390-447`:

```rust
pub enum WindowLayoutNode {
    Window { window_kind_id: String, title: Option<String>, instance_id: Option<String>, template_id: Option<String>, corner: Option<WindowStackCorner> },
    Stack { size: Option<f64>, active_window_kind_id: Option<String>, children: Vec<WindowLayoutNode> },
    Split { axis: Axis, size: Option<f64>, children: Vec<WindowLayoutNode> },
}
pub struct WindowLayout { pub root: WindowLayoutNode }
```

Builder helper `create_default_layout(window_ids, direction, sizes, titles)` —
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:960` — builds a `Split` root of
`Window` leaves, e.g. `create_default_layout(&["a","b"], "row", Some(&[68.0,32.0]), Some(&["A".into(),"B".into()]))`.

### 1.2 Declaring two window kinds + a layout — generation3d exemplar (flow graph + 3D preview)

Flow window kind — `…/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:24-40`:

```rust
pub const GENERATION_3D_PLAY_WINDOW_MAIN: &str = "procedural-main";
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { id: GENERATION_3D_PLAY_WINDOW_MAIN.into(), label: LocalizedLabel::native("Flow", "Workflow"),
        body_key: GENERATION_3D_PLAY_BODY_MAIN.into(), surface_kind: SurfaceKind::NodeGraph, icon_id: "flow-graph".into(),
        options: WindowOptions::default(), actions: Vec::new(), utilities: Vec::new(), interactions: Vec::new(), .. }
}
```

Preview window kind — `…/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:19-31`:

```rust
pub const GENERATION_3D_PLAY_WINDOW_PREVIEW: &str = "procedural-preview";
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { id: GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: GENERATION_3D_PLAY_BODY_PREVIEW.into(), surface_kind: SurfaceKind::World3d, icon_id: "preview".into(), .. }
}
```

Mode declares BOTH window kinds and the layout — `…/🎭️modes/✏️edit/🦀️.rs:14-27`:

```rust
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GENERATION_3D_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"),
        icon_id: "pencil".into(), tools: vec![...], layout_id: None, commands: Vec::new() }
}
pub fn layout() -> WindowLayout {
    create_default_layout(&[flow::GENERATION_3D_PLAY_WINDOW_MAIN.into(), preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into()], "row", Some(&[68.0, 32.0]), Some(&["Flow".into(), "Preview".into()]))
}
```

App manifest stitches window kinds, modes, and layouts together (editor root `🦀️.rs:2340-2351`):

```rust
Editor::builder(crate::GENERATION3D_DIALECT).document(["semio", "procedural", "3d"])
    .mode_def(edit::definition())
    .mode_def(generate::definition())
    .default_mode_id(edit::GENERATION_3D_PLAY_MODE_EDIT)
    .mode_layout(generate::GENERATION_3D_PLAY_MODE_GENERATE, generate::GENERATION_3D_PLAY_LAYOUT_GENERATE)
    .window_kind_def(flow_window::definition())
    .window_kind_def(edit_preview::definition())
    .window_kind_def(generations::definition())
    .window_kind_def(form::definition())
    .window_kind_def(generate_preview::definition())
    .tool(crate::preview_eval::preview_eval_tool_definition())
    .default_layout(edit::layout())
    .named_layout(generate::layout())
    ...
    .window_kind_utilities(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
    .window_kind_action_refs(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, vec![...])
    .window_kind_interactions(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
```

**Two-window pattern for wfc `bitmap`/`2d-grid`/`3d-grid`/2d/3d**: each is exactly this
generation3d shape — one `mod.rs`-equivalent per window kind under `🎭️modes/✏️edit/🪟️windows/<name>/`,
each with its own `pub const …_WINDOW_…: &str` id and `pub fn definition() -> WindowKindDefinition`,
stitched by `.window_kind_def(...)` calls and one `create_default_layout(&[idA, idB], "row", ...)`
call in the mode's `layout()`.

### 1.3 Rendering a different UiNode tree per window id

Each window module owns its own `render(...)` function returning `UiAssemblyResult<BuiltNode>`
(no giant match on window id inside one function — the SPLIT INTO MODULES *is* the per-window
branch). E.g. preview window `render` (`…/🪟️windows/👁️preview/🦀️.rs:80-103`):

```rust
pub fn render(document: &Generation3dSnapshot, config: &Generation3dConfig, preview_eval_text: Option<&str>, ...) -> UiAssemblyResult<BuiltNode> {
    let payload = preview_payload(&eval_json, &document.host_snapshot, config, Some(session), marks);
    let (meshes_json, instances_json) = (payload.meshes_json, payload.instances_json);
    crate::accessible_scene_surface(GENERATION_3D_PLAY_SURFACE_PREVIEW, SurfaceKind::World3d,
        &World3dScene { status_json, fit_json: Some(fit_json), ..world3d_scene(preview_camera_json(config), meshes_json, instances_json, selection_json, &sun) },
        labels.preview_canvas.as_str(), labels.preview_canvas_hint.as_str(), Liveness::Polite)
}
```

vs. flow window's own `render` (in `…/🪟️windows/🕸️flow/🦀️.rs`) building a `NodeGraphScene`
instead. The top-level `ArtifactEditor`/`AppDefinition` dispatch looks up the right module by
matching `window.window_kind_id()` (see generation3d editor root `🦀️.rs:279`:
`if window.window_kind_id() == generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW { ... }`),
but per-window RENDER LOGIC itself is just "call that window module's `render`" — the branch is
structural (module boundary), not a big match arm.

### 1.4 Sibling references worth copying directly

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d` — same two-window (flow + 2d preview)
  shape as generation3d, plus `🎮️commands/🔌️connect-media-ports` and `🚚️move-media-node` (a node
  graph whose nodes ARE media/images — closely relevant to a "graph with slots that hold tiles").
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag` — single-window `NodeGraphScene` editor, the cleanest
  minimal "graph with slots" example (§5).
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d` (5499-line editor) and `🧊️3d` (8722-line editor) — both
  multi-pane (`overview`/`detail`/`selection` for 2d; `main` for 3d) with a shared per-instance
  `🪟️window/🦀️.rs` config module (§7).

---

## 2. Bitmap / pixel editing (`🖨️raster`) & transport limits

**Key finding: raster does NOT store pixels as an inline JSON/DslValue array on the document.**
`RasterArtifact` (`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:8-21`):

```rust
pub struct RasterArtifact {
    pub schema: String, pub id: String, pub title: Option<String>,
    pub layers: Vec<RasterLayerNode>,
    pub assets: RasterOwnedMap<RasterAssetChild>,
}
```

`RasterLayerNode::Pixel` (`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:465-489`) carries only
metadata + a key into `assets`, never bytes:

```rust
pub enum RasterLayerNode {
    Pixel { id: String, name: String, visible: bool, opacity: f32, blend_mode: String,
            transform: RasterTransform, mask: Option<RasterLayerMask>,
            width: Option<u32>, height: Option<u32>,
            #[dsl(key = "image")] image_key: Option<String> },   // ← points into `assets`
    Group { ..., children: Vec<RasterLayerNode> },
    Adjustment { ..., adjustment_kind: String, params: RasterOwnedMap<DslValue> },
}
```

The actual pixel bytes live in a **composed CHILD artifact** of kind `s.stdio.semio.image`
(`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:555-624`):

```rust
pub type RasterAssetChild = store::ArtifactChild<SemioImageSnapshot>;

fn mint_asset_child_handle(asset_id: &str, content_hash: u64) -> RasterAssetChild {
    let child_id = format!("raster-asset-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("{asset_id}-image"), dialect };
    store::ArtifactChild::new(child_id, target)
}
pub fn mint_raster_asset_child(asset_id: &str, asset: &RasterImageAsset) -> RasterAssetChild {
    match io::semio_image_snapshot_from_raster_asset(asset) {
        Ok(image) => image_content_child_handle(asset_id, &image).with_local_owner(std::sync::Arc::new(image)),
        Err(_) => image_asset_child_handle(asset_id, asset),
    }
}
pub fn raster_asset(assets: &RasterOwnedMap<RasterAssetChild>, asset_id: &str) -> Option<RasterImageAsset> {
    let handle = assets.get(asset_id)?;
    let image = handle.local_owner::<SemioImageSnapshot>()?;
    io::raster_asset_from_semio_image_snapshot(image.as_ref()).ok()
}
```

Content-addressed: the child id is hashed off the DECODED canonical content
(`image_content_child_handle`), not the source bytes, so re-encoding the same image is idempotent
at the handle level (comment at `🦀️.rs:597-609`). **Implication for `wfc`'s bitmap artifact**:
store each input/output bitmap as a composed `s.stdio.semio.image` (or `s.stdio.semio.gif`, which
has real per-pixel-index mutations, see below) child, referenced by a `String` key on the wfc
document — never as an inline pixel array.

### 2.2 The actual "click a pixel → mutation" example

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif` has a real indexed-pixel document with a
`set-image-pixels` mutation
(`…/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-image-pixels/🦀️.rs:8-13`):

```rust
pub struct SetImagePixels {
    pub(crate) index: usize,
    #[dsl(base64)] pub(crate) indices: Vec<u8>,
}
```

— the whole pixel-index buffer for one GIF sub-image rides base64-encoded in ONE mutation leaf
(bulk replace, not per-pixel deltas). This is the transport-safe way to do bulk pixel writes: one
mutation carrying `Vec<u8>` (base64), sized to the image, not one mutation per clicked pixel.

### 2.3 Rendering the pixel/paint canvas — `SurfaceKind::Paint2d`

Raster's composite window (`…/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs:17-47`):

```rust
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { id: RASTER_PLAY_WINDOW_COMPOSITE.into(), surface_kind: SurfaceKind::Paint2d, icon_id: "image".into(), .. }
}
pub fn render(document: &RasterDocument, config: &RasterConfig, active_utility: &str) -> UiAssemblyResult<BuiltNode> {
    scene_surface(RASTER_PLAY_SURFACE_COMPOSITE, ContractSurfaceKind::Paint2d, &raster_scene(document, config, active_utility, "composite"))
}
```

`Paint2dScene` (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:1619-1635`):

```rust
pub struct Paint2dScene {
    pub document_sync_json: String,   // layers/strokes/masks — the RETAINED lane
    pub assets_json: String,          // one entry per imported bitmap — the RETAINED lane
    pub camera_json: String, pub selection_json: String, pub hovered_id: Option<String>,
    pub active_utility: String, pub brush_size: f64, pub brush_opacity: f64, pub view_mode: String,
    pub composite_viewport_json: Option<String>,
    pub lanes: Vec<SceneLaneRef>,     // the paged-out-of-spine manifest
}
```

Hit testing/click→mutation for raster is NOT one-mutation-per-pixel either: brush/eraser strokes
are captured client-side and committed as `🩹️patch-layer` / `🧵️patch-layers` mutations
(`✏️editor/🎮️commands/🩹️patch-layer/🦀️.rs`, `🧵️patch-layers/🦀️.rs`), i.e. coalesced batches, same
principle as gif's `set-image-pixels`.

### 2.4 The transport-limit numbers to respect

| limit | value | source |
|---|---|---|
| `UiText` capacity | **`UI_TEXT_MAX_BYTES` = 512 bytes** (fixed inline `[u8; 512]` byte array) | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:18,74-110`; TS mirror `TEXT_BYTES=512` at `…/🧬️contract/🧵️retained/📦️wire/🟦️.ts:11` |
| Fixed-capacity surface doc (`scene_surface` spine) | **`UI_FIXED_BYTES` = 32 × 1024 = 32 KiB** | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:22` |
| Per-surface reconcile ceiling | **8 MiB** (`SURFACE_RECONCILE_SURFACE_BYTES`) of a 32 MiB aggregate, ≤3 surfaces/frame | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:1866,2284` |
| Wasm-guest single-allocation ceiling | **`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` = 65 536 = 64 KiB** — dlmalloc's `memory.grow` granularity; exceeding it on a per-command/per-turn guest path risks an unrecoverable trap | `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs:36-47` (doc comment cites a real build-#29 incident: 262 272 B/command at 4 Hz tripped `plugin.command-page-allocation` 32× before trapping) |
| Assembled host-answer ceiling (one outstanding request) | **`GUEST_HOST_ANSWER_CEILING_BYTES` = 8 388 608 = 8 MiB**, split into 64 KiB pages via `guest_host_answer_pages(bytes)` | same file, `:49-65` |
| `UiPatch`/document bounds | `UiDocumentLimits { max_nodes: 20_000, max_depth: 128, max_children: 4_096, max_text_bytes: 65_536, max_patch_ops: 4_096, max_patch_bytes: 1_048_576 }` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🦀️.rs:29-61` |
| Raster's own bitmap-asset capacity | `RASTER_OWNED_MAP_CAPACITY = 64` distinct imported bitmaps, `RASTER_OWNED_MAP_PAGE_CAPACITY = 8`, `RASTER_OWNED_MAP_PAGE_BACKING_BYTES = 16 * 1024` (16 KiB/page credit) | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:27-30` |

**Raster's actual pixel-editing loop is entirely client-side WASM, not per-pixel document
mutations at all.** `RasterSession` (`🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs`) exports
`pointerDownScreen`/`pointerMoveScreen`/`pointerUpScreen` via wasm-bindgen; `paint_at(world)`
(`:520-555`) stamps a circular brush directly into an in-memory `HashMap<String, Vec<u8>>` keyed
`"layer:{id}"` (raw RGBA8, `width*height*4` bytes) — **no mutation is emitted per stroke sample**.
Only a committed whole-layer/whole-region result is later synced back to the document as a
PNG-encoded, base64 `RasterImageAsset { mime, data }` via `add-layer-asset`/`patch-layer`. The
generic single-image `framework.window.image` body (`ImageWindowKit`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31226-31258`) is capped by the 512-byte
`UiText` `src` field and is therefore **only viable for a thumbnail**, never a full-resolution
bitmap — confirmed by raster reserving it for its separate read-only *viewer* app, never for the
live paintable composite.

**Decision for `wfc`'s bitmap artifact**: if per-cell click→mutation semantics are wanted (rather
than raster's free-hand-then-batch-commit model), there is no existing "one mutation per pixel"
precedent to copy — design a bounded region-patch mutation (e.g. `set-cell`/`set-region`, base64
`Vec<u8>` payload like gif's `set-image-pixels`, §2.2) and, if a single edit can exceed the 64 KiB
guest-allocation / 1 MiB `UiPatch` ceilings, drive it through the **point-invertible retained-row**
mechanism (`ArtifactRetainedWorkCapacity`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:104-160`)
— `rows(items) = items * store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS`, i.e. split a big
edit into N bounded, independently-undoable rows instead of one oversized mutation.

**The mechanism that reconciles "bitmaps can be large" with "the surface doc is 32 KiB fixed"**:
`SceneDoc::split_lanes`/`merge_lane`, implemented identically for `Paint2dScene` AND for puzzle's
`Board2dScene` (§4) — large fields (`document_sync_json`, `assets_json`) are pulled OUT of the
32 KiB spine into their own `SceneLanePayload`s carried as separate reserved body keys
(`framework.scene.paint2d.documentSync`, `…assets`), referenced from the spine only by a
`SceneLaneRef { lane, bytes, hash }`. Doc comment, `🎬️scene/🎬️scenes/🦀️.rs:1660-1668`:

> "Both lanes scale with the DOCUMENT, not with the frame … Either outgrows `UI_FIXED_BYTES`
> (32 KiB) on a real painting, and a surface doc cannot page — `scene_surface.encode` refuses the
> whole surface outright, which is why a paint-2d window silently stopped updating past that size
> while Canvas2d and Board2d did not."

**wfc implication**: a `bitmap`/`2d-grid`/`3d-grid` output window whose pixel/cell payload can
exceed ~32 KiB MUST define its own `SceneDoc`-style lane split (bulk pixel/cell array riding a
lane body key, never inline in the spine JSON), exactly mirroring `Paint2dSceneLane` /
`Board2dSceneLane::Fixture` (§4).

### 2.5 Three distinct kinds of cross-artifact/media reference — pick the right one

The framework distinguishes CHILD (owned, embedded lifecycle) from LINK (independent lifecycle,
pinned) — doc comment at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2856-2876`. Full
detail in §3.3-3.4 below; summary:

| relationship | type | lifecycle | use when |
|---|---|---|---|
| CHILD | `store::ArtifactChild<S>` (+ `ChildRef`, `genesis_child_pack`) | parent creates/deletes it; nests inline in UI; still its own `ArtifactEnvelope` | the tile/bitmap is OWNED by this wfc document (embedded, content-addressed) |
| LINK | `store::ArtifactLink { target, pin: LinkPin, role }` | independent lifecycle; renders as a chip, never nested inline | the tile references an EXTERNAL, independently-versioned artifact (e.g. a shared tile-catalog artifact) |
| plain URL/hash reference | `mesh_url: Option<String>` / `ImageLink { path, hash, proxy_data_url }` | resolved client-side by the renderer, lazily, keyed by id/url | the tile references a renderer-resolved asset (glTF/PNG) rather than another semio artifact |

---

## 3. Vector graphics tiles (`🖍️draw`) & cross-artifact references

### 3.1 Vector model

`DrawingArtifact` (`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:18-27`):

```rust
pub struct DrawingArtifact {
    pub schema: String, pub id: String, pub title: Option<String>,
    pub layers: Vec<DrawingLayerNode>,
    pub assets: BTreeMap<String, DrawingImageAsset>,   // raster assets embedded for reference tracing
    pub artboard: Option<DrawingArtboard>,
}
```

Path segments — `…/🦀️.rs:378-393` (`PathSegment` enum, SVG-flavoured):

```rust
pub enum PathSegment {
    #[dsl(key = "M")] Move { to: [f64; 2] },
    #[dsl(key = "L")] Line { to: [f64; 2] },
    #[dsl(key = "Q")] Quad { ctrl: [f64; 2], to: [f64; 2] },   // + Cubic/Close, per the full enum
}
```

Rendered scene node (`DrawingSceneNode`, `…/🦀️.rs:131-149`):

```rust
pub struct DrawingSceneNode {
    pub id: String, pub transform: [f64; 6], pub segments: Vec<PathSegment>,
    pub fill: Option<FillStyle>, pub stroke: Option<StrokeStyle>, pub opacity: f64,
    pub blend_mode: String, pub visible: bool, pub fill_rule: Option<String>,
    pub text: Option<DrawingSceneText>, pub image: Option<DrawingSceneImage>,
}
```

### 3.2 Framework rendering primitive — `SurfaceKind::Canvas2d`

`…/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs:9,116`: the drawing window declares
`surface_kind: SurfaceKind::Canvas2d` — draw's vector paths ride the SAME generic `Canvas2d`
surface used by many unrelated 2D editors (architect's adjacency window, animate's tile-cropper).
There is no dedicated "vector/scene2d" SurfaceKind — `Canvas2d` carries an app-defined scene
(here a list of `DrawingSceneNode`s with path segments) as opaque pack-encoded payload; the
renderer target (wgpu/react) interprets `doc_schema` to know it's a drawing scene.

### 3.3 Cross-artifact references — the `ArtifactChild`/`ArtifactRef` pattern

Two independent, structurally identical, confirmed examples of "one artifact embeds a reference to
another artifact's content" (the mechanism `wfc`'s "tile = vector-graphic-OR-bitmap" and "3d-grid
cell = mesh" needs):

**Raster → image** (`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:567`):
```rust
pub type RasterAssetChild = store::ArtifactChild<SemioImageSnapshot>;
```
(full construction shown in §2.1).

**Remodel → mesh** (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🦀️.rs:431-436`):
```rust
fn mesh_child_handle(child_id: String, artifact_id: String) -> RemodelingMeshChild {
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    let target = store::os_io::ArtifactRef { artifact_id, dialect };
    store::ArtifactChild::new(child_id, target)
}
```

Both use `store::ArtifactChild<T>` wrapping a `store::os_io::ArtifactRef { artifact_id, dialect: ArtifactDialect { artifact_kind, standard, subset } }`
— a content-addressed, composed CHILD artifact reference (see project memory "Composed Child Load
Requirements": `child_id == target id + unique`, `Members` on both surfaces). `local_owner::<T>()`
resolves the child's materialized content when the host has hydrated it; a wire-decoded child not
yet materialized "fails soft" (`raster_asset`, §2.1) rather than panicking.

`lowpoly` also exposes a `LowpolyMeshHandle` JSON-schema type
(`💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-object/🧬️schema/🔣️.json:41`)
for the same purpose in its own document.

### 3.4 Importing external media into a document — `host_media_handler`

Plugins that need to accept an externally-authored mesh/image register an import bridge on the
plugin manifest, e.g. `cad` (`✏️s/🔌️plugins/📐️cad/🦀️.rs:31`):

```rust
.host_media_handler(HostMediaHandlerDeclaration::mesh_import(
    "s.cad.host-media.mesh-import", crate::artifacts::cad::artifact_kind(),
    crate::artifacts::cad::CAD_DOCUMENT_SCHEMA, crate::artifacts::cad::io::cad_document_from_mesh)?)
```
and `procedural` (`✏️s/🔌️plugins/🌀️procedural/🦀️.rs:101`) do the same. The builder
`mesh_import(id, artifact_kind, artifact_schema, importer)` is defined at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:3626`; the host resolves it via
`self.runtime.host_media_handlers.mesh_import(request)` (same file, `:30584`).

**wfc implication**: a bitmap/2d-grid/3d-grid tile that is "a vector graphic OR a bitmap" should be
modeled as an enum of two `ArtifactChild` variants (one pointing at `s.draw.drawing`, one at
`s.stdio.semio/v1/image`) referenced by id from the WFC document's tile-catalogue, exactly like
raster's `image_key: Option<String>` into `assets: RasterOwnedMap<RasterAssetChild>`. Do not embed
raw path/pixel data inline in the WFC document if catalogues can be large.

### 3.5 `SemioMembers` — the closed catalogue a composed child opens against

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1163-1185` declares an 18-variant closed enum of
composable content kinds via `dsl::space_members!`:

```rust
dsl::space_members! {
    pub enum SemioMembers, SemioMembersOpen {
        Animation(...), Audio(...), Brep(...), Cad(...), Document(...), Drawing(...), Flow(...),
        Graph(...), Image(...), Kit(...), Mesh(...), Model(...), Object(...), Presentation(...),
        Table(...), Text(...), Value(...), Video(...),
    }
}
```

`Drawing`, `Image`, and `Mesh` are first-class member subsets here — exactly the three payload
kinds wfc's tiles need (vector, bitmap, mesh). A container artifact opens a child via
`MemberFactory::create`/`open` (`store/🦀️.rs:19802-19810`); `NoMembers` is the uninhabited
stand-in when an artifact composes nothing. Trinity's `jack` artifact is a clean, complete worked
example of one container owning a composed child through this path —
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs:180` (`type JackContentChild = store::ArtifactChild<SemioGraphSnapshot>`),
`:285-295` (content-addressed handle minting), `:328-334` (`genesis_jack_child_pack`, the load-time
pack-derivation callback `ChildRestoreProjection` calls — see project memory "Composed Child Load
Requirements": `child_id == target.artifact_id`, uniqueness, complete admitted set).

### 3.6 `ArtifactLink` — for referencing an independently-owned tile/catalog artifact

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🦀️.rs:1-29`:

```rust
pub struct ArtifactLink { pub target: store::os_io::ArtifactRef, pub pin: LinkPin, pub role: String }
pub enum LinkPin { Head, Checkpoint { id: String }, Snapshot { blob: BlobRef } }
```

`ArtifactRefs` trait (`store/🦀️.rs:3220-3226`) is what a snapshot implements to declare both its
children AND links (`async fn child_refs()`/`async fn links()`, both default-empty); resolution
(`LinkResolver`/`MemberDirectory`, `:3239-3290`) is lazy/host-side, never during composition
dispatch. `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️.rs:44-77` shows BOTH mechanisms on one
artifact — `background_drawing: Option<LayoutDrawingChild>` (owned CHILD, embedded page art) vs.
`referenced_model: Option<store::ArtifactLink>` with `#[link_slot(roles("model"))]` (a LINK: "layout
pages can reference an architecture `model` artifact … without owning/duplicating its content").
**This is the direct precedent for a wfc tile catalogue that lives in its own shared artifact**
(e.g. multiple wfc boards drawing tiles from one catalog) — use `ArtifactLink{role:"tile-catalog"}`
rather than duplicating the catalog into every board document.

`ImageLink` (`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️.rs:347-364`) is a THIRD, lighter option
— an InDesign-style external-file reference (`path`, content `hash`, plus an inline base64
`proxy_data_url` so a placed frame renders before the real file resolves) — worth considering if
wfc tiles should point at externally-managed image files rather than other semio artifacts.

### 3.7 `Media`/`MediaType`/`MediaPayload` — cross-PLUGIN port wiring, not document storage

`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:5280-5657` — the value shape that flows over a workflow
port (`export_media`/`import_media`), distinct from CHILD/LINK document storage:

```rust
pub enum MediaClass { TwoD, ThreeD, Text, Data, Graph, Kit, Computation, Presentation }
pub enum MediaForm { Any, Vector, Raster, Brep, Mesh, Document, Value, Dag, Trinity, Type, Design, Kit, Flow, Sequence, Procedure, Deck }
pub struct MediaType { pub class: MediaClass, pub form: MediaForm }
pub enum MediaPayload { Structured { schema: String, json: String }, Binary { format_kind: String, blob_hash: String } }
```

Draw's own vector export port uses `MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }`
(`✏️s/🔌️plugins/🖍️draw/…/✏️editor/🦀️.rs:1637-1668`). No literal `MediaHandle` type exists in the
repo — `Media`/`MediaPayload`/`MediaFingerprint` is the real mechanism, and `MediaPayload::Binary`'s
`blob_hash` is itself a handle into `store::BlobStore`. Relevant to wfc only if it exposes a flow
port (e.g. "tile catalog in" / "solved grid out") rather than for the document's own tile storage.

### 3.8 Simplest option — plain URL/id reference, resolved client-side (puzzle-3d's `mesh_url`)

`FixtureObject.mesh_url: Option<String>` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/🧬️schema/🦀️.rs:451-466,584`)
is resolved ONLY in the viewer/renderer, never fetched in Rust:
```rust
fn puzzle3d_view_meshes_json(document: &Puzzle3dSnapshot) -> String {
    let urls: Vec<String> = document.objects.iter().filter_map(|o| o.mesh_url.clone()).collect();
    world3d_meshes_json_from_kinds_and_urls(&[PUZZLE3D_VIEW_FALLBACK_MESH_KIND.to_string()], &urls)
}
```
`world3d_mesh_id_from_url(url)` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:39223-39229`)
derives a stable `mesh:<slug>` id by stripping the path/extension; `meshes_json`'s `{id, url}`
entries are "A REFERENCE, never the geometry" (comment at same file `:39204`) — bytes are fetched
lazily client-side, keyed by id/url, mirroring `imageCache.get(layer.image.src)` for 2D bitmaps in
`Canvas2dHost` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx:251`).
This is the LOWEST-ceremony option if wfc tiles are simple externally-hosted assets rather than
first-class semio artifacts.

---

## 4. Grid canvas with cells — `SurfaceKind::Board2d`

`Board2d` is the shared "free-placed nodes/handles/edges on a snap-grid, each with a per-cell
glyph" surface, used by `🧩️puzzle`'s 2d/5d artifacts and `🧱️block`'s 2d/5d artifacts. It is closer
to "arbitrary graph with a grid backdrop" than "literal fixed 2D array of cells" — see below for
what to actually copy for wfc's **regular** grid.

`Board2dScene` (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:2018-2075`):

```rust
pub struct Board2dScene {
    pub fixture_json: String,           // nodes/edges/handles — the RETAINED lane
    pub camera_json: String,
    pub glyph_catalogs_json: String,    // per-node TILE/glyph catalogue (image or vector look-up)
    pub selection_json: String,
    pub interactive: bool, pub hovered_id: Option<String>, pub active_utility: Option<String>,
    pub selection_method: String,
    pub grid_visible: bool, pub grid_snap_enabled: bool, pub grid_factor: f64,
    pub selectable_nodes: bool, pub selectable_edges: bool, pub selectable_handles: bool,
    pub area_brush_size: Option<String>,   // "one click paints a WxH area" — literally grid-cell painting
    ...
}
```

Puzzle2d's `render_canvas` (`…/🧩️puzzle/🗿️artifacts/◻️2d/.../✏️editor/🎭️modes/✏️edit/🦀️.rs:219-226`):

```rust
/// 🖼️ The board-2d surface node for one pane — bound by each window's own `render()`. The fixture rides
/// as its own paged lane carrier (`Board2dSceneLane::Fixture`), so a 180-node board is bounded by its
/// carrier pages instead of by the 32 KiB surface doc that refused it outright.
pub fn render_canvas(document_json: &str, envelope: &Puzzle2dScene, pane: &str) -> UiAssemblyResult<BuiltNode> {
    let scene = puzzle2d_board_scene(document_json, envelope, pane);
    scene_surface(&format!("{PUZZLE2D_PLAY_SURFACE_ID}.{pane}"), SurfaceKind::Board2d, &scene)
}
```

Same three panes (`overview`/`detail`/`selection`) all declare `surface_kind: SurfaceKind::Board2d`
(`…/🪟️windows/👁️overview/🦀️.rs:29`, `🔍️detail/🦀️.rs:24`, `🎯️selection/🦀️.rs:24`) — i.e. ONE
`Board2dScene` payload rendered into three windows at different zoom/focus, the same "one document,
several window-scoped views" idea generation3d uses across flow/preview.

Puzzle2d also has an `area_brush_size`/`area_brush_width`/`area_brush_height` concept — "one click
paints a WxH world-space rectangle" — the closest existing analog to "click paints a grid cell"
hit-testing, and its persisted per-window camera/grid config
(`Puzzle2dWindowConfig { camera_x, camera_y, camera_zoom, grid_visible, grid_snap_enabled,
grid_factor, area_brush_width, area_brush_height, ... }`, §7) is the pattern to copy for a wfc
grid window's own per-window state.

**For a literal fixed-size regular grid (wfc's `2d-grid`)**: reuse `Board2d` with
`grid_snap_enabled: true` and `grid_factor` = cell size, `fixture_json` holding one "piece" node
per occupied cell (position quantized to the grid), and `glyph_catalogs_json` holding the small
catalogue of tile glyphs (vector or bitmap) that `wfc`'s tile ids resolve to — this reuses the
exact same paged-lane mechanism (`Board2dSceneLane::Fixture`) that already solves "the grid can be
bigger than 32 KiB."

---

## 5. Graph with slots/edges — the reusable `NodeGraphScene` primitive

`NodeGraphNodeRecord` / `NodeGraphEdgeRecord` / `NodeGraphPortRecord`
(`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:881-928`):

```rust
pub struct NodeGraphPortRecord { pub id: String, pub label: Option<String>, pub code: Option<String>,
    pub abbreviation: Option<String>, pub full_name: Option<String>, pub artifact_kind: Option<String>, pub value_type: Option<String> }

pub struct NodeGraphNodeRecord { pub id: String, pub label: Option<String>, pub x: f64, pub y: f64,
    pub width: f64, pub height: f64, pub inputs: Vec<NodeGraphPortRecord>, pub outputs: Vec<NodeGraphPortRecord>,
    pub instance_id: Option<String>, pub plugin_id: Option<String>, pub app_id: Option<String>, pub icon: Option<String> }

pub struct NodeGraphEdgeRecord { pub id: String, pub source_node_id: String, pub source_port_id: String,
    pub target_node_id: String, pub target_port_id: String, pub label: Option<String> }
```

`NodeGraphScene` (`…/🦀️.rs:1273` onward) wraps `nodes: Vec<NodeGraphNodeRecord>`,
`edges: Vec<NodeGraphEdgeRecord>`, `viewport`, `selection`, `hover`, `highlighted`, plus optional
`operators`/`find_items` for a node palette. This is THE reusable "arbitrary-shape graph of
slots+adjacencies" canvas primitive — exactly wfc's `2d`/`3d` window requirement (nodes = slots,
edges = adjacencies, arbitrary/non-grid layout).

### 5.1 Minimal working example — `🕸️dag`

Window kind (`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/.../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️main/🦀️.rs:18-35`):
```rust
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { id: DAG_PLAY_WINDOW_MAIN.into(), surface_kind: SurfaceKind::NodeGraph, icon_id: "graph-dag".into(), .. }
}
```
Render (`…/🦀️.rs:41-46`):
```rust
pub fn render(document: &DagSnapshot, camera: &DagCamera, _labels: &DagPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let (nodes, edges) = document_to_workflow(document);
    let viewport = Viewport2d { x: camera.x, y: camera.y, zoom: camera.zoom };
    scene_surface(DAG_PLAY_SURFACE_MAIN, SurfaceKind::NodeGraph, &NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) })
}
```
Connect gesture → mutation (`…/🧬️schema/🧬️mutations/🤝️connect-nodes/🦠️mutation/🦀️.rs:9-19`):
```rust
pub struct ConnectNodes { pub id: String, pub source: String, pub target: String, pub route_style: EdgeRouteStyle, pub properties: PropertyBag }
pub fn connect_nodes(id: String, source: String, target: String, route_style: EdgeRouteStyle, properties: PropertyBag) -> DagMutation { ... }
```
`source`/`target` are `"<nodeId>@<portId>"` strings. Node move/create/delete/rename/resize/reorder
each get their own mutation-leaf folder under `…/🧬️mutations/` with `🦠️mutation`, `🔺️diff`,
`↩️inverse`, `🧪️tests` subfolders (`↔️move-node`, `🌱create-node`, `🗑️delete-node`,
`🔤change-node-name`, `📐resize-node`, `🔀reorder-nodes`, `🤝️connect-nodes`, `✂️disconnect-nodes`
— all present under `dag/…/🧬️schema/🧬️mutations/`).

### 5.2 Richer graph — generation3d's flow window

Same `NodeGraphScene` machinery but with typed operator ports and a node-graph-edit sub-operation
protocol (`…/🎮️commands/✏️node-graph-edit/🦀️.rs`): a SINGLE `NodeGraphEdit { operations_json: String }`
mutation carries a batch of sub-operations (`setHostSnapshot`, `deleteSelection`, `connect`,
`disconnect`, `move`, `setSlider`) coalesced per user gesture — see doc comment at `:24-35`:
`setSlider` "arrives once per coalesced round trip rather than once per pointer move … Its whole
press folds into ONE undoable edit". This is the pattern for a WFC slot-graph editor's drag/connect
gestures too — coalesce into one mutation per gesture, not one per pointer-move tick.

### 5.3 `🧩️assembly` — closest domain match: a WFC problem-spec editor that ALREADY EXISTS

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly` is an editor for a `AssemblySnapshot` whose
document is *literally* a WFC-shaped spec: `seed`, `slots` (`AssemblySlot { id, x, y, z,
pinned_module_id }`), `edges` (`AssemblySlotEdge { id, from_slot_id, to_slot_id }`), `modules`,
`weights`, `rules` — and its doc comment explicitly states the collapsed/solved assignment is
**never persisted**:

`…/✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️.rs:1-9`:
> "Assembly editor — `structure` window: a real, EDITABLE overview tree of the whole WFC problem
> spec (`AssemblySnapshot`'s seed/slots/edges/modules/weights/rules — never the solved assignment,
> which is an inference, not persisted state), built from the framework `TreeWindowKit` … `AssemblySlot`
> carries `x`/`y`/`z`, but no module ASSIGNMENT is ever stored on the snapshot … so a mesh view would
> have nothing solved to place — a rule/slot tree is the honest first-pass representation of the
> PROBLEM this artifact actually persists. A spatial view over `slots`' raw coordinates is a plausible
> follow-up".

It uses the framework **`TreeWindowKit`** (editable outline-tree window kit,
`semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit}`) rather than
`NodeGraphScene`, with actions `create-slot`/`delete-slot`/`create-rule`/`delete-rule`/
`connect-slots`/`disconnect-slots`/`change-weight`/`remove-weight`/`change-seed`
(`…/🦀️.rs:24-33`). **Read this artifact's whole schema
(`…/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/`) before designing
wfc's own slot/rule/weight schema — it is architecturally the same problem** (rules constrain which
module/tile may occupy a slot adjacent to another), and its doc comment flags the natural next step
(a spatial `NodeGraphScene`/`Board2dScene` view over `slots`' raw x/y/z) as exactly what wfc's
`2d`/`3d` graph-with-slots window should BE.

---

## 6. 3D mesh preview & instancing

`World3dScene` (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:348-388`, relevant fields):

```rust
pub struct World3dScene {
    pub camera_json: String,
    pub meshes_json: String,             // small catalogue: mesh_id → geometry
    pub instances_json: String,          // many records: {id, meshId, position, rotation, scale, ...}
    pub instances_delta_json: Option<String>,  // incremental {base, revision, count, changed, removed}
    pub selection_json: String,
    ...
}
```

The doc comment on `instances_delta_json` (`:355-362`) states the residency contract: `instances_json`
stays AUTHORITATIVE on every publication; a consumer that tracked a prior `base` revision may apply
only `changed`/`removed` instead of re-parsing the whole set. This is the mechanism for a `3d-grid`
window with hundreds of cells where only a few change per tick.

### 6.1 Concrete instance-record shape — puzzle-3d

`Puzzle3dInstanceResidency` / `instance_record_json`
(`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/.../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:206-235`):

```rust
fn instance_record_json(object: &Puzzle3dObject, mesh_id: &str, provisional: bool) -> String {
    let instance = json!({
        "id": object.id, "meshId": mesh_id,
        "position": [object.origin[0], object.origin[1], object.origin[2]],
        "rotation": object.orientation.unwrap_or([0.0,0.0,0.0,1.0]),
        "scale": object_scale_json(object),
        "label": ..., "disabled": object.locked,
    });
    ...
}
pub struct Puzzle3dInstanceResidency { /* fingerprints each object, tracks delta */ }
```

— exactly N instances each referencing ONE of a handful of `meshId`s, with pose (position/
quaternion rotation/scale) per instance. **This is the pattern for wfc's `3d-grid` preview**: mint
one mesh entry per distinct wfc tile/module (few), then emit one `instances_json` record per
occupied grid cell (many) referencing that tile's `meshId` with `position` = the cell's world
coordinate — no per-cell mesh duplication.

### 6.2 Non-uniform grid / per-axis cell sizes

No literal "non-uniform box grid" primitive was found pre-built in `World3dScene`/puzzle-3d/lowpoly
— cell placement is computed by the PLUGIN (via `object.origin`/`position`) before it ever reaches
the scene payload, so per-axis non-uniform spacing is just an artifact-side concern: compute each
cell's world-space center from a `Vec<f64>` of per-axis cell-boundary offsets (cumulative sums per
axis) when building `instances_json`'s `position` field — there is no framework-level grid-geometry
helper to lean on, but nothing prevents it either since positions are freeform floats.

### 6.3 Mesh import / cross-artifact mesh reference

See §3.3 (`store::ArtifactChild<MeshSnapshot>` via `RemodelingMeshChild`/`LowpolyMeshHandle`) and
`HostMediaHandlerDeclaration::mesh_import(id, artifact_kind, artifact_schema, importer)`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:3626`) for importing externally-authored
meshes as composed children. `register_mesh_importer`/`register_mesh_exporter` at the OS-host level
(`🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:2215,4920`) are the lower-level registration points these
plugin-level declarations funnel into.

Generation3d's preview window builds `meshes_json`/`instances_json` from its own tessellation
result via `preview_payload(...)` (`…/🪟️windows/👁️preview/🦀️.rs:83-88`) — worth reading alongside
puzzle-3d's residency code if wfc's 3d preview needs incremental re-tessellation too. Its dedup key
is the geometry HANDLE, not the emitting widget (`…/✏️editor/🦀️.rs:2890-2905`): two different
widgets/slots that resolve to the same handle share ONE tessellated mesh entry in `meshes` and each
still get their own `instances` entry — "tessellate once per distinct geometry handle, push one
lightweight instance record per placement" is the exact pattern wfc's 3d-grid needs (one mesh per
distinct tile/module, one instance per occupied cell).

### 6.4 What actually makes this "instancing" (GPU-level proof)

The wire-level dedup (`meshes_json`/`instances_json`) is backed by REAL single-draw-call GPU
instancing, not just a JSON convenience. `Instance3d`/`SceneDraw3d`
(`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs:978-991`):

```rust
pub struct Instance3d { pub id: String, pub model: Mat4, pub color: [f32; 4], pub selected: bool, pub hovered: bool }
pub struct SceneDraw3d { pub mesh_key: String, pub mesh_version: u64, pub instances: Vec<Instance3d> }
```

Every `Instance3d` sharing one `mesh_key` is grouped into one `SceneDraw3d` and drawn with a single
hardware instanced draw call — `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:3019-3027`:

```rust
fn draw_world_range<'a>(pass: &mut wgpu::RenderPass<'a>, mesh_store: &MeshGpuTable, draw_call: &WorldDrawRange, instance_buffer: wgpu::BufferSlice<'a>, instance_stride: u64) {
    let Some(mesh) = mesh_store.get_versioned(&draw_call.mesh_key, draw_call.mesh_version) else { return };
    pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    pass.set_vertex_buffer(1, instance_buffer.slice(byte_offset..byte_offset + draw_call.instance_count as u64 * instance_stride));
    pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(0..mesh.index_count, 0, 0..draw_call.instance_count);
}
```

Mesh vertex/index buffers upload ONCE into `MeshGpuTable` keyed by `mesh_key`+`mesh_version`; per
frame every instance sharing that mesh packs its model matrix into one instance buffer slice and is
drawn with ONE `draw_indexed(..., 0..instance_count)` call. **Confirms**: a `3d-grid` with hundreds
of cells sharing a handful of tile meshes costs one draw call per distinct tile kind, not per cell
— exactly what dedup-by-`meshId` in `instances_json` is for.

### 6.5 Modeling a whole-grid WFC collapse as a background `ToolRunDefinition`

Puzzle-3d's `🪣️fill` tool is architecturally the closest precedent for "a generator that places many
instances into a 3D grid over time, with live provisional preview, then commits once" —
`…/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:41-56`:

```rust
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: true,
        rebase: ToolRunRebasePolicy::Revalidate,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: puzzle3d_fill_run_unit(), stages: puzzle3d_fill_run_stages(), counters: puzzle3d_fill_run_counters(), reasons: puzzle3d_fill_run_reasons(),
        trace: ToolRunTraceKind::Instance3d,
        run_job: JobKindId::new(RUN_JOB_KIND), revalidate_job: Some(JobKindId::new(REVALIDATE_JOB_KIND)),
        settings: ToolRunSettingsReads { config: RUN_SETTINGS_CONFIG, ..Default::default() },
        windows: Vec::new(),
    }
}
```

The framework injects start/pause/resume/step/abort/finalize/dismiss; the app supplies only
`build_run_job` (`:100-117`), which emits `FillRunEvent::Constructed { mesh_url, origin,
orientation, scale }` per placement into a provisional ledger — rendered live via
`request.provisional` (`ToolRunTraceKind::Instance3d` — the trace kind literally matches
`Instance3d`, §6.4) — and only the FINAL state publishes as one document edit on finalize. **This
is the template for a WFC collapse run**: model the whole-grid solve as a `ToolRunDefinition {
mutating: true, trace: ToolRunTraceKind::Instance3d, ... }`, where each run step collapses one or
more cells and the provisional ledger shows the in-progress board live, committing the settled grid
as one edit at the end (or per-checkpoint, if incremental undo points are wanted).

---

## 7. Modes, tools, transient state & per-window config

### 7.1 Modes + tools

`ModeDefinition` (used above, generation3d `…/🎭️modes/✏️edit/🦀️.rs:14-27`) carries `id`, `label`,
`icon_id`, `tools: Vec<ToolRef>`, `layout_id`, `commands`. A **tool** declared at app level
(`.tool(crate::preview_eval::preview_eval_tool_definition())`) MUST be referenced by at least one
mode or the builder refuses the manifest — doc comment, `…/🎭️modes/✏️edit/🦀️.rs:10-13`:

> "a declared tool that no mode references is refused outright by the plugin builder
> (`app-definition.invalid: … tool previewEval is not referenced by any mode`) … so mode ownership
> is where a tool's scope is stated".

Pointer tools reach the editor as typed commands dispatched per gesture, one `🎮️commands/<name>/🦀️.rs`
folder per command (e.g. generation2d's `⬇️canvas-pointer-down`, `🖱️canvas-pointer-move`,
`⬆️canvas-pointer-up`, `🛞️canvas-wheel` — a literal click/drag/release/scroll quartet feeding a
canvas tool), each producing an `Emit<Mutation, ConfigMutation>` (see `apply_operations` in
`node-graph-edit`, §5.2, for the pattern of parsing a JS-side gesture payload into typed
sub-operations before emitting mutations).

### 7.2 Per-window persisted config vs. app-local transient vs. document

Three distinct state tiers, each with its own storage:

1. **Document** (`RasterArtifact`, `Generation3dSnapshot`, …) — persisted, versioned, mutated via
   the mutation/diff/inverse machinery.
2. **App-local transient** (`Generation3dTransient`,
   `…/✏️editor/🫧️transient/🦀️.rs:1-10`) — computed/cached state shared across ALL windows of one
   app instance, NOT part of the document, round-trips through its own DSL/pack envelope but has a
   trivial `MutationDiff` (`apply` just replaces wholesale, `absorb` overwrites) — i.e. it's a
   cache, not an edit history:
   ```rust
   pub struct Generation3dTransient { pub generation_preview_text: Option<String> }
   impl protocol::MutationDiff<Generation3dTransient> for Generation3dTransient {
       fn apply(&self, _base: &Generation3dTransient) -> protocol::MutationApplyResult<Generation3dTransient> { Ok(self.clone()) }
       fn absorb(&mut self, other: Self) { *self = other; }
   }
   ```
3. **Per-window-INSTANCE persisted config** — `Puzzle2dWindowConfig`
   (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/.../✏️editor/🪟️window/🦀️.rs:13-30`): camera, grid
   visibility/snap/factor, LOD mode, brush extents, selectable-kind toggles — ONE instance PER pane
   (three puzzle-2d panes = three independent `Puzzle2dWindowConfig`s), persisted via its own
   `dsl(id = "s.puzzle.puzzle2d.windowconfig", extension = "puzzle2dwindowcfg")` envelope, loaded/
   saved by `load_window`/`save_window` (per puzzle-3d's window file doc comment).

This maps directly onto the project convention **"Per-Frame State Belongs In Window Transient"**:
anything that changes every pointer-move tick (hover id, drag-preview position, slider-live-value)
must NOT go through document mutation (O(n²) coalesced-amend cost) — it belongs in the app
transient (tier 2) or is carried ephemerally in the scene payload itself (`hovered_id`,
`selection_json` on `Board2dScene`/`NodeGraphScene`/`World3dScene`), while only the FINAL committed
gesture (mouse-up, connect, drop) becomes a real mutation.

**The framework enforces this split at the type level** via `EphemeralEmit<A>`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10739-10763`):

```rust
/// The two EPHEMERAL lanes' emission, deliberately separate from Emit. The document lanes
/// (artifact/config/draft) all have an op log, an edit id, an undo group and a failure mode;
/// presence and transient have NONE of those. They cannot fail, cannot be undone, never enter a
/// checkpoint and never appear in the command log.
pub struct EphemeralEmit<A: ArtifactApp> {
    pub presence: Vec<A::PresenceMutation>,               // broadcast to peers, never persisted
    pub transient: Vec<A::TransientMutation>,              // app-local, never leaves this client
    pub window_transient: Vec<WindowTransientMutation>,    // local-only, addressed to one window instance
}
```

Concretely, lowpoly's drag-begin command returns `ArtifactCommandWorkStep::CompleteWithEphemeral`
with an EMPTY `Emit::default()` and only a `transient` mutation
(`✏️s/🔌️plugins/💠️lowpoly/…/✏️editor/🦀️.rs:2027-2031`) — mid-drag ticks emit ZERO document
operations; only `transformEnd` produces a real `Emit`, per the module doc comment
(`…/✏️editor/🎮️commands/🧲️transform/🦀️.rs:1-3`): *"Mid-drag ticks emit zero operations; the whole
drag commits as one `Objects(Patch)` on `transformEnd`."* **This is the exact shape a wfc collapse
gesture (drag-to-paint constraints, live candidate-set preview) should follow**: every intermediate
tick → `EphemeralEmit{transient/window_transient}` only; the settled result → one real `Emit`.

### 7.3 "Active Utility Is Per-Window"

`ViewModel.active_utility_by_window_id: HashMap<String, String>` is the framework field; the
convention is: look up the CURRENT window first, fall back to the FOCUSED window, then fall back to
a legacy flat `active_utility_id`, then a hard default. Canonical implementation, `🗒️note`'s editor
(`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:99-111`):

```rust
/// 🧰️ The utility armed for the window being rendered or dispatched: the React host arms utilities per
/// window instance (`active_utility_by_window_id`) and mirrors only the shell's ACTIVE window into the flat
/// `active_utility_id`, so a pane that is not the active window read `None` and ran every pencil drag as a
/// `selectDirect` pick (ticket 26/09/17/NOTE-PLUGIN-END-TO-END; draw's `drawing_active_utility` precedent).
pub fn note_active_utility(view: &semio_framework_plugin::ViewModel) -> &str {
    view.window_id.as_deref()
        .and_then(|window| view.active_utility_by_window_id.get(window))
        .or_else(|| view.focused_window_id.as_deref().and_then(|window| view.active_utility_by_window_id.get(window)))
        .map(String::as_str)
        .filter(|utility| !utility.is_empty())
        .or(view.active_utility_id.as_deref())
        .unwrap_or("selectDirect")
}
```

The same helper exists in puzzle-3d (`…/✏️editor/🦀️.rs:1003-1028`, `puzzle3d_scene_active_utility`)
and puzzle-5d (`…/✏️editor/🦀️.rs:944`) — **copy this exact 5-step fallback chain** for any wfc
window that arms different tools per pane (e.g. paint tool active in the bitmap INPUT window but
not the read-only OUTPUT window).

**Important distinction the field's own doc comment draws** (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4508-4517`):
*utilities are scoped PER WINDOW INSTANCE* (`active_utility_by_window_id`), while *tools are scoped
MODE-WIDE* (`active_tool_id`, mutually exclusive with a utility — e.g. a background `ToolRunDefinition`
like puzzle-3d's fill tool, §6.5). The React shell assembles the map fresh every dispatch —
`baseDispatchViewState` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6715-6727`
(`activeUtilityByWindowId: buildActiveUtilityByWindowId(...)`, `focusedWindowId: activeWindowIdRef.current`).
**For wfc**: scope the per-cell editing utility (brush-collapse / constrain / erase) per window
INSTANCE via `active_utility_by_window_id` (so a dual-pane grid can differ per pane), but keep the
whole-grid WFC solve itself mode-wide via `active_tool_id`/`ToolRef` — it is one run, not a
per-window setting.

---

## 8. Summary table — which primitive to reuse for each wfc artifact

| wfc artifact | window A | window B | primitive(s) to copy |
|---|---|---|---|
| `bitmap` | input bitmap (editable) | output bitmap (read-only result) | `SurfaceKind::Paint2d` + `Paint2dScene` (raster), pixel bytes as composed `s.stdio.semio.image`/`gif` child (§2), bulk `set-image-pixels`-style mutation, `Paint2dSceneLane` for >32 KiB payloads |
| `2d-grid` | grid canvas w/ cells | 2d preview (tile per cell) | `SurfaceKind::Board2d` + `Board2dScene` (puzzle-2d), `grid_snap_enabled`/`grid_factor`, `glyph_catalogs_json` for per-cell tile lookup, `Board2dSceneLane::Fixture` paging, tile = `ArtifactChild` into draw or image (§3) |
| `2d` | graph with slots (2d) | 2d preview | `SurfaceKind::NodeGraph` + `NodeGraphScene` (dag/generation3d), `AssemblySlot`/`AssemblySlotEdge` schema shape (§5.3), coalesced gesture→mutation (`node-graph-edit`) |
| `3d-grid` | grid box w/ cells | 3d preview (mesh per cell) | same grid concept as `2d-grid` but cell positions computed per-axis by the plugin (§6.2), `SurfaceKind::World3d` + `World3dScene.instances_json` keyed by few `meshId`s (puzzle-3d `instance_record_json`, §6.1) |
| `3d` | graph with slots (same as `2d`) | 3d preview (mesh per slot) | `NodeGraphScene` window identical to `2d`'s, `World3dScene` preview identical to `3d-grid`'s |

All five share: two-window-kind + one-layout declaration exactly like generation3d's `edit` mode
(§1.2), per-window persisted config like `Puzzle2dWindowConfig` (§7.2), and the
`active_utility_by_window_id` fallback chain (§7.3).
