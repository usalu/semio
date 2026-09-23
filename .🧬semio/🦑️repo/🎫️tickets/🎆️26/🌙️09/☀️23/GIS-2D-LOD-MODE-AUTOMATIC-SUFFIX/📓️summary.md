# GIS 2D LOD Mode Automatic Suffix

When window LOD mode is `automatic`, the map window **LOD Mode** select shows the zoom-resolved band on the Automatic row as `Automatic (<id>)` (localized base label + band id from `MapHost::current_lod_json`).

Forced LOD modes keep the plain Automatic dropdown label.

## Verification

- `cargo check -p semio-s-artifact-gis-gismap --lib` — OK
- Unit tests added under `☑️options/🔽️lod-mode/🧪️tests`; full crate `cargo test --lib` blocked by pre-existing `Arc<GisMapSnapshot>: ArtifactCompositionFields` errors in binary mutation tests (unrelated).
