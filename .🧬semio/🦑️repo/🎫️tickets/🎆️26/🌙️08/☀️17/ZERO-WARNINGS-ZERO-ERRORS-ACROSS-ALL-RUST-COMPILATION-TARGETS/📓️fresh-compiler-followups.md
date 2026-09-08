# Fresh Compiler Follow-Ups

Pass 343 addresses concrete diagnostics from native325 and the running WASI324 check:

- Removed one duplicated test attribute from the map vector-cache interaction law; the remaining attribute still registers the test.
- Moved the final component kind into the OS icon string instead of cloning the consumed registration row.
- Used fixed three-element array chunks in Stdio Base64 and glTF morph-delta traversal. Base64 retains its exact one/two-byte remainder and padding logic; glTF still validates the full component count before applying every position delta.
- Used the existing owned path field directly in the glTF permutation failure.

Changed files:

- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-core/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/🧬️mutation-support/🎞️material-animation/🦀️.rs`

Native325 had passed all 13 selected Surface laws, but retained the duplicate-attribute warning. The new compiler diagnostics were read directly from its saved JSON output with the system jq tool while new Nx graph construction was unavailable. These follow-up edits still require fresh compiler verification; no warning-free claim is made.
