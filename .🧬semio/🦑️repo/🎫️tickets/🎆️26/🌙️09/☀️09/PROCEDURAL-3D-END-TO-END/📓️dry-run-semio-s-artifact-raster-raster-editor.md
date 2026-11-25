# Dry run: semio-s-artifact-raster-raster-editor
manifest `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml`, source 272727 bytes, carve metadata: False
note: the editor crate keeps wasm, args_bridge, demo_session, resource_policy and re-exports; consumers (8 dependents) keep their paths

## New crates
- `semio-s-artifact-raster-raster-media` (100260 bytes) depends on groups []; files: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️reserved-import/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️reserved-import/🪜️base64/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📤️export/🦀️.rs`
- `semio-s-artifact-raster-raster-app` (61634 bytes) depends on groups []; files: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔌️app-assembly/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖥️app/🦀️.rs`
- `semio-s-artifact-raster-raster-retained` (64388 bytes) depends on groups []; files: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌱️initialization/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📤️export/🧵️work/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📬️store-preparation/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖥️app/🌱️ephemeral/🦀️.rs`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/🦀️.rs`

group cycles: none

## Edits
- facade `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` + `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: replace the moved `#[path] mod` lines with `pub use <new crate>::...`; add the new crates as dependencies
- workspace manifest `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml`: members and [workspace.dependencies.<new crate>]
- workspace manifest `🌎️hub/Cargo.toml`: members and [workspace.dependencies.<new crate>]
- workspace manifest `🌎️hub/🧩️compositions/🖨️raster/📦️packages/🦀️rust/Cargo.toml`: members and [workspace.dependencies.<new crate>]
- direct dependents that name the facade (no change when the facade re-exports): 8
- `📋️build-owners.json`: one row per new crate, root = its `📦️packages/🦀️rust`
- 2 Cargo.lock files gain one package block per new crate and the facade's dependency edges: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.lock`, `🌎️hub/Cargo.lock`

## Verify (when released)
`repo cargo --store=shared --configuration=prototype --parallelism=1 -- check -p <each new crate> -p semio-s-artifact-raster-raster-editor --lib`, memory_pressure normal, at least 40 percent free, one cargo at a time; then re-read `units.jsonl` peaks and drop the carving entry when below 1536 MB.
