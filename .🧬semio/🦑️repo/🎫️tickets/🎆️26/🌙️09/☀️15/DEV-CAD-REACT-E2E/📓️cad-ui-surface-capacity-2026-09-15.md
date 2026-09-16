# CAD React UI Surface Capacity Fix (2026-09-15)

> ⚠️ Superseded 2026-09-16 — the kind-reference workaround rendered every solid as a unit box at the origin. Inline tessellation rides the paged `meshes` lane and renders fine; see [cad-react-e2e-2026-09-16.md](./📓️cad-react-e2e-2026-09-16.md).

## Symptom

After load, Shape/Building windows fault with:

```
[DEBUG] PluginRuntime: actor cad#1 stopped without publishing requested UI surfaces
(missing=["1:cad-play-shape","1:cad-play-building"], status=idle)
```

## Root cause

`edit::world_meshes_json` inlined full BREP tessellation for every object into `meshesJson`. The Concrete Forest fixture encodes ~43 KiB per pane, which exceeds the fixed `UiFixedBytes` world-3d surface budget (`ui.fixed-capacity` at `mesh-window.scene`).

`MeshWindowKit::render` then fails; the reactor emits a surface render fault but **no** `UiPatch`, so `settlePluginTurn` throws while the host still has `surface.view.root === null`.

Repro (native):

```text
renders_world_scene_for_each_pane
→ PluginAssemblyError { code: "ui.fixed-capacity", message: "... 42943 bytes" }
```

## Fix

Publish **kind/url mesh references** (same contract as puzzle3d `world3d_meshes_json_from_kinds_and_urls`) instead of inline `"data"` buffers. Instances reference `typology_mesh_kind` or URL-derived `mesh:{slug}` ids.

## Files

- `✏️s/🔌️plugins/📐️cad/.../✏️editor/🎭️modes/✏️edit/🦀️.rs` — `world_meshes_json`, `world_instances_json`
- `✏️s/🔌️plugins/📐️cad/.../✏️editor/🧪️tests/🔬️unit/🦀️.rs` — regression tests

## Verify locally

1. `bun nx run @semio-tech/cad-plugin:component-dev`
2. `bun nx run @semio-tech/framework-os-dev:activate-cad-react-dev`
3. Hard reload `🛠️dev📐️cad⚛️react` (port 6020)

Console should no longer show the missing-surface settle error; look for `ui.surface-render` only if another fault remains.
