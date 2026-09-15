# Puzzle 3d reference planes missing in wgpu 3d

## Symptom

Concrete Forest (and any puzzle 3d document with `references`) lists reference rows in the document tree but shows no reference image planes in the 3d viewport under wgpu.

## Root cause

Puzzle 3d publishes `referencesJson` (and sibling overlay lanes) on `World3dScene`. React `World3dHost` parses `referencesJson` every frame. The wgpu path in `sync_world3d_state` never copied those JSON lanes into `World3dState.references`, so `render_world_3d` iterated an empty vector.

Reference image fetches were also stuck: the asset decode pump only published GLB mesh leases at `Ready`, so `ReferenceImage` responses never reached `apply_reference_image_bytes`.

## Fix

- `sync_world3d_scene_document_lanes` mirrors vortices, attractions, target volumes, and references from the scene into `World3dState` (digest-gated, bumps `interaction_revision` on change).
- `render_world_3d` reserves `ReferenceImage` asset requests for visible references without decoded pixels.
- Renderer asset pump applies decoded reference image bytes via `collect_world3d_asset_bytes` + `apply_reference_image_bytes`.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
