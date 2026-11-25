# Dry run: semio-s-artifact-raster-raster-editor-session
manifest `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️session/📦️packages/🦀️rust/Cargo.toml`, source 140480 bytes, carve metadata: False
note: config alone is 47 KB

## New crates
- `semio-s-artifact-raster-raster-editor-config` (46909 bytes) depends on groups []; files: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `semio-s-artifact-raster-raster-editor-presence` (20297 bytes) depends on groups []; files: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖱️selection/🦀️.rs`
- `semio-s-artifact-raster-raster-editor-scene` (38016 bytes) depends on groups []; files: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌱️initialization/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📬️preparation/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️scene-helpers/🦀️.rs`

group cycles: none

## Edits
- facade `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️session/📦️packages/🦀️rust/Cargo.toml` + `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️session/🦀️.rs`: replace the moved `#[path] mod` lines with `pub use <new crate>::...`; add the new crates as dependencies
- workspace manifest `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml`: members and [workspace.dependencies.<new crate>]
- direct dependents that name the facade (no change when the facade re-exports): 5
- `📋️build-owners.json`: one row per new crate, root = its `📦️packages/🦀️rust`
- 2 Cargo.lock files gain one package block per new crate and the facade's dependency edges: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.lock`, `🌎️hub/Cargo.lock`

## Verify (when released)
`repo cargo --store=shared --configuration=prototype --parallelism=1 -- check -p <each new crate> -p semio-s-artifact-raster-raster-editor-session --lib`, memory_pressure normal, at least 40 percent free, one cargo at a time; then re-read `units.jsonl` peaks and drop the carving entry when below 1536 MB.
